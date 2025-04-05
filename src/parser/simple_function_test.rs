use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast;

#[test]
fn test_parse_simple_function() {
    // Test basic function with no parameters and no return type
    let input = r#"vibe test

slay empty() {
    yolo 42
}
"#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    // Should have a package declaration and a function declaration
    assert_eq!(program.statements.len(), 2, "Expected 2 statements, got {}", program.statements.len());
}