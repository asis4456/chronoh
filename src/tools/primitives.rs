use crate::error::{Error, Result};
use std::path::Path;
use std::time::Duration;
use tokio::fs;
use tokio::process::Command;
use tokio::time::timeout;

pub struct ToolSet;

#[derive(Debug)]
pub struct ExecResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub success: bool,
}

impl ToolSet {
    pub async fn read(path: &Path, offset: Option<usize>, limit: Option<usize>) -> Result<String> {
        let content = fs::read_to_string(path).await?;

        let lines: Vec<&str> = content.lines().collect();
        let start = offset.unwrap_or(0).min(lines.len());
        let end = limit
            .map(|l| (start + l).min(lines.len()))
            .unwrap_or(lines.len());

        Ok(lines[start..end].join("\n"))
    }

    pub async fn write(path: &Path, content: &str) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let temp_path = path.with_extension("tmp");
        fs::write(&temp_path, content).await?;
        fs::rename(&temp_path, path).await?;

        Ok(())
    }

    pub async fn edit(path: &Path, old_string: &str, new_string: &str) -> Result<()> {
        let content = fs::read_to_string(path).await?;

        if !content.contains(old_string) {
            return Err(Error::Validation(format!(
                "Edit target not found in file: {}",
                old_string
            )));
        }

        let new_content = content.replace(old_string, new_string);
        Self::write(path, &new_content).await
    }

    pub async fn bash(
        command: &str,
        timeout_secs: Option<u64>,
        cwd: Option<&Path>,
    ) -> Result<ExecResult> {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(command);

        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        let output = if let Some(secs) = timeout_secs {
            match timeout(Duration::from_secs(secs), cmd.output()).await {
                Ok(Ok(output)) => output,
                Ok(Err(e)) => return Err(e.into()),
                Err(_) => {
                    return Err(Error::Validation(format!(
                        "Command timed out after {} seconds: {}",
                        secs, command
                    )));
                }
            }
        } else {
            cmd.output().await?
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code();
        let success = output.status.success();

        if !success && stderr.len() > 0 {
            return Err(Error::tool_execution(command, stderr.clone()));
        }

        Ok(ExecResult {
            stdout,
            stderr,
            exit_code,
            success,
        })
    }
}
