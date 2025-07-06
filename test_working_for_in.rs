use cursed::lexer::{Lexer, TokenKind};
use cursed::parser::Parser;
use cursed::ast::Statement;

fn main() {
    let input = "bestie person in mylist { }";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize().unwrap();
    
    println!("Tokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  #{} {:?} '{}'", i, token.kind, token.lexeme);
    }
    
    let mut parser = Parser::from_tokens(tokens);
    match parser.parse_for_statement() {
        Ok(Statement::ForIn(for_in)) => {
            println!("✅ Successfully parsed for-in loop!");
            println!("   Variable: {}", for_in.variable);
            println!("   Body statements: {}", for_in.body.len());
        },
        Ok(other) => {
            println!("❌ Got different statement type: {:?}", std::mem::discriminant(&other));
        },
        Err(e) => {
            println!("❌ Parse error: {}", e);
        }
    }
}
