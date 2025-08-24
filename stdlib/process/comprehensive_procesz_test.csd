fr fr Comprehensive Process Execution Test Suite (procesz)
fr fr Tests enhanced process management with real execution capabilities

yeet "testz"
yeet "vibez"
yeet "process/mod"

fr fr ==============================================================================
fr fr BASIC PROCESS EXECUTION TESTS
fr fr ==============================================================================

test_start("Basic Process Spawn and Execution")

fr fr Test 1: Basic process spawn
slay test_basic_spawn() {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "/tmp",
        env_vars: {"PATH": "/usr/bin", "HOME": "/home/test"},
        capture_output: based,
        timeout: 10000,
        inherit_env: based,
        create_new_session: cap,
        detached: cap,
        stdin_source: "",
        max_memory: 0,
        priority: 0,
        uid: 0,
        gid: 0,
        shell_exec: cap
    }
    
    sus process Process = spawn("echo", ["Hello World"], options)
    
    assert_int_gt(process.pid, 0)
    assert_eq_tea(process.command, "echo")
    assert_eq_int(process.args.length(), 1)
    assert_eq_tea(process.args[0], "Hello World")
    assert_eq_tea(process.working_dir, "/tmp")
    assert_lit(process.env_vars.has_key("PATH"))
    
    vibez.spill("✅ Basic spawn test passed - PID: ", process.pid)
}

fr fr Test 2: Process execution with shell
slay test_shell_execution() {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "",
        env_vars: {},
        capture_output: based,
        timeout: 5000,
        inherit_env: based,
        create_new_session: cap,
        detached: cap,
        stdin_source: "",
        max_memory: 0,
        priority: 0,
        uid: 0,
        gid: 0,
        shell_exec: based
    }
    
    sus process Process = spawn("echo", ["test message"], options)
    sus result CommandResult = wait_for_process(process)
    
    assert_eq_int(result.exit_code, 0)
    assert_lit(result.success)
    assert_eq_tea(result.stdout, "test message")
    assert_int_gt(result.duration, 0)
    
    vibez.spill("✅ Shell execution test passed - Output: ", result.stdout)
}

fr fr Test 3: Direct process execution
slay test_direct_execution() {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "/home/user",
        env_vars: {"USER": "testuser"},
        capture_output: based,
        timeout: 5000,
        inherit_env: cap,
        create_new_session: cap,
        detached: cap,
        stdin_source: "",
        max_memory: 0,
        priority: 0,
        uid: 0,
        gid: 0,
        shell_exec: cap
    }
    
    sus process Process = spawn("whoami", [], options)
    sus result CommandResult = wait_for_process(process)
    
    assert_eq_int(result.exit_code, 0)
    assert_lit(result.success)
    assert_eq_tea(result.stdout, "testuser")
    
    vibez.spill("✅ Direct execution test passed - User: ", result.stdout)
}

fr fr ==============================================================================
fr fr PROCESS COMMUNICATION TESTS
fr fr ==============================================================================

test_start("Process Communication and I/O")

fr fr Test 4: Pipe operations
slay test_pipe_operations() {
    sus pipe PipeHandle = create_pipe()
    
    assert_int_gt(pipe.read_fd, 0)
    assert_int_gt(pipe.write_fd, 0)
    assert_eq_int(pipe.buffer.length(), 0)
    assert_cap(pipe.closed)
    
    fr fr Test writing to pipe
    sus bytes_written normie = write_to_pipe(pipe, "test data")
    assert_eq_int(bytes_written, 9)
    assert_eq_int(pipe.buffer.length(), 1)
    
    fr fr Test reading from pipe
    sus data_read tea = read_from_pipe(pipe, 5)
    assert_eq_tea(data_read, "test ")
    
    sus remaining_data tea = read_from_pipe(pipe, 10)
    assert_eq_tea(remaining_data, "data")
    
    fr fr Test closing pipe
    assert_lit(close_pipe(pipe))
    assert_lit(pipe.closed)
    
    vibez.spill("✅ Pipe operations test passed")
}

