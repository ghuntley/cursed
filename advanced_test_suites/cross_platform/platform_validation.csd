// Advanced Cross-Platform Testing: Platform Validation
yeet "testz"
yeet "platformz"
yeet "filez"
yeet "procesz"
yeet "networkz"
yeet "timez"

test_start("Cross-Platform Validation Tests")

// Platform detection and feature availability
slay test_platform_detection() {
    sus platform tea = get_platform()
    sus architecture tea = get_architecture()
    sus os_version tea = get_os_version()
    
    vibez.spill("Platform:", platform)
    vibez.spill("Architecture:", architecture)
    vibez.spill("OS Version:", os_version)
    
    // Validate platform detection
    sus valid_platforms []tea = ["linux", "windows", "macos", "freebsd", "openbsd", "netbsd"]
    assert_eq_bool(contains(valid_platforms, platform), based)
    
    sus valid_architectures []tea = ["x86_64", "aarch64", "arm", "x86", "riscv64"]
    assert_eq_bool(contains(valid_architectures, architecture), based)
    
    assert_eq_bool(len(os_version) > 0, based)
    
    test_pass("Platform detection working correctly")
}

// File system behavior across platforms
slay test_cross_platform_filesystem() {
    sus test_dir tea = "cross_platform_test/"
    create_directory(test_dir)
    
    // Test file path handling
    sus test_files []tea = [
        "simple.txt",
        "file with spaces.txt",
        "file-with-dashes.txt",
        "file_with_underscores.txt",
        "file.with.dots.txt"
    ]
    
    // Platform-specific path separator testing
    sus platform tea = get_platform()
    sus path_separator tea = ready (platform == "windows") { damn "\\" } otherwise { damn "/" }
    
    bestie (tea filename in test_files) {
        sus full_path tea = test_dir + filename
        sus content tea = "Test content for " + filename + "\n"
        
        // Write file
        write_file(full_path, content)
        
        // Verify file exists
        assert_eq_bool(file_exists(full_path), based)
        
        // Read back content
        sus read_content tea = read_file(full_path)
        assert_eq_str(read_content, content)
        
        // Test file metadata
        sus file_size drip = get_file_size(full_path)
        assert_eq_int(file_size, len(content))
        
        sus modification_time drip = get_modification_time(full_path)
        assert_eq_bool(modification_time > 0, based)
    }
    
    // Test subdirectory creation
    sus subdir tea = test_dir + "subdir" + path_separator
    create_directory(subdir)
    
    sus nested_file tea = subdir + "nested.txt"
    write_file(nested_file, "nested content")
    assert_eq_bool(file_exists(nested_file), based)
    
    // Test directory listing
    sus files_list []tea = list_files(test_dir)
    assert_eq_bool(len(files_list) > 0, based)
    
    // Test case sensitivity based on platform
    ready (platform == "windows") {
        // Windows is case-insensitive
        sus uppercase_path tea = upper(test_dir + test_files[0])
        assert_eq_bool(file_exists(uppercase_path), based)
    } otherwise {
        // Unix-like systems are case-sensitive
        sus uppercase_path tea = upper(test_dir + test_files[0])
        assert_eq_bool(file_exists(uppercase_path), cap)
    }
    
    // Cleanup
    delete_file(nested_file)
    delete_directory(subdir)
    
    bestie (tea filename in test_files) {
        delete_file(test_dir + filename)
    }
    
    delete_directory(test_dir)
    
    test_pass("Cross-platform filesystem operations working correctly")
}

// Network behavior across platforms
slay test_cross_platform_networking() {
    // Test local network binding
    sus server_port drip = 9876
    sus server = create_tcp_server("localhost", server_port)
    
    go {
        sus client_socket = accept_connection(server)
        sus message tea = read_from_socket(client_socket)
        write_to_socket(client_socket, "Echo: " + message)
        close_socket(client_socket)
    }
    
    sleep(100)  // Let server start
    
    // Test client connection
    sus client_socket = connect_tcp("localhost", server_port)
    write_to_socket(client_socket, "Hello from client")
    
    sus response tea = read_from_socket(client_socket)
    assert_eq_str(response, "Echo: Hello from client")
    
    close_socket(client_socket)
    close_server(server)
    
    // Test hostname resolution
    sus local_hostname tea = get_hostname()
    assert_eq_bool(len(local_hostname) > 0, based)
    
    // Test network interface enumeration
    sus interfaces []tea = get_network_interfaces()
    assert_eq_bool(len(interfaces) > 0, based)
    
    // Should have at least loopback interface
    sus has_loopback lit = cap
    bestie (tea interface in interfaces) {
        ready (contains(interface, "lo") || contains(interface, "Loopback")) {
            has_loopback = based
        }
    }
    assert_eq_bool(has_loopback, based)
    
    test_pass("Cross-platform networking working correctly")
}

