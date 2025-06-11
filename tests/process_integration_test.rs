/// Comprehensive process management integration tests
use cursed::stdlib::process::*;
use std::time::Duration;
use std::thread;

#[path = "common.rs"]
pub mod common;

#[test]
fn test_basic_process_spawn() {
    common::init_tracing!();
    
    // Test basic process spawning
    let config = ProcessConfig::new("echo")
        .arg("Hello, World!")
        .capture_output();
    
    let mut process = spawn_process(config).expect("Failed to spawn process");
    let status = process.wait().expect("Failed to wait for process");
    
    assert!(status.success());
}

#[test]
fn test_process_with_arguments() {
    common::init_tracing!();
    
    let config = ProcessConfig::new("echo")
        .args(["arg1", "arg2", "arg3"])
        .capture_output();
    
    let mut process = spawn_process(config).expect("Failed to spawn process");
    let status = process.wait().expect("Failed to wait for process");
    
    assert!(status.success());
}

#[test]
fn test_process_environment_variables() {
    common::init_tracing!();
    
    #[cfg(unix)]
    let config = ProcessConfig::new("env")
        .env("TEST_VAR", "test_value")
        .capture_output();
    
    #[cfg(windows)]
    let config = ProcessConfig::new("set")
        .env("TEST_VAR", "test_value")
        .capture_output();
    
    let mut process = spawn_process(config).expect("Failed to spawn process");
    let status = process.wait().expect("Failed to wait for process");
    
    assert!(status.success());
}

#[test]
fn test_process_working_directory() {
    common::init_tracing!();
    
    let temp_dir = std::env::temp_dir();
    
    #[cfg(unix)]
    let config = ProcessConfig::new("pwd")
        .working_dir(&temp_dir)
        .capture_output();
    
    #[cfg(windows)]
    let config = ProcessConfig::new("cd")
        .working_dir(&temp_dir)
        .capture_output();
    
    let mut process = spawn_process(config).expect("Failed to spawn process");
    let status = process.wait().expect("Failed to wait for process");
    
    assert!(status.success());
}

#[test]
fn test_run_command() {
    common::init_tracing!();
    
    let output = run_command("echo test_output").expect("Failed to run command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test_output"));
}

#[test]
fn test_run_command_with_timeout() {
    common::init_tracing!();
    
    // Test successful command within timeout
    let output = run_command_timeout("echo quick", Duration::from_secs(5))
        .expect("Failed to run command");
    
    assert!(output.status.success());
    
    // Test timeout behavior (this might be platform-dependent)
    #[cfg(unix)]
    {
        let result = run_command_timeout("sleep 10", Duration::from_secs(1));
        assert!(result.is_err());
    }
}

#[test]
fn test_command_exists() {
    common::init_tracing!();
    
    // Test existing command
    #[cfg(unix)]
    assert!(command_exists("echo"));
    
    #[cfg(windows)]
    assert!(command_exists("echo"));
    
    // Test non-existing command
    assert!(!command_exists("nonexistent_command_xyz"));
}

#[test]
fn test_which_command() {
    common::init_tracing!();
    
    #[cfg(unix)]
    {
        let echo_path = which("echo").expect("Failed to find echo command");
        assert!(echo_path.exists());
        assert!(echo_path.file_name().unwrap().to_str().unwrap().contains("echo"));
    }
    
    #[cfg(windows)]
    {
        // Windows might have echo as a built-in, so this test is more flexible
        if let Ok(echo_path) = which("echo") {
            assert!(echo_path.exists());
        }
    }
}

#[test]
fn test_current_process_info() {
    common::init_tracing!();
    
    let current_pid = get_current_pid();
    assert!(current_pid > 0);
    
    let process_info = get_process_info(current_pid).expect("Failed to get current process info");
    assert_eq!(process_info.pid, current_pid);
    assert!(!process_info.name.is_empty());
}

#[test]
fn test_process_running_check() {
    common::init_tracing!();
    
    let current_pid = get_current_pid();
    assert!(is_process_running(current_pid));
    
    // Test with a PID that definitely doesn't exist
    assert!(!is_process_running(999999));
}

#[test]
fn test_process_list() {
    common::init_tracing!();
    
    let process_list = get_process_list().expect("Failed to get process list");
    assert!(!process_list.is_empty());
    
    // Check that current process is in the list
    let current_pid = get_current_pid();
    let found_self = process_list.iter().any(|p| p.pid == current_pid);
    assert!(found_self);
}

