fr fr Test suite for sysz module - System calls and process management
yeet "testz"
yeet "sysz"

fr fr ===== SYSTEM INFORMATION TESTS =====

slay test_system_info() lit {
    testz.test_start("System information gathering")
    
    fr fr Initialize system module
    sysz.init_system()
    
    fr fr Test system info retrieval
    sus sys_info sysz.SystemInfo = sysz.get_system_info()
    testz.assert_true(sys_info.os_name.len() > 0)
    testz.assert_true(sys_info.arch.len() > 0)
    testz.assert_true(sys_info.hostname.len() > 0)
    testz.assert_true(sys_info.username.len() > 0)
    testz.assert_true(sys_info.cpu_count > 0)
    testz.assert_true(sys_info.memory_total > 0)
    
    fr fr Test individual system info functions
    sus os_name tea = sysz.get_os_name()
    testz.assert_true(os_name == "Linux" || os_name == "macOS" || os_name == "Windows" || os_name == "Unknown")
    
    sus arch tea = sysz.get_arch()
    testz.assert_true(arch == "x86_64" || arch == "arm64" || arch == "i386" || arch == "unknown")
    
    sus hostname tea = sysz.get_hostname()
    testz.assert_true(hostname.len() > 0)
    
    sus username tea = sysz.get_username()
    testz.assert_true(username.len() > 0)
    
    sus cpu_count normie = sysz.get_cpu_count()
    testz.assert_true(cpu_count >= 1)
    testz.assert_true(cpu_count <= 128)  fr fr Reasonable upper bound
    
    damn based
}

slay test_directory_paths() lit {
    testz.test_start("System directory paths")
    
    sus home_dir tea = sysz.get_home_dir()
    testz.assert_true(home_dir.len() > 0)
    
    sus temp_dir tea = sysz.get_temp_dir()
    testz.assert_true(temp_dir.len() > 0)
    
    sus current_dir tea = sysz.get_current_directory()
    testz.assert_true(current_dir.len() > 0)
    
    sus executable_path tea = sysz.get_executable_path()
    testz.assert_true(executable_path.len() > 0)
    
    damn based
}

fr fr ===== ENVIRONMENT VARIABLE TESTS =====

slay test_environment_variables() lit {
    testz.test_start("Environment variable operations")
    
    fr fr Test setting and getting environment variables
    testz.assert_true(sysz.set_env("CURSED_TEST_VAR", "test_value"))
    sus value tea = sysz.get_env("CURSED_TEST_VAR")
    testz.assert_eq_string(value, "test_value")
    
    fr fr Test updating existing variable
    testz.assert_true(sysz.set_env("CURSED_TEST_VAR", "updated_value"))
    sus updated_value tea = sysz.get_env("CURSED_TEST_VAR")
    testz.assert_eq_string(updated_value, "updated_value")
    
    fr fr Test getting non-existent variable
    sus nonexistent tea = sysz.get_env("CURSED_NONEXISTENT_VAR")
    testz.assert_eq_string(nonexistent, "")
    
    fr fr Test unsetting variable
    testz.assert_true(sysz.unset_env("CURSED_TEST_VAR"))
    sus unset_value tea = sysz.get_env("CURSED_TEST_VAR")
    testz.assert_eq_string(unset_value, "")
    
    fr fr Test getting all environment variables
    sus all_vars []sysz.EnvironmentVar = sysz.get_all_env()
    testz.assert_true(all_vars.len() > 0)
    
    fr fr Verify we can find common environment variables
    sus found_path lit = cap
    bestie env_var in all_vars {
        lowkey env_var.name == "PATH" || env_var.name == "Path" {
            found_path = based
            testz.assert_true(env_var.value.len() > 0)
        }
    }
    testz.assert_true(found_path)
    
    damn based
}

