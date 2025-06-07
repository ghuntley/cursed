//! # Interface Type Assertion Common Utilities
//!
//! This module provides common utilities and shared functions for the interface type assertion system.
//! It centralizes duplicate definitions that were previously spread across multiple files.

use std::sync::Arc;
use tracing::{debug, error, info, instrument, trace, warn};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue, FunctionValue};
use inkwell::IntPredicate;
use inkwell::basic_block::BasicBlock;
use inkwell::AddressSpace;

use crate::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_type_registry_helpers::TypeNameRegistry;
use crate::error::Error;
use crate::error::SourceLocation;

/// Get the LLVM type for Result structure
pub fn get_result_type<'ctx>(codegen: &LlvmCodeGenerator<'ctx>, value_type: BasicTypeEnum<'ctx>) -> StructType<'ctx> {
    let ctx = codegen.context();
    
    // Enhanced Result structure:
    // 1. Value of generic type
    // 2. Success flag (bool)
    // 3. Error message (string pointer)
    // 4. Source location information
    // 5. Expected type ID (i32) - for error reporting
    // 6. Actual type ID (i32) - for error reporting
    
    ctx.struct_type(&[
        value_type,
        ctx.bool_type().into(),
        ctx.i8_type().ptr_type(AddressSpace::default()).into(),
        get_source_location_type(codegen).into(),
        ctx.i32_type().into(), // Expected type ID
        ctx.i32_type().into()  // Actual type ID
    ], false)
}

/// Get the LLVM type for source location information
pub fn get_source_location_type<'ctx>(codegen: &LlvmCodeGenerator<'ctx>) -> StructType<'ctx> {
    let ctx = codegen.context();
    
    // Source location structure:
    // 1. Line number (i32)
    // 2. Column number (i32)
    // 3. File name (string pointer)
    // 4. Source line text (string pointer)
    
    ctx.struct_type(&[
        ctx.i32_type().into(),
        ctx.i32_type().into(),
        ctx.i8_type().ptr_type(AddressSpace::default()).into(),
        ctx.i8_type().ptr_type(AddressSpace::default()).into()
    ], false)
}

/// Build a struct value from field values
pub fn build_struct_value<'ctx>(codegen: &LlvmCodeGenerator<'ctx>, fields: &[BasicValueEnum<'ctx>]) -> inkwell::values::StructValue<'ctx> {
    let ctx = codegen.context();
    let builder = codegen.builder();
    
    // Create struct type from field types
    let struct_type = ctx.struct_type(
        &fields.iter().map(|v| v.get_type()).collect::<Vec<_>>(),
        false
    );
    
    // Create empty struct
    let mut struct_value = struct_type.const_named_struct(&[]);
    
    // Insert each field
    for (i, field) in fields.iter().enumerate() {
        struct_value = builder.build_insert_value(
            struct_value,
            *field,
            i as u32,
            &format!("field_{}", i)
        ).expect("Failed to insert struct field").into_struct_value();
    }
    
    struct_value
}

/// Call the runtime error propagation function with enhanced type information
pub fn call_error_propagation_function<'ctx>(
    codegen: &LlvmCodeGenerator<'ctx>,
    error_message: BasicValueEnum<'ctx>,
    location_info: BasicValueEnum<'ctx>
) -> Result<BasicValueEnum<'ctx>, Error> {
    // Get current module and context
    let module = codegen.module();
    let ctx = codegen.context();
    let builder = codegen.builder();
    
    // Get or declare the enhanced error propagation function with type information
    let propagate_fn = match module.get_function("__cursed_propagate_error_with_type_info") {
        Some(func) => func,
        None => {
            // Declare the enhanced function if it doesn't exist
            let void_type = ctx.void_type();
            let fn_type = void_type.fn_type(&[
                // Error message
                ctx.i8_type().ptr_type(AddressSpace::default()).into(),
                // Source location info
                get_source_location_type(codegen).into(),
                // Expected type ID
                ctx.i32_type().into(),
                // Actual type ID 
                ctx.i32_type().into(),
                // Error type (1 = type assertion error)
                ctx.i32_type().into()
            ], false);
            
            module.add_function("__cursed_propagate_error_with_type_info", fn_type, None)
        }
    };
    
    // Default type IDs (would be replaced with actual values in the full implementation)
    let expected_type_id = ctx.i32_type().const_int(0, false);
    let actual_type_id = ctx.i32_type().const_int(0, false);
    
    // Type assertion error code = 1
    let error_type = ctx.i32_type().const_int(1, false);
    
    // Call the enhanced function with type information
    builder.build_call(
        propagate_fn,
        &[
            error_message.into(),
            location_info.into(),
            expected_type_id.into(),
            actual_type_id.into(),
            error_type.into()
        ],
        "propagate_error_call"
    ).map_err(|e| Error::Compilation(e.to_string()))?;
    
    // This function should never return normally, but we need to emit valid LLVM IR
    Ok(ctx.i8_type().const_int(0, false).into())
}

