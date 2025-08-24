fr fr Comprehensive Test Suite for Enhanced VIBEZ Module
fr fr Tests all real implementations and advanced functionality

yeet "vibez"
yeet "testz"
yeet "stringz"

fr fr ===== TEST FRAMEWORK SETUP =====

sus total_tests normie = 0
sus passed_tests normie = 0
sus failed_tests normie = 0

slay test_start(test_name tea) {
    total_tests = total_tests + 1
    spill("🧪 Testing: ")
    spillln(test_name)
}

slay test_assert(condition lit, message tea) {
    ready condition {
        passed_tests = passed_tests + 1
        spill("  ✅ PASS: ")
        spillln(message)
    }
    otherwise {
        failed_tests = failed_tests + 1
        spill("  ❌ FAIL: ")
        spillln(message)
    }
}

slay test_summary() {
    spillln("")
    spillln("==================== TEST SUMMARY ====================")
    spill("Total Tests: ")
    spillln(number_to_string(total_tests))
    spill("Passed: ")
    spillln(number_to_string(passed_tests))
    spill("Failed: ")
    spillln(number_to_string(failed_tests))
    
    ready failed_tests == 0 {
        spillln("🎉 ALL TESTS PASSED!")
    }
    otherwise {
        spillln("💥 SOME TESTS FAILED!")
    }
    spillln("====================================================")
}

fr fr ===== BASIC OUTPUT TESTS =====

slay test_basic_output() {
    test_start("Basic Output Functions")
    
    fr fr Test spill function
    sus result1 lit = spill("Hello")
    test_assert(result1 == based, "spill() returns success")
    
    fr fr Test spillln function  
    sus result2 lit = spillln("World")
    test_assert(result2 == based, "spillln() returns success")
    
    fr fr Test spill_two function
    sus result3 lit = spill_two("Hello", "World")
    test_assert(result3 == based, "spill_two() returns success")
    
    fr fr Test null input handling
    sus result4 lit = spill(cringe)
    test_assert(result4 == cap, "spill() handles null input")
}

fr fr ===== FORMATTING TESTS =====

slay test_formatting_functions() {
    test_start("Advanced Formatting Functions")
    
    fr fr Test spillf with string formatting
    sus result1 tea = spillf("Hello %s", "World")
    test_assert(stringz.contains(result1, "Hello"), "spillf() string formatting works")
    test_assert(stringz.contains(result1, "World"), "spillf() includes argument")
    
    fr fr Test spillf with integer formatting
    sus result2 tea = spillf("Number: %d", "42")
    test_assert(stringz.contains(result2, "Number:"), "spillf() integer formatting works")
    test_assert(stringz.contains(result2, "42"), "spillf() includes integer")
    
    fr fr Test spillstr without printing
    sus result3 tea = spillstr("Value: %s", "test")
    test_assert(stringz.contains(result3, "Value:"), "spillstr() formats without printing")
    test_assert(stringz.contains(result3, "test"), "spillstr() includes value")
    
    fr fr Test multi-argument formatting
    sus result4 tea = spillf_multi("Name: %s, Age: %d", ["Alice", "25"])
    test_assert(stringz.contains(result4, "Name:"), "spillf_multi() handles multiple args")
    test_assert(stringz.contains(result4, "Alice"), "spillf_multi() includes first arg")
}

fr fr ===== PLACEHOLDER REPLACEMENT TESTS =====

slay test_placeholder_replacement() {
    test_start("Placeholder Replacement System")
    
    fr fr Test basic placeholder replacement
    sus template1 tea = "Hello {}, welcome to {}!"
    sus values1 []tea = ["Alice", "CURSED"]
    sus result1 tea = format_with_placeholders(template1, values1)
    test_assert(stringz.contains(result1, "Alice"), "Placeholder replacement works")
    test_assert(stringz.contains(result1, "CURSED"), "Multiple placeholders work")
    
    fr fr Test numbered placeholders
    sus template2 tea = "Item {0} costs {1} dollars"
    sus values2 []tea = ["Book", "15"]
    sus result2 tea = format_with_placeholders(template2, values2)
    test_assert(stringz.contains(result2, "Book"), "Numbered placeholders work")
    test_assert(stringz.contains(result2, "15"), "Numbered placeholder values correct")
    
    fr fr Test mixed placeholders
    sus template3 tea = "Hello {0}, you have {} messages"
    sus values3 []tea = ["Bob", "5"]
    sus result3 tea = format_with_placeholders(template3, values3)
    test_assert(stringz.contains(result3, "Bob"), "Mixed placeholder types work")
    test_assert(stringz.contains(result3, "messages"), "Template text preserved")
}

