//! Configuration management for OctoFHIR Codegen
//!
//! This module handles parsing and validation of the `codegen.toml` configuration file.
//! It supports environment variable overrides and provides sensible defaults.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Main configuration structure for OctoFHIR Codegen
///
/// This structure represents the complete configuration from `codegen.toml`.
/// All sections are optional with sensible defaults.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct CodegenConfig {
    /// Project metadata
    #[serde(default)]
    pub project: ProjectConfig,

    /// FHIR-specific configuration
    #[serde(default)]
    pub fhir: FhirConfig,

    /// Output directory and options
    #[serde(default)]
    pub output: OutputConfig,

    /// Language generator configurations
    #[serde(default)]
    pub generators: GeneratorsConfig,

    /// Canonical manager integration settings
    #[serde(default)]
    pub canonical_manager: Option<CanonicalManagerConfig>,

    /// Logging configuration
    #[serde(default)]
    pub logging: LoggingConfig,
}

/// Project metadata configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectConfig {
    /// Project name
    #[serde(default = "default_project_name")]
    pub name: String,

    /// Project version (semver)
    #[serde(default = "default_version")]
    pub version: String,

    /// Project description
    pub description: Option<String>,

    /// Authors
    #[serde(default)]
    pub authors: Vec<String>,

    /// License identifier (e.g., "MIT", "Apache-2.0")
    pub license: Option<String>,

    /// Repository URL
    pub repository: Option<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: default_project_name(),
            version: default_version(),
            description: None,
            authors: Vec::new(),
            license: None,
            repository: None,
        }
    }
}

/// FHIR-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FhirConfig {
    /// FHIR version to target (R4, R5, R6)
    #[serde(default = "default_fhir_version")]
    pub version: String,

    /// FHIR packages to include (e.g., "hl7.fhir.r4.core@4.0.1")
    #[serde(default)]
    pub packages: Vec<String>,

    /// Specific resource types to generate (if empty, generates all)
    /// Example: ["Patient", "Observation", "Condition"]
    #[serde(default)]
    pub include_resources: Vec<String>,

    /// Resource types to exclude from generation (applied after include_resources)
    /// Note: If include_resources is set, this is typically not needed
    #[serde(default)]
    pub exclude_resources: Vec<String>,

    /// Specific profiles to generate (optional, generates all if omitted)
    #[serde(default)]
    pub include_profiles: Vec<String>,

    /// Extension URLs to include
    #[serde(default)]
    pub include_extensions: Vec<String>,

    /// Whether to generate only core resources (no profiles or extensions)
    #[serde(default)]
    pub core_only: bool,
}

impl Default for FhirConfig {
    fn default() -> Self {
        Self {
            version: default_fhir_version(),
            packages: Vec::new(),
            include_resources: Vec::new(),
            exclude_resources: Vec::new(),
            include_profiles: Vec::new(),
            include_extensions: Vec::new(),
            core_only: false,
        }
    }
}

/// Output directory and file management configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutputConfig {
    /// Output directory for generated code
    #[serde(default = "default_output_directory")]
    pub directory: PathBuf,

    /// Clean output directory before generation
    #[serde(default = "default_true")]
    pub clean: bool,

    /// Create directory if it doesn't exist
    #[serde(default = "default_true")]
    pub create_if_missing: bool,

    /// Overwrite existing files
    #[serde(default = "default_true")]
    pub overwrite: bool,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            directory: default_output_directory(),
            clean: true,
            create_if_missing: true,
            overwrite: true,
        }
    }
}

/// Language generator configurations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GeneratorsConfig {
    /// TypeScript generator configuration
    pub typescript: Option<TypeScriptGeneratorConfig>,

    /// Rust generator configuration
    pub rust: Option<RustGeneratorConfig>,

    /// Python generator configuration
    pub python: Option<PythonGeneratorConfig>,

    /// Java generator configuration
    pub java: Option<JavaGeneratorConfig>,
}

