/// Enhanced Process Management Integration Tests
/// 
/// Comprehensive integration tests for the enhanced process management and IPC system,
/// demonstrating all features from ExecSlay and ExecVibez including process pipelines,
/// background task coordination, real-time monitoring, error handling, and cross-platform
/// compatibility.

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use cursed::stdlib::process::{
    // ExecSlay types
    SlayCommand, SlayCommandBuilder, SlayTask, SlayPipeline, SlayOptions, SlayProcess,
    ProcessStats, SignalOptions, ProcessStdin, ProcessStdout, ProcessStderr,
    run_shell, shell_output, run_shell_with_env, run_shell_in_dir,
    run_with_timeout, output_with_timeout, combined_output_with_timeout,
    new_slay_command, new_slay_command_builder, run_background,
    
    // Enhanced ExecVibez types
    EnhancedCmd, EnhancedProcess, EnhancedProcessState, EnhancedEnvironment,
    ProcessGroup, ProcessGroupOptions, OutputStreamer, InputGenerator,
    ProcessContext, look_path, command, command_context, new_environment,
    new_process_group, new_output_streamer, new_input_generator,
    
    // Error types
    ProcessError, ProcessResult,
};

// Common test utilities
mod common;

#[cfg(test)]
mod integration_tests {
    use super::*;
    use common::*;

    /// Test basic SlayCommand functionality
    #[test]
    fn test_slay_command_basic() {
        init_tracing!();
        
        let mut cmd = SlayCommand::new("echo", &["Hello", "CURSED"]);
        
        // Test basic execution
        let output = cmd.output().expect("Command should execute successfully");
        let output_str = String::from_utf8(output).expect("Output should be valid UTF-8");
        assert!(output_str.contains("Hello"));
        assert!(output_str.contains("CURSED"));
        
        tracing::info!("Basic SlayCommand test completed successfully");
    }

    /// Test SlayCommandBuilder fluent API
    #[test]
    fn test_slay_command_builder() {
        init_tracing!();
        
        let cmd = SlayCommandBuilder::new("echo")
            .with_args(&["Builder", "Test"])
            .with_timeout(Duration::from_secs(5))
            .use_shell(false)
            .build();
        
        assert_eq!(cmd.path, "echo");
        assert_eq!(cmd.args, vec!["Builder", "Test"]);
        
        tracing::info!("SlayCommandBuilder test completed successfully");
    }

    /// Test SlayPipeline process chaining
    #[test]
    fn test_slay_pipeline() {
        init_tracing!();
        
        // Create a test file for the pipeline
        let test_file = "test_pipeline_input.txt";
        fs::write(test_file, "apple\nbanana\ncherry\napricot\nblueberry\n")
            .expect("Should write test file");
        
        // Create pipeline: cat file | grep "a" | wc -l
        let cat_cmd = SlayCommand::new("cat", &[test_file]);
        let grep_cmd = SlayCommand::new("grep", &["a"]);
        let wc_cmd = SlayCommand::new("wc", &["-l"]);
        
        let mut pipeline = SlayPipeline::new(vec![cat_cmd, grep_cmd, wc_cmd]);
        
        let result = pipeline.output().expect("Pipeline should execute successfully");
        let count_str = String::from_utf8(result).expect("Output should be valid UTF-8");
        let count: i32 = count_str.trim().parse().expect("Should parse count");
        
        // Should find 4 lines containing 'a': apple, banana, apricot, blueberry
        assert_eq!(count, 4);
        
        // Cleanup
        let _ = fs::remove_file(test_file);
        
        tracing::info!("SlayPipeline test completed successfully");
    }

    /// Test SlayTask background execution
    #[test]
    fn test_slay_task_background() {
        init_tracing!();
        
        let cmd = SlayCommand::new("sleep", &["1"]);
        let mut task = run_background(cmd);
        
        // Task should be running
        assert!(task.is_running());
        
        // Wait for completion
        task.wait().expect("Task should complete successfully");
        
        // Task should be finished
        assert!(task.finished);
        assert_eq!(task.exit_code, Some(0));
        
        tracing::info!("SlayTask background execution test completed successfully");
    }

