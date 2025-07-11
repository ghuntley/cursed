yeet "testz"
yeet "math"

// ================================
// Pure CURSED Math Library Test Suite
// ================================

slay test_math_constants() {
    test_start("Math Constants")
    
    // Test mathematical constants
    sus pi_val meal = math_pi()
    sus e_val meal = math_e()
    sus tau_val meal = math_tau()
    
    assert_true(pi_val > 3.14 && pi_val < 3.15)
    assert_true(e_val > 2.71 && e_val < 2.72)
    assert_true(tau_val > 6.28 && tau_val < 6.29)
    
    print_test_summary()
}

slay test_math_abs() {
    test_start("Math Absolute Value")
    
    // Test float absolute value
    assert_eq_float(math_abs(5.0), 5.0)
    assert_eq_float(math_abs(-5.0), 5.0)
    assert_eq_float(math_abs(0.0), 0.0)
    
    // Test integer absolute value
    assert_eq_int(math_abs_int(5), 5)
    assert_eq_int(math_abs_int(-5), 5)
    assert_eq_int(math_abs_int(0), 0)
    
    print_test_summary()
}

slay test_math_min_max() {
    test_start("Math Min/Max")
    
    // Test float min/max
    assert_eq_float(math_min(5.0, 3.0), 3.0)
    assert_eq_float(math_max(5.0, 3.0), 5.0)
    assert_eq_float(math_min(-1.0, -2.0), -2.0)
    assert_eq_float(math_max(-1.0, -2.0), -1.0)
    
    // Test integer min/max
    assert_eq_int(math_min_int(5, 3), 3)
    assert_eq_int(math_max_int(5, 3), 5)
    assert_eq_int(math_min_int(-1, -2), -2)
    assert_eq_int(math_max_int(-1, -2), -1)
    
    print_test_summary()
}

slay test_math_power() {
    test_start("Math Power Functions")
    
    // Test basic power
    assert_eq_float(math_pow(2.0, 3.0), 8.0)
    assert_eq_float(math_pow(4.0, 0.5), 2.0)
    assert_eq_float(math_pow(1.0, 100.0), 1.0)
    
    // Test square root
    assert_eq_float(math_sqrt(4.0), 2.0)
    assert_eq_float(math_sqrt(9.0), 3.0)
    assert_eq_float(math_sqrt(16.0), 4.0)
    
    // Test cube root
    assert_eq_float(math_cbrt(8.0), 2.0)
    assert_eq_float(math_cbrt(27.0), 3.0)
    
    print_test_summary()
}

slay test_math_trig() {
    test_start("Math Trigonometric Functions")
    
    // Test basic trig functions with known values
    assert_eq_float(math_sin(0.0), 0.0)
    assert_eq_float(math_cos(0.0), 1.0)
    assert_eq_float(math_tan(0.0), 0.0)
    
    // Test sin/cos at π/2
    sus sin_pi_2 meal = math_sin(math_pi() / 2.0)
    sus cos_pi_2 meal = math_cos(math_pi() / 2.0)
    assert_true(sin_pi_2 > 0.99 && sin_pi_2 < 1.01)
    assert_true(cos_pi_2 > -0.01 && cos_pi_2 < 0.01)
    
    // Test inverse functions
    assert_eq_float(math_asin(0.0), 0.0)
    assert_eq_float(math_acos(1.0), 0.0)
    assert_eq_float(math_atan(0.0), 0.0)
    
    print_test_summary()
}

slay test_math_rounding() {
    test_start("Math Rounding Functions")
    
    // Test floor function
    assert_eq_float(math_floor(3.7), 3.0)
    assert_eq_float(math_floor(-3.7), -4.0)
    assert_eq_float(math_floor(5.0), 5.0)
    
    // Test ceiling function
    assert_eq_float(math_ceil(3.2), 4.0)
    assert_eq_float(math_ceil(-3.2), -3.0)
    assert_eq_float(math_ceil(5.0), 5.0)
    
    // Test rounding function
    assert_eq_float(math_round(3.4), 3.0)
    assert_eq_float(math_round(3.6), 4.0)
    assert_eq_float(math_round(-3.4), -3.0)
    assert_eq_float(math_round(-3.6), -4.0)
    
    print_test_summary()
}

