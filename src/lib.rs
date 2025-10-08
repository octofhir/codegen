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

pub mod core;
pub mod generator;
pub mod languages;
pub mod templates;

#[cfg(feature = "cli")]
pub mod cli;

pub use crate::core::error::{Error, Result};

/// Re-export commonly used types and traits
pub mod prelude {
    pub use crate::Error;
    pub use crate::Result;
    pub use crate::core::*;
    pub use crate::generator::*;
}
