// Test range patterns implementation
yeet "testz"

slay test_range_patterns() lit {
    test_start("Range Patterns")
    
    // Test integer ranges
    sus x drip = 5
    ready (x) {
        0..10 => vibez.spill("In range 0-10 (inclusive)")
        11...20 => vibez.spill("In range 11-19 (exclusive)")
        _ => vibez.spill("Out of range")
    }
    
    // Test character ranges  
    sus ch tea = "m"
    ready (ch) {
        "a".."z" => vibez.spill("Lowercase letter")
        "A".."Z" => vibez.spill("Uppercase letter") 
        "0".."9" => vibez.spill("Digit")
        _ => vibez.spill("Other character")
    }
    
    // Test ranges with variables
    sus min drip = 0
    sus max drip = 100
    sus value drip = 42
    
    ready (value) {
        min..max => vibez.spill("Within dynamic range")
        _ => vibez.spill("Outside dynamic range")
    }
    
    print_test_summary()
    damn based
}

test_range_patterns()
