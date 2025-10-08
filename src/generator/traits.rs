//! Core traits for code generation

use crate::core::ir::TypeGraph;
use crate::core::{Error, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main trait for code generators
#[async_trait]
pub trait CodeGenerator: Send + Sync {
    /// Generate code from a type graph
    ///
    /// # Arguments
    ///
    /// * `graph` - The type graph to generate from
    /// * `config` - Generator configuration
    ///
    /// # Returns
    ///
    /// Generated code with all files and metadata
    async fn generate(&self, graph: &TypeGraph, config: &GeneratorConfig) -> Result<GeneratedCode>;

    /// Get the target language for this generator
    fn language(&self) -> Language;

    /// Get generator metadata and capabilities
    fn metadata(&self) -> GeneratorMetadata;

    /// Validate configuration before generation
    ///
    /// This is called before `generate()` to catch configuration errors early
    fn validate_config(&self, config: &GeneratorConfig) -> Result<()> {
        let _ = config;
        Ok(())
    }

    /// Get default configuration for this generator
    fn default_config(&self) -> GeneratorConfig {
        GeneratorConfig::default()
    }
}

/// Language backend for type mapping and code formatting
pub trait LanguageBackend: Send + Sync {
    /// Map an IR property type to target language type
    ///
    /// # Example
    ///
    /// - TypeScript: `PropertyType::Primitive("string")` → `"string"`
    /// - Rust: `PropertyType::Primitive("string")` → `"String"`
    fn map_type(&self, property_type: &crate::core::ir::PropertyType) -> String;

    /// Generate import/using statements for dependencies
    ///
    /// # Example
    ///
    /// TypeScript: `["Address", "HumanName"]` → `import { Address, HumanName } from './types';`
    fn generate_imports(&self, dependencies: &[String]) -> Vec<String>;

    /// Format an identifier according to language conventions
    ///
    /// # Example
    ///
    /// - TypeScript interface: `"patient_name"` → `"PatientName"`
    /// - Rust struct: `"patient_name"` → `"PatientName"`
    /// - TypeScript field: `"birth_date"` → `"birthDate"`
    fn format_identifier(&self, name: &str, context: IdentifierContext) -> String;

    /// Generate documentation comment
    ///
    /// # Example
    ///
    /// TypeScript: `/** Documentation here */`
    /// Rust: `/// Documentation here`
    fn generate_doc_comment(&self, doc: &crate::core::ir::Documentation) -> Vec<String>;

    /// Get file extension for this language
    fn file_extension(&self) -> &str;

    /// Format generated code (run prettier, rustfmt, etc.)
    fn format_code(&self, code: &str) -> Result<String> {
        // Default: no formatting
        Ok(code.to_string())
    }
}

/// Template engine abstraction
#[async_trait]
pub trait TemplateEngine: Send + Sync {
    /// Render a template with context
    ///
    /// # Arguments
    ///
    /// * `template_name` - Name/identifier of the template
    /// * `context` - Data to pass to template
    ///
    /// # Returns
    ///
    /// Rendered string
    async fn render(&self, template_name: &str, context: &TemplateContext) -> Result<String>;

    /// Check if a template exists
    fn has_template(&self, template_name: &str) -> bool;

    /// List all available templates
    fn list_templates(&self) -> Vec<String>;
}

/// Context passed to templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateContext {
    /// Main data
    pub data: serde_json::Value,

    /// Additional helpers/variables
    pub variables: HashMap<String, serde_json::Value>,
}

impl TemplateContext {
    /// Create new context with data
    pub fn new(data: serde_json::Value) -> Self {
        Self { data, variables: HashMap::new() }
    }

    /// Add a variable to context
    pub fn with_variable(mut self, key: String, value: serde_json::Value) -> Self {
        self.variables.insert(key, value);
        self
    }
}

/// Supported target languages
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// TypeScript
    TypeScript,
    /// Rust
    Rust,
    /// Python
    Python,
    /// Java
    Java,
    /// Go
    Go,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::TypeScript => write!(f, "TypeScript"),
            Language::Rust => write!(f, "Rust"),
            Language::Python => write!(f, "Python"),
            Language::Java => write!(f, "Java"),
            Language::Go => write!(f, "Go"),
        }
    }
}

