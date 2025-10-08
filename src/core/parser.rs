//! FHIR StructureDefinition parser
//!
//! Converts FHIR StructureDefinition JSON into IR types

use crate::core::ir::*;
use crate::core::{Error, Result};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, warn};

/// Parser for FHIR StructureDefinitions
pub struct StructureDefinitionParser {
    /// Cache of parsed structures
    cache: HashMap<String, ParsedStructure>,
}

impl StructureDefinitionParser {
    /// Create a new parser
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Parse a StructureDefinition JSON into IR
    ///
    /// # Arguments
    ///
    /// * `json` - StructureDefinition as JSON value
    ///
    /// # Returns
    ///
    /// Parsed structure ready for IR conversion
    pub fn parse(&mut self, json: &Value) -> Result<ParsedStructure> {
        // Validate it's a StructureDefinition
        let resource_type = json
            .get("resourceType")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Parser("Missing resourceType".to_string()))?;

        if resource_type != "StructureDefinition" {
            return Err(Error::Parser(format!(
                "Expected StructureDefinition, got {}",
                resource_type
            )));
        }

        // Extract basic metadata
        let url = json
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Parser("Missing url".to_string()))?
            .to_string();

        let name = json
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Parser("Missing name".to_string()))?
            .to_string();

        let kind = json
            .get("kind")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Parser("Missing kind".to_string()))?;

        let structure_kind = match kind {
            "resource" => StructureKind::Resource,
            "complex-type" => StructureKind::ComplexType,
            "primitive-type" => StructureKind::PrimitiveType,
            "logical" => StructureKind::LogicalModel,
            _ => {
                return Err(Error::Parser(format!("Unknown kind: {}", kind)));
            }
        };

        let base_definition = json
            .get("baseDefinition")
            .and_then(|v| v.as_str())
            .map(String::from);

        let is_abstract = json
            .get("abstract")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // Check if we have snapshot or differential
        let snapshot = json.get("snapshot");
        let differential = json.get("differential");

        let (elements, is_differential) = if let Some(snap) = snapshot {
            let elements = self.parse_element_definitions(snap)?;
            (elements, false)
        } else if let Some(diff) = differential {
            let elements = self.parse_element_definitions(diff)?;
            (elements, true)
        } else {
            return Err(Error::Parser(
                "Missing both snapshot and differential".to_string(),
            ));
        };

        debug!(
            "Parsed {} with {} elements (differential: {})",
            name,
            elements.len(),
            is_differential
        );

        let parsed = ParsedStructure {
            url,
            name,
            kind: structure_kind,
            base_definition,
            is_abstract,
            elements,
            differential: is_differential,
        };

        // Cache the result
        self.cache.insert(parsed.url.clone(), parsed.clone());

        Ok(parsed)
    }

    /// Parse element definitions from snapshot or differential
    fn parse_element_definitions(&self, container: &Value) -> Result<Vec<ElementDefinition>> {
        let elements = container
            .get("element")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::Parser("Missing element array".to_string()))?;

        elements
            .iter()
            .map(|elem| self.parse_element(elem))
            .collect()
    }

    /// Parse a single element definition
    fn parse_element(&self, elem: &Value) -> Result<ElementDefinition> {
        let path = elem
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Parser("Element missing path".to_string()))?
            .to_string();

        let short = elem.get("short").and_then(|v| v.as_str()).map(String::from);

        let definition = elem
            .get("definition")
            .and_then(|v| v.as_str())
            .map(String::from);

        let min = elem
            .get("min")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;

        let max = elem
            .get("max")
            .and_then(|v| v.as_str())
            .map(|s| {
                if s == "*" {
                    Cardinality::Unbounded
                } else {
                    s.parse::<u32>()
                        .map(Cardinality::Finite)
                        .unwrap_or(Cardinality::Finite(1))
                }
            })
            .unwrap_or(Cardinality::Finite(1));

        // Parse types
        let types = if let Some(type_array) = elem.get("type").and_then(|v| v.as_array()) {
            type_array
                .iter()
                .filter_map(|t| self.parse_element_type(t).ok())
                .collect()
        } else {
            vec![]
        };

        // Parse binding
        let binding = elem
            .get("binding")
            .and_then(|b| self.parse_binding(b).ok());

        // Parse constraints
        let constraints = if let Some(constraint_array) =
            elem.get("constraint").and_then(|v| v.as_array())
        {
            constraint_array
                .iter()
                .filter_map(|c| self.parse_constraint(c).ok())
                .collect()
        } else {
            vec![]
        };

        Ok(ElementDefinition {
            path,
            short,
            definition,
            min,
            max,
            types,
            binding,
            constraints,
        })
    }

    /// Parse element type information
    fn parse_element_type(&self, type_obj: &Value) -> Result<ElementType> {
        let code = type_obj
            .get("code")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Parser("Type missing code".to_string()))?
            .to_string();

        // Handle Reference types with targetProfile
        let target_profiles = if code == "Reference" {
            type_obj
                .get("targetProfile")
                .and_then(|v| match v {
                    Value::String(s) => Some(vec![s.clone()]),
                    Value::Array(arr) => Some(
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect(),
                    ),
                    _ => None,
                })
                .unwrap_or_default()
        } else {
            vec![]
        };

        Ok(ElementType {
            code,
            target_profiles,
        })
    }

    /// Parse value set binding
    fn parse_binding(&self, binding: &Value) -> Result<Binding> {
        let strength = binding
            .get("strength")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Parser("Binding missing strength".to_string()))?;

        let value_set = binding
            .get("valueSet")
            .and_then(|v| v.as_str())
            .map(String::from);

        let description = binding
            .get("description")
            .and_then(|v| v.as_str())
            .map(String::from);

        Ok(Binding {
            strength: strength.to_string(),
            value_set,
            description,
        })
    }

    /// Parse constraint/invariant
    fn parse_constraint(&self, constraint: &Value) -> Result<Constraint> {
        let key = constraint
            .get("key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Parser("Constraint missing key".to_string()))?
            .to_string();

        let severity = constraint
            .get("severity")
            .and_then(|v| v.as_str())
            .unwrap_or("error")
            .to_string();

        let human = constraint
            .get("human")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Parser("Constraint missing human description".to_string()))?
            .to_string();

        let expression = constraint
            .get("expression")
            .and_then(|v| v.as_str())
            .map(String::from);

        let xpath = constraint
            .get("xpath")
            .and_then(|v| v.as_str())
            .map(String::from);

        Ok(Constraint {
            key,
            severity,
            human,
            expression,
            xpath,
        })
    }

    /// Convert parsed structure to IR ResourceType
    pub fn to_resource_type(&self, parsed: &ParsedStructure) -> Result<ResourceType> {
        if parsed.kind != StructureKind::Resource {
            return Err(Error::Parser(format!(
                "{} is not a resource",
                parsed.name
            )));
        }

        // Convert elements to properties
        let properties = self.elements_to_properties(&parsed.elements, &parsed.name)?;

        Ok(ResourceType {
            name: parsed.name.clone(),
            base: parsed.base_definition.clone(),
            properties,
            search_parameters: vec![], // Will be populated by resolver
            documentation: Documentation {
                short: format!("FHIR {} Resource", parsed.name),
                definition: String::new(),
                comments: None,
                requirements: None,
                usage_notes: vec![],
                url: Some(parsed.url.clone()),
            },
            url: parsed.url.clone(),
            is_abstract: parsed.is_abstract,
        })
    }

    /// Convert parsed structure to IR DataType
    pub fn to_datatype(&self, parsed: &ParsedStructure) -> Result<DataType> {
        if parsed.kind != StructureKind::ComplexType {
            return Err(Error::Parser(format!(
                "{} is not a complex type",
                parsed.name
            )));
        }

        let properties = self.elements_to_properties(&parsed.elements, &parsed.name)?;

        Ok(DataType {
            name: parsed.name.clone(),
            base: parsed.base_definition.clone(),
            properties,
            documentation: Documentation {
                short: format!("FHIR {} DataType", parsed.name),
                definition: String::new(),
                comments: None,
                requirements: None,
                usage_notes: vec![],
                url: Some(parsed.url.clone()),
            },
            url: parsed.url.clone(),
            is_abstract: parsed.is_abstract,
        })
    }

    /// Convert element definitions to IR properties
    fn elements_to_properties(
        &self,
        elements: &[ElementDefinition],
        type_name: &str,
    ) -> Result<Vec<Property>> {
        let root_path = type_name;
        let mut properties = Vec::new();

        for elem in elements {
            // Skip the root element itself
            if elem.path == root_path {
                continue;
            }

            // Extract property name from path (e.g., "Patient.name" -> "name")
            let name = elem
                .path
                .strip_prefix(&format!("{}.", root_path))
                .unwrap_or(&elem.path)
                .split('.')
                .next()
                .unwrap_or(&elem.path)
                .to_string();

            // Skip if we already have this property (backbone elements)
            if properties.iter().any(|p: &Property| p.name == name) {
                continue;
            }

            // Determine if this is a choice element
            let is_choice = name.ends_with("[x]");
            let base_name = if is_choice {
                name.trim_end_matches("[x]").to_string()
            } else {
                name.clone()
            };

            // Get choice types if applicable
            let choice_types = if is_choice {
                elem.types.iter().map(|t| t.code.clone()).collect()
            } else {
                vec![]
            };

            // Determine property type
            let property_type = if elem.types.is_empty() {
                // Backbone element
                PropertyType::BackboneElement { properties: vec![] }
            } else if elem.types.len() == 1 {
                let elem_type = &elem.types[0];
                if elem_type.code == "Reference" && !elem_type.target_profiles.is_empty() {
                    PropertyType::Reference {
                        target_types: elem_type
                            .target_profiles
                            .iter()
                            .filter_map(|url| url.split('/').last().map(String::from))
                            .collect(),
                    }
                } else if Self::is_primitive(&elem_type.code) {
                    PropertyType::Primitive {
                        type_name: elem_type.code.clone(),
                    }
                } else {
                    PropertyType::Complex {
                        type_name: elem_type.code.clone(),
                    }
                }
            } else {
                // Multiple types = choice
                PropertyType::Choice {
                    types: elem.types.iter().map(|t| t.code.clone()).collect(),
                }
            };

            // Convert cardinality
            let cardinality = CardinalityRange {
                min: elem.min,
                max: match elem.max {
                    Cardinality::Finite(n) => Some(n),
                    Cardinality::Unbounded => None,
                },
            };

            properties.push(Property {
                name: base_name,
                path: elem.path.clone(),
                property_type,
                cardinality,
                is_choice,
                choice_types,
                is_modifier: false, // TODO: Extract from element
                is_summary: false,  // TODO: Extract from element
                binding: None,      // TODO: Convert binding
                constraints: vec![], // TODO: Convert constraints
                short_description: elem.short.clone().unwrap_or_default(),
                definition: elem.definition.clone().unwrap_or_default(),
                comments: None,
                examples: vec![],
            });
        }

        Ok(properties)
    }

    /// Check if a type code represents a primitive
    fn is_primitive(type_code: &str) -> bool {
        matches!(
            type_code,
            "base64Binary"
                | "boolean"
                | "canonical"
                | "code"
                | "date"
                | "dateTime"
                | "decimal"
                | "id"
                | "instant"
                | "integer"
                | "integer64"
                | "markdown"
                | "oid"
                | "positiveInt"
                | "string"
                | "time"
                | "unsignedInt"
                | "uri"
                | "url"
                | "uuid"
                | "xhtml"
        )
    }
}

