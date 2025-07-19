yeet "testz"
yeet "math/trigonometry"

test_start("Trigonometry Module Test Suite")

# Helper function for approximate equality
slay approx_equal(a meal, b meal, tolerance meal) lit {
    damn math_abs(a - b) < tolerance
}

test_start("Basic Trigonometric Functions")

# Test sine function
assert_true(approx_equal(math_sin_impl(0.0), 0.0, 1e-10))
assert_true(approx_equal(math_sin_impl(MATH_PI / 2.0), 1.0, 1e-10))
assert_true(approx_equal(math_sin_impl(MATH_PI), 0.0, 1e-10))
assert_true(approx_equal(math_sin_impl(3.0 * MATH_PI / 2.0), -1.0, 1e-10))

# Test cosine function
assert_true(approx_equal(math_cos_impl(0.0), 1.0, 1e-10))
assert_true(approx_equal(math_cos_impl(MATH_PI / 2.0), 0.0, 1e-10))
assert_true(approx_equal(math_cos_impl(MATH_PI), -1.0, 1e-10))
assert_true(approx_equal(math_cos_impl(2.0 * MATH_PI), 1.0, 1e-10))

# Test tangent function
assert_true(approx_equal(math_tan_impl(0.0), 0.0, 1e-10))
assert_true(approx_equal(math_tan_impl(MATH_PI / 4.0), 1.0, 1e-10))
assert_true(approx_equal(math_tan_impl(-MATH_PI / 4.0), -1.0, 1e-10))

test_start("Inverse Trigonometric Functions")

# Test arcsine function
assert_true(approx_equal(math_asin_impl(0.0), 0.0, 1e-10))
assert_true(approx_equal(math_asin_impl(1.0), MATH_PI / 2.0, 1e-10))
assert_true(approx_equal(math_asin_impl(-1.0), -MATH_PI / 2.0, 1e-10))
assert_true(approx_equal(math_asin_impl(0.5), MATH_PI / 6.0, 1e-10))

# Test arccosine function
assert_true(approx_equal(math_acos_impl(1.0), 0.0, 1e-10))
assert_true(approx_equal(math_acos_impl(0.0), MATH_PI / 2.0, 1e-10))
assert_true(approx_equal(math_acos_impl(-1.0), MATH_PI, 1e-10))

# Test arctangent function
assert_true(approx_equal(math_atan_impl(0.0), 0.0, 1e-10))
assert_true(approx_equal(math_atan_impl(1.0), MATH_PI / 4.0, 1e-10))
assert_true(approx_equal(math_atan_impl(-1.0), -MATH_PI / 4.0, 1e-10))

# Test atan2 function
assert_true(approx_equal(math_atan2_impl(1.0, 1.0), MATH_PI / 4.0, 1e-10))
assert_true(approx_equal(math_atan2_impl(1.0, 0.0), MATH_PI / 2.0, 1e-10))
assert_true(approx_equal(math_atan2_impl(0.0, 1.0), 0.0, 1e-10))
assert_true(approx_equal(math_atan2_impl(-1.0, -1.0), -3.0 * MATH_PI / 4.0, 1e-10))

test_start("Hyperbolic Functions")

# Test hyperbolic sine
assert_true(approx_equal(math_sinh_impl(0.0), 0.0, 1e-10))
assert_true(math_sinh_impl(1.0) > 1.0)  # sinh(1) ≈ 1.175
assert_true(math_sinh_impl(-1.0) < -1.0)  # sinh(-1) ≈ -1.175

# Test hyperbolic cosine
assert_true(approx_equal(math_cosh_impl(0.0), 1.0, 1e-10))
assert_true(math_cosh_impl(1.0) > 1.0)  # cosh(1) ≈ 1.543
assert_true(math_cosh_impl(-1.0) > 1.0)  # cosh(-1) ≈ 1.543 (even function)

# Test hyperbolic tangent
assert_true(approx_equal(math_tanh_impl(0.0), 0.0, 1e-10))
assert_true(math_tanh_impl(1.0) > 0.0 && math_tanh_impl(1.0) < 1.0)
assert_true(math_tanh_impl(-1.0) < 0.0 && math_tanh_impl(-1.0) > -1.0)

test_start("Exponential and Logarithmic Functions")

# Test exponential function
assert_true(approx_equal(math_exp_impl(0.0), 1.0, 1e-10))
assert_true(approx_equal(math_exp_impl(1.0), MATH_E, 1e-10))
assert_true(math_exp_impl(2.0) > 7.0 && math_exp_impl(2.0) < 8.0)  # e^2 ≈ 7.389