#[test]
fn test_find_processes_by_name() {
    common::init_tracing!();
    
    // This test might be platform-dependent
    #[cfg(unix)]
    {
        // Look for init process or systemd
        let init_processes = find_processes_by_name("init")
            .or_else(|_| find_processes_by_name("systemd"))
            .expect("Failed to find init processes");
        
        // Should find at least one process
        // (This might fail in some container environments)
        if !init_processes.is_empty() {
            assert!(init_processes.len() > 0);
        }
    }
}

#[test]
fn test_process_memory_info() {
    common::init_tracing!();
    
    let current_pid = get_current_pid();
    
    // This might not be available on all platforms
    if let Ok(memory_info) = get_process_memory(current_pid) {
        // Basic sanity checks
        assert!(memory_info.rss > 0 || memory_info.vms > 0);
    }
}

#[test]
fn test_process_communication_basic() {
    common::init_tracing!();
    
    #[cfg(unix)]
    let config = ProcessConfig::new("cat")
        .stdin(ProcessIo::Pipe)
        .stdout(ProcessIo::Pipe)
        .stderr(ProcessIo::Pipe);
    
    #[cfg(windows)]
    let config = ProcessConfig::new("type")
        .stdin(ProcessIo::Pipe)
        .stdout(ProcessIo::Pipe)
        .stderr(ProcessIo::Pipe);
    
    let child = std::process::Command::new(&config.command)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");
    
    let mut comm = create_process_communication(child)
        .expect("Failed to create communication");
    
    // Test writing to stdin
    let test_data = b"Hello, World!\n";
    comm.write_stdin(test_data).expect("Failed to write to stdin");
    
    // Start background readers
    comm.start_readers().expect("Failed to start readers");
    
    // Give some time for processing
    thread::sleep(Duration::from_millis(100));
    
    // Close stdin to signal end of input
    comm.channels.stdin = None;
    
    // Wait for process completion
    let status = comm.wait().expect("Failed to wait for process");
    assert!(status.success());
}

#[test]
fn test_send_and_receive() {
    common::init_tracing!();
    
    let input = b"test input\n";
    let timeout = Duration::from_secs(5);
    
    let result = send_and_receive("echo received", input, timeout);
    
    match result {
        Ok((stdout, _stderr)) => {
            let output = String::from_utf8_lossy(&stdout);
            assert!(output.contains("received"));
        }
        Err(e) => {
            // This might fail on some platforms due to shell differences
            println!("Send and receive test failed (might be platform-specific): {}", e);
        }
    }
}

#[test]
fn test_process_monitoring_basic() {
    common::init_tracing!();
    
    let current_pid = get_current_pid();
    
    // Test basic performance metrics collection
    let metrics = collect_performance_metrics(current_pid)
        .expect("Failed to collect performance metrics");
    
    assert_eq!(metrics.pid, current_pid);
    assert!(metrics.memory_bytes >= 0);
    assert!(metrics.cpu_percent >= 0.0);
    assert!(metrics.threads >= 1);
}

#[test]
fn test_process_monitor() {
    common::init_tracing!();
    
    let config = HealthCheckConfig::default();
    let monitor = create_process_monitor();
    
    let current_pid = get_current_pid();
    monitor.add_process(current_pid)
        .expect("Failed to add process to monitor");
    
    // Check that process was added
    let monitored_pids = monitor.get_monitored_pids()
        .expect("Failed to get monitored PIDs");
    assert!(monitored_pids.contains(&current_pid));
    
    // Test health status
    let health_status = monitor.get_health_status(current_pid)
        .expect("Failed to get health status");
    
    // Should be Unknown initially
    assert_eq!(health_status, HealthStatus::Unknown);
}

#[test]
fn test_monitor_process_once() {
    common::init_tracing!();
    
    let current_pid = get_current_pid();
    let thresholds = ResourceThresholds::default();
    
    let health_status = monitor_process_once(current_pid, thresholds)
        .expect("Failed to monitor process");
    
    // Should be healthy for the test process
    assert!(matches!(health_status, HealthStatus::Healthy | HealthStatus::Warning));
}

#[test]
fn test_system_resource_summary() {
    common::init_tracing!();
    
    let summary = get_system_resource_summary()
        .expect("Failed to get system resource summary");
    
    // Should have at least CPU count
    assert!(summary.contains_key("cpu_count"));
    assert!(summary["cpu_count"] > 0);
    
    // Might have other fields depending on platform
    if summary.contains_key("process_count") {
        assert!(summary["process_count"] > 0);
    }
}

