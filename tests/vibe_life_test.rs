/// Tests for VibeLife module - OS functionality with Gen Z flair
/// 
/// This test suite validates the comprehensive operating system functionality
/// provided by the VibeLife module, ensuring all OS operations work correctly
/// with Gen Z naming conventions.

#[cfg(test)]
mod tests {
    use cursed::stdlib::vibe_life::*;
    use std::time::Duration;

    #[test]
    fn test_environment_operations() {
        // Test setting and getting environment variable
        assert!(set_env_vibe("TEST_VAR_VIBE", "test_value").is_ok());
        assert_eq!(get_env_vibe("TEST_VAR_VIBE"), Some("test_value".to_string()));
        assert!(env_exists_vibe("TEST_VAR_VIBE"));
        
        // Test with default value
        assert_eq!(get_env_vibe_or("NONEXISTENT_VAR_VIBE", "default"), "default");
        assert_eq!(get_env_vibe_or("TEST_VAR_VIBE", "default"), "test_value");
        
        // Test getting all environment variables
        let all_env = get_all_env_vibes().unwrap();
        assert!(all_env.contains_key("TEST_VAR_VIBE"));
        
        // Clean up
        assert!(remove_env_vibe("TEST_VAR_VIBE").is_ok());
        assert!(!env_exists_vibe("TEST_VAR_VIBE"));
    }

    #[test]
    fn test_directory_operations() {
        // Test getting current directory
        let current_dir = get_current_vibe().unwrap();
        assert!(!current_dir.is_empty());
        
        // Test getting home directory (may not be available in all environments)
        let _home_dir = get_home_vibe();
        
        // Test getting temp directory
        let temp_dir = get_temp_vibe();
        assert!(!temp_dir.is_empty());
        
        // Test that temp directory exists
        assert!(path_exists_vibe(&temp_dir));
        assert!(is_directory_vibe(&temp_dir));
    }

    #[test]
    fn test_path_operations() {
        // Test path joining
        let joined = join_path_vibe("/home/user", "documents/file.txt");
        assert!(joined.contains("home"));
        assert!(joined.contains("user"));
        assert!(joined.contains("documents"));
        assert!(joined.contains("file.txt"));
        
        // Test filename extraction
        let filename = get_filename_vibe("/home/user/file.txt");
        assert_eq!(filename, Some("file.txt".to_string()));
        
        let filename = get_filename_vibe("/home/user/");
        assert_eq!(filename, Some("user".to_string()));
        
        let filename = get_filename_vibe("/");
        assert_eq!(filename, None);
        
        // Test extension extraction
        let extension = get_extension_vibe("/home/user/file.txt");
        assert_eq!(extension, Some("txt".to_string()));
        
        let extension = get_extension_vibe("/home/user/file");
        assert_eq!(extension, None);
        
        let extension = get_extension_vibe("/home/user/.hidden");
        assert_eq!(extension, None);
        
        // Test parent directory extraction
        let parent = get_parent_vibe("/home/user/file.txt");
        assert!(parent.is_some());
        assert!(parent.unwrap().contains("user"));
        
        // Test path normalization
        let normalized = normalize_path_vibe("/home/user/../user/./file.txt");
        assert!(normalized.contains("user"));
        assert!(normalized.contains("file.txt"));
        assert!(!normalized.contains(".."));
        assert!(!normalized.contains("."));
        
        let normalized = normalize_path_vibe("./file.txt");
        assert_eq!(normalized, "file.txt");
        
        let normalized = normalize_path_vibe("../parent/file.txt");
        assert_eq!(normalized, "../parent/file.txt");
    }

