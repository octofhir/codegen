//! Generate command implementation
//!
//! This module orchestrates the complete SDK generation pipeline including:
//! - Configuration loading and validation
//! - Type graph building from FHIR packages
//! - Generator selection and execution
//! - File output management

use crate::cli::{CodegenConfig, OutputFormatter};
use crate::core::TypeGraphBuilder;
use crate::core::ir::{FhirVersion, TypeGraph};
use crate::languages::typescript::{PackageConfig, TypeScriptSdkGenerator};
use anyhow::{Context, Result};
use indicatif::ProgressBar;
use octofhir_canonical_manager::{CanonicalManager, FcmConfig};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Options for SDK generation
#[derive(Debug, Clone)]
pub struct GenerateOptions {
    /// Configuration file path
    pub config_path: PathBuf,
    /// Override language from CLI
    pub language_override: Option<String>,
    /// Override output directory from CLI
    pub output_override: Option<PathBuf>,
    /// Override FHIR version from CLI
    pub fhir_version_override: Option<String>,
    /// Additional packages from CLI
    pub additional_packages: Vec<String>,
    /// Skip validation
    pub skip_validation: bool,
    /// Don't clean output directory
    pub no_clean: bool,
}

/// Result of SDK generation
#[derive(Debug)]
pub struct GenerationResult {
    /// Number of files generated
    pub files_generated: usize,
    /// Output directory path
    pub output_path: PathBuf,
    /// Language generated
    pub language: String,
    /// FHIR version used
    pub fhir_version: String,
}

/// Execute the generate command
pub async fn execute_generate(
    options: GenerateOptions,
    formatter: &OutputFormatter,
) -> Result<GenerationResult> {
    // Step 1: Load and validate configuration
    formatter.info("Loading configuration...");
    let mut config = CodegenConfig::from_file(&options.config_path)
        .context("Failed to load configuration file")?;

    // Apply CLI overrides
    apply_cli_overrides(&mut config, &options);

    // Validate configuration
    if !options.skip_validation {
        formatter.info("Validating configuration...");
        let warnings = config.validate()?;
        for warning in warnings {
            formatter.warning(&warning);
        }
    }

    // Step 2: Determine which generator to use
    let language = determine_language(&config, &options)?;
    formatter.success(&format!("Generating {} SDK", language));

    // Step 3: Set up output directory
    let output_dir = determine_output_dir(&config, &options)?;
    setup_output_directory(&output_dir, config.output.clean && !options.no_clean, formatter)?;

    // Step 4: Initialize canonical manager
    formatter.info("Initializing FHIR package manager...");
    let canonical_manager = initialize_canonical_manager(&config).await?;

    // Step 5: Build type graph
    formatter.info("Building type graph from FHIR packages...");
    let mut type_graph = build_type_graph(&config, canonical_manager, formatter).await?;

    // Step 5.5: Apply resource filtering
    filter_type_graph(&mut type_graph, &config, formatter);

    formatter.success(&format!(
        "Type graph built: {} resources, {} datatypes",
        type_graph.resources.len(),
        type_graph.datatypes.len()
    ));

    // Step 6: Generate SDK files
    formatter.info(&format!("Generating {} SDK files...", language));
    let files = generate_sdk_files(&config, &type_graph, &language)?;

    // Step 7: Write files to disk
    formatter.info("Writing files to disk...");
    write_files_to_disk(&output_dir, &files, formatter)?;

    Ok(GenerationResult {
        files_generated: files.len(),
        output_path: output_dir,
        language,
        fhir_version: config.fhir.version.clone(),
    })
}

/// Apply CLI overrides to configuration
fn apply_cli_overrides(config: &mut CodegenConfig, options: &GenerateOptions) {
    if let Some(ref lang) = options.language_override {
        // Enable the specified generator
        match lang.to_lowercase().as_str() {
            "typescript" => {
                config.generators.typescript =
                    Some(crate::cli::config::TypeScriptGeneratorConfig {
                        enabled: true,
                        ..Default::default()
                    });
            }
            "rust" => {
                config.generators.rust = Some(crate::cli::config::RustGeneratorConfig {
                    enabled: true,
                    ..Default::default()
                });
            }
            _ => {}
        }
    }

    if let Some(ref output) = options.output_override {
        config.output.directory = output.clone();
    }

    if let Some(ref version) = options.fhir_version_override {
        config.fhir.version.clone_from(version);
    }

    if !options.additional_packages.is_empty() {
        config.fhir.packages.extend(options.additional_packages.clone());
    }
}

/// Determine which language to generate
fn determine_language(config: &CodegenConfig, options: &GenerateOptions) -> Result<String> {
    if let Some(ref lang) = options.language_override {
        return Ok(lang.clone());
    }

    // Check which generator is enabled in config
    if config.generators.typescript.as_ref().is_some_and(|g| g.enabled) {
        return Ok("TypeScript".to_string());
    }

    if config.generators.rust.as_ref().is_some_and(|g| g.enabled) {
        return Ok("Rust".to_string());
    }

    if config.generators.python.as_ref().is_some_and(|g| g.enabled) {
        return Ok("Python".to_string());
    }

    if config.generators.java.as_ref().is_some_and(|g| g.enabled) {
        return Ok("Java".to_string());
    }

    anyhow::bail!("No generator enabled. Enable at least one generator in the configuration.");
}

