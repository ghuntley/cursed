fr fr Advanced String Interpolation Test
yeet "testz"

slay test_string_interpolation_types() {
    test_start("String Interpolation Type Handling")
    
    fr fr Integer interpolation
    sus count normie = 42
    sus int_str tea = "Count: ${count}"
    assert_true(stringz.contains(int_str, "42"))
    
    fr fr Float interpolation
    sus price meal = 19.99
    sus float_str tea = "Price: $${price}"
    assert_true(stringz.contains(float_str, "19.99"))
    
    fr fr Boolean interpolation (should work with future implementation)
    sus flag lit = based
    sus bool_str tea = "Status: ${flag}"
    vibez.spill(bool_str)
    
    fr fr Character interpolation
    sus initial sip = 'A'
    sus char_str tea = "Initial: ${initial}"
    vibez.spill(char_str)
}

slay test_string_interpolation_expressions() {
    test_start("String Interpolation Expression Evaluation")
    
    sus a normie = 10
    sus b normie = 5
    
    fr fr Arithmetic expressions
    sus add_result tea = "Addition: ${a + b}"
    assert_true(stringz.contains(add_result, "15"))
    
    sus sub_result tea = "Subtraction: ${a - b}"
    assert_true(stringz.contains(sub_result, "5"))
    
    sus mul_result tea = "Multiplication: ${a * b}"
    assert_true(stringz.contains(mul_result, "50"))
    
    sus div_result tea = "Division: ${a / b}"
    assert_true(stringz.contains(div_result, "2"))
}

slay test_string_interpolation_nested() {
    test_start("Nested String Interpolation")
    
    sus inner tea = "World"
    sus outer tea = "Hello ${inner}!"
    sus nested tea = "Message: ${outer}"
    
    assert_true(stringz.contains(nested, "Hello World!"))
    vibez.spill(nested)
}

slay test_string_interpolation_empty() {
    test_start("Empty and Edge Case String Interpolation")
    
    fr fr Empty variable
    sus empty tea = ""
    sus empty_interpolated tea = "Value: ${empty}"
    assert_true(stringz.contains(empty_interpolated, "Value: "))
    
    fr fr Multiple consecutive interpolations
    sus first tea = "A"
    sus second tea = "B"
    sus consecutive tea = "${first}${second}"
    assert_eq_string(consecutive, "AB")
}

slay test_string_interpolation_performance() {
    test_start("String Interpolation Performance")
    
    sus iterations normie = 100
    sus i normie = 0
    bestie i < iterations {
        sus dynamic tea = "Iteration ${i} of ${iterations}"
        fr fr Just create the string, don't print every one
        vibes i % 10 == 0 {
            vibez.spill(dynamic)
        }
        i = i + 1
    }
}

slay main() {
    vibez.spill("=== Advanced String Interpolation Tests ===")
    
    test_string_interpolation_types()
    test_string_interpolation_expressions()
    test_string_interpolation_nested()
    test_string_interpolation_empty()
    test_string_interpolation_performance()
    
    print_test_summary()
    vibez.spill("=== Advanced Tests Complete ===")
}
