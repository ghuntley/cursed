yeet "testz"
yeet "math"

# ==========================================
# CURSED Math Module - Comprehensive Test Suite
# ==========================================

test_start("math comprehensive test suite")

# ==========================================
# Basic Arithmetic Tests
# ==========================================

# Test integer arithmetic
sus result1 normie = add(5, 3)
assert_eq_int(result1, 8)

sus result2 normie = subtract(10, 4)
assert_eq_int(result2, 6)

sus result3 normie = multiply(6, 7)
assert_eq_int(result3, 42)

sus result4 normie = divide(15, 3)
assert_eq_int(result4, 5)

# Test power function
sus result5 normie = pow_int(2, 3)
assert_eq_int(result5, 8)

sus result6 normie = pow_int(5, 0)
assert_eq_int(result6, 1)

# Test absolute values
sus result7 normie = abs_int(-5)
assert_eq_int(result7, 5)

sus result8 normie = abs_int(10)
assert_eq_int(result8, 10)

# Test min/max functions
sus result9 normie = max_int(10, 20)
assert_eq_int(result9, 20)

sus result10 normie = min_int(10, 20)
assert_eq_int(result10, 10)

# ==========================================
# Float Operations Tests
# ==========================================

# Test float absolute value
sus float_abs1 meal = abs_float(-3.14)
assert_true(float_abs1 > 3.13)
assert_true(float_abs1 < 3.15)

# Test float min/max
sus float_max1 meal = max_float(2.5, 3.7)
assert_true(float_max1 > 3.6)
assert_true(float_max1 < 3.8)

sus float_min1 meal = min_float(2.5, 3.7)
assert_true(float_min1 > 2.4)
assert_true(float_min1 < 2.6)

# ==========================================
# Square Root Tests
# ==========================================

sus sqrt_result1 meal = sqrt(4.0)
assert_true(sqrt_result1 > 1.99)
assert_true(sqrt_result1 < 2.01)

sus sqrt_result2 meal = sqrt(9.0)
assert_true(sqrt_result2 > 2.99)
assert_true(sqrt_result2 < 3.01)

sus sqrt_result3 meal = sqrt(16.0)
assert_true(sqrt_result3 > 3.99)
assert_true(sqrt_result3 < 4.01)

# ==========================================
# Exponential Function Tests
# ==========================================

sus exp_result1 meal = exp_float(0.0)
assert_true(exp_result1 > 0.99)
assert_true(exp_result1 < 1.01)

sus exp_result2 meal = exp_float(1.0)
assert_true(exp_result2 > 2.7)
assert_true(exp_result2 < 2.8)

sus exp_result3 meal = exp2(3.0)
assert_true(exp_result3 > 7.9)
assert_true(exp_result3 < 8.1)

# ==========================================
# Logarithm Tests
# ==========================================

sus ln_result1 meal = ln(1.0)
assert_true(ln_result1 > -0.01)
assert_true(ln_result1 < 0.01)

sus ln_result2 meal = ln(E)
assert_true(ln_result2 > 0.99)
assert_true(ln_result2 < 1.01)

sus log10_result1 meal = log10(10.0)
assert_true(log10_result1 > 0.99)
assert_true(log10_result1 < 1.01)

sus log10_result2 meal = log10(100.0)
assert_true(log10_result2 > 1.99)
assert_true(log10_result2 < 2.01)

sus log2_result1 meal = log2(8.0)
assert_true(log2_result1 > 2.99)
assert_true(log2_result1 < 3.01)

# ==========================================
# Trigonometric Function Tests
# ==========================================

# Test sine function
sus sin_result1 meal = sin(0.0)
assert_true(sin_result1 > -0.01)
assert_true(sin_result1 < 0.01)

sus sin_result2 meal = sin(PI / 2.0)
assert_true(sin_result2 > 0.99)
assert_true(sin_result2 < 1.01)

sus sin_result3 meal = sin(PI)
assert_true(sin_result3 > -0.01)
assert_true(sin_result3 < 0.01)

