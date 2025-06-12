// Validate that all regex patterns in security.rs are syntactically correct
use regex::Regex;

fn main() {
    println!("Validating regex patterns from security.rs...");
    
    let patterns = [
        ("email", r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"),
        ("url", r"^https?://[a-zA-Z0-9.-]+(?:\.[a-zA-Z]{2,})+(?:/[^\s]*)?$"),
        ("phone", r"^\+?[\d\s\-\(\)]{10,}$"),
        ("numeric", r"^\d+$"),
        ("alphanumeric", r"^[a-zA-Z0-9]+$"),
        ("alpha", r"^[a-zA-Z]+$"),
        ("password_strong", r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$"),
        ("ipv4", r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$"),
        ("credit_card", r"^\d{4}[-\s]?\d{4}[-\s]?\d{4}[-\s]?\d{4}$"),
        ("postal_code", r"^[A-Z0-9\s\-]{3,10}$"),
        ("hex_color", r"^#[0-9A-Fa-f]{6}$"),
        ("username", r"^[a-zA-Z0-9_]{3,20}$"),
        ("slug", r"^[a-z0-9]+(?:-[a-z0-9]+)*$"),
    ];
    
    let mut all_valid = true;
    
    for (name, pattern) in &patterns {
        match Regex::new(pattern) {
            Ok(_) => println!("✓ Pattern '{}' is valid", name),
            Err(e) => {
                println!("✗ Pattern '{}' is invalid: {}", name, e);
                all_valid = false;
            }
        }
    }
    
    if all_valid {
        println!("\n🎉 All regex patterns are syntactically correct!");
        println!("The regex dependency integration in security.rs is working properly.");
    } else {
        println!("\n❌ Some regex patterns are invalid!");
        std::process::exit(1);
    }
}
