// Comprehensive test suite for stringz module
yeet "testz"
yeet "stringz"

// ================================
// Core Specification Function Tests
// ================================

slay test_core_spec_functions() {
    test_start("Core specification functions")
    
    // Test Contains()
    assert_eq_bool(stringz.Contains("hello world", "world"), based)
    assert_eq_bool(stringz.Contains("hello world", "xyz"), cap)
    test_pass("Contains() function works")
    
    // Test HasPrefix()
    assert_eq_bool(stringz.HasPrefix("hello world", "hello"), based)
    assert_eq_bool(stringz.HasPrefix("hello world", "world"), cap)
    test_pass("HasPrefix() function works")
    
    // Test HasSuffix()
    assert_eq_bool(stringz.HasSuffix("hello world", "world"), based)
    assert_eq_bool(stringz.HasSuffix("hello world", "hello"), cap)
    test_pass("HasSuffix() function works")
    
    // Test Split()
    sus parts tea[value] = stringz.Split("hello,world,test", ",")
    assert_eq_int(len(parts), 3)
    assert_eq_string(parts[0], "hello")
    assert_eq_string(parts[1], "world")
    assert_eq_string(parts[2], "test")
    test_pass("Split() function works")
    
    // Test Join()
    sus words tea[value] = ["hello", "world", "test"]
    sus joined tea = stringz.Join(words, " ")
    assert_eq_string(joined, "hello world test")
    test_pass("Join() function works")
    
    // Test ToLower()
    sus lower tea = stringz.ToLower("HELLO WORLD")
    assert_eq_string(lower, "hello world")
    test_pass("ToLower() function works")
    
    // Test ToUpper()
    sus upper tea = stringz.ToUpper("hello world")
    assert_eq_string(upper, "HELLO WORLD")
    test_pass("ToUpper() function works")
    
    // Test Trim()
    sus trimmed tea = stringz.Trim("  hello world  ", " ")
    assert_eq_string(trimmed, "hello world")
    test_pass("Trim() function works")
    
    // Test Count()
    sus count normie = stringz.Count("hello hello hello", "hello")
    assert_eq_int(count, 3)
    test_pass("Count() function works")
}

// ================================
// String Validation Tests
// ================================

slay test_string_validation() {
    test_start("String validation functions")
    
    // Test IsEmpty()
    assert_eq_bool(stringz.IsEmpty(""), based)
    assert_eq_bool(stringz.IsEmpty("hello"), cap)
    test_pass("IsEmpty() function works")
    
    // Test IsNumeric()
    assert_eq_bool(stringz.IsNumeric("12345"), based)
    assert_eq_bool(stringz.IsNumeric("hello"), cap)
    assert_eq_bool(stringz.IsNumeric("123a45"), cap)
    test_pass("IsNumeric() function works")
    
    // Test IsAlpha()
    assert_eq_bool(stringz.IsAlpha("hello"), based)
    assert_eq_bool(stringz.IsAlpha("hello123"), cap)
    assert_eq_bool(stringz.IsAlpha(""), cap)
    test_pass("IsAlpha() function works")
    
    // Test IsAlphanumeric()
    assert_eq_bool(stringz.IsAlphanumeric("hello123"), based)
    assert_eq_bool(stringz.IsAlphanumeric("hello!"), cap)
    assert_eq_bool(stringz.IsAlphanumeric("123"), based)
    test_pass("IsAlphanumeric() function works")
    
    // Test IsWhitespace()
    assert_eq_bool(stringz.IsWhitespace("   "), based)
    assert_eq_bool(stringz.IsWhitespace("hello"), cap)
    assert_eq_bool(stringz.IsWhitespace(""), cap)
    test_pass("IsWhitespace() function works")
    
    // Test IsAscii()
    assert_eq_bool(stringz.IsAscii("hello123"), based)
    assert_eq_bool(stringz.IsAscii("hello world!"), based)
    test_pass("IsAscii() function works")
}

// ================================
// String Manipulation Tests
// ================================

