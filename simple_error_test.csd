yeet "testz"

slay test_basic_error_handling() {
    test_start("Basic error handling test")
    
    // Test simple yikes error
    fam {
        yikes "Test error", 100
    } catch(err) {
        vibez.spill("Caught error: Test error")
        assert_true(based)  // We should reach here
    }
    
    vibez.spill("✅ Basic error handling works")
}

slay test_defer_cleanup() {
    test_start("Defer cleanup test")
    
    sus cleanup_called lit = cringe
    
    {
        later {
            cleanup_called = based
        }
        
        vibez.spill("About to exit scope")
    }
    
    assert_true(cleanup_called)
    vibez.spill("✅ Defer cleanup works")
}

slay test_simple_propagation() {
    test_start("Simple error propagation test")
    
    slay might_fail() shook {
        yikes "Function failed", 200
    }
    
    fam {
        sus result = shook might_fail()
        vibez.spill("This shouldn't execute")
    } catch(err) {
        vibez.spill("Caught propagated error")
        assert_true(based)
    }
    
    vibez.spill("✅ Error propagation works")
}

slay main() {
    vibez.spill("🚀 Starting simple error handling tests...")
    
    test_basic_error_handling()
    test_defer_cleanup()
    test_simple_propagation()
    
    print_test_summary()
    vibez.spill("🎉 Simple error tests completed!")
}
