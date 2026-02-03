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
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Self::Io { source }
    }
}
