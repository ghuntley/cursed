yeet "testz"
yeet "vibez"

test_start("Basic I/O Tests")

# Test basic output
vibez.spill("Testing basic output")
vibez.spill("Number: " + str(42))
vibez.spill("Boolean: " + str(based))

# Test formatted output with spillf
sus name tea = "CURSED"
sus version drip = 1
vibez.spillf("Language: %s v%d", name, version)

# Test different data types
sus numbers := [1, 2, 3, 4, 5]
vibez.spill("Array: " + str(numbers))

sus point := squad {
    x: 10,
    y: 20
}
vibez.spill("Point: x=" + str(point.x) + ", y=" + str(point.y))

# Test string operations
sus message tea = "Hello"
sus target tea = "World"
sus combined tea = message + ", " + target + "!"
vibez.spill(combined)

# Test numeric formatting
sus pi meal = 3.14159
vibez.spillf("Pi: %.2f", pi)

# Simple assertions
assert_eq_string(combined, "Hello, World!")
assert_true(based)

print_test_summary()
