//! Integration tests for CURSED type system
//! 
//! Tests the complete type checking pipeline including inference,
//! constraint resolution, and variance checking.

use crate::ast::*;
use crate::error::CursedError;
use super::{TypeChecker, TypeSystem, TypeInference};
use crate::type_system::variance::VarianceAnalyzer;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_complete_type_checking_pipeline() {
        let mut checker = TypeChecker::new();
        
        // Create a simple program with type inference
        let program = create_test_program();
        
        // Run complete type checking
        let result = checker.check_program(&program);
        
        match result {
            Ok(()) => {
                println!("✓ Type checking passed");
            }
            Err(errors) => {
                println!("✗ Type checking failed with {} errors:", errors.len());
                for error in errors {
                    println!("  - {}", error.message);
                }
                assert!(false, "Type checking should not fail for valid program");
            }
        }
    }
    
    #[test]
    fn test_type_inference_with_constraints() {
        let mut inference = TypeInference::new();
        
        // Test binary expression type inference
        let expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Integer(1)),
            operator: "+".to_string(),
            right: Box::new(Expression::Integer(2)),
        });
        
        let result = inference.infer_expression_type(&expr).unwrap();
        assert_eq!(result.name, Some("int".to_string()));
        
        // Test array type inference
        let array_expr = Expression::Array(vec![
            Expression::Integer(1),
            Expression::Integer(2),
            Expression::Integer(3),
        ]);
        
        let array_result = inference.infer_expression_type(&array_expr).unwrap();
        assert_eq!(array_result.name, Some("Array".to_string()));
        assert_eq!(array_result.parameters.len(), 1);
        assert_eq!(array_result.parameters[0].name, Some("int".to_string()));
    }
    
    #[test]
    fn test_function_type_inference() {
        let mut checker = TypeChecker::new();
        
        // Test function with return statement
        let func_stmt = FunctionStatement {
            name: "add".to_string(),
            type_parameters: vec![],
            parameters: vec![
                Parameter { name: "x".to_string(), param_type: None },
                Parameter { name: "y".to_string(), param_type: None }
            ],
            body: vec![
                Statement::Return(ReturnStatement {
                    value: Some(Expression::Binary(BinaryExpression {
                        left: Box::new(Expression::Identifier("x".to_string())),
                        operator: "+".to_string(),
                        right: Box::new(Expression::Identifier("y".to_string())),
                    })),
                })
            ],
            return_type: None,
            where_clause: None,
            visibility: crate::ast::Visibility::Private,
        };
        
        let result = checker.check_function_complete(&func_stmt).unwrap();
        assert!(result.return_type.is_some());
    }
    
    #[test]
    fn test_method_call_type_checking() {
        let mut checker = TypeChecker::new();
        
        // Test vibez.spill() method call
        let method_call = Expression::Call(CallExpression {
            function: Box::new(Expression::MemberAccess(MemberAccessExpression {
                object: Box::new(Expression::Identifier("vibez".to_string())),
                property: "spill".to_string(),
            })),
            arguments: vec![Expression::String("hello world".to_string())],
        });
        
        let result = checker.check_expression(&method_call).unwrap();
        assert_eq!(result.name, Some("void".to_string()));
    }
    
    #[test]
    fn test_type_error_detection() {
        let mut checker = TypeChecker::new();
        
        // Test type mismatch in binary operation
        let bad_expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Integer(1)),
            operator: "+".to_string(),
            right: Box::new(Expression::String("hello".to_string())),
        });
        
        let result = checker.check_expression(&bad_expr);
        assert!(result.is_err());
        
        // Test undefined variable
        let undefined_expr = Expression::Identifier("nonexistent".to_string());
        let result = checker.check_expression(&undefined_expr);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_variance_analysis_integration() {
        let mut analyzer = VarianceAnalyzer::new();
        
        // Create a simple generic type for testing
        let array_type = super::super::TypeDefinition {
            name: "Array".to_string(),
            kind: super::super::TypeKind::Struct,
            type_parameters: vec!["T".to_string()],
            constraints: Vec::new(),
            methods: vec![
                super::super::MethodSignature {
                    name: "get".to_string(),
                    parameters: vec![super::super::TypeExpression::named("int")],
                    return_type: Some(super::super::TypeExpression::named("T")),
                    type_parameters: Vec::new(),
                    constraints: Vec::new(),
                }
            ],
            is_builtin: false,
        };
        
        let variances = analyzer.compute_variance(&array_type).unwrap();
        assert_eq!(variances.len(), 1);
        // Array should be covariant in T since T only appears in return position
        assert_eq!(variances[0], super::super::variance::Variance::Covariant);
    }
    
    #[test]
    fn test_constraint_resolution() {
        let mut system = TypeSystem::new();
        
        // Test basic type compatibility
        let int_type = super::super::TypeExpression::named("int");
        let string_type = super::super::TypeExpression::named("string");
        
        assert!(system.types_compatible(&int_type, &int_type));
        assert!(!system.types_compatible(&int_type, &string_type));
    }
    
    #[test]
    fn test_complex_expression_typing() {
        let mut checker = TypeChecker::new();
        
        // Test nested expressions
        let complex_expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Binary(BinaryExpression {
                left: Box::new(Expression::Integer(1)),
                operator: "+".to_string(),
                right: Box::new(Expression::Integer(2)),
            })),
            operator: "*".to_string(),
            right: Box::new(Expression::Integer(3)),
        });
        
        let result = checker.check_expression(&complex_expr).unwrap();
        assert_eq!(result.name, Some("int".to_string()));
        
        // Test comparison expression
        let comparison_expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Integer(1)),
            operator: "<".to_string(),
            right: Box::new(Expression::Integer(2)),
        });
        
        let result = checker.check_expression(&comparison_expr).unwrap();
        assert_eq!(result.name, Some("bool".to_string()));
    }
    
    #[test]
    fn test_if_statement_type_checking() {
        let mut checker = TypeChecker::new();
        
        let if_stmt = IfStatement {
            condition: Expression::Boolean(true),
            then_branch: vec![
                Statement::Expression(Expression::Integer(42))
            ],
            else_branch: Some(vec![
                Statement::Expression(Expression::String("hello".to_string()))
            ]),
        };
        
        let result = checker.check_if_statement(&if_stmt);
        assert!(result.is_ok());
        
        // Test invalid condition type
        let bad_if_stmt = IfStatement {
            condition: Expression::Integer(42), // Should be bool
            then_branch: vec![],
            else_branch: None,
        };
        
        let result = checker.check_if_statement(&bad_if_stmt);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_while_statement_type_checking() {
        let mut checker = TypeChecker::new();
        
        let while_stmt = WhileStatement {
            condition: Expression::Boolean(true),
            body: vec![
                Statement::Expression(Expression::Integer(42))
            ],
        };
        
        let result = checker.check_while_statement(&while_stmt);
        assert!(result.is_ok());
        
        // Test invalid condition type
        let bad_while_stmt = WhileStatement {
            condition: Expression::String("not a bool".to_string()),
            body: vec![],
        };
        
        let result = checker.check_while_statement(&bad_while_stmt);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_let_statement_type_checking() {
        let mut checker = TypeChecker::new();
        
        let let_stmt = LetStatement {
            name: "x".to_string(),
            value: Expression::Integer(42),
            var_type: None,
            visibility: crate::ast::Visibility::Private,
        };
        
        let result = checker.check_let_statement(&let_stmt).unwrap();
        assert_eq!(result.name, Some("int".to_string()));
        
        // Verify variable is added to scope
        let identifier_expr = Expression::Identifier("x".to_string());
        let lookup_result = checker.check_expression(&identifier_expr).unwrap();
        assert_eq!(lookup_result.name, Some("int".to_string()));
    }
    
    #[test]
    fn test_array_type_checking() {
        let mut checker = TypeChecker::new();
        
        // Homogeneous array
        let array_expr = Expression::Array(vec![
            Expression::Integer(1),
            Expression::Integer(2),
            Expression::Integer(3),
        ]);
        
        let result = checker.check_expression(&array_expr).unwrap();
        assert_eq!(result.name, Some("Array".to_string()));
        assert_eq!(result.parameters.len(), 1);
        assert_eq!(result.parameters[0].name, Some("int".to_string()));
        
        // Heterogeneous array (should fail)
        let bad_array_expr = Expression::Array(vec![
            Expression::Integer(1),
            Expression::String("hello".to_string()),
        ]);
        
        let result = checker.check_expression(&bad_array_expr);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_map_type_checking() {
        let mut checker = TypeChecker::new();
        
        // Homogeneous map
        let map_expr = Expression::Map(vec![
            (Expression::String("key1".to_string()), Expression::Integer(1)),
            (Expression::String("key2".to_string()), Expression::Integer(2)),
        ]);
        
        let result = checker.check_expression(&map_expr).unwrap();
        assert_eq!(result.name, Some("Map".to_string()));
        assert_eq!(result.parameters.len(), 2);
        assert_eq!(result.parameters[0].name, Some("string".to_string()));
        assert_eq!(result.parameters[1].name, Some("int".to_string()));
        
        // Mixed value types (should fail)
        let bad_map_expr = Expression::Map(vec![
            (Expression::String("key1".to_string()), Expression::Integer(1)),
            (Expression::String("key2".to_string()), Expression::String("value".to_string())),
        ]);
        
        let result = checker.check_expression(&bad_map_expr);
        assert!(result.is_err());
    }
    
    // Helper function to create a test program
    fn create_test_program() -> Program {
        Program {
            statements: vec![
                Statement::Let(LetStatement {
                    name: "x".to_string(),
                    value: Expression::Integer(42),
                    var_type: None,
                    visibility: crate::ast::Visibility::Private,
                }),
                Statement::Let(LetStatement {
                    name: "message".to_string(),
                    value: Expression::String("Hello, CURSED!".to_string()),
                    var_type: None,
                    visibility: crate::ast::Visibility::Private,
                }),
                Statement::Expression(Expression::Call(CallExpression {
                    function: Box::new(Expression::MemberAccess(MemberAccessExpression {
                        object: Box::new(Expression::Identifier("vibez".to_string())),
                        property: "spill".to_string(),
                    })),
                    arguments: vec![Expression::Identifier("message".to_string())],
                })),
                Statement::Function(FunctionStatement {
                    name: "add".to_string(),
                    type_parameters: vec![],
                    parameters: vec![
                        Parameter { name: "a".to_string(), param_type: None },
                        Parameter { name: "b".to_string(), param_type: None }
                    ],
                    body: vec![
                        Statement::Return(ReturnStatement {
                            value: Some(Expression::Binary(BinaryExpression {
                                left: Box::new(Expression::Identifier("a".to_string())),
                                operator: "+".to_string(),
                                right: Box::new(Expression::Identifier("b".to_string())),
                            })),
                        })
                    ],
                    return_type: None,
                    where_clause: None,
                    visibility: crate::ast::Visibility::Private,
                }),
                Statement::If(IfStatement {
                    condition: Expression::Binary(BinaryExpression {
                        left: Box::new(Expression::Identifier("x".to_string())),
                        operator: ">".to_string(),
                        right: Box::new(Expression::Integer(0)),
                    }),
                    then_branch: vec![
                        Statement::Expression(Expression::Call(CallExpression {
                            function: Box::new(Expression::MemberAccess(MemberAccessExpression {
                                object: Box::new(Expression::Identifier("vibez".to_string())),
                                property: "spill".to_string(),
                            })),
                            arguments: vec![Expression::String("x is positive".to_string())],
                        }))
                    ],
                    else_branch: Some(vec![
                        Statement::Expression(Expression::Call(CallExpression {
                            function: Box::new(Expression::MemberAccess(MemberAccessExpression {
                                object: Box::new(Expression::Identifier("vibez".to_string())),
                                property: "spill".to_string(),
                            })),
                            arguments: vec![Expression::String("x is not positive".to_string())],
                        }))
                    ]),
                }),
            ],
            imports: Vec::new(),
            package: None,
        }
    }
}
