yeet "testz"
yeet "arrayz"

test_start("arrayz Array Operations Comprehensive Tests")

fr fr ===== ARRAY ARITHMETIC TESTS =====

test_group("Array Arithmetic Operations")

fr fr Test array sum
sus test_array []drip = [1, 2, 3, 4, 5]
sus sum_result drip = sum_array(test_array)
assert_eq_int(sum_result, 15, "Array sum calculation")

sus empty_array []drip = []
sum_result = sum_array(empty_array)
assert_eq_int(sum_result, 0, "Empty array sum is zero")

sus single_element []drip = [42]
sum_result = sum_array(single_element)
assert_eq_int(sum_result, 42, "Single element sum")

sus negative_array []drip = [-1, -2, -3]
sum_result = sum_array(negative_array)
assert_eq_int(sum_result, -6, "Negative numbers sum")

fr fr Test array average
sus average_result drip = average_array(test_array)
assert_eq_int(average_result, 3, "Array average calculation")

average_result = average_array(empty_array)
assert_eq_int(average_result, 0, "Empty array average is zero")

average_result = average_array(single_element)
assert_eq_int(average_result, 42, "Single element average")

fr fr Test array product
sus product_result drip = product_array([2, 3, 4])
assert_eq_int(product_result, 24, "Array product calculation")

product_result = product_array(empty_array)
assert_eq_int(product_result, 0, "Empty array product is zero")

product_result = product_array(single_element)
assert_eq_int(product_result, 42, "Single element product")

sus zero_array []drip = [1, 0, 5]
product_result = product_array(zero_array)
assert_eq_int(product_result, 0, "Product with zero element")

fr fr ===== ARRAY SEARCH TESTS =====

test_group("Array Search Operations")

fr fr Test find maximum
sus max_result drip = find_max([5, 3, 9, 1, 7])
assert_eq_int(max_result, 9, "Find maximum value")

max_result = find_max(single_element)
assert_eq_int(max_result, 42, "Single element maximum")

max_result = find_max(empty_array)
assert_eq_int(max_result, 0, "Empty array maximum default")

max_result = find_max([-5, -3, -9, -1, -7])
assert_eq_int(max_result, -1, "Maximum of negative numbers")

fr fr Test find minimum
sus min_result drip = find_min([5, 3, 9, 1, 7])
assert_eq_int(min_result, 1, "Find minimum value")

min_result = find_min(single_element)
assert_eq_int(min_result, 42, "Single element minimum")

min_result = find_min(empty_array)
assert_eq_int(min_result, 0, "Empty array minimum default")

min_result = find_min([-5, -3, -9, -1, -7])
assert_eq_int(min_result, -9, "Minimum of negative numbers")

fr fr Test linear search
sus search_result drip = linear_search([10, 20, 30, 40, 50], 30)
assert_eq_int(search_result, 2, "Linear search found element")

search_result = linear_search([10, 20, 30, 40, 50], 99)
assert_eq_int(search_result, -1, "Linear search element not found")

search_result = linear_search(empty_array, 5)
assert_eq_int(search_result, -1, "Linear search in empty array")

search_result = linear_search([7], 7)
assert_eq_int(search_result, 0, "Linear search single element found")

fr fr ===== ARRAY MANIPULATION TESTS =====

test_group("Array Manipulation")

fr fr Test array reversal
sus original_array []drip = [1, 2, 3, 4, 5]
sus reversed_array []drip = reverse_array(original_array)
assert_eq_int(reversed_array[0], 5, "First element after reverse")
assert_eq_int(reversed_array[1], 4, "Second element after reverse")
assert_eq_int(reversed_array[4], 1, "Last element after reverse")

sus empty_reversed []drip = reverse_array(empty_array)
assert_eq_int(len(empty_reversed), 0, "Empty array reverse")

sus single_reversed []drip = reverse_array(single_element)
assert_eq_int(single_reversed[0], 42, "Single element reverse")

fr fr Test array copy
sus copied_array []drip = copy_array(original_array)
assert_eq_int(len(copied_array), len(original_array), "Copy array length")
assert_eq_int(copied_array[0], original_array[0], "Copy first element")
assert_eq_int(copied_array[4], original_array[4], "Copy last element")

