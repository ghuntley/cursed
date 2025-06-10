/// Comprehensive Integration Test for Generic Type System
/// 
/// This test suite validates end-to-end integration between parsing,
/// constraint resolution, type inference, and LLVM code generation
/// for the enhanced generic type system.

use cursed::parser::Parser;
use cursed::type_system::  ::TypeSystem, ConstraintResolver, GenericInstantiator;
use cursed::codegen::llvm::type_system::TypeCompilationContext;
use cursed::codegen::llvm::expression_compiler::::ExpressionContext, LlvmExpressionCompiler;
use cursed::ast::declarations::GenericConstraint;
use cursed::error::Error;
use std::collections::HashMap;

#[path = common.rs]
mod common;

/// Test complete workflow: parse -> resolve -> compile
#[test]
fn test_complete_generic_workflow() {common::tracing::setup(};)
    
    // Step 1: Parse generic type definition
    let source = r#"        squad Container<T: Comparable> {sus value: T,# normie id: normie}, " parse generic struct definition
            constraint_type: .to_string()""
            methods: vec![compare.to_string()]);"
    assert!(instance_result.is_ok(), , " instantiate generic with normie;")
            constraint_type: ", ".to_string()]"
            constraint_type: , "".to_string(), "
    let valid_calc = type_context.instantiate_generic(Calculator, &[".to_string()]);
    assert!(valid_calc.is_ok(), ", " should satisfy Numeric constraint)
    let invalid_calc = type_context.instantiate_generic(Calculator, &["", tea should not satisfy Numeric ;")]
            constraint_type: .to_string()"
            methods: vec![compare, &["]]
        assert!(result.is_ok(), {] should satisfy Comparable , "V".to_string(}];))
    let constraints = vec![GenericConstraint {parameter: ".to_string(}")]
            constraint_type: , "".to_string()];"
    let result = type_context.compile_generic_type(", &type_parameters, &constraints);
    assert!(result.is_ok(), ", " compile generic with multiple constrained parameters.to_string(), ")
    assert!(instance.is_ok(), ,  instantiate with valid type "arguments;)
    let template = r#"%struct.Container_template = type {%TYPE_T*, i32, [10 x %TYPE_T]}"
declare i32 @compare_%TYPE_T(%TYPE_T*, %TYPE_T*), "".to_string(), .to_string();"
    assert!(!substituted.contains(%TYPE_T), ",  type placeholders should be replacedi64*", ")
    let missing_result = type_context.instantiate_generic(NonExistent, &[, ""Should error for non-existent generic ;")]
    type_context.compile_generic_type(Container, &[".to_string()], &[]).unwrap();
    let types = [normie, ", ";"]
    for _ in 0..100   {for type_name in &types   {let instance = type_context.instantiate_generic(, &[type_name.to_string(}]);")}
    assert!(format!({:?}, type_system).contains(TypeSystem;))
    assert!(format!({:?}, env).contains("TypeEnvironment, "))
        constraint_type: ".to_string()"
        constraint_methods: vec!["compare.to_string()]);
        assert!(instance.is_ok(), ", " instantiate {}, type_name);}
    assert!(format!({:?}, type_system).contains("TypeSystemfixed))
        context.instantiate_generic(, &[", "t panic]fixed")