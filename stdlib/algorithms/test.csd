yeet "testz"
yeet "algorithms"

test_start("Algorithms Critical Tests")

// Basic sorting correctness tests
slay test_bubble_sort() lit {
    sus arr []drip = [3, 1, 4, 1, 5, 9, 2, 6]
    sus sorted []drip = bubble_sort(arr)
    sus expected []drip = [1, 1, 2, 3, 4, 5, 6, 9]
    
    bestie (i drip = 0; i < len(sorted); i += 1) {
        assert_eq_int(sorted[i], expected[i])
    }
    damn based
}

slay test_quick_sort() lit {
    sus arr []drip = [64, 34, 25, 12, 22, 11, 90]
    sus sorted []drip = quick_sort(arr)
    sus expected []drip = [11, 12, 22, 25, 34, 64, 90]
    
    bestie (i drip = 0; i < len(sorted); i += 1) {
        assert_eq_int(sorted[i], expected[i])
    }
    damn based
}

slay test_merge_sort() lit {
    sus arr []drip = [12, 11, 13, 5, 6, 7]
    sus sorted []drip = merge_sort(arr)
    sus expected []drip = [5, 6, 7, 11, 12, 13]
    
    bestie (i drip = 0; i < len(sorted); i += 1) {
        assert_eq_int(sorted[i], expected[i])
    }
    damn based
}

slay test_heap_sort() lit {
    sus arr []drip = [4, 10, 3, 5, 1]
    sus sorted []drip = heap_sort(arr)
    sus expected []drip = [1, 3, 4, 5, 10]
    
    bestie (i drip = 0; i < len(sorted); i += 1) {
        assert_eq_int(sorted[i], expected[i])
    }
    damn based
}

// Binary search tests
slay test_binary_search() lit {
    sus arr []drip = [1, 3, 5, 7, 9, 11, 13, 15]
    sus index drip = binary_search(arr, 7)
    assert_eq_int(index, 3)
    
    sus not_found drip = binary_search(arr, 8)
    assert_eq_int(not_found, -1)
    damn based
}

// Edge case tests - empty arrays
slay test_empty_array_sorting() lit {
    sus empty_arr []drip = []
    sus sorted []drip = quick_sort(empty_arr)
    assert_eq_int(len(sorted), 0)
    damn based
}

// Single element tests
slay test_single_element() lit {
    sus single []drip = [42]
    sus sorted []drip = merge_sort(single)
    assert_eq_int(len(sorted), 1)
    assert_eq_int(sorted[0], 42)
    damn based
}

// Already sorted array tests
slay test_already_sorted() lit {
    sus arr []drip = [1, 2, 3, 4, 5]
    sus sorted []drip = bubble_sort(arr)
    
    bestie (i drip = 0; i < len(sorted); i += 1) {
        assert_eq_int(sorted[i], i + 1)
    }
    damn based
}

// Reverse sorted array tests
slay test_reverse_sorted() lit {
    sus arr []drip = [5, 4, 3, 2, 1]
    sus sorted []drip = quick_sort(arr)
    
    bestie (i drip = 0; i < len(sorted); i += 1) {
        assert_eq_int(sorted[i], i + 1)
    }
    damn based
}

// Performance test with large arrays
slay test_large_array_performance() lit {
    sus large_arr []drip = []
    bestie (i drip = 0; i < 1000; i += 1) {
        large_arr = append(large_arr, 1000 - i)
    }
    
    sus sorted []drip = quick_sort(large_arr)
    assert_eq_int(len(sorted), 1000)
    
    // Verify sorted correctly
    bestie (i drip = 1; i < len(sorted); i += 1) {
        assert_true(sorted[i-1] <= sorted[i])
    }
    damn based
}

// String sorting tests
slay test_string_sorting() lit {
    sus strings []tea = ["zebra", "apple", "banana", "cherry"]
    sus sorted []tea = sort_strings(strings)
    sus expected []tea = ["apple", "banana", "cherry", "zebra"]
    
    bestie (i drip = 0; i < len(sorted); i += 1) {
        assert_eq_string(sorted[i], expected[i])
    }
    damn based
}

// Duplicate elements test
slay test_duplicate_elements() lit {
    sus arr []drip = [3, 1, 3, 1, 2, 2]
    sus sorted []drip = merge_sort(arr)
    sus expected []drip = [1, 1, 2, 2, 3, 3]
    
    bestie (i drip = 0; i < len(sorted); i += 1) {
        assert_eq_int(sorted[i], expected[i])
    }
    damn based
}

// Search in unsorted array
slay test_linear_search() lit {
    sus arr []drip = [64, 34, 25, 12, 22, 11, 90]
    sus index drip = linear_search(arr, 22)
    assert_eq_int(index, 4)
    
    sus not_found drip = linear_search(arr, 99)
    assert_eq_int(not_found, -1)
    damn based
}

// Hash table/map operations
slay test_hash_operations() lit {
    sus map = create_hash_map()
    hash_insert(map, "key1", 100)
    hash_insert(map, "key2", 200)
    
    sus value1 drip = hash_get(map, "key1")
    assert_eq_int(value1, 100)
    
    sus exists lit = hash_contains(map, "key2")
    assert_true(exists)
    
    sus not_exists lit = hash_contains(map, "key3")
    assert_false(not_exists)
    damn based
}

// Memory safety test - sorting large arrays
slay test_memory_safety() lit {
    bestie (iteration drip = 0; iteration < 10; iteration += 1) {
        sus arr []drip = []
        bestie (i drip = 0; i < 100; i += 1) {
            arr = append(arr, i % 50)
        }
        
        sus sorted []drip = heap_sort(arr)
        assert_eq_int(len(sorted), 100)
        
        // Verify no memory corruption
        bestie (i drip = 1; i < len(sorted); i += 1) {
            assert_true(sorted[i-1] <= sorted[i])
        }
    }
    damn based
}

// Run all tests
test_bubble_sort()
test_quick_sort() 
test_merge_sort()
test_heap_sort()
test_binary_search()
test_empty_array_sorting()
test_single_element()
test_already_sorted()
test_reverse_sorted()
test_large_array_performance()
test_string_sorting()
test_duplicate_elements()
test_linear_search()
test_hash_operations()
test_memory_safety()

print_test_summary()
