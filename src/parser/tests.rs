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
        ("sus foobar = cap;", "foobar", ExpectedLiteral::Boolean(false)),
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

#[test]
fn test_parse_for_statements() {
    struct TestCase {
        input: &'static str,
        expected_init_type: Option<&'static str>, // "sus" or "facts" or expression
        expected_condition_type: Option<&'static str>, // expression
        expected_post_type: Option<&'static str>, // expression
        expected_body_stmts: usize,
    }

    let tests = vec![
        TestCase {
            input: "bestie { sus x = 1; }", // Infinite loop
            expected_init_type: None,
            expected_condition_type: None,
            expected_post_type: None,
            expected_body_stmts: 1,
        },
        TestCase {
            input: "bestie i < 10 { sus y = 2; facts z = 3; }", // Condition-only loop
            expected_init_type: None,
            expected_condition_type: Some("infix"),
            expected_post_type: None,
            expected_body_stmts: 2,
        },
        TestCase {
            input: "bestie sus i = 0; i < 10; i = i + 1 { sus a = i; }", // C-style loop
            expected_init_type: Some("sus"),
            expected_condition_type: Some("infix"),
            expected_post_type: Some("expression"),
            expected_body_stmts: 1,
        },
        TestCase {
            input: "bestie ; i < 5; { }", // C-style loop, no init or post
            expected_init_type: None,
            expected_condition_type: Some("infix"),
            expected_post_type: None,
            expected_body_stmts: 0,
        },
        TestCase {
            input: "bestie ; ; i = i + 1 { }", // C-style loop, no init or condition
            expected_init_type: None,
            expected_condition_type: None,
            expected_post_type: Some("expression"),
            expected_body_stmts: 0,
        },
         TestCase {
            input: "bestie facts i = 0; ; { }", // C-style loop, no condition or post
            expected_init_type: Some("facts"),
            expected_condition_type: None,
            expected_post_type: None,
            expected_body_stmts: 0,
        },
        TestCase {
            input: "bestie ; ; { }", // C-style loop, only body
            expected_init_type: None,
            expected_condition_type: None,
            expected_post_type: None,
            expected_body_stmts: 0,
        },
    ];

    for test in tests {
        let mut lexer = Lexer::new(test.input);
        let mut parser = Parser::new(&mut lexer).unwrap();
        let program = parser.parse_program().unwrap();
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1, "program should have 1 statement");

        let stmt = program.statements[0].as_any().downcast_ref::<ast::ForStatement>();
        assert!(stmt.is_some(), "Statement is not ForStatement");
        let for_stmt = stmt.unwrap();

        // Check init
        match (test.expected_init_type, &for_stmt.init) {
            (Some("sus"), Some(init_stmt)) => {
                assert!(init_stmt.as_any().is::<ast::LetStatement>());
            }
            (Some("facts"), Some(init_stmt)) => {
                assert!(init_stmt.as_any().is::<ast::FactsStatement>());
            }
            (Some("expression"), Some(init_stmt)) => {
                 // Note: Init can only be 'sus' or 'facts' currently
                 // If we allow expression statements later, this would check:
                 // assert!(init_stmt.as_any().is::<ast::ExpressionStatement>());
                 panic!("Unexpected expression init type check");
            }
             (None, None) => { /* Expected no init */ }
             (Some(_), None) => panic!("Expected init statement, got None"),
             (None, Some(_)) => panic!("Expected no init statement, got Some"),
             _ => panic!("Mismatch in init statement check logic"),
        }
        
        // Check condition
        match (test.expected_condition_type, &for_stmt.condition) {
            (Some("infix"), Some(cond_expr)) => {
                assert!(cond_expr.as_any().is::<ast::InfixExpression>());
            }
            (Some("identifier"), Some(cond_expr)) => {
                 assert!(cond_expr.as_any().is::<ast::Identifier>());
             }
             // Add other expected condition expression types if needed
             (None, None) => { /* Expected no condition */ }
             (Some(_), None) => panic!("Expected condition expression, got None"),
             (None, Some(_)) => panic!("Expected no condition expression, got Some"),
             _ => panic!("Mismatch in condition expression check logic"),
        }

        // Check post
        match (test.expected_post_type, &for_stmt.post) {
            (Some("expression"), Some(post_stmt)) => {
                assert!(post_stmt.as_any().is::<ast::ExpressionStatement>());
                // Further check the expression type within ExpressionStatement if needed
            }
            (None, None) => { /* Expected no post */ }
            (Some(_), None) => panic!("Expected post statement, got None"),
            (None, Some(_)) => panic!("Expected no post statement, got Some"),
             _ => panic!("Mismatch in post statement check logic"),
        }

        // Check body
        assert_eq!(for_stmt.body.statements.len(), test.expected_body_stmts);
    }
}