slay test_string_manipulation() {
    test_start("String manipulation functions")
    
    // Test IndexOf()
    assert_eq_int(stringz.IndexOf("hello world", "world"), 6)
    assert_eq_int(stringz.IndexOf("hello world", "xyz"), -1)
    test_pass("IndexOf() function works")
    
    // Test LastIndexOf()
    assert_eq_int(stringz.LastIndexOf("hello hello", "hello"), 6)
    assert_eq_int(stringz.LastIndexOf("hello world", "xyz"), -1)
    test_pass("LastIndexOf() function works")
    
    // Test ReplaceAll()
    sus replaced tea = stringz.ReplaceAll("hello world hello", "hello", "hi")
    assert_eq_string(replaced, "hi world hi")
    test_pass("ReplaceAll() function works")
    
    // Test Repeat()
    sus repeated tea = stringz.Repeat("abc", 3)
    assert_eq_string(repeated, "abcabcabc")
    test_pass("Repeat() function works")
    
    // Test Reverse()
    sus reversed tea = stringz.Reverse("hello")
    assert_eq_string(reversed, "olleh")
    test_pass("Reverse() function works")
    
    // Test Capitalize()
    sus capitalized tea = stringz.Capitalize("hello world")
    assert_eq_string(capitalized, "Hello world")
    test_pass("Capitalize() function works")
}

// ================================
// String Trimming Tests
// ================================

slay test_string_trimming() {
    test_start("String trimming functions")
    
    // Test TrimLeft()
    sus trimmed_left tea = stringz.TrimLeft("  hello world  ", " ")
    assert_eq_string(trimmed_left, "hello world  ")
    test_pass("TrimLeft() function works")
    
    // Test TrimRight()
    sus trimmed_right tea = stringz.TrimRight("  hello world  ", " ")
    assert_eq_string(trimmed_right, "  hello world")
    test_pass("TrimRight() function works")
    
    // Test Trim() with custom cutset
    sus trimmed_custom tea = stringz.Trim("!!!hello world!!!", "!")
    assert_eq_string(trimmed_custom, "hello world")
    test_pass("Trim() with custom cutset works")
}

// ================================
// String Utility Tests
// ================================

slay test_string_utilities() {
    test_start("String utility functions")
    
    // Test Len()
    assert_eq_int(stringz.Len("hello"), 5)
    assert_eq_int(stringz.Len(""), 0)
    test_pass("Len() function works")
    
    // Test Substring()
    sus sub tea = stringz.Substring("hello world", 6, 5)
    assert_eq_string(sub, "world")
    test_pass("Substring() function works")
    
    // Test Slice()
    sus slice tea = stringz.Slice("hello world", 0, 5)
    assert_eq_string(slice, "hello")
    test_pass("Slice() function works")
    
    // Test CharAt()
    sus char tea = stringz.CharAt("hello", 1)
    assert_eq_string(char, "e")
    test_pass("CharAt() function works")
}

// ================================
// String Comparison Tests
// ================================

slay test_string_comparison() {
    test_start("String comparison functions")
    
    // Test Compare()
    assert_eq_int(stringz.Compare("apple", "banana"), -1)
    assert_eq_int(stringz.Compare("banana", "apple"), 1)
    assert_eq_int(stringz.Compare("apple", "apple"), 0)
    test_pass("Compare() function works")
    
    // Test Equals()
    assert_eq_bool(stringz.Equals("hello", "hello"), based)
    assert_eq_bool(stringz.Equals("hello", "world"), cap)
    test_pass("Equals() function works")
    
    // Test EqualsIgnoreCase()
    assert_eq_bool(stringz.EqualsIgnoreCase("Hello", "hello"), based)
    assert_eq_bool(stringz.EqualsIgnoreCase("Hello", "world"), cap)
    test_pass("EqualsIgnoreCase() function works")
}

// ================================
// String Conversion Tests
// ================================

slay test_string_conversion() {
    test_start("String conversion functions")
    
    // Test ToInt()
    assert_eq_int(stringz.ToInt("123"), 123)
    assert_eq_int(stringz.ToInt("-456"), -456)
    test_pass("ToInt() function works")
    
    // Test ToFloat()
    assert_eq_float(stringz.ToFloat("123.45"), 123.45)
    assert_eq_float(stringz.ToFloat("-67.89"), -67.89)
    test_pass("ToFloat() function works")
    
    // Test ToBool()
    assert_eq_bool(stringz.ToBool("true"), based)
    assert_eq_bool(stringz.ToBool("false"), cap)
    test_pass("ToBool() function works")
    
    // Test FromInt()
    assert_eq_string(stringz.FromInt(123), "123")
    assert_eq_string(stringz.FromInt(-456), "-456")
    test_pass("FromInt() function works")
    
    // Test FromFloat()
    sus float_str tea = stringz.FromFloat(123.45)
    assert_true(stringz.Contains(float_str, "123"))
    test_pass("FromFloat() function works")
    
    // Test FromBool()
    assert_eq_string(stringz.FromBool(based), "true")
    assert_eq_string(stringz.FromBool(cap), "false")
    test_pass("FromBool() function works")
}

// ================================
// String Padding Tests
// ================================

