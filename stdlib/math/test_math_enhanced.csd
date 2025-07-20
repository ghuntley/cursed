yeet "testz"
yeet "math"

# Comprehensive Enhanced Mathematical Functions Test Suite
test_start("Enhanced Mathematical Functions Test")

# Basic Arithmetic Tests
assert_eq_int(add(5, 3), 8)
assert_eq_int(add(-5, 3), -2)
assert_eq_int(add(0, 0), 0)

assert_eq_int(subtract(10, 4), 6)
assert_eq_int(subtract(5, 8), -3)
assert_eq_int(subtract(0, 0), 0)

assert_eq_int(multiply(6, 7), 42)
assert_eq_int(multiply(-3, 4), -12)
assert_eq_int(multiply(0, 5), 0)

assert_eq_int(divide(20, 4), 5)
assert_eq_int(divide(15, 3), 5)
assert_eq_int(divide(-12, 3), -4)

# Absolute Value Tests
assert_eq_int(abs_int(-42), 42)
assert_eq_int(abs_int(42), 42)
assert_eq_int(abs_int(0), 0)

assert_true(abs_float(-3.14) > 3.13 && abs_float(-3.14) < 3.15)
assert_true(abs_float(3.14) > 3.13 && abs_float(3.14) < 3.15)
assert_true(abs_float(0.0) == 0.0)

# Min/Max Tests
assert_eq_int(max_int(10, 5), 10)
assert_eq_int(max_int(-3, -7), -3)
assert_eq_int(max_int(0, 0), 0)

assert_eq_int(min_int(10, 5), 5)
assert_eq_int(min_int(-3, -7), -7)
assert_eq_int(min_int(0, 0), 0)

assert_true(max_float(3.14, 2.71) > 3.13 && max_float(3.14, 2.71) < 3.15)
assert_true(min_float(3.14, 2.71) > 2.70 && min_float(3.14, 2.71) < 2.72)

# Power Functions Tests
assert_eq_int(pow_int(2, 3), 8)
assert_eq_int(pow_int(5, 2), 25)
assert_eq_int(pow_int(10, 0), 1)
assert_eq_int(pow_int(3, 4), 81)

sus pow_result meal = pow_float(2.0, 3.0)
assert_true(pow_result > 7.9 && pow_result < 8.1)

# Square Root Tests
sus sqrt_result meal = sqrt(9.0)
assert_true(sqrt_result > 2.9 && sqrt_result < 3.1)

sus sqrt_result2 meal = sqrt(16.0)
assert_true(sqrt_result2 > 3.9 && sqrt_result2 < 4.1)

sus sqrt_result3 meal = sqrt(0.0)
assert_true(sqrt_result3 == 0.0)

# Exponential and Logarithmic Tests
sus exp_result meal = exp_float(0.0)
assert_true(exp_result > 0.9 && exp_result < 1.1)

sus exp_result2 meal = exp_float(1.0)
assert_true(exp_result2 > 2.7 && exp_result2 < 2.8)

sus ln_result meal = ln(1.0)
assert_true(ln_result == 0.0)

sus ln_result2 meal = ln(E)
assert_true(ln_result2 > 0.9 && ln_result2 < 1.1)

sus log10_result meal = log10(100.0)
assert_true(log10_result > 1.9 && log10_result < 2.1)

sus log2_result meal = log2(8.0)
assert_true(log2_result > 2.9 && log2_result < 3.1)

# Trigonometric Function Tests
sus sin_result meal = sin(0.0)
assert_true(sin_result > -0.1 && sin_result < 0.1)

sus sin_result2 meal = sin(PI / 2.0)
assert_true(sin_result2 > 0.9 && sin_result2 < 1.1)

sus cos_result meal = cos(0.0)
assert_true(cos_result > 0.9 && cos_result < 1.1)

sus cos_result2 meal = cos(PI)
assert_true(cos_result2 > -1.1 && cos_result2 < -0.9)

sus tan_result meal = tan(0.0)
assert_true(tan_result > -0.1 && tan_result < 0.1)

sus tan_result2 meal = tan(PI / 4.0)
assert_true(tan_result2 > 0.9 && tan_result2 < 1.1)

# Inverse Trigonometric Tests
sus asin_result meal = asin(0.0)
assert_true(asin_result > -0.1 && asin_result < 0.1)

sus asin_result2 meal = asin(1.0)
assert_true(asin_result2 > PI / 2.0 - 0.1 && asin_result2 < PI / 2.0 + 0.1)

sus acos_result meal = acos(1.0)
assert_true(acos_result > -0.1 && acos_result < 0.1)

sus atan_result meal = atan(0.0)
assert_true(atan_result > -0.1 && atan_result < 0.1)

