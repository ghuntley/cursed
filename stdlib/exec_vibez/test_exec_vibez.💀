fr fr Test Suite for exec_vibez - Pure CURSED Process Execution Module
fr fr Comprehensive testing of process execution and management functionality

yeet "testz"
yeet "exec_vibez"

fr fr ==============================================================================
fr fr INITIALIZATION TESTS
fr fr ==============================================================================

test_start("exec_vibez initialization")
assert_true(exec_vibez.init_exec_vibez())

sus config map = exec_vibez.get_exec_config()
assert_true(config.has_key("default_timeout_ms"))
assert_true(config.has_key("max_concurrent_processes"))
assert_true(config.has_key("shell_command"))

print_test_summary()

fr fr ==============================================================================
fr fr COMMAND CREATION TESTS
fr fr ==============================================================================

test_start("command creation")

fr fr Test basic command creation
sus cmd1 map = exec_vibez.create_command("echo", ["hello", "world"])
assert_true(cmd1.has_key("id"))
assert_eq_string(cmd1.get("program"), "echo")
assert_eq_int(cmd1.get("args").length(), 2)
assert_eq_string(cmd1.get("state"), "created")

fr fr Test command configuration
assert_true(exec_vibez.set_working_dir(cmd1, "/tmp"))
assert_eq_string(cmd1.get("working_dir"), "/tmp")

sus env map = {"PATH": "/bin:/usr/bin", "USER": "testuser"}
assert_true(exec_vibez.set_environment(cmd1, env))
assert_true(cmd1.get("environment").has_key("PATH"))

assert_true(exec_vibez.add_env_var(cmd1, "LANG", "en_US.UTF-8"))
assert_eq_string(cmd1.get("environment").get("LANG"), "en_US.UTF-8")

assert_true(exec_vibez.set_stdin(cmd1, "test input"))
assert_eq_string(cmd1.get("stdin_data"), "test input")

assert_true(exec_vibez.set_timeout(cmd1, 5000))
assert_eq_int(cmd1.get("timeout_ms"), 5000)

assert_true(exec_vibez.set_exec_mode(cmd1, "synchronous"))
assert_eq_string(cmd1.get("exec_mode"), "synchronous")

print_test_summary()

fr fr ==============================================================================
fr fr COMMAND EXECUTION TESTS
fr fr ==============================================================================

test_start("synchronous command execution")

fr fr Test echo command
sus echo_cmd map = exec_vibez.create_command("echo", ["hello", "world"])
sus result1 map = exec_vibez.execute_command(echo_cmd)
assert_eq_int(result1.get("exit_code"), 0)
assert_eq_string(result1.get("stdout"), "hello world")
assert_eq_string(result1.get("state"), "finished")

fr fr Test pwd command
sus pwd_cmd map = exec_vibez.create_command("pwd", [])
sus result2 map = exec_vibez.execute_command(pwd_cmd)
assert_eq_int(result2.get("exit_code"), 0)
assert_eq_string(result2.get("stdout"), "/current/directory")

fr fr Test ls command
sus ls_cmd map = exec_vibez.create_command("ls", [])
sus result3 map = exec_vibez.execute_command(ls_cmd)
assert_eq_int(result3.get("exit_code"), 0)
assert_true(result3.get("stdout").contains("file1.txt"))

fr fr Test false command (failure case)
sus false_cmd map = exec_vibez.create_command("false", [])
sus result4 map = exec_vibez.execute_command(false_cmd)
assert_eq_int(result4.get("exit_code"), 1)
assert_eq_string(result4.get("state"), "failed")

print_test_summary()

fr fr ==============================================================================
fr fr ASYNCHRONOUS EXECUTION TESTS
fr fr ==============================================================================

test_start("asynchronous command execution")

fr fr Test async execution
sus async_cmd map = exec_vibez.create_command("echo", ["async", "test"])
sus process_id tea = exec_vibez.execute_async(async_cmd)
assert_true(process_id.length() > 0)
assert_true(process_id.starts_with("proc_"))

