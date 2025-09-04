vibe main
yeet "vibez"

fr fr Overflow Edge Cases Test
fr fr Tests: Integer overflow promotion and edge values
fr fr Expected: No crashes, proper type promotion

slay main_character() {
    vibez.spill("=== Overflow Edge Cases Test ===")
    
    fr fr Test maximum 32-bit integer
    sus max_int normie = 2147483647
    vibez.spill("Max int:")
    vibez.spill(max_int)
    
    fr fr Test overflow promotion to float
    sus overflow_result = max_int + 1
    vibez.spill("Max int + 1 (should promote to float):")
    vibez.spill(overflow_result)
    
    fr fr Test minimum 32-bit integer
    sus min_int normie = -2147483648
    vibez.spill("Min int:")
    vibez.spill(min_int)
    
    fr fr Test unary negation of min int (should promote)
    sus neg_min = -min_int
    vibez.spill("Negation of min int (should promote):")
    vibez.spill(neg_min)
    
    fr fr Test multiplication overflow
    sus big1 normie = 1000000
    sus big2 normie = 3000
    sus mult_overflow = big1 * big2
    vibez.spill("Large multiplication (should promote):")
    vibez.spill(mult_overflow)
    
    vibez.spill("=== Test Complete ===")
}
