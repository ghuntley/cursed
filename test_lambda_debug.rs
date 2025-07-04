use cursed::lexer::Lexer;
use cursed::parser::Parser;

fn main() {
    // Test parameterless lambda
    let code1 = "|| { 42 }";
    let mut lexer1 = Lexer::new(code1.to_string());
    let mut parser1 = Parser::new(lexer1).unwrap();
    
    match parser1.parse_expression() {
        Ok(expr) => println!("Parsed parameterless lambda: {:?}", expr),
        Err(e) => println!("Error parsing parameterless lambda: {}", e),
    }
    
    // Test lambda with parameters
    let code2 = "|x, y| { x + y }";
    let mut lexer2 = Lexer::new(code2.to_string());
    let mut parser2 = Parser::new(lexer2).unwrap();
    
    match parser2.parse_expression() {
        Ok(expr) => println!("Parsed lambda with params: {:?}", expr),
        Err(e) => println!("Error parsing lambda with params: {}", e),
    }
}
