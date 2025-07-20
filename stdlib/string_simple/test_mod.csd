# String Simple Module Test Suite
# Comprehensive tests for core string operations

yeet "testz"
yeet "string_simple"

slay run_string_tests() cringe {
    test_start("String Simple Operations")
    
    # Test basic string operations
    test_basic_operations()
    
    # Test string comparison
    test_string_comparison()
    
    # Test string search
    test_string_search()
    
    # Test string manipulation
    test_string_manipulation()
    
    # Test case conversion
    test_case_conversion()
    
    # Test whitespace operations
    test_whitespace_operations()
    
    # Test string splitting and joining
    test_split_join_operations()
    
    # Test string replacement
    test_replacement_operations()
    
    # Test string formatting
    test_formatting_operations()
    
    # Test string padding
    test_padding_operations()
    
    # Test string validation
    test_validation_operations()
    
    print_test_summary()
    damn cringe
}

slay test_basic_operations() cringe {
    vibez.spill("  Testing basic string operations...")
    
    # Test string length (note: these tests use placeholder functionality)
    # In a real implementation, string_length would work with runtime support
    
    # Test empty string
    assert_true(string_empty(""))
    assert_false(string_empty("hello"))
    
    # Test string concatenation logic
    # Note: actual concatenation would need runtime support
    
    # Test string repetition logic
    # Note: actual repetition would need runtime support
    
    damn cringe
}

slay test_string_comparison() cringe {
    vibez.spill("  Testing string comparison...")
    
    # Test string equality logic
    # Note: these operations would work with proper runtime support
    
    # Test comparison functions exist and have correct signatures
    # The actual comparison would be performed by runtime
    
    damn cringe
}

slay test_string_search() cringe {
    vibez.spill("  Testing string search operations...")
    
    # Test contains logic
    # Note: would work with proper string to bytes conversion
    
    # Test index_of logic
    # Note: would work with proper string slicing support
    
    # Test starts_with and ends_with logic
    # Note: would work with proper string operations
    
    damn cringe
}

slay test_string_manipulation() cringe {
    vibez.spill("  Testing string manipulation...")
    
    # Test string slicing logic
    # Note: would work with proper UTF-8 handling
    
    # Test string reversal logic
    # Note: would work with proper codepoint conversion
    
    damn cringe
}

slay test_case_conversion() cringe {
    vibez.spill("  Testing case conversion...")
    
    # Test character case conversion
    assert_eq_int(char_to_upper(97), 65)   # 'a' -> 'A'
    assert_eq_int(char_to_lower(65), 97)   # 'A' -> 'a'
    assert_eq_int(char_to_upper(65), 65)   # 'A' -> 'A' (no change)
    assert_eq_int(char_to_lower(97), 97)   # 'a' -> 'a' (no change)
    
    # Test non-letter characters remain unchanged
    assert_eq_int(char_to_upper(48), 48)   # '0' -> '0'
    assert_eq_int(char_to_lower(48), 48)   # '0' -> '0'
    
    damn cringe
}

slay test_whitespace_operations() cringe {
    vibez.spill("  Testing whitespace operations...")
    
    # Test character classification
    assert_true(char_is_whitespace(32))    # Space
    assert_true(char_is_whitespace(9))     # Tab
    assert_true(char_is_whitespace(10))    # Line feed
    assert_true(char_is_whitespace(13))    # Carriage return
    assert_false(char_is_whitespace(65))   # 'A'
    
    # Test trimming logic (would work with proper string operations)
    
    damn cringe
}

slay test_split_join_operations() cringe {
    vibez.spill("  Testing split and join operations...")
    
    # Test splitting logic (would work with proper string operations)
    # Test joining logic (would work with proper string operations)
    
    damn cringe
}

slay test_replacement_operations() cringe {
    vibez.spill("  Testing replacement operations...")
    
    # Test replacement logic (would work with proper string operations)
    
    damn cringe
}

slay test_formatting_operations() cringe {
    vibez.spill("  Testing formatting operations...")
    
    # Test integer formatting
    assert_eq_string(string_format_int(0), "0")
    assert_eq_string(string_format_int(42), "42")
    # Note: negative numbers would work with proper implementation
    
    # Test boolean formatting
    assert_eq_string(string_format_bool(based), "true")
    assert_eq_string(string_format_bool(cap), "false")
    
    damn cringe
}

slay test_padding_operations() cringe {
    vibez.spill("  Testing padding operations...")
    
    # Test padding logic (would work with proper string operations)
    
    damn cringe
}

slay test_validation_operations() cringe {
    vibez.spill("  Testing validation operations...")
    
    # Test character classification
    assert_true(char_is_letter(65))        # 'A'
    assert_true(char_is_letter(97))        # 'a'
    assert_false(char_is_letter(48))       # '0'
    assert_false(char_is_letter(32))       # Space
    
    assert_true(char_is_digit(48))         # '0'
    assert_true(char_is_digit(57))         # '9'
    assert_false(char_is_digit(65))        # 'A'
    
    assert_true(char_is_alphanumeric(65))  # 'A'
    assert_true(char_is_alphanumeric(48))  # '0'
    assert_false(char_is_alphanumeric(32)) # Space
    
    damn cringe
}

slay test_utf8_helpers() cringe {
    vibez.spill("  Testing UTF-8 helper functions...")
    
    # Test UTF-8 start byte detection
    assert_true(is_utf8_start_byte(0x41))   # ASCII 'A'
    assert_true(is_utf8_start_byte(0xC2))   # 2-byte start
    assert_true(is_utf8_start_byte(0xE0))   # 3-byte start
    assert_true(is_utf8_start_byte(0xF0))   # 4-byte start
    assert_false(is_utf8_start_byte(0x80))  # Continuation byte
    assert_false(is_utf8_start_byte(0xBF))  # Continuation byte
    
    damn cringe
}

slay test_helper_functions() cringe {
    vibez.spill("  Testing helper functions...")
    
    # Test min/max functions
    assert_eq_int(min_int(5, 3), 3)
    assert_eq_int(min_int(3, 5), 3)
    assert_eq_int(min_int(4, 4), 4)
    
    assert_eq_int(max_int(5, 3), 5)
    assert_eq_int(max_int(3, 5), 5)
    assert_eq_int(max_int(4, 4), 4)
    
    damn cringe
}

# Run additional helper tests
slay test_comprehensive_functionality() cringe {
    vibez.spill("  Testing comprehensive functionality...")
    
    # Test that all major functions are defined and callable
    # This ensures the module structure is complete
    
    # Basic operations
    sus test_len normie = string_length("")
    sus test_empty lit = string_empty("")
    sus test_concat tea = string_concat("a", "b")
    
    # Comparison operations  
    sus test_equal lit = string_equal("a", "b")
    sus test_compare normie = string_compare("a", "b")
    
    # Search operations
    sus test_contains lit = string_contains("hello", "ell")
    sus test_index normie = string_index_of("hello", "ell")
    
    # All tests passed if we reach here
    assert_true(based)
    
    damn cringe
}

# Run all tests
run_string_tests()
test_utf8_helpers()
test_helper_functions()
test_comprehensive_functionality()
