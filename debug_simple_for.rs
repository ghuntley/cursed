use cursed::lexer::Lexer;
use cursed::parser::Parser;

fn main() {
    let code = "bestie i := 0; i < 5; i++ { vibez.spill(\"hi\") }";
    
    println!("Parsing: {}", code);
    
    let mut lexer = Lexer::new(code.to_string());
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token().unwrap();
        tokens.push(token.clone());
        if matches!(token.kind, cursed::lexer::TokenKind::Eof) {
            break;
        }
    }
    
    println!("Tokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?}", i, token);
    }
    
    let mut parser = Parser::new(tokens);
    match parser.parse_for_statement() {
        Ok(stmt) => {
            println!("SUCCESS: {:?}", stmt);
        }
        Err(e) => {
            println!("ERROR: {}", e);
        }
    }
}
