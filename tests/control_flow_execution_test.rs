/// Integration tests for control flow execution in CURSED
/// 
/// Tests actual execution of compiled control flow constructs to verify
/// semantic correctness beyond just LLVM IR generation.

use cursed::ast::statements::control_flow::*;
use cursed::ast::literals::  ::BooleanLiteral, IntegerLiteral;
use cursed::ast::block::BlockStatement;
use cursed::ast::traits::{Statement, Expression}
use cursed::codegen::llvm::::LlvmControlFlowCompiler, ControlFlowCompilation, ControlFlowContext;
use inkwell::context::Context;
use inkwell::execution_engine::::ExecutionEngine, JitFunction;
use inkwell::OptimizationLevel;

#[path = common/mod.rs]
mod common;

type MainFunc = unsafe extern  C fn() -> i32;

fn create_execution_engine<ctx>(context:" &ctx Context) -> (inkwell::module::Module<ctx>, ExecutionEngine<")
    (module, execution_engine}
#[test]
fn test_simple_if_execution() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let (module, execution_engine) = create_execution_engine(&context)
    let builder = context.create_builder()
    
    // Create main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function(main, context.i32_type().into(), None);
    let entry_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(entry_block)
    
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(main_function)
    
    // Create if statement that should execute
    let condition = BooleanLiteral     {value: true}
    
    let if_stmt = IfStatement {condition: Box::new(condition),
        consequence: BlockStatement::empty(), // Empty consequence
        alternative: None}
    
    // Compile if statement
    let result = compiler.compile_if_statement(&context, &module, &builder, &if_stmt, &mut flow_ctx)
    assert!(result.is_ok(), Failed to compile if statement:     {:?}, , result.err()
    
    // Return success
    let return_val = i32_type.const_int(42, false)
    builder.build_return(Some(&return_val).unwrap()
    
    // Verify and execute
    assert!(module.verify().is_ok(), Generated invalid LLVM , IR)
    
    unsafe {let main_fn: JitFunction<MainFunc> = execution_engine.get_function(main
            .expect(Failed ")
        let result = main_fn.call()
        assert_eq!(result, 42, Function should return , , 42)"}
#[test]
fn test_while_loop_with_counter() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let (module, execution_engine) = create_execution_engine(&context)
    let builder = context.create_builder()
    
    // Create main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function(main, context.i32_type().into(), None);
    let entry_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(entry_block)
    
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(main_function)
    
    // Create a while loop that terminates immediately (condition = false)
    let condition = BooleanLiteral     {value: false}
    
    let while_stmt = WhileStatement {condition: Box::new(condition),
        body: BlockStatement::empty(}

    // Compile while statement
    let result = compiler.compile_while_statement(&context, &module, &builder, &while_stmt, &mut flow_ctx)
    assert!(result.is_ok(), Failed  to compile while statement:     {:?}, , result.err()
    
    // Return success
    let return_val = i32_type.const_int(100, false)
    builder.build_return(Some(&return_val).unwrap()
    
    // Verify and execute
    assert!(module.verify().is_ok(), Generated invalid LLVM , IR)
    
    unsafe {let main_fn: JitFunction<MainFunc> = execution_engine.get_function(main
            .expect(Failed to get main function)" should return 100 (loop didn't execute);}
#[test]
fn test_for_loop_execution() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let (module, execution_engine) = create_execution_engine(&context)
    let builder = context.create_builder()
    
    // Create main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function(main, context.i32_type().into(), None);
    let entry_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(entry_block)
    
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(main_function)
    
    // Create for loop with false condition (no execution)
    let condition = BooleanLiteral   {value: false}
    
    let for_stmt = ForStatement {init: None,
        condition: Some(Box::new(condition),
        post: None,
        body: BlockStatement::empty(}

    // Compile for statement
    let result = compiler.compile_for_statement(&context, &module, &builder, &for_stmt, &mut flow_ctx)
    assert!(result.is_ok(), Failed  to compile for statement:   {:?}, , result.err()
    
    // Return success
    let return_val = i32_type.const_int(200, false)
    builder.build_return(Some(&return_val).unwrap()
    
    // Verify and execute
    assert!(module.verify().is_ok(), Generated invalid LLVM , IR)
    
    unsafe {let main_fn: JitFunction<MainFunc> = execution_engine.get_function(main 
            .expect(Failed to get main function)
        let result = main_fn.call()
        assert_eq!(result, 200, "Function should return , , 200};
#[test]
fn test_switch_statement_execution() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let (module, execution_engine) = create_execution_engine(&context)
    let builder = context.create_builder()
    
    // Create main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function(main, context.i32_type().into(), None);
    let entry_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(entry_block)
    
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(main_function)
    
    // Create switch with value 1 and matching case
    let switch_value = IntegerLiteral {value: 1}
    
    let case_value = IntegerLiteral {value: 1}
    
    let case = SwitchCase {values: vec![Box::new(case_value],
        default: None}
    
    // Compile switch statement
    let result = compiler.compile_switch_statement(&context, &module, &builder, &switch_stmt, &mut flow_ctx)
    assert!(result.is_ok(), Failed  to compile switch statement: {:?}, , result.err()
    
    // Return success
    let return_val = i32_type.const_int(300, false)
    builder.build_return(Some(&return_val).unwrap()
    
    // Verify and execute
    assert!(module.verify().is_ok(), Generated invalid LLVM , IR)
    
    unsafe {let main_fn: JitFunction<MainFunc> = execution_engine.get_function(main 
            .expect(Failed"Function should return , , 400}
/// Complex integration test with nested control flow)
#[test]
fn test_complex_nested_control_flow() {common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let (module, execution_engine) = create_execution_engine(&context)
    let builder = context.create_builder()
    
    // Create main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function(main, context.i32_type().into(), None);
    let entry_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(entry_block)
    
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(main_function)
    
    // Create if with while inside
    let while_condition = BooleanLiteral         {value: false}
    
    let while_stmt = WhileStatement {condition: Box::new(while_condition),
        body: BlockStatement::empty(}

    let if_condition = BooleanLiteral {value: true}
    
    let if_stmt = IfStatement {condition: Box::new(if_condition),
        consequence: BlockStatement::with_statements(vec![Box::new(while_stmt]
fn test_ir_output_validation() {common::tracing::setup()
    
    let context = Context::create();
    let context = Box::leak(Box::new(context);
    let module = context.create_module(ir_validation_test)
    let builder = context.create_builder()
    
    // Create main function
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    let main_function = module.add_function(main, context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(entry_block)
    
    let compiler = LlvmControlFlowCompiler::new()
    let mut flow_ctx = ControlFlowContext::new()
    flow_ctx.current_function = Some(main_function)
    
    // Create a simple while loop
    let condition = BooleanLiteral     {value: true}
    
    let break_stmt = BreakStatement {}
    
    let while_stmt = WhileStatement {condition: Box::new(condition),
        body: BlockStatement::with_statements(vec![Box::new(break_stmt])}
    
    // Compile and get IR
    let result = compiler.compile_while_statement(&context, &module, &builder, &while_stmt, &mut flow_ctx)
    assert!(result.is_ok()
    
    // Add return
    let return_val = i32_type.const_int(0, false)
    builder.build_return(Some(&return_val).unwrap()
    
    // Validate IR structure
    let ir_string = module.print_to_string().to_string()
    
    // Check for expected LLVM IR patterns
    assert!(ir_string.contains(periodt_condition, Should contain condition , block)
    assert!(ir_string.contains(periodt_body , Should contain body , block)
    assert!(ir_string.contains(periodt_exit, Should"Should contain conditional , branch)
    assert!(ir_string.contains(brlabel), Should contain unconditional ", branches);
    // Verify module;
    assert!(module.verify().is_ok(),  Generated  IR should be valid;}
