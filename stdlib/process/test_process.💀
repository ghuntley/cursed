yeet "testz"
yeet "process"

fr fr ========================================
fr fr Process Module Comprehensive Tests
fr fr ========================================

test_start("Basic Process Spawn Tests")

fr fr Test process spawning
sus options ProcessOptions = ProcessOptions{
    working_dir: "/home/user",
    env_vars: ["PATH=/usr/bin"],
    capture_output: based,
    timeout: 5000
}

sus process Process = spawn("echo", ["hello"], options)
assert_true(process.pid > 0)
assert_eq_string(process.command, "echo")
assert_eq_int(process.args.length(), 1)
assert_eq_string(process.args[0], "hello")

test_start("Command Execution Tests")

fr fr Test echo command
sus echo_result CommandResult = echo("Hello World")
assert_eq_int(echo_result.exit_code, 0)
assert_true(echo_result.success)
assert_eq_string(echo_result.stdout, "Hello World")

fr fr Test ls command
sus ls_result CommandResult = list_directory("")
assert_eq_int(ls_result.exit_code, 0)
assert_true(ls_result.success)
assert_true(ls_result.stdout.contains("file1.txt"))

fr fr Test pwd command
sus pwd_result CommandResult = print_working_directory()
assert_eq_int(pwd_result.exit_code, 0)
assert_eq_string(pwd_result.stdout, "/home/user/projects")

fr fr Test whoami command
sus whoami_result CommandResult = who_am_i()
assert_eq_int(whoami_result.exit_code, 0)
assert_eq_string(whoami_result.stdout, "user")

test_start("Process Management Tests")

fr fr Test process PID functions
sus current_pid normie = getpid()
assert_eq_int(current_pid, 12345)

sus parent_pid normie = getppid()
assert_eq_int(parent_pid, 1234)

fr fr Test process listing
sus processes Process[value] = get_processes()
assert_true(processes.length() >= 2)
assert_eq_string(processes[0].command, "init")

test_start("Process Status Tests")

fr fr Test process finding
sus found_process Process = find_process(getpid())
assert_eq_int(found_process.pid, getpid())
assert_eq_string(found_process.command, "cursed")

fr fr Test process running check
assert_true(is_process_running(getpid()))
assert_false(is_process_running(99999))  fr fr non-existent PID

fr fr Test process status
sus status tea = get_process_status(getpid())
assert_eq_string(status, "running")

sus not_found_status tea = get_process_status(99999)
assert_eq_string(not_found_status, "not_found")

test_start("Environment Variable Tests")

fr fr Test getting environment variables
sus path tea = getenv("PATH")
assert_eq_string(path, "/usr/local/bin:/usr/bin:/bin")

sus home tea = getenv("HOME")
assert_eq_string(home, "/home/user")

sus user tea = getenv("USER")
assert_eq_string(user, "user")

sus nonexistent tea = getenv("NONEXISTENT")
assert_eq_string(nonexistent, "")

fr fr Test setting environment variables
assert_true(setenv("TEST_VAR", "test_value"))
assert_false(setenv("", ""))

fr fr Test unsetting environment variables
assert_true(unsetenv("TEST_VAR"))
assert_false(unsetenv(""))

test_start("Environment Listing Tests")

fr fr Test environment variable listing
sus env_vars tea[value] = environ()
assert_true(env_vars.length() > 0)
assert_true(env_vars[0].contains("PATH="))

test_start("Directory Operations Tests")

fr fr Test current working directory
sus cwd tea = getcwd()
assert_eq_string(cwd, "/home/user/projects")

fr fr Test changing directory
assert_true(chdir("/home/user"))
assert_true(chdir("./subdir"))
assert_true(chdir("../"))
assert_false(chdir(""))

test_start("Pipe Communication Tests")

fr fr Test pipe creation
sus pipe Pipe = create_pipe()
assert_true(pipe.read_fd > 0)
assert_true(pipe.write_fd > 0)

fr fr Test pipe write and read
sus bytes_written normie = pipe.write("Hello Pipe")
assert_eq_int(bytes_written, 10)

sus data tea = pipe.read(5)
assert_eq_string(data, "Hello")

sus remaining tea = pipe.read(10)
assert_eq_string(remaining, " Pipe")

fr fr Test pipe close
assert_true(pipe.close())

test_start("Process Statistics Tests")

fr fr Test process stats
sus stats ProcessStats = get_process_stats(getpid())
assert_eq_int(stats.cpu_percent, 15)
assert_eq_int(stats.memory_mb, 128)
assert_eq_int(stats.open_files, 12)
assert_eq_int(stats.threads, 4)
assert_eq_int(stats.uptime, 3600)

test_start("System Information Tests")

fr fr Test system info
sus sys_info tea = get_system_info()
assert_true(sys_info.contains("Linux"))
assert_true(sys_info.contains("x86_64"))

fr fr Test CPU count
sus cpu_count normie = get_cpu_count()
assert_eq_int(cpu_count, 8)

fr fr Test memory info
sus mem_info tea = get_memory_info()
assert_true(mem_info.contains("16GB"))

test_start("Process Group Tests")

fr fr Test process group creation
sus pgid normie = create_process_group()
assert_eq_int(pgid, getpid())

fr fr Test process group operations
assert_true(set_process_group(getpid(), pgid))
assert_false(set_process_group(0, 0))

sus current_pgid normie = get_process_group(getpid())
assert_eq_int(current_pgid, getpid())

test_start("Signal Tests")

fr fr Test signal constants
assert_eq_int(SIGTERM, 15)
assert_eq_int(SIGKILL, 9)
assert_eq_int(SIGINT, 2)

test_start("Command Execution with Options Tests")

fr fr Test execution with custom options
sus custom_options ProcessOptions = ProcessOptions{
    working_dir: "/tmp",
    env_vars: ["PATH=/usr/bin", "HOME=/tmp"],
    capture_output: based,
    timeout: 10000
}

sus custom_result CommandResult = exec_with_options("echo", ["test"], custom_options)
assert_eq_int(custom_result.exit_code, 0)
assert_true(custom_result.success)

test_start("Error Handling Tests")

fr fr Test failed command
sus false_result CommandResult = exec("false", [])
assert_eq_int(false_result.exit_code, 1)
assert_false(false_result.success)

fr fr Test nonexistent command
sus bad_result CommandResult = exec("nonexistent", [])
assert_eq_int(bad_result.exit_code, 127)
assert_false(bad_result.success)

test_start("Process Wait Tests")

fr fr Test waiting for process
sus wait_process Process = spawn("sleep", ["1"], options)
sus wait_result CommandResult = wait_for_process(wait_process)
assert_eq_int(wait_result.exit_code, 0)
assert_true(wait_result.success)
assert_true(wait_result.duration >= 0)

test_start("Process Kill Tests")

fr fr Test process killing
sus kill_process Process = spawn("sleep", ["10"], options)
assert_true(kill_process(kill_process))
assert_eq_int(kill_process.state, 3)  fr fr killed state
assert_eq_int(kill_process.exit_code, -9)

test_start("Signal Sending Tests")

fr fr Test sending SIGTERM
sus term_process Process = spawn("sleep", ["5"], options)
assert_true(send_signal(term_process, SIGTERM))
assert_eq_int(term_process.state, 1)  fr fr finished state
assert_eq_int(term_process.exit_code, 0)

fr fr Test sending SIGINT
sus int_process Process = spawn("sleep", ["5"], options)
assert_true(send_signal(int_process, SIGINT))
assert_eq_int(int_process.state, 1)  fr fr finished state
assert_eq_int(int_process.exit_code, 130)

print_test_summary()
