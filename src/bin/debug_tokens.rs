use cursed::{Lexer, TokenKind};

fn main() {
    let code = r#"lowkey isAwesome {
        vibez.spill("This language is based!")
    } highkey {
        vibez.spill("Something is sus...")
    }"#;
    
    let mut lexer = Lexer::new(code.to_string());
    let tokens = lexer.tokenize().expect("Tokenization failed");
    
    println!("Tokens generated:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?} = '{}'", i, token.kind, token.lexeme);
    }
}