impl Default for StructureDefinitionParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Parsed structure (intermediate form before IR conversion)
#[derive(Debug, Clone)]
pub struct ParsedStructure {
    pub url: String,
    pub name: String,
    pub kind: StructureKind,
    pub base_definition: Option<String>,
    pub is_abstract: bool,
    pub elements: Vec<ElementDefinition>,
    pub differential: bool,
}

/// Structure kind
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructureKind {
    Resource,
    ComplexType,
    PrimitiveType,
    LogicalModel,
}

/// Element definition from StructureDefinition
#[derive(Debug, Clone)]
pub struct ElementDefinition {
    pub path: String,
    pub short: Option<String>,
    pub definition: Option<String>,
    pub min: u32,
    pub max: Cardinality,
    pub types: Vec<ElementType>,
    pub binding: Option<Binding>,
    pub constraints: Vec<Constraint>,
}

/// Cardinality max value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cardinality {
    Finite(u32),
    Unbounded,
}

/// Element type information
#[derive(Debug, Clone)]
pub struct ElementType {
    pub code: String,
    pub target_profiles: Vec<String>,
}

/// Value set binding
#[derive(Debug, Clone)]
pub struct Binding {
    pub strength: String,
    pub value_set: Option<String>,
    pub description: Option<String>,
}

