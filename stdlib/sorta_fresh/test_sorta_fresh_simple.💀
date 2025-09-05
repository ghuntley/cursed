// CURSED SortaFresh Module Tests - Simplified Version
// Basic test suite for core sorting functionality

yeet "testz"
yeet "sorta_fresh"

// Test basic integer sorting
slay test_sort_ints() {
    test_start("SortInts basic functionality")
    
    sus arr normie[value] = [3, 1, 4, 1, 5]
    sus sorted normie[value] = SortInts(arr)
    
    assert_eq_int(sorted[0], 1)
    assert_eq_int(sorted[1], 1)
    assert_eq_int(sorted[2], 3)
    assert_eq_int(sorted[3], 4)
    assert_eq_int(sorted[4], 5)
    
    test_start("SortInts single element")
    sus single normie[value] = [42]
    sus sorted_single normie[value] = SortInts(single)
    assert_eq_int(sorted_single[0], 42)
}

// Test string sorting
slay test_sort_strings() {
    test_start("SortStrings basic functionality")
    
    sus arr tea[value] = ["zebra", "apple", "banana"]
    sus sorted tea[value] = SortStrings(arr)
    
    assert_eq_string(sorted[0], "apple")
    assert_eq_string(sorted[1], "banana")
    assert_eq_string(sorted[2], "zebra")
}

// Test sorted checking
slay test_is_sorted() {
    test_start("IntsAreSorted true case")
    sus sorted normie[value] = [1, 2, 3, 4, 5]
    assert_true(IntsAreSorted(sorted))
    
    test_start("IntsAreSorted false case")
    sus unsorted normie[value] = [3, 1, 4, 1, 5]
    assert_false(IntsAreSorted(unsorted))
}

// Test binary search
slay test_binary_search() {
    test_start("SearchInts basic functionality")
    sus arr normie[value] = [1, 2, 3, 4, 5]
    
    assert_eq_int(SearchInts(arr, 3), 2)
    assert_eq_int(SearchInts(arr, 1), 0)
    assert_eq_int(SearchInts(arr, 5), 4)
    
    test_start("SearchInts not found")
    assert_eq_int(SearchInts(arr, 10), -1)
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

// Test Gen Z sorting
slay test_gen_z_sorting() {
    test_start("NoCapSort basic functionality")
    sus arr normie[value] = [5, 3, 8, 1, 9]
    sus no_cap_sorted normie[value] = NoCapSort(arr)
    assert_true(IntsAreSorted(no_cap_sorted))
    
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

// Test edge cases
slay test_edge_cases() {
    test_start("single element arrays")
    sus single_int normie[value] = [42]
    sus sorted_single_int normie[value] = SortInts(single_int)
    assert_eq_int(sorted_single_int[0], 42)
    
    test_start("duplicate elements")
    sus duplicates normie[value] = [5, 5, 5]
    sus sorted_duplicates normie[value] = SortInts(duplicates)
    assert_true(IntsAreSorted(sorted_duplicates))
    assert_eq_int(sorted_duplicates[0], 5)
    assert_eq_int(sorted_duplicates[2], 5)
    
    test_start("already sorted arrays")
    sus already_sorted normie[value] = [1, 2, 3, 4, 5]
    sus sorted_already normie[value] = SortInts(already_sorted)
    assert_true(IntsAreSorted(sorted_already))
}

// Run all tests
slay main_character() {
    vibez.spill("Running SortaFresh Module Tests (Simplified)...")
    
    test_sort_ints()
    test_sort_strings()
    test_is_sorted()
    test_binary_search()
    test_reverse_sort()
    test_gen_z_sorting()
    test_edge_cases()
    
    print_test_summary()
    vibez.spill("SortaFresh Module Tests Complete!")
}
