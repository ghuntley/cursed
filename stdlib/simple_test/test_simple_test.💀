yeet "testz"
yeet "simple_test"

# Real functional tests for simple_test module

test_start("test_simple_func")
# Test basic function
sus result lit = simple_func()
assert_true(result)

# Test function with different conditions
sus result2 lit = simple_func()
assert_true(result2)
print_test_summary()

test_start("test_add_two")
# Test addition function
sus result drip = add_two(5, 3)
assert_eq_int(result, 8)

# Test with zeros
sus zero_result drip = add_two(0, 0)
assert_eq_int(zero_result, 0)

# Test with negative numbers
sus negative_result drip = add_two(-5, 3)
assert_eq_int(negative_result, -2)

# Test with large numbers
sus large_result drip = add_two(1000, 2000)
assert_eq_int(large_result, 3000)
print_test_summary()

# Integration test
test_start("integration_simple_workflow")
simple_func()
sus sum drip = add_two(10, 20)
assert_eq_int(sum, 30)
print_test_summary()
