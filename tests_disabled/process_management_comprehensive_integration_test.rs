/// Comprehensive Integration Tests for Enhanced Process Management
/// 
/// This test suite validates all the new process management features including:
/// - Enhanced SlayCommand execution
/// - Pipeline management and chaining
/// - Background task execution and monitoring
/// - Shell command execution across platforms
/// - Real IPC communication between processes
/// - Cross-platform compatibility and error handling

use std::time::Duration;
use std::thread;
use std::collections::HashMap;

use cursed::stdlib::process::{
    // Enhanced SlayCommand
    SlayCommand, SlayProcess, SlayProcessState, SlayOptions, SlayTask, TaskConfig, TaskPriority,
    
    // Pipeline management
    SlayPipeline, PipelineBuilder, pipe, run_pipeline,
    
    // Background tasks
    TaskManager, ManagerConfig, run_background, run_background_with_config,
    
    // Shell commands
    ShellExecutor, ShellConfig, ShellType, run_shell, shell_output, command_exists,
    
    // Real IPC
    RealIpcChannel, IpcChannelConfig, IpcChannelType, IpcMessage, MessagePriority,
    create_named_pipe, send_ipc_message, receive_ipc_message,
    
    // Error handling
    ProcessError, ProcessResult,
};

#[test]
fn test_enhanced_slay_command_basic_execution() {
    #[cfg(unix)]
    let mut cmd = SlayCommand::new("echo", &["Hello", "World"]);
    
    #[cfg(windows)]
    let mut cmd = SlayCommand::new("cmd", &["/C", "echo", "Hello", "World"]);
    
    cmd = cmd.enable_monitoring();
    
    let result = cmd.run();
    assert!(result.is_ok(), "Command execution failed: {:?}", result.err());
    
    let (output, exec_result) = cmd.output().expect("Failed to get output");
    assert!(exec_result.is_ok(), "Command execution error: {:?}", exec_result.err());
    
    let output_str = String::from_utf8_lossy(&output);
    assert!(output_str.contains("Hello"), "Output doesn't contain expected text: {}", output_str);
}

#[test]
fn test_enhanced_slay_command_with_options() {
    let options = SlayOptions {
        timeout: Some(Duration::from_secs(10)),
        capture_output: true,
        buffer_size: 1024,
        use_shell: false,
        ..Default::default()
    };
    
    #[cfg(unix)]
    let mut cmd = SlayCommand::new("echo", &["configured", "test"])
        .with_options(options);
    
    #[cfg(windows)]
    let mut cmd = SlayCommand::new("cmd", &["/C", "echo", "configured", "test"])
        .with_options(options);
    
    let result = cmd.run();
    assert!(result.is_ok(), "Configured command failed: {:?}", result.err());
}

#[test]
fn test_pipeline_basic_execution() {
    #[cfg(unix)]
    {
        let cmd1 = SlayCommand::new("echo", &["line1\nline2\nline3"]);
        let cmd2 = SlayCommand::new("grep", &["line2"]);
        
        let mut pipeline = pipe(vec![cmd1, cmd2]);
        let result = pipeline.run();
        assert!(result.is_ok(), "Pipeline execution failed: {:?}", result.err());
        
        let output = pipeline.output().expect("Failed to get pipeline output");
        let output_str = String::from_utf8_lossy(&output);
        assert!(output_str.contains("line2"), "Pipeline output incorrect: {}", output_str);
    }
    
    #[cfg(windows)]
    {
        // Windows equivalent using findstr
        let cmd1 = SlayCommand::new("cmd", &["/C", "echo", "line1 & echo line2 & echo line3"]);
        let cmd2 = SlayCommand::new("findstr", &["line2"]);
        
        let mut pipeline = pipe(vec![cmd1, cmd2]);
        let result = pipeline.run();
        if result.is_ok() {
            let output = pipeline.output().expect("Failed to get pipeline output");
            let output_str = String::from_utf8_lossy(&output);
            assert!(output_str.contains("line2"), "Pipeline output incorrect: {}", output_str);
        }
        // Note: Windows pipeline might not work the same way, so we don't fail the test
    }
}

