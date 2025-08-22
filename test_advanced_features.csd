# CURSED Advanced Features Test
# Testing functions, control flow, loops

# Function definitions
slay greet(name tea) tea {
    damn "Hello, " + name + "!"
}

slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay factorial(n drip) drip {
    sus result drip = 1
    bestie (sus i drip = 1; i <= n; i = i + 1) {
        result = result * i
    }
    damn result
}

# Control flow testing
slay test_control(x drip) tea {
    ready (x > 10) {
        damn "Large number"
    } otherwise ready (x > 5) {
        damn "Medium number"
    } otherwise {
        damn "Small number"
    }
}

# Main test execution
spill("=== ADVANCED FEATURES TEST ===")

# Test functions
spill("Greeting:", greet("CURSED"))
spill("Fibonacci(7):", fibonacci(7))
spill("Factorial(5):", factorial(5))

# Test control flow
spill("Control test (15):", test_control(15))
spill("Control test (7):", test_control(7))
spill("Control test (3):", test_control(3))

# Test loops
spill("Loop test:")
bestie (sus i drip = 1; i <= 5; i = i + 1) {
    spill("  Iteration:", i)
}

# Array testing if supported
sus numbers []drip = [1, 2, 3, 4, 5]
spill("Array length:", len(numbers))
bestie (sus j drip = 0; j < len(numbers); j = j + 1) {
    spill("  numbers[" + str(j) + "] =", numbers[j])
}