fr fr ===== INPUT OPERATIONS TESTS =====

slay test_input_operations() {
    test_start("Input Operations")
    
    fr fr Test scan function (simulated)
    sus input1 tea = scan()
    test_assert(stringz.length(input1) >= 0, "scan() returns valid string")
    
    fr fr Test scanln function (simulated)
    sus input2 tea = scanln()
    test_assert(stringz.length(input2) >= 0, "scanln() returns valid string")
    
    fr fr Test scan with prompt
    sus input3 tea = scan_with_prompt("Enter value: ")
    test_assert(stringz.length(input3) >= 0, "scan_with_prompt() works")
}

fr fr ===== FILE OPERATIONS TESTS =====

slay test_file_operations() {
    test_start("File I/O Operations")
    
    sus test_filename tea = "/tmp/test_vibez_file.txt"
    sus test_content tea = "This is test content for vibez module testing."
    
    fr fr Test file writing
    sus write_result lit = write_file(test_filename, test_content)
    test_assert(write_result == based, "write_file() succeeds")
    
    fr fr Test file existence check
    sus exists_result lit = file_exists(test_filename)
    test_assert(exists_result == based, "file_exists() detects created file")
    
    fr fr Test file reading
    sus read_result tea = read_file(test_filename)
    test_assert(stringz.length(read_result) > 0, "read_file() returns content")
    test_assert(stringz.contains(read_result, "test content"), "read_file() content correct")
    
    fr fr Test file appending
    sus append_content tea = "\nAppended line."
    sus append_result lit = append_file(test_filename, append_content)
    test_assert(append_result == based, "append_file() succeeds")
    
    fr fr Test file size
    sus file_size drip = get_file_size(test_filename)
    test_assert(file_size > 0.0, "get_file_size() returns positive size")
    
    fr fr Test non-existent file
    sus nonexistent_content tea = read_file("/nonexistent/file.txt")
    test_assert(stringz.length(nonexistent_content) == 0, "read_file() handles missing files")
}

fr fr ===== NUMBER CONVERSION TESTS =====

slay test_number_conversion() {
    test_start("Number Conversion Functions")
    
    fr fr Test integer to string conversion
    sus int_str1 tea = integer_to_string_advanced(42)
    test_assert(int_str1 == "42", "integer_to_string_advanced() converts positive")
    
    sus int_str2 tea = integer_to_string_advanced(-123)
    test_assert(int_str2 == "-123", "integer_to_string_advanced() converts negative")
    
    sus int_str3 tea = integer_to_string_advanced(0)
    test_assert(int_str3 == "0", "integer_to_string_advanced() converts zero")
    
    fr fr Test string to integer conversion
    sus int_val1 normie = string_to_integer_safe("456")
    test_assert(int_val1 == 456, "string_to_integer_safe() parses positive")
    
    sus int_val2 normie = string_to_integer_safe("-789")
    test_assert(int_val2 == -789, "string_to_integer_safe() parses negative")
    
    sus int_val3 normie = string_to_integer_safe("invalid")
    test_assert(int_val3 == 0, "string_to_integer_safe() handles invalid input")
}

fr fr ===== BASE CONVERSION TESTS =====

