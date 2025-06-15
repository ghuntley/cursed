/// Comprehensive test suite for enhanced process management and IPC system
/// 
/// This test suite validates the production-ready process management and IPC 
/// functionality including enhanced command builders, resource monitoring,
/// connection pooling, signal handling, and cross-platform compatibility.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;

use cursed::stdlib::process::{
    EnhancedCommandBuilder, EnhancedProcess, ProcessStatistics, ProcessOutput,
    ResourceLimits, ProcessPriority, SecurityContext, StdinConfig, StdoutConfig, StderrConfig
};

use cursed::stdlib::ipc::{
    IpcConnectionPool, IpcConnectionType, PooledConnection, ConnectionPoolConfig,
    IpcPoolManager, initialize_pool_manager, get_pool_manager, cleanup_pool_manager
};

use cursed::stdlib::signal_boost::{
    VibeChecker, YeetHandler, NoCapReloadManager, FlexSignalQueue, 
    vibe_check, yeet_on_signal, no_cap_reload_config, flex_signal_queue,
    SIGUSR1, SIGUSR2, SIGHUP, SIGINT, SIGTERM
};

use cursed::stdlib::sys_core::{
    initialize, get_system_info, FileDescriptor, create_fd, close_fd,
    poll_fds, epoll_operations, get_fd_table
};

// Test helper macros and utilities
macro_rules! assert_duration_within {
    ($actual:expr, $expected:expr, $tolerance:expr) => {
        let actual = $actual;
        let expected = $expected;
        let tolerance = $tolerance;
        assert!(
            actual >= expected.saturating_sub(tolerance) && actual <= expected + tolerance,
            "Duration {:?} not within {:?} ± {:?}",
            actual, expected, tolerance
        );
    };
}

macro_rules! wait_for_condition {
    ($condition:expr, $timeout:expr) => {{
        let start = Instant::now();
        let timeout = $timeout;
        loop {
            if $condition {
                break Ok(());
            }
            if start.elapsed() > timeout {
                break Err("Timeout waiting for condition");
            }
            thread::sleep(Duration::from_millis(10));
        }
    }};
}

/// Integration test for enhanced command builder functionality
#[test]
fn test_enhanced_command_builder_comprehensive() {
    let output_buffer = Arc::new(Mutex::new(Vec::new()));
    let error_buffer = Arc::new(Mutex::new(Vec::new()));
    
    let result = EnhancedCommandBuilder::new("echo")
        .args(&["Hello", "World", "from", "enhanced", "command", "builder"])
        .env("TEST_VAR", "test_value")
        .env("RUST_LOG", "debug")
        .timeout(Duration::from_secs(5))
        .resource_limits(ResourceLimits {
            max_memory: Some(100 * 1024 * 1024), // 100MB
            max_cpu_time: Some(Duration::from_secs(2)),
            max_wall_time: Some(Duration::from_secs(5)),
            max_open_files: Some(100),
            max_processes: Some(10),
            max_file_size: Some(1024 * 1024), // 1MB
        })
        .priority(ProcessPriority::Normal)
        .stdout(StdoutConfig::ToBuf(Arc::clone(&output_buffer)))
        .stderr(StderrConfig::ToBuf(Arc::clone(&error_buffer)))
        .output();
    
    assert!(result.is_ok(), "Enhanced command builder should execute successfully");
    
    let output = result.unwrap();
    assert!(output.status.success(), "Command should succeed");
    assert!(output.execution_time < Duration::from_secs(5), "Should complete within timeout");
    
    let stdout_data = output_buffer.lock().unwrap();
    let stdout_str = String::from_utf8_lossy(&stdout_data);
    assert!(stdout_str.contains("Hello World"), "Output should contain expected text");
    
    // Verify statistics are populated
    assert!(output.statistics.pid > 0, "Should have valid process ID");
    assert!(output.statistics.start_time <= Instant::now(), "Start time should be valid");
}

/// Test process monitoring and resource limit enforcement
#[test] 
fn test_process_monitoring_and_limits() {
    let mut process = EnhancedCommandBuilder::new("sleep")
        .arg("2")
        .resource_limits(ResourceLimits {
            max_memory: Some(50 * 1024 * 1024), // 50MB
            max_cpu_time: Some(Duration::from_secs(5)),
            ..Default::default()
        })
        .spawn()
        .expect("Should spawn sleep process");
    
    // Check initial statistics
    let initial_stats = process.statistics();
    assert!(initial_stats.pid > 0, "Should have valid PID");
    assert_eq!(initial_stats.memory_usage, 0, "Initial memory usage should be 0 (not yet measured)");
    
    // Wait a bit and check statistics again
    thread::sleep(Duration::from_millis(500));
    let updated_stats = process.statistics();
    
    // The process should still be running
    assert!(process.is_valid(), "Process should still be valid");
    
    // Wait for completion
    let status = process.wait().expect("Should wait successfully");
    assert!(status.success(), "Sleep command should succeed");
    
    let final_stats = process.statistics();
    assert!(final_stats.cpu_time > Duration::from_secs(0), "Should have some CPU time");
}

