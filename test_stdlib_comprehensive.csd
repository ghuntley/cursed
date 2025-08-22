// Standard Library Comprehensive Test

spill("=== CURSED Standard Library Test Suite ===")

// Test basic I/O functions
spill("Testing basic I/O functions:")
spill("Hello from CURSED!")

// Test string functions
sus test_string tea = "Hello World"
spill("String functions:")
spill("Original string:", test_string)
spill("String length:", len(test_string))

// Test array functions  
sus numbers []drip = [1, 2, 3, 4, 5]
spill("Array functions:")
spill("Original array:", numbers)
spill("Array length:", len(numbers))

// Test mathematical operations
sus math_a drip = 10
sus math_b drip = 3
spill("Mathematical operations:")
spill("Addition:", math_a + math_b)
spill("Subtraction:", math_a - math_b)
spill("Multiplication:", math_a * math_b)
spill("Division:", math_a / math_b)

// Test control structures
spill("Control structure tests:")
ready (math_a > math_b) {
    spill("Conditional: math_a is greater than math_b")
} otherwise {
    spill("Conditional: math_a is not greater than math_b")
}

// Test loop functionality
spill("Loop functionality:")
bestie (i drip = 1; i <= 3; i = i + 1) {
    spill("Loop iteration:", i)
}

// Test function definitions
slay stdlib_test_function(x drip) drip {
    damn x * 2
}

sus func_result drip = stdlib_test_function(21)
spill("Function test result:", func_result)

// Test boolean operations
sus bool1 lit = based
sus bool2 lit = cap
spill("Boolean operations:")
spill("bool1 AND bool2:", bool1 && bool2)
spill("bool1 OR bool2:", bool1 || bool2)
spill("NOT bool1:", !bool1)

spill("=== All Standard Library Tests Completed ===")
