fr fr Parser Improvement Validation Test
fr fr This test specifically validates the 5 main parser improvements

yeet "testz"

fr fr Test 1: Operator Precedence Handling
slay test_operator_precedence() lit {
    test_start("Operator precedence with parentheses")
    
    fr fr Test basic arithmetic precedence
    sus simple drip = 2 + 3 * 4  fr fr Should be 14, not 20
    assert_eq_int(simple, 14)
    
    fr fr Test parentheses override precedence
    sus with_parens drip = (2 + 3) * 4  fr fr Should be 20
    assert_eq_int(with_parens, 20)
    
    fr fr Test complex nested expressions
    sus complex drip = ((5 + 3) * 2) - (4 / 2)  fr fr Should be 14
    assert_eq_int(complex, 14)
    
    damn based
}

fr fr Test 2: Function Calls Within Expressions
slay test_function_calls_in_expressions() lit {
    test_start("Function calls within expressions")
    
    fr fr Simple function call in arithmetic
    sus result1 drip = abs(-5) + 10  fr fr Should be 15
    assert_eq_int(result1, 15)
    
    fr fr Nested function calls
    sus result2 drip = max(min(10, 8), 5)  fr fr Should be 8
    assert_eq_int(result2, 8)
    
    fr fr Function calls with complex arguments
    sus result3 drip = pow(2 + 1, 3 - 1)  fr fr Should be 9 (3^2)
    assert_eq_int(result3, 9)
    
    damn based
}

fr fr Test 3: Array Type Parsing
slay test_array_type_parsing() lit {
    test_start("Array type parsing")
    
    fr fr Basic array types
    sus int_array drip[] = [1, 2, 3]
    sus string_array tea[] = ["hello", "world"]
    sus bool_array lit[] = [based, cringe]
    
    fr fr Array access
    assert_eq_int(int_array[0], 1)
    assert_eq_string(string_array[1], "world")
    assert_true(bool_array[0])
    
    fr fr Sized arrays
    sus fixed_array drip[5] = [1, 2, 3, 4, 5]
    assert_eq_int(fixed_array[4], 5)
    
    fr fr Different numeric types
    sus small_array smol[] = [10, 20]
    sus large_array thicc[] = [1000000]
    sus float_array meal[] = [1.5, 2.5]
    
    assert_eq_int(small_array[1], 20)
    
    damn based
}

fr fr Test 4: String Concatenation
slay test_string_concatenation() lit {
    test_start("String concatenation expressions")
    
    fr fr Basic string concatenation
    sus hello tea = "Hello"
    sus world tea = "World"
    sus greeting tea = hello + " " + world + "!"
    assert_eq_string(greeting, "Hello World!")
    
    fr fr String concatenation with variables
    sus name tea = "Alice"
    sus age drip = 25
    sus message tea = "Name: " + name + ", Age: " + str(age)
    
    fr fr Mixed concatenation
    sus complex tea = "Result: " + str(10 + 5) + " items"
    
    damn based
}

fr fr Test 5: Comment Handling
slay test_comment_handling() lit {
    test_start("Comment handling in parsing")
    
    fr fr Comments in variable declarations
    sus x drip = 42 fr fr this is a comment
    assert_eq_int(x, 42)
    
    fr fr Comments in function calls
    sus result drip = max(
        10, fr fr first argument
        20  fr fr second argument
    )
    assert_eq_int(result, 20)
    
    fr fr Comments in complex expressions
    sus complex drip = (
        fr fr starting calculation
        (10 + 5) * 2 fr fr multiply by 2
    ) - 5 fr fr subtract 5
    assert_eq_int(complex, 25)
    
    damn based
}

fr fr Bonus: Complex Integration Test
slay test_complex_integration() lit {
    test_start("Complex integration of all features")
    
    fr fr Create test data
    sus numbers drip[] = [10, 20, 30, 40, 50]
    sus names tea[] = ["Alice", "Bob", "Charlie"]
    
    fr fr Complex expression combining all features
    sus index drip = 2
    sus result drip = (
        fr fr get array element
        numbers[index] + fr fr add base value
        (5 * 2) fr fr multiply factor
    ) / 2 fr fr divide by 2
    
    fr fr Should be (30 + 10) / 2 = 20
    assert_eq_int(result, 20)
    
    fr fr String manipulation with function calls
    sus full_name tea = names[0] + " " + names[1]
    sus formatted tea = upper(full_name) + "!"
    
    fr fr Array slicing with expressions
    sus slice_start drip = 1
    sus slice_end drip = 3
    sus subset drip[] = numbers[slice_start:slice_end]
    
    damn based
}

fr fr Main test function
slay main() {
    vibez.spill("=== Parser Improvement Validation ===")
    
    test_operator_precedence()
    test_function_calls_in_expressions()
    test_array_type_parsing()
    test_string_concatenation()
    test_comment_handling()
    test_complex_integration()
    
    print_test_summary()
    
    vibez.spill("Parser improvement validation completed!")
}
