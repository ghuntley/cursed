yeet "testz"

// Test Suite for CURSED Parser Implementation
// Testing basic CURSED syntax elements and AST generation

test_start("Parser Test Suite")

// Test 1: Simple variable declarations
slay test_variable_declarations() {
    // Test mutable variable
    sus x drip = 42
    assert_eq_int(x, 42)
    
    // Test immutable variable  
    facts y drip = 24
    assert_eq_int(y, 24)
    
    // Test with type annotation
    sus name tea = "hello"
    assert_eq_string(name, "hello")
    
    // Test boolean variables
    sus flag lit = based
    assert_true(flag)
    
    facts readonly lit = cringe
    assert_false(readonly)
    
    vibez.spill("✓ Variable declaration tests passed")
}

// Test 2: Function definitions
slay test_function_definitions() {
    // Test simple function with no parameters
    slay simple_func() {
        vibez.spill("simple function")
    }
    
    // Test function with parameters and return type
    slay add_numbers(a drip, b drip) drip {
        damn a + b
    }
    
    // Test function call
    sus result drip = add_numbers(10, 20)
    assert_eq_int(result, 30)
    
    // Test function with multiple parameter types
    slay greet_user(name tea, age drip) tea {
        damn "Hello " + name + ", age " + age
    }
    
    vibez.spill("✓ Function definition tests passed")
}

// Test 3: Function calls
slay test_function_calls() {
    // Test basic function call
    vibez.spill("hello world")
    
    // Test function call with parameters
    sus sum drip = add_numbers(5, 15)
    assert_eq_int(sum, 20)
    
    // Test chained function calls
    vibez.spill(greet_user("Alice", 25))
    
    // Test method-style calls
    sus text tea = "test string"
    sus length drip = text.length()
    
    vibez.spill("✓ Function call tests passed")
}

// Test 4: Basic control flow (lowkey/highkey)
slay test_control_flow() {
    sus x drip = 10
    
    // Test if statement (lowkey)
    lowkey x > 5 {
        vibez.spill("x is greater than 5")
        assert_true(based)
    }
    
    // Test if-else statement (lowkey/highkey)
    lowkey x < 5 {
        assert_false(based)
    } highkey {
        assert_true(based)
        vibez.spill("x is not less than 5")
    }
    
    // Test complex condition
    lowkey x > 0 && x < 20 {
        vibez.spill("x is in valid range")
        assert_true(based)
    }
    
    vibez.spill("✓ Control flow tests passed")
}

// Test 5: Advanced syntax elements
slay test_advanced_syntax() {
    // Test arrays
    sus numbers drip[] = [1, 2, 3, 4, 5]
    assert_eq_int(numbers[0], 1)
    
    // Test struct definition and usage
    squad Point {
        spill x drip
        spill y drip
    }
    
    sus p Point = Point{x: 10, y: 20}
    assert_eq_int(p.x, 10)
    assert_eq_int(p.y, 20)
    
    // Test interface
    collab Drawable {
        slay draw()
    }
    
    vibez.spill("✓ Advanced syntax tests passed")
}

// Test 6: Error conditions and edge cases
slay test_error_handling() {
    // Test error handling with yikes/fam
    yikes {
        sus risky_value drip = might_fail()
        vibez.spill("Operation succeeded")
    } fam error {
        vibez.spill("Caught error: " + error.message)
    }
    
    vibez.spill("✓ Error handling tests passed")
}

// Test 7: Comments and whitespace handling
slay test_comments_whitespace() {
    // Single line comment
    sus x drip = 42 // End of line comment
    
    /*
     * Multi-line comment
     * Testing parser handling
     */
    sus y drip = 24
    
    // Test whitespace sensitivity
    sus    spaced_var    drip    =    100
    assert_eq_int(spaced_var, 100)
    
    vibez.spill("✓ Comments and whitespace tests passed")
}

// Helper function for error testing
slay might_fail() drip {
    damn 42
}

// Main test runner
slay main_character() {
    test_variable_declarations()
    test_function_definitions()  
    test_function_calls()
    test_control_flow()
    test_advanced_syntax()
    test_error_handling()
    test_comments_whitespace()
    
    print_test_summary()
    vibez.spill("🎉 All parser tests completed!")
}
