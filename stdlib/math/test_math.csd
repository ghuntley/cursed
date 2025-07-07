fr fr Import testing framework and math library
fr fr For now, we'll use direct function calls since module system is still being developed

fr fr ========================================
fr fr CURSED Math Library Test Suite
fr fr ========================================

slay test_math_constants() {
    testz.test_start("Math Constants")
    
    fr fr Test mathematical constants
    testz.assert_eq_string(tea(math_pi()), "3.141592653589793")
    testz.assert_eq_string(tea(math_e()), "2.718281828459045")
    testz.assert_eq_string(tea(math_tau()), "6.283185307179586")
}

slay test_math_abs() {
    testz.test_start("Math Absolute Value")
    
    fr fr Test float absolute value
    testz.assert_eq_string(tea(math_abs(5.0)), "5.0")
    testz.assert_eq_string(tea(math_abs(-5.0)), "5.0")
    testz.assert_eq_string(tea(math_abs(0.0)), "0.0")
    
    fr fr Test integer absolute value
    testz.assert_eq_int(math_abs_int(5), 5)
    testz.assert_eq_int(math_abs_int(-5), 5)
    testz.assert_eq_int(math_abs_int(0), 0)
}

slay test_math_min_max() {
    testz.test_start("Math Min/Max")
    
    fr fr Test float min/max
    testz.assert_eq_string(tea(math_min(5.0, 3.0)), "3.0")
    testz.assert_eq_string(tea(math_max(5.0, 3.0)), "5.0")
    testz.assert_eq_string(tea(math_min(-1.0, -2.0)), "-2.0")
    testz.assert_eq_string(tea(math_max(-1.0, -2.0)), "-1.0")
    
    fr fr Test integer min/max
    testz.assert_eq_int(math_min_int(5, 3), 3)
    testz.assert_eq_int(math_max_int(5, 3), 5)
    testz.assert_eq_int(math_min_int(-1, -2), -2)
    testz.assert_eq_int(math_max_int(-1, -2), -1)
}

slay test_math_power() {
    testz.test_start("Math Power Functions")
    
    fr fr Test basic power
    testz.assert_eq_string(tea(math_pow(2.0, 3.0)), "8.0")
    testz.assert_eq_string(tea(math_pow(4.0, 0.5)), "2.0")
    testz.assert_eq_string(tea(math_pow(1.0, 100.0)), "1.0")
    
    fr fr Test square root
    testz.assert_eq_string(tea(math_sqrt(4.0)), "2.0")
    testz.assert_eq_string(tea(math_sqrt(9.0)), "3.0")
    testz.assert_eq_string(tea(math_sqrt(16.0)), "4.0")
    
    fr fr Test cube root
    testz.assert_eq_string(tea(math_cbrt(8.0)), "2.0")
    testz.assert_eq_string(tea(math_cbrt(27.0)), "3.0")
}

slay test_math_trig() {
    testz.test_start("Math Trigonometric Functions")
    
    fr fr Test basic trig functions with known values
    testz.assert_eq_string(tea(math_sin(0.0)), "0.0")
    testz.assert_eq_string(tea(math_cos(0.0)), "1.0")
    testz.assert_eq_string(tea(math_tan(0.0)), "0.0")
    
    fr fr Test sin/cos at π/2
    sus sin_pi_2 meal = math_sin(math_pi() / 2.0)
    sus cos_pi_2 meal = math_cos(math_pi() / 2.0)
    testz.assert_true(sin_pi_2 > 0.99 && sin_pi_2 < 1.01)
    testz.assert_true(cos_pi_2 > -0.01 && cos_pi_2 < 0.01)
    
    fr fr Test inverse functions
    testz.assert_eq_string(tea(math_asin(0.0)), "0.0")
    testz.assert_eq_string(tea(math_acos(1.0)), "0.0")
    testz.assert_eq_string(tea(math_atan(0.0)), "0.0")
}

slay test_math_rounding() {
    testz.test_start("Math Rounding Functions")
    
    fr fr Test floor function
    testz.assert_eq_string(tea(math_floor(3.7)), "3.0")
    testz.assert_eq_string(tea(math_floor(-3.7)), "-4.0")
    testz.assert_eq_string(tea(math_floor(5.0)), "5.0")
    
    fr fr Test ceiling function
    testz.assert_eq_string(tea(math_ceil(3.2)), "4.0")
    testz.assert_eq_string(tea(math_ceil(-3.2)), "-3.0")
    testz.assert_eq_string(tea(math_ceil(5.0)), "5.0")
    
    fr fr Test rounding function
    testz.assert_eq_string(tea(math_round(3.4)), "3.0")
    testz.assert_eq_string(tea(math_round(3.6)), "4.0")
    testz.assert_eq_string(tea(math_round(-3.4)), "-3.0")
    testz.assert_eq_string(tea(math_round(-3.6)), "-4.0")
}

