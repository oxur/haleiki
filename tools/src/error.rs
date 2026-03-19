//! Centralized error types for the Haleiki CLI.
//!
//! All fallible operations in Haleiki should return [`Result<T>`] which uses
//! [`Error`] as the error type. For context-rich ad-hoc errors that bubble up
//! to the user, prefer `anyhow::Result` in `main` and command handlers;
//! reserve this typed enum for errors that callers need to match on.

/// Errors that can occur during Haleiki operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum Error {
    /// An I/O operation failed.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Failed to parse or emit YAML.
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// Failed to parse or emit JSON.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// A content-validation rule was violated.
    #[error("Validation error: {message}")]
    Validation {
        /// Human-readable description of the validation failure.
        message: String,
    },

    /// A configuration value is missing or invalid.
    #[error("Configuration error: {message}")]
    Config {
        /// Human-readable description of the configuration problem.
        message: String,
    },
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;
