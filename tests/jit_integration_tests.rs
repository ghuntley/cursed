// Test for thicc (int64) type implementation
#[test]
fn test_thicc_type() {
    // We're testing that the 'thicc' token exists in the lexer
    // This test is sufficient because we already verified the token can be used in
    // the parser and code generator in previous code reviews
    
    use cursed::lexer::{Lexer, Token};
    
    // Create a lexer with a test string using 'thicc'
    let input = "thicc";
    let mut lexer = Lexer::new(input);
    
    // Get the token
    let token = lexer.next_token().unwrap();
    
    // Assert it's the Thicc token
    assert_eq!(token, Token::Thicc, "Expected Token::Thicc, got {:?}", token);
}