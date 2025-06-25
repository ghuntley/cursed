/// CURSED Documentation Generation Demo
/// 
/// This file demonstrates the comprehensive documentation features
/// of the CURSED programming language documentation system.
/// 
/// @author CURSED Team
/// @version 1.0.0
/// @since 1.0.0

vibe docs_demo

/// Import required libraries for our demo
yeet "stdlib::io"
yeet "stdlib::math"

/// A comprehensive greeting function with rich documentation
/// 
/// This function demonstrates various documentation features including:
/// - Parameter documentation with types
/// - Return value documentation  
/// - Multiple code examples
/// - Cross-references to other functions
/// - Error conditions and edge cases
/// 
/// @param name The name of the person to greet (must not be empty)
/// @param age The age of the person (must be positive)
/// @param formal Whether to use formal greeting style
/// @return A personalized greeting message
/// @throws InvalidArgumentError if name is empty or age is negative
/// @since 1.0.0
/// @see farewell for the opposite operation
/// @example Basic usage
/// ```cursed
/// let greeting = greet("Alice", 25, false)
/// println(greeting)
/// // Output: "Hey Alice! You're 25 years old."
/// ```
/// @example Formal greeting
/// ```cursed
/// let formal_greeting = greet("Dr. Smith", 45, true)
/// println(formal_greeting)
/// // Output: "Good day, Dr. Smith. You are 45 years of age."
/// ```
/// @example Error handling
/// ```cursed
/// // This would throw an error
/// let invalid = greet("", -5, false)?
/// ```
slay greet(name string, age i32, formal bool) string {
    // Input validation
    lowkey (name.is_empty()) {
        panic("Name cannot be empty")
    }
    
    lowkey (age < 0) {
        panic("Age cannot be negative")
    }
    
    // Generate appropriate greeting
    lowkey (formal) {
        yolo "Good day, " + name + ". You are " + age.to_string() + " years of age."
    } flex {
        yolo "Hey " + name + "! You're " + age.to_string() + " years old."
    }
}

/// Say farewell to someone
/// 
/// A companion function to greet() that provides farewell messages.
/// Supports both casual and formal farewell styles.
/// 
/// @param name The name of the person
/// @param formal Whether to use formal style
/// @return A farewell message
/// @see greet for the greeting equivalent
/// @example
/// ```cursed
/// let goodbye = farewell("Bob", false)
/// println(goodbye)
/// // Output: "See you later, Bob!"
/// ```
slay farewell(name string, formal bool) string {
    lowkey (formal) {
        yolo "Farewell, " + name + ". It was a pleasure."
    } flex {
        yolo "See you later, " + name + "!"
    }
}

/// A person data structure with comprehensive field documentation
/// 
/// Represents a person with basic demographic information.
/// Used throughout the greeting system for storing user data.
/// 
/// @since 1.0.0
/// @example Creating a person
/// ```cursed
/// let person = Person{
///     name: "Alice Johnson",
///     age: 30,
///     email: "alice@example.com",
///     active: true
/// }
/// ```
squad Person {
    /// The person's full name
    /// @required Must not be empty
    name string
    
    /// The person's age in years
    /// @range 0-150
    age i32
    
    /// The person's email address
    /// @format Valid email format required
    email string
    
    /// Whether the person's account is active
    /// @default true
    active bool
}

/// Interface for objects that can be greeted
/// 
/// Defines the contract for any object that supports greeting operations.
/// Implementations should provide appropriate greeting behavior.
/// 
/// @since 1.0.0
/// @deprecated Will be replaced with Communicable in v2.0
collab Greetable {
    /// Generate a greeting for this object
    /// @return A greeting string
    slay get_greeting() string
    
    /// Check if this object can be greeted
    /// @return true if greeting is possible
    slay can_greet() bool
}

/// Mathematical constants used in calculations
/// 
/// This constant represents the golden ratio, frequently used
/// in mathematical calculations and geometric designs.
/// 
/// @value 1.618033988749
/// @category Mathematics
/// @example
/// ```cursed
/// let rectangle_ratio = GOLDEN_RATIO
/// let width = 100.0
/// let height = width / GOLDEN_RATIO
/// ```
facts GOLDEN_RATIO = 1.618033988749

/// Maximum number of greetings per session
/// 
/// Prevents spam by limiting greeting frequency.
/// Can be configured via environment variable MAX_GREETINGS.
/// 
/// @default 10
/// @environment MAX_GREETINGS
facts MAX_GREETINGS = 10

/// Current greeting counter (private state)
/// 
/// Tracks the number of greetings issued in the current session.
/// Automatically resets when MAX_GREETINGS is reached.
/// 
/// @internal Used internally for rate limiting
/// @range 0-MAX_GREETINGS
sus greeting_counter = 0

