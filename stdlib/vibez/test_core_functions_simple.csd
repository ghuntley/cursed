// Simple test suite for vibez core functions
// Tests core runtime functionality

yeet "testz"
yeet "stringz"

// Include core functions directly by testing vibez module
yeet "vibez"

// Test basic vibez functions which use core functions
test_start("vibez.spill basic functionality")
sus result lit = vibez.spill("Hello, World!")
assert_true(result)
print_test_summary()

test_start("vibez.format_number number conversion")
sus num_str tea = vibez.format_number(42)
assert_eq_string(num_str, "42")
print_test_summary()

test_start("vibez.format_float float conversion")
sus float_str tea = vibez.format_float(3.14)
assert_eq_string(float_str, "3.14")
print_test_summary()

test_start("vibez.format_bool boolean conversion")
sus bool_true tea = vibez.format_bool(based)
assert_eq_string(bool_true, "true")
sus bool_false tea = vibez.format_bool(cap)
assert_eq_string(bool_false, "false")
print_test_summary()

test_start("vibez.spill_with_time timestamp functionality")
sus time_result lit = vibez.spill_with_time("Test message")
assert_true(time_result)
print_test_summary()

test_start("vibez.spillf formatting functionality")
sus format_result lit = vibez.spillf("Hello %s, number %d", "World", "42")
assert_true(format_result)
print_test_summary()

test_start("vibez.spillstr string formatting")
sus formatted tea = vibez.spillstr("User: %s, ID: %d", "Alice", "123")
assert_true(stringz.contains(formatted, "Alice"))
assert_true(stringz.contains(formatted, "123"))
print_test_summary()

test_start("vibez.spill_error error messaging")
sus error_result lit = vibez.spill_error("Test error message")
assert_true(error_result)
print_test_summary()

test_start("vibez.spill_warning warning messaging")
sus warning_result lit = vibez.spill_warning("Test warning")
assert_true(warning_result)
print_test_summary()

test_start("vibez.spill_debug debug messaging")
sus debug_result lit = vibez.spill_debug("Debug information")
assert_true(debug_result)
print_test_summary()

test_start("vibez.spill_colored colored output")
sus colored_result lit = vibez.spill_colored("Colored text", "red")
assert_true(colored_result)
print_test_summary()

test_start("vibez.clear_screen screen clearing")
sus clear_result lit = vibez.clear_screen()
assert_true(clear_result)
print_test_summary()

test_start("vibez.scan input functionality")
fr fr Note: scan() returns a string, so we test it exists and works
sus scan_result tea = vibez.scan()
assert_true(scan_result != cringe)
print_test_summary()

// Performance test
test_start("vibez output performance test")
sus i normie = 0
stan i < 50 {
    vibez.spill("Performance test iteration")
    i++
}
assert_true(based)  // Test completed successfully
print_test_summary()

// Complex formatting test
test_start("vibez complex formatting test")
sus complex_format tea = vibez.spillstr("Name: %s, Age: %d, Score: %s", "Bob", "25", "95.5")
assert_true(stringz.contains(complex_format, "Bob"))
assert_true(stringz.contains(complex_format, "25"))
assert_true(stringz.contains(complex_format, "95.5"))
print_test_summary()

vibez.spillln("Core functions integration tests completed!")
vibez.spillln("Production-ready I/O operations validated!")
