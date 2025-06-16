/// # CURSED Documentation Demo
/// 
/// This file demonstrates comprehensive documentation features
/// for the CURSED programming language, including Gen Z slang
/// keywords and comprehensive API documentation.
/// 
/// @author CURSED Team
/// @version 1.0.0
/// @since 2024

import "stdlib::io";
import "stdlib::math";

/// A simple greeting function that's totally fire
/// 
/// This function demonstrates basic CURSED syntax and
/// shows how documentation works with the `slay` keyword.
/// 
/// @param name The name to greet (can't be empty bestie)
/// @param times How many times to greet (must be positive)
/// @return A greeting string that hits different
/// 
/// @example Basic usage
/// ```cursed
/// facts greeting = greet_user("Alice", 1);
/// println(greeting)?;
/// ```
/// 
/// @example Multiple greetings
/// ```cursed
/// facts excited_greeting = greet_user("Bob", 3);
/// println(excited_greeting)?;
/// ```
slay greet_user(name: string, times: i32) -> string {
    sus result = "";
    
    lowkey (sus i = 0; i < times; i++) {
        result += format!("Hello {}, you're looking fire today! ", name);
    }
    
    result
}

/// A struct representing a person in our system
/// 
/// This demonstrates struct documentation with the `squad` keyword.
/// All fields are documented with their purposes and constraints.
/// 
/// @example Creating a person
/// ```cursed
/// facts person = Person {
///     name: "Alice",
///     age: 25,
///     is_online: true,
/// };
/// ```
squad Person {
    /// The person's name (required, no cap)
    name: string,
    
    /// Age in years (must be positive, obviously)
    age: i32,
    
    /// Whether the person is currently online
    is_online: bool,
}

/// Interface for things that can be greeted
/// 
/// This shows interface documentation using the `collab` keyword.
/// Perfect for when you need that polymorphic energy.
/// 
/// @example Implementation
/// ```cursed
/// impl Greetable for Person {
///     slay greet() -> string {
///         format!("Hey there, I'm {}!", self.name)
///     }
/// }
/// ```
collab Greetable {
    /// Generate a greeting message
    /// 
    /// @return A personalized greeting string
    slay greet() -> string;
    
    /// Check if the entity can be greeted right now
    /// 
    /// @return true if greeting is appropriate, periodt
    slay can_greet() -> bool;
}

/// Implementation of Greetable for Person
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

/// Calculate someone's vibe score based on various factors
/// 
/// This function uses advanced Gen Z algorithms to determine
/// how much someone is serving looks today.
/// 
/// @param base_score Starting score (1-10 scale)
/// @param modifiers Array of modifier values
/// @param boost_factor Multiplier for extra sauce
/// @return Final vibe score (higher = more main character energy)
/// 
/// @throws ValueError if base_score is outside valid range
/// @throws EmptyArrayError if modifiers array is empty
/// 
/// @example Basic calculation
/// ```cursed
/// facts score = calculate_vibe_score(7, [1.2, 0.8, 1.5], 1.1);
/// println("Your vibe score: {}", score)?;
/// ```
/// 
/// @note This function may return decimal values
/// @see format_vibe_message for displaying results
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

/// Format a vibe score into a human-readable message
/// 
/// Takes a numeric vibe score and converts it into Gen Z appropriate
/// descriptive text that hits different depending on the score range.
/// 
/// @param score The vibe score to format (0-100)
/// @return Formatted message string with appropriate energy level
/// 
/// @example
/// ```cursed
/// facts message = format_vibe_message(85.5);
/// // Returns: "Absolutely slaying! Your vibe is immaculate! 🔥✨"
/// ```
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

/// Constants for the vibe calculation system
/// 
/// These values have been scientifically determined through
/// extensive Gen Z research and TikTok analysis.

/// Maximum possible vibe score (this is the ceiling, periodt)
facts MAX_VIBE_SCORE: f64 = 100.0;

/// Minimum vibe score (we don't go below zero energy)
facts MIN_VIBE_SCORE: f64 = 0.0;

/// Default boost factor for regular calculations
facts DEFAULT_BOOST: f64 = 1.0;

/// Premium boost factor for when you're feeling extra
facts PREMIUM_BOOST: f64 = 1.25;

/// Main function demonstrating all the documented features
/// 
/// This is where the magic happens - we bring together all
/// our documented functions to create something that slaps.
/// 
/// @example Running the demo
/// ```shell
/// cursed run examples/cursed_docs_demo.csd
/// ```
slay main() {
    println("🎉 Welcome to the CURSED Documentation Demo!")?;
    println("This program showcases comprehensive documentation features.")?;
    println()?;
    
    // Create a person instance
    facts person = Person {
        name: "Alex",
        age: 22,
        is_online: true,
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

// Module-level documentation for overall context
/// # Module: cursed_docs_demo
/// 
/// This module serves as a comprehensive demonstration of the CURSED
/// programming language's documentation capabilities, featuring:
/// 
/// - **Functions**: Documented with parameters, return values, and examples
/// - **Structs**: Field documentation and usage examples  
/// - **Interfaces**: Method documentation and implementation examples
/// - **Constants**: Value documentation and usage context
/// - **Error Handling**: Comprehensive error documentation
/// - **Gen Z Slang**: Proper documentation of language-specific keywords
/// 
/// The documentation system supports multiple output formats:
/// - HTML with search functionality and navigation
/// - Markdown for GitHub and other platforms
/// - JSON for tooling integration
/// - XML for enterprise documentation systems
/// 
/// @package cursed_examples
/// @category documentation
/// @license MIT
/// @repository https://github.com/cursed-lang/cursed
