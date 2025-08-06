yeet "testz"

test_start("Testing Framework Validation")

# Test basic assertions
assert_true(based)
assert_false(cringe)
assert_eq_int(42, 42)
assert_eq_string("hello", "hello")

# Test failing assertions (these should be caught)
sus test_failures drip = 0

# Mock failing test (in real implementation, this would be isolated)
vibez.spill("Testing assertion failures...")

# Test numeric assertions
assert_eq_int(10, 10)
assert_eq_int(0, 0)
assert_eq_int(-5, -5)

# Test string assertions  
assert_eq_string("", "")
assert_eq_string("test", "test")
assert_eq_string("CURSED", "CURSED")

# Test boolean assertions
assert_true(10 > 5)
assert_true("hello" == "hello")
assert_false(5 > 10)
assert_false("a" == "b")

# Test complex conditions
sus x drip = 15
sus y drip = 30

assert_true(x < y)
assert_true(x + y == 45)
assert_false(x > y)
assert_eq_int(x * 2, y)

# Test with functions
slay add(a drip, b drip) drip {
    damn a + b
}

slay is_even(n drip) lit {
    damn n % 2 == 0
}

assert_eq_int(add(5, 3), 8)
assert_eq_int(add(0, 0), 0)
assert_eq_int(add(-2, 7), 5)

assert_true(is_even(4))
assert_true(is_even(0))
assert_false(is_even(3))
assert_false(is_even(7))

# Test with arrays
sus numbers := [1, 2, 3, 4, 5]
assert_eq_int(len(numbers), 5)
assert_eq_int(numbers[0], 1)
assert_eq_int(numbers[4], 5)

# Test with structs
squad Point {
    spill x drip
    spill y drip
}

sus p Point = Point{x: 10, y: 20}
assert_eq_int(p.x, 10)
assert_eq_int(p.y, 20)

# Test string operations
sus greeting tea = "Hello, " + "World!"
assert_eq_string(greeting, "Hello, World!")

sus name tea = "CURSED"
assert_eq_string(name, "CURSED")
assert_true(len(name) == 6)

vibez.spill("All testz framework tests completed")

print_test_summary()
