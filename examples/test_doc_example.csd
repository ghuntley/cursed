/// CURSED Documentation Example
/// 
/// This example demonstrates the CURSED documentation generator
/// with various language constructs and documentation comments.

/// Calculate the factorial of a number
/// @param n The number to calculate factorial for
/// @return The factorial result
/// @example Basic usage
/// facts result = factorial(5);
slay factorial(n: i32) -> i32 {
    lowkey (n <= 1) {
        return 1;
    } bestie {
        return n * factorial(n - 1);
    }
}

/// Person struct representing a user
/// Contains basic information about a person
squad Person {
    /// Person's full name
    name: string,
    /// Person's age in years  
    age: i32,
    /// Person's email address
    email: string,
}

/// Greeting interface for objects that can greet
collab Greeter {
    /// Say hello to someone
    /// @param name The name to greet
    /// @return A greeting message
    slay greet(name: string) -> string;
}

/// Maximum number of retries
facts MAX_RETRIES = 3;

/// Current application version
facts VERSION = "1.0.0";

/// Main application entry point
/// Demonstrates various CURSED language features
slay main() {
    sus person = Person {
        name: "Alice",
        age: 30,
        email: "alice@example.com",
    };
    
    sus result = factorial(5);
    println("Factorial of 5 is: {}", result);
    
    lowkey (person.age >= 18) {
        println("{} is an adult", person.name);
    }
}
