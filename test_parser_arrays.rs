// Simple test to verify array type parsing
use std::io::Write;

mod ast;
mod parser;
mod lexer;
mod error;

fn main() {
    let test_cases = vec![
        "sus numbers []normie = [1, 2, 3]",
        "sus items [5]tea = [\"hello\", \"world\"]", 
        "sus matrix [][]normie = [[1, 2], [3, 4]]",
    ];
    
    for test_case in test_cases {
        println!("Testing: {}", test_case);
        
        match test_parse(test_case) {
            Ok(_) => println!("✓ Parsed successfully"),
            Err(e) => println!("✗ Parse error: {}", e),
        }
        println!();
    }
}

fn test_parse(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut lexer = lexer::Lexer::new(source.to_string());
    let tokens = lexer.tokenize()?;
    
    let mut parser = parser::Parser::new(tokens);
    let _program = parser.parse()?;
    
    Ok(())
}
