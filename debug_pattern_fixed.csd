// Improved pattern matching test

yeet "vibez"

slay main() {
    vibez.spill("=== Testing CURSED Pattern Matching ===")
    
    sus number_value drip = 42
    vibez.spill("Testing value:", number_value)
    
    vibe_check number_value {
        mood 1:
            vibez.spill("Case 1 - Should NOT execute")
        mood 42:
            vibez.spill("Case 42 - Should execute - CORRECT!")
        mood 100:
            vibez.spill("Case 100 - Should NOT execute")
        basic:
            vibez.spill("Default case - Should NOT execute")
    }
    
    vibez.spill("=== Test Complete ===")
}
