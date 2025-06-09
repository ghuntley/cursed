//! Expression code generation for LLVM IR
//!
//! This module handles the compilation of various expression types from the CURSED
//! Abstract Syntax Tree (AST) to LLVM Intermediate Representation (IR). It serves
//! as the primary dispatcher for expression compilation, routing different expression
//! types to their appropriate compilation handlers.
//!
//! ## Supported Expression Types
//!
//! Currently supported expression types include:
//!
//! * **Slice Literals** - `[]Type{element1, element2, ...}` syntax
//!   - Compiles to LLVM struct containing data pointer and length
//!   - Supports all CURSED primitive types (normie, tea, lit, etc.)
//!   - Runtime bounds checking and memory management integration
//!   - Automatic type inference from element type annotation
//!
//! ## Architecture
//!
//! The expression compilation follows a pattern where:
//! 1. Expressions are dispatched by type using `as_any().downcast_ref()`
//! 2. Specialized compilation functions handle each expression type
//! 3. Type inference and validation occur during compilation
//! 4. LLVM IR generation includes proper error handling and tracing
//!
//! ## Integration
//!
//! This module integrates with:
//! - Parser layer for AST input
//! - Type checker for type inference and validation
//! - LLVM backend for IR generation
//! - Runtime system for memory management and operations
//!
//! ## Future Extensions
//!
//! Additional expression types can be added by:
//! 1. Adding a new downcast branch in `generate_expression`
//! 2. Implementing a specialized compilation function
//! 3. Integrating with appropriate type checking and validation

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue};
use crate::ast::Expression;
use crate::ast::expressions::slice_literal::SliceLiteral;
use crate::codegen::llvm::slice_literal::{SliceLiteralCompiler, SliceLiteralCompilerImpl};
use crate::codegen::llvm::slice_operations::{SliceOperations, create_slice_operations};
use crate::core::type_checker::Type;
use crate::error::Error;
use std::any::Any;
use tracing::{debug, error, info, instrument, warn};

/// Generate LLVM IR for an expression.
#[instrument(skip(context, module, builder, expression, function), level = "debug")]
pub fn generate_expression<'ctx>(
    context: &'ctx Context,
    module: &Module<'ctx>,
    builder: &Builder<'ctx>,
    expression: &dyn Expression,
    function: &FunctionValue<'ctx>,
) -> Result<BasicValueEnum<'ctx>, String> {
    debug!("Generating LLVM IR for expression: {}", expression.string());
    
    // Try to downcast to specific expression types
    if let Some(slice_literal) = expression.as_any().downcast_ref::<SliceLiteral>() {
        info!("Compiling slice literal expression with {} elements", slice_literal.elements.len());
        return compile_slice_literal_expression(context, module, builder, slice_literal)
            .map_err(|e| {
                error!(error = ?e, slice_elements = slice_literal.elements.len(), 
                       "Failed to compile slice literal");
                format!("Failed to compile slice literal: {}", e)
            });
    }
    
    // Handle other expression types here
    // TODO: Add more expression type handlers as they are implemented
    
    warn!("Unsupported expression type: {}", expression.string());
    Err(format!("Unsupported expression type: {}", expression.string()))
}

/// Compile a slice literal expression to LLVM IR
#[instrument(skip(context, module, builder, slice_literal), level = "debug")]
fn compile_slice_literal_expression<'ctx>(
    context: &'ctx Context,
    module: &Module<'ctx>,
    builder: &Builder<'ctx>,
    slice_literal: &SliceLiteral,
) -> Result<BasicValueEnum<'ctx>, String> {
    debug!("Compiling slice literal with {} elements", slice_literal.elements.len());
    
    // Extract element type from slice_literal.element_type
    let element_type = infer_element_type(slice_literal)
        .map_err(|e| format!("Type inference failed: {}", e))?;
    
    info!("Inferred element type: {:?} for slice with {} elements", 
          element_type, slice_literal.elements.len());
    
    // Use the slice compiler to generate LLVM IR
    let compiler = SliceLiteralCompilerImpl;
    let compile_result = compiler.compile_slice_literal(context, module, builder, slice_literal, &element_type);
    
    match compile_result {
        Ok(value) => {
            info!("Successfully compiled slice literal to LLVM IR");
            Ok(value)
        },
        Err(e) => {
            error!(error = ?e, element_type = ?element_type, 
                   elements_count = slice_literal.elements.len(),
                   "Slice literal compilation failed");
            Err(format!("Slice literal compilation failed: {}", e))
        }
    }
}

/// Infer the element type from a slice literal
/// TODO: This should be replaced with proper type checking integration
#[instrument(skip(slice_literal), level = "debug")]
fn infer_element_type(slice_literal: &SliceLiteral) -> Result<Type, String> {
    // This is a placeholder implementation
    // In a real implementation, this would:
    // 1. Look up the element_type expression in the type checker
    // 2. Resolve any type parameters or generic types
    // 3. Return the concrete type
    
    let type_expr_string = slice_literal.element_type.string();
    debug!("Inferring element type from expression: {}", type_expr_string);
    
    match type_expr_string.as_str() {
        "lit" => Ok(Type::Lit),
        "smol" => Ok(Type::Smol),
        "mid" => Ok(Type::Mid),
        "normie" => Ok(Type::Normie),
        "thicc" => Ok(Type::Thicc),
        "snack" => Ok(Type::Snack),
        "meal" => Ok(Type::Meal),
        "tea" => Ok(Type::Tea),
        "sip" => Ok(Type::Sip),
        "rune" => Ok(Type::Rune),
        "byte" => Ok(Type::Byte),
        "extra" => Ok(Type::Extra),
        _ => {
            warn!("Unknown type in slice literal: {}", type_expr_string);
            Err(format!("Unknown element type: {}", type_expr_string))
        }
    }
}