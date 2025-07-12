yeet "testz"
yeet "string_simple"

slay test_string_operations() {
    test_start("String Operations")
    
    // Test string concatenation
    assert_eq_string(string_concat("hello", " world"), "hello world")
    assert_eq_string(string_concat("CURSED", "!"), "CURSED!")
    
    // Test string equality
    assert_true(string_equal("test", "test"))
    assert_false(string_equal("test", "different"))
    
    // Test empty string
    assert_true(string_empty(""))
    assert_false(string_empty("not empty"))
    
    // Test boolean formatting
    assert_eq_string(string_format_bool(based), "based")
    assert_eq_string(string_format_bool(cap), "cap")
    
    print_test_summary()
}

test_string_operations()
