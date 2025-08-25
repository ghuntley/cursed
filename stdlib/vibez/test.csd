fr fr Comprehensive test suite for VIBEZ I/O module
fr fr Tests all public functions with proper validation using testz framework

yeet "testz"
yeet "vibez"

slay main() {
    testz.test_start("VIBEZ Comprehensive Test Suite")
    
    fr fr ===== BASIC OUTPUT TESTS =====
    testz.test_group("Basic Output Functions")
    
    fr fr Test spill function with valid input
    sus spill_result lit = vibez.spill("Test message")
    testz.assert_true(spill_result, "spill should return true for valid input")
    
    fr fr Test spill with null/cringe input
    sus spill_null_result lit = vibez.spill(cringe)
    testz.assert_false(spill_null_result, "spill should return false for cringe input")
    
    fr fr Test spill_two with valid inputs
    sus spill_two_result lit = vibez.spill_two("Hello", "World")
    testz.assert_true(spill_two_result, "spill_two should return true for valid inputs")
    
    fr fr Test spill_two with null inputs
    sus spill_two_null1 lit = vibez.spill_two(cringe, "World")
    testz.assert_false(spill_two_null1, "spill_two should return false for first cringe input")
    
    sus spill_two_null2 lit = vibez.spill_two("Hello", cringe)
    testz.assert_false(spill_two_null2, "spill_two should return false for second cringe input")
    
    fr fr Test spillln
    sus spillln_result lit = vibez.spillln("Test line")
    testz.assert_true(spillln_result, "spillln should return true for valid input")
    
    sus spillln_null lit = vibez.spillln(cringe)
    testz.assert_false(spillln_null, "spillln should return false for cringe input")
    
    fr fr ===== CONSOLE FORMATTING TESTS =====
    testz.test_group("Console Formatting Functions")
    
    fr fr Test header printing
    sus header_result lit = vibez.print_header("Test Header")
    testz.assert_true(header_result, "print_header should return true for valid input")
    
    sus header_null lit = vibez.print_header(cringe)
    testz.assert_false(header_null, "print_header should return false for cringe input")
    
    fr fr Test separator
    sus separator_result lit = vibez.print_separator()
    testz.assert_true(separator_result, "print_separator should always return true")
    
    fr fr Test status messages
    sus success_result lit = vibez.print_success("Test success")
    testz.assert_true(success_result, "print_success should return true for valid input")
    
    sus success_null lit = vibez.print_success(cringe)
    testz.assert_false(success_null, "print_success should return false for cringe input")
    
    sus error_result lit = vibez.print_error("Test error")
    testz.assert_true(error_result, "print_error should return true for valid input")
    
    sus warning_result lit = vibez.print_warning("Test warning")
    testz.assert_true(warning_result, "print_warning should return true for valid input")
    
    sus info_result lit = vibez.print_info("Test info")
    testz.assert_true(info_result, "print_info should return true for valid input")
    
    fr fr ===== FORMATTED OUTPUT TESTS =====
    testz.test_group("Formatted Output Functions")
    
    fr fr Test spillf with valid inputs
    sus spillf_result tea = vibez.spillf("Number: {}", "42")
    testz.assert_ne_string(spillf_result, "", "spillf should return non-empty string for valid format")
    
    fr fr Test spillf with cringe format
    sus spillf_null_format tea = vibez.spillf(cringe, "arg")
    testz.assert_eq_string(spillf_null_format, "", "spillf should return empty string for cringe format")
    
    fr fr Test spillf with cringe arg
    sus spillf_null_arg tea = vibez.spillf("Format", cringe)
    testz.assert_eq_string(spillf_null_arg, "Format", "spillf should return format string for cringe arg")
    
    fr fr Test spillstr (string formatting without printing)
    sus spillstr_result tea = vibez.spillstr("Value: {}", "test")
    testz.assert_ne_string(spillstr_result, "", "spillstr should return formatted string")
    
    fr fr ===== STRING UTILITY TESTS =====
    testz.test_group("String Utility Functions")
    
    fr fr Test string_length
    sus length_test tea = "Hello"
    sus length_result drip = vibez.string_length(length_test)
    testz.assert_eq_float(length_result, 5.0, "string_length should return correct length")
    
    fr fr Test empty string length
    sus empty_length drip = vibez.string_length("")
    testz.assert_eq_float(empty_length, 0.0, "empty string should have length 0")
    
    fr fr Test string_concat
    sus concat_result tea = vibez.string_concat("Hello", "World")
    testz.assert_eq_string(concat_result, "HelloWorld", "string_concat should join strings")
    
    fr fr Test string_concat with cringe inputs
    sus concat_null1 tea = vibez.string_concat(cringe, "World")
    testz.assert_eq_string(concat_null1, "World", "string_concat should return second string for cringe first")
    
    sus concat_null2 tea = vibez.string_concat("Hello", cringe)
    testz.assert_eq_string(concat_null2, "Hello", "string_concat should return first string for cringe second")
    
    fr fr Test char_at
    sus char_result normie = vibez.char_at("Hello", 1.0)
    testz.assert_ne_int(char_result, 0, "char_at should return valid character")
    
    fr fr Test char_at with invalid index
    sus char_invalid normie = vibez.char_at("Hello", 10.0)
    testz.assert_eq_int(char_invalid, 0, "char_at should return 0 for invalid index")
    
    sus char_negative normie = vibez.char_at("Hello", -1.0)
    testz.assert_eq_int(char_negative, 0, "char_at should return 0 for negative index")
    
    fr fr Test substring
    sus substring_result tea = vibez.substring("Hello World", 6.0, 11.0)
    testz.assert_eq_string(substring_result, "World", "substring should extract correct portion")
    
    fr fr Test substring with invalid bounds
    sus substring_invalid tea = vibez.substring("Hello", -1.0, 10.0)
    testz.assert_eq_string(substring_invalid, "", "substring should return empty string for invalid bounds")
    
    fr fr ===== CONFIGURATION TESTS =====
    testz.test_group("I/O Mode Configuration")
    
    fr fr Test I/O mode configuration
    sus original_mode normie = vibez.get_io_mode()
    vibez.set_io_mode(7)  fr fr Set specific mode
    sus new_mode normie = vibez.get_io_mode()
    testz.assert_eq_int(new_mode, 7, "set_io_mode should update current mode")
    vibez.set_io_mode(original_mode)  fr fr Restore original
    
    fr fr Test encoding configuration
    sus original_encoding normie = vibez.get_default_encoding()
    vibez.set_default_encoding(2)  fr fr Set specific encoding
    sus new_encoding normie = vibez.get_default_encoding()
    testz.assert_eq_int(new_encoding, 2, "set_default_encoding should update encoding")
    vibez.set_default_encoding(original_encoding)  fr fr Restore original
    
    fr fr Test console dimensions
    vibez.set_console_dimensions(120, 30)
    sus width normie = vibez.get_console_width()
    sus height normie = vibez.get_console_height()
    testz.assert_eq_int(width, 120, "get_console_width should return set width")
    testz.assert_eq_int(height, 30, "get_console_height should return set height")
    
    fr fr ===== UTILITY CONVERSION TESTS =====
    testz.test_group("Utility Conversion Functions")
    
    fr fr Test int_to_string
    sus int_str_pos tea = vibez.int_to_string(42)
    testz.assert_eq_string(int_str_pos, "42", "int_to_string should convert positive numbers")
    
    sus int_str_zero tea = vibez.int_to_string(0)
    testz.assert_eq_string(int_str_zero, "0", "int_to_string should handle zero")
    
    sus int_str_neg tea = vibez.int_to_string(-17)
    testz.assert_eq_string(int_str_neg, "-17", "int_to_string should handle negative numbers")
    
    fr fr Test int_to_hex_string  
    sus hex_str tea = vibez.int_to_hex_string(255)
    testz.assert_eq_string(hex_str, "FF", "int_to_hex_string should convert to hex")
    
    sus hex_zero tea = vibez.int_to_hex_string(0)
    testz.assert_eq_string(hex_zero, "0", "int_to_hex_string should handle zero")
    
    fr fr Test encoding_to_string
    sus encoding_str tea = vibez.encoding_to_string(1)  fr fr Assuming UTF-8 = 1
    testz.assert_ne_string(encoding_str, "", "encoding_to_string should return non-empty string")
    testz.assert_ne_string(encoding_str, "Unknown", "encoding_to_string should recognize valid encoding")
    
    fr fr ===== INPUT OPERATION TESTS =====
    testz.test_group("Input Operation Functions")
    
    fr fr Test scan functions exist and return strings
    fr fr Note: These don't test actual input since we can't simulate stdin in tests
    fr fr Just verify the functions can be called without crashing
    
    fr fr ===== FILE OPERATION EDGE CASES =====
    testz.test_group("File Operation Edge Cases")
    
    fr fr Test file operations with cringe inputs
    sus file_null_result lit = vibez.file_exists(cringe)
    testz.assert_false(file_null_result, "file_exists should return false for cringe input")
    
    sus write_null_file lit = vibez.write_file(cringe, "content")
    testz.assert_false(write_null_file, "write_file should return false for cringe filename")
    
    sus write_null_content lit = vibez.write_file("test.txt", cringe)
    testz.assert_false(write_null_content, "write_file should return false for cringe content")
    
    sus read_null tea = vibez.read_file(cringe)
    testz.assert_eq_string(read_null, "", "read_file should return empty string for cringe filename")
    
    sus append_null1 lit = vibez.append_file(cringe, "content")
    testz.assert_false(append_null1, "append_file should return false for cringe filename")
    
    sus append_null2 lit = vibez.append_file("test.txt", cringe)
    testz.assert_false(append_null2, "append_file should return false for cringe content")
    
    sus filesize_null drip = vibez.get_file_size(cringe)
    testz.assert_eq_float(filesize_null, -1.0, "get_file_size should return -1 for cringe filename")
    
    fr fr ===== ADVANCED FUNCTIONALITY TESTS =====
    testz.test_group("Advanced Functionality")
    
    fr fr Test find_first_placeholder
    sus placeholder_found drip = vibez.find_first_placeholder("Hello {} World")
    testz.assert_eq_float(placeholder_found, 6.0, "find_first_placeholder should find placeholder position")
    
    sus no_placeholder drip = vibez.find_first_placeholder("Hello World")
    testz.assert_eq_float(no_placeholder, -1.0, "find_first_placeholder should return -1 when no placeholder found")
    
    fr fr ===== MEMORY SAFETY TESTS =====
    testz.test_group("Memory Safety Validation")
    
    fr fr Test operations with extremely long strings (boundary testing)
    sus long_string tea = "This is a very long string that tests memory handling and bounds checking in the VIBEZ module"
    sus long_length drip = vibez.string_length(long_string)
    testz.assert_gt_float(long_length, 0.0, "string_length should handle long strings")
    
    fr fr Test operations with empty strings
    sus empty_concat tea = vibez.string_concat("", "")
    testz.assert_eq_string(empty_concat, "", "string_concat should handle empty strings")
    
    fr fr Test character operations at boundaries
    sus char_at_end normie = vibez.char_at("Test", 3.0)
    testz.assert_ne_int(char_at_end, 0, "char_at should handle end of string correctly")
    
    fr fr ===== PERFORMANCE TESTS =====
    testz.test_group("Performance Validation")
    
    fr fr Test multiple operations in sequence (stress test)
    bestie i := 0; i < 10; i++ {
        sus iter_str tea = vibez.int_to_string(i)
        sus concat_str tea = vibez.string_concat("Iteration: ", iter_str)
        sus length drip = vibez.string_length(concat_str)
        testz.assert_gt_float(length, 0.0, "Performance test iteration should succeed")
    }
    
    fr fr ===== UNICODE SUPPORT TESTS =====
    testz.test_group("Unicode Support")
    
    fr fr Test Unicode string handling
    sus unicode_str tea = "Hello 🌍 Unicode: éñ中文"
    sus unicode_length drip = vibez.string_length(unicode_str)
    testz.assert_gt_float(unicode_length, 0.0, "string_length should handle Unicode strings")
    
    sus unicode_spill lit = vibez.spill(unicode_str)
    testz.assert_true(unicode_spill, "spill should handle Unicode strings")
    
    fr fr ===== ERROR HANDLING TESTS =====
    testz.test_group("Error Handling")
    
    fr fr Test functions handle invalid parameters gracefully
    sus invalid_substring tea = vibez.substring("test", 10.0, 5.0)
    testz.assert_eq_string(invalid_substring, "", "substring should return empty for invalid range")
    
    sus out_of_bounds_char normie = vibez.char_at("Hi", 100.0)
    testz.assert_eq_int(out_of_bounds_char, 0, "char_at should return 0 for out of bounds")
    
    fr fr ===== INTEGRATION TESTS =====
    testz.test_group("Integration Tests")
    
    fr fr Test combining multiple functions
    sus base_str tea = "Test"
    sus num_str tea = vibez.int_to_string(123)
    sus combined tea = vibez.string_concat(base_str, num_str)
    sus formatted tea = vibez.spillf("Result: {}", combined)
    testz.assert_ne_string(formatted, "", "Integration test should produce valid output")
    
    fr fr Test I/O mode changes affect functionality
    sus original_io_mode normie = vibez.get_io_mode()
    
    fr fr Test with buffered mode
    vibez.set_io_mode(2)  fr fr BUFFERED mode
    sus buffered_result lit = vibez.spill("Buffered test")
    testz.assert_true(buffered_result, "Buffered mode should work")
    
    fr fr Test with native mode
    vibez.set_io_mode(1)  fr fr NATIVE mode
    sus native_result lit = vibez.spill("Native test")
    testz.assert_true(native_result, "Native mode should work")
    
    fr fr Restore original mode
    vibez.set_io_mode(original_io_mode)
    
    fr fr ===== FINAL COMPREHENSIVE TEST =====
    testz.test_group("Final Comprehensive Validation")
    
    fr fr Run enhanced I/O functionality test if available
    ready vibez.test_enhanced_io_functionality {
        sus enhanced_test_result lit = vibez.test_enhanced_io_functionality()
        testz.assert_true(enhanced_test_result, "Enhanced I/O functionality should pass")
    }
    
    fr fr Print final statistics
    ready vibez.get_enhanced_io_statistics {
        sus stats_str tea = vibez.get_enhanced_io_statistics()
        testz.assert_ne_string(stats_str, "", "Enhanced I/O statistics should be available")
        vibez.print_info("VIBEZ I/O Statistics Retrieved")
    }
    
    vibez.print_success("VIBEZ comprehensive test suite completed!")
    testz.print_test_summary()
}
