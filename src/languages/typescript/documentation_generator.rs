use crate::core::ir::{Documentation, Example, Property};

/// Enhanced documentation generator for TypeScript with JSDoc and TypeDoc support
pub struct DocumentationGenerator;

impl DocumentationGenerator {
    /// Generate comprehensive JSDoc for a property
    ///
    /// Includes:
    /// - Short description
    /// - Full definition
    /// - Requirements
    /// - Usage notes
    /// - Examples
    /// - FHIR spec link
    pub fn generate_property_doc(property: &Property, resource_url: Option<&str>) -> Vec<String> {
        let mut lines = vec!["/**".to_string()];

        // Short description
        if !property.short_description.is_empty() {
            lines.push(format!(" * {}", property.short_description));
        }

        // Full definition (if different from short)
        if !property.definition.is_empty() && property.definition != property.short_description {
            lines.push(" *".to_string());
            for line in Self::wrap_text(&property.definition, 90) {
                lines.push(format!(" * {}", line));
            }
        }

        // Comments
        if let Some(comments) = &property.comments {
            lines.push(" *".to_string());
            for line in Self::wrap_text(comments, 90) {
                lines.push(format!(" * {}", line));
            }
        }

        // Cardinality information
        lines.push(" *".to_string());
        let cardinality_desc = Self::describe_cardinality(property);
        lines.push(format!(" * @cardinality {}", cardinality_desc));

        // Modifier flag
        if property.is_modifier {
            lines.push(" * @modifier This element is a modifier element".to_string());
        }

        // Summary flag
        if property.is_summary {
            lines.push(" * @summary This element is a summary element".to_string());
        }

        // Examples
        if !property.examples.is_empty() {
            lines.push(" *".to_string());
            for example in &property.examples {
                Self::add_example(&mut lines, example);
            }
        }

        // FHIR spec link
        if let Some(url) = resource_url {
            lines.push(" *".to_string());
            lines.push(format!(" * @see {{@link {}#{} | FHIR Specification}}", url, property.name));
        }

        lines.push(" */".to_string());
        lines
    }

    /// Generate JSDoc for a resource or datatype
    pub fn generate_type_doc(doc: &Documentation) -> Vec<String> {
        let mut lines = vec!["/**".to_string()];

        // Short description
        if !doc.short.is_empty() {
            lines.push(format!(" * {}", doc.short));
        }

        // Full definition
        if !doc.definition.is_empty() && doc.definition != doc.short {
            lines.push(" *".to_string());
            for line in Self::wrap_text(&doc.definition, 90) {
                lines.push(format!(" * {}", line));
            }
        }

        // Comments
        if let Some(comments) = &doc.comments {
            lines.push(" *".to_string());
            for line in Self::wrap_text(comments, 90) {
                lines.push(format!(" * {}", line));
            }
        }

        // Requirements
        if let Some(requirements) = &doc.requirements {
            lines.push(" *".to_string());
            lines.push(" * **Requirements:**".to_string());
            for line in Self::wrap_text(requirements, 90) {
                lines.push(format!(" * {}", line));
            }
        }

        // Usage notes
        if !doc.usage_notes.is_empty() {
            lines.push(" *".to_string());
            lines.push(" * **Usage Notes:**".to_string());
            for note in &doc.usage_notes {
                for line in Self::wrap_text(note, 90) {
                    lines.push(format!(" * - {}", line));
                }
            }
        }

        // FHIR spec link
        if let Some(url) = &doc.url {
            lines.push(" *".to_string());
            lines.push(format!(" * @see {{@link {} | FHIR Specification}}", url));
        }

        lines.push(" */".to_string());
        lines
    }

    /// Generate minimal JSDoc for simple cases
    pub fn generate_simple_doc(description: &str) -> Vec<String> {
        vec![
            "/**".to_string(),
            format!(" * {}", description),
            " */".to_string(),
        ]
    }

    /// Describe cardinality in human-readable format
    fn describe_cardinality(property: &Property) -> String {
        let min = property.cardinality.min;
        let max = property.cardinality.max;

        match (min, max) {
            (0, Some(1)) => "0..1 (optional)".to_string(),
            (0, None) => "0..* (optional, array)".to_string(),
            (0, Some(max)) => format!("0..{} (optional, max {} items)", max, max),
            (1, Some(1)) => "1..1 (required)".to_string(),
            (1, None) => "1..* (required, array)".to_string(),
            (1, Some(max)) => format!("1..{} (required, max {} items)", max, max),
            (min, None) => format!("{}..* (array)", min),
            (min, Some(max)) => format!("{}..{}", min, max),
        }
    }

    /// Add example to JSDoc
    fn add_example(lines: &mut Vec<String>, example: &Example) {
        lines.push(format!(" * @example {}", example.label));
        lines.push(" * ```typescript".to_string());

        // Format the example value as TypeScript
        if let Ok(formatted) = serde_json::to_string_pretty(&example.value) {
            for line in formatted.lines() {
                lines.push(format!(" * {}", line));
            }
        } else {
            lines.push(format!(" * {}", example.value));
        }

        lines.push(" * ```".to_string());
    }

