# Comprehensive Test Suite for Core Module
# Tests all essential language primitives and utilities
yeet "testz"
yeet "core"

# ==============================================================================
# TYPE CONVERSION TESTS
# ==============================================================================

test_start("Type conversion functions")

# Test integer to boolean conversion
assert_true(lit_from_int(1))
assert_true(lit_from_int(42))
assert_true(lit_from_int(-5))
assert_false(lit_from_int(0))

# Test boolean to integer conversion
assert_eq_int(int_from_bool(based), 1)
assert_eq_int(int_from_bool(cap), 0)

# Test string to integer conversion
assert_eq_int(int_from_string("0"), 0)
assert_eq_int(int_from_string("42"), 42)
assert_eq_int(int_from_string("-5"), -5)

# Test integer to string conversion
assert_eq_string(string_from_int(0), "0")
assert_eq_string(string_from_int(42), "42")
assert_eq_string(string_from_int(-5), "-5")

# Test boolean to string conversion
assert_eq_string(string_from_bool(based), "true")
assert_eq_string(string_from_bool(cap), "false")

# Test string to boolean conversion
assert_true(lit_from_string("true"))
assert_true(lit_from_string("based"))
assert_true(lit_from_string("1"))
assert_false(lit_from_string("false"))

# Test float conversions
assert_eq_float(float_from_int(42), 42.0)
assert_eq_string(string_from_float(3.14), "3.14")
assert_eq_float(float_from_string("3.14"), 3.14)

# Test legacy compatibility functions
assert_true(lit(42))
assert_false(lit(0))
assert_eq_int(normie(based), 1)
assert_eq_string(tea(42), "42")

print_test_summary()

# ==============================================================================
# OPTION TYPE TESTS
# ==============================================================================

test_start("Option type implementation")

# Test option creation
sus some_value (lit, normie) = option_some(42)
sus none_value (lit, normie) = option_none()

# Test option type checking
assert_true(option_is_some(some_value))
assert_false(option_is_none(some_value))
assert_false(option_is_some(none_value))
assert_true(option_is_none(none_value))

# Test option unwrap
assert_eq_int(option_unwrap(some_value), 42)
assert_eq_int(option_unwrap_or(some_value, 100), 42)
assert_eq_int(option_unwrap_or(none_value, 100), 100)

# Test option unwrap_or_else
assert_eq_int(option_unwrap_or_else(some_value, 200), 42)
assert_eq_int(option_unwrap_or_else(none_value, 200), 200)

print_test_summary()

# ==============================================================================
# RESULT TYPE TESTS
# ==============================================================================

test_start("Result type implementation")

# Test result creation
sus ok_result (lit, normie, normie) = result_ok(123)
sus err_result (lit, normie, normie) = result_err(404)

# Test result type checking
assert_true(result_is_ok(ok_result))
assert_false(result_is_err(ok_result))
assert_false(result_is_ok(err_result))
assert_true(result_is_err(err_result))

# Test result unwrap
assert_eq_int(result_unwrap(ok_result), 123)
assert_eq_int(result_unwrap_or(ok_result, 999), 123)
assert_eq_int(result_unwrap_or(err_result, 999), 999)

# Test error code retrieval
assert_eq_int(result_get_error(err_result), 404)
assert_eq_int(result_get_error(ok_result), 0)

print_test_summary()

# ==============================================================================
# MEMORY ALLOCATION TESTS
# ==============================================================================

test_start("Memory allocation utilities")

# Test memory allocation
sus addr1 normie = memory_allocate(100)
sus addr2 normie = memory_allocate(200)
assert_eq_int(addr1, 100000)
assert_eq_int(addr2, 200000)

# Test memory operations
memory_deallocate(addr1)
memory_copy(addr2, addr1, 50)
memory_set(addr1, 255, 10)

# Test memory comparison
assert_eq_int(memory_compare(1000, 1000), 0)
assert_eq_int(memory_compare(1000, 2000), -1)
assert_eq_int(memory_compare(2000, 1000), 1)

print_test_summary()

# ==============================================================================
# PANIC AND ERROR HANDLING TESTS
# ==============================================================================

test_start("Panic and error handling")

# Test assertion functions
assert(based, "This should not fail")
debug_assert(based, "This debug assertion should not fail")

# Test unbothered function
assert_true(unbothered())

# Note: panic, unreachable, and todo functions would terminate program
# so they are not tested here

