use octofhir_codegen::core::ir::{
    CardinalityRange, Documentation, Property, PropertyType, ResourceType,
};
use octofhir_codegen::languages::typescript::{TypeScriptBackend, ValidationGenerator};

fn create_patient_resource() -> ResourceType {
    ResourceType {
        name: "Patient".to_string(),
        base: Some("DomainResource".to_string()),
        properties: vec![
            Property {
                name: "id".to_string(),
                path: "Patient.id".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "string".to_string(),
                },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Logical id".to_string(),
                definition: "The logical id of the resource".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "name".to_string(),
                path: "Patient.name".to_string(),
                property_type: PropertyType::Complex {
                    type_name: "HumanName".to_string(),
                },
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
            Property {
                name: "active".to_string(),
                path: "Patient.active".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "boolean".to_string(),
                },
                cardinality: CardinalityRange::required(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: true,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Whether this patient's record is in active use".to_string(),
                definition: "Whether this patient record is in active use".to_string(),
                comments: None,
                examples: vec![],
            },
        ],
        search_parameters: vec![],
        documentation: Documentation {
            short: "Information about an individual or animal receiving care".to_string(),
            definition: "Demographics and other administrative information about an individual or animal receiving care or other health-related services.".to_string(),
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
        properties: vec![
            Property {
                name: "status".to_string(),
                path: "Observation.status".to_string(),
                property_type: PropertyType::Primitive { type_name: "code".to_string() },
                cardinality: CardinalityRange::required(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: true,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "registered | preliminary | final | amended +".to_string(),
                definition: "The status of the result value.".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "code".to_string(),
                path: "Observation.code".to_string(),
                property_type: PropertyType::Complex { type_name: "CodeableConcept".to_string() },
                cardinality: CardinalityRange::required(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Type of observation".to_string(),
                definition:
                    "Describes what was observed. Sometimes this is called the observation 'name'."
                        .to_string(),
                comments: None,
                examples: vec![],
            },
        ],
        search_parameters: vec![],
        documentation: Documentation {
            short: "Measurements and simple assertions".to_string(),
            definition:
                "Measurements and simple assertions made about a patient, device or other subject."
                    .to_string(),
            comments: None,
            requirements: None,
            usage_notes: vec![],
            url: None,
        },
        url: "http://hl7.org/fhir/StructureDefinition/Observation".to_string(),
        is_abstract: false,
    }
}

#[test]
fn test_generate_validation_types() {
    let backend = TypeScriptBackend::new();
    let generator = ValidationGenerator::new(backend);

    let result = generator.generate_validation_types().unwrap();

    insta::assert_snapshot!("validation_types", result);
}

#[test]
fn test_generate_patient_validation() {
    let backend = TypeScriptBackend::new();
    let generator = ValidationGenerator::new(backend);
    let patient = create_patient_resource();

    let result = generator.generate_resource_validation(&patient).unwrap();

    insta::assert_snapshot!("patient_validation", result);
}

#[test]
fn test_generate_observation_validation() {
    let backend = TypeScriptBackend::new();
    let generator = ValidationGenerator::new(backend);
    let observation = create_observation_resource();

    let result = generator.generate_resource_validation(&observation).unwrap();

    insta::assert_snapshot!("observation_validation", result);
}

#[test]
fn test_generate_validation_types_module() {
    let backend = TypeScriptBackend::new();
    let generator = ValidationGenerator::new(backend);

    let result = generator.generate_validation_types_module().unwrap();

    insta::assert_snapshot!("validation_types_module", result);
}

#[test]
fn test_generate_patient_validation_module() {
    let backend = TypeScriptBackend::new();
    let generator = ValidationGenerator::new(backend);
    let patient = create_patient_resource();

    let result = generator.generate_validation_module(&patient).unwrap();

    insta::assert_snapshot!("patient_validation_module", result);
}

#[test]
fn test_generate_is_defined_helper() {
    let backend = TypeScriptBackend::new();
    let generator = ValidationGenerator::new(backend);

    let result = generator.generate_is_defined_helper().unwrap();

    insta::assert_snapshot!("is_defined_helper", result);
}

#[test]
fn test_validation_types_contain_all_fields() {
    let backend = TypeScriptBackend::new();
    let generator = ValidationGenerator::new(backend);

    let result = generator.generate_validation_types().unwrap();

    // Verify ValidationError fields
    assert!(result.contains("export interface ValidationError"));
    assert!(result.contains("path: string"));
    assert!(result.contains("message: string"));
    assert!(result.contains("severity: \"error\" | \"warning\" | \"info\""));
    assert!(result.contains("constraint?: string"));

    // Verify ValidationResult fields
    assert!(result.contains("export interface ValidationResult"));
    assert!(result.contains("valid: boolean"));
    assert!(result.contains("errors: ValidationError[]"));
}

#[test]
fn test_patient_validation_checks_required_fields() {
    let backend = TypeScriptBackend::new();
    let generator = ValidationGenerator::new(backend);
    let patient = create_patient_resource();

    let result = generator.generate_resource_validation(&patient).unwrap();

    // Should check the required 'active' field
    assert!(result.contains("// Check required field: active"));
    assert!(result.contains("if (!resource.active)"));
    assert!(result.contains("Required field 'active' is missing"));
}

#[test]
fn test_validation_checks_resource_type() {
    let backend = TypeScriptBackend::new();
    let generator = ValidationGenerator::new(backend);
    let patient = create_patient_resource();

    let result = generator.generate_resource_validation(&patient).unwrap();

    // Should validate resourceType
    assert!(result.contains("// Validate resourceType"));
    assert!(result.contains("if (resource.resourceType !== \"Patient\")"));
    assert!(result.contains("Invalid resourceType"));
}

#[test]
fn test_validation_returns_result() {
    let backend = TypeScriptBackend::new();
    let generator = ValidationGenerator::new(backend);
    let patient = create_patient_resource();

    let result = generator.generate_resource_validation(&patient).unwrap();

    // Should return ValidationResult
    assert!(result.contains("return {"));
    assert!(result.contains("valid: errors.filter(e => e.severity === \"error\").length === 0"));
    assert!(result.contains("errors"));
}

#[test]
fn test_validation_module_has_imports() {
    let backend = TypeScriptBackend::new();
    let generator = ValidationGenerator::new(backend);
    let patient = create_patient_resource();

    let result = generator.generate_validation_module(&patient).unwrap();

    // Should import resource and validation types
    assert!(result.contains("import { Patient } from './patient'"));
    assert!(result.contains("import { ValidationResult, ValidationError } from './validation'"));
}

#[test]
fn test_observation_validation_checks_multiple_required() {
    let backend = TypeScriptBackend::new();
    let generator = ValidationGenerator::new(backend);
    let observation = create_observation_resource();

    let result = generator.generate_resource_validation(&observation).unwrap();

    // Should check both required fields
    assert!(result.contains("// Check required field: status"));
    assert!(result.contains("// Check required field: code"));
}
