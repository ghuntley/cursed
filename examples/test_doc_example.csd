fr fr/ CURSED Documentation Example
fr fr/ 
fr fr/ This example demonstrates the CURSED documentation generator
fr fr/ with various language constructs and documentation comments.

fr fr/ Calculate the factorial of a number
fr fr/ @param n The number to calculate factorial for
fr fr/ @return The factorial result
fr fr/ @example Basic usage
fr fr/ facts result = factorial(5);
slay factorial(n: i32) -> i32 {
    lowkey (n <= 1) {
        return 1;
    } bestie {
        return n * factorial(n - 1);
    }
}

fr fr/ Person struct representing a user
fr fr/ Contains basic information about a person
squad Person {
    /// Person's full name
    name: string,
    /// Person's age in years  
    age: i32,
    /// Person's email address
    email: string,
}

fr fr/ Greeting interface for objects that can greet
collab Greeter {
    /// Say hello to someone
    /// @param name The name to greet
    /// @return A greeting message
    slay greet(name: string) -> string;
}

fr fr/ Maximum number of retries
facts MAX_RETRIES = 3;

fr fr/ Current application version
facts VERSION = "1.0.0";

fr fr/ Main application entry point
fr fr/ Demonstrates various CURSED language features
slay main_character() {
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
