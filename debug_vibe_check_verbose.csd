// Test vibe_check with explicit debug output

yeet "vibez"

slay main() {
    vibez.spill("=== CURSED Pattern Matching Test ===")
    
    sus test_value drip = 42
    vibez.spill("Test value:", test_value)
    
    vibez.spill("Starting vibe_check...")
    
    vibe_check test_value {
        mood 1:
            vibez.spill("✅ Matched case 1")
        mood 42:
            vibez.spill("✅ Matched case 42 - CORRECT!")
        mood 100:
            vibez.spill("✅ Matched case 100")
        basic:
            vibez.spill("❌ Hit default case - WRONG!")
    }
    
    vibez.spill("Pattern matching completed")
    
    // Test string patterns
    sus text_value tea = "hello"
    vibez.spill("String test value:", text_value)
    
    vibe_check text_value {
        mood "hello":
            vibez.spill("✅ Matched string 'hello' - CORRECT!")
        mood "world":
            vibez.spill("✅ Matched string 'world'")  
        basic:
            vibez.spill("❌ Hit string default - WRONG!")
    }
    
    vibez.spill("=== Test Complete ===")
}
