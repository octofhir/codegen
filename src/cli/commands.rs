//! CLI command definitions using clap
//!
//! This module defines all CLI commands, arguments, and subcommands for OctoFHIR Codegen.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// OctoFHIR Codegen - Generate type-safe FHIR SDKs for multiple languages
///
/// A powerful code generator that creates production-ready FHIR SDKs for TypeScript,
/// Rust, Python, Java, and more from FHIR StructureDefinitions.
///
/// Examples:
///   # Initialize a new project
///   octofhir-codegen init --template typescript
///
///   # Generate SDK from configuration file
///   octofhir-codegen generate
///
///   # Generate with specific options
///   octofhir-codegen generate --language typescript --output ./sdk
///
///   # List available generators
///   octofhir-codegen list-generators
#[derive(Parser, Debug)]
#[command(
    name = "octofhir-codegen",
    version,
    author,
    about = "Generate type-safe FHIR SDKs for multiple languages",
    long_about = None,
    after_help = "For more information, visit: https://github.com/octofhir/codegen"
)]
pub struct Cli {
    /// Path to configuration file (default: codegen.toml)
    #[arg(short, long, global = true, value_name = "FILE", help = "Path to configuration file")]
    pub config: Option<PathBuf>,

    /// Enable verbose logging (can be repeated: -v, -vv, -vvv)
    #[arg(
        short,
        long,
        global = true,
        action = clap::ArgAction::Count,
        help = "Increase logging verbosity"
    )]
    pub verbose: u8,

    /// Disable colored output
    #[arg(long, global = true, help = "Disable colored output")]
    pub no_color: bool,

    /// Output format (pretty, json, compact)
    #[arg(long, global = true, value_name = "FORMAT", help = "Output format")]
    pub format: Option<String>,

    /// The subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new codegen project with configuration template
    ///
    /// Creates a new codegen.toml configuration file with sensible defaults
    /// for the specified language target.
    ///
    /// Examples:
    ///   octofhir-codegen init
    ///   octofhir-codegen init --template typescript
    ///   octofhir-codegen init --output ./my-project
    Init {
        /// Template to use (typescript, rust, python, java, multi)
        #[arg(short, long, value_name = "TEMPLATE", help = "Template to use for initialization")]
        template: Option<String>,

        /// Output directory for the new project
        #[arg(short, long, value_name = "DIR", default_value = ".", help = "Output directory")]
        output: PathBuf,

        /// Overwrite existing configuration file
        #[arg(long, help = "Overwrite existing codegen.toml")]
        force: bool,

        /// Don't prompt for interactive configuration
        #[arg(long, help = "Skip interactive prompts")]
        non_interactive: bool,
    },

    /// Generate SDKs from configuration
    ///
    /// Reads the configuration file and generates SDKs for all enabled
    /// language targets. Can override specific configuration options
    /// via command-line arguments.
    ///
    /// Examples:
    ///   octofhir-codegen generate
    ///   octofhir-codegen generate --language typescript
    ///   octofhir-codegen generate --output ./custom-output
    ///   octofhir-codegen generate --package hl7.fhir.r4.core@4.0.1
    Generate {
        /// Target language to generate (overrides config)
        #[arg(
            short,
            long,
            value_name = "LANGUAGE",
            help = "Target language (typescript, rust, python, java)"
        )]
        language: Option<String>,

        /// Output directory (overrides config)
        #[arg(short, long, value_name = "DIR", help = "Output directory")]
        output: Option<PathBuf>,

        /// FHIR version (overrides config)
        #[arg(long, value_name = "VERSION", help = "FHIR version (R4, R5, R6)")]
        fhir_version: Option<String>,

        /// Additional packages to include (can be specified multiple times)
        #[arg(long, value_name = "PACKAGE", help = "FHIR package (format: name@version)")]
        package: Vec<String>,

        /// Skip validation before generation
        #[arg(long, help = "Skip configuration validation")]
        skip_validation: bool,

        /// Don't clean output directory before generation
        #[arg(long, help = "Don't clean output directory")]
        no_clean: bool,

        /// Watch mode: regenerate on configuration changes
        #[arg(short, long, help = "Watch for changes and regenerate")]
        watch: bool,
    },

    /// Validate configuration file
    ///
    /// Checks the configuration file for errors and provides detailed
    /// feedback about any issues found.
    ///
    /// Examples:
    ///   octofhir-codegen validate
    ///   octofhir-codegen validate --config ./custom-config.toml
    Validate {
        /// Show detailed validation results
        #[arg(long, help = "Show detailed validation information")]
        detailed: bool,
    },

    /// List available code generators
    ///
    /// Shows all available language generators and their current status.
    ///
    /// Examples:
    ///   octofhir-codegen list-generators
    ///   octofhir-codegen list-generators --verbose
    ListGenerators {
        /// Show detailed generator information
        #[arg(long, help = "Show detailed generator capabilities")]
        detailed: bool,
    },

    /// Describe a specific generator's capabilities
    ///
    /// Shows detailed information about a specific generator including
    /// supported features, configuration options, and examples.
    ///
    /// Examples:
    ///   octofhir-codegen describe typescript
    ///   octofhir-codegen describe rust --examples
    Describe {
        /// Generator name to describe
        #[arg(value_name = "GENERATOR", help = "Generator to describe")]
        generator: String,

        /// Show configuration examples
        #[arg(long, help = "Show configuration examples")]
        examples: bool,
    },

    /// Analyze FHIR packages and show statistics
    ///
    /// Analyzes FHIR packages and displays detailed information about
    /// resources, datatypes, profiles, and extensions.
    ///
    /// Examples:
    ///   octofhir-codegen analyze --package hl7.fhir.r4.core@4.0.1
    ///   octofhir-codegen analyze --package hl7.fhir.us.core@6.1.0 --show-profiles
    Analyze {
        /// FHIR package to analyze (format: name@version)
        #[arg(long, value_name = "PACKAGE", help = "FHIR package to analyze")]
        package: String,

        /// Show detailed profile information
        #[arg(long, help = "Show profiles")]
        show_profiles: bool,

        /// Show detailed extension information
        #[arg(long, help = "Show extensions")]
        show_extensions: bool,

        /// Show detailed resource information
        #[arg(long, help = "Show resources")]
        show_resources: bool,

        /// Show detailed datatype information
        #[arg(long, help = "Show datatypes")]
        show_datatypes: bool,

        /// Show dependency tree
        #[arg(long, help = "Show package dependencies")]
        show_dependencies: bool,

        /// Export analysis to file
        #[arg(long, value_name = "FILE", help = "Export analysis to file")]
        export: Option<PathBuf>,
    },

    /// Update canonical-manager packages
    ///
    /// Updates FHIR packages managed by the canonical-manager.
    ///
    /// Examples:
    ///   octofhir-codegen update-packages
    ///   octofhir-codegen update-packages --package hl7.fhir.r4.core
    UpdatePackages {
        /// Specific package to update (if omitted, updates all)
        #[arg(long, value_name = "PACKAGE", help = "Package to update")]
        package: Option<String>,

        /// Force re-download even if package exists
        #[arg(long, help = "Force re-download")]
        force: bool,
    },

    /// Show version information
    ///
    /// Displays detailed version information including build details.
    Version {
        /// Show detailed build information
        #[arg(long, help = "Show detailed build information")]
        detailed: bool,
    },

    /// Clean generated output directory
    ///
    /// Removes all generated files from the output directory.
    ///
    /// Examples:
    ///   octofhir-codegen clean
    ///   octofhir-codegen clean --output ./custom-output
    Clean {
        /// Output directory to clean (uses config if not specified)
        #[arg(short, long, value_name = "DIR", help = "Output directory")]
        output: Option<PathBuf>,

        /// Don't prompt for confirmation
        #[arg(short, long, help = "Force clean without confirmation")]
        force: bool,
    },
}

