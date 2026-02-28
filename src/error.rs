use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Sled error: {0}")]
    Sled(#[from] sled::Error),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("State corrupted for key '{key}': {reason}")]
    StateCorrupted { key: String, reason: String },

    #[error("State not found: {0}")]
    StateNotFound(String),

    #[error("Git operation failed: {0}")]
    GitError(String),

    #[error("Tool execution failed: {command} - {stderr}")]
    ToolExecution { command: String, stderr: String },

    #[error("Hook blocked: {reason}")]
    HookBlocked { reason: String },

    #[error("Session limit exceeded: {turns} turns")]
    SessionLimitExceeded { turns: u32 },

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Agent error: {0}")]
    Agent(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub fn state_corrupted(key: impl Into<String>, reason: impl Into<String>) -> Self {
        Error::StateCorrupted {
            key: key.into(),
            reason: reason.into(),
        }
    }

    pub fn tool_execution(command: impl Into<String>, stderr: impl Into<String>) -> Self {
        Error::ToolExecution {
            command: command.into(),
            stderr: stderr.into(),
        }
    }
}
