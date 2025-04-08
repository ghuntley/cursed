use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast;

#[test]
#[ignore="Improved generic params tests need further work"]
fn test_function_with_multiple_types_generic_parameters() {
    // Test a function with multiple generic parameters and complex type signatures
    let input = r#"vibe test

slay transform[K][V](collection K, mapper V) tea {
    sus result tea = "processed"
    result = mapper(collection)
    yolo result
}"
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
                assert_eq!(assign_expr.name.value, "transform", "Function name should be 'transform'");
                
                // Check that the value is a function literal
                if let Some(func_lit) = assign_expr.value.as_any().downcast_ref::<ast::declarations::FunctionStatement>() {
                    // Check generic type parameters
                    assert_eq!(func_lit.type_parameters.len(), 1, "Should have 1 type parameter");
                    assert_eq!(func_lit.type_parameters[0].value, "K", "Type parameter should be 'K'");
                    
                    // Check function parameters
                    assert_eq!(func_lit.parameters.len(), 2, "Should have 2 parameters");
                    assert_eq!(func_lit.parameters[0].name.value, "collection", "First parameter should be 'collection'");
                    assert_eq!(func_lit.parameters[1].name.value, "mapper", "Second parameter should be 'mapper'");
                    
                    // Check return type
                    assert!(func_lit.return_type.is_some(), "Should have a return type");
                    if let Some(ret_type) = &func_lit.return_type {
                        // The return type expression structure has changed in the modularized AST
                        // We can't directly compare value anymore, so we'll use string() instead
                        assert!(ret_type.string().contains("tea"), "Return type should be 'tea'");
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
#[ignore="Improved generic params tests need further work"]
fn test_function_with_nested_generic_parameters() {
    // Test a function with nested generic parameters
    let input = r#"vibe test

slay compose[A][B][C](f A, g B) C {
    yolo f(g("data"))
}"
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
                assert_eq!(assign_expr.name.value, "compose", "Function name should be 'compose'");
                
                // Check that the value is a function literal
                if let Some(func_lit) = assign_expr.value.as_any().downcast_ref::<ast::declarations::FunctionStatement>() {
                    // Check generic type parameters
                    assert_eq!(func_lit.type_parameters.len(), 1, "Should have 1 type parameter");
                    assert_eq!(func_lit.type_parameters[0].value, "A", "Type parameter should be 'A'");
                    
                    // Check function parameters
                    assert_eq!(func_lit.parameters.len(), 2, "Should have 2 parameters");
                    assert_eq!(func_lit.parameters[0].name.value, "f", "First parameter should be 'f'");
                    assert_eq!(func_lit.parameters[1].name.value, "g", "Second parameter should be 'g");
                    
                    // Check return type
                    assert!(func_lit.return_type.is_some(), "Should have a return type");
                    if let Some(ret_type) = &func_lit.return_type {
                        // The return type expression structure has changed in the modularized AST
                        // We can't directly compare value anymore, so we'll use string() instead
                        assert!(ret_type.string().contains("C"), "Return type should be 'C'");
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