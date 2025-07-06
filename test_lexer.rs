use cursed::lexer::Lexer;

fn main() {
    let input = "bestie person in people";
    let mut lexer = Lexer::new(input.to_string());
    
    match lexer.tokenize() {
        Ok(tokens) => {
            for (i, token) in tokens.iter().enumerate() {
                println!("#{} {:?} '{}' at line {}, col {}", 
                    i + 1, token.kind, token.lexeme, token.line, token.column);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
