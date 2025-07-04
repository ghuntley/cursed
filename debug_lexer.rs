use cursed::lexer::Lexer;

fn main() {
    let source = r#"slay main() {
    yolo
}"#;
    
    let mut lexer = Lexer::new(source.to_string());
    match lexer.tokenize() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
