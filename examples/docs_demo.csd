fr fr CURSED Documentation Generation Demo
fr fr This example showcases the documentation generation system

fr fr/ Main entry point for the CURSED docs demo
fr fr/ 
fr fr/ This function demonstrates how CURSED documentation works
fr fr/ with Gen Z slang keywords and comprehensive features.
fr fr/ 
fr fr/ @example Basic usage
fr fr/ ```cursed
fr fr/ main()
fr fr/ ```
fr fr/ 
fr fr/ @author CURSED Documentation Team
fr fr/ @since 1.0.0
slay main_character() {
    println("🔥 Welcome to CURSED Documentation Demo!")
    
    // Demonstrate different language features
    demonstrate_variables()
    demonstrate_functions()
    demonstrate_structs()
    demonstrate_interfaces()
    demonstrate_control_flow()
}

fr fr/ Demonstrates variable declarations in CURSED
fr fr/ 
fr fr/ Shows the difference between mutable (sus) and immutable (facts) variables.
fr fr/ The Gen Z slang makes variable declarations more expressive and fun!
fr fr/ 
fr fr/ @example Variable declarations
fr fr/ ```cursed
fr fr/ sus count = 0        // mutable variable (kinda sus)
fr fr/ facts pi = 3.14159   // constant (straight facts)
fr fr/ ```
fr fr/ 
fr fr/ @see demonstrate_constants
fr fr/ @since 1.0.0
slay demonstrate_variables() {
    println("📝 Variable Demonstrations:")
    
    // Mutable variable (sus because it can change)
    sus count = 0
    count = count + 1
    println("Count is now: " + count.to_string())
    
    // Immutable constant (facts because it's based)
    facts pi = 3.14159
    println("Pi is always: " + pi.to_string())
    
    // String variable with Gen Z energy
    sus vibe = "lowkey amazing"
    println("Current vibe: " + vibe)
}

fr fr/ Demonstrates function definitions and calls
fr fr/ 
fr fr/ CURSED uses the 'slay' keyword for functions because
fr fr/ good functions absolutely slay! 💯
fr fr/ 
fr fr/ @param name The name to greet
fr fr/ @return Greeting message as string
fr fr/ @example Function with parameters
fr fr/ ```cursed
fr fr/ slay greet(name: string) -> string {
fr fr/     return "Hey " + name + "! You're serving looks! ✨"
fr fr/ }
fr fr/ ```
slay demonstrate_functions() {
    println("🎯 Function Demonstrations:")
    
    facts greeting = greet_user("bestie")
    println(greeting)
    
    facts calculation = calculate_vibes(8, 2)
    println("Vibe calculation result: " + calculation.to_string())
}

fr fr/ Greets a user with Gen Z energy
fr fr/ 
fr fr/ @param name User's name or nickname
fr fr/ @return Personalized greeting with emoji
slay greet_user(name: string) -> string {
    return "Hey " + name + "! You're absolutely iconic! 👑"
}

fr fr/ Calculates vibes using advanced mathematics
fr fr/ 
fr fr/ @param base_vibe Initial vibe level (1-10)
fr fr/ @param multiplier Vibe amplification factor
fr fr/ @return Total vibe score
slay calculate_vibes(base_vibe: i32, multiplier: i32) -> i32 {
    return base_vibe * multiplier
}

fr fr/ Demonstrates struct (squad) definitions
fr fr/ 
fr fr/ In CURSED, structs are called 'squads' because
fr fr/ they organize data like a friend group! 👥
fr fr/ 
fr fr/ @example Squad definition
fr fr/ ```cursed
fr fr/ squad Person {
fr fr/     name: string,
fr fr/     age: i32,
fr fr/     vibe_level: f64
fr fr/ }
fr fr/ ```
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

