fr fr/ # CURSED Documentation Demo
fr fr/ 
fr fr/ This file demonstrates comprehensive documentation features
fr fr/ for the CURSED programming language, including Gen Z slang
fr fr/ keywords and comprehensive API documentation.
fr fr/ 
fr fr/ @author CURSED Team
fr fr/ @version 1.0.0
fr fr/ @since 2024

yeet "stdlib::io"
yeet "stdlib::math"

fr fr/ A simple greeting function that's totally fire
fr fr/ 
fr fr/ This function demonstrates basic CURSED syntax and
fr fr/ shows how documentation works with the `slay` keyword.
fr fr/ 
fr fr/ @param name The name to greet (can't be empty bestie)
fr fr/ @param times How many times to greet (must be positive)
fr fr/ @return A greeting string that hits different
fr fr/ 
fr fr/ @example Basic usage
fr fr/ ```cursed
fr fr/ facts greeting = greet_user("Alice", 1);
fr fr/ println(greeting)?;
fr fr/ ```
fr fr/ 
fr fr/ @example Multiple greetings
fr fr/ ```cursed
fr fr/ facts excited_greeting = greet_user("Bob", 3);
fr fr/ println(excited_greeting)?;
fr fr/ ```
slay greet_user(name: string, times: i32) -> string {
    sus result = "";
    
    lowkey (sus i = 0; i < times; i++) {
        result += format!("Hello {}, you're looking fire today! ", name);
    }
    
    result
}

fr fr/ A struct representing a person in our system
fr fr/ 
fr fr/ This demonstrates struct documentation with the `squad` keyword.
fr fr/ All fields are documented with their purposes and constraints.
fr fr/ 
fr fr/ @example Creating a person
fr fr/ ```cursed
fr fr/ facts person = Person {
fr fr/     name: "Alice",
fr fr/     age: 25,
fr fr/     is_online: based,
fr fr/ };
fr fr/ ```
squad Person {
    /// The person's name (required, no cap)
    name: string,
    
    /// Age in years (must be positive, obviously)
    age: i32,
    
    /// Whether the person is currently online
    is_online: bool,
}

fr fr/ Interface for things that can be greeted
fr fr/ 
fr fr/ This shows interface documentation using the `collab` keyword.
fr fr/ Perfect for when you need that polymorphic energy.
fr fr/ 
fr fr/ @example Implementation
fr fr/ ```cursed
fr fr/ impl Greetable for Person {
fr fr/     slay greet() -> string {
fr fr/         format!("Hey there, I'm {}!", self.name)
fr fr/     }
fr fr/ }
fr fr/ ```
collab Greetable {
    /// Generate a greeting message
    /// 
    /// @return A personalized greeting string
    slay greet() -> string;
    
    /// Check if the entity can be greeted right now
    /// 
    /// @return based if greeting is appropriate, periodt
    slay can_greet() -> bool;
}

fr fr/ Implementation of Greetable for Person
impl Greetable for Person {
    slay greet() -> string {
        lowkey (self.is_online) {
            format!("Yo! I'm {} and I'm {} years old - slide into my DMs! 💯", self.name, self.age)
        } highkey {
            format!("Sorry bestie, {} is offline rn 😴", self.name)
        }
    }
    
    slay can_greet() -> bool {
        self.is_online && self.age >= 13
    }
}

fr fr/ Calculate someone's vibe score based on various factors
fr fr/ 
fr fr/ This function uses advanced Gen Z algorithms to determine
fr fr/ how much someone is serving looks today.
fr fr/ 
fr fr/ @param base_score Starting score (1-10 scale)
fr fr/ @param modifiers Array of modifier values
fr fr/ @param boost_factor Multiplier for extra sauce
fr fr/ @return Final vibe score (higher = more main character energy)
fr fr/ 
fr fr/ @throws ValueError if base_score is outside valid range
fr fr/ @throws EmptyArrayError if modifiers array is empty
fr fr/ 
fr fr/ @example Basic calculation
fr fr/ ```cursed
fr fr/ facts score = calculate_vibe_score(7, [1.2, 0.8, 1.5], 1.1);
fr fr/ println("Your vibe score: {}", score)?;
fr fr/ ```
fr fr/ 
fr fr/ @note This function may return decimal values
fr fr/ @see format_vibe_message for displaying results
slay calculate_vibe_score(base_score: f64, modifiers: [f64], boost_factor: f64) -> Result<f64, string> {
    // Validate inputs because we're not here for invalid data
    lowkey (base_score < 1.0 || base_score > 10.0) {
        Err("Base score must be between 1 and 10, that's just basic math bestie")
    }
    
    lowkey (modifiers.is_empty()) {
        Err("Modifiers array can't be empty - that's giving nothing energy")
    }
    
    // Calculate the total score with that mathematical precision
    sus total = base_score;
    
    // Apply each modifier (this is where the magic happens)
    lowkey (sus modifier in modifiers) {
        total *= modifier;
    }
    
    // Apply the boost factor for extra spice
    total *= boost_factor;
    
    // Clamp the result because we can't have infinite vibe
    lowkey (total > 100.0) {
        total = 100.0;
    } highkey lowkey (total < 0.0) {
        total = 0.0;
    }
    
    Ok(total)
}

fr fr/ Format a vibe score into a human-readable message
fr fr/ 
fr fr/ Takes a numeric vibe score and converts it into Gen Z appropriate
fr fr/ descriptive text that hits different depending on the score range.
fr fr/ 
fr fr/ @param score The vibe score to format (0-100)
fr fr/ @return Formatted message string with appropriate energy level
fr fr/ 
fr fr/ @example
fr fr/ ```cursed
fr fr/ facts message = format_vibe_message(85.5);
fr fr/ // Returns: "Absolutely slaying! Your vibe is immaculate! 🔥✨"
fr fr/ ```
slay format_vibe_message(score: f64) -> string {
    bestie score {
        90.0..=100.0 => "Absolutely slaying! Your vibe is immaculate! 🔥✨",
        75.0..90.0 => "Living your best life! Main character energy for sure! 💫",
        60.0..75.0 => "Pretty solid vibes, you're doing great sweetie! 💖",
        40.0..60.0 => "Mid vibes but that's okay, we all have off days 🤷‍♀️",
        20.0..40.0 => "Giving tired energy, maybe take a self-care day? 😴",
        0.0..20.0 => "Bestie, you need some serious glow-up time 💔",
        basic => "Error: Invalid vibe score detected 🚫"
    }.to_string()
}

fr fr/ Constants for the vibe calculation system
fr fr/ 
fr fr/ These values have been scientifically determined through
fr fr/ extensive Gen Z research and TikTok analysis.

fr fr/ Maximum possible vibe score (this is the ceiling, periodt)
facts MAX_VIBE_SCORE: f64 = 100.0;

fr fr/ Minimum vibe score (we don't go below zero energy)
facts MIN_VIBE_SCORE: f64 = 0.0;

fr fr/ Default boost factor for regular calculations
facts DEFAULT_BOOST: f64 = 1.0;

fr fr/ Premium boost factor for when you're feeling extra
facts PREMIUM_BOOST: f64 = 1.25;

fr fr/ Main function demonstrating all the documented features
fr fr/ 
fr fr/ This is where the magic happens - we bring together all
fr fr/ our documented functions to create something that slaps.
fr fr/ 
fr fr/ @example Running the demo
fr fr/ ```shell
fr fr/ cursed run examples/cursed_docs_demo.csd
fr fr/ ```
slay main() {
    println("🎉 Welcome to the CURSED Documentation Demo!")?;
    println("This program showcases comprehensive documentation features.")?;
    println()?;
    
    // Create a person instance
    facts person = Person {
        name: "Alex",
        age: 22,
        is_online: based,
    };
    
    // Test greeting functionality
    facts greeting = greet_user("Documentation User", 2);
    println("Greeting: {}", greeting)?;
    
    // Test interface implementation
    lowkey (person.can_greet()) {
        facts personal_greeting = person.greet();
        println("Personal greeting: {}", personal_greeting)?;
    }
    
    // Test vibe calculation with comprehensive error handling
    facts modifiers = [1.2, 0.9, 1.4, 1.1];
    
    bestie calculate_vibe_score(8.5, modifiers, PREMIUM_BOOST) {
        Ok(score) => {
            facts message = format_vibe_message(score);
            println("Your vibe: {} (Score: {:.1})", message, score)?;
        }
        Err(error) => {
            println("Error calculating vibe: {}", error)?;
        }
    }
    
    println()?;
    println("✨ Demo completed! Check out the generated docs for more info! ✨")?;
}

fr fr Module-level documentation for overall context
fr fr/ # Module: cursed_docs_demo
fr fr/ 
fr fr/ This module serves as a comprehensive demonstration of the CURSED
fr fr/ programming language's documentation capabilities, featuring:
fr fr/ 
fr fr/ - **Functions**: Documented with parameters, return values, and examples
fr fr/ - **Structs**: Field documentation and usage examples  
fr fr/ - **Interfaces**: Method documentation and implementation examples
fr fr/ - **Constants**: Value documentation and usage context
fr fr/ - **Error Handling**: Comprehensive error documentation
fr fr/ - **Gen Z Slang**: Proper documentation of language-specific keywords
fr fr/ 
fr fr/ The documentation system supports multiple output formats:
fr fr/ - HTML with search functionality and navigation
fr fr/ - Markdown for GitHub and other platforms
fr fr/ - JSON for tooling integration
fr fr/ - XML for enterprise documentation systems
fr fr/ 
fr fr/ @package cursed_examples
fr fr/ @category documentation
fr fr/ @license MIT
fr fr/ @repository https://github.com/cursed-lang/cursed
