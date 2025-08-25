yeet "testz"
yeet "process_real"

test_start("Process Real Module Tests")

fr fr === Process Creation Tests ===
test_case("Basic Process Creation") {
    sus command tea = "echo"
    sus args []tea = ["Hello, CURSED!"]
    
    sus proc Process = process_spawn(command, args)
    assert_not_equal(proc.process_id, 0)
    assert_greater_than(proc.pid, 0)
    assert_eq_string(proc.command, command)
    assert_eq_bool(proc.is_running, based)
}

test_case("Process Creation with Multiple Args") {
    sus command tea = "ls"
    sus args []tea = ["-la", "/tmp"]
    
    sus proc Process = process_spawn(command, args)
    assert_not_equal(proc.process_id, 0)
    assert_eq_int(proc.args.len(), 2)
    assert_eq_string(proc.args[0], "-la")
    assert_eq_string(proc.args[1], "/tmp")
}

test_case("Process Creation No Args") {
    sus command tea = "date"
    sus args []tea = []
    
    sus proc Process = process_spawn(command, args)
    assert_not_equal(proc.process_id, 0)
    assert_eq_int(proc.args.len(), 0)
}

test_case("Invalid Command Handling") {
    sus invalid_command tea = "nonexistent_command_12345"
    sus args []tea = []
    
    fam {
        sus proc Process = process_spawn(invalid_command, args)
        fail("Should have thrown error for invalid command")
    } shook (err tea) {
        assert_contains(err, "command", "not found", "executable")
    }
}

fr fr === Process Wait and Status Tests ===
test_case("Process Wait for Completion") {
    sus command tea = "sleep"
    sus args []tea = ["0.1"]  fr fr Short sleep
    
    sus proc Process = process_spawn(command, args)
    assert_eq_bool(proc.is_running, based)
    
    sus exit_code normie = process_wait(proc)
    assert_eq_int(exit_code, 0)  fr fr sleep should exit successfully
    assert_eq_bool(proc.is_running, cap)
    assert_eq_int(proc.exit_code, 0)
}

test_case("Process Status Check") {
    sus command tea = "sleep"
    sus args []tea = ["1.0"]  fr fr Longer sleep
    
    sus proc Process = process_spawn(command, args)
    
    fr fr Should be running initially
    sus status normie = process_status(proc)
    assert_eq_int(status, PROCESS_RUNNING)
    
    fr fr Terminate and check status
    process_kill(proc, SIGTERM)
    sleep_ms(100)  fr fr Give time for termination
    
    sus final_status normie = process_status(proc)
    assert_not_equal(final_status, PROCESS_RUNNING)
}

test_case("Process Non-blocking Wait") {
    sus command tea = "sleep"
    sus args []tea = ["0.5"]
    
    sus proc Process = process_spawn(command, args)
    
    fr fr Non-blocking wait on running process
    sus exit_code normie = process_try_wait(proc)
    assert_eq_int(exit_code, -1)  fr fr Should return -1 for still running
    
    fr fr Wait for completion
    process_wait(proc)
    
    fr fr Now try_wait should return exit code
    sus final_code normie = process_try_wait(proc)
    assert_eq_int(final_code, 0)
}

fr fr === Process Termination Tests ===
test_case("Process Kill with SIGTERM") {
    sus command tea = "sleep"
    sus args []tea = ["10"]  fr fr Long sleep
    
    sus proc Process = process_spawn(command, args)
    assert_eq_bool(proc.is_running, based)
    
    sus kill_result lit = process_kill(proc, SIGTERM)
    assert_eq_bool(kill_result, based)
    
    fr fr Wait for termination
    sleep_ms(100)
    sus status normie = process_status(proc)
    assert_not_equal(status, PROCESS_RUNNING)
}

test_case("Process Kill with SIGKILL") {
    sus command tea = "sleep"
    sus args []tea = ["10"]
    
    sus proc Process = process_spawn(command, args)
    
    sus kill_result lit = process_kill(proc, SIGKILL)
    assert_eq_bool(kill_result, based)
    
    fr fr SIGKILL should terminate immediately
    sleep_ms(50)
    sus status normie = process_status(proc)
    assert_not_equal(status, PROCESS_RUNNING)
}

test_case("Process Terminate Gracefully") {
    sus command tea = "sleep"
    sus args []tea = ["5"]
    
    sus proc Process = process_spawn(command, args)
    
    sus terminate_result lit = process_terminate(proc)
    assert_eq_bool(terminate_result, based)
    
    fr fr Should be terminated
    sleep_ms(100)
    assert_eq_bool(proc.is_running, cap)
}

