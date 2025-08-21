# Simple Memory Safety Test for CURSED v1.0

yeet "vibez"

# Test 1: Basic variable allocation and deallocation
slay test_variables() drip {
    sus x drip = 42
    sus y drip = 100
    sus result drip = x + y
    vibez.spill("Variables test:", result)
    damn result
}

# Test 2: Simple string operations
slay test_strings() tea {
    sus str1 tea = "Hello"
    sus str2 tea = "CURSED"
    vibez.spill("String test:", str1, str2)
    damn str1
}

# Test 3: Simple recursive function
slay simple_factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * simple_factorial(n - 1)
}

slay test_recursion() drip {
    sus result drip = simple_factorial(5)
    vibez.spill("Recursion test - factorial(5):", result)
    damn result
}

vibez.spill("Starting Simple Memory Safety Test")
vibez.spill("==================================")

test_variables()
test_strings()  
test_recursion()

vibez.spill("Simple Memory Safety Test Completed")
