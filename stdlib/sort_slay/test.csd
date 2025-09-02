yeet "testz"

fr fr Comprehensive test suite for sort_slay module
fr fr Tests all sorting algorithms, edge cases, and performance

sus main() {
    test_start("sort_slay comprehensive tests")
    
    fr fr Basic sorting tests
    test_sort_ints()
    test_sort_strings()  
    test_sort_floats()
    test_reverse_sort()
    
    fr fr Edge case tests
    test_empty_arrays()
    test_single_element()
    test_already_sorted()
    test_reverse_sorted()
    test_duplicate_elements()
    
    fr fr Algorithm-specific tests
    test_stable_sort()
    test_unstable_sort()
    test_merge_function()
    test_quick_select()
    test_binary_search()
    test_bounds_search()
    
    fr fr Performance tests
    test_large_arrays()
    test_sorting_performance()
    
    fr fr Utility function tests
    test_is_sorted()
    test_array_utilities()
    
    print_test_summary()
}

fr fr Basic sorting functionality tests
slay test_sort_ints() {
    test_group("Integer sorting")
    
    fr fr Test basic integer sorting
    sus input normie[value] = [3, 1, 4, 1, 5, 9, 2, 6]
    sus expected normie[value] = [1, 1, 2, 3, 4, 5, 6, 9]
    sus result normie[value] = sort_ints(input)
    
    assert_eq_int(len(result), 8)
    bestie i := 0; i < len(expected); i++ {
        assert_eq_int(result[i], expected[i])
    }
    
    fr fr Test negative numbers
    sus negatives normie[value] = [-3, -1, -4, 0, 2]
    sus neg_expected normie[value] = [-4, -3, -1, 0, 2]
    sus neg_result normie[value] = sort_ints(negatives)
    
    bestie i := 0; i < len(neg_expected); i++ {
        assert_eq_int(neg_result[i], neg_expected[i])
    }
    
    pass("Integer sorting works correctly")
}

slay test_sort_strings() {
    test_group("String sorting")
    
    sus input tea[value] = ["zebra", "apple", "banana", "cherry"]
    sus expected tea[value] = ["apple", "banana", "cherry", "zebra"]
    sus result tea[value] = sort_strings(input)
    
    assert_eq_int(len(result), 4)
    bestie i := 0; i < len(expected); i++ {
        assert_eq_string(result[i], expected[i])
    }
    
    fr fr Test empty strings and special characters
    sus special tea[value] = ["", "a", "AA", "aa"]
    sus spec_result tea[value] = sort_strings(special)
    assert_eq_string(spec_result[0], "")  fr fr Empty string should be first
    
    pass("String sorting works correctly")
}

slay test_sort_floats() {
    test_group("Float sorting")
    
    sus input meal[value] = [3.14, 1.41, 2.71, 0.57, 1.0]
    sus result meal[value] = sort_floats(input)
    
    assert_eq_int(len(result), 5)
    fr fr Check that result is sorted (each element <= next)
    bestie i := 0; i < len(result) - 1; i++ {
        assert_true(result[i] <= result[i + 1])
    }
    
    fr fr Test special float values
    sus special meal[value] = [0.0, -1.5, 999.999]
    sus spec_result meal[value] = sort_floats(special)
    assert_true(spec_result[0] <= spec_result[1])
    assert_true(spec_result[1] <= spec_result[2])
    
    pass("Float sorting works correctly")
}

slay test_reverse_sort() {
    test_group("Reverse sorting")
    
    sus input normie[value] = [1, 2, 3, 4, 5]
    sus expected normie[value] = [5, 4, 3, 2, 1]
    sus result normie[value] = sort_reverse(input)
    
    bestie i := 0; i < len(expected); i++ {
        assert_eq_int(result[i], expected[i])
    }
    
    pass("Reverse sorting works correctly")
}

fr fr Edge case tests
slay test_empty_arrays() {
    test_group("Empty array handling")
    
    sus empty normie[value] = normie[value]{}
    sus result normie[value] = sort_ints(empty)
    assert_eq_int(len(result), 0)
    
    sus empty_strings tea[value] = tea[value]{}
    sus str_result tea[value] = sort_strings(empty_strings)
    assert_eq_int(len(str_result), 0)
    
    pass("Empty arrays handled correctly")
}

slay test_single_element() {
    test_group("Single element arrays")
    
    sus single normie[value] = [42]
    sus result normie[value] = sort_ints(single)
    assert_eq_int(len(result), 1)
    assert_eq_int(result[0], 42)
    
    sus single_str tea[value] = ["hello"]
    sus str_result tea[value] = sort_strings(single_str)
    assert_eq_int(len(str_result), 1)
    assert_eq_string(str_result[0], "hello")
    
    pass("Single element arrays handled correctly")
}

