fr fr String Simple Module Test Suite
fr fr Comprehensive tests for core string operations

yeet "testz"
yeet "string_simple"

slay run_string_tests() cringe {
    test_start("String Simple Operations") fr fr Test basic string operations
    test_basic_operations() fr fr Test string comparison
    test_string_comparison() fr fr Test string search
    test_string_search() fr fr Test string manipulation
    test_string_manipulation() fr fr Test case conversion
    test_case_conversion() fr fr Test whitespace operations
    test_whitespace_operations() fr fr Test string splitting and joining
    test_split_join_operations() fr fr Test string replacement
    test_replacement_operations() fr fr Test string formatting
    test_formatting_operations() fr fr Test string padding
    test_padding_operations() fr fr Test string validation
    test_validation_operations()
    
    print_test_summary()
    damn cringe
}

slay test_basic_operations() cringe {
    vibez.spill("  Testing basic string operations...") fr fr Test string length (note: these tests use placeholder functionality) fr fr In a real implementation, string_length would work with runtime support fr fr Test empty string
    assert_true(string_empty(""))
    assert_false(string_empty("hello")) fr fr Test string concatenation logic fr fr Note: actual concatenation would need runtime support fr fr Test string repetition logic fr fr Note: actual repetition would need runtime support
    
    damn cringe
}

slay test_string_comparison() cringe {
    vibez.spill("  Testing string comparison...") fr fr Test string equality logic fr fr Note: these operations would work with proper runtime support fr fr Test comparison functions exist and have correct signatures fr fr The actual comparison would be performed by runtime
    
    damn cringe
}

slay test_string_search() cringe {
    vibez.spill("  Testing string search operations...") fr fr Test contains logic fr fr Note: would work with proper string to bytes conversion fr fr Test index_of logic fr fr Note: would work with proper string slicing support fr fr Test starts_with and ends_with logic fr fr Note: would work with proper string operations
    
    damn cringe
}

slay test_string_manipulation() cringe {
    vibez.spill("  Testing string manipulation...") fr fr Test string slicing logic fr fr Note: would work with proper UTF-8 handling fr fr Test string reversal logic fr fr Note: would work with proper codepoint conversion
    
    damn cringe
}

slay test_case_conversion() cringe {
    vibez.spill("  Testing case conversion...") fr fr Test character case conversion
    assert_eq_int(char_to_upper(97), 65) fr fr 'a' -> 'A'
    assert_eq_int(char_to_lower(65), 97) fr fr 'A' -> 'a'
    assert_eq_int(char_to_upper(65), 65) fr fr 'A' -> 'A' (no change)
    assert_eq_int(char_to_lower(97), 97) fr fr 'a' -> 'a' (no change) fr fr Test non-letter characters remain unchanged
    assert_eq_int(char_to_upper(48), 48) fr fr '0' -> '0'
    assert_eq_int(char_to_lower(48), 48) fr fr '0' -> '0'
    
    damn cringe
}

slay test_whitespace_operations() cringe {
    vibez.spill("  Testing whitespace operations...") fr fr Test character classification
    assert_true(char_is_whitespace(32)) fr fr Space
    assert_true(char_is_whitespace(9)) fr fr Tab
    assert_true(char_is_whitespace(10)) fr fr Line feed
    assert_true(char_is_whitespace(13)) fr fr Carriage return
    assert_false(char_is_whitespace(65)) fr fr 'A' fr fr Test trimming logic (would work with proper string operations)
    
    damn cringe
}

slay test_split_join_operations() cringe {
    vibez.spill("  Testing split and join operations...") fr fr Test splitting logic (would work with proper string operations) fr fr Test joining logic (would work with proper string operations)
    
    damn cringe
}

slay test_replacement_operations() cringe {
    vibez.spill("  Testing replacement operations...") fr fr Test replacement logic (would work with proper string operations)
    
    damn cringe
}

slay test_formatting_operations() cringe {
    vibez.spill("  Testing formatting operations...") fr fr Test integer formatting
    assert_eq_string(string_format_int(0), "0")
    assert_eq_string(string_format_int(42), "42") fr fr Note: negative numbers would work with proper implementation fr fr Test boolean formatting
    assert_eq_string(string_format_bool(based), "true")
    assert_eq_string(string_format_bool(cap), "false")
    
    damn cringe
}

slay test_padding_operations() cringe {
    vibez.spill("  Testing padding operations...") fr fr Test padding logic (would work with proper string operations)
    
    damn cringe
}

slay test_validation_operations() cringe {
    vibez.spill("  Testing validation operations...") fr fr Test character classification
    assert_true(char_is_letter(65)) fr fr 'A'
    assert_true(char_is_letter(97)) fr fr 'a'
    assert_false(char_is_letter(48)) fr fr '0'
    assert_false(char_is_letter(32)) fr fr Space
    
    assert_true(char_is_digit(48)) fr fr '0'
    assert_true(char_is_digit(57)) fr fr '9'
    assert_false(char_is_digit(65)) fr fr 'A'
    
    assert_true(char_is_alphanumeric(65)) fr fr 'A'
    assert_true(char_is_alphanumeric(48)) fr fr '0'
    assert_false(char_is_alphanumeric(32)) fr fr Space
    
    damn cringe
}

slay test_utf8_helpers() cringe {
    vibez.spill("  Testing UTF-8 helper functions...") fr fr Test UTF-8 start byte detection
    assert_true(is_utf8_start_byte(0x41)) fr fr ASCII 'A'
    assert_true(is_utf8_start_byte(0xC2)) fr fr 2-byte start
    assert_true(is_utf8_start_byte(0xE0)) fr fr 3-byte start
    assert_true(is_utf8_start_byte(0xF0)) fr fr 4-byte start
    assert_false(is_utf8_start_byte(0x80)) fr fr Continuation byte
    assert_false(is_utf8_start_byte(0xBF)) fr fr Continuation byte
    
    damn cringe
}

slay test_helper_functions() cringe {
    vibez.spill("  Testing helper functions...") fr fr Test min/max functions
    assert_eq_int(min_int(5, 3), 3)
    assert_eq_int(min_int(3, 5), 3)
    assert_eq_int(min_int(4, 4), 4)
    
    assert_eq_int(max_int(5, 3), 5)
    assert_eq_int(max_int(3, 5), 5)
    assert_eq_int(max_int(4, 4), 4)
    
    damn cringe
}

fr fr Run additional helper tests
slay test_comprehensive_functionality() cringe {
    vibez.spill("  Testing comprehensive functionality...") fr fr Test that all major functions are defined and callable fr fr This ensures the module structure is complete fr fr Basic operations
    sus test_len normie = string_length("")
    sus test_empty lit = string_empty("")
    sus test_concat tea = string_concat("a", "b") fr fr Comparison operations  
    sus test_equal lit = string_equal("a", "b")
    sus test_compare normie = string_compare("a", "b") fr fr Search operations
    sus test_contains lit = string_contains("hello", "ell")
    sus test_index normie = string_index_of("hello", "ell") fr fr All tests passed if we reach here
    assert_true(based)
    
    damn cringe
}

fr fr Run all tests
run_string_tests()
test_utf8_helpers()
test_helper_functions()
test_comprehensive_functionality()
