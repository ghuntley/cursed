use cursed::{Lexer, Parser};

fn main() {
    let code = r#"slay demonstrateBasics() {
    sus radius snack = 5.0
    sus userName tea = "Developer"
    sus isAwesome lit = based
    
    lowkey isAwesome {
        vibez.spill("This language is based!")
    } highkey {
        vibez.spill("Something is sus...")
    }
}"#;
    
    let mut lexer = Lexer::new(code.to_string());
    let tokens = lexer.tokenize().expect("Tokenization failed");
    
    println!("Tokens generated:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?} = '{}'", i, token.kind, token.lexeme);
    }
    
    let mut parser = Parser::from_tokens(tokens);
    match parser.parse() {
        Ok(ast) => println!("Parsing succeeded!"),
        Err(e) => println!("Parsing failed: {:?}", e),
    }
}
