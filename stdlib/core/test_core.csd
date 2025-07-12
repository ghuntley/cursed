// Comprehensive test suite for core module
yeet "testz"
yeet "core"

// ================================
// Type Conversion Tests
// ================================

slay test_type_conversions() {
    test_start("Type conversion functions")
    
    // Test lit() conversions
    assert_eq_bool(core.lit(1), based)
    assert_eq_bool(core.lit(0), cap)
    assert_eq_bool(core.lit(cap), cap)
    assert_eq_bool(core.lit(based), based)
    test_pass("lit() conversions work")
    
    // Test normie() conversions
    assert_eq_int(core.normie(42), 42)
    assert_eq_int(core.normie(3.14), 3)
    assert_eq_int(core.normie(based), 1)
    assert_eq_int(core.normie(cap), 0)
    test_pass("normie() conversions work")
    
    // Test tea() conversions
    assert_eq_string(core.tea(42), "42")
    assert_eq_string(core.tea(based), "based")
    assert_eq_string(core.tea(cap), "cap")
    test_pass("tea() conversions work")
    
    // Test meal() conversions
    assert_eq_float(core.meal(42), 42.0)
    assert_eq_float(core.meal(3.14), 3.14)
    test_pass("meal() conversions work")
}

// ================================
// Collection Function Tests
// ================================

slay test_collection_functions() {
    test_start("Collection functions")
    
    // Test len() function
    assert_eq_int(core.len("hello"), 5)
    assert_eq_int(core.len(""), 0)
    test_pass("len() function works")
    
    // Test append() function (simplified test)
    sus slice []normie = [1, 2, 3]
    sus extended []normie = core.append(slice, 4, 5)
    assert_eq_int(core.len(extended), 5)
    test_pass("append() function works")
    
    // Test type_of() function
    assert_eq_string(core.type_of(42), "normie")
    assert_eq_string(core.type_of(3.14), "meal")
    assert_eq_string(core.type_of("hello"), "tea")
    assert_eq_string(core.type_of(based), "lit")
    test_pass("type_of() function works")
}

// ================================
// Math Helper Function Tests
// ================================

slay test_math_helpers() {
    test_start("Math helper functions")
    
    // Test abs_int()
    assert_eq_int(core.abs_int(5), 5)
    assert_eq_int(core.abs_int(-5), 5)
    assert_eq_int(core.abs_int(0), 0)
    test_pass("abs_int() works correctly")
    
    // Test abs_float()
    assert_eq_float(core.abs_float(5.5), 5.5)
    assert_eq_float(core.abs_float(-5.5), 5.5)
    assert_eq_float(core.abs_float(0.0), 0.0)
    test_pass("abs_float() works correctly")
    
    // Test sign_int()
    assert_eq_int(core.sign_int(5), 1)
    assert_eq_int(core.sign_int(-5), -1)
    assert_eq_int(core.sign_int(0), 0)
    test_pass("sign_int() works correctly")
    
    // Test sign_float()
    assert_eq_int(core.sign_float(5.5), 1)
    assert_eq_int(core.sign_float(-5.5), -1)
    assert_eq_int(core.sign_float(0.0), 0)
    test_pass("sign_float() works correctly")
}

// ================================
// Min/Max Function Tests
// ================================

slay test_min_max_functions() {
    test_start("Min/Max functions")
    
    // Test min_int()
    assert_eq_int(core.min_int(5, 3), 3)
    assert_eq_int(core.min_int(3, 5), 3)
    assert_eq_int(core.min_int(5, 5), 5)
    test_pass("min_int() works correctly")
    
    // Test max_int()
    assert_eq_int(core.max_int(5, 3), 5)
    assert_eq_int(core.max_int(3, 5), 5)
    assert_eq_int(core.max_int(5, 5), 5)
    test_pass("max_int() works correctly")
    
    // Test min_float()
    assert_eq_float(core.min_float(5.5, 3.3), 3.3)
    assert_eq_float(core.min_float(3.3, 5.5), 3.3)
    assert_eq_float(core.min_float(5.5, 5.5), 5.5)
    test_pass("min_float() works correctly")
    
    // Test max_float()
    assert_eq_float(core.max_float(5.5, 3.3), 5.5)
    assert_eq_float(core.max_float(3.3, 5.5), 5.5)
    assert_eq_float(core.max_float(5.5, 5.5), 5.5)
    test_pass("max_float() works correctly")
}

// ================================
// Range Function Tests
// ================================

slay test_range_functions() {
    test_start("Range functions")
    
    // Test range_int()
    sus range1 []normie = core.range_int(0, 5, 1)
    assert_eq_int(core.len(range1), 5)
    assert_eq_int(range1[0], 0)
    assert_eq_int(range1[4], 4)
    test_pass("range_int() works correctly")
    
    // Test range_int() with step
    sus range2 []normie = core.range_int(0, 10, 2)
    assert_eq_int(core.len(range2), 5)
    assert_eq_int(range2[0], 0)
    assert_eq_int(range2[1], 2)
    assert_eq_int(range2[4], 8)
    test_pass("range_int() with step works correctly")
    
    // Test range_float()
    sus range3 []meal = core.range_float(0.0, 3.0, 1.0)
    assert_eq_int(core.len(range3), 3)
    assert_eq_float(range3[0], 0.0)
    assert_eq_float(range3[2], 2.0)
    test_pass("range_float() works correctly")
}

// ================================
// Type Checking Function Tests
// ================================