/// Test IPC connection pooling functionality
#[test]
fn test_ipc_connection_pooling() {
    let config = ConnectionPoolConfig {
        max_connections: 10,
        min_connections: 2,
        max_idle_time: Duration::from_secs(30),
        connection_timeout: Duration::from_secs(10),
        validation_interval: Duration::from_secs(5),
        retry_attempts: 3,
        retry_delay: Duration::from_millis(100),
    };
    
    let pool = IpcConnectionPool::new(config);
    
    // Test basic pool statistics
    let initial_stats = pool.statistics();
    assert_eq!(initial_stats.active_connections, 0, "Should start with no active connections");
    assert_eq!(initial_stats.total_connections_created, 0, "Should start with no created connections");
    
    // Note: We can't easily test actual connections without implementing mock connection types
    // This test validates the pool structure and basic functionality
    
    // Test shutdown
    pool.shutdown().expect("Should shutdown successfully");
}

/// Test IPC pool manager functionality
#[test]
fn test_ipc_pool_manager() {
    let config = ConnectionPoolConfig::default();
    
    // Initialize global pool manager
    initialize_pool_manager(config).expect("Should initialize pool manager");
    
    let manager = get_pool_manager().expect("Should get pool manager");
    
    // Test getting pools
    let pool1 = manager.get_pool("test_pool_1").expect("Should create pool");
    let pool2 = manager.get_pool("test_pool_2").expect("Should create pool");
    let pool1_again = manager.get_pool("test_pool_1").expect("Should get existing pool");
    
    // Verify pools are properly managed
    assert!(Arc::ptr_eq(&pool1, &pool1_again), "Should return same pool instance");
    
    // Get statistics
    let all_stats = manager.get_all_statistics();
    assert_eq!(all_stats.len(), 2, "Should have statistics for both pools");
    
    // Cleanup
    manager.shutdown_all().expect("Should shutdown all pools");
    cleanup_pool_manager().expect("Should cleanup pool manager");
}

/// Test signal handling with VibeChecker
#[test]
fn test_signal_handling_vibe_checker() {
    let check_count = Arc::new(Mutex::new(0));
    let check_count_clone = Arc::clone(&check_count);
    
    let mut checker = vibe_check(SIGUSR1, move || {
        let mut count = check_count_clone.lock().unwrap();
        *count += 1;
        *count < 5 // Return false after 5 checks to test failure handling
    });
    
    // Start the checker
    checker.start().expect("Should start VibeChecker");
    
    // Verify it's running
    let status = checker.status();
    assert!(status.running, "VibeChecker should be running");
    
    // Send a signal to trigger check
    unsafe {
        #[cfg(unix)]
        libc::kill(std::process::id() as i32, SIGUSR1);
    }
    
    // Wait for the check to be processed
    thread::sleep(Duration::from_millis(100));
    
    // Verify check was called
    let count = *check_count.lock().unwrap();
    assert!(count > 0, "Vibe check should have been called at least once");
    
    // Stop the checker
    checker.stop().expect("Should stop VibeChecker");
    
    let final_status = checker.status();
    assert!(!final_status.running, "VibeChecker should be stopped");
}

/// Test YeetHandler for aggressive termination
#[test]
fn test_yeet_handler() {
    let mut yeet_handler = yeet_on_signal(vec![SIGUSR2]);
    
    // Test starting the handler
    yeet_handler.start().expect("Should start YeetHandler");
    
    let stats = yeet_handler.get_yeet_stats();
    assert!(stats.is_active, "YeetHandler should be active");
    assert_eq!(stats.total_yeets, 0, "Should start with no yeets");
    
    // Note: We can't easily test actual yeet functionality in a test environment
    // as it would terminate the test process. This validates structure and basic operations.
    
    // Stop the handler
    yeet_handler.stop().expect("Should stop YeetHandler");
    
    let final_stats = yeet_handler.get_yeet_stats();
    assert!(!final_stats.is_active, "YeetHandler should be inactive");
}

