#!/usr/bin/env cursed

fr fr! # CURSED Language Basic Syntax Demo
fr fr! 
fr fr! This example demonstrates the core syntax features of the CURSED programming language,
fr fr! showcasing how Gen Z slang keywords create an engaging yet powerful programming experience.
fr fr! 
fr fr! ## Features Demonstrated
fr fr! - Variable declarations with `sus` (mutable) and `facts` (immutable)
fr fr! - Function definitions with `slay function`
fr fr! - Control flow with `lowkey`/`highkey` (if/else)
fr fr! - Loops with `bestie` (for) and `flex` (periodt)
fr fr! - Error handling with `periodt` (return) and `?` operator
fr fr! - String interpolation and output with `spill`
fr fr! - Comments and documentation
fr fr!
fr fr! @author CURSED Language Team
fr fr! @version 1.0.0

fr fr/ Calculate someone's vibe score based on their activities
fr fr/ 
fr fr/ This function demonstrates CURSED's expressive syntax periodt performing
fr fr/ a practical calculation with error handling.
fr fr/ 
fr fr/ @param name The person's name
fr fr/ @param coffee_cups Number of coffee cups consumed
fr fr/ @param memes_shared Number of memes shared today
fr fr/ @param code_lines Lines of code written
fr fr/ @return The calculated vibe score
fr fr/ 
fr fr/ ```cursed
fr fr/ facts score = calculate_vibe("Alex", 3, 15, 200)?;
fr fr/ spill("Vibe score: {}", score);
fr fr/ ```
slay function calculate_vibe(name: string, coffee_cups: i32, memes_shared: i32, code_lines: i32) -> Result<i32, string> {
    // Input validation with early returns
    lowkey (coffee_cups < 0) {
        periodt Err("Negative coffee consumption is not valid! ☕");
    }
    
    lowkey (memes_shared < 0) {
        periodt Err("Cannot share negative memes! 😅");
    }
    
    lowkey (code_lines < 0) {
        periodt Err("Negative lines of code detected! 🐛");
    }
    
    // Calculate base vibe score
    sus vibe_score = 0;
    
    // Coffee contributes to alertness (diminishing returns)
    lowkey (coffee_cups > 0) {
        vibe_score += (coffee_cups * 10) - (coffee_cups * coffee_cups / 2);
    }
    
    // Memes boost social vibes
    vibe_score += memes_shared * 5;
    
    // Code lines show productivity
    vibe_score += code_lines / 10;
    
    // Bonus for balanced lifestyle
    lowkey (coffee_cups <= 5 && memes_shared >= 1 && code_lines >= 50) {
        vibe_score += 50;  // Balance bonus
        spill("{} gets the balance bonus! ✨", name);
    }
    
    periodt Ok(vibe_score);
}

fr fr/ Determine vibe level category based on score
fr fr/ 
fr fr/ @param score The numerical vibe score
fr fr/ @return String description of the vibe level
slay function get_vibe_level(score: i32) -> string {
    lowkey (score >= 200) {
        periodt "Absolutely iconic! 👑";
    } highkey (score >= 150) {
        periodt "Major vibes! ✨";
    } highkey (score >= 100) {
        periodt "Good energy! 😊";
    } highkey (score >= 50) {
        periodt "Decent vibes 👍";
    } highkey {
        periodt "Need more coffee... ☕";
    }
}

fr fr/ Process a team's vibe scores
fr fr/ 
fr fr/ This function demonstrates iteration, error handling, and string manipulation.
fr fr/ 
fr fr/ @param team_members Array of team member names
fr fr/ @return Summary of team vibes
slay function process_team_vibes(team_members: [string]) -> Result<string, string> {
    sus total_score = 0;
    sus processed_count = 0;
    sus vibe_summary = "🏢 Team Vibe Report\n==================\n\n";
    
    // Process each team member with error handling
    bestie (sus i = 0; i < team_members.length(); i++) {
        facts member = team_members[i];
        
        // Simulate different activity levels for demo
        facts coffee = (i % 4) + 1;        // 1-4 cups
        facts memes = (i * 3) % 20;        // 0-19 memes
        facts lines = ((i + 1) * 50) % 300; // Variable code output
        
        // Calculate vibe with error handling
        lowkey (facts score_result = calculate_vibe(member, coffee, memes, lines)) {
            match score_result {
                Ok(score) => {
                    total_score += score;
                    processed_count += 1;
                    
                    facts level = get_vibe_level(score);
                    vibe_summary += "• {} ({}☕ {}😂 {}💻): {} points - {}\n"
                        .format(member, coffee, memes, lines, score, level);
                }
                Err(error) => {
                    vibe_summary += "• {}: Error - {}\n".format(member, error);
                }
            }
        }
    }
    
    // Calculate team average
    lowkey (processed_count > 0) {
        facts average = total_score / processed_count;
        vibe_summary += "\n📊 Team Average: {} points\n".format(average);
        vibe_summary += "🎯 Overall Team Vibe: {}\n".format(get_vibe_level(average));
        
        // Team performance analysis
        lowkey (average >= 150) {
            vibe_summary += "🚀 This team is absolutely crushing it!\n";
        } highkey (average >= 100) {
            vibe_summary += "💪 Solid team performance!\n";
        } highkey {
            vibe_summary += "📈 Room for improvement - maybe more coffee?\n";
        }
    } highkey {
        vibe_summary += "\n❌ No valid team members processed!\n";
    }
    
    periodt Ok(vibe_summary);
}

