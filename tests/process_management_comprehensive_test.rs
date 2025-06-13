/// Comprehensive tests for CURSED process management module
/// 
/// This test suite validates the complete process management functionality
/// including spawning, lifecycle management, environment handling, and IPC.

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use std::env;

use cursed::stdlib::process::{
    ProcessError, ProcessResult, ProcessConfig, ProcessIo, Process,
    ProcessLifecycleManager, ProcessLifecycleState,
    EnvironmentManager, EnvironmentUtils,
    ProcessInfo, ProcessStatus, Signal, Priority,
    spawn_process, run_command, run_command_timeout, which, command_exists,
    get_current_pid, is_process_running, ProcessControl,
};

fn init_tracing() {
    tracing_setup::init_test_tracing();
}

// Basic Process Operations Tests

#[test]
fn test_process_basic_spawning() {
    init_tracing();
    tracing::info!("Testing basic process spawning");
    
    let config = ProcessConfig::new("echo")
        .args(&["Hello", "World"])
        .stdout(ProcessIo::Capture);
    
    let mut process = Process::spawn(config).unwrap();
    let output = process.wait_with_output().unwrap();
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Hello World"));
}

#[test]
fn test_process_with_timeout() {
    init_tracing();
    tracing::info!("Testing process with timeout");
    
    // Use a command that will definitely timeout
    #[cfg(unix)]
    let config = ProcessConfig::new("sleep")
        .args(&["5"])
        .timeout(Duration::from_millis(100));
    
    #[cfg(windows)]
    let config = ProcessConfig::new("timeout")
        .args(&["5"])
        .timeout(Duration::from_millis(100));
    
    let result = run_command_timeout(config, Duration::from_millis(200));
    assert!(result.is_err());
}

#[test]
fn test_process_environment_variables() {
    init_tracing();
    tracing::info!("Testing process environment variables");
    
    let mut env_vars = HashMap::new();
    env_vars.insert("TEST_VAR".to_string(), "test_value".to_string());
    
    #[cfg(unix)]
    let config = ProcessConfig::new("env")
        .env_vars(env_vars)
        .stdout(ProcessIo::Capture);
    
    #[cfg(windows)]
    let config = ProcessConfig::new("cmd")
        .args(&["/C", "set"])
        .env_vars(env_vars)
        .stdout(ProcessIo::Capture);
    
    let output = run_command(config).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("TEST_VAR"));
    assert!(stdout.contains("test_value"));
}

#[test]
fn test_process_working_directory() {
    init_tracing();
    tracing::info!("Testing process working directory");
    
    let temp_dir = env::temp_dir();
    
    #[cfg(unix)]
    let config = ProcessConfig::new("pwd")
        .working_dir(temp_dir.clone())
        .stdout(ProcessIo::Capture);
    
    #[cfg(windows)]
    let config = ProcessConfig::new("cmd")
        .args(&["/C", "cd"])
        .working_dir(temp_dir.clone())
        .stdout(ProcessIo::Capture);
    
    let output = run_command(config).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Normalize paths for comparison
    let expected_path = temp_dir.to_string_lossy();
    assert!(stdout.trim().contains(&expected_path) || 
            stdout.trim().ends_with(&expected_path));
}

#[test]
fn test_process_stdin_input() {
    init_tracing();
    tracing::info!("Testing process stdin input");
    
    #[cfg(unix)]
    let config = ProcessConfig::new("cat")
        .stdin(ProcessIo::Pipe)
        .stdout(ProcessIo::Capture);
    
    #[cfg(windows)]
    let config = ProcessConfig::new("findstr")
        .args(&[".*"])
        .stdin(ProcessIo::Pipe)
        .stdout(ProcessIo::Capture);
    
    let mut process = Process::spawn(config).unwrap();
    
    // Write to stdin
    let stdin = process.stdin().unwrap();
    stdin.write_all(b"test input\n").unwrap();
    drop(stdin); // Close stdin
    
    let output = process.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("test input"));
}

// Environment Manager Tests

#[test]
fn test_environment_manager_basic_operations() {
    init_tracing();
    tracing::info!("Testing environment manager basic operations");
    
    let mut env = EnvironmentManager::new();
    
    // Test setting and getting
    env.set("TEST_VAR", "test_value");
    assert_eq!(env.get("TEST_VAR").unwrap().to_string_lossy(), "test_value");
    
    // Test contains
    assert!(env.contains_key("TEST_VAR"));
    assert!(!env.contains_key("NONEXISTENT"));
    
    // Test removal
    env.remove("TEST_VAR");
    assert!(!env.contains_key("TEST_VAR"));
}

