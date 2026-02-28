use chrono::Utc;
use chrono_h::types::{EventType, Phase, ProgressEvent, Role};
use uuid::Uuid;

#[test]
fn test_phase_serialization() {
    let phase = Phase::InfrastructureReady;
    let json = serde_json::to_string(&phase).unwrap();
    assert_eq!(json, "\"infrastructure_ready\"");

    let decoded: Phase = serde_json::from_str(&json).unwrap();
    assert!(matches!(decoded, Phase::InfrastructureReady));
}

#[test]
fn test_progress_event_roundtrip() {
    let event = ProgressEvent {
        timestamp: Utc::now(),
        event_type: EventType::Init {
            version: "1.0".to_string(),
        },
        session_id: Some(Uuid::new_v4()),
        git_commit: Some("abc123".to_string()),
        phase: Phase::InfrastructureReady,
        metadata: serde_json::json!({"key": "value"}),
    };

    let json = serde_json::to_string(&event).unwrap();
    let decoded: ProgressEvent = serde_json::from_str(&json).unwrap();

    assert!(matches!(decoded.event_type, EventType::Init { .. }));
}
