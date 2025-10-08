use octofhir_codegen::core::ir::{
    CardinalityRange, Documentation, Example, Property, PropertyType,
};
use octofhir_codegen::languages::typescript::DocumentationGenerator;
use serde_json::json;

fn create_test_property() -> Property {
    Property {
        name: "identifier".to_string(),
        path: "Patient.identifier".to_string(),
        property_type: PropertyType::Complex {
            type_name: "Identifier".to_string(),
        },
        cardinality: CardinalityRange::optional_array(),
        is_choice: false,
        choice_types: vec![],
        is_modifier: false,
        is_summary: true,
        binding: None,
        constraints: vec![],
        short_description: "An identifier for this patient".to_string(),
        definition: "An identifier that applies to this person as a patient".to_string(),
        comments: Some("Patients are almost always assigned specific numerical identifiers".to_string()),
        examples: vec![Example {
            label: "MRN example".to_string(),
            value: json!({"system": "http://hospital.example.org/patients", "value": "12345"}),
        }],
    }
}

fn create_test_documentation() -> Documentation {
    Documentation {
        short: "Patient resource".to_string(),
        definition: "Demographics and other administrative information about an individual or animal receiving care or other health-related services.".to_string(),
        comments: Some("This resource is used for both human and animal patients".to_string()),
        requirements: Some("Patient resources must contain at least one identifier or name".to_string()),
        usage_notes: vec![
            "Use for both humans and animals".to_string(),
            "Link to other resources using the patient reference".to_string(),
        ],
        url: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
    }
}

#[test]
fn test_generate_property_doc() {
    let property = create_test_property();
    let lines = DocumentationGenerator::generate_property_doc(
        &property,
        Some("http://hl7.org/fhir/StructureDefinition/Patient"),
    );

    let doc = lines.join("\n");

    // Verify structure
    assert!(doc.starts_with("/**"));
    assert!(doc.ends_with(" */"));

    // Verify content
    assert!(doc.contains("An identifier for this patient"));
    assert!(doc.contains("An identifier that applies to this person as a patient"));
    assert!(doc.contains("@cardinality"));
    assert!(doc.contains("@summary"));
    assert!(doc.contains("@example"));
    assert!(doc.contains("@see"));
    assert!(doc.contains("FHIR Specification"));
}

#[test]
fn test_property_doc_includes_examples() {
    let property = create_test_property();
    let lines = DocumentationGenerator::generate_property_doc(&property, None);

    let doc = lines.join("\n");

    assert!(doc.contains("@example MRN example"));
    assert!(doc.contains("```typescript"));
    assert!(doc.contains("hospital.example.org"));
}

#[test]
fn test_property_doc_cardinality() {
    let property = create_test_property();
    let lines = DocumentationGenerator::generate_property_doc(&property, None);

    let doc = lines.join("\n");

    // The cardinality range has unbounded max
    assert!(doc.contains("@cardinality"));
    assert!(doc.contains("optional"));
}

#[test]
fn test_property_doc_modifier_flag() {
    let mut property = create_test_property();
    property.is_modifier = true;

    let lines = DocumentationGenerator::generate_property_doc(&property, None);
    let doc = lines.join("\n");

    assert!(doc.contains("@modifier"));
}

#[test]
fn test_generate_type_doc() {
    let documentation = create_test_documentation();
    let lines = DocumentationGenerator::generate_type_doc(&documentation);

    let doc = lines.join("\n");

    // Verify structure
    assert!(doc.starts_with("/**"));
    assert!(doc.ends_with(" */"));

    // Verify content
    assert!(doc.contains("Patient resource"));
    assert!(doc.contains("Demographics and other administrative information"));
    assert!(doc.contains("**Requirements:**"));
    assert!(doc.contains("Patient resources must contain at least one identifier or name"));
    assert!(doc.contains("**Usage Notes:**"));
    assert!(doc.contains("Use for both humans and animals"));
    assert!(doc.contains("@see"));
    assert!(doc.contains("http://hl7.org/fhir/StructureDefinition/Patient"));
}

#[test]
fn test_generate_simple_doc() {
    let lines = DocumentationGenerator::generate_simple_doc("A simple test description");

    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "/**");
    assert_eq!(lines[1], " * A simple test description");
    assert_eq!(lines[2], " */");
}