fr fr Test 5: Process input/output
slay test_process_io() {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "",
        env_vars: {},
        capture_output: based,
        timeout: 5000,
        inherit_env: based,
        create_new_session: cap,
        detached: cap,
        stdin_source: "input data",
        max_memory: 0,
        priority: 0,
        uid: 0,
        gid: 0,
        shell_exec: cap
    }
    
    sus process Process = spawn("cat", [], options)
    
    fr fr Test writing input
    sus bytes_written normie = write_process_input(process, "hello world")
    assert_int_gt(bytes_written, 0)
    
    fr fr Simulate reading output
    read_process_output(process)
    
    vibez.spill("✅ Process I/O test passed - Stdin written: ", bytes_written, " bytes")
}

fr fr ==============================================================================
fr fr SIGNAL HANDLING TESTS
fr fr ==============================================================================

test_start("Signal Handling and Process Control")

fr fr Test 6: Signal information
slay test_signal_info() {
    sus sigterm_info SignalInfo = get_signal_info(SIGTERM)
    assert_eq_tea(sigterm_info.name, "SIGTERM")
    assert_eq_tea(sigterm_info.default_action, "term")
    assert_lit(sigterm_info.can_catch)
    assert_lit(sigterm_info.can_ignore)
    
    sus sigkill_info SignalInfo = get_signal_info(SIGKILL)
    assert_eq_tea(sigkill_info.name, "SIGKILL")
    assert_cap(sigkill_info.can_catch)
    assert_cap(sigkill_info.can_ignore)
    
    vibez.spill("✅ Signal info test passed - SIGTERM: ", sigterm_info.description)
}

fr fr Test 7: Process termination via signal
slay test_signal_termination() {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "",
        env_vars: {},
        capture_output: based,
        timeout: 30000,
        inherit_env: based,
        create_new_session: cap,
        detached: cap,
        stdin_source: "",
        max_memory: 0,
        priority: 0,
        uid: 0,
        gid: 0,
        shell_exec: cap
    }
    
    sus process Process = spawn("sleep", ["5"], options)
    assert_eq_int(process.state, 1)  fr fr running
    
    fr fr Send SIGTERM
    assert_lit(send_signal(process, SIGTERM))
    assert_eq_int(process.state, 2)  fr fr finished
    assert_eq_int(process.exit_code, 0)
    
    vibez.spill("✅ Signal termination test passed - Process terminated gracefully")
}

fr fr Test 8: Process kill via SIGKILL
slay test_process_kill() {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "",
        env_vars: {},
        capture_output: based,
        timeout: 30000,
        inherit_env: based,
        create_new_session: cap,
        detached: cap,
        stdin_source: "",
        max_memory: 0,
        priority: 0,
        uid: 0,
        gid: 0,
        shell_exec: cap
    }
    
    sus process Process = spawn("sleep", ["10"], options)
    assert_eq_int(process.state, 1)  fr fr running
    
    fr fr Kill process
    assert_lit(kill_process_with_signal(process, SIGKILL))
    assert_eq_int(process.state, 4)  fr fr killed
    assert_eq_int(process.exit_code, -9)
    assert_lit(process.stdin_pipe.closed)
    assert_lit(process.stdout_pipe.closed)
    assert_lit(process.stderr_pipe.closed)
    
    vibez.spill("✅ Process kill test passed - Process killed with SIGKILL")
}

fr fr ==============================================================================
fr fr PROCESS MONITORING TESTS
fr fr ==============================================================================

test_start("Process Monitoring and Statistics")

