// Test basic pattern matching works
yeet "testz"

slay test_basic_patterns() lit {
    test_start("Basic Patterns")
    
    // Test wildcard pattern
    sus x drip = 42
    ready (x) {
        _ => vibez.spill("Wildcard matched")
    }
    
    // Test literal patterns
    sus flag lit = based
    ready (flag) {
        based => vibez.spill("True matched")
        cringe => vibez.spill("False matched")
        _ => vibez.spill("Other")
    }
    
    // Test variable pattern
    sus y drip = 100
    ready (y) {
        value => vibez.spill("Variable pattern captured value")
    }
    
    print_test_summary()
    damn based
}

test_basic_patterns()