/// TypeScript generator-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeScriptGeneratorConfig {
    /// Enable/disable TypeScript generator
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Module/package name
    pub module_name: Option<String>,

    /// Generate validation functions
    #[serde(default = "default_true")]
    pub emit_validation: bool,

    /// Generate helper methods
    #[serde(default = "default_true")]
    pub emit_helpers: bool,

    /// Generate test files
    #[serde(default)]
    pub emit_tests: bool,

    /// TypeScript version to target
    #[serde(default = "default_ts_version")]
    pub target_version: String,

    /// Custom type mappings (FHIR type -> TypeScript type)
    #[serde(default)]
    pub type_mappings: HashMap<String, String>,

    /// Additional compiler options for generated tsconfig.json
    #[serde(default)]
    pub compiler_options: HashMap<String, serde_json::Value>,
}

impl Default for TypeScriptGeneratorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            module_name: None,
            emit_validation: true,
            emit_helpers: true,
            emit_tests: false,
            target_version: default_ts_version(),
            type_mappings: HashMap::new(),
            compiler_options: HashMap::new(),
        }
    }
}

/// Rust generator-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RustGeneratorConfig {
    /// Enable/disable Rust generator
    #[serde(default)]
    pub enabled: bool,

    /// Crate name
    pub crate_name: Option<String>,

    /// Generate validation implementations
    #[serde(default = "default_true")]
    pub emit_validation: bool,

    /// Generate helper methods
    #[serde(default = "default_true")]
    pub emit_helpers: bool,

    /// Generate test modules
    #[serde(default)]
    pub emit_tests: bool,

    /// Rust edition (2021, 2024)
    #[serde(default = "default_rust_edition")]
    pub edition: String,

    /// Additional derives to include
    #[serde(default)]
    pub additional_derives: Vec<String>,
}

impl Default for RustGeneratorConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            crate_name: None,
            emit_validation: true,
            emit_helpers: true,
            emit_tests: false,
            edition: default_rust_edition(),
            additional_derives: Vec::new(),
        }
    }
}

/// Python generator-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PythonGeneratorConfig {
    /// Enable/disable Python generator
    #[serde(default)]
    pub enabled: bool,

    /// Package name
    pub package_name: Option<String>,

    /// Generate Pydantic models
    #[serde(default = "default_true")]
    pub use_pydantic: bool,

    /// Generate type stubs (.pyi files)
    #[serde(default = "default_true")]
    pub generate_stubs: bool,

    /// Python version to target
    #[serde(default = "default_python_version")]
    pub target_version: String,
}

impl Default for PythonGeneratorConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            package_name: None,
            use_pydantic: true,
            generate_stubs: true,
            target_version: default_python_version(),
        }
    }
}

/// Java generator-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JavaGeneratorConfig {
    /// Enable/disable Java generator
    #[serde(default)]
    pub enabled: bool,

    /// Package name (e.g., "com.example.fhir")
    pub package_name: Option<String>,

    /// Java version to target
    #[serde(default = "default_java_version")]
    pub target_version: String,

    /// Use Jackson for JSON serialization
    #[serde(default = "default_true")]
    pub use_jackson: bool,

    /// Generate validation annotations
    #[serde(default = "default_true")]
    pub emit_validation: bool,
}

impl Default for JavaGeneratorConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            package_name: None,
            target_version: default_java_version(),
            use_jackson: true,
            emit_validation: true,
        }
    }
}

/// Canonical manager integration configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CanonicalManagerConfig {
    /// Path to canonical-manager configuration file
    pub config_path: Option<PathBuf>,

    /// Registry URL (overrides config)
    pub registry_url: Option<String>,

    /// Local cache directory
    pub cache_dir: Option<PathBuf>,

    /// Enable offline mode
    #[serde(default)]
    pub offline: bool,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub level: String,

    /// Log format (pretty, json, compact)
    #[serde(default = "default_log_format")]
    pub format: String,

    /// Enable colored output
    #[serde(default = "default_true")]
    pub color: bool,

    /// Log to file
    pub file: Option<PathBuf>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self { level: default_log_level(), format: default_log_format(), color: true, file: None }
    }
}

