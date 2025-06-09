/// Integration tests for control flow execution in CURSED
/// 
/// Tests actual execution of compiled control flow constructs to verify
/// semantic correctness beyond just LLVM IR generation.

use cursed::ast::statements::control_flow::*;
use cursed::ast::literals::{BooleanLiteral, IntegerLiteral};
use cursed::ast::block::BlockStatement;
use cursed::ast::traits::{Statement, Expression};
use cursed::codegen::llvm::{LlvmControlFlowCompiler, ControlFlowCompilation, ControlFlowContext};
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::OptimizationLevel;

#[path = "common/mod.rs"]
mod common;

type MainFunc = unsafe extern "C" fn() -> i32;

fn create_execution_engine<'ctx>(context: &'ctx Context) -> (inkwell::module::Module<'ctx>, ExecutionEngine<'ctx>) {
    let module = context.create_module("control_flow_test");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)
        .expect("Failed to create execution engine");
    (module, execution_engine)
}

#[test]
fn test_simple_if_execution() {
    common::tracing::setup();
    
    let context = Context::create();
    let (module, execution_engine) = create_execution_engine(&context);
    let builder = context.create_builder();
    
    // Create main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function("main", fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    builder.position_at_end(entry_block);
    
    let compiler = LlvmControlFlowCompiler::new();
    let mut flow_ctx = ControlFlowContext::new();
    flow_ctx.current_function = Some(main_function);
    
    // Create if statement that should execute
    let condition = BooleanLiteral {
        token: "true".to_string(),
        value: true,
    };
    
    let if_stmt = IfStatement {
        token: "lowkey".to_string(),
        condition: Box::new(condition),
        consequence: BlockStatement::empty(), // Empty consequence
        alternative: None,
    };
    
    // Compile if statement
    let result = compiler.compile_if_statement(&context, &module, &builder, &if_stmt, &mut flow_ctx);
    assert!(result.is_ok(), "Failed to compile if statement: {:?}", result.err());
    
    // Return success
    let return_val = i32_type.const_int(42, false);
    builder.build_return(Some(&return_val)).unwrap();
    
    // Verify and execute
    assert!(module.verify().is_ok(), "Generated invalid LLVM IR");
    
    unsafe {
        let main_fn: JitFunction<MainFunc> = execution_engine.get_function("main")
            .expect("Failed to get main function");
        let result = main_fn.call();
        assert_eq!(result, 42, "Function should return 42");
    }
}

#[test]
fn test_while_loop_with_counter() {
    common::tracing::setup();
    
    let context = Context::create();
    let (module, execution_engine) = create_execution_engine(&context);
    let builder = context.create_builder();
    
    // Create main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function("main", fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    builder.position_at_end(entry_block);
    
    let compiler = LlvmControlFlowCompiler::new();
    let mut flow_ctx = ControlFlowContext::new();
    flow_ctx.current_function = Some(main_function);
    
    // Create a while loop that terminates immediately (condition = false)
    let condition = BooleanLiteral {
        token: "false".to_string(),
        value: false,
    };
    
    let while_stmt = WhileStatement {
        token: "periodt".to_string(),
        condition: Box::new(condition),
        body: BlockStatement::empty(),
    };
    
    // Compile while statement
    let result = compiler.compile_while_statement(&context, &module, &builder, &while_stmt, &mut flow_ctx);
    assert!(result.is_ok(), "Failed to compile while statement: {:?}", result.err());
    
    // Return success
    let return_val = i32_type.const_int(100, false);
    builder.build_return(Some(&return_val)).unwrap();
    
    // Verify and execute
    assert!(module.verify().is_ok(), "Generated invalid LLVM IR");
    
    unsafe {
        let main_fn: JitFunction<MainFunc> = execution_engine.get_function("main")
            .expect("Failed to get main function");
        let result = main_fn.call();
        assert_eq!(result, 100, "Function should return 100 (loop didn't execute)");
    }
}

