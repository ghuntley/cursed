yeet "testz"

// Pattern Matching LLVM Codegen Test Suite for CURSED

test_start("LLVM Pattern Matching Test")

// Test 1: Basic literal pattern matching with LLVM optimization
slay test_literal_pattern_llvm() {
    vibez.spill("Testing LLVM literal pattern optimization...")
    
    sus value drip = 42
    
    // This should generate an optimized LLVM switch instruction
    vibe_check value {
        mood 10: {
            vibez.spill("Case 10")
        }
        mood 20: {
            vibez.spill("Case 20")
        }
        mood 30: {
            vibez.spill("Case 30")
        }
        mood 42: {
            vibez.spill("✓ Found target case 42")
            assert_true(based)
        }
        mood 50: {
            vibez.spill("Case 50")
        }
        basic: {
            vibez.spill("✗ Should not reach default")
            assert_false(based)
        }
    }
}

// Test 2: Jump table optimization threshold
slay test_jump_table_optimization() {
    vibez.spill("Testing jump table optimization...")
    
    sus test_val drip = 7
    
    // This should trigger jump table optimization (>=8 literal cases)
    vibe_check test_val {
        mood 1: vibez.spill("One")
        mood 2: vibez.spill("Two")
        mood 3: vibez.spill("Three")
        mood 4: vibez.spill("Four")
        mood 5: vibez.spill("Five")
        mood 6: vibez.spill("Six")
        mood 7: {
            vibez.spill("✓ Seven - jump table dispatch")
            assert_true(based)
        }
        mood 8: vibez.spill("Eight")
        mood 9: vibez.spill("Nine")
        mood 10: vibez.spill("Ten")
        basic: {
            vibez.spill("✗ Should not reach default")
            assert_false(based)
        }
    }
}

// Test 3: String pattern matching with LLVM optimization
slay test_string_pattern_llvm() {
    vibez.spill("Testing string pattern matching...")
    
    sus message tea = "hello"
    
    vibe_check message {
        mood "goodbye": {
            vibez.spill("✗ Wrong string")
            assert_false(based)
        }
        mood "hello": {
            vibez.spill("✓ Correct string match")
            assert_true(based)
        }
        mood "world": {
            vibez.spill("✗ Wrong string")
            assert_false(based)
        }
        basic: {
            vibez.spill("✗ Should not reach default")
            assert_false(based)
        }
    }
}

// Test 4: Variable binding pattern
slay test_variable_binding_pattern() {
    vibez.spill("Testing variable binding patterns...")
    
    sus input drip = 123
    
    vibe_check input {
        mood x: {
            vibez.spill("✓ Variable binding successful")
            vibez.spill("Captured value: ")
            vibez.spill_int(x)
            assert_eq_int(x, 123)
        }
        basic: {
            vibez.spill("✗ Should not reach default")
            assert_false(based)
        }
    }
}

// Test 5: Wildcard pattern
slay test_wildcard_pattern() {
    vibez.spill("Testing wildcard patterns...")
    
    sus random drip = 999
    
    vibe_check random {
        mood 1: {
            vibez.spill("✗ Should not match 1")
            assert_false(based)
        }
        mood 2: {
            vibez.spill("✗ Should not match 2")
            assert_false(based)
        }
        basic: {
            vibez.spill("✓ Wildcard catches all")
            assert_true(based)
        }
    }
}

// Test 6: Complex pattern with sequential matching
slay test_complex_sequential_pattern() {
    vibez.spill("Testing complex sequential patterns...")
    
    sus data = {
        type: "user",
        value: 42
    }
    
    // This should use sequential pattern matching (not jump table)
    vibe_check data {
        mood { type: "admin", value: _ }: {
            vibez.spill("✗ Should not match admin")
            assert_false(based)
        }
        mood { type: "user", value: x }: {
            vibez.spill("✓ Complex pattern matched")
            vibez.spill("User value: ")
            vibez.spill_int(x)
            assert_eq_int(x, 42)
        }
        basic: {
            vibez.spill("✗ Should not reach default")
            assert_false(based)
        }
    }
}

