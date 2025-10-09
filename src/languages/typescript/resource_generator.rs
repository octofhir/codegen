//! TypeScript resource generation (interfaces or classes)

use crate::core::Result;
use crate::core::ir::{ResourceType, TypeGraph};
use crate::generator::LanguageBackend;
use crate::languages::typescript::backend::TypeScriptBackend;
use crate::languages::typescript::class_generator::ClassGenerator;
use crate::languages::typescript::templates::TypeScriptTemplates;
use crate::templates::genco_engine::{GencoTemplateEngine, helpers};
use genco::prelude::*;
use std::collections::HashSet;

/// Generator for TypeScript resource interfaces or classes
pub struct ResourceGenerator<B: LanguageBackend> {
    /// Language backend for type mapping
    backend: B,
    /// Generate classes instead of interfaces
    use_classes: bool,
}

impl<B: LanguageBackend> ResourceGenerator<B> {
    /// Create a new resource generator
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            use_classes: false, // Default to interfaces for backward compatibility
        }
    }

    /// Create a new resource generator with class generation enabled
    pub fn new_with_classes(backend: B) -> Self {
        Self { backend, use_classes: true }
    }

    /// Generate TypeScript code for a single resource (interface or class)
    pub fn generate_resource(&self, resource: &ResourceType) -> Result<String> {
        if self.use_classes {
            ClassGenerator::generate_resource_class(resource, &self.backend)
        } else {
            TypeScriptTemplates::resource_to_interface(resource, &self.backend)
        }
    }

    /// Generate TypeScript interfaces for all resources in a type graph
    pub fn generate_all_resources(&self, graph: &TypeGraph) -> Result<Vec<(String, String)>> {
        let mut results = Vec::new();

        for (name, resource) in &graph.resources {
            let interface = self.generate_resource(resource)?;
            results.push((name.clone(), interface));
        }

        Ok(results)
    }

    /// Generate a type guard function for a resource
    ///
    /// Creates a function like:
    /// ```typescript
    /// export function isPatient(resource: Resource): resource is Patient {
    ///   return resource.resourceType === "Patient";
    /// }
    /// ```
    pub fn generate_type_guard(&self, resource_name: &str) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // Sanitize name for valid TypeScript identifier
        let sanitized_name = TypeScriptBackend::sanitize_identifier(resource_name);

        // JSDoc comment
        let doc = vec![
            format!("Type guard to check if a resource is a {}", resource_name),
            String::new(),
            "@param resource - The resource to check".to_string(),
            format!("@returns True if the resource is a {}", resource_name),
        ];

        tokens.append(helpers::jsdoc_comment(&doc));
        tokens.push();

        // Function signature
        tokens.append("export function is");
        tokens.append(&sanitized_name);
        tokens.append("(resource: Resource): resource is ");
        tokens.append(&sanitized_name);
        tokens.append(" {");
        tokens.push();

        // Function body (use original name for runtime check)
        tokens.append("  return resource.resourceType === \"");
        tokens.append(resource_name);
        tokens.append("\";");
        tokens.push();

        tokens.append("}");

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate all type guards for resources in a type graph
    pub fn generate_all_type_guards(&self, graph: &TypeGraph) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // Header comment
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Type guard functions for FHIR resources");
        tokens.push();
        tokens.append(" * ");
        tokens.push();
        tokens.append(" * These functions allow you to narrow resource types at runtime.");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.push();

        // Import Resource base type
        tokens.append("import { Resource } from './Resource';");
        tokens.push();

        // Import all resource types
        let resource_names: Vec<String> = graph.resources.keys().cloned().collect();
        if !resource_names.is_empty() {
            tokens.append("import { ");
            tokens.append(resource_names.join(", "));
            tokens.append(" } from './types';");
            tokens.push();
            tokens.push();
        }

        // Generate each type guard
        for (i, resource_name) in resource_names.iter().enumerate() {
            if i > 0 {
                tokens.push();
            }

            let guard = self.generate_type_guard(resource_name)?;
            tokens.append(&guard);
        }

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate a resource type union
    ///
    /// Creates a type like:
    /// ```typescript
    /// export type AnyResource = Patient | Observation | Practitioner | ...;
    /// ```
    pub fn generate_resource_union(&self, graph: &TypeGraph) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // JSDoc comment
        let doc = vec![
            "Union type of all FHIR resources".to_string(),
            String::new(),
            "This type represents any valid FHIR resource.".to_string(),
        ];

        tokens.append(helpers::jsdoc_comment(&doc));
        tokens.push();

        // Type alias
        tokens.append("export type AnyResource = ");

        let resource_names: Vec<String> = graph.resources.keys().cloned().collect();
        if !resource_names.is_empty() {
            tokens.append(helpers::union_type(&resource_names));
        } else {
            tokens.append("never");
        }

        tokens.append(";");

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Collect all dependencies for a resource
    ///
    /// Returns the set of complex types that this resource depends on
    pub fn collect_dependencies(&self, resource: &ResourceType) -> HashSet<String> {
        let mut deps = HashSet::new();

        for property in &resource.properties {
            // Collect types based on property type variant
            match &property.property_type {
                crate::core::ir::PropertyType::Primitive { type_name } => {
                    if !Self::is_primitive(type_name) {
                        deps.insert(type_name.to_string());
                    }
                }
                crate::core::ir::PropertyType::Complex { type_name } => {
                    deps.insert(type_name.to_string());
                }
                crate::core::ir::PropertyType::Reference { .. } => {
                    // Add Reference itself
                    deps.insert("Reference".to_string());
                    // Note: We don't add target types as dependencies because we use
                    // string literal unions (Reference<"Patient" | "Group">), not actual types
                }
                crate::core::ir::PropertyType::Choice { types } => {
                    // Add all choice types
                    for choice_type in types {
                        if !Self::is_primitive(choice_type) {
                            deps.insert(choice_type.to_string());
                        }
                    }
                }
                crate::core::ir::PropertyType::BackboneElement { .. } => {
                    deps.insert("BackboneElement".to_string());
                }
            }
        }

        // Add base type if present
        if let Some(base) = &resource.base {
            deps.insert(base.clone());
        }

        deps
    }

    /// Check if a type is a FHIR primitive
    fn is_primitive(type_name: &str) -> bool {
        matches!(
            type_name,
            "boolean"
                | "integer"
                | "integer64"
                | "decimal"
                | "string"
                | "code"
                | "id"
                | "markdown"
                | "uri"
                | "url"
                | "canonical"
                | "oid"
                | "uuid"
                | "date"
                | "dateTime"
                | "instant"
                | "time"
                | "base64Binary"
                | "xhtml"
                | "positiveInt"
                | "unsignedInt"
        )
    }

    /// Generate a complete resource file with imports
    pub fn generate_resource_file(
        &self,
        resource: &ResourceType,
        dependencies: &HashSet<String>,
    ) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // Generate imports (use relative path from resources/ to types/)
        if !dependencies.is_empty() {
            let mut deps_vec: Vec<String> = dependencies.iter().cloned().collect();
            deps_vec.sort();

            tokens.append(helpers::imports(&[(deps_vec, "../types".to_string())]));
            tokens.push();
        }

        // Generate the resource interface
        let interface = self.generate_resource(resource)?;
        tokens.append(&interface);
        tokens.push();

        // Generate type guard for this resource
        tokens.push();
        let guard = self.generate_type_guard(&resource.name)?;
        tokens.append(&guard);

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate resource constants (resourceType literals)
    pub fn generate_resource_constants(&self, graph: &TypeGraph) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // Header comment
        tokens.append("/**");
        tokens.push();
        tokens.append(" * FHIR Resource Type Constants");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.push();

        // Generate constants for each resource
        let mut resource_names: Vec<String> = graph.resources.keys().cloned().collect();
        resource_names.sort();

        for resource_name in &resource_names {
            tokens.append("export const RESOURCE_TYPE_");
            tokens.append(resource_name.to_uppercase());
            tokens.append(" = \"");
            tokens.append(resource_name);
            tokens.append("\" as const;");
            tokens.push();
        }

        tokens.push();

        // Generate array of all resource types
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Array of all FHIR resource types");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.append("export const ALL_RESOURCE_TYPES = [");
        tokens.push();

        for resource_name in &resource_names {
            tokens.append("  \"");
            tokens.append(resource_name);
            tokens.append("\",");
            tokens.push();
        }

        tokens.append("] as const;");

        GencoTemplateEngine::format_typescript(&tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ir::*;
    use crate::languages::typescript::TypeScriptBackend;

    fn create_test_resource() -> ResourceType {
        ResourceType {
            name: "Patient".to_string(),
            base: Some("DomainResource".to_string()),
            properties: vec![
                Property {
                    name: "active".to_string(),
                    path: "Patient.active".to_string(),
                    property_type: PropertyType::Primitive { type_name: "boolean".to_string() },
                    cardinality: CardinalityRange::optional(),
                    is_choice: false,
                    choice_types: vec![],
                    is_modifier: false,
                    is_summary: true,
                    binding: None,
                    constraints: vec![],
                    short_description: "Whether this patient record is in active use".to_string(),
                    definition: "Whether this patient's record is in active use".to_string(),
                    comments: None,
                    examples: vec![],
                },
                Property {
                    name: "name".to_string(),
                    path: "Patient.name".to_string(),
                    property_type: PropertyType::Complex { type_name: "HumanName".to_string() },
                    cardinality: CardinalityRange::optional_array(),
                    is_choice: false,
                    choice_types: vec![],
                    is_modifier: false,
                    is_summary: true,
                    binding: None,
                    constraints: vec![],
                    short_description: "A name associated with the patient".to_string(),
                    definition: "A name associated with the patient".to_string(),
                    comments: None,
                    examples: vec![],
                },
            ],
            search_parameters: vec![],
            extensions: vec![],
            documentation: Documentation {
                short: "Information about an individual or animal".to_string(),
                definition: "Demographics and administrative information".to_string(),
                ..Default::default()
            },
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            is_abstract: false,
        }
    }

    #[test]
    fn test_generate_resource() {
        let backend = TypeScriptBackend::new();
        let generator = ResourceGenerator::new(backend);
        let resource = create_test_resource();

        let result = generator.generate_resource(&resource);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("export interface Patient extends DomainResource"));
        assert!(output.contains("resourceType: \"Patient\""));
        assert!(output.contains("active?: boolean"));
        assert!(output.contains("name?: HumanName[]"));
    }

    #[test]
    fn test_generate_type_guard() {
        let backend = TypeScriptBackend::new();
        let generator = ResourceGenerator::new(backend);

        let result = generator.generate_type_guard("Patient");
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("export function isPatient"));
        assert!(output.contains("resource is Patient"));
        assert!(output.contains("resourceType === \"Patient\""));
    }

    #[test]
    fn test_collect_dependencies() {
        let backend = TypeScriptBackend::new();
        let generator = ResourceGenerator::new(backend);
        let resource = create_test_resource();

        let deps = generator.collect_dependencies(&resource);

        assert!(deps.contains("HumanName"));
        assert!(deps.contains("DomainResource"));
        assert!(!deps.contains("boolean")); // Primitive should not be included
    }

    #[test]
    fn test_is_primitive() {
        assert!(ResourceGenerator::<TypeScriptBackend>::is_primitive("boolean"));
        assert!(ResourceGenerator::<TypeScriptBackend>::is_primitive("string"));
        assert!(ResourceGenerator::<TypeScriptBackend>::is_primitive("dateTime"));
        assert!(!ResourceGenerator::<TypeScriptBackend>::is_primitive("HumanName"));
        assert!(!ResourceGenerator::<TypeScriptBackend>::is_primitive("Patient"));
    }

    #[test]
    fn test_generate_resource_union() {
        let backend = TypeScriptBackend::new();
        let generator = ResourceGenerator::new(backend);

        let mut graph = TypeGraph::new(FhirVersion::R4);
        graph.add_resource("Patient".to_string(), create_test_resource());

        let result = generator.generate_resource_union(&graph);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("export type AnyResource"));
        assert!(output.contains("Patient"));
    }

    #[test]
    fn test_generate_resource_file() {
        let backend = TypeScriptBackend::new();
        let generator = ResourceGenerator::new(backend);
        let resource = create_test_resource();

        let mut deps = HashSet::new();
        deps.insert("HumanName".to_string());
        deps.insert("DomainResource".to_string());

        let result = generator.generate_resource_file(&resource, &deps);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("import"));
        assert!(output.contains("HumanName"));
        assert!(output.contains("export interface Patient"));
        assert!(output.contains("export function isPatient"));
    }
}
