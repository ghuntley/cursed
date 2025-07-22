yeet "testz"
yeet "math"

fr fr Test enhanced mathematical function implementations

test_start("sorting algorithm in median function")

fr fr Test median with odd number of elements
sus odd_values [meal] = [3.0, 1.0, 5.0, 2.0, 4.0]
sus odd_median meal = median(odd_values)
fr fr After sorting: [1.0, 2.0, 3.0, 4.0, 5.0], median should be 3.0
assert_eq_float(odd_median, 3.0, 0.01)

fr fr Test median with even number of elements
sus even_values [meal] = [4.0, 2.0, 6.0, 1.0]
sus even_median meal = median(even_values)
fr fr After sorting: [1.0, 2.0, 4.0, 6.0], median should be (2.0 + 4.0) / 2 = 3.0
assert_eq_float(even_median, 3.0, 0.01)

fr fr Test median with single element
sus single_values [meal] = [42.0]
sus single_median meal = median(single_values)
assert_eq_float(single_median, 42.0, 0.01)

fr fr Test median with two elements
sus pair_values [meal] = [10.0, 20.0]
sus pair_median meal = median(pair_values)
assert_eq_float(pair_median, 15.0, 0.01)

test_start("math_pow_impl binary exponentiation")

fr fr Test basic power operations
sus pow_result1 meal = math_pow_impl(2.0, 3.0)
assert_eq_float(pow_result1, 8.0, 0.01)

sus pow_result2 meal = math_pow_impl(5.0, 2.0)
assert_eq_float(pow_result2, 25.0, 0.01)

fr fr Test power of 0
sus pow_zero meal = math_pow_impl(10.0, 0.0)
assert_eq_float(pow_zero, 1.0, 0.01)

fr fr Test power of 1
sus pow_one meal = math_pow_impl(42.0, 1.0)
assert_eq_float(pow_one, 42.0, 0.01)

fr fr Test negative exponent
sus pow_negative meal = math_pow_impl(2.0, -2.0)
assert_eq_float(pow_negative, 0.25, 0.01)

test_start("math_sin_impl Taylor series")

fr fr Test sine of 0
sus sin_zero meal = math_sin_impl(0.0)
assert_eq_float(sin_zero, 0.0, 0.01)

fr fr Test sine of π/2 (approximately 1.5708)
sus sin_pi_half meal = math_sin_impl(1.5708)
assert_eq_float(sin_pi_half, 1.0, 0.1) fr fr Allow larger tolerance for approximation

fr fr Test sine of π (approximately 3.14159)
sus sin_pi meal = math_sin_impl(3.14159)
assert_eq_float(sin_pi, 0.0, 0.1)

fr fr Test sine of small angle
sus sin_small meal = math_sin_impl(0.1)
fr fr sin(0.1) ≈ 0.0998
assert_eq_float(sin_small, 0.0998, 0.01)

test_start("math_cos_impl Taylor series")

fr fr Test cosine of 0
sus cos_zero meal = math_cos_impl(0.0)
assert_eq_float(cos_zero, 1.0, 0.01)

fr fr Test cosine of π/2 (approximately 1.5708)
sus cos_pi_half meal = math_cos_impl(1.5708)
assert_eq_float(cos_pi_half, 0.0, 0.1) fr fr Allow larger tolerance for approximation

fr fr Test cosine of π (approximately 3.14159)
sus cos_pi meal = math_cos_impl(3.14159)
assert_eq_float(cos_pi, -1.0, 0.1)

fr fr Test cosine of small angle
sus cos_small meal = math_cos_impl(0.1)
fr fr cos(0.1) ≈ 0.995
assert_eq_float(cos_small, 0.995, 0.01)

test_start("trigonometric identity verification")

fr fr Test sin²(x) + cos²(x) = 1 for various angles
sus test_angle meal = 0.5
sus sin_val meal = math_sin_impl(test_angle)
sus cos_val meal = math_cos_impl(test_angle)
sus identity_result meal = (sin_val * sin_val) + (cos_val * cos_val)
assert_eq_float(identity_result, 1.0, 0.1)

fr fr Test another angle
sus test_angle2 meal = 1.0
sus sin_val2 meal = math_sin_impl(test_angle2)
sus cos_val2 meal = math_cos_impl(test_angle2)
sus identity_result2 meal = (sin_val2 * sin_val2) + (cos_val2 * cos_val2)
assert_eq_float(identity_result2, 1.0, 0.1)

print_test_summary()
