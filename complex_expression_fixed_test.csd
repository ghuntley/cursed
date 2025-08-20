# Complex expression parsing test
yeet "vibez"

# Define helper functions first
slay max(a drip, b drip) drip {
    damn ready (a > b) { a } otherwise { b }
}

slay min(a drip, b drip) drip {
    damn ready (a < b) { a } otherwise { b }
}

# Test nested parentheses
sus result1 drip = ((5 + 3) * 2 - 1)
vibez.spill("Nested parentheses:", result1)

# Test operator precedence without parentheses
sus result2 drip = 5 + 3 * 2 - 1
vibez.spill("Operator precedence:", result2)

# Test complex logical expressions
sus a lit = based
sus b lit = cringe  
sus c lit = based
sus logical_result lit = (a and b) or (c and (not b))
vibez.spill("Complex logical:", logical_result)

# Test nested function calls with expressions
sus nested_call_result drip = max(min(10, 20), 5 + 3)
vibez.spill("Nested calls:", nested_call_result)

# Test array access with complex expressions
sus arr []drip = [1, 2, 3, 4, 5]
sus complex_index drip = arr[2 + 1] * arr[0 + 1]
vibez.spill("Complex array access:", complex_index)

# Test string concatenation with complex expressions  
sus str_result tea = "Result: " ++ (5 + 3) ++ " end"
vibez.spill("Complex string concat:", str_result)

# Test very complex nested expressions
sus very_complex drip = ((2 + 3) * (4 - 1)) + (max(7, 9) - min(2, 4))
vibez.spill("Very complex:", very_complex)

# Test mixed parentheses and operators
sus mixed_expr drip = (10 + 5) * 3 - (8 / 2)
vibez.spill("Mixed expression:", mixed_expr)
