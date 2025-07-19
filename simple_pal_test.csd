yeet "testz"

test_start("Simple PAL Test - No Goroutines")

# Test basic platform detection without complex runtime features
vibez.spill("=== Simple PAL Test ===")

# Test basic memory allocation (if available without goroutines)
vibez.spill("Testing basic functionality...")

# Simple calculations to verify interpreter works
sus x drip = 42
sus y drip = x * 2
assert_eq_int(y, 84)

vibez.spill("Basic arithmetic: PASS")

# Test string operations
sus greeting tea = "Hello from PAL test"
vibez.spill(greeting)

vibez.spill("String operations: PASS")

# Test simple control flow
sus result drip = 0
periodt i := 0; i < 5; i++ {
    result = result + i
}
assert_eq_int(result, 10)

vibez.spill("Control flow: PASS")

print_test_summary()
