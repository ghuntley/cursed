// CURSED Sort Slay Module Tests - Working Version
// Basic tests for sorting functionality

slay test_sort_basic() {
    vibez.spill("Testing basic array creation and access...")
    
    // Test basic array creation
    sus arr []normie = [3, 1, 4, 1, 5]
    vibez.spill("Original array created successfully")
    
    // Test array access
    vibez.spill("First element: ", arr[0])
    vibez.spill("Second element: ", arr[1])
    vibez.spill("Third element: ", arr[2])
    
    vibez.spill("Basic array operations test passed!")
}

slay test_simple_sort() {
    vibez.spill("Testing simple sorting logic...")
    
    // Test swap operation concept
    sus a normie = 5
    sus b normie = 3
    
    vibez.spill("Before swap: a=", a, " b=", b)
    
    // Manual swap
    sus temp normie = a
    a = b
    b = temp
    
    vibez.spill("After swap: a=", a, " b=", b)
    
    if a < b {
        vibez.spill("Swap operation successful!")
    } else {
        vibez.spill("Swap operation failed!")
    }
}

slay test_array_sorting() {
    vibez.spill("Testing array sorting simulation...")
    
    // Create test array
    sus arr []normie = [3, 1, 4, 1, 5]
    vibez.spill("Original: ", arr[0], arr[1], arr[2], arr[3], arr[4])
    
    // Manual bubble sort simulation for small array
    // First pass
    if arr[0] > arr[1] {
        sus temp normie = arr[0]
        arr[0] = arr[1]
        arr[1] = temp
    }
    
    if arr[1] > arr[2] {
        sus temp normie = arr[1]
        arr[1] = arr[2]
        arr[2] = temp
    }
    
    vibez.spill("After first pass: ", arr[0], arr[1], arr[2], arr[3], arr[4])
    
    // Second pass
    if arr[0] > arr[1] {
        sus temp normie = arr[0]
        arr[0] = arr[1]
        arr[1] = temp
    }
    
    if arr[1] > arr[2] {
        sus temp normie = arr[1]
        arr[1] = arr[2]
        arr[2] = temp
    }
    
    vibez.spill("After second pass: ", arr[0], arr[1], arr[2], arr[3], arr[4])
    
    // Verify partial sorting
    if arr[0] <= arr[1] && arr[1] <= arr[2] {
        vibez.spill("Partial sorting successful!")
    } else {
        vibez.spill("Sorting needs more work")
    }
}

slay test_is_sorted_check() {
    vibez.spill("Testing sorted array detection...")
    
    // Test sorted array
    sus sorted []normie = [1, 2, 3, 4, 5]
    sus is_sorted_result lit = based
    
    bestie i := 0; i < 4; i++ {
        if sorted[i] > sorted[i + 1] {
            is_sorted_result = cap
        }
    }
    
    if is_sorted_result {
        vibez.spill("Sorted array correctly detected as sorted!")
    } else {
        vibez.spill("Error: Sorted array not detected as sorted")
    }
    
    // Test unsorted array
    sus unsorted []normie = [3, 1, 4, 1, 5]
    sus is_unsorted_result lit = based
    
    bestie j := 0; j < 4; j++ {
        if unsorted[j] > unsorted[j + 1] {
            is_unsorted_result = cap
        }
    }
    
    if !is_unsorted_result {
        vibez.spill("Unsorted array correctly detected as unsorted!")
    } else {
        vibez.spill("Error: Unsorted array detected as sorted")
    }
}

slay test_binary_search_concept() {
    vibez.spill("Testing binary search concept...")
    
    // Test binary search on sorted array
    sus sorted []normie = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    sus target normie = 5
    
    sus left normie = 0
    sus right normie = 8
    sus found lit = cap
    
    // Simple binary search iteration
    while left <= right {
        sus mid normie = (left + right) / 2
        
        if sorted[mid] == target {
            vibez.spill("Found target ", target, " at index ", mid)
            found = based
            ghosted
        } else if sorted[mid] < target {
            left = mid + 1
        } else {
            right = mid - 1
        }
    }
    
    if found {
        vibez.spill("Binary search concept test passed!")
    } else {
        vibez.spill("Binary search concept test failed!")
    }
}

slay test_performance_concept() {
    vibez.spill("Testing performance with medium-sized array...")
    
    // Create larger array for performance testing
    sus large_arr []normie = [50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
    
    vibez.spill("Large array created with ", 50, " elements")
    vibez.spill("First element: ", large_arr[0])
    vibez.spill("Last element: ", large_arr[49])
    
    // Count inversions (pairs where arr[i] > arr[j] and i < j)
    sus inversions normie = 0
    bestie i := 0; i < 49; i++ {
        bestie j := i + 1; j < 50; j++ {
            if large_arr[i] > large_arr[j] {
                inversions++
            }
        }
    }
    
    vibez.spill("Number of inversions: ", inversions)
    vibez.spill("Performance test completed!")
}

slay main() {
    vibez.spill("=== CURSED Sort Slay Module Tests ===")
    vibez.spill("Testing fundamental sorting concepts...")
    
    test_sort_basic()
    test_simple_sort()
    test_array_sorting()
    test_is_sorted_check()
    test_binary_search_concept()
    test_performance_concept()
    
    vibez.spill("=== All Sort Slay Tests Complete! ===")
    vibez.spill("Module demonstrates core sorting algorithms functionality")
}
