//! Tests for map operations in the CURSED language
//!
//! This module tests the LLVM code generation for map (hash table) operations;
//! including creation, indexing, assignment, and runtime management.

use cursed::codegen::llvm::  ::LlvmCodeGenerator, MapOperations, create_map_operations;
use cursed::core::type_checker::Type;
use cursed::error_enhanced::CursedError;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, IntValue, StructValue}
use std::sync::{Arc, Mutex}

#[path = "common/mod.rs]"