use octofhir_codegen::core::ir::*;

#[test]
fn test_serialize_type_graph() {
    let mut graph = TypeGraph::new(FhirVersion::R4);

    // Set fixed timestamp for snapshot testing
    graph.metadata.generated_at = "2025-10-08T14:00:00+00:00".to_string();

    // Add a simple resource
    let patient = ResourceType {
        name: "Patient".to_string(),
        base: Some("DomainResource".to_string()),
        properties: vec![Property {
            name: "birthDate".to_string(),
            path: "Patient.birthDate".to_string(),
            property_type: PropertyType::Primitive {
                type_name: "date".to_string(),
            },
            cardinality: CardinalityRange::optional(),
            is_choice: false,
            choice_types: Vec::new(),
            is_modifier: false,
            is_summary: true,
            binding: None,
            constraints: Vec::new(),
            short_description: "The date of birth for the individual".to_string(),
            definition: "The date of birth for the individual.".to_string(),
            comments: None,
            examples: Vec::new(),
        }],
        search_parameters: Vec::new(),
        documentation: Documentation {
            short: "Information about an individual receiving health care services".to_string(),
            definition: "Demographics and other administrative information about an individual receiving care or other health-related services.".to_string(),
            comments: None,
            requirements: None,
            usage_notes: Vec::new(),
            url: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        },
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        is_abstract: false,
    };

    graph.add_resource("Patient".to_string(), patient);

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&graph).unwrap();
    insta::assert_snapshot!(json);
}

#[test]
fn test_serialize_complex_property_types() {
    let reference = PropertyType::Reference {
        target_types: vec!["Patient".to_string(), "Practitioner".to_string()],
    };
    let json = serde_json::to_string_pretty(&reference).unwrap();
    insta::assert_snapshot!("reference_type", json);

    let choice =
        PropertyType::Choice { types: vec!["string".to_string(), "CodeableConcept".to_string()] };
    let json = serde_json::to_string_pretty(&choice).unwrap();
    insta::assert_snapshot!("choice_type", json);

    let backbone = PropertyType::BackboneElement {
        properties: vec![Property {
            name: "use".to_string(),
            path: "HumanName.use".to_string(),
            property_type: PropertyType::Primitive { type_name: "code".to_string() },
            cardinality: CardinalityRange::optional(),
            is_choice: false,
            choice_types: Vec::new(),
            is_modifier: false,
            is_summary: true,
            binding: Some(ValueSetBinding {
                strength: BindingStrength::Required,
                value_set: "http://hl7.org/fhir/ValueSet/name-use".to_string(),
                description: Some("The use of a human name.".to_string()),
            }),
            constraints: Vec::new(),
            short_description: "usual | official | temp | nickname | anonymous | old | maiden"
                .to_string(),
            definition: "Identifies the purpose for this name.".to_string(),
            comments: None,
            examples: Vec::new(),
        }],
    };
    let json = serde_json::to_string_pretty(&backbone).unwrap();
    insta::assert_snapshot!("backbone_element_type", json);
}

#[test]
fn test_serialize_cardinality_variations() {
    let required = CardinalityRange::required();
    let json = serde_json::to_string_pretty(&required).unwrap();
    insta::assert_snapshot!("cardinality_required", json);

    let optional = CardinalityRange::optional();
    let json = serde_json::to_string_pretty(&optional).unwrap();
    insta::assert_snapshot!("cardinality_optional", json);

    let required_array = CardinalityRange::required_array();
    let json = serde_json::to_string_pretty(&required_array).unwrap();
    insta::assert_snapshot!("cardinality_required_array", json);

    let optional_array = CardinalityRange::optional_array();
    let json = serde_json::to_string_pretty(&optional_array).unwrap();
    insta::assert_snapshot!("cardinality_optional_array", json);
}

#[test]
fn test_serialize_binding_strengths() {
    for &strength in &[
        BindingStrength::Required,
        BindingStrength::Extensible,
        BindingStrength::Preferred,
        BindingStrength::Example,
    ] {
        let binding = ValueSetBinding {
            strength,
            value_set: "http://example.org/ValueSet/test".to_string(),
            description: Some("Test binding".to_string()),
        };
        let json = serde_json::to_string_pretty(&binding).unwrap();
        let name = format!("binding_{:?}", strength).to_lowercase();
        insta::assert_snapshot!(name, json);
    }
}

#[test]
fn test_serialize_invariant_rule() {
    let invariant = InvariantRule {
        key: "pat-1".to_string(),
        severity: ConstraintSeverity::Error,
        human: "SHALL at least contain a contact's details or a reference to an organization"
            .to_string(),
        expression: Some(
            "name.exists() or telecom.exists() or address.exists() or organization.exists()"
                .to_string(),
        ),
        xpath: Some("f:name or f:telecom or f:address or f:organization".to_string()),
    };

    let json = serde_json::to_string_pretty(&invariant).unwrap();
    insta::assert_snapshot!(json);
}

#[test]
fn test_serialize_search_parameter() {
    let param = SearchParameter {
        code: "birthdate".to_string(),
        param_type: SearchParamType::Date,
        description: "The patient's date of birth".to_string(),
        expression: Some("Patient.birthDate".to_string()),
        target_types: Vec::new(),
    };

    let json = serde_json::to_string_pretty(&param).unwrap();
    insta::assert_snapshot!(json);
}

#[test]
fn test_round_trip_serialization() {
    // Create a complex type graph
    let mut graph = TypeGraph::new(FhirVersion::R5);

    let datatype = DataType {
        name: "HumanName".to_string(),
        base: Some("Element".to_string()),
        properties: vec![
            Property {
                name: "family".to_string(),
                path: "HumanName.family".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "string".to_string(),
                },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: Vec::new(),
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: Vec::new(),
                short_description: "Family name (often called 'Surname')".to_string(),
                definition: "The part of a name that links to the genealogy.".to_string(),
                comments: None,
                examples: Vec::new(),
            },
            Property {
                name: "given".to_string(),
                path: "HumanName.given".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "string".to_string(),
                },
                cardinality: CardinalityRange::optional_array(),
                is_choice: false,
                choice_types: Vec::new(),
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: Vec::new(),
                short_description: "Given names (not always 'first')".to_string(),
                definition: "Given name.".to_string(),
                comments: Some("If only initials are recorded, they may be used in place of the full name parts.".to_string()),
                examples: Vec::new(),
            },
        ],
        documentation: Documentation {
            short: "Name of a human - parts and usage".to_string(),
            definition: "A human's name with the ability to identify parts and usage.".to_string(),
            comments: None,
            requirements: Some("Need to be able to record names, along with notes about their use.".to_string()),
            usage_notes: vec!["Names may be changed, or repudiated, or people may have different names in different contexts.".to_string()],
            url: Some("http://hl7.org/fhir/StructureDefinition/HumanName".to_string()),
        },
        url: "http://hl7.org/fhir/StructureDefinition/HumanName".to_string(),
        is_abstract: false,
    };

    graph.add_datatype("HumanName".to_string(), datatype);

    // Serialize
    let json = serde_json::to_string_pretty(&graph).unwrap();

    // Deserialize
    let deserialized: TypeGraph = serde_json::from_str(&json).unwrap();

    // Verify round-trip
    assert_eq!(graph, deserialized);
    assert_eq!(deserialized.fhir_version, FhirVersion::R5);
    assert_eq!(deserialized.datatypes.len(), 1);
    assert!(deserialized.datatypes.contains_key("HumanName"));
}
