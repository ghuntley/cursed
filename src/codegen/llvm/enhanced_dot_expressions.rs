//! Enhanced dot expression compilation with proper symbol resolution
//!
//! This module provides enhanced dot expression compilation that integrates
//! with the comprehensive symbol resolution system.

use crate::ast::expressions::DotExpression;
use crate::codegen::llvm::context::LlvmCodeGenerator;
use crate::error::Error;
// use crate::resolver::{Resolver, SymbolType, ResolvedSymbol}; // Commented out - using new resolver
use inkwell::values::{BasicValueEnum, PointerValue};

/// Enhanced trait for compiling dot expressions with proper symbol resolution
/*
pub trait EnhancedDotExpressionCompilation<'ctx> {
    /// Compile a dot expression using the symbol resolver
    fn compile_dot_expression_enhanced(&mut self, expr: &DotExpression, resolver: &Resolver) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Resolve and compile a qualified symbol reference
    fn resolve_qualified_symbol(&mut self, qualified_name: &str, resolver: &Resolver) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check symbol accessibility and visibility
    fn check_symbol_access(&self, symbol: &ResolvedSymbol, current_package: &str) -> Result<(), Error>;
    
    /// Get the current package context
    fn current_package_context(&self) -> String;
}

impl<'ctx> EnhancedDotExpressionCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_dot_expression_enhanced(&mut self, expr: &DotExpression, resolver: &Resolver) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the current package context
        let current_package = self.current_package_context();
        
        // Build the qualified name from the dot expression
        let object_str = expr.object.string();
        let qualified_name = format!("{}.{}", object_str, expr.property);
        
        tracing::debug!(
            qualified_name = %qualified_name,
            current_package = %current_package,
            "Compiling enhanced dot expression"
        );
        
        // Attempt to resolve as a qualified symbol
        match resolver.resolve_qualified(&current_package, &qualified_name) {
            Ok(resolved_symbol) => {
                // Check access permissions
                self.check_symbol_access(&resolved_symbol, &current_package)?;
                
                // Compile based on symbol type
                self.compile_resolved_symbol(&resolved_symbol)
            }
            Err(_) => {
                // Fallback to legacy dot expression handling or struct field access
                self.compile_legacy_dot_expression(expr, resolver)
            }
        }
    }
    
    fn resolve_qualified_symbol(&mut self, qualified_name: &str, resolver: &Resolver) -> Result<BasicValueEnum<'ctx>, Error> {
        let current_package = self.current_package_context();
        
        // Resolve the qualified symbol
        let resolved = resolver.resolve_qualified(&current_package, qualified_name)?;
        
        // Check access permissions
        self.check_symbol_access(&resolved, &current_package)?;
        
        // Compile the resolved symbol
        self.compile_resolved_symbol(&resolved)
    }
    
    fn check_symbol_access(&self, symbol: &ResolvedSymbol, current_package: &str) -> Result<(), Error> {
        // If accessing symbol from same package, always allowed
        if symbol.package == current_package {
            return Ok(());
        }
        
        // Check if symbol is public (exported)
        if symbol.visibility == crate::resolver::SymbolVisibility::Private {
            return Err(Error::from_str(&format!(
                "Cannot access private symbol '{}' from package '{}' in package '{}'",
                symbol.name, symbol.package, current_package
            )));
        }
        
        // Additional checks could be added here (e.g., import validation)
        Ok(())
    }
    
    fn current_package_context(&self) -> String {
        // This should return the current package being compiled
        // For now, return a default or extract from the code generator context
        self.current_package_name()
    }
}

/// Additional methods for the enhanced dot expression compilation
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile a resolved symbol to LLVM IR
    fn compile_resolved_symbol(&mut self, symbol: &ResolvedSymbol) -> Result<BasicValueEnum<'ctx>, Error> {
        match symbol.symbol_type {
            SymbolType::Function => self.compile_function_reference(symbol),
            SymbolType::Variable => self.compile_variable_reference(symbol),
            SymbolType::Constant => self.compile_constant_reference(symbol),
            SymbolType::Type => self.compile_type_reference(symbol),
            SymbolType::Interface => self.compile_interface_reference(symbol),
            SymbolType::Package => Err(Error::from_str(&format!(
                "Cannot directly reference package '{}' as a value",
                symbol.name
            ))),
        }
    }
    
    /// Compile a function reference
    fn compile_function_reference(&mut self, symbol: &ResolvedSymbol) -> Result<BasicValueEnum<'ctx>, Error> {
        // Generate the mangled function name
        let mangled_name = self.mangle_function_name(&symbol.package, &symbol.name);
        
        tracing::debug!(
            symbol = %symbol.name,
            package = %symbol.package,
            mangled_name = %mangled_name,
            "Compiling function reference"
        );
        
        // Try to find the function in the LLVM module
        if let Some(function) = self.module().get_function(&mangled_name) {
            tracing::debug!(mangled_name = %mangled_name, "Found mangled function");
            return Ok(BasicValueEnum::PointerValue(function.as_global_value().as_pointer_value()));
        }
        
        // Try unmangled name for built-ins
        if let Some(function) = self.module().get_function(&symbol.name) {
            tracing::debug!(symbol = %symbol.name, "Found unmangled function");
            return Ok(BasicValueEnum::PointerValue(function.as_global_value().as_pointer_value()));
        }
        
        // Special handling for standard library functions
        if let Some(stdlib_function) = self.get_stdlib_function(&symbol.package, &symbol.name) {
            return Ok(BasicValueEnum::PointerValue(stdlib_function.as_global_value().as_pointer_value()));
        }
        
        Err(Error::from_str(&format!(
            "Function '{}' from package '{}' not found in LLVM module",
            symbol.name, symbol.package
        )))
    }
    
    /// Compile a variable reference
    fn compile_variable_reference(&mut self, symbol: &ResolvedSymbol) -> Result<BasicValueEnum<'ctx>, Error> {
        let mangled_name = self.mangle_variable_name(&symbol.package, &symbol.name);
        
        // Try to find the global variable
        if let Some(global) = self.module().get_global(&mangled_name) {
            // Load the value from the global variable
            let builder = self.builder();
            let loaded_value = builder.build_load(
                global.as_pointer_value(),
                &format!("load_{}", symbol.name),
            );
            return loaded_value.map_err(|_| Error::from_str("Failed to load global variable"));
        }
        
        Err(Error::from_str(&format!(
            "Variable '{}' from package '{}' not found",
            symbol.name, symbol.package
        )))
    }
    
    /// Compile a constant reference
    fn compile_constant_reference(&mut self, symbol: &ResolvedSymbol) -> Result<BasicValueEnum<'ctx>, Error> {
        // Constants might be compiled as global values or inlined
        let mangled_name = self.mangle_constant_name(&symbol.package, &symbol.name);
        
        if let Some(global) = self.module().get_global(&mangled_name) {
            let builder = self.builder();
            let loaded_value = builder.build_load(
                global.as_pointer_value(),
                &format!("load_const_{}", symbol.name),
            );
            return loaded_value.map_err(|_| Error::from_str("Failed to load constant"));
        }
        
        Err(Error::from_str(&format!(
            "Constant '{}' from package '{}' not found",
            symbol.name, symbol.package
        )))
    }
    
    /// Compile a type reference
    fn compile_type_reference(&mut self, symbol: &ResolvedSymbol) -> Result<BasicValueEnum<'ctx>, Error> {
        // Type references might be used for type assertions or constructors
        // For now, return an error as types aren't values in the traditional sense
        Err(Error::from_str(&format!(
            "Type '{}' cannot be used as a value expression",
            symbol.name
        )))
    }
    
    /// Compile an interface reference
    fn compile_interface_reference(&mut self, symbol: &ResolvedSymbol) -> Result<BasicValueEnum<'ctx>, Error> {
        // Interface references might be used for type assertions
        Err(Error::from_str(&format!(
            "Interface '{}' cannot be used as a value expression",
            symbol.name
        )))
    }
    
    /// Legacy dot expression compilation for backward compatibility
    fn compile_legacy_dot_expression(&mut self, expr: &DotExpression, resolver: &Resolver) -> Result<BasicValueEnum<'ctx>, Error> {
        // This would call the original dot expression compilation logic
        // For now, delegate to the existing implementation
        
        let object_str = expr.object.string();
        
        // Check if this is a known package in the resolver
        let current_package = self.current_package_context();
        
        // Try to resolve as a simple identifier first
        if let Ok(resolved) = resolver.resolve_identifier(&current_package, &object_str) {
            // This might be a struct field access or method call
            return self.compile_struct_field_access(&resolved, &expr.property);
        }
        
        // Fallback to standard library or builtin handling
        self.compile_stdlib_dot_expression(expr)
    }
    
    /// Compile struct field access
    fn compile_struct_field_access(&mut self, struct_symbol: &ResolvedSymbol, field_name: &str) -> Result<BasicValueEnum<'ctx>, Error> {
        // This would implement struct field access
        // For now, return an error
        Err(Error::from_str(&format!(
            "Struct field access not yet implemented: {}.{}",
            struct_symbol.name, field_name
        )))
    }
    
    /// Compile standard library dot expressions
    fn compile_stdlib_dot_expression(&mut self, expr: &DotExpression) -> Result<BasicValueEnum<'ctx>, Error> {
        // Delegate to existing stdlib handling
        // This could call the existing dot_registry system
        let object_str = expr.object.string();
        
        // Check for standard library packages
        if let Some(stdlib_function) = self.get_stdlib_function(&object_str, &expr.property) {
            return Ok(BasicValueEnum::PointerValue(stdlib_function.as_global_value().as_pointer_value()));
        }
        
        Err(Error::from_str(&format!(
            "Unresolved dot expression: {}.{}",
            object_str, expr.property
        )))
    }
    
    /// Helper method to get standard library functions
    fn get_stdlib_function(&self, package: &str, function: &str) -> Option<inkwell::values::FunctionValue<'ctx>> {
        // Check for special standard library functions
        match (package, function) {
            ("vibez", "spill") => self.module().get_function("puts"),
            ("core", "len") => self.module().get_function("strlen"),
            // Add more mappings as needed
            _ => None,
        }
    }
    
    /// Helper methods for name mangling
    fn mangle_function_name(&self, package: &str, function: &str) -> String {
        format!("{}_{}_{}", self.current_package_name(), package, function)
    }
    
    fn mangle_variable_name(&self, package: &str, variable: &str) -> String {
        format!("{}_{}_var_{}", self.current_package_name(), package, variable)
    }
    
    fn mangle_constant_name(&self, package: &str, constant: &str) -> String {
        format!("{}_{}_const_{}", self.current_package_name(), package, constant)
    }
}
*/
