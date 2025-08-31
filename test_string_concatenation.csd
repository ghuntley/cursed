#!/usr/bin/env cursed

# Test string concatenation support in CURSED
func main() {
    # Basic string concatenation  
    hello := "Hello"
    world := "World"
    greeting := hello + " " + world + "!"
    print(greeting)  # Should print: Hello World!
    
    # Test with variables and assignments
    first := "CURSED"
    second := " is"
    third := " awesome!"
    result := first + second + third
    print(result)  # Should print: CURSED is awesome!
    
    # Test assignment with concatenation
    message := "Test: "
    message = message + "passed"
    print(message)  # Should print: Test: passed
    
    # Test mixed with numbers (should work if we convert to strings)
    name := "Test "
    number := 42
    # This will test if we need to handle mixed types
    # final := name + number  # This might fail, we'll see
    
    print("String concatenation tests completed!")
}

# Call main
main()
