//==============================================================================
// Enhanced String Processing Test Suite
// Comprehensive testing of advanced string algorithms
// Tests for proper Unicode handling and algorithm correctness
//==============================================================================

yeet "stringz_enhanced_complete"
yeet "vibez"

// Testing framework functions
sus test_count drip = 0
sus test_passed drip = 0
sus test_failed drip = 0

slay test_start(name tea) {
    test_count = test_count + 1
    vibez.spill("🧪 Running test: " + name)
}

slay test_pass(message tea) {
    test_passed = test_passed + 1
    vibez.spill("  ✅ PASS: " + message)
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    vibez.spill("  ❌ FAIL: " + message)
}

slay assert_eq_string(actual tea, expected tea, context tea) {
    ready actual == expected {
        test_pass(context + ": got '" + actual + "'")
    } otherwise {
        test_fail(context + ": expected '" + expected + "', got '" + actual + "'")
    }
}

slay assert_eq_int(actual drip, expected drip, context tea) {
    ready actual == expected {
        test_pass(context + ": got " + to_int(actual))
    } otherwise {
        test_fail(context + ": expected " + to_int(expected) + ", got " + to_int(actual))
    }
}

slay assert_eq_bool(actual lit, expected lit, context tea) {
    ready actual == expected {
        test_pass(context + ": got " + to_string(actual))
    } otherwise {
        test_fail(context + ": expected " + to_string(expected) + ", got " + to_string(actual))
    }
}

slay assert_string_array_eq(actual []tea, expected []tea, context tea) {
    sus actual_len drip = length_string_array(actual)
    sus expected_len drip = length_string_array(expected)
    
    ready actual_len != expected_len {
        test_fail(context + ": length mismatch - expected " + to_int(expected_len) + ", got " + to_int(actual_len))
        damn
    }
    
    sus i drip = 0
    bestie i < actual_len {
        ready actual[i] != expected[i] {
            test_fail(context + ": element " + to_int(i) + " mismatch - expected '" + expected[i] + "', got '" + actual[i] + "'")
            damn
        }
        i = i + 1
    }
    
    test_pass(context + ": arrays match")
}

//==============================================================================
// STRING SPLITTING TESTS
//==============================================================================

slay test_string_splitting() {
    test_start("Enhanced String Splitting")
    
    // Basic splitting
    sus result1 []tea = split("a,b,c", ",")
    sus expected1 []tea = ["a", "b", "c"]
    assert_string_array_eq(result1, expected1, "Basic comma split")
    
    // Empty string splitting
    sus result2 []tea = split("", ",")
    sus expected2 []tea = []
    assert_string_array_eq(result2, expected2, "Empty string split")
    
    // No delimiter found
    sus result3 []tea = split("hello world", ",")
    sus expected3 []tea = ["hello world"]
    assert_string_array_eq(result3, expected3, "No delimiter found")
    
    // Empty delimiter
    sus result4 []tea = split("test", "")
    sus expected4 []tea = ["test"]
    assert_string_array_eq(result4, expected4, "Empty delimiter")
    
    // Multiple character delimiter
    sus result5 []tea = split("one::two::three", "::")
    sus expected5 []tea = ["one", "two", "three"]
    assert_string_array_eq(result5, expected5, "Multi-char delimiter")
    
    // Consecutive delimiters
    sus result6 []tea = split("a,,b", ",")
    sus expected6 []tea = ["a", "", "b"]
    assert_string_array_eq(result6, expected6, "Consecutive delimiters")
}

//==============================================================================
// STRING JOINING TESTS
//==============================================================================

slay test_string_joining() {
    test_start("Enhanced String Joining")
    
    // Basic joining
    sus parts1 []tea = ["hello", "world", "test"]
    sus result1 tea = join(parts1, " ")
    assert_eq_string(result1, "hello world test", "Basic space join")
    
    // Empty array joining
    sus parts2 []tea = []
    sus result2 tea = join(parts2, ",")
    assert_eq_string(result2, "", "Empty array join")
    
    // Single element joining
    sus parts3 []tea = ["single"]
    sus result3 tea = join(parts3, ",")
    assert_eq_string(result3, "single", "Single element join")
    
    // Empty delimiter joining
    sus parts4 []tea = ["a", "b", "c"]
    sus result4 tea = join(parts4, "")
    assert_eq_string(result4, "abc", "Empty delimiter join")
    
    // Multi-character delimiter
    sus parts5 []tea = ["one", "two", "three"]
    sus result5 tea = join(parts5, " :: ")
    assert_eq_string(result5, "one :: two :: three", "Multi-char delimiter join")
}

