fr fr Enhanced Concurrency Testing Suite - Real OS Primitives and Timing Validation
fr fr Comprehensive test for replaced simplified implementations with actual OS integration

yeet "concurrenz"
yeet "os_primitives"  
yeet "real_goroutine_tracking"
yeet "testz"
yeet "vibez"

fr fr =============================================================================
fr fr REAL TIMING MECHANISM TESTS
fr fr =============================================================================

fr fr Test high-resolution timestamp functionality
slay test_real_timing_mechanisms() lit {
    vibez.spill("🕐 Testing Real Timing Mechanisms...")
    
    fr fr Test high-resolution timestamp
    sus start_time thicc = concurrenz.get_time_ns()
    testz.assert_greater_than_i64(start_time, 0, "High-resolution timestamp should be positive")
    
    fr fr Test microsecond precision sleep
    sus pre_sleep_time thicc = concurrenz.get_time_ns()
    concurrenz.sleep_us(1000)  fr fr 1ms = 1000 microseconds
    sus post_sleep_time thicc = concurrenz.get_time_ns()
    
    sus sleep_duration thicc = post_sleep_time - pre_sleep_time
    testz.assert_greater_than_i64(sleep_duration, 800000, "Sleep should be at least 800µs")  fr fr Allow some variance
    testz.assert_less_than_i64(sleep_duration, 2000000, "Sleep should be under 2ms")
    
    fr fr Test nanosecond precision sleep (will be rounded to microseconds)
    sus pre_nanosleep thicc = concurrenz.get_time_ns()
    concurrenz.sleep_ns(500000)  fr fr 500µs in nanoseconds
    sus post_nanosleep thicc = concurrenz.get_time_ns()
    
    sus nanosleep_duration thicc = post_nanosleep - pre_nanosleep
    testz.assert_greater_than_i64(nanosleep_duration, 400000, "Nanosecond sleep converted to microseconds")
    
    fr fr Test CPU pause instruction
    concurrenz.cpu_pause()  fr fr Should not crash
    
    vibez.spill("✅ Real timing mechanisms test passed")
    damn based
}

fr fr =============================================================================
fr fr REAL GOROUTINE TRACKING TESTS
fr fr =============================================================================

fr fr Test goroutine ID tracking and context management
slay test_real_goroutine_tracking() lit {
    vibez.spill("🎯 Testing Real Goroutine Tracking...")
    
    fr fr Initialize goroutine registry
    testz.assert_true(real_goroutine_tracking.init_goroutine_registry(), "Goroutine registry should initialize")
    
    fr fr Test current goroutine ID (should be main goroutine = 0)
    sus main_goroutine_id thicc = real_goroutine_tracking.get_current_goroutine_id()
    testz.assert_equal_i64(main_goroutine_id, 0, "Main goroutine should have ID 0")
    
    fr fr Test main goroutine detection
    testz.assert_true(real_goroutine_tracking.is_main_goroutine(), "Should be in main goroutine")
    
    fr fr Test active goroutine count (should be at least 1 for main)
    sus active_count normie = real_goroutine_tracking.get_active_goroutine_count()
    testz.assert_greater_than_int(active_count, 0, "Should have at least 1 active goroutine")
    
    fr fr Test goroutine metadata access
    sus metadata *real_goroutine_tracking.GoroutineMetadata = real_goroutine_tracking.get_current_goroutine_metadata()
    testz.assert_not_null(metadata, "Should be able to get current goroutine metadata")
    testz.assert_equal_i64(metadata.id, 0, "Metadata ID should match current goroutine")
    testz.assert_equal_string(metadata.function_name, "main", "Main function should be named 'main'")
    
    fr fr Test goroutine context access
    sus context *real_goroutine_tracking.GoroutineExecutionContext = real_goroutine_tracking.get_current_goroutine_context()
    testz.assert_not_null(context, "Should be able to get current goroutine context")
    
    vibez.spill("✅ Real goroutine tracking test passed")
    damn based
}

fr fr =============================================================================
fr fr OS PRIMITIVE INTEGRATION TESTS
fr fr =============================================================================

