//! TypeScript datatype interface generation

use crate::core::Result;
use crate::core::ir::{DataType, TypeGraph};
use crate::generator::LanguageBackend;
use crate::languages::typescript::backend::TypeScriptBackend;
use crate::languages::typescript::class_generator::ClassGenerator;
use crate::languages::typescript::templates::TypeScriptTemplates;
use crate::templates::genco_engine::{GencoTemplateEngine, helpers};
use genco::prelude::*;
use std::collections::HashSet;

/// Generator for TypeScript datatype interfaces
pub struct DatatypeGenerator<B: LanguageBackend> {
    /// Language backend for type mapping
    backend: B,
    /// Whether to generate classes instead of interfaces
    use_classes: bool,
}

impl<B: LanguageBackend> DatatypeGenerator<B> {
    /// Create a new datatype generator
    pub fn new(backend: B) -> Self {
        Self { backend, use_classes: false }
    }

    /// Create a new datatype generator with class generation enabled
    pub fn new_with_classes(backend: B) -> Self {
        Self { backend, use_classes: true }
    }

    /// Generate TypeScript interface or class for a complex datatype
    pub fn generate_datatype(&self, datatype: &DataType) -> Result<String> {
        if self.use_classes {
            ClassGenerator::generate_datatype_class(datatype, &self.backend)
        } else {
            TypeScriptTemplates::datatype_to_interface(datatype, &self.backend)
        }
    }

    /// Generate TypeScript interfaces for all datatypes in a type graph
    pub fn generate_all_datatypes(&self, graph: &TypeGraph) -> Result<Vec<(String, String)>> {
        let mut results = Vec::new();

        for (name, datatype) in &graph.datatypes {
            let interface = self.generate_datatype(datatype)?;
            results.push((name.clone(), interface));
        }

        Ok(results)
    }

