fr fr Comprehensive Process Management Test
fr fr Tests real process management implementation with actual syscalls

yeet "process_real"
yeet "testz"

fr fr Test runner  
slay run_comprehensive_process_tests() {
    vibez.spill("=== CURSED Process Management Real Implementation Test ===")
    
    test_start("Process Management Real Implementation")
    
    fr fr Test 1: Process Information
    test_process_information()
    
    fr fr Test 2: Environment Variables
    test_environment_variables()
    
    fr fr Test 3: Process Spawning
    test_process_spawning()
    
    fr fr Test 4: Resource Limits
    test_resource_limits()
    
    fr fr Test 5: Pipe Operations
    test_pipe_operations()
    
    fr fr Test 6: System Information
    test_system_information()
    
    fr fr Test 7: Process Communication
    test_process_communication()
    
    fr fr Test 8: Error Handling
    test_error_handling()
    
    print_test_summary()
    vibez.spill("=== Process Management Real Implementation Tests Complete ===")
}

slay test_process_information() {
    vibez.spill("\n--- Testing Process Information ---")
    
    fr fr Test PID functions
    sus current_pid normie = get_current_pid()
    assert_ne_int(current_pid, 0, "Current PID should not be 0")
    vibez.spill("Current PID: %d", current_pid)
    
    sus parent_pid normie = get_parent_pid()
    assert_ne_int(parent_pid, 0, "Parent PID should not be 0")
    vibez.spill("Parent PID: %d", parent_pid)
    
    fr fr Test system info structure
    sus info ProcessInfo = get_system_info()
    assert_eq_int(info.pid, current_pid, "System info PID matches current PID")
    assert_eq_int(info.ppid, parent_pid, "System info PPID matches parent PID")
    
    vibez.spill("✓ Process information tests passed")
}

slay test_environment_variables() {
    vibez.spill("\n--- Testing Environment Variables ---")
    
    fr fr Test setting and getting environment variables
    sus test_name tea = "CURSED_TEST_VAR"
    sus test_value tea = "test_value_123"
    
    sus set_result lit = env_set(test_name, test_value)
    assert_eq_bool(set_result, true, "Environment variable should set successfully")
    
    sus retrieved_value tea = env_get(test_name)
    assert_eq_string(retrieved_value, test_value, "Retrieved value should match set value")
    
    fr fr Test environment variable existence
    sus exists lit = env_exists(test_name)
    assert_eq_bool(exists, true, "Environment variable should exist")
    
    fr fr Test getting non-existent variable
    sus non_existent tea = env_get("CURSED_NON_EXISTENT_VAR")
    assert_eq_string(non_existent, "", "Non-existent variable should return empty string")
    
    fr fr Test default environment variables
    sus all_vars []EnvironmentVar = env_get_all()
    assert_gt_int(len(all_vars), 0, "Should have default environment variables")
    
    vibez.spill("✓ Environment variable tests passed")
}

slay test_process_spawning() {
    vibez.spill("\n--- Testing Process Spawning ---")
    
    fr fr Test basic process spawning
    sus args []tea = []tea{"arg1", "arg2"}
    sus process Process = process_spawn("echo", args)
    
    assert_ne_int(process.process_id, 0, "Process ID should not be 0")
    assert_eq_bool(process.is_running, true, "Process should be running initially")
    assert_eq_string(process.command, "echo", "Command should be preserved")
    
    vibez.spill("Spawned process with ID: %d", process.process_id)
    
    fr fr Test process waiting
    sus exit_code normie = process_wait(&process)
    assert_ge_int(exit_code, 0, "Exit code should be non-negative")
    assert_eq_bool(process.is_running, false, "Process should not be running after wait")
    
    vibez.spill("Process exited with code: %d", exit_code)
    
    fr fr Test process status checking
    sus is_running lit = process_is_running(&process)
    assert_eq_bool(is_running, false, "Process should not be running")
    
    vibez.spill("✓ Process spawning tests passed")
}

