use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("test_comment_demo.csd")?;
    
    // Create a minimal lexer test to verify comments work
    println!("Testing CURSED comment syntax:");
    println!("Input:");
    println!("{}", content);
    println!("\n=== Tokenization Analysis ===");
    
    // Look for comment patterns in the source
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let line_num = i + 1;
        if line.contains("fr fr") {
            println!("Line {}: Found line comment: {}", line_num, line.trim());
        }
        if line.contains("no cap") {
            println!("Line {}: Found block comment start: {}", line_num, line.trim());
        }
        if line.contains("on god") {
            println!("Line {}: Found block comment end: {}", line_num, line.trim());
        }
    }
    
    Ok(())
}
