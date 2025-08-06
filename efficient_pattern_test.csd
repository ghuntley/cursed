yeet "vibez"

slay main() drip {
    vibez.spill("=== Efficient Pattern Matching Test ===")
    
    // Test case: Should only execute the matching branch
    sus target drip = 7
    
    vibez.spill("Testing value: 7")
    vibe_check target {
        mood 7: {
            vibez.spill("✓ CORRECT: Found 7")
            damn 0  // Exit immediately if correct
        }
        basic: {
            vibez.spill("✗ WRONG: Should not reach default")
            damn 1  // Exit with error if we reach here
        }
    }
    
    vibez.spill("✗ WRONG: Should not reach end")
    damn 1
}
