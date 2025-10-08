use insta::assert_snapshot;
use octofhir_codegen::core::ir::*;
use octofhir_codegen::generator::LanguageBackend;
use octofhir_codegen::languages::typescript::TypeScriptBackend;

#[test]
fn test_complex_interface_generation() {
    let backend = TypeScriptBackend::new();

    let doc = Documentation {
        short: "Patient demographics and information".to_string(),
        definition: "Demographics and other administrative information about an individual or animal receiving care or other health-related services.".to_string(),
        url: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        ..Default::default()
    };

    let props = vec![
        ("resourceType".to_string(), "\"Patient\"".to_string(), false),
        ("id".to_string(), "id".to_string(), true),
        ("active".to_string(), "boolean".to_string(), true),
        ("name".to_string(), "HumanName[]".to_string(), true),
    ];

    let output = backend.generate_interface("Patient", Some("DomainResource"), &props, Some(&doc));

    assert_snapshot!("patient_interface", output);
}

#[test]
fn test_type_alias_generation() {
    let backend = TypeScriptBackend::new();

    let doc = Documentation {
        short: "Patient gender".to_string(),
        definition: "Administrative gender - male | female | other | unknown".to_string(),
        ..Default::default()
    };

    let output = backend.generate_type_alias(
        "PatientGender",
        "\"male\" | \"female\" | \"other\" | \"unknown\"",
        Some(&doc),
    );

    assert_snapshot!("patient_gender_type", output);
}

#[test]
fn test_primitive_type_mapping() {
    let backend = TypeScriptBackend::new();

    let boolean_type = PropertyType::Primitive { type_name: "boolean".to_string() };
    assert_eq!(backend.map_type(&boolean_type), "boolean");

    let integer_type = PropertyType::Primitive { type_name: "integer".to_string() };
    assert_eq!(backend.map_type(&integer_type), "number");

    let string_type = PropertyType::Primitive { type_name: "string".to_string() };
    assert_eq!(backend.map_type(&string_type), "string");

    let datetime_type = PropertyType::Primitive { type_name: "dateTime".to_string() };
    assert_eq!(backend.map_type(&datetime_type), "string");
}

#[test]
fn test_complex_type_mapping() {
    let backend = TypeScriptBackend::new();

    let complex_type = PropertyType::Complex { type_name: "HumanName".to_string() };
    assert_eq!(backend.map_type(&complex_type), "HumanName");
}

#[test]
fn test_reference_type_mapping() {
    let backend = TypeScriptBackend::new();

    // Empty reference
    let empty_ref = PropertyType::Reference { target_types: vec![] };
    assert_eq!(backend.map_type(&empty_ref), "Reference");

    // Single target
    let single_ref = PropertyType::Reference { target_types: vec!["Patient".to_string()] };
    assert_eq!(backend.map_type(&single_ref), "Reference<Patient>");

    // Multiple targets
    let multi_ref = PropertyType::Reference {
        target_types: vec!["Patient".to_string(), "Organization".to_string()],
    };
    assert_eq!(backend.map_type(&multi_ref), "Reference<Patient | Organization>");
}

#[test]
fn test_choice_type_mapping() {
    let backend = TypeScriptBackend::new();

    let choice_type = PropertyType::Choice {
        types: vec!["string".to_string(), "number".to_string(), "boolean".to_string()],
    };
    assert_eq!(backend.map_type(&choice_type), "string | number | boolean");
}

#[test]
fn test_generate_imports() {
    let backend = TypeScriptBackend::new();

    // Empty dependencies
    let empty_imports = backend.generate_imports(&[]);
    assert_eq!(empty_imports.len(), 0);

    // Single dependency
    let single_imports = backend.generate_imports(&["HumanName".to_string()]);
    assert_eq!(single_imports.len(), 1);
    assert_eq!(single_imports[0], "import { HumanName } from './types';");

    // Multiple dependencies
    let multi_imports = backend.generate_imports(&["HumanName".to_string(), "Address".to_string()]);
    assert_eq!(multi_imports.len(), 1);
    assert_eq!(multi_imports[0], "import { HumanName, Address } from './types';");
}

#[test]
fn test_doc_comment_generation() {
    let backend = TypeScriptBackend::new();

    let doc = Documentation {
        short: "Patient resource".to_string(),
        definition:
            "This is a longer definition that describes the patient resource in more detail."
                .to_string(),
        comments: Some("Additional comments here".to_string()),
        url: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        ..Default::default()
    };

    let output = backend.generate_doc_comment(&doc);

    assert_snapshot!("doc_comment", output.join("\n"));
}

#[test]
fn test_interface_without_doc() {
    let backend = TypeScriptBackend::new();

    let props = vec![
        ("id".to_string(), "string".to_string(), true),
        ("name".to_string(), "string".to_string(), false),
    ];

    let output = backend.generate_interface("SimpleInterface", None, &props, None);

    assert_snapshot!("simple_interface", output);
}

#[test]
fn test_type_alias_without_doc() {
    let backend = TypeScriptBackend::new();

    let output = backend.generate_type_alias("Status", "\"active\" | \"inactive\"", None);

    assert_snapshot!("simple_type_alias", output);
}
