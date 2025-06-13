/// Comprehensive tests for CURSED template LLVM compilation
/// 
/// This test suite validates the complete LLVM compilation pipeline for the
/// CURSED template system, ensuring that templates can be compiled to
/// efficient LLVM IR and executed properly.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use cursed::codegen::llvm::template::{
    LlvmTemplateCompiler, TemplateCompiler, TemplateCompilationContext, 
    TemplateOptimizationLevel, CompiledTemplate, TemplateCompilationError,
    declare_template_runtime_functions, register_standard_filters
};
use cursed::codegen::llvm::{LlvmCodeGenerator, LlvmType, LlvmValue};
use cursed::stdlib::template::{
    TemplateAst, TemplateNode, TemplateExpression, FilterCall, BinaryOperator, UnaryOperator,
    TemplateConfig, TemplateContext, SecurityLevel, OutputFormat, BlockNode
};
use cursed::stdlib::template::template_syntax::{MatchCase, SourceLocation};
use cursed::object::Object as CursedObject;
use cursed::error::Error as CursedError;

#[path = "common.rs"]
mod common;

/// Helper to create a basic template compilation context
fn create_test_context() -> TemplateCompilationContext {
    let mut context = TemplateCompilationContext::new(
        "test_template".to_string(),
        TemplateConfig::default()
    );
    context.optimization_level = TemplateOptimizationLevel::Basic;
    context.security_level = SecurityLevel::Strict;
    context.output_format = OutputFormat::Html;
    
    // Add some test variables
    context.add_variable("user".to_string(), LlvmType::String);
    context.add_variable("count".to_string(), LlvmType::Int64);
    context.add_variable("active".to_string(), LlvmType::Boolean);
    
    context
}

/// Helper to create a basic LLVM template compiler
fn create_test_compiler() -> LlvmTemplateCompiler {
    let generator = Arc::new(LlvmCodeGenerator::new());
    let mut compiler = LlvmTemplateCompiler::new(generator);
    
    // Register some test filters
    let mut context = create_test_context();
    register_standard_filters(&mut context);
    
    compiler
}

#[test]
fn test_template_literal_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let result = compiler.compile_template_literal("Hello, World!", &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::String);
    assert!(llvm_value.is_constant);
    assert!(llvm_value.llvm_name.starts_with("%literal_"));
}

#[test] 
fn test_template_literal_with_escaping() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let mut context = create_test_context();
    context.security_level = SecurityLevel::Strict;
    context.output_format = OutputFormat::Html;
    
    let result = compiler.compile_template_literal("<script>alert('xss')</script>", &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::String);
}

#[test]
fn test_variable_expression_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let var_expr = TemplateExpression::Variable("user".to_string());
    let result = compiler.compile_template_expression(&var_expr, &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::String);
    assert!(!llvm_value.is_constant);
}

#[test]
fn test_literal_expression_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    // Test string literal
    let string_expr = TemplateExpression::Literal(CursedObject::String("test".to_string()));
    let result = compiler.compile_template_expression(&string_expr, &context);
    assert!(result.is_ok());
    
    // Test integer literal  
    let int_expr = TemplateExpression::Literal(CursedObject::Integer(42));
    let result = compiler.compile_template_expression(&int_expr, &context);
    assert!(result.is_ok());
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Int64);
    
    // Test boolean literal
    let bool_expr = TemplateExpression::Literal(CursedObject::Boolean(true));
    let result = compiler.compile_template_expression(&bool_expr, &context);
    assert!(result.is_ok());
}

#[test]
fn test_binary_operation_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let left = TemplateExpression::Literal(CursedObject::Integer(5));
    let right = TemplateExpression::Literal(CursedObject::Integer(3));
    let binary_expr = TemplateExpression::Binary {
        left: Box::new(left),
        operator: BinaryOperator::Add,
        right: Box::new(right),
    };
    
    let result = compiler.compile_template_expression(&binary_expr, &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::String);
    assert!(!llvm_value.is_constant);
}

