//! Configuration for code generation
//!
//! Defines configuration structures for controlling code generation behavior
//! across different language backends.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete codegen configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodegenConfig {
    /// FHIR version to generate for
    pub fhir_version: String,

    /// Output directory
    pub output_dir: String,

    /// Generator-specific configurations
    pub generators: HashMap<String, GeneratorConfig>,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            fhir_version: "R4".to_string(),
            output_dir: "./generated".to_string(),
            generators: HashMap::new(),
        }
    }
}

/// Configuration for a specific language generator
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GeneratorConfig {
    /// TypeScript generator configuration
    #[serde(rename = "typescript")]
    TypeScript(TypeScriptConfig),

    /// Rust generator configuration (future)
    #[serde(rename = "rust")]
    Rust(RustConfig),
}

/// TypeScript generator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptConfig {
    /// Whether this generator is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Module/package name
    #[serde(default = "default_module_name")]
    pub module_name: String,

    /// Generate classes instead of interfaces
    #[serde(default)]
    pub generate_classes: bool,

    /// Generate separate builder classes
    #[serde(default)]
    pub generate_builders: bool,

    /// Generate extension helper methods
    #[serde(default = "default_true")]
    pub generate_extensions: bool,

    /// Use TypeScript strict mode
    #[serde(default = "default_true")]
    pub strict_mode: bool,

    /// Emit validation functions
    #[serde(default = "default_true")]
    pub emit_validation: bool,

    /// Emit helper methods
    #[serde(default = "default_true")]
    pub emit_helpers: bool,

    /// Emit test files
    #[serde(default)]
    pub emit_tests: bool,

    /// Target TypeScript version
    #[serde(default = "default_ts_version")]
    pub target_version: String,

    /// TypeScript compiler options
    #[serde(default)]
    pub compiler_options: TypeScriptCompilerOptions,
}

impl Default for TypeScriptConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            module_name: "fhir-r4".to_string(),
            generate_classes: false,
            generate_builders: false,
            generate_extensions: true,
            strict_mode: true,
            emit_validation: true,
            emit_helpers: true,
            emit_tests: false,
            target_version: "5.3".to_string(),
            compiler_options: TypeScriptCompilerOptions::default(),
        }
    }
}

/// TypeScript compiler options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptCompilerOptions {
    /// Enable all strict type-checking options
    #[serde(default = "default_true")]
    pub strict: bool,

    /// Enable ES module interop
    #[serde(default = "default_true")]
    pub es_module_interop: bool,

    /// Generate declaration files
    #[serde(default = "default_true")]
    pub declaration: bool,

    /// Generate source maps
    #[serde(default)]
    pub source_map: bool,

    /// Module resolution strategy
    #[serde(default = "default_module_resolution")]
    pub module_resolution: String,
}

impl Default for TypeScriptCompilerOptions {
    fn default() -> Self {
        Self {
            strict: true,
            es_module_interop: true,
            declaration: true,
            source_map: false,
            module_resolution: "node".to_string(),
        }
    }
}

/// Rust generator configuration (placeholder for future)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustConfig {
    /// Whether this generator is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Crate name
    #[serde(default = "default_rust_crate_name")]
    pub crate_name: String,

    /// Use serde for serialization
    #[serde(default = "default_true")]
    pub use_serde: bool,

    /// Generate builder pattern
    #[serde(default = "default_true")]
    pub generate_builders: bool,
}

impl Default for RustConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            crate_name: "fhir-r4".to_string(),
            use_serde: true,
            generate_builders: true,
        }
    }
}

// Default value functions for serde
fn default_true() -> bool {
    true
}

fn default_module_name() -> String {
    "fhir-r4".to_string()
}

fn default_ts_version() -> String {
    "5.3".to_string()
}

fn default_module_resolution() -> String {
    "node".to_string()
}

fn default_rust_crate_name() -> String {
    "fhir-r4".to_string()
}

impl CodegenConfig {
    /// Load configuration from TOML file
    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }

    /// Save configuration to TOML string
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Get TypeScript configuration if present
    pub fn typescript_config(&self) -> Option<&TypeScriptConfig> {
        self.generators.get("typescript").and_then(|g| {
            if let GeneratorConfig::TypeScript(config) = g { Some(config) } else { None }
        })
    }

    /// Get Rust configuration if present
    pub fn rust_config(&self) -> Option<&RustConfig> {
        self.generators
            .get("rust")
            .and_then(|g| if let GeneratorConfig::Rust(config) = g { Some(config) } else { None })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CodegenConfig::default();
        assert_eq!(config.fhir_version, "R4");
        assert_eq!(config.output_dir, "./generated");
    }

    #[test]
    fn test_typescript_config_defaults() {
        let config = TypeScriptConfig::default();
        assert!(config.enabled);
        assert_eq!(config.module_name, "fhir-r4");
        assert!(!config.generate_classes);
        assert!(config.generate_extensions);
        assert!(config.strict_mode);
    }

    #[test]
    fn test_toml_serialization() {
        let config = CodegenConfig::default();
        let toml_str = config.to_toml().unwrap();
        assert!(toml_str.contains("fhir_version"));
        assert!(toml_str.contains("output_dir"));
    }

    #[test]
    fn test_toml_deserialization() {
        let toml_str = r#"
            fhir_version = "R4"
            output_dir = "./output"

            [generators.typescript]
            type = "typescript"
            enabled = true
            module_name = "my-fhir"
            generate_classes = true
            generate_extensions = true
        "#;

        let config: CodegenConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.fhir_version, "R4");
        assert_eq!(config.output_dir, "./output");

        let ts_config = config.typescript_config().unwrap();
        assert_eq!(ts_config.module_name, "my-fhir");
        assert!(ts_config.generate_classes);
    }
}
