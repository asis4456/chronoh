use crate::error::Result;
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct HandoffManager {
    handoff_path: PathBuf,
}

impl HandoffManager {
    pub async fn new(state_path: &Path) -> Result<Self> {
        let handoff_path = state_path.join("handoff.md");

        if let Some(parent) = handoff_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        Ok(Self { handoff_path })
    }

    pub async fn write_handoff(
        &self,
        phase: &str,
        completed: Vec<String>,
        todo: Vec<(String, String)>,
        decisions: Vec<String>,
    ) -> Result<()> {
        let mut content = format!("## Phase: {}\n\n", phase);

        content.push_str("### Completed\n");
        for item in completed {
            content.push_str(&format!("- [x] {}\n", item));
        }
        content.push('\n');

        content.push_str("### Todo (Prioritized)\n");
        for (priority, task) in todo {
            content.push_str(&format!("- [{}] {}\n", priority, task));
        }
        content.push('\n');

        content.push_str("### Technical Decisions\n");
        for decision in decisions {
            content.push_str(&format!("- {}\n", decision));
        }
        content.push('\n');

        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        content.push_str(&format!("\n---\n*Last updated: {}*\n", timestamp));

        fs::write(&self.handoff_path, content).await?;
        Ok(())
    }

    pub async fn read_handoff(&self) -> Result<String> {
        match fs::read_to_string(&self.handoff_path).await {
            Ok(content) => Ok(content),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                Ok("# No handoff document found\n".to_string())
            }
            Err(e) => Err(e.into()),
        }
    }

    pub async fn append_to_section(&self, section: &str, content: &str) -> Result<()> {
        let mut current = self.read_handoff().await?;

        let section_header = format!("### {}", section);
        if let Some(pos) = current.find(&section_header) {
            let insert_pos = current[pos..]
                .find('\n')
                .map(|i| pos + i + 1)
                .unwrap_or(current.len());
            current.insert_str(insert_pos, &format!("{}\n", content));
            fs::write(&self.handoff_path, current).await?;
        }

        Ok(())
    }
}
