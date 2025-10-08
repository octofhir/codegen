use crate::core::Result;
use crate::core::ir::{Property, ResourceType};
use crate::languages::typescript::backend::TypeScriptBackend;
use crate::templates::genco_engine::GencoTemplateEngine;
use genco::prelude::*;
use heck::ToLowerCamelCase;

/// Generator for TypeScript validation functions
#[allow(dead_code)]
pub struct ValidationGenerator {
    backend: TypeScriptBackend,
}

impl ValidationGenerator {
    /// Create a new validation generator
    pub fn new(backend: TypeScriptBackend) -> Self {
        Self { backend }
    }

    /// Generate the ValidationResult and ValidationError type definitions
    pub fn generate_validation_types(&self) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // ValidationError interface
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Validation error details");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.append("export interface ValidationError {");
        tokens.indent();
        tokens.push();
        tokens.append("/** Path to the field that failed validation */");
        tokens.push();
        tokens.append("path: string;");
        tokens.push();
        tokens.append("/** Error message */");
        tokens.push();
        tokens.append("message: string;");
        tokens.push();
        tokens.append("/** Severity level */");
        tokens.push();
        tokens.append("severity: \"error\" | \"warning\" | \"info\";");
        tokens.push();
        tokens.append("/** FHIR constraint key (if applicable) */");
        tokens.push();
        tokens.append("constraint?: string;");
        tokens.unindent();
        tokens.push();
        tokens.append("}");
        tokens.push();
        tokens.push();

        // ValidationResult interface
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Validation result");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.append("export interface ValidationResult {");
        tokens.indent();
        tokens.push();
        tokens.append("/** Whether the resource is valid (no errors) */");
        tokens.push();
        tokens.append("valid: boolean;");
        tokens.push();
        tokens.append("/** List of validation errors/warnings */");
        tokens.push();
        tokens.append("errors: ValidationError[];");
        tokens.unindent();
        tokens.push();
        tokens.append("}");

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate validation function for a single property
    fn generate_property_validation(
        &self,
        tokens: &mut js::Tokens,
        property: &Property,
        resource_name: &str,
    ) {
        let field_name = &property.name;
        let path = format!("{}.{}", resource_name, field_name);

        // Check required fields
        if property.cardinality.min > 0 {
            tokens.append(format!("// Check required field: {}", field_name));
            tokens.push();
            tokens.append(format!("if (!resource.{}) {{", field_name));
            tokens.indent();
            tokens.push();
            tokens.append("errors.push({");
            tokens.indent();
            tokens.push();
            tokens.append(format!("path: \"{}\",", path));
            tokens.push();
            tokens.append(format!("message: \"Required field '{}' is missing\",", field_name));
            tokens.push();
            tokens.append("severity: \"error\"");
            tokens.unindent();
            tokens.push();
            tokens.append("});");
            tokens.unindent();
            tokens.push();
            tokens.append("}");
            tokens.push();
            tokens.push();
        }

        // Check array cardinality
        if property.cardinality.max.is_some() && property.cardinality.is_array() {
            tokens.append(format!("// Check array cardinality for: {}", field_name));
            tokens.push();
            tokens.append(format!(
                "if (resource.{} && Array.isArray(resource.{})) {{",
                field_name, field_name
            ));
            tokens.indent();
            tokens.push();

            if let Some(max) = property.cardinality.max
                && max != u32::MAX
            {
                tokens.append(format!("if (resource.{}.length > {}) {{", field_name, max));
                tokens.indent();
                tokens.push();
                tokens.append("errors.push({");
                tokens.indent();
                tokens.push();
                tokens.append(format!("path: \"{}\",", path));
                tokens.push();
                tokens.append(format!(
                    "message: \"Array '{}' exceeds maximum length of {}\",",
                    field_name, max
                ));
                tokens.push();
                tokens.append("severity: \"error\"");
                tokens.unindent();
                tokens.push();
                tokens.append("});");
                tokens.unindent();
                tokens.push();
                tokens.append("}");
                tokens.push();
            }

            if property.cardinality.min > 0 {
                tokens.append(format!(
                    "if (resource.{}.length < {}) {{",
                    field_name, property.cardinality.min
                ));
                tokens.indent();
                tokens.push();
                tokens.append("errors.push({");
                tokens.indent();
                tokens.push();
                tokens.append(format!("path: \"{}\",", path));
                tokens.push();
                tokens.append(format!(
                    "message: \"Array '{}' requires at least {} element(s)\",",
                    field_name, property.cardinality.min
                ));
                tokens.push();
                tokens.append("severity: \"error\"");
                tokens.unindent();
                tokens.push();
                tokens.append("});");
                tokens.unindent();
                tokens.push();
                tokens.append("}");
                tokens.push();
            }

            tokens.unindent();
            tokens.push();
            tokens.append("}");
            tokens.push();
            tokens.push();
        }
    }

    /// Generate validation function for a resource
    pub fn generate_resource_validation(&self, resource: &ResourceType) -> Result<String> {
        let mut tokens = js::Tokens::new();

        let resource_name = &resource.name;
        let function_name = format!("validate{}", resource_name);

        // JSDoc
        tokens.append("/**");
        tokens.push();
        tokens.append(format!(" * Validate a {} resource", resource_name));
        tokens.push();
        tokens.append(" * ");
        tokens.push();
        tokens.append(format!(" * @param resource - The {} to validate", resource_name));
        tokens.push();
        tokens.append(" * @returns Validation result with any errors");
        tokens.push();
        tokens.append(" */");
        tokens.push();

        // Function declaration
        tokens.append(format!(
            "export function {}(resource: {}): ValidationResult {{",
            function_name, resource_name
        ));
        tokens.indent();
        tokens.push();

        // Initialize errors array
        tokens.append("const errors: ValidationError[] = [];");
        tokens.push();
        tokens.push();

        // Validate resourceType
        tokens.append("// Validate resourceType");
        tokens.push();
        tokens.append(format!("if (resource.resourceType !== \"{}\") {{", resource_name));
        tokens.indent();
        tokens.push();
        tokens.append("errors.push({");
        tokens.indent();
        tokens.push();
        tokens.append(format!("path: \"{}.resourceType\",", resource_name));
        tokens.push();
        tokens.append(format!(
            "message: `Invalid resourceType: ${{resource.resourceType}}. Expected '{}'`,",
            resource_name
        ));
        tokens.push();
        tokens.append("severity: \"error\"");
        tokens.unindent();
        tokens.push();
        tokens.append("});");
        tokens.unindent();
        tokens.push();
        tokens.append("}");
        tokens.push();
        tokens.push();

        // Validate each property
        for property in &resource.properties {
            self.generate_property_validation(&mut tokens, property, resource_name);
        }

        // Return result
        tokens.append("return {");
        tokens.indent();
        tokens.push();
        tokens.append("valid: errors.filter(e => e.severity === \"error\").length === 0,");
        tokens.push();
        tokens.append("errors");
        tokens.unindent();
        tokens.push();
        tokens.append("};");

        tokens.unindent();
        tokens.push();
        tokens.append("}");

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate validation module for a resource
    pub fn generate_validation_module(&self, resource: &ResourceType) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // File header
        tokens.append("/**");
        tokens.push();
        tokens.append(format!(" * Validation functions for {} resource", resource.name));
        tokens.push();
        tokens.append(" * ");
        tokens.push();
        tokens.append(" * This file is auto-generated. Do not edit manually.");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.push();

        // Imports
        tokens.append(format!(
            "import {{ {} }} from './{}';",
            resource.name,
            resource.name.to_lower_camel_case()
        ));
        tokens.push();
        tokens.append("import { ValidationResult, ValidationError } from './validation';");
        tokens.push();
        tokens.push();

        // Generate validation function
        tokens.append(self.generate_resource_validation(resource)?);

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate the main validation types module
    pub fn generate_validation_types_module(&self) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // File header
        tokens.append("/**");
        tokens.push();
        tokens.append(" * FHIR validation types");
        tokens.push();
        tokens.append(" * ");
        tokens.push();
        tokens.append(" * This file is auto-generated. Do not edit manually.");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.push();

        // Generate types
        tokens.append(self.generate_validation_types()?);

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate a helper to check if a value is defined
    pub fn generate_is_defined_helper(&self) -> Result<String> {
        let mut tokens = js::Tokens::new();

        tokens.append("/**");
        tokens.push();
        tokens.append(" * Check if a value is defined (not null or undefined)");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.append("function isDefined<T>(value: T | null | undefined): value is T {");
        tokens.indent();
        tokens.push();
        tokens.append("return value !== null && value !== undefined;");
        tokens.unindent();
        tokens.push();
        tokens.append("}");

        GencoTemplateEngine::format_typescript(&tokens)
    }
}
