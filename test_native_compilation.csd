fr fr CURSED Native Compilation Test Program
fr fr Tests all major language features for native executable generation

fr fr Basic function definition
slay greet(name tea) tea {
    damn "Hello, " + name + "!"
}

fr fr Variable declarations and basic types
slay test_variables() {
    sus age drip = 25
    sus height meal = 175.5
    sus is_ready lit = based
    sus initial sip = 'A'
    
    vibez.spill("Age: ")
    vibez.spill(age)
    vibez.spill("Height: ")
    vibez.spill(height)
    vibez.spill("Ready: ")
    vibez.spill(is_ready)
}

fr fr Struct definition and usage
squad Person {
    name tea,
    age drip,
    active lit
}

slay create_person(name tea, age drip) Person {
    damn Person{
        name: name,
        age: age,
        active: based
    }
}

fr fr Tuple usage
slay test_tuples() {
    sus coords_tuple = (100, 200, 300)
    sus x = coords_tuple.0
    sus y = coords_tuple.1
    sus z = coords_tuple.2
    
    vibez.spill("Coordinates:")
    vibez.spill(x)
    vibez.spill(y)
    vibez.spill(z)
}

fr fr Control flow
slay test_control_flow(num drip) {
    yo (num > 10) {
        vibez.spill("Number is greater than 10")
    } nah {
        vibez.spill("Number is 10 or less")
    }
    
    sus count drip = 0
    bestie (count < 5) {
        vibez.spill("Count: ")
        vibez.spill(count)
        count = count + 1
    }
}

fr fr Array operations
slay test_arrays() {
    sus numbers = [1, 2, 3, 4, 5]
    sus first = numbers[0]
    sus last = numbers[4]
    
    vibez.spill("First number: ")
    vibez.spill(first)
    vibez.spill("Last number: ")
    vibez.spill(last)
}

fr fr String operations
slay test_strings() {
    sus greeting tea = "Hello"
    sus name tea = "CURSED"
    sus combined = greeting + ", " + name + "!"
    
    vibez.spill(combined)
}

fr fr Mathematical operations
slay test_math(a drip, b drip) drip {
    sus sum = a + b
    sus diff = a - b
    sus product = a * b
    sus quotient = a / b
    sus remainder = a % b
    
    vibez.spill("Mathematical operations:")
    vibez.spill("Sum: ")
    vibez.spill(sum)
    vibez.spill("Difference: ")
    vibez.spill(diff)
    vibez.spill("Product: ")
    vibez.spill(product)
    
    damn sum
}

fr fr Boolean operations
slay test_boolean_logic(x lit, y lit) lit {
    sus and_result = x && y
    sus or_result = x || y
    sus not_x = !x
    
    vibez.spill("Boolean operations:")
    vibez.spill("AND: ")
    vibez.spill(and_result)
    vibez.spill("OR: ")
    vibez.spill(or_result)
    vibez.spill("NOT x: ")
    vibez.spill(not_x)
    
    damn and_result
}

fr fr Main function
slay main_character() {
    vibez.spill("=== CURSED Native Compilation Test ===")
    
    fr fr Test basic variables
    test_variables()
    
    fr fr Test person creation
    sus person = create_person("Alice", 30)
    vibez.spill("Created person:")
    vibez.spill(person.name)
    vibez.spill(person.age)
    
    fr fr Test tuples
    test_tuples()
    
    fr fr Test control flow
    test_control_flow(15)
    test_control_flow(5)
    
    fr fr Test arrays
    test_arrays()
    
    fr fr Test strings
    test_strings()
    
    fr fr Test math
    sus result = test_math(20, 5)
    vibez.spill("Math result: ")
    vibez.spill(result)
    
    fr fr Test boolean logic
    sus bool_result = test_boolean_logic(based, cringe)
    vibez.spill("Boolean result: ")
    vibez.spill(bool_result)
    
    fr fr Test greeting function
    sus greeting_msg = greet("CURSED Compiler")
    vibez.spill(greeting_msg)
    
    vibez.spill("=== All tests completed successfully! ===")
}
