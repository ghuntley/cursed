yeet "testz"
yeet "test_vibez_call"
yeet "vibez"

test_start("TEST_VIBEZ_CALL I/O Function Call Validation")

// Test basic vibez.spill function calls
vibez.spill("Testing basic spill")
vibez.spill("Number:", 42)
vibez.spill("String:", "hello")
vibez.spill("Boolean:", based)

// Test multiple parameter spill calls
vibez.spill("Multiple values:", 1, 2, 3)
vibez.spill("Mixed types:", "text", 99, nocap)

// Test spill with variables
sus test_var drip = 123
sus test_str tea = "variable"
vibez.spill("Variables:", test_var, test_str)

// Test spill with expressions
vibez.spill("Expression:", 10 + 20)
vibez.spill("Calculated:", 5 * 6)

// Test spill with function results
slay get_test_number() drip {
    damn 777
}

vibez.spill("Function result:", get_test_number())

// Test spill in conditional context
ready (based) {
    vibez.spill("Conditional spill works")
}

// Test spill in loop context
bestie (sus i drip = 1; i <= 3; i++) {
    vibez.spill("Loop iteration:", i)
}

// Test empty spill call
vibez.spill()

// Test single value spill calls
vibez.spill(42)
vibez.spill("single string")
vibez.spill(based)

// Test spill with complex expressions
sus x drip = 10
sus y drip = 5
vibez.spill("Complex:", (x + y) * 2, "equals", 30)

// Test spill with string concatenation
sus greeting tea = "Hello"
sus name tea = "World"
vibez.spill(greeting + " " + name)

// Test nested function calls with spill
slay double_value(val drip) drip {
    damn val * 2
}

vibez.spill("Nested call:", double_value(21))

// Test spill performance with many calls
sus perf_start drip = get_nanoseconds()
bestie (sus i drip = 0; i < 100; i++) {
    vibez.spill("Performance test:", i)
}
sus perf_end drip = get_nanoseconds()
sus perf_duration drip = perf_end - perf_start

// Validate spill call functionality
assert_true(perf_duration > 0)
assert_true(perf_duration < 500000000) // Less than 500ms for 100 calls

// Test spill with array-like structures (if supported)
sus test_values []drip = [1, 2, 3]
vibez.spill("Array values:", test_values[0], test_values[1], test_values[2])

// Test error handling with spill
sus error_test drip = 0
ready (error_test == 0) {
    vibez.spill("Error condition handled")
} otherwise {
    vibez.spill("Should not reach here")
}

// Test spill formatting edge cases
vibez.spill("Zero:", 0)
vibez.spill("Negative:", -42)
vibez.spill("Empty string:", "")
vibez.spill("False:", nocap)

// Final validation spill
vibez.spill("TEST_VIBEZ_CALL validation complete")

print_test_summary()
