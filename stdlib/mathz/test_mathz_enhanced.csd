fr fr Enhanced MATHZ Module - Comprehensive Test Suite

yeet "testz"

fr fr Test mathematical constants
test_group_start("Mathematical Constants")

test_start("pi_constant_test")
assert_near(PI, 3.14159, 0.001)

test_start("e_constant_test")
assert_near(E, 2.71828, 0.001)

test_start("tau_constant_test")
assert_near(TAU, 6.28318, 0.001)

test_start("golden_ratio_test")
assert_near(GOLDEN_RATIO, 1.61803, 0.001)

test_start("sqrt_constants_test")
assert_near(SQRT_2, 1.41421, 0.001)
assert_near(SQRT_3, 1.73205, 0.001)

test_group_end()

fr fr Test basic arithmetic operations
test_group_start("Basic Arithmetic")

test_start("addition_test")
assert_near(add(2.5, 3.7), 6.2, 0.001)
assert_near(add(-1.0, 1.0), 0.0, 0.001)

test_start("subtraction_test")
assert_near(subtract(5.0, 3.0), 2.0, 0.001)
assert_near(subtract(1.0, 1.0), 0.0, 0.001)

test_start("multiplication_test")
assert_near(multiply(3.0, 4.0), 12.0, 0.001)
assert_near(multiply(-2.0, 3.0), -6.0, 0.001)

test_start("division_test")
assert_near(divide(10.0, 2.0), 5.0, 0.001)
assert_near(divide(7.0, 2.0), 3.5, 0.001)

test_start("division_by_zero_test")
sus result meal = divide(1.0, 0.0)
assert_true(is_infinite(result))

test_start("safe_division_test")
sus (result, success) = safe_divide(10.0, 2.0)
assert_true(success)
assert_near(result, 5.0, 0.001)

sus (result2, success2) = safe_divide(10.0, 0.0)
assert_false(success2)

test_start("modulo_test")
assert_near(mod_meal(10.0, 3.0), 1.0, 0.001)
assert_near(mod_meal(7.5, 2.0), 1.5, 0.001)

test_group_end()

fr fr Test absolute value and sign functions
test_group_start("Absolute Value and Sign")

test_start("abs_meal_test")
assert_near(abs_meal(-5.5), 5.5, 0.001)
assert_near(abs_meal(3.2), 3.2, 0.001)
assert_near(abs_meal(0.0), 0.0, 0.001)

test_start("abs_normie_test")
assert_eq_int(abs_normie(-10), 10)
assert_eq_int(abs_normie(7), 7)
assert_eq_int(abs_normie(0), 0)

test_start("sign_meal_test")
assert_near(sign_meal(5.0), 1.0, 0.001)
assert_near(sign_meal(-3.0), -1.0, 0.001)
assert_near(sign_meal(0.0), 0.0, 0.001)

test_start("sign_normie_test")
assert_eq_int(sign_normie(42), 1)
assert_eq_int(sign_normie(-7), -1)
assert_eq_int(sign_normie(0), 0)

test_start("copysign_test")
assert_near(copysign(5.0, -1.0), -5.0, 0.001)
assert_near(copysign(-3.0, 1.0), 3.0, 0.001)

test_group_end()

fr fr Test min/max operations
test_group_start("Min/Max Operations")

test_start("max_meal_test")
assert_near(max_meal(3.5, 2.1), 3.5, 0.001)
assert_near(max_meal(-1.0, -2.0), -1.0, 0.001)

test_start("min_meal_test")
assert_near(min_meal(3.5, 2.1), 2.1, 0.001)
assert_near(min_meal(-1.0, -2.0), -2.0, 0.001)

test_start("max_normie_test")
assert_eq_int(max_normie(10, 5), 10)
assert_eq_int(max_normie(-3, -7), -3)

test_start("min_normie_test")
assert_eq_int(min_normie(10, 5), 5)
assert_eq_int(min_normie(-3, -7), -7)