fr fr === Process Information Tests ===
test_case("Get Process Info") {
    sus command tea = "sleep"
    sus args []tea = ["1"]
    
    sus proc Process = process_spawn(command, args)
    
    sus info ProcessInfo = process_get_info(proc.pid)
    assert_eq_int(info.pid, proc.pid)
    assert_greater_than(info.ppid, 0)  fr fr Should have parent process
    assert_contains(info.command, "sleep")
    assert_greater_than_or_equal(info.cpu_usage, 0.0)
    assert_greater_than(info.memory_usage, 0)
    assert_greater_than(info.start_time, 0)
    
    process_terminate(proc)
}

test_case("Get Current Process Info") {
    sus current_pid normie = process_get_current_pid()
    assert_greater_than(current_pid, 0)
    
    sus info ProcessInfo = process_get_info(current_pid)
    assert_eq_int(info.pid, current_pid)
}

test_case("Get Parent Process ID") {
    sus ppid normie = process_get_parent_pid()
    assert_greater_than(ppid, 0)
}

fr fr === Environment Variable Tests ===
test_case("Set Environment Variable") {
    sus var_name tea = "CURSED_TEST_VAR"
    sus var_value tea = "test_value_123"
    
    sus set_result lit = process_set_env(var_name, var_value)
    assert_eq_bool(set_result, based)
    
    sus retrieved_value tea = process_get_env(var_name)
    assert_eq_string(retrieved_value, var_value)
}

test_case("Get Environment Variable") {
    fr fr Test getting existing environment variable
    sus path tea = process_get_env("PATH")
    assert_not_empty(path)
    
    fr fr Test getting non-existent variable
    sus nonexistent tea = process_get_env("NONEXISTENT_VAR_12345")
    assert_eq_string(nonexistent, "")
}

test_case("Unset Environment Variable") {
    sus var_name tea = "CURSED_TEST_UNSET"
    
    fr fr Set then unset
    process_set_env(var_name, "temporary_value")
    sus unset_result lit = process_unset_env(var_name)
    assert_eq_bool(unset_result, based)
    
    sus value_after_unset tea = process_get_env(var_name)
    assert_eq_string(value_after_unset, "")
}

test_case("Get All Environment Variables") {
    sus env_vars []EnvironmentVar = process_get_all_env()
    
    assert_greater_than(env_vars.len(), 0)
    
    fr fr Should contain PATH
    sus found_path lit = cap
    bestie (sus i normie = 0; i < env_vars.len(); i++) {
        yo env_vars[i].name == "PATH" {
            found_path = based
            assert_not_empty(env_vars[i].value)
            vibes
        }
    }
    assert_eq_bool(found_path, based)
}

fr fr === Process Spawning with Environment Tests ===
test_case("Spawn Process with Custom Environment") {
    sus command tea = "env"
    sus args []tea = []
    sus custom_env []EnvironmentVar = [
        EnvironmentVar{name: "CUSTOM_VAR1", value: "value1"},
        EnvironmentVar{name: "CUSTOM_VAR2", value: "value2"}
    ]
    
    sus proc Process = process_spawn_with_env(command, args, custom_env)
    assert_not_equal(proc.process_id, 0)
    
    sus output tea = process_get_output(proc)
    assert_contains(output, "CUSTOM_VAR1=value1")
    assert_contains(output, "CUSTOM_VAR2=value2")
}

test_case("Spawn Process with Working Directory") {
    sus command tea = "pwd"
    sus args []tea = []
    sus working_dir tea = "/tmp"
    
    sus proc Process = process_spawn_with_cwd(command, args, working_dir)
    
    sus output tea = process_get_output(proc)
    assert_contains(output, "/tmp")
}

fr fr === Process I/O Tests ===
test_case("Process Output Capture") {
    sus command tea = "echo"
    sus args []tea = ["CURSED output test"]
    
    sus proc Process = process_spawn(command, args)
    
    sus output tea = process_get_output(proc)
    assert_contains(output, "CURSED output test")
}

test_case("Process Error Output Capture") {
    sus command tea = "sh"
    sus args []tea = ["-c", "echo 'error message' >&2"]
    
    sus proc Process = process_spawn(command, args)
    
    sus stderr_output tea = process_get_error_output(proc)
    assert_contains(stderr_output, "error message")
}

test_case("Process Input Feeding") {
    sus command tea = "cat"
    sus args []tea = []
    
    sus proc Process = process_spawn(command, args)
    
    sus input_data tea = "Hello from stdin!"
    sus write_result lit = process_write_input(proc, input_data)
    assert_eq_bool(write_result, based)
    
    process_close_input(proc)  fr fr Signal end of input
    
    sus output tea = process_get_output(proc)
    assert_contains(output, "Hello from stdin!")
}

