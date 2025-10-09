//! TypeScript-specific template utilities

use crate::core::Result;
use crate::core::ir::{DataType, Documentation, Property, ResourceType};
use crate::generator::LanguageBackend;
use crate::languages::typescript::backend::TypeScriptBackend;
use crate::templates::genco_engine::{GencoTemplateEngine, helpers};
use genco::prelude::*;

/// TypeScript template utilities
pub struct TypeScriptTemplates;

impl TypeScriptTemplates {
    /// Generate TypeScript interface from ResourceType
    pub fn resource_to_interface<B: LanguageBackend>(
        resource: &ResourceType,
        backend: &B,
    ) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // Sanitize resource name for valid TypeScript identifier
        let sanitized_name = TypeScriptBackend::sanitize_identifier(&resource.name);

        // Generate documentation
        let doc_lines = Self::format_documentation(&resource.documentation);

        // Generate properties
        let properties = Self::format_properties(&resource.properties, backend);

        // Add resourceType literal (keep original name for runtime value)
        let mut all_props =
            vec![("resourceType".to_string(), helpers::string_literal(&resource.name), false)];
        all_props.extend(properties);

        // Determine base type
        let extends = resource.base.as_deref();

        tokens.append(helpers::interface(&sanitized_name, extends, &all_props, Some(&doc_lines)));

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate TypeScript interface from DataType
    pub fn datatype_to_interface<B: LanguageBackend>(
        datatype: &DataType,
        backend: &B,
    ) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // Sanitize datatype name for valid TypeScript identifier
        let sanitized_name = TypeScriptBackend::sanitize_identifier(&datatype.name);

        // Generate documentation
        let doc_lines = Self::format_documentation(&datatype.documentation);

        // Generate properties
        let properties = Self::format_properties(&datatype.properties, backend);

        // Determine base type
        let extends = datatype.base.as_deref();

        tokens.append(helpers::interface(&sanitized_name, extends, &properties, Some(&doc_lines)));

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate a file with multiple interfaces
    pub fn generate_file<B: LanguageBackend>(
        interfaces: &[String],
        imports: &[(Vec<String>, String)],
    ) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // Add imports
        if !imports.is_empty() {
            tokens.append(helpers::imports(imports));
            tokens.push();
        }

        // Add each interface
        for (i, interface_code) in interfaces.iter().enumerate() {
            if i > 0 {
                tokens.push();
            }
            tokens.append(interface_code);
        }

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Format documentation into JSDoc comment lines
    fn format_documentation(doc: &Documentation) -> Vec<String> {
        let mut lines = Vec::new();

        if !doc.short.is_empty() {
            lines.push(doc.short.clone());
        }

        if !doc.definition.is_empty() && doc.definition != doc.short {
            if !lines.is_empty() {
                lines.push(String::new()); // Empty line
            }
            // Wrap long lines
            for line in Self::wrap_text(&doc.definition, 80) {
                lines.push(line);
            }
        }

        if let Some(comments) = &doc.comments {
            if !lines.is_empty() {
                lines.push(String::new());
            }
            for line in Self::wrap_text(comments, 80) {
                lines.push(line);
            }
        }

        if let Some(url) = &doc.url {
            if !lines.is_empty() {
                lines.push(String::new());
            }
            lines.push(format!("@see {}", url));
        }

        lines
    }

    /// Format properties for interface generation
    fn format_properties<B: LanguageBackend>(
        properties: &[Property],
        backend: &B,
    ) -> Vec<(String, String, bool)> {
        properties
            .iter()
            .map(|prop| {
                let name = prop.name.clone();
                let mut type_name = backend.map_type(&prop.property_type);

                // Handle arrays
                if prop.cardinality.is_array() {
                    type_name = helpers::array_type(&type_name);
                }

                let optional = prop.cardinality.is_optional();

                (name, type_name, optional)
            })
            .collect()
    }

