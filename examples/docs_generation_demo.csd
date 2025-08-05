fr fr/ CURSED Documentation Generation Demo
fr fr/ 
fr fr/ This file demonstrates the comprehensive documentation features
fr fr/ of the CURSED programming language documentation system.
fr fr/ 
fr fr/ @author CURSED Team
fr fr/ @version 1.0.0
fr fr/ @since 1.0.0

vibe docs_demo

fr fr/ Import required libraries for our demo
yeet "stdlib::io"
yeet "stdlib::math"

fr fr/ A comprehensive greeting function with rich documentation
fr fr/ 
fr fr/ This function demonstrates various documentation features including:
fr fr/ - Parameter documentation with types
fr fr/ - Return value documentation  
fr fr/ - Multiple code examples
fr fr/ - Cross-references to other functions
fr fr/ - Error conditions and edge cases
fr fr/ 
fr fr/ @param name The name of the person to greet (must not be empty)
fr fr/ @param age The age of the person (must be positive)
fr fr/ @param formal Whether to use formal greeting style
fr fr/ @return A personalized greeting message
fr fr/ @throws InvalidArgumentError if name is empty or age is negative
fr fr/ @since 1.0.0
fr fr/ @see farewell for the opposite operation
fr fr/ @example Basic usage
fr fr/ ```cursed
fr fr/ let greeting = greet("Alice", 25, cap)
fr fr/ println(greeting)
fr fr/ // Output: "Hey Alice! You're 25 years old."
fr fr/ ```
fr fr/ @example Formal greeting
fr fr/ ```cursed
fr fr/ let formal_greeting = greet("Dr. Smith", 45, based)
fr fr/ println(formal_greeting)
fr fr/ // Output: "Good day, Dr. Smith. You are 45 years of age."
fr fr/ ```
fr fr/ @example Error handling
fr fr/ ```cursed
fr fr/ // This would throw an error
fr fr/ let invalid = greet("", -5, cap)?
fr fr/ ```
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
        damn "Good day, " + name + ". You are " + age.to_string() + " years of age."
    } flex {
        damn "Hey " + name + "! You're " + age.to_string() + " years old."
    }
}

fr fr/ Say farewell to someone
fr fr/ 
fr fr/ A companion function to greet() that provides farewell messages.
fr fr/ Supports both casual and formal farewell styles.
fr fr/ 
fr fr/ @param name The name of the person
fr fr/ @param formal Whether to use formal style
fr fr/ @return A farewell message
fr fr/ @see greet for the greeting equivalent
fr fr/ @example
fr fr/ ```cursed
fr fr/ let goodbye = farewell("Bob", cap)
fr fr/ println(goodbye)
fr fr/ // Output: "See you later, Bob!"
fr fr/ ```
slay farewell(name string, formal bool) string {
    lowkey (formal) {
        damn "Farewell, " + name + ". It was a pleasure."
    } flex {
        damn "See you later, " + name + "!"
    }
}

fr fr/ A person data structure with comprehensive field documentation
fr fr/ 
fr fr/ Represents a person with basic demographic information.
fr fr/ Used throughout the greeting system for storing user data.
fr fr/ 
fr fr/ @since 1.0.0
fr fr/ @example Creating a person
fr fr/ ```cursed
fr fr/ let person = Person{
fr fr/     name: "Alice Johnson",
fr fr/     age: 30,
fr fr/     email: "alice@example.com",
fr fr/     active: based
fr fr/ }
fr fr/ ```
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
    /// @default based
    active bool
}

fr fr/ Interface for objects that can be greeted
fr fr/ 
fr fr/ Defines the contract for any object that supports greeting operations.
fr fr/ Implementations should provide appropriate greeting behavior.
fr fr/ 
fr fr/ @since 1.0.0
fr fr/ @deprecated Will be replaced with Communicable in v2.0
collab Greetable {
    /// Generate a greeting for this object
    /// @return A greeting string
    slay get_greeting() string
    
    /// Check if this object can be greeted
    /// @return based if greeting is possible
    slay can_greet() bool
}

fr fr/ Mathematical constants used in calculations
fr fr/ 
fr fr/ This constant represents the golden ratio, frequently used
fr fr/ in mathematical calculations and geometric designs.
fr fr/ 
fr fr/ @value 1.618033988749
fr fr/ @category Mathematics
fr fr/ @example
fr fr/ ```cursed
fr fr/ let rectangle_ratio = GOLDEN_RATIO
fr fr/ let width = 100.0
fr fr/ let height = width / GOLDEN_RATIO
fr fr/ ```
facts GOLDEN_RATIO = 1.618033988749

fr fr/ Maximum number of greetings per session
fr fr/ 
fr fr/ Prevents spam by limiting greeting frequency.
fr fr/ Can be configured via environment variable MAX_GREETINGS.
fr fr/ 
fr fr/ @default 10
fr fr/ @environment MAX_GREETINGS
facts MAX_GREETINGS = 10