# Test cosine function
sus cos_result1 meal = cos(0.0)
assert_true(cos_result1 > 0.99)
assert_true(cos_result1 < 1.01)

sus cos_result2 meal = cos(PI / 2.0)
assert_true(cos_result2 > -0.01)
assert_true(cos_result2 < 0.01)

sus cos_result3 meal = cos(PI)
assert_true(cos_result3 > -1.01)
assert_true(cos_result3 < -0.99)

# Test tangent function
sus tan_result1 meal = tan(0.0)
assert_true(tan_result1 > -0.01)
assert_true(tan_result1 < 0.01)

sus tan_result2 meal = tan(PI / 4.0)
assert_true(tan_result2 > 0.99)
assert_true(tan_result2 < 1.01)

# ==========================================
# Inverse Trigonometric Function Tests
# ==========================================

sus asin_result1 meal = asin(0.0)
assert_true(asin_result1 > -0.01)
assert_true(asin_result1 < 0.01)

sus asin_result2 meal = asin(1.0)
assert_true(asin_result2 > 1.56)
assert_true(asin_result2 < 1.58)

sus acos_result1 meal = acos(1.0)
assert_true(acos_result1 > -0.01)
assert_true(acos_result1 < 0.01)

sus acos_result2 meal = acos(0.0)
assert_true(acos_result2 > 1.56)
assert_true(acos_result2 < 1.58)

sus atan_result1 meal = atan(0.0)
assert_true(atan_result1 > -0.01)
assert_true(atan_result1 < 0.01)

sus atan_result2 meal = atan(1.0)
assert_true(atan_result2 > 0.78)
assert_true(atan_result2 < 0.79)

# Test atan2
sus atan2_result1 meal = atan2(1.0, 1.0)
assert_true(atan2_result1 > 0.78)
assert_true(atan2_result1 < 0.79)

# ==========================================
# Hyperbolic Function Tests
# ==========================================

sus sinh_result1 meal = sinh(0.0)
assert_true(sinh_result1 > -0.01)
assert_true(sinh_result1 < 0.01)

sus sinh_result2 meal = sinh(1.0)
assert_true(sinh_result2 > 1.17)
assert_true(sinh_result2 < 1.18)

sus cosh_result1 meal = cosh(0.0)
assert_true(cosh_result1 > 0.99)
assert_true(cosh_result1 < 1.01)

sus cosh_result2 meal = cosh(1.0)
assert_true(cosh_result2 > 1.54)
assert_true(cosh_result2 < 1.55)

sus tanh_result1 meal = tanh(0.0)
assert_true(tanh_result1 > -0.01)
assert_true(tanh_result1 < 0.01)

# ==========================================
# Special Function Tests
# ==========================================

sus factorial_result1 meal = factorial(0)
assert_true(factorial_result1 > 0.99)
assert_true(factorial_result1 < 1.01)

sus factorial_result2 meal = factorial(5)
assert_true(factorial_result2 > 119.0)
assert_true(factorial_result2 < 121.0)

sus gamma_result1 meal = gamma(1.0)
assert_true(gamma_result1 > 0.99)
assert_true(gamma_result1 < 1.01)

sus gamma_result2 meal = gamma(2.0)
assert_true(gamma_result2 > 0.99)
assert_true(gamma_result2 < 1.01)

# ==========================================
# Utility Function Tests
# ==========================================

# Test floor and ceiling
sus floor_result1 meal = floor_float(3.7)
assert_true(floor_result1 > 2.99)
assert_true(floor_result1 < 3.01)

sus floor_result2 meal = floor_float(-2.3)
assert_true(floor_result2 > -3.01)
assert_true(floor_result2 < -2.99)

sus ceil_result1 meal = ceil_float(3.2)
assert_true(ceil_result1 > 3.99)
assert_true(ceil_result1 < 4.01)

