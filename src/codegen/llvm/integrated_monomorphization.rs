//! Integrated Monomorphization Module for LLVM code generation
//!
//! This module integrates the various monomorphization components to provide a
//! complete generic code generation system.

use crate::ast::declarations::{FunctionStatement, SquadStatement, GenericConstraint};
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::enhanced_monomorphization::EnhancedMonomorphization;
use crate::codegen::llvm::improved_field_accessors::ImprovedFieldAccessors;
use crate::codegen::llvm::struct_monomorphization::StructMonomorphization;
use crate::core::type_checker::Type;
use crate::error::Error;
use std::collections::{HashMap, HashSet};
use tracing::{debug, info, warn, error, span, Level};

/// Trait for fully integrated monomorphization
pub trait IntegratedMonomorphization<'ctx> {
    /// Generate a specialized struct with field accessors
    fn generate_specialized_struct_with_accessors(
        &mut self,
        generic_struct: &SquadStatement,
        specialized_name: &str,
        type_args: &[Type],
    ) -> Result<(), Error>;
}

impl<'ctx> IntegratedMonomorphization<'ctx> for LlvmCodeGenerator<'ctx> {
    /// Generate a specialized struct with field accessors
    #[tracing::instrument(skip(self, generic_struct), fields(struct_name = %generic_struct.name.value, specialized_name = %specialized_name), level = "info")]
    fn generate_specialized_struct_with_accessors(
        &mut self,
        generic_struct: &SquadStatement,
        specialized_name: &str,
        type_args: &[Type],
    ) -> Result<(), Error> {
        // First, generate the specialized struct
        // We need to explicitly call the StructMonomorphization trait method
        let _struct_type = crate::codegen::llvm::StructMonomorphization::generate_specialized_struct(
            self,
            generic_struct,
            specialized_name,
            type_args,
        )?;
        
        debug!("Specialized struct created, now generating field accessors");
        
        // Then, generate the field accessors using our improved implementation directly
        debug!("Using improved field accessors with proper error handling");
        self.generate_improved_field_accessors(generic_struct, specialized_name, type_args)?;
        
        // Register with interface registry if needed
        // Note: SquadStatement doesn't have interface constraints directly,
        // so we just log that this functionality will be handled by the interface system instead
        debug!("Specialized struct created, interface registration handled by interface system");
        
        info!("Specialized struct with accessors generated successfully");
        
        Ok(())
    }
}

/// Extension to the LlvmCodeGenerator to register the integrated monomorphization module
pub fn register_integrated_monomorphization() {
    debug!("Registering integrated monomorphization module");
    // No actual registration needed, the trait implementation automatically becomes available
}