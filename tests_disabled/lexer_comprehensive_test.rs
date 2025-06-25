use cursed::lexer::{Lexer, TokenType};
use cursed::error::Error;

#[test]
fn test_cursed_line_comments() {
    let input = r#"
fr fr This is a line comment
facts x = 42
fr fr Another comment
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    
    // Should have: Newline, Comment, Newline, Facts, Identifier, Assign, Integer, Newline, Comment, Newline, EOF
    assert_eq!(tokens.len(), 11);
    assert_eq!(tokens[1].token_type, TokenType::Comment);
    assert_eq!(tokens[1].literal, "fr fr This is a line comment");
    assert_eq!(tokens[9].token_type, TokenType::Comment);
    assert_eq!(tokens[9].literal, "fr fr Another comment");
}

#[test]
fn test_cursed_block_comments() {
    let input = r#"
no cap This is a block comment
spanning multiple lines
on god
facts x = 42
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    
    // Find the comment token
    let comment_token = tokens.iter().find(|t| t.token_type == TokenType::Comment).unwrap();
    assert!(comment_token.literal.starts_with("no cap"));
    assert!(comment_token.literal.ends_with("on god"));
    assert!(comment_token.literal.contains("spanning multiple lines"));
}

#[test]
fn test_hexadecimal_numbers() {
    let input = "0x1A 0X2B 0xff";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens[0].token_type, TokenType::Integer);
    assert_eq!(tokens[0].literal, "0x1A");
    assert_eq!(tokens[1].token_type, TokenType::Integer);
    assert_eq!(tokens[1].literal, "0X2B");
    assert_eq!(tokens[2].token_type, TokenType::Integer);
    assert_eq!(tokens[2].literal, "0xff");
}

#[test]
fn test_octal_numbers() {
    let input = "0o177 0O755";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens[0].token_type, TokenType::Integer);
    assert_eq!(tokens[0].literal, "0o177");
    assert_eq!(tokens[1].token_type, TokenType::Integer);
    assert_eq!(tokens[1].literal, "0O755");
}

#[test]
fn test_binary_numbers() {
    let input = "0b1010 0B1100";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens[0].token_type, TokenType::Integer);
    assert_eq!(tokens[0].literal, "0b1010");
    assert_eq!(tokens[1].token_type, TokenType::Integer);
    assert_eq!(tokens[1].literal, "0B1100");
}

#[test]
fn test_later_keyword() {
    let input = "later cleanup()";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens[0].token_type, TokenType::Later);
    assert_eq!(tokens[0].literal, "later");
}

#[test]
fn test_boolean_literals_disambiguation() {
    let input = r#"
sus x = cap     fr fr x is mutable and true
facts y = no_cap fr fr y is immutable and false
nil             fr fr nil value
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    
    // Find the relevant tokens
    let sus_token = tokens.iter().find(|t| t.token_type == TokenType::Sus).unwrap();
    assert_eq!(sus_token.literal, "sus");
    
    let cap_token = tokens.iter().find(|t| t.token_type == TokenType::Cap).unwrap();
    assert_eq!(cap_token.literal, "cap");
    
    let no_cap_token = tokens.iter().find(|t| t.token_type == TokenType::NoCap).unwrap();
    assert_eq!(no_cap_token.literal, "no_cap");
    
    let nil_token = tokens.iter().find(|t| t.token_type == TokenType::Nil).unwrap();
    assert_eq!(nil_token.literal, "nil");
}

#[test]
fn test_mixed_number_formats() {
    let input = "42 3.14 0xFF 0o777 0b1010";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens[0].token_type, TokenType::Integer);
    assert_eq!(tokens[0].literal, "42");
    
    assert_eq!(tokens[1].token_type, TokenType::Float);
    assert_eq!(tokens[1].literal, "3.14");
    
    assert_eq!(tokens[2].token_type, TokenType::Integer);
    assert_eq!(tokens[2].literal, "0xFF");
    
    assert_eq!(tokens[3].token_type, TokenType::Integer);
    assert_eq!(tokens[3].literal, "0o777");
    
    assert_eq!(tokens[4].token_type, TokenType::Integer);
    assert_eq!(tokens[4].literal, "0b1010");
}

#[test]
fn test_complex_program_with_new_features() {
    let input = r#"
fr fr This is a complete CURSED program with new features
no cap
Program demonstrating all new lexer features:
- Comments with fr fr and no cap/on god
- Number formats: hex, octal, binary
- Boolean disambiguation
- Later keyword for defer
on god

slay main_character() {
    sus hex_val = 0xFF
    facts oct_val = 0o777
    sus bin_val = 0b1010
    
    fr fr Traditional booleans still work
    facts is_true = true
    sus is_false = false
    
    fr fr But now we have CURSED booleans
    facts cursed_true = cap
    sus cursed_false = no_cap
    
    fr fr And nil values
    sus nothing = nil
    
    fr fr Defer with later
    later cleanup()
    
    yolo 0
}
"#;
    
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize();
    assert!(result.is_ok());
    
    let tokens = result.unwrap();
    
    // Verify we have the expected token types
    let token_types: Vec<TokenType> = tokens.iter().map(|t| t.token_type.clone()).collect();
    
    assert!(token_types.contains(&TokenType::Comment));
    assert!(token_types.contains(&TokenType::Later));
    assert!(token_types.contains(&TokenType::Cap));
    assert!(token_types.contains(&TokenType::NoCap));
    assert!(token_types.contains(&TokenType::Nil));
    assert!(token_types.contains(&TokenType::Sus));
    assert!(token_types.contains(&TokenType::Facts));
    
    // Verify hex, octal, and binary numbers are properly tokenized
    let hex_tokens: Vec<&str> = tokens.iter()
        .filter(|t| t.token_type == TokenType::Integer && t.literal.starts_with("0x"))
        .map(|t| t.literal.as_str())
        .collect();
    assert!(hex_tokens.contains(&"0xFF"));
    
    let oct_tokens: Vec<&str> = tokens.iter()
        .filter(|t| t.token_type == TokenType::Integer && t.literal.starts_with("0o"))
        .map(|t| t.literal.as_str())
        .collect();
    assert!(oct_tokens.contains(&"0o777"));
    
    let bin_tokens: Vec<&str> = tokens.iter()
        .filter(|t| t.token_type == TokenType::Integer && t.literal.starts_with("0b"))
        .map(|t| t.literal.as_str())
        .collect();
    assert!(bin_tokens.contains(&"0b1010"));
}

#[test]
fn test_comment_edge_cases() {
    // Test line comment at end of file
    let input = "facts x = 42 fr fr comment";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let comment_token = tokens.iter().find(|t| t.token_type == TokenType::Comment).unwrap();
    assert_eq!(comment_token.literal, "fr fr comment");
    
    // Test block comment without proper termination (should read to end of file)
    let input = "no cap unterminated comment";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let comment_token = tokens.iter().find(|t| t.token_type == TokenType::Comment).unwrap();
    assert!(comment_token.literal.starts_with("no cap"));
}

#[test]
fn test_backward_compatibility() {
    // Ensure traditional syntax still works
    let input = r#"
function main() {
    let x = true;
    let y = false;
    return 0;
}
"#;
    
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize();
    assert!(result.is_ok());
    
    let tokens = result.unwrap();
    let boolean_tokens: Vec<&str> = tokens.iter()
        .filter(|t| t.token_type == TokenType::Boolean)
        .map(|t| t.literal.as_str())
        .collect();
    
    assert!(boolean_tokens.contains(&"true"));
    assert!(boolean_tokens.contains(&"false"));
}
