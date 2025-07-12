yeet "testz_simple"

slay test_assertions() {
    test_start("Basic Assertions")
    
    // Test integer assertions
    assert_eq_int(5, 5)
    assert_eq_int(2 + 3, 5)
    
    // Test string assertions
    assert_eq_string("hello", "hello")
    assert_eq_string("test" + "ing", "testing")
    
    // Test boolean assertions
    assert_true(based)
    assert_false(cap)
    assert_true(5 > 3)
    assert_false(2 > 5)
    
    print_test_summary()
}

test_assertions()
