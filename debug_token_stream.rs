use cursed::Lexer;

fn main() {
    let demo_content = include_str!("../demo_cursed_hello.csd");
    
    let mut lexer = Lexer::new(demo_content.to_string());
    let tokens = lexer.tokenize().expect("Tokenization failed");
    
    println!("All tokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  Token {}: {:?} = '{}'", i, token.kind, token.lexeme);
    }
}
