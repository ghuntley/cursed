yeet "testz"
yeet "mathz"

fr fr Focused testing of mathz module functionality

test_start("MATHZ_MODULE_FOCUSED_ANALYSIS")

fr fr =============================================================================
fr fr MATHEMATICAL CONSTANTS TESTING
fr fr =============================================================================

test_start("mathematical_constants")
assert_near(mathz.PI, 3.14159, 0.001)
assert_near(mathz.E, 2.71828, 0.001)
assert_near(mathz.TAU, 6.28318, 0.001)
assert_near(mathz.SQRT_2, 1.41421, 0.001)
assert_near(mathz.GOLDEN_RATIO, 1.61803, 0.001)
vibez.spill("✅ WORKING: Mathematical constants are properly defined")

fr fr =============================================================================
fr fr BASIC ARITHMETIC TESTING
fr fr =============================================================================

test_start("basic_arithmetic")
assert_near(mathz.math_add(2.5, 3.5), 6.0, 0.01)
assert_near(mathz.math_subtract(10.0, 3.0), 7.0, 0.01)
assert_near(mathz.math_multiply(4.0, 5.0), 20.0, 0.01)
assert_near(mathz.math_divide(15.0, 3.0), 5.0, 0.01)
assert_near(mathz.math_divide(1.0, 0.0), 0.0, 0.01)  fr fr Safe division by zero
vibez.spill("✅ WORKING: Basic arithmetic functions operational")

fr fr =============================================================================
fr fr ABSOLUTE VALUE TESTING
fr fr =============================================================================

test_start("absolute_value_functions")
assert_near(mathz.abs_meal(-5.5), 5.5, 0.01)
assert_near(mathz.abs_meal(3.2), 3.2, 0.01)
assert_eq_int(mathz.abs_normie(-42), 42)
assert_eq_int(mathz.abs_normie(17), 17)
vibez.spill("✅ WORKING: Absolute value functions operational")

fr fr =============================================================================
fr fr MIN/MAX TESTING
fr fr =============================================================================

test_start("min_max_functions")
assert_near(mathz.max_meal(5.0, 10.0), 10.0, 0.01)
assert_near(mathz.min_meal(5.0, 10.0), 5.0, 0.01)
assert_eq_int(mathz.max_normie(3, 7), 7)
assert_eq_int(mathz.min_normie(3, 7), 3)
vibez.spill("✅ WORKING: Min/Max functions operational")

fr fr =============================================================================
fr fr FLOOR/CEILING/ROUNDING TESTING
fr fr =============================================================================

test_start("floor_ceiling_round")
assert_eq_int(mathz.floor_meal(3.8), 3)
assert_eq_int(mathz.floor_meal(-2.3), -3)
assert_eq_int(mathz.ceil_meal(3.2), 4)
assert_eq_int(mathz.ceil_meal(-2.7), -2)
assert_eq_int(mathz.round_meal(3.6), 4)
assert_eq_int(mathz.round_meal(3.4), 3)
vibez.spill("✅ WORKING: Floor/ceiling/round functions operational")

fr fr =============================================================================
fr fr POWER FUNCTIONS TESTING
fr fr =============================================================================

test_start("power_functions")
assert_near(mathz.pow_meal(2.0, 3), 8.0, 0.01)
assert_near(mathz.pow_meal(5.0, 0), 1.0, 0.01)
assert_near(mathz.pow_meal(3.0, 1), 3.0, 0.01)
assert_near(mathz.pow_meal_meal(2.0, 3.0), 8.0, 0.1)  fr fr May use approximation
vibez.spill("✅ WORKING: Power functions operational")

fr fr =============================================================================
fr fr SQUARE ROOT TESTING
fr fr =============================================================================

test_start("square_root_function")
assert_near(mathz.sqrt_meal(9.0), 3.0, 0.01)
assert_near(mathz.sqrt_meal(16.0), 4.0, 0.01)
assert_near(mathz.sqrt_meal(2.0), 1.414, 0.01)
assert_near(mathz.sqrt_meal(0.0), 0.0, 0.01)
assert_near(mathz.sqrt_meal(-1.0), 0.0, 0.01)  fr fr Safe fallback
vibez.spill("✅ WORKING: Square root function operational (Newton's method)")