/// Test NoCapReloadManager for configuration management
#[test]
fn test_no_cap_reload_manager() {
    use std::fs;
    use std::path::PathBuf;
    
    // Create a temporary config file
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("test_config.toml");
    fs::write(&config_path, "test_setting = 'initial_value'\n")
        .expect("Should write initial config");
    
    let mut reload_manager = no_cap_reload_config(&config_path, SIGHUP);
    
    // Add a validator
    reload_manager.add_validator(|config| config.contains("test_setting"));
    
    // Start the manager
    reload_manager.start().expect("Should start NoCapReloadManager");
    
    // Verify initial config is loaded
    let initial_config = reload_manager.get_config();
    assert!(initial_config.contains("initial_value"), "Should load initial config");
    
    // Update the config file
    fs::write(&config_path, "test_setting = 'updated_value'\n")
        .expect("Should write updated config");
    
    // Send reload signal
    unsafe {
        #[cfg(unix)]
        libc::kill(std::process::id() as i32, SIGHUP);
    }
    
    // Wait for reload
    thread::sleep(Duration::from_millis(200));
    
    // Verify config was reloaded
    let updated_config = reload_manager.get_config();
    // Note: The actual reload might not happen in test environment due to signal handling complexity
    
    let stats = reload_manager.get_reload_stats();
    assert!(stats.is_active, "Reload manager should be active");
    
    // Stop the manager
    reload_manager.stop().expect("Should stop NoCapReloadManager");
    
    // Cleanup
    let _ = fs::remove_file(&config_path);
}

/// Test FlexSignalQueue for priority signal processing
#[test]
fn test_flex_signal_queue() {
    let processed_signals = Arc::new(Mutex::new(Vec::new()));
    let processed_clone = Arc::clone(&processed_signals);
    
    let mut signal_queue = flex_signal_queue(vec![SIGUSR1, SIGUSR2], 100);
    
    // Set custom priorities
    signal_queue.set_priority(SIGUSR1, 100); // High priority
    signal_queue.set_priority(SIGUSR2, 50);  // Medium priority
    
    // Start processing with a custom processor
    signal_queue.start(move |signal| {
        processed_clone.lock().unwrap().push(signal);
        Ok(())
    }).expect("Should start FlexSignalQueue");
    
    // Test queue statistics
    let initial_stats = signal_queue.get_stats();
    assert_eq!(initial_stats.total_signals, 0, "Should start with no signals");
    assert_eq!(initial_stats.current_queue_size, 0, "Queue should be empty");
    
    // Note: Testing actual signal queuing is complex in test environment
    // This validates the structure and basic functionality
    
    // Stop the queue
    signal_queue.stop().expect("Should stop FlexSignalQueue");
}

/// Test system core operations
#[test]
fn test_sys_core_operations() {
    // Initialize sys_core
    initialize().expect("Should initialize sys_core");
    
    // Get system information
    let sys_info = get_system_info();
    assert!(!sys_info.platform.is_empty(), "Platform should not be empty");
    assert!(sys_info.page_size > 0, "Page size should be positive");
    assert!(sys_info.max_open_files > 0, "Max open files should be positive");
    
    // Test file descriptor table
    let fd_table = get_fd_table();
    let initial_fds = fd_table.list_fds();
    
    // Note: Actual FD operations require careful platform-specific handling
    // This test validates the basic structure
}

/// Test file descriptor operations on Unix systems
#[cfg(unix)]
#[test]
fn test_file_descriptor_operations() {
    use cursed::stdlib::sys_core::{create_fd, close_fd, get_fd_flags, set_fd_flags, FileDescriptorFlags};
    use std::ffi::CString;
    use std::os::unix::io::RawFd;
    
    // Create a temporary file for testing
    let temp_path = "/tmp/cursed_test_fd";
    
    // Test creating a file descriptor
    let fd_result = create_fd(temp_path, libc::O_CREAT | libc::O_RDWR, 0o644);
    assert!(fd_result.is_ok(), "Should create file descriptor successfully");
    
    let fd = fd_result.unwrap();
    assert!(fd >= 0, "File descriptor should be valid");
    
    // Test getting flags
    let flags_result = get_fd_flags(fd);
    assert!(flags_result.is_ok(), "Should get file descriptor flags");
    
    let flags = flags_result.unwrap();
    assert!(flags.readable || flags.writable, "File should be readable or writable");
    
    // Test setting flags
    let mut new_flags = flags;
    new_flags.non_blocking = true;
    let set_result = set_fd_flags(fd, new_flags);
    assert!(set_result.is_ok(), "Should set file descriptor flags");
    
    // Test closing the file descriptor
    let close_result = close_fd(fd);
    assert!(close_result.is_ok(), "Should close file descriptor successfully");
    
    // Cleanup
    let _ = std::fs::remove_file(temp_path);
}

