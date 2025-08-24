yeet "testz"
yeet "vibez"
yeet "arrayz_optimized"

vibez.spill("🧪 Testing arrayz_optimized basic functionality...")

# Test basic array creation
test_start("basic_array_creation")
sus test_arr []drip = create_array(5)
assert_eq_int(len(test_arr), 5)
vibez.spill("✅ Array creation works")
print_test_summary()

# Test predicate evaluation
test_start("predicate_evaluation")
assert_true(evaluate_predicate("positive", 10))
assert_false(evaluate_predicate("positive", -5))
assert_true(evaluate_predicate("even", 4))
assert_false(evaluate_predicate("even", 3))
vibez.spill("✅ Predicate evaluation works")
print_test_summary()

# Test transformations
test_start("transformations")
assert_eq_int(apply_transform("double", 5), 10)
assert_eq_int(apply_transform("square", 3), 9)
assert_eq_int(apply_transform("increment", 7), 8)
vibez.spill("✅ Transformations work")
print_test_summary()

# Test array resizing
test_start("array_resizing")
sus original []drip = [1, 2, 3]
sus resized []drip = resize_array(original, 5)
assert_eq_int(len(resized), 5)
assert_eq_int(resized[0], 1)
assert_eq_int(resized[3], 0)  # Padded with zeros
vibez.spill("✅ Array resizing works")
print_test_summary()

# Test element access
test_start("element_access")
sus access_arr []drip = [10, 20, 30]
assert_eq_int(get_array_element(access_arr, 1), 20)
assert_eq_int(get_array_element(access_arr, -1), 0)  # Out of bounds
vibez.spill("✅ Element access works")
print_test_summary()

# Test array statistics
test_start("array_statistics")
sus stats_input []drip = [1, 2, 3, 4, 5]
sus stats_result []drip = array_statistics(stats_input)
assert_eq_int(len(stats_result), 4)
assert_eq_int(stats_result[0], 1)  # min
assert_eq_int(stats_result[1], 5)  # max
assert_eq_int(stats_result[2], 3)  # mean
vibez.spill("✅ Array statistics work")
print_test_summary()

# Test min_length utility
test_start("utilities")
assert_eq_int(min_length(10, 5), 5)
assert_eq_int(min_length(3, 8), 3)
vibez.spill("✅ Utilities work")
print_test_summary()

vibez.spill("🚀 Basic arrayz_optimized functionality verified!")
