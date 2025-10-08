use octofhir_codegen::core::ir::{
    CardinalityRange, Documentation, Property, PropertyType, ResourceType,
};
use octofhir_codegen::languages::typescript::{HelpersGenerator, TypeScriptBackend};

fn create_patient_resource() -> ResourceType {
    ResourceType {
        name: "Patient".to_string(),
        base: Some("DomainResource".to_string()),
        properties: vec![
            Property {
                name: "id".to_string(),
                path: "Patient.id".to_string(),
                property_type: PropertyType::Primitive { type_name: "string".to_string() },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Logical id of this artifact".to_string(),
                definition: "The logical id of the resource".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "name".to_string(),
                path: "Patient.name".to_string(),
                property_type: PropertyType::Complex { type_name: "HumanName".to_string() },
                cardinality: CardinalityRange::optional_array(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "A name associated with the patient".to_string(),
                definition: "A name associated with the individual".to_string(),
                comments: None,
                examples: vec![],
            },
        ],
        search_parameters: vec![],
        documentation: Documentation {
            short: "Patient resource".to_string(),
            definition: "Demographics and administrative information".to_string(),
            comments: None,
            requirements: None,
            usage_notes: vec![],
            url: None,
        },
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        is_abstract: false,
    }
}

fn create_observation_resource() -> ResourceType {
    ResourceType {
        name: "Observation".to_string(),
        base: Some("DomainResource".to_string()),
        properties: vec![],
        search_parameters: vec![],
        documentation: Documentation {
            short: "Observation resource".to_string(),
            definition: "Measurements and simple assertions".to_string(),
            comments: None,
            requirements: None,
            usage_notes: vec![],
            url: None,
        },
        url: "http://hl7.org/fhir/StructureDefinition/Observation".to_string(),
        is_abstract: false,
    }
}

fn create_generic_resource() -> ResourceType {
    ResourceType {
        name: "Condition".to_string(),
        base: Some("DomainResource".to_string()),
        properties: vec![],
        search_parameters: vec![],
        documentation: Documentation {
            short: "Condition resource".to_string(),
            definition: "Detailed information about conditions".to_string(),
            comments: None,
            requirements: None,
            usage_notes: vec![],
            url: None,
        },
        url: "http://hl7.org/fhir/StructureDefinition/Condition".to_string(),
        is_abstract: false,
    }
}

#[test]
fn test_generate_extract_primitive_value() {
    let backend = TypeScriptBackend::new();
    let generator = HelpersGenerator::new(backend);

    let result = generator.generate_extract_primitive_value().unwrap();

    insta::assert_snapshot!("extract_primitive_value", result);
}

#[test]
fn test_generate_patient_helpers() {
    let backend = TypeScriptBackend::new();
    let generator = HelpersGenerator::new(backend);
    let patient = create_patient_resource();

    let result = generator.generate_resource_helpers(&patient).unwrap();

    insta::assert_snapshot!("patient_helpers", result);
}

#[test]
fn test_generate_observation_helpers() {
    let backend = TypeScriptBackend::new();
    let generator = HelpersGenerator::new(backend);
    let observation = create_observation_resource();

    let result = generator.generate_resource_helpers(&observation).unwrap();

    insta::assert_snapshot!("observation_helpers", result);
}

#[test]
fn test_generate_generic_helpers() {
    let backend = TypeScriptBackend::new();
    let generator = HelpersGenerator::new(backend);
    let condition = create_generic_resource();

    let result = generator.generate_resource_helpers(&condition).unwrap();

    insta::assert_snapshot!("generic_helpers", result);
}

#[test]
fn test_generate_utilities_module() {
    let backend = TypeScriptBackend::new();
    let generator = HelpersGenerator::new(backend);

    let result = generator.generate_utilities_module().unwrap();

    insta::assert_snapshot!("utilities_module", result);
}

#[test]
fn test_generate_patient_helpers_module() {
    let backend = TypeScriptBackend::new();
    let generator = HelpersGenerator::new(backend);
    let patient = create_patient_resource();

    let result = generator.generate_helpers_module(&patient).unwrap();

    insta::assert_snapshot!("patient_helpers_module", result);
}

#[test]
fn test_generate_observation_helpers_module() {
    let backend = TypeScriptBackend::new();
    let generator = HelpersGenerator::new(backend);
    let observation = create_observation_resource();

    let result = generator.generate_helpers_module(&observation).unwrap();

    insta::assert_snapshot!("observation_helpers_module", result);
}

#[test]
fn test_patient_helpers_contain_all_methods() {
    let backend = TypeScriptBackend::new();
    let generator = HelpersGenerator::new(backend);
    let patient = create_patient_resource();

    let result = generator.generate_resource_helpers(&patient).unwrap();

    // Verify all expected methods are present
    assert!(result.contains("export function getOfficialName"));
    assert!(result.contains("export function getFullName"));
    assert!(result.contains("export function isDeceased"));
    assert!(result.contains("export function getAge"));
}

#[test]
fn test_observation_helpers_contain_all_methods() {
    let backend = TypeScriptBackend::new();
    let generator = HelpersGenerator::new(backend);
    let observation = create_observation_resource();

    let result = generator.generate_resource_helpers(&observation).unwrap();

    // Verify expected methods
    assert!(result.contains("export function getValue"));
    assert!(result.contains("export function hasValue"));
}

#[test]
fn test_generic_helpers_contain_base_methods() {
    let backend = TypeScriptBackend::new();
    let generator = HelpersGenerator::new(backend);
    let condition = create_generic_resource();

    let result = generator.generate_resource_helpers(&condition).unwrap();

    // Verify base methods
    assert!(result.contains("export function getId"));
    assert!(result.contains("export function getMeta"));
}

#[test]
fn test_utilities_module_structure() {
    let backend = TypeScriptBackend::new();
    let generator = HelpersGenerator::new(backend);

    let result = generator.generate_utilities_module().unwrap();

    // Verify structure
    assert!(result.contains("Common FHIR utility functions"));
    assert!(result.contains("import { FhirPrimitive } from './primitives'"));
    assert!(result.contains("export function extractPrimitiveValue"));
    assert!(result.contains("This file is auto-generated"));
}

#[test]
fn test_helpers_module_has_correct_imports() {
    let backend = TypeScriptBackend::new();
    let generator = HelpersGenerator::new(backend);
    let patient = create_patient_resource();

    let result = generator.generate_helpers_module(&patient).unwrap();

    // Verify imports
    assert!(result.contains("import { Patient } from './patient'"));
    assert!(result.contains("import { HumanName } from './humanName'"));
    assert!(result.contains("import { extractPrimitiveValue } from './utilities'"));
}

#[test]
fn test_extract_primitive_value_has_jsdoc() {
    let backend = TypeScriptBackend::new();
    let generator = HelpersGenerator::new(backend);

    let result = generator.generate_extract_primitive_value().unwrap();

    // Verify JSDoc is present
    assert!(result.contains("/**"));
    assert!(result.contains("Extract primitive value from FhirPrimitive<T> wrapper"));
    assert!(result.contains("@param primitive"));
    assert!(result.contains("@returns"));
}