impl Cli {
    /// Parse CLI arguments from environment
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Get the log level based on verbosity flag
    pub fn log_level(&self) -> &str {
        match self.verbose {
            0 => "info",
            1 => "debug",
            2 => "trace",
            _ => "trace",
        }
    }

    /// Check if colored output should be used
    pub fn use_color(&self) -> bool {
        use std::io::IsTerminal;
        !self.no_color && std::io::stdout().is_terminal()
    }

    /// Get the configuration file path (explicit or None for auto-discovery)
    pub fn config_path(&self) -> Option<PathBuf> {
        self.config.clone()
    }

    /// Get the configuration file path, falling back to default
    pub fn config_path_or_default(&self) -> PathBuf {
        self.config.clone().unwrap_or_else(|| PathBuf::from("codegen.toml"))
    }
}

/// Command execution result
#[derive(Debug)]
pub struct CommandResult {
    /// Exit code (0 for success)
    pub exit_code: i32,
    /// Optional message to display
    pub message: Option<String>,
}

impl CommandResult {
    /// Create a successful result
    pub fn success() -> Self {
        Self { exit_code: 0, message: None }
    }

    /// Create a successful result with message
    pub fn success_with_message(message: impl Into<String>) -> Self {
        Self { exit_code: 0, message: Some(message.into()) }
    }

