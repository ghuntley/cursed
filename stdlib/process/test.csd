yeet "testz"
yeet "process"

test_start("process Tests")

// Test Basic Process Creation
test_case("Basic Process Creation") {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "/tmp",
        env_vars: {},
        capture_output: based,
        timeout: 5000,
        inherit_env: based,
        create_new_session: false,
        detached: false,
        stdin_source: "",
        max_memory: 0,
        priority: 0
    }
    
    sus process Process = create_process("echo", ["Hello, CURSED!"], options)
    
    assert(process.pid > 0)
    assert_eq_string(process.command, "echo")
    assert_eq_int(len(process.args), 1)
    assert_eq_string(process.args[0], "Hello, CURSED!")
    assert_eq_string(process.working_dir, "/tmp")
}

// Test Process Execution
test_case("Process Execution and Output Capture") {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "/tmp",
        env_vars: {},
        capture_output: based,
        timeout: 5000,
        inherit_env: based,
        create_new_session: false,
        detached: false,
        stdin_source: "",
        max_memory: 0,
        priority: 0
    }
    
    sus process Process = create_process("echo", ["CURSED process test"], options)
    sus result ProcessResult = execute_process(process)
    
    assert(result.success)
    assert_eq_int(result.exit_code, 0)
    assert(string_contains(result.stdout, "CURSED process test"))
    assert_eq_string(result.stderr, "")
    assert(result.execution_time > 0)
}

// Test Process with Environment Variables
test_case("Process with Environment Variables") {
    sus env_vars map<tea, tea> = {}
    env_vars["CURSED_TEST"] = "environment_test"
    env_vars["CURSED_MODE"] = "testing"
    
    sus options ProcessOptions = ProcessOptions{
        working_dir: "/tmp",
        env_vars: env_vars,
        capture_output: based,
        timeout: 5000,
        inherit_env: false,
        create_new_session: false,
        detached: false,
        stdin_source: "",
        max_memory: 0,
        priority: 0
    }
    
    sus process Process = create_process("sh", ["-c", "echo $CURSED_TEST $CURSED_MODE"], options)
    sus result ProcessResult = execute_process(process)
    
    assert(result.success)
    assert(string_contains(result.stdout, "environment_test"))
    assert(string_contains(result.stdout, "testing"))
}

// Test Process Timeout
test_case("Process Timeout Handling") {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "/tmp",
        env_vars: {},
        capture_output: based,
        timeout: 1000,  // 1 second timeout
        inherit_env: based,
        create_new_session: false,
        detached: false,
        stdin_source: "",
        max_memory: 0,
        priority: 0
    }
    
    // Command that sleeps for 3 seconds (should timeout)
    sus process Process = create_process("sleep", ["3"], options)
    sus start_time drip = get_current_time_ms()
    sus result ProcessResult = execute_process(process)
    sus elapsed_time drip = get_current_time_ms() - start_time
    
    assert(!result.success)
    assert(result.timed_out)
    assert(elapsed_time >= 1000)  // Should have waited at least 1 second
    assert(elapsed_time < 2000)   // Should not have waited 3 seconds
}

// Test Process Input/Output Pipes
test_case("Process Input/Output Pipes") {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "/tmp",
        env_vars: {},
        capture_output: based,
        timeout: 5000,
        inherit_env: based,
        create_new_session: false,
        detached: false,
        stdin_source: "CURSED input test\nSecond line\n",
        max_memory: 0,
        priority: 0
    }
    
    // Use cat command to echo stdin to stdout
    sus process Process = create_process("cat", [], options)
    sus result ProcessResult = execute_process(process)
    
    assert(result.success)
    assert_eq_int(result.exit_code, 0)
    assert(string_contains(result.stdout, "CURSED input test"))
    assert(string_contains(result.stdout, "Second line"))
}

// Test Process Working Directory
test_case("Process Working Directory") {
    sus temp_dir tea = "/tmp/cursed_process_test"
    sus test_file tea = temp_dir + "/workdir_test.txt"
    
    // Create test directory and file
    create_directory(temp_dir)
    write_file(test_file, "working directory test")
    
    sus options ProcessOptions = ProcessOptions{
        working_dir: temp_dir,
        env_vars: {},
        capture_output: based,
        timeout: 5000,
        inherit_env: based,
        create_new_session: false,
        detached: false,
        stdin_source: "",
        max_memory: 0,
        priority: 0
    }
    
    // List current directory contents
    sus process Process = create_process("ls", ["-la"], options)
    sus result ProcessResult = execute_process(process)
    
    assert(result.success)
    assert(string_contains(result.stdout, "workdir_test.txt"))
    
    // Cleanup
    delete_file(test_file)
    delete_directory(temp_dir)
}

