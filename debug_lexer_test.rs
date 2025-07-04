use cursed::lexer::Lexer;

fn main() {
    let source = "based lies lowkey";
    let mut lexer = Lexer::new(source.to_string());
    
    while let Ok(token) = lexer.next_token() {
        if token.kind == cursed::lexer::TokenKind::Eof {
            break;
        }
        println!("Token: {:?} - '{}'", token.kind, token.lexeme);
    }
}
