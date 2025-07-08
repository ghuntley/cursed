# Core Module Test Suite
# Comprehensive tests for essential built-in functions

yeet "testz"
yeet "core"

# Test Type Conversions
slay test_string_to_int() {
    test_start("string_to_int")
    assert_eq_int(core.string_to_int("42"), 42)
    assert_eq_int(core.string_to_int("0"), 0)
    assert_eq_int(core.string_to_int("-15"), -15)
    assert_eq_int(core.string_to_int("123"), 123)
    assert_eq_int(core.string_to_int("invalid"), 0)
    assert_eq_int(core.string_to_int(""), 0)
    assert_eq_int(core.string_to_int("+25"), 25)
    assert_eq_int(core.string_to_int("-0"), 0)
}

slay test_int_to_string() {
    test_start("int_to_string")
    assert_eq_string(core.int_to_string(42), "42")
    assert_eq_string(core.int_to_string(0), "0")
    assert_eq_string(core.int_to_string(-15), "-15")
    assert_eq_string(core.int_to_string(123), "123")
    assert_eq_string(core.int_to_string(1000), "1000")
    assert_eq_string(core.int_to_string(-999), "-999")
}

slay test_bool_to_string() {
    test_start("bool_to_string")
    assert_eq_string(core.bool_to_string(based), "based")
    assert_eq_string(core.bool_to_string(cap), "cap")
}

slay test_string_to_bool() {
    test_start("string_to_bool")
    assert_eq_bool(core.string_to_bool("based"), based)
    assert_eq_bool(core.string_to_bool("true"), based)
    assert_eq_bool(core.string_to_bool("1"), based)
    assert_eq_bool(core.string_to_bool("cap"), cap)
    assert_eq_bool(core.string_to_bool("false"), cap)
    assert_eq_bool(core.string_to_bool("0"), cap)
    assert_eq_bool(core.string_to_bool(""), cap)
    assert_eq_bool(core.string_to_bool("invalid"), cap)
}

slay test_float_to_string() {
    test_start("float_to_string")
    assert_eq_string(core.float_to_string(42.0), "42.0")
    assert_eq_string(core.float_to_string(0.0), "0.0")
    assert_eq_string(core.float_to_string(3.14), "3.14")
    assert_eq_string(core.float_to_string(-2.5), "-2.5")
}

slay test_string_to_float() {
    test_start("string_to_float")
    assert_eq_float(core.string_to_float("42.0"), 42.0)
    assert_eq_float(core.string_to_float("0.0"), 0.0)
    assert_eq_float(core.string_to_float("3.14"), 3.14)
    assert_eq_float(core.string_to_float("-2.5"), -2.5)
    assert_eq_float(core.string_to_float("42"), 42.0)
}

# Test Mathematical Operations
slay test_abs() {
    test_start("abs")
    assert_eq_int(core.abs(42), 42)
    assert_eq_int(core.abs(-42), 42)
    assert_eq_int(core.abs(0), 0)
    assert_eq_int(core.abs(1), 1)
    assert_eq_int(core.abs(-1), 1)
}

slay test_abs_float() {
    test_start("abs_float")
    assert_eq_float(core.abs_float(42.5), 42.5)
    assert_eq_float(core.abs_float(-42.5), 42.5)
    assert_eq_float(core.abs_float(0.0), 0.0)
    assert_eq_float(core.abs_float(1.5), 1.5)
    assert_eq_float(core.abs_float(-1.5), 1.5)
}

slay test_min_max() {
    test_start("min_max")
    assert_eq_int(core.min(5, 3), 3)
    assert_eq_int(core.min(3, 5), 3)
    assert_eq_int(core.min(5, 5), 5)
    assert_eq_int(core.min(-2, -5), -5)
    
    assert_eq_int(core.max(5, 3), 5)
    assert_eq_int(core.max(3, 5), 5)
    assert_eq_int(core.max(5, 5), 5)
    assert_eq_int(core.max(-2, -5), -2)
}

