# Test file for macro expansion order validation

# Define basic object-like macro
#define MAX_SIZE 1024

# Define function-like macro
#macro multiply(a, b) {
    a * b
}

# Define macro with dependencies
#macro square(x) {
    @multiply(x, x)
}

# Define macro with potential hygiene issues
#macro temp_var(value) {
    sus temp drip = value;
    temp
}

# Define priority macros
#macro high_priority(x) {
    @sizeof(x) + 1
}

# Test immediate expansion
vibez.spill("Max size:", @MAX_SIZE)

# Test function-like expansion
sus result drip = @multiply(5, 6)
vibez.spill("Multiply result:", result)

# Test nested expansion (depends on multiply)
sus squared drip = @square(7)
vibez.spill("Square result:", squared)

# Test potential variable capture
sus temp drip = 42
sus captured drip = @temp_var(100)
vibez.spill("Original temp:", temp)
vibez.spill("Captured temp:", captured)

# Test compile-time expansion
sus type_size drip = @high_priority(drip)
vibez.spill("Type size:", type_size)

# Test recursive macro detection (should fail gracefully)
#macro recursive_test(x) {
    @recursive_test(x - 1)
}

# This should be caught by recursion detection
# sus bad_result drip = @recursive_test(5)
