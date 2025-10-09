use crate::core::Result;
use crate::core::ir::TypeGraph;
use crate::languages::typescript::{
    DatatypeGenerator, HelpersGenerator, ManifestGenerator, PackageConfig, ResourceGenerator,
    TypeScriptBackend, ValidationGenerator,
};
use std::collections::HashMap;

/// Complete TypeScript SDK generator that orchestrates all component generators
pub struct TypeScriptSdkGenerator {
    #[allow(dead_code)]
    backend: TypeScriptBackend,
    #[allow(dead_code)]
    resource_generator: ResourceGenerator<TypeScriptBackend>,
    #[allow(dead_code)]
    datatype_generator: DatatypeGenerator<TypeScriptBackend>,
    validation_generator: ValidationGenerator,
    helpers_generator: HelpersGenerator,
    manifest_generator: ManifestGenerator,
}

/// Generated file content
#[derive(Debug, Clone)]
pub struct GeneratedFile {
    /// Relative path from SDK root
    pub path: String,
    /// File content
    pub content: String,
}

impl TypeScriptSdkGenerator {
    /// Create a new SDK generator with custom package config
    pub fn new(config: PackageConfig) -> Self {
        let backend = TypeScriptBackend::new();
        Self {
            resource_generator: ResourceGenerator::new(backend.clone()),
            datatype_generator: DatatypeGenerator::new(backend.clone()),
            validation_generator: ValidationGenerator::new(backend.clone()),
            helpers_generator: HelpersGenerator::new(backend.clone()),
            manifest_generator: ManifestGenerator::new(backend.clone(), config),
            backend,
        }
    }

    /// Create a new SDK generator with class generation enabled
    pub fn new_with_classes(config: PackageConfig) -> Self {
        let backend = TypeScriptBackend::new();
        Self {
            resource_generator: ResourceGenerator::new_with_classes(backend.clone()),
            datatype_generator: DatatypeGenerator::new(backend.clone()),
            validation_generator: ValidationGenerator::new(backend.clone()),
            helpers_generator: HelpersGenerator::new(backend.clone()),
            manifest_generator: ManifestGenerator::new(backend.clone(), config),
            backend,
        }
    }

    /// Create SDK generator with default configuration
    pub fn with_defaults() -> Self {
        Self::new(PackageConfig::default())
    }

    /// Create SDK generator with default configuration and class generation
    pub fn with_defaults_and_classes() -> Self {
        Self::new_with_classes(PackageConfig::default())
    }

    /// Generate a complete TypeScript SDK from a type graph
    ///
    /// Returns a map of file paths to their content
    pub fn generate_sdk(&self, type_graph: &TypeGraph) -> Result<HashMap<String, String>> {
        let mut files = HashMap::new();

        // Generate package.json
        files.insert("package.json".to_string(), self.manifest_generator.generate_package_json()?);

        // Generate tsconfig.json
        files
            .insert("tsconfig.json".to_string(), self.manifest_generator.generate_tsconfig_json()?);

        // Generate README.md
        files.insert("README.md".to_string(), self.manifest_generator.generate_readme()?);

        // Generate .gitignore
        files.insert(".gitignore".to_string(), self.manifest_generator.generate_gitignore()?);

        // Generate .npmignore
        files.insert(".npmignore".to_string(), self.manifest_generator.generate_npmignore()?);

        // Generate biome.json
        files.insert("biome.json".to_string(), self.manifest_generator.generate_biome_json()?);

        // Generate primitive types
        files.insert(
            "src/primitives.ts".to_string(),
            self.datatype_generator.generate_primitive_types(type_graph)?,
        );

        // Generate datatypes
        for (name, datatype) in &type_graph.datatypes {
            let dependencies = self.datatype_generator.collect_dependencies(datatype);
            let file_content =
                self.datatype_generator.generate_datatype_file(datatype, &dependencies)?;
            files.insert(format!("src/types/{}.ts", name), file_content);
        }

        // Generate resources
        for (name, resource) in &type_graph.resources {
            let dependencies = self.resource_generator.collect_dependencies(resource);
            let file_content =
                self.resource_generator.generate_resource_file(resource, &dependencies)?;
            files.insert(format!("src/resources/{}.ts", name), file_content);
        }

        // Generate validation types
        files.insert(
            "src/validation.ts".to_string(),
            self.validation_generator.generate_validation_types_module()?,
        );

        // Generate utility functions
        files.insert(
            "src/utilities.ts".to_string(),
            self.helpers_generator.generate_utilities_module()?,
        );

        // Generate main index
        files.insert("src/index.ts".to_string(), self.generate_main_index(type_graph)?);

        Ok(files)
    }

    /// Generate the main index.ts file
    fn generate_main_index(&self, type_graph: &TypeGraph) -> Result<String> {
        let mut exports = vec![
            "// Auto-generated TypeScript SDK index".to_string(),
            "// This file is auto-generated. Do not edit manually.".to_string(),
            "".to_string(),
            "// Primitive types".to_string(),
            "export * from './primitives';".to_string(),
            "".to_string(),
            "// Validation types and functions".to_string(),
            "export * from './validation';".to_string(),
            "".to_string(),
            "// Utility functions".to_string(),
            "export * from './utilities';".to_string(),
            "".to_string(),
        ];

        // Export datatypes
        if !type_graph.datatypes.is_empty() {
            exports.push("// Datatypes".to_string());
            for name in type_graph.datatypes.keys() {
                exports.push(format!("export * from './types/{}';", name));
            }
            exports.push("".to_string());
        }

        // Export resources
        if !type_graph.resources.is_empty() {
            exports.push("// Resources".to_string());
            for name in type_graph.resources.keys() {
                exports.push(format!("export * from './resources/{}';", name));
            }
            exports.push("".to_string());
        }

        Ok(exports.join("\n"))
    }