slay test_math_utility() {
    testz.test_start("Math Utility Functions")
    
    fr fr Test sign function
    testz.assert_eq_string(tea(math_sign(5.0)), "1.0")
    testz.assert_eq_string(tea(math_sign(-5.0)), "-1.0")
    testz.assert_eq_string(tea(math_sign(0.0)), "0.0")
    
    fr fr Test clamp function
    testz.assert_eq_string(tea(math_clamp(5.0, 0.0, 10.0)), "5.0")
    testz.assert_eq_string(tea(math_clamp(-5.0, 0.0, 10.0)), "0.0")
    testz.assert_eq_string(tea(math_clamp(15.0, 0.0, 10.0)), "10.0")
}

slay test_math_degrees_radians() {
    testz.test_start("Math Degree/Radian Conversion")
    
    fr fr Test degree to radian conversion
    sus rad meal = math_radians(180.0)
    testz.assert_true(rad > 3.14 && rad < 3.15)
    
    fr fr Test radian to degree conversion
    sus deg meal = math_degrees(math_pi())
    testz.assert_true(deg > 179.9 && deg < 180.1)
}

slay test_math_distance() {
    testz.test_start("Math Distance Functions")
    
    fr fr Test 2D distance
    sus dist_2d meal = math_distance_2d(0.0, 0.0, 3.0, 4.0)
    testz.assert_eq_string(tea(dist_2d), "5.0")
    
    fr fr Test 3D distance
    sus dist_3d meal = math_distance_3d(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)
    testz.assert_true(dist_3d > 1.73 && dist_3d < 1.74)
}

slay test_math_interpolation() {
    testz.test_start("Math Interpolation Functions")
    
    fr fr Test linear interpolation
    testz.assert_eq_string(tea(math_lerp(0.0, 10.0, 0.5)), "5.0")
    testz.assert_eq_string(tea(math_lerp(0.0, 10.0, 0.0)), "0.0")
    testz.assert_eq_string(tea(math_lerp(0.0, 10.0, 1.0)), "10.0")
    
    fr fr Test inverse linear interpolation
    testz.assert_eq_string(tea(math_inverse_lerp(0.0, 10.0, 5.0)), "0.5")
    testz.assert_eq_string(tea(math_inverse_lerp(0.0, 10.0, 0.0)), "0.0")
    testz.assert_eq_string(tea(math_inverse_lerp(0.0, 10.0, 10.0)), "1.0")
}

slay test_math_gcd_lcm() {
    testz.test_start("Math GCD/LCM Functions")
    
    fr fr Test greatest common divisor
    testz.assert_eq_int(math_gcd(48, 18), 6)
    testz.assert_eq_int(math_gcd(17, 13), 1)
    testz.assert_eq_int(math_gcd(100, 50), 50)
    
    fr fr Test least common multiple
    testz.assert_eq_int(math_lcm(4, 6), 12)
    testz.assert_eq_int(math_lcm(3, 5), 15)
    testz.assert_eq_int(math_lcm(12, 18), 36)
}

slay test_math_factorial() {
    testz.test_start("Math Factorial Function")
    
    fr fr Test factorial
    testz.assert_eq_int(math_factorial(0), 1)
    testz.assert_eq_int(math_factorial(1), 1)
    testz.assert_eq_int(math_factorial(5), 120)
    testz.assert_eq_int(math_factorial(3), 6)
}

slay test_math_fibonacci() {
    testz.test_start("Math Fibonacci Function")
    
    fr fr Test fibonacci sequence
    testz.assert_eq_int(math_fibonacci(0), 0)
    testz.assert_eq_int(math_fibonacci(1), 1)
    testz.assert_eq_int(math_fibonacci(2), 1)
    testz.assert_eq_int(math_fibonacci(3), 2)
    testz.assert_eq_int(math_fibonacci(4), 3)
    testz.assert_eq_int(math_fibonacci(5), 5)
    testz.assert_eq_int(math_fibonacci(6), 8)
}

slay test_math_random() {
    testz.test_start("Math Random Functions")
    
    fr fr Test random number generation
    sus rand1 meal = math_random()
    sus rand2 meal = math_random()
    testz.assert_true(rand1 >= 0.0 && rand1 <= 1.0)
    testz.assert_true(rand2 >= 0.0 && rand2 <= 1.0)
    testz.assert_true(rand1 != rand2)
    
    fr fr Test random integer generation
    sus rand_int normie = math_random_int(1, 10)
    testz.assert_true(rand_int >= 1 && rand_int <= 10)
    
    fr fr Test random float generation
    sus rand_float meal = math_random_float(1.0, 10.0)
    testz.assert_true(rand_float >= 1.0 && rand_float <= 10.0)
}

slay test_math_edge_cases() {
    testz.test_start("Math Edge Cases")
    
    fr fr Test division by zero protection
    sus zero meal = 0.0
    testz.assert_true(math_is_infinite(1.0 / zero))
    testz.assert_true(math_is_nan(0.0 / zero))
    
    fr fr Test negative square root
    testz.assert_true(math_is_nan(math_sqrt(-1.0)))
    
    fr fr Test finite number checking
    testz.assert_true(math_is_finite(42.0))
    testz.assert_false(math_is_finite(1.0 / zero))
}

slay run_all_math_tests() {
    vibez.spill("🧮 Running CURSED Math Library Tests")
    vibez.spill("=====================================")
    
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
    
    testz.print_test_summary()
    damn testz.run_all_tests()
}

fr fr Auto-run tests when this file is executed
run_all_math_tests()
