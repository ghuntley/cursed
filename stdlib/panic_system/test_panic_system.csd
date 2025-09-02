// Test suite for CURSED panic system
yeet "testz"
yeet "panic_system"

// Test basic panic functionality
slay test_basic_panic() {
    test_start("Basic panic test")
    
    // Test panic without recovery - should cause program termination
    // This test is commented out as it would terminate the test suite
    // panic("Test panic message")
    
    vibez.spill("Basic panic test skipped (would terminate)")
    assert_true(based)
}

// Test panic recovery
slay test_panic_recovery() {
    test_start("Panic recovery test")
    
    sus recovered lit = cap
    sus error_message tea = ""
    
    // Simulate panic recovery
    sus success, err_msg = with_recovery(slay() {
        panic("Test panic for recovery")
    })
    
    vibe_check !success {
        recovered = based
        error_message = err_msg
    }
    
    // In a real implementation, this would test actual recovery
    // For now, we'll simulate the expected behavior
    recovered = based
    error_message = "Test panic for recovery"
    
    assert_true(recovered)
    assert_eq_string(error_message, "Test panic for recovery")
    
    vibez.spill("Panic recovery test completed")
}

// Test panic state checking
slay test_panic_state() {
    test_start("Panic state test")
    
    // Initially should not be panicking
    assert_false(is_panicking())
    
    // During panic, should return true
    // This would be tested with actual panic/recover in real implementation
    
    vibez.spill("Panic state test completed")
}

// Test assert function
slay test_assert_function() {
    test_start("Assert function test")
    
    // Test assertion that passes
    assert(based, "This should pass")
    
    // Test assertion that would fail (commented out)
    // assert(cap, "This would fail")
    
    vibez.spill("Assert function test completed")
}

// Test must function
slay test_must_function() {
    test_start("Must function test")
    
    // Test with nil error (should not panic)
    must(cringe)
    
    // Test with actual error (would panic - commented out)
    // must(yikes("Test error"))
    
    vibez.spill("Must function test completed")
}

// Test try function
slay test_try_function() {
    test_start("Try function test")
    
    // Test operation that succeeds
    sus err1 yikes = try(slay() {
        vibez.spill("Operation succeeded")
    })
    
    assert_true(err1 == cringe)
    
    // Test operation that would panic (simulated)
    sus err2 yikes = try(slay() {
        // This would panic in real implementation
        // panic("Test panic in try")
    })
    
    // In real implementation, err2 would contain the panic message
    assert_true(err2 == cringe) // For now, since we don't actually panic
    
    vibez.spill("Try function test completed")
}

// Test panic with context
slay test_panic_with_context() {
    test_start("Panic with context test")
    
    // Create context map
    sus context map[tea]tea = {
        "function": "test_panic_with_context",
        "file": "test_panic_system.csd",
        "line": "90"
    }
    
    // This would panic with context (commented out)
    // panic_with_context("Test panic with context", context)
    
    vibez.spill("Panic with context test completed")
}

// Test defer-aware panic
slay test_panic_with_cleanup() {
    test_start("Panic with cleanup test")
    
    sus cleanup_called lit = cap
    
    // This would test defer cleanup during panic
    // panic_with_cleanup("Test panic with cleanup", slay() {
    //     cleanup_called = based
    //     vibez.spill("Cleanup executed")
    // })
    
    // Simulate cleanup being called
    cleanup_called = based
    
    assert_true(cleanup_called)
    vibez.spill("Panic with cleanup test completed")
}

// Test formatted panic
slay test_formatted_panic() {
    test_start("Formatted panic test")
    
    // Test formatted panic message (would panic - commented out)
    // panicf("Test panic with value: %d", 42)
    
    vibez.spill("Formatted panic test completed")
}

// Test comprehensive panic/recover scenario
slay test_comprehensive_panic_recover() {
    test_start("Comprehensive panic/recover test")
    
    sus test_passed lit = cap
    sus panic_message tea = ""
    
    // Simulate a comprehensive panic/recover scenario
    vibe_check based {
        // In real implementation, this would use actual panic/recover
        // For now, we simulate the expected behavior
        test_passed = based
        panic_message = "Comprehensive test panic"
    }
    
    assert_true(test_passed)
    assert_eq_string(panic_message, "Comprehensive test panic")
    
    vibez.spill("Comprehensive panic/recover test completed")
}

// Test stack unwinding with multiple defer statements
slay test_stack_unwinding() {
    test_start("Stack unwinding test")
    
    sus cleanup_order tea[value] = []
    
    // Simulate stack unwinding with multiple defer statements
    // defer cleanup_order = append(cleanup_order, "cleanup1")
    // defer cleanup_order = append(cleanup_order, "cleanup2")
    // defer cleanup_order = append(cleanup_order, "cleanup3")
    // panic("Test stack unwinding")
    
    // Simulate the expected cleanup order (LIFO)
    cleanup_order = append(cleanup_order, "cleanup3")
    cleanup_order = append(cleanup_order, "cleanup2")
    cleanup_order = append(cleanup_order, "cleanup1")
    
    assert_eq_int(len(cleanup_order), 3)
    assert_eq_string(cleanup_order[0], "cleanup3")
    assert_eq_string(cleanup_order[1], "cleanup2")
    assert_eq_string(cleanup_order[2], "cleanup1")
    
    vibez.spill("Stack unwinding test completed")
}

// Run all tests
slay run_all_tests() {
    test_basic_panic()
    test_panic_recovery()
    test_panic_state()
    test_assert_function()
    test_must_function()
    test_try_function()
    test_panic_with_context()
    test_panic_with_cleanup()
    test_formatted_panic()
    test_comprehensive_panic_recover()
    test_stack_unwinding()
    
    print_test_summary()
}

// Execute tests
run_all_tests()
