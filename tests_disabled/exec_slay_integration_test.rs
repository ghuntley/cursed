//! Integration tests for the ExecSlay module
//! 
//! Comprehensive testing of process execution functionality including:
//! - Command execution and output capture
//! - Pipeline execution
//! - Background task management
//! - Shell command execution
//! - Process monitoring
//! - Builder pattern usage

#[path = "common.rs"]
mod common;

use cursed::stdlib::exec_slay::*;
use cursed::error::CursedError;
use std::time::Duration;
use std::collections::HashMap;

// Initialize test tracing
init_tracing!();

#[test]
fn test_basic_command_execution() {
    common::tracing::setup();
    tracing::info!("Testing basic command execution");
    
    #[cfg(unix)]
    {
        let mut cmd = SlayCommand::new("echo", &["hello", "world"]);
        let result = cmd.run();
        assert!(result.is_ok(), "Command execution should succeed");
        
        let output = cmd.output().unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("hello"), "Output should contain 'hello'");
        assert!(output_str.contains("world"), "Output should contain 'world'");
    }
    
    #[cfg(windows)]
    {
        let mut cmd = SlayCommand::new("echo", &["hello", "world"]);
        let result = cmd.run();
        assert!(result.is_ok(), "Command execution should succeed");
    }
}

#[test]
fn test_command_with_options() {
    common::tracing::setup();
    tracing::info!("Testing command with options");
    
    let mut options = SlayOptions::default();
    options.timeout = Some(Duration::from_secs(5));
    options.collect_output = true;
    
    #[cfg(unix)]
    {
        let mut cmd = SlayCommand::new("echo", &["test"])
            .with_options(options);
        
        let result = cmd.run();
        assert!(result.is_ok(), "Command with options should succeed");
        
        let output = cmd.output().unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("test"), "Output should contain 'test'");
    }
}

#[test]
fn test_command_timeout() {
    common::tracing::setup();
    tracing::info!("Testing command timeout");
    
    #[cfg(unix)]
    {
        let mut cmd = SlayCommand::new("sleep", &["2"]);
        cmd.set_timeout(Duration::from_millis(100));
        
        let result = cmd.run();
        assert!(result.is_err(), "Command should timeout");
        
        if let Err(CursedError::RuntimeError(msg)) = result {
            assert!(msg.contains("timeout"), "Error should mention timeout");
        }
    }
}

#[test]
fn test_command_builder() {
    common::tracing::setup();
    tracing::info!("Testing command builder");
    
    #[cfg(unix)]
    {
        let cmd = SlayCommandBuilder::new("echo")
            .with_args(&["hello", "builder"])
            .with_timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        
        assert_eq!(cmd.name, "echo");
        assert_eq!(cmd.args, vec!["hello", "builder"]);
        assert_eq!(cmd.options.timeout, Some(Duration::from_secs(5)));
    }
}

#[test]
fn test_command_builder_validation() {
    common::tracing::setup();
    tracing::info!("Testing command builder validation");
    
    // Empty command should fail
    let result = SlayCommandBuilder::new("").build();
    assert!(result.is_err(), "Empty command should fail validation");
    
    // Invalid CPU limit should fail
    let result = SlayCommandBuilder::new("test")
        .with_cpu_limit(150.0)
        .build();
    assert!(result.is_err(), "Invalid CPU limit should fail validation");
    
    // Zero buffer size should fail
    let result = SlayCommandBuilder::new("test")
        .with_buffer_size(0)
        .build();
    assert!(result.is_err(), "Zero buffer size should fail validation");
}

#[test]
fn test_convenience_builders() {
    common::tracing::setup();
    tracing::info!("Testing convenience builders");
    
    let echo_cmd = SlayCommandBuilder::echo("hello").build().unwrap();
    assert_eq!(echo_cmd.name, "echo");
    assert_eq!(echo_cmd.args, vec!["hello"]);
    
    let grep_cmd = SlayCommandBuilder::grep("pattern").build().unwrap();
    assert_eq!(grep_cmd.name, "grep");
    assert_eq!(grep_cmd.args, vec!["pattern"]);
    
    let ls_cmd = SlayCommandBuilder::ls().build().unwrap();
    assert_eq!(ls_cmd.name, "ls");
}

#[test]
fn test_shell_command_execution() {
    common::tracing::setup();
    tracing::info!("Testing shell command execution");
    
    #[cfg(unix)]
    {
        let result = run_shell("echo 'shell test'");
        assert!(result.is_ok(), "Shell command should succeed");
        
        let output = shell_output("echo 'hello shell'").unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("hello shell"), "Shell output should contain expected text");
    }
    
    #[cfg(windows)]
    {
        let result = run_shell("echo shell test");
        assert!(result.is_ok(), "Shell command should succeed");
    }
}

