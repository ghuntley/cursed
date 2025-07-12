yeet "testz"
yeet "math"

test_start("math arithmetic tests")

# Test addition
sus result1 normie = add(5, 3)
assert_eq_int(result1, 8)

# Test subtraction
sus result2 normie = subtract(10, 4)
assert_eq_int(result2, 6)

# Test multiplication
sus result3 normie = multiply(6, 7)
assert_eq_int(result3, 42)

# Test division
sus result4 normie = divide(15, 3)
assert_eq_int(result4, 5)

# Test power
sus result5 normie = power(2, 3)
assert_eq_int(result5, 8)

# Test absolute value
sus result6 normie = abs(-5)
assert_eq_int(result6, 5)

# Test max
sus result7 normie = max(10, 20)
assert_eq_int(result7, 20)

# Test min
sus result8 normie = min(10, 20)
assert_eq_int(result8, 10)

print_test_summary()
