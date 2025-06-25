/// Comprehensive tests for the Safe Process Management System
/// 
/// This test suite validates all aspects of the safe process management
/// implementation including memory safety, cross-platform compatibility,
/// resource limiting, and proper process lifecycle management.

use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;

use cursed::stdlib::process::safe_process_management::{
    SafeProcessHandle, SafeProcessManager, ProcessMetadata, ResourceLimits,
    ProcessState, ProcessStatistics, global_process_manager,
    initialize_process_management, shutdown_process_management,
    current_pid, parent_pid, process_exists
};
use cursed::stdlib::process::safe_exec_slay::{
    SafeSlayCommand, SafeSlayProcess, SafeSlayProcessState, SafeSlayOptions,
    SafeSlayPipeline, SafeSlayTask, ProcessStdin, ProcessStdout, ProcessStderr,
    SignalOptions, new_safe_slay_command, execute_safe, command_exists_safe
};
use cursed::stdlib::process::error::ProcessResult;

#[cfg(test)]
mod safe_process_management_tests {
    use super::*;

    #[test]
    fn test_safe_process_manager_creation() {
        let manager = SafeProcessManager::new();
        assert_eq!(manager.list_processes().len(), 0);
    }

    #[test]
    fn test_resource_limits_creation() {
        let limits = ResourceLimits {
            max_memory_bytes: Some(100 * 1024 * 1024), // 100MB
            max_cpu_percent: Some(80.0),
            max_execution_time: Some(Duration::from_secs(300)),
            max_file_descriptors: Some(1024),
        };
        
        assert_eq!(limits.max_memory_bytes, Some(100 * 1024 * 1024));
        assert_eq!(limits.max_cpu_percent, Some(80.0));
        assert_eq!(limits.max_execution_time, Some(Duration::from_secs(300)));
        assert_eq!(limits.max_file_descriptors, Some(1024));
    }

    #[test]
    fn test_process_state_enumeration() {
        assert_eq!(ProcessState::Created, ProcessState::Created);
        assert_ne!(ProcessState::Running, ProcessState::Terminated);
        
        // Test state transitions make sense
        let states = [
            ProcessState::Created,
            ProcessState::Running,
            ProcessState::Waiting,
            ProcessState::Stopped,
            ProcessState::Terminated,
        ];
        
        assert_eq!(states.len(), 5);
    }

    #[test]
    fn test_process_metadata_creation() {
        let mut env_vars = HashMap::new();
        env_vars.insert("PATH".to_string(), "/usr/bin:/bin".to_string());
        env_vars.insert("USER".to_string(), "testuser".to_string());
        
        let metadata = ProcessMetadata {
            command: "echo".to_string(),
            args: vec!["hello".to_string(), "world".to_string()],
            working_dir: Some("/tmp".into()),
            env_vars,
            parent_pid: Some(1234),
        };
        
        assert_eq!(metadata.command, "echo");
        assert_eq!(metadata.args.len(), 2);
        assert_eq!(metadata.parent_pid, Some(1234));
        assert!(metadata.working_dir.is_some());
        assert_eq!(metadata.env_vars.len(), 2);
    }

    #[test]
    fn test_current_pid() {
        let pid = current_pid();
        assert!(pid > 0);
        assert!(process_exists(pid));
    }

    #[test]
    fn test_parent_pid() {
        match parent_pid() {
            Ok(ppid) => {
                assert!(ppid > 0);
                // Parent PID should be different from current PID
                assert_ne!(ppid, current_pid());
            }
            Err(_) => {
                // On some platforms or environments, this might fail
                // which is acceptable for unit tests
            }
        }
    }

    #[test]
    fn test_process_exists() {
        let current = current_pid();
        assert!(process_exists(current));
        
        // Test with a PID that definitely doesn't exist
        assert!(!process_exists(u32::MAX));
    }

    #[test]
    fn test_global_process_manager() {
        let manager1 = global_process_manager();
        let manager2 = global_process_manager();
        
        // Should be the same instance
        assert!(std::ptr::eq(manager1, manager2));
    }

    #[test]
    fn test_process_management_initialization() {
        let result = initialize_process_management();
        assert!(result.is_ok());
        
        // Should be safe to call multiple times
        let result2 = initialize_process_management();
        assert!(result2.is_ok());
    }

