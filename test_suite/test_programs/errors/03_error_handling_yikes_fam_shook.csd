vibe main
yeet "vibez"
yeet "mathz"

// Test comprehensive error handling with yikes/fam/shook patterns
slay divide_with_error(a normie, b normie) normie yikes {
    ready (b == 0) {
        yikes "Division by zero error"
    }
    damn a / b
}

slay multiply_with_overflow(a normie, b normie) normie yikes {
    ready (a > 1000 && b > 1000) {
        yikes "Overflow risk detected"  
    }
    damn a * b
}

slay main() {
    vibez.spill("=== CURSED Error Handling Tests ===")
    
    // Test 1: Basic error creation and handling
    fam {
        sus result = divide_with_error(10, 2) shook
        vibez.spill("Success: 10/2 =", result)
    } sus error {
        vibez.spill("Unexpected error:", error.message())
    }
    
    // Test 2: Error propagation with shook
    fam {
        sus bad_result = divide_with_error(10, 0) shook
        vibez.spill("This should not print")
    } sus error {
        vibez.spill("Caught error:", error.message())
    }
    
    // Test 3: Nested error handling 
    fam {
        sus val1 = divide_with_error(20, 4) shook
        sus val2 = multiply_with_overflow(val1, 200) shook
        vibez.spill("Nested operations result:", val2)
    } sus error {
        vibez.spill("Nested error caught:", error.message())
    }
    
    // Test 4: Multiple error conditions
    fam {
        sus a = divide_with_error(100, 5) shook
        sus b = divide_with_error(a, 0) shook
        vibez.spill("Should not reach here")
    } sus error {
        vibez.spill("Second operation failed:", error.message())
    }
    
    vibez.spill("Error handling tests completed")
    damn
}