    #[test]
    fn test_system_information() {
        // Test OS name
        let os_name = get_os_name_vibe();
        assert!(!os_name.is_empty());
        assert!(["linux", "windows", "macos", "freebsd", "unknown"].contains(&os_name.as_str()));
        
        // Test architecture
        let arch = get_architecture_vibe();
        assert!(!arch.is_empty());
        
        // Test CPU count
        let cpu_count = get_cpu_count_vibe();
        assert!(cpu_count > 0);
        assert!(cpu_count <= 1024); // Reasonable upper bound
        
        // Test username (may not be available in all environments)
        let _username = get_username_vibe();
        
        // Test hostname (may not be available in all environments)
        let _hostname = get_hostname_vibe();
    }

    #[test]
    fn test_process_operations() {
        // Test current process ID
        let current_pid = get_current_process_vibe();
        assert!(current_pid > 0);
        
        // Test parent process ID
        let parent_pid = get_parent_process_vibe();
        assert!(parent_pid > 0);
        
        // Test that current process is alive
        assert!(is_process_alive_vibe(current_pid));
        
        // Test with an invalid PID (very high number unlikely to exist)
        assert!(!is_process_alive_vibe(999999999));
    }

    #[test]
    fn test_command_operations() {
        // Test command existence
        #[cfg(unix)]
        {
            assert!(command_exists_vibe("ls"));
            assert!(command_exists_vibe("cat"));
            assert!(!command_exists_vibe("this_command_definitely_does_not_exist_12345"));
        }
        
        #[cfg(windows)]
        {
            assert!(command_exists_vibe("dir"));
            assert!(command_exists_vibe("type"));
            assert!(!command_exists_vibe("this_command_definitely_does_not_exist_12345"));
        }
        
        // Test which command
        #[cfg(unix)]
        {
            let ls_path = which_command_vibe("ls");
            assert!(ls_path.is_some());
            assert!(ls_path.unwrap().contains("ls"));
        }
        
        #[cfg(windows)]
        {
            let cmd_path = which_command_vibe("cmd");
            if cmd_path.is_some() {
                assert!(cmd_path.unwrap().to_lowercase().contains("cmd"));
            }
        }
    }

    #[test]
    fn test_command_execution() {
        // Test simple command execution
        #[cfg(unix)]
        {
            let output = run_command_vibe("echo", &["hello"]);
            assert!(output.is_ok());
            let result = output.unwrap();
            assert!(result.contains("hello"));
        }
        
        #[cfg(windows)]
        {
            let output = run_command_vibe("echo", &["hello"]);
            assert!(output.is_ok());
            let result = output.unwrap();
            assert!(result.contains("hello"));
        }
        
        // Test command with timeout
        #[cfg(unix)]
        {
            let output = run_command_timeout_vibe("echo", &["timeout_test"], Duration::from_secs(1));
            assert!(output.is_ok());
            let result = output.unwrap();
            assert!(result.contains("timeout_test"));
        }
        
        // Test non-existent command
        let output = run_command_vibe("this_command_does_not_exist_12345", &[]);
        assert!(output.is_err());
    }

    #[test]
    fn test_time_operations() {
        // Test timestamp
        let timestamp = get_timestamp_vibe();
        assert!(timestamp > 0);
        
        let timestamp_ms = get_timestamp_millis_vibe();
        assert!(timestamp_ms > 0);
        assert!(timestamp_ms > timestamp); // Milliseconds should be larger
        
        // Test sleep (very short to avoid slowing tests)
        let start = std::time::Instant::now();
        sleep_millis_vibe(1);
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(1));
        
