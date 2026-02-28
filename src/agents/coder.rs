use crate::error::{Error, Result};
use crate::hooks::{CleanStateHook, Hook, HookChain, HookResult, SessionEndContext};
use crate::state::{HandoffManager, StateEngine};
use crate::types::{EndReason, EventType, ProgressEvent, Role};
use std::path::Path;
use tracing::{debug, info, warn};
use uuid::Uuid;

pub struct CoderAgent {
    state: StateEngine,
    handoff: HandoffManager,
    session_id: Uuid,
    turn_count: u32,
    max_turns: u32,
    clean_state_hook: CleanStateHook,
}

impl CoderAgent {
    pub async fn new(state: StateEngine, max_turns: u32) -> Result<Self> {
        let handoff = HandoffManager::new(std::path::Path::new(".pi/state")).await?;

        Ok(Self {
            state,
            handoff,
            session_id: Uuid::new_v4(),
            turn_count: 0,
            max_turns,
            clean_state_hook: CleanStateHook::new(),
        })
    }

    pub fn max_turns(&self) -> u32 {
        self.max_turns
    }

    pub fn current_turn(&self) -> u32 {
        self.turn_count
    }

    pub async fn start_session(&mut self, project_path: &Path) -> Result<()> {
        info!("Starting Coder session {}", self.session_id);

        let phase = self.state.get_current_phase().await?;
        let handoff_content = self.handoff.read_handoff().await?;

        debug!("Current phase: {:?}", phase);
        debug!("Handoff content: {}", handoff_content);

        let event =
            ProgressEvent::new(EventType::SessionStart { role: Role::Coder }, phase.clone())
                .with_session_id(self.session_id)
                .with_role(Role::Coder);

        self.state.append_event(event).await?;

        info!("Coder session started. Max turns: {}", self.max_turns);
        Ok(())
    }

    pub async fn increment_turn(&mut self) -> Result<()> {
        self.turn_count += 1;

        if self.turn_count >= self.max_turns {
            warn!("Turn limit reached: {}/{}", self.turn_count, self.max_turns);
            return Err(Error::SessionLimitExceeded {
                turns: self.turn_count,
            });
        }

        Ok(())
    }

    pub async fn end_session(&self, project_path: &Path, reason: EndReason) -> Result<()> {
        info!("Ending Coder session {}", self.session_id);

        let ctx = SessionEndContext {
            turn_count: self.turn_count,
            project_path: project_path.to_path_buf(),
        };

        let chain: HookChain<SessionEndContext> = HookChain::new(vec![
            Box::new(CleanStateHook::new()) as Box<dyn Hook<SessionEndContext>>
        ]);
        
        let hook_result = chain.execute(ctx.clone()).await?;
        
        match hook_result {
            HookResult::Continue => {
                info!("Clean state check passed, ending session...");
            }
            HookResult::Block { reason } => {
                warn!("Clean state check blocked session end: {}", reason);
                return Err(Error::HookBlocked { reason });
            }
        }

        let phase = self.state.get_current_phase().await?;
        let event = ProgressEvent::new(
            EventType::SessionEnd {
                turns_used: self.turn_count,
                reason,
            },
            phase,
        )
        .with_session_id(self.session_id)
        .with_role(Role::Coder);

        self.state.append_event(event).await?;

        info!(
            "Coder session {} ended after {} turns",
            self.session_id, self.turn_count
        );
        Ok(())
    }
}