//==============================================================================
// STRING REPLACEMENT TESTS
//==============================================================================

slay test_string_replacement() {
    test_start("Enhanced String Replacement")
    
    // Basic replace (first occurrence)
    sus result1 tea = replace("hello hello world", "hello", "hi")
    assert_eq_string(result1, "hi hello world", "Basic first replacement")
    
    // Replace all occurrences
    sus result2 tea = replace_all("hello hello world", "hello", "hi")
    assert_eq_string(result2, "hi hi world", "Replace all occurrences")
    
    // Replace with longer string
    sus result3 tea = replace("test", "test", "replacement")
    assert_eq_string(result3, "replacement", "Replace with longer string")
    
    // Replace with shorter string
    sus result4 tea = replace("hello", "hello", "hi")
    assert_eq_string(result4, "hi", "Replace with shorter string")
    
    // Replace with empty string
    sus result5 tea = replace("remove this", " this", "")
    assert_eq_string(result5, "remove", "Replace with empty string")
    
    // Pattern not found
    sus result6 tea = replace("hello world", "xyz", "abc")
    assert_eq_string(result6, "hello world", "Pattern not found")
}

//==============================================================================
// STRING PADDING TESTS
//==============================================================================

slay test_string_padding() {
    test_start("Enhanced String Padding")
    
    // Left padding
    sus result1 tea = pad_left("hello", 10, " ")
    assert_eq_string(result1, "     hello", "Basic left padding")
    
    // Right padding
    sus result2 tea = pad_right("hello", 10, " ")
    assert_eq_string(result2, "hello     ", "Basic right padding")
    
    // Center padding
    sus result3 tea = center("hello", 11, " ")
    assert_eq_string(result3, "   hello   ", "Basic center padding")
    
    // No padding needed
    sus result4 tea = pad_left("hello", 5, " ")
    assert_eq_string(result4, "hello", "No padding needed")
    
    // Custom pad character
    sus result5 tea = pad_left("test", 8, "0")
    assert_eq_string(result5, "0000test", "Custom pad character")
}

//==============================================================================
// STRING PARSING TESTS
//==============================================================================

slay test_string_parsing() {
    test_start("Enhanced String Parsing")
    
    // Integer parsing
    assert_eq_int(parse_int("123"), 123, "Basic positive integer")
    assert_eq_int(parse_int("-456"), -456, "Basic negative integer")
    assert_eq_int(parse_int("0"), 0, "Zero parsing")
    assert_eq_int(parse_int("  42  "), 42, "Integer with whitespace")
    assert_eq_int(parse_int("+789"), 789, "Positive sign integer")
    
    // Boolean parsing
    assert_eq_bool(parse_bool("true"), based, "Boolean true")
    assert_eq_bool(parse_bool("false"), cap, "Boolean false")
    assert_eq_bool(parse_bool("yes"), based, "Boolean yes")
    assert_eq_bool(parse_bool("no"), cap, "Boolean no")
    assert_eq_bool(parse_bool("1"), based, "Boolean 1")
    assert_eq_bool(parse_bool("0"), cap, "Boolean 0")
    
    // String conversion from integer
    assert_eq_string(to_int(42), "42", "Integer to string")
    assert_eq_string(to_int(-123), "-123", "Negative integer to string")
    assert_eq_string(to_int(0), "0", "Zero to string")
    
    // String conversion from boolean
    assert_eq_string(to_string(based), "true", "True to string")
    assert_eq_string(to_string(cap), "false", "False to string")
}

//==============================================================================
// STRING VALIDATION TESTS
//==============================================================================

