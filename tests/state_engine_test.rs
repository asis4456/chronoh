use chrono_h::state::StateEngine;
use chrono_h::types::{ProgressEvent, EventType, Phase};
use tempfile::TempDir;

#[tokio::test]
async fn test_state_engine_create_and_append() {
    let temp_dir = TempDir::new().unwrap();
    let engine = StateEngine::new(temp_dir.path()).await.unwrap();
    
    let event = ProgressEvent::new(
        EventType::Init { version: "1.0".to_string() },
        Phase::InfrastructureReady,
    );
    
    engine.append_event(event.clone()).await.unwrap();
    
    let events = engine.get_all_events().await.unwrap();
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0].event_type, EventType::Init { .. }));
}

#[tokio::test]
async fn test_state_engine_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_path_buf();
    
    {
        let engine = StateEngine::new(&path).await.unwrap();
        let event = ProgressEvent::new(
            EventType::ProjectComplete,
            Phase::ProductionReady,
        );
        engine.append_event(event).await.unwrap();
    }
    
    {
        let engine = StateEngine::new(&path).await.unwrap();
        let events = engine.get_all_events().await.unwrap();
        assert_eq!(events.len(), 1);
    }
}