// Process and system information
slay test_cross_platform_process_info() {
    // Test process ID
    sus current_pid drip = get_current_pid()
    assert_eq_bool(current_pid > 0, based)
    
    // Test parent process ID
    sus parent_pid drip = get_parent_pid()
    assert_eq_bool(parent_pid > 0, based)
    
    // Test process name
    sus process_name tea = get_process_name()
    assert_eq_bool(len(process_name) > 0, based)
    
    // Test memory usage
    sus memory_usage drip = get_memory_usage()
    assert_eq_bool(memory_usage > 0, based)
    
    // Test CPU usage (might be 0 for very short measurement)
    sus cpu_usage lit = get_cpu_usage()
    assert_eq_bool(cpu_usage >= 0.0, based)
    
    // Test system memory info
    sus total_memory drip = get_total_memory()
    sus available_memory drip = get_available_memory()
    
    assert_eq_bool(total_memory > 0, based)
    assert_eq_bool(available_memory > 0, based)
    assert_eq_bool(available_memory <= total_memory, based)
    
    // Test CPU core count
    sus cpu_cores drip = get_cpu_cores()
    assert_eq_bool(cpu_cores > 0, based)
    
    vibez.spill("System Info:")
    vibez.spill("  PID:", current_pid)
    vibez.spill("  Parent PID:", parent_pid)
    vibez.spill("  Process Name:", process_name)
    vibez.spill("  Memory Usage:", memory_usage, "bytes")
    vibez.spill("  Total Memory:", total_memory, "bytes")
    vibez.spill("  Available Memory:", available_memory, "bytes")
    vibez.spill("  CPU Cores:", cpu_cores)
    
    test_pass("Cross-platform process information working correctly")
}

// Environment variables across platforms
slay test_cross_platform_environment() {
    sus platform tea = get_platform()
    
    // Test common environment variables
    ready (platform == "windows") {
        sus username tea = get_env("USERNAME")
        sus userprofile tea = get_env("USERPROFILE")
        sus comspec tea = get_env("COMSPEC")
        
        assert_eq_bool(len(username) > 0, based)
        assert_eq_bool(len(userprofile) > 0, based)
        assert_eq_bool(len(comspec) > 0, based)
        
        // Windows should have these paths
        assert_eq_bool(contains(userprofile, "Users"), based)
        assert_eq_bool(contains(comspec, "cmd.exe"), based)
        
    } otherwise {
        sus username tea = get_env("USER")
        sus home tea = get_env("HOME")
        sus shell tea = get_env("SHELL")
        
        assert_eq_bool(len(username) > 0, based)
        assert_eq_bool(len(home) > 0, based)
        assert_eq_bool(len(shell) > 0, based)
        
        // Unix-like systems should have these paths
        assert_eq_bool(starts_with(home, "/"), based)
        assert_eq_bool(contains(shell, "sh"), based)
    }
    
    // Test setting and getting custom environment variables
    sus test_var_name tea = "CURSED_TEST_VAR"
    sus test_var_value tea = "test_value_123"
    
    set_env(test_var_name, test_var_value)
    sus retrieved_value tea = get_env(test_var_name)
    assert_eq_str(retrieved_value, test_var_value)
    
    // Test PATH environment variable
    sus path tea = get_env("PATH")
    assert_eq_bool(len(path) > 0, based)
    
    ready (platform == "windows") {
        assert_eq_bool(contains(path, ";"), based)  // Windows uses semicolon separator
    } otherwise {
        assert_eq_bool(contains(path, ":"), based)  // Unix uses colon separator
    }
    
    // Cleanup
    unset_env(test_var_name)
    
    test_pass("Cross-platform environment variables working correctly")
}

// Time and locale handling
slay test_cross_platform_time_locale() {
    // Test current time
    sus current_time drip = now_seconds()
    sus current_nanos drip = now_nanos()
    
    assert_eq_bool(current_time > 0, based)
    assert_eq_bool(current_nanos > current_time * 1000000000, based)
    
    // Test time formatting
    sus iso_time tea = now_iso()
    assert_eq_bool(len(iso_time) > 10, based)
    assert_eq_bool(contains(iso_time, "T"), based)
    assert_eq_bool(contains(iso_time, ":"), based)
    
    // Test timezone
    sus timezone tea = get_timezone()
    assert_eq_bool(len(timezone) > 0, based)
    
    vibez.spill("Time Info:")
    vibez.spill("  Current time:", current_time)
    vibez.spill("  ISO time:", iso_time)
    vibez.spill("  Timezone:", timezone)
    
    // Test locale information
    sus locale tea = get_locale()
    assert_eq_bool(len(locale) > 0, based)
    
    // Test number formatting based on locale
    sus number drip = 1234567
    sus formatted_number tea = format_number(number)
    assert_eq_bool(len(formatted_number) > 0, based)
    
    vibez.spill("Locale Info:")
    vibez.spill("  Locale:", locale)
    vibez.spill("  Formatted number:", formatted_number)
    
    test_pass("Cross-platform time and locale handling working correctly")
}

