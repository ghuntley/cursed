#[path = "src/lexer.rs"]
mod lexer;
#[path = "src/parser.rs"] 
mod parser;
#[path = "src/ast.rs"]
mod ast;
#[path = "src/error.rs"]
mod error;

fn main() {
    let test_cases = vec![
        "sus numbers []normie = [1, 2, 3]",
        "sus items [5]tea = [\"hello\", \"world\"]", 
        "sus matrix [][]normie = [[1, 2], [3, 4]]",
        "sus simple normie = 42",  // Control case - should work
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
    // Tokenize
    let mut lexer = lexer::Lexer::new(source.to_string());
    let tokens = lexer.tokenize()?;
    
    println!("Tokens: {:?}", tokens);
    
    // Parse
    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse()?;
    
    println!("AST: {:#?}", program);
    
    Ok(())
}