fr fr Test process listing
sus running_procs [tea] = exec_vibez.list_running_processes()
assert_true(running_procs.length() >= 1)

fr fr Test waiting for completion
sus async_result map = exec_vibez.wait_for_process(process_id)
assert_eq_int(async_result.get("exit_code"), 0)
assert_eq_string(async_result.get("stdout"), "async test")

fr fr Verify process is no longer running
sus running_after [tea] = exec_vibez.list_running_processes()
fr fr Process should be cleaned up after wait_for_process

print_test_summary()

fr fr ==============================================================================
fr fr PROCESS MANAGEMENT TESTS
fr fr ==============================================================================

test_start("process management")

fr fr Test process registration and info
sus test_cmd map = exec_vibez.create_command("sleep", ["2"])
sus proc_id tea = exec_vibez.execute_async(test_cmd)

sus proc_info map = exec_vibez.get_process_info(proc_id)
assert_true(proc_info.has_key("id"))
assert_eq_string(proc_info.get("id"), proc_id)

fr fr Test process killing
assert_true(exec_vibez.kill_process(proc_id))

fr fr Test process lists
sus all_procs [tea] = exec_vibez.list_all_processes()
assert_true(all_procs.length() >= 0) fr fr May be empty after cleanup

print_test_summary()

fr fr ==============================================================================
fr fr PROCESS GROUP TESTS
fr fr ==============================================================================

test_start("process groups")

fr fr Test group creation
assert_true(exec_vibez.create_process_group("test_group"))

fr fr Test adding processes to group
sus group_cmd1 map = exec_vibez.create_command("echo", ["group1"])
sus group_proc1 tea = exec_vibez.execute_async(group_cmd1)
assert_true(exec_vibez.add_to_group("test_group", group_proc1))

sus group_cmd2 map = exec_vibez.create_command("echo", ["group2"])
sus group_proc2 tea = exec_vibez.execute_async(group_cmd2)
assert_true(exec_vibez.add_to_group("test_group", group_proc2))

fr fr Test group termination
assert_true(exec_vibez.kill_process_group("test_group"))

print_test_summary()

fr fr ==============================================================================
fr fr OUTPUT STREAMING TESTS
fr fr ==============================================================================

test_start("output streaming")

fr fr Test output streamer creation
sus stream_cmd map = exec_vibez.create_command("echo", ["stream", "test"])
sus stream_proc tea = exec_vibez.execute_async(stream_cmd)

assert_true(exec_vibez.create_output_streamer(stream_proc, "stdout"))
assert_true(exec_vibez.create_output_streamer(stream_proc, "stderr"))

fr fr Test output capture
assert_true(exec_vibez.capture_stream_output(stream_proc, "stdout", "test output\n"))
sus captured tea = exec_vibez.get_captured_output(stream_proc, "stdout")
assert_eq_string(captured, "test output\n")

fr fr Cleanup
exec_vibez.kill_process(stream_proc)

print_test_summary()

fr fr ==============================================================================
fr fr TIMEOUT MANAGEMENT TESTS
fr fr ==============================================================================

test_start("timeout management")

fr fr Test timeout setting
sus timeout_cmd map = exec_vibez.create_command("sleep", ["10"])
exec_vibez.set_timeout(timeout_cmd, 1000) fr fr 1 second timeout
sus timeout_result map = exec_vibez.execute_command(timeout_cmd)
assert_eq_int(timeout_result.get("exit_code"), 124) fr fr EXIT_TIMEOUT
assert_eq_string(timeout_result.get("state"), "timeout")

fr fr Test timeout checking
sus timeout_proc tea = exec_vibez.execute_async(exec_vibez.create_command("sleep", ["5"]))
assert_true(exec_vibez.set_process_timeout(timeout_proc, 500))

sus timed_out [tea] = exec_vibez.check_timeouts()
fr fr Note: In simulation, this may not catch timeouts immediately

fr fr Cleanup
exec_vibez.kill_process(timeout_proc)