#[test]
fn test_environment_manager_inheritance() {
    init_tracing();
    tracing::info!("Testing environment manager inheritance");
    
    // Set a test environment variable
    env::set_var("CURSED_TEST_INHERIT", "inherited_value");
    
    let env = EnvironmentManager::inherit_current().unwrap();
    assert!(env.contains_key("CURSED_TEST_INHERIT"));
    
    // Clean up
    env::remove_var("CURSED_TEST_INHERIT");
}

#[test]
fn test_environment_manager_path_operations() {
    init_tracing();
    tracing::info!("Testing environment manager PATH operations");
    
    let mut env = EnvironmentManager::new();
    
    // Test setting PATH
    let paths = vec!["/usr/bin", "/bin"];
    env.set_path(&paths).unwrap();
    
    let path_value = env.get("PATH").unwrap().to_string_lossy();
    let expected = if cfg!(windows) { "/usr/bin;/bin" } else { "/usr/bin:/bin" };
    assert_eq!(path_value, expected);
    
    // Test adding to PATH
    env.add_to_path("/usr/local/bin").unwrap();
    let path_value = env.get("PATH").unwrap().to_string_lossy();
    let expected = if cfg!(windows) { 
        "/usr/local/bin;/usr/bin;/bin" 
    } else { 
        "/usr/local/bin:/usr/bin:/bin" 
    };
    assert_eq!(path_value, expected);
}

#[test]
fn test_environment_variable_expansion() {
    init_tracing();
    tracing::info!("Testing environment variable expansion");
    
    env::set_var("CURSED_TEST_EXPAND", "expanded");
    
    // Test Unix format
    let result = EnvironmentUtils::expand_variables("$CURSED_TEST_EXPAND").unwrap();
    assert_eq!(result, "expanded");
    
    let result = EnvironmentUtils::expand_variables("${CURSED_TEST_EXPAND}").unwrap();
    assert_eq!(result, "expanded");
    
    let result = EnvironmentUtils::expand_variables("prefix_$CURSED_TEST_EXPAND_suffix").unwrap();
    assert_eq!(result, "prefix_expanded_suffix");
    
    env::remove_var("CURSED_TEST_EXPAND");
}

#[test]
fn test_environment_validation() {
    init_tracing();
    tracing::info!("Testing environment validation");
    
    // Test valid key
    assert!(EnvironmentManager::validate_key("VALID_KEY").is_ok());
    
    // Test invalid keys
    assert!(EnvironmentManager::validate_key("").is_err());
    assert!(EnvironmentManager::validate_key("KEY=VALUE").is_err());
    assert!(EnvironmentManager::validate_key("KEY\0NULL").is_err());
}

// Process Lifecycle Manager Tests

#[test]
fn test_lifecycle_manager_basic_operations() {
    init_tracing();
    tracing::info!("Testing lifecycle manager basic operations");
    
    let manager = ProcessLifecycleManager::new();
    assert_eq!(manager.active_process_count(), 0);
    
    // Spawn a simple process
    let config = ProcessConfig::new("echo")
        .args(&["test"])
        .timeout(Duration::from_secs(5));
    
    let pid = manager.spawn(config).unwrap();
    assert!(pid > 0);
    assert_eq!(manager.active_process_count(), 1);
    
    // Wait for process to complete
    let exit_status = manager.wait(pid).unwrap();
    assert!(exit_status.success());
}

#[test]
fn test_lifecycle_manager_process_timeout() {
    init_tracing();
    tracing::info!("Testing lifecycle manager process timeout");
    
    let manager = ProcessLifecycleManager::new();
    
    // Create a long-running command that should timeout
    #[cfg(unix)]
    let config = ProcessConfig::new("sleep")
        .args(&["10"])
        .timeout(Duration::from_millis(100));
    
    #[cfg(windows)]
    let config = ProcessConfig::new("timeout")
        .args(&["10"])
        .timeout(Duration::from_millis(100));
    
    let pid = manager.spawn(config).unwrap();
    
    // Wait with a short timeout
    let result = manager.wait_with_timeout(pid, Some(Duration::from_millis(200)));
    assert!(result.is_err());
}

