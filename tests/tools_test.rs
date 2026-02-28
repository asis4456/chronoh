use chrono_h::tools::ToolSet;
use tempfile::TempDir;
use tokio::fs;

#[tokio::test]
async fn test_read_file_with_offset() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    
    let content = "line1\nline2\nline3\nline4\nline5";
    fs::write(&file_path, content).await.unwrap();
    
    let result = ToolSet::read(&file_path, Some(1), Some(2)).await.unwrap();
    assert_eq!(result, "line2\nline3");
}

#[tokio::test]
async fn test_write_file_atomic() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    
    ToolSet::write(&file_path, "hello world").await.unwrap();
    
    let content = fs::read_to_string(&file_path).await.unwrap();
    assert_eq!(content, "hello world");
}

#[tokio::test]
async fn test_edit_file_precise() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    
    fs::write(&file_path, "hello world").await.unwrap();
    ToolSet::edit(&file_path, "world", "rust").await.unwrap();
    
    let content = fs::read_to_string(&file_path).await.unwrap();
    assert_eq!(content, "hello rust");
}

#[tokio::test]
async fn test_bash_execution() {
    let result = ToolSet::bash("echo 'hello'", Some(5), None).await.unwrap();
    
    assert!(result.success);
    assert!(result.stdout.contains("hello"));
    assert!(result.exit_code == Some(0));
}

#[tokio::test]
async fn test_bash_timeout() {
    let result = ToolSet::bash("sleep 10", Some(1), None).await;
    
    assert!(result.is_err());
}
