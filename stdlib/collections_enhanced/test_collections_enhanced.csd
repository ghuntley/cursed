yeet "testz"
yeet "collections_enhanced"

fr fr Test the enhanced collections module

test_start("Basic Array Operations")
sus test_array drip[value] = [1, 2, 3, 4, 5]
assert_eq_int(array_length(test_array), 5)
assert_false(array_is_empty(test_array))
assert_eq_int(array_get_safe(test_array, 2), 3)
assert_true(array_contains(test_array, 4))
assert_eq_int(array_find_index(test_array, 3), 2)

test_start("Array Mathematical Operations")
assert_eq_int(array_sum(test_array), 15)
assert_eq_int(array_product(test_array), 120)
assert_eq_int(array_min(test_array), 1)
assert_eq_int(array_max(test_array), 5)
assert_eq_int(array_average(test_array), 3)

test_start("Array Counting Operations")
sus mixed_array drip[value] = [1, 2, 2, 3, 2]
assert_eq_int(array_count_occurrences(mixed_array, 2), 3)
assert_eq_int(array_count_positive(mixed_array), 5)
assert_eq_int(array_count_even(mixed_array), 3)
assert_eq_int(array_count_odd(mixed_array), 2)

test_start("Sorting Algorithms")
sus sort_test drip[value] = [3, 1, 4, 1, 5]
bubble_sort_modify(sort_test)
assert_eq_int(sort_test[0], 1)
assert_eq_int(sort_test[4], 5)

sus sort_test2 drip[value] = [5, 2, 8, 1, 9]
selection_sort_modify(sort_test2)
assert_eq_int(sort_test2[0], 1)
assert_eq_int(sort_test2[4], 9)

test_start("Array Transformation")
sus transform_array drip[value] = [1, 2, 3]
array_reverse_modify(transform_array)
assert_eq_int(transform_array[0], 3)
assert_eq_int(transform_array[2], 1)

sus multiply_array drip[value] = [2, 4, 6]
array_multiply_by_two_modify(multiply_array)
assert_eq_int(multiply_array[0], 4)
assert_eq_int(multiply_array[1], 8)
assert_eq_int(multiply_array[2], 12)

test_start("Search Algorithms")
sus search_array drip[value] = [10, 20, 30, 40, 50]
assert_eq_int(linear_search(search_array, 30), 2)
assert_eq_int(linear_search(search_array, 100), -1)
assert_eq_int(binary_search(search_array, 40), 3)
assert_eq_int(binary_search(search_array, 15), -1)

test_start("Set Operations")
sus set1 drip[value] = [1, 2, 3, 2, 1]
sus new_length drip = array_remove_duplicates_modify(set1)
assert_eq_int(new_length, 3)

sus arr1 drip[value] = [1, 2, 3, 4]
sus arr2 drip[value] = [3, 4, 5, 6]
assert_eq_int(array_intersection_count(arr1, arr2), 2)

test_start("Hash Table Operations")
sus keys drip[value] = [-1, -1, -1, -1, -1]
sus values drip[value] = [0, 0, 0, 0, 0]
hash_table_init(keys, values, 5)

assert_true(hash_table_put(keys, values, 5, 10, 100))
assert_true(hash_table_put(keys, values, 5, 20, 200))
assert_eq_int(hash_table_get(keys, values, 5, 10), 100)
assert_eq_int(hash_table_get(keys, values, 5, 20), 200)
assert_true(hash_table_contains(keys, values, 5, 10))
assert_false(hash_table_contains(keys, values, 5, 99))

test_start("Statistical Operations")
sus stats_array drip[value] = [1, 2, 3, 4, 5]
assert_eq_int(array_median(stats_array), 3)
assert_eq_int(array_range(stats_array), 4)
sus variance drip = array_variance(stats_array)
assert_true(variance > 0)  fr fr Variance should be positive

print_test_summary()
