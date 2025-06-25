/// Simple Process Management Demo for CURSED
/// Demonstrates basic process management functionality using Rust

use cursed::stdlib::process::*;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 CURSED Simple Process Management Demo");
    println!("========================================");
    
    // Demo 1: Basic process spawning
    println!("\n1. Basic Process Spawning:");
    let config = ProcessConfig::new("echo")
        .arg("Hello from CURSED process!")
        .capture_output();
    
    let mut process = spawn_process(config)?;
    let output = process.wait_for_output()?;
    
    println!("Process output: {}", String::from_utf8_lossy(&output.stdout));
    println!("Exit code: {}", output.status.code().unwrap_or(-1));
    
    // Demo 2: Environment variables
    println!("\n2. Environment Variables:");
    #[cfg(unix)]
    let env_config = ProcessConfig::new("env")
        .env("CURSED_DEMO", "environment_test")
        .capture_output();
    
    #[cfg(windows)]
    let env_config = ProcessConfig::new("set")
        .env("CURSED_DEMO", "environment_test")
        .capture_output();
    
    let mut env_process = spawn_process(env_config)?;
    let env_output = env_process.wait_for_output()?;
    
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
    let wd_config = ProcessConfig::new("pwd")
        .working_dir(&temp_dir)
        .capture_output();
    
    #[cfg(windows)]
    let wd_config = ProcessConfig::new("cd")
        .working_dir(&temp_dir)
        .capture_output();
    
    let mut wd_process = spawn_process(wd_config)?;
    let wd_output = wd_process.wait_for_output()?;
    
    println!("Working directory output: {}", String::from_utf8_lossy(&wd_output.stdout));
    
    // Demo 4: Command utilities
    println!("\n4. Command Utilities:");
    println!("Echo command exists: {}", command_exists("echo"));
    println!("Nonexistent command exists: {}", command_exists("nonexistent_xyz"));
    
    if let Ok(echo_path) = which("echo") {
        println!("Echo command path: {}", echo_path.display());
    }
    
    // Demo 5: Process information
    println!("\n5. Process Information:");
    let current_pid = get_current_pid();
    println!("Current PID: {}", current_pid);
    
    if let Ok(process_info) = get_process_info(current_pid) {
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
            Ok(output) => println!("✓ Short command completed within timeout"),
            Err(e) => println!("✗ Short command failed: {}", e),
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
    
    println!("\n✅ Simple process demo completed successfully!");
    
    Ok(())
}
