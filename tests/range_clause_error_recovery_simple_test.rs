use cursed::parser::{Parser, RangeClauseErrorRecoverySimple};
use cursed::lexer::Lexer;
use cursed::error::Error;
use std::sync::Arc;

// Simple tests for range clause error recovery


#[test]
fn test_range_error_recovery_simple() {
    // Test a simple range clause with error recovery
    let mut lexer = Lexer::new("flex 10");
    let mut parser = Parser::new(&mut lexer).unwrap();
    
    // Implement the RangeClauseErrorRecoverySimple trait for Parser
    let range_clause = parser.parse_range_clause_with_recovery_simple();
    
    // Should recover successfully
    assert!(range_clause.is_ok());
    
    // Test with invalid syntax
    let mut lexer = Lexer::new("flex "); // Missing value after flex
    let mut parser = Parser::new(&mut lexer).unwrap();
    
    // Should recover and return a fallback range clause
    let range_clause = parser.parse_range_clause_with_recovery_simple();
    assert!(range_clause.is_ok());
    
    // The fallback should be a range from 0 to 0
    let range = range_clause.unwrap();
    assert_eq!(range.is_container, false);
}