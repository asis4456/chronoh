use chrono_h::git::GitBridge;
use tempfile::TempDir;

#[tokio::test]
async fn test_git_init_and_commit() {
    let temp_dir = TempDir::new().unwrap();
    let mut bridge = GitBridge::new(temp_dir.path()).await.unwrap();
    
    bridge.init().await.unwrap();
    
    let file_path = temp_dir.path().join("test.txt");
    tokio::fs::write(&file_path, "hello").await.unwrap();
    
    let commit_hash = bridge.commit_all("Initial commit").await.unwrap();
    
    assert!(!commit_hash.is_empty());
    assert_eq!(commit_hash.len(), 40);
}

#[tokio::test]
async fn test_git_status_clean() {
    let temp_dir = TempDir::new().unwrap();
    let mut bridge = GitBridge::new(temp_dir.path()).await.unwrap();
    
    bridge.init().await.unwrap();
    
    let is_clean = bridge.is_clean().await.unwrap();
    assert!(is_clean);
    
    let file_path = temp_dir.path().join("dirty.txt");
    tokio::fs::write(&file_path, "dirty").await.unwrap();
    
    let is_clean = bridge.is_clean().await.unwrap();
    assert!(!is_clean);
}