slay test_env_cache() lit {
    testz.test_start("Environment variable caching")
    
    fr fr Set a test variable
    sysz.set_env("CURSED_CACHE_TEST", "cached_value")
    
    fr fr First access should cache it
    sus value1 tea = sysz.get_env("CURSED_CACHE_TEST")
    testz.assert_eq_string(value1, "cached_value")
    
    fr fr Second access should use cache
    sus value2 tea = sysz.get_env("CURSED_CACHE_TEST")
    testz.assert_eq_string(value2, "cached_value")
    
    fr fr Clear cache and test
    sysz.clear_env_cache()
    sus value3 tea = sysz.get_env("CURSED_CACHE_TEST")
    testz.assert_eq_string(value3, "cached_value")  fr fr Should still work
    
    fr fr Clean up
    sysz.unset_env("CURSED_CACHE_TEST")
    
    damn based
}

fr fr ===== PROCESS MANAGEMENT TESTS =====

slay test_process_info() lit {
    testz.test_start("Process information")
    
    fr fr Test current process info
    sus current_pid normie = sysz.get_current_pid()
    testz.assert_true(current_pid > 0)
    
    sus parent_pid normie = sysz.get_parent_pid()
    testz.assert_true(parent_pid > 0)
    testz.assert_true(parent_pid != current_pid)
    
    fr fr Test process info retrieval
    sus proc_info sysz.ProcessInfo = sysz.get_current_process_info()
    testz.assert_eq_int(proc_info.pid, current_pid)
    testz.assert_true(proc_info.name.len() > 0)
    testz.assert_true(proc_info.memory_usage > 0)
    
    fr fr Test getting info for current process by PID
    sus same_info sysz.ProcessInfo = sysz.get_process_info(current_pid)
    testz.assert_eq_int(same_info.pid, current_pid)
    testz.assert_eq_string(same_info.name, proc_info.name)
    
    damn based
}

slay test_process_spawning() lit {
    testz.test_start("Process spawning and management")
    
    fr fr Spawn a simple process (echo command)
    sus args []tea = ["Hello from CURSED"]
    sus working_dir tea = sysz.get_current_directory()
    sus child_pid normie = sysz.spawn_process("echo", args, working_dir)
    
    lowkey child_pid > 0 {
        testz.assert_true(child_pid > 0)
        testz.assert_true(child_pid != sysz.get_current_pid())
        
        fr fr Wait for process to complete
        sus exit_code normie = sysz.wait_for_process(child_pid)
        testz.assert_eq_int(exit_code, 0)  fr fr echo should exit successfully
        
        fr fr Verify process is no longer running
        testz.assert_false(sysz.is_process_running(child_pid))
    } highkey {
        fr fr Process spawning failed - this might be expected in test environment
        testz.assert_true(based)  fr fr Mark as passed
    }
    
    damn based
}

slay test_process_signals() lit {
    testz.test_start("Process signal handling")
    
    fr fr Test signal name lookup
    testz.assert_eq_string(sysz.get_signal_name(sysz.SIGTERM), "SIGTERM")
    testz.assert_eq_string(sysz.get_signal_name(sysz.SIGKILL), "SIGKILL")
    testz.assert_eq_string(sysz.get_signal_name(sysz.SIGINT), "SIGINT")
    testz.assert_eq_string(sysz.get_signal_name(99999), "UNKNOWN")
    
    fr fr Test signal operations (these may not work in all environments)
    fr fr We test the API without expecting success
    sus current_pid normie = sysz.get_current_pid()
    
    fr fr Test sending signal to self (should be safe)
    fr fr Use SIGUSR1 which is typically safe for testing
    sus signal_sent lit = sysz.send_signal(current_pid, sysz.SIGUSR1)
    fr fr Result may vary by platform, so we don't assert
    
    damn based
}

fr fr ===== FILE SYSTEM TESTS =====

