yeet "testz"
yeet "pure_math"

test_start("Pure CURSED Math Module Tests")

// Test basic operations
assert_eq_float(add(2.5, 3.5), 6.0)
assert_eq_float(subtract(10.0, 4.0), 6.0)
assert_eq_float(multiply(3.0, 4.0), 12.0)
assert_eq_float(divide(15.0, 3.0), 5.0)

// Test constants
assert_true(pi() > 3.14 && pi() < 3.15)
assert_true(e() > 2.71 && e() < 2.72)
assert_true(tau() > 6.28 && tau() < 6.29)

// Test abs function
assert_eq_float(abs(-5.5), 5.5)
assert_eq_float(abs(5.5), 5.5)
assert_eq_float(abs(0.0), 0.0)

// Test min/max
assert_eq_float(min(3.0, 7.0), 3.0)
assert_eq_float(max(3.0, 7.0), 7.0)

// Test clamp
assert_eq_float(clamp(5.0, 2.0, 8.0), 5.0)
assert_eq_float(clamp(1.0, 2.0, 8.0), 2.0)
assert_eq_float(clamp(10.0, 2.0, 8.0), 8.0)

// Test sign
assert_eq_float(sign(5.0), 1.0)
assert_eq_float(sign(-5.0), -1.0)
assert_eq_float(sign(0.0), 0.0)

// Test power
assert_eq_float(power(2.0, 3.0), 8.0)
assert_eq_float(power(5.0, 2.0), 25.0)

// Test square root
sus sqrt_result meal = sqrt(16.0);
assert_true(sqrt_result > 3.99 && sqrt_result < 4.01)

// Test trigonometric functions
sus sin_result meal = sin(0.0);
assert_true(sin_result > -0.01 && sin_result < 0.01)

sus cos_result meal = cos(0.0);
assert_true(cos_result > 0.99 && cos_result < 1.01)

// Test rounding functions
assert_eq_float(floor(3.7), 3.0)
assert_eq_float(ceil(3.2), 4.0)
assert_eq_float(round(3.6), 4.0)
assert_eq_float(trunc(3.9), 3.0)

// Test logarithms
sus ln_result meal = ln(e());
assert_true(ln_result > 0.99 && ln_result < 1.01)

// Test exponential
sus exp_result meal = exp(1.0);
assert_true(exp_result > 2.71 && exp_result < 2.72)

// Test random functions
seed_random(42);
sus rand1 meal = random();
sus rand2 meal = random();
assert_true(rand1 >= 0.0 && rand1 <= 1.0)
assert_true(rand2 >= 0.0 && rand2 <= 1.0)
assert_true(rand1 != rand2)

sus rand_int normie = random_int(1, 10);
assert_true(rand_int >= 1 && rand_int <= 10)

// Test statistical functions
sus values [meal] = [1.0, 2.0, 3.0, 4.0, 5.0];
assert_eq_float(sum(values), 15.0)
assert_eq_float(mean(values), 3.0)

sus variance_result meal = variance(values);
assert_true(variance_result > 1.9 && variance_result < 2.1)

// Test utility functions
assert_false(is_nan(5.0))
assert_true(is_finite(5.0))

sus deg_result meal = degrees(pi());
assert_true(deg_result > 179.9 && deg_result < 180.1)

sus rad_result meal = radians(180.0);
assert_true(rad_result > 3.14 && rad_result < 3.15)

// Test integer functions
assert_eq_int(gcd(12, 8), 4)
assert_eq_int(lcm(4, 6), 12)
assert_eq_int(factorial(5), 120)
assert_eq_int(fibonacci(7), 13)

// Test geometry functions
sus distance meal = distance_2d(0.0, 0.0, 3.0, 4.0);
assert_true(distance > 4.99 && distance < 5.01)

assert_eq_float(dot_product_2d(1.0, 2.0, 3.0, 4.0), 11.0)

sus lerp_result meal = lerp(0.0, 10.0, 0.5);
assert_eq_float(lerp_result, 5.0)

print_test_summary()
