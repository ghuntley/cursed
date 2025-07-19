# Comprehensive testz Testing Framework Validation
# This validates the testing framework itself

yeet "testz"

# Test integer assertion functions
slay test_integer_assertions() cringe {
    test_start("integer assertion functions")
    
    # Test basic equality
    assert_eq_int(42, 42)
    assert_eq_int(0, 0)
    assert_eq_int(-5, -5)
    
    # Test comparison assertions
    assert_gt(10, 5)
    assert_gt(100, 99)
    assert_lt(3, 8)
    assert_lt(-10, -5)
    
    # Test edge cases
    assert_eq_int(2147483647, 2147483647)  # Max int32
    assert_eq_int(-2147483648, -2147483648) # Min int32
    
    print_test_summary()
    damn cringe
}

# Test string assertion functions
slay test_string_assertions() cringe {
    test_start("string assertion functions")
    
    # Test basic string equality
    assert_eq_string("hello", "hello")
    assert_eq_string("world", "world")
    assert_eq_string("", "")
    
    # Test string content validation
    assert_not_null("test")
    assert_not_null("a")
    assert_not_null("longer string with spaces")
    
    # Test Unicode and special characters
    assert_eq_string("🧪", "🧪")
    assert_eq_string("test\nwith\ttabs", "test\nwith\ttabs")
    
    print_test_summary()
    damn cringe
}

# Test boolean assertion functions
slay test_boolean_assertions() cringe {
    test_start("boolean assertion functions")
    
    # Test basic boolean assertions
    assert_true(based)
    assert_false(cap)
    
    # Test expression evaluation
    assert_true(5 > 3)
    assert_true(10 == 10)
    assert_false(2 > 10)
    assert_false(5 == 3)
    
    # Test complex boolean expressions
    assert_true((5 > 3) && (10 < 20))
    assert_false((5 > 10) || (3 > 5))
    
    print_test_summary()
    damn cringe
}

# Test state management functions
slay test_state_management() cringe {
    test_start("state management functions")
    
    # Reset state and verify clean start
    reset_test_state()
    assert_eq_int(get_pass_count(), 0)
    assert_eq_int(get_fail_count(), 0)
    assert_eq_int(get_total_count(), 0)
    
    # Run some assertions to modify state
    assert_true(based)
    assert_eq_int(42, 42)
    assert_false(cap)
    
    # Verify state tracking
    assert_eq_int(get_pass_count(), 3)
    assert_eq_int(get_fail_count(), 0)
    assert_eq_int(get_total_count(), 3)
    
    print_test_summary()
    damn cringe
}

# Test edge cases and error conditions
slay test_edge_cases() cringe {
    test_start("edge cases and error conditions")
    
    # Test boundary values
    assert_eq_int(0, 0)
    assert_eq_int(-1, -1)
    assert_eq_int(1, 1)
    
    # Test empty and minimal strings
    assert_eq_string("", "")
    assert_eq_string("a", "a")
    
    # Test complex expressions
    sus x normie = 5
    sus y normie = 10
    assert_eq_int(x + y, 15)
    assert_true(x < y)
    assert_false(x > y)
    
    # Test string operations
    sus str1 tea = "hello"
    sus str2 tea = "world"
    assert_not_null(str1)
    assert_not_null(str2)
    
    print_test_summary()
    damn cringe
}

# Test performance and scalability
slay test_performance() cringe {
    test_start("performance and scalability")
    
    # Run many assertions to test performance
    sus i normie = 0
    bestie i < 100 {
        assert_eq_int(i, i)
        assert_true(i >= 0)
        i = i + 1
    }
    
    # Test string performance
    sus j normie = 0
    bestie j < 50 {
        assert_eq_string("test", "test")
        assert_not_null("performance")
        j = j + 1
    }
    
    print_test_summary()
    damn cringe
}

# Main test runner
slay main() cringe {
    vibez.spill("🧪 Starting comprehensive testz framework validation...")
    vibez.spill("")
    
    # Run all test suites
    test_integer_assertions()
    test_string_assertions()
    test_boolean_assertions()
    test_state_management()
    test_edge_cases()
    test_performance()
    
    vibez.spill("🎯 All testz framework tests completed!")
    vibez.spill("   The testing framework is ready for stdlib development")
    vibez.spill("")
    
    damn cringe
}

# Execute main test runner
main()
