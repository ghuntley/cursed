use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast;

#[test]
fn test_function_with_generic_parameters() {
    // Test a function with generic parameters
    let input = r#"vibe test

slay add[T](x T, y T) T {
    yolo x + y
}
"#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    // Should have a package declaration and a function declaration
    assert_eq!(program.statements.len(), 2, "Expected 2 statements, got {}", program.statements.len());
    
    // Check that the second statement is a function declaration
    if let Some(expr_stmt) = program.statements[1].as_any().downcast_ref::<ast::ExpressionStatement>() {
        if let Some(expr) = &expr_stmt.expression {
            if let Some(assign_expr) = expr.as_any().downcast_ref::<ast::AssignmentExpression>() {
                // Check function name
                assert_eq!(assign_expr.name.value, "add", "Function name should be 'add'");
                
                // Check that the value is a function literal
                if let Some(func_lit) = assign_expr.value.as_any().downcast_ref::<ast::FunctionLiteral>() {
                    // Check generic type parameters
                    assert_eq!(func_lit.type_parameters.len(), 1, "Should have 1 type parameter");
                    assert_eq!(func_lit.type_parameters[0].value, "T", "Type parameter should be 'T'");
                    
                    // Check function parameters
                    assert_eq!(func_lit.parameters.len(), 2, "Should have 2 parameters");
                    assert_eq!(func_lit.parameters[0].value, "x", "First parameter should be 'x'");
                    assert_eq!(func_lit.parameters[1].value, "y", "Second parameter should be 'y'");
                    
                    // Check return type
                    assert!(func_lit.return_type.is_some(), "Should have a return type");
                    if let Some(ret_type) = &func_lit.return_type {
                        assert_eq!(ret_type.value, "T", "Return type should be 'T'");
                    }
                } else {
                    panic!("Value is not a function literal");
                }
            } else {
                panic!("Expression is not an assignment expression");
            }
        } else {
            panic!("ExpressionStatement has no expression");
        }
    } else {
        panic!("Second statement is not an ExpressionStatement");
    }
}

#[test]
fn test_function_with_multiple_generic_parameters() {
    // Test a function with multiple generic parameters
    let input = r#"vibe test

slay pair[A, B](first A, second B) {
    yolo first
}
"#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    // Should have a package declaration and a function declaration
    assert_eq!(program.statements.len(), 2, "Expected 2 statements, got {}", program.statements.len());
    
    // Check that the second statement is a function declaration
    if let Some(expr_stmt) = program.statements[1].as_any().downcast_ref::<ast::ExpressionStatement>() {
        if let Some(expr) = &expr_stmt.expression {
            if let Some(assign_expr) = expr.as_any().downcast_ref::<ast::AssignmentExpression>() {
                // Check function name
                assert_eq!(assign_expr.name.value, "pair", "Function name should be 'pair'");
                
                // Check that the value is a function literal
                if let Some(func_lit) = assign_expr.value.as_any().downcast_ref::<ast::FunctionLiteral>() {
                    // Check generic type parameters
                    assert_eq!(func_lit.type_parameters.len(), 2, "Should have 2 type parameters");
                    assert_eq!(func_lit.type_parameters[0].value, "A", "First type parameter should be 'A'");
                    assert_eq!(func_lit.type_parameters[1].value, "B", "Second type parameter should be 'B'");
                    
                    // Check function parameters
                    assert_eq!(func_lit.parameters.len(), 2, "Should have 2 parameters");
                    assert_eq!(func_lit.parameters[0].value, "first", "First parameter should be 'first'");
                    assert_eq!(func_lit.parameters[1].value, "second", "Second parameter should be 'second'");
                    
                    // No return type in this case
                    assert!(func_lit.return_type.is_none(), "Should not have a return type");
                } else {
                    panic!("Value is not a function literal");
                }
            } else {
                panic!("Expression is not an assignment expression");
            }
        } else {
            panic!("ExpressionStatement has no expression");
        }
    } else {
        panic!("Second statement is not an ExpressionStatement");
    }
}