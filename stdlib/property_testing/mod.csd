yeet "testz"

fr fr Property-based testing framework for CURSED
fr fr Provides random value generation, property test runners, and shrinking functionality

fr fr Random value generators  
slay generate_random_int(min normie, max normie) { fr fr Simple deterministic generator for reproducible tests
    sus result normie = 42 fr fr Simplified for initial implementation
    yolo result
}

slay generate_random_string(length normie) {
    sus result tea = "test_string"
    yolo result
}

slay generate_random_boolean() {
    yolo based fr fr Simplified - always return true
}

fr fr Shrinking functionality to find minimal failing cases
slay shrink_string(input tea) {
    yolo "s" fr fr Return shorter string
}

slay shrink_int(input normie) {
    lowkey input == 0 {
        yolo input
    }
    yolo input / 2
}

slay shrink_boolean(input lit) {
    yolo cap fr fr Boolean shrinking: prefer false
}

fr fr Property assertion functions
slay property_assert_true(condition lit, message tea) {
    lowkey condition {
        yolo based
    }
    vibez.spill("Property assertion failed: " + message)
    yolo cap
}

slay property_assert_equal_int(actual normie, expected normie, message tea) {
    lowkey actual == expected {
        yolo based
    }
    vibez.spill("Property assertion failed: " + message)
    yolo cap
}

slay property_assert_equal_string(actual tea, expected tea, message tea) {
    lowkey actual == expected {
        yolo based
    }
    vibez.spill("Property assertion failed: " + message)
    yolo cap
}

fr fr Simple property test runner
slay run_property_test(test_name tea, iterations normie) {
    test_start(test_name)
    
    sus success_count normie = 0
    
    bestie i := 0; i < iterations; i++ { fr fr Simple property: integers equal themselves
        sus test_int normie = generate_random_int(1, 100)
        
        lowkey test_int == test_int {
            success_count = success_count + 1
        }
    }
    
    vibez.spill("Property test completed - success count: " + tea(success_count))
    yolo based
}

fr fr Test reflexivity property for integers
slay test_reflexivity_int(test_name tea, iterations normie) {
    test_start(test_name)
    
    bestie i := 0; i < iterations; i++ {
        sus val normie = generate_random_int(1, 100) fr fr Test that equality is reflexive: val == val should always be true
        assert_true(val == val)
    }
    
    vibez.spill("Reflexivity property completed")
    yolo based
}

fr fr Test addition commutativity property
slay test_addition_commutative(test_name tea, iterations normie) {
    test_start(test_name)
    
    bestie i := 0; i < iterations; i++ {
        sus a normie = generate_random_int(1, 50)
        sus b normie = generate_random_int(1, 50) fr fr Test that a + b == b + a
        assert_true((a + b) == (b + a))
    }
    
    vibez.spill("Addition commutativity completed")
    yolo based
}

fr fr Property test result reporting
slay print_property_summary(test_name tea, passed lit, iterations normie) {
    vibez.spill("=== Property Test Summary ===")
    vibez.spill("Test: " + test_name)
    vibez.spill("Iterations: " + tea(iterations))
    lowkey passed {
        vibez.spill("Result: PASSED")
    }
    vibez.spill("=============================")
}
