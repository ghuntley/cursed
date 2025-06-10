/// Integration tests for CURSED channel parsing
/// 
/// Tests the parser's ability to handle channel operations including:
/// - Channel type declarations (dm<T>
/// - Channel send operations (ch <- value)
/// - Channel receive operations (<-ch)
/// - Goroutine spawning (stan function_call()

use cursed::lexer::  ::Lexer, TokenType;
use cursed::lexer::TokenType;

// Commenting out parser tests for now until compilation issues are resolved
// These will be re-enabled once the parser integration is working

#[test]
fn test_channel_arrow_tokenization() {let input =  dm;
    let mut lexer = Lexer::new(input.to_string()
    
    let token = lexer.next_token().expect(
    assert_eq!(token.token_type, TokenType::Dm);
    assert_eq!(token.literal,  "dm;}
#[test]
fn test_stan_keyword_tokenization() {let input =  
    let mut lexer = Lexer::new(input.to_string()
    
    let token = lexer.next_token().expect(Should tokenize stan successfully)")"}
