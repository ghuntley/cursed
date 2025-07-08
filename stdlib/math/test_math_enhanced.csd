yeet "testz"
yeet "error_core"
yeet "math"  # This would import the enhanced math module

# Test basic math functions with error handling
slay test_math_clamp_with_errors() {
    test_start("Math clamp with error handling")
    
    # Test valid clamp
    sus result, err = math_clamp(5.0, 0.0, 10.0)
    assert_eq(err, cringe)
    assert_eq_float(result, 5.0)
    
    # Test clamp with invalid range
    sus result2, err2 = math_clamp(5.0, 10.0, 0.0)
    assert_ne(err2, cringe)
    assert_true(is_error_type(err2, "value_error"))
    
    vibez.spill("✅ Math clamp with error handling test passed")
}

slay test_math_sqrt_with_errors() {
    test_start("Math sqrt with error handling")
    
    # Test valid sqrt
    sus result, err = math_sqrt(16.0)
    assert_eq(err, cringe)
    assert_eq_float(result, 4.0)
    
    # Test sqrt of negative number
    sus result2, err2 = math_sqrt(-1.0)
    assert_ne(err2, cringe)
    assert_true(is_error_type(err2, "value_error"))
    assert_eq_string(err2.message(), "Cannot compute square root of negative number")
    
    vibez.spill("✅ Math sqrt with error handling test passed")
}

slay test_math_pow_with_errors() {
    test_start("Math pow with error handling")
    
    # Test valid power
    sus result, err = math_pow(2.0, 3.0)
    assert_eq(err, cringe)
    
    # Test zero to negative power
    sus result2, err2 = math_pow(0.0, -1.0)
    assert_ne(err2, cringe)
    assert_true(is_error_type(err2, "value_error"))
    assert_eq_string(err2.message(), "Cannot raise zero to negative power")
    
    # Test negative base to fractional power
    sus result3, err3 = math_pow(-2.0, 0.5)
    assert_ne(err3, cringe)
    assert_true(is_error_type(err3, "value_error"))
    
    vibez.spill("✅ Math pow with error handling test passed")
}

slay test_math_log_with_errors() {
    test_start("Math log with error handling")
    
    # Test valid log
    sus result, err = math_log(2.71828)
    assert_eq(err, cringe)
    
    # Test log of zero
    sus result2, err2 = math_log(0.0)
    assert_ne(err2, cringe)
    assert_true(is_error_type(err2, "value_error"))
    assert_eq_string(err2.message(), "Cannot compute logarithm of non-positive number")
    
    # Test log of negative number
    sus result3, err3 = math_log(-1.0)
    assert_ne(err3, cringe)
    assert_true(is_error_type(err3, "value_error"))
    
    vibez.spill("✅ Math log with error handling test passed")
}

slay test_math_trig_with_errors() {
    test_start("Math trigonometric functions with error handling")
    
    # Test valid asin
    sus result, err = math_asin(0.5)
    assert_eq(err, cringe)
    
    # Test asin out of range
    sus result2, err2 = math_asin(2.0)
    assert_ne(err2, cringe)
    assert_true(is_error_type(err2, "value_error"))
    assert_eq_string(err2.message(), "Arcsine domain error")
    
    # Test valid acos
    sus result3, err3 = math_acos(0.5)
    assert_eq(err3, cringe)
    
    # Test acos out of range
    sus result4, err4 = math_acos(-2.0)
    assert_ne(err4, cringe)
    assert_true(is_error_type(err4, "value_error"))
    assert_eq_string(err4.message(), "Arccosine domain error")
    
    # Test atan2 with both zeros
    sus result5, err5 = math_atan2(0.0, 0.0)
    assert_ne(err5, cringe)
    assert_true(is_error_type(err5, "value_error"))
    assert_eq_string(err5.message(), "Atan2 undefined for (0, 0)")
    
    vibez.spill("✅ Math trigonometric functions with error handling test passed")
}

