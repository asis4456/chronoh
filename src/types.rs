use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    InfrastructureReady,
    AuthReady,
    CoreApiReady,
    ProductionReady,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Initializer,
    Coder,
    Reviewer,
    Compactor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventType {
    Init {
        version: String,
    },
    SessionStart {
        role: Role,
    },
    Checkpoint {
        message: String,
        files: Vec<String>,
    },
    TaskComplete {
        task: String,
        tests_passed: Option<u32>,
    },
    ContextCompaction {
        compression_ratio: f32,
    },
    SessionEnd {
        turns_used: u32,
        reason: EndReason,
    },
    ProjectComplete,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EndReason {
    TurnLimitReached,
    TaskCompleted,
    UserRequested,
    Error { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEvent {
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub event_type: EventType,
    pub session_id: Option<Uuid>,
    pub git_commit: Option<String>,
    pub phase: Phase,
    pub metadata: serde_json::Value,
}

impl ProgressEvent {
    pub fn new(event_type: EventType, phase: Phase) -> Self {
        Self {
            timestamp: Utc::now(),
            event_type,
            session_id: None,
            git_commit: None,
            phase,
            metadata: serde_json::Value::Null,
        }
    }

    pub fn with_session_id(mut self, id: Uuid) -> Self {
        self.session_id = Some(id);
        self
    }

    pub fn with_role(mut self, role: Role) -> Self {
        self.event_type = match self.event_type {
            EventType::SessionStart { .. } => EventType::SessionStart { role },
            other => other,
        };
        self
    }

    pub fn with_git_commit(mut self, commit: impl Into<String>) -> Self {
        self.git_commit = Some(commit.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub max_turns: u32,
    pub session_cap: u32,
    pub auto_compact: bool,
    pub clean_state_required: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            max_turns: 50,
            session_cap: 3,
            auto_compact: true,
            clean_state_required: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub project_type: String,
    pub state_path: std::path::PathBuf,
}
