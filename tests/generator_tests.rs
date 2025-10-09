use async_trait::async_trait;
use octofhir_codegen::Result;
use octofhir_codegen::core::ir::*;
use octofhir_codegen::generator::*;
use std::path::PathBuf;

/// Mock generator for testing
pub struct MockGenerator {
    language: Language,
}

impl MockGenerator {
    pub fn new(language: Language) -> Self {
        Self { language }
    }
}

#[async_trait]
impl CodeGenerator for MockGenerator {
    async fn generate(&self, graph: &TypeGraph, config: &GeneratorConfig) -> Result<GeneratedCode> {
        // Generate a simple mock file
        let file = GeneratedFile::new(
            PathBuf::from("mock.txt"),
            format!("Generated {} types for {}", graph.total_types(), self.language),
            FileType::Other,
        );

        let manifest = GenerationManifest {
            generated_at: chrono::Utc::now().to_rfc3339(),
            generator: self.metadata(),
            config: config.clone(),
            statistics: GenerationStatistics {
                resources: graph.resources.len(),
                datatypes: graph.datatypes.len(),
                primitives: graph.primitives.len(),
                profiles: graph.profiles.len(),
                total_files: 1,
                total_lines: 1,
                generation_time_ms: 0,
            },
            warnings: vec![],
        };

        Ok(GeneratedCode::new(vec![file], manifest))
    }

    fn language(&self) -> Language {
        self.language
    }

    fn metadata(&self) -> GeneratorMetadata {
        GeneratorMetadata {
            name: "MockGenerator".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            language: self.language,
            description: "Mock generator for testing".to_string(),
            author: "OctoFHIR Team".to_string(),
            capabilities: GeneratorCapabilities {
                validation: false,
                helpers: false,
                tests: false,
                documentation: false,
                search_parameters: false,
                profiles: false,
                extensions: false,
            },
        }
    }
}

/// Mock language backend for testing
pub struct MockBackend;

impl LanguageBackend for MockBackend {
    fn map_type(&self, property_type: &PropertyType) -> String {
        match property_type {
            PropertyType::Primitive { type_name } => type_name.clone(),
            PropertyType::Complex { type_name } => type_name.clone(),
            PropertyType::Reference { target_types } => {
                if target_types.is_empty() {
                    "Reference".to_string()
                } else {
                    format!("Reference<{}>", target_types.join(" | "))
                }
            }
            PropertyType::BackboneElement { .. } => "BackboneElement".to_string(),
            PropertyType::Choice { types } => types.join(" | "),
        }
    }

    fn generate_imports(&self, dependencies: &[String]) -> Vec<String> {
        if dependencies.is_empty() {
            vec![]
        } else {
            vec![format!("import {{ {} }};", dependencies.join(", "))]
        }
    }

    fn format_identifier(&self, name: &str, context: IdentifierContext) -> String {
        match context {
            IdentifierContext::TypeName => name.to_string(),
            IdentifierContext::FieldName => name.to_lowercase(),
            _ => name.to_string(),
        }
    }

    fn generate_doc_comment(&self, doc: &Documentation) -> Vec<String> {
        if doc.short.is_empty() { vec![] } else { vec![format!("// {}", doc.short)] }
    }

    fn file_extension(&self) -> &str {
        ".mock"
    }
}

/// Mock template engine for testing
pub struct MockTemplateEngine;

#[async_trait]
impl TemplateEngine for MockTemplateEngine {
    async fn render(&self, template_name: &str, context: &TemplateContext) -> Result<String> {
        Ok(format!("Template: {}\nData: {:?}", template_name, context.data))
    }

    fn has_template(&self, template_name: &str) -> bool {
        template_name == "test_template"
    }

    fn list_templates(&self) -> Vec<String> {
        vec!["test_template".to_string()]
    }
}

// Tests

#[tokio::test]
async fn test_mock_generator() {
    let generator = MockGenerator::new(Language::TypeScript);

    assert_eq!(generator.language(), Language::TypeScript);

    let meta = generator.metadata();
    assert_eq!(meta.name, "MockGenerator");
    assert_eq!(meta.language, Language::TypeScript);
}

#[tokio::test]
async fn test_mock_generator_generate() {
    let generator = MockGenerator::new(Language::Rust);
    let mut graph = TypeGraph::new(FhirVersion::R4);

    // Add a test resource
    graph.add_resource(
        "Patient".to_string(),
        ResourceType {
            name: "Patient".to_string(),
            base: Some("DomainResource".to_string()),
            properties: vec![],
            search_parameters: vec![],
            extensions: vec![],
            documentation: Documentation::default(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            is_abstract: false,
        },
    );

    let config = GeneratorConfig::default();
    let result = generator.generate(&graph, &config).await.unwrap();

    assert_eq!(result.file_count(), 1);
    assert_eq!(result.manifest.statistics.resources, 1);
    assert!(result.files[0].content.contains("Generated 1 types for Rust"));
}

#[test]
fn test_mock_backend_map_type() {
    let backend = MockBackend;

    let prim = PropertyType::Primitive { type_name: "string".to_string() };
    assert_eq!(backend.map_type(&prim), "string");

    let complex = PropertyType::Complex { type_name: "HumanName".to_string() };
    assert_eq!(backend.map_type(&complex), "HumanName");

    let reference = PropertyType::Reference {
        target_types: vec!["Patient".to_string(), "Practitioner".to_string()],
    };
    assert_eq!(backend.map_type(&reference), "Reference<Patient | Practitioner>");
}

#[test]
fn test_mock_backend_format_identifier() {
    let backend = MockBackend;

    let type_name = backend.format_identifier("PatientName", IdentifierContext::TypeName);
    assert_eq!(type_name, "PatientName");

    let field_name = backend.format_identifier("BirthDate", IdentifierContext::FieldName);
    assert_eq!(field_name, "birthdate");
}

#[test]
fn test_mock_backend_generate_imports() {
    let backend = MockBackend;

    let imports = backend.generate_imports(&["Patient".to_string(), "Observation".to_string()]);
    assert_eq!(imports.len(), 1);
    assert!(imports[0].contains("Patient"));
    assert!(imports[0].contains("Observation"));

    let empty = backend.generate_imports(&[]);
    assert!(empty.is_empty());
}

#[test]
fn test_mock_backend_file_extension() {
    let backend = MockBackend;
    assert_eq!(backend.file_extension(), ".mock");
}

#[tokio::test]
async fn test_mock_template_engine() {
    let engine = MockTemplateEngine;

    assert!(engine.has_template("test_template"));
    assert!(!engine.has_template("nonexistent"));

    let templates = engine.list_templates();
    assert_eq!(templates, vec!["test_template"]);

    let context = TemplateContext::new(serde_json::json!({"name": "Patient"}));
    let rendered = engine.render("test_template", &context).await.unwrap();

    assert!(rendered.contains("Template: test_template"));
    assert!(rendered.contains("Patient"));
}
