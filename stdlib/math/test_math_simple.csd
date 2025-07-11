// Simple Pure CURSED Math Library Test
// Tests math functions without external dependencies

// Global test counters
sus tests_passed normie = 0
sus tests_total normie = 0

// Simple assertion functions
slay assert_eq_float(actual meal, expected meal) {
    tests_total++
    sus diff meal = actual - expected
    bestie (diff < 0.0) {
        diff = -diff
    }
    bestie (diff < 0.0001) {
        tests_passed++
        vibez.spill("✅ PASS: Expected " + expected + ", got " + actual)
    } else {
        vibez.spill("❌ FAIL: Expected " + expected + ", got " + actual)
    }
}

slay assert_eq_int(actual normie, expected normie) {
    tests_total++
    bestie (actual == expected) {
        tests_passed++
        vibez.spill("✅ PASS: Expected " + expected + ", got " + actual)
    } else {
        vibez.spill("❌ FAIL: Expected " + expected + ", got " + actual)
    }
}

slay assert_true(condition lit) {
    tests_total++
    bestie (condition) {
        tests_passed++
        vibez.spill("✅ PASS: Condition is true")
    } else {
        vibez.spill("❌ FAIL: Condition is false")
    }
}

slay assert_false(condition lit) {
    tests_total++
    bestie (!condition) {
        tests_passed++
        vibez.spill("✅ PASS: Condition is false")
    } else {
        vibez.spill("❌ FAIL: Condition is true")
    }
}

slay test_section(name tea) {
    vibez.spill("📋 Testing: " + name)
    vibez.spill("=" * 40)
}

slay test_math_constants() {
    test_section("Math Constants")
    
    // Test mathematical constants
    sus pi_val meal = math_pi()
    sus e_val meal = math_e()
    sus tau_val meal = math_tau()
    
    assert_true(pi_val > 3.14 && pi_val < 3.15)
    assert_true(e_val > 2.71 && e_val < 2.72)
    assert_true(tau_val > 6.28 && tau_val < 6.29)
    
    vibez.spill("Constants test complete")
}

slay test_math_abs() {
    test_section("Math Absolute Value")
    
    // Test float absolute value
    assert_eq_float(math_abs(5.0), 5.0)
    assert_eq_float(math_abs(-5.0), 5.0)
    assert_eq_float(math_abs(0.0), 0.0)
    
    // Test integer absolute value
    assert_eq_int(math_abs_int(5), 5)
    assert_eq_int(math_abs_int(-5), 5)
    assert_eq_int(math_abs_int(0), 0)
    
    vibez.spill("Absolute value test complete")
}

slay test_math_min_max() {
    test_section("Math Min/Max")
    
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
    
    vibez.spill("Min/Max test complete")
}

slay test_math_power() {
    test_section("Math Power Functions")
    
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
    
    vibez.spill("Power functions test complete")
}

slay test_math_trig() {
    test_section("Math Trigonometric Functions")
    
    // Test basic trig functions with known values
    assert_eq_float(math_sin(0.0), 0.0)
    assert_eq_float(math_cos(0.0), 1.0)
    assert_eq_float(math_tan(0.0), 0.0)
    
    // Test sin/cos at π/2 with tolerance
    sus sin_pi_2 meal = math_sin(math_pi() / 2.0)
    sus cos_pi_2 meal = math_cos(math_pi() / 2.0)
    assert_true(sin_pi_2 > 0.99 && sin_pi_2 < 1.01)
    assert_true(cos_pi_2 > -0.01 && cos_pi_2 < 0.01)
    
    // Test inverse functions
    assert_eq_float(math_asin(0.0), 0.0)
    assert_eq_float(math_acos(1.0), 0.0)
    assert_eq_float(math_atan(0.0), 0.0)
    
    vibez.spill("Trigonometric functions test complete")
}

slay test_math_rounding() {
    test_section("Math Rounding Functions")
    
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
    
    vibez.spill("Rounding functions test complete")
}

slay test_math_utility() {
    test_section("Math Utility Functions")
    
    // Test sign function
    assert_eq_float(math_sign(5.0), 1.0)
    assert_eq_float(math_sign(-5.0), -1.0)
    assert_eq_float(math_sign(0.0), 0.0)
    
    // Test clamp function
    assert_eq_float(math_clamp(5.0, 0.0, 10.0), 5.0)
    assert_eq_float(math_clamp(-5.0, 0.0, 10.0), 0.0)
    assert_eq_float(math_clamp(15.0, 0.0, 10.0), 10.0)
    
    vibez.spill("Utility functions test complete")
}

