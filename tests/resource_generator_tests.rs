use insta::assert_snapshot;
use octofhir_codegen::core::ir::*;
use octofhir_codegen::languages::typescript::{ResourceGenerator, TypeScriptBackend};

fn create_patient_resource() -> ResourceType {
    ResourceType {
        name: "Patient".to_string(),
        base: Some("DomainResource".to_string()),
        properties: vec![
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
                definition: "An identifier for this patient".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "active".to_string(),
                path: "Patient.active".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "boolean".to_string(),
                },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: true,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Whether this patient's record is in active use".to_string(),
                definition: "Whether this patient's record is in active use".to_string(),
                comments: Some("If a record is inactive, and linked to an active record, then future patient/record updates should occur on the other patient.".to_string()),
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
                name: "telecom".to_string(),
                path: "Patient.telecom".to_string(),
                property_type: PropertyType::Complex {
                    type_name: "ContactPoint".to_string(),
                },
                cardinality: CardinalityRange::optional_array(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "A contact detail for the individual".to_string(),
                definition: "A contact detail (e.g. a telephone number or an email address) by which the individual may be contacted.".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "gender".to_string(),
                path: "Patient.gender".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "code".to_string(),
                },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: Some(ValueSetBinding {
                    strength: BindingStrength::Required,
                    value_set: "http://hl7.org/fhir/ValueSet/administrative-gender".to_string(),
                    description: Some("The gender of a person used for administrative purposes.".to_string()),
                }),
                constraints: vec![],
                short_description: "male | female | other | unknown".to_string(),
                definition: "Administrative Gender - the gender that the patient is considered to have for administration and record keeping purposes.".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "birthDate".to_string(),
                path: "Patient.birthDate".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "date".to_string(),
                },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "The date of birth for the individual".to_string(),
                definition: "The date of birth for the individual".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "deceased".to_string(),
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
                definition: "Indicates if the individual is deceased or not".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "address".to_string(),
                path: "Patient.address".to_string(),
                property_type: PropertyType::Complex {
                    type_name: "Address".to_string(),
                },
                cardinality: CardinalityRange::optional_array(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "An address for the individual".to_string(),
                definition: "An address for the individual".to_string(),
                comments: None,
                examples: vec![],
            },
        ],
        search_parameters: vec![],
        documentation: Documentation {
            short: "Information about an individual or animal receiving care or other health-related services".to_string(),
            definition: "Demographics and other administrative information about an individual or animal receiving care or other health-related services.".to_string(),
            url: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
            ..Default::default()
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
                name: "identifier".to_string(),
                path: "Observation.identifier".to_string(),
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
                short_description: "Business Identifier for observation".to_string(),
                definition: "A unique identifier assigned to this observation".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "status".to_string(),
                path: "Observation.status".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "code".to_string(),
                },
                cardinality: CardinalityRange::required(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: true,
                is_summary: true,
                binding: Some(ValueSetBinding {
                    strength: BindingStrength::Required,
                    value_set: "http://hl7.org/fhir/ValueSet/observation-status".to_string(),
                    description: Some("Codes providing the status of an observation.".to_string()),
                }),
                constraints: vec![],
                short_description: "registered | preliminary | final | amended +".to_string(),
                definition: "The status of the result value".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "code".to_string(),
                path: "Observation.code".to_string(),
                property_type: PropertyType::Complex {
                    type_name: "CodeableConcept".to_string(),
                },
                cardinality: CardinalityRange::required(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Type of observation (code / type)".to_string(),
                definition: "Describes what was observed. Sometimes this is called the observation \"name\".".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "subject".to_string(),
                path: "Observation.subject".to_string(),
                property_type: PropertyType::Reference {
                    target_types: vec!["Patient".to_string(), "Group".to_string(), "Device".to_string(), "Location".to_string()],
                },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Who and/or what the observation is about".to_string(),
                definition: "The patient, or group of patients, location, or device this observation is about".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "value".to_string(),
                path: "Observation.value[x]".to_string(),
                property_type: PropertyType::Choice {
                    types: vec![
                        "Quantity".to_string(),
                        "CodeableConcept".to_string(),
                        "string".to_string(),
                        "boolean".to_string(),
                        "integer".to_string(),
                        "Range".to_string(),
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
                ],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Actual result".to_string(),
                definition: "The information determined as a result of making the observation".to_string(),
                comments: None,
                examples: vec![],
            },
        ],
        search_parameters: vec![],
        documentation: Documentation {
            short: "Measurements and simple assertions".to_string(),
            definition: "Measurements and simple assertions made about a patient, device or other subject.".to_string(),
            url: Some("http://hl7.org/fhir/StructureDefinition/Observation".to_string()),
            ..Default::default()
        },
        url: "http://hl7.org/fhir/StructureDefinition/Observation".to_string(),
        is_abstract: false,
    }
}