fn check_parser_errors(parser: &Parser) {
    // Implementation of check_parser_errors function
}

#[test]
fn test_parse_switch_statements() -> Result<(), Error> {
    // Test case 1: Basic switch with mood and basic
    let input1 = r#"
        vibe_check day {
            mood "Monday", "Tuesday":
                yolo 1;
            mood "Friday":
                yolo 2;
            basic:
                yolo 0;
        }
    "#;
    let program1 = test_parser_with_input(input1)?;
    assert_eq!(program1.statements.len(), 1, "Program 1 failed: incorrect statement count");
    if let Some(switch_stmt) = program1.statements[0].as_any().downcast_ref::<ast::SwitchStatement>() {
        assert_eq!(switch_stmt.token_literal(), "vibe_check");
        // Check value (should be identifier 'day')
        assert!(switch_stmt.value.as_any().is::<ast::Identifier>(), "Switch value is not Identifier");
        assert_eq!(switch_stmt.value.string(), "day");
        // Check cases
        assert_eq!(switch_stmt.cases.len(), 2, "Program 1 should have 2 mood cases");
        // Case 1: mood "Monday", "Tuesday"
        assert_eq!(switch_stmt.cases[0].expressions.len(), 2, "Case 1 should have 2 expressions");
        assert_eq!(switch_stmt.cases[0].expressions[0].string(), "\"Monday\"");
        assert_eq!(switch_stmt.cases[0].expressions[1].string(), "\"Tuesday\"");
        assert_eq!(switch_stmt.cases[0].body.statements.len(), 1, "Case 1 body should have 1 statement");
        // Case 2: mood "Friday"
        assert_eq!(switch_stmt.cases[1].expressions.len(), 1, "Case 2 should have 1 expression");
        assert_eq!(switch_stmt.cases[1].expressions[0].string(), "\"Friday\"");
        assert_eq!(switch_stmt.cases[1].body.statements.len(), 1, "Case 2 body should have 1 statement");
        // Check default
        assert!(switch_stmt.default.is_some(), "Program 1 should have a basic case");
        assert_eq!(switch_stmt.default.as_ref().unwrap().statements.len(), 1, "Basic case should have 1 statement");
    } else {
        panic!("Program 1 is not a SwitchStatement");
    }

    // Test case 2: Switch without basic case
    let input2 = r#"
        vibe_check status {
            mood 1:
                yolo "ok";
            mood 2:
                yolo "error";
        }
    "#;
    let program2 = test_parser_with_input(input2)?;
    assert_eq!(program2.statements.len(), 1, "Program 2 failed: incorrect statement count");
    if let Some(switch_stmt) = program2.statements[0].as_any().downcast_ref::<ast::SwitchStatement>() {
        assert_eq!(switch_stmt.cases.len(), 2, "Program 2 should have 2 mood cases");
        assert!(switch_stmt.default.is_none(), "Program 2 should not have a basic case");
    } else {
        panic!("Program 2 is not a SwitchStatement");
    }

    // Test case 3: Switch with complex expression
    let input3 = r#"
        vibe_check x + 10 {
            mood 20:
                y = "twenty";
            mood 30:
                y = "thirty";
            basic:
                y = "other";
        }
    "#;
    let program3 = test_parser_with_input(input3)?;
    assert_eq!(program3.statements.len(), 1, "Program 3 failed: incorrect statement count");
    if let Some(switch_stmt) = program3.statements[0].as_any().downcast_ref::<ast::SwitchStatement>() {
        // Check value (should be infix expression)
        assert!(switch_stmt.value.as_any().is::<ast::InfixExpression>(), "Switch value is not InfixExpression");
        assert_eq!(switch_stmt.value.string(), "x + 10");
        assert_eq!(switch_stmt.cases.len(), 2, "Program 3 should have 2 mood cases");
        assert!(switch_stmt.default.is_some(), "Program 3 should have a basic case");
    } else {
        panic!("Program 3 is not a SwitchStatement");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expression, Identifier, IntegerLiteral, StringLiteral, BooleanLiteral, PrefixExpression, InfixExpression, ArrayLiteral, HashLiteral, FunctionLiteral};

    /// Test the parser with the given input string
    fn test_parser_with_input(input: &str) -> Result<Program, Error> {
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;
        parser.parse_program()
    }

    /// Helper to check for parser errors
    fn check_parser_errors(parser: &Parser) {
        let errors = parser.errors();
        if errors.is_empty() {
            return;
        }

        eprintln!("\nParser encountered {} errors:", errors.len());
        for msg in errors {
            eprintln!("  Parser error: {}", msg);
        }
        eprintln!(); // Add a newline for better formatting
        panic!("Parser errors occurred");
    }

    /// Helper to test integer literal parsing
    fn test_integer_literal(expr: &Box<dyn Expression>, expected_value: i64) {
        let int_lit = expr.as_any().downcast_ref::<IntegerLiteral>();
        assert!(int_lit.is_some(), "Expression is not an IntegerLiteral");
        let int_lit = int_lit.unwrap();
        assert_eq!(int_lit.value, expected_value);
        assert_eq!(int_lit.token_literal(), expected_value.to_string());
    }

    /// Helper to test boolean literal parsing
    fn test_boolean_literal(expr: &Box<dyn Expression>, expected_value: bool) {
        let bool_lit = expr.as_any().downcast_ref::<BooleanLiteral>();
        assert!(bool_lit.is_some(), "Expression is not a BooleanLiteral");
        let bool_lit = bool_lit.unwrap();
        assert_eq!(bool_lit.value, expected_value);
        assert_eq!(bool_lit.token_literal(), if expected_value { "based" } else { "cap" });
    }

    /// Helper to test identifier parsing
    fn test_identifier(expr: &Box<dyn Expression>, expected_value: &str) {
        let ident = expr.as_any().downcast_ref::<Identifier>();
        assert!(ident.is_some(), "Expression is not an Identifier");
        let ident = ident.unwrap();
        assert_eq!(ident.value, expected_value);
        assert_eq!(ident.token_literal(), expected_value);
    }

    /// Helper to test literal expression parsing
    fn test_literal_expression(expr: &Box<dyn Expression>, expected: &LiteralType) {
        match expected {
            LiteralType::Int(val) => test_integer_literal(expr, *val),
            LiteralType::Bool(val) => test_boolean_literal(expr, *val),
            LiteralType::Ident(val) => test_identifier(expr, val),
            LiteralType::String(val) => {
                let str_lit = expr.as_any().downcast_ref::<StringLiteral>();
                assert!(str_lit.is_some(), "Expression is not a StringLiteral");
                let str_lit = str_lit.unwrap();
                assert_eq!(str_lit.value, *val);
                assert_eq!(str_lit.token_literal(), *val); // Assuming token_literal includes quotes
            }
        }
    }

    /// Helper to test infix expression parsing
    fn test_infix_expression(
        expr: &Box<dyn Expression>,
        expected_left: &LiteralType,
        expected_op: &str,
        expected_right: &LiteralType,
    ) {
        let infix_expr = expr.as_any().downcast_ref::<InfixExpression>();
        assert!(infix_expr.is_some(), "Expression is not an InfixExpression");
        let infix_expr = infix_expr.unwrap();

        test_literal_expression(&infix_expr.left, expected_left);
        assert_eq!(infix_expr.operator, expected_op);
        test_literal_expression(&infix_expr.right, expected_right);
    }

    #[test]
    fn test_parse_sus_statements() -> Result<(), Error> {
        struct TestCase<'a> {
            input: &'a str,
            expected_ident: &'a str,
            expected_value: LiteralType<'a>,
        }

        let tests = vec![
            TestCase { input: "sus x = 5;", expected_ident: "x", expected_value: LiteralType::Int(5) },
            TestCase { input: "sus y = based;", expected_ident: "y", expected_value: LiteralType::Bool(true) },
            TestCase { input: "sus z = cap;", expected_ident: "z", expected_value: LiteralType::Bool(false) },
            TestCase { input: "sus foo = \"bar\";", expected_ident: "foo", expected_value: LiteralType::String("bar") },
            TestCase { input: "sus another = y;", expected_ident: "another", expected_value: LiteralType::Ident("y") },
        ];

        for tt in tests {
            let mut lexer = Lexer::new(tt.input);
            let mut parser = Parser::new(&mut lexer)?;
            let program = parser.parse_program()?;
            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1, "program.statements does not contain 1 statement. got={}", program.statements.len());

            let stmt = program.statements[0].as_any().downcast_ref::<ast::LetStatement>();
            assert!(stmt.is_some(), "statement is not LetStatement");
            let stmt = stmt.unwrap();

            assert_eq!(stmt.name.value, tt.expected_ident);
            assert_eq!(stmt.name.token_literal(), tt.expected_ident);

            match &stmt.value {
                Some(val) => test_literal_expression(val, &tt.expected_value),
                None => panic!("LetStatement value is None"),
            }
        }

        Ok(())
    }

    #[test]
    fn test_parse_return_statements() -> Result<(), Error> {
        struct TestCase<'a> {
            input: &'a str,
            expected_value: Option<LiteralType<'a>>,
        }

        let tests = vec![
            TestCase { input: "yolo 5;", expected_value: Some(LiteralType::Int(5)) },
            TestCase { input: "yolo based;", expected_value: Some(LiteralType::Bool(true)) },
            TestCase { input: "yolo;", expected_value: None },
            TestCase { input: "yolo x;", expected_value: Some(LiteralType::Ident("x")) },
        ];

        for tt in tests {
            let mut lexer = Lexer::new(tt.input);
            let mut parser = Parser::new(&mut lexer)?;
            let program = parser.parse_program()?;
            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1, "program.statements does not contain 1 statement. got={}", program.statements.len());

            let stmt = program.statements[0].as_any().downcast_ref::<ast::ReturnStatement>();
            assert!(stmt.is_some(), "statement is not ReturnStatement");
            let stmt = stmt.unwrap();

            match (&stmt.return_value, &tt.expected_value) {
                (Some(val), Some(expected)) => test_literal_expression(val, expected),
                (None, None) => { /* Both are None, this is correct */ },
                (Some(_), None) => panic!("Expected no return value, but got one"),
                (None, Some(_)) => panic!("Expected a return value, but got none"),
            }
        }

        Ok(())
    }

    #[test]
    fn test_parse_identifier_expression() -> Result<(), Error> {
        let input = "foobar;";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;
        let program = parser.parse_program()?;
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        let ident = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<Identifier>().unwrap();

        assert_eq!(ident.value, "foobar");
        assert_eq!(ident.token_literal(), "foobar");

        Ok(())
    }

    #[test]
    fn test_parse_integer_literal_expression() -> Result<(), Error> {
        let input = "5;";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;
        let program = parser.parse_program()?;
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        test_integer_literal(stmt.expression.as_ref().unwrap(), 5);

        Ok(())
    }

    #[test]
    fn test_parse_boolean_literal_expression() -> Result<(), Error> {
        let tests = vec![
            ("based;", true),
            ("cap;", false),
        ];

        for (input, expected) in tests {
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(&mut lexer)?;
            let program = parser.parse_program()?;
            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1);
            let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
            test_boolean_literal(stmt.expression.as_ref().unwrap(), expected);
        }
        Ok(())
    }

    #[test]
    fn test_parse_prefix_expressions() -> Result<(), Error> {
        struct TestCase<'a> {
            input: &'a str,
            expected_operator: &'a str,
            expected_value: LiteralType<'a>,
        }

        let tests = vec![
            TestCase { input: "!5;", expected_operator: "!", expected_value: LiteralType::Int(5) },
            TestCase { input: "-15;", expected_operator: "-", expected_value: LiteralType::Int(15) },
            TestCase { input: "!based;", expected_operator: "!", expected_value: LiteralType::Bool(true) },
            TestCase { input: "!cap;", expected_operator: "!", expected_value: LiteralType::Bool(false) },
        ];

        for tt in tests {
            let mut lexer = Lexer::new(tt.input);
            let mut parser = Parser::new(&mut lexer)?;
            let program = parser.parse_program()?;
            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1);
            let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
            let expr = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<PrefixExpression>().unwrap();

            assert_eq!(expr.operator, tt.expected_operator);
            test_literal_expression(&expr.right, &tt.expected_value);
        }
        Ok(())
    }

    #[test]
    fn test_parse_infix_expressions() -> Result<(), Error> {
        struct TestCase<'a> {
            input: &'a str,
            left_value: LiteralType<'a>,
            operator: &'a str,
            right_value: LiteralType<'a>,
        }

        let tests = vec![
            TestCase { input: "5 + 5;", left_value: LiteralType::Int(5), operator: "+", right_value: LiteralType::Int(5) },
            TestCase { input: "5 - 5;", left_value: LiteralType::Int(5), operator: "-", right_value: LiteralType::Int(5) },
            TestCase { input: "5 * 5;", left_value: LiteralType::Int(5), operator: "*", right_value: LiteralType::Int(5) },
            TestCase { input: "5 / 5;", left_value: LiteralType::Int(5), operator: "/", right_value: LiteralType::Int(5) },
            TestCase { input: "5 > 5;", left_value: LiteralType::Int(5), operator: ">", right_value: LiteralType::Int(5) },
            TestCase { input: "5 < 5;", left_value: LiteralType::Int(5), operator: "<", right_value: LiteralType::Int(5) },
            TestCase { input: "5 == 5;", left_value: LiteralType::Int(5), operator: "==", right_value: LiteralType::Int(5) },
            TestCase { input: "5 != 5;", left_value: LiteralType::Int(5), operator: "!=", right_value: LiteralType::Int(5) },
            TestCase { input: "based == based;", left_value: LiteralType::Bool(true), operator: "==", right_value: LiteralType::Bool(true) },
            TestCase { input: "based != cap;", left_value: LiteralType::Bool(true), operator: "!=", right_value: LiteralType::Bool(false) },
            TestCase { input: "cap == cap;", left_value: LiteralType::Bool(false), operator: "==", right_value: LiteralType::Bool(false) },
        ];

        for tt in tests {
            let mut lexer = Lexer::new(tt.input);
            let mut parser = Parser::new(&mut lexer)?;
            let program = parser.parse_program()?;
            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1);
            let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
            test_infix_expression(stmt.expression.as_ref().unwrap(), &tt.left_value, tt.operator, &tt.right_value);
        }
        Ok(())
    }

    #[test]
    fn test_operator_precedence_parsing() -> Result<(), Error> {
        struct TestCase<'a> {
            input: &'a str,
            expected: &'a str,
        }

        let tests = vec![
            TestCase { input: "-a * b", expected: "((-a) * b)" },
            TestCase { input: "!-a", expected: "(!(-a))" },
            TestCase { input: "a + b + c", expected: "((a + b) + c)" },
            TestCase { input: "a + b - c", expected: "((a + b) - c)" },
            TestCase { input: "a * b * c", expected: "((a * b) * c)" },
            TestCase { input: "a * b / c", expected: "((a * b) / c)" },
            TestCase { input: "a + b / c", expected: "(a + (b / c))" },
            TestCase { input: "a + b * c + d / e - f", expected: "(((a + (b * c)) + (d / e)) - f)" },
            TestCase { input: "3 + 4; -5 * 5", expected: "(3 + 4)((-5) * 5)" }, // Two separate statements
            TestCase { input: "5 > 4 == 3 < 4", expected: "((5 > 4) == (3 < 4))" },
            TestCase { input: "5 < 4 != 3 > 4", expected: "((5 < 4) != (3 > 4))" },
            TestCase { input: "3 + 4 * 5 == 3 * 1 + 4 * 5", expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))" },
            TestCase { input: "based", expected: "based" },
            TestCase { input: "cap", expected: "cap" },
            TestCase { input: "3 > 5 == cap", expected: "((3 > 5) == cap)" },
            TestCase { input: "3 < 5 == based", expected: "((3 < 5) == based)" },
            TestCase { input: "1 + (2 + 3) + 4", expected: "((1 + (2 + 3)) + 4)" },
            TestCase { input: "(5 + 5) * 2", expected: "((5 + 5) * 2)" },
            TestCase { input: "2 / (5 + 5)", expected: "(2 / (5 + 5))" },
            TestCase { input: "-(5 + 5)", expected: "(-(5 + 5))" },
            TestCase { input: "!(based == based)", expected: "(!(based == based))" },
            TestCase { input: "a + add(b * c) + d", expected: "((a + add((b * c))) + d)" },
            TestCase { input: "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))", expected: "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))" },
            TestCase { input: "add(a + b + c * d / f + g)", expected: "add((((a + b) + ((c * d) / f)) + g))" },
            TestCase { input: "a * [1, 2, 3, 4][b * c] * d", expected: "((a * (crew[1, 2, 3, 4][(b * c)])) * d)" },
            TestCase { input: "add(a * b[2], b[1], 2 * [1, 2][1])", expected: "add((a * (b[2])), (b[1]), (2 * (crew[1, 2][1])))" },
        ];

        for tt in tests {
            let mut lexer = Lexer::new(tt.input);
            let mut parser = Parser::new(&mut lexer)?;
            let program = parser.parse_program()?;
            check_parser_errors(&parser);

            let actual = program.to_string();
            assert_eq!(actual, tt.expected, "for input: \"{}\"", tt.input);
        }
        Ok(())
    }

    #[test]
    fn test_parse_if_statement() -> Result<(), Error> {
        let input = "lowkey (x < y) { x } highkey { y }";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;
        let program = parser.parse_program()?;
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements[0].as_any().downcast_ref::<ast::IfStatement>().unwrap();

        test_infix_expression(&stmt.condition, &LiteralType::Ident("x"), "<", &LiteralType::Ident("y"));

        assert_eq!(stmt.consequence.statements.len(), 1);
        let consequence_stmt = stmt.consequence.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        test_identifier(consequence_stmt.expression.as_ref().unwrap(), "x");

        assert!(stmt.alternative.is_some());
        let alternative = stmt.alternative.as_ref().unwrap();
        assert_eq!(alternative.statements.len(), 1);
        let alternative_stmt = alternative.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        test_identifier(alternative_stmt.expression.as_ref().unwrap(), "y");

        Ok(())
    }

    #[test]
    fn test_parse_if_no_else_statement() -> Result<(), Error> {
        let input = "lowkey (x < y) { x }";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;
        let program = parser.parse_program()?;
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements[0].as_any().downcast_ref::<ast::IfStatement>().unwrap();

        test_infix_expression(&stmt.condition, &LiteralType::Ident("x"), "<", &LiteralType::Ident("y"));

        assert_eq!(stmt.consequence.statements.len(), 1);
        let consequence_stmt = stmt.consequence.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        test_identifier(consequence_stmt.expression.as_ref().unwrap(), "x");

        assert!(stmt.alternative.is_none());

        Ok(())
    }

    #[test]
    fn test_parse_for_statements() -> Result<(), Error> {
        #[derive(Debug)]
        enum StatementType {
            Sus, // LetStatement
            Facts, // FactsStatement
            Expression, // ExpressionStatement
            None, // Option is None
        }

        #[derive(Debug)]
        enum ExpressionType {
            Infix, // InfixExpression
            Expression, // Any Expression
            None, // Option is None
        }

        struct TestCase<'a> {
            input: &'a str,
            expected_init: StatementType,
            expected_condition: ExpressionType,
            expected_post: StatementType,
            expected_body_statements: usize,
        }

        let tests = vec![
            TestCase {
                input: "bestie { sus x = 1; }",
                expected_init: StatementType::None,
                expected_condition: ExpressionType::None,
                expected_post: StatementType::None,
                expected_body_statements: 1,
            },
            TestCase {
                input: "bestie i < 10 { sus y = 2; facts z = 3; }",
                expected_init: StatementType::None,
                expected_condition: ExpressionType::Infix,
                expected_post: StatementType::None,
                expected_body_statements: 2,
            },
            TestCase {
                input: "bestie sus i = 0; i < 10; i = i + 1 { sus a = i; }",
                expected_init: StatementType::Sus,
                expected_condition: ExpressionType::Infix,
                expected_post: StatementType::Expression,
                expected_body_statements: 1,
            },
            TestCase {
                input: "bestie ; i < 10 ; i = i + 1 { sus b = 1; }",
                expected_init: StatementType::None,
                expected_condition: ExpressionType::Infix,
                expected_post: StatementType::Expression,
                expected_body_statements: 1,
            },
            TestCase {
                input: "bestie facts j = 0; ; j = j + 1 { sus c = 2; }",
                expected_init: StatementType::Facts,
                expected_condition: ExpressionType::None,
                expected_post: StatementType::Expression,
                expected_body_statements: 1,
            },
            TestCase {
                input: "bestie sus k = 0; k < 5 ; { sus d = 3; }",
                expected_init: StatementType::Sus,
                expected_condition: ExpressionType::Infix,
                expected_post: StatementType::None,
                expected_body_statements: 1,
            },
            TestCase {
                input: "bestie ; ; { /* empty */ }",
                expected_init: StatementType::None,
                expected_condition: ExpressionType::None,
                expected_post: StatementType::None,
                expected_body_statements: 0,
            },
        ];

        for (i, tt) in tests.iter().enumerate() {
            let mut lexer = Lexer::new(tt.input);
            let mut parser = Parser::new(&mut lexer)?;
            let program = parser.parse_program()?;
            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1, "Test[{}]: program.statements does not contain 1 statement. got={}", i, program.statements.len());

            let stmt = program.statements[0].as_any().downcast_ref::<ast::ForStatement>();
            assert!(stmt.is_some(), "Test[{}]: statement is not ForStatement", i);
            let stmt = stmt.unwrap();

            // Check init statement type
            match tt.expected_init {
                StatementType::Sus => assert!(stmt.init.as_ref().map_or(false, |s| s.as_any().is::<ast::LetStatement>()), "Test[{}]: Init is not Sus", i),
                StatementType::Facts => assert!(stmt.init.as_ref().map_or(false, |s| s.as_any().is::<ast::FactsStatement>()), "Test[{}]: Init is not Facts", i),
                StatementType::Expression => assert!(stmt.init.as_ref().map_or(false, |s| s.as_any().is::<ast::ExpressionStatement>()), "Test[{}]: Init is not Expression", i),
                StatementType::None => assert!(stmt.init.is_none(), "Test[{}]: Init is not None", i),
            }

            // Check condition expression type
            match tt.expected_condition {
                ExpressionType::Infix => assert!(stmt.condition.as_ref().map_or(false, |e| e.as_any().is::<ast::InfixExpression>()), "Test[{}]: Condition is not Infix", i),
                ExpressionType::Expression => assert!(stmt.condition.is_some(), "Test[{}]: Condition is None", i),
                ExpressionType::None => assert!(stmt.condition.is_none(), "Test[{}]: Condition is not None", i),
            }

            // Check post statement type
            match tt.expected_post {
                StatementType::Sus => assert!(stmt.post.as_ref().map_or(false, |s| s.as_any().is::<ast::LetStatement>()), "Test[{}]: Post is not Sus", i),
                StatementType::Facts => assert!(stmt.post.as_ref().map_or(false, |s| s.as_any().is::<ast::FactsStatement>()), "Test[{}]: Post is not Facts", i),
                StatementType::Expression => assert!(stmt.post.as_ref().map_or(false, |s| s.as_any().is::<ast::ExpressionStatement>()), "Test[{}]: Post is not Expression", i),
                StatementType::None => assert!(stmt.post.is_none(), "Test[{}]: Post is not None", i),
            }

            // Check body statement count
            assert_eq!(stmt.body.statements.len(), tt.expected_body_statements, "Test[{}]: Incorrect number of body statements", i);
        }

        Ok(())
    }

    #[test]
    fn test_parse_periodt_statements() -> Result<(), Error> {
        // Test with different while statement (periodt) formats
        let inputs = vec![
            "periodt x < 10 { x = x + 1; }",
            "periodt (x < 10) { x = x + 1; }",
            "periodt based { x = x + 1; }",
            "periodt 1 < 2 { print(\"hello\"); }"
        ];

        for input in inputs {
            let program = test_parser_with_input(input)?;
            check_parser_errors(&Parser::new(&mut Lexer::new(input))?);

            // Verify we have exactly one statement
            assert_eq!(program.statements.len(), 1, "Failed to parse: {}", input);

            // Verify it's a while statement (periodt)
            let while_stmt = program.statements[0].as_any().downcast_ref::<ast::WhileStatement>();
            assert!(while_stmt.is_some(), "Not a while statement: {}", input);

            // Verify it has a condition and body
            let while_stmt = while_stmt.unwrap();
            assert!(while_stmt.condition.token_literal().len() > 0, "Missing condition in: {}", input);
            assert!(while_stmt.body.statements.len() > 0, "Empty body in: {}", input);
        }

        Ok(())
    }

    #[test]
    fn test_parse_parenthesized_periodt() -> Result<(), Error> {
        let input = "periodt (x < 10) { x = x + 1; }";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;

        // Start parsing
        let token = parser.current_token.clone();
        assert_eq!(token, Token::Periodt);

        // Move to next token after periodt
        parser.next_token()?;
        assert_eq!(parser.current_token, Token::LParen);

        // Parse the condition
        let condition = parser.parse_expression(Precedence::Lowest)?;

        // After parsing the condition, the current token should be LBrace
        assert_eq!(parser.current_token, Token::LBrace, "Current token after parsing condition should be LBrace but is {:?}", parser.current_token);

        Ok(())
    }

    #[test]
    fn test_parse_array_literals() -> Result<(), Error> {
        let input = "crew[1, 2 * 2, 3 + 3]";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;
        let program = parser.parse_program()?;
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        let array_lit = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<ArrayLiteral>().unwrap();

        assert_eq!(array_lit.elements.len(), 3);
        test_integer_literal(&array_lit.elements[0], 1);
        test_infix_expression(&array_lit.elements[1], &LiteralType::Int(2), "*", &LiteralType::Int(2));
        test_infix_expression(&array_lit.elements[2], &LiteralType::Int(3), "+", &LiteralType::Int(3));

        Ok(())
    }

    #[test]
    fn test_parse_empty_array_literal() -> Result<(), Error> {
        let input = "crew[]";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;
        let program = parser.parse_program()?;
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        let array_lit = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<ArrayLiteral>().unwrap();

        assert_eq!(array_lit.elements.len(), 0);
        Ok(())
    }

    #[test]
    fn test_parse_hash_literals() -> Result<(), Error> {
        let input = "tea{\"one\": 1, \"two\": 2, \"three\": 3}";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;
        let program = parser.parse_program()?;
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        let hash_lit = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<HashLiteral>().unwrap();

        assert_eq!(hash_lit.pairs.len(), 3);

        let expected = vec![
            ("one", LiteralType::Int(1)),
            ("two", LiteralType::Int(2)),
            ("three", LiteralType::Int(3)),
        ];

        for (i, (key, value)) in hash_lit.pairs.iter().enumerate() {
             let (expected_key, expected_val) = &expected[i];
             let key_str_lit = key.as_any().downcast_ref::<StringLiteral>();
             assert!(key_str_lit.is_some(), "Key is not a StringLiteral. got={:?}", key);
             assert_eq!(&key_str_lit.unwrap().value, expected_key);
             test_literal_expression(value, expected_val);
        }
        Ok(())
    }

     #[test]
    fn test_parse_empty_hash_literal() -> Result<(), Error> {
        let input = "tea{}";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;
        let program = parser.parse_program()?;
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        let hash_lit = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<HashLiteral>().unwrap();

        assert_eq!(hash_lit.pairs.len(), 0);
        Ok(())
    }

     #[test]
    fn test_parse_hash_literals_with_expressions() -> Result<(), Error> {
        let input = "tea{\"one\": 0 + 1, \"two\": 10 - 8, \"three\": 15 / 5}";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;
        let program = parser.parse_program()?;
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        let hash_lit = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<HashLiteral>().unwrap();

        assert_eq!(hash_lit.pairs.len(), 3);

        let expected_values = vec![
            (LiteralType::Int(0), "+", LiteralType::Int(1)),
            (LiteralType::Int(10), "-", LiteralType::Int(8)),
            (LiteralType::Int(15), "/", LiteralType::Int(5)),
        ];

        for (i, (_key, value)) in hash_lit.pairs.iter().enumerate() {
             let (left, op, right) = &expected_values[i];
             test_infix_expression(value, left, op, right);
        }
        Ok(())
    }

    #[test]
    fn test_parse_function_literal() -> Result<(), Error> {
        let input = "stan(x, y) { x + y; }";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;
        let program = parser.parse_program()?;
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        let func_lit = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<FunctionLiteral>().unwrap();

        assert_eq!(func_lit.parameters.len(), 2);
        assert_eq!(func_lit.parameters[0].value, "x");
        assert_eq!(func_lit.parameters[1].value, "y");

        assert_eq!(func_lit.body.statements.len(), 1);
        let body_stmt = func_lit.body.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        test_infix_expression(&body_stmt.expression.as_ref().unwrap(), &LiteralType::Ident("x"), "+", &LiteralType::Ident("y"));

        Ok(())
    }

    #[test]
    fn test_parse_function_parameters() -> Result<(), Error> {
        struct TestCase<'a> {
            input: &'a str,
            expected_params: Vec<&'a str>,
        }

        let tests = vec![
            TestCase { input: "stan() {};", expected_params: vec![] },
            TestCase { input: "stan(x) {};", expected_params: vec!["x"] },
            TestCase { input: "stan(x, y, z) {};", expected_params: vec!["x", "y", "z"] },
        ];

        for tt in tests {
            let mut lexer = Lexer::new(tt.input);
            let mut parser = Parser::new(&mut lexer)?;
            let program = parser.parse_program()?;
            check_parser_errors(&parser);

            let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
            let func_lit = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<FunctionLiteral>().unwrap();

            assert_eq!(func_lit.parameters.len(), tt.expected_params.len());
            for (i, expected_param) in tt.expected_params.iter().enumerate() {
                assert_eq!(func_lit.parameters[i].value, *expected_param);
            }
        }
        Ok(())
    }
} 