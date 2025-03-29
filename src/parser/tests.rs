#[test]
fn test_parse_let_statements() -> Result<(), Error> {
    let inputs = vec![
        ("sus x = 5;", "x", 5i64),
        ("sus y = 10", "y", 10i64),
        ("sus foobar = 838383;", "foobar", 838383i64),
    ];

    for (input, expected_identifier, expected_value) in inputs {
        let program = test_parser_with_input(input)?;
        
        assert_eq!(program.statements.len(), 1, "Program does not contain 1 statement. Got={}", program.statements.len());
        
        let stmt = &program.statements[0];
        if let Some(let_stmt) = stmt.as_any().downcast_ref::<ast::LetStatement>() {
            assert_eq!(let_stmt.token_literal(), "sus");
            assert_eq!(let_stmt.name.value, expected_identifier);
            
            if let Some(value_expr) = &let_stmt.value {
                if let Some(int_literal) = value_expr.as_any().downcast_ref::<ast::IntegerLiteral>() {
                    assert_eq!(int_literal.value, expected_value);
                } else {
                    panic!("Value is not an IntegerLiteral");
                }
            } else {
                panic!("Let statement value is None");
            }
        } else {
            panic!("Statement is not a LetStatement");
        }
    }

    Ok(())
}

// Helper enum to represent expected literal values in tests
enum ExpectedLiteral {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

#[test]
fn test_parse_facts_statements() -> Result<(), Error> {
    let inputs = vec![
        ("facts PI = 3.14159;", "PI", ExpectedLiteral::Float(3.14159)),
        ("facts GREETING = \"Hello\";", "GREETING", ExpectedLiteral::String("Hello".to_string())),
        ("facts COUNT = 100;", "COUNT", ExpectedLiteral::Integer(100)),
        ("facts IS_REAL = based;", "IS_REAL", ExpectedLiteral::Boolean(true)),
    ];

    for (input, expected_identifier, expected_literal) in inputs {
        let program = test_parser_with_input(input)?;
        
        assert_eq!(program.statements.len(), 1, "Program for input '{}' does not contain 1 statement. Got={}", input, program.statements.len());
        
        let stmt = &program.statements[0];
        if let Some(facts_stmt) = stmt.as_any().downcast_ref::<ast::FactsStatement>() {
            assert_eq!(facts_stmt.token_literal(), "facts", "Incorrect token literal for input: {}", input);
            assert_eq!(facts_stmt.name.value, expected_identifier, "Incorrect identifier name for input: {}", input);
            
            match expected_literal {
                ExpectedLiteral::Integer(expected_value) => {
                    if let Some(int_literal) = facts_stmt.value.as_any().downcast_ref::<ast::IntegerLiteral>() {
                        assert_eq!(int_literal.value, expected_value, "Integer value mismatch for input: {}", input);
                    } else {
                        panic!("Value is not an IntegerLiteral for input: {}", input);
                    }
                },
                ExpectedLiteral::Float(expected_value) => {
                    if let Some(float_literal) = facts_stmt.value.as_any().downcast_ref::<ast::FloatLiteral>() {
                        assert!((float_literal.value - expected_value).abs() < f64::EPSILON, "Float value mismatch for input: {}. Expected {}, got {}", input, expected_value, float_literal.value);
                    } else {
                        panic!("Value is not a FloatLiteral for input: {}", input);
                    }
                },
                ExpectedLiteral::String(expected_value) => {
                    if let Some(string_literal) = facts_stmt.value.as_any().downcast_ref::<ast::StringLiteral>() {
                        assert_eq!(string_literal.value, expected_value, "String value mismatch for input: {}", input);
                    } else {
                        panic!("Value is not a StringLiteral for input: {}", input);
                    }
                },
                ExpectedLiteral::Boolean(expected_value) => {
                    if let Some(bool_literal) = facts_stmt.value.as_any().downcast_ref::<ast::BooleanLiteral>() {
                        assert_eq!(bool_literal.value, expected_value, "Boolean value mismatch for input: {}", input);
                    } else {
                        panic!("Value is not a BooleanLiteral for input: {}", input);
                    }
                },
            }
        } else {
            panic!("Statement is not a FactsStatement for input: {}", input);
        }
    }

    Ok(())
}

#[test]
fn test_parse_sus_statements() -> Result<(), Error> {
    let inputs = vec![
        ("sus x = 5;", "x", ExpectedLiteral::Integer(5)),
        ("sus y = based;", "y", ExpectedLiteral::Boolean(true)),
        ("sus foobar = cap;", "foobar", ExpectedLiteral::Boolean(false)), // Assuming 'cap' parses to false
        ("sus message = \"hello\";", "message", ExpectedLiteral::String("hello".to_string())),
    ];

    for (input, expected_identifier, expected_literal) in inputs {
        let program = test_parser_with_input(input)?;
        
        assert_eq!(program.statements.len(), 1, "Program for input '{}' does not contain 1 statement. Got={}", input, program.statements.len());
        
        let stmt = &program.statements[0];
        if let Some(let_stmt) = stmt.as_any().downcast_ref::<ast::LetStatement>() {
            assert_eq!(let_stmt.token_literal(), "sus", "Incorrect token literal for input: {}", input);
            assert_eq!(let_stmt.name.value, expected_identifier, "Incorrect identifier name for input: {}", input);
            
            let value_expr = let_stmt.value.as_ref().expect(&format!("Let statement value is None for input: {}", input));

            match expected_literal {
                ExpectedLiteral::Integer(expected_value) => {
                    if let Some(int_literal) = value_expr.as_any().downcast_ref::<ast::IntegerLiteral>() {
                        assert_eq!(int_literal.value, expected_value, "Integer value mismatch for input: {}", input);
                    } else {
                        panic!("Value is not an IntegerLiteral for input: {}", input);
                    }
                },
                ExpectedLiteral::Float(expected_value) => {
                    if let Some(float_literal) = value_expr.as_any().downcast_ref::<ast::FloatLiteral>() {
                        assert!((float_literal.value - expected_value).abs() < f64::EPSILON, "Float value mismatch for input: {}. Expected {}, got {}", input, expected_value, float_literal.value);
                    } else {
                        panic!("Value is not a FloatLiteral for input: {}", input);
                    }
                },
                ExpectedLiteral::String(expected_value) => {
                    if let Some(string_literal) = value_expr.as_any().downcast_ref::<ast::StringLiteral>() {
                        assert_eq!(string_literal.value, expected_value, "String value mismatch for input: {}", input);
                    } else {
                        panic!("Value is not a StringLiteral for input: {}", input);
                    }
                },
                ExpectedLiteral::Boolean(expected_value) => {
                    if let Some(bool_literal) = value_expr.as_any().downcast_ref::<ast::BooleanLiteral>() {
                        assert_eq!(bool_literal.value, expected_value, "Boolean value mismatch for input: {}", input);
                    } else {
                        panic!("Value is not a BooleanLiteral for input: {}", input);
                    }
                },
            }
        } else {
            panic!("Statement is not a LetStatement for input: {}", input);
        }
    }

    Ok(())
}

#[test]
fn test_parse_return_statements() -> Result<(), Error> {
    let inputs = vec![
        ("yolo 5;", ExpectedLiteral::Integer(5)),
        ("yolo based;", ExpectedLiteral::Boolean(true)),
        ("yolo foobar;", ExpectedLiteral::String("foobar".to_string())), // Treat identifier as string for this test case
        ("yolo \"hello\";", ExpectedLiteral::String("hello".to_string())),
        // ("yolo;", ExpectedLiteral::None), // Need to adapt ExpectedLiteral or add a new way to test None
    ];

    for (input, expected_literal) in inputs {
        let program = test_parser_with_input(input)?;
        
        assert_eq!(program.statements.len(), 1, "Program for input '{}' does not contain 1 statement. Got={}", input, program.statements.len());
        
        let stmt = &program.statements[0];
        if let Some(return_stmt) = stmt.as_any().downcast_ref::<ast::ReturnStatement>() {
            assert_eq!(return_stmt.token_literal(), "yolo", "Incorrect token literal for input: {}", input);
            
            let return_value_expr = return_stmt.return_value.as_ref().expect(&format!("Return statement value is None for input: {}", input));

            match expected_literal {
                ExpectedLiteral::Integer(expected_value) => {
                    if let Some(int_literal) = return_value_expr.as_any().downcast_ref::<ast::IntegerLiteral>() {
                        assert_eq!(int_literal.value, expected_value, "Integer value mismatch for input: {}", input);
                    } else {
                        panic!("Return value is not an IntegerLiteral for input: {}", input);
                    }
                },
                ExpectedLiteral::Float(expected_value) => {
                     if let Some(float_literal) = return_value_expr.as_any().downcast_ref::<ast::FloatLiteral>() {
                         assert!((float_literal.value - expected_value).abs() < f64::EPSILON, "Float value mismatch for input: {}. Expected {}, got {}", input, expected_value, float_literal.value);
                     } else {
                         panic!("Return value is not a FloatLiteral for input: {}", input);
                     }
                 },
                ExpectedLiteral::String(expected_value) => {
                    // Handle both Identifier and StringLiteral for this test
                    if let Some(string_literal) = return_value_expr.as_any().downcast_ref::<ast::StringLiteral>() {
                        assert_eq!(string_literal.value, expected_value, "String value mismatch for input: {}", input);
                    } else if let Some(identifier) = return_value_expr.as_any().downcast_ref::<ast::Identifier>() {
                        assert_eq!(identifier.value, expected_value, "Identifier value mismatch for input: {}", input);
                    } else {
                        panic!("Return value is not a StringLiteral or Identifier for input: {}", input);
                    }
                },
                ExpectedLiteral::Boolean(expected_value) => {
                    if let Some(bool_literal) = return_value_expr.as_any().downcast_ref::<ast::BooleanLiteral>() {
                        assert_eq!(bool_literal.value, expected_value, "Boolean value mismatch for input: {}", input);
                    } else {
                        panic!("Return value is not a BooleanLiteral for input: {}", input);
                    }
                },
            }
        } else {
            panic!("Statement is not a ReturnStatement for input: {}", input);
        }
    }
    
    // Test case for 'yolo;' (no return value)
    let program_no_value = test_parser_with_input("yolo;")?;
    assert_eq!(program_no_value.statements.len(), 1, "Program for 'yolo;' does not contain 1 statement.");
    let stmt_no_value = &program_no_value.statements[0];
    if let Some(return_stmt_no_value) = stmt_no_value.as_any().downcast_ref::<ast::ReturnStatement>() {
        assert_eq!(return_stmt_no_value.token_literal(), "yolo", "Incorrect token literal for 'yolo;'");
        assert!(return_stmt_no_value.return_value.is_none(), "Return value should be None for 'yolo;'");
    } else {
        panic!("Statement is not a ReturnStatement for 'yolo;'");
    }

    Ok(())
}

#[test]
fn test_parse_if_statements() -> Result<(), Error> {
    // Test case 1: Simple if statement
    let input1 = "lowkey (x < y) { yolo x; }";
    let program1 = test_parser_with_input(input1)?;
    assert_eq!(program1.statements.len(), 1, "Program 1 failed: incorrect statement count");
    if let Some(if_stmt) = program1.statements[0].as_any().downcast_ref::<ast::IfStatement>() {
        assert_eq!(if_stmt.token_literal(), "lowkey");
        // Check condition (simplified check)
        assert!(if_stmt.condition.string().contains("x"));
        assert!(if_stmt.condition.string().contains("<"));
        assert!(if_stmt.condition.string().contains("y"));
        // Check consequence
        assert_eq!(if_stmt.consequence.statements.len(), 1, "Consequence block in program 1 should have 1 statement");
        assert!(if_stmt.consequence.statements[0].as_any().is::<ast::ReturnStatement>(), "Consequence statement in program 1 is not ReturnStatement");
        // Check alternative (should be None)
        assert!(if_stmt.alternative.is_none(), "Alternative block in program 1 should be None");
    } else {
        panic!("Program 1 is not an IfStatement");
    }

    // Test case 2: If-else statement
    let input2 = "lowkey (x > y) { yolo x; } highkey { yolo y; }";
    let program2 = test_parser_with_input(input2)?;
    assert_eq!(program2.statements.len(), 1, "Program 2 failed: incorrect statement count");
    if let Some(if_stmt) = program2.statements[0].as_any().downcast_ref::<ast::IfStatement>() {
        assert_eq!(if_stmt.token_literal(), "lowkey");
        // Check condition
        assert!(if_stmt.condition.string().contains("x > y"));
        // Check consequence
        assert_eq!(if_stmt.consequence.statements.len(), 1, "Consequence block in program 2 should have 1 statement");
        // Check alternative (should exist)
        assert!(if_stmt.alternative.is_some(), "Alternative block in program 2 should exist");
        if let Some(alt_block) = &if_stmt.alternative {
            assert_eq!(alt_block.statements.len(), 1, "Alternative block in program 2 should have 1 statement");
            assert!(alt_block.statements[0].as_any().is::<ast::ReturnStatement>(), "Alternative statement in program 2 is not ReturnStatement");
        } else {
            panic!("Alternative block expected but not found in program 2");
        }
    } else {
        panic!("Program 2 is not an IfStatement");
    }
    
    // Test case 3: If statement with multiple statements in consequence
    let input3 = "lowkey (based) { sus a = 1; yolo a; }";
    let program3 = test_parser_with_input(input3)?;
    assert_eq!(program3.statements.len(), 1, "Program 3 failed: incorrect statement count");
    if let Some(if_stmt) = program3.statements[0].as_any().downcast_ref::<ast::IfStatement>() {
        assert_eq!(if_stmt.consequence.statements.len(), 2, "Consequence block in program 3 should have 2 statements");
        assert!(if_stmt.consequence.statements[0].as_any().is::<ast::LetStatement>(), "First consequence statement in program 3 is not LetStatement");
        assert!(if_stmt.consequence.statements[1].as_any().is::<ast::ReturnStatement>(), "Second consequence statement in program 3 is not ReturnStatement");
        assert!(if_stmt.alternative.is_none(), "Alternative block in program 3 should be None");
    } else {
        panic!("Program 3 is not an IfStatement");
    }

    Ok(())
} 