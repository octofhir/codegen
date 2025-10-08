use insta::assert_snapshot;
use octofhir_codegen::core::ir::*;
use octofhir_codegen::languages::typescript::{DatatypeGenerator, TypeScriptBackend};

fn create_human_name() -> DataType {
    DataType {
        name: "HumanName".to_string(),
        base: Some("Element".to_string()),
        properties: vec![
            Property {
                name: "use".to_string(),
                path: "HumanName.use".to_string(),
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
                    value_set: "http://hl7.org/fhir/ValueSet/name-use".to_string(),
                    description: Some("The use of a human name.".to_string()),
                }),
                constraints: vec![],
                short_description: "usual | official | temp | nickname | anonymous | old | maiden".to_string(),
                definition: "Identifies the purpose for this name".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "text".to_string(),
                path: "HumanName.text".to_string(),
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
                short_description: "Text representation of the full name".to_string(),
                definition: "Specifies the entire name as it should be displayed".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "family".to_string(),
                path: "HumanName.family".to_string(),
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
                short_description: "Family name (often called 'Surname')".to_string(),
                definition: "The part of a name that links to the genealogy".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "given".to_string(),
                path: "HumanName.given".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "string".to_string(),
                },
                cardinality: CardinalityRange::optional_array(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Given names (not always 'first'). Includes middle names".to_string(),
                definition: "Given name".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "prefix".to_string(),
                path: "HumanName.prefix".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "string".to_string(),
                },
                cardinality: CardinalityRange::optional_array(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Parts that come before the name".to_string(),
                definition: "Part of the name that is acquired as a title due to academic, legal, employment or nobility status".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "suffix".to_string(),
                path: "HumanName.suffix".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "string".to_string(),
                },
                cardinality: CardinalityRange::optional_array(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Parts that come after the name".to_string(),
                definition: "Part of the name that is acquired as a title due to academic, legal, employment or nobility status".to_string(),
                comments: None,
                examples: vec![],
            },
        ],
        documentation: Documentation {
            short: "Name of a human - parts and usage".to_string(),
            definition: "A human's name with the ability to identify parts and usage.".to_string(),
            url: Some("http://hl7.org/fhir/StructureDefinition/HumanName".to_string()),
            ..Default::default()
        },
        url: "http://hl7.org/fhir/StructureDefinition/HumanName".to_string(),
        is_abstract: false,
    }
}

fn create_address() -> DataType {
    DataType {
        name: "Address".to_string(),
        base: Some("Element".to_string()),
        properties: vec![
            Property {
                name: "use".to_string(),
                path: "Address.use".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "code".to_string(),
                },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: true,
                is_summary: true,
                binding: Some(ValueSetBinding {
                    strength: BindingStrength::Required,
                    value_set: "http://hl7.org/fhir/ValueSet/address-use".to_string(),
                    description: Some("The use of an address.".to_string()),
                }),
                constraints: vec![],
                short_description: "home | work | temp | old | billing".to_string(),
                definition: "The purpose of this address".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "type".to_string(),
                path: "Address.type".to_string(),
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
                    value_set: "http://hl7.org/fhir/ValueSet/address-type".to_string(),
                    description: Some("The type of an address.".to_string()),
                }),
                constraints: vec![],
                short_description: "postal | physical | both".to_string(),
                definition: "Distinguishes between physical addresses and mailing addresses".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "line".to_string(),
                path: "Address.line".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "string".to_string(),
                },
                cardinality: CardinalityRange::optional_array(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Street name, number, direction & P.O. Box etc.".to_string(),
                definition: "This component contains the house number, apartment number, street name, street direction, P.O. Box number, delivery hints, and similar address information.".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "city".to_string(),
                path: "Address.city".to_string(),
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
                short_description: "Name of city, town etc.".to_string(),
                definition: "The name of the city, town, suburb, village or other community or delivery center".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "state".to_string(),
                path: "Address.state".to_string(),
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
                short_description: "Sub-unit of country".to_string(),
                definition: "Sub-unit of a country with limited sovereignty in a federally organized country".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "postalCode".to_string(),
                path: "Address.postalCode".to_string(),
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
                short_description: "Postal code for area".to_string(),
                definition: "A postal code designating a region defined by the postal service".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "country".to_string(),
                path: "Address.country".to_string(),
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
                short_description: "Country (e.g. can be ISO 3166 2 or 3 letter code)".to_string(),
                definition: "Country - a nation as commonly understood or generally accepted".to_string(),
                comments: None,
                examples: vec![],
            },
        ],
        documentation: Documentation {
            short: "An address expressed using postal conventions (as opposed to GPS or other location definition formats)".to_string(),
            definition: "An address expressed using postal conventions (as opposed to GPS or other location definition formats). This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery. There are a variety of postal address formats defined around the world.".to_string(),
            url: Some("http://hl7.org/fhir/StructureDefinition/Address".to_string()),
            ..Default::default()
        },
        url: "http://hl7.org/fhir/StructureDefinition/Address".to_string(),
        is_abstract: false,
    }
}

