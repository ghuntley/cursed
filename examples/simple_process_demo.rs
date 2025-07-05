/// Simple Process Management Demo for CURSED
/// Demonstrates basic process management functionality

use cursed::stdlib::process::safe_process_management::{SecurityContext, ProcessIsolation};
use std::time::Duration;
use std::process::{Command, Stdio};
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 CURSED Simple Process Management Demo");
    println!("========================================");
    
    // Demo 1: Basic process spawning
    println!("\n1. Basic Process Spawning:");
    let mut child = Command::new("echo")
        .arg("Hello from CURSED process!")
        .stdout(Stdio::piped())
        .spawn()?;
    
    let output = child.wait_with_output()?;
    println!("Process output: {}", String::from_utf8_lossy(&output.stdout));
    println!("Exit code: {}", output.status.code().unwrap_or(-1));
    
    // Demo 2: Environment variables
    println!("\n2. Environment Variables:");
    #[cfg(unix)]
    let env_command = "env";
    #[cfg(windows)]
    let env_command = "set";
    
    let mut env_child = Command::new(env_command)
        .env("CURSED_DEMO", "environment_test")
        .stdout(Stdio::piped())
        .spawn()?;
    
    let env_output = env_child.wait_with_output()?;
    let env_str = String::from_utf8_lossy(&env_output.stdout);
    
    if env_str.contains("CURSED_DEMO") {
        println!("✓ Environment variable successfully set and retrieved");
    } else {
        println!("Environment variables (partial output):");
        let lines: Vec<&str> = env_str.lines().take(5).collect();
        for line in lines {
            println!("  {}", line);
        }
    }
    
    // Demo 3: Working directory
    println!("\n3. Working Directory:");
    let temp_dir = std::env::temp_dir();
    
    #[cfg(unix)]
    let wd_command = "pwd";
    #[cfg(windows)]
    let wd_command = "cd";
    
    let mut wd_child = Command::new(wd_command)
        .current_dir(&temp_dir)
        .stdout(Stdio::piped())
        .spawn()?;
    
    let wd_output = wd_child.wait_with_output()?;
    println!("Working directory output: {}", String::from_utf8_lossy(&wd_output.stdout));
    
    // Demo 4: Command utilities
    println!("\n4. Command Utilities:");
    println!("Echo command exists: {}", command_exists("echo"));
    println!("Nonexistent command exists: {}", command_exists("nonexistent_xyz"));
    
    if let Some(echo_path) = which("echo") {
        println!("Echo command path: {}", echo_path);
    }
    
    // Demo 5: Process information
    println!("\n5. Process Information:");
    let current_pid = get_current_pid();
    println!("Current PID: {}", current_pid);
    
    if let Some(process_info) = get_process_info(current_pid) {
        println!("Process name: {}", process_info.name);
        println!("Process running: {}", is_process_running(current_pid));
    }
    
    // Demo 6: Simple shell command execution
    println!("\n6. Shell Command Execution:");
    let shell_output = run_command("echo 'Shell command executed successfully'")?;
    println!("Shell output: {}", String::from_utf8_lossy(&shell_output.stdout));
    
    // Demo 7: Timeout handling
    println!("\n7. Timeout Handling:");
    #[cfg(unix)]
    {
        match run_command_timeout("sleep 0.1", Duration::from_millis(200)) {
            Ok(_) => println!("✓ Short command completed within timeout"),
            Err(_) => println!("✗ Short command failed"),
        }
        
        match run_command_timeout("sleep 2", Duration::from_millis(100)) {
            Ok(_) => println!("✗ Long command unexpectedly completed"),
            Err(_) => println!("✓ Long command correctly timed out"),
        }
    }
    
    #[cfg(windows)]
    {
        println!("Timeout demo skipped on Windows (uses sleep command)");
    }
    
    // Demo 8: Security Context
    println!("\n8. Security Context:");
    let security_ctx = SecurityContext::new("demo_user".to_string())
        .with_permissions(vec!["read".to_string(), "write".to_string()])
        .sandboxed();
    
    println!("Security context created for user: {}", security_ctx.user_id);
    println!("Permissions: {:?}", security_ctx.permissions);
    println!("Sandboxed: {}", security_ctx.sandbox);
    
    // Demo 9: Process Isolation
    println!("\n9. Process Isolation:");
    let isolation = ProcessIsolation::new("demo_namespace".to_string());
    
    println!("Namespace: {}", isolation.namespace);
    println!("Memory limit: {} bytes", isolation.resource_limits.max_memory);
    println!("CPU limit: {}%", isolation.resource_limits.max_cpu);
    println!("File descriptors: {}", isolation.resource_limits.max_file_descriptors);
    
    println!("\n✅ Simple process demo completed successfully!");
    
    Ok(())
}

// Utility functions for process management
fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn which(command: &str) -> Option<String> {
    Command::new("which")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()
        .ok()?
        .wait_with_output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
}

fn get_current_pid() -> u32 {
    std::process::id()
}

struct ProcessInfo {
    name: String,
}

fn get_process_info(pid: u32) -> Option<ProcessInfo> {
    // Simplified process info lookup
    #[cfg(unix)]
    {
        let output = Command::new("ps")
            .arg("-p")
            .arg(pid.to_string())
            .arg("-o")
            .arg("comm=")
            .stdout(Stdio::piped())
            .spawn()
            .ok()?
            .wait_with_output()
            .ok()?;
        
        if output.status.success() {
            let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !name.is_empty() {
                return Some(ProcessInfo { name });
            }
        }
    }
    
    // Fallback
    Some(ProcessInfo { name: "cursed_process".to_string() })
}

fn is_process_running(pid: u32) -> bool {
    #[cfg(unix)]
    {
        Command::new("ps")
            .arg("-p")
            .arg(pid.to_string())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
    
    #[cfg(windows)]
    {
        Command::new("tasklist")
            .arg("/FI")
            .arg(&format!("PID eq {}", pid))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
}

fn run_command(command: &str) -> Result<std::process::Output, Box<dyn std::error::Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    
    Ok(output)
}

fn run_command_timeout(command: &str, timeout: Duration) -> Result<std::process::Output, Box<dyn std::error::Error>> {
    use std::sync::mpsc;
    use std::thread;
    
    let (tx, rx) = mpsc::channel();
    let command = command.to_string();
    
    thread::spawn(move || {
        let result = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .and_then(|child| child.wait_with_output());
        
        let _ = tx.send(result);
    });
    
    match rx.recv_timeout(timeout) {
        Ok(result) => result.map_err(|e| e.into()),
        Err(_) => Err("Command timed out".into()),
    }
}
