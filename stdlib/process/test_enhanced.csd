yeet "testz"
yeet "process"

test_start("Process spawning")
sus options ProcessOptions = ProcessOptions{
    working_dir: "/home/user",
    env_vars: ["PATH=/usr/bin"],
    capture_output: based,
    timeout: 5000
}

sus process Process = spawn("echo", ["Hello World"], options)
assert_true(process.pid > 0)
assert_eq_string(process.command, "echo")

test_start("Command execution")
sus result CommandResult = exec("echo", ["test message"])
assert_eq_int(result.exit_code, 0)
assert_true(result.success)

test_start("Directory listing")
sus ls_result CommandResult = exec("ls", ["/home/user"])
assert_eq_int(ls_result.exit_code, 0)
assert_true(len(ls_result.stdout) > 0)

test_start("File operations")
sus cat_result CommandResult = exec("cat", ["file1.txt"])
assert_eq_int(cat_result.exit_code, 0)

sus mkdir_result CommandResult = exec("mkdir", ["test_dir"])
assert_eq_int(mkdir_result.exit_code, 0)

test_start("Process monitoring")
sus current_pid normie = getpid()
assert_true(current_pid > 0)

sus parent_pid normie = getppid()
assert_true(parent_pid > 0)

sus running lit = is_process_running(current_pid)
assert_true(running)

test_start("Process signals")
sus test_process Process = spawn("sleep", ["10"], options)
sus kill_success lit = send_signal(test_process, 15)  fr fr SIGTERM
assert_true(kill_success)

test_start("Environment variables")
sus path_env tea = getenv("PATH")
assert_true(len(path_env) > 0)

sus home_env tea = getenv("HOME")
assert_eq_string(home_env, "/home/user")

sus set_success lit = setenv("TEST_VAR", "test_value")
assert_true(set_success)

test_start("Working directory")
sus cwd tea = getcwd()
assert_true(len(cwd) > 0)

sus chdir_success lit = chdir("/tmp")
assert_true(chdir_success)

test_start("Process statistics")
sus stats ProcessStats = get_process_stats(current_pid)
assert_true(stats.cpu_percent >= 0)
assert_true(stats.memory_mb >= 0)
assert_true(stats.threads > 0)

test_start("System information")
sus sys_info tea = get_system_info()
assert_true(len(sys_info) > 0)

sus cpu_count normie = get_cpu_count()
assert_true(cpu_count > 0)

test_start("Pipe communication")
sus pipe Pipe = create_pipe()
sus written normie = pipe.write("test data")
assert_true(written > 0)

sus read_data tea = pipe.read(4)
assert_true(len(read_data) >= 0)

sus close_success lit = pipe.close()
assert_true(close_success)

test_start("Error conditions")
sus fail_result CommandResult = exec("false", [])
assert_eq_int(fail_result.exit_code, 1)
assert_false(fail_result.success)

sus not_found_result CommandResult = exec("nonexistent_command", [])
assert_eq_int(not_found_result.exit_code, 127)

test_start("Convenience functions")
sus echo_result CommandResult = echo("Hello from convenience function")
assert_eq_int(echo_result.exit_code, 0)

sus pwd_result CommandResult = print_working_directory()
assert_eq_int(pwd_result.exit_code, 0)

sus whoami_result CommandResult = who_am_i()
assert_eq_int(whoami_result.exit_code, 0)

print_test_summary()