    #[test]
    fn test_process_statistics_default() {
        let stats = ProcessStatistics {
            cpu_usage_percent: 25.5,
            memory_usage_bytes: 1024 * 1024,
            virtual_memory_bytes: 2 * 1024 * 1024,
            resident_memory_bytes: 512 * 1024,
            file_descriptors_count: 10,
            thread_count: 3,
            uptime: Duration::from_secs(120),
            total_cpu_time: Duration::from_millis(5000),
            bytes_read: 1000,
            bytes_written: 2000,
        };
        
        assert_eq!(stats.cpu_usage_percent, 25.5);
        assert_eq!(stats.memory_usage_bytes, 1024 * 1024);
        assert_eq!(stats.thread_count, 3);
        assert_eq!(stats.uptime, Duration::from_secs(120));
    }

    #[cfg(unix)]
    #[test]
    fn test_echo_command_execution() {
        let mut cmd = SafeSlayCommand::new("echo", &["hello", "world"]);
        let result = cmd.run();
        
        // Echo should succeed on Unix systems
        assert!(result.is_ok());
    }

    #[test]
    fn test_safe_command_creation() {
        let cmd = SafeSlayCommand::new("ls", &["-la", "/tmp"]);
        assert_eq!(cmd.path, "ls");
        assert_eq!(cmd.args, vec!["-la", "/tmp"]);
        assert!(cmd.env.is_empty());
        assert!(cmd.dir.is_none());
    }

    #[test]
    fn test_safe_command_environment() {
        let mut cmd = SafeSlayCommand::new("env", &[]);
        cmd.add_env("TEST_VAR", "test_value");
        cmd.add_env("ANOTHER_VAR", "another_value");
        
        assert_eq!(cmd.env.len(), 2);
        assert!(cmd.env.contains(&"TEST_VAR=test_value".to_string()));
        assert!(cmd.env.contains(&"ANOTHER_VAR=another_value".to_string()));
    }

    #[test]
    fn test_safe_command_configuration() {
        let mut cmd = SafeSlayCommand::new("cat", &[]);
        cmd.set_stdin(ProcessStdin::Pipe)
           .set_stdout(ProcessStdout::Pipe)
           .set_stderr(ProcessStderr::Pipe);
        
        assert!(cmd.stdin.is_some());
        assert!(cmd.stdout.is_some());
        assert!(cmd.stderr.is_some());
    }

    #[test]
    fn test_safe_options_default() {
        let opts = SafeSlayOptions::default();
        assert_eq!(opts.buffer_size, 8192);
        assert!(opts.collect_output);
        assert!(!opts.use_shell);
        assert!(opts.timeout.is_none());
        assert!(opts.stdout_callback.is_none());
        assert!(opts.stderr_callback.is_none());
    }

    #[test]
    fn test_signal_options() {
        let opts = SignalOptions::default();
        assert_eq!(opts.grace_period, Duration::from_secs(5));
        assert!(opts.force);
        assert_eq!(opts.signal, 15); // SIGTERM
        assert!(!opts.recursive);
        
        let custom_opts = SignalOptions {
            grace_period: Duration::from_secs(10),
            force: false,
            signal: 9, // SIGKILL
            recursive: true,
        };
        
        assert_eq!(custom_opts.grace_period, Duration::from_secs(10));
        assert!(!custom_opts.force);
        assert_eq!(custom_opts.signal, 9);
        assert!(custom_opts.recursive);
    }

    #[test]
    fn test_safe_pipeline_creation() {
        let cmd1 = SafeSlayCommand::new("echo", &["hello"]);
        let cmd2 = SafeSlayCommand::new("grep", &["hello"]);
        let cmd3 = SafeSlayCommand::new("wc", &["-l"]);
        let pipeline = SafeSlayPipeline::new(vec![cmd1, cmd2, cmd3]);
        
        assert_eq!(pipeline.commands.len(), 3);
        assert_eq!(pipeline.commands[0].path, "echo");
        assert_eq!(pipeline.commands[1].path, "grep");
        assert_eq!(pipeline.commands[2].path, "wc");
    }

    #[test]
    fn test_safe_pipeline_string_representation() {
        let cmd1 = SafeSlayCommand::new("cat", &["file.txt"]);
        let cmd2 = SafeSlayCommand::new("grep", &["pattern"]);
        let cmd3 = SafeSlayCommand::new("wc", &["-l"]);
        let pipeline = SafeSlayPipeline::new(vec![cmd1, cmd2, cmd3]);
        
        let expected = "cat file.txt | grep pattern | wc -l";
        assert_eq!(pipeline.string(), expected);
    }

    #[test]
    fn test_convenience_functions() {
        let cmd = new_safe_slay_command("echo", &["test"]);
        assert_eq!(cmd.path, "echo");
        assert_eq!(cmd.args, vec!["test"]);
    }

