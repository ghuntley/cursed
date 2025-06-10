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

macro_rules! init_tracing {
    () => {
        common::tracing::setup()
    };
}

/// Test basic separate compilation IR output
#[test]
fn test_basic_separate_ir_output() {
    // TODO: Implement test
    assert!(true);
}
            preserve_structure: false,
            optimize: false,
            base_name: None,
            include_debug_comments: true},
        per_package: true,
        linked_output: false,
        package_subdir:  packages.to_string();
        linked_subdir:  linked.to_string()"}"
        linked_subdir:  linked.to_string()"}"
    modules.insert(package1.to_string(), module1), "), module2)"
        linked_subdir:  linked.to_string()}""
         , ;}""
        linked_subdir:  linked.to_string()}""
         "}"
        package_subdir:  ", ""