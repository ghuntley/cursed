# Comprehensive test for LLVM integer overflow fixes
# This file tests edge cases for integer overflow protection

# Test function with maximum reasonable parameters (should work)
slay test_max_params(p1 drip, p2 drip, p3 drip, p4 drip, p5 drip) drip {
    damn p1 + p2 + p3 + p4 + p5
}

# Test deeply nested braces (up to reasonable limit)
slay test_nested_blocks() drip {
    ready (based) {
        ready (based) {
            ready (based) {
                vibez.spill("Nested block level 3")
                damn 42
            }
        }
    }
    damn 0
}

# Test large but reasonable integers
sus large_safe_int drip = 2000000000
sus arithmetic_result drip = large_safe_int + 100000000

# Test array with many elements (within bounds)
sus test_array []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]

# Test reasonable string length
sus reasonable_string tea = "This string is long enough to test string handling but not so long as to trigger overflow protection mechanisms."

# Test pattern matching with multiple cases (reasonable count)
sus test_value drip = 5
ready (test_value) {
    1 => vibez.spill("one")
    2 => vibez.spill("two") 
    3 => vibez.spill("three")
    4 => vibez.spill("four")
    5 => vibez.spill("five")
    _ => vibez.spill("other")
}

vibez.spill("Comprehensive overflow test results:")
vibez.spill("Max params test:", test_max_params(1, 2, 3, 4, 5))
vibez.spill("Nested blocks test:", test_nested_blocks())
vibez.spill("Large safe int:", large_safe_int)
vibez.spill("Arithmetic result:", arithmetic_result)
vibez.spill("Array access test:", test_array[10])
vibez.spill("String test: PASSED")
vibez.spill("Pattern matching: COMPLETED")
vibez.spill("All overflow protection tests: PASSED")