        // Test sleep seconds
        let start = std::time::Instant::now();
        sleep_seconds_vibe(0); // 0 seconds should be instant
        let elapsed = start.elapsed();
        assert!(elapsed < Duration::from_millis(100));
    }

    #[test]
    fn test_platform_detection() {
        // Test platform detection functions
        let is_unix = is_unix_vibe();
        let is_windows = is_windows_vibe();
        
        // Exactly one should be true
        assert!(is_unix ^ is_windows);
        
        #[cfg(unix)]
        assert!(is_unix);
        
        #[cfg(windows)]
        assert!(is_windows);
        
        // Test case sensitivity
        let _is_case_sensitive = is_case_sensitive_vibe();
        
        // Test path separator
        let separator = get_path_separator_vibe();
        assert!(!separator.is_empty());
        
        #[cfg(unix)]
        assert_eq!(separator, "/");
        
        #[cfg(windows)]
        assert_eq!(separator, "\\");
    }

    #[test]
    fn test_memory_operations() {
        let memory_info = get_memory_info_vibe().unwrap();
        
        // Test that memory values are reasonable
        assert!(memory_info.total_memory > 0);
        assert!(memory_info.available_memory <= memory_info.total_memory);
        assert!(memory_info.used_memory <= memory_info.total_memory);
        assert!(memory_info.free_memory <= memory_info.total_memory);
        
        // Test that available + used is approximately total (may not be exact)
        let sum = memory_info.available_memory + memory_info.used_memory;
        assert!(sum <= memory_info.total_memory * 2); // Allow for some accounting differences
    }

    #[test]
    fn test_type_aliases() {
        // Test that type aliases work correctly
        let tea_val: Tea = "hello".to_string();
        let normie_val: Normie = 42;
        let thicc_val: Thicc = 1000000000;
        
        assert_eq!(tea_val, "hello");
        assert_eq!(normie_val, 42);
        assert_eq!(thicc_val, 1000000000);
        
        // Test type sizes
        assert_eq!(std::mem::size_of::<Normie>(), 4); // i32
        assert_eq!(std::mem::size_of::<Thicc>(), 8);  // i64
    }

    #[test]
    fn test_module_functions() {
        // Test module initialization
        assert!(init_vibe_life().is_ok());
        
        // Test module statistics
        let stats = get_vibe_life_stats();
        assert!(stats.contains_key("version"));
        assert!(stats.contains_key("operations"));
        assert!(stats.contains_key("features"));
        assert!(stats.contains_key("os"));
        assert!(stats.contains_key("arch"));
        assert!(stats.contains_key("cpu_cores"));
        
        assert_eq!(stats.get("version").unwrap(), "1.0.0");
        assert!(stats.get("operations").unwrap().contains("Environment"));
        assert!(stats.get("features").unwrap().contains("Gen Z"));
        assert!(!stats.get("os").unwrap().is_empty());
        assert!(!stats.get("arch").unwrap().is_empty());
        
        let cpu_cores_str = stats.get("cpu_cores").unwrap();
        let cpu_cores: usize = cpu_cores_str.parse().unwrap();
        assert!(cpu_cores > 0);
    }

    #[test]
    fn test_file_operations_simulation() {
        // Since we don't want to create actual files in tests,
        // we test with paths that are unlikely to exist
        
        let non_existent_file = "/this/path/definitely/does/not/exist/test_file_12345.txt";
        
        // Test existence checking
        assert!(!path_exists_vibe(non_existent_file));
        assert!(!is_file_vibe(non_existent_file));
        assert!(!is_directory_vibe(non_existent_file));
        
        // Test operations that should fail gracefully
        assert!(get_file_size_vibe(non_existent_file).is_err());
        assert!(read_file_vibe(non_existent_file).is_err());
        assert!(delete_file_vibe(non_existent_file).is_err());
        
        // Test directory operations that should fail
        assert!(list_directory_vibe(non_existent_file).is_err());
        assert!(remove_directory_vibe(non_existent_file).is_err());
    }

    #[test]
    fn test_path_edge_cases() {
        // Test empty path
        let filename = get_filename_vibe("");
        assert_eq!(filename, Some("".to_string()));
        
        // Test root path
        let parent = get_parent_vibe("/");
        assert_eq!(parent, None);
        
        // Test current directory
        let normalized = normalize_path_vibe(".");
        assert_eq!(normalized, ".");
        
        // Test complex path normalization
        let normalized = normalize_path_vibe("/a/b/../c/./d/../e");
        assert_eq!(normalized, "/a/c/e");
        
        // Test path with multiple consecutive separators
        let joined = join_path_vibe("/home//user//", "//documents//file.txt");
        assert!(joined.contains("home"));
        assert!(joined.contains("user"));
        assert!(joined.contains("documents"));
        assert!(joined.contains("file.txt"));
    }

    #[test]
    fn test_environment_edge_cases() {
        // Test setting empty value
        assert!(set_env_vibe("EMPTY_VAR_VIBE", "").is_ok());
        assert_eq!(get_env_vibe("EMPTY_VAR_VIBE"), Some("".to_string()));
        assert!(remove_env_vibe("EMPTY_VAR_VIBE").is_ok());
        
        // Test setting value with special characters
        assert!(set_env_vibe("SPECIAL_VAR_VIBE", "value with spaces and symbols!@#$%").is_ok());
        assert_eq!(get_env_vibe("SPECIAL_VAR_VIBE"), Some("value with spaces and symbols!@#$%".to_string()));
        assert!(remove_env_vibe("SPECIAL_VAR_VIBE").is_ok());
        
        // Test removing non-existent variable (should not error)
        assert!(remove_env_vibe("NON_EXISTENT_VAR_12345").is_ok());
    }

    #[test]
    fn test_timestamp_consistency() {
        // Test that timestamps are monotonically increasing
        let ts1 = get_timestamp_vibe();
        sleep_millis_vibe(1);
        let ts2 = get_timestamp_vibe();
        
        // Should be equal or ts2 should be greater
        assert!(ts2 >= ts1);
        
        // Test millisecond timestamps
        let ts1_ms = get_timestamp_millis_vibe();
        sleep_millis_vibe(2);
        let ts2_ms = get_timestamp_millis_vibe();
        
        assert!(ts2_ms > ts1_ms);
    }

    #[test]
    fn test_system_load_information() {
        // Test system uptime (may not be available on all platforms)
        let uptime_result = get_system_uptime_vibe();
        if uptime_result.is_ok() {
            let uptime = uptime_result.unwrap();
            assert!(uptime.as_secs() > 0); // System should have been up for some time
        }
        
        // Test load average (Unix-specific, may not work on all platforms)
        let load_result = get_load_average_vibe();
        if load_result.is_ok() {
            let (load1, load5, load15) = load_result.unwrap();
            assert!(load1 >= 0.0);
            assert!(load5 >= 0.0);
            assert!(load15 >= 0.0);
        }
    }

    #[test]
    fn test_error_handling() {
        // Test operations that should return specific error types
        
        // Invalid process operations
        assert!(kill_process_vibe(-1).is_err());
        assert!(force_kill_process_vibe(0).is_err());
        
        // Invalid file operations
        assert!(copy_file_vibe("non_existent_source", "target").is_err());
        assert!(move_file_vibe("non_existent_source", "target").is_err());
        
        // Invalid directory operations
        assert!(change_directory_vibe("/this/path/does/not/exist/12345").is_err());
        assert!(create_directory_vibe("/invalid/permissions/test").is_err());
    }

    #[test]
    fn test_absolute_path_operations() {
        // Test absolute path resolution
        let absolute_result = get_absolute_path_vibe(".");
        assert!(absolute_result.is_ok());
        let absolute_path = absolute_result.unwrap();
        assert!(!absolute_path.is_empty());
        
        // Test with relative path
        let absolute_result = get_absolute_path_vibe("./test");
        if absolute_result.is_ok() {
            let absolute_path = absolute_result.unwrap();
            assert!(absolute_path.contains("test"));
        }
        
        // Test with non-existent path (should still resolve to absolute)
        let absolute_result = get_absolute_path_vibe("./non_existent_path_12345");
        if absolute_result.is_ok() {
            let absolute_path = absolute_result.unwrap();
            assert!(absolute_path.contains("non_existent_path_12345"));
        }
    }
}
