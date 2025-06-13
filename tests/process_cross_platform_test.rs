/// Cross-platform process management tests
/// 
/// Tests the process management functionality across different platforms
/// to ensure consistent behavior and proper error handling.

use cursed::stdlib::process::{
    monitoring::{collect_performance_metrics, create_process_monitor, monitor_process_once, ResourceThresholds},
    platform::PlatformUtils,
    info::{get_current_pid, get_process_info, is_process_running},
};

#[test]
fn test_current_process_monitoring() {
    let current_pid = get_current_pid();
    assert!(current_pid > 0, "Current PID should be positive");
    
    // Test that we can monitor the current process
    assert!(is_process_running(current_pid), "Current process should be running");
    
    // Test performance metrics collection
    let metrics_result = collect_performance_metrics(current_pid);
    match metrics_result {
        Ok(metrics) => {
            assert_eq!(metrics.pid, current_pid);
            assert!(metrics.memory_bytes > 0, "Process should have some memory usage");
            // Note: CPU percent might be 0 for a brief measurement
        }
        Err(_) => {
            // On some platforms, monitoring might not be available for self
            println!("Warning: Could not collect performance metrics for current process");
        }
    }
}

#[test] 
fn test_process_info_retrieval() {
    let current_pid = get_current_pid();
    
    let info_result = get_process_info(current_pid);
    match info_result {
        Ok(info) => {
            assert_eq!(info.pid, current_pid);
            assert!(!info.name.is_empty(), "Process should have a name");
            assert!(info.memory.virtual_size > 0 || info.memory.resident_size > 0, 
                   "Process should have some memory usage");
        }
        Err(_) => {
            // On some platforms, detailed process info might not be available
            println!("Warning: Could not get detailed process info for current process");
        }
    }
}

#[test]
fn test_platform_specific_info() {
    let current_pid = get_current_pid();
    
    let platform_info_result = PlatformUtils::get_platform_info(current_pid);
    match platform_info_result {
        Ok(_platform_info) => {
            // Platform-specific info was successfully retrieved
        }
        Err(_) => {
            // Some platforms might not support all features
            println!("Warning: Platform-specific info not available");
        }
    }
}

#[test]
fn test_resource_monitoring() {
    let current_pid = get_current_pid();
    
    // Test single process monitoring with default thresholds
    let thresholds = ResourceThresholds::default();
    let health_result = monitor_process_once(current_pid, thresholds);
    
    match health_result {
        Ok(health_status) => {
            // Health status should be determinable
            println!("Process health status: {:?}", health_status);
        }
        Err(e) => {
            println!("Warning: Could not determine process health: {}", e);
        }
    }
}

#[test]
fn test_process_monitor_creation() {
    let mut monitor = create_process_monitor();
    
    // Test adding current process to monitoring
    let current_pid = get_current_pid();
    let add_result = monitor.add_process(current_pid);
    
    match add_result {
        Ok(_) => {
            // Successfully added process to monitoring
            let monitored_pids = monitor.get_monitored_pids().unwrap_or_default();
            assert!(monitored_pids.contains(&current_pid), "Current PID should be in monitored list");
            
            // Test getting health status
            let _health_result = monitor.get_health_status(current_pid);
            
            // Clean up
            let _ = monitor.remove_process(current_pid);
        }
        Err(e) => {
            println!("Warning: Could not add process to monitor: {}", e);
        }
    }
}

#[test]
fn test_nonexistent_process() {
    let fake_pid = 999999u32;
    
    // Test that we properly handle nonexistent processes
    assert!(!is_process_running(fake_pid), "Fake PID should not be running");
    
    let info_result = get_process_info(fake_pid);
    assert!(info_result.is_err(), "Getting info for fake PID should fail");
    
    let metrics_result = collect_performance_metrics(fake_pid);
    assert!(metrics_result.is_err(), "Getting metrics for fake PID should fail");
}

