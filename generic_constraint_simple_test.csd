yeet "vibez"

// Test basic generic function with numeric constraint
slay add_numbers[T: Numeric](a T, b T) T {
    damn a + b
}

// Test generic function with comparable constraint
slay compare_values[T: Comparable](a T, b T) lit {
    damn a == b
}

// Test main function
slay main() vibes {
    vibez.spill("Testing generic constraints...")
    
    // Test numeric constraint - should work
    sus result_int drip = add_numbers(10, 20)
    vibez.spill("Int addition result:", result_int)
    
    sus result_float meal = add_numbers(3.14, 2.86)  
    vibez.spill("Float addition result:", result_float)
    
    // Test comparable constraint - should work
    sus equal_ints lit = compare_values(10, 10)
    vibez.spill("Equal ints:", equal_ints)
    
    sus equal_strings lit = compare_values("hello", "world")
    vibez.spill("Equal strings:", equal_strings)
    
    vibez.spill("Generic constraint tests completed!")
}
