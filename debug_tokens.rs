use cursed::lexer::Lexer;

fn main() {
    let input = "vibez.spill".to_string();
    let mut lexer = Lexer::new(input);
    
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
