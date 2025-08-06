fr fr Parser Edge Cases Test
fr fr Tests edge cases and complex scenarios for the parser improvements

yeet "testz"

fr fr Edge Case 1: Deeply nested expressions with precedence
slay test_deep_nesting() lit {
    test_start("Deeply nested expressions")
    
    fr fr Very complex precedence test
    sus result drip = ((((2 + 3) * 4) - 5) / 3) + 1
    fr fr ((5 * 4) - 5) / 3 + 1 = (20 - 5) / 3 + 1 = 15/3 + 1 = 5 + 1 = 6
    assert_eq_int(result, 6)
    
    fr fr Mixed operators with function calls
    sus complex drip = max(5, min(10, 8)) + (2 * 3) - abs(-2)
    fr fr max(5, 8) + 6 - 2 = 8 + 6 - 2 = 12
    assert_eq_int(complex, 12)
    
    damn based
}

fr fr Edge Case 2: Function calls with no arguments and complex arguments
slay test_function_call_variations() lit {
    test_start("Function call variations")
    
    fr fr Function with no arguments
    sus timestamp drip = now()
    
    fr fr Function with multiple complex arguments
    sus result drip = calculate(
        (10 + 5) * 2,  fr fr first arg: 30
        max(8, 12),    fr fr second arg: 12
        min(20, 15)    fr fr third arg: 15
    )
    
    fr fr Chained function calls
    sus chained drip = process(transform(clean(getData())))
    
    damn based
}

fr fr Edge Case 3: Array types with complex expressions as sizes
slay test_complex_array_scenarios() lit {
    test_start("Complex array scenarios")
    
    fr fr Dynamic array sizing
    sus size drip = 5
    sus dynamic_array drip[size] = [1, 2, 3, 4, 5]
    
    fr fr Multi-dimensional array types
    sus matrix drip[][] = [[1, 2], [3, 4], [5, 6]]
    
    fr fr Array access with expressions
    sus i drip = 1
    sus j drip = 0
    sus value drip = matrix[i][j]  fr fr Should be 3
    assert_eq_int(value, 3)
    
    fr fr Array slicing with computed indices
    sus start drip = 1
    sus end drip = 3
    sus numbers drip[] = [10, 20, 30, 40, 50]
    sus slice drip[] = numbers[start:end]
    
    damn based
}

fr fr Edge Case 4: String concatenation with escapes and expressions
slay test_string_edge_cases() lit {
    test_start("String concatenation edge cases")
    
    fr fr Strings with special characters
    sus quote tea = "\""
    sus newline tea = "\n"
    sus complex_string tea = "Hello" + quote + "World" + quote + newline
    
    fr fr String concatenation in function arguments
    sus formatted tea = format("Value: {}", str(42 + 8))
    
    fr fr Very long concatenation chain
    sus chain tea = "a" + "b" + "c" + "d" + "e" + "f" + "g"
    assert_eq_string(chain, "abcdefg")
    
    damn based
}

fr fr Edge Case 5: Comments in complex positions
slay test_comment_edge_cases() lit {
    test_start("Comment edge cases")
    
    fr fr Comments at start of expressions
    sus x drip = fr fr comment first
        42 + 8
    
    fr fr Comments in the middle of operations
    sus y drip = 10 fr fr mid comment
        + 20 fr fr another comment
        * 2  fr fr final comment
    
    fr fr Comments in nested structures
    sus nested drip = (
        fr fr outer comment
        max(
            fr fr inner comment
            5, fr fr between args
            10 fr fr last arg
        ) fr fr after function
        + 1 fr fr final operation
    )
    
    fr fr Comments in array literals
    sus arr drip[] = [
        fr fr before elements
        1, fr fr after first
        2, fr fr after second  
        3  fr fr after last
        fr fr after all elements
    ]
    
    damn based
}

fr fr Edge Case 6: Mixed complex scenarios
slay test_mixed_complexity() lit {
    test_start("Mixed complex scenarios")
    
    fr fr Arrays with computed values
    sus base drip = 10
    sus computed_array drip[] = [
        base * 1,      fr fr 10
        base * 2,      fr fr 20  
        base * 3,      fr fr 30
        max(base, 15), fr fr 15
        min(base, 5)   fr fr 5
    ]
    
    fr fr String interpolation with array access
    sus names tea[] = ["Alice", "Bob", "Charlie"]
    sus index drip = 1
    sus greeting tea = "Hello, " + names[index] + "!"
    assert_eq_string(greeting, "Hello, Bob!")
    
    fr fr Function calls with array slice arguments
    sus numbers drip[] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sus subset drip[] = numbers[2:5]
    sus sum_result drip = sum(subset)  fr fr sum of [3, 4, 5] = 12
    
    fr fr Complex conditional expressions
    sus condition lit = (base > 5) && (computed_array[0] == 10)
    assert_true(condition)
    
    damn based
}

fr fr Edge Case 7: Type system edge cases
slay test_type_system_edge_cases() lit {
    test_start("Type system edge cases")
    
    fr fr Mixed numeric types in expressions
    sus small_val smol = 100
    sus large_val thicc = 1000000
    sus float_val meal = 3.14
    sus normal_val normie = 42
    
    fr fr Type mixing (should work with proper coercion)
    sus mixed_result meal = float_val + normal_val
    
    fr fr Array types with different numeric types
    sus mixed_numbers normie[] = [small_val, normal_val, 50]
    
    fr fr String with numeric conversion
    sus number_string tea = str(small_val) + " is small"
    
    damn based
}

fr fr Main test runner
slay main() {
    vibez.spill("=== Parser Edge Cases Test ===")
    
    test_deep_nesting()
    test_function_call_variations()
    test_complex_array_scenarios()
    test_string_edge_cases()
    test_comment_edge_cases()
    test_mixed_complexity()
    test_type_system_edge_cases()
    
    print_test_summary()
    
    vibez.spill("All edge case tests completed!")
}