sus atan_result2 meal = atan(1.0)
assert_true(atan_result2 > PI / 4.0 - 0.1 && atan_result2 < PI / 4.0 + 0.1)

sus atan2_result meal = atan2(1.0, 1.0)
assert_true(atan2_result > PI / 4.0 - 0.1 && atan2_result < PI / 4.0 + 0.1)

# Hyperbolic Function Tests
sus sinh_result meal = sinh(0.0)
assert_true(sinh_result > -0.1 && sinh_result < 0.1)

sus cosh_result meal = cosh(0.0)
assert_true(cosh_result > 0.9 && cosh_result < 1.1)

sus tanh_result meal = tanh(0.0)
assert_true(tanh_result > -0.1 && tanh_result < 0.1)

# Special Function Tests
sus factorial_result meal = factorial(5)
assert_true(factorial_result > 119.0 && factorial_result < 121.0)

sus factorial_result2 meal = factorial(0)
assert_true(factorial_result2 == 1.0)

sus factorial_result3 meal = factorial(1)
assert_true(factorial_result3 == 1.0)

sus gamma_result meal = gamma(1.0)
assert_true(gamma_result > 0.9 && gamma_result < 1.1)

sus gamma_result2 meal = gamma(2.0)
assert_true(gamma_result2 > 0.9 && gamma_result2 < 1.1)

# Helper Function Tests
sus floor_result meal = floor_float(3.7)
assert_true(floor_result == 3.0)

sus floor_result2 meal = floor_float(-2.3)
assert_true(floor_result2 == -3.0)

sus ceil_result meal = ceil_float(3.2)
assert_true(ceil_result == 4.0)

sus ceil_result2 meal = ceil_float(-2.8)
assert_true(ceil_result2 == -2.0)

sus round_result meal = round_float(3.6)
assert_true(round_result == 4.0)

sus round_result2 meal = round_float(3.4)
assert_true(round_result2 == 3.0)

sus round_result3 meal = round_float(-2.6)
assert_true(round_result3 == -3.0)

# Statistical Function Tests
sus values []meal = []meal{1.0, 2.0, 3.0, 4.0, 5.0}
sus mean_result meal = mean(values, 5)
assert_true(mean_result > 2.9 && mean_result < 3.1)

sus variance_result meal = variance(values, 5)
assert_true(variance_result > 2.4 && variance_result < 2.6)

sus std_dev_result meal = standard_deviation(values, 5)
assert_true(std_dev_result > 1.5 && std_dev_result < 1.7)

sus median_result meal = median(values, 5)
assert_true(median_result > 2.9 && median_result < 3.1)

# Numerical Analysis Tests
sus f_vals []meal = []meal{1.0, 4.0, 9.0, 16.0, 25.0}
sus integration_result meal = integrate_simpson(f_vals, 5, 1.0)
assert_true(integration_result > 0.0)

sus diff_result meal = differentiate_central(f_vals, 5, 1.0, 2)
assert_true(diff_result != 0.0)

sus linear_solution meal = solve_linear_2x2(2.0, 1.0, 5.0, 1.0, 1.0, 3.0)
assert_true(linear_solution > 1.9 && linear_solution < 2.1)

# Utility Function Tests
assert_eq_int(gcd(12, 8), 4)
assert_eq_int(gcd(17, 13), 1)
assert_eq_int(gcd(0, 5), 5)

assert_eq_int(lcm(4, 6), 12)
assert_eq_int(lcm(3, 7), 21)

assert_true(is_prime(2))
assert_true(is_prime(7))
assert_true(is_prime(17))
assert_false(is_prime(4))
assert_false(is_prime(9))
assert_false(is_prime(1))

assert_eq_int(fibonacci(0), 0)
assert_eq_int(fibonacci(1), 1)
assert_eq_int(fibonacci(5), 5)
assert_eq_int(fibonacci(8), 21)

# Edge Case Tests
sus sqrt_negative = sqrt(-1.0)
assert_true(sqrt_negative == 0.0)

sus ln_negative = ln(-1.0)
assert_true(ln_negative == 0.0)

sus ln_zero = ln(0.0)
assert_true(ln_zero == 0.0)

# Constants Tests
assert_true(PI > 3.14 && PI < 3.15)
assert_true(E > 2.71 && E < 2.72)
assert_true(LN2 > 0.69 && LN2 < 0.70)
assert_true(LN10 > 2.30 && LN10 < 2.31)
assert_true(SQRT2 > 1.41 && SQRT2 < 1.42)
assert_true(EPSILON > 0.0 && EPSILON < 1e-10)

vibez.spill("🔢 Enhanced mathematical functions tested successfully!")
vibez.spill("📐 Trigonometric, logarithmic, and statistical functions validated!")
vibez.spill("🎯 Special functions and numerical analysis working correctly!")

print_test_summary()