slay test_math_factorial_with_errors() {
    test_start("Math factorial with error handling")
    
    # Test valid factorial
    sus result, err = math_factorial(5)
    assert_eq(err, cringe)
    assert_eq_int(result, 120)
    
    # Test factorial of zero
    sus result2, err2 = math_factorial(0)
    assert_eq(err2, cringe)
    assert_eq_int(result2, 1)
    
    # Test factorial of negative number
    sus result3, err3 = math_factorial(-1)
    assert_ne(err3, cringe)
    assert_true(is_error_type(err3, "value_error"))
    assert_eq_string(err3.message(), "Factorial undefined for negative numbers")
    
    # Test factorial overflow
    sus result4, err4 = math_factorial(25)
    assert_ne(err4, cringe)
    assert_true(is_error_type(err4, "value_error"))
    assert_eq_string(err4.message(), "Factorial overflow")
    
    vibez.spill("✅ Math factorial with error handling test passed")
}

slay test_math_gcd_with_errors() {
    test_start("Math GCD with error handling")
    
    # Test valid GCD
    sus result, err = math_gcd(12, 18)
    assert_eq(err, cringe)
    assert_eq_int(result, 6)
    
    # Test GCD with one zero
    sus result2, err2 = math_gcd(0, 5)
    assert_eq(err2, cringe)
    assert_eq_int(result2, 5)
    
    # Test GCD with both zeros
    sus result3, err3 = math_gcd(0, 0)
    assert_ne(err3, cringe)
    assert_true(is_error_type(err3, "value_error"))
    assert_eq_string(err3.message(), "GCD undefined for (0, 0)")
    
    vibez.spill("✅ Math GCD with error handling test passed")
}

slay test_math_lcm_with_errors() {
    test_start("Math LCM with error handling")
    
    # Test valid LCM
    sus result, err = math_lcm(12, 18)
    assert_eq(err, cringe)
    assert_eq_int(result, 36)
    
    # Test LCM with both zeros
    sus result2, err2 = math_lcm(0, 0)
    assert_ne(err2, cringe)
    assert_true(is_error_type(err2, "value_error"))
    assert_eq_string(err2.message(), "LCM undefined for (0, 0)")
    
    vibez.spill("✅ Math LCM with error handling test passed")
}

slay test_math_combinations_with_errors() {
    test_start("Math combinations with error handling")
    
    # Test valid combinations
    sus result, err = math_combinations(5, 2)
    assert_eq(err, cringe)
    assert_eq_int(result, 10)
    
    # Test combinations with negative numbers
    sus result2, err2 = math_combinations(-1, 2)
    assert_ne(err2, cringe)
    assert_true(is_error_type(err2, "value_error"))
    assert_eq_string(err2.message(), "Combinations undefined for negative numbers")
    
    # Test combinations with k > n
    sus result3, err3 = math_combinations(3, 5)
    assert_ne(err3, cringe)
    assert_true(is_error_type(err3, "value_error"))
    assert_eq_string(err3.message(), "Cannot choose more items than available")
    
    vibez.spill("✅ Math combinations with error handling test passed")
}

slay test_math_statistical_functions_with_errors() {
    test_start("Math statistical functions with error handling")
    
    # Test valid mean
    sus values []meal = []meal{1.0, 2.0, 3.0, 4.0, 5.0}
    sus result, err = math_mean(values)
    assert_eq(err, cringe)
    assert_eq_float(result, 3.0)
    
    # Test mean of empty array
    sus empty_values []meal = []meal{}
    sus result2, err2 = math_mean(empty_values)
    assert_ne(err2, cringe)
    assert_true(is_error_type(err2, "value_error"))
    assert_eq_string(err2.message(), "Cannot compute mean of empty array")
    
    # Test valid median
    sus result3, err3 = math_median(values)
    assert_eq(err3, cringe)
    assert_eq_float(result3, 3.0)
    
    # Test median of empty array
    sus result4, err4 = math_median(empty_values)
    assert_ne(err4, cringe)
    assert_true(is_error_type(err4, "value_error"))
    assert_eq_string(err4.message(), "Cannot compute median of empty array")
    
    # Test valid variance
    sus result5, err5 = math_variance(values)
    assert_eq(err5, cringe)
    
    # Test variance of empty array
    sus result6, err6 = math_variance(empty_values)
    assert_ne(err6, cringe)
    assert_true(is_error_type(err6, "value_error"))
    assert_eq_string(err6.message(), "Cannot compute variance of empty array")
    
    vibez.spill("✅ Math statistical functions with error handling test passed")
}

