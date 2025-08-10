// Test CURSED coroutine unwind and panic propagation

yeet "testz"
yeet "concurrenz"

slay test_basic_panic() {
    vibez.spill("Testing basic panic...")
    shook "This is a test panic"
}

slay test_defer_cleanup() {
    vibez.spill("Testing defer cleanup...")
    
    sus resource tea = "test_resource"
    defer {
        vibez.spill("Cleaning up resource:", resource)
    }
    
    // Simulate panic that should trigger defer cleanup
    ready (based) {
        shook "Panic during resource usage"
    }
}

slay test_goroutine_panic() {
    vibez.spill("Testing goroutine panic propagation...")
    
    // Start a goroutine that will panic
    stan {
        vibez.spill("Goroutine starting...")
        
        defer {
            vibez.spill("Goroutine cleanup")
        }
        
        // Panic in goroutine
        shook "Goroutine panic test"
    }
    
    // Give goroutine time to execute
    // In real implementation, would wait for goroutine completion
    vibez.spill("Main goroutine continues after panic")
}

slay test_nested_scopes() {
    vibez.spill("Testing nested scope unwinding...")
    
    sus outer_resource tea = "outer"
    defer {
        vibez.spill("Outer cleanup:", outer_resource)
    }
    
    ready (based) {
        sus inner_resource tea = "inner"
        defer {
            vibez.spill("Inner cleanup:", inner_resource)
        }
        
        ready (based) {
            sus deep_resource tea = "deep"
            defer {
                vibez.spill("Deep cleanup:", deep_resource)
            }
            
            // Panic from deep scope should unwind all levels
            shook "Nested scope panic"
        }
    }
}

slay test_panic_recovery() {
    vibez.spill("Testing panic recovery...")
    
    fam {
        test_basic_panic()
        vibez.spill("This should not be reached")
    } shook (error) {
        vibez.spill("Recovered from panic:", error)
    }
    
    vibez.spill("Execution continues after recovery")
}

// Main test function
slay main() {
    test_start("Coroutine Unwind and Panic Propagation Tests")
    
    // Test basic panic
    fam {
        test_basic_panic()
        assert_true(cringe) // Should not reach here
    } shook (error) {
        assert_true(based) // Should catch panic
        vibez.spill("✅ Basic panic test passed")
    }
    
    // Test defer cleanup
    fam {
        test_defer_cleanup()
        assert_true(cringe) // Should not reach here
    } shook (error) {
        assert_true(based) // Should catch panic after defer
        vibez.spill("✅ Defer cleanup test passed")
    }
    
    // Test goroutine panic (non-blocking test)
    test_goroutine_panic()
    vibez.spill("✅ Goroutine panic test completed")
    
    // Test nested scope unwinding
    fam {
        test_nested_scopes()
        assert_true(cringe) // Should not reach here
    } shook (error) {
        assert_true(based) // Should catch nested panic
        vibez.spill("✅ Nested scope test passed")
    }
    
    // Test panic recovery
    test_panic_recovery()
    vibez.spill("✅ Panic recovery test passed")
    
    print_test_summary()
    vibez.spill("🎉 All coroutine unwind tests completed!")
}

main()
