vibe main
yeet "vibez"

// Direct call to mathz_abs_normie runtime function
// bypassing the mathz module compilation issue

extern slay mathz_abs_normie(value normie) normie

slay main_character() {
    vibez.spill("Testing direct mathz_abs_normie call")
    sus negative_value drip = -42
    sus result drip = mathz_abs_normie(negative_value)
    vibez.spill("Input: -42")
    vibez.spill("Result:")
    vibez.spill(result)
    
    // Test the problematic case that produces garbage
    sus min_int drip = -2147483648
    sus garbage_result drip = mathz_abs_normie(min_int)
    vibez.spill("Min int input: -2147483648") 
    vibez.spill("Garbage result:")
    vibez.spill(garbage_result)
}
