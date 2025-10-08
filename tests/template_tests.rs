use insta::assert_snapshot;
use octofhir_codegen::core::ir::*;
use octofhir_codegen::generator::{TemplateContext, TemplateEngine};
use octofhir_codegen::languages::typescript::TypeScriptBackend;
use octofhir_codegen::languages::typescript::templates::TypeScriptTemplates;
use octofhir_codegen::templates::genco_engine::{GencoTemplateEngine, helpers};
use serde_json::json;

#[tokio::test]
async fn test_template_engine_basic() {
    let mut engine = GencoTemplateEngine::new();

    engine.register_template("hello".to_string(), |ctx| {
        let name = ctx.data.get("name").and_then(|v| v.as_str()).unwrap_or("World");
        Ok(format!("Hello, {}!", name))
    });

    let ctx = TemplateContext::new(json!({"name": "TypeScript"}));
    let result = engine.render("hello", &ctx).await.unwrap();

    assert_eq!(result, "Hello, TypeScript!");
}

#[tokio::test]
async fn test_template_engine_missing() {
    let engine = GencoTemplateEngine::new();
    let ctx = TemplateContext::new(json!({}));

    let result = engine.render("nonexistent", &ctx).await;
    assert!(result.is_err());
}

#[test]
fn test_jsdoc_simple() {
    let lines = vec!["Simple comment".to_string()];
    let tokens = helpers::jsdoc_comment(&lines);
    let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

    assert_snapshot!("jsdoc_simple", output);
}

#[test]
fn test_jsdoc_multiline() {
    let lines = vec![
        "Patient resource".to_string(),
        "".to_string(),
        "Contains demographic and administrative information".to_string(),
        "".to_string(),
        "@see http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
    ];

    let tokens = helpers::jsdoc_comment(&lines);
    let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

    assert_snapshot!("jsdoc_multiline", output);
}

#[test]
fn test_interface_simple() {
    let props = vec![
        ("id".to_string(), "string".to_string(), true),
        ("name".to_string(), "string".to_string(), false),
    ];

    let tokens = helpers::interface("User", None, &props, None);
    let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

    assert_snapshot!("interface_simple", output);
}

#[test]
fn test_interface_with_extends() {
    let props = vec![
        ("resourceType".to_string(), "\"Patient\"".to_string(), false),
        ("id".to_string(), "string".to_string(), true),
        ("active".to_string(), "boolean".to_string(), true),
    ];

    let doc = vec!["Patient resource".to_string()];

    let tokens = helpers::interface("Patient", Some("DomainResource"), &props, Some(&doc));
    let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

    assert_snapshot!("interface_with_extends", output);
}

#[test]
fn test_type_alias_simple() {
    let tokens = helpers::type_alias("Status", "\"active\" | \"inactive\"", None);
    let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

    assert_snapshot!("type_alias_simple", output);
}

#[test]
fn test_type_alias_with_doc() {
    let doc = vec![
        "Patient gender".to_string(),
        "".to_string(),
        "Administrative gender code".to_string(),
    ];

    let tokens = helpers::type_alias(
        "AdministrativeGender",
        "\"male\" | \"female\" | \"other\" | \"unknown\"",
        Some(&doc),
    );
    let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

    assert_snapshot!("type_alias_with_doc", output);
}

#[test]
fn test_imports() {
    let modules = vec![
        (vec!["Patient".to_string(), "Observation".to_string()], "./resources".to_string()),
        (vec!["HumanName".to_string(), "Address".to_string()], "./datatypes".to_string()),
    ];

    let tokens = helpers::imports(&modules);
    let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

    assert_snapshot!("imports", output);
}

#[test]
fn test_constant_with_type() {
    let tokens = helpers::constant("FHIR_VERSION", "\"4.0.1\"", Some("string"));
    let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

    assert_snapshot!("constant_with_type", output);
}

#[test]
fn test_typescript_resource_template() {
    let backend = TypeScriptBackend::new();

    let resource = ResourceType {
        name: "Patient".to_string(),
        base: Some("DomainResource".to_string()),
        properties: vec![
            Property {
                name: "active".to_string(),
                path: "Patient.active".to_string(),
                property_type: PropertyType::Primitive {
                    type_name: "boolean".to_string(),
                },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Whether this patient's record is in active use".to_string(),
                definition: "Whether this patient's record is in active use".to_string(),
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
                definition: "A name associated with the patient".to_string(),
                comments: None,
                examples: vec![],
            },
        ],
        search_parameters: vec![],
        documentation: Documentation {
            short: "Information about an individual or animal receiving care".to_string(),
            definition: "Demographics and other administrative information about an individual or animal receiving care or other health-related services.".to_string(),
            url: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
            ..Default::default()
        },
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        is_abstract: false,
    };

    let output = TypeScriptTemplates::resource_to_interface(&resource, &backend).unwrap();

    assert_snapshot!("resource_patient", output);
}

#[test]
fn test_typescript_datatype_template() {
    let backend = TypeScriptBackend::new();

    let datatype = DataType {
        name: "HumanName".to_string(),
        base: Some("Element".to_string()),
        properties: vec![
            Property {
                name: "use".to_string(),
                path: "HumanName.use".to_string(),
                property_type: PropertyType::Primitive { type_name: "code".to_string() },
                cardinality: CardinalityRange::optional(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "usual | official | temp | nickname | anonymous | old | maiden"
                    .to_string(),
                definition: "Identifies the purpose for this name".to_string(),
                comments: None,
                examples: vec![],
            },
            Property {
                name: "family".to_string(),
                path: "HumanName.family".to_string(),
                property_type: PropertyType::Primitive { type_name: "string".to_string() },
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
                property_type: PropertyType::Primitive { type_name: "string".to_string() },
                cardinality: CardinalityRange::optional_array(),
                is_choice: false,
                choice_types: vec![],
                is_modifier: false,
                is_summary: true,
                binding: None,
                constraints: vec![],
                short_description: "Given names (not always 'first'). Includes middle names"
                    .to_string(),
                definition: "Given name".to_string(),
                comments: None,
                examples: vec![],
            },
        ],
        documentation: Documentation {
            short: "Name of a human".to_string(),
            definition: "A human's name with the ability to identify parts and usage.".to_string(),
            ..Default::default()
        },
        url: "http://hl7.org/fhir/StructureDefinition/HumanName".to_string(),
        is_abstract: false,
    };

    let output = TypeScriptTemplates::datatype_to_interface(&datatype, &backend).unwrap();

    assert_snapshot!("datatype_humanname", output);
}

#[test]
fn test_generate_index_file() {
    let exports =
        vec!["Patient".to_string(), "Observation".to_string(), "Practitioner".to_string()];

    let output = TypeScriptTemplates::generate_index(&exports).unwrap();

    assert_snapshot!("index_file", output);
}

#[test]
fn test_generate_package_json() {
    let manifest = TypeScriptTemplates::generate_package_json(
        "@octofhir/r4",
        "1.0.0",
        "FHIR R4 TypeScript SDK",
    );

    assert_eq!(manifest["name"], "@octofhir/r4");
    assert_eq!(manifest["version"], "1.0.0");
    assert_eq!(manifest["description"], "FHIR R4 TypeScript SDK");
    assert!(manifest["devDependencies"]["typescript"].is_string());
}

#[test]
fn test_object_type_helper() {
    let props = vec![
        ("id".to_string(), "string".to_string(), true),
        ("value".to_string(), "number".to_string(), false),
    ];

    let tokens = helpers::object_type(&props);
    let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

    assert_snapshot!("object_type", output);
}