slay test_string_validation() {
    test_start("Enhanced String Validation")
    
    // Numeric validation
    assert_eq_bool(is_numeric("123"), based, "Basic numeric string")
    assert_eq_bool(is_numeric("-456"), based, "Negative numeric string")
    assert_eq_bool(is_numeric("12.34"), cap, "Decimal not numeric (integer only)")
    assert_eq_bool(is_numeric("abc"), cap, "Non-numeric string")
    assert_eq_bool(is_numeric(""), cap, "Empty string not numeric")
    assert_eq_bool(is_numeric("  123  "), based, "Numeric with whitespace")
    
    // Alphabetic validation
    assert_eq_bool(is_alpha("hello"), based, "Basic alphabetic")
    assert_eq_bool(is_alpha("Hello"), based, "Mixed case alphabetic")
    assert_eq_bool(is_alpha("hello123"), cap, "Alphabetic with numbers")
    assert_eq_bool(is_alpha(""), cap, "Empty string not alphabetic")
    
    // Alphanumeric validation
    assert_eq_bool(is_alphanumeric("hello123"), based, "Basic alphanumeric")
    assert_eq_bool(is_alphanumeric("ABC123"), based, "Uppercase alphanumeric")
    assert_eq_bool(is_alphanumeric("hello!"), cap, "Alphanumeric with special char")
    assert_eq_bool(is_alphanumeric(""), cap, "Empty string not alphanumeric")
    
    // String length
    assert_eq_int(len_string("hello"), 5, "Basic string length")
    assert_eq_int(len_string(""), 0, "Empty string length")
    assert_eq_int(len_string("test string"), 11, "String with space length")
}

//==============================================================================
// STRING COMPARISON TESTS
//==============================================================================

slay test_string_comparison() {
    test_start("Enhanced String Comparison")
    
    // Contains tests
    assert_eq_bool(contains("hello world", "world"), based, "Basic contains")
    assert_eq_bool(contains("hello world", "xyz"), cap, "Contains not found")
    assert_eq_bool(contains("hello world", ""), based, "Empty string contained")
    assert_eq_bool(contains("", "test"), cap, "Empty string contains nothing")
    
    // Starts with tests
    assert_eq_bool(starts_with("hello world", "hello"), based, "Basic starts with")
    assert_eq_bool(starts_with("hello world", "world"), cap, "Starts with false")
    assert_eq_bool(starts_with("hello world", ""), based, "Empty prefix starts with")
    assert_eq_bool(starts_with("", "test"), cap, "Empty string starts with nothing")
    
    // Ends with tests
    assert_eq_bool(ends_with("hello world", "world"), based, "Basic ends with")
    assert_eq_bool(ends_with("hello world", "hello"), cap, "Ends with false")
    assert_eq_bool(ends_with("hello world", ""), based, "Empty suffix ends with")
    assert_eq_bool(ends_with("", "test"), cap, "Empty string ends with nothing")
}

//==============================================================================
// CASE CONVERSION TESTS
//==============================================================================

slay test_case_conversion() {
    test_start("Enhanced Case Conversion")
    
    // Uppercase conversion
    assert_eq_string(to_upper("hello"), "HELLO", "Basic uppercase")
    assert_eq_string(to_upper("Hello World"), "HELLO WORLD", "Mixed case uppercase")
    assert_eq_string(to_upper("123abc"), "123ABC", "Alphanumeric uppercase")
    assert_eq_string(to_upper(""), "", "Empty string uppercase")
    
    // Lowercase conversion
    assert_eq_string(to_lower("HELLO"), "hello", "Basic lowercase")
    assert_eq_string(to_lower("Hello World"), "hello world", "Mixed case lowercase")
    assert_eq_string(to_lower("ABC123"), "abc123", "Alphanumeric lowercase")
    assert_eq_string(to_lower(""), "", "Empty string lowercase")
    
    // Title case conversion
    assert_eq_string(to_title_case("hello world"), "Hello World", "Basic title case")
    assert_eq_string(to_title_case("the quick brown fox"), "The Quick Brown Fox", "Multi-word title case")
    
    // Sentence case conversion
    assert_eq_string(to_sentence_case("hello world"), "Hello world", "Basic sentence case")
    assert_eq_string(to_sentence_case("THE QUICK BROWN FOX"), "The quick brown fox", "Uppercase to sentence case")
}

//==============================================================================
// TRIMMING TESTS
//==============================================================================

slay test_string_trimming() {
    test_start("Enhanced String Trimming")
    
    // Basic trimming
    assert_eq_string(trim("  hello world  "), "hello world", "Basic whitespace trim")
    assert_eq_string(trim("hello world"), "hello world", "No whitespace trim")
    assert_eq_string(trim("   "), "", "Only whitespace trim")
    assert_eq_string(trim(""), "", "Empty string trim")
    
    // Left trimming
    assert_eq_string(trim_left("  hello world  "), "hello world  ", "Left whitespace trim")
    
    // Right trimming
    assert_eq_string(trim_right("  hello world  "), "  hello world", "Right whitespace trim")
    
    // Custom character trimming
    assert_eq_string(trim_chars("...hello...", "."), "hello", "Custom char trim")
    assert_eq_string(trim_chars("abchelloabc", "abc"), "hello", "Multiple custom chars trim")
}

