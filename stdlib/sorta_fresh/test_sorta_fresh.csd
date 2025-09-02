// CURSED SortaFresh Module Tests
// Comprehensive test suite for modern sorting and caching library

yeet "testz"
yeet "sorta_fresh"

// Test basic integer sorting
slay test_sort_ints() {
    test_start("SortInts basic functionality")
    
    sus arr normie[value] = [3, 1, 4, 1, 5, 9, 2, 6]
    sus sorted normie[value] = SortInts(arr)
    
    assert_eq_int(sorted[0], 1)
    assert_eq_int(sorted[1], 1)
    assert_eq_int(sorted[2], 2)
    assert_eq_int(sorted[3], 3)
    assert_eq_int(sorted[4], 4)
    assert_eq_int(sorted[5], 5)
    assert_eq_int(sorted[6], 6)
    assert_eq_int(sorted[7], 9)
    
    test_start("SortInts empty array")
    sus empty normie[value] = []
    sus sorted_empty normie[value] = SortInts(empty)
    assert_eq_int(array_length(sorted_empty), 0)
    
    test_start("SortInts single element")
    sus single normie[value] = [42]
    sus sorted_single normie[value] = SortInts(single)
    assert_eq_int(sorted_single[0], 42)
    assert_eq_int(array_length(sorted_single), 1)
}

// Test string sorting
slay test_sort_strings() {
    test_start("SortStrings basic functionality")
    
    sus arr tea[value] = ["zebra", "apple", "banana", "cherry"]
    sus sorted tea[value] = SortStrings(arr)
    
    assert_eq_string(sorted[0], "apple")
    assert_eq_string(sorted[1], "banana")
    assert_eq_string(sorted[2], "cherry")
    assert_eq_string(sorted[3], "zebra")
    
    test_start("SortStrings empty array")
    sus empty tea[value] = []
    sus sorted_empty tea[value] = SortStrings(empty)
    assert_eq_int(array_length_str(sorted_empty), 0)
    
    test_start("SortStrings single element")
    sus single tea[value] = ["hello"]
    sus sorted_single tea[value] = SortStrings(single)
    assert_eq_string(sorted_single[0], "hello")
    assert_eq_int(array_length_str(sorted_single), 1)
}

// Test float sorting
slay test_sort_floats() {
    test_start("SortFloat64s basic functionality")
    
    sus arr meal[value] = [3.14, 2.71, 1.41, 1.73, 2.23]
    sus sorted meal[value] = SortFloat64s(arr)
    
    assert_true(sorted[0] < sorted[1])
    assert_true(sorted[1] < sorted[2])
    assert_true(sorted[2] < sorted[3])
    assert_true(sorted[3] < sorted[4])
    
    test_start("SortFloat64s with negatives")
    sus negative meal[value] = [-1.5, -2.5, -0.5, -3.0]
    sus sorted_neg meal[value] = SortFloat64s(negative)
    assert_true(sorted_neg[0] < sorted_neg[1])
    assert_true(sorted_neg[1] < sorted_neg[2])
    assert_true(sorted_neg[2] < sorted_neg[3])
}

// Test sorted checking functions
slay test_is_sorted() {
    test_start("IntsAreSorted true case")
    sus sorted normie[value] = [1, 2, 3, 4, 5]
    assert_true(IntsAreSorted(sorted))
    
    test_start("IntsAreSorted false case")
    sus unsorted normie[value] = [3, 1, 4, 1, 5]
    assert_false(IntsAreSorted(unsorted))
    
    test_start("StringsAreSorted true case")
    sus sorted_str tea[value] = ["apple", "banana", "cherry"]
    assert_true(StringsAreSorted(sorted_str))
    
    test_start("StringsAreSorted false case")
    sus unsorted_str tea[value] = ["zebra", "apple", "banana"]
    assert_false(StringsAreSorted(unsorted_str))
    
    test_start("Float64sAreSorted true case")
    sus sorted_float meal[value] = [1.1, 2.2, 3.3, 4.4]
    assert_true(Float64sAreSorted(sorted_float))
    
    test_start("Float64sAreSorted false case")
    sus unsorted_float meal[value] = [3.3, 1.1, 4.4, 2.2]
    assert_false(Float64sAreSorted(unsorted_float))
}

