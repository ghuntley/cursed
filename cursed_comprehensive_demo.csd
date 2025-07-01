// ============================================================================
// CURSED Programming Language - Comprehensive Demo
// Showcasing Gen-Z slang syntax mixed with traditional constructs
// ============================================================================

// Package declaration
vibe cursed_demo

// Import statements
yeet "vibecheck"
yeet "mathz"

// Global constants with Gen-Z keywords
facts app_name = "CURSED Demo Program"
facts version = "1.0.0"
facts answer_to_everything = 42

// Traditional and Gen-Z function definitions side by side
slay greet_user(name) {
    facts welcome_msg = "Welcome to CURSED!"
    yolo welcome_msg + " " + name
}

fn calculate_fibonacci(n) {
    lowkey (n <= 1) {
        yolo n
    }
    highkey {
        yolo calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
    }
}

// Demonstrating both traditional and Gen-Z syntax
slay showcase_language_features() {
    // Variable declarations
    facts message = "CURSED language"
    sus counter = 0
    
    // Control flow with Gen-Z keywords
    lowkey (counter < 5) {
        counter = counter + 1
        yolo "Counting: " + counter
    }
    
    // Traditional if-else
    if (counter == 5) {
        return "Traditional syntax works too!"
    } else {
        return "Something went wrong"
    }
}

// Main function - entry point
slay main() {
    // Print application info
    yolo app_name
    yolo "Version: " + version
    
    // Demonstrate greeting
    facts greeting = greet_user("Gen Z Developer")
    yolo greeting
    
    // Mathematical demonstration
    facts fib_result = calculate_fibonacci(8)
    yolo "Fibonacci of 8: " + fib_result
    
    // Language features showcase
    facts feature_demo = showcase_language_features()
    yolo feature_demo
    
    // Mixed syntax demonstration
    facts math_result = answer_to_everything * 2
    yolo "Answer doubled: " + math_result
    
    // Boolean demonstration with Gen-Z keywords
    facts is_awesome = truth
    facts is_basic = lies
    
    lowkey (is_awesome) {
        yolo "CURSED is absolutely slay! 🔥"
    }
    
    lowkey (!is_basic) {
        yolo "This language is no cap amazing! 💯"
    }
    
    yolo "Demo completed successfully!"
}
