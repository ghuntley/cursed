// Simple CURSED Stringz Module Test
// Tests basic string operations

yeet "testz"
yeet "stringz"

slay test_basic_operations() {
    test_start("Basic string operations")
    
    // Test Contains
    assert_true(stringz.Contains("hello world", "world"))
    assert_false(stringz.Contains("hello world", "xyz"))
    
    // Test StartsWith/EndsWith
    assert_true(stringz.StartsWith("hello world", "hello"))
    assert_true(stringz.EndsWith("hello world", "world"))
    assert_false(stringz.StartsWith("hello world", "world"))
    
    // Test case conversion
    assert_eq_string(stringz.ToLower("HELLO"), "hello")
    assert_eq_string(stringz.ToUpper("hello"), "HELLO")
    
    // Test utilities
    assert_eq_int(stringz.Len("hello"), 5)
    assert_true(stringz.IsEmpty(""))
    assert_false(stringz.IsEmpty("hello"))
    
    // Test comparison
    assert_true(stringz.Equals("hello", "hello"))
    assert_false(stringz.Equals("hello", "world"))
}

slay test_conversion() {
    test_start("String conversion")
    
    // Test string to int
    assert_eq_int(stringz.ToInt("123"), 123)
    assert_eq_int(stringz.ToInt("0"), 0)
    
    // Test int to string
    assert_eq_string(stringz.FromInt(123), "123")
    assert_eq_string(stringz.FromInt(0), "0")
    
    // Test bool conversion
    assert_eq_string(stringz.FromBool(based), "based")
    assert_eq_string(stringz.FromBool(cap), "cap")
}

slay test_utilities() {
    test_start("String utilities")
    
    // Test repeat
    assert_eq_string(stringz.Repeat("ho", 3), "hohoho")
    assert_eq_string(stringz.Repeat("a", 1), "a")
    
    // Test reverse
    assert_eq_string(stringz.Reverse("hello"), "olleh")
    assert_eq_string(stringz.Reverse("a"), "a")
    
    // Test substring
    assert_eq_string(stringz.Substring("hello world", 0, 5), "hello")
    assert_eq_string(stringz.Substring("hello world", 6, 5), "world")
    
    // Test character access
    assert_eq_string(stringz.CharAt("hello", 0), "h")
    assert_eq_string(stringz.CharAt("hello", 4), "o")
}

slay main_character() {
    vibez.spill("Running Simple CURSED Stringz Tests...")
    vibez.spill("=====================================")
    
    test_basic_operations()
    test_conversion()
    test_utilities()
    
    print_test_summary()
}

main()
