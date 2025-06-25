// CURSED Documentation Generation Demo
// This example showcases the documentation generation system

/// Main entry point for the CURSED docs demo
/// 
/// This function demonstrates how CURSED documentation works
/// with Gen Z slang keywords and comprehensive features.
/// 
/// @example Basic usage
/// ```cursed
/// main()
/// ```
/// 
/// @author CURSED Documentation Team
/// @since 1.0.0
slay main() {
    println("🔥 Welcome to CURSED Documentation Demo!")
    
    // Demonstrate different language features
    demonstrate_variables()
    demonstrate_functions()
    demonstrate_structs()
    demonstrate_interfaces()
    demonstrate_control_flow()
}

/// Demonstrates variable declarations in CURSED
/// 
/// Shows the difference between mutable (sus) and immutable (facts) variables.
/// The Gen Z slang makes variable declarations more expressive and fun!
/// 
/// @example Variable declarations
/// ```cursed
/// sus count = 0        // mutable variable (kinda sus)
/// facts pi = 3.14159   // constant (straight facts)
/// ```
/// 
/// @see demonstrate_constants
/// @since 1.0.0
slay demonstrate_variables() {
    println("📝 Variable Demonstrations:")
    
    // Mutable variable (sus because it can change)
    sus count = 0
    count = count + 1
    println("Count is now: " + count.to_string())
    
    // Immutable constant (facts because it's true)
    facts pi = 3.14159
    println("Pi is always: " + pi.to_string())
    
    // String variable with Gen Z energy
    sus vibe = "lowkey amazing"
    println("Current vibe: " + vibe)
}

/// Demonstrates function definitions and calls
/// 
/// CURSED uses the 'slay' keyword for functions because
/// good functions absolutely slay! 💯
/// 
/// @param name The name to greet
/// @return Greeting message as string
/// @example Function with parameters
/// ```cursed
/// slay greet(name: string) -> string {
///     return "Hey " + name + "! You're serving looks! ✨"
/// }
/// ```
slay demonstrate_functions() {
    println("🎯 Function Demonstrations:")
    
    facts greeting = greet_user("bestie")
    println(greeting)
    
    facts calculation = calculate_vibes(8, 2)
    println("Vibe calculation result: " + calculation.to_string())
}

/// Greets a user with Gen Z energy
/// 
/// @param name User's name or nickname
/// @return Personalized greeting with emoji
slay greet_user(name: string) -> string {
    return "Hey " + name + "! You're absolutely iconic! 👑"
}

/// Calculates vibes using advanced mathematics
/// 
/// @param base_vibe Initial vibe level (1-10)
/// @param multiplier Vibe amplification factor
/// @return Total vibe score
slay calculate_vibes(base_vibe: i32, multiplier: i32) -> i32 {
    return base_vibe * multiplier
}

/// Demonstrates struct (squad) definitions
/// 
/// In CURSED, structs are called 'squads' because
/// they organize data like a friend group! 👥
/// 
/// @example Squad definition
/// ```cursed
/// squad Person {
///     name: string,
///     age: i32,
///     vibe_level: f64
/// }
/// ```
slay demonstrate_structs() {
    println("👥 Squad (Struct) Demonstrations:")
    
    // Create a new person
    sus person = Person::new("Taylor", 22, 9.5)
    println("Created person: " + person.name)
    println("Age: " + person.age.to_string())
    println("Vibe level: " + person.vibe_level.to_string())
    
    // Update vibe level
    person.update_vibe(10.0)
    println("Updated vibe level: " + person.vibe_level.to_string())
}

/// Represents a person with Gen Z characteristics
/// 
/// This squad (struct) contains all the essential information
/// about a person including their vibe level because that's important! ✨
/// 
/// @field name Person's name or chosen nickname
/// @field age How many years they've been serving looks
/// @field vibe_level Current energy level (0.0 to 10.0)
/// 
/// @example Creating a Person
/// ```cursed
/// sus taylor = Person::new("Taylor", 22, 9.5)
/// ```
squad Person {
    name: string,
    age: i32,
    vibe_level: f64
}

/// Implementation block for Person squad
impl Person {
    /// Creates a new Person with maximum vibes
    /// 
    /// @param name The person's name
    /// @param age Their age in years
    /// @param initial_vibe Starting vibe level
    /// @return New Person instance
    slay new(name: string, age: i32, initial_vibe: f64) -> Person {
        return Person {
            name: name,
            age: age,
            vibe_level: initial_vibe
        }
    }
    
    /// Updates the person's vibe level
    /// 
    /// @param new_vibe New vibe level (0.0 to 10.0)
    slay update_vibe(sus self, new_vibe: f64) {
        self.vibe_level = new_vibe
    }
    
