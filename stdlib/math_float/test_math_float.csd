// Comprehensive Tests for Pure CURSED Math Float Module
yeet "testz"
yeet "mod"

// ================================
// Test Mathematical Constants
// ================================

slay test_constants() {
    test_start("Mathematical Constants");
    
    // Test PI
    assert_true(approximately_equal(PI(), 3.141592653589793, 1e-15));
    
    // Test E
    assert_true(approximately_equal(E(), 2.718281828459045, 1e-15));
    
    // Test TAU
    assert_true(approximately_equal(TAU(), 6.283185307179586, 1e-15));
    
    // Test SQRT_2
    assert_true(approximately_equal(SQRT_2(), 1.4142135623730951, 1e-15));
    
    // Test SQRT_3
    assert_true(approximately_equal(SQRT_3(), 1.7320508075688772, 1e-15));
    
    // Test logarithm constants
    assert_true(approximately_equal(LN_2(), 0.6931471805599453, 1e-15));
    assert_true(approximately_equal(LN_10(), 2.302585092994046, 1e-15));
    
    // Test special constants
    assert_true(approximately_equal(GOLDEN_RATIO(), 1.618033988749895, 1e-15));
    assert_true(approximately_equal(EULER_MASCHERONI(), 0.5772156649015329, 1e-15));
}

// ================================
// Test IEEE 754 Special Values
// ================================

slay test_special_values() {
    test_start("IEEE 754 Special Values");
    
    // Test infinity
    assert_true(is_infinite(INFINITY()));
    assert_true(is_infinite(NEG_INFINITY()));
    
    // Test NaN
    assert_true(is_nan(NAN()));
    
    // Test finite values
    assert_true(is_finite(42.0));
    assert_false(is_finite(INFINITY()));
    assert_false(is_finite(NAN()));
    
    // Test epsilon
    assert_true(EPSILON() > 0.0);
    assert_true(EPSILON() < 1e-15);
}

// ================================
// Test Basic Operations
// ================================

slay test_basic_operations() {
    test_start("Basic Operations");
    
    // Test abs
    assert_true(approximately_equal(abs(-5.5), 5.5, 1e-15));
    assert_true(approximately_equal(abs(5.5), 5.5, 1e-15));
    assert_true(approximately_equal(abs(0.0), 0.0, 1e-15));
    
    // Test sign
    assert_true(approximately_equal(sign(5.5), 1.0, 1e-15));
    assert_true(approximately_equal(sign(-5.5), -1.0, 1e-15));
    assert_true(approximately_equal(sign(0.0), 0.0, 1e-15));
    
    // Test min/max
    assert_true(approximately_equal(min(3.14, 2.71), 2.71, 1e-15));
    assert_true(approximately_equal(max(3.14, 2.71), 3.14, 1e-15));
    
    // Test clamp
    assert_true(approximately_equal(clamp(5.0, 1.0, 10.0), 5.0, 1e-15));
    assert_true(approximately_equal(clamp(-5.0, 1.0, 10.0), 1.0, 1e-15));
    assert_true(approximately_equal(clamp(15.0, 1.0, 10.0), 10.0, 1e-15));
}

// ================================
// Test Rounding Functions
// ================================

slay test_rounding() {
    test_start("Rounding Functions");
    
    // Test floor
    assert_true(approximately_equal(floor(3.7), 3.0, 1e-15));
    assert_true(approximately_equal(floor(-3.7), -4.0, 1e-15));
    assert_true(approximately_equal(floor(3.0), 3.0, 1e-15));
    
    // Test ceil
    assert_true(approximately_equal(ceil(3.2), 4.0, 1e-15));
    assert_true(approximately_equal(ceil(-3.2), -3.0, 1e-15));
    assert_true(approximately_equal(ceil(3.0), 3.0, 1e-15));
    
    // Test round
    assert_true(approximately_equal(round(3.4), 3.0, 1e-15));
    assert_true(approximately_equal(round(3.6), 4.0, 1e-15));
    assert_true(approximately_equal(round(-3.4), -3.0, 1e-15));
    assert_true(approximately_equal(round(-3.6), -4.0, 1e-15));
    
    // Test trunc
    assert_true(approximately_equal(trunc(3.7), 3.0, 1e-15));
    assert_true(approximately_equal(trunc(-3.7), -3.0, 1e-15));
    
    // Test frac
    assert_true(approximately_equal(frac(3.7), 0.7, 1e-14));
    assert_true(approximately_equal(frac(-3.7), -0.7, 1e-14));
}

// ================================
// Test Power Functions
// ================================

slay test_power_functions() {
    test_start("Power Functions");
    
    // Test integer powers
    assert_true(approximately_equal(pow_int(2.0, 0), 1.0, 1e-15));
    assert_true(approximately_equal(pow_int(2.0, 3), 8.0, 1e-15));
    assert_true(approximately_equal(pow_int(2.0, -2), 0.25, 1e-15));
    
    // Test sqrt
    assert_true(approximately_equal(sqrt(4.0), 2.0, 1e-14));
    assert_true(approximately_equal(sqrt(9.0), 3.0, 1e-14));
    assert_true(approximately_equal(sqrt(2.0), SQRT_2(), 1e-14));
    assert_true(is_nan(sqrt(-1.0)));
    
    // Test cbrt
    assert_true(approximately_equal(cbrt(8.0), 2.0, 1e-14));
    assert_true(approximately_equal(cbrt(27.0), 3.0, 1e-14));
    assert_true(approximately_equal(cbrt(-8.0), -2.0, 1e-14));
}

