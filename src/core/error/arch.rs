use thiserror::Error;

/// Process exit code returned from `main` via [`AppError::report`].
pub type AppExitCode = std::process::ExitCode;

/// The crate-wide result type. Every fallible function returns this.
pub type AppResult<T> = Result<T, AppError>;

/// Every failure in agentx, typed and neutral.
///
/// Control-flow signals that are NOT failures (drain, blocked) live in `app`,
/// never here — `core` must stay free of agentx semantics.
#[derive(Debug, Error)]
pub enum AppError {

    #[error("{0}")]
    Message(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{format} parse error: {message}")]
    Parse { format: String, message: String },

    #[error("not found: {0}")]
    NotFound(String),

    #[error("invalid {what}: {message}")]
    Invalid { what: String, message: String },

    #[error("{what} timed out after {secs}s")]
    Timeout { what: String, secs: u64 },

    #[error("command `{name}` failed ({code})")]
    Command { name: String, code: i32, stderr: String },
}
