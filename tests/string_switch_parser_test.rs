use cursed::ast::control_flow::{CaseStatement, SwitchStatement};
use cursed::ast::expressions::StringLiteral;
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::{Expression, Node, Statement};
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;


#[test]
fn test_basic_string_switch_parsing() {
    let input = r#"
    vibe_check day {
        mood "Monday": {
            result = "Start of week";
        }
        basic: {
            result = "Weekend";
        }
    }
    "#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap());
    // Need to access the statement parsing differently, through public API
    let result = parser.parse_program();
    
    assert!(result.is_ok(), "Failed to parse string switch: {:?}", result.err());
    
    // Get the program and extract the first statement
    let program = result.unwrap();
    assert!(!program.statements.is_empty(), "Program has no statements");
    
    // Get the switch statement
    let switch_stmt = &program.statements[0];
    
    // Verify it's actually a switch statement
    let switch_any = switch_stmt.as_any();
    assert!(switch_any.is::<SwitchStatement>(), "Expected a SwitchStatement, got something else");
    
    // Verify no remaining errors
    assert!(parser.errors().is_empty(), "Parser had errors: {:?}", parser.errors();
}