//==============================================================================
// ESCAPE/UNESCAPE TESTS
//==============================================================================

slay test_string_escaping() {
    test_start("Enhanced String Escaping")
    
    // JSON escaping
    sus json_input tea = "Hello \"World\"\nNew line"
    sus json_expected tea = "Hello \\\"World\\\"\\nNew line"
    assert_eq_string(escape_json(json_input), json_expected, "JSON escape test")
    
    // HTML escaping
    sus html_input tea = "<script>alert('test');</script>"
    sus html_expected tea = "&lt;script&gt;alert('test');&lt;/script&gt;"
    assert_eq_string(escape_html(html_input), html_expected, "HTML escape test")
    
    // URL escaping
    sus url_input tea = "hello world & test"
    sus url_expected tea = "hello%20world%20%26%20test"
    assert_eq_string(escape_url(url_input), url_expected, "URL escape test")
}

//==============================================================================
// ADVANCED UTILITY TESTS
//==============================================================================

slay test_advanced_utilities() {
    test_start("Advanced String Utilities")
    
    // String repetition
    assert_eq_string(repeat("abc", 3), "abcabcabc", "Basic string repetition")
    assert_eq_string(repeat("test", 1), "test", "Single repetition")
    assert_eq_string(repeat("x", 0), "", "Zero repetition")
    assert_eq_string(repeat("", 5), "", "Empty string repetition")
    
    // String reversal
    assert_eq_string(reverse("hello"), "olleh", "Basic string reversal")
    assert_eq_string(reverse(""), "", "Empty string reversal")
    assert_eq_string(reverse("a"), "a", "Single character reversal")
    assert_eq_string(reverse("racecar"), "racecar", "Palindrome reversal")
    
    // String comparison
    assert_eq_int(compare("abc", "abc"), 0, "Equal string comparison")
    assert_eq_int(compare("abc", "def"), -1, "Less than comparison")
    assert_eq_int(compare("def", "abc"), 1, "Greater than comparison")
    
    // Case-insensitive comparison
    assert_eq_int(compare_ignore_case("ABC", "abc"), 0, "Case-insensitive equal")
    assert_eq_int(compare_ignore_case("ABC", "DEF"), -1, "Case-insensitive less than")
}

//==============================================================================
// TEMPLATE FORMATTING TESTS
//==============================================================================

slay test_template_formatting() {
    test_start("Template Formatting")
    
    // Basic placeholder replacement
    sus replacements1 []tea = ["World"]
    sus result1 tea = format_template("Hello {}", replacements1)
    assert_eq_string(result1, "Hello World", "Basic placeholder replacement")
    
    // Multiple placeholders
    sus replacements2 []tea = ["John", "25"]
    sus result2 tea = format_template("Name: {}, Age: {}", replacements2)
    assert_eq_string(result2, "Name: John, Age: 25", "Multiple placeholder replacement")
    
    // Numbered placeholders
    sus replacements3 []tea = ["first", "second"]
    sus result3 tea = format_template("{1} comes after {0}", replacements3)
    assert_eq_string(result3, "second comes after first", "Numbered placeholder replacement")
    
    // Key-value interpolation
    sus result4 tea = interpolate("Hello {name}!", "name", "Alice")
    assert_eq_string(result4, "Hello Alice!", "Key-value interpolation")
}

//==============================================================================
// MAIN TEST RUNNER
//==============================================================================

slay run_all_tests() {
    vibez.spill("🚀 Starting Enhanced String Processing Test Suite")
    vibez.spill("=" * 60)
    
    test_string_splitting()
    test_string_joining()
    test_string_replacement()
    test_string_padding()
    test_string_parsing()
    test_string_validation()
    test_string_comparison()
    test_case_conversion()
    test_string_trimming()
    test_string_escaping()
    test_advanced_utilities()
    test_template_formatting()
    
    vibez.spill("=" * 60)
    vibez.spill("📊 Test Results Summary:")
    vibez.spill("  Total Tests: " + to_int(test_count))
    vibez.spill("  Passed: " + to_int(test_passed))
    vibez.spill("  Failed: " + to_int(test_failed))
    
    ready test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! Enhanced string processing is working correctly.")
    } otherwise {
        vibez.spill("💥 " + to_int(test_failed) + " tests failed. Need to investigate issues.")
    }
}

// Run the test suite
run_all_tests()
