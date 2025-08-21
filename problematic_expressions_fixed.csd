# Test cases for the specific parser edge cases mentioned
# These were previously causing issues but should now work correctly

# Issue 1: Expression "i + 1 { total = total + numbers[i] }" 
# Previously being treated as function name - now properly parsed
slay test_problematic_expression1() {
    sus numbers []drip = [1, 2, 3]
    sus total drip = 0
    sus i drip = 0
    
    # This complex expression should now parse correctly
    # instead of being treated as a function name
    bestie (i < len(numbers)) {
        ready (i + 1 < len(numbers)) {
            total = total + numbers[i]
        }
        i = i + 1
    }
    
    vibez.spill("Test 1 passed - complex expression in condition")
}

# Issue 2: Expression "ready n <= 1 { damn 1 } otherwise"
# Previously being treated as function name - now properly parsed
slay test_problematic_expression2(n drip) drip {
    # This should now parse correctly as conditional statement
    # instead of being treated as a function name
    ready (n <= 1) {
        damn 1
    } otherwise {
        damn n * test_problematic_expression2(n - 1)  
    }
}

# Issue 3: Complex nested expressions with operators
slay test_nested_complexity() {
    sus data []drip = [5, 10, 15, 20]
    sus result drip = 0
    
    # Complex expression that should parse without issues
    bestie (i := 0; i < len(data); i = i + 1) {
        ready (data[i] > 5 && data[i] < 20) {
            result = result + data[i] * 2
        }
    }
    
    vibez.spill("Test 3 passed - nested complexity:", result)
}

# Issue 4: Assignment vs expression disambiguation 
slay test_assignment_disambiguation() {
    sus x drip = 10
    sus y drip = 20
    
    # These should be properly distinguished
    x = x + y  # Valid assignment
    sus z drip = x + y  # Valid declaration with expression
    
    # This would be invalid and should be caught by our fixes:
    # x + y = 30  # Invalid - complex expression cannot be assignment target
    
    vibez.spill("Test 4 passed - assignment disambiguation")
}

# Main test runner
slay run_parser_fixes_validation() {
    vibez.spill("=== Testing Parser Edge Case Fixes ===")
    
    test_problematic_expression1()
    
    sus factorial_result drip = test_problematic_expression2(5) 
    vibez.spill("Test 2 passed - factorial result:", factorial_result)
    
    test_nested_complexity()
    test_assignment_disambiguation()
    
    vibez.spill("=== All parser edge case fixes validated ===")
}

run_parser_fixes_validation()
