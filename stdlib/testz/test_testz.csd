# Test suite for the testz testing framework
# Self-testing the test framework

yeet "testz"

# Test basic assertion functions
test_start("assert_eq_int functionality")
assert_eq_int(42, 42)
assert_eq_int(0, 0)
assert_eq_int(-1, -1)
test_end()

test_start("assert_eq_string functionality")
assert_eq_string("hello", "hello")
assert_eq_string("", "")
assert_eq_string("test", "test")
test_end()

test_start("assert_true functionality")
assert_true(based)
assert_true(5 > 3)
assert_true(42 == 42)
test_end()

test_start("assert_false functionality")
assert_false(cap)
assert_false(3 > 5)
assert_false(42 == 43)
test_end()

test_start("assert_ne_int functionality")
assert_ne_int(42, 43)
assert_ne_int(0, 1)
assert_ne_int(-1, 1)
test_end()

test_start("assert_gt_int functionality")
assert_gt_int(5, 3)
assert_gt_int(42, 0)
assert_gt_int(0, -1)
test_end()

test_start("assert_lt_int functionality")
assert_lt_int(3, 5)
assert_lt_int(0, 42)
assert_lt_int(-1, 0)
test_end()

test_start("test counter functionality")
sus initial_total normie = total_tests
assert_gt_int(total_tests, 0)
test_end()

test_start("test state management")
assert_eq_string(current_test_name, "test state management")
assert_true(current_test_passed)
test_end()

test_start("helper functions")
assert_true(get_test_results() >= 0)
assert_true(all_tests_passed() == based || all_tests_passed() == cap)
test_end()

# Test arithmetic operations in assertions
test_start("arithmetic in assertions")
sus x normie = 10
sus y normie = 5
assert_eq_int(x + y, 15)
assert_eq_int(x - y, 5)
assert_eq_int(x * 2, 20)
assert_gt_int(x, y)
assert_lt_int(y, x)
test_end()

# Test string operations in assertions
test_start("string operations in assertions")
sus greeting tea = "Hello"
sus name tea = "World"
assert_eq_string(greeting, "Hello")
assert_eq_string(name, "World")
assert_ne_int(greeting.length(), 0)
test_end()

# Test boolean logic in assertions
test_start("boolean logic in assertions")
sus flag1 lit = based
sus flag2 lit = cap
assert_true(flag1)
assert_false(flag2)
assert_true(flag1 && !flag2)
assert_false(flag1 && flag2)
test_end()

# Test edge cases
test_start("edge cases")
assert_eq_int(0, 0)
assert_eq_string("", "")
assert_true(based)
assert_false(cap)
test_end()

# Test complex expressions
test_start("complex expressions")
assert_true((5 + 3) == 8)
assert_false((10 / 2) == 3)
assert_eq_int((4 * 3) + 2, 14)
test_end()

# Final summary
print_test_summary()