    /// Test timeout functionality
    #[test]
    fn test_timeout_handling() {
        init_tracing!();
        
        let cmd = SlayCommand::new("sleep", &["5"]);
        let timeout = Duration::from_millis(500);
        
        let start = Instant::now();
        let result = run_with_timeout(cmd, timeout);
        let elapsed = start.elapsed();
        
        // Should timeout quickly
        assert!(result.is_err());
        assert!(elapsed < Duration::from_secs(2));
        
        tracing::info!("Timeout handling test completed successfully");
    }

    /// Test shell command shortcuts
    #[test]
    fn test_shell_commands() {
        init_tracing!();
        
        // Test basic shell command
        run_shell("echo 'Shell test' > shell_test_output.txt")
            .expect("Shell command should execute");
        
        // Test shell output capture
        let output = shell_output("cat shell_test_output.txt")
            .expect("Shell output should be captured");
        let output_str = String::from_utf8(output).expect("Output should be valid UTF-8");
        assert!(output_str.contains("Shell test"));
        
        // Test shell with environment
        let mut env = HashMap::new();
        env.insert("TEST_VAR".to_string(), "test_value".to_string());
        run_shell_with_env("echo $TEST_VAR > env_test_output.txt", env)
            .expect("Shell with env should execute");
        
        let env_output = shell_output("cat env_test_output.txt")
            .expect("Env output should be captured");
        let env_output_str = String::from_utf8(env_output).expect("Output should be valid UTF-8");
        assert!(env_output_str.contains("test_value"));
        
        // Test shell in directory
        fs::create_dir_all("test_dir").expect("Should create test directory");
        run_shell_in_dir("pwd > dir_test_output.txt", "test_dir")
            .expect("Shell in dir should execute");
        
        // Cleanup
        let _ = fs::remove_file("shell_test_output.txt");
        let _ = fs::remove_file("env_test_output.txt");
        let _ = fs::remove_file("test_dir/dir_test_output.txt");
        let _ = fs::remove_dir("test_dir");
        
        tracing::info!("Shell commands test completed successfully");
    }

    /// Test enhanced command with context
    #[test]
    fn test_enhanced_cmd_with_context() {
        init_tracing!();
        
        let ctx = ProcessContext::with_timeout(Duration::from_secs(2));
        let mut cmd = command_context(ctx, "echo", &["Enhanced", "Context", "Test"]);
        
        let output = cmd.output().expect("Enhanced command should execute");
        let output_str = String::from_utf8(output).expect("Output should be valid UTF-8");
        assert!(output_str.contains("Enhanced"));
        assert!(output_str.contains("Context"));
        
        tracing::info!("Enhanced command with context test completed successfully");
    }

    /// Test enhanced environment management
    #[test]
    fn test_enhanced_environment() {
        init_tracing!();
        
        let mut env = EnhancedEnvironment::new();
        env.set("TEST_VAR", "base_value")
           .append("PATH", "/test/path")
           .prepend("LD_LIBRARY_PATH", "/opt/lib")
           .remove("UNWANTED_VAR");
        
        // Test environment application
        let mut cmd = command("env", &[]);
        cmd.env(env);
        
        let output = cmd.output().expect("Env command should execute");
        let output_str = String::from_utf8(output).expect("Output should be valid UTF-8");
        
        assert!(output_str.contains("TEST_VAR=base_value"));
        assert!(output_str.contains("/test/path"));
        
        tracing::info!("Enhanced environment test completed successfully");
    }

    /// Test ProcessGroup coordination
    #[test]
    fn test_process_group() {
        init_tracing!();
        
        let mut group = ProcessGroup::new();
        
        // Add multiple commands to the group
        group.add_command(command("echo", &["Command", "1"]));
        group.add_command(command("echo", &["Command", "2"]));
        group.add_command(command("echo", &["Command", "3"]));
        
        // Set group options
        let options = ProcessGroupOptions {
            start_all: true,
            wait_all: true,
            continue_on_failure: true,
            group_timeout: Some(Duration::from_secs(10)),
            max_concurrent: Some(5),
            priority: None,
            kill_tree_on_failure: false,
        };
        group.options(options);
        
        // Start and wait for all commands
        group.start_all().expect("Group should start successfully");
        group.wait_all().expect("Group should complete successfully");
        
        let status = group.status();
        assert_eq!(status.total, 3);
        
        tracing::info!("ProcessGroup test completed successfully");
    }

