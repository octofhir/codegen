//! TypeScript extension helper methods generation
//!
//! Generates type-safe extension helper methods for FHIR resources:
//! - addExtension methods for adding typed extensions
//! - getExtension methods for retrieving extension values
//! - hasExtension methods for checking extension existence
//! - removeExtension methods for removing extensions

use crate::core::Result;

/// Extension definition with metadata
#[derive(Debug, Clone)]
pub struct ExtensionDefinition {
    /// Extension canonical URL
    pub url: String,

    /// Extension name (derived from URL)
    pub name: String,

    /// Value type (e.g., "string", "dateTime", "Coding")
    pub value_type: ExtensionValueType,

    /// Target resource types this extension applies to
    pub target_types: Vec<String>,

    /// Short description
    pub description: String,

    /// Cardinality (0..1 or 0..*)
    pub is_array: bool,
}

/// Extension value type
#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionValueType {
    /// Primitive type (e.g., string, boolean, integer)
    Primitive(String),
    /// Complex type (e.g., Coding, CodeableConcept)
    Complex(String),
    /// Multiple possible types
    Choice(Vec<String>),
}

impl ExtensionDefinition {
    /// Create extension definition from URL
    pub fn from_url(url: &str) -> Self {
        // Extract name from URL
        // e.g., "http://hl7.org/fhir/StructureDefinition/patient-birthTime" -> "birthTime"
        let name = url
            .rsplit('/')
            .next()
            .and_then(|s| s.split('-').last())
            .unwrap_or("extension")
            .to_string();

        Self {
            url: url.to_string(),
            name: name.clone(),
            value_type: ExtensionValueType::Primitive("string".to_string()),
            target_types: vec![],
            description: format!("{} extension", name),
            is_array: false,
        }
    }

    /// Get the TypeScript type for the value
    pub fn get_typescript_type(&self) -> String {
        match &self.value_type {
            ExtensionValueType::Primitive(t) => {
                // Map FHIR primitives to TypeScript types
                match t.as_str() {
                    "boolean" => "boolean".to_string(),
                    "integer" | "positiveInt" | "unsignedInt" => "number".to_string(),
                    "decimal" => "number".to_string(),
                    _ => "string".to_string(), // date, dateTime, time, etc. are strings
                }
            }
            ExtensionValueType::Complex(t) => t.clone(),
            ExtensionValueType::Choice(types) => types.join(" | "),
        }
    }

    /// Get the value field name (e.g., "valueString", "valueDateTime")
    pub fn get_value_field_name(&self) -> String {
        match &self.value_type {
            ExtensionValueType::Primitive(t) => {
                let capitalized = capitalize(t);
                format!("value{}", capitalized)
            }
            ExtensionValueType::Complex(t) => {
                let capitalized = capitalize(t);
                format!("value{}", capitalized)
            }
            ExtensionValueType::Choice(_) => "value".to_string(),
        }
    }
}

/// Generator for extension helper methods
pub struct ExtensionGenerator;

impl ExtensionGenerator {
    /// Generate add extension method
    pub fn generate_add_method(ext: &ExtensionDefinition) -> Result<String> {
        let method_name = format!("add{}Extension", capitalize(&ext.name));
        let ts_type = ext.get_typescript_type();
        let value_field = ext.get_value_field_name();

        let mut code = String::new();

        // JSDoc comment
        code.push_str(&format!("  /**\n   * Add {} extension\n", ext.name));
        code.push_str(&format!("   * @param value - {}\n", ext.description));
        code.push_str("   */\n");

        // Method signature
        code.push_str(&format!("  {}(value: {}): this {{\n", method_name, ts_type));

        // Method body
        code.push_str("    if (!this._extension) {\n");
        code.push_str("      this._extension = [];\n");
        code.push_str("    }\n");
        code.push_str("    this._extension.push({\n");
        code.push_str(&format!("      url: \"{}\",\n", ext.url));
        code.push_str(&format!("      {}: value\n", value_field));
        code.push_str("    });\n");
        code.push_str("    return this;\n");
        code.push_str("  }\n");

        Ok(code)
    }

