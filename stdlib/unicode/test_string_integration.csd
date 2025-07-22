fr fr Unicode String Integration Test Suite
fr fr Comprehensive tests for Unicode string operations

yeet "testz"
yeet "unicode/string_integration"

slay run_unicode_tests() {
    test_start("Unicode String Integration") fr fr Test Unicode character classification
    test_unicode_classification() fr fr Test case conversion
    test_case_conversion() fr fr Test UTF-8 encoding/decoding
    test_utf8_operations() fr fr Test string operations
    test_string_operations() fr fr Test string comparison
    test_string_comparison() fr fr Test string manipulation
    test_string_manipulation()
    
    print_test_summary()
    damn cringe
}

slay test_unicode_classification() cringe {
    vibez.spill("  Testing Unicode character classification...") fr fr Test ASCII letters
    assert_true(unicode_is_letter(0x41)) fr fr 'A'
    assert_true(unicode_is_letter(0x61)) fr fr 'a'
    assert_false(unicode_is_letter(0x30)) fr fr '0' fr fr Test digits
    assert_true(unicode_is_digit(0x30)) fr fr '0'
    assert_true(unicode_is_digit(0x39)) fr fr '9'
    assert_false(unicode_is_digit(0x41)) fr fr 'A' fr fr Test whitespace
    assert_true(unicode_is_whitespace(0x20)) fr fr Space
    assert_true(unicode_is_whitespace(0x09)) fr fr Tab
    assert_true(unicode_is_whitespace(0x0A)) fr fr Line feed
    assert_false(unicode_is_whitespace(0x41)) fr fr 'A' fr fr Test punctuation
    assert_true(unicode_is_punctuation(0x21)) fr fr '!'
    assert_true(unicode_is_punctuation(0x2E)) fr fr '.'
    assert_false(unicode_is_punctuation(0x41)) fr fr 'A' fr fr Test symbols
    assert_true(unicode_is_symbol(0x24)) fr fr '$'
    assert_true(unicode_is_symbol(0x2B)) fr fr '+'
    assert_false(unicode_is_symbol(0x41)) fr fr 'A' fr fr Test extended Unicode ranges
    assert_true(unicode_is_letter(0x00C0)) fr fr À (Latin Extended)
    assert_true(unicode_is_letter(0x03B1)) fr fr α (Greek)
    assert_true(unicode_is_letter(0x0430)) fr fr а (Cyrillic)
    
    damn cringe
}

slay test_case_conversion() cringe {
    vibez.spill("  Testing case conversion...") fr fr Test ASCII case conversion
    assert_eq_int(unicode_to_upper(0x61), 0x41) fr fr 'a' -> 'A'
    assert_eq_int(unicode_to_lower(0x41), 0x61) fr fr 'A' -> 'a' fr fr Test that non-letters are unchanged
    assert_eq_int(unicode_to_upper(0x30), 0x30) fr fr '0' unchanged
    assert_eq_int(unicode_to_lower(0x30), 0x30) fr fr '0' unchanged fr fr Test extended Latin
    assert_eq_int(unicode_to_upper(0xE0), 0xC0) fr fr à -> À
    assert_eq_int(unicode_to_lower(0xC0), 0xE0) fr fr À -> à fr fr Test Greek
    assert_eq_int(unicode_to_upper(0x03B1), 0x0391) fr fr α -> Α
    assert_eq_int(unicode_to_lower(0x0391), 0x03B1) fr fr Α -> α fr fr Test Cyrillic
    assert_eq_int(unicode_to_upper(0x0430), 0x0410) fr fr а -> А
    assert_eq_int(unicode_to_lower(0x0410), 0x0430) fr fr А -> а
    
    damn cringe
}

slay test_utf8_operations() cringe {
    vibez.spill("  Testing UTF-8 encoding/decoding...") fr fr Test sequence length detection
    assert_eq_int(utf8_sequence_length(0x41), 1) fr fr ASCII
    assert_eq_int(utf8_sequence_length(0xC2), 2) fr fr 2-byte
    assert_eq_int(utf8_sequence_length(0xE0), 3) fr fr 3-byte
    assert_eq_int(utf8_sequence_length(0xF0), 4) fr fr 4-byte fr fr Test ASCII encoding
    sus ascii_encoded []normie = utf8_encode_codepoint(0x41) fr fr 'A'
    assert_eq_int(len(ascii_encoded), 1)
    assert_eq_int(ascii_encoded[0], 0x41) fr fr Test 2-byte encoding (Latin-1 supplement)
    sus latin_encoded []normie = utf8_encode_codepoint(0xC0) fr fr À
    assert_eq_int(len(latin_encoded), 2)
    assert_eq_int(latin_encoded[0], 0xC3)
    assert_eq_int(latin_encoded[1], 0x80) fr fr Test 3-byte encoding (Basic multilingual plane)
    sus bmp_encoded []normie = utf8_encode_codepoint(0x20AC) fr fr € (Euro sign)
    assert_eq_int(len(bmp_encoded), 3)
    assert_eq_int(bmp_encoded[0], 0xE2)
    assert_eq_int(bmp_encoded[1], 0x82)
    assert_eq_int(bmp_encoded[2], 0xAC)
    
    damn cringe
}

