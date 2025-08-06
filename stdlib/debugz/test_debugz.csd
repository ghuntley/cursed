fr fr Test suite for debugz module - Debug utilities and profiling
yeet "testz"
yeet "debugz"

fr fr ===== DEBUG CONFIGURATION TESTS =====

slay test_debug_configuration() lit {
    testz.test_start("Debug configuration")
    
    fr fr Test initial configuration
    testz.assert_true(debugz.is_debug_enabled())
    
    sus initial_config debugz.DebugConfig = debugz.get_debug_config()
    testz.assert_true(initial_config.enabled)
    testz.assert_eq_int(initial_config.log_level, debugz.DEBUG_LEVEL_INFO)
    testz.assert_true(initial_config.stack_trace_enabled)
    
    fr fr Test configuration changes
    sus new_config debugz.DebugConfig = debugz.DebugConfig{
        enabled: based,
        log_level: debugz.DEBUG_LEVEL_DEBUG,
        stack_trace_enabled: based,
        memory_tracking: cap,
        performance_profiling: based,
        breakpoints_enabled: based,
        verbose_logging: cap
    }
    
    testz.assert_true(debugz.configure_debug(new_config))
    
    sus updated_config debugz.DebugConfig = debugz.get_debug_config()
    testz.assert_eq_int(updated_config.log_level, debugz.DEBUG_LEVEL_DEBUG)
    testz.assert_true(updated_config.performance_profiling)
    testz.assert_true(updated_config.breakpoints_enabled)
    testz.assert_false(updated_config.memory_tracking)
    
    fr fr Test debug level setting
    testz.assert_true(debugz.set_debug_level(debugz.DEBUG_LEVEL_TRACE))
    sus final_config debugz.DebugConfig = debugz.get_debug_config()
    testz.assert_eq_int(final_config.log_level, debugz.DEBUG_LEVEL_TRACE)
    
    damn based
}

fr fr ===== LOGGING TESTS =====

slay test_logging_functions() lit {
    testz.test_start("Logging functions")
    
    fr fr Enable verbose logging for this test
    sus config debugz.DebugConfig = debugz.get_debug_config()
    config.verbose_logging = based
    config.log_level = debugz.DEBUG_LEVEL_TRACE
    debugz.configure_debug(config)
    
    fr fr Test different log levels
    testz.assert_true(debugz.log_error("Test error message"))
    testz.assert_true(debugz.log_warn("Test warning message"))
    testz.assert_true(debugz.log_info("Test info message"))
    testz.assert_true(debugz.log_debug("Test debug message"))
    testz.assert_true(debugz.log_trace("Test trace message"))
    
    fr fr Test logging with arguments
    testz.assert_true(debugz.log_info("Test with args: ", "arg1", "arg2"))
    testz.assert_true(debugz.log_debug("Number: ", "42", " String: ", "test"))
    
    fr fr Test log buffer
    sus log_entries []tea = debugz.get_debug_log()
    testz.assert_true(log_entries.len() > 0)
    
    fr fr Clear log and verify
    testz.assert_true(debugz.clear_debug_log())
    sus cleared_log []tea = debugz.get_debug_log()
    testz.assert_eq_int(cleared_log.len(), 0)
    
    damn based
}

slay test_log_level_filtering() lit {
    testz.test_start("Log level filtering")
    
    fr fr Set to INFO level
    debugz.set_debug_level(debugz.DEBUG_LEVEL_INFO)
    debugz.clear_debug_log()
    
    fr fr These should be logged (INFO and above)
    debugz.log_error("Error message")
    debugz.log_warn("Warning message")
    debugz.log_info("Info message")
    
    fr fr These should NOT be logged (below INFO)
    debugz.log_debug("Debug message")
    debugz.log_trace("Trace message")
    
    sus log_entries []tea = debugz.get_debug_log()
    testz.assert_eq_int(log_entries.len(), 3)  fr fr Only ERROR, WARN, INFO
    
    fr fr Test with DEBUG level
    debugz.set_debug_level(debugz.DEBUG_LEVEL_DEBUG)
    debugz.clear_debug_log()
    
    debugz.log_debug("Debug message")
    debugz.log_trace("Trace message")  fr fr Should not be logged
    
    sus debug_log []tea = debugz.get_debug_log()
    testz.assert_eq_int(debug_log.len(), 1)  fr fr Only DEBUG
    
    damn based
}