    /// Checks if person has main character energy
    /// 
    /// @return true if vibe level > 8.0
    slay has_main_character_energy(self) -> bool {
        return self.vibe_level > 8.0
    }
}

/// Demonstrates interface (collab) definitions
/// 
/// Interfaces in CURSED are called 'collabs' because
/// they represent collaborative contracts between types! 🤝
/// 
/// @example Collab definition
/// ```cursed
/// collab Drawable {
///     slay draw(self)
///     slay get_color(self) -> string
/// }
/// ```
slay demonstrate_interfaces() {
    println("🤝 Collab (Interface) Demonstrations:")
    
    sus circle = Circle::new(5.0, "pink")
    circle.draw()
    println("Circle color: " + circle.get_color())
}

/// Interface for drawable objects
/// 
/// This collab (interface) defines the contract for anything
/// that can be drawn. All drawable objects must slay at drawing! 🎨
/// 
/// @method draw Renders the object to screen
/// @method get_color Returns the object's color
collab Drawable {
    slay draw(self)
    slay get_color(self) -> string
}

/// A circle that can be drawn
/// 
/// Implements the Drawable collab because circles
/// are absolutely iconic and deserve to be seen! ⭕
squad Circle {
    radius: f64,
    color: string
}

impl Circle {
    /// Creates a new circle with style
    /// 
    /// @param radius Circle radius in pixels
    /// @param color Circle color name
    /// @return New Circle instance
    slay new(radius: f64, color: string) -> Circle {
        return Circle {
            radius: radius,
            color: color
        }
    }
}

impl Drawable for Circle {
    /// Draws the circle with aesthetic vibes
    slay draw(self) {
        println("Drawing a " + self.color + " circle with radius " + self.radius.to_string())
        println("✨ Circle is serving geometric excellence! ✨")
    }
    
    /// Returns the circle's color
    /// 
    /// @return Color name as string
    slay get_color(self) -> string {
        return self.color
    }
}

/// Demonstrates control flow with Gen Z keywords
/// 
/// CURSED uses 'lowkey' for if statements and 'highkey' for else
/// because traditional keywords are lowkey boring! 🔄
/// 
/// @example Control flow
/// ```cursed
/// lowkey (age >= 18) {
///     println("You're an adult!")
/// } highkey {
///     println("Still growing!")
/// }
/// ```
slay demonstrate_control_flow() {
    println("🔄 Control Flow Demonstrations:")
    
    facts age = 22
    
    // Conditional statements
    lowkey (age >= 18) {
        println("✅ You're an adult, bestie!")
        lowkey (age >= 21) {
            println("🎉 You can legally vibe at all the spots!")
        }
    } highkey {
        println("📚 Still in your learning era!")
    }
    
    // Loops with bestie (for) and periodt (while)
    sus count = 0
    periodt (count < 3) {
        println("Loop iteration: " + count.to_string())
        count = count + 1
    }
    
    facts numbers = [1, 2, 3, 4, 5]
    bestie (number in numbers) {
        lowkey (number % 2 == 0) {
            println(number.to_string() + " is even - that's the energy!")
        }
    }
}

/// Advanced feature demonstration
/// 
/// Shows off some of CURSED's more advanced features
/// including async operations and error handling.
/// 
/// @example Async operations
/// ```cursed
/// stan async_task()  // spawn async task
/// yolo result        // await result
/// ```
slay demonstrate_advanced_features() {
    println("🚀 Advanced Feature Demonstrations:")
    
    // Async operations (using stan/yolo)
    facts future = stan fetch_data()
    facts result = yolo future
    println("Async result: " + result)
    
    // Error handling
    facts maybe_value = try_something_risky()
    lowkey (maybe_value.is_ok()) {
        println("Success! Got: " + maybe_value.unwrap())
    } highkey {
        println("Oops! Something went wrong: " + maybe_value.error())
    }
}

/// Fetches data asynchronously
/// 
/// @return Future that resolves to a string
slay fetch_data() -> Future<string> {
    // Simulate async work
    return Future::resolve("Data fetched successfully! 📡")
}

/// Demonstrates error-prone operation
/// 
/// @return Result with success value or error
slay try_something_risky() -> Result<string, string> {
    // Simulate 50/50 chance of success
    lowkey (random() > 0.5) {
        return Ok("Operation succeeded! 🎯")
    } highkey {
        return Err("Operation failed! 💥")
    }
}

/// Generates a random number
/// 
/// @return Random float between 0.0 and 1.0
slay random() -> f64 {
    // Placeholder for actual random number generation
    return 0.7
}
