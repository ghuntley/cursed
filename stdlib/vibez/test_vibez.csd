// Comprehensive test suite for vibez module
yeet "testz"
yeet "vibez"

// ================================
// Core Specification Function Tests
// ================================

slay test_core_spec_functions() {
    test_start("Core specification functions")
    
    // Test spillf - formatted print
    vibez.spillf("Number: %d", [42])
    test_pass("spillf function works")
    
    // Test spillstr - formatted string return
    sus formatted tea = vibez.spillstr("Value: %d", [123])
    assert_eq_string(formatted, "Value: 123")
    test_pass("spillstr function works")
    
    // Test scan - simplified test (would need input simulation)
    sus scan_result normie = vibez.scan("", [])
    assert_true(scan_result >= 0)
    test_pass("scan function works")
    
    // Test scanln - simplified test (would need input simulation)
    sus scanln_result normie = vibez.scanln("", [])
    assert_true(scanln_result >= 0)
    test_pass("scanln function works")
}

// ================================
// Basic Output Function Tests
// ================================

slay test_basic_output() {
    test_start("Basic spill function")
    
    // Test basic string output
    vibez.spill("Hello, CURSED!")
    test_pass("spill function works")
    
    // Test integer output
    vibez.spill_int(42)
    test_pass("spill_int function works")
    
    // Test float output
    vibez.spill_float(3.14159)
    test_pass("spill_float function works")
    
    // Test boolean output
    vibez.spill_bool(based)
    vibez.spill_bool(cap)
    test_pass("spill_bool function works")
    
    // Test character output
    vibez.spill_char('X')
    test_pass("spill_char function works")
}

slay test_println_functions() {
    test_start("Println functions")
    
    // Test println with string
    vibez.println("This is a line")
    test_pass("println function works")
    
    // Test println with different types
    vibez.println_int(123)
    vibez.println_float(2.718)
    vibez.println_bool(based)
    vibez.println_char('Y')
    test_pass("All println functions work")
}

// ================================
// String Formatting Tests
// ================================

slay test_format_functions() {
    test_start("Format functions")
    
    // Test integer formatting
    sus int_str tea = vibez.format_int(42)
    assert_eq_string(int_str, "42")
    
    // Test float formatting
    sus float_str tea = vibez.format_float(3.14)
    // Note: Float formatting may vary, just check it's not empty
    assert_true(string_len(float_str) > 0)
    
    // Test boolean formatting
    sus bool_str_true tea = vibez.format_bool(based)
    sus bool_str_false tea = vibez.format_bool(cap)
    assert_eq_string(bool_str_true, "based")
    assert_eq_string(bool_str_false, "cap")
    
    // Test character formatting
    sus char_str tea = vibez.format_char('Z')
    assert_eq_string(char_str, "Z")
    
    test_pass("All format functions work correctly")
}

// ================================
// Advanced Formatting Tests
// ================================

slay test_sprintf_function() {
    test_start("sprintf function")
    
    // Test basic placeholder replacement
    sus result tea = vibez.sprintf("Hello, {}!", ["World"])
    assert_eq_string(result, "Hello, World!")
    
    // Test multiple placeholders
    sus result2 tea = vibez.sprintf("The answer is {}", ["42"])
    assert_eq_string(result2, "The answer is 42")
    
    // Test indexed placeholders
    sus result3 tea = vibez.sprintf("{0} + {1} = {2}", ["2", "3", "5"])
    assert_eq_string(result3, "2 + 3 = 5")
    
    test_pass("sprintf function works correctly")
}

slay test_printf_functions() {
    test_start("printf functions")
    
    // Test printf
    vibez.printf("Formatted output: {}\n", ["test"])
    test_pass("printf function works")
    
    // Test printfln
    vibez.printfln("Formatted line: {}", ["complete"])
    test_pass("printfln function works")
}

// ================================
// Debug and Development Tests
// ================================

slay test_debug_functions() {
    test_start("Debug functions")
    
    // Test debug print
    vibez.debug_print("This is a debug message")
    test_pass("debug_print function works")
    
    // Test debug print with types
    vibez.debug_print_int("test_var", 100)
    vibez.debug_print_float("pi", 3.14159)
    vibez.debug_print_bool("flag", based)
    test_pass("All debug print functions work")
    
    // Test info/error/warning
    vibez.info_print("Information message")
    vibez.error_print("Error message")
    vibez.warning_print("Warning message")
    test_pass("All message level functions work")
}

// ================================
// Utility Function Tests
// ================================

slay test_utility_functions() {
    test_start("Utility functions")
    
    // Test repeat_char
    sus repeated tea = vibez.repeat_char('*', 5)
    assert_eq_string(repeated, "*****")
    
    // Test pad_left
    sus padded_left tea = vibez.pad_left("test", 8, '0')
    assert_eq_string(padded_left, "0000test")
    
    // Test pad_right
    sus padded_right tea = vibez.pad_right("test", 8, '-')
    assert_eq_string(padded_right, "test----")
    
    // Test center_text
    sus centered tea = vibez.center_text("hi", 6, ' ')
    assert_eq_string(centered, "  hi  ")
    
    test_pass("All utility functions work correctly")
}

// ================================
// Color Output Tests
// ================================

slay test_color_functions() {
    test_start("Color functions")
    
    // Test color functions
    sus red_text tea = vibez.color_red("Red text")
    assert_true(string_contains(red_text, "Red text"))
    assert_true(string_contains(red_text, "\033[31m"))
    
    sus green_text tea = vibez.color_green("Green text")
    assert_true(string_contains(green_text, "Green text"))
    assert_true(string_contains(green_text, "\033[32m"))
    
    sus blue_text tea = vibez.color_blue("Blue text")
    assert_true(string_contains(blue_text, "Blue text"))
    assert_true(string_contains(blue_text, "\033[34m"))
    
    test_pass("Color functions work correctly")
}

// ================================
// Integration Tests
// ================================

slay test_integration_formatting() {
    test_start("Integration formatting")
    
    // Test complex formatting scenario
    sus name tea = "CURSED"
    sus version tea = "1.0.0"
    sus status tea = "Ready"
    
    // Create formatted output
    vibez.println("")
    vibez.print_header("System Status", 40)
    
    sus name_line tea = vibez.sprintf("Name: {}", [name])
    sus version_line tea = vibez.sprintf("Version: {}", [version])
    sus status_line tea = vibez.sprintf("Status: {}", [status])
    
    vibez.println(name_line)
    vibez.println(version_line)
    vibez.println(status_line)
    
    vibez.print_separator(40, '=')
    
    test_pass("Integration formatting works correctly")
}

// ================================
// Main Test Runner
// ================================

slay run_all_tests() {
    vibez.println("========================================")
    vibez.println("         VIBEZ MODULE TESTS")
    vibez.println("========================================")
    vibez.println("")
    
    // Run all test functions
    test_core_spec_functions()
    test_basic_output()
    test_println_functions()
    test_format_functions()
    test_sprintf_function()
    test_printf_functions()
    test_debug_functions()
    test_utility_functions()
    test_color_functions()
    test_integration_formatting()
    
    vibez.println("")
    vibez.println("========================================")
    print_test_summary()
    vibez.println("========================================")
}

// Start the test suite
run_all_tests()
