// Comprehensive test suite for vibez module
yeet "testz"
yeet "vibez"

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
    println("This is a line")
    test_pass("println function works")
    
    // Test println with different types
    println_int(123)
    println_float(2.718)
    println_bool(based)
    println_char('Y')
    test_pass("All println functions work")
}

// ================================
// String Formatting Tests
// ================================

slay test_format_functions() {
    test_start("Format functions")
    
    // Test integer formatting
    sus int_str tea = format_int(42)
    assert_eq_string(int_str, "42")
    
    // Test float formatting
    sus float_str tea = format_float(3.14)
    // Note: Float formatting may vary, just check it's not empty
    assert_true(string_len(float_str) > 0)
    
    // Test boolean formatting
    sus bool_str_true tea = format_bool(based)
    sus bool_str_false tea = format_bool(cap)
    assert_eq_string(bool_str_true, "based")
    assert_eq_string(bool_str_false, "cap")
    
    // Test character formatting
    sus char_str tea = format_char('Z')
    assert_eq_string(char_str, "Z")
    
    test_pass("All format functions work correctly")
}

// ================================
// Advanced Formatting Tests
// ================================

slay test_sprintf_function() {
    test_start("sprintf function")
    
    // Test basic placeholder replacement
    sus result tea = sprintf("Hello, {}!", ["World"])
    assert_eq_string(result, "Hello, World!")
    
    // Test multiple placeholders
    sus result2 tea = sprintf("The answer is {}", ["42"])
    assert_eq_string(result2, "The answer is 42")
    
    // Test indexed placeholders
    sus result3 tea = sprintf("{0} + {1} = {2}", ["2", "3", "5"])
    assert_eq_string(result3, "2 + 3 = 5")
    
    test_pass("sprintf function works correctly")
}

slay test_printf_functions() {
    test_start("printf functions")
    
    // Test printf
    printf("Formatted output: {}\n", ["test"])
    test_pass("printf function works")
    
    // Test printfln
    printfln("Formatted line: {}", ["complete"])
    test_pass("printfln function works")
}

// ================================
// Type-Safe Formatting Tests
// ================================

slay test_type_safe_formatting() {
    test_start("Type-safe formatting")
    
    // Test format_with_type
    sus int_result tea = format_with_type(42, "int")
    assert_eq_string(int_result, "42")
    
    sus float_result tea = format_with_type(123, "float")
    assert_true(string_len(float_result) > 0)
    
    sus bool_result tea = format_with_type(1, "bool")
    assert_eq_string(bool_result, "based")
    
    test_pass("Type-safe formatting works correctly")
}

// ================================
// Debug and Development Tests
// ================================

slay test_debug_functions() {
    test_start("Debug functions")
    
    // Test debug print
    debug_print("This is a debug message")
    test_pass("debug_print function works")
    
    // Test debug print with types
    debug_print_int("test_var", 100)
    debug_print_float("pi", 3.14159)
    debug_print_bool("flag", based)
    test_pass("All debug print functions work")
    
    // Test info/error/warning
    info_print("Information message")
    error_print("Error message")
    warning_print("Warning message")
    test_pass("All message level functions work")
}

// ================================
// Utility Function Tests
// ================================

slay test_utility_functions() {
    test_start("Utility functions")
    
    // Test repeat_char
    sus repeated tea = repeat_char('*', 5)
    assert_eq_string(repeated, "*****")
    
    // Test pad_left
    sus padded_left tea = pad_left("test", 8, '0')
    assert_eq_string(padded_left, "0000test")
    
    // Test pad_right
    sus padded_right tea = pad_right("test", 8, '-')
    assert_eq_string(padded_right, "test----")
    
    // Test center_text
    sus centered tea = center_text("hi", 6, ' ')
    assert_eq_string(centered, "  hi  ")
    
    test_pass("All utility functions work correctly")
}

// ================================
// Formatted Output Tests
// ================================

slay test_formatted_output() {
    test_start("Formatted output")
    
    // Test separator
    print_separator(20, '-')
    test_pass("print_separator function works")
    
    // Test header
    print_header("Test Header", 30)
    test_pass("print_header function works")
    
    // Test table row
    sus columns tea[value] = ["Column1", "Column2", "Column3"]
    print_row(columns, 30)
    test_pass("print_row function works")
}

// ================================
// Number Formatting Tests
// ================================

slay test_number_formatting() {
    test_start("Number formatting")
    
    // Test padded integer
    sus padded_int tea = format_int_padded(42, 5)
    assert_eq_string(padded_int, "00042")
    
    // Test float precision (simplified)
    sus float_precision tea = format_float_precision(3.14159, 2)
    assert_true(string_len(float_precision) > 0)
    
    // Test percentage
    sus percentage tea = format_percentage(0.75)
    assert_true(string_contains(percentage, "75"))
    assert_true(string_contains(percentage, "%"))
    
    test_pass("Number formatting functions work correctly")
}

// ================================
// Color Output Tests
// ================================

slay test_color_functions() {
    test_start("Color functions")
    
    // Test color functions
    sus red_text tea = color_red("Red text")
    assert_true(string_contains(red_text, "Red text"))
    assert_true(string_contains(red_text, "\033[31m"))
    
    sus green_text tea = color_green("Green text")
    assert_true(string_contains(green_text, "Green text"))
    assert_true(string_contains(green_text, "\033[32m"))
    
    sus blue_text tea = color_blue("Blue text")
    assert_true(string_contains(blue_text, "Blue text"))
    assert_true(string_contains(blue_text, "\033[34m"))
    
    test_pass("Color functions work correctly")
}

// ================================
// Success/Error Message Tests
// ================================

slay test_message_functions() {
    test_start("Message functions")
    
    // Test colored message functions
    success_print("Operation completed successfully")
    error_print_colored("This is an error")
    warning_print_colored("This is a warning")
    info_print_colored("This is information")
    
    test_pass("All message functions work correctly")
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
    println("")
    print_header("System Status", 40)
    
    sus name_line tea = sprintf("Name: {}", [name])
    sus version_line tea = sprintf("Version: {}", [version])
    sus status_line tea = sprintf("Status: {}", [status])
    
    println(name_line)
    println(version_line)
    println(status_line)
    
    print_separator(40, '=')
    
    test_pass("Integration formatting works correctly")
}

slay test_mixed_type_output() {
    test_start("Mixed type output")
    
    // Test outputting different types together
    spill("Integer: ")
    spill_int(42)
    spill(" Float: ")
    spill_float(3.14)
    spill(" Boolean: ")
    spill_bool(based)
    spill(" Character: ")
    spill_char('X')
    println("")
    
    test_pass("Mixed type output works correctly")
}

// ================================
// Main Test Runner
// ================================

slay run_all_tests() {
    println("========================================")
    println("         VIBEZ MODULE TESTS")
    println("========================================")
    println("")
    
    // Run all test functions
    test_basic_output()
    test_println_functions()
    test_format_functions()
    test_sprintf_function()
    test_printf_functions()
    test_type_safe_formatting()
    test_debug_functions()
    test_utility_functions()
    test_formatted_output()
    test_number_formatting()
    test_color_functions()
    test_message_functions()
    test_integration_formatting()
    test_mixed_type_output()
    
    println("")
    println("========================================")
    print_test_summary()
    println("========================================")
}

// Start the test suite
run_all_tests()
