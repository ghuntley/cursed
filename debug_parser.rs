extern crate cursed;
use cursed::lexer::Lexer;

fn main() {
    let input = r#"vibe test

slay identity[T](x) {
    yolo x
}
"#;
    
    let mut lexer = Lexer::new(input);
    
    // Print all tokens
    loop {
        let token = lexer.next_token().unwrap();
        println!("{:?}", token);
        
        if token.token_literal() == "EOF" {
            break;
        }
    }
}