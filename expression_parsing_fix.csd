// Test to demonstrate the expression parsing edge case
slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n-1) + fibonacci(n-2)
}

slay test_arithmetic(a drip, b drip) drip {
    vibez.spill("DEBUG: a =", a, "b =", b)
    damn a + b
}

// These should work (and they do):
vibez.spill("Simple call:", test_arithmetic(5, 7))
vibez.spill("Variable args:", test_arithmetic(sus x drip = 2; x, sus y drip = 3; y))

// These should work but currently fail:
vibez.spill("Expression args:", test_arithmetic(1+2, 3*4))
vibez.spill("Complex expr:", fibonacci(5))

// Test precedence in arithmetic function calls
sus result drip = test_arithmetic(2*3, 4+5)
vibez.spill("Assignment result:", result)
