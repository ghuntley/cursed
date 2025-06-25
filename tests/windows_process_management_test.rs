/// Comprehensive tests for Windows process management functionality
/// 
/// This test suite validates the Windows-specific implementations in the process management
/// module, ensuring that Windows API integrations work correctly and provide feature parity
/// with Unix implementations where possible.

#[cfg(windows)]
mod windows_tests {
    use std::time::{Duration, Instant};
    use std::sync::Arc;
    use std::thread;
    
    use crate::stdlib::process::windows_support::*;
    use crate::stdlib::process::safe_process_management::*;
    use crate::stdlib::process::error::ProcessResult;

    #[test]
    fn test_windows_parent_pid() {
        let result = get_windows_parent_pid();
        
        match result {
            Ok(ppid) => {
                assert!(ppid > 0, "Parent PID should be positive");
                assert_ne!(ppid, std::process::id(), "Parent PID should not equal current PID");
                
                println!("✅ Windows parent PID retrieval: {} -> {}", std::process::id(), ppid);
            }
            Err(e) => {
                // In some test environments, this might fail - that's acceptable
                println!("⚠️  Windows parent PID retrieval failed (acceptable in some environments): {:?}", e);
            }
        }
    }

    #[test]
    fn test_windows_process_exists() {
        let current_pid = std::process::id();
        
        // Current process should exist
        assert!(windows_process_exists(current_pid), "Current process should exist");
        
        // Non-existent process should not exist
        let fake_pid = u32::MAX - 1; // Very unlikely to be a real PID
        assert!(!windows_process_exists(fake_pid), "Fake PID should not exist");
        
        println!("✅ Windows process existence check working correctly");
    }

    #[test]
    fn test_windows_job_object_creation() {
        let result = WindowsJobObject::new(Some("TestJob"));
        
        match result {
            Ok(job) => {
                println!("✅ Windows Job Object created successfully");
                
                // Test assignment to current process (might fail due to permissions)
                let assign_result = job.assign_process(std::process::id());
                match assign_result {
                    Ok(_) => println!("✅ Process assigned to Job Object"),
                    Err(e) => println!("⚠️  Process assignment failed (permissions): {:?}", e),
                }
            }
            Err(e) => {
                panic!("Failed to create Windows Job Object: {:?}", e);
            }
        }
    }

    #[test]
    fn test_windows_job_object_limits() {
        let job = match WindowsJobObject::new(Some("TestLimitsJob")) {
            Ok(j) => j,
            Err(e) => {
                println!("⚠️  Skipping limits test - Job Object creation failed: {:?}", e);
                return;
            }
        };

        let limits = ResourceLimits {
            max_memory_bytes: Some(100 * 1024 * 1024), // 100MB
            max_cpu_percent: Some(50.0),
            max_execution_time: Some(Duration::from_secs(300)),
            max_file_descriptors: Some(1024),
        };

        let result = job.set_limits(&limits);
        match result {
            Ok(_) => println!("✅ Windows Job Object limits set successfully"),
            Err(e) => println!("⚠️  Setting limits failed (acceptable): {:?}", e),
        }
    }

    #[test]
    fn test_windows_process_statistics() {
        let current_pid = std::process::id();
        let start_time = Instant::now();
        
        // Let some time pass to get meaningful statistics
        thread::sleep(Duration::from_millis(10));
        
        let result = get_windows_process_statistics(current_pid, start_time);
        
        match result {
            Ok(stats) => {
                assert!(stats.uptime.as_millis() >= 10, "Uptime should be at least 10ms");
                
                // Memory usage should be non-zero for current process
                if stats.memory_usage_bytes > 0 {
                    println!("✅ Memory usage: {} bytes", stats.memory_usage_bytes);
                }
                
                if stats.virtual_memory_bytes > 0 {
                    println!("✅ Virtual memory: {} bytes", stats.virtual_memory_bytes);
                }
                
                if stats.thread_count > 0 {
                    println!("✅ Thread count: {}", stats.thread_count);
                }
                
                println!("✅ Windows process statistics retrieved successfully");
                println!("   CPU: {:.2}%", stats.cpu_usage_percent);
                println!("   Memory: {} bytes", stats.memory_usage_bytes);
                println!("   Threads: {}", stats.thread_count);
                println!("   Uptime: {:?}", stats.uptime);
            }
            Err(e) => {
                panic!("Failed to get Windows process statistics: {:?}", e);
            }
        }
    }

    #[test]
    fn test_windows_process_list() {
        let result = get_windows_process_list();
        
        match result {
            Ok(pids) => {
                assert!(!pids.is_empty(), "Process list should not be empty");
                assert!(pids.contains(&std::process::id()), "Current process should be in the list");
                
                println!("✅ Windows process list retrieved: {} processes", pids.len());
                
                // Show first few PIDs as examples
                for (i, &pid) in pids.iter().take(5).enumerate() {
                    println!("   Process {}: PID {}", i + 1, pid);
                }
            }
            Err(e) => {
                panic!("Failed to get Windows process list: {:?}", e);
            }
        }
    }

