//! Import statement compilation for LLVM code generation
//!
//! This module handles the compilation of import statements
//! for the CURSED language to LLVM IR.

use crate::ast::statements::declarations::ImportStatement;
use std::path::PathBuf;
use std::collections::HashMap;
use inkwell::values::FunctionValue;
use crate::error::Error;
use super::context::{LlvmCodeGenerator, ImportedPackageInfo};

/// Trait for import statement compilation
pub trait ImportStatementCompilation<'ctx> {
    /// Compile an import statement
    fn compile_import_statement(&mut self, stmt: &ImportStatement) -> Result<(), Error>;
}

impl<'ctx> ImportStatementCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self, stmt), fields(path = stmt.path.value), level = "debug")]
    fn compile_import_statement(&mut self, stmt: &ImportStatement) -> Result<(), Error> {
        tracing::debug!("Compiling import statement");
        
        let package_path = &stmt.path.value;
        
        // Extract package name from path (last segment)
        let package_name = match package_path.rfind('/') {
            Some(idx) => &package_path[idx + 1..],
            None => package_path,
        };
        
        // Register the imported package
        let package_info = ImportedPackageInfo {
            path: PathBuf::from(package_path),
            functions: HashMap::new(),
            struct_types: HashMap::new(),
        };
        self.register_imported_package(package_name, package_info);
        
        // For actual implementation, we would need to:
        // 1. Resolve the package path
        // 2. Parse and compile the imported package if not already done
        // 3. Link the compiled package with the current module
        // 4. Make package functions and types available in the current scope
        
        // For now, we'll just register the package name
        // Actual module loading would happen separately
        tracing::info!("Imported package: {}", package_name);
        
        Ok(())
    }
}

// Extension methods for LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Register an imported package
    pub fn register_imported_package(&mut self, package_name: &str, info: ImportedPackageInfo<'ctx>) {
        self.imported_packages.insert(package_name.to_string(), info);
    }
    
    /// Check if a package is imported
    pub fn is_package_imported(&self, package_name: &str) -> bool {
        self.imported_packages.contains_key(package_name)
    }
    
    /// Get an imported package by alias (for "import X as Y" syntax)
    pub fn get_imported_package(&self, alias: &str) -> Option<String> {
        // In a real implementation, we would maintain a map of aliases to package names
        // For now, we'll assume the alias is the same as the package name
        if self.is_package_imported(alias) {
            Some(alias.to_string())
        } else {
            None
        }
    }
}