use crate::error::{Error, Result};
use crate::state::StateBackend;
use crate::types::ProgressEvent;
use sled::Db;
use std::path::Path;
use tracing::{debug, info};

pub struct StateEngine {
    backend: SledBackend,
}

struct SledBackend {
    db: Db,
}

#[async_trait::async_trait]
impl StateBackend for SledBackend {
    async fn append(&self, key: String, value: Vec<u8>) -> Result<()> {
        self.db.insert(key, value)?;
        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        let mut results = Vec::new();
        for item in self.db.iter() {
            let (k, v) = item?;
            results.push((k.to_vec(), v.to_vec()));
        }
        Ok(results)
    }

    async fn flush(&self) -> Result<()> {
        self.db.flush_async().await?;
        Ok(())
    }
}

impl StateEngine {
    pub async fn new(state_path: &Path) -> Result<Self> {
        let db_path = state_path.join("state.sled");
        let db = sled::open(&db_path)?;

        info!("StateEngine initialized at {:?}", db_path);

        Ok(Self {
            backend: SledBackend { db },
        })
    }

    pub async fn append_event(&self, event: ProgressEvent) -> Result<()> {
        let key = format!(
            "{}-{:?}",
            event.timestamp.timestamp_millis(),
            std::thread::current().id()
        );

        let value = serde_json::to_vec(&event)
            .map_err(|e| Error::state_corrupted(key.clone(), e.to_string()))?;

        debug!("Appending event: {:?}", event.event_type);
        self.backend.append(key, value).await?;
        self.backend.flush().await?;

        info!("Event appended and flushed to disk");
        Ok(())
    }

    pub async fn get_all_events(&self) -> Result<Vec<ProgressEvent>> {
        let raw_data = self.backend.get_all().await?;
        let mut events = Vec::new();

        for (key, value) in raw_data {
            let event: ProgressEvent = serde_json::from_slice(&value).map_err(|e| {
                let key_str = String::from_utf8_lossy(&key);
                Error::state_corrupted(key_str.to_string(), e.to_string())
            })?;
            events.push(event);
        }

        events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        Ok(events)
    }

    pub async fn get_current_phase(&self) -> Result<crate::types::Phase> {
        let events = self.get_all_events().await?;

        if let Some(last) = events.last() {
            Ok(last.phase.clone())
        } else {
            Ok(crate::types::Phase::InfrastructureReady)
        }
    }

    pub async fn get_last_session(&self) -> Result<Option<ProgressEvent>> {
        let events = self.get_all_events().await?;

        Ok(events.into_iter().rev().find(|e| {
            matches!(
                e.event_type,
                crate::types::EventType::SessionEnd { .. }
                    | crate::types::EventType::SessionStart { .. }
            )
        }))
    }
}
