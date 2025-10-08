//! TypeScript choice element (discriminated union) generation

use crate::core::Result;
use crate::core::ir::Property;
use crate::generator::LanguageBackend;
use crate::templates::genco_engine::{GencoTemplateEngine, helpers};
use genco::prelude::*;
use heck::{ToPascalCase, ToSnakeCase};

/// Generator for TypeScript choice element discriminated unions
pub struct ChoiceElementGenerator<B: LanguageBackend> {
    /// Language backend for type mapping
    #[allow(dead_code)]
    backend: B,
}

impl<B: LanguageBackend> ChoiceElementGenerator<B> {
    /// Create a new choice element generator
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    /// Generate discriminated union type for a choice element
    ///
    /// For a choice element like `value[x]` with types `string | boolean | integer`,
    /// generates:
    /// ```typescript
    /// export type ValueChoice =
    ///   | { valueString: string }
    ///   | { valueBoolean: boolean }
    ///   | { valueInteger: number };
    /// ```
    pub fn generate_choice_union(
        &self,
        property: &Property,
        resource_or_type_name: &str,
    ) -> Result<String> {
        if !property.is_choice {
            return Err(crate::core::Error::Generator(
                "Property is not a choice element".to_string(),
            ));
        }

        let mut tokens = js::Tokens::new();

        // Extract base name (remove [x] suffix)
        let base_name = property.name.replace("[x]", "");
        let choice_name = format!("{}{}Choice", resource_or_type_name, base_name.to_pascal_case());

        // JSDoc comment
        let doc = vec![
            format!("Choice type for {} property", base_name),
            String::new(),
            property.short_description.clone(),
        ];

        tokens.append(helpers::jsdoc_comment(&doc));
        tokens.push();

        // Type alias with union
        tokens.append("export type ");
        tokens.append(&choice_name);
        tokens.append(" =");
        tokens.push();

        // Generate each variant
        for (i, choice_type) in property.choice_types.iter().enumerate() {
            if i > 0 {
                tokens.push();
            }

            // Property name for this variant (e.g., valueString, valueBoolean)
            let variant_name = format!("{}{}", base_name, choice_type.to_pascal_case());

            // Map type
            let ts_type = self.map_choice_type(choice_type);

            tokens.append("  | { ");
            tokens.append(&variant_name);
            tokens.append(": ");
            tokens.append(&ts_type);
            tokens.append(" }");
        }

        tokens.append(";");

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate type guards for choice element variants
    ///
    /// For each variant, generates a type guard like:
    /// ```typescript
    /// export function hasValueString(obj: ValueChoice): obj is { valueString: string } {
    ///   return 'valueString' in obj;
    /// }
    /// ```
    pub fn generate_choice_type_guards(
        &self,
        property: &Property,
        resource_or_type_name: &str,
    ) -> Result<String> {
        if !property.is_choice {
            return Err(crate::core::Error::Generator(
                "Property is not a choice element".to_string(),
            ));
        }

        let mut tokens = js::Tokens::new();

        let base_name = property.name.replace("[x]", "");
        let choice_name = format!("{}{}Choice", resource_or_type_name, base_name.to_pascal_case());

        // Header comment
        tokens.append("/**");
        tokens.push();
        tokens.append(format!(" * Type guards for {} choice element", base_name));
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.push();

        // Generate a type guard for each variant
        for (i, choice_type) in property.choice_types.iter().enumerate() {
            if i > 0 {
                tokens.push();
                tokens.push();
            }

            let variant_name = format!("{}{}", base_name, choice_type.to_pascal_case());
            let guard_name = format!("has{}", variant_name.to_pascal_case());
            let ts_type = self.map_choice_type(choice_type);

            // JSDoc for this guard
            tokens.append("/**");
            tokens.push();
            tokens.append(format!(" * Check if choice has {} variant", variant_name));
            tokens.push();
            tokens.append(" */");
            tokens.push();

            // Function signature
            tokens.append("export function ");
            tokens.append(&guard_name);
            tokens.append("(obj: ");
            tokens.append(&choice_name);
            tokens.append("): obj is { ");
            tokens.append(&variant_name);
            tokens.append(": ");
            tokens.append(&ts_type);
            tokens.append(" } {");
            tokens.push();

            // Implementation
            tokens.append("  return '");
            tokens.append(&variant_name);
            tokens.append("' in obj;");
            tokens.push();

            tokens.append("}");
        }

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate a helper function to extract the value from a choice
    ///
    /// ```typescript
    /// export function getValueChoiceValue(choice: ValueChoice): string | boolean | number {
    ///   if (hasValueString(choice)) return choice.valueString;
    ///   if (hasValueBoolean(choice)) return choice.valueBoolean;
    ///   if (hasValueInteger(choice)) return choice.valueInteger;
    ///   throw new Error('Invalid choice type');
    /// }
    /// ```
    pub fn generate_choice_value_extractor(
        &self,
        property: &Property,
        resource_or_type_name: &str,
    ) -> Result<String> {
        if !property.is_choice {
            return Err(crate::core::Error::Generator(
                "Property is not a choice element".to_string(),
            ));
        }

        let mut tokens = js::Tokens::new();

        let base_name = property.name.replace("[x]", "");
        let choice_name = format!("{}{}Choice", resource_or_type_name, base_name.to_pascal_case());
        let function_name = format!("get{}Value", choice_name);

        // Return type (union of all choice types)
        let return_types: Vec<String> =
            property.choice_types.iter().map(|t| self.map_choice_type(t)).collect();
        let return_type = return_types.join(" | ");

        // JSDoc
        tokens.append("/**");
        tokens.push();
        tokens.append(format!(" * Extract value from {} choice", base_name));
        tokens.push();
        tokens.append(" */");
        tokens.push();

        // Function signature
        tokens.append("export function ");
        tokens.append(&function_name);
        tokens.append("(choice: ");
        tokens.append(&choice_name);
        tokens.append("): ");
        tokens.append(&return_type);
        tokens.append(" {");
        tokens.push();

        // Generate if statements for each variant
        for choice_type in &property.choice_types {
            let variant_name = format!("{}{}", base_name, choice_type.to_pascal_case());
            let guard_name = format!("has{}", variant_name.to_pascal_case());

            tokens.append("  if (");
            tokens.append(&guard_name);
            tokens.append("(choice)) return choice.");
            tokens.append(&variant_name);
            tokens.append(";");
            tokens.push();
        }

        // Fallback
        tokens.append("  throw new Error('Invalid choice type');");
        tokens.push();

        tokens.append("}");

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate complete choice element module
    ///
    /// Generates a complete TypeScript module with:
    /// - Choice type definition
    /// - Type guards
    /// - Value extractor
    pub fn generate_choice_module(
        &self,
        property: &Property,
        resource_or_type_name: &str,
    ) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // Module header
        tokens.append("/**");
        tokens.push();
        tokens
            .append(format!(" * Choice element for {}.{}", resource_or_type_name, &property.name));
        tokens.push();
        tokens.append(" * ");
        tokens.push();
        tokens.append(format!(" * {}", property.short_description));
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.push();

        // Choice type definition
        let choice_union = self.generate_choice_union(property, resource_or_type_name)?;
        tokens.append(&choice_union);
        tokens.push();
        tokens.push();

        // Type guards
        let type_guards = self.generate_choice_type_guards(property, resource_or_type_name)?;
        tokens.append(&type_guards);
        tokens.push();
        tokens.push();

        // Value extractor
        let value_extractor =
            self.generate_choice_value_extractor(property, resource_or_type_name)?;
        tokens.append(&value_extractor);

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Map choice type to TypeScript type
    fn map_choice_type(&self, choice_type: &str) -> String {
        // Use the backend's map_type for primitives
        match choice_type {
            "boolean" => "boolean".to_string(),
            "integer" | "positiveInt" | "unsignedInt" => "number".to_string(),
            "decimal" => "number".to_string(),
            "string" | "code" | "id" | "markdown" | "uri" | "url" | "canonical" => {
                "string".to_string()
            }
            "date" | "dateTime" | "instant" | "time" => "string".to_string(),
            // Complex types - keep as-is
            _ => choice_type.to_string(),
        }
    }

    /// Generate inline choice type for property
    ///
    /// For use directly in interface properties:
    /// ```typescript
    /// value?: { valueString: string } | { valueBoolean: boolean }
    /// ```
    pub fn generate_inline_choice_type(&self, property: &Property) -> Result<String> {
        if !property.is_choice {
            return Err(crate::core::Error::Generator(
                "Property is not a choice element".to_string(),
            ));
        }

        let base_name = property.name.replace("[x]", "");
        let mut parts = Vec::new();

        for choice_type in &property.choice_types {
            let variant_name = format!("{}{}", base_name, choice_type.to_pascal_case());
            let ts_type = self.map_choice_type(choice_type);
            parts.push(format!("{{ {}: {} }}", variant_name, ts_type));
        }

        Ok(parts.join(" | "))
    }

    /// Generate a const enum for choice type discrimination
    ///
    /// ```typescript
    /// export const ValueChoiceType = {
    ///   STRING: 'valueString',
    ///   BOOLEAN: 'valueBoolean',
    ///   INTEGER: 'valueInteger',
    /// } as const;
    /// ```
    pub fn generate_choice_enum(
        &self,
        property: &Property,
        resource_or_type_name: &str,
    ) -> Result<String> {
        if !property.is_choice {
            return Err(crate::core::Error::Generator(
                "Property is not a choice element".to_string(),
            ));
        }

        let mut tokens = js::Tokens::new();

        let base_name = property.name.replace("[x]", "");
        let enum_name = format!("{}{}Type", resource_or_type_name, base_name.to_pascal_case());

        // JSDoc
        tokens.append("/**");
        tokens.push();
        tokens.append(format!(" * Enum for {} choice variants", base_name));
        tokens.push();
        tokens.append(" */");
        tokens.push();

        // Const object
        tokens.append("export const ");
        tokens.append(&enum_name);
        tokens.append(" = {");
        tokens.push();

        for choice_type in &property.choice_types {
            let variant_name = format!("{}{}", base_name, choice_type.to_pascal_case());
            let enum_key = choice_type.to_snake_case().to_uppercase();

            tokens.append("  ");
            tokens.append(&enum_key);
            tokens.append(": '");
            tokens.append(&variant_name);
            tokens.append("',");
            tokens.push();
        }

        tokens.append("} as const;");

        GencoTemplateEngine::format_typescript(&tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ir::*;
    use crate::languages::typescript::TypeScriptBackend;

    fn create_choice_property() -> Property {
        Property {
            name: "value[x]".to_string(),
            path: "Observation.value[x]".to_string(),
            property_type: PropertyType::Choice {
                types: vec![
                    "string".to_string(),
                    "boolean".to_string(),
                    "integer".to_string(),
                    "Quantity".to_string(),
                ],
            },
            cardinality: CardinalityRange::optional(),
            is_choice: true,
            choice_types: vec![
                "string".to_string(),
                "boolean".to_string(),
                "integer".to_string(),
                "Quantity".to_string(),
            ],
            is_modifier: false,
            is_summary: true,
            binding: None,
            constraints: vec![],
            short_description: "Actual result".to_string(),
            definition: "The information determined as a result of making the observation"
                .to_string(),
            comments: None,
            examples: vec![],
        }
    }

    #[test]
    fn test_generate_choice_union() {
        let backend = TypeScriptBackend::new();
        let generator = ChoiceElementGenerator::new(backend);
        let property = create_choice_property();

        let result = generator.generate_choice_union(&property, "Observation");
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("export type ObservationValueChoice"));
        assert!(output.contains("valueString: string"));
        assert!(output.contains("valueBoolean: boolean"));
        assert!(output.contains("valueInteger: number"));
        assert!(output.contains("valueQuantity: Quantity"));
    }

    #[test]
    fn test_generate_choice_type_guards() {
        let backend = TypeScriptBackend::new();
        let generator = ChoiceElementGenerator::new(backend);
        let property = create_choice_property();

        let result = generator.generate_choice_type_guards(&property, "Observation");
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("export function hasValueString"));
        assert!(output.contains("export function hasValueBoolean"));
        assert!(output.contains("export function hasValueInteger"));
        assert!(output.contains("'valueString' in obj"));
    }

    #[test]
    fn test_generate_choice_value_extractor() {
        let backend = TypeScriptBackend::new();
        let generator = ChoiceElementGenerator::new(backend);
        let property = create_choice_property();

        let result = generator.generate_choice_value_extractor(&property, "Observation");
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("export function getObservationValueChoiceValue"));
        assert!(output.contains("if (hasValueString(choice))"));
        assert!(output.contains("return choice.valueString"));
    }

    #[test]
    fn test_generate_inline_choice_type() {
        let backend = TypeScriptBackend::new();
        let generator = ChoiceElementGenerator::new(backend);
        let property = create_choice_property();

        let result = generator.generate_inline_choice_type(&property);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("{ valueString: string }"));
        assert!(output.contains("{ valueBoolean: boolean }"));
        assert!(output.contains("|"));
    }

    #[test]
    fn test_generate_choice_enum() {
        let backend = TypeScriptBackend::new();
        let generator = ChoiceElementGenerator::new(backend);
        let property = create_choice_property();

        let result = generator.generate_choice_enum(&property, "Observation");
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("export const ObservationValueType"));
        assert!(output.contains("STRING: 'valueString'"));
        assert!(output.contains("BOOLEAN: 'valueBoolean'"));
    }

    #[test]
    fn test_non_choice_property_error() {
        let backend = TypeScriptBackend::new();
        let generator = ChoiceElementGenerator::new(backend);

        let non_choice = Property {
            name: "status".to_string(),
            path: "Observation.status".to_string(),
            property_type: PropertyType::Primitive { type_name: "code".to_string() },
            cardinality: CardinalityRange::required(),
            is_choice: false,
            choice_types: vec![],
            is_modifier: false,
            is_summary: true,
            binding: None,
            constraints: vec![],
            short_description: "Status".to_string(),
            definition: "Status".to_string(),
            comments: None,
            examples: vec![],
        };

        let result = generator.generate_choice_union(&non_choice, "Observation");
        assert!(result.is_err());
    }
}