    /// Generate get extension method
    pub fn generate_get_method(ext: &ExtensionDefinition) -> Result<String> {
        let method_name = format!("get{}Extension", capitalize(&ext.name));
        let ts_type = ext.get_typescript_type();
        let value_field = ext.get_value_field_name();

        let mut code = String::new();

        // JSDoc comment
        code.push_str("  /**\n");
        code.push_str(&format!("   * Get {} extension value\n", ext.name));
        if ext.is_array {
            code.push_str("   * @returns Array of extension values\n");
        } else {
            code.push_str("   * @returns Extension value or undefined\n");
        }
        code.push_str("   */\n");

        // Method signature
        if ext.is_array {
            code.push_str(&format!("  {}(): {}[] {{\n", method_name, ts_type));
            code.push_str("    if (!this._extension) {\n");
            code.push_str("      return [];\n");
            code.push_str("    }\n");
            code.push_str("    return this._extension\n");
            code.push_str(&format!("      .filter(e => e.url === \"{}\")\n", ext.url));
            code.push_str(&format!("      .map(e => e.{})\n", value_field));
            code.push_str(&format!("      .filter((v): v is {} => v !== undefined);\n", ts_type));
            code.push_str("  }\n");
        } else {
            code.push_str(&format!("  {}(): {} | undefined {{\n", method_name, ts_type));
            code.push_str("    const ext = this._extension?.find(\n");
            code.push_str(&format!("      e => e.url === \"{}\"\n", ext.url));
            code.push_str("    );\n");
            code.push_str(&format!("    return ext?.{};\n", value_field));
            code.push_str("  }\n");
        }

        Ok(code)
    }

    /// Generate has extension method
    pub fn generate_has_method(ext: &ExtensionDefinition) -> Result<String> {
        let method_name = format!("has{}Extension", capitalize(&ext.name));

        let mut code = String::new();

        // JSDoc comment
        code.push_str("  /**\n");
        code.push_str(&format!("   * Check if {} extension exists\n", ext.name));
        code.push_str("   */\n");

        // Method signature and body
        code.push_str(&format!("  {}(): boolean {{\n", method_name));
        code.push_str("    return this._extension?.some(\n");
        code.push_str(&format!("      e => e.url === \"{}\"\n", ext.url));
        code.push_str("    ) ?? false;\n");
        code.push_str("  }\n");

        Ok(code)
    }

    /// Generate remove extension method
    pub fn generate_remove_method(ext: &ExtensionDefinition) -> Result<String> {
        let method_name = format!("remove{}Extension", capitalize(&ext.name));

        let mut code = String::new();

        // JSDoc comment
        code.push_str("  /**\n");
        code.push_str(&format!("   * Remove {} extension\n", ext.name));
        code.push_str("   */\n");

        // Method signature and body
        code.push_str(&format!("  {}(): this {{\n", method_name));
        code.push_str("    if (this._extension) {\n");
        code.push_str("      this._extension = this._extension.filter(\n");
        code.push_str(&format!("        e => e.url !== \"{}\"\n", ext.url));
        code.push_str("      );\n");
        code.push_str("    }\n");
        code.push_str("    return this;\n");
        code.push_str("  }\n");

        Ok(code)
    }

    /// Generate all extension methods for an extension
    pub fn generate_all_methods(ext: &ExtensionDefinition) -> Result<String> {
        let mut code = String::new();

        code.push_str(&Self::generate_add_method(ext)?);
        code.push('\n');
        code.push_str(&Self::generate_get_method(ext)?);
        code.push('\n');
        code.push_str(&Self::generate_has_method(ext)?);
        code.push('\n');
        code.push_str(&Self::generate_remove_method(ext)?);

        Ok(code)
    }

    /// Generate generic extension methods (for DomainResource base class)
    pub fn generate_generic_extension_methods() -> Result<String> {
        let mut code = String::new();

        // Generic addExtension
        code.push_str("  /**\n");
        code.push_str("   * Add a generic extension\n");
        code.push_str("   * @param extension - Extension object to add\n");
        code.push_str("   */\n");
        code.push_str("  addExtension(extension: Extension): this {\n");
        code.push_str("    if (!this._extension) {\n");
        code.push_str("      this._extension = [];\n");
        code.push_str("    }\n");
        code.push_str("    this._extension.push(extension);\n");
        code.push_str("    return this;\n");
        code.push_str("  }\n\n");

        // Generic getExtensionByUrl
        code.push_str("  /**\n");
        code.push_str("   * Get extension by URL\n");
        code.push_str("   * @param url - Extension canonical URL\n");
        code.push_str("   */\n");
        code.push_str("  getExtensionByUrl(url: string): Extension | undefined {\n");
        code.push_str("    return this._extension?.find(e => e.url === url);\n");
        code.push_str("  }\n\n");

        // Generic getExtensionsByUrl
        code.push_str("  /**\n");
        code.push_str("   * Get all extensions by URL\n");
        code.push_str("   * @param url - Extension canonical URL\n");
        code.push_str("   */\n");
        code.push_str("  getExtensionsByUrl(url: string): Extension[] {\n");
        code.push_str("    return this._extension?.filter(e => e.url === url) ?? [];\n");
        code.push_str("  }\n\n");

        // Generic removeExtension
        code.push_str("  /**\n");
        code.push_str("   * Remove extensions by URL\n");
        code.push_str("   * @param url - Extension canonical URL\n");
        code.push_str("   */\n");
        code.push_str("  removeExtension(url: string): this {\n");
        code.push_str("    if (this._extension) {\n");
        code.push_str("      this._extension = this._extension.filter(e => e.url !== url);\n");
        code.push_str("    }\n");
        code.push_str("    return this;\n");
        code.push_str("  }\n");

        Ok(code)
    }
}

