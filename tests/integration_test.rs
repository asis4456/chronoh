use std::process::Command;
use tempfile::TempDir;

fn get_bin_path() -> String {
    let output = Command::new("cargo")
        .args([
            "metadata",
            "--format-version=1",
            "--manifest-path",
            "/Users/haomintsai/workspace/apps/linux/hardness/Cargo.toml",
        ])
        .output()
        .expect("Failed to get cargo metadata");

    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let target_dir = json["target_directory"].as_str().unwrap();
    format!("{}/debug/chrono-h", target_dir)
}

#[test]
fn test_cli_init_creates_project() {
    let temp_dir = TempDir::new().unwrap();
    let project_name = "my-test-project";
    let bin_path = get_bin_path();

    let output = Command::new(&bin_path)
        .args(["init", "--name", project_name])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    assert!(output.status.success(), "Command failed: {}", stderr);

    let project_path = temp_dir.path().join(project_name);
    assert!(project_path.exists(), "Project directory not created");

    assert!(
        project_path.join("Cargo.toml").exists(),
        "Cargo.toml not created"
    );
    assert!(
        project_path.join("src/main.rs").exists(),
        "src/main.rs not created"
    );
    assert!(project_path.join(".git").exists(), ".git not created");
    assert!(
        project_path.join(".pi/state").exists(),
        ".pi/state not created"
    );
}

#[test]
fn test_cli_status_after_init() {
    let temp_dir = TempDir::new().unwrap();
    let project_name = "status-test-project";
    let bin_path = get_bin_path();

    Command::new(&bin_path)
        .args(["init", "--name", project_name])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to init project");

    let project_path = temp_dir.path().join(project_name);

    let output = Command::new(&bin_path)
        .args(["status"])
        .current_dir(&project_path)
        .output()
        .expect("Failed to execute status command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    assert!(output.status.success(), "Status command failed: {}", stderr);
    assert!(
        stdout.contains("Project Status"),
        "Status output missing header"
    );
    assert!(stdout.contains("InfrastructureReady"), "Phase not shown");
}

#[test]
fn test_cli_dev_session() {
    let temp_dir = TempDir::new().unwrap();
    let project_name = "dev-test-project";
    let bin_path = get_bin_path();

    Command::new(&bin_path)
        .args(["init", "--name", project_name])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to init project");

    let project_path = temp_dir.path().join(project_name);

    let output = Command::new(&bin_path)
        .args(["dev"])
        .current_dir(&project_path)
        .output()
        .expect("Failed to execute dev command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    assert!(output.status.success(), "Dev command failed: {}", stderr);
    assert!(
        stdout.contains("Coder session"),
        "Coder session not started"
    );
}

#[test]
fn test_cli_not_a_project() {
    let temp_dir = TempDir::new().unwrap();
    let bin_path = get_bin_path();

    let output = Command::new(&bin_path)
        .args(["status"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute status command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("Not a ChronoH project"),
        "Should show not a project error"
    );
}
