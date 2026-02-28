pub mod engine;
pub mod handoff;

pub use engine::StateEngine;
pub use handoff::HandoffManager;

use crate::error::Result;
use crate::types::ProgressEvent;

#[async_trait::async_trait]
pub trait StateBackend: Send + Sync {
    async fn append(&self, key: String, value: Vec<u8>) -> Result<()>;
    async fn get_all(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>>;
    async fn flush(&self) -> Result<()>;
}