fr fr ===== STACK TRACE TESTS =====

slay test_stack_trace_operations() lit {
    testz.test_start("Stack trace operations")
    
    fr fr Ensure stack tracing is enabled
    sus config debugz.DebugConfig = debugz.get_debug_config()
    config.stack_trace_enabled = based
    debugz.configure_debug(config)
    
    fr fr Test stack frame operations
    testz.assert_true(debugz.push_stack_frame("test_function", "test_file.csd", 42))
    testz.assert_eq_string(debugz.get_current_function(), "test_function")
    testz.assert_eq_int(debugz.get_call_depth(), 1)
    
    fr fr Push another frame
    testz.assert_true(debugz.push_stack_frame("nested_function", "test_file.csd", 84))
    testz.assert_eq_string(debugz.get_current_function(), "nested_function")
    testz.assert_eq_int(debugz.get_call_depth(), 2)
    
    fr fr Get full stack trace
    sus stack_trace []debugz.StackFrame = debugz.get_stack_trace()
    testz.assert_eq_int(stack_trace.len(), 2)
    testz.assert_eq_string(stack_trace[0].function_name, "test_function")
    testz.assert_eq_string(stack_trace[1].function_name, "nested_function")
    testz.assert_eq_int(stack_trace[0].line_number, 42)
    testz.assert_eq_int(stack_trace[1].line_number, 84)
    
    fr fr Pop frames
    testz.assert_true(debugz.pop_stack_frame())
    testz.assert_eq_int(debugz.get_call_depth(), 1)
    testz.assert_eq_string(debugz.get_current_function(), "test_function")
    
    testz.assert_true(debugz.pop_stack_frame())
    testz.assert_eq_int(debugz.get_call_depth(), 0)
    
    damn based
}

slay test_variable_inspection() lit {
    testz.test_start("Variable inspection")
    
    fr fr Set up a stack frame
    debugz.push_stack_frame("test_variables", "test.csd", 100)
    
    fr fr Add some local variables
    testz.assert_true(debugz.add_local_variable("count", typez.get_type_by_name("normie"), 42, 0x1000))
    testz.assert_true(debugz.add_local_variable("name", typez.get_type_by_name("tea"), 0, 0x2000))
    testz.assert_true(debugz.add_local_variable("active", typez.get_type_by_name("lit"), 1, 0x3000))
    
    fr fr Inspect variables
    sus count_var debugz.VariableInfo = debugz.inspect_variable("count")
    testz.assert_eq_string(count_var.name, "count")
    testz.assert_eq_string(count_var.type_info.name, "normie")
    testz.assert_eq_int(count_var.value, 42)
    testz.assert_eq_int(count_var.address, 0x1000)
    
    sus name_var debugz.VariableInfo = debugz.inspect_variable("name")
    testz.assert_eq_string(name_var.name, "name")
    testz.assert_eq_string(name_var.type_info.name, "tea")
    testz.assert_eq_int(name_var.address, 0x2000)
    
    fr fr Test unknown variable
    sus unknown_var debugz.VariableInfo = debugz.inspect_variable("nonexistent")
    testz.assert_eq_string(unknown_var.name, "not_found")
    
    fr fr Test printing variables (should not crash)
    debugz.print_local_variables()
    
    fr fr Clean up
    debugz.pop_stack_frame()
    damn based
}

fr fr ===== PROFILING TESTS =====