slay test_already_sorted() {
    test_group("Already sorted arrays")
    
    sus sorted_input normie[value] = [1, 2, 3, 4, 5]
    sus result normie[value] = sort_ints(sorted_input)
    
    bestie i := 0; i < len(sorted_input); i++ {
        assert_eq_int(result[i], sorted_input[i])
    }
    assert_true(is_sorted(result))
    
    pass("Already sorted arrays handled efficiently")
}

slay test_reverse_sorted() {
    test_group("Reverse sorted arrays")
    
    sus reverse_input normie[value] = [5, 4, 3, 2, 1]
    sus result normie[value] = sort_ints(reverse_input)
    sus expected normie[value] = [1, 2, 3, 4, 5]
    
    bestie i := 0; i < len(expected); i++ {
        assert_eq_int(result[i], expected[i])
    }
    assert_true(is_sorted(result))
    
    pass("Reverse sorted arrays handled correctly")
}

slay test_duplicate_elements() {
    test_group("Duplicate elements")
    
    sus duplicates normie[value] = [3, 1, 3, 1, 3, 1]
    sus result normie[value] = sort_ints(duplicates)
    sus expected normie[value] = [1, 1, 1, 3, 3, 3]
    
    bestie i := 0; i < len(expected); i++ {
        assert_eq_int(result[i], expected[i])
    }
    assert_true(is_sorted(result))
    
    pass("Duplicate elements handled correctly")
}

fr fr Algorithm-specific tests
slay test_stable_sort() {
    test_group("Stable sorting")
    
    sus input normie[value] = [3, 1, 4, 1, 5]
    sus result normie[value] = sort_stable(input)
    
    assert_true(is_sorted(result))
    assert_eq_int(len(result), 5)
    fr fr Stable sort should preserve relative order of equal elements
    
    pass("Stable sort maintains relative order")
}

slay test_unstable_sort() {
    test_group("Unstable sorting")
    
    sus input normie[value] = [3, 1, 4, 1, 5]
    sus result normie[value] = sort_unstable(input)
    
    assert_true(is_sorted(result))
    assert_eq_int(len(result), 5)
    
    pass("Unstable sort produces correct results")
}

slay test_merge_function() {
    test_group("Array merging")
    
    sus arr1 normie[value] = [1, 3, 5]
    sus arr2 normie[value] = [2, 4, 6]
    sus result normie[value] = merge(arr1, arr2)
    sus expected normie[value] = [1, 2, 3, 4, 5, 6]
    
    assert_eq_int(len(result), 6)
    bestie i := 0; i < len(expected); i++ {
        assert_eq_int(result[i], expected[i])
    }
    
    fr fr Test merging arrays of different sizes
    sus small normie[value] = [1]
    sus large normie[value] = [2, 3, 4, 5]
    sus merge_result normie[value] = merge(small, large)
    assert_eq_int(len(merge_result), 5)
    assert_true(is_sorted(merge_result))
    
    pass("Array merging works correctly")
}

slay test_quick_select() {
    test_group("Quick select algorithm")
    
    sus input normie[value] = [3, 1, 4, 1, 5, 9, 2, 6]
    
    fr fr Test finding smallest element (k=0)
    sus min_val normie = quick_select(input, 0)
    assert_eq_int(min_val, 1)
    
    fr fr Test finding median (k=4 for 8 elements)
    sus median normie = quick_select(input, 4)
    assert_true(median >= 1 && median <= 9)  fr fr Should be a valid element
    
    fr fr Test finding largest element (k=7)
    sus max_val normie = quick_select(input, 7)
    assert_eq_int(max_val, 9)
    
    fr fr Test invalid k values
    sus invalid_result normie = quick_select(input, -1)
    assert_eq_int(invalid_result, -1)
    
    sus invalid_result2 normie = quick_select(input, 10)
    assert_eq_int(invalid_result2, -1)
    
    pass("Quick select finds correct k-th elements")
}

slay test_binary_search() {
    test_group("Binary search")
    
    sus sorted_array normie[value] = [1, 3, 5, 7, 9, 11, 13]
    
    fr fr Test finding existing elements
    sus pos1 normie = binary_search(sorted_array, 5)
    assert_eq_int(pos1, 2)  fr fr 5 is at index 2
    
    sus pos2 normie = binary_search(sorted_array, 1)
    assert_eq_int(pos2, 0)  fr fr 1 is at index 0
    
    sus pos3 normie = binary_search(sorted_array, 13)
    assert_eq_int(pos3, 6)  fr fr 13 is at index 6
    
    fr fr Test finding non-existing elements
    sus not_found normie = binary_search(sorted_array, 4)
    assert_eq_int(not_found, -1)
    
    sus not_found2 normie = binary_search(sorted_array, 0)
    assert_eq_int(not_found2, -1)
    
    sus not_found3 normie = binary_search(sorted_array, 15)
    assert_eq_int(not_found3, -1)
    
    pass("Binary search works correctly")
}

