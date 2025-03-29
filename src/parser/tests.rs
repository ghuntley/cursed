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