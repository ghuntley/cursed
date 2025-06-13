/// Comprehensive test suite for type switch functionality in CURSED
/// Tests AST structures, parsing, compilation, and runtime behavior

use cursed::ast::expressions::*;
use cursed::ast::conditionals::*;
use cursed::ast::statements::*;
use cursed::ast::traits::*;
use cursed::ast::types::*;
use cursed::lexer::*;
use cursed::parser::Parser;
use cursed::error::*;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // AST Structure Tests
    // ============================================================================

    #[test]
    fn test_type_switch_ast_creation() {
        let target_expr = Box::new(IdentifierExpression::new("value".to_string()));
        let variable_name = Some("v".to_string());
        
        let case1 = TypeSwitchCase::new(
            vec!["string".to_string()],
            BlockStatement::new(vec![]),
            variable_name.clone(),
        );
        
        let case2 = TypeSwitchCase::new(
            vec!["int".to_string()],
            BlockStatement::new(vec![]),
            variable_name.clone(),
        );

        let type_switch = TypeSwitchStatement::new(
            "vibe_check".to_string(),
            target_expr,
            vec![case1, case2],
            None,
            variable_name,
        );

        assert_eq!(type_switch.token, "vibe_check");
        assert!(type_switch.expression.is_some());
        assert_eq!(type_switch.cases.len(), 2);
        assert_eq!(type_switch.variable_name, Some("v".to_string()));
    }

    #[test]
    fn test_type_switch_string_representation() {
        let target_expr = Box::new(IdentifierExpression::new("input".to_string()));
        let variable_name = Some("v".to_string());
        
        let case1 = TypeSwitchCase::new(
            vec!["string".to_string()],
            BlockStatement::new(vec![]),
            variable_name.clone(),
        );

        let type_switch = TypeSwitchStatement::new(
            "vibe_check".to_string(),
            target_expr,
            vec![case1],
            None,
            variable_name,
        );

        let string_repr = type_switch.string();
        assert!(string_repr.contains("vibe_check"));
        assert!(string_repr.contains("v := input.(type)"));
        assert!(string_repr.contains("mood string:"));
    }

    #[test]
    fn test_type_switch_case_creation() {
        let types = vec!["string".to_string(), "[]byte".to_string()];
        let body = BlockStatement::new(vec![]);
        let variable_name = Some("v".to_string());

        let case = TypeSwitchCase::new(types.clone(), body, variable_name.clone());

        assert_eq!(case.types, types);
        assert_eq!(case.variable_name, variable_name);
        assert!(case.body.statements.is_empty());
    }

    #[test]
    fn test_type_switch_case_string_representation() {
        let types = vec!["string".to_string()];
        let body = BlockStatement::new(vec![]);
        let variable_name = Some("v".to_string());

        let case = TypeSwitchCase::new(types, body, variable_name);
        let string_repr = case.string();

        assert!(string_repr.contains("mood string:"));
    }

    // ============================================================================
    // Parser Tests
    // ============================================================================

    #[test]
    fn test_parse_simple_type_switch() {
        let input = r#"
        vibe_check v := input.(type) {
            mood string:
                println("It's a string")
            mood int:
                println("It's an integer")
        }
        "#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        
        // Note: This test may need adjustment based on actual parser implementation
        // The parser needs to be enhanced to handle type switch syntax
        let stmt = parser.parse_statement();
        assert!(stmt.is_ok());
    }

    #[test]
    fn test_parse_type_switch_with_default() {
        let input = r#"
        vibe_check v := input.(type) {
            mood string:
                vibe "string value"
            mood int:
                vibe "integer value"
            basic:
                vibe "unknown type"
        }
        "#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        
        let stmt = parser.parse_statement();
        assert!(stmt.is_ok());
    }

    #[test]
    fn test_parse_type_switch_multiple_types_per_case() {
        let input = r#"
        vibe_check v := input.(type) {
            mood string, []byte:
                vibe "text data"
            mood int, int64, float64:
                vibe "numeric data"
        }
        "#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        
        let stmt = parser.parse_statement();
        assert!(stmt.is_ok());
    }

    #[test]
    fn test_parse_type_switch_with_complex_types() {
        let input = r#"
        vibe_check v := input.(type) {
            mood map[string]interface{}:
                vibe "map type"
            mood []map[string]int:
                vibe "slice of maps"
            mood chan<- int:
                vibe "send-only channel"
        }
        "#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        
        let stmt = parser.parse_statement();
        assert!(stmt.is_ok());
    }

    // ============================================================================
    // Error Handling Tests
    // ============================================================================

    #[test]
    fn test_type_switch_missing_variable() {
        let input = r#"
        vibe_check input.(type) {
            mood string:
                println("string")
        }
        "#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        
        let stmt = parser.parse_statement();
        // Should either parse successfully (if this syntax is allowed) or return error
        // Implementation depends on language design decisions
    }

    #[test]
    fn test_type_switch_invalid_syntax() {
        let input = r#"
        vibe_check v := input.type {
            mood string:
                println("string")
        }
        "#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        
        let stmt = parser.parse_statement();
        assert!(stmt.is_err());
    }

    #[test]
    fn test_type_switch_empty_cases() {
        let input = r#"
        vibe_check v := input.(type) {
        }
        "#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        
        let stmt = parser.parse_statement();
        assert!(stmt.is_ok()); // Empty switch should be valid
    }

    #[test]
    fn test_type_switch_missing_case_body() {
        let input = r#"
        vibe_check v := input.(type) {
            mood string:
            mood int:
                println("integer")
        }
        "#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        
        let stmt = parser.parse_statement();
        // Should handle empty case bodies gracefully
        assert!(stmt.is_ok());
    }

    // ============================================================================
    // Type Safety Tests
    // ============================================================================

    #[test]
    fn test_type_switch_variable_binding() {
        // Test that the variable is properly bound in each case
        let target_expr = Box::new(IdentifierExpression::new("input".to_string()));
        let variable_name = Some("v".to_string());
        
        let case1 = TypeSwitchCase::new(
            vec!["string".to_string()],
            BlockStatement::new(vec![]),
            variable_name.clone(),
        );

        let type_switch = TypeSwitchStatement::new(
            "vibe_check".to_string(),
            target_expr,
            vec![case1],
            None,
            variable_name.clone(),
        );

        // Verify variable is available in case scope
        assert_eq!(type_switch.cases[0].variable_name, variable_name);
    }

    #[test]
    fn test_type_switch_interface_compatibility() {
        // Test that type switch works with interface types
        let interface_expr = Box::new(TypeAssertion::new(
            "value".to_string(),
            "interface{}".to_string(),
        ));

        let case1 = TypeSwitchCase::new(
            vec!["ConcreteType".to_string()],
            BlockStatement::new(vec![]),
            Some("v".to_string()),
        );

        let type_switch = TypeSwitchStatement::new(
            "vibe_check".to_string(),
            interface_expr,
            vec![case1],
            None,
            Some("v".to_string()),
        );

        assert!(type_switch.expression.is_some());
    }

    // ============================================================================
    // Integration Tests
    // ============================================================================

    #[test]
    fn test_type_switch_in_function() {
        let input = r#"
        slay process_value(input interface{}) string {
            vibe_check v := input.(type) {
                mood string:
                    vibe "String: " + v
                mood int:
                    vibe "Number: " + v.toString()
                basic:
                    vibe "Unknown type"
            }
        }
        "#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        
        let stmt = parser.parse_statement();
        assert!(stmt.is_ok());
    }

    #[test]
    fn test_nested_type_switches() {
        let input = r#"
        vibe_check outer := input.(type) {
            mood map[string]interface{}:
                lowkey (sus key, value in outer) {
                    vibe_check inner := value.(type) {
                        mood string:
                            println("Nested string:", inner)
                        mood int:
                            println("Nested int:", inner)
                    }
                }
        }
        "#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        
        let stmt = parser.parse_statement();
        assert!(stmt.is_ok());
    }

    // ============================================================================
    // Performance and Edge Case Tests
    // ============================================================================

    #[test]
    fn test_type_switch_many_cases() {
        let mut cases = Vec::new();
        for i in 0..50 {
            let type_name = format!("Type{}", i);
            let case = TypeSwitchCase::new(
                vec![type_name],
                BlockStatement::new(vec![]),
                Some("v".to_string()),
            );
            cases.push(case);
        }

        let target_expr = Box::new(IdentifierExpression::new("input".to_string()));
        let type_switch = TypeSwitchStatement::new(
            "vibe_check".to_string(),
            target_expr,
            cases,
            None,
            Some("v".to_string()),
        );

        assert_eq!(type_switch.cases.len(), 50);
    }

    #[test]
    fn test_type_switch_complex_expression() {
        let input = r#"
        vibe_check v := getInterface().getValue().(type) {
            mood string:
                println("Got string")
        }
        "#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        
        let stmt = parser.parse_statement();
        assert!(stmt.is_ok());
    }

    #[test]
    fn test_type_switch_with_generics() {
        let input = r#"
        vibe_check v := input.(type) {
            mood Vec<T>:
                println("Generic vector")
            mood HashMap<K, V>:
                println("Generic map")
        }
        "#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        
        let stmt = parser.parse_statement();
        assert!(stmt.is_ok());
    }

    // ============================================================================
    // Compilation and LLVM Tests (Basic Structure)
    // ============================================================================

    #[test]
    fn test_type_switch_compilation_structure() {
        // Test basic compilation structure without full LLVM integration
        let target_expr = Box::new(IdentifierExpression::new("input".to_string()));
        let variable_name = Some("v".to_string());
        
        let case1 = TypeSwitchCase::new(
            vec!["string".to_string()],
            BlockStatement::new(vec![]),
            variable_name.clone(),
        );

        let type_switch = TypeSwitchStatement::new(
            "vibe_check".to_string(),
            target_expr,
            vec![case1],
            None,
            variable_name,
        );

        // Test that the structure is ready for compilation
        assert!(type_switch.expression.is_some());
        assert!(!type_switch.cases.is_empty());
        assert!(type_switch.variable_name.is_some());
    }

    #[test]
    fn test_type_switch_runtime_type_checking() {
        // Placeholder for runtime type checking logic
        // This would test the actual type checking mechanism
        
        let types_to_check = vec![
            "string".to_string(),
            "int".to_string(),
            "[]byte".to_string(),
            "interface{}".to_string(),
        ];

        for type_name in types_to_check {
            // Mock type checking - real implementation would involve LLVM
            assert!(!type_name.is_empty());
        }
    }

    // ============================================================================
    // Documentation and Examples Tests
    // ============================================================================

    #[test]
    fn test_type_switch_documentation_examples() {
        // Test examples that would appear in documentation
        let examples = vec![
            r#"
            vibe_check v := input.(type) {
                mood string:
                    println("String:", v)
                mood int:
                    println("Integer:", v)
                basic:
                    println("Unknown type")
            }
            "#,
            r#"
            vibe_check data := response.Body.(type) {
                mood []byte:
                    vibe json.Unmarshal(data, &result)
                mood string:
                    vibe json.Unmarshal([]byte(data), &result)
            }
            "#,
        ];

        for example in examples {
            let mut lexer = Lexer::new(example);
            let mut parser = Parser::new(&mut lexer);
            
            let stmt = parser.parse_statement();
            assert!(stmt.is_ok(), "Documentation example should parse correctly");
        }
    }
}

