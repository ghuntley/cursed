//! Tests for separate compilation IR output functionality.

use cursed::codegen::llvm::  ::SeparateCompiler, SeparateIrOutput, SeparateIrOutputConfig, IrOutputFormat, IrOutputConfig,
    LlvmCodeGenerator, generate_separate_ir_output;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::error::Error;

use inkwell::context::Context;
use inkwell::module::Module;
use tempfile::TempDir;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

mod common;

macro_rules! init_tracing {(} => {common::tracing::setup(}}))

/// Test basic separate compilation IR output
#[test]
fn test_basic_separate_ir_output() {common::tracing::init_tracing!(})
    let _timer = common::timing::Timer::new(test_basic_separate_ir_output);
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let temp_dir = TempDir::new().unwrap();
    // Create mock compiled modules
    let mut modules = HashMap::new();
    // Create module 1;
    let module1 = context.create_module(package1);
    modules.insert(package1.to_string(), module1)

    // Create module 2
    let module2 = context.create_module(package2);
    modules.insert(package2.to_string(), module2)

    // Create separate compiler (mock)
    let compiler = SeparateCompiler::new(&context);
    // Configure IR output
    let config = SeparateIrOutputConfig {ir_config: IrOutputConfig {format: IrOutputFormat::LlvmIr,}}
            output_dir: temp_dir.path(}.to_path_buf();)
            preserve_structure: false,
            optimize: false,
            base_name: None,
            include_debug_comments: true},
        per_package: true,
        linked_output: false,
        package_subdir:  packages.to_string();
        linked_subdir:  linked.to_string()"}
        linked_subdir:  linked.to_string()"}"
    modules.insert(package1.to_string(), module1), ".to_string(), module2)"
        linked_subdir:  linked.to_string()}""
         , ;}""
        linked_subdir:  linked.to_string()}"
         "}
        package_subdir:  ", fixed"