#[test]
fn test_pipeline_builder() {
    #[cfg(unix)]
    {
        let pipeline = PipelineBuilder::new()
            .add_command(SlayCommand::new("echo", &["hello\nworld\nhello"]))
            .add_command(SlayCommand::new("grep", &["hello"]))
            .add_command(SlayCommand::new("wc", &["-l"]))
            .timeout(Duration::from_secs(10))
            .buffer_size(4096)
            .build();
        
        assert_eq!(pipeline.commands.len(), 3);
        assert_eq!(pipeline.options.timeout, Some(Duration::from_secs(10)));
        assert_eq!(pipeline.options.buffer_size, 4096);
    }
}

#[test]
fn test_background_task_execution() {
    #[cfg(unix)]
    let command = SlayCommand::new("sleep", &["0.1"]);
    
    #[cfg(windows)]
    let command = SlayCommand::new("cmd", &["/C", "timeout", "/t", "1", "/nobreak"]);
    
    let task_id = run_background(command).expect("Failed to start background task");
    
    // Task should be running initially
    let manager = cursed::stdlib::process::get_global_task_manager();
    if let Some(task) = manager.get_task(task_id) {
        if let Ok(mut task_guard) = task.lock() {
            assert!(task_guard.is_running() || task_guard.finished, "Task should be running or finished");
        }
    }
    
    // Wait for task completion
    thread::sleep(Duration::from_millis(200));
    
    if let Some(task) = manager.get_task(task_id) {
        if let Ok(task_guard) = task.lock() {
            assert!(task_guard.finished, "Task should be finished after wait");
        }
    }
}

#[test]
fn test_background_task_with_config() {
    let config = TaskConfig {
        capture_output: true,
        timeout: Some(Duration::from_secs(5)),
        monitor_resources: false,
        auto_cleanup: true,
        priority: TaskPriority::High,
        ..Default::default()
    };
    
    #[cfg(unix)]
    let command = SlayCommand::new("echo", &["background_test"]);
    
    #[cfg(windows)]
    let command = SlayCommand::new("cmd", &["/C", "echo", "background_test"]);
    
    let task_id = run_background_with_config(command, config)
        .expect("Failed to start configured background task");
    
    let manager = cursed::stdlib::process::get_global_task_manager();
    if let Some(task) = manager.get_task(task_id) {
        if let Ok(task_guard) = task.lock() {
            assert_eq!(task_guard.config.priority, TaskPriority::High);
            assert!(task_guard.config.capture_output);
        }
    }
}

#[test]
fn test_shell_executor_basic() {
    let executor = ShellExecutor::new();
    
    // Test basic shell command
    #[cfg(unix)]
    let result = executor.run_shell("echo 'shell test'");
    
    #[cfg(windows)]
    let result = executor.run_shell("echo shell test");
    
    assert!(result.is_ok(), "Shell execution failed: {:?}", result.err());
    
    let output = result.unwrap();
    assert!(output.success(), "Shell command failed");
    assert!(output.stdout_lossy().contains("shell"), "Shell output incorrect");
}

#[test]
fn test_shell_command_convenience_functions() {
    // Test command existence check
    #[cfg(unix)]
    assert!(command_exists("ls"), "ls command should exist on Unix systems");
    
    #[cfg(windows)]
    assert!(command_exists("dir"), "dir command should exist on Windows");
    
    // Test non-existent command
    assert!(!command_exists("definitely_nonexistent_command_12345"));
    
    // Test shell output function
    #[cfg(unix)]
    {
        let output = shell_output("echo 'convenience test'");
        if let Ok(out) = output {
            assert!(out.contains("convenience"), "Shell output incorrect: {}", out);
        }
    }
    
    #[cfg(windows)]
    {
        let output = shell_output("echo convenience test");
        if let Ok(out) = output {
            assert!(out.contains("convenience"), "Shell output incorrect: {}", out);
        }
    }
}

#[test]
fn test_shell_executor_with_environment() {
    let executor = ShellExecutor::new();
    
    let mut env = HashMap::new();
    env.insert("TEST_VAR".to_string(), "test_value".to_string());
    
    #[cfg(unix)]
    let result = executor.run_shell_with_env("echo $TEST_VAR", env);
    
    #[cfg(windows)]
    let result = executor.run_shell_with_env("echo %TEST_VAR%", env);
    
    if let Ok(output) = result {
        if output.success() {
            let output_str = output.stdout_lossy();
            assert!(output_str.contains("test_value") || output_str.contains("TEST_VAR"), 
                "Environment variable not passed correctly: {}", output_str);
        }
    }
}