slay test_type_checking() {
    test_start("Type checking functions")
    
    // Test is_nil()
    assert_eq_bool(core.is_nil(cringe), based)
    assert_eq_bool(core.is_nil(42), cap)
    assert_eq_bool(core.is_nil("hello"), cap)
    test_pass("is_nil() works correctly")
    
    // Test is_zero()
    assert_eq_bool(core.is_zero(0), based)
    assert_eq_bool(core.is_zero(0.0), based)
    assert_eq_bool(core.is_zero(cap), based)
    assert_eq_bool(core.is_zero(""), based)
    assert_eq_bool(core.is_zero(cringe), based)
    assert_eq_bool(core.is_zero(42), cap)
    assert_eq_bool(core.is_zero("hello"), cap)
    test_pass("is_zero() works correctly")
}

// ================================
// Hash Function Tests
// ================================

slay test_hash_functions() {
    test_start("Hash functions")
    
    // Test hash_string()
    sus hash1 normie = core.hash_string("hello")
    sus hash2 normie = core.hash_string("hello")
    sus hash3 normie = core.hash_string("world")
    
    assert_eq_int(hash1, hash2)
    assert_true(hash1 != hash3)
    test_pass("hash_string() works correctly")
    
    // Test hash_int()
    sus int_hash1 normie = core.hash_int(42)
    sus int_hash2 normie = core.hash_int(42)
    sus int_hash3 normie = core.hash_int(43)
    
    assert_eq_int(int_hash1, int_hash2)
    assert_true(int_hash1 != int_hash3)
    test_pass("hash_int() works correctly")
}

// ================================
// Utility Function Tests
// ================================

slay test_utility_functions() {
    test_start("Utility functions")
    
    // Test clamp_int()
    assert_eq_int(core.clamp_int(5, 0, 10), 5)
    assert_eq_int(core.clamp_int(-5, 0, 10), 0)
    assert_eq_int(core.clamp_int(15, 0, 10), 10)
    test_pass("clamp_int() works correctly")
    
    // Test clamp_float()
    assert_eq_float(core.clamp_float(5.5, 0.0, 10.0), 5.5)
    assert_eq_float(core.clamp_float(-5.5, 0.0, 10.0), 0.0)
    assert_eq_float(core.clamp_float(15.5, 0.0, 10.0), 10.0)
    test_pass("clamp_float() works correctly")
    
    // Test lerp()
    assert_eq_float(core.lerp(0.0, 10.0, 0.5), 5.0)
    assert_eq_float(core.lerp(0.0, 10.0, 0.0), 0.0)
    assert_eq_float(core.lerp(0.0, 10.0, 1.0), 10.0)
    test_pass("lerp() works correctly")
    
    // Test format_bytes()
    assert_eq_string(core.format_bytes(512), "512 B")
    assert_eq_string(core.format_bytes(1024), "1 KB")
    assert_eq_string(core.format_bytes(1024 * 1024), "1 MB")
    test_pass("format_bytes() works correctly")
}

// ================================
// Slice Helper Function Tests
// ================================

slay test_slice_helpers() {
    test_start("Slice helper functions")
    
    // Test reverse_slice()
    sus original []normie = [1, 2, 3, 4, 5]
    sus reversed []normie = core.reverse_slice(original)
    assert_eq_int(reversed[0], 5)
    assert_eq_int(reversed[4], 1)
    assert_eq_int(core.len(reversed), 5)
    test_pass("reverse_slice() works correctly")
    
    // Test contains_slice()
    sus numbers []normie = [1, 2, 3, 4, 5]
    assert_eq_bool(core.contains_slice(numbers, 3), based)
    assert_eq_bool(core.contains_slice(numbers, 6), cap)
    test_pass("contains_slice() works correctly")
    
    // Test index_of_slice()
    sus letters []tea = ["a", "b", "c", "d"]
    assert_eq_int(core.index_of_slice(letters, "c"), 2)
    assert_eq_int(core.index_of_slice(letters, "z"), -1)
    test_pass("index_of_slice() works correctly")
}

// ================================
// Integration Tests
// ================================

slay test_integration() {
    test_start("Integration tests")
    
    // Test combination of functions
    sus input normie = -42
    sus abs_value normie = core.abs_int(input)
    sus as_string tea = core.tea(abs_value)
    sus string_length normie = core.len(as_string)
    
    assert_eq_int(abs_value, 42)
    assert_eq_string(as_string, "42")
    assert_eq_int(string_length, 2)
    test_pass("Function combination works correctly")
    
    // Test range and contains
    sus range []normie = core.range_int(1, 11, 2)
    assert_eq_bool(core.contains_slice(range, 5), based)
    assert_eq_bool(core.contains_slice(range, 6), cap)
    test_pass("Range and contains integration works")
    
    // Test min/max with type conversion
    sus float_values []meal = [3.14, 2.71, 1.41, 1.73]
    sus min_value meal = core.min_float(float_values[0], float_values[1])
    sus max_value meal = core.max_float(float_values[2], float_values[3])
    
    assert_eq_float(min_value, 2.71)
    assert_eq_float(max_value, 1.73)
    test_pass("Min/max with arrays works correctly")
}

// ================================
// Main Test Runner
// ================================

slay run_all_tests() {
    println("========================================")
    println("           CORE MODULE TESTS")
    println("========================================")
    println("")
    
    // Run all test functions
    test_type_conversions()
    test_collection_functions()
    test_math_helpers()
    test_min_max_functions()
    test_range_functions()
    test_type_checking()
    test_hash_functions()
    test_utility_functions()
    test_slice_helpers()
    test_integration()
    
    println("")
    println("========================================")
    print_test_summary()
    println("========================================")
}

// Start the test suite
run_all_tests()
