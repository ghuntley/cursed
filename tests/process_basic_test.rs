/// Basic process management test
use std::time::Duration;

#[test]
fn test_current_pid() {
    let pid = std::process::id();
    assert!(pid > 0);
    println!("Current PID: {}", pid);
}

#[test]
fn test_echo_command() {
    use std::process::Command;
    
    let output = Command::new("echo")
        .arg("test")
        .output()
        .expect("Failed to execute echo");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test"));
}

#[test]
fn test_basic_process_spawn() {
    use std::process::{Command, Stdio};
    
    let mut child = Command::new("echo")
        .arg("Hello, World!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");
    
    let status = child.wait().expect("Failed to wait for process");
    assert!(status.success());
}
