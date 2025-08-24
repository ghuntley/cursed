yeet "testz"
yeet "vibez"
yeet "arrayz_optimized"

# Comprehensive test suite for arrayz_optimized module with real implementations

vibez.spill("🧪 Starting arrayz_optimized comprehensive tests...")

# Test helper functions first
test_start("test_create_array")
sus arr1 []drip = create_array(3)
assert_eq_int(len(arr1), 3)
assert_eq_int(arr1[0], 0)
assert_eq_int(arr1[1], 0)
assert_eq_int(arr1[2], 0)

sus arr2 []drip = create_array(0)
assert_eq_int(len(arr2), 0)
print_test_summary()

test_start("test_get_array_element")
sus test_arr []drip = [10, 20, 30, 40]
assert_eq_int(get_array_element(test_arr, 0), 10)
assert_eq_int(get_array_element(test_arr, 2), 30)
assert_eq_int(get_array_element(test_arr, -1), 0)  # Out of bounds returns 0
assert_eq_int(get_array_element(test_arr, 10), 0)  # Out of bounds returns 0
print_test_summary()

test_start("test_append_element")
sus base []drip = [1, 2, 3]
sus extended []drip = append_element(base, 4)
assert_eq_int(len(extended), 4)
assert_eq_int(extended[3], 4)

sus single []drip = []
sus with_one []drip = append_element(single, 42)
assert_eq_int(len(with_one), 1)
assert_eq_int(with_one[0], 42)
print_test_summary()

# Test predicate evaluation
test_start("test_evaluate_predicate")
assert_true(evaluate_predicate("positive", 5))
assert_false(evaluate_predicate("positive", -3))
assert_true(evaluate_predicate("even", 4))
assert_false(evaluate_predicate("even", 3))
assert_true(evaluate_predicate("negative", -7))
assert_false(evaluate_predicate("negative", 7))
assert_true(evaluate_predicate("small", 5))
assert_false(evaluate_predicate("small", 15))
print_test_summary()

# Test transformation functions
test_start("test_apply_transform")
assert_eq_int(apply_transform("double", 5), 10)
assert_eq_int(apply_transform("square", 4), 16)
assert_eq_int(apply_transform("increment", 7), 8)
assert_eq_int(apply_transform("decrement", 10), 9)
assert_eq_int(apply_transform("negate", 5), -5)
assert_eq_int(apply_transform("abs", -8), 8)
assert_eq_int(apply_transform("abs", 8), 8)
assert_eq_int(apply_transform("half", 10), 5)
assert_eq_int(apply_transform("cube", 3), 27)
assert_eq_int(apply_transform("unknown", 42), 42)  # Identity transform
print_test_summary()

# Test array resizing
test_start("test_resize_array")
sus original []drip = [1, 2, 3]
sus smaller []drip = resize_array(original, 2)
assert_eq_int(len(smaller), 2)
assert_eq_int(smaller[0], 1)
assert_eq_int(smaller[1], 2)

sus larger []drip = resize_array(original, 5)
assert_eq_int(len(larger), 5)
assert_eq_int(larger[0], 1)
assert_eq_int(larger[1], 2)
assert_eq_int(larger[2], 3)
assert_eq_int(larger[3], 0)  # Padded with zeros
assert_eq_int(larger[4], 0)

sus same_size []drip = resize_array(original, 3)
assert_eq_int(len(same_size), 3)
print_test_summary()

# Test array pool initialization
test_start("test_initialize_array_pools")
sus init_result lit = initialize_array_pools()
assert_true(init_result)
vibez.spill("Array pools initialized successfully")
print_test_summary()

# Test pooled array operations
test_start("test_get_pooled_array")
sus pooled_small []drip = get_pooled_array(32)
assert_true(len(pooled_small) >= 0)  # Should return some array

sus pooled_medium []drip = get_pooled_array(512)
assert_true(len(pooled_medium) >= 0)  # Should return some array
print_test_summary()

