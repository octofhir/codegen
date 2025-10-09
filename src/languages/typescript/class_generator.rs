//! TypeScript class generation with fluent builder API
//!
//! Generates TypeScript classes for FHIR resources and datatypes with:
//! - Private fields with public getters/setters
//! - Fluent API (method chaining)
//! - Extension helper methods
//! - JSON serialization (toJSON/parse)

use crate::core::Result;
use crate::core::ir::{CardinalityRange, DataType, Property, PropertyType, ResourceType};
use crate::generator::LanguageBackend;
use crate::languages::typescript::extension_generator::{
    ExtensionDefinition, ExtensionGenerator, ExtensionValueType,
};

/// Generator for TypeScript classes
pub struct ClassGenerator;

impl ClassGenerator {
    /// Generate a TypeScript class for a resource
    pub fn generate_resource_class<B: LanguageBackend>(
        resource: &ResourceType,
        backend: &B,
    ) -> Result<String> {
        let mut code = String::new();

        // Generate class documentation
        if !resource.documentation.short.is_empty() {
            code.push_str(&Self::generate_jsdoc(&resource.documentation.short));
            code.push('\n');
        }

        // Class declaration
        let class_name = &resource.name;
        if let Some(base) = &resource.base {
            code.push_str(&format!("export class {} extends {} {{\n", class_name, base));
        } else {
            code.push_str(&format!("export class {} {{\n", class_name));
        }

        // Generate resourceType field for resources
        code.push_str("  resourceType: string;\n\n");

        // Generate private fields
        for prop in &resource.properties {
            code.push_str(&Self::generate_private_field(prop, backend)?);
        }

        code.push('\n');

        // Generate constructor
        code.push_str(&Self::generate_constructor(resource)?);

        code.push('\n');

        // Generate getters and setters
        for prop in &resource.properties {
            code.push_str(&Self::generate_getter(prop, backend)?);
            code.push('\n');
            code.push_str(&Self::generate_setter(prop, backend)?);
            code.push('\n');
        }

        // Generate fluent builder methods (add/has methods)
        for prop in &resource.properties {
            if Self::is_array_property(prop) {
                code.push_str(&Self::generate_add_method(prop, backend)?);
                code.push('\n');
            }
            code.push_str(&Self::generate_has_method(prop)?);
            code.push('\n');
        }

        // Generate extension helper methods if extensions are defined
        if !resource.extensions.is_empty() {
            code.push_str("  // Extension helper methods\n\n");
            for ext in &resource.extensions {
                let ext_def = Self::convert_extension_to_definition(ext);
                code.push_str(&ExtensionGenerator::generate_all_methods(&ext_def)?);
                code.push('\n');
            }
        }

        // Generate toJSON method
        code.push_str(&Self::generate_to_json(resource)?);
        code.push('\n');

        // Generate static parse method
        code.push_str(&Self::generate_parse_method(resource)?);

        // Close class
        code.push_str("}\n");

        Ok(code)
    }

    /// Generate a TypeScript class for a datatype
    pub fn generate_datatype_class<B: LanguageBackend>(
        datatype: &DataType,
        backend: &B,
    ) -> Result<String> {
        let mut code = String::new();

        // Generate class documentation
        if !datatype.documentation.short.is_empty() {
            code.push_str(&Self::generate_jsdoc(&datatype.documentation.short));
            code.push('\n');
        }

        // Class declaration
        let class_name = &datatype.name;
        if let Some(base) = &datatype.base {
            code.push_str(&format!("export class {} extends {} {{\n", class_name, base));
        } else {
            code.push_str(&format!("export class {} {{\n", class_name));
        }

        // Generate private fields
        for prop in &datatype.properties {
            code.push_str(&Self::generate_private_field(prop, backend)?);
        }

        code.push('\n');

        // Generate constructor (no resourceType for datatypes)
        code.push_str("  constructor() {\n");
        if datatype.base.is_some() {
            code.push_str("    super();\n");
        }
        code.push_str("  }\n\n");

        // Generate getters and setters
        for prop in &datatype.properties {
            code.push_str(&Self::generate_getter(prop, backend)?);
            code.push('\n');
            code.push_str(&Self::generate_setter(prop, backend)?);
            code.push('\n');
        }

        // Generate fluent builder methods (add/has methods)
        for prop in &datatype.properties {
            if Self::is_array_property(prop) {
                code.push_str(&Self::generate_add_method(prop, backend)?);
                code.push('\n');
            }
            code.push_str(&Self::generate_has_method(prop)?);
            code.push('\n');
        }

        // Generate toJSON method for datatype
        code.push_str(&Self::generate_datatype_to_json(datatype)?);
        code.push('\n');

        // Generate static parse method for datatype
        code.push_str(&Self::generate_datatype_parse_method(datatype)?);

        // Close class
        code.push_str("}\n");

        Ok(code)
    }

    /// Generate JSDoc comment
    fn generate_jsdoc(text: &str) -> String {
        format!("/**\n * {}\n */", text)
    }

