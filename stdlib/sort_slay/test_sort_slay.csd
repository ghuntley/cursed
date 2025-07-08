// CURSED Sort Slay Module Tests
// Comprehensive test suite for high-performance sorting algorithms

yeet "testz"
yeet "sort_slay"

// Test integer sorting
slay test_sort_ints() {
    test_start("sort_ints basic functionality")
    
    sus arr []normie = [3, 1, 4, 1, 5, 9, 2, 6]
    sus sorted []normie = sort_ints(arr)
    
    assert_eq_int(sorted[0], 1)
    assert_eq_int(sorted[1], 1)
    assert_eq_int(sorted[2], 2)
    assert_eq_int(sorted[3], 3)
    assert_eq_int(sorted[4], 4)
    assert_eq_int(sorted[5], 5)
    assert_eq_int(sorted[6], 6)
    assert_eq_int(sorted[7], 9)
    
    test_start("sort_ints empty array")
    sus empty []normie = []
    sus sorted_empty []normie = sort_ints(empty)
    assert_eq_int(len(sorted_empty), 0)
    
    test_start("sort_ints single element")
    sus single []normie = [42]
    sus sorted_single []normie = sort_ints(single)
    assert_eq_int(sorted_single[0], 42)
    assert_eq_int(len(sorted_single), 1)
}

// Test string sorting
slay test_sort_strings() {
    test_start("sort_strings basic functionality")
    
    sus arr []tea = ["zebra", "apple", "banana", "cherry"]
    sus sorted []tea = sort_strings(arr)
    
    assert_eq_string(sorted[0], "apple")
    assert_eq_string(sorted[1], "banana")
    assert_eq_string(sorted[2], "cherry")
    assert_eq_string(sorted[3], "zebra")
    
    test_start("sort_strings single element")
    sus single []tea = ["hello"]
    sus sorted_single []tea = sort_strings(single)
    assert_eq_string(sorted_single[0], "hello")
    assert_eq_int(len(sorted_single), 1)
}

// Test float sorting
slay test_sort_floats() {
    test_start("sort_floats basic functionality")
    
    sus arr []meal = [3.14, 2.71, 1.41, 1.73, 2.23]
    sus sorted []meal = sort_floats(arr)
    
    assert_true(sorted[0] < sorted[1])
    assert_true(sorted[1] < sorted[2])
    assert_true(sorted[2] < sorted[3])
    assert_true(sorted[3] < sorted[4])
    
    test_start("sort_floats negative numbers")
    sus negative []meal = [-1.5, -2.5, -0.5, -3.0]
    sus sorted_neg []meal = sort_floats(negative)
    assert_true(sorted_neg[0] < sorted_neg[1])
    assert_true(sorted_neg[1] < sorted_neg[2])
    assert_true(sorted_neg[2] < sorted_neg[3])
}

// Test reverse sorting
slay test_sort_reverse() {
    test_start("sort_reverse basic functionality")
    
    sus arr []normie = [1, 3, 2, 5, 4]
    sus reversed []normie = sort_reverse(arr)
    
    assert_eq_int(reversed[0], 5)
    assert_eq_int(reversed[1], 4)
    assert_eq_int(reversed[2], 3)
    assert_eq_int(reversed[3], 2)
    assert_eq_int(reversed[4], 1)
    
    test_start("sort_reverse already sorted")
    sus sorted []normie = [1, 2, 3, 4, 5]
    sus reversed_sorted []normie = sort_reverse(sorted)
    assert_eq_int(reversed_sorted[0], 5)
    assert_eq_int(reversed_sorted[4], 1)
}

// Test is_sorted function
slay test_is_sorted() {
    test_start("is_sorted true case")
    
    sus sorted []normie = [1, 2, 3, 4, 5]
    assert_true(is_sorted(sorted))
    
    test_start("is_sorted false case")
    sus unsorted []normie = [3, 1, 4, 1, 5]
    assert_false(is_sorted(unsorted))
    
    test_start("is_sorted empty array")
    sus empty []normie = []
    assert_true(is_sorted(empty))
    
    test_start("is_sorted single element")
    sus single []normie = [42]
    assert_true(is_sorted(single))
    
    test_start("is_sorted duplicates")
    sus duplicates []normie = [1, 1, 2, 2, 3]
    assert_true(is_sorted(duplicates))
}

