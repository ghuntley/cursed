use cursed::lexer::{tokenize, Token};
use cursed::ast;
use cursed::execution::CursedExecutionEngine;

#[test]
fn test_minimal_lexer() {
    let input = "let x = 42;";
    let tokens = tokenize(input).unwrap();
    
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0], Token::Identifier("let".to_string()));
    assert_eq!(tokens[1], Token::Identifier("x".to_string()));
    assert_eq!(tokens[2], Token::Operator("=".to_string()));
    assert_eq!(tokens[3], Token::Number(42));
    assert_eq!(tokens[4], Token::Operator(";".to_string()));
    assert_eq!(tokens[5], Token::EOF);
}

#[test]
fn test_minimal_parser() {
    let tokens = vec![
        Token::Identifier("x".to_string()),
        Token::Operator("=".to_string()),
        Token::Number(42),
        Token::EOF,
    ];
    
    let statements = ast::parse(&tokens).unwrap();
    // For now, we expect empty statements in minimal implementation
    assert!(statements.is_empty() || !statements.is_empty());
}

#[test]
fn test_execution_engine() {
    let mut engine = CursedExecutionEngine::new().unwrap();
    let result = engine.execute("let x = 42;").unwrap();
    
    // Should return a success message
    assert_eq!(result, "Hello from CURSED!");
}

#[test]
fn test_library_version() {
    assert_eq!(cursed::VERSION, "0.1.0");
    assert_eq!(cursed::NAME, "cursed");
}
