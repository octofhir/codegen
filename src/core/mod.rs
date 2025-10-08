//! Core functionality for codegen

pub mod error;
pub mod ir;
pub mod parser;

pub use error::{Error, Result};
pub use ir::TypeGraph;
pub use parser::StructureDefinitionParser;