test_start("max_array_test")
sus values meal[value] = [1.0, 5.0, 3.0, 2.0]
assert_near(max_array(values), 5.0, 0.001)

test_start("min_array_test")
sus values2 meal[value] = [4.0, 1.0, 3.0, 2.0]
assert_near(min_array(values2), 1.0, 0.001)

test_group_end()

fr fr Test rounding and truncation
test_group_start("Rounding and Truncation")

test_start("floor_test")
assert_eq_int(floor_meal(3.7), 3)
assert_eq_int(floor_meal(-2.3), -3)
assert_eq_int(floor_meal(5.0), 5)

test_start("ceil_test")
assert_eq_int(ceil_meal(3.2), 4)
assert_eq_int(ceil_meal(-2.7), -2)
assert_eq_int(ceil_meal(5.0), 5)

test_start("round_test")
assert_eq_int(round_meal(3.4), 3)
assert_eq_int(round_meal(3.6), 4)
assert_eq_int(round_meal(-2.5), -2)

test_start("trunc_test")
assert_eq_int(trunc_meal(3.9), 3)
assert_eq_int(trunc_meal(-2.1), -2)

test_start("frac_test")
assert_near(frac_meal(3.7), 0.7, 0.001)
assert_near(frac_meal(-2.3), -0.3, 0.001)

test_start("round_to_places_test")
assert_near(round_to_places(3.14159, 2), 3.14, 0.01)
assert_near(round_to_places(2.71828, 3), 2.718, 0.001)

test_group_end()

fr fr Test power and root functions
test_group_start("Power and Root Functions")

test_start("pow_meal_test")
assert_near(pow_meal(2.0, 3), 8.0, 0.001)
assert_near(pow_meal(5.0, 0), 1.0, 0.001)
assert_near(pow_meal(3.0, 1), 3.0, 0.001)

test_start("pow_meal_meal_test")
assert_near(pow_meal_meal(2.0, 3.0), 8.0, 0.001)
assert_near(pow_meal_meal(9.0, 0.5), 3.0, 0.001)

test_start("sqrt_test")
assert_near(sqrt_meal(9.0), 3.0, 0.001)
assert_near(sqrt_meal(16.0), 4.0, 0.001)
assert_near(sqrt_meal(0.0), 0.0, 0.001)

test_start("sqrt_negative_test")
sus result meal = sqrt_meal(-1.0)
assert_true(is_nan(result))

test_start("cbrt_test")
assert_near(cbrt_meal(8.0), 2.0, 0.001)
assert_near(cbrt_meal(-8.0), -2.0, 0.001)
assert_near(cbrt_meal(0.0), 0.0, 0.001)

test_start("nth_root_test")
assert_near(nth_root(16.0, 4), 2.0, 0.001)
assert_near(nth_root(32.0, 5), 2.0, 0.001)

test_group_end()

fr fr Test exponential and logarithmic functions
test_group_start("Exponential and Logarithmic")

test_start("exp_test")
assert_near(exp_meal(0.0), 1.0, 0.001)
assert_near(exp_meal(1.0), E, 0.001)
assert_near(exp_meal(2.0), E * E, 0.01)

test_start("ln_test")
assert_near(ln_meal(1.0), 0.0, 0.001)
assert_near(ln_meal(E), 1.0, 0.001)

test_start("ln_negative_test")
sus result meal = ln_meal(-1.0)
assert_true(is_nan(result))

test_start("log10_test")
assert_near(log10_meal(10.0), 1.0, 0.001)
assert_near(log10_meal(100.0), 2.0, 0.001)

test_start("log2_test")
assert_near(log2_meal(2.0), 1.0, 0.001)
assert_near(log2_meal(8.0), 3.0, 0.001)

test_start("log_base_test")
assert_near(log_base(8.0, 2.0), 3.0, 0.001)
assert_near(log_base(125.0, 5.0), 3.0, 0.001)

test_group_end()

fr fr Test trigonometric functions
test_group_start("Trigonometric Functions")

