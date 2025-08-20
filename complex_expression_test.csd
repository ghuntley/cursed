# Complex expression parsing test
yeet "vibez"

# Test nested parentheses
sus result1 drip = ((5 + 3) * 2 - 1)
vibez.spill("Nested parentheses:", result1)

# Test operator precedence without parentheses
sus result2 drip = 5 + 3 * 2 - 1  # Should be 5 + (3 * 2) - 1 = 10
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

# Test ternary-like expressions with ready/otherwise
sus ternary_result drip = ready (5 > 3) { 10 + 5 } otherwise { 2 * 3 }
vibez.spill("Ternary result:", ternary_result)

slay max(a drip, b drip) drip {
    damn ready (a > b) { a } otherwise { b }
}

slay min(a drip, b drip) drip {
    damn ready (a < b) { a } otherwise { b }
}
