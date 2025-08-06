yeet "testz"

// Comprehensive Pattern Matching Test Suite for CURSED

test_start("Pattern Matching Comprehensive Test")

// Test 1: Literal Pattern Matching
slay test_literal_patterns() {
    vibez.spill("Testing literal patterns...")
    
    // Numbers
    sus value drip = 42
    vibe_check value {
        mood 42: {
            vibez.spill("✓ Integer literal pattern works")
        }
        basic: {
            vibez.spill("✗ Integer literal pattern failed")
        }
    }
    
    // Strings
    sus message tea = "hello"
    vibe_check message {
        mood "hello": {
            vibez.spill("✓ String literal pattern works")
        }
        basic: {
            vibez.spill("✗ String literal pattern failed")
        }
    }
    
    // Booleans
    sus flag lit = based
    vibe_check flag {
        mood based: {
            vibez.spill("✓ Boolean literal pattern works")
        }
        mood cringe: {
            vibez.spill("✗ Boolean literal pattern failed")
        }
        basic: {
            vibez.spill("✗ Boolean literal pattern unexpected")
        }
    }
}

// Test 2: Variable Binding Patterns
slay test_variable_binding() {
    vibez.spill("Testing variable binding patterns...")
    
    sus input drip = 123
    vibe_check input {
        mood x: {
            vibez.spill("✓ Variable binding works, captured value: ")
            vibez.spill_int(x)
            assert_eq_int(x, 123)
        }
        basic: {
            vibez.spill("✗ Variable binding failed")
        }
    }
}

// Test 3: Wildcard Patterns
slay test_wildcard_patterns() {
    vibez.spill("Testing wildcard patterns...")
    
    sus random drip = 999
    vibe_check random {
        mood 1: {
            vibez.spill("✗ Should not match 1")
        }
        mood 2: {
            vibez.spill("✗ Should not match 2")
        }
        basic: {
            vibez.spill("✓ Wildcard pattern catches all")
        }
    }
}

// Test 4: Tuple Destructuring Patterns
slay test_tuple_patterns() {
    vibez.spill("Testing tuple destructuring patterns...")
    
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
        }
    }
    
    // Nested tuples
    sus nested = ((1, 2), (3, 4))
    vibe_check nested {
        mood ((a, b), (c, d)): {
            vibez.spill("✓ Nested tuple destructuring works")
            assert_eq_int(a, 1)
            assert_eq_int(b, 2)
            assert_eq_int(c, 3)
            assert_eq_int(d, 4)
        }
        basic: {
            vibez.spill("✗ Nested tuple destructuring failed")
        }
    }
}

// Test 5: Struct Destructuring Patterns
squad Point {
    spill x drip
    spill y drip
}

slay test_struct_patterns() {
    vibez.spill("Testing struct destructuring patterns...")
    
    sus pt Point = Point{ x: 5, y: 15 }
    vibe_check pt {
        mood Point{ x: px, y: py }: {
            vibez.spill("✓ Struct destructuring works")
            assert_eq_int(px, 5)
            assert_eq_int(py, 15)
        }
        basic: {
            vibez.spill("✗ Struct destructuring failed")
        }
    }
    
    // Partial struct matching
    vibe_check pt {
        mood Point{ x: 5, y: _ }: {
            vibez.spill("✓ Partial struct matching works")
        }
        basic: {
            vibez.spill("✗ Partial struct matching failed")
        }
    }
}

// Test 6: Array Patterns with Rest Elements
slay test_array_patterns() {
    vibez.spill("Testing array patterns...")
    
    sus numbers = [1, 2, 3, 4, 5]
    vibe_check numbers {
        mood [first, second, ..rest]: {
            vibez.spill("✓ Array destructuring with rest works")
            assert_eq_int(first, 1)
            assert_eq_int(second, 2)
            vibez.spill("Rest elements: ")
            // Rest should contain [3, 4, 5]
        }
        basic: {
            vibez.spill("✗ Array destructuring failed")
        }
    }
    
    // Fixed-size array
    sus small = [10, 20]
    vibe_check small {
        mood [a, b]: {
            vibez.spill("✓ Fixed-size array matching works")
            assert_eq_int(a, 10)
            assert_eq_int(b, 20)
        }
        basic: {
            vibez.spill("✗ Fixed-size array matching failed")
        }
    }
}

