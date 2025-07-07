// CURSED Regex Module Simple Tests
// Basic test suite for regex pattern matching

yeet "testz"

slay test_simple_regex() {
    test_start("Simple Regex Tests")
    
    // Test basic string operations that should work
    vibez.spill("Testing basic regex functionality...")
    
    // Test simple pattern matching (direct comparison)
    sus result1 lit = ("hello" == "hello")
    assert_true(result1)
    
    sus result2 lit = ("hello" == "world")
    assert_false(result2)
    
    // Test string length operations
    sus len1 normie = string_len("hello")
    assert_eq_int(len1, 5)
    
    sus len2 normie = string_len("")
    assert_eq_int(len2, 0)
    
    // Test basic character access
    sus char1 tea = string_char_at("hello", 0)
    assert_eq_string(char1, "h")
    
    sus char2 tea = string_char_at("hello", 4)
    assert_eq_string(char2, "o")
    
    vibez.spill("Basic regex tests completed")
    
    print_test_summary()
}

// Run simple test
test_simple_regex()