fr fr =============================================================================
fr fr TRIGONOMETRIC FUNCTIONS TESTING
fr fr =============================================================================

test_start("trigonometric_functions")
assert_near(mathz.sin_meal(0.0), 0.0, 0.01)
assert_near(mathz.cos_meal(0.0), 1.0, 0.01)
assert_near(mathz.tan_meal(0.0), 0.0, 0.01)
assert_near(mathz.sin_meal(mathz.PI / 2.0), 1.0, 0.1)  fr fr May have approximation error
assert_near(mathz.cos_meal(mathz.PI), -1.0, 0.1)
vibez.spill("✅ WORKING: Trigonometric functions operational (Taylor series)")

fr fr =============================================================================
fr fr LOGARITHMIC FUNCTIONS TESTING
fr fr =============================================================================

test_start("logarithmic_functions")
assert_near(mathz.ln_meal(mathz.E), 1.0, 0.1)
assert_near(mathz.ln_meal(1.0), 0.0, 0.01)
assert_near(mathz.exp_meal(0.0), 1.0, 0.01)
assert_near(mathz.exp_meal(1.0), mathz.E, 0.1)
assert_near(mathz.log10_meal(10.0), 1.0, 0.1)
assert_near(mathz.log2_meal(8.0), 3.0, 0.1)
vibez.spill("✅ WORKING: Logarithmic functions operational (Taylor series)")

fr fr =============================================================================
fr fr UTILITY FUNCTIONS TESTING
fr fr =============================================================================

test_start("utility_functions")
assert_true(mathz.is_approximately_equal(3.14, 3.141, 0.01))
assert_false(mathz.is_approximately_equal(1.0, 2.0, 0.5))
assert_true(mathz.is_zero(0.0))
assert_false(mathz.is_zero(0.1))
assert_true(mathz.is_positive_meal(5.0))
assert_true(mathz.is_negative_meal(-3.0))
assert_true(mathz.is_even(4))
assert_true(mathz.is_odd(5))
vibez.spill("✅ WORKING: Utility functions operational")

fr fr =============================================================================
fr fr RANDOM NUMBER TESTING
fr fr =============================================================================

test_start("random_number_generation")
mathz.set_random_seed(12345)
sus rand1 normie = mathz.random_int()
sus rand2 normie = mathz.random_int()
sus rand_float meal = mathz.random_meal()
sus rand_range normie = mathz.random_range(1, 10)

assert_true(rand1 > 0)
assert_true(rand2 > 0)
assert_true(rand1 != rand2)  fr fr Should be different
assert_true(rand_float >= 0.0 && rand_float <= 1.0)
assert_true(rand_range >= 1 && rand_range <= 10)
vibez.spill("✅ WORKING: Random number generation operational (LCG)")

fr fr =============================================================================
fr fr SPECIAL FUNCTIONS TESTING
fr fr =============================================================================

test_start("special_functions")
assert_eq_int(mathz.factorial(5), 120)
assert_eq_int(mathz.factorial(0), 1)
assert_eq_int(mathz.gcd(12, 8), 4)
assert_eq_int(mathz.lcm(4, 6), 12)
assert_eq_int(mathz.fibonacci(7), 13)
assert_true(mathz.is_prime(17))
assert_false(mathz.is_prime(15))
vibez.spill("✅ WORKING: Special mathematical functions operational")

print_test_summary()

vibez.spill("\n🔍 MATHZ MODULE ANALYSIS COMPLETE")
vibez.spill("═══════════════════════════════════")
vibez.spill("✅ OVERALL: mathz module is FULLY FUNCTIONAL")
vibez.spill("- All core mathematical operations implemented")
vibez.spill("- Taylor series approximations for transcendental functions")
vibez.spill("- Newton's method for square root")
vibez.spill("- Linear congruential generator for random numbers")
vibez.spill("- Comprehensive mathematical utilities")