    /// Generate the FhirPrimitive wrapper type
    ///
    /// Creates a type like:
    /// ```typescript
    /// export interface FhirPrimitive<T> {
    ///   value?: T;
    ///   id?: string;
    ///   extension?: Extension[];
    /// }
    /// ```
    pub fn generate_primitive_wrapper() -> Result<String> {
        let mut tokens = js::Tokens::new();

        // JSDoc comment
        let doc = vec![
            "FHIR Primitive wrapper type".to_string(),
            String::new(),
            "Wraps primitive values to support FHIR extensions and element properties.".to_string(),
            String::new(),
            "@template T - The primitive value type".to_string(),
        ];

        tokens.append(helpers::jsdoc_comment(&doc));
        tokens.push();

        // Interface declaration
        tokens.append("export interface FhirPrimitive<T> {");
        tokens.push();

        // Properties
        tokens.append("  /** The actual primitive value */");
        tokens.push();
        tokens.append("  value?: T;");
        tokens.push();
        tokens.push();

        tokens.append("  /** Unique id for inter-element referencing */");
        tokens.push();
        tokens.append("  id?: string;");
        tokens.push();
        tokens.push();

        tokens.append("  /** Additional content defined by implementations */");
        tokens.push();
        tokens.append("  extension?: Extension[];");
        tokens.push();

        tokens.append("}");

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate primitive type aliases
    ///
    /// Creates types like:
    /// ```typescript
    /// export type fhir_boolean = FhirPrimitive<boolean>;
    /// export type fhir_string = FhirPrimitive<string>;
    /// ```
    pub fn generate_primitive_types(&self, graph: &TypeGraph) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // Header comment
        tokens.append("/**");
        tokens.push();
        tokens.append(" * FHIR Primitive Type Definitions");
        tokens.push();
        tokens.append(" * ");
        tokens.push();
        tokens
            .append(" * These types wrap native TypeScript primitives to support FHIR extensions.");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.push();

        // Import Extension for the FhirPrimitive type
        tokens.append("import { Extension } from './types/Extension';");
        tokens.push();
        tokens.push();

        // Generate FhirPrimitive wrapper
        let wrapper = Self::generate_primitive_wrapper()?;
        tokens.append(&wrapper);
        tokens.push();
        tokens.push();

        // Generate type aliases for each primitive
        let mut primitive_names: Vec<String> = graph.primitives.keys().cloned().collect();
        primitive_names.sort();

        for primitive_name in &primitive_names {
            if let Some(primitive) = graph.primitives.get(primitive_name) {
                let ts_type = self.map_primitive_to_ts(primitive_name);

                // Create a proper TypeScript type name (capitalize and prefix with Fhir to avoid shadowing)
                let type_name = TypeScriptBackend::sanitize_identifier(primitive_name);
                // For primitives that would shadow built-in types, prefix with Fhir
                let safe_type_name =
                    if matches!(type_name.as_str(), "string" | "boolean" | "number") {
                        format!(
                            "Fhir{}",
                            type_name.chars().next().unwrap().to_uppercase().to_string()
                                + &type_name[1..]
                        )
                    } else {
                        type_name
                    };

                // JSDoc for the type
                let doc = vec![primitive.documentation.short.clone()];

                tokens.append(helpers::jsdoc_comment(&doc));
                tokens.push();

                // Type alias
                tokens.append("export type ");
                tokens.append(&safe_type_name);
                tokens.append(" = FhirPrimitive<");
                tokens.append(&ts_type);
                tokens.append(">;");
                tokens.push();
                tokens.push();
            }
        }

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Map FHIR primitive name to TypeScript base type
    fn map_primitive_to_ts(&self, primitive_name: &str) -> String {
        match primitive_name {
            "boolean" => "boolean",
            "integer" | "positiveInt" | "unsignedInt" | "integer64" | "decimal" => "number",
            _ => "string", // Most FHIR primitives map to string
        }
        .to_string()
    }

    /// Generate a simple primitive type definition (without wrapper)
    ///
    /// For use when extensions are not needed:
    /// ```typescript
    /// export type boolean = boolean;
    /// export type string = string;
    /// ```
    pub fn generate_simple_primitive_types() -> Result<String> {
        let mut tokens = js::Tokens::new();

        // Header
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Simple FHIR primitive type aliases");
        tokens.push();
        tokens.append(" * ");
        tokens.push();
        tokens.append(" * Use these when you don't need extension support.");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.push();

        // Common primitives
        let primitives = vec![
            ("boolean", "boolean", "Boolean value"),
            ("integer", "number", "32-bit integer"),
            ("string", "string", "String value"),
            ("decimal", "number", "Decimal number"),
            ("uri", "string", "URI/URL"),
            ("url", "string", "URL"),
            ("canonical", "string", "Canonical URL"),
            ("code", "string", "Code value"),
            ("id", "string", "Element ID"),
            ("date", "string", "Date (YYYY-MM-DD)"),
            ("dateTime", "string", "DateTime"),
            ("instant", "string", "Instant in time"),
            ("time", "string", "Time"),
        ];

        for (fhir_type, ts_type, desc) in primitives {
            tokens.append("/** ");
            tokens.append(desc);
            tokens.append(" */");
            tokens.push();
            tokens.append("export type ");
            tokens.append(fhir_type);
            tokens.append(" = ");
            tokens.append(ts_type);
            tokens.append(";");
            tokens.push();
            tokens.push();
        }

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Collect all dependencies for a datatype
    pub fn collect_dependencies(&self, datatype: &DataType) -> HashSet<String> {
        let mut deps = HashSet::new();

        for property in &datatype.properties {
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
        if let Some(base) = &datatype.base {
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

    /// Generate a complete datatype file with imports
    pub fn generate_datatype_file(
        &self,
        datatype: &DataType,
        dependencies: &HashSet<String>,
    ) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // Generate imports (datatypes in src/types/ import from same directory)
        if !dependencies.is_empty() {
            let mut deps_vec: Vec<String> = dependencies.iter().cloned().collect();
            deps_vec.sort();

            // Use '.' for same directory imports
            tokens.append(helpers::imports(&[(deps_vec, ".".to_string())]));
            tokens.push();
        }

        // Generate the datatype interface
        let interface = self.generate_datatype(datatype)?;
        tokens.append(&interface);

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate an index file that exports all datatypes
    pub fn generate_datatypes_index(&self, graph: &TypeGraph) -> Result<String> {
        let mut exports = Vec::new();

        // Collect all datatype names
        for name in graph.datatypes.keys() {
            exports.push(name.clone());
        }

        exports.sort();

        TypeScriptTemplates::generate_index(&exports)
    }

    /// Generate type guards for common datatypes
    pub fn generate_datatype_type_guards(&self, datatype_names: &[String]) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // Header
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Type guards for FHIR datatypes");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.push();

        // Import types
        if !datatype_names.is_empty() {
            tokens.append("import { ");
            tokens.append(datatype_names.join(", "));
            tokens.append(" } from './types';");
            tokens.push();
            tokens.push();
        }

        // Generate guards for datatypes that have a discriminator
        // For now, just document the concept
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Type guard for checking if an object is a valid FHIR datatype");
        tokens.push();
        tokens.append(" * ");
        tokens.push();
        tokens.append(" * Note: This is a basic implementation. In production, you would");
        tokens.push();
        tokens.append(" * want to validate all required properties.");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.append(
            "export function isValidDatatype(obj: unknown): obj is Record<string, unknown> {",
        );
        tokens.push();
        tokens.append("  return typeof obj === 'object' && obj !== null;");
        tokens.push();
        tokens.append("}");

        GencoTemplateEngine::format_typescript(&tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ir::*;
    use crate::languages::typescript::TypeScriptBackend;

    fn create_human_name_datatype() -> DataType {
        DataType {
            name: "HumanName".to_string(),
            base: Some("Element".to_string()),
            properties: vec![
                Property {
                    name: "use".to_string(),
                    path: "HumanName.use".to_string(),
                    property_type: PropertyType::Primitive { type_name: "code".to_string() },
                    cardinality: CardinalityRange::optional(),
                    is_choice: false,
                    choice_types: vec![],
                    is_modifier: false,
                    is_summary: true,
                    binding: Some(ValueSetBinding {
                        strength: BindingStrength::Required,
                        value_set: "http://hl7.org/fhir/ValueSet/name-use".to_string(),
                        description: Some("The use of a human name.".to_string()),
                    }),
                    constraints: vec![],
                    short_description:
                        "usual | official | temp | nickname | anonymous | old | maiden".to_string(),
                    definition: "Identifies the purpose for this name".to_string(),
                    comments: None,
                    examples: vec![],
                },
                Property {
                    name: "text".to_string(),
                    path: "HumanName.text".to_string(),
                    property_type: PropertyType::Primitive { type_name: "string".to_string() },
                    cardinality: CardinalityRange::optional(),
                    is_choice: false,
                    choice_types: vec![],
                    is_modifier: false,
                    is_summary: true,
                    binding: None,
                    constraints: vec![],
                    short_description: "Text representation of the full name".to_string(),
                    definition: "Specifies the entire name as it should be displayed".to_string(),
                    comments: None,
                    examples: vec![],
                },
                Property {
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
                    short_description: "Family name (often called 'Surname')".to_string(),
                    definition: "The part of a name that links to the genealogy".to_string(),
                    comments: None,
                    examples: vec![],
                },
                Property {
                    name: "given".to_string(),
                    path: "HumanName.given".to_string(),
                    property_type: PropertyType::Primitive { type_name: "string".to_string() },
                    cardinality: CardinalityRange::optional_array(),
                    is_choice: false,
                    choice_types: vec![],
                    is_modifier: false,
                    is_summary: true,
                    binding: None,
                    constraints: vec![],
                    short_description: "Given names (not always 'first')".to_string(),
                    definition: "Given name".to_string(),
                    comments: None,
                    examples: vec![],
                },
            ],
            documentation: Documentation {
                short: "Name of a human".to_string(),
                definition: "A human's name with the ability to identify parts and usage."
                    .to_string(),
                ..Default::default()
            },
            url: "http://hl7.org/fhir/StructureDefinition/HumanName".to_string(),
            is_abstract: false,
        }
    }

    #[test]
    fn test_generate_datatype() {
        let backend = TypeScriptBackend::new();
        let generator = DatatypeGenerator::new(backend);
        let datatype = create_human_name_datatype();

        let result = generator.generate_datatype(&datatype);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("export interface HumanName extends Element"));
        assert!(output.contains("use?: string"));
        assert!(output.contains("family?: string"));
        assert!(output.contains("given?: string[]"));
    }

    #[test]
    fn test_generate_primitive_wrapper() {
        let result = DatatypeGenerator::<TypeScriptBackend>::generate_primitive_wrapper();
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("export interface FhirPrimitive<T>"));
        assert!(output.contains("value?: T"));
        assert!(output.contains("extension?: Extension[]"));
    }

    #[test]
    fn test_generate_simple_primitive_types() {
        let result = DatatypeGenerator::<TypeScriptBackend>::generate_simple_primitive_types();
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("export type boolean = boolean"));
        assert!(output.contains("export type string = string"));
        assert!(output.contains("export type integer = number"));
    }

    #[test]
    fn test_collect_dependencies() {
        let backend = TypeScriptBackend::new();
        let generator = DatatypeGenerator::new(backend);
        let datatype = create_human_name_datatype();

        let deps = generator.collect_dependencies(&datatype);

        assert!(deps.contains("Element"));
        assert!(!deps.contains("string")); // Primitives should not be included
        assert!(!deps.contains("code"));
    }

    #[test]
    fn test_is_primitive() {
        assert!(DatatypeGenerator::<TypeScriptBackend>::is_primitive("boolean"));
        assert!(DatatypeGenerator::<TypeScriptBackend>::is_primitive("string"));
        assert!(DatatypeGenerator::<TypeScriptBackend>::is_primitive("dateTime"));
        assert!(!DatatypeGenerator::<TypeScriptBackend>::is_primitive("HumanName"));
        assert!(!DatatypeGenerator::<TypeScriptBackend>::is_primitive("Address"));
    }

    #[test]
    fn test_map_primitive_to_ts() {
        let backend = TypeScriptBackend::new();
        let generator = DatatypeGenerator::new(backend);

        assert_eq!(generator.map_primitive_to_ts("boolean"), "boolean");
        assert_eq!(generator.map_primitive_to_ts("integer"), "number");
        assert_eq!(generator.map_primitive_to_ts("decimal"), "number");
        assert_eq!(generator.map_primitive_to_ts("string"), "string");
        assert_eq!(generator.map_primitive_to_ts("date"), "string");
    }
}
