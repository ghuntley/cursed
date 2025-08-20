# Final comprehensive test of variable dereferencing in expressions

# Basic variables
sus a drip = 10
sus b drip = 20
sus message tea = "Hello"

# Test 1: Variables in arithmetic expressions
sus result1 drip = a + b
vibez.spill("Test 1 - Arithmetic:", result1)

# Test 2: Variables in string concatenation
sus greeting tea = message + " World"
vibez.spill("Test 2 - String concat:", greeting)

# Test 3: Variables in array literals
sus my_numbers []drip = [a, b, 30]
vibez.spill("Test 3 - Array with vars:", my_numbers)

# Test 4: Variables in complex expressions
sus complex_result drip = (a + b) * 2 + a
vibez.spill("Test 4 - Complex expr:", complex_result)

# Test 5: Variables in conditionals
ready (a < b) {
    vibez.spill("Test 5 - Variable comparison works!")
}