#[test]
fn test_lifecycle_manager_process_termination() {
    init_tracing();
    tracing::info!("Testing lifecycle manager process termination");
    
    let manager = ProcessLifecycleManager::new();
    
    // Create a long-running command
    #[cfg(unix)]
    let config = ProcessConfig::new("sleep").args(&["60"]);
    
    #[cfg(windows)]
    let config = ProcessConfig::new("timeout").args(&["60"]);
    
    let pid = manager.spawn(config).unwrap();
    
    // Give the process time to start
    std::thread::sleep(Duration::from_millis(100));
    
    // Terminate the process
    assert!(manager.terminate(pid, Some(Duration::from_secs(1))).is_ok());
}

#[test]
fn test_lifecycle_manager_multiple_processes() {
    init_tracing();
    tracing::info!("Testing lifecycle manager with multiple processes");
    
    let manager = ProcessLifecycleManager::new();
    
    // Spawn multiple processes
    let config1 = ProcessConfig::new("echo").args(&["test1"]);
    let config2 = ProcessConfig::new("echo").args(&["test2"]);
    
    let pid1 = manager.spawn(config1).unwrap();
    let pid2 = manager.spawn(config2).unwrap();
    
    let active_pids = manager.list_active_processes();
    assert!(active_pids.contains(&pid1));
    assert!(active_pids.contains(&pid2));
    assert_eq!(manager.active_process_count(), 2);
    
    // Wait for both processes
    let _ = manager.wait(pid1);
    let _ = manager.wait(pid2);
}

#[test]
fn test_lifecycle_manager_concurrent_limit() {
    init_tracing();
    tracing::info!("Testing lifecycle manager concurrent limit");
    
    let manager = ProcessLifecycleManager::with_config(1, Duration::from_secs(60));
    
    // First process should succeed
    let config1 = ProcessConfig::new("echo").args(&["test1"]);
    let _pid1 = manager.spawn(config1).unwrap();
    
    // Second process should fail due to limit
    let config2 = ProcessConfig::new("echo").args(&["test2"]);
    let result = manager.spawn(config2);
    assert!(result.is_err());
}

// Utility Function Tests

#[test]
fn test_command_exists() {
    init_tracing();
    tracing::info!("Testing command_exists function");
    
    // Test with a command that should exist on all systems
    #[cfg(unix)]
    assert!(command_exists("echo").unwrap());
    
    #[cfg(windows)]
    assert!(command_exists("cmd").unwrap());
    
    // Test with a command that shouldn't exist
    assert!(!command_exists("this_command_definitely_does_not_exist").unwrap());
}

#[test]
fn test_which_command() {
    init_tracing();
    tracing::info!("Testing which function");
    
    #[cfg(unix)]
    {
        let echo_path = which("echo").unwrap();
        assert!(echo_path.is_some());
        let path = echo_path.unwrap();
        assert!(path.exists());
        assert!(path.is_file());
    }
    
    #[cfg(windows)]
    {
        let cmd_path = which("cmd").unwrap();
        assert!(cmd_path.is_some());
        let path = cmd_path.unwrap();
        assert!(path.exists());
        assert!(path.is_file());
    }
}

#[test]
fn test_current_process_info() {
    init_tracing();
    tracing::info!("Testing current process info");
    
    let current_pid = get_current_pid().unwrap();
    assert!(current_pid > 0);
    
    let is_running = is_process_running(current_pid).unwrap();
    assert!(is_running);
}

// Error Handling Tests

#[test]
fn test_process_error_handling() {
    init_tracing();
    tracing::info!("Testing process error handling");
    
    // Test with non-existent command
    let config = ProcessConfig::new("this_command_does_not_exist");
    let result = Process::spawn(config);
    assert!(result.is_err());
}

#[test]
fn test_invalid_working_directory() {
    init_tracing();
    tracing::info!("Testing invalid working directory");
    
    let config = ProcessConfig::new("echo")
        .args(&["test"])
        .working_dir("/this/directory/does/not/exist");
    
    let result = Process::spawn(config);
    assert!(result.is_err());
}

#[test]
fn test_environment_validation_errors() {
    init_tracing();
    tracing::info!("Testing environment validation errors");
    
    // Test empty key
    assert!(EnvironmentManager::validate_key("").is_err());
    
    // Test key with equals sign
    assert!(EnvironmentManager::validate_key("KEY=VALUE").is_err());
    
    // Test key with null character
    assert!(EnvironmentManager::validate_key("KEY\0NULL").is_err());
}