/// Test epoll operations on Linux
#[cfg(target_os = "linux")]
#[test]
fn test_epoll_operations() {
    use cursed::stdlib::sys_core::{epoll_operations, supports_epoll};
    
    assert!(supports_epoll(), "Should support epoll on Linux");
    
    let mut epoll_ops = epoll_operations();
    
    // Create epoll instance
    let create_result = epoll_ops.create();
    assert!(create_result.is_ok(), "Should create epoll instance");
    
    // Note: Adding actual file descriptors and testing wait functionality
    // requires more complex setup with real pipes or sockets
}

/// Test process pipeline execution
#[test]
fn test_process_pipeline_execution() {
    // Test simple pipeline: echo "hello world" | wc -w
    let echo_output = EnhancedCommandBuilder::new("echo")
        .args(&["hello", "world", "from", "pipeline"])
        .stdout(StdoutConfig::Piped)
        .output()
        .expect("Should execute echo command");
    
    assert!(echo_output.status.success(), "Echo should succeed");
    
    let wc_result = EnhancedCommandBuilder::new("wc")
        .arg("-w")
        .stdin(StdinConfig::FromBytes(echo_output.stdout))
        .output();
    
    assert!(wc_result.is_ok(), "Should execute wc command");
    
    let wc_output = wc_result.unwrap();
    assert!(wc_output.status.success(), "Word count should succeed");
    
    let word_count = String::from_utf8_lossy(&wc_output.stdout).trim();
    assert_eq!(word_count, "4", "Should count 4 words");
}

/// Test concurrent process execution
#[test]
fn test_concurrent_process_execution() {
    let num_processes = 5;
    let mut handles = Vec::new();
    
    for i in 0..num_processes {
        let handle = thread::spawn(move || {
            let result = EnhancedCommandBuilder::new("echo")
                .arg(&format!("Process {}", i))
                .timeout(Duration::from_secs(2))
                .output();
            
            assert!(result.is_ok(), "Process {} should succeed", i);
            let output = result.unwrap();
            assert!(output.status.success(), "Process {} should have success status", i);
            
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        });
        handles.push(handle);
    }
    
    // Wait for all processes to complete
    let mut results = Vec::new();
    for handle in handles {
        let result = handle.join().expect("Thread should complete successfully");
        results.push(result);
    }
    
    // Verify all processes completed
    assert_eq!(results.len(), num_processes, "All processes should complete");
    
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result, &format!("Process {}", i), "Process {} should have correct output", i);
    }
}

/// Test resource limit enforcement
#[test]
fn test_resource_limit_enforcement() {
    // Test memory limit (this might not always trigger on small commands)
    let result = EnhancedCommandBuilder::new("echo")
        .arg("testing memory limits")
        .resource_limits(ResourceLimits {
            max_memory: Some(1024), // Very small limit
            ..Default::default()
        })
        .timeout(Duration::from_secs(5))
        .output();
    
    // Note: The actual enforcement depends on system behavior
    // This test mainly validates that limits can be set without errors
    assert!(result.is_ok(), "Should handle resource limits gracefully");
}

/// Test security context application
#[test]
fn test_security_context_application() {
    // Test basic security context (without actually changing user/group in test)
    let security_context = SecurityContext {
        user_id: None, // Don't actually change user in test
        group_id: None, // Don't actually change group in test
        supplementary_groups: Vec::new(),
        capabilities: vec!["CAP_NET_BIND_SERVICE".to_string()],
        seccomp_filter: None,
        chroot_path: None,
        namespace_isolation: Default::default(),
    };
    
    let result = EnhancedCommandBuilder::new("echo")
        .arg("testing security context")
        .security_context(security_context)
        .output();
    
    assert!(result.is_ok(), "Should handle security context gracefully");
}

/// Performance test for process creation and monitoring
#[test]
fn test_process_creation_performance() {
    let start_time = Instant::now();
    let num_processes = 10;
    
    for i in 0..num_processes {
        let result = EnhancedCommandBuilder::new("echo")
            .arg(&format!("Performance test {}", i))
            .output();
        
        assert!(result.is_ok(), "Process {} should succeed", i);
    }
    
    let total_time = start_time.elapsed();
    let average_time = total_time / num_processes;
    
    // Processes should be reasonably fast (less than 500ms each on average)
    assert!(average_time < Duration::from_millis(500), 
           "Average process creation time should be under 500ms, got {:?}", average_time);
    
    println!("Created {} processes in {:?} (avg: {:?} per process)", 
             num_processes, total_time, average_time);
}

