use cursed::lexer::Lexer;
use cursed::parser::Parser;

#[test]
fn test_function_parameters_parsing() {
    // Test basic function with no parameters
    let input = r#"vibe test

slay empty() {
    yolo 42
}
"#;
    
    println!("Testing input:\n{}", input);
    let mut lexer = Lexer::new(input);
    
    // Collect all tokens for debugging
    let mut tokens = Vec::new();
    loop {
        match lexer.next_token() {
            Ok(token) => {
                let is_eof = token == cursed::lexer::Token::Eof;
                tokens.push(token.clone());
                if is_eof { break; }
            },
            Err(e) => {
                println!("Lexer error: {:?}", e);
                break;
            }
        }
    }
    
    println!("Tokens: {:?}", tokens);
    
    // Start over with a new lexer
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program();
    
    assert!(program.is_ok(), "Failed to parse basic function: {}", program.err().unwrap());
    
    // Test function with simple parameters (no type annotations)
    let input = r#"vibe test

slay add(x, y) {
    yolo x + y
}
"#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program();
    
    assert!(program.is_ok(), "Failed to parse function with parameters: {}", program.err().unwrap());
    
    // Test function with type annotations
    let input = r#"vibe test

slay add(x normie, y normie) normie {
    yolo x + y
}
"#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program();
    
    assert!(program.is_ok(), "Failed to parse function with type annotations: {}", program.err().unwrap());
    
    // Test function with generic type parameter
    let input = r#"vibe test

slay identity[T](x T) T {
    yolo x
}
"#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program();
    
    assert!(program.is_ok(), "Failed to parse generic function: {}", program.err().unwrap());
    
    // Test function with multiple generic type parameters
    let input = r#"vibe test

slay map[K, V](key K) V {
    yolo lookup(key)
}
"#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program();
    
    assert!(program.is_ok(), "Failed to parse function with multiple generic parameters: {}", program.err().unwrap());
    
    // Test complex generic function
    let input = r#"vibe test

slay convert[T, U](source T, converter stan(T) U) U {
    yolo converter(source)
}
"#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program();
    
    assert!(program.is_ok(), "Failed to parse complex generic function: {}", program.err().unwrap());
}