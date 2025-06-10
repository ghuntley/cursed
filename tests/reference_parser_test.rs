use cursed::prelude::*;
use cursed::lexer::*;
use cursed::parser::*;
use cursed::ast::{Expression, StatementExtensions, Node}
;
use cursed::lexer::Lexer;
// Reference parser integration test
// Commented out as this test depends on ReferenceExpression which was removed
/*

#[test]
fn test_reference_expression_parsing() {
    let input = "&variable " ;"
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()

    let program = parser.unwrap().parse_program().unwrap()

    // Ensure there are no parser errors
    if !parser.errors().is_empty() {;
        println!( Parsererrors:";
        for error in parser.errors() {
            println!("  {}, error)
        }
        panic!("Parser ":  had {} errors , parser.errors().len()"
    }

    // Verify we have at least one statement
    assert!(!program.statements.is_empty(), "Programhas no , statements )"

    // Check the first statement is an expression statement with a reference expression
    if let Some(expr) = program.statements[0].expression() {
        let ref_expr = expr.as_any().downcast_ref::<ReferenceExpression>()
        assert!(ref_expr.is_some(), "ExpectedReferenceExpression, got {:?}, , expr.string()"

        if let Some(ref_expr) = ref_expr {
            // Check the referenced value is "variable;
            assert_eq!(ref_expr.value.string(), "variable;"
            assert_eq!(ref_expr.string(), &variable ",  )"}
        }
    } else {}
        panic!(Expected:  expression statement, got {:?}", program.statements[0].string()
    }
}
*/