slay test_string_padding() {
    test_start("String padding functions")
    
    // Test PadLeft()
    sus padded_left tea = stringz.PadLeft("hello", 10, "0")
    assert_eq_string(padded_left, "00000hello")
    test_pass("PadLeft() function works")
    
    // Test PadRight()
    sus padded_right tea = stringz.PadRight("hello", 10, "0")
    assert_eq_string(padded_right, "hello00000")
    test_pass("PadRight() function works")
    
    // Test PadCenter()
    sus padded_center tea = stringz.PadCenter("hello", 11, " ")
    assert_eq_string(padded_center, "   hello   ")
    test_pass("PadCenter() function works")
}

// ================================
// Advanced String Tests
// ================================

slay test_advanced_string_functions() {
    test_start("Advanced string functions")
    
    // Test Hash()
    sus hash1 normie = stringz.Hash("hello")
    sus hash2 normie = stringz.Hash("hello")
    sus hash3 normie = stringz.Hash("world")
    
    assert_eq_int(hash1, hash2)
    assert_true(hash1 != hash3)
    test_pass("Hash() function works")
    
    // Test Escape()
    sus escaped tea = stringz.Escape("hello\nworld")
    assert_true(stringz.Contains(escaped, "\\n"))
    test_pass("Escape() function works")
    
    // Test Unescape()
    sus unescaped tea = stringz.Unescape("hello\\nworld")
    assert_true(stringz.Contains(unescaped, "\n"))
    test_pass("Unescape() function works")
}

// ================================
// Integration Tests
// ================================

slay test_integration() {
    test_start("Integration tests")
    
    // Test complex string processing
    sus original tea = "  Hello, World!  "
    sus processed tea = stringz.ToLower(stringz.Trim(original, " "))
    assert_eq_string(processed, "hello, world!")
    test_pass("Complex string processing works")
    
    // Test split and join roundtrip
    sus sentence tea = "The quick brown fox"
    sus words tea[value] = stringz.Split(sentence, " ")
    sus rejoined tea = stringz.Join(words, " ")
    assert_eq_string(rejoined, sentence)
    test_pass("Split and join roundtrip works")
    
    // Test string validation chain
    sus test_string tea = "hello123"
    assert_eq_bool(stringz.IsAlphanumeric(test_string), based)
    assert_eq_bool(stringz.IsNumeric(test_string), cap)
    assert_eq_bool(stringz.IsAlpha(test_string), cap)
    test_pass("String validation chain works")
    
    // Test case conversion consistency
    sus mixed_case tea = "HeLLo WoRLd"
    sus lower_case tea = stringz.ToLower(mixed_case)
    sus upper_case tea = stringz.ToUpper(mixed_case)
    
    assert_eq_string(lower_case, "hello world")
    assert_eq_string(upper_case, "HELLO WORLD")
    assert_eq_bool(stringz.EqualsIgnoreCase(lower_case, upper_case), based)
    test_pass("Case conversion consistency works")
}

// ================================
// Performance Tests
// ================================

slay test_performance() {
    test_start("Performance tests")
    
    // Test with large strings
    sus large_string tea = stringz.Repeat("Hello World! ", 100)
    assert_eq_int(stringz.Len(large_string), 1300)
    
    sus contains_result lit = stringz.Contains(large_string, "World")
    assert_eq_bool(contains_result, based)
    
    sus count_result normie = stringz.Count(large_string, "Hello")
    assert_eq_int(count_result, 100)
    
    test_pass("Performance with large strings works")
    
    // Test multiple operations
    sus base_string tea = "test string for performance"
    sus operations normie = 0
    
    bestie i normie := 0; i < 10; i++ {
        sus upper tea = stringz.ToUpper(base_string)
        sus lower tea = stringz.ToLower(upper)
        sus trimmed tea = stringz.Trim(lower, " ")
        sus replaced tea = stringz.ReplaceAll(trimmed, "test", "best")
        operations = operations + 4
    }
    
    assert_eq_int(operations, 40)
    test_pass("Multiple operations performance works")
}

// ================================
// Main Test Runner
// ================================

slay run_all_tests() {
    println("========================================")
    println("         STRINGZ MODULE TESTS")
    println("========================================")
    println("")
    
    // Run all test functions
    test_core_spec_functions()
    test_string_validation()
    test_string_manipulation()
    test_string_trimming()
    test_string_utilities()
    test_string_comparison()
    test_string_conversion()
    test_string_padding()
    test_advanced_string_functions()
    test_integration()
    test_performance()
    
    println("")
    println("========================================")
    print_test_summary()
    println("========================================")
}

// Start the test suite
run_all_tests()
