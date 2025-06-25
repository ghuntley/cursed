/// Integration test for the minimal CURSED build
/// This test only uses the available minimal modules

use cursed::{tokenize, parse, Lexer, Parser, Error};

#[test]
fn test_minimal_tokenization() {
    let source = r#"facts x = 42;"#;
    
    let tokens = tokenize(source).expect("Tokenization should succeed");
    
    // Should have: facts, x, =, 42, ;
    assert_eq!(tokens.len(), 5);
}

#[test]
fn test_minimal_parsing() {
    let source = r#"facts name = "CURSED";"#;
    
    let program = parse(source).expect("Parsing should succeed");
    
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_function_parsing() {
    let source = r#"slay greet(name) { facts x = 1; }"#;
    
    let program = parse(source).expect("Parsing should succeed");
    
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_direct_lexer_usage() {
    let source = r#"facts test = "working";"#;
    let lexer = Lexer::new(source.to_string());
    let tokens: Vec<_> = lexer.collect();
    
    assert!(!tokens.is_empty());
    assert_eq!(tokens[0].literal, "facts");
}

#[test]
fn test_direct_parser_usage() {
    let source = r#"facts working = 42;"#;
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer).expect("Parser creation should succeed");
    let program = parser.parse_program().expect("Parsing should succeed");
    
    assert_eq!(program.statements.len(), 1);
}