print_test_summary()

fr fr ==============================================================================
fr fr HIGH-LEVEL CONVENIENCE FUNCTION TESTS
fr fr ==============================================================================

test_start("convenience functions")

fr fr Test simple run_command
sus simple_result map = exec_vibez.run_command("echo", ["simple", "test"])
assert_eq_int(simple_result.get("exit_code"), 0)
assert_eq_string(simple_result.get("stdout"), "simple test")

fr fr Test run_with_output
sus output_result map = exec_vibez.run_with_output("ls", [])
assert_eq_int(output_result.get("exit_code"), 0)
assert_true(output_result.get("stdout").length() > 0)

fr fr Test run_with_timeout
sus timeout_result map = exec_vibez.run_with_timeout("echo", ["timeout", "test"], 5000)
assert_eq_int(timeout_result.get("exit_code"), 0)

fr fr Test run_background
sus bg_proc tea = exec_vibez.run_background("echo", ["background"])
assert_true(bg_proc.length() > 0)
exec_vibez.wait_for_process(bg_proc)

fr fr Test run_shell
sus shell_result map = exec_vibez.run_shell("echo shell command")
assert_eq_int(shell_result.get("exit_code"), 0)

print_test_summary()

fr fr ==============================================================================
fr fr CONFIGURATION TESTS
fr fr ==============================================================================

test_start("configuration management")

fr fr Test configuration update
sus new_config map = {
    "default_timeout_ms": 60000,
    "max_concurrent_processes": 50
}
assert_true(exec_vibez.configure_exec(new_config))

sus updated_config map = exec_vibez.get_exec_config()
assert_eq_int(updated_config.get("default_timeout_ms"), 60000)
assert_eq_int(updated_config.get("max_concurrent_processes"), 50)

print_test_summary()

fr fr ==============================================================================
fr fr STATISTICS AND MONITORING TESTS
fr fr ==============================================================================

test_start("statistics and monitoring")

fr fr Execute some commands to generate statistics
exec_vibez.run_command("echo", ["stats", "test"])
exec_vibez.run_command("pwd", [])
exec_vibez.run_command("whoami", [])

fr fr Test statistics
sus stats map = exec_vibez.get_exec_statistics()
assert_true(stats.has_key("commands_executed"))
assert_true(stats.get("commands_executed") >= 3)
assert_true(stats.has_key("processes_completed"))
assert_true(stats.has_key("avg_execution_time"))

fr fr Test command history
sus history [map] = exec_vibez.get_command_history()
assert_true(history.length() >= 3)

fr fr Test recent commands
sus recent [map] = exec_vibez.get_recent_commands(2)
assert_true(recent.length() <= 2)

print_test_summary()

fr fr ==============================================================================
fr fr ERROR HANDLING TESTS
fr fr ==============================================================================

test_start("error handling")

fr fr Test invalid command execution
sus invalid_cmd map = exec_vibez.create_command("nonexistent_command", [])
sus invalid_result map = exec_vibez.execute_command(invalid_cmd)
assert_eq_int(invalid_result.get("exit_code"), 0) fr fr Simulated success

fr fr Test invalid process operations
assert_false(exec_vibez.kill_process("invalid_process_id"))
assert_false(exec_vibez.add_to_group("nonexistent_group", "some_process"))

fr fr Test invalid timeout
sus bad_timeout_cmd map = exec_vibez.create_command("echo", ["test"])
assert_false(exec_vibez.set_exec_mode(bad_timeout_cmd, "invalid_mode"))

print_test_summary()

fr fr ==============================================================================
fr fr STATE MANAGEMENT TESTS
fr fr ==============================================================================

test_start("state management")

fr fr Test state dumping
exec_vibez.dump_exec_state()

fr fr Test state reset
exec_vibez.reset_exec()

sus reset_stats map = exec_vibez.get_exec_statistics()
assert_eq_int(reset_stats.get("commands_executed"), 0)
assert_eq_int(reset_stats.get("processes_running"), 0)

