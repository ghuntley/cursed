# Comprehensive test for complex expression parsing fixes
yeet "vibez"

slay test_complex_arithmetic_expressions() {
    vibez.spill("Testing complex arithmetic expressions")
    
    sus a drip = 10
    sus b drip = 5
    sus c drip = 2
    
    # Test nested parentheses
    sus result1 drip = ((a + b) * c) - (a / b)
    vibez.spill("Nested parentheses result:", result1)
    
    # Test operator precedence
    sus result2 drip = a + b * c - a / b
    vibez.spill("Operator precedence result:", result2)
    
    # Test complex assignment
    a = a + b * c
    vibez.spill("Complex assignment result:", a)
}

slay test_brace_separated_statements() {
    vibez.spill("Testing brace-separated statements")
    
    sus counter drip = 0
    sus total drip = 0
    
    # This should be parsed as expression followed by block
    counter + 1 { 
        total = total + counter 
        vibez.spill("Inside block, total:", total)
    }
    
    # This should be parsed as assignment followed by block
    counter = counter + 1 { 
        total = total * 2 
        vibez.spill("After assignment block, total:", total)
    }
}

slay test_loop_with_complex_expressions() {
    vibez.spill("Testing loops with complex expressions")
    
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus sum drip = 0
    sus i drip = 0
    
    bestie (i < len(numbers)) {
        sum = sum + numbers[i]
        i = i + 1
        
        # This should not be parsed as function call
        ready (i % 2 == 0) {
            vibez.spill("Even index:", i)
        }
    }
    
    vibez.spill("Final sum:", sum)
}

slay test_ready_expressions_with_braces() {
    vibez.spill("Testing ready expressions with braces")
    
    sus x drip = 15
    sus y drip = 10
    
    # Complex conditional with nested expressions
    sus result tea = ready ((x + y) > 20 and (x - y) > 0) {
        "both conditions met"
    } otherwise {
        "conditions not met"
    }
    
    vibez.spill("Conditional result:", result)
}

slay test_function_calls_vs_expressions() {
    vibez.spill("Testing function calls vs expressions")
    
    sus value drip = 42
    
    # This should be parsed as expression, not function call
    value + 10 {
        vibez.spill("This is a block after expression")
    }
    
    # This should be parsed correctly as function call
    test_helper_function(value)
    
    # This should be parsed as assignment + block
    value = value * 2 {
        vibez.spill("Block after assignment")
    }
}

slay test_helper_function(param drip) {
    vibez.spill("Helper function called with:", param)
}

slay test_array_operations_with_expressions() {
    vibez.spill("Testing array operations with complex expressions")
    
    sus data []drip = [10, 20, 30, 40, 50]
    sus index drip = 0
    sus multiplier drip = 2
    
    # Complex array access with expressions
    sus result drip = data[index + 1] * multiplier + data[index]
    vibez.spill("Complex array result:", result)
    
    # Array access in loops
    bestie (index < len(data) - 1) {
        sus current drip = data[index]
        sus next drip = data[index + 1]
        vibez.spill("Current:", current, "Next:", next)
        
        index = index + 1
    }
}

# Run all tests
test_complex_arithmetic_expressions()
test_brace_separated_statements()
test_loop_with_complex_expressions()
test_ready_expressions_with_braces()
test_function_calls_vs_expressions()
test_array_operations_with_expressions()

vibez.spill("All complex expression parsing tests completed!")