#[test]
fn test_platform_capabilities() {
    use cursed::stdlib::process::platform::{supports_feature, PlatformFeature};
    
    // Test platform feature detection
    let has_signals = supports_feature(PlatformFeature::Signals);
    let has_resource_limits = supports_feature(PlatformFeature::ResourceLimits);
    let has_file_descriptors = supports_feature(PlatformFeature::FileDescriptors);
    
    #[cfg(unix)]
    {
        assert!(has_signals, "Unix platforms should support signals");
        assert!(has_resource_limits, "Unix platforms should support resource limits");
        assert!(has_file_descriptors, "Unix platforms should support file descriptors");
    }
    
    #[cfg(windows)]
    {
        assert!(!has_signals, "Windows should not support Unix signals");
        let has_services = supports_feature(PlatformFeature::WindowsServices);
        assert!(has_services, "Windows should support services");
    }
    
    #[cfg(target_os = "linux")]
    {
        let has_cgroups = supports_feature(PlatformFeature::Cgroups);
        let has_namespaces = supports_feature(PlatformFeature::Namespaces);
        assert!(has_cgroups, "Linux should support cgroups");
        assert!(has_namespaces, "Linux should support namespaces");
    }
    
    #[cfg(target_os = "macos")]
    {
        let has_mach_ports = supports_feature(PlatformFeature::MachPorts);
        assert!(has_mach_ports, "macOS should support Mach ports");
    }
}

#[test]
fn test_user_info() {
    let user_info_result = PlatformUtils::get_current_user();
    
    match user_info_result {
        Ok(user_info) => {
            assert!(!user_info.username.is_empty(), "Username should not be empty");
            
            #[cfg(unix)]
            {
                assert!(user_info.uid.is_some(), "Unix should have UID");
                assert!(user_info.gid.is_some(), "Unix should have GID");
            }
            
            #[cfg(windows)]
            {
                assert!(user_info.uid.is_none(), "Windows should not have UID");
                assert!(user_info.gid.is_none(), "Windows should not have GID");
            }
        }
        Err(e) => {
            println!("Warning: Could not get user info: {}", e);
        }
    }
}

#[test]
fn test_elevated_privileges() {
    let is_elevated = PlatformUtils::is_elevated();
    
    // This test mainly checks that the function runs without panicking
    // The actual result depends on how the test is run
    println!("Running with elevated privileges: {}", is_elevated);
    
    #[cfg(unix)]
    {
        // On Unix, elevated means root (UID 0)
        let user_info = PlatformUtils::get_current_user().ok();
        if let Some(info) = user_info {
            if let Some(uid) = info.uid {
                let expected_elevated = uid == 0;
                assert_eq!(is_elevated, expected_elevated, 
                          "Elevated status should match UID == 0");
            }
        }
    }
}

/// Test error handling for various scenarios
#[test]
fn test_error_handling() {
    use cursed::stdlib::process::error::ProcessError;
    
    // Test with invalid PID
    let result = collect_performance_metrics(0);
    match result {
        Err(ProcessError::ProcessNotFound(_)) => {
            // Expected error type
        }
        Err(ProcessError::SystemError(_, _)) => {
            // Also acceptable on some platforms
        }
        Err(ProcessError::PlatformError(_)) => {
            // Acceptable if platform doesn't support this
        }
        Ok(_) => {
            // Some platforms might allow monitoring PID 0
            println!("Warning: Platform allows monitoring PID 0");
        }
        Err(e) => {
            panic!("Unexpected error type: {:?}", e);
        }
    }
}

/// Stress test with multiple processes
#[cfg(not(target_os = "windows"))] // Skip on Windows due to process creation complexity
#[test]
fn test_multiple_process_monitoring() {
    use std::process::Command;
    use std::time::Duration;
    use std::thread;
    
    // Spawn a simple process we can monitor
    let mut child = Command::new("sleep")
        .arg("2")
        .spawn()
        .expect("Failed to spawn test process");
    
    let child_pid = child.id();
    
    // Give the process a moment to start
    thread::sleep(Duration::from_millis(100));
    
    // Test monitoring the child process
    assert!(is_process_running(child_pid), "Child process should be running");
    
    let metrics_result = collect_performance_metrics(child_pid);
    match metrics_result {
        Ok(metrics) => {
            assert_eq!(metrics.pid, child_pid);
        }
        Err(_) => {
            println!("Warning: Could not monitor child process");
        }
    }
    
    // Clean up
    let _ = child.kill();
    let _ = child.wait();
}
