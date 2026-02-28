pub mod agents;
pub mod cli;
pub mod error;
pub mod git;
pub mod hooks;
pub mod state;
pub mod tools;
pub mod types;

pub use agents::InitializerAgent;
pub use error::{Error, Result};
pub use git::GitBridge;
pub use hooks::*;
pub use state::{HandoffManager, StateEngine};
pub use tools::{ExecResult, ToolSet};
pub use types::*;
