//! # OctoFHIR Codegen
//!
//! Generate type-safe FHIR SDKs for multiple programming languages.
//!
//! ## Features
//!
//! - Multi-language support (TypeScript, Rust, Python, Go)
//! - Trait-based extensible architecture
//! - Deep integration with canonical-manager
//! - Superior type safety and developer experience
//!
//! ## Example
//!
//! ```rust,no_run
//! use octofhir_codegen::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Implementation will be added in subsequent tasks
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod config;
pub mod core;
pub mod generator;
pub mod languages;
pub mod templates;

#[cfg(feature = "cli")]
pub mod cli;

pub use crate::core::error::{Error, Result};

/// Build a TypeGraph from a FHIR package
///
/// # Example
///
/// ```rust,no_run
/// use octofhir_codegen::build_type_graph;
/// use octofhir_codegen::core::ir::FhirVersion;
/// use octofhir_canonical_manager::{CanonicalManager, FcmConfig};
/// use std::sync::Arc;
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = FcmConfig::load().await?;
/// let manager = Arc::new(CanonicalManager::new(config).await?);
///
/// let graph = build_type_graph(
///     manager,
///     FhirVersion::R4,
///     "hl7.fhir.r4.core",
///     "4.0.1"
/// ).await?;
///
/// println!("Built graph with {} types", graph.total_types());
/// # Ok(())
/// # }
/// ```
pub async fn build_type_graph(
    manager: std::sync::Arc<octofhir_canonical_manager::CanonicalManager>,
    fhir_version: core::ir::FhirVersion,
    package_name: &str,
    package_version: &str,
) -> Result<core::ir::TypeGraph> {
    let builder = core::TypeGraphBuilder::new(manager, fhir_version);
    builder.build_from_package(package_name, package_version).await
}

/// Re-export commonly used types and traits
pub mod prelude {
    pub use crate::Error;
    pub use crate::Result;
    pub use crate::core::*;
    pub use crate::generator::*;
}
