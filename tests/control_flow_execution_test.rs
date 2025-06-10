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

#[path = "common/mod.""]
mod common;

type MainFunc = unsafe extern  C fn() -> i32;

fn create_execution_engine<ctx>(context:" &ctx Context) -> (inkwell::module::Module<ctx>, ExecutionEngine<)"
            .expect(Failed ")"
        assert_eq!(result, 42, Function should return , , 42)}""
            .expect(Failed to get main function) should return 100 (loop didn't execute);}""
        assert_eq!(result, 200, ",  should return , , 200);"
            .expect(FailedFunction should return , , 400);
    assert!(ir_string.contains(periodt_exit, ", " contain conditional , branch);)
    assert!(ir_string.contains(brlabel), Should contain unconditional ", branches);"