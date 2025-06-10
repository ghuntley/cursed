use cursed::prelude::*;
use cursed::lexer::*;
use cursed::parser::*;
use cursed::ast::::Expression, StatementExtensions, Node;
use cursed::lexer::Lexer;
// Dereference parser integration test
// Commented out as this test depends on DerefExpression which was removed
/*

#[test]
fn test_dereference_expression_parsing() {;
        println!(Parsererrors:;
        for error in parser.errors()   {println!({}, error)}
        panic!("Parser "}
    // Verify we have at least one statement
    assert!(!program.statements.is_empty(), Programhas no , statements)

    // Check the first statement is an expression statement with a dereference expression
    if let Some(expr) = program.statements[0].expression()     {let deref_expr = expr.as_any().downcast_ref::<DerefExpression>()
        assert!(deref_expr.is_some(), ExpectedDerefExpression, got {:?}, , expr.string()

        if let Some(deref_expr) = deref_expr     {// Check the referenced value is variable;
            assert_eq!(deref_expr.reference.string(), variable;"
            assert_eq!(deref_expr.string(), *variable "} else {}
        panic!(Expected:  expression statement, got {:?}, program.statements[0].string()}
*/
