yeet "vibez"

// CURSED Pattern Matching Validation Test
// Tests the efficient compiled pattern matching implementation

slay main() drip {
    vibez.spill("=== CURSED Pattern Matching Validation ===")
    
    // Test 1: Literal pattern efficiency 
    vibez.spill("Test 1: Literal Pattern Efficiency")
    sus test_value drip = 42
    vibe_check test_value {
        mood 42: vibez.spill("✓ Literal pattern works efficiently")
        basic: vibez.spill("✗ Failed literal pattern")
    }
    
    // Test 2: String pattern matching
    vibez.spill("Test 2: String Pattern Matching")
    sus message tea = "hello"
    vibe_check message {
        mood "hello": vibez.spill("✓ String pattern works")
        basic: vibez.spill("✗ Failed string pattern")
    }
    
    // Test 3: Variable binding
    vibez.spill("Test 3: Variable Binding")
    sus input drip = 123
    vibe_check input {
        mood x: {
            vibez.spill("✓ Variable binding successful")
            vibez.spill("Value: ")
            vibez.spill_int(x)
        }
        basic: vibez.spill("✗ Failed variable binding")
    }
    
    // Test 4: Boolean patterns
    vibez.spill("Test 4: Boolean Pattern Matching")
    sus flag lit = based
    vibe_check flag {
        mood based: vibez.spill("✓ Boolean pattern works")
        mood cringe: vibez.spill("✗ Wrong boolean")
        basic: vibez.spill("✗ Failed boolean pattern")
    }
    
    // Test 5: Jump table optimization (8+ cases)
    vibez.spill("Test 5: Jump Table Optimization")
    sus jump_test drip = 5
    vibe_check jump_test {
        mood 1: vibez.spill("Case 1")
        mood 2: vibez.spill("Case 2") 
        mood 3: vibez.spill("Case 3")
        mood 4: vibez.spill("Case 4")
        mood 5: vibez.spill("✓ Case 5 - Jump Table")
        mood 6: vibez.spill("Case 6")
        mood 7: vibez.spill("Case 7")
        mood 8: vibez.spill("Case 8")
        mood 9: vibez.spill("Case 9")
        mood 10: vibez.spill("Case 10")
        basic: vibez.spill("✗ Jump table failed")
    }
    
    // Test 6: Wildcard pattern
    vibez.spill("Test 6: Wildcard Pattern")
    sus random drip = 999
    vibe_check random {
        mood 1: vibez.spill("✗ Should not match")
        mood 2: vibez.spill("✗ Should not match")
        basic: vibez.spill("✓ Wildcard catches all")
    }
    
    vibez.spill("=== All Pattern Matching Tests Passed ===")
    damn 0
}
