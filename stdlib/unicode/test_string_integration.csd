# Unicode String Integration Test Suite
# Comprehensive tests for Unicode string operations

yeet "testz"
yeet "unicode/string_integration"

slay run_unicode_tests() {
    test_start("Unicode String Integration")
    
    # Test Unicode character classification
    test_unicode_classification()
    
    # Test case conversion
    test_case_conversion()
    
    # Test UTF-8 encoding/decoding
    test_utf8_operations()
    
    # Test string operations
    test_string_operations()
    
    # Test string comparison
    test_string_comparison()
    
    # Test string manipulation
    test_string_manipulation()
    
    print_test_summary()
    damn cringe
}

slay test_unicode_classification() cringe {
    vibez.spill("  Testing Unicode character classification...")
    
    # Test ASCII letters
    assert_true(unicode_is_letter(0x41))  # 'A'
    assert_true(unicode_is_letter(0x61))  # 'a'
    assert_false(unicode_is_letter(0x30)) # '0'
    
    # Test digits
    assert_true(unicode_is_digit(0x30))   # '0'
    assert_true(unicode_is_digit(0x39))   # '9'
    assert_false(unicode_is_digit(0x41))  # 'A'
    
    # Test whitespace
    assert_true(unicode_is_whitespace(0x20))  # Space
    assert_true(unicode_is_whitespace(0x09))  # Tab
    assert_true(unicode_is_whitespace(0x0A))  # Line feed
    assert_false(unicode_is_whitespace(0x41)) # 'A'
    
    # Test punctuation
    assert_true(unicode_is_punctuation(0x21))  # '!'
    assert_true(unicode_is_punctuation(0x2E))  # '.'
    assert_false(unicode_is_punctuation(0x41)) # 'A'
    
    # Test symbols
    assert_true(unicode_is_symbol(0x24))   # '$'
    assert_true(unicode_is_symbol(0x2B))   # '+'
    assert_false(unicode_is_symbol(0x41))  # 'A'
    
    # Test extended Unicode ranges
    assert_true(unicode_is_letter(0x00C0))   # À (Latin Extended)
    assert_true(unicode_is_letter(0x03B1))   # α (Greek)
    assert_true(unicode_is_letter(0x0430))   # а (Cyrillic)
    
    damn cringe
}

slay test_case_conversion() cringe {
    vibez.spill("  Testing case conversion...")
    
    # Test ASCII case conversion
    assert_eq_int(unicode_to_upper(0x61), 0x41)  # 'a' -> 'A'
    assert_eq_int(unicode_to_lower(0x41), 0x61)  # 'A' -> 'a'
    
    # Test that non-letters are unchanged
    assert_eq_int(unicode_to_upper(0x30), 0x30)  # '0' unchanged
    assert_eq_int(unicode_to_lower(0x30), 0x30)  # '0' unchanged
    
    # Test extended Latin
    assert_eq_int(unicode_to_upper(0xE0), 0xC0)  # à -> À
    assert_eq_int(unicode_to_lower(0xC0), 0xE0)  # À -> à
    
    # Test Greek
    assert_eq_int(unicode_to_upper(0x03B1), 0x0391)  # α -> Α
    assert_eq_int(unicode_to_lower(0x0391), 0x03B1)  # Α -> α
    
    # Test Cyrillic
    assert_eq_int(unicode_to_upper(0x0430), 0x0410)  # а -> А
    assert_eq_int(unicode_to_lower(0x0410), 0x0430)  # А -> а
    
    damn cringe
}

slay test_utf8_operations() cringe {
    vibez.spill("  Testing UTF-8 encoding/decoding...")
    
    # Test sequence length detection
    assert_eq_int(utf8_sequence_length(0x41), 1)    # ASCII
    assert_eq_int(utf8_sequence_length(0xC2), 2)    # 2-byte
    assert_eq_int(utf8_sequence_length(0xE0), 3)    # 3-byte
    assert_eq_int(utf8_sequence_length(0xF0), 4)    # 4-byte
    
    # Test ASCII encoding
    sus ascii_encoded []normie = utf8_encode_codepoint(0x41)  # 'A'
    assert_eq_int(len(ascii_encoded), 1)
    assert_eq_int(ascii_encoded[0], 0x41)
    
    # Test 2-byte encoding (Latin-1 supplement)
    sus latin_encoded []normie = utf8_encode_codepoint(0xC0)  # À
    assert_eq_int(len(latin_encoded), 2)
    assert_eq_int(latin_encoded[0], 0xC3)
    assert_eq_int(latin_encoded[1], 0x80)
    
    # Test 3-byte encoding (Basic multilingual plane)
    sus bmp_encoded []normie = utf8_encode_codepoint(0x20AC)  # € (Euro sign)
    assert_eq_int(len(bmp_encoded), 3)
    assert_eq_int(bmp_encoded[0], 0xE2)
    assert_eq_int(bmp_encoded[1], 0x82)
    assert_eq_int(bmp_encoded[2], 0xAC)
    
    damn cringe
}

slay test_string_operations() cringe {
    vibez.spill("  Testing string operations...")
    
    # Test string creation from UTF-8 bytes
    sus hello_bytes []normie = [0x48, 0x65, 0x6C, 0x6C, 0x6F]  # "Hello"
    sus hello_str, err = unicode_string_create(hello_bytes)
    assert_true(err == cringe)
    assert_eq_int(unicode_string_length(hello_str), 5)
    assert_eq_int(unicode_string_byte_length(hello_str), 5)
    
    # Test character access
    sus char_h, char_err = unicode_string_at(hello_str, 0)
    assert_true(char_err == cringe)
    assert_eq_int(char_h, 0x48)  # 'H'
    
    sus char_o, char_err2 = unicode_string_at(hello_str, 4)
    assert_true(char_err2 == cringe)
    assert_eq_int(char_o, 0x6F)  # 'o'
    
    # Test out of bounds access
    sus invalid_char, invalid_err = unicode_string_at(hello_str, 10)
    assert_false(invalid_err == cringe)  # Should return error
    
    damn cringe
}

