use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {source}")]
    Io { source: std::io::Error },

    #[error("Parse error: {message}")]
    Parse { message: String },

    #[error("Skill not found: {name}")]
    NotFound { name: String },

    #[error("Unsupported format: {format}")]
    UnsupportedFormat { format: String },

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Installation failed: {reason}")]
    Install { reason: String },

    #[error("Git error: {source}")]
    Git { source: git2::Error },

    #[error("JSON error: {source}")]
    Json { source: serde_json::Error },
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