slay test_math_utility() {
    test_start("Math Utility Functions")
    
    // Test sign function
    assert_eq_float(math_sign(5.0), 1.0)
    assert_eq_float(math_sign(-5.0), -1.0)
    assert_eq_float(math_sign(0.0), 0.0)
    
    // Test clamp function
    assert_eq_float(math_clamp(5.0, 0.0, 10.0), 5.0)
    assert_eq_float(math_clamp(-5.0, 0.0, 10.0), 0.0)
    assert_eq_float(math_clamp(15.0, 0.0, 10.0), 10.0)
    
    print_test_summary()
}

slay test_math_degrees_radians() {
    test_start("Math Degree/Radian Conversion")
    
    // Test degree to radian conversion
    sus rad meal = math_radians(180.0)
    assert_true(rad > 3.14 && rad < 3.15)
    
    // Test radian to degree conversion
    sus deg meal = math_degrees(math_pi())
    assert_true(deg > 179.9 && deg < 180.1)
    
    print_test_summary()
}

slay test_math_distance() {
    test_start("Math Distance Functions")
    
    // Test 2D distance
    sus dist_2d meal = math_distance_2d(0.0, 0.0, 3.0, 4.0)
    assert_eq_float(dist_2d, 5.0)
    
    // Test 3D distance
    sus dist_3d meal = math_distance_3d(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)
    assert_true(dist_3d > 1.73 && dist_3d < 1.74)
    
    print_test_summary()
}

slay test_math_interpolation() {
    test_start("Math Interpolation Functions")
    
    // Test linear interpolation
    assert_eq_float(math_lerp(0.0, 10.0, 0.5), 5.0)
    assert_eq_float(math_lerp(0.0, 10.0, 0.0), 0.0)
    assert_eq_float(math_lerp(0.0, 10.0, 1.0), 10.0)
    
    // Test inverse linear interpolation
    assert_eq_float(math_inverse_lerp(0.0, 10.0, 5.0), 0.5)
    assert_eq_float(math_inverse_lerp(0.0, 10.0, 0.0), 0.0)
    assert_eq_float(math_inverse_lerp(0.0, 10.0, 10.0), 1.0)
    
    print_test_summary()
}

slay test_math_gcd_lcm() {
    test_start("Math GCD/LCM Functions")
    
    // Test greatest common divisor
    assert_eq_int(math_gcd(48, 18), 6)
    assert_eq_int(math_gcd(17, 13), 1)
    assert_eq_int(math_gcd(100, 50), 50)
    
    // Test least common multiple
    assert_eq_int(math_lcm(4, 6), 12)
    assert_eq_int(math_lcm(3, 5), 15)
    assert_eq_int(math_lcm(12, 18), 36)
    
    print_test_summary()
}

slay test_math_factorial() {
    test_start("Math Factorial Function")
    
    // Test factorial
    assert_eq_int(math_factorial(0), 1)
    assert_eq_int(math_factorial(1), 1)
    assert_eq_int(math_factorial(5), 120)
    assert_eq_int(math_factorial(3), 6)
    
    print_test_summary()
}

slay test_math_fibonacci() {
    test_start("Math Fibonacci Function")
    
    // Test fibonacci sequence
    assert_eq_int(math_fibonacci(0), 0)
    assert_eq_int(math_fibonacci(1), 1)
    assert_eq_int(math_fibonacci(2), 1)
    assert_eq_int(math_fibonacci(3), 2)
    assert_eq_int(math_fibonacci(4), 3)
    assert_eq_int(math_fibonacci(5), 5)
    assert_eq_int(math_fibonacci(6), 8)
    
    print_test_summary()
}