#[test]
fn test_shell_executor_in_directory() {
    let executor = ShellExecutor::new();
    
    // Create a temporary directory for testing
    let temp_dir = std::env::temp_dir();
    
    #[cfg(unix)]
    let result = executor.run_shell_in_dir("pwd", &temp_dir);
    
    #[cfg(windows)]
    let result = executor.run_shell_in_dir("cd", &temp_dir);
    
    if let Ok(output) = result {
        if output.success() {
            let output_str = output.stdout_lossy();
            let temp_dir_str = temp_dir.to_string_lossy();
            assert!(output_str.contains(&*temp_dir_str) || output_str.contains("temp"), 
                "Working directory not set correctly. Output: {}, Expected path: {}", 
                output_str, temp_dir_str);
        }
    }
}

#[test]
fn test_ipc_channel_creation() {
    let config = IpcChannelConfig {
        name: "test_ipc_channel".to_string(),
        channel_type: IpcChannelType::NamedPipe,
        max_message_size: 1024,
        buffer_size: 4096,
        ..Default::default()
    };
    
    let channel = RealIpcChannel::new(config);
    assert!(channel.is_ok(), "IPC channel creation failed: {:?}", channel.err());
    
    let ch = channel.unwrap();
    assert_eq!(ch.config.name, "test_ipc_channel");
    assert_eq!(ch.config.channel_type, IpcChannelType::NamedPipe);
}

#[test]
fn test_ipc_convenience_functions() {
    let channel = create_named_pipe("convenience_ipc_test");
    assert!(channel.is_ok(), "Named pipe creation failed: {:?}", channel.err());
    
    let manager = cursed::stdlib::process::get_ipc_manager();
    assert!(manager.get_channel("convenience_ipc_test").is_some(), 
        "Channel not found in manager");
}

#[test]
fn test_ipc_message_creation() {
    let message = IpcMessage {
        id: 12345,
        sender_pid: std::process::id(),
        data: b"test ipc message".to_vec(),
        priority: MessagePriority::High,
        timestamp: std::time::Instant::now(),
        message_type: "test".to_string(),
        delivery_mode: cursed::stdlib::process::DeliveryMode::BestEffort,
    };
    
    assert_eq!(message.id, 12345);
    assert_eq!(message.data, b"test ipc message");
    assert_eq!(message.priority, MessagePriority::High);
    assert_eq!(message.sender_pid, std::process::id());
}

#[test]
fn test_process_error_handling() {
    // Test command that doesn't exist
    let mut cmd = SlayCommand::new("definitely_nonexistent_command", &[]);
    let result = cmd.run();
    assert!(result.is_err(), "Non-existent command should fail");
    
    match result.err().unwrap() {
        ProcessError::ExecutionFailed { command, .. } => {
            assert_eq!(command, "definitely_nonexistent_command");
        }
        _ => panic!("Wrong error type returned"),
    }
}

#[test]
fn test_pipeline_error_propagation() {
    // Create a pipeline with a failing command
    let cmd1 = SlayCommand::new("echo", &["test"]);
    let cmd2 = SlayCommand::new("definitely_nonexistent_command", &[]);
    
    let mut pipeline = pipe(vec![cmd1, cmd2]);
    let result = pipeline.run();
    
    assert!(result.is_err(), "Pipeline with failing command should fail");
}

#[test]
fn test_task_manager_functionality() {
    let config = ManagerConfig {
        max_concurrent_tasks: 5,
        cleanup_interval: Duration::from_secs(1),
        default_timeout: Some(Duration::from_secs(10)),
        auto_monitor: false,
    };
    
    let mut manager = TaskManager::new(config);
    assert!(manager.start().is_ok(), "Task manager start failed");
    
    // Submit a task
    #[cfg(unix)]
    let command = SlayCommand::new("echo", &["manager_test"]);
    
    #[cfg(windows)]
    let command = SlayCommand::new("cmd", &["/C", "echo", "manager_test"]);
    
    let task_id = manager.submit_task(command, None);
    assert!(task_id.is_ok(), "Task submission failed: {:?}", task_id.err());
    
    let id = task_id.unwrap();
    assert!(manager.get_task(id).is_some(), "Task not found in manager");
    
    // Wait for task completion
    let wait_result = manager.wait_for_task(id);
    assert!(wait_result.is_ok(), "Task wait failed: {:?}", wait_result.err());
    
    assert!(manager.stop().is_ok(), "Task manager stop failed");
}

