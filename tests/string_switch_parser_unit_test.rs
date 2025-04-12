//! Unit tests for the string switch statement parsing functionality
//! This test checks that the parser can correctly parse switch statements with string cases

use cursed::ast::control_flow::{CaseStatement, SwitchStatement};
use cursed::ast::{Expression, Node, Statement};
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;

/// Test parsing a simple switch statement with string literals as case values
#[test]
fn test_simple_switch_parse() -> Result<(), Error> {
    let input = r#"
    vibe_check day {
        mood "Monday": yolo "Start of the week";
        mood "Friday": yolo "End of the week";
        basic: yolo "Some other day";
    }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;
    
    assert_eq!(
        parser.errors().len(),
        0,
        "Parser had errors: {:?}",
        parser.errors()
    );
    
    assert_eq!(
        program.statements.len(),
        1,
        "Program should have exactly one statement (the switch)"
    );
    
    // Since we're testing structure recognition, we only need to check that the
    // statement is a SwitchStatement, not its exact contents
    let statement = &program.statements[0];
    let switch = statement.as_any().downcast_ref::<SwitchStatement>();
    
    assert!(switch.is_some(), "Expected a SwitchStatement, but got something else");
    
    if let Some(switch) = switch {
        assert_eq!(switch.cases.len(), 2, "Switch should have 2 cases");
        assert!(switch.default.is_some(), "Switch should have a default case");
    }
    
    Ok(())
}

/// Test parsing a switch statement with multiple expressions in a single case
#[test]
fn test_multi_value_case_parse() -> Result<(), Error> {
    let input = r#"
    vibe_check day {
        mood "Monday", "Tuesday", "Wednesday": yolo "Weekday";
        basic: yolo "Weekend";
    }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;
    
    assert_eq!(
        parser.errors().len(),
        0,
        "Parser had errors: {:?}",
        parser.errors()
    );
    
    let statement = &program.statements[0];
    let switch = statement.as_any().downcast_ref::<SwitchStatement>();
    
    assert!(switch.is_some(), "Expected a SwitchStatement");
    
    if let Some(switch) = switch {
        assert_eq!(switch.cases.len(), 1, "Switch should have 1 multi-value case");
        let case = &switch.cases[0];
        assert_eq!(case.expressions.len(), 3, "Case should have 3 expressions");
    }
    
    Ok(())
}

/// Test parsing a switch without a default case
#[test]
fn test_no_default_case_parse() -> Result<(), Error> {
    let input = r#"
    vibe_check day {
        mood "Monday": yolo "Monday vibes";
        mood "Friday": yolo "Friday vibes";
    }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;
    
    assert_eq!(
        parser.errors().len(),
        0,
        "Parser had errors: {:?}",
        parser.errors()
    );
    
    let statement = &program.statements[0];
    let switch = statement.as_any().downcast_ref::<SwitchStatement>();
    
    assert!(switch.is_some(), "Expected a SwitchStatement");
    
    if let Some(switch) = switch {
        assert_eq!(switch.cases.len(), 2, "Switch should have 2 cases");
        assert!(switch.default.is_none(), "Switch should not have a default case");
    }
    
    Ok(())
}