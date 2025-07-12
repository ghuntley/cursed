# Test core module functionality
yeet "testz"
yeet "core"

# Test type conversion functions
test_start("Type conversion functions")

# Test lit() function
assert_true(lit(1))
assert_false(lit(0))
assert_true(lit(42))

# Test normie() function
assert_eq_int(normie(based), 1)
assert_eq_int(normie(cap), 0)

# Test tea() function
assert_eq_string(tea(0), "0")
assert_eq_string(tea(1), "1")
assert_eq_string(tea(42), "42")

print_test_summary()

# Test utility functions
test_start("Utility functions")

# Test max() function
assert_eq_int(max(5, 3), 5)
assert_eq_int(max(3, 7), 7)

# Test min() function
assert_eq_int(min(5, 3), 3)
assert_eq_int(min(3, 7), 3)

# Test abs() function
assert_eq_int(abs(5), 5)
assert_eq_int(abs(-5), 5)
assert_eq_int(abs(0), 0)

print_test_summary()

# Test boolean utilities
test_start("Boolean utilities")

# Test not() function
assert_true(not(cap))
assert_false(not(based))

# Test and() function
assert_true(and(based, based))
assert_false(and(based, cap))

# Test or() function
assert_true(or(based, based))
assert_true(or(based, cap))
assert_false(or(cap, cap))

print_test_summary()

# Test string utilities
test_start("String utilities")

# Test string_concat() function
assert_eq_string(string_concat("hello", "world"), "helloworld")
assert_eq_string(string_concat("", "test"), "test")

print_test_summary()

# Test mathematical utilities
test_start("Mathematical utilities")

# Test pow() function
assert_eq_int(pow(2, 3), 8)
assert_eq_int(pow(5, 2), 25)
assert_eq_int(pow(3, 0), 1)

print_test_summary()

# Test panic recovery
test_start("Panic and recovery")

# Test unbothered() function
assert_true(unbothered())

vibez.spill("All core module tests completed!")
print_test_summary()