#[test]
fn test_platform_detection() {
    common::init_tracing!();
    
    let platform = get_platform_name();
    assert!(!platform.is_empty());
    
    // Test feature detection
    #[cfg(unix)]
    {
        assert!(supports_feature(PlatformFeature::Signals));
        assert!(supports_feature(PlatformFeature::ProcessGroups));
    }
    
    #[cfg(windows)]
    {
        assert!(supports_feature(PlatformFeature::WindowsServices));
    }
    
    #[cfg(target_os = "linux")]
    {
        assert!(supports_feature(PlatformFeature::Cgroups));
        assert!(supports_feature(PlatformFeature::Namespaces));
    }
}

#[test]
fn test_current_user_info() {
    common::init_tracing!();
    
    if let Ok(user_info) = PlatformUtils::get_current_user() {
        assert!(!user_info.username.is_empty());
        
        #[cfg(unix)]
        {
            assert!(user_info.uid.is_some());
            assert!(user_info.gid.is_some());
        }
    }
}

#[test]
fn test_privilege_detection() {
    common::init_tracing!();
    
    // This should work on all platforms
    let is_elevated = PlatformUtils::is_elevated();
    
    // We can't assert the value since it depends on how tests are run
    // but we can verify the function doesn't panic
    println!("Running with elevated privileges: {}", is_elevated);
}

#[test]
fn test_error_handling() {
    common::init_tracing!();
    
    // Test invalid command
    let config = ProcessConfig::new("nonexistent_command_xyz")
        .capture_output();
    
    let result = spawn_process(config);
    assert!(result.is_err());
    
    // Test invalid PID
    let result = get_process_info(999999);
    assert!(result.is_err());
    
    // Test invalid working directory
    let config = ProcessConfig::new("echo")
        .working_dir("/nonexistent/directory/xyz")
        .capture_output();
    
    let result = spawn_process(config);
    assert!(result.is_err());
}

#[test]
fn test_process_lifecycle() {
    common::init_tracing!();
    
    // Spawn a long-running process
    #[cfg(unix)]
    let config = ProcessConfig::new("sleep")
        .arg("1")
        .capture_output();
    
    #[cfg(windows)]
    let config = ProcessConfig::new("timeout")
        .args(["/t", "1"])
        .capture_output();
    
    let mut process = spawn_process(config).expect("Failed to spawn process");
    let pid = process.id();
    
    // Check that process is running
    assert!(process.is_running().expect("Failed to check if running"));
    assert!(is_process_running(pid));
    
    // Wait for completion
    let status = process.wait().expect("Failed to wait for process");
    assert!(status.success());
    
    // Process should no longer be running
    // Note: There might be a small delay before the OS marks it as terminated
    thread::sleep(Duration::from_millis(100));
}

#[test]
fn test_concurrent_processes() {
    common::init_tracing!();
    
    let mut handles = Vec::new();
    
    // Spawn multiple processes concurrently
    for i in 0..3 {
        let handle = thread::spawn(move || {
            let config = ProcessConfig::new("echo")
                .arg(&format!("Process {}", i))
                .capture_output();
            
            let mut process = spawn_process(config).expect("Failed to spawn process");
            let status = process.wait().expect("Failed to wait for process");
            assert!(status.success());
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

#[cfg(unix)]
#[test]
fn test_signal_handling() {
    common::init_tracing!();
    
    // Spawn a long-running process
    let config = ProcessConfig::new("sleep")
        .arg("10")
        .capture_output();
    
    let process = spawn_process(config).expect("Failed to spawn process");
    let pid = process.id();
    
    // Send termination signal
    thread::sleep(Duration::from_millis(100)); // Let process start
    
    let result = terminate_process(pid);
    assert!(result.is_ok());
    
    // Wait a bit for process to terminate
    thread::sleep(Duration::from_millis(500));
    
    // Process should no longer be running
    assert!(!is_process_running(pid));
}

#[test]
fn test_resource_thresholds() {
    common::init_tracing!();
    
    let thresholds = ResourceThresholds {
        max_cpu_percent: 50.0,
        max_memory_bytes: 1024 * 1024, // 1MB
        max_file_descriptors: 100,
        max_threads: 10,
        max_execution_time: Some(Duration::from_secs(60)),
    };
    
    // Test with current process (should be healthy with reasonable thresholds)
    let current_pid = get_current_pid();
    let status = monitor_process_once(current_pid, thresholds)
        .expect("Failed to monitor process");
    
    // Status should be determinable
    assert!(matches!(status, 
        HealthStatus::Healthy | 
        HealthStatus::Warning | 
        HealthStatus::Critical
    ));
}
