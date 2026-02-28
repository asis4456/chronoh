use chrono_h::error::Error;

#[test]
fn test_io_error_conversion() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err: Error = io_err.into();

    match err {
        Error::Io(_) => (),
        _ => panic!("Expected Io error"),
    }
}

#[test]
fn test_state_error_display() {
    let err = Error::StateCorrupted {
        key: "progress".to_string(),
        reason: "invalid JSON".to_string(),
    };

    let msg = err.to_string();
    assert!(msg.contains("progress"));
    assert!(msg.contains("invalid JSON"));
}
