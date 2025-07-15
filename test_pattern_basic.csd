yeet "testz"

slay test_pattern_basic() {
    test_start("basic pattern matching tests")
    
    // Test basic literal patterns
    sus value = 42
    lowkey value vibes 42 {
        assert_true(based)
        vibez.spill("Literal pattern matching works!")
    } else {
        assert_true(cap)
        vibez.spill("Literal pattern matching failed!")
    }
    
    // Test variable pattern  
    sus value2 = "hello"
    lowkey value2 vibes x {
        assert_eq_string(x, "hello")
        vibez.spill("Variable pattern matching works!")
    }
    
    // Test wildcard pattern
    sus value3 = 123
    lowkey value3 vibes _ {
        assert_true(based)
        vibez.spill("Wildcard pattern matching works!")
    }
    
    // Test boolean patterns
    sus flag = based
    lowkey flag vibes based {
        assert_true(based)
        vibez.spill("Boolean pattern matching works!")
    }
    
    print_test_summary()
}

test_pattern_basic()
