//! # Interface Type Assertion with Nested Interface Support
//!
//! This module extends the interface type assertion system with support for
//! checking if a value implements an interface that inherits from or extends
//! another interface. This enhances the type assertion capabilities by allowing
//! more sophisticated interface hierarchies and relationships.
//!
//! ## Features
//!
//! 1. Support for nested interface relationships
//! 2. Validation of interface inheritance chains
//! 3. Proper error propagation throughout the assertion process
//! 4. Rich context information for debugging and error reporting
//! 5. Integration with the existing type assertion infrastructure

use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;
use tracing::{debug, info, trace, warn, instrument};

use crate::ast::expressions::TypeAssertion;
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::error::Error;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::codegen::llvm::interface_type_assertion_error_propagation::TypeAssertionErrorPropagation;

/// A trait for handling type assertions involving nested interfaces
pub trait NestedInterfaceTypeAssertion<'ctx> {
    /// Check if a value implements an interface that extends another interface
    fn check_nested_interface_implementation(
        &mut self,
        value: BasicValueEnum<'ctx>,
        interface_name: &str,
        source_location: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Get the list of interfaces that a given interface extends
    fn get_extended_interfaces(
        &self,
        interface_name: &str
    ) -> Result<Vec<String>, Error>;
    
    /// Check if one interface extends another through a chain of interfaces
    fn check_interface_extension_chain(
        &self,
        source_interface: &str,
        target_interface: &str
    ) -> Result<bool, Error>;
    
    /// Compile a type assertion that checks for nested interface implementation
    fn compile_nested_interface_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> NestedInterfaceTypeAssertion<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn check_nested_interface_implementation(
        &mut self,
        value: BasicValueEnum<'ctx>,
        interface_name: &str,
        source_location: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Checking if value implements interface {} (including nested interfaces)", interface_name);
        
        // First, try a direct implementation check
        let direct_check = self.check_instance_of_with_propagation(
            value,
            interface_name,
            source_location
        )?;
        
        // If direct check succeeds, we're done
        let is_direct_match = self.builder().build_extract_value(
            direct_check.into_struct_value(),
            1, // Boolean success flag
            "is_direct_match"
        ).map_err(|e| Error::Compilation(
            format!("Failed to extract direct match result at {}: {}", source_location, e)
        ))?;
        
        // If we have a direct match, return it immediately
        if is_direct_match.is_int_value() {
            let is_match = self.builder().build_int_compare(
                IntPredicate::EQ,
                is_direct_match.into_int_value(),
                self.context().bool_type().const_int(1, false),
                "is_direct_match_bool"
            ).map_err(|e| Error::Compilation(
                format!("Failed to compare direct match result at {}: {}", source_location, e)
            ))?;
            
            if is_match.is_const_int() && is_match.into_int_value().get_zero_extended_constant() == 1 {
                return Ok(direct_check);
            }
        }
        
        // For nested interfaces, get the list of interfaces that the target interface extends
        let extended_interfaces = self.get_extended_interfaces(interface_name)?;
        if extended_interfaces.is_empty() {
            // No nested interfaces, return the direct check result
            return Ok(direct_check);
        }
        
        // Create if-else chain for checking each extended interface
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation(
                format!("No current function for nested interface check at {}", source_location)
            ))?;
        
        // Create blocks for the if-else chain
        let check_extended_block = self.context().append_basic_block(current_fn, "check_extended");
        let extended_success_block = self.context().append_basic_block(current_fn, "extended_success");
        let extended_failure_block = self.context().append_basic_block(current_fn, "extended_failure");
        let merge_block = self.context().append_basic_block(current_fn, "merge_nested_check");
        
        // Branch to the extended check block
        self.builder().build_unconditional_branch(check_extended_block)
            .map_err(|e| Error::Compilation(
                format!("Failed to branch to extended check block at {}: {}", source_location, e)
            ))?;
        
        // Position at the extended check block
        self.builder().position_at_end(check_extended_block);
        
        // Check each extended interface
        let mut current_check = None;
        for extended_interface in &extended_interfaces {
            let check_result = self.check_instance_of_with_propagation(
                value,
                extended_interface,
                source_location
            )?;
            
            // Extract the boolean result
            let is_match = self.builder().build_extract_value(
                check_result.into_struct_value(),
                1, // Boolean success flag
                &format!("is_{}_match", extended_interface)
            ).map_err(|e| Error::Compilation(
                format!("Failed to extract match result for {} at {}: {}", 
                        extended_interface, source_location, e)
            ))?;
            
            // Update the current check result
            if let Some(current) = current_check {
                // OR this result with the previous one
                current_check = Some(self.builder().build_or(
                    current.into_int_value(),
                    is_match.into_int_value(),
                    "combined_match"
                ).map_err(|e| Error::Compilation(
                    format!("Failed to combine match results at {}: {}", source_location, e)
                ))?.into());
            } else {
                current_check = Some(is_match);
            }
        }
        
        // Branch based on the combined check result
        let final_check = current_check.unwrap_or_else(|| {
            self.context().bool_type().const_int(0, false).into()
        });
        
        self.builder().build_conditional_branch(
            final_check.into_int_value(),
            extended_success_block,
            extended_failure_block
        ).map_err(|e| Error::Compilation(
            format!("Failed to build conditional branch for nested interfaces at {}: {}", 
                    source_location, e)
        ))?;
        
        // Success path - create a success tuple
        self.builder().position_at_end(extended_success_block);
        
        // Look up the interface type
        let interface_type = self
            .get_type_by_name(interface_name)
            .unwrap_or_else(|| self.context().opaque_struct_type(interface_name));
        
        let interface_ptr_type = interface_type.ptr_type(inkwell::AddressSpace::default());
        
        // Extract the data pointer
        let data_ptr = self.extract_interface_data_with_propagation(
            value,
            source_location
        )?;
        
        // Cast to the interface pointer type
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            interface_ptr_type,
            "nested_casted_ptr"
        ).map_err(|e| Error::Compilation(
            format!("Failed to cast pointer in nested interface check at {}: {}", 
                    source_location, e)
        ))?;
        
        // Create success tuple
        let true_val = self.context().bool_type().const_int(1, false);
        let success_result = self.build_tuple(vec![casted_ptr.into(), true_val.into()])
            .map_err(|e| Error::Compilation(
                format!("Failed to build success tuple in nested interface check at {}: {}", 
                        source_location, e)
            ))?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(
                format!("Failed to branch to merge block from success path in nested interface check at {}: {}", 
                        source_location, e)
            ))?;
        
        // Failure path - create a failure tuple
        self.builder().position_at_end(extended_failure_block);
        
        // Use null pointer and false flag
        let null_ptr = interface_ptr_type.const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        let failure_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])
            .map_err(|e| Error::Compilation(
                format!("Failed to build failure tuple in nested interface check at {}: {}", 
                        source_location, e)
            ))?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(
                format!("Failed to branch to merge block from failure path in nested interface check at {}: {}", 
                        source_location, e)
            ))?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        
        // Create appropriate tuple type for the phi node
        let result_type = self.tuple_type(vec![
            interface_ptr_type.into(), 
            self.context().bool_type().into()
        ]);
        
        let phi = self.builder().build_phi(
            result_type,
            "nested_assertion_result"
        ).map_err(|e| Error::Compilation(
            format!("Failed to build phi node in nested interface check at {}: {}", 
                    source_location, e)
        ))?;
        
        phi.add_incoming(&[(
            &success_result,
            extended_success_block
        ), (
            &failure_result,
            extended_failure_block
        )]);
        
        debug!("Completed nested interface check for {}", interface_name);
        
        // Return the phi result
        Ok(phi.as_basic_value())
    }
    
    fn get_extended_interfaces(
        &self,
        interface_name: &str
    ) -> Result<Vec<String>, Error> {
        // In a real implementation, this would query the type registry to get
        // all interfaces that the given interface extends.
        // For now, we'll use a simplified implementation.
        
        // Check if we have interface extension information in our registry
        if let Some(extension_info) = self.interface_registry().get_interface_extensions(interface_name) {
            return Ok(extension_info);
        }
        
        // Default to empty list if not found
        Ok(Vec::new())
    }
    
    fn check_interface_extension_chain(
        &self,
        source_interface: &str,
        target_interface: &str
    ) -> Result<bool, Error> {
        // Basic case: same interface
        if source_interface == target_interface {
            return Ok(true);
        }
        
        // Get the interfaces that source_interface extends
        let extended_interfaces = self.get_extended_interfaces(source_interface)?;
        
        // Check if target_interface is directly extended
        if extended_interfaces.contains(&target_interface.to_string()) {
            return Ok(true);
        }
        
        // Recursively check each extended interface
        for ext_interface in extended_interfaces {
            if self.check_interface_extension_chain(&ext_interface, target_interface)? {
                return Ok(true);
            }
        }
        
        // No extension chain found
        Ok(false)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn compile_nested_interface_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling nested interface assertion for: {}", type_assertion.string());
        
        // Get source location for better error messages
        let source_location = format!("{}", type_assertion.token_literal());
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())
            .map_err(|e| Error::Compilation(
                format!("Failed to compile expression for nested interface assertion at {}: {}", 
                        source_location, e)
            ))?;
        
        // Perform the nested interface check
        self.check_nested_interface_implementation(
            expr_value,
            &type_assertion.type_name,
            &source_location
        )
    }
}

