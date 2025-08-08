// Test cases that should produce type errors
yeet "testz"

// Test 1: Type mismatch that should be caught
slay numeric_only[T: Numeric](x T) T {
    damn x + 1
}

// This should cause a constraint violation error
sus should_fail = numeric_only("hello")  // Tea doesn't satisfy Numeric constraint

vibez.spill("This should not execute if type checking works correctly")
