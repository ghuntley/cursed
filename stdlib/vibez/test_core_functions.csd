// Comprehensive test suite for vibez core functions
// Tests all production-ready core runtime functions

yeet "testz"
yeet "stringz"

// Test initialization and runtime state
test_start("core_funcs.init_runtime basic initialization")
sus init_result lit = core_funcs.init_runtime()
assert_true(init_result)
print_test_summary()

test_start("core_funcs.is_runtime_ready runtime state check")
sus ready_state lit = core_funcs.is_runtime_ready()
assert_true(ready_state)
print_test_summary()

test_start("core_funcs.get_last_error initial error state")
sus initial_error normie = core_funcs.get_last_error()
assert_eq_int(initial_error, 0)
print_test_summary()

// Test basic print functionality
test_start("core_funcs.print valid message")
sus print_result lit = core_funcs.print("Hello, World!")
assert_true(print_result)
sus error_after_print normie = core_funcs.get_last_error()
assert_eq_int(error_after_print, 0)
print_test_summary()

test_start("core_funcs.print empty string")
sus empty_print lit = core_funcs.print("")
assert_true(empty_print)
print_test_summary()

test_start("core_funcs.print_safe safe printing")
sus safe_print lit = core_funcs.print_safe("Safe print test")
assert_true(safe_print)
print_test_summary()

test_start("core_funcs.emergency_print emergency output")
sus emergency_result lit = core_funcs.emergency_print("Emergency message")
assert_true(emergency_result)
print_test_summary()

// Test input functionality
test_start("core_funcs.read_line basic input reading")
sus input_result tea = core_funcs.read_line()
assert_true(input_result != cringe)
sus input_length normie = stringz.length(input_result)
assert_true(input_length >= 0)
print_test_summary()

test_start("core_funcs.read_line_safe safe input reading")
sus safe_input tea = core_funcs.read_line_safe(5000)
assert_true(safe_input != cringe)
print_test_summary()

test_start("core_funcs.is_valid_input input validation")
sus valid_test1 lit = core_funcs.is_valid_input("valid input")
assert_true(valid_test1)
sus valid_test2 lit = core_funcs.is_valid_input("")
assert_true(valid_test2)  // Empty is valid
print_test_summary()

// Test timestamp functionality
test_start("core_funcs.get_timestamp timestamp generation")
sus timestamp tea = core_funcs.get_timestamp()
assert_true(timestamp != cringe)
sus timestamp_length normie = stringz.length(timestamp)
assert_true(timestamp_length > 10)  // ISO format should be long
assert_true(stringz.contains(timestamp, "T"))  // Should have ISO separator
assert_true(stringz.contains(timestamp, "Z"))  // Should have UTC indicator
print_test_summary()

test_start("core_funcs.format_timestamp timestamp formatting")
sus formatted tea = core_funcs.format_timestamp(2024, 7, 16, 14, 30, 45, 123)
assert_eq_string(formatted, "2024-07-16T14:30:45.123Z")
print_test_summary()

test_start("core_funcs.pad_number number padding")
sus padded_2 tea = core_funcs.pad_number(5, 2)
assert_eq_string(padded_2, "05")
sus padded_4 tea = core_funcs.pad_number(42, 4)
assert_eq_string(padded_4, "0042")
print_test_summary()

test_start("core_funcs.get_timestamp_ms millisecond timestamp")
sus ms_timestamp normie = core_funcs.get_timestamp_ms()
assert_true(ms_timestamp > 0)
print_test_summary()

test_start("core_funcs.get_timestamp_us microsecond timestamp")
sus us_timestamp normie = core_funcs.get_timestamp_us()
assert_true(us_timestamp > 0)
assert_true(us_timestamp > ms_timestamp)  // Should be higher precision
print_test_summary()

// Test number to string conversion
test_start("core_funcs.number_to_string zero")
sus zero_str tea = core_funcs.number_to_string(0)
assert_eq_string(zero_str, "0")
print_test_summary()

test_start("core_funcs.number_to_string positive number")
sus pos_str tea = core_funcs.number_to_string(42)
assert_eq_string(pos_str, "42")
print_test_summary()

