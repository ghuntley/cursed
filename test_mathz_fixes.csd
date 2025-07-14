yeet "testz"
yeet "mathz"

test_start("mathz type consistency and function calls")

// Test constants with correct types
sus pi_test meal = mathz.Pi
sus epsilon_test meal = mathz.Epsilon
sus e_test meal = mathz.E

// Test arithmetic operations with consistent types
sus abs_result meal = mathz.Abs(0.0 - 5.5)
assert_eq_float(abs_result, 5.5)

sus max_result meal = mathz.Max(3.14, 2.71)
assert_eq_float(max_result, 3.14)

// Test sqrt function (the problematic line 81)
sus sqrt_result meal = mathz.Sqrt(9.0)
assert_eq_float(sqrt_result, 3.0)

// Test more complex sqrt
sus sqrt_complex meal = mathz.Sqrt(2.0)
assert_true(sqrt_complex > 1.4)
assert_true(sqrt_complex < 1.5)

// Test power function
sus pow_result meal = mathz.Pow(2.0, 3)
assert_eq_float(pow_result, 8.0)

// Test trigonometric conversions
sus deg_result meal = mathz.RadToDeg(mathz.Pi / 2.0)
assert_true(deg_result > 89.0)
assert_true(deg_result < 91.0)

print_test_summary()