/// Determine output directory
fn determine_output_dir(config: &CodegenConfig, options: &GenerateOptions) -> Result<PathBuf> {
    let dir = options.output_override.as_ref().unwrap_or(&config.output.directory);
    Ok(dir.clone())
}

/// Set up output directory (create or clean)
fn setup_output_directory(
    output_dir: &Path,
    should_clean: bool,
    formatter: &OutputFormatter,
) -> Result<()> {
    if output_dir.exists() && should_clean {
        formatter.warning(&format!("Cleaning output directory: {}", output_dir.display()));
        fs::remove_dir_all(output_dir).context("Failed to clean output directory")?;
    }

    if !output_dir.exists() {
        formatter.info(&format!("Creating output directory: {}", output_dir.display()));
        fs::create_dir_all(output_dir).context("Failed to create output directory")?;
    }

    Ok(())
}

/// Initialize canonical manager for FHIR package resolution
async fn initialize_canonical_manager(config: &CodegenConfig) -> Result<Arc<CanonicalManager>> {
    let fcm_config = if let Some(ref cm_config) = config.canonical_manager {
        // Load from specified path or use defaults
        if let Some(ref _path) = cm_config.config_path {
            // TODO: FcmConfig doesn't have load_from method yet
            // For now, use default config
            FcmConfig::load().await?
        } else {
            FcmConfig::load().await?
        }
    } else {
        // Use default configuration
        FcmConfig::load().await?
    };

    let manager = CanonicalManager::new(fcm_config).await?;
    Ok(Arc::new(manager))
}

/// Build type graph from FHIR packages
async fn build_type_graph(
    config: &CodegenConfig,
    canonical_manager: Arc<CanonicalManager>,
    formatter: &OutputFormatter,
) -> Result<TypeGraph> {
    // Parse FHIR version
    let fhir_version = match config.fhir.version.as_str() {
        "R4" => FhirVersion::R4,
        "R4B" => FhirVersion::R4B,
        "R5" => FhirVersion::R5,
        "R6" => FhirVersion::R6,
        _ => anyhow::bail!(
            "Invalid FHIR version: {}. Expected R4, R4B, R5, or R6",
            config.fhir.version
        ),
    };

    // Create type graph builder
    let builder = TypeGraphBuilder::new(canonical_manager, fhir_version);

    // Build from each package
    let mut type_graph: Option<TypeGraph> = None;

    for package_spec in &config.fhir.packages {
        // Parse package@version
        let parts: Vec<&str> = package_spec.split('@').collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid package format: '{}'. Expected 'package@version'", package_spec);
        }

        let package_name = parts[0];
        let package_version = parts[1];

        formatter.info(&format!("Processing package: {}@{}", package_name, package_version));

        let spinner = crate::cli::output::create_spinner(&format!(
            "Building type graph from {}@{}",
            package_name, package_version
        ));

        let graph = builder
            .build_from_package(package_name, package_version)
            .await
            .context(format!("Failed to build type graph from package: {}", package_spec))?;

        spinner.finish_with_message(format!(
            "✓ Loaded {}@{}: {} resources, {} datatypes",
            package_name,
            package_version,
            graph.resources.len(),
            graph.datatypes.len()
        ));

        // Merge graphs if we have multiple packages
        if let Some(existing) = type_graph {
            type_graph = Some(merge_type_graphs(existing, graph)?);
        } else {
            type_graph = Some(graph);
        }
    }

    type_graph.ok_or_else(|| anyhow::anyhow!("No FHIR packages specified in configuration"))
}

/// Merge two type graphs (simple implementation - later will be more sophisticated)
fn merge_type_graphs(mut base: TypeGraph, other: TypeGraph) -> Result<TypeGraph> {
    // Merge resources
    for (name, resource) in other.resources {
        base.resources.insert(name, resource);
    }

    // Merge datatypes
    for (name, datatype) in other.datatypes {
        base.datatypes.insert(name, datatype);
    }

    // Merge primitives
    for (name, primitive) in other.primitives {
        base.primitives.insert(name, primitive);
    }

    Ok(base)
}

/// Generate SDK files based on configuration and language
fn generate_sdk_files(
    config: &CodegenConfig,
    type_graph: &TypeGraph,
    language: &str,
) -> Result<HashMap<String, String>> {
    match language {
        "TypeScript" => generate_typescript_sdk(config, type_graph),
        "Rust" => anyhow::bail!("Rust generator not yet implemented"),
        "Python" => anyhow::bail!("Python generator not yet implemented"),
        "Java" => anyhow::bail!("Java generator not yet implemented"),
        _ => anyhow::bail!("Unknown language: {}", language),
    }
}