/// Constraint/invariant
#[derive(Debug, Clone)]
pub struct Constraint {
    pub key: String,
    pub severity: String,
    pub human: String,
    pub expression: Option<String>,
    pub xpath: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_primitive() {
        assert!(StructureDefinitionParser::is_primitive("string"));
        assert!(StructureDefinitionParser::is_primitive("boolean"));
        assert!(StructureDefinitionParser::is_primitive("dateTime"));
        assert!(!StructureDefinitionParser::is_primitive("HumanName"));
        assert!(!StructureDefinitionParser::is_primitive("Reference"));
    }

    #[test]
    fn test_parse_minimal_sd() {
        let json = serde_json::json!({
            "resourceType": "StructureDefinition",
            "url": "http://example.com/Test",
            "name": "Test",
            "kind": "resource",
            "snapshot": {
                "element": [
                    {
                        "path": "Test",
                        "min": 0,
                        "max": "*"
                    }
                ]
            }
        });

        let mut parser = StructureDefinitionParser::new();
        let result = parser.parse(&json);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.name, "Test");
        assert_eq!(parsed.kind, StructureKind::Resource);
    }

    #[test]
    fn test_parse_with_properties() {
        let json = serde_json::json!({
            "resourceType": "StructureDefinition",
            "url": "http://example.com/TestResource",
            "name": "TestResource",
            "kind": "resource",
            "snapshot": {
                "element": [
                    {
                        "path": "TestResource",
                        "min": 0,
                        "max": "*"
                    },
                    {
                        "path": "TestResource.active",
                        "short": "Active flag",
                        "min": 0,
                        "max": "1",
                        "type": [{ "code": "boolean" }]
                    },
                    {
                        "path": "TestResource.name",
                        "short": "Name field",
                        "min": 1,
                        "max": "1",
                        "type": [{ "code": "string" }]
                    }
                ]
            }
        });

        let mut parser = StructureDefinitionParser::new();
        let parsed = parser.parse(&json).unwrap();

        assert_eq!(parsed.elements.len(), 3);

        let resource_type = parser.to_resource_type(&parsed).unwrap();
        assert_eq!(resource_type.name, "TestResource");
        assert_eq!(resource_type.properties.len(), 2);

        let active_prop = resource_type.properties.iter().find(|p| p.name == "active").unwrap();
        assert_eq!(active_prop.cardinality.min, 0);
        assert_eq!(active_prop.cardinality.max, Some(1));
    }

    #[test]
    fn test_parse_choice_element() {
        let json = serde_json::json!({
            "resourceType": "StructureDefinition",
            "url": "http://example.com/TestChoice",
            "name": "TestChoice",
            "kind": "resource",
            "snapshot": {
                "element": [
                    {
                        "path": "TestChoice",
                        "min": 0,
                        "max": "*"
                    },
                    {
                        "path": "TestChoice.value[x]",
                        "short": "Value",
                        "min": 0,
                        "max": "1",
                        "type": [
                            { "code": "string" },
                            { "code": "integer" },
                            { "code": "boolean" }
                        ]
                    }
                ]
            }
        });

        let mut parser = StructureDefinitionParser::new();
        let parsed = parser.parse(&json).unwrap();
        let resource_type = parser.to_resource_type(&parsed).unwrap();

        let value_prop = resource_type.properties.iter().find(|p| p.name == "value").unwrap();
        assert!(value_prop.is_choice);
        assert_eq!(value_prop.choice_types.len(), 3);

        if let PropertyType::Choice { types } = &value_prop.property_type {
            assert_eq!(types.len(), 3);
            assert!(types.contains(&"string".to_string()));
        } else {
            panic!("Expected Choice type");
        }
    }

    #[test]
    fn test_parse_reference_with_targets() {
        let json = serde_json::json!({
            "resourceType": "StructureDefinition",
            "url": "http://example.com/TestRef",
            "name": "TestRef",
            "kind": "resource",
            "snapshot": {
                "element": [
                    {
                        "path": "TestRef",
                        "min": 0,
                        "max": "*"
                    },
                    {
                        "path": "TestRef.subject",
                        "short": "Subject reference",
                        "min": 1,
                        "max": "1",
                        "type": [{
                            "code": "Reference",
                            "targetProfile": [
                                "http://hl7.org/fhir/StructureDefinition/Patient",
                                "http://hl7.org/fhir/StructureDefinition/Group"
                            ]
                        }]
                    }
                ]
            }
        });

        let mut parser = StructureDefinitionParser::new();
        let parsed = parser.parse(&json).unwrap();
        let resource_type = parser.to_resource_type(&parsed).unwrap();

        let subject_prop = resource_type.properties.iter().find(|p| p.name == "subject").unwrap();

        if let PropertyType::Reference { target_types } = &subject_prop.property_type {
            assert_eq!(target_types.len(), 2);
            assert!(target_types.contains(&"Patient".to_string()));
            assert!(target_types.contains(&"Group".to_string()));
        } else {
            panic!("Expected Reference type");
        }
    }

    #[test]
    fn test_cardinality_parsing() {
        let json = serde_json::json!({
            "resourceType": "StructureDefinition",
            "url": "http://example.com/TestCard",
            "name": "TestCard",
            "kind": "resource",
            "snapshot": {
                "element": [
                    {
                        "path": "TestCard",
                        "min": 0,
                        "max": "*"
                    },
                    {
                        "path": "TestCard.required",
                        "min": 1,
                        "max": "1",
                        "type": [{ "code": "string" }]
                    },
                    {
                        "path": "TestCard.optional",
                        "min": 0,
                        "max": "1",
                        "type": [{ "code": "string" }]
                    },
                    {
                        "path": "TestCard.array",
                        "min": 0,
                        "max": "*",
                        "type": [{ "code": "string" }]
                    }
                ]
            }
        });

        let mut parser = StructureDefinitionParser::new();
        let parsed = parser.parse(&json).unwrap();
        let resource_type = parser.to_resource_type(&parsed).unwrap();

        let required = resource_type.properties.iter().find(|p| p.name == "required").unwrap();
        assert_eq!(required.cardinality.min, 1);
        assert_eq!(required.cardinality.max, Some(1));

        let optional = resource_type.properties.iter().find(|p| p.name == "optional").unwrap();
        assert_eq!(optional.cardinality.min, 0);
        assert_eq!(optional.cardinality.max, Some(1));

        let array = resource_type.properties.iter().find(|p| p.name == "array").unwrap();
        assert_eq!(array.cardinality.min, 0);
        assert_eq!(array.cardinality.max, None); // unbounded
    }
}