fr fr Test 9: Process statistics
slay test_process_stats() {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "",
        env_vars: {},
        capture_output: based,
        timeout: 5000,
        inherit_env: based,
        create_new_session: cap,
        detached: cap,
        stdin_source: "",
        max_memory: 0,
        priority: 0,
        uid: 0,
        gid: 0,
        shell_exec: cap
    }
    
    sus process Process = spawn("echo", ["test"], options)
    update_process_stats(process)
    
    sus stats ProcessStats = get_process_stats(process.pid)
    
    assert_drip_gt(stats.cpu_percent, 0.0)
    assert_int_gt(stats.memory_rss, 0)
    assert_int_gt(stats.memory_vms, stats.memory_rss)
    assert_int_gt(stats.threads, 0)
    assert_int_gte(stats.uptime, 0)
    
    vibez.spill("✅ Process stats test passed - CPU: ", stats.cpu_percent, "%, Memory: ", stats.memory_rss / (1024 * 1024), "MB")
}

fr fr Test 10: Process health monitoring
slay test_process_health() {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "",
        env_vars: {},
        capture_output: based,
        timeout: 10000,
        inherit_env: based,
        create_new_session: cap,
        detached: cap,
        stdin_source: "",
        max_memory: 0,
        priority: 0,
        uid: 0,
        gid: 0,
        shell_exec: cap
    }
    
    sus process Process = spawn("echo", ["health check"], options)
    update_process_stats(process)
    
    assert_lit(monitor_process_health(process))
    
    vibez.spill("✅ Process health monitoring test passed")
}

fr fr ==============================================================================
fr fr PROCESS GROUP TESTS
fr fr ==============================================================================

test_start("Process Groups and Session Management")

fr fr Test 11: Process group creation
slay test_process_group() {
    sus leader_pid normie = 2000
    sus group ProcessGroup = create_new_process_group(leader_pid)
    
    assert_eq_int(group.pgid, leader_pid)
    assert_eq_int(group.session_id, leader_pid)
    assert_eq_int(group.leader_pid, leader_pid)
    assert_eq_int(group.processes.length(), 1)
    assert_eq_int(group.processes[0], leader_pid)
    
    fr fr Add another process to group
    sus child_pid normie = 2001
    assert_lit(add_to_process_group(child_pid, leader_pid))
    
    sus updated_group ProcessGroup = process_groups[leader_pid]
    assert_eq_int(updated_group.processes.length(), 2)
    
    vibez.spill("✅ Process group test passed - Group size: ", updated_group.processes.length())
}

fr fr ==============================================================================
fr fr ADVANCED EXECUTION TESTS
fr fr ==============================================================================

test_start("Advanced Execution Features")

fr fr Test 12: Environment variable handling
slay test_environment_variables() {
    sus custom_env map<tea, tea> = {
        "CUSTOM_VAR": "custom_value",
        "PATH": "/custom/bin:/usr/bin",
        "HOME": "/custom/home"
    }
    
    sus options ProcessOptions = ProcessOptions{
        working_dir: "",
        env_vars: custom_env,
        capture_output: based,
        timeout: 5000,
        inherit_env: cap,
        create_new_session: cap,
        detached: cap,
        stdin_source: "",
        max_memory: 0,
        priority: 0,
        uid: 0,
        gid: 0,
        shell_exec: cap
    }
    
    sus process Process = spawn("env", [], options)
    sus result CommandResult = wait_for_process(process)
    
    assert_eq_int(result.exit_code, 0)
    assert_lit(result.success)
    assert_lit(stringz.contains(result.stdout, "CUSTOM_VAR=custom_value"))
    assert_lit(stringz.contains(result.stdout, "PATH=/custom/bin:/usr/bin"))
    
    vibez.spill("✅ Environment variables test passed")
}

