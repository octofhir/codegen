//! TypeScript base class generation
//!
//! Generates base classes for FHIR type hierarchy:
//! - Resource (abstract base for all resources)
//! - DomainResource (extends Resource, adds narrative and extensions)
//! - Element (base for all data types)
//! - BackboneElement (extends Element, for nested structures)

use crate::core::Result;
use crate::languages::typescript::extension_generator::ExtensionGenerator;

/// Generator for TypeScript base classes
pub struct BaseClassGenerator;

impl BaseClassGenerator {
    /// Generate Resource abstract base class
    pub fn generate_resource_base_class() -> Result<String> {
        let mut code = String::new();

        // JSDoc
        code.push_str("/**\n");
        code.push_str(" * Base Resource class for all FHIR resources\n");
        code.push_str(" * @abstract\n");
        code.push_str(" */\n");

        // Class declaration
        code.push_str("export abstract class Resource {\n");

        // resourceType field (public, set by subclasses)
        code.push_str("  resourceType: string;\n\n");

        // Private fields common to all resources
        code.push_str("  private _id?: string;\n");
        code.push_str("  private _meta?: Meta;\n");
        code.push_str("  private _implicitRules?: string;\n");
        code.push_str("  private _language?: string;\n\n");

        // Constructor
        code.push_str("  protected constructor(resourceType: string) {\n");
        code.push_str("    this.resourceType = resourceType;\n");
        code.push_str("  }\n\n");

        // Getters and setters for common fields
        Self::add_property_methods(&mut code, "id", "string", "Logical id of this resource");
        Self::add_property_methods(&mut code, "meta", "Meta", "Metadata about the resource");
        Self::add_property_methods(
            &mut code,
            "implicitRules",
            "string",
            "A set of rules under which this content was created",
        );
        Self::add_property_methods(
            &mut code,
            "language",
            "string",
            "Language of the resource content",
        );

        // Abstract toJSON method
        code.push_str("  /**\n");
        code.push_str("   * Serialize to JSON\n");
        code.push_str("   */\n");
        code.push_str("  abstract toJSON(): any;\n");

        // Close class
        code.push_str("}\n");

        Ok(code)
    }

    /// Generate DomainResource abstract base class
    pub fn generate_domain_resource_base_class() -> Result<String> {
        let mut code = String::new();

        // JSDoc
        code.push_str("/**\n");
        code.push_str(" * Base DomainResource class - parent for most FHIR resources\n");
        code.push_str(" * Adds narrative, contained resources, and extensions\n");
        code.push_str(" * @abstract\n");
        code.push_str(" */\n");

        // Class declaration
        code.push_str("export abstract class DomainResource extends Resource {\n");

        // Private fields specific to DomainResource
        code.push_str("  private _text?: Narrative;\n");
        code.push_str("  private _contained?: Resource[];\n");
        code.push_str("  private _extension?: Extension[];\n");
        code.push_str("  private _modifierExtension?: Extension[];\n\n");

        // Constructor
        code.push_str("  protected constructor(resourceType: string) {\n");
        code.push_str("    super(resourceType);\n");
        code.push_str("  }\n\n");

        // Getters and setters for DomainResource fields
        Self::add_property_methods(&mut code, "text", "Narrative", "Text summary of the resource");
        Self::add_array_property_methods(
            &mut code,
            "contained",
            "Resource",
            "Contained, inline Resources",
        );
        Self::add_array_property_methods(
            &mut code,
            "extension",
            "Extension",
            "Additional content defined by implementations",
        );
        Self::add_array_property_methods(
            &mut code,
            "modifierExtension",
            "Extension",
            "Extensions that cannot be ignored",
        );

        // Generic extension helper methods
        code.push('\n');
        code.push_str(&ExtensionGenerator::generate_generic_extension_methods()?);

        // Close class
        code.push_str("}\n");

        Ok(code)
    }

