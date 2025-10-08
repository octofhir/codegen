//! Code generation traits and utilities

pub mod traits;

pub use traits::{
    CodeGenerator, FileType, GeneratedCode, GeneratedFile, GenerationManifest,
    GenerationStatistics, GeneratorCapabilities, GeneratorConfig, GeneratorMetadata,
    IdentifierContext, Language, LanguageBackend, TemplateContext, TemplateEngine,
};