// Test binary search functions
slay test_binary_search() {
    test_start("SearchInts basic functionality")
    sus arr normie[value] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    
    assert_eq_int(SearchInts(arr, 5), 4)
    assert_eq_int(SearchInts(arr, 1), 0)
    assert_eq_int(SearchInts(arr, 9), 8)
    
    test_start("SearchInts not found")
    assert_eq_int(SearchInts(arr, 10), -1)
    assert_eq_int(SearchInts(arr, 0), -1)
    
    test_start("SearchStrings basic functionality")
    sus str_arr tea[value] = ["apple", "banana", "cherry", "date"]
    assert_eq_int(SearchStrings(str_arr, "banana"), 1)
    assert_eq_int(SearchStrings(str_arr, "apple"), 0)
    assert_eq_int(SearchStrings(str_arr, "date"), 3)
    
    test_start("SearchStrings not found")
    assert_eq_int(SearchStrings(str_arr, "zebra"), -1)
    
    test_start("SearchFloat64s basic functionality")
    sus float_arr meal[value] = [1.1, 2.2, 3.3, 4.4, 5.5]
    assert_eq_int(SearchFloat64s(float_arr, 3.3), 2)
    assert_eq_int(SearchFloat64s(float_arr, 1.1), 0)
    assert_eq_int(SearchFloat64s(float_arr, 5.5), 4)
    
    test_start("SearchFloat64s not found")
    assert_eq_int(SearchFloat64s(float_arr, 6.6), -1)
}

// Test stable sorting
slay test_stable_sort() {
    test_start("StableSort basic functionality")
    sus arr normie[value] = [3, 1, 4, 1, 5, 9, 2, 6]
    sus sorted normie[value] = StableSort(arr)
    
    assert_eq_int(sorted[0], 1)
    assert_eq_int(sorted[1], 1)
    assert_eq_int(sorted[2], 2)
    assert_eq_int(sorted[3], 3)
    assert_eq_int(sorted[4], 4)
    assert_eq_int(sorted[5], 5)
    assert_eq_int(sorted[6], 6)
    assert_eq_int(sorted[7], 9)
    
    assert_true(IntsAreSorted(sorted))
}

// Test reverse sorting
slay test_reverse_sort() {
    test_start("ReverseSort basic functionality")
    sus arr normie[value] = [1, 3, 2, 5, 4]
    sus reversed normie[value] = ReverseSort(arr)
    
    assert_eq_int(reversed[0], 5)
    assert_eq_int(reversed[1], 4)
    assert_eq_int(reversed[2], 3)
    assert_eq_int(reversed[3], 2)
    assert_eq_int(reversed[4], 1)
}

// Test shuffling
slay test_shuffle() {
    test_start("Shuffle basic functionality")
    sus arr normie[value] = [1, 2, 3, 4, 5]
    sus shuffled normie[value] = Shuffle(arr)
    
    // Check that all elements are still present
    assert_eq_int(array_length(shuffled), 5)
    
    // Check that shuffled array contains all original elements
    sus found_1 lit = cap
    sus found_2 lit = cap
    sus found_3 lit = cap
    sus found_4 lit = cap
    sus found_5 lit = cap
    
    bestie i := 0; i < 5; i++ {
        if shuffled[i] == 1 { found_1 = based }
        if shuffled[i] == 2 { found_2 = based }
        if shuffled[i] == 3 { found_3 = based }
        if shuffled[i] == 4 { found_4 = based }
        if shuffled[i] == 5 { found_5 = based }
    }
    
    assert_true(found_1)
    assert_true(found_2)
    assert_true(found_3)
    assert_true(found_4)
    assert_true(found_5)
}

// Test Top-K and Bottom-K functions
slay test_top_bottom_k() {
    test_start("TopK basic functionality")
    sus arr normie[value] = [3, 1, 4, 1, 5, 9, 2, 6]
    sus top3 normie[value] = TopK(arr, 3)
    
    assert_eq_int(array_length(top3), 3)
    assert_eq_int(top3[0], 9)
    assert_eq_int(top3[1], 6)
    assert_eq_int(top3[2], 5)
    
    test_start("BottomK basic functionality")
    sus bottom3 normie[value] = BottomK(arr, 3)
    
    assert_eq_int(array_length(bottom3), 3)
    assert_eq_int(bottom3[0], 1)
    assert_eq_int(bottom3[1], 1)
    assert_eq_int(bottom3[2], 2)
    
    test_start("TopK with k=0")
    sus top0 normie[value] = TopK(arr, 0)
    assert_eq_int(array_length(top0), 0)
    
    test_start("TopK with k > array length")
    sus top100 normie[value] = TopK(arr, 100)
    assert_eq_int(array_length(top100), 0)
}