#[test]
fn test_cursed_binary_operations() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let left = TemplateExpression::Variable("user".to_string());
    let right = TemplateExpression::Literal(CursedObject::String("admin".to_string()));
    
    // Test CURSED-style vibe operator
    let vibe_expr = TemplateExpression::Binary {
        left: Box::new(left.clone()),
        operator: BinaryOperator::Vibe,
        right: Box::new(right.clone()),
    };
    
    let result = compiler.compile_template_expression(&vibe_expr, &context);
    assert!(result.is_ok());
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Boolean);
    
    // Test CURSED-style slay operator
    let slay_expr = TemplateExpression::Binary {
        left: Box::new(left),
        operator: BinaryOperator::Slay,
        right: Box::new(right),
    };
    
    let result = compiler.compile_template_expression(&slay_expr, &context);
    assert!(result.is_ok());
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Boolean);
}

#[test]
fn test_unary_operation_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let operand = TemplateExpression::Variable("active".to_string());
    let unary_expr = TemplateExpression::Unary {
        operator: UnaryOperator::Not,
        operand: Box::new(operand),
    };
    
    let result = compiler.compile_template_expression(&unary_expr, &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Boolean);
}

#[test]
fn test_cursed_unary_operations() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let operand = TemplateExpression::Variable("user".to_string());
    
    // Test sus (truthiness check)
    let sus_expr = TemplateExpression::Sus(Box::new(operand.clone()));
    let result = compiler.compile_template_expression(&sus_expr, &context);
    assert!(result.is_ok());
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Boolean);
    
    // Test cap (falsy check)
    let cap_expr = TemplateExpression::Cap(Box::new(operand.clone()));
    let result = compiler.compile_template_expression(&cap_expr, &context);
    assert!(result.is_ok());
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Boolean);
    
    // Test facts (type check)
    let facts_expr = TemplateExpression::Facts(Box::new(operand));
    let result = compiler.compile_template_expression(&facts_expr, &context);
    assert!(result.is_ok());
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::String);
}

#[test]
fn test_property_access_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let object = TemplateExpression::Variable("user".to_string());
    let prop_expr = TemplateExpression::PropertyAccess {
        object: Box::new(object),
        property: "name".to_string(),
    };
    
    let result = compiler.compile_template_expression(&prop_expr, &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::String);
    assert!(!llvm_value.is_constant);
}

#[test]
fn test_index_access_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let object = TemplateExpression::Variable("user".to_string());
    let index = TemplateExpression::Literal(CursedObject::Integer(0));
    let index_expr = TemplateExpression::IndexAccess {
        object: Box::new(object),
        index: Box::new(index),
    };
    
    let result = compiler.compile_template_expression(&index_expr, &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::String);
}

#[test]
fn test_function_call_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let args = vec![
        TemplateExpression::Variable("user".to_string()),
        TemplateExpression::Literal(CursedObject::String("format".to_string())),
    ];
    let func_expr = TemplateExpression::FunctionCall {
        name: "format_user".to_string(),
        arguments: args,
    };
    
    let result = compiler.compile_template_expression(&func_expr, &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::String);
}

#[test]
fn test_array_construction_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let elements = vec![
        TemplateExpression::Literal(CursedObject::Integer(1)),
        TemplateExpression::Literal(CursedObject::Integer(2)),
        TemplateExpression::Literal(CursedObject::Integer(3)),
    ];
    let array_expr = TemplateExpression::Array(elements);
    
    let result = compiler.compile_template_expression(&array_expr, &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Array);
}

#[test]
fn test_object_construction_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let mut fields = HashMap::new();
    fields.insert("name".to_string(), TemplateExpression::Variable("user".to_string()));
    fields.insert("count".to_string(), TemplateExpression::Variable("count".to_string()));
    let object_expr = TemplateExpression::Object(fields);
    
    let result = compiler.compile_template_expression(&object_expr, &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Object);
}

#[test]
fn test_conditional_expression_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let condition = TemplateExpression::Variable("active".to_string());
    let then_expr = TemplateExpression::Literal(CursedObject::String("Yes".to_string()));
    let else_expr = TemplateExpression::Literal(CursedObject::String("No".to_string()));
    
    let cond_expr = TemplateExpression::Conditional {
        condition: Box::new(condition),
        then_expr: Box::new(then_expr),
        else_expr: Box::new(else_expr),
    };
    
    let result = compiler.compile_template_expression(&cond_expr, &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::String);
}

#[test]
fn test_filter_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let mut context = create_test_context();
    
    // Register a filter
    context.register_filter("upper".to_string(), "cursed_filter_upper".to_string());
    
    let input_value = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%test_input".to_string(),
        is_constant: false,
    };
    
    let filter = FilterCall {
        name: "upper".to_string(),
        arguments: vec![],
    };
    
    let result = compiler.compile_template_filter(&filter, input_value, &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::String);
}

