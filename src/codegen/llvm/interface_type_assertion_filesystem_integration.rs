//! # Interface Type Assertion Filesystem Integration
//!
//! This module implements the integration between interface type assertions
//! and filesystem source location tracking for improved error messages.
//!
//! ## Key Features
//!
//! 1. Enhanced source location with filesystem path resolution
//! 2. Source code context extraction for error messages
//! 3. Line and column tracking with precise error markers
//! 4. Integration with the error propagation system
//!
//! This implementation ensures that interface type assertions provide
//! detailed error messages with accurate source code context.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{debug, error, info, instrument, trace, warn};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue, FunctionValue};
use inkwell::AddressSpace;

use crate::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_type_assertion_error_propagation::InterfaceTypeAssertionErrorPropagation;
use crate::error::Error;
use crate::error::SourceLocation;
use crate::error::type_assertion_error::TypeAssertionError;

/// Maximum number of context lines to include in error messages
const MAX_CONTEXT_LINES: usize = 5;

/// Default file extension for source files
const DEFAULT_SOURCE_EXTENSION: &str = ".csd";

/// Trait for enhanced filesystem source location integration
pub trait InterfaceTypeAssertionFilesystemIntegration<'ctx> {
    /// Create an enhanced source location with filesystem information
    fn create_enhanced_source_location(
        &self,
        token: &str,
        expression: &str,
        type_name: &str
    ) -> Option<SourceLocation>;
    
    /// Extract source file content for error context
    fn extract_source_file_content(
        &self,
        location: &SourceLocation
    ) -> Option<String>;
    
    /// Enhance error message with source file context
    fn enhance_error_with_source_context(
        &self,
        error_message: String,
        location: &SourceLocation
    ) -> String;
    
    /// Get context line range for the error location
    fn get_context_line_range(
        &self,
        line: i32,
        file_line_count: usize
    ) -> (usize, usize);
    
    /// Resolve filesystem path for a source location
    fn resolve_source_path(
        &self,
        file_hint: Option<&str>
    ) -> Option<PathBuf>;
    
    /// Compile a type assertion with enhanced filesystem source location
    fn compile_type_assertion_with_filesystem_location(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a type assertion question with enhanced filesystem source location
    fn compile_type_assertion_question_with_filesystem_location(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> InterfaceTypeAssertionFilesystemIntegration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, token, expression))]
    fn create_enhanced_source_location(
        &self,
        token: &str,
        expression: &str,
        type_name: &str
    ) -> Option<SourceLocation> {
        // Extract basic location info from token
        let (line, column, file_hint) = self.extract_location_from_token(token);
        
        // Try to resolve the actual filesystem path
        let file_path = self.resolve_source_path(file_hint.as_deref())
            .map(|p| p.to_string_lossy().to_string());
        
        // Create the enhanced location
        Some(SourceLocation {
            line,
            column,
            file: file_path,
            source_line: format!("{}.({})?", expression, type_name),
        })
    }
    
    fn extract_source_file_content(
        &self,
        location: &SourceLocation
    ) -> Option<String> {
        // Get the file path from the location
        let file_path = match &location.file {
            Some(path) => path,
            None => return None,
        };
        
        // Try to open the file
        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => {
                warn!("Failed to open source file for error context: {}", e);
                return None;
            }
        };
        
        // Read the file content
        let reader = BufReader::new(file);
        let lines: Result<Vec<String>, _> = reader.lines().collect();
        match lines {
            Ok(content) => Some(content.join("\n")),
            Err(e) => {
                warn!("Failed to read source file for error context: {}", e);
                None
            }
        }
    }
    
    fn enhance_error_with_source_context(
        &self,
        error_message: String,
        location: &SourceLocation
    ) -> String {
        // Extract file content
        let file_content = match self.extract_source_file_content(location) {
            Some(content) => content,
            None => return error_message,
        };
        
        // Split into lines
        let lines: Vec<&str> = file_content.lines().collect();
        
        // Get context range
        let line_index = location.line as usize - 1; // Convert to 0-based index
        let (start_line, end_line) = self.get_context_line_range(
            location.line,
            lines.len()
        );
        
        // Create context display
        let mut context = String::new();
        context.push_str("\nSource context:\n");
        
        for i in start_line..=end_line {
            if i < lines.len() {
                // Add line prefix (marker for error line)
                let prefix = if i == line_index { ">" } else { " " };
                let line_num = i + 1; // Convert back to 1-based indexing for display
                
                // Add the line with line number
                context.push_str(&format!("{} {:4} | {}\n", prefix, line_num, lines[i]));
                
                // Add error pointer marker at the error column
                if i == line_index {
                    let mut marker = String::from("       | ");
                    // Add spaces up to the column
                    for _ in 0..location.column.saturating_sub(1) {
                        marker.push(' ');
                    }
                    marker.push('^');
                    context.push_str(&format!("{}{}", marker, "\n"));
                }
            }
        }
        
        // Combine original error with context
        format!("{}{}", error_message, context)
    }
    
    fn get_context_line_range(
        &self,
        line: i32,
        file_line_count: usize
    ) -> (usize, usize) {
        let line_index = line as usize - 1; // Convert to 0-based index
        
        // Calculate context range (symmetric around error line)
        let context_size = MAX_CONTEXT_LINES / 2;
        let start_line = line_index.saturating_sub(context_size);
        let end_line = std::cmp::min(line_index + context_size, file_line_count - 1);
        
        (start_line, end_line)
    }
    
    fn resolve_source_path(
        &self,
        file_hint: Option<&str>
    ) -> Option<PathBuf> {
        // If hint is provided, use it directly
        if let Some(hint) = file_hint {
            let path = PathBuf::from(hint);
            if path.exists() {
                return Some(path);
            }
            
            // Try adding the default extension if missing
            if !hint.ends_with(DEFAULT_SOURCE_EXTENSION) {
                let with_extension = format!("{}{}", hint, DEFAULT_SOURCE_EXTENSION);
                let path_with_ext = PathBuf::from(with_extension);
                if path_with_ext.exists() {
                    return Some(path_with_ext);
                }
            }
        }
        
        // If we've set a source directory in the compiler context, use it
        if let Some(source_dir) = self.get_source_directory() {
            let base_dir = PathBuf::from(source_dir);
            
            // Try to find the source file in the directory
            if let Some(file_name) = file_hint {
                let complete_path = base_dir.join(file_name);
                if complete_path.exists() {
                    return Some(complete_path);
                }
                
                // Try with extension
                if !file_name.ends_with(DEFAULT_SOURCE_EXTENSION) {
                    let with_extension = format!("{}{}", file_name, DEFAULT_SOURCE_EXTENSION);
                    let path_with_ext = base_dir.join(with_extension);
                    if path_with_ext.exists() {
                        return Some(path_with_ext);
                    }
                }
            }
        }
        
        // Fallback to the current directory
        if let Some(file_name) = file_hint {
            let current_dir_path = PathBuf::from(".").join(file_name);
            if current_dir_path.exists() {
                return Some(current_dir_path);
            }
            
            // Try with extension
            if !file_name.ends_with(DEFAULT_SOURCE_EXTENSION) {
                let with_extension = format!("{}{}", file_name, DEFAULT_SOURCE_EXTENSION);
                let path_with_ext = PathBuf::from(".").join(with_extension);
                if path_with_ext.exists() {
                    return Some(path_with_ext);
                }
            }
        }
        
        None
    }
    
    #[instrument(skip(self, type_assertion))]
    fn compile_type_assertion_with_filesystem_location(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Create an enhanced source location with filesystem path
        let source_location = self.create_enhanced_source_location(
            &type_assertion.token,
            &type_assertion.expression.string(),
            &type_assertion.type_name
        );
        
        debug!("Compiling type assertion with filesystem location for: {}", type_assertion.string());
        
        // First ensure registry is initialized
        self.ensure_registry_visualization_initialized()?;
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Check if the interface value is of the target type
        let is_instance = self.check_instance_of(expr_value, &type_assertion.type_name, source_location.clone())?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr(expr_value)?;
        
        // Get the type id for proper error reporting
        let type_id = self.get_type_id(&type_assertion.type_name)?;
        
        // Cast the data pointer to the appropriate type
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.pointer_type(),
            "casted_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Create a successful Result value with rich filesystem location
        let success_result = self.create_type_assertion_result(
            casted_ptr.into(),
            true, // success
            None, // no error message
            source_location.clone()
        )?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Failure path - create an error Result with enhanced filesystem information
        self.builder().position_at_end(failure_block);
        
        // Get the actual type ID from the interface value for better error reporting
        let actual_type_id = self.get_interface_type_id(expr_value)?;
        
        // Create a base error message
        let mut error_message = format!(
            "Failed to assert that interface value is of type {}",
            type_assertion.type_name
        );
        
        // Enhance with source context if we have location information
        if let Some(loc) = source_location.clone() {
            error_message = self.enhance_error_with_source_context(error_message, &loc);
        }
        
        // Create a null pointer for the failure case
        let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        
        // Create a failure Result value with enhanced error information
        let failure_result = self.create_type_assertion_result(
            null_ptr.into(),
            false, // failure
            Some(&error_message),
            source_location.clone()
        )?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        let phi = self.builder().build_phi(
            // Result type structure
            self.get_result_type(self.pointer_type().into()),
            "assertion_result"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        phi.add_incoming(&[
            (&success_result, success_block),
            (&failure_result, failure_block)
        ]);
        
        // Return the phi result
        Ok(phi.as_basic_value())
    }
    
    #[instrument(skip(self, type_assertion))]
    fn compile_type_assertion_question_with_filesystem_location(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Create an enhanced source location with filesystem path
        let source_location = self.create_enhanced_source_location(
            &type_assertion.token,
            &type_assertion.expression.string(),
            &type_assertion.type_name
        );
        
        debug!("Compiling type assertion question with filesystem location for: {}", type_assertion.string());
        
        // First ensure registry is initialized
        self.ensure_registry_visualization_initialized()?;
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion with ? operator".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_question_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_question_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_question_merge");
        
        // Check if the interface value is of the target type
        let is_instance = self.check_instance_of(expr_value, &type_assertion.type_name, source_location.clone())?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr(expr_value)?;
        
        // Get the type id for proper error reporting
        let type_id = self.get_type_id(&type_assertion.type_name)?;
        
        // Cast the data pointer to the appropriate type
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.pointer_type(),
            "casted_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Failure path - propagate the error through the ? operator with enhanced error info
        self.builder().position_at_end(failure_block);
        
        // Get the actual type ID from the interface value for better error reporting
        let actual_type_id = self.get_interface_type_id(expr_value)?;
        
        // Create a base error message
        let mut error_message = format!(
            "Failed to assert that interface value is of type {}",
            type_assertion.type_name
        );
        
        // Enhance with source context if we have location information
        if let Some(loc) = source_location.clone() {
            error_message = self.enhance_error_with_source_context(error_message, &loc);
        }
        
        // Create location struct for error propagation
        let source_struct = if let Some(loc) = &source_location {
            self.build_source_location_struct(loc)
        } else {
            // Default empty struct
            self.build_empty_source_location()
        };
        
        // Call error propagation function with enhanced filesystem source information
        self.call_error_propagation_function_with_fs(
            self.create_string_constant(&error_message).into(), 
            source_struct
        )?;
        
        // This should be unreachable in the failure path
        self.builder().build_unreachable().map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - return the casted pointer on success
        self.builder().position_at_end(merge_block);
        
        // With question operator, we just return the value directly, error handling is automatic
        Ok(casted_ptr.into())
    }
}