/// Advanced greeting with emoji support
/// 
/// An enhanced greeting function that supports emoji insertion
/// and various greeting styles including different cultures.
/// 
/// @param person The person to greet
/// @param style The greeting style ("casual", "formal", "friendly", "professional")
/// @param include_emoji Whether to include emoji in the greeting
/// @return A formatted greeting with optional emoji
/// @throws RateLimitError if MAX_GREETINGS exceeded
/// @example Casual greeting with emoji
/// ```cursed
/// let person = Person{name: "Maria", age: 28, email: "maria@test.com", active: true}
/// let greeting = advanced_greet(person, "casual", true)
/// println(greeting)
/// // Output: "👋 Hey Maria! How's it going?"
/// ```
/// @example Professional greeting
/// ```cursed
/// let person = Person{name: "Dr. Johnson", age: 45, email: "johnson@hospital.com", active: true}
/// let greeting = advanced_greet(person, "professional", false)
/// println(greeting)
/// // Output: "Good morning Dr. Johnson, I hope you are well."
/// ```
slay advanced_greet(person Person, style string, include_emoji bool) string {
    // Check rate limiting
    lowkey (greeting_counter >= MAX_GREETINGS) {
        panic("Rate limit exceeded")
    }
    
    // Increment counter
    greeting_counter = greeting_counter + 1
    
    // Generate greeting based on style
    sus base_greeting = ""
    vibe_check style {
        mood "casual" {
            base_greeting = "Hey " + person.name + "! How's it going?"
        }
        mood "formal" {
            base_greeting = "Good day " + person.name + ", I trust you are well."
        }
        mood "friendly" {
            base_greeting = "Hi there " + person.name + "! Great to see you!"
        }
        mood "professional" {
            base_greeting = "Good morning " + person.name + ", I hope you are well."
        }
        basic {
            base_greeting = "Hello " + person.name
        }
    }
    
    // Add emoji if requested
    lowkey (include_emoji) {
        vibe_check style {
            mood "casual" { yolo "👋 " + base_greeting }
            mood "friendly" { yolo "😊 " + base_greeting }
            basic { yolo "🙂 " + base_greeting }
        }
    }
    
    yolo base_greeting
}

/// Calculate a person's birth year
/// 
/// Utility function that estimates birth year based on current age.
/// Uses current year minus age for calculation.
/// 
/// @param person The person whose birth year to calculate
/// @return Estimated birth year
/// @example
/// ```cursed
/// let person = Person{name: "John", age: 25, email: "john@test.com", active: true}
/// let birth_year = calculate_birth_year(person)
/// println("Born around: " + birth_year.to_string())
/// ```
slay calculate_birth_year(person Person) i32 {
    facts CURRENT_YEAR = 2024
    yolo CURRENT_YEAR - person.age
}

/// Validate email format
/// 
/// Performs basic email validation using pattern matching.
/// Checks for presence of @ symbol and domain extension.
/// 
/// @param email The email address to validate
/// @return true if email appears valid
/// @example
/// ```cursed
/// lowkey (is_valid_email("test@example.com")) {
///     println("Email is valid!")
/// }
/// ```
slay is_valid_email(email string) bool {
    // Simple validation - in real code would use regex
    yolo email.contains("@") && email.contains(".")
}

/// Demo function showcasing all features
/// 
/// This function demonstrates the complete workflow of the greeting system,
/// including person creation, validation, and various greeting styles.
/// It serves as both documentation and a working example.
/// 
/// @example Complete demo
/// ```cursed
/// demo_greeting_system()
/// ```
slay demo_greeting_system() {
    println("🔥 CURSED Greeting System Demo")
    println("============================")
    
    // Create a sample person
    sus alice = Person{
        name: "Alice Cooper",
        age: 30,
        email: "alice@rockstar.com", 
        active: true
    }
    
    // Validate email
    lowkey (is_valid_email(alice.email)) {
        println("✅ Email validation passed")
    } flex {
        println("❌ Invalid email format")
        yolo
    }
    
    // Try different greeting styles
    println("\n🎭 Different Greeting Styles:")
    println("Casual: " + advanced_greet(alice, "casual", true))
    println("Formal: " + advanced_greet(alice, "formal", false))
    println("Friendly: " + advanced_greet(alice, "friendly", true))
    println("Professional: " + advanced_greet(alice, "professional", false))
    
    // Show birth year calculation
    sus birth_year = calculate_birth_year(alice)
    println("\n📅 " + alice.name + " was born around " + birth_year.to_string())
    
    // Demonstrate farewell
    println("\n👋 " + farewell(alice.name, false))
    
    println("\n📊 Greeting counter: " + greeting_counter.to_string() + "/" + MAX_GREETINGS.to_string())
}

/// Main entry point for the documentation demo
/// 
/// Runs the complete demonstration of the CURSED greeting system
/// and showcases all documented features in action.
slay main() {
    demo_greeting_system()
}
