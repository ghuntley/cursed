use cursed::lexer::Lexer;

fn main() {
    println!("Testing simple lexer...");
    let source = "slay hello() { yolo 42; }";
    println!("Source: {}", source);
    
    let mut lexer = Lexer::new(source.to_string());
    match lexer.tokenize() {
        Ok(tokens) => {
            for token in tokens {
                println!("Token: {:?} -> '{}'", token.kind, token.lexeme);
            }
        }
        Err(e) => {
            println!("Lexer error: {:?}", e);
        }
    }
}