slay test_resource_limits() {
    vibez.spill("\n--- Testing Resource Limits ---")
    
    fr fr Test memory limit setting
    sus memory_limit thicc = 100 * 1024 * 1024 fr fr 100MB
    sus memory_result lit = set_memory_limit(memory_limit)
    assert_eq_bool(memory_result, true, "Memory limit should set successfully")
    
    vibez.spill("Set memory limit: %d bytes", memory_limit)
    
    fr fr Test CPU limit setting  
    sus cpu_limit meal = 10.0 fr fr 10 seconds
    sus cpu_result lit = set_cpu_limit(cpu_limit)
    assert_eq_bool(cpu_result, true, "CPU limit should set successfully")
    
    vibez.spill("Set CPU limit: %.1f seconds", cpu_limit)
    
    fr fr Test resource usage monitoring
    sus memory_usage thicc = get_memory_usage()
    assert_gt_int(memory_usage, 0, "Memory usage should be positive")
    
    sus cpu_usage meal = get_cpu_usage()
    assert_ge_float(cpu_usage, 0.0, "CPU usage should be non-negative")
    
    vibez.spill("Current memory usage: %d bytes", memory_usage)
    vibez.spill("Current CPU usage: %.2f%%", cpu_usage)
    
    vibez.spill("✓ Resource limit tests passed")
}

slay test_pipe_operations() {
    vibez.spill("\n--- Testing Pipe Operations ---")
    
    fr fr Test pipe creation
    sus (read_fd, write_fd) = create_pipe()
    assert_gt_int(read_fd, 0, "Read FD should be positive")
    assert_gt_int(write_fd, 0, "Write FD should be positive")
    assert_ne_int(read_fd, write_fd, "Read and write FDs should be different")
    
    vibez.spill("Created pipe: read_fd=%d, write_fd=%d", read_fd, write_fd)
    
    fr fr Test stdout redirection
    sus stdout_result lit = redirect_stdout(write_fd)
    assert_eq_bool(stdout_result, true, "Stdout redirection should succeed")
    
    fr fr Test stderr redirection
    sus stderr_result lit = redirect_stderr(write_fd)
    assert_eq_bool(stderr_result, true, "Stderr redirection should succeed")
    
    vibez.spill("✓ Pipe operation tests passed")
}

slay test_system_information() {
    vibez.spill("\n--- Testing System Information ---")
    
    fr fr Test hostname
    sus hostname tea = get_hostname()
    assert_ne_string(hostname, "", "Hostname should not be empty")
    vibez.spill("Hostname: %s", hostname)
    
    fr fr Test current user
    sus user tea = get_current_user()
    vibez.spill("Current user: %s", user)
    
    fr fr Test current directory
    sus cwd tea = get_current_dir()
    vibez.spill("Current directory: %s", cwd)
    
    fr fr Test home directory
    sus home tea = get_home_dir()
    vibez.spill("Home directory: %s", home)
    
    fr fr Test shell
    sus shell tea = get_shell()
    assert_ne_string(shell, "", "Shell should not be empty")
    vibez.spill("Shell: %s", shell)
    
    vibez.spill("✓ System information tests passed")
}

slay test_process_communication() {
    vibez.spill("\n--- Testing Process Communication ---")
    
    fr fr Test process signals
    sus args []tea = []tea{"test"}
    sus process Process = process_spawn("sleep", args)
    
    assert_eq_bool(process.is_running, true, "Process should be running")
    
    fr fr Test process termination
    sus term_result lit = process_terminate(&process)
    assert_eq_bool(term_result, true, "Process termination should succeed")
    
    fr fr Test process force kill
    sus process2 Process = process_spawn("sleep", args)
    sus kill_result lit = process_force_kill(&process2)
    assert_eq_bool(kill_result, true, "Process force kill should succeed")
    
    fr fr Test process interrupt
    sus process3 Process = process_spawn("sleep", args)
    sus int_result lit = process_interrupt(&process3)
    assert_eq_bool(int_result, true, "Process interrupt should succeed")
    
    vibez.spill("✓ Process communication tests passed")
}

slay test_error_handling() {
    vibez.spill("\n--- Testing Error Handling ---")
    
    fr fr Test invalid process operations
    sus invalid_process Process = {
        process_id: -1,
        pid: -1,
        command: "",
        args: []tea{},
        is_running: false,
        exit_code: -1
    }
    
    sus wait_result normie = process_wait(&invalid_process)
    assert_eq_int(wait_result, -1, "Invalid process wait should fail")
    
    sus kill_result lit = process_kill(&invalid_process, SIGTERM)
    assert_eq_bool(kill_result, false, "Invalid process kill should fail")
    
    fr fr Test error structure creation
    sus error ProcessError = create_process_error("Test error", "test_cmd", 1)
    assert_eq_string(error.message, "Test error", "Error message should be preserved")
    assert_eq_string(error.command, "test_cmd", "Error command should be preserved")
    assert_eq_int(error.error_code, 1, "Error code should be preserved")
    
    sus handle_result lit = handle_process_error(error)
    assert_eq_bool(handle_result, false, "Error handling should return false")
    
    vibez.spill("✓ Error handling tests passed")
}

fr fr Entry point
run_comprehensive_process_tests()