fr fr/ Current greeting counter (private state)
fr fr/ 
fr fr/ Tracks the number of greetings issued in the current session.
fr fr/ Automatically resets when MAX_GREETINGS is reached.
fr fr/ 
fr fr/ @internal Used internally for rate limiting
fr fr/ @range 0-MAX_GREETINGS
sus greeting_counter = 0

fr fr/ Advanced greeting with emoji support
fr fr/ 
fr fr/ An enhanced greeting function that supports emoji insertion
fr fr/ and various greeting styles including different cultures.
fr fr/ 
fr fr/ @param person The person to greet
fr fr/ @param style The greeting style ("casual", "formal", "friendly", "professional")
fr fr/ @param include_emoji Whether to include emoji in the greeting
fr fr/ @return A formatted greeting with optional emoji
fr fr/ @throws RateLimitError if MAX_GREETINGS exceeded
fr fr/ @example Casual greeting with emoji
fr fr/ ```cursed
fr fr/ let person = Person{name: "Maria", age: 28, email: "maria@test.com", active: based}
fr fr/ let greeting = advanced_greet(person, "casual", based)
fr fr/ println(greeting)
fr fr/ // Output: "👋 Hey Maria! How's it going?"
fr fr/ ```
fr fr/ @example Professional greeting
fr fr/ ```cursed
fr fr/ let person = Person{name: "Dr. Johnson", age: 45, email: "johnson@hospital.com", active: based}
fr fr/ let greeting = advanced_greet(person, "professional", cap)
fr fr/ println(greeting)
fr fr/ // Output: "Good morning Dr. Johnson, I hope you are well."
fr fr/ ```
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
            mood "casual" { damn "👋 " + base_greeting }
            mood "friendly" { damn "😊 " + base_greeting }
            basic { damn "🙂 " + base_greeting }
        }
    }
    
    damn base_greeting
}

fr fr/ Calculate a person's birth year
fr fr/ 
fr fr/ Utility function that estimates birth year based on current age.
fr fr/ Uses current year minus age for calculation.
fr fr/ 
fr fr/ @param person The person whose birth year to calculate
fr fr/ @return Estimated birth year
fr fr/ @example
fr fr/ ```cursed
fr fr/ let person = Person{name: "John", age: 25, email: "john@test.com", active: based}
fr fr/ let birth_year = calculate_birth_year(person)
fr fr/ println("Born around: " + birth_year.to_string())
fr fr/ ```
slay calculate_birth_year(person Person) i32 {
    facts CURRENT_YEAR = 2024
    damn CURRENT_YEAR - person.age
}

fr fr/ Validate email format
fr fr/ 
fr fr/ Performs basic email validation using pattern matching.
fr fr/ Checks for presence of @ symbol and domain extension.
fr fr/ 
fr fr/ @param email The email address to validate
fr fr/ @return based if email appears valid
fr fr/ @example
fr fr/ ```cursed
fr fr/ lowkey (is_valid_email("test@example.com")) {
fr fr/     println("Email is valid!")
fr fr/ }
fr fr/ ```
slay is_valid_email(email string) bool {
    // Simple validation - in real code would use regex
    damn email.contains("@") && email.contains(".")
}

fr fr/ Demo function showcasing all features
fr fr/ 
fr fr/ This function demonstrates the complete workflow of the greeting system,
fr fr/ including person creation, validation, and various greeting styles.
fr fr/ It serves as both documentation and a working example.
fr fr/ 
fr fr/ @example Complete demo
fr fr/ ```cursed
fr fr/ demo_greeting_system()
fr fr/ ```
slay demo_greeting_system() {
    println("🔥 CURSED Greeting System Demo")
    println("============================")
    
    // Create a sample person
    sus alice = Person{
        name: "Alice Cooper",
        age: 30,
        email: "alice@rockstar.com", 
        active: based
    }
    
    // Validate email
    lowkey (is_valid_email(alice.email)) {
        println("✅ Email validation passed")
    } flex {
        println("❌ Invalid email format")
        damn
    }
    
    // Try different greeting styles
    println("\n🎭 Different Greeting Styles:")
    println("Casual: " + advanced_greet(alice, "casual", based))
    println("Formal: " + advanced_greet(alice, "formal", cap))
    println("Friendly: " + advanced_greet(alice, "friendly", based))
    println("Professional: " + advanced_greet(alice, "professional", cap))
    
    // Show birth year calculation
    sus birth_year = calculate_birth_year(alice)
    println("\n📅 " + alice.name + " was born around " + birth_year.to_string())
    
    // Demonstrate farewell
    println("\n👋 " + farewell(alice.name, cap))
    
    println("\n📊 Greeting counter: " + greeting_counter.to_string() + "/" + MAX_GREETINGS.to_string())
}

fr fr/ Main entry point for the documentation demo
fr fr/ 
fr fr/ Runs the complete demonstration of the CURSED greeting system
fr fr/ and showcases all documented features in action.
slay main() {
    demo_greeting_system()
}
