// AST Backend Functionality Test
// Test function parsing, variable scoping, and basic AST operations

slay test_function_parsing() drip {
    sus local_var drip = 42
    sus another_var tea = "test"
    damn local_var + 8
}

slay test_nested_scopes(param drip) drip {
    sus outer_var drip = param
    ready (param > 10) {
        sus inner_var drip = outer_var * 2
        damn inner_var
    } otherwise {
        sus inner_var drip = outer_var + 5
        damn inner_var
    }
}

slay test_complex_expressions() drip {
    sus a drip = 10
    sus b drip = 20
    sus c drip = 30
    
    sus result drip = (a + b) * c - (a * 2)
    damn result
}

// Main test execution
sus result1 drip = test_function_parsing()
sus result2 drip = test_nested_scopes(15)
sus result3 drip = test_complex_expressions()

spill("AST Function Parsing Test:", result1)
spill("AST Nested Scopes Test:", result2)
spill("AST Complex Expressions Test:", result3)
