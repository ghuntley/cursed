# Test the fixed testz module
yeet "testz"

slay test_testz_functionality() lit {
    test_start("testz functionality test")
    
    # Test basic assertions
    assert_true(based)
    assert_false(cringe)
    assert_eq_int(42, 42)
    assert_eq_string("hello", "hello")
    
    # Test comparison assertions  
    assert_greater_than(10, 5)
    assert_less_than(3, 7)
    assert_in_range(5, 1, 10)
    
    # Test string operations
    sus result tea = create_temp_data("test")
    assert_eq_string(result, "test_temp")
    
    print_test_summary()
    damn based
}

test_testz_functionality()