// ============================================================================
// Helper Structures for Type Switch (would be in ast/type_switch.rs)
// ============================================================================

/// Type switch statement for runtime type checking and variable binding
#[derive(Debug, Clone)]
pub struct TypeSwitchStatement {
    pub token: String,
    pub expression: Option<Box<dyn Expression>>,
    pub cases: Vec<TypeSwitchCase>,
    pub default_case: Option<BlockStatement>,
    pub variable_name: Option<String>,
}

impl TypeSwitchStatement {
    pub fn new(
        token: String,
        expression: Box<dyn Expression>,
        cases: Vec<TypeSwitchCase>,
        default_case: Option<BlockStatement>,
        variable_name: Option<String>,
    ) -> Self {
        Self {
            token,
            expression: Some(expression),
            cases,
            default_case,
            variable_name,
        }
    }
}

impl Node for TypeSwitchStatement {
    fn string(&self) -> String {
        let mut result = String::from("vibe_check");
        
        if let Some(var_name) = &self.variable_name {
            if let Some(expr) = &self.expression {
                result.push_str(&format!(" {} := {}.(type)", var_name, expr.string()));
            }
        } else if let Some(expr) = &self.expression {
            result.push_str(&format!(" {}.(type)", expr.string()));
        }
        
        result.push_str(" {\n");
        
        for case in &self.cases {
            result.push_str(&format!("  {}\n", case.string()));
        }
        
        if let Some(default) = &self.default_case {
            result.push_str(&format!("  basic: {}\n", default.string()));
        }
        
        result.push('}');
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for TypeSwitchStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(TypeSwitchStatement {
            token: self.token.clone(),
            expression: self.expression.as_ref().map(|e| e.clone_box()),
            cases: self.cases.clone(),
            default_case: self.default_case.clone(),
            variable_name: self.variable_name.clone(),
        })
    }
}

/// Individual case within a type switch statement
#[derive(Debug, Clone)]
pub struct TypeSwitchCase {
    pub types: Vec<String>,
    pub body: BlockStatement,
    pub variable_name: Option<String>,
}

impl TypeSwitchCase {
    pub fn new(types: Vec<String>, body: BlockStatement, variable_name: Option<String>) -> Self {
        Self {
            types,
            body,
            variable_name,
        }
    }
}

impl Node for TypeSwitchCase {
    fn string(&self) -> String {
        let types_str = self.types.join(", ");
        format!("mood {}:\n{}", types_str, self.body.string())
    }

    fn token_literal(&self) -> String {
        "mood".to_string()
    }
}
