//! Type graph builder
//!
//! Orchestrates parsing and resolution to build complete IR TypeGraph

use crate::core::ir::*;
use crate::core::parser::{StructureDefinitionParser, StructureKind};
use crate::core::resolver::SchemaResolver;
use crate::core::{Error, Result};
use octofhir_canonical_manager::CanonicalManager;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

/// Type graph builder
pub struct TypeGraphBuilder {
    /// Parser for StructureDefinitions
    parser: Arc<Mutex<StructureDefinitionParser>>,

    /// Resolver for type lookups
    resolver: Arc<SchemaResolver>,

    /// Canonical manager
    manager: Arc<CanonicalManager>,

    /// FHIR version being processed
    fhir_version: FhirVersion,
}

impl TypeGraphBuilder {
    /// Create a new type graph builder
    ///
    /// # Arguments
    ///
    /// * `manager` - Canonical manager instance
    /// * `fhir_version` - FHIR version to build for
    pub fn new(manager: Arc<CanonicalManager>, fhir_version: FhirVersion) -> Self {
        let resolver = Arc::new(SchemaResolver::new(Arc::clone(&manager)));
        let mut parser = StructureDefinitionParser::new();
        parser.set_resolver(Arc::clone(&resolver));

        Self { parser: Arc::new(Mutex::new(parser)), resolver, manager, fhir_version }
    }

    /// Build complete type graph from installed packages
    ///
    /// # Returns
    ///
    /// Complete TypeGraph with all resources, datatypes, and primitives
    pub async fn build(&self) -> Result<TypeGraph> {
        info!("Building type graph for {}", self.fhir_version);

        let mut graph = TypeGraph::new(self.fhir_version);

        // Get all StructureDefinitions
        let structure_defs = self.load_all_structure_definitions().await?;
        info!("Loaded {} StructureDefinitions", structure_defs.len());

        // Categorize by kind
        let (resources, datatypes, primitives) =
            self.categorize_structures(&structure_defs).await?;

        info!(
            "Categorized: {} resources, {} datatypes, {} primitives",
            resources.len(),
            datatypes.len(),
            primitives.len()
        );

        // Process primitives first (no dependencies)
        for (name, sd_json) in primitives {
            if let Ok(primitive) = self.process_primitive(&sd_json).await {
                graph.add_primitive(name, primitive);
            }
        }

        // Process datatypes (may depend on primitives and other datatypes)
        for (name, sd_json) in datatypes {
            if let Ok(datatype) = self.process_datatype(&sd_json).await {
                graph.add_datatype(name, datatype);
            }
        }

        // Process resources (may depend on primitives and datatypes)
        for (name, sd_json) in resources {
            if let Ok(resource) = self.process_resource(&sd_json).await {
                graph.add_resource(name, resource);
            }
        }

        // Add search parameters
        self.add_search_parameters(&mut graph).await?;

        // Update metadata
        graph.metadata.source_packages = self.get_installed_packages().await?;

        info!("Type graph built successfully: {} total types", graph.total_types());

        Ok(graph)
    }

    /// Load all StructureDefinitions from canonical manager
    async fn load_all_structure_definitions(&self) -> Result<Vec<Value>> {
        let version_str = match self.fhir_version {
            FhirVersion::R4 => "R4",
            FhirVersion::R4B => "R4B",
            FhirVersion::R5 => "R5",
            FhirVersion::R6 => "R6",
        };

        self.resolver.get_all_structure_definitions(version_str).await
    }

    /// Categorize StructureDefinitions by kind
    pub async fn categorize_structures(
        &self,
        structure_defs: &[Value],
    ) -> Result<(HashMap<String, Value>, HashMap<String, Value>, HashMap<String, Value>)> {
        let mut resources = HashMap::new();
        let mut datatypes = HashMap::new();
        let mut primitives = HashMap::new();

        for sd in structure_defs {
            let name = sd.get("name").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();

            let kind = sd.get("kind").and_then(|v| v.as_str());

            match kind {
                Some("resource") => {
                    resources.insert(name, sd.clone());
                }
                Some("complex-type") => {
                    datatypes.insert(name, sd.clone());
                }
                Some("primitive-type") => {
                    primitives.insert(name, sd.clone());
                }
                _ => {
                    warn!("Unknown or missing kind for {}", name);
                }
            }
        }

        Ok((resources, datatypes, primitives))
    }

    /// Process a primitive type
    async fn process_primitive(&self, sd_json: &Value) -> Result<PrimitiveType> {
        let mut parser = self.parser.lock().await;
        let parsed = parser.parse(sd_json)?;

        if parsed.kind != StructureKind::PrimitiveType {
            return Err(Error::Parser(format!("{} is not a primitive type", parsed.name)));
        }

        // Extract pattern from elements if available
        let pattern =
            parsed.elements.iter().find(|e| e.path == format!("{}.value", parsed.name)).and(
                // Try to get pattern from element
                // This is a simplified version - real implementation would parse constraints
                None,
            );

        Ok(PrimitiveType {
            name: parsed.name.clone(),
            base: parsed.base_definition.clone(),
            pattern,
            documentation: Documentation {
                short: format!("FHIR primitive type {}", parsed.name),
                definition: String::new(),
                url: Some(parsed.url.clone()),
                ..Default::default()
            },
            url: parsed.url,
        })
    }

    /// Process a complex datatype
    async fn process_datatype(&self, sd_json: &Value) -> Result<DataType> {
        let mut parser = self.parser.lock().await;
        let parsed = parser.parse(sd_json)?;
        parser.to_datatype(&parsed)
    }