    /// Generate Element abstract base class
    pub fn generate_element_base_class() -> Result<String> {
        let mut code = String::new();

        // JSDoc
        code.push_str("/**\n");
        code.push_str(" * Base Element class for all FHIR data types\n");
        code.push_str(" * @abstract\n");
        code.push_str(" */\n");

        // Class declaration
        code.push_str("export abstract class Element {\n");

        // Private fields
        code.push_str("  private _id?: string;\n");
        code.push_str("  private _extension?: Extension[];\n\n");

        // Constructor
        code.push_str("  protected constructor() {}\n\n");

        // Getters and setters
        Self::add_property_methods(
            &mut code,
            "id",
            "string",
            "Unique id for inter-element referencing",
        );
        Self::add_array_property_methods(
            &mut code,
            "extension",
            "Extension",
            "Additional content defined by implementations",
        );

        // Generic extension methods
        code.push('\n');
        code.push_str(&ExtensionGenerator::generate_generic_extension_methods()?);

        // Abstract toJSON method
        code.push_str("\n  /**\n");
        code.push_str("   * Serialize to JSON\n");
        code.push_str("   */\n");
        code.push_str("  abstract toJSON(): any;\n");

        // Close class
        code.push_str("}\n");

        Ok(code)
    }

    /// Generate BackboneElement abstract base class
    pub fn generate_backbone_element_base_class() -> Result<String> {
        let mut code = String::new();

        // JSDoc
        code.push_str("/**\n");
        code.push_str(" * Base BackboneElement class\n");
        code.push_str(" * Used for nested complex structures within resources\n");
        code.push_str(" * @abstract\n");
        code.push_str(" */\n");

        // Class declaration
        code.push_str("export abstract class BackboneElement extends Element {\n");

        // Private fields specific to BackboneElement
        code.push_str("  private _modifierExtension?: Extension[];\n\n");

        // Constructor
        code.push_str("  protected constructor() {\n");
        code.push_str("    super();\n");
        code.push_str("  }\n\n");

        // Getters and setters
        Self::add_array_property_methods(
            &mut code,
            "modifierExtension",
            "Extension",
            "Extensions that cannot be ignored even if unrecognized",
        );

        // Close class
        code.push_str("}\n");

        Ok(code)
    }

    /// Generate all base classes in one file
    pub fn generate_all_base_classes() -> Result<String> {
        let mut code = String::new();

        // File header
        code.push_str("/**\n");
        code.push_str(" * FHIR Base Classes\n");
        code.push_str(" * Generated base class hierarchy for FHIR resources and data types\n");
        code.push_str(" */\n\n");

        // Import statements
        code.push_str("import { Extension } from './Extension';\n");
        code.push_str("import { Meta } from './Meta';\n");
        code.push_str("import { Narrative } from './Narrative';\n\n");

        // Generate Resource
        code.push_str(&Self::generate_resource_base_class()?);
        code.push_str("\n\n");

        // Generate DomainResource
        code.push_str(&Self::generate_domain_resource_base_class()?);
        code.push_str("\n\n");

        // Generate Element
        code.push_str(&Self::generate_element_base_class()?);
        code.push_str("\n\n");

        // Generate BackboneElement
        code.push_str(&Self::generate_backbone_element_base_class()?);

        Ok(code)
    }

    /// Helper to add getter, setter, and has methods for a property
    fn add_property_methods(code: &mut String, prop_name: &str, ts_type: &str, description: &str) {
        let field_name = format!("_{}", prop_name);
        let setter_name = format!("set{}", capitalize(prop_name));
        let has_name = format!("has{}", capitalize(prop_name));

        // Getter
        code.push_str(&format!("  /**\n   * {}\n   */\n", description));
        code.push_str(&format!("  get {}(): {} | undefined {{\n", prop_name, ts_type));
        code.push_str(&format!("    return this.{};\n", field_name));
        code.push_str("  }\n\n");

        // Setter
        code.push_str(&format!("  /**\n   * Set {}\n   */\n", prop_name));
        code.push_str(&format!("  {}(value: {}): this {{\n", setter_name, ts_type));
        code.push_str(&format!("    this.{} = value;\n", field_name));
        code.push_str("    return this;\n");
        code.push_str("  }\n\n");

        // Has method
        code.push_str(&format!("  /**\n   * Check if {} is set\n   */\n", prop_name));
        code.push_str(&format!("  {}(): boolean {{\n", has_name));
        code.push_str(&format!(
            "    return this.{} !== undefined && this.{} !== null;\n",
            field_name, field_name
        ));
        code.push_str("  }\n\n");
    }