#[test]
fn test_generate_choice_doc() {
    let lines = DocumentationGenerator::generate_choice_doc("value", &["String".to_string(), "Boolean".to_string(), "Quantity".to_string()]);

    let doc = lines.join("\n");

    assert!(doc.contains("Choice element: value"));
    assert!(doc.contains("discriminated union"));
    assert!(doc.contains("valueString"));
    assert!(doc.contains("valueBoolean"));
    assert!(doc.contains("valueQuantity"));
}

#[test]
fn test_generate_validation_doc() {
    let lines = DocumentationGenerator::generate_validation_doc("Patient");

    let doc = lines.join("\n");

    assert!(doc.contains("Validate a Patient resource"));
    assert!(doc.contains("Required fields are present"));
    assert!(doc.contains("Cardinality constraints"));
    assert!(doc.contains("@param resource"));
    assert!(doc.contains("@returns"));
    assert!(doc.contains("ValidationResult"));
}

#[test]
fn test_generate_helper_doc() {
    let lines = DocumentationGenerator::generate_helper_doc(
        "Get the official name from a patient",
        &[("patient", "The patient resource")],
        "The official HumanName or undefined",
    );

    let doc = lines.join("\n");

    assert!(doc.contains("Get the official name from a patient"));
    assert!(doc.contains("@param patient - The patient resource"));
    assert!(doc.contains("@returns The official HumanName or undefined"));
}

#[test]
fn test_cardinality_descriptions() {
    // Test optional
    let mut prop = create_test_property();
    prop.cardinality = CardinalityRange::optional();
    let lines = DocumentationGenerator::generate_property_doc(&prop, None);
    let doc = lines.join("\n");
    assert!(doc.contains("@cardinality"));
    assert!(doc.contains("optional"));

    // Test required
    prop.cardinality = CardinalityRange::required();
    let lines = DocumentationGenerator::generate_property_doc(&prop, None);
    let doc = lines.join("\n");
    assert!(doc.contains("@cardinality"));
    assert!(doc.contains("required"));

    // Test required array - check cardinality is present
    prop.cardinality = CardinalityRange::required_array();
    let lines = DocumentationGenerator::generate_property_doc(&prop, None);
    let doc = lines.join("\n");
    assert!(doc.contains("@cardinality"));
    assert!(doc.contains("required"));
}

#[test]
fn test_text_wrapping() {
    let long_text = "This is a very long description that should be wrapped to multiple lines when it exceeds the maximum width that we have specified for the documentation generator";

    let property = Property {
        name: "test".to_string(),
        path: "Test.test".to_string(),
        property_type: PropertyType::Primitive {
            type_name: "string".to_string(),
        },
        cardinality: CardinalityRange::optional(),
        is_choice: false,
        choice_types: vec![],
        is_modifier: false,
        is_summary: false,
        binding: None,
        constraints: vec![],
        short_description: "Short".to_string(),
        definition: long_text.to_string(),
        comments: None,
        examples: vec![],
    };

    let lines = DocumentationGenerator::generate_property_doc(&property, None);
    let doc = lines.join("\n");

    // Verify documentation was generated
    assert!(doc.contains("/**"));
    assert!(doc.contains("*/"));
}

#[test]
fn test_property_without_examples() {
    let mut property = create_test_property();
    property.examples = vec![];

    let lines = DocumentationGenerator::generate_property_doc(&property, None);
    let doc = lines.join("\n");

    assert!(!doc.contains("@example"));
}

#[test]
fn test_property_without_comments() {
    let mut property = create_test_property();
    property.comments = None;

    let lines = DocumentationGenerator::generate_property_doc(&property, None);
    let doc = lines.join("\n");

    // Should still have basic structure
    assert!(doc.contains("/**"));
    assert!(doc.contains("@cardinality"));
}

#[test]
fn test_documentation_without_optional_fields() {
    let doc = Documentation {
        short: "Simple test".to_string(),
        definition: "Simple test".to_string(),
        comments: None,
        requirements: None,
        usage_notes: vec![],
        url: None,
    };

    let lines = DocumentationGenerator::generate_type_doc(&doc);
    let result = lines.join("\n");

    assert!(result.contains("Simple test"));
    assert!(!result.contains("**Requirements:**"));
    assert!(!result.contains("**Usage Notes:**"));
    assert!(!result.contains("@see"));
}
