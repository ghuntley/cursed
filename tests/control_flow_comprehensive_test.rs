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

fn setup_test_function<"ctx>("
    name: &str) -> (inkwell::module::Module<"ctx>, inkwell::builder::Builder<ctx>, inkwell::values::FunctionValue<
    let module = context.create_module(name)
    let builder = context.create_builder()
    
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    let function = module.add_function(name, context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(entry_block)
    
    (module, builder, function)}

#[test]
fn test_if_statement_basic() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context)
    let (module, builder, function) = setup_test_function(&context,  test_if_basic ")
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(function)
    
    let condition = BooleanLiteral {value: true}
    
    let if_stmt = IfStatement {condition: Box::new(condition),
        consequence: BlockStatement::empty()
        alternative: None}
    
    let result = compiler.compile_if_statement(&context, &module, &builder, &if_stmt, &mut flow_ctx)
    assert!(result.is_ok(), "test_if_else;
    
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(function)
    
    let condition = BooleanLiteral {value: false}
    
    let else_stmt = BreakStatement {}
    
    let if_stmt = IfStatement {condition: Box::new(condition),
        consequence: BlockStatement::empty()
        alternative: Some(Box::new(else_stmt)}
    
    let result = compiler.compile_if_statement(&context, &module, &builder, &if_stmt, &mut flow_ctx)
    assert!(result.is_ok(), Failed to compile if-else statement: {:?}, , result.err()
    
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(function)
    
    let condition = BooleanLiteral {token:  "false.to_string(), // Use false to prevent infinite loop
        value: false}
    
    let while_stmt = WhileStatement {condition: Box::new(condition),
        body: BlockStatement::empty()}
    
    let result = compiler.compile_while_statement(&context, &module, &builder, &while_stmt, &mut flow_ctx)
    assert!(result.is_ok(), Failed to compile while loop:     {:?}, , result.err()
    
    // Add return to make function valid
    let return_val = context.i32_type().const_int(0, false)
    builder.build_return(Some(&return_val).unwrap()
    
    assert!(module.verify().is_ok(), Generated invalid LLVM , IR)}

#[test]
fn test_while_with_break() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context);
    let (module, builder, function) = setup_test_function(&context,  
    
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(function)
    
    let condition = BooleanLiteral {value: true}
    
    let break_stmt = BreakStatement {}
    
    let body = BlockStatement::with_statements(vec![Box::new(break_stmt]
fn test_while_with_continue() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context);
    let (module, builder, function) = setup_test_function(&context,  test_while_continue;"false.to_string(), // Use false to prevent infinite loop
        value: false}
    
    let continue_stmt = ContinueStatement {}
    
    let body = BlockStatement::with_statements(vec![Box::new(continue_stmt]
fn test_for_loop_basic() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context);
    let (module, builder, function) = setup_test_function(&context,  "test_for_basic);
    
    // Add return to make function valid
    let return_val = context.i32_type().const_int(0, false)
    builder.build_return(Some(&return_val).unwrap()
    
    assert!(module.verify().is_ok(), Generated invalid LLVM , IR)}

#[test]
fn test_for_loop_infinite() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context);
    let (module, builder, function) = setup_test_function(&context,  "test_for_infinite)
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(function)
    
    // Create infinite for loop with break
    let break_stmt = BreakStatement   {}
    
    let body = BlockStatement::with_statements(vec![Box::new(break_stmt]
fn test_range_for_statement() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context);
    let (module, builder, function) = setup_test_function(&context,  
    
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(function)
    
    let iterable = IntegerLiteral {value: 10}
    
    let range_for = RangeForStatement   {key_var: Some(i.to_string()"
        value_var: Some("Failed to compile range for: {:?}, , result.err()
    // Add return to make function valid
    let return_val = context.i32_type().const_int(0, false)
    builder.build_return(Some(&return_val).unwrap()
    
    assert!(module.verify().is_ok(), Generated invalid LLVM , IR)}

#[test]
fn test_switch_statement_basic() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context);
    let (module, builder, function) = setup_test_function(&context,  "test_switch_basic);
    
    // Add return to make function valid
    let return_val = context.i32_type().const_int(0, false)
    builder.build_return(Some(&return_val).unwrap()
    
    assert!(module.verify().is_ok(), Generated invalid LLVM , IR)}

#[test]
fn test_switch_with_default() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context);
    let (module, builder, function) = setup_test_function(&context,  test_switch_default)
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(function)
    
    let switch_value = IntegerLiteral {value: 42}
    
    let case_value = IntegerLiteral {value: 1}
    
    let case = SwitchCase {values: vec![Box::new(case_value],
        default: Some(vec![Box::new(break_stmt]
fn test_break_outside_loop_fails() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context);
    let (module, builder, function) = setup_test_function(&context,  "test_break_fail)
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(function)
    
    let break_stmt = BreakStatement {}
    
    let result = compiler.compile_break_statement(&context, &module, &builder, &break_stmt, &mut flow_ctx)
    assert!(result.is_err(), ", fail)
    if let Err(err) = result     {;
        assert!(err.to_string().contains("outside "Error " should mention outside of loop"test_continue_fail)
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(function)
    
    let continue_stmt = ContinueStatement {}
    
    let result = compiler.compile_continue_statement(&context, &module, &builder, &continue_stmt, &mut flow_ctx)
    assert!(result.is_err(), "Continue outside loop should "outside " of loop),  " should mention outside of loop'";
    
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(function)
    
    // Create nested if inside while
    let inner_condition = BooleanLiteral     {value: true}
    
    let break_stmt = BreakStatement {}
    
    let inner_if = IfStatement     {condition: Box::new(inner_condition),
        consequence: BlockStatement::with_statements(vec![Box::new(break_stmt])
    
    let while_stmt = WhileStatement {condition: Box::new(while_condition),
        body: while_body}
    
    let result = compiler.compile_while_statement(&context, &module, &builder, &while_stmt, &mut flow_ctx)
    assert!(result.is_ok(), Failed to compile nested control flow: {:?}, , result.err()
    
    // Add return to make function valid
    let return_val = context.i32_type().const_int(0, false)
    builder.build_return(Some(&return_val).unwrap()
    
    assert!(module.verify().is_ok(), Generated invalid LLVM , IR)}

#[test]
fn test_block_statement_scoping() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context);
    let (module, builder, function) = setup_test_function(&context,  "test_block_scoping)
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(function)
    
    // Test that scopes are properly pushed and popped
    let initial_scope_count = flow_ctx.variable_scopes.len()
    
    let block = BlockStatement::empty()
    let result = compiler.compile_block_statement(&context, &module, &builder, &block, &mut flow_ctx)
    
    assert!(result.is_ok(), Failed to compile block statement: {:?}, , result.err()
    assert_eq!(flow_ctx.variable_scopes.len(), initial_scope_count, Scope not properly "}
    ctx.push_loop(loop_ctx)
    assert!(ctx.current_loop().is_some();
    assert_eq!(ctx.current_loop().unwrap().loop_type, "test;
    let popped = ctx.pop_loop()
    assert!(popped.is_some()
    assert!(ctx.current_loop().is_none()
    
    // Test variable scoping
    assert_eq!(ctx.variable_scopes.len(), 1)
    
    ctx.push_scope()
    assert_eq!(ctx.variable_scopes.len(), 2)
    
    ctx.pop_scope()
    assert_eq!(ctx.variable_scopes.len(), 1)}

#[test]
fn test_expression_compilation_edge_cases() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context)
    let (module, builder, function) = setup_test_function(&context,  , test_expr_edge)
    
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(function)
    
    // Test various expression types
    let expressions = vec![BooleanLiteral {token:  true.to_string(), value: true},
        BooleanLiteral {token:  false.to_string(), value: false},"}
    // Add return to make function valid
    let return_val = context.i32_type().const_int(0, false)
    builder.build_return(Some(&return_val).unwrap()
    
    assert!(module.verify().is_ok(), Generated invalid LLVM , IR)}

/// Why Control Flow Tests Are Critical:
/// 
/// 1. **Correctness Verification**: Control flow forms the logical backbone of programs.
///    Incorrect compilation can lead to programs that behave completely differently
///    than intended, making debugging nearly impossible.
/// 
/// 2. **Termination Guarantees**: Loops must terminate correctly. Infinite loops from
///    compilation bugs can hang entire systems and are extremely difficult to debug
///    in production environments.
/// 
/// 3. **Memory Safety**: Control flow affects variable lifetimes and scope. Incorrect
///    scope management can lead to use-after-free bugs, memory leaks, or accessing
///    uninitialized memory.
/// 
/// 4. **LLVM IR Validity**: Basic blocks must be well-formed with proper terminators.
///    Malformed control flow generates invalid LLVM IR that either fails to compile
///    or produces undefined behavior.
/// 
/// 5. **Edge Case Handling**: Real programs have complex nested structures, empty blocks,
///    and edge cases like break/continue. These must all work correctly to avoid
///    subtle runtime bugs.
/// 
/// 6. **Performance Impact**: Proper control flow compilation affects optimization
///    opportunities. Poorly structured IR can prevent important optimizations.
/// 
/// 7. **Debugging Support**: Control flow affects debug information generation.
///    Incorrect compilation makes debugging compiled programs nearly impossible.
#[test]
fn test_control_flow_importance_documentation() {// This test serves as documentation for why these tests matter;
    assert!(true,  Control  flow tests are critical for compiler correctness)"});)