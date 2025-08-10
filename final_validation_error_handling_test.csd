# Error Handling Test
yeet "testz"

# Test 1: Basic error handling
slay test_division(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "Division by zero error"
    }
    damn a / b
}

# Test 2: Using error handling
sus result1 drip = test_division(10, 2) fam {
    when "Division by zero error" -> {
        vibez.spill("Caught division by zero!")
        damn 0
    }
    when _ -> {
        vibez.spill("Unknown error caught")
        damn -1
    }
}

vibez.spill("Result 1:", result1)

# Test 3: Error with zero division
sus result2 drip = test_division(10, 0) fam {
    when "Division by zero error" -> {
        vibez.spill("Successfully caught division by zero!")
        damn 999
    }
    when _ -> {
        vibez.spill("Unexpected error")
        damn -1
    }
}

vibez.spill("Result 2:", result2)

# Test 4: Nested error handling
slay nested_operation(x drip) yikes<tea> {
    ready (x < 0) {
        yikes "Negative number not allowed"
    }
    ready (x > 100) {
        yikes "Number too large"
    }
    damn x * 2
}

sus result3 drip = nested_operation(50) fam {
    when "Negative number not allowed" -> damn 0
    when "Number too large" -> damn 100
    when _ -> damn -1
}

sus result4 drip = nested_operation(-5) fam {
    when "Negative number not allowed" -> {
        vibez.spill("Caught negative number error!")
        damn 0
    }
    when _ -> damn -1
}

vibez.spill("Result 3:", result3)
vibez.spill("Result 4:", result4)

vibez.spill("Error handling test completed successfully!")
