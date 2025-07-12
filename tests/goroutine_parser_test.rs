use cursed::parser::{new_parser, Parser};
use cursed::ast::{Statement, GoroutineStatement, Expression, CallExpression, MemberAccessExpression, Literal};

#[test]
fn test_goroutine_statement_parsing() {
    // Test simple goroutine with function call
    let source = "stan worker()";
    let mut parser = new_parser(source).unwrap();
    let program = parser.parse_program().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Goroutine(goroutine_stmt) => {
            match &goroutine_stmt.expression {
                Expression::Call(call_expr) => {
                    // Check if the function is an identifier
                    match call_expr.function.as_ref() {
                        Expression::Identifier(name) => {
                            assert_eq!(name, "worker");
                        }
                        _ => panic!("Expected identifier for function name"),
                    }
                    assert_eq!(call_expr.arguments.len(), 0);
                }
                _ => panic!("Expected function call expression"),
            }
        }
        _ => panic!("Expected goroutine statement"),
    }
}

#[test]
fn test_goroutine_with_parameters() {
    // Test goroutine with function parameters
    let source = "stan worker(\"test\", 42)";
    let mut parser = new_parser(source).unwrap();
    let program = parser.parse_program().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Goroutine(goroutine_stmt) => {
            match &goroutine_stmt.expression {
                Expression::Call(call_expr) => {
                    // Check if the function is an identifier
                    match call_expr.function.as_ref() {
                        Expression::Identifier(name) => {
                            assert_eq!(name, "worker");
                        }
                        _ => panic!("Expected identifier for function name"),
                    }
                    assert_eq!(call_expr.arguments.len(), 2);
                }
                _ => panic!("Expected function call expression"),
            }
        }
        _ => panic!("Expected goroutine statement"),
    }
}

#[test]
fn test_goroutine_member_access() {
    // Test goroutine with member access
    let source = "stan vibez.spill(\"hello\")";
    let mut parser = new_parser(source).unwrap();
    let program = parser.parse_program().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Goroutine(goroutine_stmt) => {
            // The expression should be a member access call
            match &goroutine_stmt.expression {
                Expression::Call(call_expr) => {
                    // For member access calls, the function should be a member access expression
                    match call_expr.function.as_ref() {
                        Expression::MemberAccess(member_access) => {
                            assert_eq!(member_access.property, "spill");
                            match member_access.object.as_ref() {
                                Expression::Identifier(name) => {
                                    assert_eq!(name, "vibez");
                                }
                                _ => panic!("Expected identifier for object"),
                            }
                        }
                        _ => panic!("Expected member access for function"),
                    }
                }
                _ => panic!("Expected call expression for member access"),
            }
        }
        _ => panic!("Expected goroutine statement"),
    }
}

#[test]
fn test_multiple_goroutines() {
    // Test multiple goroutine statements
    let source = r#"
        stan worker1()
        stan worker2()
        stan processData(42)
    "#;
    let mut parser = new_parser(source).unwrap();
    let program = parser.parse_program().unwrap();
    
    assert_eq!(program.statements.len(), 3);
    
    for stmt in &program.statements {
        match stmt {
            Statement::Goroutine(_) => {
                // All statements should be goroutine statements
            }
            _ => panic!("Expected all statements to be goroutine statements"),
        }
    }
}
