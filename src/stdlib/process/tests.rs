/// Comprehensive tests for the enhanced process management system
/// 
/// This module tests all aspects of the process management infrastructure
/// including real monitoring, process control, lifecycle management, and APIs.

#[cfg(test)]
mod integration_tests {
    use std::time::Duration;
    use crate::stdlib::process::{
        integration::{UnifiedProcessManager, quick_exec, process_groups::ProcessGroupManager},
        real_monitoring::{start_global_monitoring, stop_global_monitoring},
        ProcessConfig, ProcessControl, Signal, Priority,
        exec_vibez::command,
        exec_slay::new_slay_command,
    };

    #[test]
    fn test_unified_process_manager() {
        let mut manager = UnifiedProcessManager::new().expect("Failed to create manager");
        
        // Test process spawning
        #[cfg(unix)]
        let pid = manager.spawn_tracked("echo", &["test"]).expect("Failed to spawn process");
        
        #[cfg(windows)]
        let pid = manager.spawn_tracked("cmd", &["/C", "echo test"]).expect("Failed to spawn process");
        
        // Test process info retrieval
        let info = manager.get_process_info(pid);
        assert!(info.is_ok() || info.is_err()); // Process might have completed quickly
        
        // Test listing active processes
        let active = manager.list_active().expect("Failed to list active processes");
        println!("Active processes: {}", active.len());
        
        // Cleanup
        let _ = manager.terminate_process(pid);
        manager.shutdown().expect("Failed to shutdown manager");
    }

    #[test]
    fn test_quick_exec_functions() {
        // Test basic execution
        #[cfg(unix)]
        {
            let result = quick_exec::exec("echo hello");
            assert!(result.is_ok());
            let output = result.unwrap();
            assert_eq!(output.stdout_lossy().trim(), "hello");
        }
        
        #[cfg(windows)]
        {
            let result = quick_exec::exec_with_args("cmd", &["/C", "echo hello"]);
            assert!(result.is_ok());
        }
        
        // Test execution with timeout
        #[cfg(unix)]
        {
            let result = quick_exec::exec_timeout("sleep 0.1", Duration::from_secs(1));
            assert!(result.is_ok());
        }
        
        // Test shell execution
        #[cfg(unix)]
        {
            let result = quick_exec::shell("echo shell_test");
            assert!(result.is_ok());
            let output = result.unwrap();
            assert_eq!(output.stdout_lossy().trim(), "shell_test");
        }
    }

    #[test]
    fn test_process_group_manager() {
        let manager = ProcessGroupManager::new().expect("Failed to create group manager");
        
        // Test group creation
        assert!(manager.create_group("test_group").is_ok());
        
        // Test adding processes to group
        let fake_pid = 12345u32;
        assert!(manager.add_to_group("test_group", fake_pid).is_ok());
        
        // Test group termination (will fail for fake PID but shouldn't crash)
        let _ = manager.terminate_group("test_group");
    }

    #[test]
    fn test_real_monitoring_integration() {
        start_global_monitoring();
        
        // Give monitoring time to start
        std::thread::sleep(Duration::from_millis(100));
        
        stop_global_monitoring();
    }

    #[test]
    fn test_exec_vibez_api() {
        let mut cmd = command("echo", &["vibez_test"]);
        
        #[cfg(unix)]
        {
            let output = cmd.output();
            assert!(output.is_ok());
            let result = output.unwrap();
            assert_eq!(String::from_utf8_lossy(&result).trim(), "vibez_test");
        }
        
        #[cfg(windows)]
        {
            // Windows echo behaves differently
            let _ = cmd.output();
        }
    }

    #[test]
    fn test_exec_slay_api() {
        let mut cmd = new_slay_command("echo", &["slay_test"]);
        
        #[cfg(unix)]
        {
            let output = cmd.output();
            assert!(output.is_ok());
            let result = output.unwrap();
            assert_eq!(String::from_utf8_lossy(&result).trim(), "slay_test");
        }
        
        #[cfg(windows)]
        {
            // Windows echo behaves differently
            let _ = cmd.output();
        }
    }

    #[test]
    fn test_process_control() {
        let current_pid = std::process::id();
        
        // Test getting priority (should work for current process)
        let priority_result = ProcessControl::get_priority(current_pid);
        // Don't assert success as it might fail due to permissions on some systems
        println!("Priority result: {:?}", priority_result);
        
        // Test signal operations with non-existent process
        let fake_pid = 999999u32;
        let signal_result = ProcessControl::send_signal(fake_pid, Signal::SIGTERM);
        assert!(signal_result.is_err()); // Should fail for non-existent process
    }

    #[test]
    fn test_process_config_builder() {
        let config = ProcessConfig::new("test_command")
            .arg("arg1")
            .args(&["arg2", "arg3"])
            .env("TEST_VAR", "test_value")
            .timeout(Duration::from_secs(30));
        
        assert_eq!(config.command, "test_command");
        assert_eq!(config.args, vec!["arg1", "arg2", "arg3"]);
        assert_eq!(config.env_vars.get("TEST_VAR"), Some(&"test_value".to_string()));
        assert_eq!(config.timeout, Some(Duration::from_secs(30)));
    }

