// Test Stage 1: Functions (slay)
// Testing function declarations and calls

// Simple function with no parameters
slay greet() {
    print("Hello from function!")
}

// Function with parameters
slay add(a: normie, b: normie) -> normie {
    return a + b
}

// Function with string parameter
slay say_hello(name: lit) {
    sus message: lit = "Hello, " + name
    print(message)
}

// Function with multiple parameters and return
slay calculate_area(width: tea, height: tea) -> tea {
    return width * height
}

// Test function calls
greet()

sus result: normie = add(10, 20)
print(result)

say_hello("CURSED")

sus area: tea = calculate_area(5.0, 3.0)
print(area)
