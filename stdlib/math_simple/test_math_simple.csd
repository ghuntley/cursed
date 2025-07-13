yeet "testz"
yeet "math_simple"

# Test basic arithmetic operations
test_start("Integer Addition")
assert_eq_int(add_int(5, 3), 8)
assert_eq_int(add_int(-5, 3), -2)
assert_eq_int(add_int(0, 0), 0)

test_start("Integer Subtraction")
assert_eq_int(subtract_int(10, 3), 7)
assert_eq_int(subtract_int(3, 10), -7)
assert_eq_int(subtract_int(5, 5), 0)

test_start("Integer Multiplication")
assert_eq_int(multiply_int(4, 5), 20)
assert_eq_int(multiply_int(-3, 4), -12)
assert_eq_int(multiply_int(0, 100), 0)

test_start("Integer Division")
assert_eq_int(divide_int(20, 4), 5)
assert_eq_int(divide_int(15, 3), 5)
assert_eq_int(divide_int(-10, 2), -5)

# Test absolute value functions
test_start("Integer Absolute Value")
assert_eq_int(abs_int(5), 5)
assert_eq_int(abs_int(-5), 5)
assert_eq_int(abs_int(0), 0)

test_start("Float Absolute Value")
sus abs_result meal = abs_float(-3.14)
assert_true(abs_result > 3.0)
sus abs_positive meal = abs_float(2.5)
assert_true(abs_positive > 2.0)

# Test min/max functions
test_start("Integer Min/Max")
assert_eq_int(min_int(5, 3), 3)
assert_eq_int(max_int(5, 3), 5)
assert_eq_int(min_int(-5, -3), -5)
assert_eq_int(max_int(-5, -3), -3)

test_start("Float Min/Max")
sus min_result meal = min_float(2.5, 3.7)
assert_true(min_result < 3.0)
sus max_result meal = max_float(2.5, 3.7)
assert_true(max_result > 3.0)

# Test power functions
test_start("Integer Power")
assert_eq_int(power_int(2, 3), 8)
assert_eq_int(power_int(5, 2), 25)
assert_eq_int(power_int(10, 0), 1)

test_start("Float Power")
sus power_result meal = power_float(2.0, 3)
assert_true(power_result > 7.0)

# Test validation functions
test_start("Integer Validation")
assert_true(is_positive_int(5))
assert_false(is_positive_int(-5))
assert_true(is_negative_int(-3))
assert_false(is_negative_int(3))
assert_true(is_zero_int(0))
assert_false(is_zero_int(1))

test_start("Float Validation")
assert_true(is_positive_float(3.14))
assert_false(is_positive_float(-2.5))
assert_true(is_negative_float(-1.0))
assert_false(is_negative_float(1.0))

# Test type conversion
test_start("Type Conversion")
sus converted_float meal = int_to_float(42)
assert_true(converted_float > 41.0)
sus converted_int normie = float_to_int(3.14)
assert_eq_int(converted_int, 3)

# Test mathematical constants
test_start("Mathematical Constants")
assert_true(PI > 3.0)
assert_true(PI < 4.0)
assert_true(E > 2.0)
assert_true(E < 3.0)

# Test factorial function
test_start("Factorial")
assert_eq_int(factorial(5), 120)
assert_eq_int(factorial(3), 6)
assert_eq_int(factorial(0), 1)

# Test GCD function
test_start("Greatest Common Divisor")
assert_eq_int(gcd(12, 8), 4)
assert_eq_int(gcd(15, 25), 5)
assert_eq_int(gcd(7, 13), 1)

# Test LCM function
test_start("Least Common Multiple")
assert_eq_int(lcm(4, 6), 12)
assert_eq_int(lcm(3, 5), 15)

# Test square root approximation
test_start("Square Root")
sus sqrt_result meal = sqrt_float(16.0)
assert_true(sqrt_result > 3.9)
assert_true(sqrt_result < 4.1)

# Test float arithmetic operations
test_start("Float Addition")
sus float_add_result meal = add_float(2.5, 3.7)
assert_true(float_add_result > 6.0)

test_start("Float Subtraction")
sus float_sub_result meal = subtract_float(5.5, 2.3)
assert_true(float_sub_result > 3.0)

test_start("Float Multiplication")
sus float_mul_result meal = multiply_float(2.5, 4.0)
assert_true(float_mul_result > 9.0)

test_start("Float Division")
sus float_div_result meal = divide_float(10.0, 2.0)
assert_true(float_div_result > 4.9)
assert_true(float_div_result < 5.1)

print_test_summary()