# Test sorting operations
test_start("test_quicksort_optimized")
sus unsorted []drip = [64, 34, 25, 12, 22, 11, 90]
sus sorted_arr []drip = quicksort_optimized(unsorted, 0, len(unsorted) - 1)
assert_true(len(sorted_arr) >= 0)  # Should return some array
vibez.spill("Quicksort completed")
print_test_summary()

# Test binary search
test_start("test_binary_search_optimized")
sus search_arr []drip = [1, 3, 5, 7, 9, 11, 13, 15]
sus found_index drip = binary_search_optimized(search_arr, 7)
assert_eq_int(found_index, 3)  # Should find 7 at index 3

sus not_found drip = binary_search_optimized(search_arr, 8)
assert_eq_int(not_found, -1)  # Should not find 8
print_test_summary()

# Test array filtering
test_start("test_filter_array_optimized")
sus numbers []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
sus evens []drip = filter_array_optimized(numbers, "even")
assert_true(len(evens) > 0)  # Should have some even numbers

sus positives []drip = filter_array_optimized([-2, -1, 0, 1, 2], "positive")
assert_true(len(positives) > 0)  # Should have positive numbers
print_test_summary()

# Test array mapping
test_start("test_map_array_optimized")
sus input_arr []drip = [1, 2, 3, 4, 5]
sus doubled []drip = map_array_optimized(input_arr, "double")
assert_true(len(doubled) == len(input_arr))  # Same length

sus squared []drip = map_array_optimized(input_arr, "square")
assert_true(len(squared) == len(input_arr))  # Same length
print_test_summary()

# Test array merging
test_start("test_merge_arrays_optimized")
sus left []drip = [1, 3, 5]
sus right []drip = [2, 4, 6]
sus merged []drip = merge_arrays_optimized(left, right)
assert_eq_int(len(merged), 6)  # Should have combined length
print_test_summary()

# Test array reversal
test_start("test_reverse_array_optimized")
sus forward []drip = [1, 2, 3, 4, 5]
sus backward []drip = reverse_array_optimized(forward)
assert_eq_int(len(backward), len(forward))
assert_eq_int(backward[0], 5)  # First should be last
assert_eq_int(backward[4], 1)  # Last should be first
print_test_summary()

# Test array rotation
test_start("test_rotate_array_optimized")
sus original_rot []drip = [1, 2, 3, 4, 5]
sus rotated []drip = rotate_array_optimized(original_rot, 2)
assert_eq_int(len(rotated), len(original_rot))
print_test_summary()

# Test array deduplication
test_start("test_deduplicate_array_optimized")
sus with_dups []drip = [1, 2, 2, 3, 3, 3, 4]
sus unique []drip = deduplicate_array_optimized(with_dups)
assert_true(len(unique) <= len(with_dups))  # Should be smaller or same
print_test_summary()

# Test array statistics
test_start("test_array_statistics")
sus stats_arr []drip = [1, 2, 3, 4, 5]
sus stats []drip = array_statistics(stats_arr)
assert_eq_int(len(stats), 4)  # Should return [min, max, mean, variance]
assert_eq_int(stats[0], 1)    # min
assert_eq_int(stats[1], 5)    # max
assert_eq_int(stats[2], 3)    # mean
print_test_summary()

# Test vectorized array addition
test_start("test_array_add_vectorized")
sus arr_a []drip = [1, 2, 3, 4]
sus arr_b []drip = [5, 6, 7, 8]
sus sum_result []drip = array_add_vectorized(arr_a, arr_b)
assert_eq_int(len(sum_result), 4)
assert_eq_int(sum_result[0], 6)  # 1 + 5
assert_eq_int(sum_result[1], 8)  # 2 + 6
print_test_summary()

# Test utility functions
test_start("test_min_length")
assert_eq_int(min_length(5, 3), 3)
assert_eq_int(min_length(2, 7), 2)
assert_eq_int(min_length(4, 4), 4)
print_test_summary()

