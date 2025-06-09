use cursed::lexer::{Lexer, Token};
use cursed::parser::Parser;

fn main() {
    let input = "1, 2, 3}";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    
    println!("Initial current: {:?}", parser.current_token);
    println!("Initial peek: {:?}", parser.peek_token);
    
    // Parse first expression
    let expr1 = parser.parse_expression(cursed::parser::precedence::Precedence::Lowest).unwrap();
    println!("After parsing 1:");
    println!("  Current: {:?}", parser.current_token);
    println!("  Peek: {:?}", parser.peek_token);
    
    if parser.peek_token_is(Token::Comma) {
        parser.next_token().unwrap(); // Move to comma
        println!("  After moving to comma - Current: {:?}", parser.current_token);
        parser.next_token().unwrap(); // Skip comma
        println!("  After skipping comma - Current: {:?}", parser.current_token);
        println!("  Peek: {:?}", parser.peek_token);
    }
}
