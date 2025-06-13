/// Comprehensive integration tests for the CURSED process management system
/// 
/// This test suite validates all aspects of the process management functionality
/// including process spawning, control, monitoring, and cross-platform compatibility.

use std::time::Duration;
use std::thread;

use cursed::stdlib::process::{
    // Core functionality
    ProcessConfig, ProcessIo, spawn_process, run_command, run_command_timeout,
    command_exists, which, exec, exec_with_args,
    
    // Process information
    get_current_pid, get_parent_pid, is_process_running, get_process_info,
    get_process_memory, get_process_cpu, get_process_list, find_processes_by_name,
    get_process_tree, get_load_average, get_cpu_count, get_system_uptime,
    
    // Process control
    Signal, Priority, ProcessControl, send_signal_to_pid, kill_process, terminate_process,
    kill_process_graceful, set_process_priority, get_process_priority, wait_for_process,
    
    // Process monitoring
    HealthStatus, ResourceThresholds, HealthCheckConfig, PerformanceMetrics,
    PerformanceHistory, ProcessMonitor, MonitoredProcess, 
    create_process_monitor, monitor_process_once,
    
    // Error handling
    ProcessError, ProcessResult,
};

#[test]
fn test_basic_process_spawning() {
    // Test basic command execution
    #[cfg(unix)]
    let config = ProcessConfig::new("echo")
        .arg("Hello, World!")
        .stdout(ProcessIo::Pipe);
    
    #[cfg(windows)]
    let config = ProcessConfig::new("cmd")
        .args(&["/C", "echo", "Hello, World!"])
        .stdout(ProcessIo::Pipe);
    
    let mut process = spawn_process(config).expect("Failed to spawn process");
    
    // Check that process is running
    assert!(process.is_running().expect("Failed to check process status"));
    
    // Wait for completion
    let status = process.wait().expect("Failed to wait for process");
    assert!(status.success());
    
    // Check captured output
    let (stdout, _stderr) = process.get_output().expect("Failed to get output");
    let output_str = String::from_utf8_lossy(&stdout);
    assert!(output_str.contains("Hello"));
}

#[test]
fn test_process_configuration() {
    let config = ProcessConfig::new("echo")
        .arg("test")
        .args(&["arg1", "arg2"])
        .env("TEST_VAR", "test_value")
        .envs(&[("VAR1", "value1"), ("VAR2", "value2")])
        .timeout(Duration::from_secs(30))
        .stdin(ProcessIo::Null)
        .stdout(ProcessIo::Pipe)
        .stderr(ProcessIo::Inherit);
    
    assert_eq!(config.command, "echo");
    assert_eq!(config.args, vec!["test", "arg1", "arg2"]);
    assert_eq!(config.env_vars.get("TEST_VAR"), Some(&"test_value".to_string()));
    assert_eq!(config.env_vars.get("VAR1"), Some(&"value1".to_string()));
    assert_eq!(config.timeout, Some(Duration::from_secs(30)));
    
    // Test stdio configuration
    assert!(matches!(config.stdin, ProcessIo::Null));
    assert!(matches!(config.stdout, ProcessIo::Pipe));
    assert!(matches!(config.stderr, ProcessIo::Inherit));
}

#[test]
fn test_command_execution() {
    // Test simple command execution
    #[cfg(unix)]
    let result = exec("echo test_output");
    
    #[cfg(windows)]
    let result = exec_with_args("cmd", &["/C", "echo", "test_output"]);
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.success());
    assert!(output.stdout_lossy().contains("test_output"));
}

#[test]
fn test_command_with_timeout() {
    // Test command execution with timeout
    #[cfg(unix)]
    let config = ProcessConfig::new("sleep").arg("1");
    
    #[cfg(windows)]
    let config = ProcessConfig::new("ping")
        .args(&["127.0.0.1", "-n", "2"]);
    
    let start = std::time::Instant::now();
    let result = run_command_timeout(config, Duration::from_millis(500));
    let elapsed = start.elapsed();
    
    // Should timeout
    assert!(result.is_err());
    assert!(elapsed < Duration::from_secs(2)); // Should not take the full command time
}

