//! CLI entry point for octofhir-codegen

use anyhow::Result;
use octofhir_codegen::cli::{
    Cli, CommandResult, Commands, GenerateOptions, OutputFormatter, ensure_config_exists,
    execute_generate,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse_args();

    // Initialize tracing with verbosity level
    let log_level = cli.log_level();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level)),
        )
        .init();

    // Create output formatter
    let formatter = if let Some(ref format_str) = cli.format {
        match format_str.parse() {
            Ok(format) => OutputFormatter::new(cli.use_color(), format),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        OutputFormatter::pretty(cli.use_color())
    };

    // Execute command
    let result = execute_command(&cli, &formatter).await;

    // Handle result
    match result {
        Ok(cmd_result) => {
            if let Some(message) = cmd_result.message {
                if cmd_result.exit_code == 0 {
                    formatter.success(&message);
                } else {
                    formatter.error(&message);
                }
            }
            std::process::exit(cmd_result.exit_code);
        }
        Err(e) => {
            formatter.error(&format!("Error: {}", e));
            tracing::error!("Command failed: {:?}", e);
            std::process::exit(1);
        }
    }
}

/// Execute the specified CLI command
async fn execute_command(cli: &Cli, formatter: &OutputFormatter) -> Result<CommandResult> {
    match &cli.command {
        Commands::Init { template, output, force, non_interactive } => {
            execute_init(template.as_deref(), output, *force, *non_interactive, formatter).await
        }
        Commands::Generate {
            language,
            output,
            fhir_version,
            package,
            skip_validation,
            no_clean,
            watch,
        } => {
            handle_generate_command(
                cli.config_path().as_ref(),
                language.as_deref(),
                output.as_ref(),
                fhir_version.as_deref(),
                package,
                *skip_validation,
                *no_clean,
                *watch,
                formatter,
            )
            .await
        }
        Commands::Validate { detailed } => {
            execute_validate(cli.config_path().as_ref(), *detailed, formatter).await
        }
        Commands::ListGenerators { detailed } => {
            execute_list_generators(*detailed, formatter).await
        }
        Commands::Describe { generator, examples } => {
            execute_describe(generator, *examples, formatter).await
        }
        Commands::Analyze {
            package,
            show_profiles,
            show_extensions,
            show_resources,
            show_datatypes,
            show_dependencies,
            export,
        } => {
            execute_analyze(
                package,
                *show_profiles,
                *show_extensions,
                *show_resources,
                *show_datatypes,
                *show_dependencies,
                export.as_ref(),
                formatter,
            )
            .await
        }
        Commands::UpdatePackages { package, force } => {
            execute_update_packages(package.as_deref(), *force, formatter).await
        }
        Commands::Version { detailed } => execute_version(*detailed, formatter).await,
        Commands::Clean { output, force } => {
            execute_clean(cli.config_path().as_ref(), output.as_ref(), *force, formatter).await
        }
    }
}

// ============================================================================
// Command implementations (stubs for now, will be implemented in future tasks)
// ============================================================================

async fn execute_init(
    template: Option<&str>,
    output: &std::path::Path,
    force: bool,
    non_interactive: bool,
    formatter: &OutputFormatter,
) -> Result<CommandResult> {
    use octofhir_codegen::cli::CodegenConfig;
    use std::fs;
    use std::io::{self, Write};

    formatter.header("Initialize New Project");

    // Determine config file path
    let config_path = output.join("codegen.toml");

    // Check if config already exists
    if config_path.exists() && !force {
        formatter.error(&format!("Configuration file already exists: {}", config_path.display()));
        formatter.info("Use --force to overwrite the existing configuration");
        return Ok(CommandResult::error("Configuration file already exists"));
    }

    // Determine template
    let template_name = if non_interactive {
        template.unwrap_or("typescript")
    } else {
        // Interactive mode: ask user for template
        if template.is_some() {
            template.unwrap()
        } else {
            println!("\nSelect a template:");
            println!("  1. TypeScript (default)");
            println!("  2. Rust");
            println!("  3. Python");
            println!("  4. Java");
            println!("  5. Multi-language (all generators)");
            print!("\nEnter choice [1-5] (default: 1): ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            match input {
                "1" | "" => "typescript",
                "2" => "rust",
                "3" => "python",
                "4" => "java",
                "5" => "multi",
                _ => {
                    formatter.warning("Invalid choice, using TypeScript template");
                    "typescript"
                }
            }
        }
    };

    formatter.info(&format!("Template: {}", template_name));
    formatter.info(&format!("Output: {}", config_path.display()));

    // Create configuration from template
    let config = CodegenConfig::template_for(template_name)?;

    // Create output directory if needed
    if !output.exists() {
        fs::create_dir_all(output)?;
        formatter.info(&format!("Created directory: {}", output.display()));
    }

    // Write configuration file with comments
    let config_content = config.to_commented_toml()?;
    fs::write(&config_path, config_content)?;

    formatter.success(&format!("Created configuration file: {}", config_path.display()));
    formatter.info("\nNext steps:");
    formatter.list_item("Edit codegen.toml to customize your configuration");
    formatter.list_item("Run 'octofhir-codegen validate' to check your configuration");
    formatter.list_item("Run 'octofhir-codegen generate' to generate your SDK");

    Ok(CommandResult::success())
}

