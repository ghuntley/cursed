use std::fs;

fn main() {
    let code = "sus counter normie = 0\nperiodt (counter < 3) {\n    counter = counter + 1\n}";
    
    // Tokenize
    let mut lexer = cursed::lexer::Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    
    println!("Tokens:");
    for token in tokens {
        println!("{:?}", token);
    }
}
