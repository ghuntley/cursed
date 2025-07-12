use cursed::{Lexer, Parser};

#[test]
fn test_demo_without_comments() {
    let demo_content = include_str!("../debug_without_comments.csd");
    
    println!("Testing demo file without comments...");
    
    // Step 1: Tokenize
    let mut lexer = Lexer::new(demo_content.to_string());
    let tokens = lexer.tokenize().expect("Tokenization failed");
    
    println!("Found {} tokens", tokens.len());
    
    // Step 2: Parse using the correct method
    let mut parser = Parser::from_tokens(tokens);
    let program = parser.parse_program().expect("Parsing failed");
    
    println!("Parsing successful! Found {} statements", program.statements.len());
}