slay test_min_max_float() {
    test_start("min_max_float")
    assert_eq_float(core.min_float(5.5, 3.3), 3.3)
    assert_eq_float(core.min_float(3.3, 5.5), 3.3)
    assert_eq_float(core.min_float(5.5, 5.5), 5.5)
    assert_eq_float(core.min_float(-2.2, -5.5), -5.5)
    
    assert_eq_float(core.max_float(5.5, 3.3), 5.5)
    assert_eq_float(core.max_float(3.3, 5.5), 5.5)
    assert_eq_float(core.max_float(5.5, 5.5), 5.5)
    assert_eq_float(core.max_float(-2.2, -5.5), -2.2)
}

slay test_clamp() {
    test_start("clamp")
    assert_eq_int(core.clamp(5, 0, 10), 5)
    assert_eq_int(core.clamp(-5, 0, 10), 0)
    assert_eq_int(core.clamp(15, 0, 10), 10)
    assert_eq_int(core.clamp(7, 5, 8), 7)
    assert_eq_int(core.clamp(3, 5, 8), 5)
    assert_eq_int(core.clamp(10, 5, 8), 8)
}

slay test_clamp_float() {
    test_start("clamp_float")
    assert_eq_float(core.clamp_float(5.5, 0.0, 10.0), 5.5)
    assert_eq_float(core.clamp_float(-5.5, 0.0, 10.0), 0.0)
    assert_eq_float(core.clamp_float(15.5, 0.0, 10.0), 10.0)
    assert_eq_float(core.clamp_float(7.5, 5.0, 8.0), 7.5)
    assert_eq_float(core.clamp_float(3.5, 5.0, 8.0), 5.0)
    assert_eq_float(core.clamp_float(10.5, 5.0, 8.0), 8.0)
}

# Test Error Handling
slay test_assert_success() {
    test_start("assert_success")
    core.assert(based, "This should not panic")
    assert_true(based)  # If we get here, assert worked
}

slay test_expect() {
    test_start("expect")
    assert_true(core.expect(based, "This should succeed"))
    assert_false(core.expect(cap, "This should fail"))
}

# Test Memory Utilities
slay test_size_of() {
    test_start("size_of")
    assert_eq_int(core.size_of_int(), 4)
    assert_eq_int(core.size_of_float(), 8)
    assert_eq_int(core.size_of_bool(), 1)
}

# Test String Utilities
slay test_len_string() {
    test_start("len_string")
    assert_eq_int(core.len_string("hello"), 5)
    assert_eq_int(core.len_string(""), 0)
    assert_eq_int(core.len_string("a"), 1)
    assert_eq_int(core.len_string("CURSED"), 6)
}

slay test_string_concat() {
    test_start("string_concat")
    assert_eq_string(core.string_concat("hello", "world"), "helloworld")
    assert_eq_string(core.string_concat("", "test"), "test")
    assert_eq_string(core.string_concat("test", ""), "test")
    assert_eq_string(core.string_concat("", ""), "")
}

slay test_string_equals() {
    test_start("string_equals")
    assert_true(core.string_equals("hello", "hello"))
    assert_false(core.string_equals("hello", "world"))
    assert_true(core.string_equals("", ""))
    assert_false(core.string_equals("a", ""))
}

# Test Utility Functions
slay test_swap_int() {
    test_start("swap_int")
    (sus a normie, sus b normie) = core.swap_int(5, 3)
    assert_eq_int(a, 3)
    assert_eq_int(b, 5)
    
    (sus x normie, sus y normie) = core.swap_int(10, 20)
    assert_eq_int(x, 20)
    assert_eq_int(y, 10)
}

slay test_swap_float() {
    test_start("swap_float")
    (sus a meal, sus b meal) = core.swap_float(5.5, 3.3)
    assert_eq_float(a, 3.3)
    assert_eq_float(b, 5.5)
    
    (sus x meal, sus y meal) = core.swap_float(10.1, 20.2)
    assert_eq_float(x, 20.2)
    assert_eq_float(y, 10.1)
}