    /// Write generated SDK to filesystem
    #[allow(dead_code)]
    pub fn write_sdk(&self, type_graph: &TypeGraph, output_dir: &std::path::Path) -> Result<()> {
        use std::fs;

        let files = self.generate_sdk(type_graph)?;

        for (path, content) in files {
            let file_path = output_dir.join(&path);

            // Create parent directories if needed
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    crate::core::Error::Io(std::io::Error::other(format!(
                        "Failed to create directory {}: {}",
                        parent.display(),
                        e
                    )))
                })?;
            }

            // Write file
            fs::write(&file_path, content).map_err(|e| {
                crate::core::Error::Io(std::io::Error::other(format!(
                    "Failed to write file {}: {}",
                    file_path.display(),
                    e
                )))
            })?;
        }

        Ok(())
    }

    /// Get a list of all files that would be generated
    pub fn list_generated_files(&self, type_graph: &TypeGraph) -> Result<Vec<String>> {
        let files = self.generate_sdk(type_graph)?;
        Ok(files.keys().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ir::{FhirVersion, GraphMetadata};

    fn create_test_graph() -> TypeGraph {
        use indexmap::IndexMap;
        use std::collections::HashMap;

        TypeGraph {
            resources: IndexMap::new(),
            datatypes: IndexMap::new(),
            primitives: IndexMap::new(),
            profiles: IndexMap::new(),
            fhir_version: FhirVersion::R4,
            metadata: GraphMetadata {
                generated_at: chrono::Utc::now().to_rfc3339(),
                generator_version: "0.1.0".to_string(),
                source_packages: vec!["test".to_string()],
                custom: HashMap::new(),
            },
        }
    }

    #[test]
    fn test_create_sdk_generator() {
        let _generator = TypeScriptSdkGenerator::with_defaults();
        // Just verify it constructs without panicking
    }

    #[test]
    fn test_generate_sdk_basic_files() {
        let generator = TypeScriptSdkGenerator::with_defaults();
        let graph = create_test_graph();

        let files = generator.generate_sdk(&graph).unwrap();

        // Verify essential files are generated
        assert!(files.contains_key("package.json"));
        assert!(files.contains_key("tsconfig.json"));
        assert!(files.contains_key("README.md"));
        assert!(files.contains_key(".gitignore"));
        assert!(files.contains_key(".npmignore"));
        assert!(files.contains_key("biome.json"));
        assert!(files.contains_key("src/index.ts"));
        assert!(files.contains_key("src/validation.ts"));
        assert!(files.contains_key("src/utilities.ts"));
    }

    #[test]
    fn test_list_generated_files() {
        let generator = TypeScriptSdkGenerator::with_defaults();
        let graph = create_test_graph();

        let file_list = generator.list_generated_files(&graph).unwrap();

        assert!(file_list.len() >= 9); // At least the basic files
        assert!(file_list.contains(&"package.json".to_string()));
        assert!(file_list.contains(&"tsconfig.json".to_string()));
    }

    #[test]
    fn test_package_json_is_valid() {
        let generator = TypeScriptSdkGenerator::with_defaults();
        let graph = create_test_graph();

        let files = generator.generate_sdk(&graph).unwrap();
        let package_json = files.get("package.json").unwrap();

        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(package_json).unwrap();
        assert_eq!(parsed["name"], "@octofhir/fhir-r4");
        assert_eq!(parsed["type"], "module");
    }

    #[test]
    fn test_tsconfig_is_valid() {
        let generator = TypeScriptSdkGenerator::with_defaults();
        let graph = create_test_graph();

        let files = generator.generate_sdk(&graph).unwrap();
        let tsconfig = files.get("tsconfig.json").unwrap();

        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(tsconfig).unwrap();
        assert_eq!(parsed["compilerOptions"]["strict"], true);
        assert_eq!(parsed["compilerOptions"]["target"], "ES2022");
    }

    #[test]
    fn test_biome_config_is_valid() {
        let generator = TypeScriptSdkGenerator::with_defaults();
        let graph = create_test_graph();

        let files = generator.generate_sdk(&graph).unwrap();
        let biome = files.get("biome.json").unwrap();

        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(biome).unwrap();
        assert_eq!(parsed["linter"]["enabled"], true);
        assert_eq!(parsed["formatter"]["enabled"], true);
    }

    #[test]
    fn test_index_exports_validation() {
        let generator = TypeScriptSdkGenerator::with_defaults();
        let graph = create_test_graph();

        let files = generator.generate_sdk(&graph).unwrap();
        let index = files.get("src/index.ts").unwrap();

        assert!(index.contains("export * from './validation'"));
        assert!(index.contains("export * from './utilities'"));
    }

    #[test]
    fn test_custom_package_config() {
        let config = PackageConfig {
            name: "@test/custom-fhir".to_string(),
            version: "1.2.3".to_string(),
            description: "Custom test SDK".to_string(),
            fhir_version: "R5".to_string(),
            repository_url: None,
            license: "MIT".to_string(),
        };

        let generator = TypeScriptSdkGenerator::new(config);
        let graph = create_test_graph();

        let files = generator.generate_sdk(&graph).unwrap();
        let package_json = files.get("package.json").unwrap();

        let parsed: serde_json::Value = serde_json::from_str(package_json).unwrap();
        assert_eq!(parsed["name"], "@test/custom-fhir");
        assert_eq!(parsed["version"], "1.2.3");
        assert_eq!(parsed["license"], "MIT");
    }
}
