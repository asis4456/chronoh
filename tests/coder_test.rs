use chrono_h::agents::CoderAgent;
use chrono_h::state::StateEngine;
use tempfile::TempDir;

#[tokio::test]
async fn test_coder_agent_creation() {
    let temp_dir = TempDir::new().unwrap();
    let state = StateEngine::new(temp_dir.path()).await.unwrap();
    
    let coder = CoderAgent::new(state, 50).await.unwrap();
    
    assert_eq!(coder.max_turns(), 50);
}