fr fr Test OS mutex functionality
slay test_os_mutex_primitives() lit {
    vibez.spill("🔒 Testing OS Mutex Primitives...")
    
    fr fr Create OS mutex
    sus mutex *os_primitives.OSMutex = os_primitives.create_os_mutex(os_primitives.MUTEX_NORMAL)
    testz.assert_not_null(mutex, "Should be able to create OS mutex")
    
    fr fr Test mutex locking
    testz.assert_equal_int(os_primitives.lock_os_mutex(mutex), 0, "Should be able to lock mutex")
    
    fr fr Test try-lock on locked mutex (should fail)
    testz.assert_equal_int(os_primitives.trylock_os_mutex(mutex), -1, "Try-lock on locked mutex should fail")
    
    fr fr Test unlock
    testz.assert_equal_int(os_primitives.unlock_os_mutex(mutex), 0, "Should be able to unlock mutex")
    
    fr fr Test try-lock on unlocked mutex (should succeed)
    testz.assert_equal_int(os_primitives.trylock_os_mutex(mutex), 0, "Try-lock on unlocked mutex should succeed")
    
    fr fr Unlock again
    testz.assert_equal_int(os_primitives.unlock_os_mutex(mutex), 0, "Should be able to unlock after try-lock")
    
    vibez.spill("✅ OS mutex primitives test passed")
    damn based
}

fr fr Test OS condition variable functionality
slay test_os_condition_primitives() lit {
    vibez.spill("📢 Testing OS Condition Variable Primitives...")
    
    fr fr Create mutex and condition variable
    sus mutex *os_primitives.OSMutex = os_primitives.create_os_mutex(os_primitives.MUTEX_NORMAL)
    sus cond *os_primitives.OSCondVar = os_primitives.create_os_condition()
    
    testz.assert_not_null(mutex, "Should be able to create mutex for condition test")
    testz.assert_not_null(cond, "Should be able to create condition variable")
    
    fr fr Test signal with no waiters (should not block)
    testz.assert_equal_int(os_primitives.signal_os_condition(cond), -1, "Signal with no waiters should return -1")
    
    fr fr Test broadcast with no waiters (should not block)
    testz.assert_equal_int(os_primitives.broadcast_os_condition(cond), 0, "Broadcast with no waiters should succeed")
    
    fr fr Test condition variable timeout (should timeout immediately)
    os_primitives.lock_os_mutex(mutex)
    sus timeout_result normie = os_primitives.wait_os_condition_timeout(cond, mutex, 1)  fr fr 1ms timeout
    os_primitives.unlock_os_mutex(mutex)
    testz.assert_equal_int(timeout_result, -1, "Wait with timeout should timeout")
    
    vibez.spill("✅ OS condition variable primitives test passed")
    damn based
}

fr fr =============================================================================
fr fr ENHANCED CHANNEL OPERATIONS TESTS
fr fr =============================================================================

fr fr Test enhanced channel operations with real tracking
slay test_enhanced_channel_operations() lit {
    vibez.spill("📡 Testing Enhanced Channel Operations...")
    
    fr fr Test channel creation with real registry
    sus channel_id thicc = concurrenz.make_channel()
    testz.assert_greater_than_i64(channel_id, 0, "Should get valid channel ID")
    
    fr fr Test buffered channel creation
    sus buffered_channel_id thicc = concurrenz.make_buffered_channel(5)
    testz.assert_greater_than_i64(buffered_channel_id, 0, "Should get valid buffered channel ID")
    testz.assert_not_equal_i64(channel_id, buffered_channel_id, "Channel IDs should be unique")
    
    fr fr Test channel closed status
    testz.assert_false(concurrenz.is_channel_closed(channel_id), "New channel should not be closed")
    
    fr fr Close channel and test status
    concurrenz.close_channel(channel_id)
    testz.assert_true(concurrenz.is_channel_closed(channel_id), "Closed channel should report closed")
    
    fr fr Test invalid channel operations
    testz.assert_false(concurrenz.send_channel(999999, 42), "Send to invalid channel should fail")
    testz.assert_equal_int(concurrenz.recv_channel(999999), 0, "Receive from invalid channel should return 0")
    
    vibez.spill("✅ Enhanced channel operations test passed")
    damn based
}

fr fr =============================================================================
fr fr PERFORMANCE MEASUREMENT TESTS
fr fr =============================================================================

