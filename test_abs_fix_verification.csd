vibe main
yeet "vibez"
yeet "mathz"

slay main_character() {
    vibez.spill("=== Testing mathz.abs_normie fix ===")
    
    // Test normal case
    sus negative drip = -42
    sus normal_result drip = mathz.abs_normie(negative)
    vibez.spill("Normal case: -42")
    vibez.spill("Result:")
    vibez.spill(normal_result)
    
    // Test the problematic case - use a smaller test value
    sus large_negative drip = -999999999
    sus large_result drip = mathz.abs_normie(large_negative)
    vibez.spill("Large negative: -999999999")
    vibez.spill("Result:")
    vibez.spill(large_result)
    
    // Test a smaller minimum for 32-bit case
    sus min_32 drip = -2147483648
    sus min_32_result drip = mathz.abs_normie(min_32)
    vibez.spill("Min 32-bit case: -2147483648") 
    vibez.spill("Result:")
    vibez.spill(min_32_result)
    
    vibez.spill("=== Test completed ===")
}