/// Function to determine if a type is a string type by examining the type name
/// 
/// This is a wrapper that supports checking string types by string name rather than LLVM type.
/// For LLVM type checks, use the StringUtilsExtension trait methods directly.
pub fn is_string_type_by_name(type_name: &str) -> bool {
    match type_name {
        "string" | "String" | "StringType" | "str" | "&str" => true,
        _ => type_name.contains("String") || type_name.contains("string")
    }
}

/// Create a string constant in the module using the string utils extension
/// 
/// This is a wrapper around the StringUtilsExtension trait method to be used in contexts
/// where the trait itself is not in scope.
/// For direct string constant creation, use the StringUtilsExtension trait methods.
pub fn create_string_constant_from_codegen<'ctx>(codegen: &mut LlvmCodeGenerator<'ctx>, value: &str) -> Result<PointerValue<'ctx>, Error> {
    use crate::codegen::llvm::string_utils::StringUtilsExtension;
    codegen.create_string_constant(value)
}

/// Find an inheritance path between types
pub fn find_inheritance_path(source_type: &str, target_type: &str, registry: &impl InterfaceRegistry) -> Result<Vec<String>, Error> {
    // Check direct implementation
    if registry.type_implements(source_type, target_type)? {
        return Ok(vec![source_type.to_string(), target_type.to_string()]);
    }
    
    // Implementation would involve breadth-first search through the inheritance hierarchy
    // This is a simplified version that just returns an empty path if no direct relationship
    Ok(vec![])
}

/// Trait for interface registry functionality
pub trait InterfaceRegistry {
    /// Check if a type implements an interface
    fn type_implements(&self, concrete_type: &str, interface_type: &str) -> Result<bool, Error>;
    
    /// Get type name by ID
    fn get_type_name_by_id(&self, type_id: u32) -> Result<String, Error>;
}

/// Helper function to convert Option<String> to Result<String, Error>
/// This should be used to standardize converting from TypeNameRegistry::get_type_name_by_id
/// to a Result type.
/// 
/// DEPRECATED: Use interface_type_registry_common::get_type_name_by_id_impl instead
pub fn get_type_name_by_id_result<T: TypeNameRegistry>(registry: &T, type_id: u32) -> Result<String, Error> {
    // Use the trait method directly
    TypeNameRegistry::get_type_name_by_id(registry, type_id)
        .ok_or_else(|| Error::Compilation(format!("Could not find type name for ID {}", type_id)))
}

/// Helper to find interface paths
pub trait InterfacePathFinder {
    /// Find all inheritance paths between types
    fn find_all_paths(&self, source_type: &str, target_type: &str, max_depth: usize) -> Vec<Vec<String>>;
}

/// Get the interface registry accessor
pub fn get_interface_registry(codegen: &LlvmCodeGenerator) -> Result<Arc<dyn InterfaceRegistry>, Error> {
    // This would return the actual registry in a full implementation
    Err(Error::Compilation("Registry not available".to_string()))
}

/// Get the interface path finder
pub fn get_interface_path_finder(codegen: &LlvmCodeGenerator) -> Result<Arc<dyn InterfacePathFinder>, Error> {
    // This would return the actual path finder in a full implementation
    Err(Error::Compilation("Path finder not available".to_string()))
}

/// Detect diamond inheritance in the interface hierarchy
pub fn detect_diamond_inheritance(type_name: &str, registry: &impl InterfaceRegistry) -> Result<bool, Error> {
    // This would implement diamond inheritance detection in a full implementation
    Ok(false)
}

/// Interface registry with mutation capabilities
pub trait MutableInterfaceRegistry: InterfaceRegistry {
    /// Register a new implementation
    fn register_implementation(&mut self, concrete_type: &str, interface_type: &str) -> Result<(), Error>;
}

/// Get a mutable reference to the interface registry
pub fn get_interface_registry_mut(codegen: &mut LlvmCodeGenerator) -> Result<Arc<dyn MutableInterfaceRegistry>, Error> {
    // This would return the actual mutable registry in a full implementation
    Err(Error::Compilation("Mutable registry not available".to_string()))
}