slay test_profiling_operations() lit {
    testz.test_start("Performance profiling")
    
    fr fr Enable profiling
    sus config debugz.DebugConfig = debugz.get_debug_config()
    config.performance_profiling = based
    debugz.configure_debug(config)
    
    fr fr Start profiling a function
    testz.assert_true(debugz.start_profiling("test_function_1"))
    testz.assert_true(debugz.start_profiling("test_function_2"))
    
    fr fr End profiling with execution times
    testz.assert_true(debugz.end_profiling("test_function_1", 1000000))  fr fr 1ms in nanoseconds
    testz.assert_true(debugz.end_profiling("test_function_2", 2000000))  fr fr 2ms in nanoseconds
    
    fr fr Start and end same function again
    testz.assert_true(debugz.start_profiling("test_function_1"))
    testz.assert_true(debugz.end_profiling("test_function_1", 1500000))  fr fr 1.5ms
    
    fr fr Get profiler results
    sus results []debugz.ProfilerResult = debugz.get_profiler_results()
    testz.assert_eq_int(results.len(), 2)
    
    fr fr Check first function results
    sus func1_result debugz.ProfilerResult = results[0]
    testz.assert_eq_string(func1_result.function_name, "test_function_1")
    testz.assert_eq_int(func1_result.call_count, 2)
    testz.assert_eq_int(func1_result.total_time_ns, 2500000)  fr fr 1ms + 1.5ms
    testz.assert_eq_int(func1_result.average_time_ns, 1250000)  fr fr 2.5ms / 2
    testz.assert_eq_int(func1_result.min_time_ns, 1000000)
    testz.assert_eq_int(func1_result.max_time_ns, 1500000)
    
    fr fr Test profiler report generation
    debugz.print_profiler_report()
    
    fr fr Reset profiler
    testz.assert_true(debugz.reset_profiler())
    sus reset_results []debugz.ProfilerResult = debugz.get_profiler_results()
    testz.assert_eq_int(reset_results.len(), 0)
    
    damn based
}

fr fr ===== BREAKPOINT TESTS =====

slay test_breakpoint_management() lit {
    testz.test_start("Breakpoint management")
    
    fr fr Enable breakpoints
    sus config debugz.DebugConfig = debugz.get_debug_config()
    config.breakpoints_enabled = based
    debugz.configure_debug(config)
    
    fr fr Add breakpoints
    sus bp1_id normie = debugz.add_breakpoint("test.csd", 42, "")
    sus bp2_id normie = debugz.add_breakpoint("test.csd", 84, "count > 10")
    sus bp3_id normie = debugz.add_breakpoint("other.csd", 100, "based")
    
    testz.assert_true(bp1_id > 0)
    testz.assert_true(bp2_id > 0)
    testz.assert_true(bp3_id > 0)
    testz.assert_true(bp1_id != bp2_id)
    testz.assert_true(bp2_id != bp3_id)
    
    fr fr Test breakpoint checking
    testz.assert_true(debugz.check_breakpoint("test.csd", 42))   fr fr Should hit
    testz.assert_false(debugz.check_breakpoint("test.csd", 43))  fr fr Should not hit
    testz.assert_true(debugz.check_breakpoint("test.csd", 84))   fr fr Should hit
    testz.assert_true(debugz.check_breakpoint("other.csd", 100)) fr fr Should hit
    
    fr fr Test breakpoint enable/disable
    testz.assert_true(debugz.disable_breakpoint(bp1_id))
    testz.assert_false(debugz.check_breakpoint("test.csd", 42))  fr fr Should not hit when disabled
    
    testz.assert_true(debugz.enable_breakpoint(bp1_id))
    testz.assert_true(debugz.check_breakpoint("test.csd", 42))   fr fr Should hit when re-enabled
    
    fr fr Test breakpoint removal
    testz.assert_true(debugz.remove_breakpoint(bp2_id))
    testz.assert_false(debugz.check_breakpoint("test.csd", 84))  fr fr Should not hit after removal
    
    fr fr Test listing breakpoints
    debugz.list_breakpoints()  fr fr Should not crash
    
    fr fr Clean up remaining breakpoints
    debugz.remove_breakpoint(bp1_id)
    debugz.remove_breakpoint(bp3_id)
    
    damn based
}