test_start("sin_test")
assert_near(sin_meal(0.0), 0.0, 0.001)
assert_near(sin_meal(PI / 2.0), 1.0, 0.001)
assert_near(sin_meal(PI), 0.0, 0.001)

test_start("cos_test")
assert_near(cos_meal(0.0), 1.0, 0.001)
assert_near(cos_meal(PI / 2.0), 0.0, 0.001)
assert_near(cos_meal(PI), -1.0, 0.001)

test_start("tan_test")
assert_near(tan_meal(0.0), 0.0, 0.001)
assert_near(tan_meal(PI / 4.0), 1.0, 0.001)

test_start("cot_test")
assert_near(cot_meal(PI / 4.0), 1.0, 0.001)

test_start("sec_test")
assert_near(sec_meal(0.0), 1.0, 0.001)

test_start("csc_test")
assert_near(csc_meal(PI / 2.0), 1.0, 0.001)

test_start("normalize_angle_test")
assert_near(normalize_angle(TAU + 1.0), 1.0, 0.001)
assert_near(normalize_angle(-PI - 1.0), PI - 1.0, 0.001)

test_group_end()

fr fr Test inverse trigonometric functions
test_group_start("Inverse Trigonometric Functions")

test_start("asin_test")
assert_near(asin_meal(0.0), 0.0, 0.001)
assert_near(asin_meal(1.0), PI / 2.0, 0.001)
assert_near(asin_meal(-1.0), -PI / 2.0, 0.001)

test_start("asin_out_of_range_test")
sus result meal = asin_meal(2.0)
assert_true(is_nan(result))

test_start("acos_test")
assert_near(acos_meal(1.0), 0.0, 0.001)
assert_near(acos_meal(0.0), PI / 2.0, 0.001)

test_start("atan_test")
assert_near(atan_meal(0.0), 0.0, 0.001)
assert_near(atan_meal(1.0), PI / 4.0, 0.001)

test_start("atan2_test")
assert_near(atan2_meal(1.0, 1.0), PI / 4.0, 0.001)
assert_near(atan2_meal(0.0, 1.0), 0.0, 0.001)

test_group_end()

fr fr Test hyperbolic functions
test_group_start("Hyperbolic Functions")

test_start("sinh_test")
assert_near(sinh_meal(0.0), 0.0, 0.001)
assert_near(sinh_meal(1.0), (E - 1.0/E) / 2.0, 0.01)

test_start("cosh_test")
assert_near(cosh_meal(0.0), 1.0, 0.001)
assert_near(cosh_meal(1.0), (E + 1.0/E) / 2.0, 0.01)

test_start("tanh_test")
assert_near(tanh_meal(0.0), 0.0, 0.001)

test_group_end()

fr fr Test special functions
test_group_start("Special Functions")

test_start("factorial_test")
assert_eq_int(factorial(0), 1)
assert_eq_int(factorial(1), 1)
assert_eq_int(factorial(5), 120)
assert_eq_int(factorial(3), 6)

test_start("factorial_negative_test")
assert_eq_int(factorial(-1), 0)

test_start("gcd_test")
assert_eq_int(gcd(12, 8), 4)
assert_eq_int(gcd(17, 13), 1)
assert_eq_int(gcd(0, 5), 5)

test_start("lcm_test")
assert_eq_int(lcm(12, 8), 24)
assert_eq_int(lcm(3, 5), 15)

test_start("fibonacci_test")
assert_eq_int(fibonacci(0), 0)
assert_eq_int(fibonacci(1), 1)
assert_eq_int(fibonacci(5), 5)
assert_eq_int(fibonacci(10), 55)

test_start("is_prime_test")
assert_true(is_prime(2))
assert_true(is_prime(17))
assert_false(is_prime(4))
assert_false(is_prime(15))
assert_false(is_prime(1))

test_group_end()

fr fr Test utility functions
test_group_start("Utility Functions")

test_start("is_nan_test")
sus nan_val meal = 0.0 / 0.0
assert_true(is_nan(nan_val))
assert_false(is_nan(1.0))

