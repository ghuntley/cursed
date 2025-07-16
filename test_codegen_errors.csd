# Test file for codegen error recovery
# Tests LLVM code generation error handling

# Function with complex expressions that might fail codegen
slay complex_function(a normie, b normie, c normie) normie {
    # Complex nested expression that might cause register allocation issues
    damn ((a + b) * (c - a)) / ((b * c) + (a - c)) + ((a * b * c) / (a + b + c))
}

# Valid simple function
slay simple_function() normie {
    damn 42
}

# Function with potential type issues
slay type_problem_function(param tea) normie {
    # Try to do arithmetic on string (should be caught earlier but test codegen recovery)
    damn param + 42
}

# Arrays and complex data structures
sus complex_array [10]normie
complex_array[0] = 1
complex_array[1] = 2

# Loops with potential issues
bestie i := 0; i < 10; i++ {
    complex_array[i] = complex_function(i, i+1, i*2)
}

# Valid code that should compile correctly
sus simple_var normie = 100
vibez.spill("Simple code works")
vibez.spill(simple_var)

# Function calls
sus result normie = simple_function()
vibez.spill(result)

# More complex valid code
lowkey simple_var > 50 {
    vibez.spill("Value is greater than 50")
} else {
    vibez.spill("Value is less than or equal to 50")
}
