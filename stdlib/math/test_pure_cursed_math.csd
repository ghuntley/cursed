// Comprehensive test suite for pure CURSED math implementation
yeet "testz"
yeet "math"

// ================================
// Test Constants
// ================================

slay test_math_constants() {
    test_start("Math constants");
    
    # Test PI
    sus pi_val meal = math_pi();
    assert_true(abs(pi_val - 3.141592653589793) < 1e-10);
    
    # Test E
    sus e_val meal = math_e();
    assert_true(abs(e_val - 2.718281828459045) < 1e-10);
    
    # Test TAU
    sus tau_val meal = math_tau();
    assert_true(abs(tau_val - 6.283185307179586) < 1e-10);
    
    print_test_summary();
}

// ================================
// Test Basic Operations
// ================================

slay test_basic_operations() {
    test_start("Basic operations");
    
    # Test absolute value
    assert_eq_int(math_abs(5.0).(normie), 5);
    assert_eq_int(math_abs(-5.0).(normie), 5);
    assert_eq_int(math_abs_int(-42), 42);
    assert_eq_int(math_abs_int(42), 42);
    
    # Test min/max
    assert_eq_int(math_min(3.5, 7.2).(normie), 3);
    assert_eq_int(math_max(3.5, 7.2).(normie), 7);
    assert_eq_int(math_min_int(10, 5), 5);
    assert_eq_int(math_max_int(10, 5), 10);
    
    # Test clamp
    assert_eq_int(math_clamp(15.0, 0.0, 10.0).(normie), 10);
    assert_eq_int(math_clamp(-5.0, 0.0, 10.0).(normie), 0);
    assert_eq_int(math_clamp(5.0, 0.0, 10.0).(normie), 5);
    
    # Test sign
    assert_eq_int(math_sign(5.0).(normie), 1);
    assert_eq_int(math_sign(-5.0).(normie), -1);
    assert_eq_int(math_sign(0.0).(normie), 0);
    
    print_test_summary();
}

// ================================
// Test Power Functions
// ================================

slay test_power_functions() {
    test_start("Power functions");
    
    # Test power
    assert_true(abs(math_pow(2.0, 3.0) - 8.0) < 1e-10);
    assert_true(abs(math_pow(9.0, 0.5) - 3.0) < 1e-10);
    assert_true(abs(math_pow(4.0, 0.0) - 1.0) < 1e-10);
    
    # Test square root
    assert_true(abs(math_sqrt(4.0) - 2.0) < 1e-10);
    assert_true(abs(math_sqrt(9.0) - 3.0) < 1e-10);
    assert_true(abs(math_sqrt(16.0) - 4.0) < 1e-10);
    
    # Test cube root
    assert_true(abs(math_cbrt(8.0) - 2.0) < 1e-6);
    assert_true(abs(math_cbrt(27.0) - 3.0) < 1e-6);
    assert_true(abs(math_cbrt(-8.0) + 2.0) < 1e-6);
    
    # Test exponential
    assert_true(abs(math_exp(0.0) - 1.0) < 1e-10);
    assert_true(abs(math_exp(1.0) - math_e()) < 1e-6);
    
    # Test exp2
    assert_true(abs(math_exp2(3.0) - 8.0) < 1e-6);
    assert_true(abs(math_exp2(0.0) - 1.0) < 1e-10);
    
    print_test_summary();
}

// ================================
// Test Logarithmic Functions
// ================================

slay test_logarithmic_functions() {
    test_start("Logarithmic functions");
    
    # Test natural logarithm
    assert_true(abs(math_log(1.0) - 0.0) < 1e-10);
    assert_true(abs(math_log(math_e()) - 1.0) < 1e-6);
    
    # Test log10
    assert_true(abs(math_log10(1.0) - 0.0) < 1e-10);
    assert_true(abs(math_log10(10.0) - 1.0) < 1e-6);
    assert_true(abs(math_log10(100.0) - 2.0) < 1e-6);
    
    # Test log2
    assert_true(abs(math_log2(1.0) - 0.0) < 1e-10);
    assert_true(abs(math_log2(2.0) - 1.0) < 1e-6);
    assert_true(abs(math_log2(8.0) - 3.0) < 1e-6);
    
    print_test_summary();
}

// ================================
// Test Trigonometric Functions
// ================================

