// Test exhaustiveness checking for pattern matches
yeet "testz"

enum Status {
    Success,
    Error,
    Pending
}

slay test_exhaustive_patterns() lit {
    test_start("Exhaustive Patterns")
    
    // This should be exhaustive (covers all boolean values)
    sus flag lit = based
    ready (flag) {
        based => vibez.spill("True case")
        cringe => vibez.spill("False case")
        // No wildcard needed - exhaustive
    }
    
    // This should be exhaustive (has wildcard)
    sus x drip = 42
    ready (x) {
        0 => vibez.spill("Zero")
        1 => vibez.spill("One") 
        _ => vibez.spill("Other")
    }
    
    print_test_summary()
    damn based
}

slay test_non_exhaustive_patterns() lit {
    test_start("Non-Exhaustive Patterns")
    
    // This should warn about non-exhaustiveness
    sus flag lit = based
    ready (flag) {
        based => vibez.spill("True case")
        // Missing 'cringe' case - should warn
    }
    
    // This should warn about missing enum variants
    sus status = Status.Success
    ready (status) {
        Status.Success => vibez.spill("Success")
        Status.Error => vibez.spill("Error")
        // Missing Status.Pending - should warn
    }
    
    print_test_summary()
    damn based
}

slay test_exhaustive_with_ranges() lit {
    test_start("Exhaustive with Ranges")
    
    // Range patterns with exhaustive coverage
    sus x drip = 15
    ready (x) {
        0..10 => vibez.spill("Low range")
        11..20 => vibez.spill("Mid range") 
        _ => vibez.spill("High range")
    }
    
    print_test_summary()
    damn based
}

test_exhaustive_patterns()
test_non_exhaustive_patterns()
test_exhaustive_with_ranges()