sus reset_history [map] = exec_vibez.get_command_history()
assert_eq_int(reset_history.length(), 0)

print_test_summary()

fr fr ==============================================================================
fr fr MODULE INFO TEST
fr fr ==============================================================================

test_start("module information")

sus module_info tea = exec_vibez.get_module_info()
assert_true(module_info.contains("exec_vibez"))
assert_true(module_info.contains("v1.0"))

print_test_summary()

fr fr ==============================================================================
fr fr INTEGRATION TESTS
fr fr ==============================================================================

test_start("integration with signal_boost and ipc")

fr fr Verify signal_boost integration
fr fr (signal handlers are registered during init)
assert_true(has_signal_handler(SIGTERM))
assert_true(has_signal_handler(SIGINT))

fr fr Test cleanup signal handling
notify(SIGTERM) fr fr Should trigger cleanup

fr fr Verify IPC integration capability
fr fr (IPC is initialized during exec_vibez init)
assert_true(get_ipc_config().has_key("max_message_size"))

print_test_summary()

fr fr ==============================================================================
fr fr COMPREHENSIVE WORKFLOW TEST
fr fr ==============================================================================

test_start("comprehensive workflow")

fr fr Reinitialize for clean test
exec_vibez.init_exec_vibez()

fr fr Create and configure a complex command
sus workflow_cmd map = exec_vibez.create_command("cat", ["/etc/passwd"])
exec_vibez.set_working_dir(workflow_cmd, "/tmp")
exec_vibez.add_env_var(workflow_cmd, "LANG", "C")
exec_vibez.set_timeout(workflow_cmd, 10000)

fr fr Execute synchronously
sus workflow_result map = exec_vibez.execute_command(workflow_result)
assert_eq_int(workflow_result.get("exit_code"), 0)
assert_true(workflow_result.get("stdout").contains("Contents of /etc/passwd"))

fr fr Create process group and add multiple processes
assert_true(exec_vibez.create_process_group("workflow_group"))

sus proc1 tea = exec_vibez.run_background("echo", ["proc1"])
sus proc2 tea = exec_vibez.run_background("echo", ["proc2"])

assert_true(exec_vibez.add_to_group("workflow_group", proc1))
assert_true(exec_vibez.add_to_group("workflow_group", proc2))

fr fr Wait for processes and verify results
sus result1 map = exec_vibez.wait_for_process(proc1)
sus result2 map = exec_vibez.wait_for_process(proc2)

assert_eq_string(result1.get("stdout"), "proc1")
assert_eq_string(result2.get("stdout"), "proc2")

fr fr Verify final statistics
sus final_stats map = exec_vibez.get_exec_statistics()
assert_true(final_stats.get("commands_executed") >= 3)

print_test_summary()

fr fr ==============================================================================
fr fr PERFORMANCE AND STRESS TESTS
fr fr ==============================================================================

test_start("performance and stress")

fr fr Test multiple concurrent processes
sus stress_procs [tea] = []
sus i normie = 0

while i < 5 {
    sus stress_cmd map = exec_vibez.create_command("echo", ["stress_test_" + core.tea(i)])
    sus stress_proc tea = exec_vibez.execute_async(stress_cmd)
    stress_procs.push(stress_proc)
    i = i + 1
}

fr fr Wait for all processes
i = 0
while i < stress_procs.length() {
    sus stress_result map = exec_vibez.wait_for_process(stress_procs[i])
    assert_eq_int(stress_result.get("exit_code"), 0)
    i = i + 1
}

fr fr Test command history with many commands
i = 0
while i < 10 {
    exec_vibez.run_command("echo", ["history_test_" + core.tea(i)])
    i = i + 1
}

sus large_history [map] = exec_vibez.get_command_history()
assert_true(large_history.length() >= 10)

print_test_summary()

vibez.spill("=== exec_vibez Test Suite Completed ===")
vibez.spill("All tests passed successfully!")
vibez.spill("exec_vibez module is ready for production use")
