// Comprehensive CURSED program to test type system implementation
// Tests various features including inference, constraints, and error detection

package test_type_system;

// Test basic variable declarations and type inference
let x = 42;              // Should infer int
let name = "CURSED";     // Should infer string  
let flag = true;         // Should infer bool
let numbers = [1, 2, 3]; // Should infer Array[int]

// Test function declarations with type inference
slay add(a, b) -> cap {
    return a + b;  // Should infer int -> int -> int
}

slay greet(name) -> cap {
    vibez.spill("Hello, " + name);  // Should infer string -> void
}

// Test complex expressions
let result = add(x, 10);           // Should type check
let doubled = result * 2;          // Should type check
let is_positive = result > 0;      // Should infer bool

// Test conditional statements
if is_positive {
    vibez.spill("Result is positive");
} else {
    vibez.spill("Result is not positive");
}

// Test loops
let counter = 0;
while counter < 5 {
    vibez.spill("Count: " + counter);
    counter = counter + 1;
}

// Test function calls and method calls
let greeting = greet(name);        // Should type check
vibez.spill("Program completed");  // Built-in method call

// Test array operations
let fruits = ["apple", "banana", "orange"];  // Array[string]
let first_fruit = fruits[0];                 // Should infer string

// Test map operations  
let scores = {
    "Alice": 95,
    "Bob": 87,
    "Charlie": 92
};  // Map[string, int]

// Test generic function (if supported)
slay identity(value) -> cap {
    return value;  // Should be polymorphic
}

let int_val = identity(42);      // Should infer int
let str_val = identity("test");  // Should infer string

// Test error cases (these should be caught by type checker)
// let error1 = 42 + "hello";           // Type error: int + string
// let error2 = add(42, "world");       // Type error: wrong argument type
// let error3 = undefined_var + 1;      // Undefined variable error
// if 42 { ... }                       // Type error: non-bool condition