fr fr Test performance counter functionality
slay test_performance_measurement() lit {
    vibez.spill("⚡ Testing Performance Measurement...")
    
    fr fr Test performance counter
    sus counter_start thicc = os_primitives.get_performance_counter()
    testz.assert_greater_than_i64(counter_start, 0, "Performance counter should be positive")
    
    fr fr Perform some work and measure
    sus work_iterations normie = 1000
    sus i normie = 0
    bestie i < work_iterations {
        concurrenz.cpu_pause()  fr fr Lightweight work
        i = i + 1
    }
    
    sus counter_end thicc = os_primitives.get_performance_counter()
    testz.assert_greater_than_i64(counter_end, counter_start, "Performance counter should advance")
    
    fr fr Test CPU information retrieval
    sus cpu_info *os_primitives.CPUInfo = os_primitives.get_cpu_info()
    testz.assert_not_null(cpu_info, "Should be able to get CPU information")
    testz.assert_greater_than_int(cpu_info.logical_cores, 0, "Should detect at least 1 logical CPU")
    testz.assert_greater_than_int(cpu_info.cache_line_size, 0, "Should have positive cache line size")
    testz.assert_greater_than_int(cpu_info.page_size, 0, "Should have positive page size")
    
    vibez.spill("✅ Performance measurement test passed")
    damn based
}

fr fr =============================================================================
fr fr THREADING INTEGRATION TESTS  
fr fr =============================================================================

fr fr Test OS thread integration
slay test_os_thread_integration() lit {
    vibez.spill("🧵 Testing OS Thread Integration...")
    
    fr fr Test thread ID retrieval
    sus thread_id thicc = os_primitives.get_current_thread_id()
    testz.assert_greater_than_i64(thread_id, 0, "Should get valid thread ID")
    
    fr fr Test thread yield (should not crash)
    os_primitives.os_thread_yield()
    
    fr fr Test multiple yields in sequence
    sus yield_count normie = 5
    sus j normie = 0
    bestie j < yield_count {
        os_primitives.os_thread_yield()
        j = j + 1
    }
    
    fr fr Test microsleep precision
    sus precision_test_start thicc = os_primitives.get_real_time_ns()
    os_primitives.microsleep_precise(500)  fr fr 500 microseconds
    sus precision_test_end thicc = os_primitives.get_real_time_ns()
    
    sus precision_duration thicc = precision_test_end - precision_test_start
    testz.assert_greater_than_i64(precision_duration, 400000, "Microsleep should be at least 400µs")
    testz.assert_less_than_i64(precision_duration, 1000000, "Microsleep should be under 1ms")
    
    vibez.spill("✅ OS thread integration test passed")
    damn based
}

fr fr =============================================================================
fr fr GOROUTINE STATE MANAGEMENT TESTS
fr fr =============================================================================

fr fr Test goroutine state tracking and transitions
slay test_goroutine_state_management() lit {
    vibez.spill("🎭 Testing Goroutine State Management...")
    
    sus main_id thicc = real_goroutine_tracking.get_current_goroutine_id()
    
    fr fr Test state update
    testz.assert_true(real_goroutine_tracking.update_goroutine_state(main_id, real_goroutine_tracking.GOROUTINE_RUNNING), 
                      "Should be able to update goroutine state")
    
    fr fr Test yield recording
    testz.assert_true(real_goroutine_tracking.record_goroutine_yield(main_id), 
                      "Should be able to record goroutine yield")
    
    fr fr Test blocking state recording
    testz.assert_true(real_goroutine_tracking.record_goroutine_blocked_on_channel(main_id, 123), 
                      "Should be able to record channel blocking")
    
    testz.assert_true(real_goroutine_tracking.record_goroutine_blocked_on_mutex(main_id, 456), 
                      "Should be able to record mutex blocking")
    
    fr fr Test memory allocation recording
    real_goroutine_tracking.record_memory_allocation(main_id, 1024)
    
    fr fr Test goroutine debug information
    sus debug_info *real_goroutine_tracking.GoroutineDebugInfo = real_goroutine_tracking.get_goroutine_debug_info(main_id)
    testz.assert_not_null(debug_info, "Should be able to get debug info")
    testz.assert_equal_i64(debug_info.metadata.id, main_id, "Debug info should match goroutine ID")
    
    vibez.spill("✅ Goroutine state management test passed")
    damn based
}

fr fr =============================================================================
fr fr DEBUGGING AND INTROSPECTION TESTS
fr fr =============================================================================