# Test natural logarithm
assert_true(approx_equal(math_log_impl(1.0), 0.0, 1e-10))
assert_true(approx_equal(math_log_impl(MATH_E), 1.0, 1e-10))
assert_true(math_log_impl(10.0) > 2.0 && math_log_impl(10.0) < 3.0)  # ln(10) ≈ 2.303

# Test log base 10
assert_true(approx_equal(math_log10_impl(1.0), 0.0, 1e-10))
assert_true(approx_equal(math_log10_impl(10.0), 1.0, 1e-10))
assert_true(approx_equal(math_log10_impl(100.0), 2.0, 1e-10))

# Test log base 2
assert_true(approx_equal(math_log2_impl(1.0), 0.0, 1e-10))
assert_true(approx_equal(math_log2_impl(2.0), 1.0, 1e-10))
assert_true(approx_equal(math_log2_impl(8.0), 3.0, 1e-10))

test_start("Square Root Function")

# Test square root
assert_true(approx_equal(math_sqrt_impl(0.0), 0.0, 1e-10))
assert_true(approx_equal(math_sqrt_impl(1.0), 1.0, 1e-10))
assert_true(approx_equal(math_sqrt_impl(4.0), 2.0, 1e-10))
assert_true(approx_equal(math_sqrt_impl(9.0), 3.0, 1e-10))
assert_true(approx_equal(math_sqrt_impl(16.0), 4.0, 1e-10))
assert_true(approx_equal(math_sqrt_impl(25.0), 5.0, 1e-10))

test_start("Rounding Functions")

# Test ceiling function
assert_true(approx_equal(math_ceil_impl(1.1), 2.0, 1e-10))
assert_true(approx_equal(math_ceil_impl(1.0), 1.0, 1e-10))
assert_true(approx_equal(math_ceil_impl(-1.1), -1.0, 1e-10))
assert_true(approx_equal(math_ceil_impl(-1.0), -1.0, 1e-10))

# Test floor function
assert_true(approx_equal(math_floor_impl(1.9), 1.0, 1e-10))
assert_true(approx_equal(math_floor_impl(1.0), 1.0, 1e-10))
assert_true(approx_equal(math_floor_impl(-1.1), -2.0, 1e-10))
assert_true(approx_equal(math_floor_impl(-1.0), -1.0, 1e-10))

test_start("Angle Normalization")

# Test angle normalization
assert_true(approx_equal(normalize_angle(0.0), 0.0, 1e-10))
assert_true(approx_equal(normalize_angle(MATH_PI), MATH_PI, 1e-10))
assert_true(approx_equal(normalize_angle(-MATH_PI), -MATH_PI, 1e-10))
assert_true(approx_equal(normalize_angle(3.0 * MATH_PI), MATH_PI, 1e-10))
assert_true(approx_equal(normalize_angle(-3.0 * MATH_PI), -MATH_PI, 1e-10))

test_start("Mathematical Identities")

# Test sin²(x) + cos²(x) = 1
sus test_angle meal = MATH_PI / 6.0  # 30 degrees
sus sin_val meal = math_sin_impl(test_angle)
sus cos_val meal = math_cos_impl(test_angle)
assert_true(approx_equal(sin_val * sin_val + cos_val * cos_val, 1.0, 1e-10))

# Test sin(2x) = 2*sin(x)*cos(x)
sus sin_2x meal = math_sin_impl(2.0 * test_angle)
sus sin_x meal = math_sin_impl(test_angle)
sus cos_x meal = math_cos_impl(test_angle)
assert_true(approx_equal(sin_2x, 2.0 * sin_x * cos_x, 1e-10))

# Test exp(ln(x)) = x for x > 0
sus test_val meal = 5.0
sus log_val meal = math_log_impl(test_val)
sus exp_log meal = math_exp_impl(log_val)
assert_true(approx_equal(exp_log, test_val, 1e-10))

test_start("Edge Cases")

# Test very small values
assert_true(approx_equal(math_sin_impl(1e-10), 1e-10, 1e-15))
assert_true(approx_equal(math_cos_impl(1e-10), 1.0, 1e-15))

# Test values near π/2 for tangent
sus near_pi_2 meal = MATH_PI / 2.0 - 1e-10
assert_true(math_abs(math_tan_impl(near_pi_2)) > 1e9)  # Should be very large

print_test_summary()
