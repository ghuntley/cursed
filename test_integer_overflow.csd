# Test cases for LLVM integer overflow bug
# This file tests various scenarios where integer overflow might occur

# Test large integer values
sus very_large_int drip = 2147483647
sus max_i32 drip = 2147483647
sus overflow_int drip = 2147483648

# Test arithmetic that might overflow
sus add_result drip = 2000000000 + 1000000000
sus mult_result drip = 1000000 * 1000000
sus power_result drip = 10 * 10 * 10 * 10 * 10 * 10 * 10 * 10

# Test complex expressions with large numbers
sus complex_calc drip = (2147483647 + 1) * 2 - 100

# Test array with large indices
sus large_array []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
sus large_index drip = 4294967295

# Test function with large parameter counts or deep nesting
slay test_large_function(a drip, b drip, c drip, d drip, e drip, f drip, g drip, h drip, i drip, j drip) drip {
    damn a + b + c + d + e + f + g + h + i + j
}

# Test deeply nested function calls
slay nested_func1(x drip) drip { damn nested_func2(x + 1) }
slay nested_func2(x drip) drip { damn nested_func3(x + 1) }
slay nested_func3(x drip) drip { damn nested_func4(x + 1) }
slay nested_func4(x drip) drip { damn x + 1000000000 }

# Test large string literals that might cause buffer overflows
sus large_string tea = "This is a very long string that contains many characters and might cause issues with buffer management in the compiler when processing large amounts of text data that could potentially trigger integer overflow conditions in string length calculations or buffer size computations within the LLVM backend compilation process."

vibez.spill("Testing integer overflow scenarios...")
vibez.spill("Large int:", very_large_int)
vibez.spill("Max i32:", max_i32) 
vibez.spill("Overflow:", overflow_int)
vibez.spill("Add result:", add_result)
vibez.spill("Mult result:", mult_result)
vibez.spill("Complex:", complex_calc)
vibez.spill("Large function result:", test_large_function(1, 2, 3, 4, 5, 6, 7, 8, 9, 10))
vibez.spill("Nested result:", nested_func1(1))
