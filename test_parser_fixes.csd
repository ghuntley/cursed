// Test source location tracking and pattern matching fixes
yeet "testz"

// Test function with complex patterns
slay test_pattern_matching(value normie) lit {
    vibe_check value {
        1 -> {
            vibez.spill("Value is 1")
            damn based
        }
        2 | 3 -> {
            vibez.spill("Value is 2 or 3")
            damn based
        }
        _ -> {
            vibez.spill("Value is something else")
            damn cringe
        }
    }
}

// Test channel type parsing
slay test_channel_creation() {
    sus ch dm<normie> = make(dm<normie>, 10)
    sus ch2 dm<tea> = make(dm<tea>)
    vibez.spill("Channel creation test complete")
}

// Test interface with source location tracking
collab TestInterface {
    slay test_method(param normie) tea
    slay another_method() lit
}

// Main test function
slay main() {
    test_start("Parser fixes test")
    
    // Test pattern matching
    assert_true(test_pattern_matching(1))
    assert_true(test_pattern_matching(2))
    assert_false(test_pattern_matching(5))
    
    // Test channel creation
    test_channel_creation()
    
    vibez.spill("All parser fixes working correctly!")
    print_test_summary()
}
