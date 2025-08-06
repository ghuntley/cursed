yeet "vibez"

slay main() drip {
    vibez.spill("=== Advanced Pattern Matching Test ===")
    
    // Test 1: Literal patterns with different values
    sus value drip = 42
    vibe_check value {
        mood 10: vibez.spill("Matched 10")
        mood 42: vibez.spill("✓ Matched 42 correctly")
        mood 100: vibez.spill("Matched 100")
        basic: vibez.spill("No match")
    }
    
    // Test 2: String literal patterns
    sus message tea = "hello"
    vibe_check message {
        mood "goodbye": vibez.spill("Matched goodbye")
        mood "hello": vibez.spill("✓ Matched hello correctly")
        mood "world": vibez.spill("Matched world")
        basic: vibez.spill("No string match")
    }
    
    // Test 3: Boolean patterns
    sus flag lit = based
    vibe_check flag {
        mood cringe: vibez.spill("Matched false")
        mood based: vibez.spill("✓ Matched true correctly")
        basic: vibez.spill("No boolean match")
    }
    
    // Test 4: Multiple cases efficiency test
    sus num drip = 7
    vibe_check num {
        mood 1: vibez.spill("One")
        mood 2: vibez.spill("Two")
        mood 3: vibez.spill("Three")
        mood 4: vibez.spill("Four")
        mood 5: vibez.spill("Five")
        mood 6: vibez.spill("Six")
        mood 7: vibez.spill("✓ Seven - efficient dispatch")
        mood 8: vibez.spill("Eight")
        mood 9: vibez.spill("Nine")
        mood 10: vibez.spill("Ten")
        basic: vibez.spill("Other number")
    }
    
    vibez.spill("=== Pattern Matching Test Complete ===")
    damn 0
}