impl std::str::FromStr for Language {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "typescript" | "ts" => Ok(Language::TypeScript),
            "rust" | "rs" => Ok(Language::Rust),
            "python" | "py" => Ok(Language::Python),
            "java" => Ok(Language::Java),
            "go" | "golang" => Ok(Language::Go),
            _ => Err(Error::Generator(format!("Unknown language: {}", s))),
        }
    }
}

/// Generator metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorMetadata {
    /// Generator name
    pub name: String,

    /// Generator version
    pub version: String,

    /// Target language
    pub language: Language,

    /// Description
    pub description: String,

    /// Author
    pub author: String,

    /// Capabilities
    pub capabilities: GeneratorCapabilities,
}

/// Generator capabilities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneratorCapabilities {
    /// Can generate validation code
    pub validation: bool,

    /// Can generate helper methods
    pub helpers: bool,

    /// Can generate tests
    pub tests: bool,

    /// Can generate documentation
    pub documentation: bool,

    /// Can generate search parameters
    pub search_parameters: bool,

    /// Supports profiles
    pub profiles: bool,

    /// Supports extensions
    pub extensions: bool,
}

/// Configuration for code generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    /// Output directory
    pub output_dir: PathBuf,

    /// Module/package name
    pub module_name: Option<String>,

    /// Emit validation code
    pub emit_validation: bool,

    /// Emit helper methods
    pub emit_helpers: bool,

    /// Emit tests
    pub emit_tests: bool,

    /// Package/module version
    pub package_version: String,

    /// Clean output directory before generation
    pub clean_output: bool,

    /// Custom options (language-specific)
    pub custom_options: HashMap<String, serde_json::Value>,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./generated"),
            module_name: None,
            emit_validation: true,
            emit_helpers: true,
            emit_tests: false,
            package_version: "1.0.0".to_string(),
            clean_output: false,
            custom_options: HashMap::new(),
        }
    }
}

impl GeneratorConfig {
    /// Get a custom option
    pub fn get_custom<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.custom_options.get(key).and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Set a custom option
    pub fn set_custom<T: Serialize>(&mut self, key: String, value: T) -> Result<()> {
        let json_value = serde_json::to_value(value)
            .map_err(|e| Error::Generator(format!("Failed to serialize custom option: {}", e)))?;
        self.custom_options.insert(key, json_value);
        Ok(())
    }
}

/// Generated code output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCode {
    /// Generated files
    pub files: Vec<GeneratedFile>,

    /// Generation manifest
    pub manifest: GenerationManifest,
}

impl GeneratedCode {
    /// Create new generated code
    pub fn new(files: Vec<GeneratedFile>, manifest: GenerationManifest) -> Self {
        Self { files, manifest }
    }

    /// Get total number of files
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Get total size in bytes
    pub fn total_size(&self) -> usize {
        self.files.iter().map(|f| f.content.len()).sum()
    }
}

/// A single generated file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    /// Relative path from output directory
    pub path: PathBuf,

    /// File content
    pub content: String,

    /// File type/category
    pub file_type: FileType,
}

impl GeneratedFile {
    /// Create a new generated file
    pub fn new(path: PathBuf, content: String, file_type: FileType) -> Self {
        Self { path, content, file_type }
    }
}

/// Type of generated file
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileType {
    /// Resource type definition
    Resource,
    /// Datatype definition
    DataType,
    /// Primitive type definition
    Primitive,
    /// Profile definition
    Profile,
    /// Helper/utility code
    Helper,
    /// Validation code
    Validation,
    /// Test file
    Test,
    /// Documentation
    Documentation,
    /// Package manifest (package.json, Cargo.toml, etc.)
    Manifest,
    /// Index/entry point
    Index,
    /// Other
    Other,
}

