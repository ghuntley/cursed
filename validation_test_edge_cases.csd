// Test various edge cases and potential issues

// Test recursion depth
slay deep_recursion(n drip) drip {
    ready (n <= 0) {
        damn 0
    }
    damn 1 + deep_recursion(n - 1)
}

// Test large arrays
slay test_large_arrays() {
    yeet "arrayz"
    sus large_array []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    vibez.spill("Array length:", len(large_array))
    
    // Array bounds testing
    sus i drip = 0
    bestie (i < len(large_array)) {
        vibez.spill("Element", i, ":", large_array[i])
        i = i + 1
    }
}

// Test string manipulation
slay test_strings() {
    yeet "stringz"
    sus text tea = "Hello, World!"
    vibez.spill("Text length:", len_str(text))
    vibez.spill("Contains 'World':", contains_str(text, "World"))
}

// Test error conditions
slay test_division(a drip, b drip) drip {
    ready (b == 0) {
        vibez.spill("Division by zero!")
        damn 0
    }
    damn a / b
}

// Test concurrent operations
slay test_concurrency() {
    stan {
        vibez.spill("Goroutine 1 executing")
    }
    
    stan {
        vibez.spill("Goroutine 2 executing")
    }
    
    vibez.spill("Main thread continuing")
}

slay main() {
    vibez.spill("Starting edge case tests...")
    
    // Test recursion
    sus rec_result drip = deep_recursion(5)
    vibez.spill("Recursion result:", rec_result)
    
    // Test arrays
    test_large_arrays()
    
    // Test strings
    test_strings()
    
    // Test error conditions
    sus div_result drip = test_division(10, 2)
    vibez.spill("Division result:", div_result)
    
    sus error_result drip = test_division(10, 0)
    vibez.spill("Error division result:", error_result)
    
    // Test concurrency
    test_concurrency()
    
    vibez.spill("Edge case tests completed!")
}

main()