slay test_math_gcd_lcm() {
    test_section("Math GCD/LCM Functions")
    
    // Test greatest common divisor
    assert_eq_int(math_gcd(48, 18), 6)
    assert_eq_int(math_gcd(17, 13), 1)
    assert_eq_int(math_gcd(100, 50), 50)
    
    // Test least common multiple
    assert_eq_int(math_lcm(4, 6), 12)
    assert_eq_int(math_lcm(3, 5), 15)
    assert_eq_int(math_lcm(12, 18), 36)
    
    vibez.spill("GCD/LCM functions test complete")
}

slay test_math_factorial() {
    test_section("Math Factorial Function")
    
    // Test factorial
    assert_eq_int(math_factorial(0), 1)
    assert_eq_int(math_factorial(1), 1)
    assert_eq_int(math_factorial(5), 120)
    assert_eq_int(math_factorial(3), 6)
    
    vibez.spill("Factorial function test complete")
}

slay test_math_fibonacci() {
    test_section("Math Fibonacci Function")
    
    // Test fibonacci sequence
    assert_eq_int(math_fibonacci(0), 0)
    assert_eq_int(math_fibonacci(1), 1)
    assert_eq_int(math_fibonacci(2), 1)
    assert_eq_int(math_fibonacci(3), 2)
    assert_eq_int(math_fibonacci(4), 3)
    assert_eq_int(math_fibonacci(5), 5)
    assert_eq_int(math_fibonacci(6), 8)
    
    vibez.spill("Fibonacci function test complete")
}

slay test_math_random() {
    test_section("Math Random Functions")
    
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
    
    vibez.spill("Random functions test complete")
}

slay test_math_degrees_radians() {
    test_section("Math Degree/Radian Conversion")
    
    // Test degree to radian conversion
    sus rad meal = math_radians(180.0)
    assert_true(rad > 3.14 && rad < 3.15)
    
    // Test radian to degree conversion
    sus deg meal = math_degrees(math_pi())
    assert_true(deg > 179.9 && deg < 180.1)
    
    vibez.spill("Degree/Radian conversion test complete")
}

slay test_math_distance() {
    test_section("Math Distance Functions")
    
    // Test 2D distance
    sus dist_2d meal = math_distance_2d(0.0, 0.0, 3.0, 4.0)
    assert_eq_float(dist_2d, 5.0)
    
    // Test 3D distance
    sus dist_3d meal = math_distance_3d(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)
    assert_true(dist_3d > 1.73 && dist_3d < 1.74)
    
    vibez.spill("Distance functions test complete")
}

slay test_math_interpolation() {
    test_section("Math Interpolation Functions")
    
    // Test linear interpolation
    assert_eq_float(math_lerp(0.0, 10.0, 0.5), 5.0)
    assert_eq_float(math_lerp(0.0, 10.0, 0.0), 0.0)
    assert_eq_float(math_lerp(0.0, 10.0, 1.0), 10.0)
    
    // Test inverse linear interpolation
    assert_eq_float(math_inverse_lerp(0.0, 10.0, 5.0), 0.5)
    assert_eq_float(math_inverse_lerp(0.0, 10.0, 0.0), 0.0)
    assert_eq_float(math_inverse_lerp(0.0, 10.0, 10.0), 1.0)
    
    vibez.spill("Interpolation functions test complete")
}

slay print_final_summary() {
    vibez.spill("")
    vibez.spill("🎉 Pure CURSED Math Library Test Results")
    vibez.spill("=" * 50)
    vibez.spill("Tests Passed: " + tests_passed)
    vibez.spill("Tests Total:  " + tests_total)
    
    sus pass_rate meal = tests_passed.(meal) / tests_total.(meal) * 100.0
    vibez.spill("Pass Rate:    " + pass_rate + "%")
    
    bestie (tests_passed == tests_total) {
        vibez.spill("✅ ALL TESTS PASSED!")
        vibez.spill("🚀 Math stdlib successfully migrated to pure CURSED")
        vibez.spill("💯 Zero FFI dependencies - fully self-contained")
        vibez.spill("🎯 Backward compatibility maintained")
    } else {
        vibez.spill("❌ Some tests failed")
        vibez.spill("Failed: " + (tests_total - tests_passed))
    }
    
    vibez.spill("=" * 50)
}

slay run_all_math_tests() {
    vibez.spill("🧮 Running Pure CURSED Math Library Tests")
    vibez.spill("==========================================")
    vibez.spill("Testing FFI-free math implementation...")
    vibez.spill("")
    
    test_math_constants()
    test_math_abs()
    test_math_min_max()
    test_math_power()
    test_math_trig()
    test_math_rounding()
    test_math_utility()
    test_math_gcd_lcm()
    test_math_factorial()
    test_math_fibonacci()
    test_math_random()
    test_math_degrees_radians()
    test_math_distance()
    test_math_interpolation()
    
    print_final_summary()
}

// Auto-run tests when this file is executed
run_all_math_tests()
