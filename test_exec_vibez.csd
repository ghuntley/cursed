yeet "testz"
yeet "exec_vibez"

// Test command creation
test_start("exec_vibez command creation")
exec_reset_state()
sus success lit = exec_new_command("echo")
assert_true(success)

// Test adding arguments
test_start("exec_vibez add arguments")
sus arg_success lit = exec_add_arg("hello")
assert_true(arg_success)
sus arg_success2 lit = exec_add_arg("world")
assert_true(arg_success2)

// Test setting working directory
test_start("exec_vibez set working directory")
sus dir_success lit = exec_set_dir("/tmp")
assert_true(dir_success)

// Test setting timeout
test_start("exec_vibez set timeout")
sus timeout_success lit = exec_set_timeout(60)
assert_true(timeout_success)

// Test command info
test_start("exec_vibez command info")
sus info tea = exec_get_command_info()
assert_true(len(info) > 0)

// Test command execution
test_start("exec_vibez run command")
sus run_success lit = exec_run_command()
assert_true(run_success)
assert_eq_int(exec_get_exit_code(), 0)
assert_true(exec_get_success())
assert_true(len(exec_get_stdout()) > 0)

// Test simple command execution
test_start("exec_vibez simple execution")
exec_reset_state()
sus simple_success lit = exec_simple("ls", "")
assert_true(simple_success)
assert_eq_int(exec_get_exit_code(), 0)
assert_true(exec_get_success())
sus ls_output tea = exec_get_stdout()
assert_true(len(ls_output) > 0)

// Test command with arguments
test_start("exec_vibez command with args")
exec_reset_state()
sus args_success lit = exec_simple("echo", "hello world")
assert_true(args_success)
assert_eq_int(exec_get_exit_code(), 0)
sus echo_output tea = exec_get_stdout()
assert_true(len(echo_output) > 0)

// Test command existence check
test_start("exec_vibez command exists")
assert_true(exec_command_exists("echo"))
assert_true(exec_command_exists("ls"))
assert_true(exec_command_exists("pwd"))
assert_false(exec_command_exists("nonexistent_command"))

// Test environment variable simulation
test_start("exec_vibez environment variables")
sus home tea = exec_get_env("HOME")
assert_eq_string(home, "/home/cursed_user")
sus path tea = exec_get_env("PATH")
assert_true(len(path) > 0)
sus empty tea = exec_get_env("NONEXISTENT")
assert_eq_string(empty, "")

// Test command line parsing and execution
test_start("exec_vibez command line execution")
exec_reset_state()
sus cmdline_success lit = exec_command_line("echo hello world")
assert_true(cmdline_success)
assert_eq_int(exec_get_exit_code(), 0)
assert_true(exec_get_success())

// Test empty command line
test_start("exec_vibez empty command line")
exec_reset_state()
sus empty_success lit = exec_command_line("")
assert_false(empty_success)
assert_eq_int(exec_get_exit_code(), 1)
assert_false(exec_get_success())

// Test different command types
test_start("exec_vibez different commands")
exec_reset_state()
exec_simple("pwd", "")
sus pwd_output tea = exec_get_stdout()
assert_true(len(pwd_output) > 0)

exec_reset_state()
exec_simple("date", "")
sus date_output tea = exec_get_stdout()
assert_true(len(date_output) > 0)

exec_reset_state()
exec_simple("whoami", "")
sus whoami_output tea = exec_get_stdout()
assert_true(len(whoami_output) > 0)

// Test process operations
test_start("exec_vibez process operations")
sus kill_success lit = exec_kill_process(1234)
assert_true(kill_success)
sus invalid_kill lit = exec_kill_process(0)
assert_false(invalid_kill)

// Test directory operations
test_start("exec_vibez directory operations")
sus cwd tea = exec_getcwd()
assert_true(len(cwd) > 0)
sus chdir_success lit = exec_chdir("/tmp")
assert_true(chdir_success)
sus chdir_fail lit = exec_chdir("")
assert_false(chdir_fail)

// Test path existence
test_start("exec_vibez path exists")
assert_true(exec_path_exists("/usr/bin"))
assert_true(exec_path_exists("some_file.txt"))
assert_false(exec_path_exists(""))

// Test background execution
test_start("exec_vibez background execution")
sus bg_pid normie = exec_background("sleep", "10")
assert_true(bg_pid > 0)

// Test system information
test_start("exec_vibez system info")
sus system_info tea = exec_get_system_info()
assert_true(len(system_info) > 0)

// Test system metrics
test_start("exec_vibez system metrics")
sus uptime normie = exec_get_uptime()
assert_true(uptime > 0)
sus load drip = exec_get_load_average()
assert_true(load >= 0.0)

// Test process information
test_start("exec_vibez process info")
sus proc_pid normie = exec_get_process_pid()
assert_true(proc_pid > 0)
sus proc_status tea = exec_get_process_status(1234)
assert_eq_string(proc_status, "running")
sus invalid_status tea = exec_get_process_status(0)
assert_eq_string(invalid_status, "not_found")

// Test file operations
test_start("exec_vibez file operations")
assert_true(exec_file_exists("test.txt"))
assert_false(exec_file_exists(""))
sus file_content tea = exec_read_file("test.txt")
assert_true(len(file_content) > 0)
sus write_success lit = exec_write_file("output.txt", "test content")
assert_true(write_success)

// Test network operations
test_start("exec_vibez network operations")
assert_true(exec_ping("localhost"))
assert_true(exec_ping("127.0.0.1"))
assert_false(exec_ping("invalid_host"))
sus download_success lit = exec_download("http://example.com", "file.txt")
assert_true(download_success)

// Test error handling
test_start("exec_vibez error handling")
exec_reset_state()
exec_new_command("")
sus error_run lit = exec_run_command()
assert_false(error_run)
assert_eq_int(exec_get_exit_code(), 1)
assert_false(exec_get_success())
sus error_msg tea = exec_get_stderr()
assert_true(len(error_msg) > 0)

// Test state reset
test_start("exec_vibez state reset")
exec_reset_state()
sus reset_success lit = exec_reset_state()
assert_true(reset_success)

print_test_summary()