#[test]
fn test_template_conditional_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let condition = TemplateExpression::Variable("active".to_string());
    let then_branch = vec![TemplateNode::Text("Active!".to_string())];
    let else_branch = vec![TemplateNode::Text("Inactive".to_string())];
    
    let result = compiler.compile_template_conditional(
        &condition,
        &then_branch,
        Some(&else_branch),
        &context
    );
    assert!(result.is_ok());
}

#[test]
fn test_template_loop_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let iterator = TemplateExpression::Variable("users".to_string());
    let body = vec![
        TemplateNode::Text("User: ".to_string()),
        TemplateNode::Variable {
            expression: TemplateExpression::Variable("item".to_string()),
            filters: vec![],
            location: None,
        },
    ];
    
    let result = compiler.compile_template_loop("item", &iterator, &body, &context);
    assert!(result.is_ok());
}

#[test]
fn test_complete_template_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let ast = TemplateAst {
        nodes: vec![
            TemplateNode::Text("Hello ".to_string()),
            TemplateNode::Variable {
                expression: TemplateExpression::Variable("user".to_string()),
                filters: vec![],
                location: None,
            },
            TemplateNode::Text("!".to_string()),
        ],
    };
    
    let result = compiler.compile_template(&ast, &context);
    assert!(result.is_ok());
    
    let compiled = result.unwrap();
    assert_eq!(compiled.name, "test_template");
    assert!(compiled.metadata.optimization_level == TemplateOptimizationLevel::Basic);
    assert!(compiled.metadata.security_level == SecurityLevel::Strict);
}

#[test]
fn test_template_compilation_error_handling() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    // Test unknown variable
    let unknown_var = TemplateExpression::Variable("unknown_var".to_string());
    let result = compiler.compile_template_expression(&unknown_var, &context);
    assert!(result.is_err());
    
    if let Err(error) = result {
        match error {
            TemplateCompilationError::ExpressionError { .. } => {
                // Expected error type
            }
            _ => panic!("Unexpected error type"),
        }
    }
}

#[test]
fn test_template_runtime_functions_declaration() {
    common::init_tracing!();
    let module = cursed::codegen::llvm::DummyModule::new();
    
    let result = declare_template_runtime_functions(&module);
    assert!(result.is_ok());
    
    let functions = result.unwrap();
    assert!(functions.len() > 0);
    
    // Check that some essential functions are declared
    assert!(functions.contains_key("escape_html"));
    assert!(functions.contains_key("is_truthy"));
    assert!(functions.contains_key("concat_strings"));
    assert!(functions.contains_key("add_values"));
    assert!(functions.contains_key("values_equal"));
}

#[test]
fn test_template_optimization_levels() {
    common::init_tracing!();
    let generator = Arc::new(LlvmCodeGenerator::new());
    let mut compiler = LlvmTemplateCompiler::new(generator);
    
    let mut context = create_test_context();
    context.optimization_level = TemplateOptimizationLevel::Aggressive;
    
    let ast = TemplateAst {
        nodes: vec![TemplateNode::Text("Test".to_string())],
    };
    
    let result = compiler.compile_template(&ast, &context);
    assert!(result.is_ok());
    
    let compiled = result.unwrap();
    assert_eq!(compiled.metadata.optimization_level, TemplateOptimizationLevel::Aggressive);
}

#[test]
fn test_template_caching() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let ast = TemplateAst {
        nodes: vec![TemplateNode::Text("Cached template".to_string())],
    };
    
    // First compilation
    let result1 = compiler.compile_template(&ast, &context);
    assert!(result1.is_ok());
    assert_eq!(compiler.cache_size(), 1);
    
    // Second compilation should hit cache
    let result2 = compiler.compile_template(&ast, &context);
    assert!(result2.is_ok());
    
    let stats = compiler.get_stats();
    assert!(stats.cache_hits > 0);
}

#[test]
fn test_template_while_loop_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let condition = TemplateExpression::Variable("active".to_string());
    let body = vec![TemplateNode::Text("Loop iteration".to_string())];
    
    let result = compiler.compile_template_while(&condition, &body, &context);
    assert!(result.is_ok());
}

