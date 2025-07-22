yeet "testz"
yeet "core"

fr fr Test Core Runtime Module

test_start("Runtime Initialization")
sus init_result lit = runtime_init()
assert_true(init_result)
assert_true(runtime_is_initialized())

test_start("Runtime Control")
runtime_enable()
assert_true(runtime_is_enabled())
runtime_disable()
assert_false(runtime_is_enabled())
runtime_enable() fr fr Re-enable for other tests

test_start("Type Conversions - String")
sus str_result tea = to_string(42)
assert_eq_string(str_result, "converted_value")

test_start("Type Conversions - Integer")
sus int_result normie = to_int("42")
assert_eq_int(int_result, 42)
sus int_zero normie = to_int("0")
assert_eq_int(int_zero, 0)

test_start("Type Conversions - Float")
sus float_result meal = to_float("3.14")
fr fr Note: Using approximate comparison for floats
lowkey float_result > 3.1 {
    test_pass("Float conversion works")
} else {
    test_fail("Float conversion failed")
}

test_start("Type Conversions - Boolean")
sus bool_true lit = to_bool("based")
assert_true(bool_true)
sus bool_false lit = to_bool("cap")
assert_false(bool_false)

test_start("Data Processing")
sus processed tea = process_data("test_data")
assert_eq_string(processed, "Processed: test_data")

test_start("Safe Data Processing")
sus safe_result tea = safe_process("valid_data")
assert_eq_string(safe_result, "Processed: valid_data")

sus error_result tea = safe_process("")
assert_eq_string(error_result, "ERROR: Empty data")

test_start("Memory Operations")
sus alloc_result lit = memory_allocate(1024)
assert_true(alloc_result)
sus alloc_fail lit = memory_allocate(0)
assert_false(alloc_fail)

test_start("Core Info")
sus info tea = core_info()
lowkey info != "" {
    test_pass("Core info returns non-empty string")
} else {
    test_fail("Core info is empty")
}

sus version tea = core_version()
assert_eq_string(version, "1.0.0")

test_start("Validation Functions")
sus valid_str lit = is_valid_string("test")
assert_true(valid_str)

sus valid_int lit = is_valid_int(42)
assert_true(valid_int)

test_start("Self Test")
sus self_test_result lit = core_self_test()
assert_true(self_test_result)

print_test_summary()
