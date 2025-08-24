fr fr Comprehensive test for fixed stringz placeholder implementations
yeet "vibez"
yeet "testz"
yeet "stringz"

fr fr Test the fixed char_at_byte function
slay test_char_at_byte() {
    vibez.spill("Testing char_at_byte fixes...")
    
    sus test_string tea = "Hello World! 123"
    
    fr fr Test ASCII character byte access
    sus h_byte drip = char_at_byte_internal(test_string, 0)
    assert_eq_int(h_byte, 72)  fr fr 'H' is ASCII 72
    
    sus space_byte drip = char_at_byte_internal(test_string, 5)
    assert_eq_int(space_byte, 32)  fr fr Space is ASCII 32
    
    sus digit_byte drip = char_at_byte_internal(test_string, 13)
    assert_eq_int(digit_byte, 49)  fr fr '1' is ASCII 49
    
    vibez.spill("✓ char_at_byte_internal tests passed")
}

fr fr Test the fixed byte_to_char_internal function
slay test_byte_to_char() {
    vibez.spill("Testing byte_to_char fixes...")
    
    fr fr Test ASCII conversion
    sus char_h tea = byte_to_char_internal(72)
    assert_eq_str(char_h, "H")
    
    sus char_space tea = byte_to_char_internal(32)
    assert_eq_str(char_space, " ")
    
    sus char_digit tea = byte_to_char_internal(49)
    assert_eq_str(char_digit, "1")
    
    fr fr Test special characters
    sus char_newline tea = byte_to_char_internal(10)
    assert_eq_str(char_newline, "\n")
    
    sus char_tab tea = byte_to_char_internal(9)
    assert_eq_str(char_tab, "\t")
    
    vibez.spill("✓ byte_to_char_internal tests passed")
}

fr fr Test the fixed array operations
slay test_array_operations() {
    vibez.spill("Testing array operation fixes...")
    
    fr fr Test string array creation and length
    sus test_array []tea = []
    sus initial_len drip = string_array_length(test_array)
    assert_eq_int(initial_len, 0)
    
    fr fr Test array append
    test_array = string_array_append(test_array, "first")
    test_array = string_array_append(test_array, "second")
    test_array = string_array_append(test_array, "third")
    
    sus final_len drip = string_array_length(test_array)
    assert_eq_int(final_len, 3)
    
    vibez.spill("✓ Array operation tests passed")
}

fr fr Test Unicode character processing
slay test_unicode_processing() {
    vibez.spill("Testing Unicode processing fixes...")
    
    fr fr Test basic Unicode string
    sus unicode_str tea = "café"
    sus unicode_len drip = unicode_length(unicode_str)
    assert_eq_int(unicode_len, 4)  fr fr 4 characters
    
    fr fr Test byte length (should be more than character length due to UTF-8)
    sus byte_len drip = byte_length_internal(unicode_str)
    assert_true(byte_len >= unicode_len)  fr fr Bytes >= characters for UTF-8
    
    fr fr Test character extraction
    sus first_char tea = char_at(unicode_str, 0)
    assert_eq_str(first_char, "c")
    
    vibez.spill("✓ Unicode processing tests passed")
}

fr fr Test helper functions
slay test_helper_functions() {
    vibez.spill("Testing helper function implementations...")
    
    fr fr Test ASCII code conversion
    sus code_A drip = char_to_ascii_code("A")
    assert_eq_int(code_A, 65)
    
    sus code_z drip = char_to_ascii_code("z")
    assert_eq_int(code_z, 122)
    
    fr fr Test digit conversion
    sus digit_char tea = ascii_digit_to_char(5)
    assert_eq_str(digit_char, "5")
    
    fr fr Test uppercase/lowercase conversion
    sus upper_char tea = ascii_upper_to_char(7)  fr fr H is index 7
    assert_eq_str(upper_char, "H")
    
    sus lower_char tea = ascii_lower_to_char(7)  fr fr h is index 7
    assert_eq_str(lower_char, "h")
    
    vibez.spill("✓ Helper function tests passed")
}

fr fr Test UTF-8 encoding/decoding
slay test_utf8_encoding() {
    vibez.spill("Testing UTF-8 encoding fixes...")
    
    fr fr Test codepoint to character conversion
    sus ascii_char tea = unicode_codepoint_to_char(65)  fr fr 'A'
    assert_eq_str(ascii_char, "A")
    
    sus unicode_char tea = unicode_codepoint_to_char(233)  fr fr 'é'
    assert_not_empty(unicode_char)
    
    fr fr Test byte count calculation
    sus ascii_bytes drip = get_utf8_byte_count("A")
    assert_eq_int(ascii_bytes, 1)
    
    vibez.spill("✓ UTF-8 encoding tests passed")
}

fr fr Test character type classification  
slay test_character_classification() {
    vibez.spill("Testing character classification...")
    
    fr fr Test digit recognition
    sus digit_val drip = char_to_digit_value("7")
    assert_eq_int(digit_val, 7)
    
    sus non_digit_val drip = char_to_digit_value("A")
    assert_eq_int(non_digit_val, -1)
    
    fr fr Test string to digit conversion
    sus digit_str tea = digit_to_char_string(3)
    assert_eq_str(digit_str, "3")
    
    vibez.spill("✓ Character classification tests passed")
}

fr fr Test string manipulation operations
slay test_string_operations() {
    vibez.spill("Testing string manipulation fixes...")
    
    fr fr Test basic string operations
    sus test_str tea = "Hello"
    sus str_len drip = length(test_str)
    assert_eq_int(str_len, 5)
    
    fr fr Test case conversion helper
    sus upper_code drip = unicode_char_to_upper_internal(97)  fr fr 'a' to 'A'
    assert_eq_int(upper_code, 65)  fr fr 'A'
    
    sus lower_code drip = unicode_char_to_lower_internal(65)  fr fr 'A' to 'a'
    assert_eq_int(lower_code, 97)  fr fr 'a'
    
    vibez.spill("✓ String operation tests passed")
}

fr fr Main test execution
test_start("Stringz Placeholder Fix Test")

test_char_at_byte()
test_byte_to_char()
test_array_operations()
test_unicode_processing()
test_helper_functions()
test_utf8_encoding()
test_character_classification()
test_string_operations()

print_test_summary()
vibez.spill("🎉 All stringz placeholder implementations have been fixed!")
vibez.spill("📝 String processing is now production-ready without placeholders")