    /// Process a resource type
    async fn process_resource(&self, sd_json: &Value) -> Result<ResourceType> {
        let mut parser = self.parser.lock().await;
        let parsed = parser.parse(sd_json)?;
        parser.to_resource_type(&parsed)
    }

    /// Add search parameters to resources
    async fn add_search_parameters(&self, graph: &mut TypeGraph) -> Result<()> {
        debug!("Loading search parameters");

        // Get all SearchParameter resources
        let search_params =
            self.resolver.get_resources_by_type("SearchParameter", Some(1000)).await?;

        debug!("Found {} search parameters", search_params.len());

        // Group by resource type
        let mut params_by_resource: HashMap<String, Vec<SearchParameter>> = HashMap::new();

        for sp_resource in search_params {
            let content = &sp_resource.resource.content;

            // Extract base resource types
            let bases = content
                .get("base")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>()
                })
                .unwrap_or_default();

            // Create SearchParameter
            if let Ok(param) = self.parse_search_parameter(content) {
                // Add to each base resource
                for base in bases {
                    params_by_resource.entry(base).or_default().push(param.clone());
                }
            }
        }

        // Add to resources in graph
        for (name, resource) in &mut graph.resources {
            if let Some(params) = params_by_resource.get(name) {
                resource.search_parameters = params.clone();
                debug!("Added {} search parameters to {}", params.len(), name);
            }
        }

        Ok(())
    }

    /// Parse a SearchParameter resource
    fn parse_search_parameter(&self, content: &Value) -> Result<SearchParameter> {
        let code = content
            .get("code")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Parser("SearchParameter missing code".to_string()))?
            .to_string();

        let param_type = content
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Parser("SearchParameter missing type".to_string()))?;

        let param_type = match param_type {
            "number" => SearchParamType::Number,
            "date" => SearchParamType::Date,
            "string" => SearchParamType::String,
            "token" => SearchParamType::Token,
            "reference" => SearchParamType::Reference,
            "composite" => SearchParamType::Composite,
            "quantity" => SearchParamType::Quantity,
            "uri" => SearchParamType::Uri,
            "special" => SearchParamType::Special,
            _ => {
                return Err(Error::Parser(format!(
                    "Unknown search parameter type: {}",
                    param_type
                )));
            }
        };

        let description =
            content.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();

        let expression = content.get("expression").and_then(|v| v.as_str()).map(String::from);

        let target_types = if param_type == SearchParamType::Reference {
            content
                .get("target")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default()
        } else {
            vec![]
        };

        Ok(SearchParameter { code, param_type, description, expression, target_types })
    }

    /// Get list of installed packages
    async fn get_installed_packages(&self) -> Result<Vec<String>> {
        self.manager
            .list_packages()
            .await
            .map_err(|e| Error::Other(format!("Failed to list packages: {}", e)))
    }

    /// Build graph from specific package
    ///
    /// # Arguments
    ///
    /// * `package_name` - Name of the package (e.g., "hl7.fhir.r4.core")
    /// * `package_version` - Version of the package
    pub async fn build_from_package(
        &self,
        package_name: &str,
        package_version: &str,
    ) -> Result<TypeGraph> {
        info!("Building type graph from {}@{}", package_name, package_version);

        // Ensure package is installed
        self.manager
            .install_package(package_name, package_version)
            .await
            .map_err(|e| Error::Other(format!("Failed to install package: {}", e)))?;

        // Build graph from all installed StructureDefinitions
        self.build().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use octofhir_canonical_manager::FcmConfig;
    use tempfile::TempDir;

    async fn create_test_builder() -> TypeGraphBuilder {
        let temp_dir = TempDir::new().unwrap();
        let mut config = FcmConfig::default();
        config.storage.packages_dir = temp_dir.path().join("packages");
        config.storage.cache_dir = temp_dir.path().join("cache");
        config.storage.index_dir = temp_dir.path().join("index");
        let manager = Arc::new(CanonicalManager::new(config).await.unwrap());
        TypeGraphBuilder::new(manager, FhirVersion::R4)
    }

    #[tokio::test]
    async fn test_builder_creation() {
        let _builder = create_test_builder().await;
        // Just verify it creates successfully
    }

    #[tokio::test]
    async fn test_categorize_empty() {
        let builder = create_test_builder().await;
        let (resources, datatypes, primitives) = builder.categorize_structures(&[]).await.unwrap();

        assert_eq!(resources.len(), 0);
        assert_eq!(datatypes.len(), 0);
        assert_eq!(primitives.len(), 0);
    }

    #[tokio::test]
    async fn test_categorize_mixed() {
        let builder = create_test_builder().await;

        let structures = vec![
            serde_json::json!({
                "name": "Patient",
                "kind": "resource"
            }),
            serde_json::json!({
                "name": "HumanName",
                "kind": "complex-type"
            }),
            serde_json::json!({
                "name": "string",
                "kind": "primitive-type"
            }),
        ];

        let (resources, datatypes, primitives) =
            builder.categorize_structures(&structures).await.unwrap();

        assert_eq!(resources.len(), 1);
        assert!(resources.contains_key("Patient"));

        assert_eq!(datatypes.len(), 1);
        assert!(datatypes.contains_key("HumanName"));

        assert_eq!(primitives.len(), 1);
        assert!(primitives.contains_key("string"));
    }

    #[tokio::test]
    async fn test_build_empty_graph() {
        let builder = create_test_builder().await;

        // Building with no packages should return empty graph
        let graph = builder.build().await.unwrap();

        assert_eq!(graph.resources.len(), 0);
        assert_eq!(graph.datatypes.len(), 0);
        assert_eq!(graph.primitives.len(), 0);
        assert_eq!(graph.fhir_version, FhirVersion::R4);
    }
}