slay test_breakpoint_conditions() lit {
    testz.test_start("Breakpoint conditions")
    
    fr fr Add conditional breakpoints
    sus bp_true normie = debugz.add_breakpoint("test.csd", 10, "based")
    sus bp_false normie = debugz.add_breakpoint("test.csd", 20, "cap")
    sus bp_empty normie = debugz.add_breakpoint("test.csd", 30, "")
    
    fr fr Test condition evaluation
    testz.assert_true(debugz.check_breakpoint("test.csd", 10))   fr fr "based" should be true
    testz.assert_false(debugz.check_breakpoint("test.csd", 20))  fr fr "cap" should be false
    testz.assert_true(debugz.check_breakpoint("test.csd", 30))   fr fr Empty condition should be true
    
    fr fr Clean up
    debugz.remove_breakpoint(bp_true)
    debugz.remove_breakpoint(bp_false)
    debugz.remove_breakpoint(bp_empty)
    
    damn based
}

fr fr ===== ASSERTION TESTS =====

slay test_debug_assertions() lit {
    testz.test_start("Debug assertions")
    
    fr fr Test successful assertions
    testz.assert_true(debugz.debug_assert(based, "This should pass"))
    testz.assert_true(debugz.debug_assert_eq(42, 42, "Numbers should be equal"))
    testz.assert_true(debugz.debug_assert_not_null(0x1000, "Pointer should not be null"))
    
    fr fr Test failing assertions (should return false but not crash)
    testz.assert_false(debugz.debug_assert(cap, "This should fail"))
    testz.assert_false(debugz.debug_assert_eq(42, 43, "Numbers should not be equal"))
    testz.assert_false(debugz.debug_assert_not_null(0, "Null pointer assertion"))
    
    damn based
}

fr fr ===== MEMORY DEBUGGING TESTS =====

slay test_memory_debugging() lit {
    testz.test_start("Memory debugging")
    
    fr fr Enable memory tracking
    sus config debugz.DebugConfig = debugz.get_debug_config()
    config.memory_tracking = based
    debugz.configure_debug(config)
    
    fr fr Test memory tracking
    testz.assert_true(debugz.track_memory_allocation(0x1000, 1024, "TestType"))
    testz.assert_true(debugz.track_memory_deallocation(0x1000, 1024))
    
    fr fr Test memory corruption checking
    testz.assert_false(debugz.check_memory_corruption(0, 100))  fr fr Null pointer
    testz.assert_true(debugz.check_memory_corruption(0x1000, 100))  fr fr Valid pointer (simplified)
    
    damn based
}

fr fr ===== UTILITY FUNCTION TESTS =====

slay test_debug_utilities() lit {
    testz.test_start("Debug utility functions")
    
    fr fr Test debug state dumping
    debugz.dump_debug_state()  fr fr Should not crash
    
    fr fr Test debug help
    debugz.print_debug_help()  fr fr Should not crash
    
    fr fr Test timestamp and thread functions
    sus timestamp tea = debugz.get_current_timestamp()
    testz.assert_true(timestamp.len() > 0)
    
    sus thread_id normie = debugz.get_current_thread_id()
    testz.assert_true(thread_id > 0)
    
    fr fr Test hex formatting
    testz.assert_eq_string(debugz.format_hex(0), "0")
    testz.assert_ne_string(debugz.format_hex(255), "0")
    
    damn based
}

fr fr ===== ERROR HANDLING TESTS =====

