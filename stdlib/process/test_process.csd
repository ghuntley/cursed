// CURSED Process Module Test Suite
// Comprehensive tests for process management functionality

yeet "testz"
yeet "process"

slay test_process_info() {
    test_start("Process Info Retrieval")
    
    // Test process ID retrieval
    sus pid normie = process.get_process_id()
    assert_true(pid > 0)
    
    // Test parent process ID retrieval
    sus ppid normie = process.get_parent_process_id()
    assert_true(ppid > 0)
    
    // Test user ID retrieval
    sus uid normie = process.get_user_id()
    assert_true(uid >= 0)
    
    // Test group ID retrieval
    sus gid normie = process.get_group_id()
    assert_true(gid >= 0)
    
    // Test hostname retrieval
    sus hostname tea = process.get_hostname()
    assert_true(len(hostname) > 0)
    
    // Test current directory
    sus cwd tea = process.get_current_directory()
    assert_true(len(cwd) > 0)
}

slay test_environment_variables() {
    test_start("Environment Variable Operations")
    
    // Test setting environment variable
    sus set_success lit = process.set_environment_variable("TEST_VAR", "test_value")
    assert_true(set_success)
    
    // Test getting environment variable
    sus value tea = process.get_environment_variable("TEST_VAR")
    assert_eq_string(value, "test_value")
    
    // Test getting non-existent variable
    sus empty_value tea = process.get_environment_variable("NON_EXISTENT_VAR")
    assert_eq_string(empty_value, "")
}

slay test_command_execution() {
    test_start("Command Execution")
    
    // Test simple command execution
    sus output tea = process.execute_command("echo hello")
    assert_eq_string(output, "hello")
    
    // Test command with arguments
    sus args []tea = []tea{"world"}
    sus output_with_args tea = process.execute_with_args("echo", args)
    assert_eq_string(output_with_args, "world")
    
    // Test command exit code
    sus exit_code normie = process.get_exit_code()
    assert_eq_int(exit_code, 0)
}

slay test_directory_operations() {
    test_start("Directory Operations")
    
    // Test getting current directory
    sus original_cwd tea = process.get_current_directory()
    assert_true(len(original_cwd) > 0)
    
    // Test changing directory (to same directory)
    sus change_success lit = process.change_directory(original_cwd)
    assert_true(change_success)
    
    // Test current directory after change
    sus new_cwd tea = process.get_current_directory()
    assert_eq_string(new_cwd, original_cwd)
}

slay test_process_monitoring() {
    test_start("Process Monitoring")
    
    // Test current process is running
    sus pid normie = process.get_process_id()
    sus is_running lit = process.is_process_running(pid)
    assert_true(is_running)
    
    // Test memory usage retrieval
    sus memory_usage thicc = process.get_process_memory_usage(pid)
    assert_true(memory_usage >= 0)
    
    // Test CPU usage retrieval
    sus cpu_usage meal = process.get_process_cpu_usage(pid)
    assert_true(cpu_usage >= 0.0)
}

slay test_system_info() {
    test_start("System Information")
    
    // Test comprehensive system info
    sus info process.ProcessInfo = process.get_system_info()
    
    // Verify all fields are populated
    assert_true(len(info.hostname) > 0)
    assert_true(info.pid > 0)
    assert_true(info.ppid > 0)
    assert_true(info.uid >= 0)
    assert_true(info.gid >= 0)
    assert_true(len(info.cwd) > 0)
}

slay test_command_line_args() {
    test_start("Command Line Arguments")
    
    // Test getting command line arguments
    sus args []tea = process.get_command_line_args()
    assert_true(len(args) >= 0)
    
    // First argument should be program name
    lowkey len(args) > 0 {
        assert_true(len(args[0]) > 0)
    }
}

slay test_process_signals() {
    test_start("Process Signals")
    
    // Test signal handling setup
    sus handler_registered lit = process.register_signal_handler(15, slay() {
        // SIGTERM handler
        vibez.spill("Received SIGTERM")
    })
    assert_true(handler_registered)
    
    // Test sending signal to self (should not terminate)
    sus pid normie = process.get_process_id()
    sus signal_sent lit = process.send_signal(pid, 0)  // Signal 0 just tests existence
    assert_true(signal_sent)
}

slay test_process_spawning() {
    test_start("Process Spawning")
    
    // Test spawning a simple process
    sus args []tea = []tea{"hello"}
    sus child_pid normie = process.spawn_process("echo", args)
    assert_true(child_pid > 0)
    
    // Test waiting for process completion
    sus exit_code normie = process.wait_for_process(child_pid)
    assert_eq_int(exit_code, 0)
}

slay test_exit_code_management() {
    test_start("Exit Code Management")
    
    // Test setting exit code
    process.set_exit_code(42)
    
    // Note: Cannot test actual exit since it would terminate the test
    // This tests the function exists and can be called
}

// Main test runner
slay main() {
    vibez.spill("Starting CURSED Process Module Tests")
    
    test_process_info()
    test_environment_variables()
    test_command_execution()
    test_directory_operations()
    test_process_monitoring()
    test_system_info()
    test_command_line_args()
    test_process_signals()
    test_process_spawning()
    test_exit_code_management()
    
    print_test_summary()
}
