//! Dot expression compilation for LLVM code generator
//!
//! This module implements compilation of dot expressions (e.g., `module.function()`) 
//! to LLVM IR. It handles both simple property access and method calls on objects.

use crate::ast::expressions::DotExpression;
use crate::codegen::llvm::context::LlvmCodeGenerator;
use crate::error::Error;
use inkwell::values::{BasicValueEnum, PointerValue};
use std::collections::HashMap;

/// Extension trait for compiling dot expressions with the LLVM code generator
pub trait DotExpressionCompilation<'ctx> {
    /// Compile a dot expression (obj.prop)
    fn compile_dot_expression(&mut self, expr: &DotExpression) -> Result<BasicValueEnum<'ctx>, Error>;

    /// Get a reference to an imported package by name
    fn get_imported_package(&self, package_name: &str) -> Option<String>;

    /// Check if a function exists in the current or imported modules
    fn function_exists(&self, package_name: &str, function_name: &str) -> bool;
    
    /// Track imported packages in the symbol table
    fn register_imported_package(&mut self, package_name: &str);
}

impl<'ctx> DotExpressionCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_dot_expression(&mut self, expr: &DotExpression) -> Result<BasicValueEnum<'ctx>, Error> {
        // First, evaluate the object expression
        let object_str = expr.object.string();
        
        println!("DEBUG: Compiling dot expression: {}.{}", object_str, expr.property);
        
        // Check if this is a package access (like vibez.spill)
        if let Some(package_name) = self.get_imported_package(&object_str) {
            println!("DEBUG: Found package: {}", package_name);
            // This is a package access, get the function from the package
            let function_name = &expr.property;
            
            // Construct the mangled function name (for imported packages)
            let mangled_name = format!("{}_{}_{}", self.current_package_name(), package_name, function_name);
            
            // Debug output
            println!("DEBUG: Looking for function '{}' in package '{}' (mangled: {})", 
                function_name, package_name, mangled_name);
            
            // Try to find the function in the module
            if let Some(function) = self.module().get_function(&mangled_name) {
                println!("DEBUG: Found mangled function: {}", mangled_name);
                // Return a function pointer (we'll need to call it separately)
                return Ok(BasicValueEnum::PointerValue(function.as_global_value().as_pointer_value()));
            } else if let Some(function) = self.module().get_function(function_name) {
                println!("DEBUG: Found unmangled function: {}", function_name);
                // Also check for unmangled function in case it's a built-in
                return Ok(BasicValueEnum::PointerValue(function.as_global_value().as_pointer_value()));
            } else {
                // Function not found, try to handle specially recognized ones
                if package_name == "vibez" && function_name == "spill" {
                    println!("DEBUG: Special handling for vibez.spill");
                    // Handle vibez.spill specially - use the built-in puts function
                    if let Some(puts_fn) = self.module().get_function("puts") {
                        println!("DEBUG: Found puts function for vibez.spill");
                        // Return the puts function pointer, which will be called later
                        // We need to override the function call handling in CallExpression to handle this
                        return Ok(BasicValueEnum::PointerValue(puts_fn.as_global_value().as_pointer_value()));
                    } else {
                        println!("DEBUG: Could not find puts function");
                    }
                }
                
                // Function not found, return error
                return Err(Error::from_str(&format!(
                    "Failed to find function '{}' in package '{}'", 
                    function_name, package_name
                )));
            }
        } else {
            // This could be a struct field access or method call
            // For now, we'll just return an error
            Err(Error::from_str(&format!(
                "Dot expression on non-package objects not yet supported: {}.{}", 
                object_str, expr.property
            )))
        }
    }
    
    fn get_imported_package(&self, package_name: &str) -> Option<String> {
        // Check common known packages
        match package_name {
            "vibez" | "htmlrizzler" | "mathz" | "timez" | "reflectz" | 
            "cryptz" | "stringz" | "dropz" | "concurrenz" | "web_vibez" | 
            "rizztemplate" => {
                Some(package_name.to_string())
            },
            _ => None
        }
    }
    
    fn function_exists(&self, package_name: &str, function_name: &str) -> bool {
        // Check if the function exists in the module
        let mangled_name = format!("{}_{}_{}", self.current_package_name(), package_name, function_name);
        self.module().get_function(&mangled_name).is_some() || 
            self.module().get_function(function_name).is_some()
    }
    
    fn register_imported_package(&mut self, package_name: &str) {
        // Register the package in the symbol table
        println!("DEBUG: Registering imported package: {}", package_name);
        // In a real implementation, we would add this to a symbol table
    }
}