/// Generate TypeScript SDK
fn generate_typescript_sdk(
    config: &CodegenConfig,
    type_graph: &TypeGraph,
) -> Result<HashMap<String, String>> {
    let ts_config = config
        .generators
        .typescript
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("TypeScript generator not configured"))?;

    // Create package config
    let package_config = PackageConfig {
        name: ts_config
            .module_name
            .clone()
            .or_else(|| Some(config.project.name.clone()))
            .unwrap_or_else(|| "fhir-sdk".to_string()),
        version: config.project.version.clone(),
        description: config
            .project
            .description
            .clone()
            .unwrap_or_else(|| "FHIR SDK generated by OctoFHIR Codegen".to_string()),
        fhir_version: config.fhir.version.clone(),
        repository_url: config.project.repository.clone(),
        license: config.project.license.clone().unwrap_or_else(|| "Apache-2.0".to_string()),
    };

    // Create SDK generator
    let generator = TypeScriptSdkGenerator::new(package_config);

    // Generate SDK files
    Ok(generator.generate_sdk(type_graph)?)
}

/// Write generated files to disk
fn write_files_to_disk(
    output_dir: &Path,
    files: &HashMap<String, String>,
    formatter: &OutputFormatter,
) -> Result<()> {
    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len}")
            .expect("Invalid progress bar template")
            .progress_chars("#>-"),
    );
    pb.set_message("Writing files");

    for (rel_path, content) in files {
        let full_path = output_dir.join(rel_path);

        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create directory: {}", parent.display()))?;
        }

        // Write file
        fs::write(&full_path, content)
            .context(format!("Failed to write file: {}", full_path.display()))?;

        pb.inc(1);
    }

    pb.finish_with_message("✓ All files written");

    formatter.success(&format!("Generated {} files", files.len()));

    Ok(())
}

/// Filter type graph based on include/exclude resource configuration
fn filter_type_graph(
    type_graph: &mut TypeGraph,
    config: &CodegenConfig,
    formatter: &OutputFormatter,
) {
    // If include_resources is specified, only keep those resources
    if !config.fhir.include_resources.is_empty() {
        let include_set: std::collections::HashSet<_> =
            config.fhir.include_resources.iter().cloned().collect();

        type_graph.resources.retain(|name, _| include_set.contains(name));

        formatter.info(&format!("Filtered to {} included resources", type_graph.resources.len()));
    }

    // Apply exclude_resources filter
    if !config.fhir.exclude_resources.is_empty() {
        let exclude_set: std::collections::HashSet<_> =
            config.fhir.exclude_resources.iter().cloned().collect();

        type_graph.resources.retain(|name, _| !exclude_set.contains(name));

        formatter.info(&format!(
            "Excluded {} resources, {} remaining",
            exclude_set.len(),
            type_graph.resources.len()
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_cli_overrides_language() {
        let mut config = CodegenConfig::default();
        let options = GenerateOptions {
            config_path: PathBuf::from("test.toml"),
            language_override: Some("TypeScript".to_string()),
            output_override: None,
            fhir_version_override: None,
            additional_packages: vec![],
            skip_validation: false,
            no_clean: false,
        };

        apply_cli_overrides(&mut config, &options);

        assert!(config.generators.typescript.is_some());
        assert!(config.generators.typescript.as_ref().unwrap().enabled);
    }

    #[test]
    fn test_apply_cli_overrides_output() {
        let mut config = CodegenConfig::default();
        let options = GenerateOptions {
            config_path: PathBuf::from("test.toml"),
            language_override: None,
            output_override: Some(PathBuf::from("/custom/output")),
            fhir_version_override: None,
            additional_packages: vec![],
            skip_validation: false,
            no_clean: false,
        };

        apply_cli_overrides(&mut config, &options);

        assert_eq!(config.output.directory, PathBuf::from("/custom/output"));
    }

    #[test]
    fn test_determine_language_from_override() {
        let config = CodegenConfig::default();
        let options = GenerateOptions {
            config_path: PathBuf::from("test.toml"),
            language_override: Some("TypeScript".to_string()),
            output_override: None,
            fhir_version_override: None,
            additional_packages: vec![],
            skip_validation: false,
            no_clean: false,
        };

        let lang = determine_language(&config, &options).unwrap();
        assert_eq!(lang, "TypeScript");
    }

    #[test]
    fn test_determine_language_from_config() {
        let mut config = CodegenConfig::default();
        config.generators.typescript = Some(crate::cli::config::TypeScriptGeneratorConfig {
            enabled: true,
            ..Default::default()
        });

        let options = GenerateOptions {
            config_path: PathBuf::from("test.toml"),
            language_override: None,
            output_override: None,
            fhir_version_override: None,
            additional_packages: vec![],
            skip_validation: false,
            no_clean: false,
        };

        let lang = determine_language(&config, &options).unwrap();
        assert_eq!(lang, "TypeScript");
    }
}
