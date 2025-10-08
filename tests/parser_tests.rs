use insta::assert_json_snapshot;
use octofhir_codegen::core::parser::StructureDefinitionParser;
use serde_json::Value;
use std::fs;

#[test]
fn test_parse_patient_structure_definition() {
    let json_str =
        fs::read_to_string("tests/fixtures/patient.json").expect("Failed to read patient fixture");

    let json: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    let mut parser = StructureDefinitionParser::new();
    let parsed = parser.parse(&json).expect("Failed to parse StructureDefinition");

    assert_eq!(parsed.name, "Patient");
    assert_eq!(parsed.url, "http://hl7.org/fhir/StructureDefinition/Patient");
    assert!(!parsed.is_abstract);
    assert!(!parsed.differential);
    assert_eq!(parsed.elements.len(), 9);
}

#[test]
fn test_convert_to_resource_type() {
    let json_str =
        fs::read_to_string("tests/fixtures/patient.json").expect("Failed to read patient fixture");

    let json: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    let mut parser = StructureDefinitionParser::new();
    let parsed = parser.parse(&json).expect("Failed to parse");

    let resource_type =
        parser.to_resource_type(&parsed).expect("Failed to convert to ResourceType");

    assert_eq!(resource_type.name, "Patient");
    assert!(!resource_type.properties.is_empty());

    // Check specific properties
    let active_prop = resource_type
        .properties
        .iter()
        .find(|p| p.name == "active")
        .expect("active property not found");
    assert_eq!(active_prop.cardinality.min, 0);
    assert_eq!(active_prop.cardinality.max, Some(1));

    let name_prop = resource_type
        .properties
        .iter()
        .find(|p| p.name == "name")
        .expect("name property not found");
    assert_eq!(name_prop.cardinality.min, 0);
    assert_eq!(name_prop.cardinality.max, None); // unbounded
}

#[test]
fn test_patient_choice_element() {
    let json_str =
        fs::read_to_string("tests/fixtures/patient.json").expect("Failed to read patient fixture");

    let json: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    let mut parser = StructureDefinitionParser::new();
    let parsed = parser.parse(&json).expect("Failed to parse");
    let resource_type = parser.to_resource_type(&parsed).expect("Failed to convert");

    // Check deceased[x] choice element
    let deceased_prop = resource_type
        .properties
        .iter()
        .find(|p| p.name == "deceased")
        .expect("deceased property not found");

    assert!(deceased_prop.is_choice);
    assert_eq!(deceased_prop.choice_types.len(), 2);
    assert!(deceased_prop.choice_types.contains(&"boolean".to_string()));
    assert!(deceased_prop.choice_types.contains(&"dateTime".to_string()));
}

#[test]
fn test_patient_reference_element() {
    let json_str =
        fs::read_to_string("tests/fixtures/patient.json").expect("Failed to read patient fixture");

    let json: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    let mut parser = StructureDefinitionParser::new();
    let parsed = parser.parse(&json).expect("Failed to parse");
    let resource_type = parser.to_resource_type(&parsed).expect("Failed to convert");

    // Check generalPractitioner reference
    let gp_prop = resource_type
        .properties
        .iter()
        .find(|p| p.name == "generalPractitioner")
        .expect("generalPractitioner property not found");

    if let octofhir_codegen::core::ir::PropertyType::Reference { target_types } =
        &gp_prop.property_type
    {
        assert_eq!(target_types.len(), 3);
        assert!(target_types.contains(&"Practitioner".to_string()));
        assert!(target_types.contains(&"Organization".to_string()));
        assert!(target_types.contains(&"PractitionerRole".to_string()));
    } else {
        panic!("Expected Reference type");
    }
}

#[test]
fn test_patient_backbone_element() {
    let json_str =
        fs::read_to_string("tests/fixtures/patient.json").expect("Failed to read patient fixture");

    let json: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    let mut parser = StructureDefinitionParser::new();
    let parsed = parser.parse(&json).expect("Failed to parse");
    let resource_type = parser.to_resource_type(&parsed).expect("Failed to convert");

    // Check contact backbone element (has no types defined)
    let contact_prop = resource_type
        .properties
        .iter()
        .find(|p| p.name == "contact")
        .expect("contact property not found");

    if let octofhir_codegen::core::ir::PropertyType::BackboneElement { properties: _ } =
        &contact_prop.property_type
    {
        // Expected
    } else {
        panic!("Expected BackboneElement type");
    }
}

#[test]
fn test_snapshot_parsed_structure() {
    let json_str =
        fs::read_to_string("tests/fixtures/patient.json").expect("Failed to read patient fixture");

    let json: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    let mut parser = StructureDefinitionParser::new();
    let parsed = parser.parse(&json).expect("Failed to parse");

    // Snapshot a subset for readability
    let snapshot_data = serde_json::json!({
        "name": parsed.name,
        "url": parsed.url,
        "kind": format!("{:?}", parsed.kind),
        "is_abstract": parsed.is_abstract,
        "differential": parsed.differential,
        "element_count": parsed.elements.len(),
        "elements": parsed.elements.iter().map(|e| serde_json::json!({
            "path": e.path,
            "short": e.short,
            "min": e.min,
            "max": format!("{:?}", e.max),
            "type_count": e.types.len(),
        })).collect::<Vec<_>>()
    });

    assert_json_snapshot!("parsed_patient_sd", snapshot_data);
}

#[test]
fn test_snapshot_resource_type() {
    let json_str =
        fs::read_to_string("tests/fixtures/patient.json").expect("Failed to read patient fixture");

    let json: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    let mut parser = StructureDefinitionParser::new();
    let parsed = parser.parse(&json).expect("Failed to parse");
    let resource_type = parser.to_resource_type(&parsed).expect("Failed to convert");

    assert_json_snapshot!("patient_resource_type", resource_type);
}
