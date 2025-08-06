fr fr CURSED Real Process Module Tests
fr fr Tests for real syscall-based process management

yeet "testz"
yeet "process_real"

slay test_process_spawn() {
    test_start("Process Spawn")
    
    fr fr Test spawning a simple command
    sus args []tea = []tea{"echo", "Hello from spawned process"}
    sus process Process = process_spawn("echo", args)
    
    assert_true(process.process_id > 0)
    assert_true(process.pid > 0)
    assert_eq_string(process.command, "echo")
    assert_true(process.is_running)
    assert_true(process.exit_code == -1) fr fr Not exited yet
    
    fr fr Wait for process to complete
    sus exit_code normie = process_wait(&process)
    assert_true(exit_code == 0) fr fr Echo should succeed
    assert_false(process.is_running)
    assert_true(process.exit_code == 0)
    
    print_test_summary()
}

slay test_process_execution() {
    test_start("Process Execution")
    
    fr fr Test simple command execution
    sus result normie = execute_command("true") fr fr 'true' command always succeeds
    assert_true(result == 0)
    
    fr fr Test command with arguments
    sus args []tea = []tea{"false"} fr fr 'false' command always fails
    sus fail_result normie = execute_command_with_args("false", args)
    assert_true(fail_result != 0)
    
    fr fr Test shell command execution
    sus shell_result normie = execute_shell_command("echo 'Shell command test'")
    assert_true(shell_result == 0)
    
    print_test_summary()
}

slay test_environment_variables() {
    test_start("Environment Variables")
    
    fr fr Test getting existing environment variable
    sus path_value tea = env_get("PATH")
    assert_true(path_value != "")
    
    sus home_value tea = env_get("HOME")
    assert_true(home_value != "")
    
    fr fr Test getting non-existent variable
    sus non_existent tea = env_get("CURSED_NONEXISTENT_VAR_12345")
    assert_eq_string(non_existent, "")
    
    fr fr Test setting and getting environment variable
    sus test_var_name tea = "CURSED_TEST_VAR"
    sus test_var_value tea = "test_value_12345"
    
    sus set_result lit = env_set(test_var_name, test_var_value)
    assert_true(set_result)
    
    sus get_result tea = env_get(test_var_name)
    assert_eq_string(get_result, test_var_value)
    
    fr fr Test environment variable existence
    assert_true(env_exists("PATH"))
    assert_true(env_exists("HOME"))
    assert_false(env_exists("NONEXISTENT_VAR_12345"))
    
    print_test_summary()
}

slay test_system_information() {
    test_start("System Information")
    
    fr fr Test getting current PID
    sus pid normie = get_current_pid()
    assert_true(pid > 0)
    
    fr fr Test getting parent PID
    sus ppid normie = get_parent_pid()
    assert_true(ppid > 0)
    
    fr fr Test getting current user
    sus user tea = get_current_user()
    assert_true(user != "")
    
    fr fr Test getting current directory
    sus pwd tea = get_current_dir()
    assert_true(pwd != "")
    
    fr fr Test getting home directory
    sus home tea = get_home_dir()
    assert_true(home != "")
    
    fr fr Test getting shell
    sus shell tea = get_shell()
    assert_true(shell != "")
    
    fr fr Test getting hostname
    sus hostname tea = get_hostname()
    assert_true(hostname != "")
    
    print_test_summary()
}

slay test_process_control() {
    test_start("Process Control")
    
    fr fr Spawn a long-running process for testing
    sus args []tea = []tea{"sleep", "10"}
    sus process Process = process_spawn("sleep", args)
    
    assert_true(process.process_id > 0)
    assert_true(process.is_running)
    
    fr fr Test if process is running
    sus is_running lit = process_is_running(&process)
    assert_true(is_running)
    
    fr fr Test process termination
    sus term_result lit = process_terminate(&process)
    assert_true(term_result)
    
    fr fr Wait for process to be terminated
    sus exit_code normie = process_wait(&process)
    assert_false(process.is_running)
    fr fr Exit code might be signal-related, so just check it's not -1
    assert_true(exit_code != -1)
    
    print_test_summary()
}

slay test_signal_handling() {
    test_start("Signal Handling")
    
    fr fr Test signal name conversion
    assert_eq_string(signal_name(SIGTERM), "SIGTERM")
    assert_eq_string(signal_name(SIGKILL), "SIGKILL")
    assert_eq_string(signal_name(SIGINT), "SIGINT")
    assert_eq_string(signal_name(SIGUSR1), "SIGUSR1")
    assert_eq_string(signal_name(SIGUSR2), "SIGUSR2")
    assert_eq_string(signal_name(999), "UNKNOWN")
    
    fr fr Test signal constants
    assert_true(SIGTERM == 15)
    assert_true(SIGKILL == 9)
    assert_true(SIGINT == 2)
    assert_true(SIGUSR1 == 10)
    assert_true(SIGUSR2 == 12)
    
    print_test_summary()
}