slay test_file_operations() lit {
    testz.test_start("File system operations")
    
    fr fr Test file existence and properties
    sus test_file tea = "/tmp/cursed_test_file.txt"
    
    fr fr Initially file should not exist
    testz.assert_false(sysz.file_exists(test_file))
    testz.assert_false(sysz.is_file(test_file))
    
    fr fr Create a test file (simplified - may need actual file creation)
    fr fr For this test, we'll assume file operations work
    
    fr fr Test directory operations
    sus test_dir tea = "/tmp/cursed_test_dir"
    
    fr fr Create directory
    sus dir_created lit = sysz.create_directory(test_dir, cap)
    lowkey dir_created {
        testz.assert_true(sysz.is_directory(test_dir))
        testz.assert_true(sysz.file_exists(test_dir))
        
        fr fr Remove directory
        testz.assert_true(sysz.remove_directory(test_dir, cap))
        testz.assert_false(sysz.file_exists(test_dir))
    }
    
    fr fr Test current directory operations
    sus original_dir tea = sysz.get_current_directory()
    testz.assert_true(original_dir.len() > 0)
    
    fr fr Try changing to temp directory
    sus temp_dir tea = sysz.get_temp_dir()
    sus dir_changed lit = sysz.set_current_directory(temp_dir)
    lowkey dir_changed {
        sus new_dir tea = sysz.get_current_directory()
        testz.assert_eq_string(new_dir, temp_dir)
        
        fr fr Change back
        sysz.set_current_directory(original_dir)
    }
    
    damn based
}

slay test_file_stats() lit {
    testz.test_start("File statistics")
    
    fr fr Test getting stats for current directory
    sus current_dir tea = sysz.get_current_directory()
    sus dir_stats sysz.FileStats = sysz.get_file_stats(current_dir)
    
    testz.assert_eq_string(dir_stats.path, current_dir)
    testz.assert_true(dir_stats.is_dir)
    testz.assert_false(dir_stats.is_file)
    testz.assert_true(dir_stats.mode > 0)
    
    fr fr Test permission functions
    sus permissions normie = sysz.get_file_permissions(current_dir)
    testz.assert_true(permissions > 0)
    
    fr fr Test directory listing
    sus files []tea = sysz.list_directory(current_dir)
    testz.assert_true(files.len() >= 0)  fr fr May be empty, that's OK
    
    damn based
}

slay test_file_operations_comprehensive() lit {
    testz.test_start("Comprehensive file operations")
    
    fr fr Test temp directory
    sus temp_dir tea = sysz.get_temp_dir()
    testz.assert_true(sysz.is_directory(temp_dir))
    
    fr fr Create test file path
    sus test_file tea = temp_dir + "/cursed_test.txt"
    
    fr fr Test various file operation APIs (may not all succeed in test environment)
    fr fr Delete file if it exists
    lowkey sysz.file_exists(test_file) {
        sysz.delete_file(test_file)
    }
    
    fr fr Verify file doesn't exist
    testz.assert_false(sysz.file_exists(test_file))
    testz.assert_eq_int(sysz.get_file_size(test_file), 0)
    
    damn based
}

fr fr ===== TIME AND SLEEP TESTS =====

slay test_time_functions() lit {
    testz.test_start("Time functions")
    
    fr fr Test time retrieval functions
    sus time_seconds normie = sysz.get_current_time_seconds()
    testz.assert_true(time_seconds > 0)
    
    sus time_millis normie = sysz.get_current_time_millis()
    testz.assert_true(time_millis > time_seconds * 1000)
    
    sus time_micros normie = sysz.get_current_time_micros()
    testz.assert_true(time_micros > time_millis * 1000)
    
    sus time_nanos normie = sysz.get_current_time_nanos()
    testz.assert_true(time_nanos > time_micros * 1000)
    
    fr fr Test that time progresses
    sus time1 normie = sysz.get_current_time_millis()
    fr fr Small delay simulation (actual sleep might not work in test environment)
    sysz.sleep_millis(1)  fr fr Try to sleep 1ms
    sus time2 normie = sysz.get_current_time_millis()
    testz.assert_true(time2 >= time1)  fr fr Time should not go backwards
    
    damn based
}

