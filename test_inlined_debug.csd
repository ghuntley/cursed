# Test program to demonstrate inlined function debug information
# This tests the debug info preservation when functions are inlined

yeet "mathz"
yeet "vibez"

# Small function that should be inlined by the optimizer
slay add_numbers(a drip, b drip) drip {
    sus result drip = a + b
    vibez.spill("Adding:", a, "+", b, "=", result)
    damn result
}

# Another small function for inlining
slay multiply_by_two(x drip) drip {
    sus doubled drip = x * 2
    vibez.spill("Doubling:", x, "->", doubled)
    damn doubled
}

# Helper function with local variables
slay calculate_area(width drip, height drip) drip {
    sus area drip = width * height
    sus perimeter drip = 2 * (width + height)
    vibez.spill("Rectangle - Width:", width, "Height:", height)
    vibez.spill("Area:", area, "Perimeter:", perimeter)
    damn area
}

# Main function that calls the functions above
# These calls should be inlined with proper debug info preservation
slay main() drip {
    vibez.spill("Testing inlined function debug information")
    
    # These function calls should be inlined during optimization
    sus sum drip = add_numbers(5, 3)
    vibez.spill("Sum result:", sum)
    
    sus doubled drip = multiply_by_two(sum)
    vibez.spill("Doubled result:", doubled)
    
    sus area drip = calculate_area(4, 6)
    vibez.spill("Area result:", area)
    
    # Nested inlining test - multiply_by_two(add_numbers(...))
    sus nested_result drip = multiply_by_two(add_numbers(10, 15))
    vibez.spill("Nested inlining result:", nested_result)
    
    # Test with variables that should maintain debug info when inlined
    sus x drip = 42
    sus y drip = 17
    sus final_sum drip = add_numbers(x, y)
    vibez.spill("Final sum with variables:", final_sum)
    
    damn 0
}

# When this program is compiled with optimization, the small functions
# should be inlined, but debuggers should still be able to:
# 1. Show the original function names in stack traces
# 2. Set breakpoints in the original function source locations
# 3. Display variable names from both inlined and calling functions
# 4. Step through the original source lines even when inlined
