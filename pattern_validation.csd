yeet "vibez"

slay main() drip {
    vibez.spill("=== Pattern Matching Validation ===")
    
    // Simple literal pattern test
    sus x drip = 42
    vibe_check x {
        mood 42: {
            vibez.spill("✓ Literal pattern 42 matched")
        }
        basic: {
            vibez.spill("✗ Should have matched 42")
        }
    }
    
    // String pattern test
    sus name tea = "Alice"
    vibe_check name {
        mood "Alice": {
            vibez.spill("✓ String pattern Alice matched")
        }
        basic: {
            vibez.spill("✗ Should have matched Alice")
        }
    }
    
    // Boolean pattern test
    sus active lit = based
    vibe_check active {
        mood based: {
            vibez.spill("✓ Boolean pattern true matched")
        }
        mood cringe: {
            vibez.spill("✗ Should not match false")
        }
        basic: {
            vibez.spill("✗ Unexpected boolean case")
        }
    }
    
    // Multiple case efficiency test (should generate jump table)
    sus choice drip = 5
    vibe_check choice {
        mood 1: vibez.spill("Case 1")
        mood 2: vibez.spill("Case 2")
        mood 3: vibez.spill("Case 3")
        mood 4: vibez.spill("Case 4")
        mood 5: vibez.spill("✓ Case 5 - efficient dispatch")
        mood 6: vibez.spill("Case 6")
        mood 7: vibez.spill("Case 7")
        mood 8: vibez.spill("Case 8")
        mood 9: vibez.spill("Case 9")
        mood 10: vibez.spill("Case 10")
        basic: vibez.spill("Default case")
    }
    
    vibez.spill("=== Validation Complete ===")
    damn 0
}