test_start("core_funcs.number_to_string negative number")
sus neg_str tea = core_funcs.number_to_string(-123)
assert_eq_string(neg_str, "-123")
print_test_summary()

test_start("core_funcs.number_to_string large number")
sus large_str tea = core_funcs.number_to_string(12345)
assert_true(stringz.contains(large_str, "12345"))
print_test_summary()

test_start("core_funcs.digit_to_char digit conversion")
sus digit_0 tea = core_funcs.digit_to_char(0)
assert_eq_string(digit_0, "0")
sus digit_9 tea = core_funcs.digit_to_char(9)
assert_eq_string(digit_9, "9")
sus invalid_digit tea = core_funcs.digit_to_char(15)
assert_eq_string(invalid_digit, "?")
print_test_summary()

// Test float to string conversion
test_start("core_funcs.float_to_string basic conversion")
sus float_str tea = core_funcs.float_to_string(3.14)
assert_true(stringz.contains(float_str, "3"))
assert_true(stringz.contains(float_str, "."))
print_test_summary()

test_start("core_funcs.float_to_string_precision precision control")
sus precise_str tea = core_funcs.float_to_string_precision(2.5, 2)
assert_true(stringz.contains(precise_str, "2"))
assert_true(stringz.contains(precise_str, "."))
print_test_summary()

test_start("core_funcs.float_to_string zero float")
sus zero_float tea = core_funcs.float_to_string(0.0)
assert_eq_string(zero_float, "0.0")
print_test_summary()

test_start("core_funcs.float_to_int float truncation")
sus truncated normie = core_funcs.float_to_int(3.14)
assert_eq_int(truncated, 3)
sus zero_trunc normie = core_funcs.float_to_int(0.9)
assert_eq_int(zero_trunc, 0)
print_test_summary()

test_start("core_funcs.int_to_float integer conversion")
sus int_as_float drip = core_funcs.int_to_float(42)
assert_eq_float(int_as_float, 42.0)
print_test_summary()

// Test string to number parsing
test_start("core_funcs.string_to_number valid number")
core_funcs.clear_error()
sus parsed_num normie = core_funcs.string_to_number("123")
sus parse_error normie = core_funcs.get_last_error()
assert_eq_int(parse_error, 0)
assert_eq_int(parsed_num, 123)
print_test_summary()

test_start("core_funcs.string_to_number negative number")
core_funcs.clear_error()
sus parsed_neg normie = core_funcs.string_to_number("-456")
sus neg_error normie = core_funcs.get_last_error()
assert_eq_int(neg_error, 0)
assert_eq_int(parsed_neg, -456)
print_test_summary()

test_start("core_funcs.string_to_number invalid input")
core_funcs.clear_error()
sus invalid_parse normie = core_funcs.string_to_number("not_a_number")
sus invalid_error normie = core_funcs.get_last_error()
assert_true(invalid_error != 0)  // Should have error
print_test_summary()

test_start("core_funcs.string_to_number empty string")
core_funcs.clear_error()
sus empty_parse normie = core_funcs.string_to_number("")
sus empty_error normie = core_funcs.get_last_error()
assert_true(empty_error != 0)  // Should have error
print_test_summary()

test_start("core_funcs.is_valid_number_string validation")
sus valid_num1 lit = core_funcs.is_valid_number_string("123", 0)
assert_true(valid_num1)
sus valid_num2 lit = core_funcs.is_valid_number_string("-456", 1)  // Skip negative sign
assert_true(valid_num2)
sus invalid_num lit = core_funcs.is_valid_number_string("12a3", 0)
assert_false(invalid_num)
print_test_summary()

test_start("core_funcs.is_digit_char character validation")
sus is_digit_5 lit = core_funcs.is_digit_char("5")
assert_true(is_digit_5)
sus is_not_digit lit = core_funcs.is_digit_char("a")
assert_false(is_not_digit)
sus is_not_digit_multi lit = core_funcs.is_digit_char("12")
assert_false(is_not_digit_multi)  // Multi-char should be false
print_test_summary()