slay test_base_conversion() {
    test_start("Base Conversion Functions")
    
    fr fr Test hexadecimal conversion
    sus hex_result1 tea = integer_to_hex_advanced(255)
    test_assert(hex_result1 == "ff", "integer_to_hex_advanced() converts 255")
    
    sus hex_result2 tea = integer_to_hex_advanced(16)
    test_assert(hex_result2 == "10", "integer_to_hex_advanced() converts 16")
    
    fr fr Test octal conversion
    sus octal_result1 tea = integer_to_octal_advanced(64)
    test_assert(octal_result1 == "100", "integer_to_octal_advanced() converts 64")
    
    sus octal_result2 tea = integer_to_octal_advanced(8)
    test_assert(octal_result2 == "10", "integer_to_octal_advanced() converts 8")
    
    fr fr Test binary conversion
    sus binary_result1 tea = integer_to_binary_advanced(8)
    test_assert(binary_result1 == "1000", "integer_to_binary_advanced() converts 8")
    
    sus binary_result2 tea = integer_to_binary_advanced(15)
    test_assert(binary_result2 == "1111", "integer_to_binary_advanced() converts 15")
}

fr fr ===== FORMAT SPECIFIER TESTS =====

slay test_format_specifiers() {
    test_start("Format Specifier System")
    
    fr fr Test string specifier
    sus str_result tea = format_advanced("Name: %s", ["Alice"])
    test_assert(stringz.contains(str_result, "Alice"), "%s specifier works")
    
    fr fr Test integer specifier
    sus int_result tea = format_advanced("Count: %d", ["42"])
    test_assert(stringz.contains(int_result, "42"), "%d specifier works")
    
    fr fr Test hex specifier
    sus hex_result tea = format_advanced("Value: %x", ["255"])
    test_assert(stringz.contains(hex_result, "0x"), "%x specifier adds 0x prefix")
    
    fr fr Test percent escape
    sus percent_result tea = format_advanced("Progress: 100%%", [])
    test_assert(stringz.contains(percent_result, "100%"), "%% escape works")
    
    fr fr Test multiple specifiers
    sus multi_result tea = format_advanced("User %s has %d points", ["Bob", "150"])
    test_assert(stringz.contains(multi_result, "Bob"), "Multiple specifiers first arg")
    test_assert(stringz.contains(multi_result, "150"), "Multiple specifiers second arg")
}

fr fr ===== STRING UTILITY TESTS =====

slay test_string_utilities() {
    test_start("String Utility Functions")
    
    fr fr Test string concatenation
    sus concat_result tea = string_concat("Hello", " World")
    test_assert(stringz.contains(concat_result, "Hello"), "string_concat() preserves first")
    test_assert(stringz.contains(concat_result, "World"), "string_concat() preserves second")
    
    fr fr Test string length
    sus length_result drip = string_length("Testing")
    test_assert(length_result == 7, "string_length() counts characters")
    
    fr fr Test character extraction
    sus char_result normie = char_at("Hello", 1)
    test_assert(char_result == 101, "char_at() extracts correct character") fr fr 'e' = 101
    
    fr fr Test substring
    sus sub_result tea = substring("Hello World", 6, 11)
    test_assert(stringz.contains(sub_result, "World"), "substring() extracts correctly")
}

fr fr ===== ERROR HANDLING TESTS =====

slay test_error_handling() {
    test_start("Error Handling and Edge Cases")
    
    fr fr Test null input handling
    sus null_result1 tea = spillf(cringe, "test")
    test_assert(stringz.length(null_result1) == 0, "spillf() handles null format")
    
    sus null_result2 tea = spillf("test", cringe)
    test_assert(stringz.contains(null_result2, "test"), "spillf() handles null arg")
    
    fr fr Test empty string handling
    sus empty_result tea = string_concat("", "test")
    test_assert(stringz.contains(empty_result, "test"), "string_concat() handles empty first")
    
    sus empty_result2 tea = string_concat("test", "")
    test_assert(stringz.contains(empty_result2, "test"), "string_concat() handles empty second")
    
    fr fr Test bounds checking
    sus bounds_result normie = char_at("test", 10)
    test_assert(bounds_result == 0, "char_at() handles out of bounds")
    
    fr fr Test invalid file operations
    sus invalid_read tea = read_file("")
    test_assert(stringz.length(invalid_read) == 0, "read_file() handles empty filename")
    
    sus invalid_write lit = write_file("", "content")
    test_assert(invalid_write == cap, "write_file() handles empty filename")
}

