yeet "testz"
yeet "mathz"

fr fr Mathematical Functions Test Suite

test_start("Mathematical Constants")

fr fr Test mathematical constants
assert_true(mathz.PI > 3.14 && mathz.PI < 3.15)
assert_true(mathz.E > 2.71 && mathz.E < 2.72)
assert_true(mathz.TAU > 6.28 && mathz.TAU < 6.29)

test_start("Basic Arithmetic")

fr fr Test basic arithmetic operations
sus add_result meal = mathz.math_add(2.0, 3.0)
assert_true(add_result == 5.0)

sus sub_result meal = mathz.math_subtract(5.0, 3.0)
assert_true(sub_result == 2.0)

sus mul_result meal = mathz.math_multiply(4.0, 3.0)
assert_true(mul_result == 12.0)

sus div_result meal = mathz.math_divide(10.0, 2.0)
assert_true(div_result == 5.0)

fr fr Test division by zero safety
sus div_zero meal = mathz.math_divide(5.0, 0.0)
assert_true(div_zero == 0.0)

test_start("Absolute Value Functions")

fr fr Test absolute value for floats
sus abs1 meal = mathz.abs_meal(-5.5)
assert_true(abs1 == 5.5)

sus abs2 meal = mathz.abs_meal(3.14)
assert_true(abs2 == 3.14)

fr fr Test absolute value for integers
sus abs3 normie = mathz.abs_normie(-42)
assert_eq_int(abs3, 42)

sus abs4 normie = mathz.abs_normie(15)
assert_eq_int(abs4, 15)

test_start("Min/Max Functions")

fr fr Test min/max for floats
sus max1 meal = mathz.max_meal(3.14, 2.71)
assert_true(max1 == 3.14)

sus min1 meal = mathz.min_meal(3.14, 2.71)
assert_true(min1 == 2.71)

fr fr Test min/max for integers
sus max2 normie = mathz.max_normie(42, 24)
assert_eq_int(max2, 42)

sus min2 normie = mathz.min_normie(42, 24)
assert_eq_int(min2, 24)

test_start("Floor, Ceiling, and Rounding")

fr fr Test floor function
sus floor1 normie = mathz.floor_meal(3.7)
assert_eq_int(floor1, 3)

sus floor2 normie = mathz.floor_meal(-2.3)
assert_eq_int(floor2, -3)

fr fr Test ceiling function
sus ceil1 normie = mathz.ceil_meal(3.2)
assert_eq_int(ceil1, 4)

sus ceil2 normie = mathz.ceil_meal(-2.7)
assert_eq_int(ceil2, -2)

fr fr Test rounding function
sus round1 normie = mathz.round_meal(3.7)
assert_eq_int(round1, 4)

sus round2 normie = mathz.round_meal(3.2)
assert_eq_int(round2, 3)

test_start("Power Functions")

fr fr Test integer power function
sus pow1 meal = mathz.pow_meal(2.0, 3)
assert_true(pow1 == 8.0)

sus pow2 meal = mathz.pow_meal(5.0, 0)
assert_true(pow2 == 1.0)

sus pow3 meal = mathz.pow_meal(3.0, 1)
assert_true(pow3 == 3.0)

test_start("Square Root")

fr fr Test square root function
sus sqrt1 meal = mathz.sqrt_meal(25.0)
assert_true(sqrt1 >= 4.9 && sqrt1 <= 5.1)

sus sqrt2 meal = mathz.sqrt_meal(0.0)
assert_true(sqrt2 == 0.0)

sus sqrt3 meal = mathz.sqrt_meal(-5.0)
assert_true(sqrt3 == 0.0) fr fr Safe fallback for negative

test_start("Trigonometric Functions")

fr fr Test sine function
sus sin1 meal = mathz.sin_meal(0.0)
assert_true(sin1 >= -0.1 && sin1 <= 0.1)

sus sin2 meal = mathz.sin_meal(mathz.PI / 2.0)
assert_true(sin2 >= 0.9 && sin2 <= 1.1)

fr fr Test cosine function
sus cos1 meal = mathz.cos_meal(0.0)
assert_true(cos1 >= 0.9 && cos1 <= 1.1)

sus cos2 meal = mathz.cos_meal(mathz.PI)
assert_true(cos2 >= -1.1 && cos2 <= -0.9)

test_start("Utility Functions")

fr fr Test factorial function
sus fact1 normie = mathz.factorial(0)
assert_eq_int(fact1, 1)

sus fact2 normie = mathz.factorial(5)
assert_eq_int(fact2, 120)

fr fr Test GCD function
sus gcd1 normie = mathz.gcd(12, 18)
assert_eq_int(gcd1, 6)

sus gcd2 normie = mathz.gcd(7, 13)
assert_eq_int(gcd2, 1)

fr fr Test LCM function
sus lcm1 normie = mathz.lcm(12, 18)
assert_eq_int(lcm1, 36)

fr fr Test even/odd checks
assert_true(mathz.is_even(4))
assert_false(mathz.is_even(5))
assert_true(mathz.is_odd(5))
assert_false(mathz.is_odd(4))

test_start("Random Number Generation")

fr fr Test random number generation
mathz.set_random_seed(12345)
sus rand1 normie = mathz.random_int()
assert_true(rand1 > 0)

sus rand2 meal = mathz.random_meal()
assert_true(rand2 >= 0.0 && rand2 <= 1.0)

sus rand3 normie = mathz.random_range(10, 20)
assert_true(rand3 >= 10 && rand3 < 20)

test_start("Fibonacci Sequence")

fr fr Test Fibonacci function
sus fib1 normie = mathz.fibonacci(0)
assert_eq_int(fib1, 0)

sus fib2 normie = mathz.fibonacci(1)
assert_eq_int(fib2, 1)

sus fib3 normie = mathz.fibonacci(5)
assert_eq_int(fib3, 5)

sus fib4 normie = mathz.fibonacci(10)
assert_eq_int(fib4, 55)

print_test_summary()