test_start("core_funcs.char_to_digit character conversion")
sus char_digit_0 normie = core_funcs.char_to_digit("0")
assert_eq_int(char_digit_0, 0)
sus char_digit_9 normie = core_funcs.char_to_digit("9")
assert_eq_int(char_digit_9, 9)
sus char_invalid normie = core_funcs.char_to_digit("x")
assert_eq_int(char_invalid, -1)
print_test_summary()

// Test environment variable functions
test_start("core_funcs.get_env_var known variable")
core_funcs.clear_error()
sus home_var tea = core_funcs.get_env_var("HOME")
sus home_error normie = core_funcs.get_last_error()
assert_eq_int(home_error, 0)
assert_eq_string(home_var, "/home/user")
print_test_summary()

test_start("core_funcs.get_env_var unknown variable")
core_funcs.clear_error()
sus unknown_var tea = core_funcs.get_env_var("UNKNOWN_VAR_12345")
sus unknown_error normie = core_funcs.get_last_error()
assert_true(unknown_error != 0)  // Should have error
assert_eq_string(unknown_var, "")
print_test_summary()

test_start("core_funcs.set_env_var variable setting")
core_funcs.clear_error()
sus set_result lit = core_funcs.set_env_var("TEST_VAR", "test_value")
sus set_error normie = core_funcs.get_last_error()
assert_eq_int(set_error, 0)
assert_true(set_result)
print_test_summary()

// Test file system functions
test_start("core_funcs.file_exists existing file")
core_funcs.clear_error()
sus exists_result lit = core_funcs.file_exists("/etc/passwd")
assert_true(exists_result)
print_test_summary()

test_start("core_funcs.file_exists cursed file")
sus csd_exists lit = core_funcs.file_exists("test.csd")
assert_true(csd_exists)  // Should recognize .csd files
print_test_summary()

test_start("core_funcs.file_exists non-existent file")
sus not_exists lit = core_funcs.file_exists("/non/existent/file/path")
assert_false(not_exists)
print_test_summary()

test_start("core_funcs.get_file_size file size")
core_funcs.clear_error()
sus file_size normie = core_funcs.get_file_size("test.csd")
sus size_error normie = core_funcs.get_last_error()
assert_eq_int(size_error, 0)
assert_true(file_size >= 0)
print_test_summary()

test_start("core_funcs.get_file_size non-existent file")
core_funcs.clear_error()
sus no_size normie = core_funcs.get_file_size("/non/existent")
sus no_size_error normie = core_funcs.get_last_error()
assert_true(no_size_error != 0)  // Should have error
assert_eq_int(no_size, -1)
print_test_summary()

// Test memory tracking functions
test_start("core_funcs.track_memory_alloc memory allocation")
core_funcs.clear_error()
sus alloc_result lit = core_funcs.track_memory_alloc(1024)
sus alloc_error normie = core_funcs.get_last_error()
assert_eq_int(alloc_error, 0)
assert_true(alloc_result)
print_test_summary()

test_start("core_funcs.get_memory_usage usage tracking")
sus memory_used normie = core_funcs.get_memory_usage()
assert_true(memory_used >= 1024)  // Should include previous allocation
print_test_summary()

test_start("core_funcs.get_available_memory available memory")
sus available normie = core_funcs.get_available_memory()
assert_true(available >= 0)
assert_true(available < 1048576)  // Should be less than max
print_test_summary()

test_start("core_funcs.track_memory_free memory deallocation")
core_funcs.clear_error()
sus free_result lit = core_funcs.track_memory_free(512)
sus free_error normie = core_funcs.get_last_error()
assert_eq_int(free_error, 0)
assert_true(free_result)
print_test_summary()

test_start("core_funcs.track_memory_alloc excessive allocation")
core_funcs.clear_error()
sus excessive_alloc lit = core_funcs.track_memory_alloc(2000000)  // Too large
sus excessive_error normie = core_funcs.get_last_error()
assert_true(excessive_error != 0)  // Should fail
assert_false(excessive_alloc)
print_test_summary()

// Test error handling and state management
test_start("core_funcs.clear_error error clearing")
core_funcs.track_memory_alloc(-1)  // Generate error
sus error_before normie = core_funcs.get_last_error()
assert_true(error_before != 0)
core_funcs.clear_error()
sus error_after normie = core_funcs.get_last_error()
assert_eq_int(error_after, 0)
print_test_summary()

