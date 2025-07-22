yeet "testz"
yeet "simple_math"

fr fr Comprehensive test suite for simple_math module

test_start("add positive numbers")
sus result normie = add(5, 3)
assert_eq_int(result, 8)

test_start("add negative numbers")
result = add(-5, -3)
assert_eq_int(result, -8)

test_start("add mixed signs")
result = add(-5, 3)
assert_eq_int(result, -2)

test_start("add zero")
result = add(5, 0)
assert_eq_int(result, 5)

test_start("add large numbers")
result = add(1000000, 2000000)
assert_eq_int(result, 3000000)

test_start("subtract positive numbers")
result = subtract(10, 3)
assert_eq_int(result, 7)

test_start("subtract negative numbers")
result = subtract(-10, -3)
assert_eq_int(result, -7)

test_start("subtract mixed signs")
result = subtract(-10, 3)
assert_eq_int(result, -13)

test_start("subtract to negative")
result = subtract(3, 10)
assert_eq_int(result, -7)

test_start("subtract zero")
result = subtract(5, 0)
assert_eq_int(result, 5)

test_start("multiply positive numbers")
result = multiply(4, 3)
assert_eq_int(result, 12)

test_start("multiply negative numbers")
result = multiply(-4, -3)
assert_eq_int(result, 12)

test_start("multiply mixed signs")
result = multiply(-4, 3)
assert_eq_int(result, -12)

test_start("multiply by zero")
result = multiply(5, 0)
assert_eq_int(result, 0)

test_start("multiply by one")
result = multiply(5, 1)
assert_eq_int(result, 5)

test_start("multiply large numbers")
result = multiply(1000, 2000)
assert_eq_int(result, 2000000)

test_start("divide positive numbers")
result = divide(12, 3)
assert_eq_int(result, 4)

test_start("divide negative numbers")
result = divide(-12, -3)
assert_eq_int(result, 4)

test_start("divide mixed signs")
result = divide(-12, 3)
assert_eq_int(result, -4)

test_start("divide by one")
result = divide(5, 1)
assert_eq_int(result, 5)

test_start("divide by zero")
result = divide(5, 0)
assert_eq_int(result, 0) fr fr Should return 0 as per implementation

test_start("divide with remainder")
result = divide(10, 3)
assert_eq_int(result, 3) fr fr Integer division

test_start("complex operations")
fr fr Test combinations
sus a normie = add(5, 3) fr fr 8
sus b normie = multiply(2, 4) fr fr 8
sus c normie = subtract(a, b) fr fr 0
sus d normie = divide(10, 2) fr fr 5
sus final_result normie = add(c, d) fr fr 5
assert_eq_int(final_result, 5)

test_start("edge case: large number operations")
sus large1 normie = 999999
sus large2 normie = 1
sus large_add normie = add(large1, large2)
assert_eq_int(large_add, 1000000)

test_start("edge case: zero operations")
assert_eq_int(add(0, 0), 0)
assert_eq_int(subtract(0, 0), 0)
assert_eq_int(multiply(0, 0), 0)
assert_eq_int(divide(0, 1), 0)

print_test_summary()