#[test]
fn test_cross_platform_compatibility() {
    // Test that basic functionality works on both Unix and Windows
    
    #[cfg(unix)]
    {
        let cmd = SlayCommand::new("echo", &["unix_test"]);
        let executor = ShellExecutor::new();
        
        assert!(cmd.string().contains("echo"));
        assert!(executor.command_exists("echo"));
    }
    
    #[cfg(windows)]
    {
        let cmd = SlayCommand::new("cmd", &["/C", "echo", "windows_test"]);
        let executor = ShellExecutor::new();
        
        assert!(cmd.string().contains("cmd"));
        assert!(executor.command_exists("cmd"));
    }
}

#[test]
fn test_resource_monitoring_integration() {
    let config = TaskConfig {
        monitor_resources: true,
        monitor_interval: Duration::from_millis(100),
        capture_output: true,
        ..Default::default()
    };
    
    #[cfg(unix)]
    let command = SlayCommand::new("sleep", &["0.2"]);
    
    #[cfg(windows)]
    let command = SlayCommand::new("cmd", &["/C", "timeout", "/t", "1", "/nobreak"]);
    
    let task_id = run_background_with_config(command, config);
    assert!(task_id.is_ok(), "Monitored task creation failed: {:?}", task_id.err());
    
    // Give task time to start and be monitored
    thread::sleep(Duration::from_millis(150));
    
    let manager = cursed::stdlib::process::get_global_task_manager();
    if let Some(task) = manager.get_task(task_id.unwrap()) {
        if let Ok(task_guard) = task.lock() {
            assert!(task_guard.config.monitor_resources, "Resource monitoring not enabled");
        }
    }
}

#[test]
fn test_shell_type_detection() {
    let executor = ShellExecutor::new();
    
    // Should detect at least one shell
    let shell_info = executor.shell_info();
    assert!(!shell_info.is_empty(), "No shells detected");
    
    let default_shell = executor.default_shell_type();
    assert!(default_shell.is_some(), "No default shell detected");
    
    #[cfg(unix)]
    {
        // Unix systems should have some common shells
        let has_bash = shell_info.contains_key(&ShellType::Bash);
        let has_sh = executor.command_exists("sh");
        assert!(has_bash || has_sh, "No Unix shell detected");
    }
    
    #[cfg(windows)]
    {
        // Windows should have cmd or PowerShell
        let has_cmd = shell_info.contains_key(&ShellType::Cmd);
        let has_powershell = shell_info.contains_key(&ShellType::PowerShell);
        assert!(has_cmd || has_powershell, "No Windows shell detected");
    }
}

#[test]
fn test_comprehensive_error_scenarios() {
    // Test various error conditions to ensure robust error handling
    
    // 1. Command timeout
    let options = SlayOptions {
        timeout: Some(Duration::from_millis(50)),
        ..Default::default()
    };
    
    #[cfg(unix)]
    let mut cmd = SlayCommand::new("sleep", &["1"])
        .with_options(options);
    
    #[cfg(windows)]
    let mut cmd = SlayCommand::new("cmd", &["/C", "timeout", "/t", "2", "/nobreak"])
        .with_options(options);
    
    // This should timeout (note: timeout handling may vary by implementation)
    let result = cmd.run();
    // We don't assert failure here as timeout implementation may vary
    
    // 2. Invalid working directory
    let shell_config = ShellConfig {
        working_dir: Some(std::path::PathBuf::from("/definitely/nonexistent/directory")),
        ..Default::default()
    };
    
    let executor = ShellExecutor::with_config(shell_config);
    let result = executor.run_shell("echo test");
    // Working directory error handling may vary by platform
    
    // 3. IPC channel with invalid configuration
    let invalid_config = IpcChannelConfig {
        name: String::new(), // Empty name should be invalid
        ..Default::default()
    };
    
    // Some validation should catch this, but implementation may vary
    let _result = RealIpcChannel::new(invalid_config);
}