    /// Wrap text to a maximum line width
    fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + word.len() < max_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    /// Generate JSDoc for a choice element
    pub fn generate_choice_doc(base_name: &str, types: &[String]) -> Vec<String> {
        let mut lines = vec!["/**".to_string()];

        lines.push(format!(" * Choice element: {}", base_name));
        lines.push(" *".to_string());
        lines.push(" * This is a FHIR choice element represented as a discriminated union.".to_string());
        lines.push(" * Only one of the following properties should be present:".to_string());

        for type_name in types {
            lines.push(format!(" * - {}{}", base_name, type_name.to_pascal_case()));
        }

        lines.push(" */".to_string());
        lines
    }

    /// Generate JSDoc for a validation function
    pub fn generate_validation_doc(resource_name: &str) -> Vec<String> {
        vec![
            "/**".to_string(),
            format!(" * Validate a {} resource", resource_name),
            " *".to_string(),
            " * Performs runtime validation checking:".to_string(),
            " * - Required fields are present".to_string(),
            " * - Cardinality constraints are met".to_string(),
            " * - Resource type is correct".to_string(),
            " *".to_string(),
            format!(" * @param resource - The {} resource to validate", resource_name),
            " * @returns {{ValidationResult}} Validation result with any errors or warnings".to_string(),
            " */".to_string(),
        ]
    }

    /// Generate JSDoc for a helper function
    pub fn generate_helper_doc(description: &str, params: &[(&str, &str)], returns: &str) -> Vec<String> {
        let mut lines = vec!["/**".to_string()];

        lines.push(format!(" * {}", description));
        lines.push(" *".to_string());

        for (param_name, param_desc) in params {
            lines.push(format!(" * @param {} - {}", param_name, param_desc));
        }

        lines.push(format!(" * @returns {}", returns));
        lines.push(" */".to_string());

        lines
    }
}

// Helper trait for case conversion
trait ToPascalCase {
    fn to_pascal_case(&self) -> String;
}

impl ToPascalCase for str {
    fn to_pascal_case(&self) -> String {
        heck::ToPascalCase::to_pascal_case(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ir::CardinalityRange;

    #[test]
    fn test_describe_cardinality_optional() {
        let property = Property {
            name: "test".to_string(),
            path: "Test.test".to_string(),
            property_type: crate::core::ir::PropertyType::Primitive {
                type_name: "string".to_string(),
            },
            cardinality: CardinalityRange::optional(),
            is_choice: false,
            choice_types: vec![],
            is_modifier: false,
            is_summary: false,
            binding: None,
            constraints: vec![],
            short_description: "Test".to_string(),
            definition: "Test".to_string(),
            comments: None,
            examples: vec![],
        };

        let desc = DocumentationGenerator::describe_cardinality(&property);
        assert_eq!(desc, "0..1 (optional)");
    }

    #[test]
    fn test_describe_cardinality_required() {
        let property = Property {
            name: "test".to_string(),
            path: "Test.test".to_string(),
            property_type: crate::core::ir::PropertyType::Primitive {
                type_name: "string".to_string(),
            },
            cardinality: CardinalityRange::required(),
            is_choice: false,
            choice_types: vec![],
            is_modifier: false,
            is_summary: false,
            binding: None,
            constraints: vec![],
            short_description: "Test".to_string(),
            definition: "Test".to_string(),
            comments: None,
            examples: vec![],
        };

        let desc = DocumentationGenerator::describe_cardinality(&property);
        assert_eq!(desc, "1..1 (required)");
    }

    #[test]
    fn test_describe_cardinality_array() {
        let property = Property {
            name: "test".to_string(),
            path: "Test.test".to_string(),
            property_type: crate::core::ir::PropertyType::Primitive {
                type_name: "string".to_string(),
            },
            cardinality: CardinalityRange::optional_array(),
            is_choice: false,
            choice_types: vec![],
            is_modifier: false,
            is_summary: false,
            binding: None,
            constraints: vec![],
            short_description: "Test".to_string(),
            definition: "Test".to_string(),
            comments: None,
            examples: vec![],
        };

        let desc = DocumentationGenerator::describe_cardinality(&property);
        assert_eq!(desc, "0..* (optional, array)");
    }

    #[test]
    fn test_wrap_text() {
        let text = "This is a very long text that should be wrapped to multiple lines when it exceeds the maximum width";
        let lines = DocumentationGenerator::wrap_text(text, 30);

        assert!(lines.len() > 1);
        for line in &lines {
            assert!(line.len() <= 30 || line.split_whitespace().count() == 1);
        }
    }

    #[test]
    fn test_generate_simple_doc() {
        let lines = DocumentationGenerator::generate_simple_doc("Test description");

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "/**");
        assert_eq!(lines[1], " * Test description");
        assert_eq!(lines[2], " */");
    }
}