#[test]
fn test_for_loop_execution() {
    common::tracing::setup();
    
    let context = Context::create();
    let (module, execution_engine) = create_execution_engine(&context);
    let builder = context.create_builder();
    
    // Create main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function("main", fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    builder.position_at_end(entry_block);
    
    let compiler = LlvmControlFlowCompiler::new();
    let mut flow_ctx = ControlFlowContext::new();
    flow_ctx.current_function = Some(main_function);
    
    // Create for loop with false condition (no execution)
    let condition = BooleanLiteral {
        token: "false".to_string(),
        value: false,
    };
    
    let for_stmt = ForStatement {
        token: "bestie".to_string(),
        init: None,
        condition: Some(Box::new(condition)),
        post: None,
        body: BlockStatement::empty(),
    };
    
    // Compile for statement
    let result = compiler.compile_for_statement(&context, &module, &builder, &for_stmt, &mut flow_ctx);
    assert!(result.is_ok(), "Failed to compile for statement: {:?}", result.err());
    
    // Return success
    let return_val = i32_type.const_int(200, false);
    builder.build_return(Some(&return_val)).unwrap();
    
    // Verify and execute
    assert!(module.verify().is_ok(), "Generated invalid LLVM IR");
    
    unsafe {
        let main_fn: JitFunction<MainFunc> = execution_engine.get_function("main")
            .expect("Failed to get main function");
        let result = main_fn.call();
        assert_eq!(result, 200, "Function should return 200");
    }
}

#[test]
fn test_switch_statement_execution() {
    common::tracing::setup();
    
    let context = Context::create();
    let (module, execution_engine) = create_execution_engine(&context);
    let builder = context.create_builder();
    
    // Create main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function("main", fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    builder.position_at_end(entry_block);
    
    let compiler = LlvmControlFlowCompiler::new();
    let mut flow_ctx = ControlFlowContext::new();
    flow_ctx.current_function = Some(main_function);
    
    // Create switch with value 1 and matching case
    let switch_value = IntegerLiteral {
        token: "1".to_string(),
        value: 1,
    };
    
    let case_value = IntegerLiteral {
        token: "1".to_string(),
        value: 1,
    };
    
    let case = SwitchCase {
        values: vec![Box::new(case_value)],
        statements: vec![], // Empty case body
    };
    
    let switch_stmt = SwitchStatement {
        token: "vibe_check".to_string(),
        value: Box::new(switch_value),
        cases: vec![case],
        default: None,
    };
    
    // Compile switch statement
    let result = compiler.compile_switch_statement(&context, &module, &builder, &switch_stmt, &mut flow_ctx);
    assert!(result.is_ok(), "Failed to compile switch statement: {:?}", result.err());
    
    // Return success
    let return_val = i32_type.const_int(300, false);
    builder.build_return(Some(&return_val)).unwrap();
    
    // Verify and execute
    assert!(module.verify().is_ok(), "Generated invalid LLVM IR");
    
    unsafe {
        let main_fn: JitFunction<MainFunc> = execution_engine.get_function("main")
            .expect("Failed to get main function");
        let result = main_fn.call();
        assert_eq!(result, 300, "Function should return 300");
    }
}

#[test]
fn test_range_for_execution() {
    common::tracing::setup();
    
    let context = Context::create();
    let (module, execution_engine) = create_execution_engine(&context);
    let builder = context.create_builder();
    
    // Create main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function("main", fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    builder.position_at_end(entry_block);
    
    let compiler = LlvmControlFlowCompiler::new();
    let mut flow_ctx = ControlFlowContext::new();
    flow_ctx.current_function = Some(main_function);
    
    // Create range for with small iteration count
    let iterable = IntegerLiteral {
        token: "3".to_string(),
        value: 3,
    };
    
    let range_for = RangeForStatement {
        token: "bestie".to_string(),
        key_var: Some("i".to_string()),
        value_var: Some("v".to_string()),
        iterable: Box::new(iterable),
        body: BlockStatement::empty(),
    };
    
    // Compile range for statement
    let result = compiler.compile_range_for_statement(&context, &module, &builder, &range_for, &mut flow_ctx);
    assert!(result.is_ok(), "Failed to compile range for statement: {:?}", result.err());
    
    // Return success
    let return_val = i32_type.const_int(400, false);
    builder.build_return(Some(&return_val)).unwrap();
    
    // Verify and execute
    assert!(module.verify().is_ok(), "Generated invalid LLVM IR");
    
    unsafe {
        let main_fn: JitFunction<MainFunc> = execution_engine.get_function("main")
            .expect("Failed to get main function");
        let result = main_fn.call();
        assert_eq!(result, 400, "Function should return 400");
    }
}