sus round_result1 meal = round_float(3.6)
assert_true(round_result1 > 3.99)
assert_true(round_result1 < 4.01)

sus round_result2 meal = round_float(3.4)
assert_true(round_result2 > 2.99)
assert_true(round_result2 < 3.01)

# ==========================================
# Statistical Function Tests
# ==========================================

# Create test array for statistical functions
sus test_values [5]meal
test_values[0] = 1.0
test_values[1] = 2.0
test_values[2] = 3.0
test_values[3] = 4.0
test_values[4] = 5.0

sus mean_result meal = mean(test_values, 5)
assert_true(mean_result > 2.99)
assert_true(mean_result < 3.01)

sus variance_result meal = variance(test_values, 5)
assert_true(variance_result > 2.4)
assert_true(variance_result < 2.6)

sus std_dev_result meal = standard_deviation(test_values, 5)
assert_true(std_dev_result > 1.5)
assert_true(std_dev_result < 1.6)

sus median_result meal = median(test_values, 5)
assert_true(median_result > 2.99)
assert_true(median_result < 3.01)

# ==========================================
# Number Theory Tests
# ==========================================

sus gcd_result1 normie = gcd(48, 18)
assert_eq_int(gcd_result1, 6)

sus gcd_result2 normie = gcd(17, 13)
assert_eq_int(gcd_result2, 1)

sus lcm_result1 normie = lcm(4, 6)
assert_eq_int(lcm_result1, 12)

# Test prime checking
assert_true(is_prime(2))
assert_true(is_prime(17))
assert_false(is_prime(4))
assert_false(is_prime(15))

# Test Fibonacci sequence
sus fib_result1 normie = fibonacci(0)
assert_eq_int(fib_result1, 0)

sus fib_result2 normie = fibonacci(1)
assert_eq_int(fib_result2, 1)

sus fib_result3 normie = fibonacci(6)
assert_eq_int(fib_result3, 8)

sus fib_result4 normie = fibonacci(10)
assert_eq_int(fib_result4, 55)

# ==========================================
# Power Function Tests
# ==========================================

sus pow_result1 meal = pow_float(2.0, 3.0)
assert_true(pow_result1 > 7.99)
assert_true(pow_result1 < 8.01)

sus pow_result2 meal = pow_float(5.0, 0.0)
assert_true(pow_result2 > 0.99)
assert_true(pow_result2 < 1.01)

sus pow_result3 meal = pow_float(10.0, 0.5)
assert_true(pow_result3 > 3.15)
assert_true(pow_result3 < 3.17)

# ==========================================
# Advanced Function Tests
# ==========================================

# Test Bessel function J₀
sus bessel_result1 meal = bessel_j0(0.0)
assert_true(bessel_result1 > 0.99)
assert_true(bessel_result1 < 1.01)

# Test beta function
sus beta_result1 meal = beta(1.0, 1.0)
assert_true(beta_result1 > 0.99)
assert_true(beta_result1 < 1.01)

# ==========================================
# Numerical Analysis Tests
# ==========================================

# Test linear solver (simple 2x2 system)
sus linear_result meal = solve_linear_2x2(2.0, 1.0, 3.0, 1.0, 1.0, 2.0)
assert_true(linear_result > 0.99)
assert_true(linear_result < 1.01)

# Test integration (Simpson's rule)
sus integration_values [5]meal
integration_values[0] = 1.0
integration_values[1] = 4.0
integration_values[2] = 6.0
integration_values[3] = 4.0
integration_values[4] = 1.0

sus integration_result meal = integrate_simpson(integration_values, 5, 0.5)
assert_true(integration_result > 5.0)
assert_true(integration_result < 6.0)

# Test differentiation
sus diff_values [3]meal
diff_values[0] = 1.0
diff_values[1] = 4.0
diff_values[2] = 9.0

sus diff_result meal = differentiate_central(diff_values, 3, 1.0, 1)
assert_true(diff_result > 3.9)
assert_true(diff_result < 4.1)

print_test_summary()
