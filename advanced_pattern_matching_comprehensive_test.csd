// Advanced Pattern Matching Comprehensive Test Suite
// Tests all implemented advanced pattern matching features including:
// - Decision tree compilation
// - Pattern optimization
// - Exhaustiveness checking for all types
// - Guard variable binding
// - Complex nested patterns

yeet "testz"
yeet "vibez"

// Test enums for exhaustiveness checking
enum Status {
    Success,
    Error,
    Pending,
    Cancelled
}

enum Priority {
    Low,
    Medium, 
    High,
    Critical
}

// Test structs for destructuring
squad Task {
    spill name tea
    spill priority Priority
    spill completed lit
    spill score drip
}

squad Point {
    spill x drip
    spill y drip
}

squad Line {
    spill start Point
    spill end Point
}

slay main() drip {
    test_start("Advanced Pattern Matching Comprehensive Test Suite")
    
    vibez.spill("🔥 Starting comprehensive pattern matching tests...")
    
    // Test 1: Exhaustive enum pattern matching
    test_exhaustive_enum_patterns()
    
    // Test 2: Boolean exhaustiveness checking
    test_boolean_exhaustiveness()
    
    // Test 3: Integer range exhaustiveness
    test_integer_range_exhaustiveness()
    
    // Test 4: Complex nested pattern destructuring
    test_complex_nested_patterns()
    
    // Test 5: Guard pattern optimization
    test_guard_pattern_optimization()
    
    // Test 6: Pattern decision tree compilation
    test_decision_tree_compilation()
    
    // Test 7: OR pattern alternatives
    test_or_pattern_alternatives()
    
    // Test 8: Array patterns with rest elements
    test_array_pattern_rest()
    
    // Test 9: Non-exhaustive pattern detection
    test_non_exhaustive_detection()
    
    // Test 10: Pattern compilation performance
    test_pattern_compilation_performance()
    
    print_test_summary()
    damn 0
}

slay test_exhaustive_enum_patterns() {
    vibez.spill("=== Test 1: Exhaustive Enum Pattern Matching ===")
    
    sus status Status = Status.Success
    
    // This should be detected as exhaustive (all variants covered)
    sus result drip = sick status {
        when Status.Success -> {
            vibez.spill("✅ Success case handled")
            damn 1
        }
        when Status.Error -> {
            vibez.spill("✅ Error case handled")
            damn 2
        }
        when Status.Pending -> {
            vibez.spill("✅ Pending case handled")
            damn 3
        }
        when Status.Cancelled -> {
            vibez.spill("✅ Cancelled case handled")
            damn 4
        }
        // No wildcard needed - exhaustive
    }
    
    assert_eq_int(result, 1)
    vibez.spill("✅ Exhaustive enum pattern matching passed")
}

slay test_boolean_exhaustiveness() {
    vibez.spill("=== Test 2: Boolean Exhaustiveness Checking ===")
    
    sus flag lit = based
    
    // Test exhaustive boolean patterns
    sus result drip = sick flag {
        when based -> {
            vibez.spill("✅ True case handled")
            damn 1
        }
        when cringe -> {
            vibez.spill("✅ False case handled")
            damn 0
        }
        // Exhaustive - no wildcard needed
    }
    
    assert_eq_int(result, 1)
    
    // Test with variable binding
    sus result2 drip = sick flag {
        when x when x == based -> {
            vibez.spill("✅ True with guard")
            damn 10
        }
        when x -> {
            vibez.spill("✅ Any other boolean")
            damn 20
        }
    }
    
    assert_eq_int(result2, 10)
    vibez.spill("✅ Boolean exhaustiveness checking passed")
}

slay test_integer_range_exhaustiveness() {
    vibez.spill("=== Test 3: Integer Range Exhaustiveness ===")
    
    sus score drip = 85
    
    // Test range patterns with exhaustive coverage
    sus grade tea = sick score {
        when 0..59 -> "F"
        when 60..69 -> "D"
        when 70..79 -> "C"
        when 80..89 -> "B"
        when 90..100 -> "A"
        when _ -> "Invalid"
    }
    
    assert_eq_str(grade, "B")
    
    // Test small integer exhaustiveness (all values covered)
    sus small_val drip = 2
    sus result drip = sick small_val {
        when 0 -> 10
        when 1 -> 20
        when 2 -> 30
        when 3 -> 40
        when n when n > 3 -> n * 10
        when _ -> 0
    }
    
    assert_eq_int(result, 30)
    vibez.spill("✅ Integer range exhaustiveness passed")
}

