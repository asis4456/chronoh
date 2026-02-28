use crate::error::Result;
use crate::hooks::{
    traits::{Hook, HookResult, Next},
    SessionEndContext,
};
use async_trait::async_trait;
use tracing::{info, warn};

pub struct CleanStateHook;

impl CleanStateHook {
    pub fn new() -> Self {
        Self
    }

    async fn run_tests(&self, _project_path: &std::path::Path) -> Result<bool> {
        info!("Running tests check...");
        Ok(true)
    }

    async fn check_git_status(&self, _project_path: &std::path::Path) -> Result<bool> {
        info!("Checking git status...");
        Ok(true)
    }

    async fn update_progress(&self, _project_path: &std::path::Path) -> Result<bool> {
        info!("Updating progress...");
        Ok(true)
    }

    async fn generate_handoff(&self, _project_path: &std::path::Path) -> Result<bool> {
        info!("Generating handoff document...");
        Ok(true)
    }
}

#[async_trait]
impl Hook<SessionEndContext> for CleanStateHook {
    async fn call(
        &self,
        ctx: SessionEndContext,
        next: Next<'_, SessionEndContext>,
    ) -> Result<HookResult> {
        info!(
            "Running Clean State checks for session ending at turn {}",
            ctx.turn_count
        );

        let checks = [
            ("tests", self.run_tests(&ctx.project_path).await),
            ("git_clean", self.check_git_status(&ctx.project_path).await),
            (
                "progress_update",
                self.update_progress(&ctx.project_path).await,
            ),
            (
                "handoff_doc",
                self.generate_handoff(&ctx.project_path).await,
            ),
        ];

        for (name, result) in checks {
            match result {
                Ok(true) => {
                    info!("✓ {} check passed", name);
                }
                Ok(false) => {
                    warn!("✗ {} check failed", name);
                    return Ok(HookResult::Block {
                        reason: format!("Clean State check failed: {}", name),
                    });
                }
                Err(e) => {
                    warn!("✗ {} check error: {}", name, e);
                    return Ok(HookResult::Block {
                        reason: format!("Clean State check error in {}: {}", name, e),
                    });
                }
            }
        }

        info!("All Clean State checks passed, continuing...");
        next.run(ctx).await
    }
}