/// Capitalize first letter of a string
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_definition_from_url() {
        let ext = ExtensionDefinition::from_url(
            "http://hl7.org/fhir/StructureDefinition/patient-birthTime",
        );
        assert_eq!(ext.name, "birthTime");
        assert_eq!(ext.url, "http://hl7.org/fhir/StructureDefinition/patient-birthTime");
    }

    #[test]
    fn test_get_typescript_type() {
        let mut ext = ExtensionDefinition::from_url("test");

        ext.value_type = ExtensionValueType::Primitive("boolean".to_string());
        assert_eq!(ext.get_typescript_type(), "boolean");

        ext.value_type = ExtensionValueType::Primitive("integer".to_string());
        assert_eq!(ext.get_typescript_type(), "number");

        ext.value_type = ExtensionValueType::Primitive("dateTime".to_string());
        assert_eq!(ext.get_typescript_type(), "string");

        ext.value_type = ExtensionValueType::Complex("Coding".to_string());
        assert_eq!(ext.get_typescript_type(), "Coding");
    }

    #[test]
    fn test_get_value_field_name() {
        let mut ext = ExtensionDefinition::from_url("test");

        ext.value_type = ExtensionValueType::Primitive("string".to_string());
        assert_eq!(ext.get_value_field_name(), "valueString");

        ext.value_type = ExtensionValueType::Primitive("dateTime".to_string());
        assert_eq!(ext.get_value_field_name(), "valueDateTime");

        ext.value_type = ExtensionValueType::Complex("Coding".to_string());
        assert_eq!(ext.get_value_field_name(), "valueCoding");
    }

    #[test]
    fn test_generate_add_method() {
        let ext = ExtensionDefinition {
            url: "http://hl7.org/fhir/StructureDefinition/patient-birthTime".to_string(),
            name: "birthTime".to_string(),
            value_type: ExtensionValueType::Primitive("dateTime".to_string()),
            target_types: vec!["Patient".to_string()],
            description: "Birth time".to_string(),
            is_array: false,
        };

        let code = ExtensionGenerator::generate_add_method(&ext).unwrap();
        assert!(code.contains("addBirthTimeExtension"));
        assert!(code.contains("value: string"));
        assert!(code.contains("valueDateTime: value"));
    }

    #[test]
    fn test_generate_get_method() {
        let ext = ExtensionDefinition {
            url: "http://hl7.org/fhir/StructureDefinition/patient-birthTime".to_string(),
            name: "birthTime".to_string(),
            value_type: ExtensionValueType::Primitive("dateTime".to_string()),
            target_types: vec!["Patient".to_string()],
            description: "Birth time".to_string(),
            is_array: false,
        };

        let code = ExtensionGenerator::generate_get_method(&ext).unwrap();
        assert!(code.contains("getBirthTimeExtension"));
        assert!(code.contains("string | undefined"));
    }

    #[test]
    fn test_generate_has_method() {
        let ext = ExtensionDefinition {
            url: "http://hl7.org/fhir/StructureDefinition/patient-birthTime".to_string(),
            name: "birthTime".to_string(),
            value_type: ExtensionValueType::Primitive("dateTime".to_string()),
            target_types: vec!["Patient".to_string()],
            description: "Birth time".to_string(),
            is_array: false,
        };

        let code = ExtensionGenerator::generate_has_method(&ext).unwrap();
        assert!(code.contains("hasBirthTimeExtension"));
        assert!(code.contains("boolean"));
    }

    #[test]
    fn test_generate_remove_method() {
        let ext = ExtensionDefinition {
            url: "http://hl7.org/fhir/StructureDefinition/patient-birthTime".to_string(),
            name: "birthTime".to_string(),
            value_type: ExtensionValueType::Primitive("dateTime".to_string()),
            target_types: vec!["Patient".to_string()],
            description: "Birth time".to_string(),
            is_array: false,
        };

        let code = ExtensionGenerator::generate_remove_method(&ext).unwrap();
        assert!(code.contains("removeBirthTimeExtension"));
        assert!(code.contains("this"));
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("birthTime"), "BirthTime");
        assert_eq!(capitalize("name"), "Name");
        assert_eq!(capitalize(""), "");
    }
}