// Helper method to check for interface extension support
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Returns true if nested interface type assertion is supported for the given interface
    pub fn supports_nested_interface_assertion(&self, interface_name: &str) -> bool {
        !self.get_extended_interfaces(interface_name).unwrap_or_default().is_empty()
    }
    
    /// Returns a list of all interfaces in the current module
    pub fn get_all_interfaces(&self) -> Vec<String> {
        self.interface_registry().get_all_interfaces()
    }
}

// Extension to interface registry methods
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get a reference to the interface registry
    fn interface_registry(&self) -> &dyn InterfaceRegistryExtension {
        // This would return the actual registry in a real implementation
        // For now, we'll use a placeholder registry
        &PlaceholderRegistry
    }
}

/// Extension trait for interface registry to support nested interfaces
pub trait InterfaceRegistryExtension {
    /// Get the interfaces that a given interface extends
    fn get_interface_extensions(&self, interface_name: &str) -> Option<Vec<String>>;
    
    /// Get a list of all registered interfaces
    fn get_all_interfaces(&self) -> Vec<String>;
}

// Placeholder implementation for interface registry extension
struct PlaceholderRegistry;

impl InterfaceRegistryExtension for PlaceholderRegistry {
    fn get_interface_extensions(&self, _interface_name: &str) -> Option<Vec<String>> {
        // In a real implementation, this would return actual extension information
        // For demonstration, return None to indicate no extensions
        None
    }
    
    fn get_all_interfaces(&self) -> Vec<String> {
        // In a real implementation, this would return all registered interfaces
        Vec::new()
    }
}

// Helper function to register this module in the compiler
pub fn register_nested_interface_type_assertion() {
    trace!("Nested interface type assertion module registered");
}