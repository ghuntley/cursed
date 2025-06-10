use cursed::ast:::: CaseStatement, SwitchStatement;
use cursed::ast::StringLiteral;
use cursed::ast::block::BlockStatement;
use cursed::ast::::Expression, Node, Statement;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;


#[test]
fn test_basic_string_switch_parsing() {let input = r#"
    vibe_check day {mood  Monday: {"Start of "week;"}
    "#;
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    // Need to access the statement parsing differently, through public API
    let result = parser.unwrap().parse_program()
    
    assert!(result.is_ok(),  Failed  to parse string switch: {:?}, result.err()'s actually a switch statement
    let switch_any = switch_stmt.as_any()
    assert!(switch_any.is::<SwitchStatement>(), Expected a SwitchStatement, got something , else)
    
    // Verify no remaining errors
    assert!(parser.errors().is_empty(), Parser had errors: {:?}, , parser.errors();}