#[test]
fn test_template_range_for_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let start = TemplateExpression::Literal(CursedObject::Integer(0));
    let end = TemplateExpression::Literal(CursedObject::Integer(10));
    let step = Some(TemplateExpression::Literal(CursedObject::Integer(2)));
    let body = vec![TemplateNode::Text("Range iteration".to_string())];
    
    let result = compiler.compile_template_range_for(
        "i", &start, &end, step.as_ref(), &body, &context
    );
    assert!(result.is_ok());
}

#[test] 
fn test_template_match_compilation() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    let value = TemplateExpression::Variable("status".to_string());
    let cases = vec![
        MatchCase {
            pattern: TemplateExpression::Literal(CursedObject::String("active".to_string())),
            body: vec![TemplateNode::Text("Status is active".to_string())],
        },
        MatchCase {
            pattern: TemplateExpression::Literal(CursedObject::String("inactive".to_string())),
            body: vec![TemplateNode::Text("Status is inactive".to_string())],
        },
    ];
    let default_case = Some(vec![TemplateNode::Text("Unknown status".to_string())]);
    
    let result = compiler.compile_template_match(&value, &cases, default_case.as_deref(), &context);
    assert!(result.is_ok());
}

#[test]
fn test_comprehensive_template_features() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let mut context = create_test_context();
    
    // Register filters
    register_standard_filters(&mut context);
    
    // Create a complex template with multiple features
    let ast = TemplateAst {
        nodes: vec![
            TemplateNode::Text("<!DOCTYPE html><html><body>".to_string()),
            TemplateNode::LowkeyIf {
                condition: TemplateExpression::Variable("user".to_string()),
                then_branch: vec![
                    TemplateNode::Text("<h1>Welcome ".to_string()),
                    TemplateNode::Variable {
                        expression: TemplateExpression::PropertyAccess {
                            object: Box::new(TemplateExpression::Variable("user".to_string())),
                            property: "name".to_string(),
                        },
                        filters: vec![
                            FilterCall {
                                name: "upper".to_string(),
                                arguments: vec![],
                            }
                        ],
                        location: None,
                    },
                    TemplateNode::Text("!</h1>".to_string()),
                ],
                else_branch: Some(vec![
                    TemplateNode::Text("<h1>Welcome Guest!</h1>".to_string()),
                ]),
                location: None,
            },
            TemplateNode::StanLoop {
                variable: "item".to_string(),
                iterator: TemplateExpression::Variable("items".to_string()),
                body: vec![
                    TemplateNode::Text("<li>".to_string()),
                    TemplateNode::Variable {
                        expression: TemplateExpression::Variable("item".to_string()),
                        filters: vec![],
                        location: None,
                    },
                    TemplateNode::Text("</li>".to_string()),
                ],
                location: None,
            },
            TemplateNode::Text("</body></html>".to_string()),
        ],
    };
    
    let result = compiler.compile_template(&ast, &context);
    assert!(result.is_ok());
    
    let compiled = result.unwrap();
    assert_eq!(compiled.name, "test_template");
    assert!(compiled.metadata.used_filters.contains(&"upper".to_string()));
}

#[test]
fn test_error_recovery_and_reporting() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    // Test various error scenarios
    
    // Unregistered filter
    let filter = FilterCall {
        name: "nonexistent_filter".to_string(),
        arguments: vec![],
    };
    let input_value = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%test".to_string(),
        is_constant: false,
    };
    
    let result = compiler.compile_template_filter(&filter, input_value, &context);
    assert!(result.is_err());
    
    if let Err(TemplateCompilationError::FilterError { filter_name, .. }) = result {
        assert_eq!(filter_name, "nonexistent_filter");
    } else {
        panic!("Expected FilterError");
    }
}

#[test]
fn test_performance_and_statistics() {
    common::init_tracing!();
    let mut compiler = create_test_compiler();
    let context = create_test_context();
    
    // Compile multiple templates to test statistics
    for i in 0..5 {
        let ast = TemplateAst {
            nodes: vec![
                TemplateNode::Text(format!("Template {}", i)),
                TemplateNode::Variable {
                    expression: TemplateExpression::Variable("user".to_string()),
                    filters: vec![],
                    location: None,
                },
            ],
        };
        
        let mut template_context = context.clone();
        template_context.template_name = format!("template_{}", i);
        
        let result = compiler.compile_template(&ast, &template_context);
        assert!(result.is_ok());
    }
    
    let stats = compiler.get_stats();
    assert!(stats.templates_compiled >= 5);
    assert!(stats.total_compilation_time > Duration::from_nanos(0));
}