fr fr Test debugging and introspection capabilities
slay test_debugging_introspection() lit {
    vibez.spill("🔍 Testing Debugging and Introspection...")
    
    fr fr Enable debugging mode
    real_goroutine_tracking.enable_goroutine_debugging()
    
    sus main_id thicc = real_goroutine_tracking.get_current_goroutine_id()
    
    fr fr Test stack trace printing (should not crash)
    real_goroutine_tracking.print_goroutine_stack_trace(main_id)
    
    fr fr Test active goroutines retrieval
    sus active_goroutines thicc[value] = real_goroutine_tracking.get_all_active_goroutines()
    testz.assert_not_null(active_goroutines, "Should be able to get active goroutines")
    
    fr fr Test state string conversion
    sus state_str tea = real_goroutine_tracking.goroutine_state_string(real_goroutine_tracking.GOROUTINE_RUNNING)
    testz.assert_equal_string(state_str, "RUNNING", "State string should match state")
    
    sus unknown_state_str tea = real_goroutine_tracking.goroutine_state_string(999)
    testz.assert_equal_string(unknown_state_str, "UNKNOWN", "Unknown state should return UNKNOWN")
    
    vibez.spill("✅ Debugging and introspection test passed")
    damn based
}

fr fr =============================================================================
fr fr COMPREHENSIVE INTEGRATION TEST
fr fr =============================================================================

fr fr Test comprehensive integration of all enhanced functionality
slay test_comprehensive_integration() lit {
    vibez.spill("🎪 Testing Comprehensive Integration...")
    
    fr fr Initialize timing test
    sus integration_start thicc = concurrenz.get_time_ns()
    
    fr fr Test goroutine count tracking
    sus initial_count normie = concurrenz.num_goroutines()
    testz.assert_greater_than_int(initial_count, 0, "Should have active goroutines")
    
    fr fr Test channel operations with timing
    sus test_channel_id thicc = concurrenz.make_buffered_channel(3)
    
    fr fr Test yield with timing
    sus pre_yield_time thicc = concurrenz.get_time_ns()
    concurrenz.runtime_yield()
    sus post_yield_time thicc = concurrenz.get_time_ns()
    testz.assert_greater_than_i64(post_yield_time, pre_yield_time, "Time should advance after yield")
    
    fr fr Test memory fence operation
    concurrenz.memory_fence()
    
    fr fr Test sleep with state tracking
    sus pre_sleep_goroutines normie = concurrenz.num_goroutines()
    concurrenz.sleep_ms(5)  fr fr 5ms sleep
    sus post_sleep_goroutines normie = concurrenz.num_goroutines()
    testz.assert_equal_int(pre_sleep_goroutines, post_sleep_goroutines, "Goroutine count should remain stable")
    
    fr fr Clean up channel
    concurrenz.close_channel(test_channel_id)
    testz.assert_true(concurrenz.is_channel_closed(test_channel_id), "Test channel should be closed")
    
    sus integration_end thicc = concurrenz.get_time_ns()
    sus total_time thicc = integration_end - integration_start
    testz.assert_greater_than_i64(total_time, 5000000, "Integration test should take at least 5ms")
    
    vibez.spill("✅ Comprehensive integration test passed")
    damn based
}

fr fr =============================================================================
fr fr MAIN TEST EXECUTION
fr fr =============================================================================

fr fr Execute all enhanced concurrency tests
slay run_all_enhanced_concurrency_tests() lit {
    vibez.spill("🚀 Running Enhanced Concurrency Test Suite")
    vibez.spill("==============================================")
    
    testz.test_start("Enhanced Concurrency Tests")
    
    fr fr Execute all test suites
    testz.assert_true(test_real_timing_mechanisms(), "Real timing mechanisms should work")
    testz.assert_true(test_real_goroutine_tracking(), "Real goroutine tracking should work")
    testz.assert_true(test_os_mutex_primitives(), "OS mutex primitives should work")
    testz.assert_true(test_os_condition_primitives(), "OS condition primitives should work")
    testz.assert_true(test_enhanced_channel_operations(), "Enhanced channel operations should work")
    testz.assert_true(test_performance_measurement(), "Performance measurement should work")
    testz.assert_true(test_os_thread_integration(), "OS thread integration should work")
    testz.assert_true(test_goroutine_state_management(), "Goroutine state management should work")
    testz.assert_true(test_debugging_introspection(), "Debugging and introspection should work")
    testz.assert_true(test_comprehensive_integration(), "Comprehensive integration should work")
    
    fr fr Print final results
    testz.print_test_summary()
    
    vibez.spill("==============================================")
    vibez.spill("✅ All Enhanced Concurrency Tests Completed")
    
    damn based
}

fr fr Main test execution
run_all_enhanced_concurrency_tests()