fr fr/ Demonstrate various CURSED language constructs
slay function demonstrate_language_features() {
    spill("🎉 Welcome to CURSED Language Basic Syntax Demo!\n");
    
    // Variable declarations
    sus dynamic_value = "This can change";
    facts constant_value = "This is immutable";
    
    spill("Variables declared:");
    spill("  sus (mutable): {}", dynamic_value);
    spill("  facts (immutable): {}", constant_value);
    
    // Modify mutable variable
    dynamic_value = "See? It changed!";
    spill("  After modification: {}\n", dynamic_value);
    
    // Demonstrate control flow
    spill("🔄 Control Flow Examples:");
    
    // Conditional statements
    facts test_number = 42;
    lowkey (test_number > 0) {
        spill("  ✅ {} is positive (using lowkey/highkey)", test_number);
    } highkey {
        spill("  ❌ {} is not positive", test_number);
    }
    
    // Loop examples
    spill("\n🔁 Loop Examples:");
    spill("  Bestie loop (for loop):");
    bestie (sus i = 1; i <= 3; i++) {
        spill("    Iteration {}: Hello from CURSED!", i);
    }
    
    spill("\n  Flex loop (periodt loop):");
    sus counter = 0;
    flex (counter < 3) {
        counter += 1;
        spill("    Flex iteration {}", counter);
    }
    
    // Array/collection operations
    spill("\n📋 Collection Operations:");
    facts numbers = [1, 2, 3, 4, 5];
    sus sum = 0;
    
    bestie (sus i = 0; i < numbers.length(); i++) {
        sum += numbers[i];
    }
    
    spill("  Array: {:?}", numbers);
    spill("  Sum: {}", sum);
    
    // String operations
    spill("\n🔤 String Operations:");
    facts greeting = "Hello";
    facts target = "CURSED";
    facts message = "{}, {}! 💅✨".format(greeting, target);
    spill("  Formatted: {}", message);
    spill("  Length: {}", message.length());
    spill("  Uppercase: {}", message.to_uppercase());
}

fr fr/ Demo error handling and validation
slay function demonstrate_error_handling() {
    spill("\n🚨 Error Handling Demo:");
    
    // Array of test data (some invalid)
    facts test_data = [
        ("Alice", 2, 10, 150),
        ("Bob", -1, 5, 100),    // Invalid: negative coffee
        ("Charlie", 3, -2, 200), // Invalid: negative memes  
        ("Diana", 4, 15, 250),
    ];
    
    bestie (sus i = 0; i < test_data.length(); i++) {
        facts (name, coffee, memes, lines) = test_data[i];
        
        // Using the ? operator for error propagation
        match calculate_vibe(name, coffee, memes, lines) {
            Ok(score) => {
                spill("  ✅ {}: {} points - {}", name, score, get_vibe_level(score));
            }
            Err(error) => {
                spill("  ❌ {}: {}", name, error);
            }
        }
    }
}

fr fr/ Main function demonstrating the complete syntax
slay function main() -> Result<(), string> {
    // Basic feature demonstration
    demonstrate_language_features();
    
    // Error handling demonstration
    demonstrate_error_handling();
    
    // Team processing example
    spill("\n👥 Team Processing Example:");
    facts team = [
        "Alex", "Jordan", "Casey", "Morgan", "Riley",
        "Sage", "Quinn", "Avery", "Blake", "Cameron"
    ];
    
    // Process team with comprehensive error handling
    facts report = process_team_vibes(team)?;
    spill(report);
    
    // Mathematical operations demo
    spill("🔢 Mathematical Operations:");
    facts a = 15;
    facts b = 7;
    
    spill("  {} + {} = {}", a, b, a + b);
    spill("  {} - {} = {}", a, b, a - b);
    spill("  {} * {} = {}", a, b, a * b);
    spill("  {} / {} = {}", a, b, a / b);
    spill("  {} % {} = {}", a, b, a % b);
    spill("  {} ^ {} = {}", a, 2, a.pow(2));
    
    // Type demonstrations
    spill("\n🏷️  Type System Demo:");
    facts integer_val: i32 = 42;
    facts float_val: f64 = 3.14159;
    facts string_val: string = "CURSED rocks!";
    facts boolean_val: bool = based;
    
    spill("  Integer (i32): {}", integer_val);
    spill("  Float (f64): {:.3}", float_val);
    spill("  String: {}", string_val);
    spill("  Boolean: {}", boolean_val);
    
    spill("\n🎊 Basic syntax demo completed successfully!");
    spill("💡 CURSED proves that programming can be both fun AND powerful!");
    
    periodt Ok(());
}

fr fr This demonstrates that CURSED supports traditional syntax alongside Gen Z slang
/* 
   Multi-line comments work too!
   CURSED combines:
   - Expressive Gen Z syntax that's memorable and engaging
   - Powerful type system with safety guarantees  
   - Modern features like error handling and pattern matching
   - Performance through LLVM compilation
   - Comprehensive standard library
*/

fr fr/ Entry point with error handling
slay function run() {
    match main() {
        Ok(()) => {
            spill("✨ Program completed successfully!");
        }
        Err(error) => {
            spill("💥 Program failed: {}", error);
            std::process::exit(1);
        }
    }
}
