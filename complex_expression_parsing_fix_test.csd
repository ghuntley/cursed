# Complex Expression Parsing Fix Test
yeet "vibez"

# Test case 1: The problematic case - this should be two separate statements
slay test_problematic_parsing() {
    sus i drip = 0
    sus total drip = 0
    sus numbers []drip = [1, 2, 3, 4, 5]
    
    # These should all parse as separate statements
    i = i + 1  # Assignment statement
    { total = total + numbers[i] }  # Block statement  
    
    vibez.spill("i:", i, "total:", total)
}

# Test case 2: Complex expressions in loops
slay test_complex_loops() {
    sus data []drip = [10, 20, 30, 40, 50]
    sus sum drip = 0
    sus i drip = 0
    
    bestie (i < len(data)) {
        sum = sum + data[i]
        i = i + 1
    }
    
    vibez.spill("Sum:", sum)
}

# Test case 3: Nested expressions with parentheses
slay test_nested_expressions() {
    sus x drip = 5
    sus y drip = 10
    sus result drip = ((x + y) * 2) - (x * y / 2)
    vibez.spill("Complex result:", result)
}

# Test case 4: Mixed assignment and arithmetic
slay test_mixed_operations() {
    sus a drip = 1
    sus b drip = 2
    sus c drip = 3
    
    # These should parse correctly
    a = a + b * c
    b = (a - b) + c
    c = a * b - c
    
    vibez.spill("a:", a, "b:", b, "c:", c)
}

# Test case 5: Ready expressions with complex conditions
slay test_ready_conditions() {
    sus x drip = 15
    sus result tea = ready (x > 10 and x < 20) { "medium" } otherwise { "other" }
    vibez.spill("Classification:", result)
}

# Run all tests
test_problematic_parsing()
test_complex_loops()
test_nested_expressions()
test_mixed_operations()
test_ready_conditions()
