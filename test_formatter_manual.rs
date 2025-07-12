use cursed::formatter::SimpleCursedFormatter;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let formatter = SimpleCursedFormatter::default();
    
    // Test complex formatting
    let complex_source = fs::read_to_string("test_formatter_complex.csd")?;
    println!("=== COMPLEX FORMATTER TEST ===");
    println!("Original:");
    println!("{}", complex_source);
    println!("\nFormatted:");
    let formatted = formatter.format(&complex_source)?;
    println!("{}", formatted);
    
    // Test edge cases
    let edge_source = fs::read_to_string("test_formatter_edge_cases.csd")?;
    println!("\n=== EDGE CASES FORMATTER TEST ===");
    println!("Original:");
    println!("{}", edge_source);
    println!("\nFormatted:");
    let formatted_edge = formatter.format(&edge_source)?;
    println!("{}", formatted_edge);
    
    // Test deterministic output
    println!("\n=== DETERMINISTIC TEST ===");
    let formatted_again = formatter.format(&formatted)?;
    if formatted == formatted_again {
        println!("✅ Formatter output is deterministic!");
    } else {
        println!("❌ Formatter output is NOT deterministic!");
        println!("First format:\n{}", formatted);
        println!("Second format:\n{}", formatted_again);
    }
    
    Ok(())
}