// Integration Tests

#[test]
fn test_process_with_custom_environment() {
    init_tracing();
    tracing::info!("Testing process with custom environment");
    
    let mut env_manager = EnvironmentManager::new();
    env_manager.set("CUSTOM_VAR", "custom_value");
    env_manager.set("PATH", "/usr/bin:/bin");
    
    let env_vars: HashMap<String, String> = env_manager.get_all()
        .into_iter()
        .map(|(k, v)| (k.to_string_lossy().to_string(), v.to_string_lossy().to_string()))
        .collect();
    
    #[cfg(unix)]
    let config = ProcessConfig::new("env")
        .env_vars(env_vars)
        .stdout(ProcessIo::Capture);
    
    #[cfg(windows)]
    let config = ProcessConfig::new("cmd")
        .args(&["/C", "set"])
        .env_vars(env_vars)
        .stdout(ProcessIo::Capture);
    
    let output = run_command(config).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("CUSTOM_VAR"));
    assert!(stdout.contains("custom_value"));
}

#[test]
fn test_lifecycle_manager_with_environment() {
    init_tracing();
    tracing::info!("Testing lifecycle manager with environment");
    
    let manager = ProcessLifecycleManager::new();
    
    let mut env_vars = HashMap::new();
    env_vars.insert("LIFECYCLE_TEST".to_string(), "test_value".to_string());
    
    let config = ProcessConfig::new("echo")
        .args(&["$LIFECYCLE_TEST"])
        .env_vars(env_vars)
        .timeout(Duration::from_secs(5));
    
    let pid = manager.spawn(config).unwrap();
    let exit_status = manager.wait(pid).unwrap();
    assert!(exit_status.success());
}

// Performance and Stress Tests

#[test]
#[ignore] // This is a longer-running test
fn test_many_short_processes() {
    init_tracing();
    tracing::info!("Testing many short processes");
    
    let manager = ProcessLifecycleManager::with_config(10, Duration::from_secs(30));
    
    let mut pids = Vec::new();
    
    // Spawn many short processes
    for i in 0..20 {
        let config = ProcessConfig::new("echo")
            .args(&[&format!("process_{}", i)])
            .timeout(Duration::from_secs(5));
        
        match manager.spawn(config) {
            Ok(pid) => pids.push(pid),
            Err(_) => {
                // Some may fail due to concurrent limits, which is expected
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }
    
    // Wait for all processes to complete
    for pid in pids {
        let _ = manager.wait_with_timeout(pid, Some(Duration::from_secs(10)));
    }
    
    tracing::info!("Completed many short processes test");
}

#[test]
fn test_path_utilities() {
    init_tracing();
    tracing::info!("Testing path utilities");
    
    let separator = EnvironmentUtils::path_separator();
    assert!(!separator.is_empty());
    
    let path_value = format!("/usr/bin{}/bin{}/usr/local/bin", separator, separator);
    let paths = EnvironmentUtils::parse_path(&path_value);
    assert_eq!(paths.len(), 3);
    assert_eq!(paths[0], PathBuf::from("/usr/bin"));
    assert_eq!(paths[1], PathBuf::from("/bin"));
    assert_eq!(paths[2], PathBuf::from("/usr/local/bin"));
}

#[test]
fn test_environment_merge() {
    init_tracing();
    tracing::info!("Testing environment merge");
    
    let mut env1 = EnvironmentManager::new();
    env1.set("VAR1", "value1");
    env1.set("COMMON", "env1_value");
    
    let mut env2 = EnvironmentManager::new();
    env2.set("VAR2", "value2");
    env2.set("COMMON", "env2_value");
    env2.remove("VAR1");
    
    env1.merge(&env2);
    
    assert_eq!(env1.get("VAR2").unwrap().to_string_lossy(), "value2");
    assert_eq!(env1.get("COMMON").unwrap().to_string_lossy(), "env2_value");
    assert!(!env1.contains_key("VAR1")); // Should be removed
}

#[test]
fn test_process_statistics() {
    init_tracing();
    tracing::info!("Testing process statistics");
    
    let manager = ProcessLifecycleManager::new();
    let stats = manager.get_statistics();
    
    assert_eq!(stats.active_count, 0);
    assert!(stats.max_concurrent_reached >= 0);
}