#[test]
fn test_command_discovery() {
    // Test command existence checking
    #[cfg(unix)]
    {
        assert!(command_exists("ls"));
        assert!(command_exists("echo"));
        assert!(!command_exists("this_command_does_not_exist"));
        
        // Test which command
        let ls_path = which("ls").expect("ls should be found");
        assert!(ls_path.exists());
    }
    
    #[cfg(windows)]
    {
        assert!(command_exists("cmd"));
        assert!(command_exists("echo"));
        assert!(!command_exists("this_command_does_not_exist"));
        
        // Test which command
        let cmd_path = which("cmd").expect("cmd should be found");
        assert!(cmd_path.exists());
    }
}

#[test]
fn test_process_information() {
    let current_pid = get_current_pid();
    assert!(current_pid > 0);
    
    // Test parent PID
    let parent_pid = get_parent_pid().expect("Failed to get parent PID");
    assert!(parent_pid > 0);
    assert_ne!(parent_pid, current_pid);
    
    // Test process running check
    assert!(is_process_running(current_pid));
    assert!(!is_process_running(999999)); // Very unlikely to exist
    
    // Test detailed process information
    let info = get_process_info(current_pid).expect("Failed to get process info");
    assert_eq!(info.pid, current_pid);
    assert!(!info.name.is_empty());
    assert!(info.memory.resident_size > 0);
}

#[test]
fn test_process_memory_and_cpu() {
    let current_pid = get_current_pid();
    
    // Test memory information
    let memory_info = get_process_memory(current_pid)
        .expect("Failed to get memory info");
    assert!(memory_info.resident_size > 0);
    
    // Test CPU information
    let cpu_info = get_process_cpu(current_pid)
        .expect("Failed to get CPU info");
    assert!(cpu_info.total_time >= 0);
}

#[test]
fn test_process_listing() {
    // Test getting process list
    let process_list = get_process_list().expect("Failed to get process list");
    assert!(!process_list.is_empty());
    
    // Check that our current process is in the list
    let current_pid = get_current_pid();
    assert!(process_list.iter().any(|p| p.pid == current_pid));
    
    // Test finding processes by name
    #[cfg(unix)]
    let found_processes = find_processes_by_name("sh")
        .expect("Failed to find processes");
    
    #[cfg(windows)]
    let found_processes = find_processes_by_name("cmd")
        .expect("Failed to find processes");
    
    // Should find at least some processes
    assert!(!found_processes.is_empty());
}

#[test]
fn test_system_information() {
    // Test CPU count
    let cpu_count = get_cpu_count();
    assert!(cpu_count > 0);
    assert!(cpu_count <= 1024); // Reasonable upper bound
    
    // Test system uptime
    let uptime = get_system_uptime().expect("Failed to get system uptime");
    assert!(uptime.as_secs() > 0); // System should have been running for some time
    
    // Test load average (Unix only)
    #[cfg(unix)]
    {
        let (load1, load5, load15) = get_load_average()
            .expect("Failed to get load average");
        assert!(load1 >= 0.0);
        assert!(load5 >= 0.0);
        assert!(load15 >= 0.0);
    }
}

#[test]
fn test_signal_handling() {
    // Spawn a long-running process
    #[cfg(unix)]
    let config = ProcessConfig::new("sleep").arg("10");
    
    #[cfg(windows)]
    let config = ProcessConfig::new("ping")
        .args(&["127.0.0.1", "-n", "20"]);
    
    let mut process = spawn_process(config).expect("Failed to spawn process");
    let pid = process.id();
    
    // Verify process is running
    assert!(is_process_running(pid));
    
    // Test graceful termination
    #[cfg(unix)]
    {
        assert!(process.send_signal(Signal::SIGTERM.as_number()).is_ok());
        thread::sleep(Duration::from_millis(100));
    }
    
    // Kill the process
    assert!(process.terminate().is_ok());
    thread::sleep(Duration::from_millis(500));
    
    // Process should no longer be running
    assert!(!is_process_running(pid));
}

#[test] 
fn test_process_control() {
    // Test ProcessControl static methods
    #[cfg(unix)]
    {
        // Spawn a test process
        let config = ProcessConfig::new("sleep").arg("5");
        let mut process = spawn_process(config).expect("Failed to spawn process");
        let pid = process.id();
        
        // Test sending signals
        assert!(ProcessControl::send_signal(pid, Signal::SIGTERM).is_ok());
        thread::sleep(Duration::from_millis(100));
        
        // Process should be terminated
        assert!(!is_process_running(pid));
    }
}