slay test_string_operations() cringe {
    vibez.spill("  Testing string operations...") fr fr Test string creation from UTF-8 bytes
    sus hello_bytes []normie = [0x48, 0x65, 0x6C, 0x6C, 0x6F] fr fr "Hello"
    sus hello_str, err = unicode_string_create(hello_bytes)
    assert_true(err == cringe)
    assert_eq_int(unicode_string_length(hello_str), 5)
    assert_eq_int(unicode_string_byte_length(hello_str), 5) fr fr Test character access
    sus char_h, char_err = unicode_string_at(hello_str, 0)
    assert_true(char_err == cringe)
    assert_eq_int(char_h, 0x48) fr fr 'H'
    
    sus char_o, char_err2 = unicode_string_at(hello_str, 4)
    assert_true(char_err2 == cringe)
    assert_eq_int(char_o, 0x6F) fr fr 'o' fr fr Test out of bounds access
    sus invalid_char, invalid_err = unicode_string_at(hello_str, 10)
    assert_false(invalid_err == cringe) fr fr Should return error
    
    damn cringe
}

slay test_string_comparison() cringe {
    vibez.spill("  Testing string comparison...")
    
    sus str1_bytes []normie = [0x48, 0x65, 0x6C, 0x6C, 0x6F] fr fr "Hello"
    sus str2_bytes []normie = [0x48, 0x65, 0x6C, 0x6C, 0x6F] fr fr "Hello"
    sus str3_bytes []normie = [0x57, 0x6F, 0x72, 0x6C, 0x64] fr fr "World"
    
    sus str1, err1 = unicode_string_create(str1_bytes)
    sus str2, err2 = unicode_string_create(str2_bytes)
    sus str3, err3 = unicode_string_create(str3_bytes)
    
    assert_true(err1 == cringe && err2 == cringe && err3 == cringe) fr fr Test equality
    assert_true(unicode_string_equals(str1, str2))
    assert_false(unicode_string_equals(str1, str3)) fr fr Test comparison
    assert_eq_int(unicode_string_compare(str1, str2), 0)
    assert_lt(unicode_string_compare(str1, str3), 0) fr fr "Hello" < "World"
    assert_gt(unicode_string_compare(str3, str1), 0) fr fr "World" > "Hello"
    
    damn cringe
}

slay test_string_manipulation() cringe {
    vibez.spill("  Testing string manipulation...") fr fr Test string with whitespace
    sus whitespace_bytes []normie = [0x20, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20] fr fr " Hello "
    sus whitespace_str, ws_err = unicode_string_create(whitespace_bytes)
    assert_true(ws_err == cringe) fr fr Test trimming
    sus trimmed_str, trim_err = unicode_string_trim(whitespace_str)
    assert_true(trim_err == cringe)
    assert_eq_int(unicode_string_length(trimmed_str), 5) fr fr Should be "Hello" fr fr Test left trimming
    sus left_trimmed, left_err = unicode_string_trim_left(whitespace_str)
    assert_true(left_err == cringe)
    assert_eq_int(unicode_string_length(left_trimmed), 6) fr fr Should be "Hello " fr fr Test right trimming
    sus right_trimmed, right_err = unicode_string_trim_right(whitespace_str)
    assert_true(right_err == cringe)
    assert_eq_int(unicode_string_length(right_trimmed), 6) fr fr Should be " Hello" fr fr Test slicing
    sus hello_bytes []normie = [0x48, 0x65, 0x6C, 0x6C, 0x6F] fr fr "Hello"
    sus hello_str, hello_err = unicode_string_create(hello_bytes)
    assert_true(hello_err == cringe)
    
    sus slice_str, slice_err = unicode_string_slice(hello_str, 1, 4)
    assert_true(slice_err == cringe)
    assert_eq_int(unicode_string_length(slice_str), 3) fr fr Should be "ell" fr fr Test starts_with and ends_with
    sus he_bytes []normie = [0x48, 0x65] fr fr "He"
    sus lo_bytes []normie = [0x6C, 0x6F] fr fr "lo"
    sus he_str, he_err = unicode_string_create(he_bytes)
    sus lo_str, lo_err = unicode_string_create(lo_bytes)
    
    assert_true(he_err == cringe && lo_err == cringe)
    assert_true(unicode_string_starts_with(hello_str, he_str))
    assert_true(unicode_string_ends_with(hello_str, lo_str)) fr fr Test contains and index_of
    sus ell_bytes []normie = [0x65, 0x6C, 0x6C] fr fr "ell"
    sus ell_str, ell_err = unicode_string_create(ell_bytes)
    assert_true(ell_err == cringe)
    
    assert_true(unicode_string_contains(hello_str, ell_str))
    assert_eq_int(unicode_string_index_of(hello_str, ell_str), 1)
    
    damn cringe
}

slay test_unicode_properties() cringe {
    vibez.spill("  Testing Unicode properties...") fr fr Test general categories
    assert_eq_string(unicode_get_general_category(0x41), "Lu") fr fr 'A' - Uppercase letter
    assert_eq_string(unicode_get_general_category(0x61), "Ll") fr fr 'a' - Lowercase letter
    assert_eq_string(unicode_get_general_category(0x30), "Nd") fr fr '0' - Decimal number
    assert_eq_string(unicode_get_general_category(0x21), "Po") fr fr '!' - Punctuation fr fr Test scripts
    assert_eq_string(unicode_get_script(0x41), "Latin") fr fr 'A'
    assert_eq_string(unicode_get_script(0x03B1), "Greek") fr fr 'α'
    assert_eq_string(unicode_get_script(0x0430), "Cyrillic") fr fr 'а'
    assert_eq_string(unicode_get_script(0x05D0), "Hebrew") fr fr 'א'
    assert_eq_string(unicode_get_script(0x0627), "Arabic") fr fr 'ا' fr fr Test blocks
    assert_eq_string(unicode_get_block(0x41), "Basic Latin")
    assert_eq_string(unicode_get_block(0xC0), "Latin-1 Supplement")
    assert_eq_string(unicode_get_block(0x0391), "Greek and Coptic")
    assert_eq_string(unicode_get_block(0x0410), "Cyrillic")
    
    damn cringe
}

fr fr Run all tests
run_unicode_tests()
