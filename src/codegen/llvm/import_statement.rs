//! Import statement compilation for LLVM code generation
//!
//! This module handles the compilation of import statements
//! for the CURSED language to LLVM IR using the package resolver system.

use crate::ast::statements::declarations::ImportStatement;
use std::collections::HashMap;
use inkwell::values::FunctionValue;
use crate::error::Error;
use super::context::{LlvmCodeGenerator, ImportedPackageInfo};
use super::dot_expressions::QualifiedNameCompilation;

/// Trait for import statement compilation
pub trait ImportStatementCompilation<'ctx> {
    /// Compile an import statement and register the imported package
    fn compile_import_statement(&mut self, stmt: &ImportStatement) -> Result<(), Error>;
    
    /// Set the package resolver for import resolution
    fn set_package_resolver(&mut self, resolver: crate::resolver::PackageResolver);
    
    /// Get mutable reference to the package resolver
    fn get_package_resolver_mut(&mut self) -> Option<&mut crate::resolver::PackageResolver>;
}

impl<'ctx> ImportStatementCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self, stmt), fields(path = stmt.path.value), level = "debug")]
    fn compile_import_statement(&mut self, stmt: &ImportStatement) -> Result<(), Error> {
        tracing::info!("Compiling import statement for: {}", stmt.path.value);
        
        let import_path = &stmt.path.value;
        
        // Register the package as imported
        self.register_imported_package(import_path);
        
        tracing::info!("Successfully imported package: {}", import_path);
        
        Ok(())
    }

    fn set_package_resolver(&mut self, resolver: crate::resolver::PackageResolver) {
        self.set_extension(resolver);
    }

    fn get_package_resolver_mut(&mut self) -> Option<&mut crate::resolver::PackageResolver> {
        // Extension system doesn't support mutable access for now
        None
    }
}

// Extension trait for LlvmCodeGenerator to provide package resolver access
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Set a package resolver as an extension
    fn set_package_resolver(&mut self, resolver: crate::resolver::PackageResolver) {
        self.set_extension(resolver);
    }

    /// Get mutable reference to the package resolver
    fn get_package_resolver_mut(&mut self) -> Option<&mut crate::resolver::PackageResolver> {
        // Extension system doesn't support mutable access for now
        None
    }
}
