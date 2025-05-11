//! Extension trait for fixed range clause methods
//!
//! This module provides a public extension trait that exposes the fixed range clause methods
//! from LlvmCodeGenerator for use by the error recovery implementation.

use crate::codegen::llvm::context::LlvmCodeGenerator;
use crate::error::Error;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};

/// Extension trait for fixed range clause methods
///
/// This trait exposes the methods needed by the range clause error recovery implementation
/// that are used to handle container and map iteration properly.
pub trait RangeClauseFixedMethodsExtension<'ctx> {
    /// Get the length of a container
    fn emit_container_length_fixed(&self, container: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, Error>;
    
    /// Get an element from a container by index
    fn emit_get_element_fixed(&self, container: BasicValueEnum<'ctx>, index: IntValue<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Determine the element type of a container
    fn determine_element_type_fixed(&self, container: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error>;
    
    /// Create a map iterator
    fn emit_map_iterator_create_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<PointerValue<'ctx>, Error>;
    
    /// Check if a map iterator has a next element
    fn emit_map_iterator_has_next_fixed(&self, iterator_ptr: PointerValue<'ctx>) -> Result<IntValue<'ctx>, Error>;
    
    /// Get the current key-value pair from a map iterator
    fn emit_map_iterator_get_current_fixed(
        &self,
        iterator_ptr: PointerValue<'ctx>,
        key_ptr: PointerValue<'ctx>,
        value_ptr: PointerValue<'ctx>,
    ) -> Result<(), Error>;
    
    /// Advance a map iterator to the next element
    fn emit_map_iterator_next_fixed(&self, iterator_ptr: PointerValue<'ctx>) -> Result<(), Error>;
    
    /// Determine the key type of a map
    fn determine_map_key_type_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error>;
    
    /// Determine the value type of a map
    fn determine_map_value_type_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error>;
}

// This trait is implemented for LlvmCodeGenerator in the range_clause_fixed.rs module,
// but we're declaring it here to make the methods publicly accessible for use by the
// error recovery implementation.
impl<'ctx> RangeClauseFixedMethodsExtension<'ctx> for LlvmCodeGenerator<'ctx> {}