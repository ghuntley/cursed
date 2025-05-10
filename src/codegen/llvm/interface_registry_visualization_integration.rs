//! # Interface Registry Visualization Integration
//!
//! This module integrates the improved interface registry visualization with the LLVM code generator.
//! It provides methods to use the visualization during type assertions and error reporting.

use tracing::{debug, error, info, instrument, trace, warn};
use inkwell::values::BasicValueEnum;

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::core::interface_registry_visualization_improved::ImprovedInterfaceRegistryVisualization;
use crate::ast::expressions::TypeAssertion;
use crate::ast::traits::Node;
use crate::error::Error;

/// Trait for integrating interface registry visualization with code generation
pub trait InterfaceVisualizationIntegration<'ctx> {
    /// Compile a type assertion with enhanced error reporting using visualizations
    fn compile_type_assertion_with_visualization(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate a visual representation of an inheritance path for an error message
    fn generate_path_visualization(
        &self,
        source_type: &str,
        target_type: &str
    ) -> Result<Option<String>, Error>;
    
    /// Create an enhanced error message with visualization for type assertion failures
    fn create_enhanced_type_error(
        &self,
        source_type: &str,
        target_type: &str,
        source_location: &str
    ) -> Result<String, Error>;
}

impl<'ctx> InterfaceVisualizationIntegration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn compile_type_assertion_with_visualization(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling type assertion with visualization for: {}", type_assertion.string());
        
        // Get source location for better error messages
        let source_location = format!("{}", type_assertion.token_literal());
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // We'll need to determine the source type - for now, use a placeholder
        // In a real implementation, we would extract this from the expression's type
        let source_type = "UnknownSource"; // This should be derived from expr_value
        
        // Check if there's a valid path between the types
        let interface_registry = self.interface_registry.as_ref()
            .ok_or_else(|| Error::Compilation(
                format!("Interface registry not available for visualization at {}", source_location)
            ))?;
        
        // Attempt to find inheritance paths
        let paths = interface_registry.find_interface_paths(
            source_type, 
            &type_assertion.type_name,
            1
        )?;
        
        // If no valid path exists, provide enhanced error
        if paths.is_empty() {
            // This is where we would use the regular type assertion compilation with enhanced errors
            let error_message = interface_registry.generate_detailed_error_message(
                source_type,
                &type_assertion.type_name,
                &source_location
            )?;
            
            warn!("Type assertion failed: {}", error_message);
            
            // Fall back to normal type assertion compilation
            self.compile_type_assertion(type_assertion)
        } else {
            // A valid path exists, proceed with normal compilation
            self.compile_type_assertion(type_assertion)
        }
    }
    
    fn generate_path_visualization(
        &self,
        source_type: &str,
        target_type: &str
    ) -> Result<Option<String>, Error> {
        let interface_registry = self.interface_registry.as_ref()
            .ok_or_else(|| Error::Compilation(
                format!("Interface registry not available for visualization")
            ))?;
        
        // Attempt to find paths
        let paths = interface_registry.find_interface_paths(source_type, target_type, 1)?;
        
        if paths.is_empty() {
            return Ok(None);
        }
        
        // Create a simple ASCII visualization of the path
        let mut visualization = String::new();
        
        for (i, path) in paths.iter().enumerate() {
            if i > 0 {
                visualization.push_str("\n");
            }
            
            visualization.push_str("Path: ");
            
            for (j, item) in path.iter().enumerate() {
                if j > 0 {
                    visualization.push_str(" → ");
                }
                visualization.push_str(item);
            }
        }
        
        Ok(Some(visualization))
    }
    
    fn create_enhanced_type_error(
        &self,
        source_type: &str,
        target_type: &str,
        source_location: &str
    ) -> Result<String, Error> {
        let interface_registry = self.interface_registry.as_ref()
            .ok_or_else(|| Error::Compilation(
                format!("Interface registry not available for error enhancement")
            ))?;
        
        interface_registry.generate_detailed_error_message(
            source_type,
            target_type,
            source_location
        )
    }
}

/// Register the interface visualization integration
pub fn register_interface_visualization_integration() {
    trace!("Interface visualization integration registered");
}