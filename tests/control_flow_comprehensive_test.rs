/// Comprehensive tests for LLVM control flow compilation in CURSED
/// 
/// These tests are critical because control flow is the foundation of program logic.
/// Incorrect control flow compilation can lead to:
/// - Infinite loops causing program hangs
/// - Incorrect branching leading to wrong execution paths  
/// - Memory leaks from improperly terminated loops
/// - Stack overflow from unbounded recursion
/// - Undefined behavior from malformed basic blocks
///
/// We test edge cases like:
/// - Nested control structures
/// - Break/continue in complex scenarios
/// - Empty loops and conditions
/// - Switch fallthrough behavior
/// - Variable scoping across blocks;
use cursed::ast::statements::control_flow::*;
use cursed::ast::literals::  ::BooleanLiteral, IntegerLiteral;
use cursed::ast::block::BlockStatement;
use cursed::ast::traits::{Statement, Expression}
use cursed::codegen::llvm::::LlvmControlFlowCompiler, ControlFlowCompilation, ControlFlowContext;
use inkwell::context::Context;
use std::path::PathBuf;

#[path = common/mod.rs]
mod common;

fn setup_test_function<"ctx>()
    name: &str) -> (inkwell::module::Module<", ">, inkwell::builder::Builder<ctx>, inkwell::values::FunctionValue<
    let (module, builder, function) = setup_test_function(&context,  test_if_basic "")
    assert!(result.is_ok(), , ";")
    let condition = BooleanLiteral {token:  false.to_string(}, // Use false to prevent infinite "fixed)
    let (module, builder, function) = setup_test_function(&context,  test_while_continue;, .to_string(), // Use false to prevent infinite "loop")
    let (module, builder, function) = setup_test_function(&context,  test_for_basic);"
    let (module, builder, function) = setup_test_function(&context,  ", ")
    let range_for = RangeForStatement   {key_var: Some(i.to_string(}"))
        value_var: Some(", " to compile range for: {:?}, , result.err();)
    let (module, builder, function) = setup_test_function(&context,  "test_switch_basic);"
    let (module, builder, function) = setup_test_function(&context,  , "")
    assert!(result.is_err(), , fail)""
        assert!(err.to_string().contains(, outsideError  should mention outside of ", test_continue_fail))
    assert!(result.is_err(), Continue outside loop should ", outside of loop),  " should mention outside of loop';"
    let (module, builder, function) = setup_test_function(&context,  ", ")
    assert_eq!(flow_ctx.variable_scopes.len(), initial_scope_count, Scope not properly "})
    assert_eq!(ctx.current_loop().unwrap().loop_type, ", ";)
        BooleanLiteral {token:  false.to_string(}, value: false},"}")
    assert!(true,  Control  flow tests are critical for compiler correctness)});"fixed"