//! Core functionality for codegen

pub mod error;
pub mod graph_builder;
pub mod ir;
pub mod parser;
pub mod resolver;

pub use error::{Error, Result};
pub use graph_builder::TypeGraphBuilder;
pub use ir::TypeGraph;
pub use parser::StructureDefinitionParser;
pub use resolver::SchemaResolver;