// Test 7: Match expression (functional style)
slay test_match_expression() {
    vibez.spill("Testing match expressions...")
    
    sus input drip = 5
    
    sus result tea = match input {
        1 => "one",
        2 => "two", 
        3 => "three",
        5 => "five",
        _ => "other"
    }
    
    vibez.spill("Match result: ")
    vibez.spill(result)
    assert_eq_string(result, "five")
}

// Test 8: Tuple destructuring with LLVM optimization
slay test_tuple_destructuring() {
    vibez.spill("Testing tuple destructuring...")
    
    sus point = (10, 20)
    
    vibe_check point {
        mood (x, y): {
            vibez.spill("✓ Tuple destructuring works")
            vibez.spill("x = ")
            vibez.spill_int(x)
            vibez.spill(", y = ")
            vibez.spill_int(y)
            assert_eq_int(x, 10)
            assert_eq_int(y, 20)
        }
        basic: {
            vibez.spill("✗ Tuple destructuring failed")
            assert_false(based)
        }
    }
}

// Test 9: Array pattern matching
slay test_array_pattern() {
    vibez.spill("Testing array patterns...")
    
    sus numbers = [1, 2, 3]
    
    vibe_check numbers {
        mood [first, second, third]: {
            vibez.spill("✓ Array destructuring works")
            assert_eq_int(first, 1)
            assert_eq_int(second, 2)
            assert_eq_int(third, 3)
        }
        basic: {
            vibez.spill("✗ Array destructuring failed")
            assert_false(based)
        }
    }
}

// Test 10: Boolean pattern matching
slay test_boolean_pattern() {
    vibez.spill("Testing boolean patterns...")
    
    sus flag lit = based
    
    vibe_check flag {
        mood cringe: {
            vibez.spill("✗ Should not match false")
            assert_false(based)
        }
        mood based: {
            vibez.spill("✓ Boolean pattern works")
            assert_true(based)
        }
        basic: {
            vibez.spill("✗ Should not reach default")
            assert_false(based)
        }
    }
}

// Test 11: Performance benchmark for pattern matching
slay test_pattern_matching_performance() {
    vibez.spill("Testing pattern matching performance...")
    
    sus iterations drip = 1000
    sus counter drip = 0
    
    bestie (counter < iterations) {
        sus test_value drip = counter % 10
        
        vibe_check test_value {
            mood 0: counter = counter + 1
            mood 1: counter = counter + 1
            mood 2: counter = counter + 1
            mood 3: counter = counter + 1
            mood 4: counter = counter + 1
            mood 5: counter = counter + 1
            mood 6: counter = counter + 1
            mood 7: counter = counter + 1
            mood 8: counter = counter + 1
            mood 9: counter = counter + 1
            basic: counter = counter + 1
        }
    }
    
    vibez.spill("✓ Performance test completed")
    assert_eq_int(counter, iterations)
}

// Test 12: Exhaustiveness checking
slay test_exhaustiveness_warning() {
    vibez.spill("Testing exhaustiveness checking...")
    
    sus option drip = 1
    
    // This should generate a warning about non-exhaustive patterns
    vibe_check option {
        mood 1: vibez.spill("✓ Matched option 1")
        mood 2: vibez.spill("Matched option 2")
        // Missing basic case - should warn about non-exhaustive patterns
    }
}

// Main test execution
slay main() drip {
    vibez.spill("=== CURSED Pattern Matching LLVM Test Suite ===")
    
    test_literal_pattern_llvm()
    test_jump_table_optimization()
    test_string_pattern_llvm()
    test_variable_binding_pattern()
    test_wildcard_pattern()
    test_complex_sequential_pattern()
    test_match_expression()
    test_tuple_destructuring()
    test_array_pattern()
    test_boolean_pattern()
    test_pattern_matching_performance()
    test_exhaustiveness_warning()
    
    vibez.spill("=== Pattern Matching LLVM Tests Complete ===")
    print_test_summary()
    
    damn 0
}