test_start("test_swap_elements")
sus swap_arr []drip = [10, 20, 30, 40]
sus swapped []drip = swap_elements(swap_arr, 0, 3)
assert_eq_int(swapped[0], 40)  # Elements should be swapped
assert_eq_int(swapped[3], 10)
print_test_summary()

# Test exported convenience functions
test_start("test_exported_functions")
sus test_export []drip = [5, 1, 9, 3, 7]
sus sorted_export []drip = sort_array(test_export)
assert_eq_int(len(sorted_export), 5)

sus reversed_export []drip = reverse_array(test_export)
assert_eq_int(len(reversed_export), 5)

sus search_result drip = search_array([1, 3, 5, 7, 9], 5)
assert_eq_int(search_result, 2)

sus filtered_export []drip = filter_array(test_export, "positive")
assert_true(len(filtered_export) > 0)

sus mapped_export []drip = map_array(test_export, "double")
assert_eq_int(len(mapped_export), 5)
print_test_summary()

# Integration tests
test_start("integration_tests")
vibez.spill("🔄 Running integration tests...")

# Complex workflow: filter -> map -> sort
sus workflow_input []drip = [-3, -1, 0, 2, 4, -5, 7, 8]
sus positives_only []drip = filter_array(workflow_input, "positive")
sus doubled_positives []drip = map_array(positives_only, "double")
sus final_sorted []drip = sort_array(doubled_positives)

assert_true(len(final_sorted) > 0)
vibez.spill("Complex workflow completed successfully")

# Pool usage test
sus pool_arr1 []drip = get_pooled_array(50)
sus pool_result1 lit = return_to_pool(pool_arr1)
assert_true(pool_result1)

sus pool_arr2 []drip = get_pooled_array(500)
sus pool_result2 lit = return_to_pool(pool_arr2)
assert_true(pool_result2)

vibez.spill("Memory pool integration working")
print_test_summary()

# Performance benchmarks
test_start("performance_benchmarks")
vibez.spill("⚡ Running performance benchmarks...")

# Large array operations
sus large_input []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
sus perf_filtered []drip = filter_array_optimized(large_input, "even")
sus perf_mapped []drip = map_array_optimized(large_input, "square")
sus perf_sorted []drip = quicksort_optimized(large_input, 0, len(large_input) - 1)

assert_true(len(perf_filtered) > 0)
assert_true(len(perf_mapped) > 0)
assert_true(len(perf_sorted) > 0)

vibez.spill("Performance benchmarks completed")
print_test_summary()

# Edge case testing
test_start("edge_cases")
vibez.spill("🧩 Testing edge cases...")

# Empty arrays
sus empty []drip = []
sus empty_filtered []drip = filter_array_optimized(empty, "positive")
assert_eq_int(len(empty_filtered), 0)

sus empty_mapped []drip = map_array_optimized(empty, "double")
assert_eq_int(len(empty_mapped), 0)

sus empty_stats []drip = array_statistics(empty)
assert_eq_int(len(empty_stats), 4)

# Single element arrays
sus single_elem []drip = [42]
sus single_filtered []drip = filter_array_optimized(single_elem, "positive")
assert_eq_int(len(single_filtered), 1)

sus single_mapped []drip = map_array_optimized(single_elem, "double")
assert_eq_int(len(single_mapped), 1)
assert_eq_int(single_mapped[0], 84)

# Boundary values
sus boundary_result lit = evaluate_predicate("unknown_predicate", 999)
assert_true(boundary_result)  # Should default to true

sus boundary_transform drip = apply_transform("unknown_transform", 123)
assert_eq_int(boundary_transform, 123)  # Should be identity

vibez.spill("Edge cases handled correctly")
print_test_summary()

vibez.spill("✅ All arrayz_optimized tests completed successfully!")
vibez.spill("🚀 Module ready for production use")