#[test]
fn test_shell_with_environment() {
    common::tracing::setup();
    tracing::info!("Testing shell with environment variables");
    
    let mut env = HashMap::new();
    env.insert("TEST_VAR".to_string(), "test_value".to_string());
    
    #[cfg(unix)]
    {
        let result = run_shell_with_env("echo $TEST_VAR", &env);
        assert!(result.is_ok(), "Shell command with env should succeed");
    }
    
    #[cfg(windows)]
    {
        let result = run_shell_with_env("echo %TEST_VAR%", &env);
        assert!(result.is_ok(), "Shell command with env should succeed");
    }
}

#[test]
fn test_shell_command_builder() {
    common::tracing::setup();
    tracing::info!("Testing shell command builder");
    
    #[cfg(unix)]
    {
        let output = ShellCommandBuilder::new("echo 'builder test'")
            .env("TEST", "value")
            .output()
            .unwrap();
        
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("builder test"), "Shell builder output should contain expected text");
    }
}

#[test]
fn test_pipeline_creation() {
    common::tracing::setup();
    tracing::info!("Testing pipeline creation");
    
    let cmd1 = SlayCommand::new("echo", &["hello"]);
    let cmd2 = SlayCommand::new("grep", &["h"]);
    
    let pipeline = SlayPipeline::new(vec![cmd1, cmd2]);
    assert_eq!(pipeline.commands.len(), 2);
    
    let pipeline_str = pipeline.to_string();
    assert_eq!(pipeline_str, "echo hello | grep h");
}

#[test]
fn test_single_command_pipeline() {
    common::tracing::setup();
    tracing::info!("Testing single command pipeline");
    
    #[cfg(unix)]
    {
        let cmd = SlayCommand::new("echo", &["pipeline test"]);
        let mut pipeline = SlayPipeline::new(vec![cmd]);
        
        let output = pipeline.output().unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("pipeline test"), "Pipeline output should contain expected text");
    }
}

#[test]
fn test_background_task() {
    common::tracing::setup();
    tracing::info!("Testing background task execution");
    
    #[cfg(unix)]
    {
        let cmd = SlayCommand::new("echo", &["background test"]);
        let mut task = SlayTask::run_background(cmd);
        
        // Wait for completion
        let result = task.wait();
        assert!(result.is_ok(), "Background task should complete successfully");
        assert!(task.is_finished(), "Task should be marked as finished");
        
        let output = task.get_output().unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("background test"), "Background task output should contain expected text");
    }
}

#[test]
fn test_background_task_timeout() {
    common::tracing::setup();
    tracing::info!("Testing background task with timeout");
    
    #[cfg(unix)]
    {
        let cmd = SlayCommand::new("sleep", &["0.1"]);
        let mut task = SlayTask::run_background(cmd);
        
        // Should complete within timeout
        let completed = task.wait_timeout(Duration::from_secs(1)).unwrap();
        assert!(completed, "Task should complete within timeout");
    }
}

#[test]
fn test_task_manager() {
    common::tracing::setup();
    tracing::info!("Testing task manager");
    
    let mut manager = SlayTaskManager::new();
    assert_eq!(manager.total_count(), 0);
    assert_eq!(manager.running_count(), 0);
    
    #[cfg(unix)]
    {
        let cmd = SlayCommand::new("echo", &["manager test"]);
        let task_id = manager.run_background(cmd);
        
        assert_eq!(task_id, 0);
        assert_eq!(manager.total_count(), 1);
        
        // Wait for task completion
        if let Some(task) = manager.get_task(task_id) {
            let _ = task.wait();
        }
    }
}

#[test]
fn test_process_monitoring_config() {
    common::tracing::setup();
    tracing::info!("Testing process monitoring configuration");
    
    let config = MonitorConfig {
        interval: Duration::from_millis(500),
        cpu_threshold: Some(80.0),
        memory_threshold: Some(100 * 1024 * 1024), // 100 MB
        max_duration: Some(Duration::from_secs(30)),
        detailed_monitoring: true,
        log_stats: false,
    };
    
    assert_eq!(config.interval, Duration::from_millis(500));
    assert_eq!(config.cpu_threshold, Some(80.0));
    assert!(config.detailed_monitoring);
}

#[test]
fn test_resource_limiter() {
    common::tracing::setup();
    tracing::info!("Testing resource limiter");
    
    let limiter = ResourceLimiter::new()
        .with_cpu_limit(50.0).unwrap()
        .with_memory_limit(100); // 100 MB
    
    let stats = ProcessStats {
        cpu: 75.0, // Exceeds limit
        memory: 200 * 1024 * 1024, // 200 MB, exceeds limit
        ..Default::default()
    };
    
    let violations = limiter.check_limits(&stats);
    assert_eq!(violations.len(), 2, "Should detect both CPU and memory violations");
    
    assert_eq!(violations[0].limit_type, LimitType::Cpu);
    assert_eq!(violations[1].limit_type, LimitType::Memory);
}