    /// Test OutputStreamer real-time processing
    #[test]
    fn test_output_streamer() {
        init_tracing!();
        
        let cmd = command("echo", &["Streaming", "Output", "Test"]);
        let mut streamer = OutputStreamer::new(cmd);
        
        let captured_lines = Arc::new(Mutex::new(Vec::new()));
        let captured_lines_clone = captured_lines.clone();
        
        streamer.on_stdout_line(move |line| {
            let mut lines = captured_lines_clone.lock().unwrap();
            lines.push(line.to_string());
        }).capture_output(true);
        
        streamer.start().expect("Streamer should start successfully");
        streamer.wait().expect("Streamer should complete successfully");
        
        let lines = captured_lines.lock().unwrap();
        assert!(!lines.is_empty());
        assert!(lines[0].contains("Streaming"));
        
        let captured_stdout = streamer.get_captured_stdout();
        let stdout_str = String::from_utf8(captured_stdout).expect("Captured output should be valid UTF-8");
        assert!(stdout_str.contains("Output"));
        
        tracing::info!("OutputStreamer test completed successfully");
    }

    /// Test InputGenerator programmatic input
    #[test]
    fn test_input_generator() {
        init_tracing!();
        
        let cmd = command("cat", &[]);
        let mut generator = InputGenerator::new(cmd);
        
        // Queue some input
        generator.write_line("Line 1").expect("Should write line 1");
        generator.write_line("Line 2").expect("Should write line 2");
        generator.write_line_after("Delayed Line", Duration::from_millis(100))
            .expect("Should write delayed line");
        
        generator.set_auto_close(true);
        generator.start().expect("Generator should start successfully");
        
        // Wait a bit for input to be processed
        thread::sleep(Duration::from_millis(200));
        
        generator.close().expect("Generator should close successfully");
        
        tracing::info!("InputGenerator test completed successfully");
    }

    /// Test cross-platform LookPath
    #[test]
    fn test_look_path() {
        init_tracing!();
        
        // Test finding a common command
        #[cfg(unix)]
        let test_cmd = "sh";
        #[cfg(windows)]
        let test_cmd = "cmd";
        
        match look_path(test_cmd) {
            Ok(path) => {
                assert!(path.contains(test_cmd));
                tracing::info!("Found {} at: {}", test_cmd, path);
            }
            Err(_) => {
                tracing::warn!("{} not found in PATH (may be expected in test environment)", test_cmd);
            }
        }
        
        // Test non-existent command
        let result = look_path("definitely_not_a_real_command_12345");
        assert!(result.is_err());
        
        tracing::info!("LookPath test completed successfully");
    }

    /// Test process monitoring and statistics
    #[test]
    fn test_process_monitoring() {
        init_tracing!();
        
        let mut cmd = SlayCommand::new("sleep", &["2"]);
        cmd.start().expect("Command should start");
        
        let process = cmd.process().expect("Should get process handle");
        
        // Test statistics gathering
        match process.stats() {
            Ok(stats) => {
                assert!(stats.up_time.as_millis() > 0);
                assert!(stats.thread_count > 0);
                tracing::info!("Process stats: CPU: {}%, Memory: {} bytes", stats.cpu, stats.memory);
            }
            Err(e) => {
                tracing::warn!("Failed to get process stats (may be platform-specific): {}", e);
            }
        }
        
        // Test monitoring with callback
        let monitoring_done = Arc::new(Mutex::new(false));
        let monitoring_done_clone = monitoring_done.clone();
        
        let _ = process.monitor(Duration::from_millis(100), move |stats| {
            tracing::info!("Monitor callback: CPU {}%, Memory {} bytes", stats.cpu, stats.memory);
            *monitoring_done_clone.lock().unwrap() = true;
        });
        
        // Wait for monitoring to trigger at least once
        thread::sleep(Duration::from_millis(200));
        
        cmd.wait().expect("Command should complete");
        
        tracing::info!("Process monitoring test completed successfully");
    }