// ============================================================================
// Default value functions for serde
// ============================================================================

fn default_project_name() -> String {
    "fhir-sdk".to_string()
}

fn default_version() -> String {
    "0.1.0".to_string()
}

fn default_fhir_version() -> String {
    "R4".to_string()
}

fn default_output_directory() -> PathBuf {
    PathBuf::from("./generated")
}

fn default_true() -> bool {
    true
}

fn default_ts_version() -> String {
    "5.3".to_string()
}

fn default_rust_edition() -> String {
    "2024".to_string()
}

fn default_python_version() -> String {
    "3.11".to_string()
}

fn default_java_version() -> String {
    "17".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_format() -> String {
    "pretty".to_string()
}

// ============================================================================
// Implementation methods
// ============================================================================

impl CodegenConfig {
    /// Parse configuration from a TOML string
    pub fn from_toml_str(content: &str) -> Result<Self> {
        toml::from_str(content).context("Failed to parse TOML configuration")
    }

    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .context(format!("Failed to read config file: {:?}", path))?;
        Self::from_toml_str(&content)
    }

    /// Save configuration to a TOML file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let content = toml::to_string_pretty(self).context("Failed to serialize configuration")?;
        std::fs::write(path, content)
            .context(format!("Failed to write config file: {:?}", path))?;
        Ok(())
    }

    /// Serialize configuration to TOML string
    pub fn to_toml_string(&self) -> Result<String> {
        toml::to_string_pretty(self).context("Failed to serialize configuration")
    }

    /// Validate the configuration
    ///
    /// Checks for:
    /// - Valid FHIR version
    /// - At least one enabled generator
    /// - Valid output directory
    /// - Valid package specifications
    /// - Valid log level
    pub fn validate(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Validate FHIR version
        match self.fhir.version.as_str() {
            "R4" | "R5" | "R6" => {}
            v => {
                return Err(anyhow::anyhow!(
                    "Invalid FHIR version: '{}'. Must be one of: R4, R5, R6",
                    v
                ));
            }
        }

        // Check if at least one generator is enabled
        let has_enabled_generator = self.generators.typescript.as_ref().is_some_and(|g| g.enabled)
            || self.generators.rust.as_ref().is_some_and(|g| g.enabled)
            || self.generators.python.as_ref().is_some_and(|g| g.enabled)
            || self.generators.java.as_ref().is_some_and(|g| g.enabled);

        if !has_enabled_generator {
            warnings.push("No generators are enabled. Enable at least one generator.".to_string());
        }

        // Validate output directory is not empty
        if self.output.directory.as_os_str().is_empty() {
            return Err(anyhow::anyhow!("Output directory cannot be empty"));
        }

        // Validate FHIR packages format
        for package in &self.fhir.packages {
            if !package.contains('@') && !package.is_empty() {
                warnings.push(format!(
                    "Package '{}' may be missing version. Expected format: 'package@version'",
                    package
                ));
            }
        }

        // Validate resource filtering logic
        if !self.fhir.include_resources.is_empty() && !self.fhir.exclude_resources.is_empty() {
            warnings.push(
                "Both include_resources and exclude_resources are set. \
                 exclude_resources will be applied after include_resources. \
                 Consider using only include_resources for clarity."
                    .to_string(),
            );
        }

        // Validate log level
        match self.logging.level.to_lowercase().as_str() {
            "trace" | "debug" | "info" | "warn" | "error" => {}
            l => {
                return Err(anyhow::anyhow!(
                    "Invalid log level: '{}'. Must be one of: trace, debug, info, warn, error",
                    l
                ));
            }
        }

        // Validate log format
        match self.logging.format.to_lowercase().as_str() {
            "pretty" | "json" | "compact" => {}
            f => {
                return Err(anyhow::anyhow!(
                    "Invalid log format: '{}'. Must be one of: pretty, json, compact",
                    f
                ));
            }
        }

        // Validate TypeScript configuration
        if let Some(ref ts_config) = self.generators.typescript
            && ts_config.enabled
            && let Err(e) = Self::validate_semver(&ts_config.target_version)
        {
            warnings.push(format!("TypeScript target_version: {}", e));
        }

        // Validate Rust configuration
        if let Some(ref rust_config) = self.generators.rust
            && rust_config.enabled
        {
            match rust_config.edition.as_str() {
                "2015" | "2018" | "2021" | "2024" => {}
                e => {
                    return Err(anyhow::anyhow!(
                        "Invalid Rust edition: '{}'. Must be one of: 2015, 2018, 2021, 2024",
                        e
                    ));
                }
            }
        }

        // Validate Python configuration
        if let Some(ref py_config) = self.generators.python
            && py_config.enabled
            && let Err(e) = Self::validate_semver(&py_config.target_version)
        {
            warnings.push(format!("Python target_version: {}", e));
        }

        // Validate Java configuration
        if let Some(ref java_config) = self.generators.java
            && java_config.enabled
            && java_config.package_name.is_none()
        {
            warnings.push("Java generator enabled but package_name is not set".to_string());
        }

        Ok(warnings)
    }

    /// Apply environment variable overrides
    ///
    /// Supports the following environment variables:
    /// - `OCTOFHIR_FHIR_VERSION`: Override FHIR version
    /// - `OCTOFHIR_OUTPUT_DIR`: Override output directory
    /// - `OCTOFHIR_LOG_LEVEL`: Override log level
    /// - `OCTOFHIR_OFFLINE`: Enable offline mode for canonical manager
    pub fn apply_env_overrides(&mut self) {
        if let Ok(version) = std::env::var("OCTOFHIR_FHIR_VERSION") {
            self.fhir.version = version;
        }

        if let Ok(output_dir) = std::env::var("OCTOFHIR_OUTPUT_DIR") {
            self.output.directory = PathBuf::from(output_dir);
        }

        if let Ok(log_level) = std::env::var("OCTOFHIR_LOG_LEVEL") {
            self.logging.level = log_level;
        }

        if let Ok(offline) = std::env::var("OCTOFHIR_OFFLINE")
            && let Some(ref mut cm_config) = self.canonical_manager
        {
            cm_config.offline = offline.to_lowercase() == "true" || offline == "1";
        }
    }

    /// Merge with another configuration (other takes precedence)
    pub fn merge(&mut self, other: &CodegenConfig) {
        // Merge FHIR packages (append)
        self.fhir.packages.extend(other.fhir.packages.clone());

        // Override primitives if set in other
        if other.fhir.version != default_fhir_version() {
            self.fhir.version.clone_from(&other.fhir.version);
        }

        if other.output.directory != default_output_directory() {
            self.output.directory.clone_from(&other.output.directory);
        }

        // Merge generator configs (other takes precedence if Some)
        if other.generators.typescript.is_some() {
            self.generators.typescript = other.generators.typescript.clone();
        }
        if other.generators.rust.is_some() {
            self.generators.rust = other.generators.rust.clone();
        }
        if other.generators.python.is_some() {
            self.generators.python = other.generators.python.clone();
        }
        if other.generators.java.is_some() {
            self.generators.java = other.generators.java.clone();
        }
    }

    /// Validate semantic version format
    fn validate_semver(version: &str) -> Result<()> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.is_empty() || parts.len() > 3 {
            return Err(anyhow::anyhow!(
                "Invalid version format: '{}'. Expected semantic versioning (e.g., '1.0.0' or '5.3')",
                version
            ));
        }
        Ok(())
    }

    /// Create a default configuration with sensible values
    pub fn default_with_typescript() -> Self {
        let mut config = Self::default();
        config.generators.typescript = Some(TypeScriptGeneratorConfig::default());
        config
    }

    /// Create a configuration template for a specific language
    pub fn template_for(language: &str) -> Result<Self> {
        let mut config = Self::default();

        // Set basic project metadata
        config.project.name = format!("fhir-{}-sdk", language);
        config.project.version = "0.1.0".to_string();
        config.project.description =
            Some(format!("FHIR {} SDK generated by OctoFHIR Codegen", language));

        // Set FHIR configuration
        config.fhir.version = "R4".to_string();
        config.fhir.packages = vec!["hl7.fhir.r4.core@4.0.1".to_string()];

        // Enable the appropriate generator
        match language.to_lowercase().as_str() {
            "typescript" | "ts" => {
                config.generators.typescript = Some(TypeScriptGeneratorConfig {
                    module_name: Some("fhir-r4".to_string()),
                    ..Default::default()
                });
            }
            "rust" | "rs" => {
                config.generators.rust = Some(RustGeneratorConfig {
                    enabled: true,
                    crate_name: Some("fhir-r4".to_string()),
                    ..Default::default()
                });
            }
            "python" | "py" => {
                config.generators.python = Some(PythonGeneratorConfig {
                    enabled: true,
                    package_name: Some("fhir_r4".to_string()),
                    ..Default::default()
                });
            }
            "java" => {
                config.generators.java = Some(JavaGeneratorConfig {
                    enabled: true,
                    package_name: Some("com.example.fhir.r4".to_string()),
                    ..Default::default()
                });
            }
            "multi" | "all" => {
                // Enable all generators
                config.generators.typescript = Some(TypeScriptGeneratorConfig::default());
                config.generators.rust = Some(RustGeneratorConfig {
                    enabled: true,
                    ..Default::default()
                });
                config.generators.python = Some(PythonGeneratorConfig {
                    enabled: true,
                    ..Default::default()
                });
                config.generators.java = Some(JavaGeneratorConfig {
                    enabled: true,
                    package_name: Some("com.example.fhir.r4".to_string()),
                    ..Default::default()
                });
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Unknown language template: '{}'. Available templates: typescript, rust, python, java, multi",
                    language
                ));
            }
        }

        Ok(config)
    }

    /// Generate a commented TOML configuration template
    pub fn to_commented_toml(&self) -> Result<String> {
        let mut output = String::new();

        output.push_str("# OctoFHIR Codegen Configuration\n");
        output.push_str("# For full documentation, visit: https://github.com/octofhir/codegen\n\n");

        output.push_str("[project]\n");
        output.push_str(&format!("name = \"{}\"\n", self.project.name));
        output.push_str(&format!("version = \"{}\"\n", self.project.version));
        if let Some(ref desc) = self.project.description {
            output.push_str(&format!("description = \"{}\"\n", desc));
        }
        output.push_str("# authors = [\"Your Name <email@example.com>\"]\n");
        output.push_str("# license = \"MIT\"\n");
        output.push_str("# repository = \"https://github.com/username/repo\"\n\n");

        output.push_str("[fhir]\n");
        output.push_str(&format!("version = \"{}\"\n", self.fhir.version));
        output.push_str("packages = [\n");
        for pkg in &self.fhir.packages {
            output.push_str(&format!("  \"{}\",\n", pkg));
        }
        output.push_str("]\n");
        output.push_str("# include_resources = [\"Patient\", \"Observation\", \"Condition\"]\n");
        output.push_str("# exclude_resources = [\"Binary\"]\n");
        output.push_str("# core_only = false\n\n");

        output.push_str("[output]\n");
        output.push_str(&format!("directory = \"{}\"\n", self.output.directory.display()));
        output.push_str(&format!("clean = {}\n", self.output.clean));
        output.push_str(&format!("create_if_missing = {}\n", self.output.create_if_missing));
        output.push_str(&format!("overwrite = {}\n\n", self.output.overwrite));

        // TypeScript generator
        if let Some(ref ts) = self.generators.typescript {
            output.push_str("[generators.typescript]\n");
            output.push_str(&format!("enabled = {}\n", ts.enabled));
            if let Some(ref module) = ts.module_name {
                output.push_str(&format!("module_name = \"{}\"\n", module));
            }
            output.push_str(&format!("emit_validation = {}\n", ts.emit_validation));
            output.push_str(&format!("emit_helpers = {}\n", ts.emit_helpers));
            output.push_str(&format!("emit_tests = {}\n", ts.emit_tests));
            output.push_str(&format!("target_version = \"{}\"\n\n", ts.target_version));
        }

        // Rust generator
        if let Some(ref rust) = self.generators.rust {
            output.push_str("[generators.rust]\n");
            output.push_str(&format!("enabled = {}\n", rust.enabled));
            if let Some(ref crate_name) = rust.crate_name {
                output.push_str(&format!("crate_name = \"{}\"\n", crate_name));
            }
            output.push_str(&format!("emit_validation = {}\n", rust.emit_validation));
            output.push_str(&format!("emit_helpers = {}\n", rust.emit_helpers));
            output.push_str(&format!("emit_tests = {}\n", rust.emit_tests));
            output.push_str(&format!("edition = \"{}\"\n\n", rust.edition));
        }

        // Python generator
        if let Some(ref py) = self.generators.python {
            output.push_str("[generators.python]\n");
            output.push_str(&format!("enabled = {}\n", py.enabled));
            if let Some(ref package) = py.package_name {
                output.push_str(&format!("package_name = \"{}\"\n", package));
            }
            output.push_str(&format!("use_pydantic = {}\n", py.use_pydantic));
            output.push_str(&format!("generate_stubs = {}\n", py.generate_stubs));
            output.push_str(&format!("target_version = \"{}\"\n\n", py.target_version));
        }

        // Java generator
        if let Some(ref java) = self.generators.java {
            output.push_str("[generators.java]\n");
            output.push_str(&format!("enabled = {}\n", java.enabled));
            if let Some(ref package) = java.package_name {
                output.push_str(&format!("package_name = \"{}\"\n", package));
            }
            output.push_str(&format!("target_version = \"{}\"\n", java.target_version));
            output.push_str(&format!("use_jackson = {}\n", java.use_jackson));
            output.push_str(&format!("emit_validation = {}\n\n", java.emit_validation));
        }

        output.push_str("[logging]\n");
        output.push_str(&format!("level = \"{}\"\n", self.logging.level));
        output.push_str(&format!("format = \"{}\"\n", self.logging.format));
        output.push_str(&format!("color = {}\n", self.logging.color));
        output.push_str("# file = \"codegen.log\"\n");

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CodegenConfig::default();
        assert_eq!(config.project.name, "fhir-sdk");
        assert_eq!(config.fhir.version, "R4");
        assert_eq!(config.output.directory, PathBuf::from("./generated"));
        assert_eq!(config.logging.level, "info");
    }

    #[test]
    fn test_parse_minimal_config() {
        let toml = r#"
            [project]
            name = "my-sdk"
        "#;

        let config = CodegenConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.project.name, "my-sdk");
        assert_eq!(config.fhir.version, "R4"); // Default
    }

    #[test]
    fn test_parse_full_config() {
        let toml = r#"
            [project]
            name = "my-fhir-sdk"
            version = "1.0.0"
            description = "Custom FHIR SDK"

            [fhir]
            version = "R5"
            packages = ["hl7.fhir.r5.core@5.0.0"]

            [output]
            directory = "./dist"
            clean = true

            [generators.typescript]
            enabled = true
            module_name = "fhir-r5"
            emit_validation = true

            [logging]
            level = "debug"
        "#;

        let config = CodegenConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.project.name, "my-fhir-sdk");
        assert_eq!(config.fhir.version, "R5");
        assert_eq!(config.output.directory, PathBuf::from("./dist"));
        assert!(config.generators.typescript.is_some());
        assert_eq!(config.logging.level, "debug");
    }

    #[test]
    fn test_validation_success() {
        let mut config = CodegenConfig::default();
        config.generators.typescript = Some(TypeScriptGeneratorConfig::default());

        let warnings = config.validate().unwrap();
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_validation_invalid_fhir_version() {
        let mut config = CodegenConfig::default();
        config.fhir.version = "R3".to_string();

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid FHIR version"));
    }

    #[test]
    fn test_validation_no_generators() {
        let config = CodegenConfig::default();
        let warnings = config.validate().unwrap();
        assert!(!warnings.is_empty());
        assert!(warnings[0].contains("No generators are enabled"));
    }

    #[test]
    fn test_validation_invalid_log_level() {
        let mut config = CodegenConfig::default();
        config.logging.level = "invalid".to_string();

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid log level"));
    }

    #[test]
    fn test_env_overrides() {
        unsafe {
            std::env::set_var("OCTOFHIR_FHIR_VERSION", "R6");
            std::env::set_var("OCTOFHIR_OUTPUT_DIR", "/tmp/output");
            std::env::set_var("OCTOFHIR_LOG_LEVEL", "trace");
        }

        let mut config = CodegenConfig::default();
        config.apply_env_overrides();

        assert_eq!(config.fhir.version, "R6");
        assert_eq!(config.output.directory, PathBuf::from("/tmp/output"));
        assert_eq!(config.logging.level, "trace");

        // Cleanup
        unsafe {
            std::env::remove_var("OCTOFHIR_FHIR_VERSION");
            std::env::remove_var("OCTOFHIR_OUTPUT_DIR");
            std::env::remove_var("OCTOFHIR_LOG_LEVEL");
        }
    }

    #[test]
    fn test_merge_configs() {
        let mut base = CodegenConfig::default();
        base.fhir.packages = vec!["hl7.fhir.r4.core@4.0.1".to_string()];

        let mut override_config = CodegenConfig::default();
        override_config.fhir.version = "R5".to_string();
        override_config.fhir.packages = vec!["hl7.fhir.r5.core@5.0.0".to_string()];

        base.merge(&override_config);

        assert_eq!(base.fhir.version, "R5");
        assert_eq!(base.fhir.packages.len(), 2);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let config = CodegenConfig::default_with_typescript();
        let toml_str = config.to_toml_string().unwrap();
        let parsed = CodegenConfig::from_toml_str(&toml_str).unwrap();
        assert_eq!(config, parsed);
    }

    #[test]
    fn test_type_mappings() {
        let toml = r#"
            [generators.typescript]
            enabled = true

            [generators.typescript.type_mappings]
            date = "string"
            instant = "Date"
        "#;

        let config = CodegenConfig::from_toml_str(toml).unwrap();
        let ts_config = config.generators.typescript.unwrap();
        assert_eq!(ts_config.type_mappings.get("date"), Some(&"string".to_string()));
        assert_eq!(ts_config.type_mappings.get("instant"), Some(&"Date".to_string()));
    }

    #[test]
    fn test_include_resources() {
        let toml = r#"
            [fhir]
            version = "R4"
            packages = ["hl7.fhir.r4.core@4.0.1"]
            include_resources = ["Patient", "Observation", "Condition"]

            [generators.typescript]
            enabled = true
        "#;

        let config = CodegenConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.fhir.include_resources, vec!["Patient", "Observation", "Condition"]);

        let warnings = config.validate().unwrap();
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_include_and_exclude_resources_warning() {
        let toml = r#"
            [fhir]
            version = "R4"
            packages = ["hl7.fhir.r4.core@4.0.1"]
            include_resources = ["Patient", "Observation"]
            exclude_resources = ["Binary"]

            [generators.typescript]
            enabled = true
        "#;

        let config = CodegenConfig::from_toml_str(toml).unwrap();
        let warnings = config.validate().unwrap();
        assert!(!warnings.is_empty());
        assert!(warnings[0].contains("include_resources and exclude_resources"));
    }
}