#[allow(clippy::too_many_arguments)]
async fn handle_generate_command(
    explicit_config: Option<&std::path::PathBuf>,
    language: Option<&str>,
    output: Option<&std::path::PathBuf>,
    fhir_version: Option<&str>,
    packages: &[String],
    skip_validation: bool,
    no_clean: bool,
    watch: bool,
    formatter: &OutputFormatter,
) -> Result<CommandResult> {
    formatter.header("Generate SDK");

    // Discover configuration file
    let config_path = ensure_config_exists(explicit_config)?;

    // Handle watch mode
    if watch {
        formatter.warning("Watch mode not yet implemented. Running single generation...");
    }

    // Create generation options
    let options = GenerateOptions {
        config_path,
        language_override: language.map(|s| s.to_string()),
        output_override: output.cloned(),
        fhir_version_override: fhir_version.map(|s| s.to_string()),
        additional_packages: packages.to_vec(),
        skip_validation,
        no_clean,
    };

    // Execute generation
    let result = execute_generate(options, formatter).await?;

    // Report results
    formatter.success(&format!(
        "Successfully generated {} {} SDK with {} files",
        result.fhir_version, result.language, result.files_generated
    ));
    formatter.info(&format!("Output directory: {}", result.output_path.display()));

    Ok(CommandResult::success())
}

async fn execute_validate(
    explicit_config: Option<&std::path::PathBuf>,
    detailed: bool,
    formatter: &OutputFormatter,
) -> Result<CommandResult> {
    formatter.header("Validate Configuration");

    // Discover configuration file
    let config_path = ensure_config_exists(explicit_config)?;
    formatter.info(&format!("Config: {}", config_path.display()));
    formatter.info(&format!("Detailed: {}", detailed));

    // Try to load and validate the configuration
    match octofhir_codegen::cli::CodegenConfig::from_file(&config_path) {
        Ok(config) => match config.validate() {
            Ok(warnings) => {
                formatter.success("Configuration is valid");
                if !warnings.is_empty() {
                    formatter.warning(&format!("Found {} warning(s):", warnings.len()));
                    for warning in warnings {
                        formatter.warning(&format!("  • {}", warning));
                    }
                }
                Ok(CommandResult::success())
            }
            Err(e) => {
                formatter.error(&format!("Validation failed: {}", e));
                Ok(CommandResult::error(format!("Validation failed: {}", e)))
            }
        },
        Err(e) => {
            formatter.error(&format!("Failed to load configuration: {}", e));
            Ok(CommandResult::error(format!("Failed to load configuration: {}", e)))
        }
    }
}

async fn execute_list_generators(
    detailed: bool,
    formatter: &OutputFormatter,
) -> Result<CommandResult> {
    formatter.header("Available Generators");
    formatter.info(&format!("Detailed: {}", detailed));

    let generators = vec![
        ("TypeScript", "Ready", "Generate type-safe TypeScript SDKs"),
        ("Rust", "In Progress", "Generate Rust SDKs with serde"),
        ("Python", "Planned", "Generate Python SDKs with Pydantic"),
        ("Java", "Planned", "Generate Java SDKs with Jackson"),
    ];

    if detailed {
        for (name, status, description) in generators {
            formatter.list_item(&format!("{} [{}]: {}", name, status, description));
        }
    } else {
        formatter.table(
            &["Generator", "Status"],
            &generators
                .iter()
                .map(|(name, status, _)| vec![name.to_string(), status.to_string()])
                .collect::<Vec<_>>(),
        );
    }

    Ok(CommandResult::success())
}