slay test_math_fmod_with_errors() {
    test_start("Math fmod with error handling")
    
    # Test valid fmod
    sus result, err = math_fmod(7.5, 2.5)
    assert_eq(err, cringe)
    
    # Test fmod by zero
    sus result2, err2 = math_fmod(7.5, 0.0)
    assert_ne(err2, cringe)
    assert_true(is_error_type(err2, "value_error"))
    assert_eq_string(err2.message(), "Modulo by zero")
    
    vibez.spill("✅ Math fmod with error handling test passed")
}

slay test_math_overflow_protection() {
    test_start("Math overflow protection")
    
    # Test exp overflow protection
    sus result, err = math_exp(750.0)
    assert_ne(err, cringe)
    assert_true(is_error_type(err, "value_error"))
    assert_eq_string(err.message(), "Exponent too large, would cause overflow")
    
    # Test sinh overflow protection
    sus result2, err2 = math_sinh(750.0)
    assert_ne(err2, cringe)
    assert_true(is_error_type(err2, "value_error"))
    assert_eq_string(err2.message(), "Hyperbolic sine overflow")
    
    # Test cosh overflow protection
    sus result3, err3 = math_cosh(750.0)
    assert_ne(err3, cringe)
    assert_true(is_error_type(err3, "value_error"))
    assert_eq_string(err3.message(), "Hyperbolic cosine overflow")
    
    vibez.spill("✅ Math overflow protection test passed")
}

slay test_math_error_propagation() {
    test_start("Math error propagation")
    
    # Test that errors propagate correctly through calculations
    slay calculate_complex_expression(x meal) (meal, yikes) {
        sus sqrt_result, err = math_sqrt(x)
        vibe_check err != cringe {
            damn 0.0, wrap_error(err, "Complex calculation failed")
        }
        
        sus log_result, err2 = math_log(sqrt_result)
        vibe_check err2 != cringe {
            damn 0.0, wrap_error(err2, "Complex calculation failed")
        }
        
        damn log_result, cringe
    }
    
    # Test successful calculation
    sus result, err = calculate_complex_expression(4.0)
    assert_eq(err, cringe)
    
    # Test error propagation
    sus result2, err2 = calculate_complex_expression(-1.0)
    assert_ne(err2, cringe)
    assert_eq_string(err2.message(), "Complex calculation failed: Cannot compute square root of negative number")
    
    vibez.spill("✅ Math error propagation test passed")
}

slay test_math_utility_functions() {
    test_start("Math utility functions")
    
    # Test sign function
    assert_eq_float(math_sign(5.0), 1.0)
    assert_eq_float(math_sign(-5.0), -1.0)
    assert_eq_float(math_sign(0.0), 0.0)
    
    # Test degree/radian conversion
    sus rad_val = math_deg_to_rad(180.0)
    sus deg_val = math_rad_to_deg(math_pi())
    assert_eq_float(rad_val, math_pi())
    assert_eq_float(deg_val, 180.0)
    
    # Test finite/infinite/NaN checks
    assert_true(math_is_finite(5.0))
    assert_false(math_is_nan(5.0))
    assert_false(math_is_inf(5.0))
    
    vibez.spill("✅ Math utility functions test passed")
}

# Main test runner
test_math_clamp_with_errors()
test_math_sqrt_with_errors()
test_math_pow_with_errors()
test_math_log_with_errors()
test_math_trig_with_errors()
test_math_factorial_with_errors()
test_math_gcd_with_errors()
test_math_lcm_with_errors()
test_math_combinations_with_errors()
test_math_statistical_functions_with_errors()
test_math_fmod_with_errors()
test_math_overflow_protection()
test_math_error_propagation()
test_math_utility_functions()

print_test_summary()
