use insta::assert_snapshot;
use octofhir_codegen::core::ir::*;
use octofhir_codegen::languages::typescript::{ChoiceElementGenerator, TypeScriptBackend};

fn create_observation_value_choice() -> Property {
    Property {
        name: "value[x]".to_string(),
        path: "Observation.value[x]".to_string(),
        property_type: PropertyType::Choice {
            types: vec![
                "Quantity".to_string(),
                "CodeableConcept".to_string(),
                "string".to_string(),
                "boolean".to_string(),
                "integer".to_string(),
                "Range".to_string(),
                "Ratio".to_string(),
                "SampledData".to_string(),
                "time".to_string(),
                "dateTime".to_string(),
                "Period".to_string(),
            ],
        },
        cardinality: CardinalityRange::optional(),
        is_choice: true,
        choice_types: vec![
            "Quantity".to_string(),
            "CodeableConcept".to_string(),
            "string".to_string(),
            "boolean".to_string(),
            "integer".to_string(),
            "Range".to_string(),
            "Ratio".to_string(),
            "SampledData".to_string(),
            "time".to_string(),
            "dateTime".to_string(),
            "Period".to_string(),
        ],
        is_modifier: false,
        is_summary: true,
        binding: None,
        constraints: vec![],
        short_description: "Actual result".to_string(),
        definition: "The information determined as a result of making the observation, if the information has a simple value.".to_string(),
        comments: Some("Used when observation has a simple result.".to_string()),
        examples: vec![],
    }
}

fn create_patient_deceased_choice() -> Property {
    Property {
        name: "deceased[x]".to_string(),
        path: "Patient.deceased[x]".to_string(),
        property_type: PropertyType::Choice {
            types: vec!["boolean".to_string(), "dateTime".to_string()],
        },
        cardinality: CardinalityRange::optional(),
        is_choice: true,
        choice_types: vec!["boolean".to_string(), "dateTime".to_string()],
        is_modifier: true,
        is_summary: true,
        binding: None,
        constraints: vec![],
        short_description: "Indicates if the individual is deceased or not".to_string(),
        definition: "Indicates if the individual is deceased or not. Can be a boolean or dateTime."
            .to_string(),
        comments: None,
        examples: vec![],
    }
}

fn create_medication_request_dose_choice() -> Property {
    Property {
        name: "dose[x]".to_string(),
        path: "MedicationRequest.dosageInstruction.doseAndRate.dose[x]".to_string(),
        property_type: PropertyType::Choice {
            types: vec!["Range".to_string(), "Quantity".to_string()],
        },
        cardinality: CardinalityRange::optional(),
        is_choice: true,
        choice_types: vec!["Range".to_string(), "Quantity".to_string()],
        is_modifier: false,
        is_summary: false,
        binding: None,
        constraints: vec![],
        short_description: "Amount of medication per dose".to_string(),
        definition: "Amount of medication per dose".to_string(),
        comments: None,
        examples: vec![],
    }
}

#[test]
fn test_generate_observation_value_choice_union() {
    let backend = TypeScriptBackend::new();
    let generator = ChoiceElementGenerator::new(backend);
    let property = create_observation_value_choice();

    let output = generator.generate_choice_union(&property, "Observation").unwrap();
    assert_snapshot!("observation_value_choice_union", output);
}

#[test]
fn test_generate_patient_deceased_choice_union() {
    let backend = TypeScriptBackend::new();
    let generator = ChoiceElementGenerator::new(backend);
    let property = create_patient_deceased_choice();

    let output = generator.generate_choice_union(&property, "Patient").unwrap();
    assert_snapshot!("patient_deceased_choice_union", output);
}

#[test]
fn test_generate_observation_value_type_guards() {
    let backend = TypeScriptBackend::new();
    let generator = ChoiceElementGenerator::new(backend);
    let property = create_observation_value_choice();

    let output = generator.generate_choice_type_guards(&property, "Observation").unwrap();
    assert_snapshot!("observation_value_type_guards", output);
}

#[test]
fn test_generate_patient_deceased_type_guards() {
    let backend = TypeScriptBackend::new();
    let generator = ChoiceElementGenerator::new(backend);
    let property = create_patient_deceased_choice();

    let output = generator.generate_choice_type_guards(&property, "Patient").unwrap();
    assert_snapshot!("patient_deceased_type_guards", output);
}

#[test]
fn test_generate_observation_value_extractor() {
    let backend = TypeScriptBackend::new();
    let generator = ChoiceElementGenerator::new(backend);
    let property = create_observation_value_choice();

    let output = generator.generate_choice_value_extractor(&property, "Observation").unwrap();
    assert_snapshot!("observation_value_extractor", output);
}

#[test]
fn test_generate_patient_deceased_value_extractor() {
    let backend = TypeScriptBackend::new();
    let generator = ChoiceElementGenerator::new(backend);
    let property = create_patient_deceased_choice();

    let output = generator.generate_choice_value_extractor(&property, "Patient").unwrap();
    assert_snapshot!("patient_deceased_value_extractor", output);
}

#[test]
fn test_generate_observation_choice_module() {
    let backend = TypeScriptBackend::new();
    let generator = ChoiceElementGenerator::new(backend);
    let property = create_observation_value_choice();

    let output = generator.generate_choice_module(&property, "Observation").unwrap();
    assert_snapshot!("observation_choice_module", output);
}

#[test]
fn test_generate_dose_choice_module() {
    let backend = TypeScriptBackend::new();
    let generator = ChoiceElementGenerator::new(backend);
    let property = create_medication_request_dose_choice();

    let output = generator.generate_choice_module(&property, "MedicationRequest").unwrap();
    assert_snapshot!("dose_choice_module", output);
}

#[test]
fn test_generate_inline_choice_type() {
    let backend = TypeScriptBackend::new();
    let generator = ChoiceElementGenerator::new(backend);
    let property = create_patient_deceased_choice();

    let output = generator.generate_inline_choice_type(&property).unwrap();
    assert_snapshot!("patient_deceased_inline_choice", output);
}

#[test]
fn test_generate_observation_choice_enum() {
    let backend = TypeScriptBackend::new();
    let generator = ChoiceElementGenerator::new(backend);
    let property = create_observation_value_choice();

    let output = generator.generate_choice_enum(&property, "Observation").unwrap();
    assert_snapshot!("observation_value_choice_enum", output);
}

#[test]
fn test_generate_patient_deceased_enum() {
    let backend = TypeScriptBackend::new();
    let generator = ChoiceElementGenerator::new(backend);
    let property = create_patient_deceased_choice();

    let output = generator.generate_choice_enum(&property, "Patient").unwrap();
    assert_snapshot!("patient_deceased_enum", output);
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
        short_description: "registered | preliminary | final | amended +".to_string(),
        definition: "The status of the result value".to_string(),
        comments: None,
        examples: vec![],
    };

    let result = generator.generate_choice_union(&non_choice, "Observation");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not a choice element"));
}

// Note: map_choice_type is private and tested indirectly through other tests