async fn execute_describe(
    generator: &str,
    examples: bool,
    formatter: &OutputFormatter,
) -> Result<CommandResult> {
    formatter.header(&format!("Generator: {}", generator));

    match generator.to_lowercase().as_str() {
        "typescript" | "ts" => {
            formatter.key_value("Language", "TypeScript");
            formatter.key_value("Status", "Ready");
            formatter.key_value("Target Versions", "5.0+");
            formatter.info("\nDescription:");
            formatter
                .info("  Generates type-safe TypeScript SDKs with comprehensive type definitions,");
            formatter
                .info("  validation functions, helper methods, and full IntelliSense support.");
            formatter.info("\nFeatures:");
            formatter.list_item("Union types for choice elements (e.g., value[x])");
            formatter.list_item("Literal types for required bindings");
            formatter.list_item("Optional validation functions");
            formatter.list_item("Helper methods for common operations");
            formatter.list_item("JSDoc documentation");
            formatter.list_item("Tree-shakeable exports");

            if examples {
                formatter.info("\nConfiguration Example:");
                println!(
                    r#"
[generators.typescript]
enabled = true
module_name = "fhir-r4"
emit_validation = true
emit_helpers = true
emit_tests = false
target_version = "5.3"
"#
                );
            }
        }
        "rust" | "rs" => {
            formatter.key_value("Language", "Rust");
            formatter.key_value("Status", "In Progress");
            formatter.key_value("Target Editions", "2021, 2024");
            formatter.info("\nDescription:");
            formatter.info("  Generates idiomatic Rust SDKs with serde support, builder patterns,");
            formatter.info("  and compile-time type safety.");
            formatter.info("\nFeatures:");
            formatter.list_item("Serde serialization/deserialization");
            formatter.list_item("Builder patterns for complex resources");
            formatter.list_item("Enums for coded values");
            formatter.list_item("Optional validation traits");
            formatter.list_item("Comprehensive documentation");

            if examples {
                formatter.info("\nConfiguration Example:");
                println!(
                    r#"
[generators.rust]
enabled = true
crate_name = "fhir-r4"
emit_validation = true
emit_helpers = true
edition = "2024"
additional_derives = ["Hash", "Eq"]
"#
                );
            }
        }
        "python" | "py" => {
            formatter.key_value("Language", "Python");
            formatter.key_value("Status", "Planned");
            formatter.key_value("Target Versions", "3.10+");
            formatter.info("\nDescription:");
            formatter.info("  Generates Python SDKs with Pydantic models for validation and");
            formatter.info("  type hints for static analysis.");
            formatter.info("\nPlanned Features:");
            formatter.list_item("Pydantic V2 models");
            formatter.list_item("Type stubs (.pyi files)");
            formatter.list_item("Dataclasses support");
            formatter.list_item("Comprehensive docstrings");

            if examples {
                formatter.info("\nConfiguration Example:");
                println!(
                    r#"
[generators.python]
enabled = true
package_name = "fhir_r4"
use_pydantic = true
generate_stubs = true
target_version = "3.11"
"#
                );
            }
        }
        "java" => {
            formatter.key_value("Language", "Java");
            formatter.key_value("Status", "Planned");
            formatter.key_value("Target Versions", "11, 17, 21");
            formatter.info("\nDescription:");
            formatter.info("  Generates Java SDKs with Jackson annotations, builder patterns,");
            formatter.info("  and Bean Validation support.");
            formatter.info("\nPlanned Features:");
            formatter.list_item("Jackson serialization");
            formatter.list_item("Builder patterns");
            formatter.list_item("Bean Validation annotations");
            formatter.list_item("Fluent interfaces");
            formatter.list_item("Comprehensive JavaDoc");

            if examples {
                formatter.info("\nConfiguration Example:");
                println!(
                    r#"
[generators.java]
enabled = true
package_name = "com.example.fhir.r4"
target_version = "17"
use_jackson = true
emit_validation = true
"#
                );
            }
        }
        _ => {
            formatter.error(&format!("Unknown generator: {}", generator));
            formatter.info("\nAvailable generators: typescript, rust, python, java");
            return Ok(CommandResult::error("Unknown generator"));
        }
    }

    Ok(CommandResult::success())
}