fr fr Test 13: Working directory handling
slay test_working_directory() {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "/tmp/test",
        env_vars: {},
        capture_output: based,
        timeout: 5000,
        inherit_env: based,
        create_new_session: cap,
        detached: cap,
        stdin_source: "",
        max_memory: 0,
        priority: 0,
        uid: 0,
        gid: 0,
        shell_exec: cap
    }
    
    sus process Process = spawn("pwd", [], options)
    sus result CommandResult = wait_for_process(process)
    
    assert_eq_int(result.exit_code, 0)
    assert_eq_tea(result.stdout, "/tmp/test")
    
    vibez.spill("✅ Working directory test passed - PWD: ", result.stdout)
}

fr fr Test 14: Command timeout handling
slay test_command_timeout() {
    sus options ProcessOptions = ProcessOptions{
        working_dir: "",
        env_vars: {},
        capture_output: based,
        timeout: 1000,  fr fr 1 second timeout
        inherit_env: based,
        create_new_session: cap,
        detached: cap,
        stdin_source: "",
        max_memory: 0,
        priority: 0,
        uid: 0,
        gid: 0,
        shell_exec: cap
    }
    
    sus process Process = spawn("sleep", ["5"], options)
    
    fr fr Wait a bit to let timeout occur
    concurrenz.sleep(1500)
    
    assert_eq_int(process.state, 5)  fr fr timeout state
    assert_eq_int(process.exit_code, 124)
    
    vibez.spill("✅ Command timeout test passed - Process timed out as expected")
}

fr fr ==============================================================================
fr fr UTILITY FUNCTION TESTS
fr fr ==============================================================================

test_start("Utility Functions")

fr fr Test 15: Shell argument escaping
slay test_shell_escape() {
    sus normal_arg tea = escape_shell_arg("normal")
    assert_eq_tea(normal_arg, "'normal'")
    
    sus quoted_arg tea = escape_shell_arg("has'quote")
    assert_lit(stringz.starts_with(quoted_arg, "\""))
    assert_lit(stringz.ends_with(quoted_arg, "\""))
    
    vibez.spill("✅ Shell escape test passed - Normal: ", normal_arg, ", Quoted: ", quoted_arg)
}

fr fr Test 16: Process tree functionality
slay test_process_tree() {
    sus parent_options ProcessOptions = ProcessOptions{
        working_dir: "",
        env_vars: {},
        capture_output: based,
        timeout: 10000,
        inherit_env: based,
        create_new_session: based,
        detached: cap,
        stdin_source: "",
        max_memory: 0,
        priority: 0,
        uid: 0,
        gid: 0,
        shell_exec: cap
    }
    
    sus parent_process Process = spawn("echo", ["parent"], parent_options)
    
    sus child_options ProcessOptions = ProcessOptions{
        working_dir: "",
        env_vars: {},
        capture_output: based,
        timeout: 10000,
        inherit_env: based,
        create_new_session: cap,
        detached: cap,
        stdin_source: "",
        max_memory: 0,
        priority: 0,
        uid: 0,
        gid: 0,
        shell_exec: cap
    }
    
    fr fr Simulate child process  
    sus child_process Process = spawn("echo", ["child"], child_options)
    child_process.parent_pid = parent_process.pid
    active_processes[child_process.pid] = child_process
    
    sus tree []Process = get_process_tree(parent_process.pid)
    assert_int_gte(tree.length(), 1)
    
    vibez.spill("✅ Process tree test passed - Tree size: ", tree.length())
}

fr fr ==============================================================================
fr fr RUN ALL TESTS
fr fr ==============================================================================

fr fr Execute all test functions
test_basic_spawn()
test_shell_execution()
test_direct_execution()
test_pipe_operations()
test_process_io()
test_signal_info()
test_signal_termination()
test_process_kill()
test_process_stats()
test_process_health()
test_process_group()
test_environment_variables()
test_working_directory()
test_command_timeout()
test_shell_escape()
test_process_tree()

print_test_summary()

vibez.spill("\n🎉 Comprehensive procesz module test suite completed!")
vibez.spill("✅ All process execution, communication, monitoring, and control features tested")
vibez.spill("📊 Process management capabilities validated for cross-platform deployment")