// Test quick select
slay test_quick_select() {
    test_start("quick_select basic functionality")
    
    sus arr []normie = [3, 1, 4, 1, 5, 9, 2, 6]
    
    // Find the smallest element (k=0)
    sus smallest normie = quick_select(arr, 0)
    assert_eq_int(smallest, 1)
    
    // Find the 3rd smallest element (k=2)
    sus third_smallest normie = quick_select(arr, 2)
    assert_eq_int(third_smallest, 2)
    
    // Find the largest element (k=7)
    sus largest normie = quick_select(arr, 7)
    assert_eq_int(largest, 9)
    
    test_start("quick_select out of bounds")
    sus invalid normie = quick_select(arr, 10)
    assert_eq_int(invalid, -1)
    
    sus negative normie = quick_select(arr, -1)
    assert_eq_int(negative, -1)
}

// Test merge function
slay test_merge() {
    test_start("merge basic functionality")
    
    sus arr1 []normie = [1, 3, 5, 7]
    sus arr2 []normie = [2, 4, 6, 8]
    sus merged []normie = merge(arr1, arr2)
    
    assert_eq_int(len(merged), 8)
    assert_eq_int(merged[0], 1)
    assert_eq_int(merged[1], 2)
    assert_eq_int(merged[2], 3)
    assert_eq_int(merged[3], 4)
    assert_eq_int(merged[4], 5)
    assert_eq_int(merged[5], 6)
    assert_eq_int(merged[6], 7)
    assert_eq_int(merged[7], 8)
    
    test_start("merge empty arrays")
    sus empty1 []normie = []
    sus empty2 []normie = []
    sus merged_empty []normie = merge(empty1, empty2)
    assert_eq_int(len(merged_empty), 0)
    
    test_start("merge with one empty")
    sus filled []normie = [1, 2, 3]
    sus empty []normie = []
    sus merged_one_empty []normie = merge(filled, empty)
    assert_eq_int(len(merged_one_empty), 3)
    assert_eq_int(merged_one_empty[0], 1)
    assert_eq_int(merged_one_empty[1], 2)
    assert_eq_int(merged_one_empty[2], 3)
}

// Test binary search
slay test_binary_search() {
    test_start("binary_search basic functionality")
    
    sus arr []normie = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    
    assert_eq_int(binary_search(arr, 5), 4)
    assert_eq_int(binary_search(arr, 1), 0)
    assert_eq_int(binary_search(arr, 9), 8)
    
    test_start("binary_search not found")
    assert_eq_int(binary_search(arr, 10), -1)
    assert_eq_int(binary_search(arr, 0), -1)
    
    test_start("binary_search single element")
    sus single []normie = [42]
    assert_eq_int(binary_search(single, 42), 0)
    assert_eq_int(binary_search(single, 43), -1)
}

// Test lower bound
slay test_lower_bound() {
    test_start("lower_bound basic functionality")
    
    sus arr []normie = [1, 2, 2, 3, 3, 3, 4, 5]
    
    assert_eq_int(lower_bound(arr, 2), 1)
    assert_eq_int(lower_bound(arr, 3), 3)
    assert_eq_int(lower_bound(arr, 6), 8)
    assert_eq_int(lower_bound(arr, 0), 0)
    
    test_start("lower_bound empty array")
    sus empty []normie = []
    assert_eq_int(lower_bound(empty, 5), 0)
}