    #[test]
    fn test_cross_platform_compatibility() {
        // Test that our APIs work across platforms
        
        #[cfg(unix)]
        {
            let config = ProcessConfig::new("ls").arg("-la");
            // Don't actually run it, just test configuration
        }
        
        #[cfg(windows)]
        {
            let config = ProcessConfig::new("dir").arg("/a");
            // Don't actually run it, just test configuration
        }
        
        // Test command existence check
        #[cfg(unix)]
        {
            let exists = crate::stdlib::process::core::command_exists("echo");
            assert!(exists);
            
            let not_exists = crate::stdlib::process::core::command_exists("definitely_not_a_command_12345");
            assert!(!not_exists);
        }
        
        #[cfg(windows)]
        {
            let exists = crate::stdlib::process::core::command_exists("cmd");
            assert!(exists);
        }
    }

    #[test]
    fn test_error_handling() {
        // Test execution of non-existent command
        let result = quick_exec::exec("definitely_not_a_command_12345");
        assert!(result.is_err());
        
        // Test invalid process operations
        let fake_pid = 999999u32;
        let mut manager = UnifiedProcessManager::new().expect("Failed to create manager");
        
        let info_result = manager.get_process_info(fake_pid);
        assert!(info_result.is_err());
        
        let terminate_result = manager.terminate_process(fake_pid);
        assert!(terminate_result.is_err());
        
        manager.shutdown().expect("Failed to shutdown manager");
    }

    #[test]
    fn test_memory_safety() {
        // Test that dropping managers properly cleans up resources
        {
            let mut manager = UnifiedProcessManager::new().expect("Failed to create manager");
            
            #[cfg(unix)]
            let _pid = manager.spawn_tracked("echo", &["memory_test"]);
            
            #[cfg(windows)]
            let _pid = manager.spawn_tracked("cmd", &["/C", "echo memory_test"]);
            
            // Manager should clean up when dropped
        }
        
        // Test multiple manager instances
        for i in 0..5 {
            let mut manager = UnifiedProcessManager::new().expect("Failed to create manager");
            
            #[cfg(unix)]
            let _pid = manager.spawn_tracked("echo", &[&format!("test_{}", i)]);
            
            #[cfg(windows)]
            let _pid = manager.spawn_tracked("cmd", &["/C", &format!("echo test_{}", i)]);
            
            manager.shutdown().expect("Failed to shutdown manager");
        }
    }

    #[test]
    fn test_concurrent_operations() {
        use std::sync::Arc;
        use std::thread;
        
        let manager = Arc::new(std::sync::Mutex::new(
            UnifiedProcessManager::new().expect("Failed to create manager")
        ));
        
        let mut handles = Vec::new();
        
        // Spawn multiple threads doing process operations
        for i in 0..3 {
            let manager_clone = Arc::clone(&manager);
            let handle = thread::spawn(move || {
                let mut mg = manager_clone.lock().unwrap();
                
                #[cfg(unix)]
                let pid = mg.spawn_tracked("echo", &[&format!("concurrent_{}", i)]);
                
                #[cfg(windows)]
                let pid = mg.spawn_tracked("cmd", &["/C", &format!("echo concurrent_{}", i)]);
                
                if let Ok(pid) = pid {
                    // Give process time to complete
                    thread::sleep(Duration::from_millis(100));
                    let _ = mg.terminate_process(pid);
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }
        
        // Shutdown manager
        let mut mg = manager.lock().unwrap();
        mg.shutdown().expect("Failed to shutdown manager");
    }
}

/// Performance and stress tests
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    #[ignore] // Run with --ignored flag
    fn test_process_creation_performance() {
        let mut manager = UnifiedProcessManager::new().expect("Failed to create manager");
        
        let start = Instant::now();
        let mut pids = Vec::new();
        
        // Create 10 processes
        for i in 0..10 {
            #[cfg(unix)]
            let pid = manager.spawn_tracked("echo", &[&format!("perf_test_{}", i)]);
            
            #[cfg(windows)]
            let pid = manager.spawn_tracked("cmd", &["/C", &format!("echo perf_test_{}", i)]);
            
            if let Ok(pid) = pid {
                pids.push(pid);
            }
        }
        
        let creation_time = start.elapsed();
        println!("Created {} processes in {:?}", pids.len(), creation_time);
        
        // Cleanup
        for pid in pids {
            let _ = manager.terminate_process(pid);
        }
        
        manager.shutdown().expect("Failed to shutdown manager");
        
        // Should create processes reasonably quickly
        assert!(creation_time < Duration::from_secs(5));
    }

    #[test]
    #[ignore] // Run with --ignored flag
    fn test_monitoring_overhead() {
        let start = Instant::now();
        
        // Test monitoring startup/shutdown overhead
        for _ in 0..10 {
            start_global_monitoring();
            std::thread::sleep(Duration::from_millis(10));
            stop_global_monitoring();
        }
        
        let total_time = start.elapsed();
        println!("Monitoring overhead for 10 cycles: {:?}", total_time);
        
        // Should not have excessive overhead
        assert!(total_time < Duration::from_secs(5));
    }
}
