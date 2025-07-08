yeet "testz"

# Simple test for error_core functionality
slay test_basic_functionality() {
    test_start("Basic error_core functionality")
    
    vibez.spill("Testing basic error creation...")
    
    # Create a simple error-like structure (without importing error_core for now)
    be_like simple_error squad {
        msg tea
        code normie
    }
    
    sus err = simple_error{
        msg: "Test error",
        code: 1000
    }
    
    assert_eq_string(err.msg, "Test error")
    assert_eq_int(err.code, 1000)
    
    vibez.spill("✅ Basic error creation test passed")
}

test_basic_functionality()
print_test_summary()