#[test]
fn test_process_priority() {
    let current_pid = get_current_pid();
    
    // Get current priority
    let current_priority = get_process_priority(current_pid);
    
    // On some systems, we might not have permission to read priority
    if current_priority.is_ok() {
        let priority = current_priority.unwrap();
        
        // Try to set a different priority (might fail due to permissions)
        let _ = set_process_priority(current_pid, Priority::Low);
        
        // Restore original priority (might fail due to permissions)
        let _ = set_process_priority(current_pid, priority);
    }
}

#[test]
fn test_performance_history() {
    let mut history = PerformanceHistory::new(1234, 10);
    
    // Test empty history
    assert!(history.latest().is_none());
    assert_eq!(history.average_cpu_usage(), 0.0);
    assert_eq!(history.average_memory_usage(), 0);
    assert_eq!(history.peak_memory_usage(), 0);
    
    // Add some test metrics
    for i in 1..=5 {
        let metrics = PerformanceMetrics {
            pid: 1234,
            timestamp: std::time::SystemTime::now(),
            cpu_percent: i as f64 * 10.0,
            memory_bytes: i * 1024 * 1024,
            virtual_memory_bytes: i * 2 * 1024 * 1024,
            file_descriptors: i * 10,
            threads: i,
            uptime: Duration::from_secs(i * 60),
            io_read_bytes: 0,
            io_write_bytes: 0,
        };
        history.add_sample(metrics);
    }
    
    // Test calculations
    assert!(history.latest().is_some());
    assert_eq!(history.average_cpu_usage(), 30.0); // (10+20+30+40+50)/5
    assert_eq!(history.peak_memory_usage(), 5 * 1024 * 1024);
    assert_eq!(history.peak_cpu_usage(), 50.0);
    
    // Test degradation detection
    assert!(!history.is_degrading(5.0)); // Not enough samples for trend
    
    // Clear history
    history.clear();
    assert!(history.latest().is_none());
}

#[test]
fn test_process_monitoring() {
    let config = HealthCheckConfig {
        check_interval: Duration::from_millis(100),
        thresholds: ResourceThresholds {
            max_cpu_percent: 50.0,
            max_memory_bytes: 1024 * 1024 * 1024, // 1GB
            max_file_descriptors: 1000,
            max_threads: 100,
            max_execution_time: Some(Duration::from_secs(300)),
        },
        failure_threshold: 2,
        success_threshold: 1,
        check_responsiveness: false,
        responsiveness_timeout: Duration::from_secs(1),
    };
    
    let mut monitor = ProcessMonitor::new(config);
    
    // Add current process to monitoring
    let current_pid = get_current_pid();
    assert!(monitor.add_process(current_pid).is_ok());
    
    // Start monitoring briefly
    assert!(monitor.start().is_ok());
    thread::sleep(Duration::from_millis(200));
    monitor.stop();
    
    // Check monitoring results
    let status = monitor.get_status();
    assert!(status.contains_key(&current_pid));
    
    // Test process details
    let details = monitor.get_process_details(current_pid);
    assert!(details.is_some());
    
    let details = details.unwrap();
    assert_eq!(details.info.pid, current_pid);
    assert!(!matches!(details.health_status, HealthStatus::Unknown));
}

#[test]
fn test_monitor_process_once() {
    let current_pid = get_current_pid();
    let metrics = monitor_process_once(current_pid)
        .expect("Failed to monitor process");
    
    assert_eq!(metrics.pid, current_pid);
    assert!(metrics.memory_bytes > 0);
    assert!(metrics.file_descriptors > 0);
    assert!(metrics.threads >= 1);
}

#[test]
fn test_process_tree() {
    let current_pid = get_current_pid();
    let tree = get_process_tree(current_pid)
        .expect("Failed to get process tree");
    
    // Tree should at least contain the current process
    assert!(tree.iter().any(|p| p.pid == current_pid));
}

#[test]
fn test_error_handling() {
    // Test process not found error
    let result = get_process_info(999999);
    assert!(result.is_err());
    
    if let Err(error) = result {
        assert_eq!(error.category(), "ProcessNotFound");
        assert!(!error.is_recoverable());
    }
    
    // Test invalid command execution
    let result = exec("this_command_definitely_does_not_exist");
    assert!(result.is_err());
    
    if let Err(error) = result {
        assert_eq!(error.category(), "ExecutionFailed");
    }
}

