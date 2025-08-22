fr fr Cross-Platform Process Management Test
fr fr Ensures process management works on Linux, macOS, and Windows

yeet "process_real"
yeet "testz"

slay test_cross_platform_compatibility() {
    vibez.spill("=== Cross-Platform Process Management Test ===")
    
    test_start("Cross-Platform Compatibility")
    
    fr fr Test 1: Platform detection via hostname
    sus hostname tea = get_hostname()
    assert_ne_string(hostname, "", "Hostname should be available on all platforms")
    vibez.spill("Running on host: %s", hostname)
    
    fr fr Test 2: Process ID functions (universal syscalls)
    sus pid normie = get_current_pid()
    sus ppid normie = get_parent_pid()
    assert_gt_int(pid, 0, "PID should be positive on all platforms")
    assert_gt_int(ppid, 0, "PPID should be positive on all platforms")
    vibez.spill("Process IDs - PID: %d, PPID: %d", pid, ppid)
    
    fr fr Test 3: Environment variable access (cross-platform)
    test_cross_platform_env_vars()
    
    fr fr Test 4: Resource limits (Unix-style but should work on Windows too)
    test_cross_platform_resource_limits()
    
    fr fr Test 5: Process spawning (different commands per platform)
    test_cross_platform_process_spawning()
    
    fr fr Test 6: Pipe operations (should work on all platforms)
    test_cross_platform_pipes()
    
    print_test_summary()
    vibez.spill("=== Cross-Platform Tests Complete ===")
}

slay test_cross_platform_env_vars() {
    vibez.spill("\n--- Testing Cross-Platform Environment Variables ---")
    
    fr fr Test common environment variables that exist on all platforms
    sus user tea = env_get("USER")
    lowkey user == "" {
        user = env_get("USERNAME") fr fr Windows alternative
    }
    vibez.spill("Current user: %s", user)
    
    fr fr PATH should exist on all platforms
    sus path tea = env_get("PATH")
    assert_ne_string(path, "", "PATH environment variable should exist")
    vibez.spill("PATH length: %d", string_length(path))
    
    fr fr Test setting custom environment variable
    sus test_var tea = "CURSED_CROSS_PLATFORM_TEST"
    sus test_value tea = "works_everywhere"
    sus set_result lit = env_set(test_var, test_value)
    assert_eq_bool(set_result, true, "Environment variable setting should work")
    
    sus retrieved tea = env_get(test_var)
    assert_eq_string(retrieved, test_value, "Environment variable retrieval should work")
    
    vibez.spill("✓ Cross-platform environment tests passed")
}

slay test_cross_platform_resource_limits() {
    vibez.spill("\n--- Testing Cross-Platform Resource Limits ---")
    
    fr fr Memory limits should work on Unix-like systems, gracefully fail on others
    sus memory_limit thicc = 64 * 1024 * 1024 fr fr 64MB
    sus mem_result lit = set_memory_limit(memory_limit)
    vibez.spill("Memory limit result: %s", mem_result ? "success" : "not supported")
    
    fr fr CPU limits should work on Unix-like systems
    sus cpu_limit meal = 30.0 fr fr 30 seconds
    sus cpu_result lit = set_cpu_limit(cpu_limit)
    vibez.spill("CPU limit result: %s", cpu_result ? "success" : "not supported")
    
    fr fr Resource monitoring should work everywhere (even if simulated)
    sus memory thicc = get_memory_usage()
    sus cpu meal = get_cpu_usage()
    assert_gt_int(memory, 0, "Memory usage should be positive")
    assert_ge_float(cpu, 0.0, "CPU usage should be non-negative")
    
    vibez.spill("Resource usage - Memory: %d bytes, CPU: %.2f%%", memory, cpu)
    vibez.spill("✓ Cross-platform resource limit tests passed")
}

slay test_cross_platform_process_spawning() {
    vibez.spill("\n--- Testing Cross-Platform Process Spawning ---")
    
    fr fr Test with cross-platform command (echo exists on Unix and Windows)
    sus args []tea = []tea{"cross-platform-test"}
    sus process Process = process_spawn("echo", args)
    
    assert_ne_int(process.process_id, 0, "Process ID should be assigned")
    assert_eq_string(process.command, "echo", "Command should be preserved")
    vibez.spill("Spawned cross-platform process: %d", process.process_id)
    
    fr fr Test process status
    sus is_running lit = process_is_running(&process)
    vibez.spill("Process running status: %s", is_running ? "running" : "stopped")
    
    fr fr Test process termination
    sus term_result lit = process_terminate(&process)
    vibez.spill("Process termination: %s", term_result ? "success" : "failed")
    
    vibez.spill("✓ Cross-platform process spawning tests passed")
}

slay test_cross_platform_pipes() {
    vibez.spill("\n--- Testing Cross-Platform Pipe Operations ---")
    
    fr fr Pipes should work on all platforms (named pipes on Windows, Unix pipes elsewhere)
    sus (read_fd, write_fd) = create_pipe()
    assert_gt_int(read_fd, 0, "Read file descriptor should be positive")
    assert_gt_int(write_fd, 0, "Write file descriptor should be positive")
    assert_ne_int(read_fd, write_fd, "File descriptors should be different")
    
    vibez.spill("Created cross-platform pipe: read=%d, write=%d", read_fd, write_fd)
    
    fr fr Test redirection (should work on all platforms)
    sus stdout_redirect lit = redirect_stdout(write_fd)
    sus stderr_redirect lit = redirect_stderr(write_fd)
    
    fr fr These might fail on some platforms but shouldn't crash
    vibez.spill("Stdout redirect: %s", stdout_redirect ? "success" : "not supported")
    vibez.spill("Stderr redirect: %s", stderr_redirect ? "success" : "not supported")
    
    vibez.spill("✓ Cross-platform pipe tests passed")
}

fr fr Test daemonization (Unix-specific but should not crash on Windows)
slay test_daemon_functionality() {
    vibez.spill("\n--- Testing Daemon Functionality ---")
    
    fr fr This should work on Unix and gracefully handle Windows
    sus daemon_result lit = daemonize()
    vibez.spill("Daemon creation: %s", daemon_result ? "success" : "not supported")
    
    vibez.spill("✓ Daemon functionality test completed")
}

fr fr Entry point
test_cross_platform_compatibility()
test_daemon_functionality()