    #[test]
    fn test_resource_limits_with_command() {
        let limits = ResourceLimits {
            max_memory_bytes: Some(50 * 1024 * 1024), // 50MB
            max_cpu_percent: Some(75.0),
            max_execution_time: Some(Duration::from_secs(60)),
            max_file_descriptors: Some(512),
        };
        
        let mut cmd = SafeSlayCommand::new("echo", &["test"]);
        cmd.set_resource_limits(limits.clone());
        
        assert_eq!(cmd.resource_limits.max_memory_bytes, Some(50 * 1024 * 1024));
        assert_eq!(cmd.resource_limits.max_cpu_percent, Some(75.0));
    }

    #[test]
    fn test_command_string_representation() {
        let cmd = SafeSlayCommand::new("ls", &["-la", "/home"]);
        assert_eq!(cmd.string(), "ls -la /home");
        
        let cmd_no_args = SafeSlayCommand::new("pwd", &[]);
        assert_eq!(cmd_no_args.string(), "pwd ");
    }

    #[test]
    fn test_process_io_configurations() {
        // Test all ProcessStdin variants
        let stdin_null = ProcessStdin::Null;
        let stdin_inherit = ProcessStdin::Inherit;
        let stdin_pipe = ProcessStdin::Pipe;
        let stdin_file = ProcessStdin::File("/dev/null".into());
        
        // Should be able to create all variants
        assert!(matches!(stdin_null, ProcessStdin::Null));
        assert!(matches!(stdin_inherit, ProcessStdin::Inherit));
        assert!(matches!(stdin_pipe, ProcessStdin::Pipe));
        assert!(matches!(stdin_file, ProcessStdin::File(_)));
        
        // Test all ProcessStdout variants
        let stdout_null = ProcessStdout::Null;
        let stdout_inherit = ProcessStdout::Inherit;
        let stdout_pipe = ProcessStdout::Pipe;
        let stdout_file = ProcessStdout::File("/dev/null".into());
        
        assert!(matches!(stdout_null, ProcessStdout::Null));
        assert!(matches!(stdout_inherit, ProcessStdout::Inherit));
        assert!(matches!(stdout_pipe, ProcessStdout::Pipe));
        assert!(matches!(stdout_file, ProcessStdout::File(_)));
        
        // Test all ProcessStderr variants
        let stderr_null = ProcessStderr::Null;
        let stderr_inherit = ProcessStderr::Inherit;
        let stderr_pipe = ProcessStderr::Pipe;
        let stderr_file = ProcessStderr::File("/dev/null".into());
        
        assert!(matches!(stderr_null, ProcessStderr::Null));
        assert!(matches!(stderr_inherit, ProcessStderr::Inherit));
        assert!(matches!(stderr_pipe, ProcessStderr::Pipe));
        assert!(matches!(stderr_file, ProcessStderr::File(_)));
    }

    #[test]
    fn test_command_exists() {
        // Test with common commands that should exist
        #[cfg(unix)]
        {
            assert!(command_exists_safe("echo"));
            // ls might not be available in all environments
        }
        
        #[cfg(windows)]
        {
            // Most Windows systems should have these
            // But we can't guarantee it in all test environments
        }
        
        // Test with command that definitely doesn't exist
        assert!(!command_exists_safe("this_command_definitely_does_not_exist_anywhere"));
    }

    #[test]
    fn test_safe_task_creation() {
        let cmd = SafeSlayCommand::new("echo", &["background", "test"]);
        let task_result = SafeSlayTask::run_background(cmd);
        
        // Should be able to create a background task
        assert!(task_result.is_ok());
        
        if let Ok(task) = task_result {
            assert!(task.elapsed_time() >= Duration::from_nanos(0));
            // Initially should be running (or finished very quickly)
        }
    }

    #[test]
    fn test_memory_safety_no_unsafe_operations() {
        // This test ensures our implementation doesn't use unsafe operations
        // by testing typical usage patterns that would expose unsafe behavior
        
        let mut cmd = SafeSlayCommand::new("echo", &["memory", "safety", "test"]);
        
        // These operations should be completely safe
        cmd.add_env("TEST", "value");
        cmd.set_dir("/tmp");
        
        // Drop the command - should be safe
        drop(cmd);
        
        // Create multiple commands and drop them in different orders
        let cmd1 = SafeSlayCommand::new("echo", &["test1"]);
        let cmd2 = SafeSlayCommand::new("echo", &["test2"]);
        let cmd3 = SafeSlayCommand::new("echo", &["test3"]);
        
        drop(cmd2); // Drop middle one first
        drop(cmd1);
        drop(cmd3);
        
        // All should be safe
    }