fn create_quantity() -> DataType {
    DataType {
        name: "Quantity".to_string(),
        base: Some("Element".to_string()),
        properties: vec![
            Property {
                name: "value".to_string(),
                path: "Quantity.value".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "decimal".to_string(),
                },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Numerical value (with implicit precision)".to_string(),
                definition: "The value of the measured amount".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "comparator".to_string(),
                path: "Quantity.comparator".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "code".to_string(),
                },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: true,
                is_summary: true,
                binding: Some(ValueSetBinding {
                    strength: BindingStrength::Required,
                    value_set: "http://hl7.org/fhir/ValueSet/quantity-comparator".to_string(),
                    description: Some("How the Quantity should be understood and represented.".to_string()),
                }),
                constraints: vec![],
                short_description: "< | <= | >= | >".to_string(),
                definition: "How the value should be understood and represented".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "unit".to_string(),
                path: "Quantity.unit".to_string(),
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
                short_description: "Unit representation".to_string(),
                definition: "A human-readable form of the unit".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "system".to_string(),
                path: "Quantity.system".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "uri".to_string(),
                },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "System that defines coded unit form".to_string(),
                definition: "The identification of the system that provides the coded form of the unit".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "code".to_string(),
                path: "Quantity.code".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "code".to_string(),
                },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Coded form of the unit".to_string(),
                definition: "A computer processable form of the unit in some unit representation system".to_string(),
                comments: None,
                examples: vec![],
            },
        ],
        documentation: Documentation {
            short: "A measured or measurable amount".to_string(),
            definition: "A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.".to_string(),
            url: Some("http://hl7.org/fhir/StructureDefinition/Quantity".to_string()),
            ..Default::default()
        },
        url: "http://hl7.org/fhir/StructureDefinition/Quantity".to_string(),
        is_abstract: false,
    }
}

#[test]
fn test_generate_human_name() {
    let backend = TypeScriptBackend::new();
    let generator = DatatypeGenerator::new(backend);
    let datatype = create_human_name();

    let output = generator.generate_datatype(&datatype).unwrap();
    assert_snapshot!("human_name_datatype", output);
}

#[test]
fn test_generate_address() {
    let backend = TypeScriptBackend::new();
    let generator = DatatypeGenerator::new(backend);
    let datatype = create_address();

    let output = generator.generate_datatype(&datatype).unwrap();
    assert_snapshot!("address_datatype", output);
}

#[test]
fn test_generate_quantity() {
    let backend = TypeScriptBackend::new();
    let generator = DatatypeGenerator::new(backend);
    let datatype = create_quantity();

    let output = generator.generate_datatype(&datatype).unwrap();
    assert_snapshot!("quantity_datatype", output);
}

#[test]
fn test_generate_primitive_wrapper() {
    let output = DatatypeGenerator::<TypeScriptBackend>::generate_primitive_wrapper().unwrap();
    assert_snapshot!("fhir_primitive_wrapper", output);
}

#[test]
fn test_generate_simple_primitive_types() {
    let output = DatatypeGenerator::<TypeScriptBackend>::generate_simple_primitive_types().unwrap();
    assert_snapshot!("simple_primitive_types", output);
}

#[test]
fn test_generate_primitive_types() {
    let backend = TypeScriptBackend::new();
    let generator = DatatypeGenerator::new(backend);

    let mut graph = TypeGraph::new(FhirVersion::R4);

    // Add some sample primitives
    graph.add_primitive(
        "boolean".to_string(),
        PrimitiveType {
            name: "boolean".to_string(),
            base: Some("Element".to_string()),
            pattern: None,
            documentation: Documentation {
                short: "true | false".to_string(),
                definition: "Value of \"true\" or \"false\"".to_string(),
                ..Default::default()
            },
            url: "http://hl7.org/fhir/StructureDefinition/boolean".to_string(),
        },
    );

    graph.add_primitive(
        "string".to_string(),
        PrimitiveType {
            name: "string".to_string(),
            base: Some("Element".to_string()),
            pattern: None,
            documentation: Documentation {
                short: "A sequence of Unicode characters".to_string(),
                definition: "A sequence of Unicode characters".to_string(),
                ..Default::default()
            },
            url: "http://hl7.org/fhir/StructureDefinition/string".to_string(),
        },
    );

    let output = generator.generate_primitive_types(&graph).unwrap();
    assert_snapshot!("primitive_types_with_wrapper", output);
}

#[test]
fn test_collect_human_name_dependencies() {
    let backend = TypeScriptBackend::new();
    let generator = DatatypeGenerator::new(backend);
    let datatype = create_human_name();

    let deps = generator.collect_dependencies(&datatype);

    assert!(deps.contains("Element"));
    assert!(!deps.contains("string")); // Primitives should not be included
    assert!(!deps.contains("code"));
}

#[test]
fn test_generate_datatype_file() {
    let backend = TypeScriptBackend::new();
    let generator = DatatypeGenerator::new(backend);
    let datatype = create_human_name();

    let deps = generator.collect_dependencies(&datatype);
    let output = generator.generate_datatype_file(&datatype, &deps).unwrap();

    assert_snapshot!("human_name_file_complete", output);
}

#[test]
fn test_generate_datatypes_index() {
    let backend = TypeScriptBackend::new();
    let generator = DatatypeGenerator::new(backend);

    let mut graph = TypeGraph::new(FhirVersion::R4);
    graph.add_datatype("HumanName".to_string(), create_human_name());
    graph.add_datatype("Address".to_string(), create_address());
    graph.add_datatype("Quantity".to_string(), create_quantity());

    let output = generator.generate_datatypes_index(&graph).unwrap();
    assert_snapshot!("datatypes_index", output);
}
