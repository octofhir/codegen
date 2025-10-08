//! Error types for codegen

use thiserror::Error;

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for codegen operations
#[derive(Error, Debug)]
pub enum Error {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON parsing error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// TOML parsing error
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    /// Canonical manager error
    #[error("Canonical manager error: {0}")]
    CanonicalManager(String),

    /// Parser error
    #[error("Parser error: {0}")]
    Parser(String),

    /// Generator error
    #[error("Generator error: {0}")]
    Generator(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Template error
    #[error("Template error: {0}")]
    Template(String),

    /// Other errors
    #[error("{0}")]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_types() {
        let err = Error::Parser("test error".to_string());
        assert_eq!(err.to_string(), "Parser error: test error");
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(err.to_string().contains("IO error"));
    }
}
