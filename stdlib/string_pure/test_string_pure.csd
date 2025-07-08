yeet "testz"
yeet "string_pure"

// ================================
// Pure CURSED String Library Tests
// ================================

slay test_string_core_functions() {
    test_start("Core String Functions")
    
    // Test string length
    assert_eq_int(string_length("hello"), 5)
    assert_eq_int(string_length(""), 0)
    assert_eq_int(string_length("CURSED"), 6)
    
    // Test empty string check
    assert_true(string_is_empty(""))
    assert_false(string_is_empty("hello"))
    
    // Test character access
    assert_eq_string(tea(string_char_at("hello", 0)), "h")
    assert_eq_string(tea(string_char_at("hello", 4)), "o")
    assert_eq_string(tea(string_char_at("hello", 2)), "l")
}

slay test_string_concatenation() {
    test_start("String Concatenation")
    
    // Test basic concatenation
    assert_eq_string(string_concatenate("hello", " world"), "hello world")
    assert_eq_string(string_concatenate("", "test"), "test")
    assert_eq_string(string_concatenate("test", ""), "test")
    assert_eq_string(string_concatenate("", ""), "")
}

slay test_string_slicing() {
    test_start("String Slicing")
    
    // Test substring
    assert_eq_string(string_substring("hello world", 0, 5), "hello")
    assert_eq_string(string_substring("hello world", 6, 5), "world")
    assert_eq_string(string_substring("hello world", 2, 6), "llo wo")
    
    // Test slice
    assert_eq_string(string_slice("hello world", 0, 5), "hello")
    assert_eq_string(string_slice("hello world", 6, 11), "world")
    assert_eq_string(string_slice("hello world", 2, 8), "llo wo")
    
    // Test edge cases
    assert_eq_string(string_substring("hello", 0, 0), "")
    assert_eq_string(string_slice("hello", 5, 5), "")
    assert_eq_string(string_substring("hello", 10, 5), "")
}

slay test_string_comparison() {
    test_start("String Comparison")
    
    // Test equality
    assert_true(string_equals("hello", "hello"))
    assert_false(string_equals("hello", "world"))
    assert_true(string_equals("", ""))
    assert_false(string_equals("hello", ""))
    
    // Test comparison
    assert_eq_int(string_compare("hello", "hello"), 0)
    assert_eq_int(string_compare("abc", "def"), -1)
    assert_eq_int(string_compare("def", "abc"), 1)
    assert_eq_int(string_compare("hello", "hello world"), -1)
    assert_eq_int(string_compare("hello world", "hello"), 1)
}

slay test_string_search() {
    test_start("String Search")
    
    // Test contains
    assert_true(string_contains("hello world", "world"))
    assert_true(string_contains("hello world", "hello"))
    assert_false(string_contains("hello world", "xyz"))
    assert_true(string_contains("hello world", ""))
    
    // Test index of
    assert_eq_int(string_index_of("hello world", "world"), 6)
    assert_eq_int(string_index_of("hello world", "hello"), 0)
    assert_eq_int(string_index_of("hello world", "xyz"), -1)
    assert_eq_int(string_index_of("hello world", ""), 0)
    
    // Test starts with
    assert_true(string_starts_with("hello world", "hello"))
    assert_true(string_starts_with("hello world", ""))
    assert_false(string_starts_with("hello world", "world"))
    
    // Test ends with
    assert_true(string_ends_with("hello world", "world"))
    assert_true(string_ends_with("hello world", ""))
    assert_false(string_ends_with("hello world", "hello"))
    
    // Test count occurrences
    assert_eq_int(string_count_occurrences("hello hello hello", "hello"), 3)
    assert_eq_int(string_count_occurrences("hello world", "l"), 3)
    assert_eq_int(string_count_occurrences("hello world", "xyz"), 0)
}

slay test_string_transformation() {
    test_start("String Transformation")
    
    // Test case conversion
    assert_eq_string(string_to_upper("hello"), "HELLO")
    assert_eq_string(string_to_upper("CURSED"), "CURSED")
    assert_eq_string(string_to_upper("MiXeD"), "MIXED")
    
    assert_eq_string(string_to_lower("HELLO"), "hello")
    assert_eq_string(string_to_lower("cursed"), "cursed")
    assert_eq_string(string_to_lower("MiXeD"), "mixed")
    
    // Test capitalize
    assert_eq_string(string_capitalize("hello"), "Hello")
    assert_eq_string(string_capitalize("HELLO"), "Hello")
    assert_eq_string(string_capitalize(""), "")
    
    // Test reverse
    assert_eq_string(string_reverse("hello"), "olleh")
    assert_eq_string(string_reverse("abc"), "cba")
    assert_eq_string(string_reverse(""), "")
    assert_eq_string(string_reverse("a"), "a")
}

