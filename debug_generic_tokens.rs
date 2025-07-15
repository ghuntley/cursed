use cursed::lexer::Lexer;

fn main() {
    let source = "slay max<T>(a T, b T) -> T { damn a }";
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.tokenize().unwrap();
    
    for token in tokens {
        println!("{:?}", token);
    }
}