    /// Test signal handling and process control
    #[test]
    fn test_signal_handling() {
        init_tracing!();
        
        #[cfg(unix)]
        {
            let mut cmd = SlayCommand::new("sleep", &["10"]);
            cmd.start().expect("Command should start");
            
            let process = cmd.process().expect("Should get process handle");
            
            // Test graceful termination
            let signal_opts = SignalOptions {
                grace_period: Duration::from_millis(100),
                force: true,
                signal: 15, // SIGTERM
                recursive: false,
            };
            
            process.terminate(signal_opts).expect("Should terminate gracefully");
            
            // Command should have been killed
            let result = cmd.wait();
            assert!(result.is_err()); // Should fail because process was killed
            
            tracing::info!("Signal handling test completed successfully");
        }
        
        #[cfg(not(unix))]
        {
            tracing::info!("Signal handling test skipped on non-Unix platform");
        }
    }

    /// Test error handling and recovery
    #[test]
    fn test_error_handling() {
        init_tracing!();
        
        // Test command not found
        let mut cmd = SlayCommand::new("definitely_not_a_real_command", &[]);
        let result = cmd.start();
        assert!(result.is_err());
        
        // Test invalid arguments
        let mut cmd2 = SlayCommand::new("ls", &["--invalid-flag-that-does-not-exist"]);
        let result2 = cmd2.run();
        assert!(result2.is_err());
        
        // Test timeout with error handling
        let cmd3 = SlayCommand::new("sleep", &["5"]);
        let result3 = run_with_timeout(cmd3, Duration::from_millis(100));
        assert!(result3.is_err());
        
        tracing::info!("Error handling test completed successfully");
    }

    /// Test IPC integration with process pipelines
    #[test]
    fn test_ipc_integration() {
        init_tracing!();
        
        // Create a test scenario that demonstrates IPC between processes
        let test_data = "Hello IPC World\nLine 2\nLine 3\n";
        let input_file = "ipc_test_input.txt";
        fs::write(input_file, test_data).expect("Should write input file");
        
        // Create a pipeline that demonstrates IPC
        let cat_cmd = SlayCommand::new("cat", &[input_file]);
        let grep_cmd = SlayCommand::new("grep", &["IPC"]);
        let sed_cmd = SlayCommand::new("sed", &["s/World/Universe/g"]);
        
        let mut pipeline = SlayPipeline::new(vec![cat_cmd, grep_cmd, sed_cmd]);
        
        let result = pipeline.output().expect("IPC pipeline should execute");
        let output_str = String::from_utf8(result).expect("Output should be valid UTF-8");
        
        assert!(output_str.contains("IPC"));
        assert!(output_str.contains("Universe"));
        assert!(!output_str.contains("World"));
        
        // Cleanup
        let _ = fs::remove_file(input_file);
        
        tracing::info!("IPC integration test completed successfully");
    }

    /// Test performance and scalability
    #[test]
    fn test_performance_scalability() {
        init_tracing!();
        
        let start_time = Instant::now();
        
        // Create multiple concurrent processes
        let mut tasks = Vec::new();
        for i in 0..5 {
            let cmd = SlayCommand::new("echo", &[&format!("Process {}", i)]);
            let task = run_background(cmd);
            tasks.push(task);
        }
        
        // Wait for all to complete
        for mut task in tasks {
            task.wait().expect(&format!("Task should complete successfully"));
            assert!(task.finished);
            assert_eq!(task.exit_code, Some(0));
        }
        
        let elapsed = start_time.elapsed();
        tracing::info!("Performance test completed in {:?}", elapsed);
        
        // Should complete reasonably quickly
        assert!(elapsed < Duration::from_secs(10));
        
        tracing::info!("Performance and scalability test completed successfully");
    }