// Test median finding
slay test_median() {
    test_start("Median odd length")
    sus arr_odd normie[value] = [1, 3, 5, 7, 9]
    sus median_odd normie = Median(arr_odd)
    assert_eq_int(median_odd, 5)
    
    test_start("Median even length")
    sus arr_even normie[value] = [1, 2, 3, 4]
    sus median_even normie = Median(arr_even)
    assert_eq_int(median_even, 2)  // Returns element at index length/2
    
    test_start("Median single element")
    sus arr_single normie[value] = [42]
    sus median_single normie = Median(arr_single)
    assert_eq_int(median_single, 42)
}

// Test quick select
slay test_quick_select() {
    test_start("QuickSelect basic functionality")
    sus arr normie[value] = [3, 1, 4, 1, 5, 9, 2, 6]
    
    // Find the smallest element (k=0)
    sus smallest normie = QuickSelect(arr, 0)
    assert_eq_int(smallest, 1)
    
    // Find the 3rd smallest element (k=2)
    sus third_smallest normie = QuickSelect(arr, 2)
    assert_eq_int(third_smallest, 2)
    
    // Find the largest element (k=7)
    sus largest normie = QuickSelect(arr, 7)
    assert_eq_int(largest, 9)
    
    test_start("QuickSelect out of bounds")
    sus invalid normie = QuickSelect(arr, 10)
    assert_eq_int(invalid, -1)
    
    sus negative normie = QuickSelect(arr, -1)
    assert_eq_int(negative, -1)
}

// Test caching functionality
slay test_cache() {
    test_start("SortCache creation")
    sus cache SortCache = NewSortCache(3)
    
    test_start("CachedSort first time")
    sus arr1 normie[value] = [3, 1, 4, 1, 5]
    sus sorted1 normie[value] = CachedSort(cache, arr1)
    assert_true(IntsAreSorted(sorted1))
    
    test_start("CachedSort cache hit")
    sus sorted1_cached normie[value] = CachedSort(cache, arr1)
    assert_true(IntsAreSorted(sorted1_cached))
    
    test_start("ClearCache functionality")
    ClearCache(cache)
}

// Test Gen Z sorting features
slay test_gen_z_sorting() {
    test_start("VibeSort basic functionality")
    sus arr normie[value] = [5, 3, 8, 1, 9]
    sus vibe_sorted normie[value] = VibeSort(arr)
    
    // Should be sorted by vibe score (higher vibe = lower value)
    assert_true(get_vibe_score(vibe_sorted[0]) >= get_vibe_score(vibe_sorted[1]))
    assert_true(get_vibe_score(vibe_sorted[1]) >= get_vibe_score(vibe_sorted[2]))
    
    test_start("NoCapSort basic functionality")
    sus no_cap_sorted normie[value] = NoCapSort(arr)
    assert_true(IntsAreSorted(no_cap_sorted))
    
    test_start("BussinSort basic functionality")
    sus bussin_sorted normie[value] = BussinSort(arr)
    
    // Should be sorted by bussin score (higher bussin = higher value)
    assert_true(get_bussin_score(bussin_sorted[0]) >= get_bussin_score(bussin_sorted[1]))
    assert_true(get_bussin_score(bussin_sorted[1]) >= get_bussin_score(bussin_sorted[2]))
    
    test_start("SlaySort basic functionality")
    sus slay_sorted normie[value] = SlaySort(arr)
    assert_true(IntsAreSorted(slay_sorted))
    
    test_start("YeetSort basic functionality")
    sus yeet_sorted normie[value] = YeetSort(arr, 5)
    
    // Should only contain elements >= 5
    bestie i := 0; i < array_length(yeet_sorted); i++ {
        assert_true(yeet_sorted[i] >= 5)
    }
    assert_true(IntsAreSorted(yeet_sorted))
}