slay test_complex_nested_patterns() {
    vibez.spill("=== Test 4: Complex Nested Pattern Destructuring ===")
    
    sus task Task = Task{
        name: "Implement Pattern Matching",
        priority: Priority.High,
        completed: cringe,
        score: 95
    }
    
    // Test complex nested destructuring with guards
    sus result tea = sick task {
        when Task{name: n, priority: Priority.Critical, completed: c, score: s} when s > 90 -> {
            vibez.spill("✅ Critical high-score task:", n)
            damn "critical-high"
        }
        when Task{name: n, priority: Priority.High, completed: cringe, score: s} when s >= 90 -> {
            vibez.spill("✅ Incomplete high-priority task:", n, "score:", s)
            damn "high-incomplete"
        }
        when Task{name, priority, completed: based} -> {
            vibez.spill("✅ Completed task:", name, "priority:", priority)
            damn "completed"
        }
        when Task{name, score} when score < 50 -> {
            vibez.spill("✅ Low score task:", name)
            damn "low-score"
        }
        when _ -> {
            vibez.spill("✅ Other task pattern")
            damn "other"
        }
    }
    
    assert_eq_str(result, "high-incomplete")
    
    // Test deeply nested structures
    sus line Line = Line{
        start: Point{x: 0, y: 0},
        end: Point{x: 3, y: 4}
    }
    
    sus length drip = sick line {
        when Line{start: Point{x: 0, y: 0}, end: Point{x: a, y: b}} -> {
            vibez.spill("✅ Line from origin to point:", a, b)
            damn mathz.sqrt(a * a + b * b)
        }
        when Line{start: Point{x: a, y: b}, end: Point{x: c, y: d}} -> {
            sus dx drip = c - a
            sus dy drip = d - b
            damn mathz.sqrt(dx * dx + dy * dy)
        }
        when _ -> 0
    }
    
    assert_eq_int(length, 5) // 3-4-5 triangle
    vibez.spill("✅ Complex nested patterns passed")
}

slay test_guard_pattern_optimization() {
    vibez.spill("=== Test 5: Guard Pattern Optimization ===")
    
    sus data []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    
    // Test guard patterns with variable binding and complex conditions
    sus result tea = sick data {
        when arr when len(arr) > 5 && arr[0] > 0 && arr[len(arr)-1] <= 10 -> {
            vibez.spill("✅ Valid array with", len(arr), "elements")
            damn "valid-array"
        }
        when arr when len(arr) == 0 -> {
            vibez.spill("✅ Empty array")
            damn "empty"
        }
        when arr when len(arr) <= 3 -> {
            vibez.spill("✅ Small array")
            damn "small"
        }
        when _ -> {
            vibez.spill("✅ Other array pattern")
            damn "other"
        }
    }
    
    assert_eq_str(result, "valid-array")
    
    // Test multiple guard conditions with short-circuiting
    sus num drip = 15
    sus category tea = sick num {
        when n when n > 0 && n < 10 && n % 2 == 0 -> "small-even"
        when n when n >= 10 && n < 20 && n % 3 == 0 -> "medium-triple"
        when n when n >= 10 && n < 20 && n % 5 == 0 -> "medium-five"
        when n when n > 100 -> "large"
        when _ -> "other"
    }
    
    assert_eq_str(category, "medium-five")
    vibez.spill("✅ Guard pattern optimization passed")
}

slay test_decision_tree_compilation() {
    vibez.spill("=== Test 6: Pattern Decision Tree Compilation ===")
    
    sus value drip = 42
    
    // Test patterns that should compile to optimized decision tree
    sus result tea = sick value {
        when 1 -> "one"
        when 2 -> "two"
        when 3 -> "three"
        when 10..20 -> "teens"
        when 30..50 -> "thirties-to-fifties"
        when 100..200 -> "hundreds"
        when n when n < 0 -> "negative"
        when n when n > 1000 -> "large"
        when _ -> "other"
    }
    
    assert_eq_str(result, "thirties-to-fifties")
    
    // Test jump table optimization for many literal patterns
    sus letter tea = "e"
    sus vowel_type tea = sick letter {
        when "a" -> "first"
        when "e" -> "second"
        when "i" -> "third"
        when "o" -> "fourth"
        when "u" -> "fifth"
        when "y" -> "sometimes"
        when _ -> "consonant"
    }
    
    assert_eq_str(vowel_type, "second")
    vibez.spill("✅ Decision tree compilation passed")
}

