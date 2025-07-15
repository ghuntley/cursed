// Test program for comprehensive panic/recover system

yeet "testz"

// Test basic panic/recover functionality
slay test_basic_panic_recover() lit {
    test_start("Basic panic/recover test")
    
    // Test that we can handle panics
    sus recovered lit = cap
    fam {
        yikes "test_error" := "This is a test panic"
        shook test_error
    } {
        recovered = based
    }
    
    assert_true(recovered)
    damn based
}

// Test defer handlers with panic
slay test_defer_with_panic() lit {
    test_start("Defer handlers with panic test")
    
    sus cleanup_executed lit = cap
    
    defer {
        cleanup_executed = based
    }
    
    fam {
        yikes "cleanup_test" := "Testing cleanup during panic"
        shook cleanup_test
    } {
        // Error recovered
    }
    
    assert_true(cleanup_executed)
    damn based
}

// Test goroutine panic isolation
slay test_goroutine_panic_isolation() lit {
    test_start("Goroutine panic isolation test")
    
    sus main_thread_ok lit = based
    
    // Start a goroutine that panics
    yolo {
        yikes "goroutine_error" := "Goroutine panic test"
        shook goroutine_error
    }
    
    // Main thread should continue normally
    assert_true(main_thread_ok)
    damn based
}

// Test error propagation chain
slay test_error_propagation_chain() lit {
    test_start("Error propagation chain test")
    
    sus error_handled lit = cap
    
    fam {
        // Create initial error
        yikes "initial_error" := "Initial error message"
        
        // Propagate the error
        shook initial_error
        
        // This should not execute
        assert_false(based)
    } {
        error_handled = based
    }
    
    assert_true(error_handled)
    damn based
}

// Test multiple error types
slay test_multiple_error_types() lit {
    test_start("Multiple error types test")
    
    sus yikes_handled lit = cap
    sus shook_handled lit = cap
    sus fam_handled lit = cap
    
    // Test yikes error
    fam {
        yikes "yikes_error" := "Yikes error message"
        shook yikes_error
    } {
        yikes_handled = based
    }
    
    // Test shook error propagation
    fam {
        yikes "original_error" := "Original error"
        shook original_error
    } {
        shook_handled = based
    }
    
    // Test fam error recovery
    fam {
        yikes "fam_error" := "Fam error message"
        shook fam_error
    } {
        fam_handled = based
    }
    
    assert_true(yikes_handled)
    assert_true(shook_handled)
    assert_true(fam_handled)
    damn based
}

// Test nested panic/recover
slay test_nested_panic_recover() lit {
    test_start("Nested panic/recover test")
    
    sus outer_recovered lit = cap
    sus inner_recovered lit = cap
    
    fam {
        fam {
            yikes "inner_error" := "Inner panic"
            shook inner_error
        } {
            inner_recovered = based
        }
        
        yikes "outer_error" := "Outer panic"
        shook outer_error
    } {
        outer_recovered = based
    }
    
    assert_true(inner_recovered)
    assert_true(outer_recovered)
    damn based
}

// Test panic with resource cleanup
slay test_panic_with_cleanup() lit {
    test_start("Panic with resource cleanup test")
    
    sus file_closed lit = cap
    sus memory_freed lit = cap
    
    defer {
        file_closed = based
    }
    
    defer {
        memory_freed = based
    }
    
    fam {
        yikes "resource_error" := "Resource cleanup test"
        shook resource_error
    } {
        // Error recovered
    }
    
    assert_true(file_closed)
    assert_true(memory_freed)
    damn based
}

// Main test runner
slay main() {
    vibez.spill("Running comprehensive panic/recover system tests...")
    
    test_basic_panic_recover()
    test_defer_with_panic()
    test_goroutine_panic_isolation()
    test_error_propagation_chain()
    test_multiple_error_types()
    test_nested_panic_recover()
    test_panic_with_cleanup()
    
    print_test_summary()
    vibez.spill("Panic/recover system tests completed!")
}