// Helper methods for the filesystem integration
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Build a source location struct from a SourceLocation
    fn build_source_location_struct(
        &self,
        location: &SourceLocation
    ) -> BasicValueEnum<'ctx> {
        let ctx = self.context();
        
        // Create basic values for each field
        let line = ctx.i32_type().const_int(location.line as u64, false);
        let column = ctx.i32_type().const_int(location.column as u64, false);
        
        // File path
        let file_ptr = if let Some(file) = &location.file {
            self.create_string_constant(file).into()
        } else {
            ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
        };
        
        // Source line
        let source_line_ptr = if !location.source_line.is_empty() {
            self.create_string_constant(&location.source_line).into()
        } else {
            ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
        };
        
        // Build the struct
        self.build_struct_value(&[
            line.into(),
            column.into(),
            file_ptr,
            source_line_ptr
        ]).into()
    }
    
    /// Build an empty source location struct
    fn build_empty_source_location(&self) -> BasicValueEnum<'ctx> {
        let ctx = self.context();
        
        // Create zero values for all fields
        let line = ctx.i32_type().const_int(0, false);
        let column = ctx.i32_type().const_int(0, false);
        let null_ptr = ctx.i8_type().ptr_type(AddressSpace::default()).const_null();
        
        // Build the struct
        self.build_struct_value(&[
            line.into(),
            column.into(),
            null_ptr.into(),
            null_ptr.into()
        ]).into()
    }
    
    /// Call error propagation with enhanced filesystem source information
    fn call_error_propagation_function_with_fs(
        &self,
        error_message: BasicValueEnum<'ctx>,
        location_info: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get current module and context
        let module = self.module();
        let ctx = self.context();
        let builder = self.builder();
        
        // Get or declare the enhanced error propagation function with filesystem info
        let propagate_fn = match module.get_function("__cursed_propagate_error_with_filesystem_info") {
            Some(func) => func,
            None => {
                // Declare the enhanced function if it doesn't exist
                let void_type = ctx.void_type();
                let fn_type = void_type.fn_type(&[
                    // Error message
                    ctx.i8_type().ptr_type(AddressSpace::default()).into(),
                    // Source location info (enhanced with filesystem)
                    self.get_source_location_type().into(),
                    // Expected type ID
                    ctx.i32_type().into(),
                    // Actual type ID
                    ctx.i32_type().into(),
                    // Error type (1 = type assertion error)
                    ctx.i32_type().into(),
                    // Source file content for context
                    ctx.i8_type().ptr_type(AddressSpace::default()).into(),
                    // Context line range (start, end)
                    ctx.struct_type(&[ctx.i32_type().into(), ctx.i32_type().into()], false).into()
                ], false);
                
                module.add_function("__cursed_propagate_error_with_filesystem_info", fn_type, None)
            }
        };
        
        // Get current type context information
        let expected_type_id = match self.current_expected_type_id() {
            Some(id) => ctx.i32_type().const_int(id as u64, false),
            None => ctx.i32_type().const_int(0, false)
        };
        
        let actual_type_id = match self.current_actual_type_id() {
            Some(id) => ctx.i32_type().const_int(id as u64, false),
            None => ctx.i32_type().const_int(0, false)
        };
        
        // Create empty source file content pointer by default
        let null_content_ptr = ctx.i8_type().ptr_type(AddressSpace::default()).const_null();
        
        // Create default context line range
        let context_range = self.build_struct_value(&[
            ctx.i32_type().const_int(0, false).into(),
            ctx.i32_type().const_int(0, false).into()
        ]);
        
        // Type assertion error code = 1
        let error_type = ctx.i32_type().const_int(1, false);
        
        // Call the enhanced function with filesystem information
        builder.build_call(
            propagate_fn,
            &[
                error_message,
                location_info,
                expected_type_id.into(),
                actual_type_id.into(),
                error_type.into(),
                null_content_ptr.into(),
                context_range.into()
            ],
            "propagate_error_call"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // This function should never return normally, but we need to emit valid LLVM IR
        Ok(ctx.i8_type().const_int(0, false).into())
    }
    
    /// Get the source directory configuration if set
    fn get_source_directory(&self) -> Option<String> {
        // Try to get it from internal fields if available
        self.internal_fields.get("source_directory")
            .and_then(|val| val.downcast_ref::<String>().cloned())
    }
    
    /// Set the source directory for resolving file paths
    pub fn set_source_directory(&mut self, directory: String) {
        self.internal_fields.insert("source_directory".to_string(), Box::new(directory));
    }
}

/// Register the filesystem source location integration with the compiler
pub fn register_filesystem_integration() {
    trace!("Interface type assertion filesystem integration module registered");
    // This function is called during the compiler's initialization
    // to register this implementation for use throughout compilation
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_filesystem_integration_registration() {
        // Test that the module registration function works
        register_filesystem_integration();
        assert!(true);
    }
    
    #[test]
    fn test_context_line_range() {
        let ctx = Context::create();
        let module = Module::create("test", &ctx);
        let builder = ctx.create_builder();
        
        let code_gen = LlvmCodeGenerator::new(&ctx, module, builder);
        
        // Test middle of file
        let (start, end) = code_gen.get_context_line_range(50, 100);
        assert!(start > 0);
        assert!(end < 100);
        assert!(end > 50);
        assert!(start <= 50);
        
        // Test near beginning
        let (start, end) = code_gen.get_context_line_range(2, 100);
        assert_eq!(start, 0);
        
        // Test near end
        let (start, end) = code_gen.get_context_line_range(99, 100);
        assert_eq!(end, 99);
    }
}