slay test_string_trimming() {
    test_start("String Trimming")
    
    // Test trim all whitespace
    assert_eq_string(string_trim("  hello  "), "hello")
    assert_eq_string(string_trim("\t\ntest\r\n"), "test")
    assert_eq_string(string_trim("no-spaces"), "no-spaces")
    assert_eq_string(string_trim(""), "")
    assert_eq_string(string_trim("   "), "")
    
    // Test trim start
    assert_eq_string(string_trim_start("  hello  "), "hello  ")
    assert_eq_string(string_trim_start("\t\ntest"), "test")
    assert_eq_string(string_trim_start("no-spaces"), "no-spaces")
    
    // Test trim end
    assert_eq_string(string_trim_end("  hello  "), "  hello")
    assert_eq_string(string_trim_end("test\r\n"), "test")
    assert_eq_string(string_trim_end("no-spaces"), "no-spaces")
}

slay test_string_replacement() {
    test_start("String Replacement")
    
    // Test replace first occurrence
    assert_eq_string(string_replace("hello hello", "hello", "hi"), "hi hello")
    assert_eq_string(string_replace("hello world", "xyz", "abc"), "hello world")
    assert_eq_string(string_replace("", "old", "new"), "")
    
    // Test replace all occurrences
    assert_eq_string(string_replace_all("hello hello", "hello", "hi"), "hi hi")
    assert_eq_string(string_replace_all("hello world", "l", "x"), "hexxo worxd")
    assert_eq_string(string_replace_all("hello world", "xyz", "abc"), "hello world")
    
    // Test repeat
    assert_eq_string(string_repeat("abc", 3), "abcabcabc")
    assert_eq_string(string_repeat("x", 0), "")
    assert_eq_string(string_repeat("test", 1), "test")
}

slay test_string_padding() {
    test_start("String Padding")
    
    // Test pad left
    assert_eq_string(string_pad_left("hello", 10, " "), "     hello")
    assert_eq_string(string_pad_left("hello", 8, "0"), "000hello")
    assert_eq_string(string_pad_left("hello", 5, "x"), "hello")
    assert_eq_string(string_pad_left("hello", 3, "x"), "hello")
    
    // Test pad right
    assert_eq_string(string_pad_right("hello", 10, " "), "hello     ")
    assert_eq_string(string_pad_right("hello", 8, "0"), "hello000")
    assert_eq_string(string_pad_right("hello", 5, "x"), "hello")
    assert_eq_string(string_pad_right("hello", 3, "x"), "hello")
    
    // Test pad center
    assert_eq_string(string_pad_center("hello", 9, " "), "  hello  ")
    assert_eq_string(string_pad_center("hi", 6, "x"), "xxhixx")
    assert_eq_string(string_pad_center("hello", 5, "x"), "hello")
}

slay test_string_validation() {
    test_start("String Validation")
    
    // Test numeric validation
    assert_true(string_is_numeric("123"))
    assert_true(string_is_numeric("123.45"))
    assert_true(string_is_numeric("-123"))
    assert_true(string_is_numeric("+123"))
    assert_false(string_is_numeric("abc"))
    assert_false(string_is_numeric("123abc"))
    assert_false(string_is_numeric(""))
    assert_false(string_is_numeric("-"))
    
    // Test alphabetic validation
    assert_true(string_is_alpha("hello"))
    assert_true(string_is_alpha("HELLO"))
    assert_true(string_is_alpha("MiXeD"))
    assert_false(string_is_alpha("hello123"))
    assert_false(string_is_alpha("123"))
    assert_false(string_is_alpha(""))
    assert_false(string_is_alpha("hello!"))
    
    // Test alphanumeric validation
    assert_true(string_is_alphanumeric("hello123"))
    assert_true(string_is_alphanumeric("ABC123"))
    assert_true(string_is_alphanumeric("hello"))
    assert_true(string_is_alphanumeric("123"))
    assert_false(string_is_alphanumeric("hello!"))
    assert_false(string_is_alphanumeric("123-456"))
    assert_false(string_is_alphanumeric(""))
    
    // Test whitespace validation
    assert_true(string_is_whitespace("   "))
    assert_true(string_is_whitespace("\t\n\r"))
    assert_true(string_is_whitespace(" \t "))
    assert_false(string_is_whitespace("hello"))
    assert_false(string_is_whitespace("  hello  "))
    assert_false(string_is_whitespace(""))
}

