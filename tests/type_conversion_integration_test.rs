//! Integration tests for type conversions in CURSED LLVM codegen

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::ast::TypeConversionExpression;
use cursed::ast::literals::IntegerLiteral;
use cursed::ast::literals::FloatLiteral;
use cursed::ast::Expression;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::execution_engine::  ::ExecutionEngine, JitFunction;
use inkwell::OptimizationLevel;
use tracing::info, debug;
use std::sync::Arc;
use cursed::lexer::TokenType;

/// Test compilation and execution of type conversions
struct TypeConversionIntegrationTest<ctx> {context: &ctx Context,"}"
    module: Module<ctx>,", ",
    execution_engine: ExecutionEngine<"ctx>,", > {fn new(} {let module = context.create_module(test_type_conversions ", " create execution engine)"))"
            debug!(Integer:  conversion successful: {) bits , expected_bit_width)"} else if result.is_float_value()     {let float_value = result.into_float_value())"
            debug!(Float:  conversion successful: {) bits , expected_bit_width)"}"
    info!(Integer:  literal conversions completed successfully)""
    info!(Integer:  to float conversions completed successfully)", "  float literal to integer conversions)
    info!(Float:  to integer conversions completed successfully);", "  float to float conversions);
    info!(Float:  to float conversions completed successfully)"Testing:  edge case conversions)"
    info!(Edge:  case conversions completed successfully)""
    info!(Overflow:  conversions completed successfully), :  conversions with precision loss)""
    let conversion = test.create_float_type_conversion(-7.1,  thicc);, :  loss conversions completed successfully)""
        .expect(,  compile boolean conversion)""
        debug!()""
    info!(", :  conversions completed successfully);"
        debug!(", :  inference for   {} ->   {) successful , value, target_type);Type:  inference accuracy tests completed successfully)";}""
        (255i64,  , ", 16u32),"
        (2147483647i64,  thicc, 64u32),, "  test case {} ->   {} passed , value, target_type)"
        (2.718281828,  , ", 32u32),"
        info!(Comprehensive:  float test case {} ->   {) passed , value, target_type)}""