// Test utility functions
slay test_utilities() {
    test_start("string_compare functionality")
    assert_eq_int(string_compare("apple", "banana"), -1)
    assert_eq_int(string_compare("banana", "apple"), 1)
    assert_eq_int(string_compare("apple", "apple"), 0)
    
    test_start("copy_int_array functionality")
    sus original normie[value] = [1, 2, 3, 4, 5]
    sus copied normie[value] = copy_int_array(original)
    
    assert_eq_int(array_length(copied), 5)
    assert_eq_int(copied[0], 1)
    assert_eq_int(copied[4], 5)
    
    test_start("arrays_equal functionality")
    sus arr1 normie[value] = [1, 2, 3]
    sus arr2 normie[value] = [1, 2, 3]
    sus arr3 normie[value] = [1, 2, 4]
    
    assert_true(arrays_equal(arr1, arr2))
    assert_false(arrays_equal(arr1, arr3))
    
    test_start("vibe and bussin scoring")
    assert_eq_int(get_vibe_score(10), 90)
    assert_eq_int(get_vibe_score(50), 50)
    
    assert_eq_int(get_bussin_score(10), 20)
    assert_eq_int(get_bussin_score(25), 50)
}

// Test edge cases
slay test_edge_cases() {
    test_start("empty array handling")
    sus empty normie[value] = []
    sus sorted_empty normie[value] = SortInts(empty)
    assert_eq_int(array_length(sorted_empty), 0)
    
    sus empty_strings tea[value] = []
    sus sorted_empty_strings tea[value] = SortStrings(empty_strings)
    assert_eq_int(array_length_str(sorted_empty_strings), 0)
    
    test_start("single element arrays")
    sus single_int normie[value] = [42]
    sus sorted_single_int normie[value] = SortInts(single_int)
    assert_eq_int(sorted_single_int[0], 42)
    
    sus single_string tea[value] = ["hello"]
    sus sorted_single_string tea[value] = SortStrings(single_string)
    assert_eq_string(sorted_single_string[0], "hello")
    
    test_start("duplicate elements")
    sus duplicates normie[value] = [5, 5, 5, 5, 5]
    sus sorted_duplicates normie[value] = SortInts(duplicates)
    assert_true(IntsAreSorted(sorted_duplicates))
    assert_eq_int(sorted_duplicates[0], 5)
    assert_eq_int(sorted_duplicates[4], 5)
    
    test_start("already sorted arrays")
    sus already_sorted normie[value] = [1, 2, 3, 4, 5]
    sus sorted_already normie[value] = SortInts(already_sorted)
    assert_true(IntsAreSorted(sorted_already))
    
    test_start("reverse sorted arrays")
    sus reverse_sorted normie[value] = [5, 4, 3, 2, 1]
    sus sorted_reverse normie[value] = SortInts(reverse_sorted)
    assert_true(IntsAreSorted(sorted_reverse))
    assert_eq_int(sorted_reverse[0], 1)
    assert_eq_int(sorted_reverse[4], 5)
}

// Test performance with larger arrays
slay test_performance() {
    test_start("performance with medium array")
    sus arr normie[value] = [50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
    
    sus sorted normie[value] = SortInts(arr)
    assert_true(IntsAreSorted(sorted))
    assert_eq_int(sorted[0], 1)
    assert_eq_int(sorted[49], 50)
    
    test_start("performance with random-ish data")
    sus random_arr normie[value] = [23, 45, 12, 78, 34, 56, 89, 1, 67, 90, 43, 21, 65, 87, 32, 54, 76, 98, 10, 33]
    sus sorted_random normie[value] = SortInts(random_arr)
    assert_true(IntsAreSorted(sorted_random))
}

// Run all tests
slay main() {
    vibez.spill("Running SortaFresh Module Tests...")
    
    test_sort_ints()
    test_sort_strings()
    test_sort_floats()
    test_is_sorted()
    test_binary_search()
    test_stable_sort()
    test_reverse_sort()
    test_shuffle()
    test_top_bottom_k()
    test_median()
    test_quick_select()
    test_cache()
    test_gen_z_sorting()
    test_utilities()
    test_edge_cases()
    test_performance()
    
    print_test_summary()
    vibez.spill("SortaFresh Module Tests Complete!")
}