#[test]
fn test_process_io_configuration() {
    // Test ProcessIo variants
    let inherit = ProcessIo::Inherit;
    let pipe = ProcessIo::Pipe;
    let null = ProcessIo::Null;
    
    // Test stdio conversion
    assert!(inherit.to_stdio().is_ok());
    assert!(pipe.to_stdio().is_ok());
    assert!(null.to_stdio().is_ok());
    
    // Test file I/O helpers
    let temp_file = std::env::temp_dir().join("test_process_io.txt");
    std::fs::write(&temp_file, "test content").expect("Failed to write test file");
    
    let file_io = ProcessIo::read_file(&temp_file);
    assert!(file_io.is_ok());
    
    let write_io = ProcessIo::write_file(&temp_file);
    assert!(write_io.is_ok());
    
    // Clean up
    let _ = std::fs::remove_file(&temp_file);
}

#[test]
fn test_process_output_methods() {
    #[cfg(unix)]
    let result = run_command(ProcessConfig::new("echo").arg("test_output"));
    
    #[cfg(windows)]
    let result = run_command(ProcessConfig::new("cmd").args(&["/C", "echo", "test_output"]));
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    assert!(output.success());
    assert!(output.exit_code().unwrap_or(-1) == 0);
    assert!(output.stdout_string().is_ok());
    assert!(output.stderr_string().is_ok());
    assert!(output.stdout_lossy().contains("test_output"));
    assert!(output.duration.as_millis() > 0);
}

#[test]
fn test_concurrent_process_management() {
    let mut handles = Vec::new();
    
    // Spawn multiple concurrent processes
    for i in 0..5 {
        let handle = thread::spawn(move || {
            #[cfg(unix)]
            let config = ProcessConfig::new("echo").arg(&format!("Process {}", i));
            
            #[cfg(windows)]
            let config = ProcessConfig::new("cmd")
                .args(&["/C", "echo", &format!("Process {}", i)]);
            
            let result = run_command(config);
            assert!(result.is_ok());
            
            let output = result.unwrap();
            assert!(output.success());
            assert!(output.stdout_lossy().contains(&format!("Process {}", i)));
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        assert!(handle.join().is_ok());
    }
}

#[test]
fn test_resource_thresholds() {
    let thresholds = ResourceThresholds::default();
    
    assert_eq!(thresholds.max_cpu_percent, 80.0);
    assert_eq!(thresholds.max_memory_bytes, 1024 * 1024 * 1024); // 1GB
    assert_eq!(thresholds.max_file_descriptors, 1000);
    assert_eq!(thresholds.max_threads, 100);
    assert_eq!(thresholds.max_execution_time, None);
    
    let config = HealthCheckConfig::default();
    assert_eq!(config.check_interval, Duration::from_secs(30));
    assert_eq!(config.failure_threshold, 3);
    assert_eq!(config.success_threshold, 2);
    assert!(config.check_responsiveness);
    assert_eq!(config.responsiveness_timeout, Duration::from_secs(5));
}

#[test]
fn test_signal_properties() {
    let sigterm = Signal::SIGTERM;
    assert_eq!(sigterm.as_number(), 15);
    assert_eq!(sigterm.name(), "SIGTERM");
    assert!(sigterm.can_be_caught());
    assert!(sigterm.is_terminating());
    
    let sigkill = Signal::SIGKILL;
    assert_eq!(sigkill.as_number(), 9);
    assert_eq!(sigkill.name(), "SIGKILL");
    assert!(!sigkill.can_be_caught());
    assert!(sigkill.is_terminating());
    
    let sigcont = Signal::SIGCONT;
    assert!(!sigcont.is_terminating());
    assert!(sigcont.can_be_caught());
}

#[test]
fn test_priority_levels() {
    let normal = Priority::Normal;
    assert_eq!(normal.nice_value(), 0);
    
    let high = Priority::High;
    assert_eq!(high.nice_value(), -10);
    
    let low = Priority::Low;
    assert_eq!(low.nice_value(), 10);
    
    // Test priority ordering
    assert!(Priority::VeryHigh < Priority::High);
    assert!(Priority::High < Priority::Normal);
    assert!(Priority::Normal < Priority::Low);
    
    // Test from_nice conversion
    assert_eq!(Priority::from_nice(-15), Priority::High);
    assert_eq!(Priority::from_nice(0), Priority::Normal);
    assert_eq!(Priority::from_nice(15), Priority::Low);
}
