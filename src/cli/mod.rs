//! Command-line interface
//!
//! This module provides CLI infrastructure including configuration management,
//! command parsing, and output formatting.

pub mod commands;
pub mod config;
pub mod discovery;
pub mod generate;
pub mod output;

pub use commands::{Cli, CommandResult, Commands};
pub use config::CodegenConfig;
pub use discovery::{DiscoveryResult, discover_config, ensure_config_exists};
pub use generate::{GenerateOptions, GenerationResult, execute_generate};
pub use output::{
    OutputFormat, OutputFormatter, ProgressReporter, Verbosity, create_detailed_progress_bar,
    create_multi_progress, create_progress_bar, create_spinner,
};