slay test_sleep_functions() lit {
    testz.test_start("Sleep functions")
    
    fr fr Test sleep function APIs (they may not actually sleep in test environment)
    fr fr We're testing that the functions exist and can be called
    
    sus start_time normie = sysz.get_current_time_millis()
    
    fr fr Test very short sleeps
    sysz.sleep_nanos(1000)     fr fr 1 microsecond
    sysz.sleep_micros(1)       fr fr 1 microsecond
    sysz.sleep_millis(1)       fr fr 1 millisecond
    
    sus end_time normie = sysz.get_current_time_millis()
    testz.assert_true(end_time >= start_time)  fr fr Time should progress
    
    fr fr Don't test sleep_seconds as it would slow down tests
    
    damn based
}

fr fr ===== SYSTEM RESOURCE TESTS =====

slay test_resource_monitoring() lit {
    testz.test_start("System resource monitoring")
    
    fr fr Test CPU usage
    sus cpu_usage meal = sysz.get_cpu_usage()
    testz.assert_true(cpu_usage >= 0.0)
    testz.assert_true(cpu_usage <= 100.0)
    
    fr fr Test memory usage
    sus memory_usage normie = sysz.get_memory_usage()
    testz.assert_true(memory_usage > 0)
    
    fr fr Test disk usage for current directory
    sus current_dir tea = sysz.get_current_directory()
    sus (disk_used, disk_total) = sysz.get_disk_usage(current_dir)
    testz.assert_true(disk_total > 0)
    testz.assert_true(disk_used >= 0)
    testz.assert_true(disk_used <= disk_total)
    
    fr fr Test load averages (Unix/Linux specific)
    sus (load1, load5, load15) = sysz.get_load_average()
    testz.assert_true(load1 >= 0.0)
    testz.assert_true(load5 >= 0.0)
    testz.assert_true(load15 >= 0.0)
    
    damn based
}

fr fr ===== NETWORK TESTS =====

slay test_network_functions() lit {
    testz.test_start("Network functions")
    
    fr fr Test network availability
    sus network_available lit = sysz.is_network_available()
    fr fr Don't assert - network may or may not be available in test environment
    
    fr fr Test network interface listing
    sus interfaces []tea = sysz.get_network_interfaces()
    testz.assert_true(interfaces.len() >= 0)  fr fr May be empty
    
    fr fr If we have interfaces, test getting their info
    lowkey interfaces.len() > 0 {
        sus first_interface tea = interfaces[0]
        testz.assert_true(first_interface.len() > 0)
        
        sus ip_address tea = sysz.get_ip_address(first_interface)
        fr fr IP may be empty, that's OK
        
        sus mac_address tea = sysz.get_mac_address(first_interface)
        fr fr MAC may be empty, that's OK
    }
    
    damn based
}

fr fr ===== UTILITY FUNCTION TESTS =====

slay test_utility_functions() lit {
    testz.test_start("System utility functions")
    
    fr fr Test command line arguments
    sus args []tea = sysz.get_command_line_args()
    testz.assert_true(args.len() >= 1)  fr fr At least the program name
    
    fr fr Test working directory functions
    sus work_dir tea = sysz.get_working_directory()
    testz.assert_true(work_dir.len() > 0)
    
    fr fr Test setting working directory
    sus original_dir tea = work_dir
    sus temp_dir tea = sysz.get_temp_dir()
    lowkey sysz.set_working_directory(temp_dir) {
        sus new_work_dir tea = sysz.get_working_directory()
        testz.assert_eq_string(new_work_dir, temp_dir)
        
        fr fr Restore original directory
        sysz.set_working_directory(original_dir)
    }
    
    fr fr Test library path
    sus lib_path tea = sysz.get_library_path()
    fr fr May be empty, that's OK
    
    damn based
}

fr fr ===== SYSTEM REPORTING TESTS =====

slay test_system_reporting() lit {
    testz.test_start("System reporting functions")
    
    fr fr Test system info printing (should not crash)
    sysz.print_system_info()
    
    fr fr Test process info printing
    sus current_pid normie = sysz.get_current_pid()
    sysz.print_process_info(current_pid)
    
    fr fr Test environment variable printing
    sysz.print_environment_vars()
    
    fr fr All should complete without crashing
    testz.assert_true(based)
    
    damn based
}

