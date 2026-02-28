use chrono_h::state::HandoffManager;
use tempfile::TempDir;

#[tokio::test]
async fn test_handoff_create_and_read() {
    let temp_dir = TempDir::new().unwrap();
    let manager = HandoffManager::new(temp_dir.path()).await.unwrap();
    
    let completed = vec!["Project skeleton".to_string()];
    let todo = vec![
        ("P0".to_string(), "User auth".to_string()),
        ("P1".to_string(), "Todo CRUD".to_string()),
    ];
    let decisions = vec!["Use FastAPI".to_string()];
    
    manager.write_handoff(
        "infrastructure_ready",
        completed,
        todo,
        decisions,
    ).await.unwrap();
    
    let content = manager.read_handoff().await.unwrap();
    assert!(content.contains("infrastructure_ready"));
    assert!(content.contains("User auth"));
    assert!(content.contains("Use FastAPI"));
}