    /// Generate private field declaration
    fn generate_private_field<B: LanguageBackend>(prop: &Property, backend: &B) -> Result<String> {
        let field_name = format!("_{}", &prop.name);
        let ts_type = backend.map_type(&prop.property_type);

        let optional = if Self::is_optional(&prop.cardinality) { "?" } else { "" };

        Ok(format!("  private {}{}: {};\n", field_name, optional, ts_type))
    }

    /// Generate constructor
    fn generate_constructor(resource: &ResourceType) -> Result<String> {
        let mut code = String::new();
        code.push_str("  constructor() {\n");

        if resource.base.is_some() {
            code.push_str("    super();\n");
        }

        code.push_str(&format!("    this.resourceType = \"{}\";\n", resource.name));
        code.push_str("  }\n");

        Ok(code)
    }

    /// Generate getter for a property
    fn generate_getter<B: LanguageBackend>(prop: &Property, backend: &B) -> Result<String> {
        let field_name = format!("_{}", &prop.name);
        let prop_name = &prop.name;
        let ts_type = backend.map_type(&prop.property_type);

        let optional = if Self::is_optional(&prop.cardinality) { " | undefined" } else { "" };

        Ok(format!(
            "  get {}(): {}{} {{\n    return this.{};\n  }}\n",
            prop_name, ts_type, optional, field_name
        ))
    }

    /// Generate setter for a property
    fn generate_setter<B: LanguageBackend>(prop: &Property, backend: &B) -> Result<String> {
        let field_name = format!("_{}", &prop.name);
        let setter_name = format!("set{}", Self::capitalize(&prop.name));
        let ts_type = backend.map_type(&prop.property_type);

        Ok(format!(
            "  {}(value: {}): this {{\n    this.{} = value;\n    return this;\n  }}\n",
            setter_name, ts_type, field_name
        ))
    }

    /// Generate add method for array properties
    fn generate_add_method<B: LanguageBackend>(prop: &Property, backend: &B) -> Result<String> {
        let field_name = format!("_{}", &prop.name);
        let method_name = format!("add{}", Self::capitalize(&prop.name));

        // Get element type (singular form of the array type)
        let element_type = Self::get_element_type(&prop.property_type, backend);

        Ok(format!(
            "  {}(value: {}): this {{\n    if (!this.{}) {{\n      this.{} = [];\n    }}\n    this.{}.push(value);\n    return this;\n  }}\n",
            method_name, element_type, field_name, field_name, field_name
        ))
    }

    /// Generate has method to check if property is set
    fn generate_has_method(prop: &Property) -> Result<String> {
        let field_name = format!("_{}", &prop.name);
        let method_name = format!("has{}", Self::capitalize(&prop.name));

        Ok(format!(
            "  {}(): boolean {{\n    return this.{} !== undefined && this.{} !== null;\n  }}\n",
            method_name, field_name, field_name
        ))
    }

    /// Generate toJSON method
    fn generate_to_json(resource: &ResourceType) -> Result<String> {
        let mut code = String::new();
        code.push_str("  toJSON(): any {\n");
        code.push_str("    const obj: any = {\n");
        code.push_str("      resourceType: this.resourceType\n");
        code.push_str("    };\n");

        for prop in &resource.properties {
            let prop_name = &prop.name;
            let field_name = format!("_{}", &prop.name);
            code.push_str(&format!(
                "    if (this.{} !== undefined) {{\n      obj.{} = this.{};\n    }}\n",
                field_name, prop_name, field_name
            ));
        }

        code.push_str("    return obj;\n");
        code.push_str("  }\n");

        Ok(code)
    }

    /// Generate static parse method
    fn generate_parse_method(resource: &ResourceType) -> Result<String> {
        let class_name = &resource.name;
        let mut code = String::new();

        code.push_str(&format!("  static parse(json: any): {} {{\n", class_name));
        code.push_str(&format!("    const instance = new {}();\n", class_name));

        for prop in &resource.properties {
            let prop_name = &prop.name;
            let setter_name = format!("set{}", Self::capitalize(prop_name));
            code.push_str(&format!(
                "    if (json.{} !== undefined) {{\n      instance.{}(json.{});\n    }}\n",
                prop_name, setter_name, prop_name
            ));
        }

        code.push_str("    return instance;\n");
        code.push_str("  }\n");

        Ok(code)
    }

    /// Generate toJSON method for datatype (no resourceType)
    fn generate_datatype_to_json(datatype: &DataType) -> Result<String> {
        let mut code = String::new();
        code.push_str("  toJSON(): any {\n");
        code.push_str("    const obj: any = {};\n");

        for prop in &datatype.properties {
            let prop_name = &prop.name;
            let field_name = format!("_{}", &prop.name);
            code.push_str(&format!(
                "    if (this.{} !== undefined) {{\n      obj.{} = this.{};\n    }}\n",
                field_name, prop_name, field_name
            ));
        }

        code.push_str("    return obj;\n");
        code.push_str("  }\n");

        Ok(code)
    }

