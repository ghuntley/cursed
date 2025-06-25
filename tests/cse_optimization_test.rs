/// Common Subexpression Elimination (CSE) Tests
/// 
/// Comprehensive test suite for the CSE optimization pass including:
/// - Basic CSE functionality
/// - Value numbering correctness
/// - Dominance analysis
/// - Local vs global CSE
/// - Complex expression patterns
/// - Performance characteristics

use cursed::optimization::passes::cse::*;
use cursed::optimization::passes::*;
use cursed::ast::*;
use cursed::error::Result;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create test expressions
    fn create_binary_expr(left: Expression, op: BinaryOperator, right: Expression) -> Expression {
        Expression::Binary(Box::new(BinaryExpression {
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        }))
    }

    fn create_variable(name: &str) -> Expression {
        Expression::Variable(Variable {
            name: name.to_string(),
            var_type: None,
        })
    }

    fn create_literal(value: i64) -> Expression {
        Expression::Literal(Literal::Integer(value))
    }

    fn create_test_function(name: &str, body: Vec<Statement>) -> Function {
        Function {
            name: name.to_string(),
            parameters: vec![],
            return_type: None,
            body,
            visibility: Visibility::Public,
            generics: None,
        }
    }

    fn create_test_module(name: &str, functions: Vec<Function>) -> Module {
        Module {
            name: name.to_string(),
            functions,
            structs: vec![],
            interfaces: vec![],
            imports: vec![],
            exports: vec![],
            visibility: Visibility::Public,
        }
    }

    fn create_test_program(modules: Vec<Module>) -> Program {
        Program { modules }
    }

    #[test]
    fn test_basic_cse_elimination() {
        let mut pass = CommonSubexpressionEliminationPass::new();
        
        // Create program with redundant expressions: a + b used twice
        let mut program = create_test_program(vec![
            create_test_module("test", vec![
                create_test_function("test_func", vec![
                    Statement::VariableDeclaration(VariableDeclaration {
                        name: "x".to_string(),
                        var_type: None,
                        initializer: Some(create_binary_expr(
                            create_variable("a"),
                            BinaryOperator::Add,
                            create_variable("b")
                        )),
                        is_mutable: false,
                    }),
                    Statement::VariableDeclaration(VariableDeclaration {
                        name: "y".to_string(),
                        var_type: None,
                        initializer: Some(create_binary_expr(
                            create_variable("a"),
                            BinaryOperator::Add,
                            create_variable("b")
                        )),
                        is_mutable: false,
                    }),
                ])
            ])
        ]);

        let eliminated = pass.eliminate_common_subexpressions(&mut program).unwrap();
        
        assert!(eliminated > 0, "Should eliminate redundant expressions");
        println!("Basic CSE: Eliminated {} expressions", eliminated);
    }

    #[test]
    fn test_value_numbering_consistency() {
        let mut context = CseContext::new(0);
        let mut pass = CommonSubexpressionEliminationPass::new();
        
        // Test that identical expressions get same value numbers
        let expr1 = create_binary_expr(
            create_variable("x"),
            BinaryOperator::Multiply,
            create_literal(2)
        );
        
        let expr2 = create_binary_expr(
            create_variable("x"),
            BinaryOperator::Multiply,
            create_literal(2)
        );
        
        let vn1 = pass.number_expression(&expr1, &mut context, 0, 0).unwrap();
        let vn2 = pass.number_expression(&expr2, &mut context, 0, 1).unwrap();
        
        assert_eq!(vn1, vn2, "Identical expressions should have same value number");
        
        // Test that different expressions get different value numbers
        let expr3 = create_binary_expr(
            create_variable("x"),
            BinaryOperator::Multiply,
            create_literal(3)
        );
        
        let vn3 = pass.number_expression(&expr3, &mut context, 0, 2).unwrap();
        assert_ne!(vn1, vn3, "Different expressions should have different value numbers");
    }

    #[test]
    fn test_nested_expressions() {
        let mut pass = CommonSubexpressionEliminationPass::new();
        
        // Create nested expressions: (a + b) * c and (a + b) * d
        let common_subexpr = create_binary_expr(
            create_variable("a"),
            BinaryOperator::Add,
            create_variable("b")
        );
        
        let mut program = create_test_program(vec![
            create_test_module("test", vec![
                create_test_function("nested_test", vec![
                    Statement::VariableDeclaration(VariableDeclaration {
                        name: "x".to_string(),
                        var_type: None,
                        initializer: Some(create_binary_expr(
                            common_subexpr.clone(),
                            BinaryOperator::Multiply,
                            create_variable("c")
                        )),
                        is_mutable: false,
                    }),
                    Statement::VariableDeclaration(VariableDeclaration {
                        name: "y".to_string(),
                        var_type: None,
                        initializer: Some(create_binary_expr(
                            common_subexpr,
                            BinaryOperator::Multiply,
                            create_variable("d")
                        )),
                        is_mutable: false,
                    }),
                ])
            ])
        ]);

        let eliminated = pass.eliminate_common_subexpressions(&mut program).unwrap();
        
        assert!(eliminated > 0, "Should eliminate common subexpressions in nested expressions");
        println!("Nested CSE: Eliminated {} expressions", eliminated);
    }

    #[test]
    fn test_dominance_analysis() {
        let mut cfg = ControlFlowGraph::new(0);
        
        // Create CFG with proper dominance relationships
        cfg.blocks.insert(0, BasicBlock::new(0));
        cfg.blocks.insert(1, BasicBlock::new(1));
        cfg.blocks.insert(2, BasicBlock::new(2));
        cfg.blocks.insert(3, BasicBlock::new(3));
        
        // Set up control flow: 0 -> {1, 2}, 1 -> 3, 2 -> 3
        cfg.blocks.get_mut(&0).unwrap().successors = vec![1, 2];
        cfg.blocks.get_mut(&1).unwrap().predecessors = vec![0];
        cfg.blocks.get_mut(&1).unwrap().successors = vec![3];
        cfg.blocks.get_mut(&2).unwrap().predecessors = vec![0];
        cfg.blocks.get_mut(&2).unwrap().successors = vec![3];
        cfg.blocks.get_mut(&3).unwrap().predecessors = vec![1, 2];
        
        cfg.compute_dominance();
        
        // Verify dominance relationships
        assert!(cfg.dominates(0, 0), "Block should dominate itself");
        assert!(cfg.dominates(0, 1), "Entry block should dominate all blocks");
        assert!(cfg.dominates(0, 2), "Entry block should dominate all blocks");
        assert!(cfg.dominates(0, 3), "Entry block should dominate all blocks");
        
        assert!(!cfg.dominates(1, 2), "Sibling blocks should not dominate each other");
        assert!(!cfg.dominates(2, 1), "Sibling blocks should not dominate each other");
        assert!(!cfg.dominates(1, 3), "Block 1 should not dominate block 3 (has multiple predecessors)");
        assert!(!cfg.dominates(2, 3), "Block 2 should not dominate block 3 (has multiple predecessors)");
        
        println!("Dominance analysis test passed");
    }

    #[test]
    fn test_immediate_dominators() {
        let mut cfg = ControlFlowGraph::new(0);
        
        // Linear CFG: 0 -> 1 -> 2 -> 3
        cfg.blocks.insert(0, BasicBlock::new(0));
        cfg.blocks.insert(1, BasicBlock::new(1));
        cfg.blocks.insert(2, BasicBlock::new(2));
        cfg.blocks.insert(3, BasicBlock::new(3));
        
        cfg.blocks.get_mut(&0).unwrap().successors = vec![1];
        cfg.blocks.get_mut(&1).unwrap().predecessors = vec![0];
        cfg.blocks.get_mut(&1).unwrap().successors = vec![2];
        cfg.blocks.get_mut(&2).unwrap().predecessors = vec![1];
        cfg.blocks.get_mut(&2).unwrap().successors = vec![3];
        cfg.blocks.get_mut(&3).unwrap().predecessors = vec![2];
        
        cfg.compute_dominance();
        
        // Check immediate dominators
        assert_eq!(cfg.blocks[&1].immediate_dominator, Some(0));
        assert_eq!(cfg.blocks[&2].immediate_dominator, Some(1));
        assert_eq!(cfg.blocks[&3].immediate_dominator, Some(2));
        
        println!("Immediate dominator test passed");
    }

    #[test]
    fn test_local_vs_global_cse() {
        let create_test_expr = || create_binary_expr(
            create_variable("a"),
            BinaryOperator::Add,
            create_variable("b")
        );
        
        let mut program = create_test_program(vec![
            create_test_module("test", vec![
                create_test_function("test_func", vec![
                    Statement::VariableDeclaration(VariableDeclaration {
                        name: "x".to_string(),
                        var_type: None,
                        initializer: Some(create_test_expr()),
                        is_mutable: false,
                    }),
                    Statement::If(IfStatement {
                        condition: create_variable("condition"),
                        then_branch: vec![
                            Statement::VariableDeclaration(VariableDeclaration {
                                name: "y".to_string(),
                                var_type: None,
                                initializer: Some(create_test_expr()),
                                is_mutable: false,
                            })
                        ],
                        else_branch: None,
                    }),
                ])
            ])
        ]);

        // Test local CSE
        let mut local_pass = CommonSubexpressionEliminationPass::with_config(false, false);
        let mut program_local = program.clone();
        let local_eliminated = local_pass.eliminate_common_subexpressions(&mut program_local).unwrap();
        
        // Test global CSE
        let mut global_pass = CommonSubexpressionEliminationPass::with_config(true, false);
        let mut program_global = program;
        let global_eliminated = global_pass.eliminate_common_subexpressions(&mut program_global).unwrap();
        
        println!("Local CSE eliminated: {}", local_eliminated);
        println!("Global CSE eliminated: {}", global_eliminated);
        
        // Global CSE should potentially eliminate more (or same) expressions
        assert!(global_eliminated >= local_eliminated, 
                "Global CSE should eliminate at least as many expressions as local CSE");
    }

    #[test]
    fn test_complex_expression_patterns() {
        let mut pass = CommonSubexpressionEliminationPass::new();
        
        // Create complex expressions with multiple levels of nesting
        let base_expr = create_binary_expr(
            create_variable("a"),
            BinaryOperator::Add,
            create_variable("b")
        );
        
        let complex_expr1 = create_binary_expr(
            base_expr.clone(),
            BinaryOperator::Multiply,
            create_binary_expr(
                create_variable("c"),
                BinaryOperator::Subtract,
                create_literal(1)
            )
        );
        
        let complex_expr2 = create_binary_expr(
            base_expr,
            BinaryOperator::Multiply,
            create_binary_expr(
                create_variable("c"),
                BinaryOperator::Subtract,
                create_literal(1)
            )
        );
        
        let mut program = create_test_program(vec![
            create_test_module("test", vec![
                create_test_function("complex_test", vec![
                    Statement::VariableDeclaration(VariableDeclaration {
                        name: "result1".to_string(),
                        var_type: None,
                        initializer: Some(complex_expr1),
                        is_mutable: false,
                    }),
                    Statement::VariableDeclaration(VariableDeclaration {
                        name: "result2".to_string(),
                        var_type: None,
                        initializer: Some(complex_expr2),
                        is_mutable: false,
                    }),
                ])
            ])
        ]);

        let eliminated = pass.eliminate_common_subexpressions(&mut program).unwrap();
        
        assert!(eliminated > 0, "Should eliminate common subexpressions in complex patterns");
        println!("Complex patterns CSE: Eliminated {} expressions", eliminated);
    }

    #[test]
    fn test_function_call_expressions() {
        let mut pass = CommonSubexpressionEliminationPass::new();
        
        // Function calls with same arguments should be treated as common subexpressions
        let func_call1 = Expression::FunctionCall(FunctionCall {
            function_name: "compute".to_string(),
            arguments: vec![
                create_variable("x"),
                create_literal(42),
            ],
        });
        
        let func_call2 = Expression::FunctionCall(FunctionCall {
            function_name: "compute".to_string(),
            arguments: vec![
                create_variable("x"),
                create_literal(42),
            ],
        });
        
        let mut program = create_test_program(vec![
            create_test_module("test", vec![
                create_test_function("func_call_test", vec![
                    Statement::VariableDeclaration(VariableDeclaration {
                        name: "result1".to_string(),
                        var_type: None,
                        initializer: Some(func_call1),
                        is_mutable: false,
                    }),
                    Statement::VariableDeclaration(VariableDeclaration {
                        name: "result2".to_string(),
                        var_type: None,
                        initializer: Some(func_call2),
                        is_mutable: false,
                    }),
                ])
            ])
        ]);

        let eliminated = pass.eliminate_common_subexpressions(&mut program).unwrap();
        
        assert!(eliminated > 0, "Should eliminate redundant function calls");
        println!("Function call CSE: Eliminated {} expressions", eliminated);
    }

    #[test]
    fn test_array_access_expressions() {
        let mut pass = CommonSubexpressionEliminationPass::new();
        
        // Array accesses with same index should be common subexpressions
        let array_access1 = Expression::ArrayAccess(Box::new(ArrayAccess {
            array: Box::new(create_variable("arr")),
            index: Box::new(create_binary_expr(
                create_variable("i"),
                BinaryOperator::Add,
                create_literal(1)
            )),
        }));
        
        let array_access2 = Expression::ArrayAccess(Box::new(ArrayAccess {
            array: Box::new(create_variable("arr")),
            index: Box::new(create_binary_expr(
                create_variable("i"),
                BinaryOperator::Add,
                create_literal(1)
            )),
        }));
        
        let mut program = create_test_program(vec![
            create_test_module("test", vec![
                create_test_function("array_test", vec![
                    Statement::VariableDeclaration(VariableDeclaration {
                        name: "val1".to_string(),
                        var_type: None,
                        initializer: Some(array_access1),
                        is_mutable: false,
                    }),
                    Statement::VariableDeclaration(VariableDeclaration {
                        name: "val2".to_string(),
                        var_type: None,
                        initializer: Some(array_access2),
                        is_mutable: false,
                    }),
                ])
            ])
        ]);

        let eliminated = pass.eliminate_common_subexpressions(&mut program).unwrap();
        
        assert!(eliminated > 0, "Should eliminate redundant array accesses");
        println!("Array access CSE: Eliminated {} expressions", eliminated);
    }

    #[test]
    fn test_temp_variable_generation() {
        let mut context = CseContext::new(0);
        
        let vn1 = ValueNumber::new(1);
        let vn2 = ValueNumber::new(2);
        let vn3 = ValueNumber::new(1); // Same as vn1
        
        let temp1 = context.get_temp_variable(vn1);
        let temp2 = context.get_temp_variable(vn2);
        let temp3 = context.get_temp_variable(vn3);
        
        assert_ne!(temp1, temp2, "Different value numbers should get different temp variables");
        assert_eq!(temp1, temp3, "Same value numbers should get same temp variable");
        assert!(temp1.starts_with("__cse_temp_"), "Temp variables should have CSE prefix");
        assert!(temp2.starts_with("__cse_temp_"), "Temp variables should have CSE prefix");
        
        println!("Generated temp variables: {} and {}", temp1, temp2);
    }

    #[test]
    fn test_cse_statistics() {
        let mut stats = CseStatistics::new();
        
        stats.expressions_analyzed = 100;
        stats.expressions_eliminated = 30;
        stats.temp_variables_created = 15;
        stats.basic_blocks_processed = 5;
        stats.calculate_elimination_rate();
        
        assert_eq!(stats.elimination_rate, 30.0);
        
        let stats_string = format!("{}", stats);
        assert!(stats_string.contains("30.00%"));
        assert!(stats_string.contains("100"));
        assert!(stats_string.contains("30"));
        assert!(stats_string.contains("15"));
        assert!(stats_string.contains("5"));
        
        println!("CSE Statistics:\n{}", stats_string);
    }

    #[test]
    fn test_expression_signature_hashing() {
        use std::collections::HashMap;
        
        let mut signatures = HashMap::new();
        
        let sig1 = ExpressionSignature::Binary {
            op: BinaryOperator::Add,
            left: ValueNumber::new(1),
            right: ValueNumber::new(2),
        };
        
        let sig2 = ExpressionSignature::Binary {
            op: BinaryOperator::Add,
            left: ValueNumber::new(1),
            right: ValueNumber::new(2),
        };
        
        let sig3 = ExpressionSignature::Unary {
            op: UnaryOperator::Minus,
            operand: ValueNumber::new(1),
        };
        
        signatures.insert(sig1.clone(), "first");
        signatures.insert(sig2.clone(), "second"); // Should overwrite first
        signatures.insert(sig3, "third");
        
        assert_eq!(signatures.len(), 2, "Identical signatures should be treated as same key");
        assert_eq!(signatures.get(&sig1), Some(&"second"));
        
        println!("Expression signature hashing test passed");
    }

    #[test]
    fn test_literal_value_conversion() {
        let int_lit = Literal::Integer(42);
        let float_lit = Literal::Float(3.14);
        let bool_lit = Literal::Boolean(true);
        let string_lit = Literal::String("test".to_string());
        let null_lit = Literal::Null;
        
        let int_val = LiteralValue::from(&int_lit);
        let float_val = LiteralValue::from(&float_lit);
        let bool_val = LiteralValue::from(&bool_lit);
        let string_val = LiteralValue::from(&string_lit);
        let null_val = LiteralValue::from(&null_lit);
        
        assert_eq!(int_val, LiteralValue::Integer(42));
        assert_eq!(float_val, LiteralValue::Float("3.14".to_string()));
        assert_eq!(bool_val, LiteralValue::Boolean(true));
        assert_eq!(string_val, LiteralValue::String("test".to_string()));
        assert_eq!(null_val, LiteralValue::Null);
        
        println!("Literal value conversion test passed");
    }

    #[test]
    fn test_availability_checking() {
        let mut context = CseContext::new(0);
        let vn = ValueNumber::new(1);
        
        // Record value location
        context.value_locations.insert(vn, (0, 5));
        
        // Test availability at different locations
        assert!(!context.is_available(vn, 0, 3), "Should not be available before definition");
        assert!(!context.is_available(vn, 0, 5), "Should not be available at definition point");
        assert!(context.is_available(vn, 0, 7), "Should be available after definition in same block");
        
        println!("Availability checking test passed");
    }

    #[test]
    fn test_optimization_pass_integration() {
        let mut pass = CommonSubexpressionEliminationPass::new();
        
        assert_eq!(pass.name(), "common-subexpression-elimination");
        assert!(pass.description().contains("dominance"));
        assert!(pass.is_repeatable());
        assert!(pass.dependencies().contains(&"constant-folding".to_string()));
        
        // Test with empty program
        let mut empty_program = create_test_program(vec![]);
        let result = pass.run(&mut empty_program).unwrap();
        
        assert!(!result.changed);
        assert_eq!(result.transformations, 0);
        
        println!("Optimization pass integration test passed");
    }

    #[test]
    fn test_debug_mode() {
        let mut debug_pass = CommonSubexpressionEliminationPass::with_config(true, true);
        
        let mut program = create_test_program(vec![
            create_test_module("test", vec![
                create_test_function("debug_test", vec![
                    Statement::VariableDeclaration(VariableDeclaration {
                        name: "x".to_string(),
                        var_type: None,
                        initializer: Some(create_binary_expr(
                            create_variable("a"),
                            BinaryOperator::Add,
                            create_variable("b")
                        )),
                        is_mutable: false,
                    }),
                ])
            ])
        ]);

        // Debug mode should not crash
        let _eliminated = debug_pass.eliminate_common_subexpressions(&mut program).unwrap();
        
        println!("Debug mode test passed");
    }

    #[test]
    fn test_performance_characteristics() {
        let mut pass = CommonSubexpressionEliminationPass::new();
        
        // Create a program with many redundant expressions
        let num_expressions = 100;
        let mut statements = Vec::new();
        
        for i in 0..num_expressions {
            statements.push(Statement::VariableDeclaration(VariableDeclaration {
                name: format!("var_{}", i),
                var_type: None,
                initializer: Some(create_binary_expr(
                    create_variable("a"),
                    BinaryOperator::Add,
                    create_variable("b")
                )),
                is_mutable: false,
            }));
        }
        
        let mut program = create_test_program(vec![
            create_test_module("perf_test", vec![
                create_test_function("perf_func", statements)
            ])
        ]);

        let start_time = std::time::Instant::now();
        let eliminated = pass.eliminate_common_subexpressions(&mut program).unwrap();
        let duration = start_time.elapsed();
        
        assert!(eliminated > 0, "Should eliminate many redundant expressions");
        assert!(duration.as_millis() < 1000, "Should complete within reasonable time");
        
        println!("Performance test: Eliminated {} expressions in {:?}", eliminated, duration);
    }
}