sus empty_copy []drip = copy_array(empty_array)
assert_eq_int(len(empty_copy), 0, "Empty array copy")

fr fr ===== ARRAY FILTERING TESTS =====

test_group("Array Filtering")

fr fr Test filter even numbers
sus mixed_numbers []drip = [1, 2, 3, 4, 5, 6, 7, 8]
sus even_numbers []drip = filter_even(mixed_numbers)
assert_eq_int(len(even_numbers), 4, "Filter even numbers count")
assert_eq_int(even_numbers[0], 2, "First even number")
assert_eq_int(even_numbers[1], 4, "Second even number")
assert_eq_int(even_numbers[2], 6, "Third even number")
assert_eq_int(even_numbers[3], 8, "Fourth even number")

sus odd_only []drip = [1, 3, 5, 7]
sus no_evens []drip = filter_even(odd_only)
assert_eq_int(len(no_evens), 0, "No even numbers to filter")

sus empty_filtered []drip = filter_even(empty_array)
assert_eq_int(len(empty_filtered), 0, "Filter even from empty array")

fr fr Test filter odd numbers
sus odd_numbers []drip = filter_odd(mixed_numbers)
assert_eq_int(len(odd_numbers), 4, "Filter odd numbers count")
assert_eq_int(odd_numbers[0], 1, "First odd number")
assert_eq_int(odd_numbers[1], 3, "Second odd number")
assert_eq_int(odd_numbers[2], 5, "Third odd number")
assert_eq_int(odd_numbers[3], 7, "Fourth odd number")

fr fr ===== ARRAY SORTING TESTS =====

test_group("Array Sorting")

fr fr Test bubble sort
sus unsorted_array []drip = [5, 2, 8, 1, 9, 3]
sus sorted_array []drip = bubble_sort(unsorted_array)
assert_eq_int(sorted_array[0], 1, "Sorted first element")
assert_eq_int(sorted_array[1], 2, "Sorted second element")
assert_eq_int(sorted_array[2], 3, "Sorted third element")
assert_eq_int(sorted_array[3], 5, "Sorted fourth element")
assert_eq_int(sorted_array[4], 8, "Sorted fifth element")
assert_eq_int(sorted_array[5], 9, "Sorted sixth element")

sus already_sorted []drip = [1, 2, 3, 4, 5]
sus still_sorted []drip = bubble_sort(already_sorted)
assert_eq_int(still_sorted[0], 1, "Already sorted remains sorted")
assert_eq_int(still_sorted[4], 5, "Already sorted last element")

sus reverse_sorted []drip = [5, 4, 3, 2, 1]
sus now_sorted []drip = bubble_sort(reverse_sorted)
assert_eq_int(now_sorted[0], 1, "Reverse sorted now first")
assert_eq_int(now_sorted[4], 5, "Reverse sorted now last")

sus empty_sorted []drip = bubble_sort(empty_array)
assert_eq_int(len(empty_sorted), 0, "Empty array sort")

sus single_sorted []drip = bubble_sort(single_element)
assert_eq_int(single_sorted[0], 42, "Single element sort")

fr fr ===== ARRAY TRANSFORMATION TESTS =====

test_group("Array Transformations")

fr fr Test map operations (if available)
sus base_array []drip = [1, 2, 3, 4, 5]

fr fr Test square mapping (simulated)
sus squared_array []drip = []
sus i drip = 0
bestie (i < len(base_array)) {
    sus squared_value drip = base_array[i] * base_array[i]
    squared_array = array_append(squared_array, squared_value)
    i = i + 1
}
assert_eq_int(len(squared_array), 5, "Squared array length")
assert_eq_int(squared_array[0], 1, "First squared value")
assert_eq_int(squared_array[1], 4, "Second squared value")
assert_eq_int(squared_array[4], 25, "Last squared value")

fr fr Test double mapping (simulated)
sus doubled_array []drip = []
i = 0
bestie (i < len(base_array)) {
    sus doubled_value drip = base_array[i] * 2
    doubled_array = array_append(doubled_array, doubled_value)
    i = i + 1
}
assert_eq_int(doubled_array[0], 2, "First doubled value")
assert_eq_int(doubled_array[2], 6, "Third doubled value")