    /// Generate static parse method for datatype
    fn generate_datatype_parse_method(datatype: &DataType) -> Result<String> {
        let class_name = &datatype.name;
        let mut code = String::new();

        code.push_str(&format!("  static parse(json: any): {} {{\n", class_name));
        code.push_str(&format!("    const instance = new {}();\n", class_name));

        for prop in &datatype.properties {
            let prop_name = &prop.name;
            let setter_name = format!("set{}", Self::capitalize(prop_name));
            code.push_str(&format!(
                "    if (json.{} !== undefined) {{\n      instance.{}(json.{});\n    }}\n",
                prop_name, setter_name, prop_name
            ));
        }

        code.push_str("    return instance;\n");
        code.push_str("  }\n");

        Ok(code)
    }

    /// Check if property is an array type
    fn is_array_property(prop: &Property) -> bool {
        // Array if max is None (unbounded) or > 1
        prop.cardinality.max.is_none() || prop.cardinality.max.unwrap() > 1
    }

    /// Check if property is optional
    fn is_optional(cardinality: &CardinalityRange) -> bool {
        cardinality.min == 0
    }

    /// Get element type for array properties
    fn get_element_type<B: LanguageBackend>(prop_type: &PropertyType, backend: &B) -> String {
        // Map the type - for arrays, the backend should include []
        // We need to extract the element type
        let full_type = backend.map_type(prop_type);
        // Remove [] suffix if present
        full_type.trim_end_matches("[]").to_string()
    }

    /// Capitalize first letter
    fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().chain(chars).collect(),
        }
    }

    /// Convert IR Extension to ExtensionDefinition
    fn convert_extension_to_definition(ext: &crate::core::ir::Extension) -> ExtensionDefinition {
        // Determine value type
        let value_type = if ext.value_types.len() == 1 {
            let type_name = &ext.value_types[0];
            // Check if it's a primitive or complex type
            if Self::is_primitive_type(type_name) {
                ExtensionValueType::Primitive(type_name.clone())
            } else {
                ExtensionValueType::Complex(type_name.clone())
            }
        } else if ext.value_types.len() > 1 {
            ExtensionValueType::Choice(ext.value_types.clone())
        } else {
            // Default to string if no value types specified
            ExtensionValueType::Primitive("string".to_string())
        };

        ExtensionDefinition {
            url: ext.url.clone(),
            name: ext.name.clone(),
            value_type,
            target_types: ext.target_types.clone(),
            description: ext.documentation.short.clone(),
            is_array: ext.cardinality.is_array(),
        }
    }

    /// Check if a type name is a FHIR primitive
    fn is_primitive_type(type_name: &str) -> bool {
        matches!(
            type_name,
            "boolean"
                | "integer"
                | "string"
                | "decimal"
                | "uri"
                | "url"
                | "canonical"
                | "base64Binary"
                | "instant"
                | "date"
                | "dateTime"
                | "time"
                | "code"
                | "oid"
                | "id"
                | "markdown"
                | "unsignedInt"
                | "positiveInt"
                | "uuid"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ir::{CardinalityRange, PropertyType};
    use crate::languages::typescript::backend::TypeScriptBackend;

    #[test]
    fn test_capitalize() {
        assert_eq!(ClassGenerator::capitalize("name"), "Name");
        assert_eq!(ClassGenerator::capitalize("birthDate"), "BirthDate");
        assert_eq!(ClassGenerator::capitalize(""), "");
    }

    #[test]
    fn test_is_array_property() {
        let prop = Property {
            name: "identifier".to_string(),
            path: "Patient.identifier".to_string(),
            property_type: PropertyType::Complex { type_name: "Identifier".to_string() },
            cardinality: CardinalityRange { min: 0, max: None }, // 0..*
            is_choice: false,
            choice_types: vec![],
            is_modifier: false,
            is_summary: false,
            binding: None,
            constraints: vec![],
            short_description: "".to_string(),
            definition: "".to_string(),
            comments: None,
            examples: vec![],
        };
        assert!(ClassGenerator::is_array_property(&prop));

        let prop2 = Property {
            name: "id".to_string(),
            path: "Patient.id".to_string(),
            property_type: PropertyType::Primitive { type_name: "string".to_string() },
            cardinality: CardinalityRange { min: 0, max: Some(1) }, // 0..1
            is_choice: false,
            choice_types: vec![],
            is_modifier: false,
            is_summary: false,
            binding: None,
            constraints: vec![],
            short_description: "".to_string(),
            definition: "".to_string(),
            comments: None,
            examples: vec![],
        };
        assert!(!ClassGenerator::is_array_property(&prop2));
    }

    #[test]
    fn test_is_optional() {
        let optional = CardinalityRange { min: 0, max: Some(1) };
        assert!(ClassGenerator::is_optional(&optional));

        let required = CardinalityRange { min: 1, max: Some(1) };
        assert!(!ClassGenerator::is_optional(&required));
    }

    #[test]
    fn test_get_element_type() {
        let backend = TypeScriptBackend::new();
        let prop_type = PropertyType::Complex { type_name: "Identifier".to_string() };

        assert_eq!(ClassGenerator::get_element_type(&prop_type, &backend), "Identifier");
    }
}