fr fr === Process List and Management Tests ===
test_case("List Running Processes") {
    sus processes []ProcessInfo = process_list_all()
    
    assert_greater_than(processes.len(), 0)
    
    fr fr Should contain current process
    sus current_pid normie = process_get_current_pid()
    sus found_self lit = cap
    
    bestie (sus i normie = 0; i < processes.len(); i++) {
        yo processes[i].pid == current_pid {
            found_self = based
            vibes
        }
    }
    assert_eq_bool(found_self, based)
}

test_case("Find Processes by Name") {
    fr fr Spawn a sleep process to find
    sus command tea = "sleep"
    sus args []tea = ["2"]
    sus proc Process = process_spawn(command, args)
    
    sus sleep_processes []ProcessInfo = process_find_by_name("sleep")
    
    sus found_our_process lit = cap
    bestie (sus i normie = 0; i < sleep_processes.len(); i++) {
        yo sleep_processes[i].pid == proc.pid {
            found_our_process = based
            vibes
        }
    }
    assert_eq_bool(found_our_process, based)
    
    process_terminate(proc)
}

fr fr === Signal Handling Tests ===
test_case("Send Custom Signal") {
    sus command tea = "sleep"
    sus args []tea = ["5"]
    
    sus proc Process = process_spawn(command, args)
    
    fr fr Send SIGUSR1
    sus signal_result lit = process_send_signal(proc, SIGUSR1)
    assert_eq_bool(signal_result, based)
    
    fr fr Clean up
    process_terminate(proc)
}

test_case("Process Signal Handler") {
    fr fr This test verifies signal handling setup
    sus handler_set lit = process_set_signal_handler(SIGINT, "default")
    assert_eq_bool(handler_set, based)
    
    fr fr Reset to default
    sus reset_result lit = process_reset_signal_handler(SIGINT)
    assert_eq_bool(reset_result, based)
}

fr fr === Resource Usage Tests ===
test_case("Process Resource Usage") {
    sus command tea = "yes"  fr fr CPU-intensive command
    sus args []tea = []
    
    sus proc Process = process_spawn(command, args)
    
    fr fr Let it run briefly
    sleep_ms(200)
    
    sus info ProcessInfo = process_get_info(proc.pid)
    assert_greater_than_or_equal(info.cpu_usage, 0.0)
    assert_greater_than(info.memory_usage, 0)
    
    process_kill(proc, SIGKILL)  fr fr Clean up
}

fr fr === Process Group Tests ===
test_case("Process Group Management") {
    sus command tea = "sleep"
    sus args []tea = ["2"]
    
    sus proc Process = process_spawn(command, args)
    
    fr fr Create new process group
    sus setpgid_result lit = process_set_process_group(proc, 0)
    assert_eq_bool(setpgid_result, based)
    
    sus pgid normie = process_get_process_group(proc)
    assert_eq_int(pgid, proc.pid)  fr fr Should be group leader
    
    process_terminate(proc)
}

fr fr === Error Handling Tests ===
test_case("Invalid PID Operations") {
    sus invalid_pid normie = 999999  fr fr Very unlikely to exist
    
    fam {
        sus info ProcessInfo = process_get_info(invalid_pid)
        fail("Should have thrown error for invalid PID")
    } shook (err tea) {
        assert_contains(err, "process", "not found", "invalid")
    }
}

test_case("Kill Non-existent Process") {
    sus dummy_proc Process = Process{
        process_id: 999999,
        pid: 999999,
        command: "dummy",
        args: [],
        is_running: cap,
        exit_code: -1
    }
    
    sus kill_result lit = process_kill(dummy_proc, SIGTERM)
    assert_eq_bool(kill_result, cap)  fr fr Should fail
}

fr fr === Utility Function Tests ===
test_case("Process Exists Check") {
    sus current_pid normie = process_get_current_pid()
    assert_eq_bool(process_exists(current_pid), based)
    
    sus invalid_pid normie = 999999
    assert_eq_bool(process_exists(invalid_pid), cap)
}

test_case("Sleep Functions") {
    sus start_time thicc = process_get_timestamp()
    
    sleep_ms(100)
    
    sus end_time thicc = process_get_timestamp()
    sus elapsed thicc = end_time - start_time
    
    fr fr Should have slept at least 100ms (allowing some tolerance)
    assert_greater_than_or_equal(elapsed, 90)
}

test_case("Process Timing") {
    sus timestamp thicc = process_get_timestamp()
    assert_greater_than(timestamp, 0)
    
    sus timestamp2 thicc = process_get_timestamp()
    assert_greater_than_or_equal(timestamp2, timestamp)
}

print_test_summary()
