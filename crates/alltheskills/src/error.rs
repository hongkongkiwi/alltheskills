//! Error types for the AllTheSkills library

use thiserror::Error;

/// Errors that can occur when working with skills
#[derive(Debug, Error)]
pub enum Error {
    /// I/O error occurred
    #[error("IO error: {source}")]
    Io {
        #[source]
        source: std::io::Error,
    },

    /// Failed to parse skill data
    #[error("Parse error: {message}")]
    Parse { message: String },

    /// Skill was not found
    #[error("Skill not found: {name}")]
    NotFound { name: String },

    /// Unsupported skill format
    #[error("Unsupported format: {format}")]
    UnsupportedFormat { format: String },

    /// Configuration error
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// Installation failed
    #[error("Installation failed: {reason}")]
    Install { reason: String },

    /// Git operation failed
    #[error("Git error: {source}")]
    Git {
        #[source]
        source: git2::Error,
    },

    /// JSON parsing error
    #[error("JSON error: {source}")]
    Json {
        #[source]
        source: serde_json::Error,
    },
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Self::Io { source }
    }
}

impl From<git2::Error> for Error {
    fn from(source: git2::Error) -> Self {
        Self::Git { source }
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Self::Json { source }
    }
}
