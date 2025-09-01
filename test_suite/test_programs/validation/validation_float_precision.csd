vibe main_character

yeet "vibez"

fr fr Test validates floating point precision consistency between interpreter and compiled modes
fr fr Focuses on edge cases like large numbers, small numbers, and precision loss scenarios

slay main_character() {
    vibez.spill("=== Float Precision Validation ===")
    
    fr fr Basic float operations
    sus small_float meal = 0.0000001
    sus large_float meal = 999999.999999
    
    vibez.spill(small_float)
    vibez.spill(large_float)
    
    fr fr Float arithmetic precision
    sus a meal = 0.1
    sus b meal = 0.2
    sus result meal = a + b
    vibez.spill(result)
    
    fr fr Large number operations
    sus big1 meal = 1234567890.123456789
    sus big2 meal = 9876543210.987654321
    sus big_sum meal = big1 + big2
    vibez.spill(big_sum)
    
    fr fr Division precision test
    sus div_result meal = 1.0 / 3.0
    vibez.spill(div_result)
    
    fr fr Multiplication with precision loss potential
    sus mult_result meal = div_result * 3.0
    vibez.spill(mult_result)
    
    fr fr Float arithmetic edge cases
    sus edge_case1 meal = 0.999999999999999
    sus edge_case2 meal = 1.0 - edge_case1
    vibez.spill(edge_case2)
    
    fr fr Negative float precision
    sus neg_float meal = -123.456789
    sus abs_manual meal = 0.0 - neg_float
    vibez.spill(abs_manual)
    
    vibez.spill("=== Test Complete ===")
}