#[test]
fn test_generate_patient_resource() {
    let backend = TypeScriptBackend::new();
    let generator = ResourceGenerator::new(backend);
    let resource = create_patient_resource();

    let output = generator.generate_resource(&resource).unwrap();
    assert_snapshot!("patient_resource", output);
}

#[test]
fn test_generate_observation_resource() {
    let backend = TypeScriptBackend::new();
    let generator = ResourceGenerator::new(backend);
    let resource = create_observation_resource();

    let output = generator.generate_resource(&resource).unwrap();
    assert_snapshot!("observation_resource", output);
}

#[test]
fn test_generate_patient_type_guard() {
    let backend = TypeScriptBackend::new();
    let generator = ResourceGenerator::new(backend);

    let output = generator.generate_type_guard("Patient").unwrap();
    assert_snapshot!("patient_type_guard", output);
}

#[test]
fn test_generate_observation_type_guard() {
    let backend = TypeScriptBackend::new();
    let generator = ResourceGenerator::new(backend);

    let output = generator.generate_type_guard("Observation").unwrap();
    assert_snapshot!("observation_type_guard", output);
}

#[test]
fn test_generate_resource_union() {
    let backend = TypeScriptBackend::new();
    let generator = ResourceGenerator::new(backend);

    let mut graph = TypeGraph::new(FhirVersion::R4);
    graph.add_resource("Patient".to_string(), create_patient_resource());
    graph.add_resource("Observation".to_string(), create_observation_resource());

    let output = generator.generate_resource_union(&graph).unwrap();
    assert_snapshot!("resource_union", output);
}

#[test]
fn test_generate_all_type_guards() {
    let backend = TypeScriptBackend::new();
    let generator = ResourceGenerator::new(backend);

    let mut graph = TypeGraph::new(FhirVersion::R4);
    graph.add_resource("Patient".to_string(), create_patient_resource());
    graph.add_resource("Observation".to_string(), create_observation_resource());

    let output = generator.generate_all_type_guards(&graph).unwrap();
    assert_snapshot!("all_type_guards", output);
}

#[test]
fn test_generate_resource_constants() {
    let backend = TypeScriptBackend::new();
    let generator = ResourceGenerator::new(backend);

    let mut graph = TypeGraph::new(FhirVersion::R4);
    graph.add_resource("Patient".to_string(), create_patient_resource());
    graph.add_resource("Observation".to_string(), create_observation_resource());

    let output = generator.generate_resource_constants(&graph).unwrap();
    assert_snapshot!("resource_constants", output);
}

#[test]
fn test_collect_patient_dependencies() {
    let backend = TypeScriptBackend::new();
    let generator = ResourceGenerator::new(backend);
    let resource = create_patient_resource();

    let deps = generator.collect_dependencies(&resource);

    assert!(deps.contains("Identifier"));
    assert!(deps.contains("HumanName"));
    assert!(deps.contains("ContactPoint"));
    assert!(deps.contains("Address"));
    assert!(deps.contains("DomainResource"));
    assert!(!deps.contains("boolean")); // Primitives should not be included
    assert!(!deps.contains("string"));
    assert!(!deps.contains("date"));
}

#[test]
fn test_collect_observation_dependencies() {
    let backend = TypeScriptBackend::new();
    let generator = ResourceGenerator::new(backend);
    let resource = create_observation_resource();

    let deps = generator.collect_dependencies(&resource);

    assert!(deps.contains("Identifier"));
    assert!(deps.contains("CodeableConcept"));
    assert!(deps.contains("DomainResource"));
    // Reference type should include target types - but currently it doesn't have type_name()
    // Choice types won't be in deps since type_name() returns None for Choice
}

#[test]
fn test_generate_patient_file_with_imports() {
    let backend = TypeScriptBackend::new();
    let generator = ResourceGenerator::new(backend);
    let resource = create_patient_resource();

    let deps = generator.collect_dependencies(&resource);
    let output = generator.generate_resource_file(&resource, &deps).unwrap();

    assert_snapshot!("patient_file_complete", output);
}
