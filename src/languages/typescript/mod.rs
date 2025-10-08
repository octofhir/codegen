//! TypeScript code generation

mod backend;
pub mod choice_generator;
pub mod datatype_generator;
/// Documentation generation for TypeScript with JSDoc and TypeDoc support
pub mod documentation_generator;
/// Helper methods generation for TypeScript
pub mod helpers_generator;
/// Package manifest generation for TypeScript
pub mod manifest_generator;
pub mod resource_generator;
/// Complete TypeScript SDK generation orchestrator
pub mod sdk_generator;
pub mod templates;
/// Validation functions generation for TypeScript
pub mod validation_generator;

pub use backend::TypeScriptBackend;
pub use choice_generator::ChoiceElementGenerator;
pub use datatype_generator::DatatypeGenerator;
pub use documentation_generator::DocumentationGenerator;
pub use helpers_generator::HelpersGenerator;
pub use manifest_generator::{ManifestGenerator, PackageConfig};
pub use resource_generator::ResourceGenerator;
pub use sdk_generator::TypeScriptSdkGenerator;
pub use validation_generator::ValidationGenerator;
