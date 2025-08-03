fr fr Memory leak test program for CURSED Zig compiler
fr fr This program tests various language features to check for memory leaks

vibez.spill("Testing basic output")
vibez.spill("String literal test")
vibez.spill("Multiple", "parameters", "test")

fr fr Test different token types
sus test_var drip = 42
vibez.spill("Variable test:", test_var)

fr fr Test function calls (if supported)
slay test_function() {
    vibez.spill("Function call test")
}

test_function()

fr fr Test comments and various syntax
vibez.spill("Memory leak test completed successfully")