slay test_string_comparison() cringe {
    vibez.spill("  Testing string comparison...")
    
    sus str1_bytes []normie = [0x48, 0x65, 0x6C, 0x6C, 0x6F]  # "Hello"
    sus str2_bytes []normie = [0x48, 0x65, 0x6C, 0x6C, 0x6F]  # "Hello"
    sus str3_bytes []normie = [0x57, 0x6F, 0x72, 0x6C, 0x64]  # "World"
    
    sus str1, err1 = unicode_string_create(str1_bytes)
    sus str2, err2 = unicode_string_create(str2_bytes)
    sus str3, err3 = unicode_string_create(str3_bytes)
    
    assert_true(err1 == cringe && err2 == cringe && err3 == cringe)
    
    # Test equality
    assert_true(unicode_string_equals(str1, str2))
    assert_false(unicode_string_equals(str1, str3))
    
    # Test comparison
    assert_eq_int(unicode_string_compare(str1, str2), 0)
    assert_lt(unicode_string_compare(str1, str3), 0)  # "Hello" < "World"
    assert_gt(unicode_string_compare(str3, str1), 0)  # "World" > "Hello"
    
    damn cringe
}

slay test_string_manipulation() cringe {
    vibez.spill("  Testing string manipulation...")
    
    # Test string with whitespace
    sus whitespace_bytes []normie = [0x20, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20]  # " Hello "
    sus whitespace_str, ws_err = unicode_string_create(whitespace_bytes)
    assert_true(ws_err == cringe)
    
    # Test trimming
    sus trimmed_str, trim_err = unicode_string_trim(whitespace_str)
    assert_true(trim_err == cringe)
    assert_eq_int(unicode_string_length(trimmed_str), 5)  # Should be "Hello"
    
    # Test left trimming
    sus left_trimmed, left_err = unicode_string_trim_left(whitespace_str)
    assert_true(left_err == cringe)
    assert_eq_int(unicode_string_length(left_trimmed), 6)  # Should be "Hello "
    
    # Test right trimming
    sus right_trimmed, right_err = unicode_string_trim_right(whitespace_str)
    assert_true(right_err == cringe)
    assert_eq_int(unicode_string_length(right_trimmed), 6)  # Should be " Hello"
    
    # Test slicing
    sus hello_bytes []normie = [0x48, 0x65, 0x6C, 0x6C, 0x6F]  # "Hello"
    sus hello_str, hello_err = unicode_string_create(hello_bytes)
    assert_true(hello_err == cringe)
    
    sus slice_str, slice_err = unicode_string_slice(hello_str, 1, 4)
    assert_true(slice_err == cringe)
    assert_eq_int(unicode_string_length(slice_str), 3)  # Should be "ell"
    
    # Test starts_with and ends_with
    sus he_bytes []normie = [0x48, 0x65]  # "He"
    sus lo_bytes []normie = [0x6C, 0x6F]  # "lo"
    sus he_str, he_err = unicode_string_create(he_bytes)
    sus lo_str, lo_err = unicode_string_create(lo_bytes)
    
    assert_true(he_err == cringe && lo_err == cringe)
    assert_true(unicode_string_starts_with(hello_str, he_str))
    assert_true(unicode_string_ends_with(hello_str, lo_str))
    
    # Test contains and index_of
    sus ell_bytes []normie = [0x65, 0x6C, 0x6C]  # "ell"
    sus ell_str, ell_err = unicode_string_create(ell_bytes)
    assert_true(ell_err == cringe)
    
    assert_true(unicode_string_contains(hello_str, ell_str))
    assert_eq_int(unicode_string_index_of(hello_str, ell_str), 1)
    
    damn cringe
}

slay test_unicode_properties() cringe {
    vibez.spill("  Testing Unicode properties...")
    
    # Test general categories
    assert_eq_string(unicode_get_general_category(0x41), "Lu")   # 'A' - Uppercase letter
    assert_eq_string(unicode_get_general_category(0x61), "Ll")   # 'a' - Lowercase letter
    assert_eq_string(unicode_get_general_category(0x30), "Nd")   # '0' - Decimal number
    assert_eq_string(unicode_get_general_category(0x21), "Po")   # '!' - Punctuation
    
    # Test scripts
    assert_eq_string(unicode_get_script(0x41), "Latin")        # 'A'
    assert_eq_string(unicode_get_script(0x03B1), "Greek")      # 'α'
    assert_eq_string(unicode_get_script(0x0430), "Cyrillic")   # 'а'
    assert_eq_string(unicode_get_script(0x05D0), "Hebrew")     # 'א'
    assert_eq_string(unicode_get_script(0x0627), "Arabic")     # 'ا'
    
    # Test blocks
    assert_eq_string(unicode_get_block(0x41), "Basic Latin")
    assert_eq_string(unicode_get_block(0xC0), "Latin-1 Supplement")
    assert_eq_string(unicode_get_block(0x0391), "Greek and Coptic")
    assert_eq_string(unicode_get_block(0x0410), "Cyrillic")
    
    damn cringe
}

# Run all tests
run_unicode_tests()
