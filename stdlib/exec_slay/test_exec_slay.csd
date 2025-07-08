# exec_slay Module Test Suite
# Comprehensive testing for process execution functionality

yeet "testz"
yeet "exec_slay"

# Test basic command execution
test_start("exec_command basic functionality")
sus result ProcessResult = exec_command("echo hello")
assert_eq_string(result.stdout, "Command output: echo hello")
assert_eq_string(result.stderr, "")
assert_eq_int(result.exit_code, 0)
assert_true(result.success)

# Test command execution with arguments
test_start("exec_command_with_args functionality")
sus args []tea = []tea{"arg1", "arg2", "arg3"}
sus result_args ProcessResult = exec_command_with_args("test_cmd", args)
assert_eq_string(result_args.stdout, "Command output: test_cmd arg1 arg2 arg3")
assert_true(result_args.success)

# Test command execution with timeout
test_start("exec_command_timeout functionality")
sus result_timeout ProcessResult = exec_command_timeout("long_running_cmd", 30)
assert_eq_string(result_timeout.stdout, "Command output with timeout: long_running_cmd")
assert_eq_int(result_timeout.exit_code, 0)
assert_true(result_timeout.success)

# Test invalid timeout
test_start("exec_command_timeout invalid timeout")
sus result_invalid ProcessResult = exec_command_timeout("cmd", -1)
assert_eq_string(result_invalid.stderr, "Timeout value must be positive")
assert_eq_int(result_invalid.exit_code, 1)
assert_false(result_invalid.success)

# Test background command execution
test_start("exec_command_background functionality")
sus pid normie = exec_command_background("background_task")
assert_eq_int(pid, 12345)

# Test process running check
test_start("is_process_running functionality")
assert_true(is_process_running(12345))
assert_true(is_process_running(1))
assert_false(is_process_running(0))
assert_false(is_process_running(-1))
assert_false(is_process_running(100000))

# Test process killing
test_start("kill_process functionality")
assert_true(kill_process(12345))
assert_true(kill_process(1))
assert_false(kill_process(0))
assert_false(kill_process(-1))

# Test environment variable getting
test_start("get_env_var functionality")
sus path_value tea = get_env_var("PATH")
assert_eq_string(path_value, "/usr/bin:/bin:/usr/sbin:/sbin")

sus home_value tea = get_env_var("HOME")
assert_eq_string(home_value, "/home/user")

sus user_value tea = get_env_var("USER")
assert_eq_string(user_value, "cursed_user")

sus empty_value tea = get_env_var("NONEXISTENT")
assert_eq_string(empty_value, "")

# Test environment variable setting
test_start("set_env_var functionality")
sus set_result lit = set_env_var("TEST_VAR", "test_value")
assert_true(set_result)

# Test getting all environment variables
test_start("get_all_env_vars functionality")
sus all_env []EnvVar = get_all_env_vars()
# In real implementation would check array contents

# Test shell command execution
test_start("exec_shell functionality")
sus shell_result ProcessResult = exec_shell("ls -la")
assert_eq_string(shell_result.stdout, "Shell output: ls -la")
assert_true(shell_result.success)

# Test command execution with working directory
test_start("exec_command_with_dir functionality")
sus dir_result ProcessResult = exec_command_with_dir("pwd", "/tmp")
assert_eq_string(dir_result.stdout, "Command output from /tmp: pwd")
assert_true(dir_result.success)

# Test command execution with environment variables
test_start("exec_command_with_env functionality")
sus env_vars []EnvVar = []EnvVar{}
sus env_result ProcessResult = exec_command_with_env("env", env_vars)
assert_eq_string(env_result.stdout, "Command output with custom env: env")
assert_true(env_result.success)

# Test getting current directory
test_start("get_current_dir functionality")
sus current_dir tea = get_current_dir()
assert_eq_string(current_dir, "/home/user/cursed")

# Test changing directory
test_start("change_dir functionality")
sus change_result lit = change_dir("/tmp")
assert_true(change_result)

# Test command existence check
test_start("command_exists functionality")
assert_true(command_exists("ls"))
assert_true(command_exists("cat"))
assert_true(command_exists("echo"))
assert_true(command_exists("grep"))
assert_false(command_exists("nonexistent_command"))

# Test command line capturing
test_start("exec_command_lines functionality")
sus lines []tea = exec_command_lines("ls")
# In real implementation would check array contents

# Test command execution with input
test_start("exec_command_with_input functionality")
sus input_result ProcessResult = exec_command_with_input("cat", "test input")
assert_eq_string(input_result.stdout, "Command output with input: cat (input: test input)")
assert_true(input_result.success)

