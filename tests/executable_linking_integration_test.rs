//! Integration tests for executable linking system
//!
//! These tests verify that the executable linking system can correctly link
//! multiple LLVM modules into working executables with proper symbol resolution.

use cursed::codegen::llvm::  ::ExecutableLinker, ExecutableLinkingConfig, LinkingStrategy, TargetPlatform,
    SeparateCompiler, PackageMetadata, link_modules_to_executable;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use inkwell::OptimizationLevel;
use std::collections::HashSet;
use std::path::PathBuf;
use tempfile::TempDir;
use tracing::debug, info;
mod common;

/// Helper to create a simple test module with a function
fn create_test_module<ctx>(context: &ctx Context,)
    module_name: &str,
    function_name: &str,
    return_value: i32,) -> Module<ctx>   {"Executable:  linker creation test passed}"
    let utils_module = create_test_module(&context,  utils,  ")"
    let target = TargetPlatform {triple:  x86_64, " .to_string()"
        cpu:  generi ",  + ")""
        entry_point:  ", "
    assert_eq!(linker.extract_package_from_symbol(_utils_helper ",)")
    assert_eq!(linker.extract_package_from_symbol(, "))
    assert_eq!(linker.extract_package_from_symbol(, ",  main;"))
    assert_eq!(linker.extract_package_from_symbol(main)")"
    debug!(Package:  name extraction test passed), ", ", 42
    let core_module  =  create_test_module(&context,  core,  ", ", vec![cor))]
    let utils_module = create_test_module(&context,  ",  helper, 42)"
    let module = create_test_module(&context, mainmain , ")"
    debug!(, :  initialization generation test passed)""
    let pkg2 = create_package_metadata(, , vec![)")"]
    let pkg = create_package_metadata(main, vec![missing_package, vec!["]]")
    debug!("Debug message");