    /// Wrap text to specified width
    fn wrap_text(text: &str, width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.len() + word.len() + 1 > width && !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
            }

            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }

    /// Generate index file that exports all types
    pub fn generate_index(exports: &[String]) -> Result<String> {
        let mut tokens = js::Tokens::new();

        for export in exports {
            tokens.append("export * from './");
            tokens.append(export);
            tokens.append("';");
            tokens.push();
        }

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate package.json manifest
    pub fn generate_package_json(
        name: &str,
        version: &str,
        description: &str,
    ) -> serde_json::Value {
        serde_json::json!({
            "name": name,
            "version": version,
            "description": description,
            "main": "index.js",
            "types": "index.d.ts",
            "scripts": {
                "build": "tsc",
                "test": "jest"
            },
            "keywords": ["fhir", "healthcare", "typescript"],
            "license": "MIT",
            "devDependencies": {
                "typescript": "^5.3.0",
                "@types/node": "^20.0.0"
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ir::*;
    use crate::languages::typescript::TypeScriptBackend;

    #[test]
    fn test_resource_to_interface() {
        let backend = TypeScriptBackend::new();

        let resource = ResourceType {
            name: "Patient".to_string(),
            base: Some("DomainResource".to_string()),
            properties: vec![Property {
                name: "id".to_string(),
                path: "Patient.id".to_string(),
                property_type: PropertyType::Primitive { type_name: "id".to_string() },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Logical id".to_string(),
                definition: "The logical id of the resource".to_string(),
                comments: None,
                examples: vec![],
            }],
            search_parameters: vec![],
            extensions: vec![],
            documentation: Documentation {
                short: "Patient resource".to_string(),
                definition: "Demographics and administrative information".to_string(),
                ..Default::default()
            },
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            is_abstract: false,
        };

        let result = TypeScriptTemplates::resource_to_interface(&resource, &backend);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("export interface Patient extends DomainResource"));
        assert!(output.contains("resourceType: \"Patient\""));
        assert!(output.contains("id?: string"));
    }

    #[test]
    fn test_datatype_to_interface() {
        let backend = TypeScriptBackend::new();

        let datatype = DataType {
            name: "HumanName".to_string(),
            base: Some("Element".to_string()),
            properties: vec![Property {
                name: "family".to_string(),
                path: "HumanName.family".to_string(),
                property_type: PropertyType::Primitive { type_name: "string".to_string() },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Family name".to_string(),
                definition: "The family name".to_string(),
                comments: None,
                examples: vec![],
            }],
            documentation: Documentation {
                short: "Human name".to_string(),
                definition: "Name of a human".to_string(),
                ..Default::default()
            },
            url: "http://hl7.org/fhir/StructureDefinition/HumanName".to_string(),
            is_abstract: false,
        };

        let result = TypeScriptTemplates::datatype_to_interface(&datatype, &backend);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("export interface HumanName extends Element"));
        assert!(output.contains("family?: string"));
    }

    #[test]
    fn test_generate_index() {
        let exports =
            vec!["Patient".to_string(), "Observation".to_string(), "HumanName".to_string()];

        let result = TypeScriptTemplates::generate_index(&exports);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("export * from './Patient'"));
        assert!(output.contains("export * from './Observation'"));
        assert!(output.contains("export * from './HumanName'"));
    }

    #[test]
    fn test_generate_package_json() {
        let manifest = TypeScriptTemplates::generate_package_json(
            "@octofhir/r4",
            "1.0.0",
            "FHIR R4 TypeScript SDK",
        );

        assert_eq!(manifest["name"], "@octofhir/r4");
        assert_eq!(manifest["version"], "1.0.0");
        assert!(manifest["devDependencies"]["typescript"].is_string());
    }

    #[test]
    fn test_wrap_text() {
        let text = "This is a very long line that should be wrapped at the specified width";
        let wrapped = TypeScriptTemplates::wrap_text(text, 30);

        assert!(wrapped.len() > 1);
        for line in wrapped {
            assert!(line.len() <= 30 || !line.contains(' '));
        }
    }

    #[test]
    fn test_format_properties_with_array() {
        let backend = TypeScriptBackend::new();

        let properties = vec![Property {
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
            short_description: "Names".to_string(),
            definition: "Person names".to_string(),
            comments: None,
            examples: vec![],
        }];

        let formatted = TypeScriptTemplates::format_properties(&properties, &backend);

        assert_eq!(formatted.len(), 1);
        assert_eq!(formatted[0].0, "name");
        assert_eq!(formatted[0].1, "HumanName[]");
        assert!(formatted[0].2); // optional
    }
}