/// Generation manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationManifest {
    /// When generation occurred
    pub generated_at: String,

    /// Generator metadata
    pub generator: GeneratorMetadata,

    /// Configuration used
    pub config: GeneratorConfig,

    /// Statistics
    pub statistics: GenerationStatistics,

    /// Warnings/notes
    pub warnings: Vec<String>,
}

/// Generation statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenerationStatistics {
    /// Number of resources generated
    pub resources: usize,

    /// Number of datatypes generated
    pub datatypes: usize,

    /// Number of primitives generated
    pub primitives: usize,

    /// Number of profiles generated
    pub profiles: usize,

    /// Total files generated
    pub total_files: usize,

    /// Total lines of code
    pub total_lines: usize,

    /// Generation time (milliseconds)
    pub generation_time_ms: u64,
}

/// Identifier context (for formatting)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentifierContext {
    /// Type name (class, interface, struct)
    TypeName,
    /// Field/property name
    FieldName,
    /// Function/method name
    FunctionName,
    /// Constant name
    ConstantName,
    /// Variable name
    VariableName,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_display() {
        assert_eq!(Language::TypeScript.to_string(), "TypeScript");
        assert_eq!(Language::Rust.to_string(), "Rust");
        assert_eq!(Language::Python.to_string(), "Python");
    }

    #[test]
    fn test_language_from_str() {
        assert_eq!("typescript".parse::<Language>().unwrap(), Language::TypeScript);
        assert_eq!("ts".parse::<Language>().unwrap(), Language::TypeScript);
        assert_eq!("rust".parse::<Language>().unwrap(), Language::Rust);
        assert_eq!("rs".parse::<Language>().unwrap(), Language::Rust);
        assert!("unknown".parse::<Language>().is_err());
    }

    #[test]
    fn test_generator_config_default() {
        let config = GeneratorConfig::default();
        assert_eq!(config.output_dir, PathBuf::from("./generated"));
        assert!(config.emit_validation);
        assert!(config.emit_helpers);
        assert!(!config.emit_tests);
    }

    #[test]
    fn test_generator_config_custom_options() {
        let mut config = GeneratorConfig::default();

        config.set_custom("indent".to_string(), 2).unwrap();
        config.set_custom("semicolons".to_string(), true).unwrap();

        let indent: i32 = config.get_custom("indent").unwrap();
        assert_eq!(indent, 2);

        let semicolons: bool = config.get_custom("semicolons").unwrap();
        assert!(semicolons);

        let missing: Option<String> = config.get_custom("missing");
        assert!(missing.is_none());
    }

    #[test]
    fn test_template_context() {
        let ctx = TemplateContext::new(serde_json::json!({"name": "Patient"}))
            .with_variable("version".to_string(), serde_json::json!("1.0.0"));

        assert_eq!(ctx.data, serde_json::json!({"name": "Patient"}));
        assert_eq!(ctx.variables.get("version").unwrap(), &serde_json::json!("1.0.0"));
    }

    #[test]
    fn test_generated_code_stats() {
        let files = vec![
            GeneratedFile::new(
                PathBuf::from("patient.ts"),
                "export interface Patient {}".to_string(),
                FileType::Resource,
            ),
            GeneratedFile::new(
                PathBuf::from("index.ts"),
                "export * from './patient';".to_string(),
                FileType::Index,
            ),
        ];

        let manifest = GenerationManifest {
            generated_at: "2025-01-01T00:00:00Z".to_string(),
            generator: GeneratorMetadata {
                name: "test".to_string(),
                version: "1.0.0".to_string(),
                language: Language::TypeScript,
                description: "Test generator".to_string(),
                author: "Test".to_string(),
                capabilities: GeneratorCapabilities::default(),
            },
            config: GeneratorConfig::default(),
            statistics: GenerationStatistics::default(),
            warnings: Vec::new(),
        };

        let generated = GeneratedCode::new(files, manifest);

        assert_eq!(generated.file_count(), 2);
        assert!(generated.total_size() > 0);
    }

    #[test]
    fn test_generator_capabilities_default() {
        let caps = GeneratorCapabilities::default();
        assert!(!caps.validation);
        assert!(!caps.helpers);
        assert!(!caps.tests);
    }
}