// ================================
// Test Exponential Functions
// ================================

slay test_exponential_functions() {
    test_start("Exponential Functions");
    
    // Test exp
    assert_true(approximately_equal(exp(0.0), 1.0, 1e-14));
    assert_true(approximately_equal(exp(1.0), E(), 1e-14));
    assert_true(approximately_equal(exp(2.0), E() * E(), 1e-14));
    
    // Test exp2
    assert_true(approximately_equal(exp2(0.0), 1.0, 1e-14));
    assert_true(approximately_equal(exp2(1.0), 2.0, 1e-14));
    assert_true(approximately_equal(exp2(3.0), 8.0, 1e-14));
}

// ================================
// Test Logarithmic Functions
// ================================

slay test_logarithmic_functions() {
    test_start("Logarithmic Functions");
    
    // Test ln
    assert_true(approximately_equal(ln(1.0), 0.0, 1e-14));
    assert_true(approximately_equal(ln(E()), 1.0, 1e-14));
    assert_true(approximately_equal(ln(E() * E()), 2.0, 1e-14));
    assert_true(is_nan(ln(-1.0)));
    assert_true(is_nan(ln(0.0)));
    
    // Test log10
    assert_true(approximately_equal(log10(1.0), 0.0, 1e-14));
    assert_true(approximately_equal(log10(10.0), 1.0, 1e-14));
    assert_true(approximately_equal(log10(100.0), 2.0, 1e-14));
    
    // Test log2
    assert_true(approximately_equal(log2(1.0), 0.0, 1e-14));
    assert_true(approximately_equal(log2(2.0), 1.0, 1e-14));
    assert_true(approximately_equal(log2(8.0), 3.0, 1e-14));
}

// ================================
// Test Trigonometric Functions
// ================================

slay test_trigonometric_functions() {
    test_start("Trigonometric Functions");
    
    // Test sin
    assert_true(approximately_equal(sin(0.0), 0.0, 1e-14));
    assert_true(approximately_equal(sin(PI() / 2.0), 1.0, 1e-14));
    assert_true(approximately_equal(sin(PI()), 0.0, 1e-14));
    assert_true(approximately_equal(sin(3.0 * PI() / 2.0), -1.0, 1e-14));
    
    // Test cos
    assert_true(approximately_equal(cos(0.0), 1.0, 1e-14));
    assert_true(approximately_equal(cos(PI() / 2.0), 0.0, 1e-14));
    assert_true(approximately_equal(cos(PI()), -1.0, 1e-14));
    assert_true(approximately_equal(cos(3.0 * PI() / 2.0), 0.0, 1e-14));
    
    // Test tan
    assert_true(approximately_equal(tan(0.0), 0.0, 1e-14));
    assert_true(approximately_equal(tan(PI() / 4.0), 1.0, 1e-14));
    assert_true(approximately_equal(tan(PI()), 0.0, 1e-14));
    
    // Test identity: sin^2 + cos^2 = 1
    sus angle meal = PI() / 3.0;
    sus sin_val meal = sin(angle);
    sus cos_val meal = cos(angle);
    assert_true(approximately_equal(sin_val * sin_val + cos_val * cos_val, 1.0, 1e-14));
}

// ================================
// Test Inverse Trigonometric Functions
// ================================

slay test_inverse_trig_functions() {
    test_start("Inverse Trigonometric Functions");
    
    // Test asin
    assert_true(approximately_equal(asin(0.0), 0.0, 1e-14));
    assert_true(approximately_equal(asin(1.0), PI() / 2.0, 1e-14));
    assert_true(approximately_equal(asin(-1.0), -PI() / 2.0, 1e-14));
    assert_true(is_nan(asin(2.0)));
    
    // Test acos
    assert_true(approximately_equal(acos(1.0), 0.0, 1e-14));
    assert_true(approximately_equal(acos(0.0), PI() / 2.0, 1e-14));
    assert_true(approximately_equal(acos(-1.0), PI(), 1e-14));
    assert_true(is_nan(acos(2.0)));
    
    // Test atan
    assert_true(approximately_equal(atan(0.0), 0.0, 1e-14));
    assert_true(approximately_equal(atan(1.0), PI() / 4.0, 1e-14));
    assert_true(approximately_equal(atan(-1.0), -PI() / 4.0, 1e-14));
    
    // Test atan2
    assert_true(approximately_equal(atan2(1.0, 1.0), PI() / 4.0, 1e-14));
    assert_true(approximately_equal(atan2(1.0, -1.0), 3.0 * PI() / 4.0, 1e-14));
    assert_true(approximately_equal(atan2(-1.0, 1.0), -PI() / 4.0, 1e-14));
}