slay test_trigonometric_functions() {
    test_start("Trigonometric functions");
    
    # Test sine
    assert_true(abs(math_sin(0.0) - 0.0) < 1e-10);
    assert_true(abs(math_sin(math_pi() / 2.0) - 1.0) < 1e-6);
    assert_true(abs(math_sin(math_pi()) - 0.0) < 1e-6);
    
    # Test cosine
    assert_true(abs(math_cos(0.0) - 1.0) < 1e-10);
    assert_true(abs(math_cos(math_pi() / 2.0) - 0.0) < 1e-6);
    assert_true(abs(math_cos(math_pi()) + 1.0) < 1e-6);
    
    # Test tangent
    assert_true(abs(math_tan(0.0) - 0.0) < 1e-10);
    assert_true(abs(math_tan(math_pi() / 4.0) - 1.0) < 1e-6);
    
    # Test inverse functions
    assert_true(abs(math_asin(0.0) - 0.0) < 1e-10);
    assert_true(abs(math_asin(1.0) - math_pi() / 2.0) < 1e-6);
    
    assert_true(abs(math_acos(1.0) - 0.0) < 1e-10);
    assert_true(abs(math_acos(0.0) - math_pi() / 2.0) < 1e-6);
    
    assert_true(abs(math_atan(0.0) - 0.0) < 1e-10);
    assert_true(abs(math_atan(1.0) - math_pi() / 4.0) < 1e-6);
    
    # Test atan2
    assert_true(abs(math_atan2(1.0, 1.0) - math_pi() / 4.0) < 1e-6);
    assert_true(abs(math_atan2(0.0, 1.0) - 0.0) < 1e-10);
    
    print_test_summary();
}

// ================================
// Test Hyperbolic Functions
// ================================

slay test_hyperbolic_functions() {
    test_start("Hyperbolic functions");
    
    # Test sinh
    assert_true(abs(math_sinh(0.0) - 0.0) < 1e-10);
    assert_true(abs(math_sinh(1.0) - 1.1752011936438014) < 1e-6);
    
    # Test cosh
    assert_true(abs(math_cosh(0.0) - 1.0) < 1e-10);
    assert_true(abs(math_cosh(1.0) - 1.5430806348152437) < 1e-6);
    
    # Test tanh
    assert_true(abs(math_tanh(0.0) - 0.0) < 1e-10);
    assert_true(abs(math_tanh(1.0) - 0.7615941559557649) < 1e-6);
    
    print_test_summary();
}

// ================================
// Test Rounding Functions
// ================================

slay test_rounding_functions() {
    test_start("Rounding functions");
    
    # Test floor
    assert_eq_int(math_floor(3.7).(normie), 3);
    assert_eq_int(math_floor(-3.7).(normie), -4);
    assert_eq_int(math_floor(5.0).(normie), 5);
    
    # Test ceil
    assert_eq_int(math_ceil(3.2).(normie), 4);
    assert_eq_int(math_ceil(-3.2).(normie), -3);
    assert_eq_int(math_ceil(5.0).(normie), 5);
    
    # Test round
    assert_eq_int(math_round(3.4).(normie), 3);
    assert_eq_int(math_round(3.6).(normie), 4);
    assert_eq_int(math_round(-3.4).(normie), -3);
    assert_eq_int(math_round(-3.6).(normie), -4);
    
    # Test trunc
    assert_eq_int(math_trunc(3.9).(normie), 3);
    assert_eq_int(math_trunc(-3.9).(normie), -3);
    
    # Test frac
    assert_true(abs(math_frac(3.7) - 0.7) < 1e-10);
    assert_true(abs(math_frac(-3.7) + 0.7) < 1e-10);
    
    print_test_summary();
}

// ================================
// Test Utility Functions
// ================================

slay test_utility_functions() {
    test_start("Utility functions");
    
    # Test conversion functions
    assert_true(abs(math_degrees(math_pi()) - 180.0) < 1e-6);
    assert_true(abs(math_radians(180.0) - math_pi()) < 1e-6);
    assert_true(abs(math_degrees(math_pi() / 2.0) - 90.0) < 1e-6);
    
    # Test NaN detection
    sus nan_val meal = 0.0 / 0.0;
    assert_true(math_is_nan(nan_val));
    assert_false(math_is_nan(5.0));
    
    # Test infinity detection
    sus inf_val meal = 1.0 / 0.0;
    assert_true(math_is_infinite(inf_val));
    assert_false(math_is_infinite(5.0));
    
    # Test finite detection
    assert_true(math_is_finite(5.0));
    assert_false(math_is_finite(inf_val));
    assert_false(math_is_finite(nan_val));
    
    print_test_summary();
}

// ================================
// Test Number Theory Functions
// ================================

slay test_number_theory() {
    test_start("Number theory functions");
    
    # Test GCD
    assert_eq_int(math_gcd(48, 18), 6);
    assert_eq_int(math_gcd(17, 13), 1);
    assert_eq_int(math_gcd(100, 25), 25);
    
    # Test LCM
    assert_eq_int(math_lcm(4, 6), 12);
    assert_eq_int(math_lcm(21, 6), 42);
    
    # Test factorial
    assert_eq_int(math_factorial(0), 1);
    assert_eq_int(math_factorial(1), 1);
    assert_eq_int(math_factorial(5), 120);
    assert_eq_int(math_factorial(6), 720);
    
    # Test fibonacci
    assert_eq_int(math_fibonacci(0), 0);
    assert_eq_int(math_fibonacci(1), 1);
    assert_eq_int(math_fibonacci(7), 13);
    assert_eq_int(math_fibonacci(10), 55);
    
    print_test_summary();
}

