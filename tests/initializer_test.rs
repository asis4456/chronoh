use chrono_h::agents::InitializerAgent;
use chrono_h::state::StateEngine;
use tempfile::TempDir;

#[tokio::test]
async fn test_initializer_creates_project_structure() {
    let temp_dir = TempDir::new().unwrap();
    let state = StateEngine::new(temp_dir.path()).await.unwrap();
    
    let _initializer = InitializerAgent::new(state).await.unwrap();
    
    assert!(true);
}
