//! Qualified name and dot expression compilation for LLVM code generator
//!
//! This module implements compilation of qualified names and dot expressions to LLVM IR.
//! It supports accessing all symbol types from imported packages including:
//! - Functions: `math.sqrt(25)`
//! - Types: `http.Request{}`
//! - Constants: `math.Pi`
//! - Variables: `config.Debug`
//! - Method calls: `obj.method()`
//! - Chained access: `package.Type.method()`

use crate::ast::expressions::{DotExpression};
use crate::ast::expressions::qualified_name::{QualifiedName, QualifiedSymbolKind};
use crate::codegen::llvm::context::LlvmCodeGenerator;
use crate::resolver::symbol_table::{Symbol, SymbolKind};
use crate::error::Error;
use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::types::BasicTypeEnum;
use std::collections::HashMap;

/// Extension trait for compiling qualified names and dot expressions with the LLVM code generator
pub trait QualifiedNameCompilation<'ctx> {
    /// Compile a qualified name (package.symbol)
    fn compile_qualified_name(&mut self, qualified: &QualifiedName) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a dot expression (obj.prop or package.symbol)
    fn compile_dot_expression(&mut self, expr: &DotExpression) -> Result<BasicValueEnum<'ctx>, Error>;

    /// Resolve a qualified symbol in the global symbol table
    fn resolve_qualified_symbol(&self, package_name: &str, symbol_name: &str) -> Option<Symbol>;

    /// Get a reference to an imported package by name
    fn get_imported_package(&self, package_name: &str) -> Option<String>;

    /// Check if a symbol exists in the current or imported modules
    fn symbol_exists(&self, package_name: &str, symbol_name: &str) -> bool;
    
    /// Track imported packages in the symbol table
    fn register_imported_package(&mut self, package_name: &str);
    
    /// Compile access to a qualified function
    fn compile_qualified_function(&mut self, package: &str, function: &str) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile access to a qualified type
    fn compile_qualified_type(&mut self, package: &str, type_name: &str) -> Result<BasicTypeEnum<'ctx>, Error>;
    
    /// Compile access to a qualified constant
    fn compile_qualified_constant(&mut self, package: &str, constant: &str) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile access to a qualified variable
    fn compile_qualified_variable(&mut self, package: &str, variable: &str) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> QualifiedNameCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self), level = "debug")]
    fn compile_qualified_name(&mut self, qualified: &QualifiedName) -> Result<BasicValueEnum<'ctx>, Error> {
        let package = qualified.effective_package_name();
        let symbol = &qualified.symbol;
        
        tracing::debug!(
            package = package,
            symbol = symbol,
            kind = ?qualified.symbol_kind,
            "Compiling qualified name"
        );
        
        // For now, skip symbol table resolution and delegate to stdlib handlers
        // TODO: Implement proper qualified symbol resolution with the new package resolver
        
        {
            // Fall back to the existing dot expression logic for backward compatibility
            match qualified.symbol_kind {
                QualifiedSymbolKind::Function => {
                    self.compile_qualified_function(package, symbol)
                },
                QualifiedSymbolKind::Constant => {
                    self.compile_qualified_constant(package, symbol)
                },
                QualifiedSymbolKind::Variable => {
                    self.compile_qualified_variable(package, symbol)
                },
                QualifiedSymbolKind::Type => {
                    return Err(Error::from_str(&format!(
                        "Type '{}' from package '{}' cannot be used as a value",
                        symbol, package
                    )));
                },
                QualifiedSymbolKind::Unknown => {
                    // Try function first (most common case), then constant, then variable
                    if let Ok(result) = self.compile_qualified_function(package, symbol) {
                        return Ok(result);
                    }
                    if let Ok(result) = self.compile_qualified_constant(package, symbol) {
                        return Ok(result);
                    }
                    if let Ok(result) = self.compile_qualified_variable(package, symbol) {
                        return Ok(result);
                    }
                    
                    return Err(Error::from_str(&format!(
                        "Unknown symbol '{}' in package '{}'", symbol, package
                    )));
                }
            }
        }
    }
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
    
    fn resolve_qualified_symbol(&self, package_name: &str, symbol_name: &str) -> Option<Symbol> {
        // TODO: Integrate with global symbol table when available
        // For now, return None to fall back to existing logic
        None
    }

    fn symbol_exists(&self, package_name: &str, symbol_name: &str) -> bool {
        // Check if the symbol exists in the module (for functions)
        let mangled_name = format!("{}_{}_{}", self.current_package_name(), package_name, symbol_name);
        self.module().get_function(&mangled_name).is_some() || 
            self.module().get_function(symbol_name).is_some()
    }
    
    fn register_imported_package(&mut self, package_name: &str) {
        // Register the package in the symbol table
        tracing::debug!(package = package_name, "Registering imported package");
        // TODO: Integrate with global symbol table
    }
    
    #[tracing::instrument(skip(self), level = "debug")]
    fn compile_qualified_function(&mut self, package: &str, function: &str) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!(package = package, function = function, "Compiling qualified function");
        
        // Construct the mangled function name (for imported packages)
        let mangled_name = format!("{}_{}_{}", self.current_package_name(), package, function);
        
        // Try to find the function in the module
        if let Some(function_value) = self.module().get_function(&mangled_name) {
            tracing::debug!(mangled_name = mangled_name, "Found mangled function");
            return Ok(BasicValueEnum::PointerValue(function_value.as_global_value().as_pointer_value()));
        } else if let Some(function_value) = self.module().get_function(function) {
            tracing::debug!(function = function, "Found unmangled function");
            return Ok(BasicValueEnum::PointerValue(function_value.as_global_value().as_pointer_value()));
        } else {
            // Handle specially recognized functions
            if package == "vibez" && function == "spill" {
                tracing::debug!("Special handling for vibez.spill");
                if let Some(puts_fn) = self.module().get_function("puts") {
                    tracing::debug!("Found puts function for vibez.spill");
                    return Ok(BasicValueEnum::PointerValue(puts_fn.as_global_value().as_pointer_value()));
                }
            }
            
            return Err(Error::from_str(&format!(
                "Function '{}' not found in package '{}'", function, package
            )));
        }
    }
    
    #[tracing::instrument(skip(self), level = "debug")]
    fn compile_qualified_type(&mut self, package: &str, type_name: &str) -> Result<BasicTypeEnum<'ctx>, Error> {
        tracing::debug!(package = package, type_name = type_name, "Compiling qualified type");
        
        // Look up the struct type in the imported package
        if let Some(struct_type) = self.get_struct_type(package, type_name) {
            return Ok(BasicTypeEnum::StructType(struct_type));
        }
        
        // TODO: Handle other types (primitives, interfaces, etc.)
        
        Err(Error::from_str(&format!(
            "Type '{}' not found in package '{}'", type_name, package
        )))
    }
    
    #[tracing::instrument(skip(self), level = "debug")]
    fn compile_qualified_constant(&mut self, package: &str, constant: &str) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!(package = package, constant = constant, "Compiling qualified constant");
        
        // Look for a global constant with the mangled name
        let mangled_name = format!("{}_{}_{}", self.current_package_name(), package, constant);
        
        if let Some(global) = self.module().get_global(&mangled_name) {
            tracing::debug!(mangled_name = mangled_name, "Found mangled constant");
            return Ok(BasicValueEnum::PointerValue(global.as_pointer_value()));
        }
        
        // Try unmangled name
        if let Some(global) = self.module().get_global(constant) {
            tracing::debug!(constant = constant, "Found unmangled constant");
            return Ok(BasicValueEnum::PointerValue(global.as_pointer_value()));
        }
        
        // Handle special constants
        match (package, constant) {
            ("math", "Pi") => {
                let pi_value = self.context.f64_type().const_float(std::f64::consts::PI);
                Ok(BasicValueEnum::FloatValue(pi_value))
            },
            ("math", "E") => {
                let e_value = self.context.f64_type().const_float(std::f64::consts::E);
                Ok(BasicValueEnum::FloatValue(e_value))
            },
            _ => Err(Error::from_str(&format!(
                "Constant '{}' not found in package '{}'", constant, package
            )))
        }
    }
    
    #[tracing::instrument(skip(self), level = "debug")]
    fn compile_qualified_variable(&mut self, package: &str, variable: &str) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!(package = package, variable = variable, "Compiling qualified variable");
        
        // Look for a global variable with the mangled name
        let mangled_name = format!("{}_{}_{}", self.current_package_name(), package, variable);
        
        if let Some(global) = self.module().get_global(&mangled_name) {
            tracing::debug!(mangled_name = mangled_name, "Found mangled variable");
            // Load the variable's value
            let loaded_value = self.builder().build_load(
                global.as_pointer_value().get_type(),
                global.as_pointer_value(),
                &format!("load_{}", variable)
            ).map_err(|e| Error::from_str(&format!("Failed to load variable: {}", e)))?;
            return Ok(loaded_value);
        }
        
        // Try unmangled name
        if let Some(global) = self.module().get_global(variable) {
            tracing::debug!(variable = variable, "Found unmangled variable");
            let loaded_value = self.builder().build_load(
                global.as_pointer_value().get_type(),
                global.as_pointer_value(),
                &format!("load_{}", variable)
            ).map_err(|e| Error::from_str(&format!("Failed to load variable: {}", e)))?;
            return Ok(loaded_value);
        }
        
        Err(Error::from_str(&format!(
            "Variable '{}' not found in package '{}'", variable, package
        )))
    }
}