// ================================
// Test Random Functions
// ================================

slay test_random_functions() {
    test_start("Random functions");
    
    # Test seeded random
    math_seed_random(12345);
    sus r1 meal = math_random();
    math_seed_random(12345);
    sus r2 meal = math_random();
    assert_true(abs(r1 - r2) < 1e-10); # Same seed should produce same result
    
    # Test random range
    assert_true(math_random() >= 0.0 && math_random() <= 1.0);
    
    # Test random int
    sus rand_int normie = math_random_int(1, 10);
    assert_true(rand_int >= 1 && rand_int <= 10);
    
    # Test random float
    sus rand_float meal = math_random_float(5.0, 15.0);
    assert_true(rand_float >= 5.0 && rand_float <= 15.0);
    
    print_test_summary();
}

// ================================
// Test Geometry Functions
// ================================

slay test_geometry_functions() {
    test_start("Geometry functions");
    
    # Test 2D distance
    assert_true(abs(math_distance_2d(0.0, 0.0, 3.0, 4.0) - 5.0) < 1e-10);
    assert_true(abs(math_distance_2d(1.0, 1.0, 4.0, 5.0) - 5.0) < 1e-10);
    
    # Test 3D distance
    assert_true(abs(math_distance_3d(0.0, 0.0, 0.0, 3.0, 4.0, 0.0) - 5.0) < 1e-10);
    
    # Test dot product
    assert_eq_int(math_dot_product_2d(2.0, 3.0, 4.0, 5.0).(normie), 23);
    
    # Test cross product
    assert_eq_int(math_cross_product_2d(2.0, 3.0, 4.0, 5.0).(normie), -2);
    
    # Test magnitude
    assert_true(abs(math_magnitude_2d(3.0, 4.0) - 5.0) < 1e-10);
    
    print_test_summary();
}

// ================================
// Test Interpolation Functions
// ================================

slay test_interpolation_functions() {
    test_start("Interpolation functions");
    
    # Test lerp
    assert_true(abs(math_lerp(0.0, 10.0, 0.5) - 5.0) < 1e-10);
    assert_true(abs(math_lerp(2.0, 8.0, 0.25) - 3.5) < 1e-10);
    
    # Test inverse lerp
    assert_true(abs(math_inverse_lerp(0.0, 10.0, 5.0) - 0.5) < 1e-10);
    assert_true(abs(math_inverse_lerp(2.0, 8.0, 3.5) - 0.25) < 1e-10);
    
    # Test smoothstep
    assert_true(abs(math_smoothstep(0.0, 1.0, 0.5) - 0.5) < 1e-6);
    
    print_test_summary();
}

// ================================
// Performance Benchmarks
// ================================

slay test_performance_benchmarks() {
    test_start("Performance benchmarks");
    
    # Test performance of core functions
    sus iterations normie = 1000;
    
    # Benchmark sqrt
    bestie (i := 0; i < iterations; i++) {
        math_sqrt(i.(meal) + 1.0);
    }
    
    # Benchmark trigonometric functions
    bestie (i := 0; i < iterations; i++) {
        math_sin(i.(meal) / 100.0);
        math_cos(i.(meal) / 100.0);
    }
    
    # Benchmark exponential functions
    bestie (i := 0; i < iterations; i++) {
        math_exp(i.(meal) / 1000.0);
        math_log(i.(meal) + 1.0);
    }
    
    vibez.spill("Performance benchmarks completed");
    print_test_summary();
}

// ================================
// Statistical Functions Test
// ================================

slay test_statistical_functions() {
    test_start("Statistical functions");
    
    # Create test array
    sus test_data [meal] = [1.0, 2.0, 3.0, 4.0, 5.0];
    
    # Test sum
    assert_true(abs(math_sum(test_data) - 15.0) < 1e-10);
    
    # Test mean
    assert_true(abs(math_mean(test_data) - 3.0) < 1e-10);
    
    # Note: Median, variance, and std_dev require array sorting
    # which is not yet implemented in pure CURSED
    
    print_test_summary();
}

// ================================
// Main Test Runner
// ================================

slay main() {
    vibez.spill("=== Pure CURSED Math Library Test Suite ===");
    
    test_math_constants();
    test_basic_operations();
    test_power_functions();
    test_logarithmic_functions();
    test_trigonometric_functions();
    test_hyperbolic_functions();
    test_rounding_functions();
    test_utility_functions();
    test_number_theory();
    test_random_functions();
    test_geometry_functions();
    test_interpolation_functions();
    test_statistical_functions();
    test_performance_benchmarks();
    
    vibez.spill("=== All tests completed ===");
}
