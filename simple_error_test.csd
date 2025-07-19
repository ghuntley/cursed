# Simple test to verify error handling syntax parsing
yeet "testz"

# Test basic error syntax
slay test_basic_error() {
    test_start("Basic Error Test")
    
    # Test yikes error creation  
    sus err = yikes("Test error message")
    assert_true(err != cringe)
    
    # Test simple function with error return
    sus result, error = simple_divide(10, 2)
    assert_eq_int(result, 5)
    assert_true(error == cringe)
    
    print_test_summary()
}

# Simple function that returns Result-like values
slay simple_divide(a normie, b normie) (normie, yikes) {
    vibe_check b {
        mood 0:
            damn 0, yikes("Division by zero")
        basic:
            damn a / b, cringe
    }
}

test_basic_error()