// Test 7: OR Patterns (Multiple Alternatives)
slay test_or_patterns() {
    vibez.spill("Testing OR patterns...")
    
    sus value drip = 2
    vibe_check value {
        mood 1 | 2 | 3: {
            vibez.spill("✓ OR pattern matches correctly")
        }
        basic: {
            vibez.spill("✗ OR pattern failed")
        }
    }
    
    sus text tea = "world"
    vibe_check text {
        mood "hello" | "world" | "test": {
            vibez.spill("✓ String OR pattern works")
        }
        basic: {
            vibez.spill("✗ String OR pattern failed")
        }
    }
}

// Test 8: Range Patterns
slay test_range_patterns() {
    vibez.spill("Testing range patterns...")
    
    sus score drip = 85
    vibe_check score {
        mood 0..60: {
            vibez.spill("✗ Should not match low range")
        }
        mood 60..80: {
            vibez.spill("✗ Should not match medium range")
        }
        mood 80..100: {
            vibez.spill("✓ Range pattern matches correctly")
        }
        basic: {
            vibez.spill("✗ Range pattern failed")
        }
    }
    
    // Inclusive range
    sus boundary drip = 100
    vibe_check boundary {
        mood 80..=100: {
            vibez.spill("✓ Inclusive range pattern works")
        }
        basic: {
            vibez.spill("✗ Inclusive range pattern failed")
        }
    }
}

// Test 9: Guard Expressions
slay test_guard_patterns() {
    vibez.spill("Testing guard patterns...")
    
    sus numbers = [1, 2, 3, 4, 5, 6]
    
    bestie (num in numbers) {
        vibe_check num {
            mood x if x % 2 == 0: {
                vibez.spill("Even number: ")
                vibez.spill_int(x)
            }
            mood x if x > 4: {
                vibez.spill("Large odd number: ")
                vibez.spill_int(x)
            }
            basic: {
                vibez.spill("Small odd number: ")
                vibez.spill_int(num)
            }
        }
    }
}

// Test 10: Complex Nested Patterns
slay test_complex_patterns() {
    vibez.spill("Testing complex nested patterns...")
    
    sus data = {
        type: "user",
        info: {
            name: "Alice",
            age: 30,
            scores: [95, 87, 92]
        }
    }
    
    vibe_check data {
        mood { type: "user", info: { name: n, age: a, scores: [first, ..rest] } }: {
            vibez.spill("✓ Complex nested destructuring works")
            vibez.spill("Name: ")
            vibez.spill(n)
            vibez.spill(", Age: ")
            vibez.spill_int(a)
            vibez.spill(", First score: ")
            vibez.spill_int(first)
        }
        basic: {
            vibez.spill("✗ Complex nested destructuring failed")
        }
    }
}

// Test 11: Performance Test - Many Literal Cases
slay test_performance_many_cases() {
    vibez.spill("Testing performance with many literal cases...")
    
    sus input drip = 42
    vibe_check input {
        mood 1: vibez.spill("Case 1")
        mood 2: vibez.spill("Case 2")
        mood 3: vibez.spill("Case 3")
        mood 4: vibez.spill("Case 4")
        mood 5: vibez.spill("Case 5")
        mood 10: vibez.spill("Case 10")
        mood 15: vibez.spill("Case 15")
        mood 20: vibez.spill("Case 20")
        mood 25: vibez.spill("Case 25")
        mood 30: vibez.spill("Case 30")
        mood 35: vibez.spill("Case 35")
        mood 40: vibez.spill("Case 40")
        mood 42: {
            vibez.spill("✓ Found target case efficiently")
        }
        mood 45: vibez.spill("Case 45")
        mood 50: vibez.spill("Case 50")
        basic: {
            vibez.spill("✗ Should have matched case 42")
        }
    }
}

// Test 12: Exhaustiveness and Error Handling
slay test_exhaustiveness() {
    vibez.spill("Testing pattern exhaustiveness...")
    
    // This should generate a warning about non-exhaustive patterns
    sus option drip = 1
    vibe_check option {
        mood 1: vibez.spill("✓ Matched option 1")
        mood 2: vibez.spill("Matched option 2")
        // Missing default case - should warn about non-exhaustive patterns
    }
}

// Main test execution
slay main() drip {
    vibez.spill("=== CURSED Pattern Matching Test Suite ===")
    
    test_literal_patterns()
    test_variable_binding()
    test_wildcard_patterns()
    test_tuple_patterns()
    test_struct_patterns()
    test_array_patterns()
    test_or_patterns()
    test_range_patterns()
    test_guard_patterns()
    test_complex_patterns()
    test_performance_many_cases()
    test_exhaustiveness()
    
    vibez.spill("=== Pattern Matching Tests Complete ===")
    print_test_summary()
    
    damn 0
}