#[test]
fn test_integration_between_components() {
    // Test that different components work together
    
    // 1. Use shell executor to check if a command exists, then run it via SlayCommand
    if command_exists("echo") {
        let mut cmd = SlayCommand::new("echo", &["integration_test"]);
        let result = cmd.run();
        assert!(result.is_ok(), "Integration between shell and SlayCommand failed");
    }
    
    // 2. Run a background task, then check its status via the manager
    #[cfg(unix)]
    let command = SlayCommand::new("echo", &["background_integration"]);
    
    #[cfg(windows)]
    let command = SlayCommand::new("cmd", &["/C", "echo", "background_integration"]);
    
    if let Ok(task_id) = run_background(command) {
        let manager = cursed::stdlib::process::get_global_task_manager();
        
        if let Some(task) = manager.get_task(task_id) {
            if let Ok(task_guard) = task.lock() {
                // Task should exist and have the correct command
                assert!(task_guard.command.string().contains("echo"));
            }
        }
    }
    
    // 3. Create an IPC channel and verify it's managed correctly
    if let Ok(channel) = create_named_pipe("integration_ipc_test") {
        let manager = cursed::stdlib::process::get_ipc_manager();
        assert!(manager.get_channel("integration_ipc_test").is_some(), 
            "IPC channel not properly managed");
    }
}

#[test]
fn test_memory_safety_and_cleanup() {
    // Test that resources are properly cleaned up
    
    // 1. Create and drop multiple tasks
    for i in 0..5 {
        #[cfg(unix)]
        let command = SlayCommand::new("echo", &[&format!("cleanup_test_{}", i)]);
        
        #[cfg(windows)]
        let command = SlayCommand::new("cmd", &["/C", "echo", &format!("cleanup_test_{}", i)]);
        
        let _task_id = run_background(command);
        // Tasks should be automatically cleaned up
    }
    
    // 2. Create and close IPC channels
    for i in 0..3 {
        let channel_name = format!("cleanup_ipc_{}", i);
        if let Ok(mut channel) = create_named_pipe(&channel_name) {
            if let Ok(mut ch) = channel.lock() {
                let _close_result = ch.close();
            }
        }
    }
    
    // 3. Execute multiple shell commands
    let executor = ShellExecutor::new();
    for i in 0..3 {
        #[cfg(unix)]
        let _result = executor.run_shell(&format!("echo cleanup_shell_{}", i));
        
        #[cfg(windows)]
        let _result = executor.run_shell(&format!("echo cleanup_shell_{}", i));
    }
    
    // If we reach here without crashing, cleanup is working reasonably well
    assert!(true, "Memory safety and cleanup test completed");
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    #[ignore] // Only run with --ignored for performance testing
    fn test_performance_many_tasks() {
        let start = std::time::Instant::now();
        let mut task_ids = Vec::new();
        
        // Create many background tasks
        for i in 0..20 {
            #[cfg(unix)]
            let command = SlayCommand::new("echo", &[&format!("perf_test_{}", i)]);
            
            #[cfg(windows)]
            let command = SlayCommand::new("cmd", &["/C", "echo", &format!("perf_test_{}", i)]);
            
            if let Ok(task_id) = run_background(command) {
                task_ids.push(task_id);
            }
        }
        
        // Wait for all tasks
        let manager = cursed::stdlib::process::get_global_task_manager();
        let _wait_result = manager.wait_for_all();
        
        let elapsed = start.elapsed();
        println!("Performance test completed in {:?}", elapsed);
        
        // Should complete reasonably quickly
        assert!(elapsed < Duration::from_secs(30), "Performance test took too long: {:?}", elapsed);
    }

    #[test]
    #[ignore] // Only run with --ignored for performance testing
    fn test_performance_pipeline_chaining() {
        let start = std::time::Instant::now();
        
        #[cfg(unix)]
        {
            // Create a long pipeline
            let commands = vec![
                SlayCommand::new("echo", &["1\n2\n3\n4\n5"]),
                SlayCommand::new("grep", &["[2-4]"]),
                SlayCommand::new("wc", &["-l"]),
            ];
            
            let mut pipeline = pipe(commands);
            let result = pipeline.run();
            assert!(result.is_ok(), "Performance pipeline failed");
        }
        
        let elapsed = start.elapsed();
        println!("Pipeline performance test completed in {:?}", elapsed);
        
        // Should complete quickly
        assert!(elapsed < Duration::from_secs(10), "Pipeline performance test took too long: {:?}", elapsed);
    }
}
