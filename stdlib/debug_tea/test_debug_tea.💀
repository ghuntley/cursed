yeet "testz"
yeet "debug_tea"

fr fr Debug Tea Module - Comprehensive Test Suite

fr fr Test debug enable/disable functionality
test_start("debug_enable_disable_test")
disable_debug()
assert_false(is_debug_enabled())
enable_debug()
assert_true(is_debug_enabled())

fr fr Test debug level management
test_start("debug_level_test")
set_debug_level(DEBUG_LEVEL_ERROR)
assert_eq_int(get_debug_level(), DEBUG_LEVEL_ERROR)
set_debug_level(DEBUG_LEVEL_DEBUG)
assert_eq_int(get_debug_level(), DEBUG_LEVEL_DEBUG)

fr fr Test debug output functions
test_start("debug_output_test")
enable_debug()
debug_error("Test error message")
debug_warn("Test warning message")
debug_info("Test info message")
debug_trace("Test trace message")

fr fr Test variable inspection
test_start("variable_inspection_test")
sus test_var tea = "test_value"
sus test_int normie = 42
sus test_bool lit = based
sus test_float meal = 3.14

inspect_var("test_var", test_var)
inspect_int("test_int", test_int)
inspect_bool("test_bool", test_bool)
inspect_float("test_float", test_float)

fr fr Test debug assertions (should pass)
test_start("debug_assertions_pass_test")
debug_assert_true(based, "This should pass")
debug_assert_false(cap, "This should pass")
debug_assert_eq_int(42, 42, "Integer equality test")
debug_assert_eq_string("hello", "hello", "String equality test")

fr fr Test stack trace utilities
test_start("stack_trace_test")
debug_print_stack_trace()
debug_print_call_stack("test_function")
debug_print_return_stack("test_function")

fr fr Test timer functions
test_start("timer_test")
debug_start_timer("test_operation")
debug_end_timer("test_operation")

fr fr Test memory inspection
test_start("memory_inspection_test")
debug_print_memory_usage()

fr fr Test validation helpers
test_start("validation_helpers_test")
debug_validate_not_nil("valid_string", "test_string")
debug_validate_range_int(50, 0, 100, "test_range")
debug_validate_positive_int(25, "test_positive")

fr fr Test debug configuration
test_start("debug_config_test")
debug_print_config()

fr fr Test hex and binary inspection
test_start("hex_binary_inspection_test")
debug_print_hex(255, "max_byte")
debug_print_binary(15, "nibble")

fr fr Test breakpoint simulation
test_start("breakpoint_test")
debug_breakpoint("Test checkpoint reached")

fr fr Test section isolation
test_start("test_section_isolation")
debug_test_section("Unit Tests")
debug_test_section("Integration Tests")

fr fr Test debug levels filtering
test_start("debug_level_filtering_test")
set_debug_level(DEBUG_LEVEL_ERROR)
debug_error("This should appear")
debug_info("This should be filtered out")
debug_trace("This should be filtered out")

set_debug_level(DEBUG_LEVEL_TRACE)
debug_error("This should appear")
debug_info("This should appear")
debug_trace("This should appear")

fr fr Test debug with disabled state
test_start("debug_disabled_test")
disable_debug()
debug_error("This should not appear")
debug_assert_true(cap, "This assertion should be ignored")
inspect_var("hidden_var", "hidden_value")

fr fr Re-enable for final tests
enable_debug()
set_debug_level(DEBUG_LEVEL_INFO)

fr fr Test comprehensive debug scenario
test_start("comprehensive_debug_scenario")
debug_test_section("Comprehensive Debug Test")

sus calculation_input normie = 10
sus calculation_result normie = 0

debug_print_call_stack("calculation_function")
inspect_int("input", calculation_input)

debug_validate_positive_int(calculation_input, "calculation_input")

calculation_result = calculation_input * 2
inspect_int("result", calculation_result)

debug_assert_eq_int(calculation_result, 20, "Calculation result check")

debug_print_return_stack("calculation_function")

fr fr Test debug levels constants
test_start("debug_levels_constants_test")
assert_eq_int(DEBUG_LEVEL_NONE, 0)
assert_eq_int(DEBUG_LEVEL_ERROR, 1)
assert_eq_int(DEBUG_LEVEL_WARN, 2)
assert_eq_int(DEBUG_LEVEL_INFO, 3)
assert_eq_int(DEBUG_LEVEL_DEBUG, 4)
assert_eq_int(DEBUG_LEVEL_TRACE, 5)

fr fr Test debug utilities with various data types
test_start("debug_various_types_test")
sus char_test sip = 'A'
sus small_int smol = 127
sus large_int thicc = 1000000

fr fr Test with different numeric types
inspect_int("small_int", small_int)
inspect_int("large_int", large_int)

fr fr Test boolean inspection variations
sus flag_true lit = based
sus flag_false lit = cap
inspect_bool("flag_true", flag_true)
inspect_bool("flag_false", flag_false)

fr fr Test validation edge cases
test_start("validation_edge_cases_test")
debug_validate_range_int(0, 0, 100, "boundary_min")
debug_validate_range_int(100, 0, 100, "boundary_max")
debug_validate_positive_int(1, "minimum_positive")

fr fr Test debug assertions with complex expressions
test_start("complex_assertions_test")
sus x normie = 5
sus y normie = 10
debug_assert_true(x < y, "x should be less than y")
debug_assert_eq_int(x + y, 15, "Sum should equal 15")

fr fr Test debug summary
test_start("debug_summary_test")
debug_print_summary()

fr fr Print test results
print_test_summary()