fr fr/ Represents a person with Gen Z characteristics
fr fr/ 
fr fr/ This squad (struct) contains all the essential information
fr fr/ about a person including their vibe level because that's important! ✨
fr fr/ 
fr fr/ @field name Person's name or chosen nickname
fr fr/ @field age How many years they've been serving looks
fr fr/ @field vibe_level Current energy level (0.0 to 10.0)
fr fr/ 
fr fr/ @example Creating a Person
fr fr/ ```cursed
fr fr/ sus taylor = Person::new("Taylor", 22, 9.5)
fr fr/ ```
squad Person {
    name: string,
    age: i32,
    vibe_level: f64
}

fr fr/ Implementation block for Person squad
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
    /// @return based if vibe level > 8.0
    slay has_main_character_energy(self) -> bool {
        return self.vibe_level > 8.0
    }
}

fr fr/ Demonstrates interface (collab) definitions
fr fr/ 
fr fr/ Interfaces in CURSED are called 'collabs' because
fr fr/ they represent collaborative contracts between types! 🤝
fr fr/ 
fr fr/ @example Collab definition
fr fr/ ```cursed
fr fr/ collab Drawable {
fr fr/     slay draw(self)
fr fr/     slay get_color(self) -> string
fr fr/ }
fr fr/ ```
slay demonstrate_interfaces() {
    println("🤝 Collab (Interface) Demonstrations:")
    
    sus circle = Circle::new(5.0, "pink")
    circle.draw()
    println("Circle color: " + circle.get_color())
}

fr fr/ Interface for drawable objects
fr fr/ 
fr fr/ This collab (interface) defines the contract for anything
fr fr/ that can be drawn. All drawable objects must slay at drawing! 🎨
fr fr/ 
fr fr/ @method draw Renders the object to screen
fr fr/ @method get_color Returns the object's color
collab Drawable {
    slay draw(self)
    slay get_color(self) -> string
}

fr fr/ A circle that can be drawn
fr fr/ 
fr fr/ Implements the Drawable collab because circles
fr fr/ are absolutely iconic and deserve to be seen! ⭕
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

fr fr/ Demonstrates control flow with Gen Z keywords
fr fr/ 
fr fr/ CURSED uses 'lowkey' for if statements and 'highkey' for else
fr fr/ because traditional keywords are lowkey boring! 🔄
fr fr/ 
fr fr/ @example Control flow
fr fr/ ```cursed
fr fr/ lowkey (age >= 18) {
fr fr/     println("You're an adult!")
fr fr/ } highkey {
fr fr/     println("Still growing!")
fr fr/ }
fr fr/ ```
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
    
    // Loops with bestie (for) and periodt (periodt)
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

fr fr/ Advanced feature demonstration
fr fr/ 
fr fr/ Shows off some of CURSED's more advanced features
fr fr/ including async operations and error handling.
fr fr/ 
fr fr/ @example Async operations
fr fr/ ```cursed
fr fr/ stan async_task()  // spawn async task
fr fr/ damn result        // await result
fr fr/ ```
slay demonstrate_advanced_features() {
    println("🚀 Advanced Feature Demonstrations:")
    
    // Async operations (using stan/damn)
    facts future = stan fetch_data()
    facts result = damn future
    println("Async result: " + result)
    
    // Error handling
    facts maybe_value = try_something_risky()
    lowkey (maybe_value.is_ok()) {
        println("Success! Got: " + maybe_value.unwrap())
    } highkey {
        println("Oops! Something went wrong: " + maybe_value.error())
    }
}

fr fr/ Fetches data asynchronously
fr fr/ 
fr fr/ @return Future that resolves to a string
slay fetch_data() -> Future<string> {
    // Simulate async work
    return Future::resolve("Data fetched successfully! 📡")
}

fr fr/ Demonstrates error-prone operation
fr fr/ 
fr fr/ @return Result with success value or error
slay try_something_risky() -> Result<string, string> {
    // Simulate 50/50 chance of success
    lowkey (random() > 0.5) {
        return Ok("Operation succeeded! 🎯")
    } highkey {
        return Err("Operation failed! 💥")
    }
}

fr fr/ Generates a random number
fr fr/ 
fr fr/ @return Random float between 0.0 and 1.0
slay random() -> f64 {
    // Placeholder for actual random number generation
    return 0.7
}
