fr fr Memory safety test for CURSED compiler
fr fr This program tests various memory-intensive operations

sus counter drip = 0
sus max_iterations drip = 1000

fr fr Test variable assignments in loop
bestie (counter < max_iterations) {
    sus temp_var drip = counter * 2
    vibez.spill("Iteration:", temp_var)
    counter = counter + 1
}

fr fr Test string operations
vibez.spill("Testing string literals")
vibez.spill("Multiple", "string", "arguments")

fr fr Test arithmetic expressions
sus result drip = 42 + 8 * 3 - 2
vibez.spill("Arithmetic result:", result)

fr fr Test variable reuse
sus x drip = 10
x = x + 5
x = x * 2
vibez.spill("Final x value:", x)

vibez.spill("Memory safety test completed successfully!")
