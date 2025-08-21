# Comprehensive test for parser edge case fixes
# Testing complex expressions that should NOT be treated as function names

# Test 1: Complex expressions in loops
slay test_loop_expressions() {
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus total drip = 0
    sus i drip = 0
    
    bestie (i < len(numbers)) {
        total = total + numbers[i]  # Valid assignment
        i = i + 1                  # Valid assignment
    }
    
    vibez.spill("Loop test passed, total:", total)
}

# Test 2: Complex conditionals in functions
slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    } otherwise {
        damn n * factorial(n - 1)
    }
}

# Test 3: Nested expressions with proper precedence
slay test_complex_precedence() {
    sus a drip = 10
    sus b drip = 20
    sus c drip = 30
    
    # Complex condition that should parse correctly
    ready (a + b < c * 2 && a > 5) {
        vibez.spill("Complex precedence test passed")
    }
    
    # Assignment with complex right-hand side
    sus result drip = a + b * c - (a / 2)
    vibez.spill("Complex assignment result:", result)
}

# Test 4: Array operations with complex indexing
slay test_array_operations() {
    sus data []drip = [10, 20, 30, 40, 50]
    sus index drip = 2
    
    # Complex array access should work correctly
    sus value drip = data[index + 1]
    vibez.spill("Array access test:", value)
    
    # Assignment to array element
    data[index] = data[index] * 2
    vibez.spill("Modified array element:", data[index])
}

# Test 5: Method calls with complex arguments
slay test_method_calls() {
    sus message tea = "Hello World"
    
    # Method call with complex arguments should parse correctly
    vibez.spill("Method call test:", message.len() + 5)
}

# Run all tests
slay main() {
    vibez.spill("=== Parser Edge Case Validation ===")
    
    test_loop_expressions()
    
    sus fact_result drip = factorial(5)
    vibez.spill("Factorial test result:", fact_result)
    
    test_complex_precedence()
    test_array_operations()
    test_method_calls()
    
    vibez.spill("=== All parser edge case tests completed ===")
}

main()