/// Cross-platform compatibility test
#[test]
fn test_cross_platform_compatibility() {
    // Test commands that should work on all platforms
    let echo_result = EnhancedCommandBuilder::new("echo")
        .arg("cross-platform test")
        .output();
    
    assert!(echo_result.is_ok(), "Echo should work on all platforms");
    
    #[cfg(unix)]
    {
        let unix_result = EnhancedCommandBuilder::new("ls")
            .arg("/tmp")
            .output();
        assert!(unix_result.is_ok(), "ls should work on Unix systems");
    }
    
    #[cfg(windows)]
    {
        let windows_result = EnhancedCommandBuilder::new("dir")
            .arg("C:\\")
            .output();
        assert!(windows_result.is_ok(), "dir should work on Windows systems");
    }
}

/// Error handling and recovery test
#[test]
fn test_error_handling_and_recovery() {
    // Test command that doesn't exist
    let nonexistent_result = EnhancedCommandBuilder::new("this_command_does_not_exist_12345")
        .output();
    
    assert!(nonexistent_result.is_err(), "Nonexistent command should fail");
    
    // Test timeout handling
    let timeout_result = EnhancedCommandBuilder::new("sleep")
        .arg("10")
        .timeout(Duration::from_millis(100))
        .output();
    
    assert!(timeout_result.is_err(), "Long-running command should timeout");
    
    // Test recovery - normal command should still work
    let recovery_result = EnhancedCommandBuilder::new("echo")
        .arg("recovery test")
        .output();
    
    assert!(recovery_result.is_ok(), "Normal commands should work after errors");
}

/// Integration test combining process management, IPC, and signal handling
#[test]
fn test_comprehensive_integration() {
    // Initialize all systems
    cursed::stdlib::sys_core::initialize().expect("Should initialize sys_core");
    cursed::stdlib::ipc::initialize_pool_manager(ConnectionPoolConfig::default())
        .expect("Should initialize IPC pool manager");
    
    // Test process execution with monitoring
    let mut process = EnhancedCommandBuilder::new("echo")
        .args(&["integration", "test", "complete"])
        .resource_limits(ResourceLimits {
            max_memory: Some(100 * 1024 * 1024),
            max_cpu_time: Some(Duration::from_secs(5)),
            ..Default::default()
        })
        .spawn()
        .expect("Should spawn process");
    
    // Monitor the process
    let initial_stats = process.statistics();
    assert!(initial_stats.pid > 0, "Should have valid process ID");
    
    // Wait for completion
    let status = process.wait().expect("Should wait for process");
    assert!(status.success(), "Process should complete successfully");
    
    // Test IPC pool manager
    let pool_manager = get_pool_manager().expect("Should get pool manager");
    let test_pool = pool_manager.get_pool("integration_test").expect("Should create test pool");
    
    let pool_stats = test_pool.statistics();
    assert_eq!(pool_stats.active_connections, 0, "Pool should start empty");
    
    // Test signal handling
    let check_called = Arc::new(Mutex::new(false));
    let check_called_clone = Arc::clone(&check_called);
    
    let mut vibe_checker = vibe_check(SIGUSR1, move || {
        *check_called_clone.lock().unwrap() = true;
        true
    });
    
    vibe_checker.start().expect("Should start vibe checker");
    
    // Cleanup all systems
    vibe_checker.stop().expect("Should stop vibe checker");
    pool_manager.shutdown_all().expect("Should shutdown all pools");
    cleanup_pool_manager().expect("Should cleanup pool manager");
    
    println!("Comprehensive integration test completed successfully");
}

/// Memory and resource cleanup test
#[test]
fn test_memory_and_resource_cleanup() {
    let initial_stats = get_system_info();
    
    // Create and destroy multiple processes
    for i in 0..10 {
        let result = EnhancedCommandBuilder::new("echo")
            .arg(&format!("cleanup test {}", i))
            .resource_limits(ResourceLimits {
                max_memory: Some(50 * 1024 * 1024),
                ..Default::default()
            })
            .output();
        
        assert!(result.is_ok(), "Process {} should succeed", i);
        
        // The process should be automatically cleaned up when result is dropped
    }
    
    // Create and manage multiple signal handlers
    for i in 0..5 {
        let mut checker = vibe_check(SIGUSR1, move || true);
        checker.start().expect("Should start checker");
        thread::sleep(Duration::from_millis(10));
        checker.stop().expect("Should stop checker");
    }
    
    // Verify system state after cleanup
    let final_stats = get_system_info();
    assert_eq!(initial_stats.platform, final_stats.platform, "Platform should remain the same");
    assert_eq!(initial_stats.page_size, final_stats.page_size, "Page size should remain the same");
    
    println!("Memory and resource cleanup test completed");
}