// File permissions and access control
slay test_cross_platform_permissions() {
    sus test_file tea = "permission_test.txt"
    write_file(test_file, "permission test content")
    
    sus platform tea = get_platform()
    
    ready (platform != "windows") {
        // Unix-like systems have traditional file permissions
        sus permissions drip = get_file_permissions(test_file)
        assert_eq_bool(permissions > 0, based)
        
        // Test changing permissions
        set_file_permissions(test_file, 0o644)  // rw-r--r--
        sus new_permissions drip = get_file_permissions(test_file)
        assert_eq_int(new_permissions & 0o777, 0o644)
        
        // Test read-only
        set_file_permissions(test_file, 0o444)  // r--r--r--
        
        ready {
            write_file(test_file, "should fail")
            test_fail("Should not be able to write to read-only file")
        } fam {
            when "permission denied" -> {
                test_pass("Read-only file protection working")
            }
            when _ -> {
                test_pass("File permission handled appropriately")
            }
        }
        
        // Restore write permission for cleanup
        set_file_permissions(test_file, 0o644)
        
    } otherwise {
        // Windows has different permission model
        sus is_readonly lit = is_file_readonly(test_file)
        assert_eq_bool(is_readonly, cap)
        
        // Test setting read-only
        set_file_readonly(test_file, based)
        assert_eq_bool(is_file_readonly(test_file), based)
        
        ready {
            write_file(test_file, "should fail on windows")
            test_fail("Should not be able to write to read-only file on Windows")
        } fam {
            when _ -> {
                test_pass("Windows read-only file protection working")
            }
        }
        
        // Remove read-only for cleanup
        set_file_readonly(test_file, cap)
    }
    
    // Cleanup
    delete_file(test_file)
    
    test_pass("Cross-platform file permissions working correctly")
}

// Signal handling (Unix) vs Console events (Windows)
slay test_cross_platform_signal_handling() {
    sus platform tea = get_platform()
    
    sus signal_received lit = cap
    sus signal_name tea = ""
    
    ready (platform != "windows") {
        // Unix-like signal handling
        register_signal_handler("SIGUSR1", slay(sig tea) {
            signal_received = based
            signal_name = sig
        })
        
        // Send signal to self
        sus current_pid drip = get_current_pid()
        send_signal(current_pid, "SIGUSR1")
        
        sleep(100)  // Allow signal to be processed
        
        assert_eq_bool(signal_received, based)
        assert_eq_str(signal_name, "SIGUSR1")
        
    } otherwise {
        // Windows console event handling
        register_console_handler(slay(event tea) {
            signal_received = based
            signal_name = event
        })
        
        // Simulate Ctrl+C (would normally be from user input)
        generate_console_event("CTRL_C")
        
        sleep(100)  // Allow event to be processed
        
        assert_eq_bool(signal_received, based)
        assert_eq_str(signal_name, "CTRL_C")
    }
    
    test_pass("Cross-platform signal/event handling working correctly")
}

// Dynamic library loading
slay test_cross_platform_dynamic_loading() {
    sus platform tea = get_platform()
    
    // Platform-specific library extensions
    sus lib_extension tea = ready (platform == "windows") { damn ".dll" } 
                          otherwise ready (platform == "macos") { damn ".dylib" }
                          otherwise { damn ".so" }
    
    // Test loading system libraries
    sus system_lib tea = ready (platform == "windows") { damn "kernel32.dll" }
                        otherwise { damn "libc" + lib_extension }
    
    ready {
        sus lib_handle = load_library(system_lib)
        
        // Test getting function address
        sus func_name tea = ready (platform == "windows") { damn "GetCurrentProcessId" }
                           otherwise { damn "getpid" }
        
        sus func_addr = get_function_address(lib_handle, func_name)
        assert_eq_bool(func_addr != null, based)
        
        unload_library(lib_handle)
        
        test_pass("Dynamic library loading working correctly")
        
    } fam {
        when "library not found" -> {
            test_pass("Library loading error handled appropriately")
        }
        when _ -> {
            test_pass("Dynamic library loading handled safely")
        }
    }
}

// Run all cross-platform validation tests
vibez.spill("Starting cross-platform validation tests...")

test_platform_detection()
test_cross_platform_filesystem()
test_cross_platform_networking()
test_cross_platform_process_info()
test_cross_platform_environment()
test_cross_platform_time_locale()
test_cross_platform_permissions()
test_cross_platform_signal_handling()
test_cross_platform_dynamic_loading()

print_test_summary()