/// Complex integration test with nested control flow
#[test]
fn test_complex_nested_control_flow() {
    common::tracing::setup();
    
    let context = Context::create();
    let (module, execution_engine) = create_execution_engine(&context);
    let builder = context.create_builder();
    
    // Create main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function("main", fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    builder.position_at_end(entry_block);
    
    let compiler = LlvmControlFlowCompiler::new();
    let mut flow_ctx = ControlFlowContext::new();
    flow_ctx.current_function = Some(main_function);
    
    // Create if with while inside
    let while_condition = BooleanLiteral {
        token: "false".to_string(),
        value: false,
    };
    
    let while_stmt = WhileStatement {
        token: "periodt".to_string(),
        condition: Box::new(while_condition),
        body: BlockStatement::empty(),
    };
    
    let if_condition = BooleanLiteral {
        token: "true".to_string(),
        value: true,
    };
    
    let if_stmt = IfStatement {
        token: "lowkey".to_string(),
        condition: Box::new(if_condition),
        consequence: BlockStatement::with_statements(vec![Box::new(while_stmt)]),
        alternative: None,
    };
    
    // Compile nested structure
    let result = compiler.compile_if_statement(&context, &module, &builder, &if_stmt, &mut flow_ctx);
    assert!(result.is_ok(), "Failed to compile nested control flow: {:?}", result.err());
    
    // Return success
    let return_val = i32_type.const_int(500, false);
    builder.build_return(Some(&return_val)).unwrap();
    
    // Verify and execute
    assert!(module.verify().is_ok(), "Generated invalid LLVM IR");
    
    unsafe {
        let main_fn: JitFunction<MainFunc> = execution_engine.get_function("main")
            .expect("Failed to get main function");
        let result = main_fn.call();
        assert_eq!(result, 500, "Function should return 500");
    }
}

#[test]
fn test_ir_output_validation() {
    common::tracing::setup();
    
    let context = Context::create();
    let module = context.create_module("ir_validation_test");
    let builder = context.create_builder();
    
    // Create main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function("main", fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    builder.position_at_end(entry_block);
    
    let compiler = LlvmControlFlowCompiler::new();
    let mut flow_ctx = ControlFlowContext::new();
    flow_ctx.current_function = Some(main_function);
    
    // Create a simple while loop
    let condition = BooleanLiteral {
        token: "true".to_string(),
        value: true,
    };
    
    let break_stmt = BreakStatement {
        token: "ghosted".to_string(),
    };
    
    let while_stmt = WhileStatement {
        token: "periodt".to_string(),
        condition: Box::new(condition),
        body: BlockStatement::with_statements(vec![Box::new(break_stmt)]),
    };
    
    // Compile and get IR
    let result = compiler.compile_while_statement(&context, &module, &builder, &while_stmt, &mut flow_ctx);
    assert!(result.is_ok());
    
    // Add return
    let return_val = i32_type.const_int(0, false);
    builder.build_return(Some(&return_val)).unwrap();
    
    // Validate IR structure
    let ir_string = module.print_to_string().to_string();
    
    // Check for expected LLVM IR patterns
    assert!(ir_string.contains("periodt_condition"), "Should contain condition block");
    assert!(ir_string.contains("periodt_body"), "Should contain body block");
    assert!(ir_string.contains("periodt_exit"), "Should contain exit block");
    assert!(ir_string.contains("br i1"), "Should contain conditional branch");
    assert!(ir_string.contains("br label"), "Should contain unconditional branches");
    
    // Verify module
    assert!(module.verify().is_ok(), "Generated IR should be valid");
}
