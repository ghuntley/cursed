//! Test basic channel operations parsing and compilation

use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::parser::Precedence;
use cursed::ast::traits::Expression;

#[test]
fn test_channel_send_parsing() {:?}, , expr.err()
    
    let expr = expr.unwrap()
    let expr_str = expr.string()
    assert!(expr_str.contains("<-Expression should contain <- operator: {}, expr_str)"<-"ch;"Failedto parse receive call: {:?}, expr.err()
    let expr = expr.unwrap()
    let expr_str = expr.string()
    assert!(expr_str.starts_with(, <-Expression " should start with <-: {}, expr_str);
#[test]
fn test_channel_creation_parsing() {let input =  " [int];
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let expr = parser.parse_expression()
    assert!(expr.is_ok(), 
    
    let expr = expr.unwrap()
    let expr_str = expr.string()
    assert!(expr_str.contains("dm [Expression " should contain dm[: {}, expr_str);;