fr fr ===== ARRAY UTILITY TESTS =====

test_group("Array Utility Functions")

fr fr Test array length
assert_eq_int(len([1, 2, 3, 4, 5]), 5, "Array length calculation")
assert_eq_int(len([]), 0, "Empty array length")
assert_eq_int(len([42]), 1, "Single element array length")

fr fr Test array element access
sus access_array []drip = [10, 20, 30, 40, 50]
assert_eq_int(access_array[0], 10, "First element access")
assert_eq_int(access_array[2], 30, "Middle element access")
assert_eq_int(access_array[4], 50, "Last element access")

fr fr Test array concatenation (if available)
sus first_part []drip = [1, 2, 3]
sus second_part []drip = [4, 5, 6]
sus concatenated []drip = concat_arrays(first_part, second_part)
assert_eq_int(len(concatenated), 6, "Concatenated array length")
assert_eq_int(concatenated[0], 1, "Concatenated first element")
assert_eq_int(concatenated[3], 4, "Concatenated fourth element")
assert_eq_int(concatenated[5], 6, "Concatenated last element")

fr fr ===== ARRAY BOUNDARY TESTS =====

test_group("Array Boundary and Edge Cases")

fr fr Test large array operations
sus large_array []drip = []
i = 0
bestie (i < 100) {
    large_array = array_append(large_array, i)
    i = i + 1
}
assert_eq_int(len(large_array), 100, "Large array creation")
assert_eq_int(large_array[0], 0, "Large array first element")
assert_eq_int(large_array[99], 99, "Large array last element")

sus large_sum drip = sum_array(large_array)
assert_eq_int(large_sum, 4950, "Large array sum (0+1+...+99)")

fr fr Test duplicate elements
sus duplicates_array []drip = [5, 2, 5, 1, 5, 2]
sus dup_max drip = find_max(duplicates_array)
assert_eq_int(dup_max, 5, "Max with duplicates")

sus dup_min drip = find_min(duplicates_array)
assert_eq_int(dup_min, 1, "Min with duplicates")

sus dup_search drip = linear_search(duplicates_array, 5)
assert_eq_int(dup_search, 0, "Search finds first occurrence")

fr fr Test negative numbers array
sus negative_test []drip = [-10, -5, 0, 5, 10]
sus neg_max drip = find_max(negative_test)
assert_eq_int(neg_max, 10, "Max with negative numbers")

sus neg_min drip = find_min(negative_test)
assert_eq_int(neg_min, -10, "Min with negative numbers")

sus neg_sum drip = sum_array(negative_test)
assert_eq_int(neg_sum, 0, "Sum with positive and negative")

fr fr ===== ARRAY PERFORMANCE TESTS =====

test_group("Array Performance")

fr fr Test sorting performance on larger array
sus perf_array []drip = [9, 7, 5, 3, 1, 8, 6, 4, 2, 10]
sus perf_sorted []drip = bubble_sort(perf_array)
assert_eq_int(perf_sorted[0], 1, "Performance sort first")
assert_eq_int(perf_sorted[9], 10, "Performance sort last")

fr fr Test multiple operations on same array
sus multi_ops_array []drip = [3, 1, 4, 1, 5, 9, 2, 6]
sus multi_sum drip = sum_array(multi_ops_array)
sus multi_max drip = find_max(multi_ops_array)
sus multi_min drip = find_min(multi_ops_array)
sus multi_avg drip = average_array(multi_ops_array)

assert_eq_int(multi_sum, 31, "Multiple ops sum")
assert_eq_int(multi_max, 9, "Multiple ops max")
assert_eq_int(multi_min, 1, "Multiple ops min")
assert_eq_int(multi_avg, 3, "Multiple ops average")

fr fr Test search in various positions
sus search_test_array []drip = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
assert_eq_int(linear_search(search_test_array, 10), 0, "Search at beginning")
assert_eq_int(linear_search(search_test_array, 50), 4, "Search in middle")
assert_eq_int(linear_search(search_test_array, 100), 9, "Search at end")
assert_eq_int(linear_search(search_test_array, 999), -1, "Search not found")

print_test_summary()
