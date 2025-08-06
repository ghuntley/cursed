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

test_start("Logarithmic Functions")

fr fr Test log10 function
sus log10_1 meal = mathz.log10_meal(100.0)
assert_true(log10_1 >= 1.9 && log10_1 <= 2.1)

sus log10_2 meal = mathz.log10_meal(1.0)
assert_true(log10_2 >= -0.1 && log10_2 <= 0.1)

fr fr Test log2 function
sus log2_1 meal = mathz.log2_meal(8.0)
assert_true(log2_1 >= 2.9 && log2_1 <= 3.1)

test_start("Inverse Trigonometric Functions")

fr fr Test asin function
sus asin_0 meal = mathz.asin_meal(0.0)
assert_true(asin_0 >= -0.1 && asin_0 <= 0.1)

sus asin_1 meal = mathz.asin_meal(1.0)
assert_true(asin_1 >= 1.5 && asin_1 <= 1.6)

fr fr Test acos function
sus acos_0 meal = mathz.acos_meal(1.0)
assert_true(acos_0 >= -0.1 && acos_0 <= 0.1)

fr fr Test atan function
sus atan_0 meal = mathz.atan_meal(0.0)
assert_true(atan_0 >= -0.1 && atan_0 <= 0.1)

sus atan_1 meal = mathz.atan_meal(1.0)
assert_true(atan_1 >= 0.7 && atan_1 <= 0.8)

test_start("Hyperbolic Functions")

fr fr Test sinh function
sus sinh_0 meal = mathz.sinh_meal(0.0)
assert_true(sinh_0 >= -0.1 && sinh_0 <= 0.1)

fr fr Test cosh function
sus cosh_0 meal = mathz.cosh_meal(0.0)
assert_true(cosh_0 >= 0.9 && cosh_0 <= 1.1)

fr fr Test tanh function
sus tanh_0 meal = mathz.tanh_meal(0.0)
assert_true(tanh_0 >= -0.1 && tanh_0 <= 0.1)

test_start("Clamp Functions")

fr fr Test clamp for floats
sus clamp1 meal = mathz.clamp_meal(5.0, 1.0, 10.0)
assert_true(clamp1 == 5.0)

sus clamp2 meal = mathz.clamp_meal(-5.0, 1.0, 10.0)
assert_true(clamp2 == 1.0)

sus clamp3 meal = mathz.clamp_meal(15.0, 1.0, 10.0)
assert_true(clamp3 == 10.0)

fr fr Test clamp for integers
sus clamp4 normie = mathz.clamp_normie(5, 1, 10)
assert_eq_int(clamp4, 5)

sus clamp5 normie = mathz.clamp_normie(-5, 1, 10)
assert_eq_int(clamp5, 1)

test_start("Additional Utility Functions")

fr fr Test linear interpolation
sus lerp1 meal = mathz.lerp_meal(0.0, 10.0, 0.5)
assert_true(lerp1 == 5.0)

sus lerp2 meal = mathz.lerp_meal(2.0, 8.0, 0.25)
assert_true(lerp2 == 3.5)

fr fr Test sign functions
sus sign1 meal = mathz.sign_meal(5.5)
assert_true(sign1 == 1.0)

sus sign2 meal = mathz.sign_meal(-3.2)
assert_true(sign2 == -1.0)

sus sign3 meal = mathz.sign_meal(0.0)
assert_true(sign3 == 0.0)

sus sign4 normie = mathz.sign_normie(42)
assert_eq_int(sign4, 1)

sus sign5 normie = mathz.sign_normie(-7)
assert_eq_int(sign5, -1)

test_start("Special Value Tests")

fr fr Test NaN and infinity detection
sus finite_val meal = 42.0
assert_true(mathz.is_finite(finite_val))

fr fr Test truncation and fractional parts
sus trunc1 normie = mathz.trunc_meal(3.7)
assert_eq_int(trunc1, 3)

sus trunc2 normie = mathz.trunc_meal(-2.9)
assert_eq_int(trunc2, -2)

sus frac1 meal = mathz.frac_meal(3.7)
assert_true(frac1 >= 0.6 && frac1 <= 0.8)

test_start("Advanced Random Functions")

fr fr Test random range for floats
mathz.set_random_seed(54321)
sus rand_range1 meal = mathz.random_meal_range(1.0, 5.0)
assert_true(rand_range1 >= 1.0 && rand_range1 <= 5.0)

fr fr Test Gaussian random (just ensure it generates values)
sus gauss1 meal = mathz.random_gaussian()
assert_true(gauss1 >= -10.0 && gauss1 <= 10.0) fr fr Reasonable range check

test_start("Prime Number Testing")

fr fr Test prime checking
assert_true(mathz.is_prime(2))
assert_true(mathz.is_prime(3))
assert_true(mathz.is_prime(5))
assert_true(mathz.is_prime(7))
assert_true(mathz.is_prime(11))
assert_true(mathz.is_prime(13))
assert_true(mathz.is_prime(17))

assert_false(mathz.is_prime(1))
assert_false(mathz.is_prime(4))
assert_false(mathz.is_prime(6))
assert_false(mathz.is_prime(8))
assert_false(mathz.is_prime(9))
assert_false(mathz.is_prime(10))

test_start("Mathematical Series")

fr fr Test arithmetic series
sus arith1 normie = mathz.arithmetic_sum(1, 10, 10)
assert_eq_int(arith1, 55)

sus arith2 normie = mathz.arithmetic_sum(2, 8, 4)
assert_eq_int(arith2, 20)

fr fr Test geometric series
sus geom1 meal = mathz.geometric_sum(1.0, 2.0, 3)
assert_true(geom1 >= 6.9 && geom1 <= 7.1) fr fr Should be 7.0

test_start("Distance Functions")

fr fr Test 2D distance
sus dist2d1 meal = mathz.distance_2d(0.0, 0.0, 3.0, 4.0)
assert_true(dist2d1 >= 4.9 && dist2d1 <= 5.1) fr fr Should be 5.0

sus dist2d2 meal = mathz.distance_2d(1.0, 1.0, 1.0, 1.0)
assert_true(dist2d2 >= -0.1 && dist2d2 <= 0.1) fr fr Should be 0.0

fr fr Test 3D distance
sus dist3d1 meal = mathz.distance_3d(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)
assert_true(dist3d1 >= 1.7 && dist3d1 <= 1.8) fr fr Should be sqrt(3)

test_start("Float Modulo")

fr fr Test floating point modulo
sus fmod1 meal = mathz.fmod_meal(7.5, 2.5)
assert_true(fmod1 >= 2.4 && fmod1 <= 2.6) fr fr Should be 2.5

sus fmod2 meal = mathz.fmod_meal(10.0, 3.0)
assert_true(fmod2 >= 0.9 && fmod2 <= 1.1) fr fr Should be 1.0

print_test_summary()