#[test]
fn test_limit_violation_display() {
    common::tracing::setup();
    tracing::info!("Testing limit violation display");
    
    let cpu_violation = LimitViolation {
        limit_type: LimitType::Cpu,
        current_value: 85.5,
        limit_value: 50.0,
    };
    
    let display_str = cpu_violation.to_string();
    assert!(display_str.contains("CPU limit violation"));
    assert!(display_str.contains("85.5%"));
    assert!(display_str.contains("50.0%"));
}

#[test]
fn test_shell_utilities() {
    common::tracing::setup();
    tracing::info!("Testing shell utilities");
    
    #[cfg(unix)]
    {
        // Test command existence
        assert!(shell::utils::command_exists("ls"), "ls command should exist");
        assert!(!shell::utils::command_exists("nonexistent_command_12345"), "Nonexistent command should not exist");
        
        // Test pwd
        let pwd = shell::utils::pwd().unwrap();
        assert!(!pwd.is_empty(), "PWD should not be empty");
        assert!(pwd.starts_with('/'), "PWD should start with /");
    }
    
    #[cfg(windows)]
    {
        assert!(shell::utils::command_exists("dir"), "dir command should exist");
    }
}

#[test]
fn test_error_handling() {
    common::tracing::setup();
    tracing::info!("Testing error handling");
    
    // Test nonexistent command
    let mut cmd = SlayCommand::new("nonexistent_command_12345", &[]);
    let result = cmd.run();
    assert!(result.is_err(), "Nonexistent command should fail");
    
    // Test empty pipeline
    let mut pipeline = SlayPipeline::new(vec![]);
    let result = pipeline.start();
    assert!(result.is_err(), "Empty pipeline should fail");
    
    // Test invalid shell command
    let result = run_shell("");
    assert!(result.is_err(), "Empty shell command should fail");
}

#[test]
fn test_string_representations() {
    common::tracing::setup();
    tracing::info!("Testing string representations");
    
    let cmd = SlayCommand::new("ls", &["-la", "my file"]);
    let cmd_str = cmd.to_string();
    assert_eq!(cmd_str, "ls -la \"my file\"", "Command string should properly quote arguments with spaces");
    
    let status = TaskStatus::Running;
    assert_eq!(status.to_string(), "Running");
    
    let status = TaskStatus::Failed("error message".to_string());
    assert_eq!(status.to_string(), "Failed: error message");
}

#[test]
fn test_process_stats_default() {
    common::tracing::setup();
    tracing::info!("Testing process stats default values");
    
    let stats = ProcessStats::default();
    assert_eq!(stats.cpu, 0.0);
    assert_eq!(stats.memory, 0);
    assert_eq!(stats.thread_count, 0);
    assert_eq!(stats.open_files, 0);
}

#[test]
fn test_signal_options() {
    common::tracing::setup();
    tracing::info!("Testing signal options");
    
    let opts = SignalOptions {
        grace_period: Duration::from_secs(10),
        force: true,
        signal: 9, // SIGKILL
        recursive: true,
    };
    
    assert_eq!(opts.grace_period, Duration::from_secs(10));
    assert!(opts.force);
    assert_eq!(opts.signal, 9);
    assert!(opts.recursive);
}

#[test]
fn test_comprehensive_command_flow() {
    common::tracing::setup();
    tracing::info!("Testing comprehensive command execution flow");
    
    #[cfg(unix)]
    {
        // Build a complex command using the builder
        let mut cmd = SlayCommandBuilder::new("echo")
            .with_args(&["comprehensive", "test"])
            .with_timeout(Duration::from_secs(5))
            .with_buffer_size(1024)
            .collect_output(true)
            .build()
            .unwrap();
        
        // Execute the command
        let result = cmd.run();
        assert!(result.is_ok(), "Comprehensive command should succeed");
        
        // Check output
        let output = cmd.output().unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("comprehensive"), "Output should contain 'comprehensive'");
        assert!(output_str.contains("test"), "Output should contain 'test'");
        
        // Check command properties
        assert!(!cmd.is_running(), "Command should not be running after completion");
        assert_eq!(cmd.exit_code(), Some(0), "Command should have exit code 0");
    }
}

#[test]
fn test_module_integration() {
    common::tracing::setup();
    tracing::info!("Testing module integration and public API");
    
    // Test that all main types are accessible
    let _options = SlayOptions::default();
    let _signal_opts = SignalOptions::default();
    let _stats = ProcessStats::default();
    
    // Test that convenience functions work
    let _builder = slay_command("test");
    
    // Test result types
    let _result: SlayResult<()> = Ok(());
}