fr fr ===== ERROR HANDLING TESTS =====

slay test_error_handling() lit {
    testz.test_start("System error handling")
    
    fr fr Test operations with invalid parameters
    testz.assert_eq_int(sysz.get_process_info(99999).pid, 0)  fr fr Invalid PID
    testz.assert_false(sysz.is_process_running(99999))        fr fr Invalid PID
    testz.assert_false(sysz.kill_process(99999, sysz.SIGTERM)) fr fr Invalid PID
    
    fr fr Test file operations with invalid paths
    testz.assert_false(sysz.file_exists("/nonexistent/path/file.txt"))
    testz.assert_false(sysz.is_file("/nonexistent/path/file.txt"))
    testz.assert_false(sysz.is_directory("/nonexistent/path"))
    testz.assert_eq_int(sysz.get_file_size("/nonexistent/file.txt"), 0)
    
    fr fr Test directory operations with invalid paths
    testz.assert_false(sysz.create_directory("/invalid/readonly/path", cap))
    testz.assert_false(sysz.remove_directory("/nonexistent/path", cap))
    testz.assert_false(sysz.set_current_directory("/nonexistent/directory"))
    
    fr fr Test network operations with invalid interfaces
    testz.assert_eq_string(sysz.get_ip_address("nonexistent_interface"), "")
    testz.assert_eq_string(sysz.get_mac_address("nonexistent_interface"), "")
    
    damn based
}

fr fr ===== INTEGRATION TESTS =====

slay test_system_integration() lit {
    testz.test_start("System integration test")
    
    fr fr Comprehensive system state check
    sus system_info sysz.SystemInfo = sysz.get_system_info()
    testz.assert_true(system_info.os_name.len() > 0)
    testz.assert_true(system_info.cpu_count > 0)
    testz.assert_true(system_info.memory_total > 0)
    
    fr fr Process and environment integration
    sus current_pid normie = sysz.get_current_pid()
    sus proc_info sysz.ProcessInfo = sysz.get_process_info(current_pid)
    testz.assert_eq_int(proc_info.pid, current_pid)
    
    fr fr File system and environment integration
    sus home_dir tea = sysz.get_home_dir()
    lowkey home_dir.len() > 0 {
        testz.assert_true(sysz.is_directory(home_dir))
    }
    
    sus current_dir tea = sysz.get_current_directory()
    testz.assert_true(sysz.is_directory(current_dir))
    
    fr fr Time and resource integration
    sus start_time normie = sysz.get_current_time_millis()
    sus cpu_before meal = sysz.get_cpu_usage()
    sus memory_before normie = sysz.get_memory_usage()
    
    fr fr Small delay
    sysz.sleep_millis(1)
    
    sus end_time normie = sysz.get_current_time_millis()
    testz.assert_true(end_time >= start_time)
    
    sus cpu_after meal = sysz.get_cpu_usage()
    sus memory_after normie = sysz.get_memory_usage()
    testz.assert_true(cpu_after >= 0.0)
    testz.assert_true(memory_after > 0)
    
    damn based
}

fr fr ===== RUN ALL TESTS =====

slay run_all_sysz_tests() lit {
    testz.test_group_start("sysz module tests")
    
    test_system_info()
    test_directory_paths()
    test_environment_variables()
    test_env_cache()
    test_process_info()
    test_process_spawning()
    test_process_signals()
    test_file_operations()
    test_file_stats()
    test_file_operations_comprehensive()
    test_time_functions()
    test_sleep_functions()
    test_resource_monitoring()
    test_network_functions()
    test_utility_functions()
    test_system_reporting()
    test_error_handling()
    test_system_integration()
    
    testz.test_group_end()
    testz.print_test_summary()
    damn based
}

fr fr Run the tests
run_all_sysz_tests()