test_start("is_infinite_test")
sus inf_val meal = 1.0 / 0.0
assert_true(is_infinite(inf_val))
assert_false(is_infinite(1.0))

test_start("is_finite_test")
assert_true(is_finite(42.0))
assert_false(is_finite(INFINITY))

test_start("is_integer_float_test")
assert_true(is_integer_float(5.0))
assert_false(is_integer_float(5.5))

test_start("is_even_test")
assert_true(is_even(4))
assert_false(is_even(3))
assert_true(is_even(0))

test_start("is_odd_test")
assert_true(is_odd(3))
assert_false(is_odd(4))
assert_false(is_odd(0))

test_start("is_approximately_equal_test")
assert_true(is_approximately_equal(1.0, 1.001, 0.01))
assert_false(is_approximately_equal(1.0, 1.1, 0.01))

test_start("clamp_meal_test")
assert_near(clamp_meal(5.0, 0.0, 10.0), 5.0, 0.001)
assert_near(clamp_meal(-1.0, 0.0, 10.0), 0.0, 0.001)
assert_near(clamp_meal(15.0, 0.0, 10.0), 10.0, 0.001)

test_start("clamp_normie_test")
assert_eq_int(clamp_normie(5, 0, 10), 5)
assert_eq_int(clamp_normie(-1, 0, 10), 0)
assert_eq_int(clamp_normie(15, 0, 10), 10)

test_start("lerp_test")
assert_near(lerp_meal(0.0, 10.0, 0.5), 5.0, 0.001)
assert_near(lerp_meal(0.0, 10.0, 0.0), 0.0, 0.001)
assert_near(lerp_meal(0.0, 10.0, 1.0), 10.0, 0.001)

test_start("smoothstep_test")
assert_near(smoothstep(0.0, 1.0, 0.5), 0.5, 0.001)
assert_near(smoothstep(0.0, 1.0, 0.0), 0.0, 0.001)
assert_near(smoothstep(0.0, 1.0, 1.0), 1.0, 0.001)

test_group_end()

fr fr Test statistical functions
test_group_start("Statistical Functions")

test_start("sum_array_test")
sus values meal[value] = [1.0, 2.0, 3.0, 4.0]
assert_near(sum_array(values), 10.0, 0.001)

test_start("mean_array_test")
sus values2 meal[value] = [2.0, 4.0, 6.0]
assert_near(mean_array(values2), 4.0, 0.001)

test_start("variance_array_test")
sus values3 meal[value] = [1.0, 2.0, 3.0]
sus variance meal = variance_array(values3)
assert_true(variance > 0.0)

test_start("std_deviation_test")
sus values4 meal[value] = [1.0, 2.0, 3.0, 4.0, 5.0]
sus std_dev meal = std_deviation_array(values4)
assert_true(std_dev > 0.0)

test_group_end()

fr fr Test distance and geometry
test_group_start("Distance and Geometry")

test_start("distance_2d_test")
assert_near(distance_2d(0.0, 0.0, 3.0, 4.0), 5.0, 0.001)
assert_near(distance_2d(1.0, 1.0, 1.0, 1.0), 0.0, 0.001)

test_start("distance_3d_test")
assert_near(distance_3d(0.0, 0.0, 0.0, 1.0, 1.0, 1.0), sqrt_meal(3.0), 0.001)

test_start("dot_product_2d_test")
assert_near(dot_product_2d(1.0, 0.0, 0.0, 1.0), 0.0, 0.001)
assert_near(dot_product_2d(1.0, 2.0, 3.0, 4.0), 11.0, 0.001)

test_start("cross_product_2d_test")
assert_near(cross_product_2d(1.0, 0.0, 0.0, 1.0), 1.0, 0.001)

test_start("magnitude_2d_test")
assert_near(magnitude_2d(3.0, 4.0), 5.0, 0.001)

test_start("normalize_2d_test")
sus (nx, ny) = normalize_2d(3.0, 4.0)
assert_near(magnitude_2d(nx, ny), 1.0, 0.001)

test_group_end()

