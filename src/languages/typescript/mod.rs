//! TypeScript code generation

mod backend;
/// Base class generation for TypeScript (Resource, DomainResource, Element)
pub mod base_class_generator;
pub mod choice_generator;
/// TypeScript class generation with fluent builder API
pub mod class_generator;
pub mod datatype_generator;
/// Documentation generation for TypeScript with JSDoc and TypeDoc support
pub mod documentation_generator;
/// Extension helper methods generation for TypeScript
pub mod extension_generator;
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
pub use base_class_generator::BaseClassGenerator;
pub use choice_generator::ChoiceElementGenerator;
pub use class_generator::ClassGenerator;
pub use datatype_generator::DatatypeGenerator;
pub use documentation_generator::DocumentationGenerator;
pub use extension_generator::{ExtensionDefinition, ExtensionGenerator, ExtensionValueType};
pub use helpers_generator::HelpersGenerator;
pub use manifest_generator::{ManifestGenerator, PackageConfig};
pub use resource_generator::ResourceGenerator;
pub use sdk_generator::TypeScriptSdkGenerator;
pub use validation_generator::ValidationGenerator;