    /// Helper to add getter, setter, add, and has methods for array properties
    fn add_array_property_methods(
        code: &mut String,
        prop_name: &str,
        element_type: &str,
        description: &str,
    ) {
        let field_name = format!("_{}", prop_name);
        let setter_name = format!("set{}", capitalize(prop_name));
        let add_name = format!("add{}", capitalize(prop_name));
        let has_name = format!("has{}", capitalize(prop_name));

        // Getter
        code.push_str(&format!("  /**\n   * {}\n   */\n", description));
        code.push_str(&format!("  get {}(): {}[] | undefined {{\n", prop_name, element_type));
        code.push_str(&format!("    return this.{};\n", field_name));
        code.push_str("  }\n\n");

        // Setter
        code.push_str(&format!("  /**\n   * Set {}\n   */\n", prop_name));
        code.push_str(&format!("  {}(value: {}[]): this {{\n", setter_name, element_type));
        code.push_str(&format!("    this.{} = value;\n", field_name));
        code.push_str("    return this;\n");
        code.push_str("  }\n\n");

        // Add method
        code.push_str(&format!("  /**\n   * Add to {}\n   */\n", prop_name));
        code.push_str(&format!("  {}(value: {}): this {{\n", add_name, element_type));
        code.push_str(&format!("    if (!this.{}) {{\n", field_name));
        code.push_str(&format!("      this.{} = [];\n", field_name));
        code.push_str("    }\n");
        code.push_str(&format!("    this.{}.push(value);\n", field_name));
        code.push_str("    return this;\n");
        code.push_str("  }\n\n");

        // Has method
        code.push_str(&format!("  /**\n   * Check if {} is set\n   */\n", prop_name));
        code.push_str(&format!("  {}(): boolean {{\n", has_name));
        code.push_str(&format!(
            "    return this.{} !== undefined && this.{} !== null && this.{}.length > 0;\n",
            field_name, field_name, field_name
        ));
        code.push_str("  }\n\n");
    }
}

/// Capitalize first letter
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
    fn test_generate_resource_base_class() {
        let result = BaseClassGenerator::generate_resource_base_class();
        assert!(result.is_ok());

        let code = result.unwrap();
        assert!(code.contains("export abstract class Resource"));
        assert!(code.contains("resourceType: string"));
        assert!(code.contains("private _id?: string"));
        assert!(code.contains("protected constructor(resourceType: string)"));
        assert!(code.contains("abstract toJSON(): any"));
    }

    #[test]
    fn test_generate_domain_resource_base_class() {
        let result = BaseClassGenerator::generate_domain_resource_base_class();
        assert!(result.is_ok());

        let code = result.unwrap();
        assert!(code.contains("export abstract class DomainResource extends Resource"));
        assert!(code.contains("private _text?: Narrative"));
        assert!(code.contains("private _extension?: Extension[]"));
        assert!(code.contains("addExtension"));
        assert!(code.contains("getExtensionByUrl"));
    }

    #[test]
    fn test_generate_element_base_class() {
        let result = BaseClassGenerator::generate_element_base_class();
        assert!(result.is_ok());

        let code = result.unwrap();
        assert!(code.contains("export abstract class Element"));
        assert!(code.contains("private _id?: string"));
        assert!(code.contains("private _extension?: Extension[]"));
    }

    #[test]
    fn test_generate_backbone_element_base_class() {
        let result = BaseClassGenerator::generate_backbone_element_base_class();
        assert!(result.is_ok());

        let code = result.unwrap();
        assert!(code.contains("export abstract class BackboneElement extends Element"));
        assert!(code.contains("private _modifierExtension?: Extension[]"));
    }

    #[test]
    fn test_generate_all_base_classes() {
        let result = BaseClassGenerator::generate_all_base_classes();
        assert!(result.is_ok());

        let code = result.unwrap();
        assert!(code.contains("export abstract class Resource"));
        assert!(code.contains("export abstract class DomainResource extends Resource"));
        assert!(code.contains("export abstract class Element"));
        assert!(code.contains("export abstract class BackboneElement extends Element"));
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("id"), "Id");
        assert_eq!(capitalize("text"), "Text");
        assert_eq!(capitalize(""), "");
    }
}