// ================================
// Test Hyperbolic Functions
// ================================

slay test_hyperbolic_functions() {
    test_start("Hyperbolic Functions");
    
    // Test sinh
    assert_true(approximately_equal(sinh(0.0), 0.0, 1e-14));
    assert_true(approximately_equal(sinh(1.0), (E() - 1.0/E()) / 2.0, 1e-14));
    
    // Test cosh
    assert_true(approximately_equal(cosh(0.0), 1.0, 1e-14));
    assert_true(approximately_equal(cosh(1.0), (E() + 1.0/E()) / 2.0, 1e-14));
    
    // Test tanh
    assert_true(approximately_equal(tanh(0.0), 0.0, 1e-14));
    
    // Test identity: cosh^2 - sinh^2 = 1
    sus x meal = 1.5;
    sus sinh_val meal = sinh(x);
    sus cosh_val meal = cosh(x);
    assert_true(approximately_equal(cosh_val * cosh_val - sinh_val * sinh_val, 1.0, 1e-14));
}

// ================================
// Test Utility Functions
// ================================

slay test_utility_functions() {
    test_start("Utility Functions");
    
    // Test is_zero
    assert_true(is_zero(0.0));
    assert_true(is_zero(EPSILON() / 2.0));
    assert_false(is_zero(1.0));
    
    // Test fmod
    assert_true(approximately_equal(fmod(5.5, 2.0), 1.5, 1e-14));
    assert_true(approximately_equal(fmod(-5.5, 2.0), -1.5, 1e-14));
    assert_true(is_nan(fmod(5.5, 0.0)));
    
    // Test remainder
    assert_true(approximately_equal(remainder(5.5, 2.0), -0.5, 1e-14));
    assert_true(approximately_equal(remainder(-5.5, 2.0), 0.5, 1e-14));
}

// ================================
// Test Conversion Functions
// ================================

slay test_conversion_functions() {
    test_start("Conversion Functions");
    
    // Test degrees/radians
    assert_true(approximately_equal(degrees(PI()), 180.0, 1e-14));
    assert_true(approximately_equal(radians(180.0), PI(), 1e-14));
    assert_true(approximately_equal(degrees(PI() / 2.0), 90.0, 1e-14));
    assert_true(approximately_equal(radians(90.0), PI() / 2.0, 1e-14));
}

// ================================
// Test Linear Interpolation
// ================================

slay test_interpolation() {
    test_start("Linear Interpolation");
    
    // Test lerp
    assert_true(approximately_equal(lerp(0.0, 10.0, 0.0), 0.0, 1e-14));
    assert_true(approximately_equal(lerp(0.0, 10.0, 1.0), 10.0, 1e-14));
    assert_true(approximately_equal(lerp(0.0, 10.0, 0.5), 5.0, 1e-14));
    
    // Test inverse_lerp
    assert_true(approximately_equal(inverse_lerp(0.0, 10.0, 0.0), 0.0, 1e-14));
    assert_true(approximately_equal(inverse_lerp(0.0, 10.0, 10.0), 1.0, 1e-14));
    assert_true(approximately_equal(inverse_lerp(0.0, 10.0, 5.0), 0.5, 1e-14));
    
    // Test smoothstep
    assert_true(approximately_equal(smoothstep(0.0, 1.0, 0.0), 0.0, 1e-14));
    assert_true(approximately_equal(smoothstep(0.0, 1.0, 1.0), 1.0, 1e-14));
    assert_true(approximately_equal(smoothstep(0.0, 1.0, 0.5), 0.5, 1e-14));
}

// ================================
// Test Accuracy and Edge Cases
// ================================

slay test_accuracy_edge_cases() {
    test_start("Accuracy and Edge Cases");
    
    // Test very small numbers
    assert_true(approximately_equal(sin(1e-10), 1e-10, 1e-20));
    assert_true(approximately_equal(cos(1e-10), 1.0, 1e-20));
    
    // Test very large numbers
    assert_true(is_finite(sin(1e6)));
    assert_true(is_finite(cos(1e6)));
    
    // Test special angle values
    assert_true(approximately_equal(sin(PI() / 6.0), 0.5, 1e-14));
    assert_true(approximately_equal(cos(PI() / 3.0), 0.5, 1e-14));
    assert_true(approximately_equal(tan(PI() / 4.0), 1.0, 1e-14));
    
    // Test logarithm edge cases
    assert_true(approximately_equal(ln(1.0), 0.0, 1e-14));
    assert_true(is_nan(ln(0.0)));
    assert_true(is_nan(ln(-1.0)));
}

// ================================
// Run All Tests
// ================================

slay main() {
    test_constants();
    test_special_values();
    test_basic_operations();
    test_rounding();
    test_power_functions();
    test_exponential_functions();
    test_logarithmic_functions();
    test_trigonometric_functions();
    test_inverse_trig_functions();
    test_hyperbolic_functions();
    test_utility_functions();
    test_conversion_functions();
    test_interpolation();
    test_accuracy_edge_cases();
    
    print_test_summary();
}

main();
