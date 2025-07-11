// Simple test without testz dependency
vibez.spill("Testing CURSED stdlib implementations")

slay test_math_basic() {
    vibez.spill("Testing basic math operations")
    sus result normie = 2 + 3
    vibez.spill("2 + 3 = ")
    vibez.spill(result)
    vibez.spill("Math test completed")
}

slay test_string_basic() {
    vibez.spill("Testing basic string operations")
    sus greeting tea = "Hello, CURSED!"
    vibez.spill(greeting)
    vibez.spill("String test completed")
}

slay main() {
    vibez.spill("Starting stdlib tests...")
    test_math_basic()
    test_string_basic()
    vibez.spill("All tests completed!")
}