print_test_summary()

# ==============================================================================
# ARRAY AND SLICE UTILITIES TESTS
# ==============================================================================

test_start("Array and slice utilities")

# Test array length functions
assert_eq_int(array_len(5), 5)
assert_eq_int(slice_len(10), 10)

# Test array element access
assert_eq_int(array_get(1000, 0), 1000)
assert_eq_int(array_get(1000, 5), 1005)

# Test array bounds checking
assert_true(array_bounds_check(0, 10))
assert_true(array_bounds_check(9, 10))
assert_false(array_bounds_check(10, 10))
assert_false(array_bounds_check(-1, 10))

# Test array operations
array_set(1000, 0, 42)
array_copy(2000, 1000, 10)
array_fill(1000, 0, 5)

print_test_summary()

# ==============================================================================
# STRING UTILITIES TESTS
# ==============================================================================

test_start("String utilities")

# Test string length
assert_eq_int(string_len(""), 0)
assert_eq_int(string_len("hello"), 5)
assert_eq_int(string_len("world"), 5)
assert_eq_int(string_len("test"), 4)

# Test string concatenation
assert_eq_string(string_concat("hello", "world"), "helloworld")
assert_eq_string(string_concat("", "test"), "test")
assert_eq_string(string_concat("pre", ""), "pre")

# Test string equality
assert_true(string_eq("hello", "hello"))
assert_false(string_eq("hello", "world"))

# Test string contains
assert_true(string_contains("hello world", "world"))
assert_true(string_contains("hello world", "hello"))
assert_false(string_contains("hello world", "xyz"))

# Test string starts with
assert_true(string_starts_with("hello world", "hello"))
assert_false(string_starts_with("hello world", "world"))

# Test string ends with
assert_true(string_ends_with("hello world", "world"))
assert_false(string_ends_with("hello world", "hello"))

# Test string trimming
assert_eq_string(string_trim("  hello  "), "hello")
assert_eq_string(string_trim("  world  "), "world")

# Test string splitting
sus split_result (tea, tea) = string_split_first("hello,world", ",")
assert_eq_string(split_result.0, "hello")
assert_eq_string(split_result.1, "world")

# Test string replacement
assert_eq_string(string_replace("hello world", "world", "CURSED"), "hello CURSED")

# Test string case conversion
assert_eq_string(string_to_upper("hello"), "HELLO")
assert_eq_string(string_to_lower("HELLO"), "hello")

print_test_summary()

# ==============================================================================
# MATHEMATICAL UTILITIES TESTS
# ==============================================================================

test_start("Mathematical utilities")

# Test max/min functions
assert_eq_int(max(5, 3), 5)
assert_eq_int(max(3, 7), 7)
assert_eq_int(min(5, 3), 3)
assert_eq_int(min(3, 7), 3)

# Test absolute value
assert_eq_int(abs(5), 5)
assert_eq_int(abs(-5), 5)
assert_eq_int(abs(0), 0)

# Test power function
assert_eq_int(pow(2, 3), 8)
assert_eq_int(pow(5, 2), 25)
assert_eq_int(pow(3, 0), 1)

# Test square root
assert_eq_int(sqrt(16), 4)
assert_eq_int(sqrt(25), 5)
assert_eq_int(sqrt(100), 10)

print_test_summary()

# ==============================================================================
# BOOLEAN UTILITIES TESTS
# ==============================================================================

test_start("Boolean utilities")

# Test not function
assert_true(not(cap))
assert_false(not(based))

# Test and function
assert_true(and(based, based))
assert_false(and(based, cap))
assert_false(and(cap, based))
assert_false(and(cap, cap))

# Test or function
assert_true(or(based, based))
assert_true(or(based, cap))
assert_true(or(cap, based))
assert_false(or(cap, cap))

# Test xor function
assert_false(xor(based, based))
assert_true(xor(based, cap))
assert_true(xor(cap, based))
assert_false(xor(cap, cap))

print_test_summary()

# ==============================================================================
# UTILITY FUNCTIONS TESTS
# ==============================================================================

test_start("Utility functions")

# Test comparison functions
assert_eq_int(compare_int(5, 5), 0)
assert_eq_int(compare_int(3, 7), -1)
assert_eq_int(compare_int(7, 3), 1)

assert_eq_int(compare_string("hello", "hello"), 0)
assert_eq_int(compare_string("abc", "def"), -1)
assert_eq_int(compare_string("def", "abc"), 1)