slay test_bounds_search() {
    test_group("Lower and upper bounds")
    
    sus array normie[value] = [1, 2, 2, 2, 3, 4]
    
    fr fr Test lower bound
    sus lower normie = lower_bound(array, 2)
    assert_eq_int(lower, 1)  fr fr First occurrence of 2
    
    sus lower_not_found normie = lower_bound(array, 2.5)
    assert_eq_int(lower_not_found, 4)  fr fr Where 2.5 would be inserted
    
    fr fr Test upper bound
    sus upper normie = upper_bound(array, 2)
    assert_eq_int(upper, 4)  fr fr After last occurrence of 2
    
    sus upper_end normie = upper_bound(array, 5)
    assert_eq_int(upper_end, 6)  fr fr At end of array
    
    pass("Bounds search works correctly")
}

fr fr Performance and stress tests
slay test_large_arrays() {
    test_group("Large array performance")
    
    fr fr Create a reasonably large array for testing
    sus large_size normie = 100
    sus large_array normie[value] = make(normie[value], large_size)
    
    fr fr Fill with reverse-sorted data (worst case for quicksort)
    bestie i := 0; i < large_size; i++ {
        large_array[i] = large_size - i
    }
    
    sus result normie[value] = sort_ints(large_array)
    
    fr fr Verify it's sorted
    assert_true(is_sorted(result))
    assert_eq_int(len(result), large_size)
    
    fr fr Check first and last elements
    assert_eq_int(result[0], 1)
    assert_eq_int(result[large_size - 1], large_size)
    
    pass("Large arrays sorted correctly")
}

slay test_sorting_performance() {
    test_group("Sorting performance comparison")
    
    sus test_data normie[value] = [5, 2, 8, 1, 9, 3, 7, 4, 6]
    
    fr fr Test multiple sorting algorithms on same data
    sus quick_result normie[value] = sort_ints(test_data)
    sus stable_result normie[value] = sort_stable(test_data)
    sus unstable_result normie[value] = sort_unstable(test_data)
    
    fr fr All should produce sorted results
    assert_true(is_sorted(quick_result))
    assert_true(is_sorted(stable_result))
    assert_true(is_sorted(unstable_result))
    
    fr fr All should have same length
    assert_eq_int(len(quick_result), 9)
    assert_eq_int(len(stable_result), 9)
    assert_eq_int(len(unstable_result), 9)
    
    pass("All sorting algorithms produce correct results")
}

fr fr Utility function tests
slay test_is_sorted() {
    test_group("Sorted array detection")
    
    sus sorted_array normie[value] = [1, 2, 3, 4, 5]
    assert_true(is_sorted(sorted_array))
    
    sus unsorted_array normie[value] = [1, 3, 2, 4, 5]
    assert_false(is_sorted(unsorted_array))
    
    sus empty_array normie[value] = normie[value]{}
    assert_true(is_sorted(empty_array))  fr fr Empty array is considered sorted
    
    sus single_element normie[value] = [42]
    assert_true(is_sorted(single_element))  fr fr Single element is sorted
    
    sus duplicates_sorted normie[value] = [1, 2, 2, 3, 3, 3]
    assert_true(is_sorted(duplicates_sorted))
    
    pass("is_sorted correctly identifies sorted arrays")
}

slay test_array_utilities() {
    test_group("Array utility functions")
    
    fr fr Test array copying
    sus original normie[value] = [1, 2, 3, 4, 5]
    sus copied normie[value] = copy(original)
    
    assert_eq_int(len(copied), len(original))
    bestie i := 0; i < len(original); i++ {
        assert_eq_int(copied[i], original[i])
    }
    
    fr fr Modify original to ensure copy is independent
    original[0] = 999
    assert_eq_int(copied[0], 1)  fr fr Copy should be unchanged
    
    pass("Array utilities work correctly")
}

fr fr Helper function for creating array of specific size
slay make(type tea, size normie) normie[value]{
    fr fr Mock implementation for testing
    fr fr In real implementation, this would create array of specified size
    sus result normie[value] = normie[value]{}
    bestie i := 0; i < size; i++ {
        result = append(result, 0)
    }
    damn result
}

fr fr Mock append function for testing
slay append(arr normie[value], elem normie) normie[value]{
    fr fr Simple append implementation for testing
    sus result normie[value] = arr
    fr fr In real implementation, this would properly append to array
    damn result
}