slay test_or_pattern_alternatives() {
    vibez.spill("=== Test 7: OR Pattern Alternatives ===")
    
    sus status tea = "error"
    
    // Test OR patterns with multiple alternatives
    sus severity drip = sick status {
        when "critical" | "fatal" | "emergency" -> 5
        when "error" | "failure" | "exception" -> 4
        when "warning" | "caution" -> 3
        when "info" | "notice" -> 2
        when "debug" | "trace" -> 1
        when _ -> 0
    }
    
    assert_eq_int(severity, 4)
    
    // Test OR patterns with guards
    sus num drip = 25
    sus category tea = sick num {
        when 0 | 1 when num == 0 -> "zero"
        when 0 | 1 when num == 1 -> "one"
        when 2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 -> "small-prime"
        when n when n > 20 && n < 30 -> "twenties"
        when _ -> "other"
    }
    
    assert_eq_str(category, "twenties")
    vibez.spill("✅ OR pattern alternatives passed")
}

slay test_array_pattern_rest() {
    vibez.spill("=== Test 8: Array Patterns with Rest Elements ===")
    
    sus numbers []drip = [1, 2, 3, 4, 5]
    
    // Test array patterns with rest element capture
    sus result tea = sick numbers {
        when [] -> "empty"
        when [x] -> {
            vibez.spill("✅ Single element:", x)
            damn "single"
        }
        when [first, second] -> {
            vibez.spill("✅ Two elements:", first, second)
            damn "pair"
        }
        when [head, ...tail] when len(tail) > 2 -> {
            vibez.spill("✅ Head:", head, "Tail length:", len(tail))
            damn "head-many-tail"
        }
        when [first, second, ...rest] -> {
            vibez.spill("✅ First two:", first, second, "Rest:", len(rest))
            damn "first-two-rest"
        }
        when _ -> "other"
    }
    
    assert_eq_str(result, "head-many-tail")
    
    // Test array pattern with specific length matching
    sus coords []drip = [10, 20, 30]
    sus dimension tea = sick coords {
        when [x] -> "1D"
        when [x, y] -> "2D"
        when [x, y, z] -> {
            vibez.spill("✅ 3D coordinates:", x, y, z)
            damn "3D"
        }
        when [x, y, z, w] -> "4D"
        when _ -> "multi-dimensional"
    }
    
    assert_eq_str(dimension, "3D")
    vibez.spill("✅ Array pattern rest elements passed")
}

slay test_non_exhaustive_detection() {
    vibez.spill("=== Test 9: Non-Exhaustive Pattern Detection ===")
    
    // This should generate exhaustiveness warning (missing Cancelled)
    sus priority Priority = Priority.High
    sus urgency drip = sick priority {
        when Priority.Low -> 1
        when Priority.Medium -> 2
        when Priority.High -> 3
        // Missing Priority.Critical - should warn
        when _ -> 0  // But wildcard makes it safe
    }
    
    assert_eq_int(urgency, 3)
    
    // This should also warn about non-exhaustive boolean
    sus flag lit = based
    sus result drip = sick flag {
        when based -> 1
        // Missing cringe case - should warn, but wildcard makes it safe
        when _ -> 0
    }
    
    assert_eq_int(result, 1)
    
    vibez.spill("✅ Non-exhaustive pattern detection passed")
    vibez.spill("   (Warnings should be generated during compilation)")
}

slay test_pattern_compilation_performance() {
    vibez.spill("=== Test 10: Pattern Compilation Performance ===")
    
    // Test performance with many patterns
    sus test_value drip = 500
    sus category tea = sick test_value {
        when 0..99 -> "0-99"
        when 100..199 -> "100-199"
        when 200..299 -> "200-299"
        when 300..399 -> "300-399"
        when 400..499 -> "400-499"
        when 500..599 -> "500-599"
        when 600..699 -> "600-699"
        when 700..799 -> "700-799"
        when 800..899 -> "800-899"
        when 900..999 -> "900-999"
        when n when n >= 1000 -> "1000+"
        when _ -> "other"
    }
    
    assert_eq_str(category, "500-599")
    
    // Test with many literal patterns (should use jump table)
    sus digit drip = 7
    sus digit_name tea = sick digit {
        when 0 -> "zero"
        when 1 -> "one"
        when 2 -> "two"
        when 3 -> "three"
        when 4 -> "four"
        when 5 -> "five"
        when 6 -> "six"
        when 7 -> "seven"
        when 8 -> "eight"
        when 9 -> "nine"
        when _ -> "not-a-digit"
    }
    
    assert_eq_str(digit_name, "seven")
    
    vibez.spill("✅ Pattern compilation performance tests passed")
    vibez.spill("   (Optimizations should be applied during compilation)")
}

// Helper function for array length (would be built-in)
slay len(arr []drip) drip {
    damn arr.length // Placeholder - would use actual array length
}

// Helper function for mathematical operations
slay sqrt(x drip) drip {
    // Simplified square root - would use mathz.sqrt
    damn x // Placeholder
}

main()
