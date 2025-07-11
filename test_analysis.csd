vibez.spill("Testing CURSED compiler")

// Test variables
sus x normie = 42
sus y drip = 3.14
sus flag lit = based
sus msg tea = "hello"
sus ch sip = 'A'

// Test tuples
sus tuple = (1, "test", based)
sus first = tuple.0

// Test short declarations
z := 10
(a, b) := (1, 2)

// Test arithmetic
sus result = x + z
sus float_result = y * 2.0

// Test function call
vibez.spill("Variables:", x, y, flag, msg, ch)
vibez.spill("Tuple:", tuple, first)
vibez.spill("Results:", result, float_result)

// Test for loop
bestie i := 0; i < 3; i++ {
    vibez.spill("Loop:", i)
}

// Test defer
defer vibez.spill("Defer test")

// Test error handling
yikes result := "test error"
vibez.spill("Error:", result)

vibez.spill("Test complete")
