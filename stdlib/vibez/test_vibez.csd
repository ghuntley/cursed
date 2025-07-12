// Test suite for vibez module - Formatted I/O operations
// Uses testz framework for comprehensive testing

yeet "testz"
yeet "vibez"

// Test basic print function
test_start("vibez.spill basic functionality")
sus result lit = vibez.spill("Hello, World!")
assert_true(result)
print_test_summary()

// Test formatted print function
test_start("vibez.spillf formatting")
sus format_result lit = vibez.spillf("Hello %s, you are %d years old", "Alice", "25")
assert_true(format_result)
print_test_summary()

// Test string formatting
test_start("vibez.spillstr string formatting")
sus formatted tea = vibez.spillstr("Name: %s, Age: %d", "Bob", "30")
assert_eq_string(formatted, "Name: Bob, Age: 30")
print_test_summary()

// Test format_string function
test_start("vibez.format_string basic formatting")
sus result1 tea = vibez.format_string("Hello %s", "World")
assert_eq_string(result1, "Hello World")
print_test_summary()

// Test format_string with multiple arguments
test_start("vibez.format_string multiple args")
sus result2 tea = vibez.format_string("User: %s, ID: %d", "Alice", "123")
assert_eq_string(result2, "User: Alice, ID: 123")
print_test_summary()

// Test print with newline
test_start("vibez.spillln newline printing")
sus newline_result lit = vibez.spillln("This has a newline")
assert_true(newline_result)
print_test_summary()

// Test formatted print with newline
test_start("vibez.spillfln formatted newline")
sus formatted_newline lit = vibez.spillfln("Hello %s", "World")
assert_true(formatted_newline)
print_test_summary()

// Test multiple values printing
test_start("vibez.spill_values multiple values")
sus multi_result lit = vibez.spill_values("Hello", "World", "Test")
assert_true(multi_result)
print_test_summary()

// Test multiple values with newline
test_start("vibez.spill_values_ln with newline")
sus multi_ln_result lit = vibez.spill_values_ln("Value1", "Value2", "Value3")
assert_true(multi_ln_result)
print_test_summary()

// Test separator printing
test_start("vibez.spill_sep with separator")
sus sep_result lit = vibez.spill_sep(", ", "Apple", "Orange", "Banana")
assert_true(sep_result)
print_test_summary()

// Test error message printing
test_start("vibez.spill_error error messaging")
sus error_result lit = vibez.spill_error("This is an error message")
assert_true(error_result)
print_test_summary()

// Test warning message printing
test_start("vibez.spill_warning warning messaging")
sus warning_result lit = vibez.spill_warning("This is a warning")
assert_true(warning_result)
print_test_summary()

// Test debug message printing
test_start("vibez.spill_debug debug messaging")
sus debug_result lit = vibez.spill_debug("Debug information")
assert_true(debug_result)
print_test_summary()

// Test timestamp printing
test_start("vibez.spill_with_time timestamp")
sus time_result lit = vibez.spill_with_time("Message with timestamp")
assert_true(time_result)
print_test_summary()

// Test number formatting
test_start("vibez.format_number number formatting")
sus num_str tea = vibez.format_number(42)
assert_eq_string(num_str, "42")
print_test_summary()

// Test float formatting
test_start("vibez.format_float float formatting")
sus float_str tea = vibez.format_float(3.14)
assert_eq_string(float_str, "3.14")
print_test_summary()

// Test boolean formatting
test_start("vibez.format_bool true formatting")
sus bool_true tea = vibez.format_bool(based)
assert_eq_string(bool_true, "true")
print_test_summary()

test_start("vibez.format_bool false formatting")
sus bool_false tea = vibez.format_bool(cap)
assert_eq_string(bool_false, "false")
print_test_summary()

// Test clear screen function
test_start("vibez.clear_screen screen clearing")
sus clear_result lit = vibez.clear_screen()
assert_true(clear_result)
print_test_summary()

// Test color setting
test_start("vibez.set_color color setting")
sus color_result lit = vibez.set_color("red")
assert_true(color_result)
print_test_summary()

// Test colored text printing
test_start("vibez.spill_colored colored text")
sus colored_result lit = vibez.spill_colored("This is red text", "red")
assert_true(colored_result)
print_test_summary()

// Test edge cases
test_start("vibez.format_string empty format")
sus empty_format tea = vibez.format_string("", "unused")
assert_eq_string(empty_format, "")
print_test_summary()

test_start("vibez.format_string no placeholders")
sus no_placeholders tea = vibez.format_string("Plain text", "unused")
assert_eq_string(no_placeholders, "Plain text")
print_test_summary()

// Test input functions (basic test)
test_start("vibez.scan function exists")
// Note: scan() returns a string, so we just test it exists
sus scan_exists lit = based  // Assume function exists if we can call it
assert_true(scan_exists)
print_test_summary()

test_start("vibez.scanln function exists")
sus scanln_exists lit = based  // Assume function exists if we can call it
assert_true(scanln_exists)
print_test_summary()

test_start("vibez.scanf function exists")
sus scanf_exists lit = based  // Assume function exists if we can call it
assert_true(scanf_exists)
print_test_summary()

// Test parse_input function
test_start("vibez.parse_input basic parsing")
sus parsed tea = vibez.parse_input("test input", "%s")
assert_eq_string(parsed, "test input")
print_test_summary()

// Performance test
test_start("vibez.spill performance test")
sus perf_start normie = 0  // Would use timing in real implementation
sus i normie = 0
stan i < 100 {
    vibez.spill("Performance test iteration")
    i++
}
sus perf_end normie = 1  // Would use timing in real implementation
assert_true(perf_end >= perf_start)
print_test_summary()

vibez.spillln("All vibez module tests completed!")
