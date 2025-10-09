//! TypeScript language backend

use crate::Result;
use crate::core::ir::{Documentation, PropertyType};
use crate::generator::{IdentifierContext, LanguageBackend};
use heck::{ToLowerCamelCase, ToPascalCase};

/// TypeScript language backend
#[derive(Clone)]
pub struct TypeScriptBackend {
    /// TypeScript version target
    #[allow(dead_code)]
    target_version: String,
}

impl TypeScriptBackend {
    /// Create new TypeScript backend
    pub fn new() -> Self {
        Self { target_version: "5.9".to_string() }
    }

    /// Create with specific TypeScript version
    pub fn with_version(version: String) -> Self {
        Self { target_version: version }
    }

    /// Sanitize name to be a valid TypeScript identifier
    ///
    /// Handles hyphens, spaces, and other invalid characters
    pub fn sanitize_identifier(name: &str) -> String {
        // Replace hyphens and spaces with underscores for now
        // Later we could convert to PascalCase properly
        name.replace('-', "_").replace(' ', "_").replace('.', "_")
    }
}

impl Default for TypeScriptBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageBackend for TypeScriptBackend {
    fn map_type(&self, property_type: &PropertyType) -> String {
        match property_type {
            PropertyType::Primitive { type_name } => Self::map_primitive_type(type_name),
            PropertyType::Complex { type_name } => type_name.clone(),
            PropertyType::Reference { target_types } => {
                if target_types.is_empty() {
                    "Reference".to_string()
                } else if target_types.len() == 1 {
                    format!("Reference<\"{}\">", target_types[0])
                } else {
                    // Use string literal union types: Reference<"Patient" | "Group">
                    let types = target_types
                        .iter()
                        .map(|t| format!("\"{}\"", t))
                        .collect::<Vec<_>>()
                        .join(" | ");
                    format!("Reference<{}>", types)
                }
            }
            PropertyType::BackboneElement { .. } => "BackboneElement".to_string(),
            PropertyType::Choice { types } => {
                // Map each choice type (primitives to TS types, complex types as-is)
                let mapped: Vec<String> = types
                    .iter()
                    .map(|t| {
                        // Check if it's a primitive by trying to map it
                        let mapped = Self::map_primitive_type(t);
                        // If mapping didn't change it (not a primitive), use as-is
                        if mapped == *t { t.clone() } else { mapped }
                    })
                    .collect();
                mapped.join(" | ")
            }
        }
    }

    fn generate_imports(&self, dependencies: &[String]) -> Vec<String> {
        if dependencies.is_empty() {
            return vec![];
        }

        // Group imports by module
        let mut imports = vec![];

        // Assume all types are from same module for now
        let types = dependencies.join(", ");
        imports.push(format!("import {{ {} }} from './types';", types));

        imports
    }

    fn format_identifier(&self, name: &str, context: IdentifierContext) -> String {
        match context {
            IdentifierContext::TypeName => name.to_pascal_case(),
            IdentifierContext::FieldName => name.to_lower_camel_case(),
            IdentifierContext::FunctionName => name.to_lower_camel_case(),
            IdentifierContext::ConstantName => name.to_uppercase(),
            IdentifierContext::VariableName => name.to_lower_camel_case(),
        }
    }

    fn generate_doc_comment(&self, doc: &Documentation) -> Vec<String> {
        let mut lines = vec![];

        lines.push("/**".to_string());

        if !doc.short.is_empty() {
            lines.push(format!(" * {}", doc.short));
        }

        if !doc.definition.is_empty() && doc.definition != doc.short {
            lines.push(" *".to_string());
            // Wrap long definitions
            for line in Self::wrap_text(&doc.definition, 80) {
                lines.push(format!(" * {}", line));
            }
        }

        if let Some(comments) = &doc.comments {
            lines.push(" *".to_string());
            for line in Self::wrap_text(comments, 80) {
                lines.push(format!(" * {}", line));
            }
        }

        if let Some(url) = &doc.url {
            lines.push(" *".to_string());
            lines.push(format!(" * @see {}", url));
        }

        lines.push(" */".to_string());

        lines
    }

