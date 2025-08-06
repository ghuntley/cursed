yeet "testz"

test_start("Function Definition and Calling Tests")

# Simple function
slay add(a drip, b drip) drip {
    damn a + b
}

# Function with string return
slay greet(name tea) tea {
    damn "Hello, " + name + "!"
}

# Function with boolean logic
slay is_positive(num drip) lit {
    damn num > 0
}

# Void function with side effects
slay print_info(value drip) {
    vibez.spill("Value: " + str(value))
}

# Function with multiple parameters
slay calculate_area(width meal, height meal) meal {
    damn width * height
}

# Test function calls
sus result drip = add(10, 20)
sus greeting tea = greet("CURSED")
sus positive lit = is_positive(42)
sus negative lit = is_positive(-5)
sus area meal = calculate_area(5.0, 3.0)

# Output results
vibez.spill("add(10, 20): " + str(result))
vibez.spill(greeting)
vibez.spill("is_positive(42): " + str(positive))
vibez.spill("is_positive(-5): " + str(negative))
print_info(result)
vibez.spill("area: " + str(area))

# Assertions
assert_eq_int(result, 30)
assert_eq_string(greeting, "Hello, CURSED!")
assert_true(positive)
assert_false(negative)

print_test_summary()