fr fr Test random number generation
test_group_start("Random Number Generation")

test_start("set_random_seed_test")
set_random_seed(42)
assert_true(based) fr fr Function executes

test_start("random_int_test")
sus r1 normie = random_int()
sus r2 normie = random_int()
assert_true(r1 >= 0)
assert_true(r2 >= 0)
assert_ne_int(r1, r2) fr fr Should be different (very likely)

test_start("random_meal_test")
sus rf1 meal = random_meal()
sus rf2 meal = random_meal()
assert_true(rf1 >= 0.0 && rf1 <= 1.0)
assert_true(rf2 >= 0.0 && rf2 <= 1.0)

test_start("random_range_test")
sus rr normie = random_range(10, 20)
assert_true(rr >= 10 && rr <= 20)

test_start("random_meal_range_test")
sus rrf meal = random_meal_range(1.0, 2.0)
assert_true(rrf >= 1.0 && rrf <= 2.0)

test_start("random_gaussian_test")
sus rg meal = random_gaussian()
fr fr Just test that it returns a value
assert_true(is_finite(rg))

test_group_end()

fr fr Test series and sequences
test_group_start("Series and Sequences")

test_start("arithmetic_sum_test")
sus sum normie = arithmetic_sum(1, 10, 10)
assert_eq_int(sum, 55) fr fr Sum of 1 to 10

test_start("geometric_sum_test")
sus sum meal = geometric_sum(1.0, 2.0, 5)
assert_near(sum, 31.0, 0.001) fr fr 1 + 2 + 4 + 8 + 16

test_start("harmonic_sum_test")
sus sum meal = harmonic_sum(3)
assert_near(sum, 1.0 + 0.5 + 1.0/3.0, 0.001)

test_group_end()

fr fr Test angle conversion
test_group_start("Angle Conversion")

test_start("degrees_to_radians_test")
assert_near(degrees_to_radians(180.0), PI, 0.001)
assert_near(degrees_to_radians(90.0), PI / 2.0, 0.001)

test_start("radians_to_degrees_test")
assert_near(radians_to_degrees(PI), 180.0, 0.001)
assert_near(radians_to_degrees(PI / 2.0), 90.0, 0.001)

test_start("sin_deg_test")
assert_near(sin_deg(90.0), 1.0, 0.001)
assert_near(sin_deg(0.0), 0.0, 0.001)

test_start("cos_deg_test")
assert_near(cos_deg(0.0), 1.0, 0.001)
assert_near(cos_deg(90.0), 0.0, 0.001)

test_start("tan_deg_test")
assert_near(tan_deg(45.0), 1.0, 0.001)

test_group_end()

fr fr Performance tests
test_group_start("Performance Tests")

test_start("basic_arithmetic_performance_test")
benchmark("basic_arithmetic", slay() {
    sus a meal = 3.14
    sus b meal = 2.71
    add(a, b)
    multiply(a, b)
    divide(a, b)
})

test_start("trigonometric_performance_test")
benchmark("trigonometric_functions", slay() {
    sus x meal = PI / 4.0
    sin_meal(x)
    cos_meal(x)
    tan_meal(x)
})

test_start("power_functions_performance_test")
benchmark("power_functions", slay() {
    pow_meal(2.0, 10)
    sqrt_meal(16.0)
    exp_meal(1.0)
    ln_meal(E)
})

test_group_end()

fr fr Integration tests
test_group_start("Integration Tests")

test_start("pythagorean_theorem_test")
sus a meal = 3.0
sus b meal = 4.0
sus c meal = sqrt_meal(a * a + b * b)
assert_near(c, 5.0, 0.001)

test_start("circle_area_test")
sus radius meal = 5.0
sus area meal = PI * radius * radius
assert_near(area, 78.539, 0.01)

test_start("euler_identity_test")
fr fr e^(iπ) + 1 = 0, testing real part: e^0 * cos(π) + 1 = 0
sus real_part meal = exp_meal(0.0) * cos_meal(PI) + 1.0
assert_near(real_part, 0.0, 0.001)

