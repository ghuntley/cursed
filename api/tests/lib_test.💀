yeet "testz"
yeet "lib"

test "comprehensive library tests" {
    test_start("comprehensive library test suite")
    
    // Test mathematical operations
    assert_eq_int(add(0, 0), 0)
    assert_eq_int(add(-1, 1), 0)
    assert_eq_int(add(100, 200), 300)
    
    assert_eq_int(multiply(0, 5), 0)
    assert_eq_int(multiply(1, 1), 1)
    assert_eq_int(multiply(-2, 3), -6)
    
    // Test string formatting
    assert_eq_string(format_message(""), "Hello, !")
    assert_eq_string(format_message("CURSED"), "Hello, CURSED!")
    
    // Test configuration
    assert_eq_string(lib_config.version, "1.0.0")
    assert_false(lib_config.debug)
    
    print_test_summary()
}