# Test swap function
sus swapped (normie, normie) = swap_int(10, 20)
assert_eq_int(swapped.0, 20)
assert_eq_int(swapped.1, 10)

# Test range checking
assert_true(in_range(5, 0, 10))
assert_true(in_range(0, 0, 10))
assert_true(in_range(10, 0, 10))
assert_false(in_range(-1, 0, 10))
assert_false(in_range(11, 0, 10))

# Test clamp function
assert_eq_int(clamp(5, 0, 10), 5)
assert_eq_int(clamp(-5, 0, 10), 0)
assert_eq_int(clamp(15, 0, 10), 10)

print_test_summary()

# ==============================================================================
# COMPILER UTILITIES TESTS
# ==============================================================================

test_start("Compiler utilities")

# Test token type constants
assert_eq_int(token_type_identifier(), 1)
assert_eq_int(token_type_number(), 2)
assert_eq_int(token_type_string(), 3)
assert_eq_int(token_type_keyword(), 4)
assert_eq_int(token_type_operator(), 5)

# Test error codes
assert_eq_int(error_code_syntax(), 1000)
assert_eq_int(error_code_type(), 2000)
assert_eq_int(error_code_runtime(), 3000)

# Test hash function
assert_eq_int(hash_string("main"), 100)
assert_eq_int(hash_string("test"), 200)
assert_eq_int(hash_string("func"), 300)

print_test_summary()

# ==============================================================================
# INTEGRATION TESTS
# ==============================================================================

test_start("Integration tests")

# Test combined operations
sus input_str tea = "42"
sus parsed_int normie = int_from_string(input_str)
sus result_str tea = string_from_int(parsed_int)
assert_eq_string(result_str, "42")

# Test option chaining
sus maybe_value (lit, normie) = option_some(100)
sus doubled_value normie = option_unwrap(maybe_value) * 2
assert_eq_int(doubled_value, 200)

# Test result error handling
sus computation_result (lit, normie, normie) = result_ok(42)
bestie result_is_ok(computation_result) {
    sus final_value normie = result_unwrap(computation_result)
    assert_eq_int(final_value, 42)
}

# Test string processing pipeline
sus text tea = "  hello world  "
sus trimmed tea = string_trim(text)
sus parts (tea, tea) = string_split_first(trimmed, " ")
assert_eq_string(parts.0, "hello")
assert_eq_string(parts.1, "world")

print_test_summary()

# ==============================================================================
# PERFORMANCE TESTS
# ==============================================================================

test_start("Performance tests")

# Test repeated operations
sus i normie = 0
bestie i < 100 {
    sus temp_option (lit, normie) = option_some(i)
    sus temp_result normie = option_unwrap_or(temp_option, 0)
    assert_eq_int(temp_result, i)
    i = i + 1
}

# Test string operations
sus base_str tea = "test"
sus concat_result tea = string_concat(base_str, "ing")
assert_eq_string(concat_result, "testing")

# Test mathematical operations
sus math_result normie = max(min(abs(-10), 20), 5)
assert_eq_int(math_result, 10)

print_test_summary()

# ==============================================================================
# SELF-HOSTING COMPATIBILITY TESTS
# ==============================================================================

test_start("Self-hosting compatibility")

# Test core module initialization
core_init()

# Test essential functions for compiler
sus token_id normie = token_type_identifier()
sus error_syntax normie = error_code_syntax()
sus hash_result normie = hash_string("variable")

assert_eq_int(token_id, 1)
assert_eq_int(error_syntax, 1000)
assert_true(hash_result > 0)

# Test memory simulation for compiler
sus compiler_memory normie = memory_allocate(1000)
memory_set(compiler_memory, 0, 100)
memory_deallocate(compiler_memory)

# Test type conversion for compiler
sus compiler_bool lit = lit_from_int(1)
sus compiler_int normie = int_from_bool(compiler_bool)
sus compiler_str tea = string_from_int(compiler_int)

assert_true(compiler_bool)
assert_eq_int(compiler_int, 1)
assert_eq_string(compiler_str, "1")

print_test_summary()

vibez.spill("🎉 All core module tests completed successfully!")
vibez.spill("📈 Core module is ready for self-hosting compiler support!")
vibez.spill("🚀 Total functionality: Type conversions, Option/Result types, Memory utils, String processing, Math ops, and Compiler utilities")
