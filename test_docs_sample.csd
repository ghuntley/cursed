// Package for testing documentation generation
// This file contains various CURSED language constructs

/// A simple structure representing a person
squad Person {
    name: String,     /// The person's name
    age: Int,         /// The person's age in years
    email: String,    /// Email address
}

/// Interface for objects that can be greeted
collab Greeter {
    /// Greet the person with a message
    slay greet(message: String) -> String;
}

/// Implementation of the Greeter interface for Person
impl Person for Greeter {
    /// Returns a personalized greeting
    slay greet(message: String) -> String {
        return message + ", " + self.name + "!";
    }
}

/// Creates a new person with the given details
/// 
/// # Parameters
/// - name: The person's full name
/// - age: Age in years (must be positive)
/// - email: Valid email address
///
/// # Returns
/// A new Person instance
slay create_person(name: String, age: Int, email: String) -> Person {
    // Validate age
    lowkey age < 0 {
        panic("Age cannot be negative");
    }
    
    return Person{
        name: name,
        age: age,
        email: email,
    };
}

/// A utility function to calculate years until retirement
///
/// # Example
/// ```cursed
/// let years = years_until_retirement(25);
/// ```
slay years_until_retirement(current_age: Int) -> Int {
    facts retirement_age = 65;
    return retirement_age - current_age;
}

/// Enum representing different mood states
enum Mood {
    Happy,
    Sad,
    Excited,
    Calm,
}

/// Generic container for holding any type of value
squad Container<T> {
    value: T,
    timestamp: Int,
}

/// Methods for the Container type
impl<T> Container<T> {
    /// Creates a new container with the given value
    slay new(value: T) -> Container<T> {
        return Container{
            value: value,
            timestamp: current_time(),
        };
    }
    
    /// Gets the stored value
    slay get() -> T {
        return self.value;
    }
}

/// A function demonstrating error handling
slay divide_numbers(a: Float, b: Float) -> Result<Float, String> {
    lowkey b == 0.0 {
        return Err("Division by zero");
    }
    return Ok(a / b);
}

/// Channel communication example
slay worker_example() {
    sus ch = make(chan String, 10);
    
    // Start worker goroutine
    vibes {
        periodt true {
            sus msg = <-ch;
            lowkey msg == "stop" {
                break;
            }
            println("Received: " + msg);
        }
    };
    
    // Send some messages
    ch <- "Hello";
    ch <- "World";
    ch <- "stop";
}