#[allow(clippy::too_many_arguments)]
async fn execute_analyze(
    package: &str,
    show_profiles: bool,
    show_extensions: bool,
    show_resources: bool,
    show_datatypes: bool,
    show_dependencies: bool,
    export: Option<&std::path::PathBuf>,
    formatter: &OutputFormatter,
) -> Result<CommandResult> {
    formatter.header("Analyze FHIR Package");
    formatter.key_value("Package", package);

    // Parse package name and version
    let parts: Vec<&str> = package.split('@').collect();
    if parts.len() != 2 {
        formatter.error("Invalid package format. Expected format: name@version");
        formatter.info("Example: hl7.fhir.r4.core@4.0.1");
        return Ok(CommandResult::error("Invalid package format"));
    }

    let (pkg_name, pkg_version) = (parts[0], parts[1]);

    formatter.info("\nPackage Information:");
    formatter.key_value("  Name", pkg_name);
    formatter.key_value("  Version", pkg_version);

    // TODO: Integrate with canonical-manager to get actual package data
    // For now, provide a meaningful placeholder with structure

    formatter.warning("\nNote: Full canonical-manager integration pending");
    formatter.info("This command will analyze FHIR packages using canonical-manager");

    // Show what would be analyzed based on flags
    let show_all = !show_profiles && !show_extensions && !show_resources && !show_datatypes;

    if show_all || show_resources {
        formatter.info("\nResources:");
        formatter.info("  • Would list all resource types in the package");
        formatter.info("  • Would show resource counts and summaries");
    }

    if show_all || show_datatypes {
        formatter.info("\nData Types:");
        formatter.info("  • Would list all data types in the package");
        formatter.info("  • Would show complex and primitive types");
    }

    if show_profiles {
        formatter.info("\nProfiles:");
        formatter.info("  • Would list all profiles in the package");
        formatter.info("  • Would show profile constraints and extensions");
    }

    if show_extensions {
        formatter.info("\nExtensions:");
        formatter.info("  • Would list all extensions in the package");
        formatter.info("  • Would show extension definitions and contexts");
    }

    if show_dependencies {
        formatter.info("\nDependencies:");
        formatter.info("  • Would show package dependency tree");
        formatter.info("  • Would list all transitive dependencies");
    }

    if let Some(export_path) = export {
        formatter.info(&format!("\nWould export analysis to: {}", export_path.display()));
        formatter.info("Export formats: JSON, Markdown");
    }

    formatter.info("\nImplementation Status:");
    formatter.list_item("Package parsing: ✓ Complete");
    formatter.list_item("Canonical-manager integration: ⏳ Pending");
    formatter.list_item("Resource analysis: ⏳ Pending");
    formatter.list_item("Profile analysis: ⏳ Pending");
    formatter.list_item("Dependency tree: ⏳ Pending");
    formatter.list_item("Export functionality: ⏳ Pending");

    Ok(CommandResult::success())
}

async fn execute_update_packages(
    package: Option<&str>,
    force: bool,
    formatter: &OutputFormatter,
) -> Result<CommandResult> {
    formatter.header("Update Packages");
    if let Some(pkg) = package {
        formatter.info(&format!("Package: {}", pkg));
    } else {
        formatter.info("Updating all packages");
    }
    formatter.info(&format!("Force: {}", force));

    formatter.warning("Update packages command implementation coming soon...");
    Ok(CommandResult::success())
}

async fn execute_version(detailed: bool, formatter: &OutputFormatter) -> Result<CommandResult> {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");

    if detailed {
        formatter.header("Version Information");
        formatter.key_value("Name", name);
        formatter.key_value("Version", version);
        formatter.key_value("Rust Version", env!("CARGO_PKG_RUST_VERSION"));
        formatter.key_value("Authors", env!("CARGO_PKG_AUTHORS"));
        formatter.key_value("Repository", env!("CARGO_PKG_REPOSITORY"));
    } else {
        println!("{} {}", name, version);
    }

    Ok(CommandResult::success())
}

async fn execute_clean(
    explicit_config: Option<&std::path::PathBuf>,
    output: Option<&std::path::PathBuf>,
    force: bool,
    formatter: &OutputFormatter,
) -> Result<CommandResult> {
    formatter.header("Clean Output Directory");

    // Discover configuration file if needed (only if output not specified)
    if output.is_none() {
        let config_path = ensure_config_exists(explicit_config)?;
        formatter.info(&format!("Config: {}", config_path.display()));
    }
    if let Some(out) = output {
        formatter.info(&format!("Output: {}", out.display()));
    }
    formatter.info(&format!("Force: {}", force));

    formatter.warning("Clean command implementation coming soon...");
    Ok(CommandResult::success())
}
