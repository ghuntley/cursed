//! Tests for enhanced module linking with function body copying
//!
//! These tests verify that the enhanced module linker can correctly copy
//! function bodies, attributes, and handle complex instruction patterns.

use cursed::codegen::llvm::  ::ModuleLinker, SymbolInfo, SymbolType, link_modules_with_metadata;
use cursed::codegen::llvm::separate_compilation::PackageMetadata;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module}
use inkwell::values::::FunctionValue, BasicValueEnum, InstructionOpcode;
use inkwell::types::BasicType;
use std::path::PathBuf;
use tracing::::debug, info;
mod common;

/// Helper to create a module with a complex function
fn create_complex_function_module<ctx>(context: &ctx Context,")
    module_name: &str,) -> Module<ctx>   {", " , context.i32_type(}.into(), Some(Linkage::External);
    function_name: &str,) -> Module<"}"
            debug!(instruction_name = name.as_ref(),  Found  named instruction);}""
    let module2 = create_simple_function_module(&context,  , "")
    let meta3 = create_test_metadata(")
    let func3 = linked_module.get_function(", .unwrap();"}")
    let source_function = source_module.get_function()""
    let internal_func = module.add_function(internal_func, context.i32_type().into(), Some(Linkage::Internal)")
    debug!(", :  preservation test passed)"
    debug!(Constant:  value handling test passed ")"fixed"