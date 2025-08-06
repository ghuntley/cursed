fr fr Complex Expression Parser Test Cases
fr fr Tests the fixed parser for complex expression parsing

fr fr Test 1: Operator precedence with parentheses
slay test_operator_precedence() {
    sus result drip = (42 + 24) * 2
    vibez.spill("Result should be 132")
    damn result
}

fr fr Test 2: Nested function calls
slay test_nested_function_calls() {
    sus value drip = max(min(10, 5), 3)
    vibez.spill("Nested calls work")
    damn value
}

fr fr Test 3: Function calls with complex arguments
slay test_function_calls_complex_args() {
    sus result drip = calculate((10 + 5) * 2, min(20, 15))
    vibez.spill("Function with complex args")
    damn result
}

fr fr Test 4: Array type parsing
slay test_array_types() {
    sus numbers drip[] = [1, 2, 3, 4, 5]
    sus names tea[] = ["Alice", "Bob", "Charlie"]
    sus flags lit[] = [based, cringe, based]
    
    vibez.spill("Array types parsed correctly")
    damn numbers[0]
}

fr fr Test 5: String concatenation
slay test_string_concatenation() {
    sus name tea = "Hello"
    sus greeting tea = name + " " + "World!"
    sus repeated tea = greeting ++ " Again!"
    
    vibez.spill(greeting)
    damn repeated
}

fr fr Test 6: Comments in expressions
slay test_comments_in_expressions() {
    sus x drip = 42 fr fr this is a number
    sus y drip = ( fr fr start group
        x + 10 fr fr add ten
    ) fr fr end group
    
    fr fr Function call with comments
    sus result drip = calculate(
        x, fr fr first argument
        y  fr fr second argument
    )
    
    vibez.spill("Comments handled correctly")
    damn result
}

fr fr Test 7: Complex expression combinations
slay test_complex_combinations() {
    sus arr drip[] = [1, 2, 3]
    sus index drip = 1
    sus multiplier drip = 2
    
    fr fr Array access in arithmetic
    sus result drip = arr[index] * multiplier + (5 - 2)
    
    fr fr String interpolation with function calls
    sus message tea = "Result: " + str(result) + " (calculated)"
    
    vibez.spill(message)
    damn result
}

fr fr Test 8: Array slicing
slay test_array_slicing() {
    sus numbers drip[] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    
    fr fr Simple indexing
    sus first drip = numbers[0]
    
    fr fr Slice operations
    sus middle drip[] = numbers[2:5]
    sus tail drip[] = numbers[5:]
    sus head drip[] = numbers[:3]
    
    vibez.spill("Array slicing works")
    damn first
}

fr fr Test 9: Member access chains
slay test_member_access_chains() {
    sus obj = createObject()
    
    fr fr Chained member access
    sus value drip = obj.data.value.number
    
    fr fr Method chaining
    sus result = obj.transform().normalize().getValue()
    
    vibez.spill("Member access chains work")
    damn value
}

fr fr Test 10: Mixed type arrays and expressions
slay test_mixed_expressions() {
    sus numbers normie[] = [1, 2, 3]
    sus floats meal[] = [1.5, 2.5, 3.5]
    sus small_ints smol[] = [10, 20, 30]
    sus large_ints thicc[] = [1000000, 2000000]
    
    fr fr Complex expression with different types
    sus result meal = floats[0] + numbers[1] * 2.0
    
    vibez.spill("Mixed type expressions work")
    damn result
}

fr fr Main test runner
slay main() {
    vibez.spill("Running complex expression parser tests...")
    
    test_operator_precedence()
    test_nested_function_calls()
    test_function_calls_complex_args()
    test_array_types()
    test_string_concatenation()
    test_comments_in_expressions()
    test_complex_combinations()
    test_array_slicing()
    test_member_access_chains()
    test_mixed_expressions()
    
    vibez.spill("All tests completed!")
}