    fn file_extension(&self) -> &str {
        "ts"
    }

    fn format_code(&self, code: &str) -> Result<String> {
        // TODO: In future, integrate with prettier
        // For now, just return as-is
        Ok(code.to_string())
    }
}

impl TypeScriptBackend {
    /// Map FHIR primitive to TypeScript type
    fn map_primitive_type(fhir_type: &str) -> String {
        match fhir_type {
            "boolean" => "boolean",
            "integer" | "positiveInt" | "unsignedInt" => "number",
            "integer64" => "number",
            "decimal" => "number",
            "string" | "code" | "id" | "markdown" | "uri" | "url" | "canonical" | "oid"
            | "uuid" => "string",
            "date" | "dateTime" | "instant" | "time" => "string",
            "base64Binary" => "string",
            "xhtml" => "string",
            _ => fhir_type,
        }
        .to_string()
    }

    /// Wrap text to specified width
    fn wrap_text(text: &str, width: usize) -> Vec<String> {
        let mut lines = vec![];
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

    /// Generate interface declaration
    pub fn generate_interface(
        &self,
        name: &str,
        extends: Option<&str>,
        properties: &[(String, String, bool)], // (name, type, optional)
        doc: Option<&Documentation>,
    ) -> String {
        let mut output = String::new();

        // Add documentation
        if let Some(d) = doc {
            for line in self.generate_doc_comment(d) {
                output.push_str(&line);
                output.push('\n');
            }
        }

        // Interface declaration
        output.push_str("export interface ");
        output.push_str(name);

        if let Some(base) = extends {
            output.push_str(" extends ");
            output.push_str(base);
        }

        output.push_str(" {\n");

        // Properties
        for (prop_name, prop_type, optional) in properties {
            output.push_str("  ");
            output.push_str(prop_name);
            if *optional {
                output.push('?');
            }
            output.push_str(": ");
            output.push_str(prop_type);
            output.push_str(";\n");
        }

        output.push_str("}\n");

        output
    }

    /// Generate type alias
    pub fn generate_type_alias(
        &self,
        name: &str,
        type_def: &str,
        doc: Option<&Documentation>,
    ) -> String {
        let mut output = String::new();

        if let Some(d) = doc {
            for line in self.generate_doc_comment(d) {
                output.push_str(&line);
                output.push('\n');
            }
        }

        output.push_str("export type ");
        output.push_str(name);
        output.push_str(" = ");
        output.push_str(type_def);
        output.push_str(";\n");

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_primitive_types() {
        assert_eq!(TypeScriptBackend::map_primitive_type("boolean"), "boolean");
        assert_eq!(TypeScriptBackend::map_primitive_type("integer"), "number");
        assert_eq!(TypeScriptBackend::map_primitive_type("string"), "string");
        assert_eq!(TypeScriptBackend::map_primitive_type("dateTime"), "string");
    }

    #[test]
    fn test_format_identifier() {
        let backend = TypeScriptBackend::new();

        assert_eq!(
            backend.format_identifier("patient_name", IdentifierContext::TypeName),
            "PatientName"
        );
        assert_eq!(
            backend.format_identifier("patient_name", IdentifierContext::FieldName),
            "patientName"
        );
        assert_eq!(
            backend.format_identifier("max_length", IdentifierContext::ConstantName),
            "MAX_LENGTH"
        );
    }

    #[test]
    fn test_generate_interface() {
        let backend = TypeScriptBackend::new();

        let props = vec![
            ("id".to_string(), "string".to_string(), true),
            ("active".to_string(), "boolean".to_string(), true),
        ];

        let output = backend.generate_interface("Patient", Some("DomainResource"), &props, None);

        assert!(output.contains("export interface Patient extends DomainResource"));
        assert!(output.contains("id?: string;"));
        assert!(output.contains("active?: boolean;"));
    }
}
