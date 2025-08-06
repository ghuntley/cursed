fr fr String Interpolation Test
fr fr Test basic string interpolation functionality

slay test_basic_interpolation() {
    sus name tea = "World"
    sus age normie = 25
    sus height meal = 5.9
    
    fr fr Basic variable interpolation
    sus greeting tea = "Hello ${name}!"
    vibez.spill(greeting)
    
    fr fr Multiple variable interpolation
    sus info tea = "Name: ${name}, Age: ${age}, Height: ${height}"
    vibez.spill(info)
    
    fr fr Expression interpolation
    sus calculation tea = "Result: ${age + 10}"
    vibez.spill(calculation)
    
    fr fr Nested strings
    sus nested tea = "Outer ${greeting} end"
    vibez.spill(nested)
}

slay test_complex_interpolation() {
    sus x normie = 42
    sus y normie = 13
    
    fr fr Complex expressions
    sus math_result tea = "Sum: ${x + y}, Product: ${x * y}, Difference: ${x - y}"
    vibez.spill(math_result)
    
    fr fr Boolean expressions
    sus is_greater tea = "Is ${x} > ${y}? ${x > y}"
    vibez.spill(is_greater)
}

slay main() {
    vibez.spill("=== String Interpolation Tests ===")
    test_basic_interpolation()
    test_complex_interpolation()
    vibez.spill("=== Tests Complete ===")
}
