use cursed::ast;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;

#[test]
fn test_parse_generic_struct() {
    let input = r#"vibe test

be_like Box[T] squad {
    value T
}
"#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();

    // Should have a package declaration and a struct declaration
    assert_eq!(
        program.statements.len(),
        2,
        "Expected 2 statements, got {}",
        program.statements.len()
    );

    // Check the struct declaration
    if let Some(squad_stmt) = program.statements[1]
        .as_any()
        .downcast_ref::<ast::SquadStatement>()
    {
        // Check struct name
        assert_eq!(squad_stmt.name.value, "Box", "Struct name should be 'Box'");

        // Check type parameters
        assert_eq!(
            squad_stmt.type_parameters.len(),
            1,
            "Should have 1 type parameter"
        );
        assert_eq!(
            squad_stmt.type_parameters[0].value, "T",
            "Type parameter should be 'T'"
        );

        // Check fields
        assert_eq!(squad_stmt.fields.len(), 1, "Should have 1 field");
        assert_eq!(
            squad_stmt.fields[0].name.value, "value",
            "Field name should be 'value'"
        );
        assert_eq!(
            squad_stmt.fields[0].type_name.value, "T",
            "Field type should be 'T'"
        );
    } else {
        panic!("Second statement is not a SquadStatement");
    }
}

#[test]
#[ignore = "Generic parsing tests need further work"]
fn test_parse_generic_function() {
    let input = r#"vibe test

slay foo[T](x normie) T {
    yolo x
}
"#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();

    match parser.parse_program() {
        Ok(program) => {
            // Should have a package declaration and a function declaration
            assert_eq!(
                program.statements.len(),
                2,
                "Expected 2 statements, got {}",
                program.statements.len()
            );

            // Check the function declaration
            if let Some(expr_stmt) = program.statements[1]
                .as_any()
                .downcast_ref::<ast::ExpressionStatement>()
            {
                if let Some(expr) = &expr_stmt.expression {
                    if let Some(assign_expr) =
                        expr.as_any().downcast_ref::<ast::AssignmentExpression>()
                    {
                        // Check function name
                        assert_eq!(
                            assign_expr.name.value, "foo",
                            "Function name should be 'foo'"
                        );

                        // Check that the value is a function literal
                        if let Some(func_lit) = assign_expr
                            .value
                            .as_any()
                            .downcast_ref::<ast::declarations::FunctionStatement>(
                        ) {
                            // Check type parameters
                            assert_eq!(
                                func_lit.type_parameters.len(),
                                1,
                                "Should have 1 type parameter"
                            );
                            assert_eq!(
                                func_lit.type_parameters[0].value, "T",
                                "Type parameter should be 'T'"
                            );

                            // Check parameters
                            assert_eq!(func_lit.parameters.len(), 1, "Should have 1 parameter");
                            assert_eq!(
                                func_lit.parameters[0].name.value, "x",
                                "Parameter should be 'x"
                            );

                            // Check return type
                            assert!(func_lit.return_type.is_some(), "Should have a return type");
                            if let Some(ret_type) = &func_lit.return_type {
                                // The return type expression structure has changed in the modularized AST
                                // We can't directly compare value anymore, so we'll use string() instead
                                assert!(
                                    ret_type.string().contains("T"),
                                    "Return type should be 'T"
                                );
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
        Err(e) => {
            println!("Parse error: {}", e);
            // Print the token being processed at the time of error
            if let Error::Parser { location, message } = &e {
                println!(
                    "Error at line {}, column {}: {}",
                    location.line, location.column, message
                );

                // Print some context from the input around the error location
                let lines: Vec<&str> = input.lines().collect();
                if location.line as usize <= lines.len() {
                    let line = lines[location.line as usize - 1];
                    println!("Line {}: {}", location.line, line);
                    // Print a caret under the error position
                    let spaces = (0..location.column - 1).map(|_| " ").collect::<String>();
                    println!("{spaces}^");
                }
            }
            panic!("Failed to parse input");
        }
    }
}

#[test]
#[ignore = "Generic parsing tests need further work"]
fn test_parse_generic_instantiation() {
    let input = r#"vibe test

sus box_int = Box[normie]{value: 42}
"#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();

    // Should have a package declaration and a let statement
    assert_eq!(
        program.statements.len(),
        2,
        "Expected 2 statements, got {}",
        program.statements.len()
    );

    // Check the let statement
    if let Some(let_stmt) = program.statements[1]
        .as_any()
        .downcast_ref::<ast::LetStatement>()
    {
        // Check variable name
        assert_eq!(
            let_stmt.name.value, "box_int",
            "Variable name should be 'box_int'"
        );

        // Check the value is a BeLikeExpression
        if let Some(expr) = &let_stmt.value {
            if let Some(be_like_expr) = expr.as_any().downcast_ref::<ast::BeLikeExpression>() {
                // Check struct name
                assert_eq!(
                    be_like_expr.struct_name.value, "Box",
                    "Struct name should be 'Box'"
                );

                // Check type arguments
                assert_eq!(
                    be_like_expr.type_arguments.len(),
                    1,
                    "Should have 1 type argument"
                );

                // Check fields
                assert_eq!(be_like_expr.fields.len(), 1, "Should have 1 field");
                assert_eq!(
                    be_like_expr.fields[0].0, "value",
                    "Field name should be 'value'"
                );
            } else {
                panic!("Value is not a BeLikeExpression");
            }
        } else {
            panic!("LetStatement has no value");
        }
    } else {
        panic!("Second statement is not a LetStatement");
    }
}

#[test]
fn test_parse_generic_function_call() {
    let input = r#"vibe test

sus result = identity[normie](42)
"#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();

    // Should have a package declaration and a let statement
    assert_eq!(
        program.statements.len(),
        2,
        "Expected 2 statements, got {}",
        program.statements.len()
    );

    // Check the let statement
    if let Some(let_stmt) = program.statements[1]
        .as_any()
        .downcast_ref::<ast::LetStatement>()
    {
        // Check variable name
        assert_eq!(
            let_stmt.name.value, "result",
            "Variable name should be 'result'"
        );

        // Check the value is a GenericCallExpression
        if let Some(expr) = &let_stmt.value {
            if let Some(call_expr) = expr.as_any().downcast_ref::<ast::GenericCallExpression>() {
                // Check function is an identifier
                if let Some(func_ident) = call_expr
                    .function
                    .as_any()
                    .downcast_ref::<ast::Identifier>()
                {
                    assert_eq!(
                        func_ident.value, "identity",
                        "Function name should be 'identity'"
                    );
                } else {
                    panic!("Function is not an identifier");
                }

                // Check arguments
                assert_eq!(call_expr.arguments.len(), 1, "Should have 1 argument");

                // Check type arguments
                assert_eq!(
                    call_expr.type_arguments.len(),
                    1,
                    "Should have 1 type argument"
                );

                // Check that the type argument is 'normie'
                if let Some(type_arg) = call_expr.type_arguments[0]
                    .as_any()
                    .downcast_ref::<ast::Identifier>()
                {
                    assert_eq!(type_arg.value, "normie", "Type argument should be 'normie'");
                } else {
                    panic!("Type argument is not an identifier");
                }
            } else {
                panic!("Value is not a GenericCallExpression");
            }
        } else {
            panic!("LetStatement has no value");
        }
    } else {
        panic!("Second statement is not a LetStatement");
    }
}