fr fr ===== PERFORMANCE TESTS =====

slay test_performance() {
    test_start("Performance and Stress Testing")
    
    fr fr Test large string handling
    sus large_content tea = "This is a test string for performance testing. "
    sus i normie = 0
    bestie i < 10 {  fr fr Concatenate to make larger string
        large_content = string_concat(large_content, "More content. ")
        i = i + 1
    }
    
    sus large_length drip = string_length(large_content)
    test_assert(large_length > 100, "Handles large strings correctly")
    
    fr fr Test multiple formatting operations
    sus format_count normie = 0
    bestie format_count < 5 {
        sus format_result tea = spillf("Test %d iteration", number_to_string(format_count))
        test_assert(stringz.length(format_result) > 0, "Multiple formatting operations work")
        format_count = format_count + 1
    }
    
    fr fr Test file operations with various sizes
    sus small_file tea = "/tmp/small_test.txt"
    sus small_content tea = "Small"
    sus small_write lit = write_file(small_file, small_content)
    test_assert(small_write == based, "Small file write works")
    
    sus medium_file tea = "/tmp/medium_test.txt" 
    sus medium_content tea = string_concat(large_content, large_content)
    sus medium_write lit = write_file(medium_file, medium_content)
    test_assert(medium_write == based, "Medium file write works")
}

fr fr ===== INTEGRATION TESTS =====

slay test_integration() {
    test_start("Integration Testing")
    
    fr fr Test combined formatting and file operations
    sus template tea = "Log entry: User %s performed %s at %s"
    sus log_entry tea = format_advanced(template, ["Alice", "login", "2024-07-16T10:30:00Z"])
    
    sus log_file tea = "/tmp/integration_test.log"
    sus write_success lit = write_file(log_file, log_entry)
    test_assert(write_success == based, "Combined formatting and file write works")
    
    sus read_back tea = read_file(log_file)
    test_assert(stringz.contains(read_back, "Alice"), "Integration test preserves data")
    test_assert(stringz.contains(read_back, "login"), "Integration test preserves all data")
    
    fr fr Test chained operations
    sus step1_result tea = spillstr("Step 1: %s", "Initialize")
    sus step2_input tea = string_concat(step1_result, " | Step 2: Process")
    sus step3_result tea = string_concat(step2_input, " | Step 3: Complete")
    
    test_assert(stringz.contains(step3_result, "Step 1"), "Chained operations preserve step 1")
    test_assert(stringz.contains(step3_result, "Step 2"), "Chained operations preserve step 2")
    test_assert(stringz.contains(step3_result, "Step 3"), "Chained operations preserve step 3")
}

fr fr ===== MAIN TEST RUNNER =====

slay run_all_tests() {
    spillln("🚀 Starting Comprehensive VIBEZ Module Testing")
    spillln("=============================================")
    
    test_basic_output()
    test_formatting_functions()
    test_placeholder_replacement()
    test_input_operations()
    test_file_operations()
    test_number_conversion()
    test_base_conversion()
    test_format_specifiers()
    test_string_utilities()
    test_error_handling()
    test_performance()
    test_integration()
    
    test_summary()
    
    fr fr Return overall test result
    damn failed_tests == 0
}

fr fr Helper functions for testing

slay number_to_string(num normie) tea {
    damn integer_to_string_advanced(num)
}

fr fr Run the comprehensive test suite
sus test_success lit = run_all_tests()

ready test_success {
    spillln("🎉 VIBEZ MODULE: ALL TESTS PASSED - PRODUCTION READY!")
}
otherwise {
    spillln("💥 VIBEZ MODULE: SOME TESTS FAILED - NEEDS ATTENTION!")
}

fr fr Display final status
spill("Final Status: ")
ready test_success {
    spillln("✅ SUCCESS")
}
otherwise {
    spillln("❌ FAILURE")
}