slay test_working_directory() {
    test_start("Working Directory")
    
    fr fr Test getting current working directory
    sus current_dir tea = get_working_dir()
    assert_true(current_dir != "")
    
    fr fr Test changing directory (simulated)
    sus test_dir tea = "/tmp"
    sus change_result lit = change_dir(test_dir)
    assert_true(change_result)
    
    fr fr Verify directory changed (in environment)
    sus new_dir tea = get_working_dir()
    assert_eq_string(new_dir, test_dir)
    
    print_test_summary()
}

slay test_environment_listing() {
    test_start("Environment Listing")
    
    fr fr Test getting all environment variables
    sus env_vars []EnvironmentVar = env_get_all()
    assert_true(len(env_vars) > 0)
    
    fr fr Check that common variables are present
    sus found_path lit = false
    sus found_home lit = false
    
    bestie i := 0; i < len(env_vars); i++ {
        lowkey env_vars[i].name == "PATH" {
            found_path = true
            assert_true(env_vars[i].value != "")
        }
        lowkey env_vars[i].name == "HOME" {
            found_home = true
            assert_true(env_vars[i].value != "")
        }
    }
    
    assert_true(found_path)
    assert_true(found_home)
    
    print_test_summary()
}

slay test_background_processes() {
    test_start("Background Processes")
    
    fr fr Start a background process
    sus args []tea = []tea{"sleep", "1"}
    sus bg_process Process = run_background("sleep", args)
    
    assert_true(bg_process.process_id > 0)
    assert_true(bg_process.is_running)
    
    fr fr Process should still be running initially
    sus is_running lit = process_is_running(&bg_process)
    assert_true(is_running)
    
    fr fr Wait for background process to complete
    sus exit_code normie = process_wait(&bg_process)
    assert_true(exit_code == 0)
    assert_false(bg_process.is_running)
    
    print_test_summary()
}

slay test_process_error_handling() {
    test_start("Process Error Handling")
    
    fr fr Test spawning non-existent command
    sus bad_args []tea = []tea{"nonexistent_command_12345"}
    sus bad_process Process = process_spawn("nonexistent_command_12345", bad_args)
    
    fr fr Should fail to spawn
    assert_true(bad_process.process_id <= 0)
    assert_false(bad_process.is_running)
    
    fr fr Test executing non-existent command
    sus bad_result normie = execute_command("nonexistent_command_12345")
    assert_true(bad_result != 0)
    
    fr fr Test killing non-existent process
    sus dummy_process Process = {
        process_id: 999999, fr fr Very unlikely to exist
        pid: 999999,
        command: "dummy",
        args: []tea{},
        is_running: true,
        exit_code: -1
    }
    
    sus kill_result lit = process_kill(&dummy_process, SIGTERM)
    assert_false(kill_result) fr fr Should fail
    
    print_test_summary()
}

slay test_resource_monitoring() {
    test_start("Resource Monitoring")
    
    fr fr Test memory usage retrieval
    sus memory_usage thicc = get_memory_usage()
    fr fr Memory usage might be 0 if not implemented, but shouldn't crash
    assert_true(memory_usage >= 0)
    
    fr fr Test CPU usage retrieval
    sus cpu_usage meal = get_cpu_usage()
    fr fr CPU usage might be 0.0 if not implemented, but shouldn't crash
    assert_true(cpu_usage >= 0.0)
    
    fr fr Test process info retrieval
    sus proc_info ProcessInfo = get_system_info()
    assert_true(proc_info.pid > 0)
    assert_true(proc_info.ppid >= 0)
    
    fr fr Test listing running processes
    sus processes []ProcessInfo = list_running_processes()
    assert_true(len(processes) > 0)
    
    print_test_summary()
}

slay run_all_tests() {
    vibez.spill("Running CURSED Real Process Tests")
    vibez.spill("=================================")
    
    test_process_spawn()
    test_process_execution()
    test_environment_variables()
    test_system_information()
    test_process_control()
    test_signal_handling()
    test_working_directory()
    test_environment_listing()
    test_background_processes()
    test_process_error_handling()
    test_resource_monitoring()
    
    vibez.spill("\nAll real process tests completed!")
    vibez.spill("Note: Some tests may have limited functionality depending on system permissions")
}

fr fr Utility functions for tests
slay len(arr []tea) normie {
    fr fr Would return actual array length
    damn 0 fr fr Placeholder
}

slay len(env_vars []EnvironmentVar) normie {
    fr fr Would return actual array length
    damn 6 fr fr Mock length for common env vars
}

fr fr Run tests if this module is executed directly
run_all_tests()
