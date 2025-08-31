vibe main

yeet "vibez"

// Test validates floating point precision consistency between interpreter and compiled modes
// Focuses on edge cases like large numbers, small numbers, and precision loss scenarios

slay main_character() {
    vibez.spill("=== Float Precision Validation ===")
    
    // Basic float operations
    sus small_float drip = 0.0000001
    sus large_float drip = 999999.999999
    
    vibez.spill(small_float)
    vibez.spill(large_float)
    
    // Float arithmetic precision
    sus a drip = 0.1
    sus b drip = 0.2
    sus result drip = a + b
    vibez.spill(result)
    
    // Large number operations
    sus big1 drip = 1234567890.123456789
    sus big2 drip = 9876543210.987654321
    sus big_sum drip = big1 + big2
    vibez.spill(big_sum)
    
    // Division precision test
    sus div_result drip = 1.0 / 3.0
    vibez.spill(div_result)
    
    // Multiplication with precision loss potential
    sus mult_result drip = div_result * 3.0
    vibez.spill(mult_result)
    
    // Float arithmetic edge cases
    sus edge_case1 drip = 0.999999999999999
    sus edge_case2 drip = 1.0 - edge_case1
    vibez.spill(edge_case2)
    
    // Negative float precision
    sus neg_float drip = -123.456789
    sus abs_manual drip = 0.0 - neg_float
    vibez.spill(abs_manual)
    
    vibez.spill("=== Test Complete ===")
}
