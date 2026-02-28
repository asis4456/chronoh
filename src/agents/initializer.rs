use crate::error::Result;
use crate::git::GitBridge;
use crate::state::{HandoffManager, StateEngine};
use crate::tools::ToolSet;
use crate::types::{EventType, Phase, ProgressEvent, Role};
use std::path::Path;
use tracing::{debug, info};
use uuid::Uuid;

pub struct InitializerAgent {
    state: StateEngine,
    handoff: HandoffManager,
    session_id: Uuid,
}

impl InitializerAgent {
    pub async fn new(state: StateEngine) -> Result<Self> {
        let handoff = HandoffManager::new(std::path::Path::new(".pi/state")).await?;

        Ok(Self {
            state,
            handoff,
            session_id: Uuid::new_v4(),
        })
    }

    pub async fn initialize(
        &self,
        project_name: &str,
        template: Option<&str>,
        project_path: &Path,
    ) -> Result<()> {
        info!("Initializing project: {}", project_name);

        let init_event = ProgressEvent::new(
            EventType::Init {
                version: "0.1.0".to_string(),
            },
            Phase::InfrastructureReady,
        )
        .with_session_id(self.session_id)
        .with_role(Role::Initializer);

        self.state.append_event(init_event).await?;

        let mut git = GitBridge::new(project_path).await?;
        git.init().await?;

        self.create_project_structure(project_name, template, project_path)
            .await?;

        let commit_hash = git.commit_all("Initial commit: project scaffold").await?;

        let checkpoint_event = ProgressEvent::new(
            EventType::Checkpoint {
                message: "Initial project scaffold".to_string(),
                files: vec![
                    "Cargo.toml".to_string(),
                    "src/main.rs".to_string(),
                    ".gitignore".to_string(),
                ],
            },
            Phase::InfrastructureReady,
        )
        .with_session_id(self.session_id)
        .with_role(Role::Initializer)
        .with_git_commit(&commit_hash);

        self.state.append_event(checkpoint_event).await?;

        self.handoff
            .write_handoff(
                "infrastructure_ready",
                vec![
                    "Project skeleton created".to_string(),
                    "Git repository initialized".to_string(),
                    format!("Template: {}", template.unwrap_or("default")),
                ],
                vec![
                    ("P0".to_string(), "Core business logic".to_string()),
                    ("P1".to_string(), "Authentication".to_string()),
                    ("P2".to_string(), "Tests and documentation".to_string()),
                ],
                vec![
                    format!("Project name: {}", project_name),
                    "Clean State protocol: 50 turn limit".to_string(),
                ],
            )
            .await?;

        info!("Project initialization complete");
        Ok(())
    }

    async fn create_project_structure(
        &self,
        project_name: &str,
        _template: Option<&str>,
        project_path: &Path,
    ) -> Result<()> {
        let cargo_toml = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
            project_name
        );

        ToolSet::write(&project_path.join("Cargo.toml"), &cargo_toml).await?;

        ToolSet::write(
            &project_path.join("src/main.rs"),
            r#"fn main() {
    println!("Hello, world!");
}
",
        ).await?;
        
        ToolSet::write(
            &project_path.join(".gitignore"),
            r#"/target
**/*.rs.bk
Cargo.lock
.pi/
.env
"#,
        )
        .await?;

        Ok(())
    }
}