    /// Test cross-platform compatibility
    #[test]
    fn test_cross_platform_compatibility() {
        init_tracing!();
        
        // Test platform-specific commands
        #[cfg(unix)]
        {
            let mut cmd = SlayCommand::new("uname", &["-s"]);
            let output = cmd.output().expect("uname should work on Unix");
            let output_str = String::from_utf8(output).expect("Output should be valid UTF-8");
            tracing::info!("Unix system: {}", output_str.trim());
        }
        
        #[cfg(windows)]
        {
            let mut cmd = SlayCommand::new("echo", &["%OS%"]);
            let output = cmd.output().expect("echo should work on Windows");
            let output_str = String::from_utf8(output).expect("Output should be valid UTF-8");
            tracing::info!("Windows system detected: {}", output_str.trim());
        }
        
        // Test cross-platform echo command
        let mut echo_cmd = SlayCommand::new("echo", &["Cross-platform", "test"]);
        let echo_output = echo_cmd.output().expect("Echo should work on all platforms");
        let echo_str = String::from_utf8(echo_output).expect("Output should be valid UTF-8");
        assert!(echo_str.contains("Cross-platform"));
        
        tracing::info!("Cross-platform compatibility test completed successfully");
    }

    /// Comprehensive integration test combining multiple features
    #[test]
    fn test_comprehensive_integration() {
        init_tracing!();
        
        tracing::info!("Starting comprehensive integration test");
        
        // 1. Create enhanced environment
        let mut env = EnhancedEnvironment::new();
        env.set("INTEGRATION_TEST", "comprehensive")
           .append("PATH", "/test/bin");
        
        // 2. Create process group with multiple commands
        let mut group = ProcessGroup::new();
        
        // Add commands with different configurations
        let mut cmd1 = command("echo", &["Group", "Command", "1"]);
        cmd1.env(env.clone());
        
        let mut cmd2 = command("echo", &["Group", "Command", "2"]);
        cmd2.context(ProcessContext::with_timeout(Duration::from_secs(5)));
        
        group.add_command(cmd1);
        group.add_command(cmd2);
        
        // 3. Configure group with comprehensive options
        let options = ProcessGroupOptions {
            start_all: true,
            wait_all: true,
            continue_on_failure: true,
            group_timeout: Some(Duration::from_secs(30)),
            max_concurrent: Some(10),
            priority: None,
            kill_tree_on_failure: false,
        };
        group.options(options);
        
        // 4. Execute the group
        group.start_all().expect("Group should start successfully");
        group.wait_all().expect("Group should complete successfully");
        
        // 5. Verify group status
        let status = group.status();
        assert_eq!(status.total, 2);
        
        // 6. Test output streaming with the same environment
        let stream_cmd = command("echo", &["Streaming", "Integration", "Test"]);
        let mut streamer = OutputStreamer::new(stream_cmd);
        
        let captured_output = Arc::new(Mutex::new(String::new()));
        let captured_output_clone = captured_output.clone();
        
        streamer.on_stdout_line(move |line| {
            let mut output = captured_output_clone.lock().unwrap();
            output.push_str(line);
            output.push('\n');
        }).capture_output(true);
        
        streamer.start().expect("Streamer should start successfully");
        streamer.wait().expect("Streamer should complete successfully");
        
        let final_output = captured_output.lock().unwrap().clone();
        assert!(final_output.contains("Integration"));
        
        // 7. Test background task with monitoring
        let bg_cmd = SlayCommand::new("sleep", &["1"]);
        let mut bg_task = run_background(bg_cmd);
        
        // Monitor the background task
        let start_time = Instant::now();
        while bg_task.is_running() && start_time.elapsed() < Duration::from_secs(5) {
            thread::sleep(Duration::from_millis(100));
        }
        
        bg_task.wait().expect("Background task should complete");
        assert!(bg_task.finished);
        
        tracing::info!("Comprehensive integration test completed successfully");
    }
}

#[path = "common.rs"]
mod common {
    //! Common test utilities and helper functions

    /// Initialize tracing for tests
    macro_rules! init_tracing {
        () => {
            let _ = tracing_subscriber::fmt()
                .with_env_filter("debug")
                .with_test_writer()
                .try_init();
        };
    }

    pub(crate) use init_tracing;
}