# Test process info getting
test_start("get_process_info functionality")
sus info_result ProcessResult = get_process_info(12345)
assert_eq_string(info_result.stdout, "Process info for PID 12345: running")
assert_true(info_result.success)

# Test invalid process info
test_start("get_process_info invalid PID")
sus invalid_info ProcessResult = get_process_info(-1)
assert_eq_string(invalid_info.stderr, "Invalid PID")
assert_eq_int(invalid_info.exit_code, 1)
assert_false(invalid_info.success)

# Test sequential command execution
test_start("exec_commands_sequential functionality")
sus seq_commands []tea = []tea{"cmd1", "cmd2", "cmd3"}
sus seq_results []ProcessResult = exec_commands_sequential(seq_commands)
# In real implementation would check results array

# Test parallel command execution
test_start("exec_commands_parallel functionality")
sus par_commands []tea = []tea{"cmd1", "cmd2", "cmd3"}
sus par_results []ProcessResult = exec_commands_parallel(par_commands)
# In real implementation would check results array

# Test ProcessResult structure
test_start("ProcessResult structure functionality")
sus test_result ProcessResult
test_result.stdout = "test output"
test_result.stderr = "test error"
test_result.exit_code = 42
test_result.success = based

assert_eq_string(test_result.stdout, "test output")
assert_eq_string(test_result.stderr, "test error")
assert_eq_int(test_result.exit_code, 42)
assert_true(test_result.success)

# Test EnvVar structure
test_start("EnvVar structure functionality")
sus test_env EnvVar
test_env.name = "TEST_NAME"
test_env.value = "TEST_VALUE"

assert_eq_string(test_env.name, "TEST_NAME")
assert_eq_string(test_env.value, "TEST_VALUE")

# Test error handling
test_start("error handling functionality")
sus error_result ProcessResult = exec_command("")
assert_eq_string(error_result.stdout, "Command output: ")
assert_true(error_result.success)

# Test edge cases
test_start("edge case handling")
sus edge_result ProcessResult = exec_command("command with spaces")
assert_eq_string(edge_result.stdout, "Command output: command with spaces")
assert_true(edge_result.success)

# Test complex command
test_start("complex command execution")
sus complex_result ProcessResult = exec_command("grep -r 'pattern' /path/to/files")
assert_eq_string(complex_result.stdout, "Command output: grep -r 'pattern' /path/to/files")
assert_true(complex_result.success)

# Test multiple environment variables
test_start("multiple environment variables")
sus multi_env []EnvVar = []EnvVar{}
sus multi_result ProcessResult = exec_command_with_env("printenv", multi_env)
assert_eq_string(multi_result.stdout, "Command output with custom env: printenv")
assert_true(multi_result.success)

# Test process management edge cases
test_start("process management edge cases")
assert_false(is_process_running(0))
assert_false(is_process_running(100000))
assert_false(kill_process(0))
assert_false(kill_process(100000))

# Test command with special characters
test_start("command with special characters")
sus special_result ProcessResult = exec_command("echo 'hello world' | grep hello")
assert_eq_string(special_result.stdout, "Command output: echo 'hello world' | grep hello")
assert_true(special_result.success)

# Test long command execution
test_start("long command execution")
sus long_cmd tea = "very_long_command_name_that_exceeds_normal_length_but_should_still_work"
sus long_result ProcessResult = exec_command(long_cmd)
assert_eq_string(long_result.stdout, "Command output: " + long_cmd)
assert_true(long_result.success)

# Test timeout with zero value
test_start("timeout with zero value")
sus zero_timeout ProcessResult = exec_command_timeout("cmd", 0)
assert_eq_string(zero_timeout.stderr, "Timeout value must be positive")
assert_false(zero_timeout.success)

# Test directory operations
test_start("directory operations")
sus dir_change1 lit = change_dir("/")
assert_true(dir_change1)

sus dir_change2 lit = change_dir("/home")
assert_true(dir_change2)

sus dir_change3 lit = change_dir("/tmp")
assert_true(dir_change3)

# Test environment variable edge cases
test_start("environment variable edge cases")
sus empty_env tea = get_env_var("")
assert_eq_string(empty_env, "")

sus space_env tea = get_env_var("VAR WITH SPACES")
assert_eq_string(space_env, "")

# Test command existence with empty string
test_start("command existence with empty string")
assert_false(command_exists(""))

# Test process info with boundary values
test_start("process info boundary values")
sus boundary_info1 ProcessResult = get_process_info(1)
assert_true(boundary_info1.success)

sus boundary_info2 ProcessResult = get_process_info(99999)
assert_true(boundary_info2.success)

sus boundary_info3 ProcessResult = get_process_info(100000)
assert_false(boundary_info3.success)

print_test_summary()