// Test Process Status Monitoring
test_case("Process Status Monitoring") {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "/tmp",
        env_vars: {},
        capture_output: based,
        timeout: 10000,
        inherit_env: based,
        create_new_session: false,
        detached: false,
        stdin_source: "",
        max_memory: 0,
        priority: 0
    }
    
    sus process Process = create_process("sleep", ["2"], options)
    
    // Start process asynchronously
    sus success lit = start_process_async(process)
    assert(success)
    assert_eq_int(process.state, PROCESS_RUNNING)
    
    // Check process status while running
    sus status ProcessStatus = get_process_status(process.pid)
    assert(status.is_running)
    assert(status.memory_usage > 0)
    assert(status.cpu_percent >= 0)
    
    // Wait for process to finish
    sus result ProcessResult = wait_for_process(process, 5000)
    assert(result.success)
    assert_eq_int(result.exit_code, 0)
}

// Test Process Termination
test_case("Process Termination") {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "/tmp",
        env_vars: {},
        capture_output: based,
        timeout: 30000,
        inherit_env: based,
        create_new_session: false,
        detached: false,
        stdin_source: "",
        max_memory: 0,
        priority: 0
    }
    
    // Start a long-running process
    sus process Process = create_process("sleep", ["10"], options)
    start_process_async(process)
    
    assert_eq_int(process.state, PROCESS_RUNNING)
    
    // Terminate process
    sus terminate_success lit = terminate_process(process.pid)
    assert(terminate_success)
    
    // Wait a moment for termination to take effect
    sleep(100)
    
    sus status ProcessStatus = get_process_status(process.pid)
    assert(!status.is_running)
}

// Test Process Kill (Force Termination)
test_case("Process Kill (Force Termination)") {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "/tmp",
        env_vars: {},
        capture_output: based,
        timeout: 30000,
        inherit_env: based,
        create_new_session: false,
        detached: false,
        stdin_source: "",
        max_memory: 0,
        priority: 0
    }
    
    // Start a process that ignores SIGTERM
    sus process Process = create_process("sleep", ["10"], options)
    start_process_async(process)
    
    assert_eq_int(process.state, PROCESS_RUNNING)
    
    // Force kill process
    sus kill_success lit = kill_process(process.pid)
    assert(kill_success)
    
    // Wait for kill to take effect
    sleep(100)
    
    sus status ProcessStatus = get_process_status(process.pid)
    assert(!status.is_running)
}

// Test Multiple Processes
test_case("Multiple Processes Management") {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "/tmp",
        env_vars: {},
        capture_output: based,
        timeout: 5000,
        inherit_env: based,
        create_new_session: false,
        detached: false,
        stdin_source: "",
        max_memory: 0,
        priority: 0
    }
    
    sus processes []Process = []
    
    // Create multiple processes
    bestie (sus i normie = 0; i < 5; i += 1) {
        sus message tea = "Process " + string_from_int(i)
        sus process Process = create_process("echo", [message], options)
        processes = array_append(processes, process)
    }
    
    // Execute all processes
    sus results []ProcessResult = []
    bestie (sus i normie = 0; i < len(processes); i += 1) {
        sus result ProcessResult = execute_process(processes[i])
        results = array_append(results, result)
    }
    
    // Verify all processes succeeded
    bestie (sus i normie = 0; i < len(results); i += 1) {
        assert(results[i].success)
        assert_eq_int(results[i].exit_code, 0)
        sus expected_output tea = "Process " + string_from_int(i)
        assert(string_contains(results[i].stdout, expected_output))
    }
}

// Test System Process Information
test_case("System Process Information") {
    // Get current process PID
    sus current_pid normie = get_current_pid()
    assert(current_pid > 0)
    
    // Get parent process PID
    sus parent_pid normie = get_parent_pid()
    assert(parent_pid > 0)
    
    // Get process group ID
    sus process_group normie = get_process_group()
    assert(process_group > 0)
    
    // Get session ID
    sus session_id normie = get_session_id()
    assert(session_id > 0)
    
    // List running processes
    sus running_processes []ProcessInfo = list_running_processes()
    assert(len(running_processes) > 0)
    
    // Find our own process in the list
    sus found_self lit = false
    bestie (sus i normie = 0; i < len(running_processes); i += 1) {
        ready (running_processes[i].pid == current_pid) {
            found_self = based
            assert(string_contains(running_processes[i].command, "cursed"))
        }
    }
    assert(found_self)
}

// Test Process Error Handling
test_case("Process Error Handling") {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "/tmp",
        env_vars: {},
        capture_output: based,
        timeout: 5000,
        inherit_env: based,
        create_new_session: false,
        detached: false,
        stdin_source: "",
        max_memory: 0,
        priority: 0
    }
    
    // Try to execute non-existent command
    sus process Process = create_process("nonexistent_command_12345", [], options)
    sus result ProcessResult = execute_process(process)
    
    assert(!result.success)
    assert(result.exit_code != 0)
    assert(string_length(result.error_message) > 0)
    
    // Try to use invalid working directory
    options.working_dir = "/invalid/nonexistent/directory"
    process = create_process("echo", ["test"], options)
    result = execute_process(process)
    
    assert(!result.success)
    assert(string_length(result.error_message) > 0)
}

print_test_summary()
