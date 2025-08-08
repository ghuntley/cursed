// Test tuple return values for error handling

// Function that returns a tuple (value, error)
slay divide_safe(a drip, b drip) {
    ready (b == 0) {
        damn (0, "division by zero")
    }
    damn (a / b, "")
}

// Test tuple return
vibez.spill("Testing tuple returns...")

// This should work once we fix the parser and return statement handling
// For now, we'll test manual tuple creation
sus result1 drip = 42
sus error1 tea = ""

// Test the variables work
vibez.spill("Result:", result1)
vibez.spill("Error:", error1)

// Call function that should return tuple
sus div_result = divide_safe(10, 2)
vibez.spill("Division result:", div_result)

sus div_zero = divide_safe(10, 0) 
vibez.spill("Division by zero:", div_zero)

vibez.spill("Tuple return tests completed!")
