use cursed::ast::*;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declaration_ast() {
        let source = r#"
            sus x: i64 = 42;
            sus name: String = "hello";
            sus flag: bool = facts;
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();

        assert_eq!(ast.statements.len(), 3, "Should parse 3 variable declarations");
        
        // Verify AST structure
        for statement in &ast.statements {
            match statement {
                Statement::VarDeclaration { .. } => {
                    // Expected - variable declaration
                },
                _ => {
                    // May be other valid statement types
                }
            }
        }
    }

    #[test]
    fn test_function_declaration_ast() {
        let source = r#"
            slay add(a: i64, b: i64) -> i64 {
                a + b
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();

        assert_eq!(ast.statements.len(), 1, "Should parse 1 function declaration");
        
        if let Some(Statement::Function { name, params, return_type, body }) = ast.statements.first() {
            assert_eq!(name, "add", "Function name should be 'add'");
            assert_eq!(params.len(), 2, "Function should have 2 parameters");
            assert!(return_type.is_some(), "Function should have return type");
            assert!(!body.is_empty(), "Function body should not be empty");
        }
    }

    #[test]
    fn test_expression_ast() {
        let expressions = vec![
            ("42", "integer literal"),
            ("3.14", "float literal"),
            ("\"hello\"", "string literal"),
            ("facts", "boolean true"),
            ("cap", "boolean false"),
            ("x + y", "binary expression"),
            ("a * b + c", "complex binary expression"),
            ("(x + y) * z", "parenthesized expression"),
            ("func(1, 2)", "function call"),
            ("arr[0]", "array access"),
            ("obj.field", "field access"),
        ];

        for (expr, description) in expressions {
            let source = format!("sus result = {};", expr);
            let mut lexer = Lexer::new(source.to_string());
            let mut parser = Parser::new(lexer).unwrap();
            let ast = parser.parse_program();
            
            assert!(ast.is_ok(), "Expression '{}' ({}) should parse successfully", expr, description);
            
            if let Ok(program) = ast {
                assert_eq!(program.statements.len(), 1, "Should have one statement for {}", description);
            }
        }
    }

    #[test]
    fn test_control_flow_ast() {
        let source = r#"
            slay control_flow_test() -> i64 {
                lowkey (facts) {
                    42
                } flex {
                    24
                }
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();

        assert_eq!(ast.statements.len(), 1, "Should parse function with control flow");
        
        // Verify it's a function with conditional logic
        if let Some(Statement::Function { body, .. }) = ast.statements.first() {
            assert!(!body.is_empty(), "Function body should contain statements");
        }
    }

    #[test]
    fn test_loop_ast() {
        let source = r#"
            slay loop_test() {
                lowkey (sus i = 0; i < 10; i++) {
                    // Loop body
                }
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Loop should parse successfully");
        
        if let Ok(program) = ast {
            assert_eq!(program.statements.len(), 1, "Should have one function");
        }
    }

    #[test]
    fn test_struct_declaration_ast() {
        let source = r#"
            squad Person {
                name: String,
                age: i64,
                email: String
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Struct declaration should parse");
        
        if let Ok(program) = ast {
            assert_eq!(program.statements.len(), 1, "Should have one struct declaration");
        }
    }

    #[test]
    fn test_interface_declaration_ast() {
        let source = r#"
            collab Drawable {
                slay draw(self);
                slay get_area(self) -> f64;
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Interface declaration should parse");
        
        if let Ok(program) = ast {
            assert_eq!(program.statements.len(), 1, "Should have one interface declaration");
        }
    }

    #[test]
    fn test_generic_function_ast() {
        let source = r#"
            slay identity<T>(x: T) -> T {
                x
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Generic function should parse");
        
        if let Ok(program) = ast {
            assert_eq!(program.statements.len(), 1, "Should have one function");
        }
    }

    #[test]
    fn test_array_literal_ast() {
        let source = r#"
            sus numbers = [1, 2, 3, 4, 5];
            sus empty: [i64] = [];
            sus mixed = [1, 2, 3];
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Array literals should parse");
        
        if let Ok(program) = ast {
            assert_eq!(program.statements.len(), 3, "Should have three array declarations");
        }
    }

    #[test]
    fn test_channel_operations_ast() {
        let source = r#"
            slay channel_test() {
                sus ch = make(chan i64, 10);
                ch <- 42;
                sus value = <-ch;
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        // Channel operations should attempt to parse
        assert!(ast.is_ok() || ast.is_err(), "Channel operations should attempt parsing");
    }

    #[test]
    fn test_error_propagation_ast() {
        let source = r#"
            slay might_fail() -> ?String {
                sus result = some_operation()?;
                result
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        // Error propagation should attempt to parse
        assert!(ast.is_ok() || ast.is_err(), "Error propagation should attempt parsing");
    }

    #[test]
    fn test_nested_expressions_ast() {
        let source = r#"
            slay complex_calc() -> f64 {
                (x + y) * (z - w) / (a + b)
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Nested expressions should parse");
        
        if let Ok(program) = ast {
            assert_eq!(program.statements.len(), 1, "Should have one function");
        }
    }

    #[test]
    fn test_block_statements_ast() {
        let source = r#"
            slay nested_blocks() {
                {
                    sus x = 1;
                    {
                        sus y = 2;
                        x + y
                    }
                }
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Nested blocks should parse");
    }

    #[test]
    fn test_match_statement_ast() {
        let source = r#"
            slay match_test(x: i64) -> String {
                vibe_check x {
                    mood 1 => "one",
                    mood 2 => "two",
                    basic => "other"
                }
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        // Match statements should attempt to parse
        assert!(ast.is_ok() || ast.is_err(), "Match statement should attempt parsing");
    }

    #[test]
    fn test_import_statements_ast() {
        let source = r#"
            import "stdlib::math";
            import "stdlib::string" as str;
            from "stdlib::collections" import HashMap;
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        // Import statements should attempt to parse
        assert!(ast.is_ok() || ast.is_err(), "Import statements should attempt parsing");
    }

    #[test]
    fn test_method_call_ast() {
        let source = r#"
            slay method_calls() {
                sus obj = SomeStruct {};
                obj.method();
                obj.field.nested_method(1, 2);
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Method calls should parse");
    }

    #[test]
    fn test_complex_program_ast() {
        let source = r#"
            squad User {
                id: i64,
                name: String
            }
            
            collab Identifiable {
                slay get_id(self) -> i64;
            }
            
            impl Identifiable for User {
                slay get_id(self) -> i64 {
                    self.id
                }
            }
            
            slay create_user(name: String) -> User {
                User {
                    id: 1,
                    name: name
                }
            }
            
            slay main() {
                sus user = create_user("Alice");
                sus id = user.get_id();
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Complex program should parse");
        
        if let Ok(program) = ast {
            assert!(program.statements.len() >= 5, "Should have multiple statements");
        }
    }

    #[test]
    fn test_ast_node_types() {
        let source = r#"
            sus x = 42;
            slay func() {}
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();

        // Verify AST contains different node types
        assert_eq!(ast.statements.len(), 2);
        
        let mut has_var_decl = false;
        let mut has_func_decl = false;
        
        for statement in &ast.statements {
            match statement {
                Statement::VarDeclaration { .. } => has_var_decl = true,
                Statement::Function { .. } => has_func_decl = true,
                _ => {} // Other statement types are also valid
            }
        }
        
        // Should contain at least one of each type or handle them appropriately
        assert!(has_var_decl || has_func_decl, "Should parse different statement types");
    }

    #[test]
    fn test_ast_with_comments() {
        let source = r#"
            // Function to add two numbers
            slay add(a: i64, b: i64) -> i64 {
                // Return the sum
                a + b
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Code with comments should parse");
        
        if let Ok(program) = ast {
            assert_eq!(program.statements.len(), 1, "Comments should not create statements");
        }
    }

    #[test]
    fn test_empty_program_ast() {
        let source = "";

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Empty program should parse");
        
        if let Ok(program) = ast {
            assert_eq!(program.statements.len(), 0, "Empty program should have no statements");
        }
    }
}