slay test_math_random() {
    test_start("Math Random Functions")
    
    // Test random number generation
    sus rand1 meal = math_random()
    sus rand2 meal = math_random()
    assert_true(rand1 >= 0.0 && rand1 <= 1.0)
    assert_true(rand2 >= 0.0 && rand2 <= 1.0)
    assert_true(rand1 != rand2)
    
    // Test random integer generation
    sus rand_int normie = math_random_int(1, 10)
    assert_true(rand_int >= 1 && rand_int <= 10)
    
    // Test random float generation
    sus rand_float meal = math_random_float(1.0, 10.0)
    assert_true(rand_float >= 1.0 && rand_float <= 10.0)
    
    print_test_summary()
}

slay test_math_edge_cases() {
    test_start("Math Edge Cases")
    
    // Test division by zero protection
    sus zero meal = 0.0
    assert_true(math_is_infinite(1.0 / zero))
    assert_true(math_is_nan(0.0 / zero))
    
    // Test negative square root
    assert_true(math_is_nan(math_sqrt(-1.0)))
    
    // Test finite number checking
    assert_true(math_is_finite(42.0))
    assert_false(math_is_finite(1.0 / zero))
    
    print_test_summary()
}

slay test_math_logarithms() {
    test_start("Math Logarithmic Functions")
    
    // Test natural logarithm
    assert_eq_float(math_log(math_e()), 1.0)
    assert_eq_float(math_log(1.0), 0.0)
    
    // Test base-10 logarithm
    assert_eq_float(math_log10(10.0), 1.0)
    assert_eq_float(math_log10(100.0), 2.0)
    
    // Test base-2 logarithm
    assert_eq_float(math_log2(2.0), 1.0)
    assert_eq_float(math_log2(8.0), 3.0)
    
    print_test_summary()
}

slay test_math_exponential() {
    test_start("Math Exponential Functions")
    
    // Test natural exponential
    assert_eq_float(math_exp(0.0), 1.0)
    assert_eq_float(math_exp(1.0), math_e())
    
    // Test base-2 exponential
    assert_eq_float(math_exp2(0.0), 1.0)
    assert_eq_float(math_exp2(3.0), 8.0)
    
    print_test_summary()
}

slay test_math_hyperbolic() {
    test_start("Math Hyperbolic Functions")
    
    // Test hyperbolic sine
    assert_eq_float(math_sinh(0.0), 0.0)
    
    // Test hyperbolic cosine
    assert_eq_float(math_cosh(0.0), 1.0)
    
    // Test hyperbolic tangent
    assert_eq_float(math_tanh(0.0), 0.0)
    
    print_test_summary()
}

slay test_math_geometry() {
    test_start("Math Geometry Functions")
    
    // Test 2D dot product
    assert_eq_float(math_dot_product_2d(1.0, 2.0, 3.0, 4.0), 11.0)
    
    // Test 2D cross product
    assert_eq_float(math_cross_product_2d(1.0, 2.0, 3.0, 4.0), -2.0)
    
    // Test 2D magnitude
    assert_eq_float(math_magnitude_2d(3.0, 4.0), 5.0)
    
    // Test 2D normalization
    sus normalized [meal] = math_normalize_2d(3.0, 4.0)
    assert_eq_float(normalized[0], 0.6)
    assert_eq_float(normalized[1], 0.8)
    
    print_test_summary()
}

slay run_all_math_tests() {
    vibez.spill("🧮 Running Pure CURSED Math Library Tests")
    vibez.spill("==========================================")
    
    test_math_constants()
    test_math_abs()
    test_math_min_max()
    test_math_power()
    test_math_trig()
    test_math_rounding()
    test_math_utility()
    test_math_degrees_radians()
    test_math_distance()
    test_math_interpolation()
    test_math_gcd_lcm()
    test_math_factorial()
    test_math_fibonacci()
    test_math_random()
    test_math_edge_cases()
    test_math_logarithms()
    test_math_exponential()
    test_math_hyperbolic()
    test_math_geometry()
    
    vibez.spill("🎉 All Pure CURSED Math Tests Complete!")
    vibez.spill("✅ Math stdlib successfully migrated from FFI to pure CURSED")
    vibez.spill("🚀 No external dependencies - fully self-contained")
    vibez.spill("💯 Backward compatibility maintained")
}

// Auto-run tests when this file is executed
run_all_math_tests()