slay test_debug_error_handling() lit {
    testz.test_start("Debug error handling")
    
    fr fr Test operations when debugging is disabled
    sus config debugz.DebugConfig = debugz.get_debug_config()
    config.enabled = cap
    debugz.configure_debug(config)
    
    fr fr These should handle disabled state gracefully
    testz.assert_true(debugz.log_info("This should be ignored"))
    testz.assert_false(debugz.push_stack_frame("test", "test.csd", 1))
    testz.assert_false(debugz.start_profiling("test"))
    
    fr fr Re-enable debugging
    config.enabled = based
    debugz.configure_debug(config)
    
    fr fr Test invalid operations
    testz.assert_false(debugz.pop_stack_frame())  fr fr No frames to pop
    testz.assert_false(debugz.end_profiling("nonexistent", 1000))  fr fr Function not being profiled
    testz.assert_false(debugz.remove_breakpoint(99999))  fr fr Non-existent breakpoint
    testz.assert_false(debugz.enable_breakpoint(99999))   fr fr Non-existent breakpoint
    testz.assert_false(debugz.disable_breakpoint(99999))  fr fr Non-existent breakpoint
    
    damn based
}

fr fr ===== INTEGRATION TESTS =====

slay test_debug_integration() lit {
    testz.test_start("Debug system integration")
    
    fr fr Set up full debug environment
    sus config debugz.DebugConfig = debugz.DebugConfig{
        enabled: based,
        log_level: debugz.DEBUG_LEVEL_DEBUG,
        stack_trace_enabled: based,
        memory_tracking: based,
        performance_profiling: based,
        breakpoints_enabled: based,
        verbose_logging: cap  fr fr Keep output clean for test
    }
    debugz.configure_debug(config)
    
    fr fr Simulate a function execution with full debugging
    debugz.push_stack_frame("integration_test", "test.csd", 200)
    debugz.start_profiling("integration_test")
    debugz.add_local_variable("test_var", typez.get_type_by_name("normie"), 123, 0x4000)
    
    fr fr Add a breakpoint
    sus bp_id normie = debugz.add_breakpoint("test.csd", 200, "")
    testz.assert_true(debugz.check_breakpoint("test.csd", 200))
    
    fr fr Simulate some logging
    debugz.log_debug("Executing integration test")
    debugz.log_info("Test variable value: ", "123")
    
    fr fr Track memory operations
    debugz.track_memory_allocation(0x5000, 512, "IntegrationTestData")
    testz.assert_true(debugz.check_memory_corruption(0x5000, 512))
    debugz.track_memory_deallocation(0x5000, 512)
    
    fr fr End profiling and function
    debugz.end_profiling("integration_test", 5000000)  fr fr 5ms
    debugz.pop_stack_frame()
    
    fr fr Verify state
    sus profiler_results []debugz.ProfilerResult = debugz.get_profiler_results()
    testz.assert_eq_int(profiler_results.len(), 1)
    testz.assert_eq_string(profiler_results[0].function_name, "integration_test")
    
    sus log_entries []tea = debugz.get_debug_log()
    testz.assert_true(log_entries.len() > 0)
    
    fr fr Clean up
    debugz.remove_breakpoint(bp_id)
    debugz.reset_profiler()
    debugz.clear_debug_log()
    
    damn based
}

fr fr ===== RUN ALL TESTS =====

slay run_all_debugz_tests() lit {
    testz.test_group_start("debugz module tests")
    
    fr fr Initialize debug system for tests
    debugz.init_debug_system()
    
    test_debug_configuration()
    test_logging_functions()
    test_log_level_filtering()
    test_stack_trace_operations()
    test_variable_inspection()
    test_profiling_operations()
    test_breakpoint_management()
    test_breakpoint_conditions()
    test_debug_assertions()
    test_memory_debugging()
    test_debug_utilities()
    test_debug_error_handling()
    test_debug_integration()
    
    testz.test_group_end()
    testz.print_test_summary()
    damn based
}

fr fr Run the tests
run_all_debugz_tests()