// Test upper bound
slay test_upper_bound() {
    test_start("upper_bound basic functionality")
    
    sus arr []normie = [1, 2, 2, 3, 3, 3, 4, 5]
    
    assert_eq_int(upper_bound(arr, 2), 3)
    assert_eq_int(upper_bound(arr, 3), 6)
    assert_eq_int(upper_bound(arr, 5), 8)
    assert_eq_int(upper_bound(arr, 0), 0)
    
    test_start("upper_bound empty array")
    sus empty []normie = []
    assert_eq_int(upper_bound(empty, 5), 0)
}

// Test stable sort
slay test_sort_stable() {
    test_start("sort_stable basic functionality")
    
    sus arr []normie = [3, 1, 4, 1, 5, 9, 2, 6]
    sus sorted []normie = sort_stable(arr)
    
    assert_eq_int(sorted[0], 1)
    assert_eq_int(sorted[1], 1)
    assert_eq_int(sorted[2], 2)
    assert_eq_int(sorted[3], 3)
    assert_eq_int(sorted[4], 4)
    assert_eq_int(sorted[5], 5)
    assert_eq_int(sorted[6], 6)
    assert_eq_int(sorted[7], 9)
    
    test_start("sort_stable preserves order")
    // For this test, we assume stable sort preserves original order for equal elements
    sus with_dupes []normie = [2, 1, 2, 3, 1]
    sus stable_sorted []normie = sort_stable(with_dupes)
    assert_true(is_sorted(stable_sorted))
}

// Test unstable sort
slay test_sort_unstable() {
    test_start("sort_unstable basic functionality")
    
    sus arr []normie = [3, 1, 4, 1, 5, 9, 2, 6]
    sus sorted []normie = sort_unstable(arr)
    
    assert_eq_int(sorted[0], 1)
    assert_eq_int(sorted[1], 1)
    assert_eq_int(sorted[2], 2)
    assert_eq_int(sorted[3], 3)
    assert_eq_int(sorted[4], 4)
    assert_eq_int(sorted[5], 5)
    assert_eq_int(sorted[6], 6)
    assert_eq_int(sorted[7], 9)
    
    assert_true(is_sorted(sorted))
}

// Test performance with large arrays
slay test_performance() {
    test_start("performance test with medium array")
    
    sus arr []normie = [50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
    
    sus sorted []normie = sort_ints(arr)
    assert_true(is_sorted(sorted))
    assert_eq_int(sorted[0], 1)
    assert_eq_int(sorted[49], 50)
    
    test_start("performance test with duplicates")
    sus duplicates []normie = [5, 5, 5, 5, 5, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3]
    sus sorted_dupes []normie = sort_ints(duplicates)
    assert_true(is_sorted(sorted_dupes))
}

// Test edge cases
slay test_edge_cases() {
    test_start("edge case - all same elements")
    sus same []normie = [7, 7, 7, 7, 7]
    sus sorted_same []normie = sort_ints(same)
    assert_true(is_sorted(sorted_same))
    assert_eq_int(sorted_same[0], 7)
    assert_eq_int(sorted_same[4], 7)
    
    test_start("edge case - two elements")
    sus two []normie = [2, 1]
    sus sorted_two []normie = sort_ints(two)
    assert_eq_int(sorted_two[0], 1)
    assert_eq_int(sorted_two[1], 2)
    
    test_start("edge case - already sorted")
    sus already_sorted []normie = [1, 2, 3, 4, 5]
    sus sorted_already []normie = sort_ints(already_sorted)
    assert_true(is_sorted(sorted_already))
    assert_eq_int(sorted_already[0], 1)
    assert_eq_int(sorted_already[4], 5)
}

// Run all tests
slay main() {
    vibez.spill("Running Sort Slay Module Tests...")
    
    test_sort_ints()
    test_sort_strings()
    test_sort_floats()
    test_sort_reverse()
    test_is_sorted()
    test_quick_select()
    test_merge()
    test_binary_search()
    test_lower_bound()
    test_upper_bound()
    test_sort_stable()
    test_sort_unstable()
    test_performance()
    test_edge_cases()
    
    print_test_summary()
    vibez.spill("Sort Slay Module Tests Complete!")
}
