use cursed::{Lexer, Parser};

fn main() {
    let demo_content = r#"
slay demonstrateBasics() {
    sus radius snack = 5.0
    sus userName tea = "Developer"
    sus isAwesome lit = based  
    
    sus area = calculateArea(radius)
    greetUser(userName)
}
"#;
    
    println!("Testing demo file parsing...");
    
    // Step 1: Tokenize
    let mut lexer = Lexer::new(demo_content.to_string());
    let tokens = lexer.tokenize().expect("Tokenization failed");
    
    println!("Found {} tokens", tokens.len());
    
    // Step 2: Parse
    let mut parser = Parser::from_tokens(tokens);
    let ast = parser.parse().expect("Parsing failed");
    
    println!("Parsing successful!");
}
