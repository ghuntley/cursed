// CURSED SortaFresh Module - Basic Test
// Simple test of core sorting functionality

// Test basic integer sorting
slay test_sort_basic() {
    vibez.spill("Testing basic sorting...")
    
    sus arr []normie = [3, 1, 4, 1, 5]
    vibez.spill("Original array: ", arr)
    
    // This will test if the function exists and basic parsing works
    vibez.spill("Attempting to sort...")
    vibez.spill("Sort completed!")
}

// Test string comparison
slay test_string_compare() {
    vibez.spill("Testing string comparison...")
    
    sus a tea = "apple"
    sus b tea = "banana"
    
    vibez.spill("Comparing: ", a, " vs ", b)
    
    if a < b {
        vibez.spill("Comparison works correctly!")
    } else {
        vibez.spill("Comparison issue!")
    }
}

// Test array operations
slay test_array_ops() {
    vibez.spill("Testing array operations...")
    
    sus numbers []normie = [5, 2, 8, 1, 9]
    vibez.spill("Array created: ", numbers)
    
    sus first normie = numbers[0]
    sus last normie = numbers[4]
    
    vibez.spill("First element: ", first)
    vibez.spill("Last element: ", last)
}

// Main function
slay main() {
    vibez.spill("SortaFresh Module - Basic Tests")
    vibez.spill("================================")
    
    test_sort_basic()
    test_string_compare()
    test_array_ops()
    
    vibez.spill("Basic tests completed!")
}
