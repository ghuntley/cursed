//! Integrated Monomorphization Module for LLVM code generation
//!
//! This module integrates the various monomorphization components to provide a
//! complete generic code generation system.

use crate::ast::declarations::{FunctionStatement, SquadStatement, GenericConstraint};
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::enhanced_monomorphization::EnhancedMonomorphization;
use crate::codegen::llvm::improved_field_accessors::ImprovedFieldAccessors;
use crate::codegen::llvm::lru_field_accessors::{LruCachedFieldAccessors, ThreadSafeFieldAccessorLruCache};
use crate::codegen::llvm::struct_monomorphization::StructMonomorphization;
use crate::core::type_checker::Type;
use crate::error::Error;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use rand;
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
        // Ensure the LRU field accessor cache is initialized before any processing
        self.ensure_lru_field_accessor_cache();
        
        // First, generate the specialized struct
        // We need to explicitly call the StructMonomorphization trait method
        let _struct_type = crate::codegen::llvm::StructMonomorphization::generate_specialized_struct(
            self,
            generic_struct,
            specialized_name,
            type_args,
        )?;
        
        debug!("Specialized struct created, now generating field accessors");
        
        // Then, generate the field accessors using our LRU cached implementation
        debug!("Using LRU cached field accessors for better performance");
        
        // Generate field accessors with LRU caching
        use crate::codegen::llvm::lru_field_accessors::LruCachedFieldAccessors;
        self.generate_lru_cached_field_accessors(generic_struct, specialized_name, type_args)?;
        
        // Log cache statistics periodically
        if rand::random::<f32>() < 0.05 { // 5% chance to log stats
            if let Some(stats) = self.get_lru_field_accessor_cache_stats() {
                info!("Field accessor LRU cache stats: {}", stats);
            }
        }
        
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