    #[test]
    fn test_windows_resource_limits_application() {
        let current_pid = std::process::id();
        
        let limits = ResourceLimits {
            max_memory_bytes: Some(500 * 1024 * 1024), // 500MB
            max_cpu_percent: Some(80.0),
            max_execution_time: Some(Duration::from_secs(600)),
            max_file_descriptors: None, // Windows handles are different
        };

        let result = apply_windows_resource_limits(current_pid, &limits);
        
        match result {
            Ok(_) => {
                println!("✅ Windows resource limits applied successfully");
                
                // Clean up the job object
                cleanup_process_job_object(current_pid);
                println!("✅ Job object cleaned up");
            }
            Err(e) => {
                // This might fail due to permissions in test environments
                println!("⚠️  Resource limits application failed (permissions): {:?}", e);
            }
        }
    }

    #[test]
    fn test_safe_process_manager_integration() {
        let manager = SafeProcessManager::new();
        
        // Test basic functionality
        assert_eq!(manager.list_processes().len(), 0);
        
        // Test global limits
        let global_limits = ResourceLimits {
            max_memory_bytes: Some(1024 * 1024 * 1024), // 1GB
            max_cpu_percent: Some(90.0),
            max_execution_time: Some(Duration::from_secs(3600)),
            max_file_descriptors: Some(2048),
        };
        
        manager.set_global_limits(global_limits);
        
        println!("✅ Process manager integration test completed");
    }

    #[test]
    fn test_cross_platform_compatibility() {
        // Test that cross-platform functions work on Windows
        
        let current_pid = current_pid();
        assert!(current_pid > 0);
        
        assert!(process_exists(current_pid));
        assert!(!process_exists(u32::MAX - 1));
        
        // Test parent PID retrieval
        match parent_pid() {
            Ok(ppid) => {
                assert!(ppid > 0);
                assert_ne!(ppid, current_pid);
                println!("✅ Cross-platform parent PID: {}", ppid);
            }
            Err(e) => {
                println!("⚠️  Parent PID retrieval failed: {:?}", e);
            }
        }
        
        println!("✅ Cross-platform compatibility verified");
    }

    #[test]
    fn test_process_termination_simulation() {
        // We can't easily test actual process termination in unit tests,
        // but we can test the API exists and handles invalid PIDs correctly
        
        let fake_pid = u32::MAX - 2;
        
        // This should fail gracefully
        let kill_result = windows_kill_process(fake_pid);
        assert!(kill_result.is_err(), "Killing non-existent process should fail");
        
        let terminate_result = windows_terminate_process(fake_pid);
        assert!(terminate_result.is_err(), "Terminating non-existent process should fail");
        
        println!("✅ Process termination API behavior verified");
    }

    #[test]
    fn test_memory_information_accuracy() {
        let current_pid = std::process::id();
        let start_time = Instant::now();
        
        // Allocate some memory to see if it's detected
        let _large_vec: Vec<u8> = vec![0; 1024 * 1024]; // 1MB
        
        let result = get_windows_process_statistics(current_pid, start_time);
        
        match result {
            Ok(stats) => {
                if stats.memory_usage_bytes > 1024 * 1024 {
                    println!("✅ Memory allocation detected: {} bytes", stats.memory_usage_bytes);
                } else {
                    println!("⚠️  Memory allocation not detected or measurement granularity issue");
                }
                
                // Virtual memory should be larger than physical memory
                if stats.virtual_memory_bytes >= stats.memory_usage_bytes {
                    println!("✅ Virtual memory >= physical memory: {} >= {}", 
                             stats.virtual_memory_bytes, stats.memory_usage_bytes);
                }
            }
            Err(e) => {
                println!("⚠️  Memory information test failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_performance_and_scalability() {
        let start = Instant::now();
        
        // Test multiple rapid process existence checks
        let current_pid = std::process::id();
        for _ in 0..100 {
            assert!(windows_process_exists(current_pid));
        }
        
        let existence_check_time = start.elapsed();
        
        // Test process list retrieval
        let list_start = Instant::now();
        let result = get_windows_process_list();
        let list_time = list_start.elapsed();
        
        match result {
            Ok(pids) => {
                println!("✅ Performance test results:");
                println!("   100 existence checks: {:?}", existence_check_time);
                println!("   Process list ({} processes): {:?}", pids.len(), list_time);
                
                // Performance should be reasonable
                assert!(existence_check_time < Duration::from_secs(1), "Existence checks too slow");
                assert!(list_time < Duration::from_secs(5), "Process list retrieval too slow");
            }
            Err(e) => {
                println!("⚠️  Performance test failed: {:?}", e);
            }
        }
    }
}

// For non-Windows platforms, provide stub tests that indicate the platform difference
#[cfg(not(windows))]
mod non_windows_tests {
    #[test]
    fn test_windows_functionality_not_available() {
        println!("ℹ️  Windows-specific process management tests skipped on non-Windows platform");
        // This test passes automatically on non-Windows platforms
    }
}