slay test_swap_string() {
    test_start("swap_string")
    (sus a tea, sus b tea) = core.swap_string("hello", "world")
    assert_eq_string(a, "world")
    assert_eq_string(b, "hello")
    
    (sus x tea, sus y tea) = core.swap_string("foo", "bar")
    assert_eq_string(x, "bar")
    assert_eq_string(y, "foo")
}

# Test Comparison Functions
slay test_compare_int() {
    test_start("compare_int")
    assert_eq_int(core.compare_int(5, 3), 1)
    assert_eq_int(core.compare_int(3, 5), -1)
    assert_eq_int(core.compare_int(5, 5), 0)
    assert_eq_int(core.compare_int(-2, -5), 1)
    assert_eq_int(core.compare_int(-5, -2), -1)
}

slay test_compare_float() {
    test_start("compare_float")
    assert_eq_int(core.compare_float(5.5, 3.3), 1)
    assert_eq_int(core.compare_float(3.3, 5.5), -1)
    assert_eq_int(core.compare_float(5.5, 5.5), 0)
    assert_eq_int(core.compare_float(-2.2, -5.5), 1)
    assert_eq_int(core.compare_float(-5.5, -2.2), -1)
}

# Test Range Functions
slay test_in_range() {
    test_start("in_range")
    assert_true(core.in_range(5, 0, 10))
    assert_true(core.in_range(0, 0, 10))
    assert_true(core.in_range(10, 0, 10))
    assert_false(core.in_range(-1, 0, 10))
    assert_false(core.in_range(11, 0, 10))
    assert_true(core.in_range(7, 5, 8))
    assert_false(core.in_range(4, 5, 8))
    assert_false(core.in_range(9, 5, 8))
}

slay test_in_range_float() {
    test_start("in_range_float")
    assert_true(core.in_range_float(5.5, 0.0, 10.0))
    assert_true(core.in_range_float(0.0, 0.0, 10.0))
    assert_true(core.in_range_float(10.0, 0.0, 10.0))
    assert_false(core.in_range_float(-1.0, 0.0, 10.0))
    assert_false(core.in_range_float(11.0, 0.0, 10.0))
    assert_true(core.in_range_float(7.5, 5.0, 8.0))
    assert_false(core.in_range_float(4.0, 5.0, 8.0))
    assert_false(core.in_range_float(9.0, 5.0, 8.0))
}

# Test Default Values
slay test_default_values() {
    test_start("default_values")
    assert_eq_int(core.default_int(), 0)
    assert_eq_float(core.default_float(), 0.0)
    assert_eq_bool(core.default_bool(), cap)
    assert_eq_string(core.default_string(), "")
}

# Run all tests
slay main() {
    vibez.spill("Running Core Module Tests...")
    
    # Type Conversion Tests
    test_string_to_int()
    test_int_to_string()
    test_bool_to_string()
    test_string_to_bool()
    test_float_to_string()
    test_string_to_float()
    
    # Mathematical Operation Tests
    test_abs()
    test_abs_float()
    test_min_max()
    test_min_max_float()
    test_clamp()
    test_clamp_float()
    
    # Error Handling Tests
    test_assert_success()
    test_expect()
    
    # Memory Utility Tests
    test_size_of()
    
    # String Utility Tests
    test_len_string()
    test_string_concat()
    test_string_equals()
    
    # Utility Function Tests
    test_swap_int()
    test_swap_float()
    test_swap_string()
    
    # Comparison Function Tests
    test_compare_int()
    test_compare_float()
    
    # Range Function Tests
    test_in_range()
    test_in_range_float()
    
    # Default Value Tests
    test_default_values()
    
    # Print test summary
    print_test_summary()
    
    vibez.spill("Core Module Tests Complete!")
}
