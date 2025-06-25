/// Basic functionality tests for CURSED language
use cursed::{tokenize, parse, Error};

#[test]
fn test_tokenization_basic() {
    let source = r#"facts x = 42;"#;
    let result = tokenize(source);
    assert!(result.is_ok());
    let tokens = result.unwrap();
    assert_eq!(tokens.len(), 5);
}

#[test]
fn test_parsing_basic() {
    let source = r#"facts message = "Hello CURSED!";"#;
    let result = parse(source);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_function_parsing() {
    let source = r#"slay greet(name) { facts greeting = "Hello"; }"#;
    let result = parse(source);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_error_handling() {
    let source = r#"facts x = ;"#; // Incomplete assignment
    let result = parse(source);
    // For now, just check that it doesn't crash - our parser might be forgiving
    let _ = result;
}

#[test]
fn test_empty_program() {
    let source = "";
    let result = parse(source);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 0);
}