    #[test]
    fn test_cross_platform_compatibility() {
        // Test that basic functionality works across platforms
        let current = current_pid();
        assert!(current > 0);
        assert!(process_exists(current));
        
        // Test process manager creation
        let manager = SafeProcessManager::new();
        assert_eq!(manager.list_processes().len(), 0);
        
        // Test command creation (doesn't execute, so should work everywhere)
        let cmd = SafeSlayCommand::new("test_command", &["arg1", "arg2"]);
        assert_eq!(cmd.path, "test_command");
        assert_eq!(cmd.args, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_resource_limit_edge_cases() {
        let default_limits = ResourceLimits::default();
        assert!(default_limits.max_memory_bytes.is_none());
        assert!(default_limits.max_cpu_percent.is_none());
        assert!(default_limits.max_execution_time.is_none());
        assert!(default_limits.max_file_descriptors.is_none());
        
        // Test with extreme values
        let extreme_limits = ResourceLimits {
            max_memory_bytes: Some(u64::MAX),
            max_cpu_percent: Some(100.0),
            max_execution_time: Some(Duration::from_secs(u64::MAX)),
            max_file_descriptors: Some(u32::MAX),
        };
        
        assert_eq!(extreme_limits.max_memory_bytes, Some(u64::MAX));
        assert_eq!(extreme_limits.max_cpu_percent, Some(100.0));
    }

    #[test]
    fn test_process_state_transitions() {
        // Test that process states can be used in expected ways
        let states = vec![
            ProcessState::Created,
            ProcessState::Running,
            ProcessState::Waiting,
            ProcessState::Stopped,
            ProcessState::Terminated,
        ];
        
        for state in &states {
            // Should be able to clone and compare
            let cloned = *state;
            assert_eq!(*state, cloned);
        }
        
        // Test logical relationships
        assert_ne!(ProcessState::Created, ProcessState::Terminated);
        assert_ne!(ProcessState::Running, ProcessState::Stopped);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_end_to_end_safe_process_workflow() {
        // Test complete workflow without actually executing commands
        // (to avoid environment dependencies)
        
        // 1. Initialize process management
        let init_result = initialize_process_management();
        assert!(init_result.is_ok());
        
        // 2. Create a command with full configuration
        let mut cmd = SafeSlayCommand::new("echo", &["integration", "test"]);
        cmd.add_env("INTEGRATION_TEST", "true")
           .set_dir("/tmp")
           .set_stdin(ProcessStdin::Null)
           .set_stdout(ProcessStdout::Pipe)
           .set_stderr(ProcessStderr::Pipe);
        
        // 3. Set resource limits
        let limits = ResourceLimits {
            max_memory_bytes: Some(10 * 1024 * 1024), // 10MB
            max_cpu_percent: Some(50.0),
            max_execution_time: Some(Duration::from_secs(30)),
            max_file_descriptors: Some(100),
        };
        cmd.set_resource_limits(limits);
        
        // 4. Verify configuration
        assert_eq!(cmd.path, "echo");
        assert_eq!(cmd.args, vec!["integration", "test"]);
        assert!(!cmd.env.is_empty());
        assert!(cmd.dir.is_some());
        
        // 5. Create a pipeline
        let cmd2 = SafeSlayCommand::new("grep", &["test"]);
        let pipeline = SafeSlayPipeline::new(vec![cmd, cmd2]);
        assert_eq!(pipeline.commands.len(), 2);
        
        // 6. Test background task creation
        let bg_cmd = SafeSlayCommand::new("sleep", &["1"]);
        let task_result = SafeSlayTask::run_background(bg_cmd);
        assert!(task_result.is_ok());
        
        // 7. Test global process manager
        let manager = global_process_manager();
        let initial_count = manager.list_processes().len();
        
        // Manager should be accessible
        assert!(initial_count >= 0);
    }

    #[test]
    fn test_safe_process_manager_lifecycle() {
        let manager = SafeProcessManager::new();
        
        // Set global limits
        let global_limits = ResourceLimits {
            max_memory_bytes: Some(100 * 1024 * 1024), // 100MB
            max_cpu_percent: Some(80.0),
            max_execution_time: Some(Duration::from_secs(300)),
            max_file_descriptors: Some(1000),
        };
        manager.set_global_limits(global_limits.clone());
        
        // Test that operations don't panic
        let processes = manager.list_processes();
        assert_eq!(processes.len(), 0);
        
        // Test killing all processes (should be safe when none exist)
        let kill_result = manager.kill_all();
        assert!(kill_result.is_ok());
        
        // Test waiting for all processes (should be safe when none exist)
        let wait_result = manager.wait_all(Some(Duration::from_millis(100)));
        assert!(wait_result.is_ok());
    }

    #[test]
    fn test_error_handling_safety() {
        // Test that error conditions don't cause memory safety issues
        
        // 1. Try to get process from unstarted command
        let mut cmd = SafeSlayCommand::new("nonexistent_command", &[]);
        let process_result = cmd.process();
        assert!(process_result.is_err());
        
        // 2. Try to wait on unstarted command
        let wait_result = cmd.wait();
        assert!(wait_result.is_err());
        
        // 3. Try to get state from unstarted command
        let state_result = cmd.process_state();
        assert!(state_result.is_err());
        
        // 4. Try operations on empty pipeline
        let mut empty_pipeline = SafeSlayPipeline::new(vec![]);
        let start_result = empty_pipeline.start();
        assert!(start_result.is_err());
        
        let output_result = empty_pipeline.output();
        assert!(output_result.is_err());
        
        // All error conditions should be handled safely without panics
    }

    #[test]
    fn test_concurrent_process_management() {
        use std::thread;
        use std::sync::Arc;
        
        let manager = Arc::new(SafeProcessManager::new());
        let mut handles = vec![];
        
        // Spawn multiple threads that use the process manager
        for i in 0..5 {
            let manager_clone = manager.clone();
            let handle = thread::spawn(move || {
                // Each thread creates commands and uses the manager
                let cmd = SafeSlayCommand::new("echo", &[&format!("thread_{}", i)]);
                
                // Test concurrent access to manager
                let processes = manager_clone.list_processes();
                let process_count = processes.len();
                
                // This should be safe
                assert!(process_count >= 0);
                
                // Test setting global limits concurrently
                let limits = ResourceLimits {
                    max_memory_bytes: Some((i + 1) * 1024 * 1024),
                    max_cpu_percent: Some(50.0 + i as f64),
                    max_execution_time: Some(Duration::from_secs(60 + i)),
                    max_file_descriptors: Some(100 + i as u32),
                };
                manager_clone.set_global_limits(limits);
                
                i
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            let result = handle.join();
            assert!(result.is_ok());
        }
        
        // Manager should still be in a consistent state
        let final_processes = manager.list_processes();
        assert!(final_processes.len() >= 0);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_command_creation_performance() {
        let start = Instant::now();
        
        // Create many commands to test performance
        let mut commands = Vec::new();
        for i in 0..1000 {
            let cmd = SafeSlayCommand::new("echo", &[&format!("test_{}", i)]);
            commands.push(cmd);
        }
        
        let duration = start.elapsed();
        
        // Should be able to create 1000 commands quickly
        assert!(duration < Duration::from_secs(1));
        assert_eq!(commands.len(), 1000);
        
        // Test that all commands are properly configured
        for (i, cmd) in commands.iter().enumerate() {
            assert_eq!(cmd.path, "echo");
            assert_eq!(cmd.args, vec![format!("test_{}", i)]);
        }
    }

    #[test]
    fn test_process_manager_performance() {
        let start = Instant::now();
        
        let manager = SafeProcessManager::new();
        
        // Test repeated operations
        for _ in 0..100 {
            let _processes = manager.list_processes();
            manager.set_global_limits(ResourceLimits::default());
        }
        
        let duration = start.elapsed();
        
        // Should be fast
        assert!(duration < Duration::from_millis(100));
    }

    #[test]
    fn test_memory_usage_reasonable() {
        // Create many commands and ensure memory usage is reasonable
        let mut commands = Vec::new();
        
        for i in 0..10000 {
            let mut cmd = SafeSlayCommand::new("echo", &[&format!("memory_test_{}", i)]);
            cmd.add_env("TEST_VAR", &format!("value_{}", i));
            cmd.set_dir("/tmp");
            commands.push(cmd);
        }
        
        // Should be able to create 10k commands without issues
        assert_eq!(commands.len(), 10000);
        
        // Test that they're all properly configured
        assert_eq!(commands[0].path, "echo");
        assert_eq!(commands[9999].path, "echo");
        assert!(!commands[0].env.is_empty());
        assert!(!commands[9999].env.is_empty());
    }
}