    /// Create an error result
    pub fn error(message: impl Into<String>) -> Self {
        Self { exit_code: 1, message: Some(message.into()) }
    }

    /// Create an error result with custom exit code
    pub fn error_with_code(exit_code: i32, message: impl Into<String>) -> Self {
        Self { exit_code, message: Some(message.into()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing_basic() {
        let cli = Cli::try_parse_from(["octofhir-codegen", "list-generators"]).unwrap();
        assert!(matches!(cli.command, Commands::ListGenerators { .. }));
    }

    #[test]
    fn test_cli_parsing_with_config() {
        let cli = Cli::try_parse_from(["octofhir-codegen", "--config", "custom.toml", "generate"])
            .unwrap();
        assert_eq!(cli.config, Some(PathBuf::from("custom.toml")));
    }

    #[test]
    fn test_cli_verbose_levels() {
        let cli = Cli::try_parse_from(["octofhir-codegen", "-vvv", "generate"]).unwrap();
        assert_eq!(cli.verbose, 3);
        assert_eq!(cli.log_level(), "trace");
    }

    #[test]
    fn test_init_command() {
        let cli = Cli::try_parse_from([
            "octofhir-codegen",
            "init",
            "--template",
            "typescript",
            "--output",
            "./my-project",
        ])
        .unwrap();

        if let Commands::Init { template, output, .. } = cli.command {
            assert_eq!(template, Some("typescript".to_string()));
            assert_eq!(output, PathBuf::from("./my-project"));
        } else {
            panic!("Expected Init command");
        }
    }

    #[test]
    fn test_generate_command() {
        let cli = Cli::try_parse_from([
            "octofhir-codegen",
            "generate",
            "--language",
            "typescript",
            "--output",
            "./sdk",
            "--package",
            "hl7.fhir.r4.core@4.0.1",
        ])
        .unwrap();

        if let Commands::Generate { language, output, package, .. } = cli.command {
            assert_eq!(language, Some("typescript".to_string()));
            assert_eq!(output, Some(PathBuf::from("./sdk")));
            assert_eq!(package, vec!["hl7.fhir.r4.core@4.0.1"]);
        } else {
            panic!("Expected Generate command");
        }
    }

    #[test]
    fn test_analyze_command() {
        let cli = Cli::try_parse_from([
            "octofhir-codegen",
            "analyze",
            "--package",
            "hl7.fhir.r4.core@4.0.1",
            "--show-profiles",
            "--show-extensions",
        ])
        .unwrap();

        if let Commands::Analyze { package, show_profiles, show_extensions, .. } = cli.command {
            assert_eq!(package, "hl7.fhir.r4.core@4.0.1");
            assert!(show_profiles);
            assert!(show_extensions);
        } else {
            panic!("Expected Analyze command");
        }
    }

    #[test]
    fn test_describe_command() {
        let cli = Cli::try_parse_from(["octofhir-codegen", "describe", "typescript", "--examples"])
            .unwrap();

        if let Commands::Describe { generator, examples } = cli.command {
            assert_eq!(generator, "typescript");
            assert!(examples);
        } else {
            panic!("Expected Describe command");
        }
    }

    #[test]
    fn test_command_result_success() {
        let result = CommandResult::success();
        assert_eq!(result.exit_code, 0);
        assert!(result.message.is_none());
    }

    #[test]
    fn test_command_result_error() {
        let result = CommandResult::error("Something went wrong");
        assert_eq!(result.exit_code, 1);
        assert_eq!(result.message, Some("Something went wrong".to_string()));
    }

    #[test]
    fn test_config_path_default() {
        let cli = Cli::try_parse_from(["octofhir-codegen", "generate"]).unwrap();
        assert_eq!(cli.config_path(), None);
        assert_eq!(cli.config_path_or_default(), PathBuf::from("codegen.toml"));
    }

    #[test]
    fn test_config_path_custom() {
        let cli = Cli::try_parse_from(["octofhir-codegen", "--config", "custom.toml", "generate"])
            .unwrap();
        assert_eq!(cli.config_path(), Some(PathBuf::from("custom.toml")));
        assert_eq!(cli.config_path_or_default(), PathBuf::from("custom.toml"));
    }
}
