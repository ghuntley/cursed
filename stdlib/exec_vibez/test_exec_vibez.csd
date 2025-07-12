yeet "testz"
yeet "exec_vibez"

// Comprehensive test suite for exec_vibez module
// Tests all process execution functionality without FFI dependencies

// Test basic command execution
test_start("Basic command execution")
sus (exit_code, stdout, stderr, success) := exec_command("echo hello")
assert_true(success)
assert_eq_int(exit_code, 0)
assert_true(len(stdout) > 0)
assert_eq_string(stderr, "")
test_end()

// Test command execution with arguments
test_start("Command execution with arguments")
sus args [tea] = ["--version", "--help"]
sus (exit_code2, stdout2, stderr2, success2) := exec_with_args("ls", args)
assert_true(success2)
assert_eq_int(exit_code2, 0)
assert_true(len(stdout2) > 0)
test_end()

// Test command execution with environment variables
test_start("Command execution with environment")
sus env_vars [tea] = ["PATH=/usr/bin", "HOME=/home/user"]
sus (exit_code3, stdout3, stderr3, success3) := exec_with_env("env", env_vars)
assert_true(success3)
assert_eq_int(exit_code3, 0)
assert_true(len(stdout3) > 0)
test_end()

// Test command execution with working directory
test_start("Command execution with working directory")
sus (exit_code4, stdout4, stderr4, success4) := exec_in_dir("pwd", "/tmp")
assert_true(success4)
assert_eq_int(exit_code4, 0)
assert_true(len(stdout4) > 0)
test_end()

// Test command execution with timeout
test_start("Command execution with timeout")
sus (exit_code5, stdout5, stderr5, success5) := exec_with_timeout("sleep 1", 5)
assert_true(success5)
assert_eq_int(exit_code5, 0)
assert_true(len(stdout5) > 0)
test_end()

// Test invalid timeout
test_start("Invalid timeout handling")
sus (exit_code6, stdout6, stderr6, success6) := exec_with_timeout("echo test", -1)
assert_false(success6)
assert_eq_int(exit_code6, 1)
assert_true(len(stderr6) > 0)
test_end()

// Test process spawning
test_start("Process spawning")
sus (running, completed, failed, timeout) := spawn_process("background_task")
assert_true(running)
assert_false(completed)
assert_false(failed)
assert_false(timeout)
test_end()

// Test process termination
test_start("Process termination")
sus termination_result := terminate_process(1234)
assert_true(termination_result)

sus invalid_termination := terminate_process(-1)
assert_false(invalid_termination)
test_end()

// Test process group management
test_start("Process group creation")
sus group_id := create_process_group()
assert_true(group_id > 0)
test_end()

// Test environment variable utilities
test_start("Environment variable management")
sus set_result := set_env_var("TEST_VAR", "test_value")
assert_true(set_result)

sus env_value := get_env_var("TEST_VAR")
assert_true(len(env_value) > 0)

sus empty_env := get_env_var("")
assert_eq_string(empty_env, "")
test_end()

// Test process information utilities
test_start("Process information")
sus pid := get_process_id()
assert_true(pid > 0)

sus ppid := get_parent_process_id()
assert_true(ppid > 0)
test_end()

// Test process resource monitoring
test_start("Process resource monitoring")
sus memory_usage := get_process_memory_usage(1234)
assert_true(memory_usage > 0)

sus cpu_usage := get_process_cpu_usage(1234)
assert_true(cpu_usage > 0.0)

sus invalid_memory := get_process_memory_usage(-1)
assert_eq_int(invalid_memory, 0)
test_end()

// Test command validation
test_start("Command validation")
sus valid_cmd := validate_command("ls -la")
assert_true(valid_cmd)

sus invalid_cmd := validate_command("")
assert_false(invalid_cmd)

sus exists := command_exists("bash")
assert_true(exists)

sus not_exists := command_exists("")
assert_false(not_exists)
test_end()

// Test process stream management
test_start("Process stream capture")
sus stdout_content := capture_stdout(1234)
assert_true(len(stdout_content) > 0)

sus stderr_content := capture_stderr(1234)
assert_true(len(stderr_content) > 0)

sus invalid_stdout := capture_stdout(-1)
assert_eq_string(invalid_stdout, "")
test_end()

// Test signal handling
test_start("Signal handling")
sus signal_result := send_signal(1234, 15)  // SIGTERM
assert_true(signal_result)

sus invalid_signal := send_signal(-1, 15)
assert_false(invalid_signal)
test_end()

// Test process waiting
test_start("Process waiting")
sus (wait_exit, wait_stdout, wait_stderr, wait_success) := wait_for_process(1234)
assert_true(wait_success)
assert_eq_int(wait_exit, 0)

sus (invalid_exit, invalid_out, invalid_err, invalid_wait) := wait_for_process(-1)
assert_false(invalid_wait)
assert_eq_int(invalid_exit, 1)
test_end()

// Test advanced command execution
test_start("Advanced command execution")
sus (adv_exit, adv_stdout, adv_stderr, adv_success) := exec_advanced("advanced_program", "--option1 value1", "VAR1=value1 VAR2=value2", "/home/user", 30)
assert_true(adv_success)
assert_eq_int(adv_exit, 0)
assert_true(len(adv_stdout) > 0)
test_end()

// Test invalid advanced command
test_start("Invalid advanced command")
sus (inv_exit, inv_stdout, inv_stderr, inv_success) := exec_advanced("", "", "", "", 0)
assert_false(inv_success)
assert_eq_int(inv_exit, 1)
assert_true(len(inv_stderr) > 0)
test_end()

// Test process cleanup utilities
test_start("Process cleanup")
sus cleanup_result := cleanup_processes()
assert_true(cleanup_result)
test_end()

// Test running processes enumeration
test_start("Running processes enumeration")
sus running_procs := get_running_processes()
assert_true(len(running_procs) > 0)
test_end()

// Test module initialization
test_start("Module initialization")
sus init_result := init_exec_vibez()
assert_true(init_result)
test_end()

// Test module statistics
test_start("Module statistics")
sus stats := get_exec_stats()
assert_true(len(stats) > 0)
test_end()

// Print comprehensive test summary
print_test_summary()