slay test_string_conversion() {
    test_start("String Conversion")
    
    // Test string to integer
    assert_eq_int(string_to_int("123"), 123)
    assert_eq_int(string_to_int("-456"), -456)
    assert_eq_int(string_to_int("+789"), 789)
    assert_eq_int(string_to_int("0"), 0)
    assert_eq_int(string_to_int(""), 0)
    
    // Test string to boolean
    assert_true(string_to_bool("true"))
    assert_true(string_to_bool("TRUE"))
    assert_true(string_to_bool("based"))
    assert_true(string_to_bool("BASED"))
    assert_true(string_to_bool("1"))
    assert_false(string_to_bool("false"))
    assert_false(string_to_bool("FALSE"))
    assert_false(string_to_bool("cap"))
    assert_false(string_to_bool("0"))
    assert_false(string_to_bool(""))
    
    // Test conversions from other types
    assert_eq_string(string_from_int(123), "123")
    assert_eq_string(string_from_int(-456), "-456")
    assert_eq_string(string_from_int(0), "0")
    assert_eq_string(string_from_bool(based), "true")
    assert_eq_string(string_from_bool(cap), "false")
}

slay test_string_utilities() {
    test_start("String Utilities")
    
    // Test hash (basic consistency check)
    sus hash1 normie = string_hash("hello")
    sus hash2 normie = string_hash("hello")
    sus hash3 normie = string_hash("world")
    assert_eq_int(hash1, hash2)
    assert_true(hash1 != hash3)
    
    // Test Levenshtein distance
    assert_eq_int(string_levenshtein_distance("hello", "hello"), 0)
    assert_eq_int(string_levenshtein_distance("hello", "hallo"), 1)
    assert_eq_int(string_levenshtein_distance("hello", ""), 5)
    assert_eq_int(string_levenshtein_distance("", "hello"), 5)
    assert_eq_int(string_levenshtein_distance("", ""), 0)
    
    // Test string similarity
    assert_eq_string(tea(string_similarity("hello", "hello")), "1.0")
    assert_true(string_similarity("hello", "hallo") > 0.0)
    assert_true(string_similarity("hello", "hallo") < 1.0)
    assert_true(string_similarity("hello", "world") < 1.0)
    assert_eq_string(tea(string_similarity("", "")), "1.0")
}

slay test_string_edge_cases() {
    test_start("String Edge Cases")
    
    // Test empty string operations
    assert_eq_string(string_trim(""), "")
    assert_eq_string(string_to_upper(""), "")
    assert_eq_string(string_to_lower(""), "")
    assert_eq_string(string_reverse(""), "")
    assert_eq_string(string_capitalize(""), "")
    assert_eq_string(string_concatenate("", ""), "")
    
    // Test single character operations
    assert_eq_string(string_to_upper("a"), "A")
    assert_eq_string(string_to_lower("A"), "a")
    assert_eq_string(string_reverse("x"), "x")
    assert_eq_int(string_length("y"), 1)
    assert_eq_string(string_capitalize("a"), "A")
    
    // Test boundary conditions
    assert_eq_string(string_substring("hello", 0, 100), "hello")
    assert_eq_string(string_slice("hello", 0, 100), "hello")
    assert_eq_string(string_pad_left("hello", 3, "x"), "hello")
    assert_eq_string(string_pad_right("hello", 3, "x"), "hello")
}

slay test_string_performance() {
    test_start("String Performance")
    
    // Test with moderately long strings
    sus long_string tea = string_repeat("Hello World! ", 10)
    assert_eq_int(string_length(long_string), 130)
    assert_true(string_contains(long_string, "Hello"))
    assert_true(string_contains(long_string, "World"))
    
    // Test string operations on longer strings
    sus upper_long tea = string_to_upper(long_string)
    assert_true(string_contains(upper_long, "HELLO"))
    assert_true(string_contains(upper_long, "WORLD"))
    
    // Test replacement on longer strings
    sus replaced tea = string_replace_all(long_string, "Hello", "Hi")
    assert_true(string_contains(replaced, "Hi"))
    assert_false(string_contains(replaced, "Hello"))
}

slay run_all_string_pure_tests() {
    vibez.spill("🔥 Running Pure CURSED String Library Tests")
    vibez.spill("==============================================")
    
    test_string_core_functions()
    test_string_concatenation()
    test_string_slicing()
    test_string_comparison()
    test_string_search()
    test_string_transformation()
    test_string_trimming()
    test_string_replacement()
    test_string_padding()
    test_string_validation()
    test_string_conversion()
    test_string_utilities()
    test_string_edge_cases()
    test_string_performance()
    
    print_test_summary()
    damn run_all_tests()
}

// Auto-run tests when this file is executed
run_all_string_pure_tests()