// Test self-test functionality
test_start("core_funcs.self_test internal validation")
sus self_test_result lit = core_funcs.self_test()
assert_true(self_test_result)
print_test_summary()

// Test runtime diagnostics
test_start("core_funcs.get_runtime_stats statistics")
sus stats tea = core_funcs.get_runtime_stats()
assert_true(stats != cringe)
assert_true(stringz.contains(stats, "Runtime Stats"))
assert_true(stringz.contains(stats, "Ready="))
assert_true(stringz.contains(stats, "Memory="))
print_test_summary()

// Test runtime reset
test_start("core_funcs.reset_runtime system reset")
core_funcs.reset_runtime()
sus reset_error normie = core_funcs.get_last_error()
assert_eq_int(reset_error, 0)
sus reset_ready lit = core_funcs.is_runtime_ready()
assert_true(reset_ready)
sus reset_memory normie = core_funcs.get_memory_usage()
assert_eq_int(reset_memory, 0)
print_test_summary()

// Performance and stress tests
test_start("core_funcs.print performance test")
sus start_time normie = core_funcs.get_timestamp_ms()
sus i normie = 0
stan i < 100 {
    core_funcs.print("Performance test iteration")
    i++
}
sus end_time normie = core_funcs.get_timestamp_ms()
assert_true(end_time >= start_time)
print_test_summary()

test_start("core_funcs.number_to_string range test")
sus range_tests lit = based
sus test_nums normie[] = [0, 1, -1, 42, -42, 123, -123, 999, -999]
sus j normie = 0
stan j < 9 {
    sus test_str tea = core_funcs.number_to_string(test_nums[j])
    lowkey stringz.length(test_str) == 0 {
        range_tests = cap
    }
    j++
}
assert_true(range_tests)
print_test_summary()

test_start("core_funcs.float_to_string range test")
sus float_range_tests lit = based
sus test_floats drip[] = [0.0, 1.0, -1.0, 3.14, -3.14, 42.5, -42.5]
sus k normie = 0
stan k < 7 {
    sus float_test_str tea = core_funcs.float_to_string(test_floats[k])
    lowkey stringz.length(float_test_str) == 0 {
        float_range_tests = cap
    }
    k++
}
assert_true(float_range_tests)
print_test_summary()

// Edge case testing
test_start("core_funcs.print edge cases")
sus edge_test1 lit = core_funcs.print("")  # Empty string
assert_true(edge_test1)
sus edge_test2 lit = core_funcs.print("A")  # Single character
assert_true(edge_test2)
sus long_string tea = ""
sus m normie = 0
stan m < 50 {
    long_string = long_string + "test"
    m++
}
sus edge_test3 lit = core_funcs.print(long_string)  # Long string
assert_true(edge_test3)
print_test_summary()

test_start("core_funcs.string_to_number edge cases")
core_funcs.clear_error()
sus edge_zero normie = core_funcs.string_to_number("0")
assert_eq_int(core_funcs.get_last_error(), 0)
assert_eq_int(edge_zero, 0)

core_funcs.clear_error()
sus edge_negative normie = core_funcs.string_to_number("-0")
assert_eq_int(core_funcs.get_last_error(), 0)
assert_eq_int(edge_negative, 0)
print_test_summary()

// Final validation
test_start("core_funcs comprehensive validation")
sus final_validation lit = based

# Test that all major functions work together
sus validation_msg tea = "Validation test"
lowkey core_funcs.print(validation_msg) == cap {
    final_validation = cap
}

sus validation_timestamp tea = core_funcs.get_timestamp()
lowkey stringz.length(validation_timestamp) < 10 {
    final_validation = cap
}

sus validation_number tea = core_funcs.number_to_string(12345)
lowkey validation_number != "12345" {
    final_validation = cap
}

sus validation_stats tea = core_funcs.get_runtime_stats()
lowkey stringz.contains(validation_stats, "Runtime Stats") == cap {
    final_validation = cap
}

assert_true(final_validation)
print_test_summary()

vibez.spillln("All core functions tests completed successfully!")
vibez.spillln("Production-ready core runtime functions validated!")
