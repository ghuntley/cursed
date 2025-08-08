yeet "testz"
yeet "mathz"

# Simplified test for working features

test_start("Simple Feature Test")

# Test basic variables
sus x drip = 42
sus y drip = 8
vibez.spill("Variables:", x, y)
assert_eq_int(x, 42)

# Test arithmetic
sus result drip = x + y
vibez.spill("Addition:", result)
assert_eq_int(result, 50)

# Test function definition and call
slay multiply(a drip, b drip) drip {
    damn a * b
}

sus product drip = multiply(6, 7)
vibez.spill("Function result:", product)
assert_eq_int(product, 42)

# Test arrays
sus numbers []drip = [1, 2, 3]
vibez.spill("Array first element:", numbers[0])
assert_eq_int(numbers[0], 1)

# Test stdlib function
sus abs_val drip = abs_normie(-15)
vibez.spill("Absolute value:", abs_val)
assert_eq_int(abs_val, 15)

# Test conditional
ready (x > 40) {
    vibez.spill("Condition worked")
    assert_true(based)
} otherwise {
    assert_true(cringe)
}

print_test_summary()