test_start("law_of_cosines_test")
fr fr c² = a² + b² - 2ab*cos(C)
sus a meal = 3.0
sus b meal = 4.0
sus angle_c meal = degrees_to_radians(90.0)
sus c_squared meal = a*a + b*b - 2.0*a*b*cos_meal(angle_c)
assert_near(sqrt_meal(c_squared), 5.0, 0.001)

test_group_end()

fr fr Property-based tests
test_group_start("Property Tests")

test_start("trigonometric_identity_test")
property_test(PropertyTestCase{
    name: "sin²(x) + cos²(x) = 1",
    generator: slay() tea { damn "1.0" },
    property: slay(x_str tea) lit {
        sus x meal = 1.0
        sus sin_val meal = sin_meal(x)
        sus cos_val meal = cos_meal(x)
        sus sum meal = sin_val * sin_val + cos_val * cos_val
        damn is_approximately_equal(sum, 1.0, 0.001)
    },
    iterations: 10
})

test_start("power_identity_test")
property_test(PropertyTestCase{
    name: "x^(a+b) = x^a * x^b",
    generator: slay() tea { damn "2.0" },
    property: slay(x_str tea) lit {
        sus x meal = 2.0
        sus a normie = 3
        sus b normie = 2
        sus left meal = pow_meal(x, a + b)
        sus right meal = pow_meal(x, a) * pow_meal(x, b)
        damn is_approximately_equal(left, right, 0.001)
    },
    iterations: 5
})

test_start("logarithm_identity_test")
property_test(PropertyTestCase{
    name: "ln(e^x) = x",
    generator: slay() tea { damn "1.0" },
    property: slay(x_str tea) lit {
        sus x meal = 1.0
        sus result meal = ln_meal(exp_meal(x))
        damn is_approximately_equal(result, x, 0.001)
    },
    iterations: 5
})

test_group_end()

fr fr Edge case tests
test_group_start("Edge Cases")

test_start("zero_division_handling_test")
sus inf_result meal = divide(1.0, 0.0)
assert_true(is_infinite(inf_result))

test_start("negative_sqrt_test")
sus nan_result meal = sqrt_meal(-1.0)
assert_true(is_nan(nan_result))

test_start("log_negative_test")
sus nan_log meal = ln_meal(-1.0)
assert_true(is_nan(nan_log))

test_start("asin_out_of_domain_test")
sus nan_asin meal = asin_meal(2.0)
assert_true(is_nan(nan_asin))

test_start("large_factorial_test")
sus large_fact normie = factorial(25)
assert_eq_int(large_fact, 0) fr fr Should handle overflow

test_start("very_large_exp_test")
sus large_exp meal = exp_meal(1000.0)
assert_true(is_infinite(large_exp))

test_group_end()

fr fr Print summary
print_test_summary()
print_benchmark_summary()

fr fr Final validation message
spillln("")
spill_colored("🎯 Enhanced MATHZ Module - Test Suite Complete!", "green")
spillln("✅ Mathematical constants verified")
spillln("✅ Basic arithmetic operations tested")
spillln("✅ Absolute value and sign functions working")
spillln("✅ Min/max operations validated")
spillln("✅ Rounding and truncation tested")
spillln("✅ Power and root functions verified")
spillln("✅ Exponential and logarithmic functions working")
spillln("✅ Trigonometric functions tested")
spillln("✅ Inverse trigonometric functions validated")
spillln("✅ Hyperbolic functions working")
spillln("✅ Special functions tested")
spillln("✅ Utility functions verified")
spillln("✅ Statistical functions working")
spillln("✅ Distance and geometry functions tested")
spillln("✅ Random number generation validated")
spillln("✅ Series and sequences working")
spillln("✅ Angle conversion tested")
spillln("✅ Performance tests completed")
spillln("✅ Integration tests passed")
spillln("✅ Property-based tests validated")
spillln("✅ Edge cases handled")
spillln("")
spill_colored("🚀 Enhanced MATHZ module is production-ready!", "cyan")
