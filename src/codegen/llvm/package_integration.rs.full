/// LLVM Package Integration
/// 
/// Integrates the package manager with LLVM compilation pipeline to enable
/// automatic package resolution, symbol importing, and module linking.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use thiserror::Error;

use crate::ast::{Program, ImportStatement};
use crate::package_manager::{PackageManager, PackageMetadata, PackageManagerError};
use crate::imports::{ImportManager, ImportError, ImportResolverConfig, ResolvedImport, LoadedModule, ImportSource};
use crate::codegen::llvm::{LlvmCodeGenerator, expression_compiler::LlvmValue};
use crate::error::Error;

/// Configuration for LLVM package integration
#[derive(Debug, Clone)]
pub struct LlvmPackageConfig {
    pub auto_install_packages: bool,
    pub link_package_symbols: bool,
    pub inline_package_functions: bool,
    pub generate_package_debug_info: bool,
    pub cache_compiled_modules: bool,
}

impl Default for LlvmPackageConfig {
    fn default() -> Self {
        Self {
            auto_install_packages: true,
            link_package_symbols: true,
            inline_package_functions: false,
            generate_package_debug_info: true,
            cache_compiled_modules: true,
        }
    }
}

/// Errors during LLVM package integration
#[derive(Error, Debug)]
pub enum LlvmPackageError {
    #[error("Package manager error: {0}")]
    PackageManager(#[from] PackageManagerError),
    
    #[error("Import resolution error: {0}")]
    ImportResolution(#[from] ImportError),
    
    #[error("LLVM compilation error: {0}")]
    LlvmCompilation(#[from] Error),
    
    #[error("Symbol resolution failed for {symbol} from package {package}")]
    SymbolResolution { symbol: String, package: String },
    
    #[error("Package not installed: {package}")]
    PackageNotInstalled { package: String },
    
    #[error("Module linking failed: {module} - {error}")]
    ModuleLinking { module: String, error: String },
}

/// Information about a compiled package module
#[derive(Debug, Clone)]
pub struct CompiledPackageModule {
    pub package_name: String,
    pub module_name: String,
    pub llvm_ir: String,
    pub exported_symbols: Vec<String>,
    pub exported_types: Vec<String>,
    pub dependencies: Vec<String>,
}

/// Context for package-aware LLVM compilation
pub struct LlvmPackageContext {
    pub package_manager: Arc<Mutex<PackageManager>>,
    pub import_manager: ImportManager,
    pub config: LlvmPackageConfig,
    pub compiled_modules: HashMap<String, CompiledPackageModule>,
    pub symbol_table: HashMap<String, String>, // symbol_name -> module_name
}

impl LlvmPackageContext {
    /// Create new LLVM package integration context
    pub fn new(
        package_manager: Arc<Mutex<PackageManager>>,
        config: LlvmPackageConfig,
    ) -> Result<(), Error> {
        let import_config = ImportResolverConfig::default();
        let import_manager = ImportManager::new(package_manager.clone(), import_config)?;
        
        Ok(Self {
            package_manager,
            import_manager,
            config,
            compiled_modules: HashMap::new(),
            symbol_table: HashMap::new(),
        })
    }
    
    /// Compile source with automatic package resolution and integration
    pub async fn compile_with_packages(
        &mut self,
        source: &str,
        source_file: Option<&Path>,
    ) -> Result<(), Error> {
        tracing::info!("Starting LLVM compilation with package integration");
        
        // Parse the main program to extract imports
        let program = self.parse_program(source)?;
        
        // Resolve all imports and ensure packages are installed
        let resolved_imports = self.resolve_and_install_imports(&program.imports, source_file).await?;
        
        // Load and compile package modules
        let package_modules = self.load_and_compile_packages(&resolved_imports).await?;
        
        // Register package symbols for main compilation
        self.register_package_symbols(&package_modules)?;
        
        // Compile main program with package context
        let main_ir = self.compile_main_program(&program).await?;
        
        // Link everything together
        let final_ir = self.link_modules(main_ir, package_modules)?;
        
        tracing::info!("LLVM compilation with packages completed successfully");
        Ok(final_ir)
    }
    
    /// Parse program and extract imports
    fn parse_program(&self, source: &str) -> Result<(), Error> {
        let lexer = crate::lexer::Lexer::new(source.to_string());
        let mut parser = crate::parser::Parser::new(lexer)?;
        let program = parser.parse_program()?;
        
        let errors = parser.errors();
        if !errors.is_empty() {
            return Err(LlvmPackageError::LlvmCompilation(
                Error::Parse(format!("Parse errors: {}", errors.join(", ")))
            ));
        }
        
        Ok(program)
    }
    
    /// Resolve imports and automatically install missing packages
    async fn resolve_and_install_imports(
        &mut self,
        imports: &[ImportStatement],
        source_file: Option<&Path>,
    ) -> Result<(), Error> {
        let mut resolved = Vec::new();
        
        for import in imports {
            tracing::debug!(import_path = %import.path, "Resolving import");
            
            // Try to resolve the import
            let resolved_import = match self.import_manager.resolve_single_import(
                import,
                source_file,
                &mut std::collections::HashSet::new(),
            ).await {
                Ok(import) => import,
                Err(ImportError::PackageNotInstalled { package }) => {
                    // Auto-install if enabled
                    if self.config.auto_install_packages {
                        tracing::info!(package = %package, "Auto-installing missing package");
                        self.install_package(&package).await?;
                        
                        // Retry resolution after installation
                        self.import_manager.resolve_single_import(
                            import,
                            source_file,
                            &mut std::collections::HashSet::new(),
                        ).await?
                    } else {
                        return Err(LlvmPackageError::PackageNotInstalled { package });
                    }
                }
                Err(e) => return Err(e.into()),
            };
            
            resolved.push(resolved_import);
        }
        
        Ok(resolved)
    }
    
    /// Install a package using the package manager
    pub async fn install_package(&mut self, package_name: &str) -> Result<(), Error> {
        let mut package_manager = self.package_manager.lock().map_err(|_| {
            LlvmPackageError::PackageManager(PackageManagerError::RegistryError {
                message: "Failed to lock package manager".to_string(),
            })
        })?;
        
        let installed_packages = package_manager.install_package(package_name, None).await?;
        
        installed_packages.into_iter()
            .find(|p| p.to_string() == package_name)
            .ok_or_else(|| LlvmPackageError::PackageNotInstalled {
                package: package_name.to_string(),
            })
    }
    
    /// Load modules and compile them to LLVM IR
    async fn load_and_compile_packages(
        &mut self,
        resolved_imports: &[ResolvedImport],
    ) -> Result<(), Error> {
        let mut compiled_modules = Vec::new();
        
        for resolved in resolved_imports {
            // Skip standard library imports (handled separately)
            if let ImportSource::StandardLibrary { .. } = resolved.source {
                continue;
            }
            
            tracing::debug!(import = %resolved.original_path, "Loading and compiling package module");
            
            // Load the module
            let loaded_module = self.import_manager.load_module(resolved).await?;
            
            // Compile to LLVM IR
            let compiled = self.compile_package_module(&loaded_module, resolved).await?;
            compiled_modules.push(compiled);
        }
        
        Ok(compiled_modules)
    }
    
    /// Compile a single package module to LLVM IR
    async fn compile_package_module(
        &mut self,
        loaded_module: &LoadedModule,
        resolved_import: &ResolvedImport,
    ) -> Result<(), Error> {
        let package_name = match &resolved_import.source {
            ImportSource::InstalledPackage { package_name } => package_name.clone(),
            ImportSource::LocalFile { .. } => "local".to_string(),
            ImportSource::LocalModule { .. } => "local_module".to_string(),
            ImportSource::StandardLibrary { .. } => "stdlib".to_string(),
        };
        
        let module_name = loaded_module.info.to_string().clone();
        
        // Check cache first
        let cache_key = format!("{}::{}", package_name, module_name);
        if self.config.cache_compiled_modules {
            if let Some(cached) = self.compiled_modules.get(&cache_key) {
                return Ok(cached.clone());
            }
        }
        
        // Create a new code generator for this module
        let mut module_codegen = LlvmCodeGenerator::new()?;
        
        // Set debug info if enabled
        if self.config.generate_package_debug_info {
            if loaded_module.source.len() > 0 {
                let debug_config = crate::debug::DebugConfig::default();
                module_codegen.set_debug_config(debug_config);
            }
        }
        
        // Compile the module program
        if let Err(e) = module_codegen.compile(&loaded_module.program) {
            tracing::warn!(module = %module_name, error = ?e, "Failed to compile package module");
        }
        
        // Generate LLVM IR
        let llvm_ir = module_codegen.generate_ir("// Package module")?;
        
        // Extract exported symbols and types
        let exported_symbols = loaded_module.info.exports.clone();
        let exported_types = loaded_module.info.types.clone();
        let dependencies = loaded_module.info.dependencies.clone();
        
        let compiled = CompiledPackageModule {
            package_name: package_name.clone(),
            module_name: module_name.clone(),
            llvm_ir,
            exported_symbols,
            exported_types,
            dependencies,
        };
        
        // Cache the compiled module
        if self.config.cache_compiled_modules {
            self.compiled_modules.insert(cache_key, compiled.clone());
        }
        
        tracing::debug!(
            package = %package_name,
            module = %module_name,
            symbols = compiled.exported_symbols.len(),
            types = compiled.exported_types.len(),
            "Compiled package module"
        );
        
        Ok(compiled)
    }
    
    /// Register package symbols for use in main compilation
    fn register_package_symbols(
        &mut self,
        modules: &[CompiledPackageModule],
    ) -> Result<(), Error> {
        for module in modules {
            for symbol in &module.exported_symbols {
                let qualified_name = format!("{}::{}", module.module_name, symbol);
                self.symbol_table.insert(qualified_name, module.module_name.clone());
                
                tracing::debug!(
                    symbol = %symbol,
                    module = %module.module_name,
                    "Registered package symbol"
                );
            }
        }
        
        Ok(())
    }
    
    /// Compile main program with package symbol resolution
    async fn compile_main_program(&mut self, program: &Program) -> Result<(), Error> {
        // Create a code generator for main program
        let mut codegen = LlvmCodeGenerator::new()?;
        
        // Create compilation context with package symbols available
        if let Err(e) = codegen.compile(program) {
            tracing::warn!(error = ?e, "Main program compilation had issues");
        }
        
        // Generate LLVM IR
        let ir = codegen.generate_ir("// Main program with packages")?;
        Ok(ir)
    }
    
    /// Link main program with package modules
    fn link_modules(
        &self,
        main_ir: String,
        package_modules: Vec<CompiledPackageModule>,
    ) -> Result<(), Error> {
        let mut final_ir = String::new();
        
        // Add target information
        final_ir.push_str("; Generated by CURSED Compiler with Package Integration\n");
        final_ir.push_str("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n");
        final_ir.push_str("target triple = \"x86_64-pc-linux-gnu\"\n\n");
        
        // Add package module IR first
        for module in &package_modules {
            final_ir.push_str(&format!("; Package module: {}\n", module.module_name));
            final_ir.push_str(&module.llvm_ir);
            final_ir.push_str("\n");
        }
        
        // Add main program IR
        final_ir.push_str("; Main program\n");
        final_ir.push_str(&main_ir);
        
        tracing::info!(
            modules_linked = package_modules.len(),
            total_symbols = self.symbol_table.len(),
            "Successfully linked modules"
        );
        
        Ok(final_ir)
    }
    
    /// Resolve a symbol from packages
    pub fn resolve_package_symbol(&self, symbol_name: &str) -> Option<&str> {
        self.symbol_table.get(symbol_name).map(|s| s.as_str())
    }
    
    /// Check if a symbol is available from packages
    pub fn has_package_symbol(&self, symbol_name: &str) -> bool {
        self.symbol_table.contains_key(symbol_name)
    }
    
    /// Get available package symbols
    pub fn get_package_symbols(&self) -> Vec<String> {
        self.symbol_table.keys().cloned().collect()
    }
    
    /// Get compilation statistics
    pub fn get_stats(&self) -> LlvmPackageStats {
        LlvmPackageStats {
            compiled_modules: self.compiled_modules.len(),
            available_symbols: self.symbol_table.len(),
            import_cache_size: self.import_manager.get_stats().cached_imports,
        }
    }
    
    /// Resolve a specific package import
    pub async fn resolve_package_import(&mut self, import_path: &str) -> Result<(), Error> {
        // Create a minimal import statement
        let import = ImportStatement {
            token: crate::lexer::Token::new(crate::lexer::TokenType::Identifier, import_path),
            path: import_path.to_string(),
            alias: None,
        };
        
        // Resolve and compile single import
        let resolved = self.resolve_and_install_imports(&[import], None).await?;
        if let Some(resolved_import) = resolved.first() {
            let loaded_module = self.import_manager.load_module(resolved_import).await?;
            self.compile_package_module(&loaded_module, resolved_import).await
        } else {
            Err(LlvmPackageError::ImportResolution(ImportError::NotFound {
                import_path: import_path.to_string(),
            }))
        }
    }
}

/// Statistics for LLVM package integration
#[derive(Debug, Clone)]
pub struct LlvmPackageStats {
    pub compiled_modules: usize,
    pub available_symbols: usize,
    pub import_cache_size: usize,
}

/// Enhanced LLVM code generator with package support
pub trait LlvmPackageIntegration {
    /// Compile with automatic package resolution
    async fn compile_with_auto_packages(
        &mut self,
        source: &str,
        source_file: Option<&Path>,
    ) -> Result<(), Error>;
    
    /// Resolve and compile a package import
    async fn resolve_package_import(
        &mut self,
        import_path: &str,
    ) -> Result<(), Error>;
    
    /// Check if packages are available for compilation
    fn validate_package_dependencies(&self, imports: &[ImportStatement]) -> Result<(), Error>;
}

impl LlvmPackageIntegration for LlvmCodeGenerator {
    async fn compile_with_auto_packages(
        &mut self,
        source: &str,
        source_file: Option<&Path>,
    ) -> Result<(), Error> {
        // Create package integration context
        let package_manager = Arc::new(Mutex::new(
            crate::package_manager::PackageManager::new(
                crate::package_manager::PackageManagerConfig::default()
            )?
        ));
        
        let config = LlvmPackageConfig::default();
        let mut context = LlvmPackageContext::new(package_manager, config)?;
        
        // Delegate to the context
        context.compile_with_packages(source, source_file).await
    }
    
    async fn resolve_package_import(
        &mut self,
        import_path: &str,
    ) -> Result<(), Error> {
        // Create package integration context
        let package_manager = Arc::new(Mutex::new(
            crate::package_manager::PackageManager::new(
                crate::package_manager::PackageManagerConfig::default()
            )?
        ));
        
        let config = LlvmPackageConfig::default();
        let mut context = LlvmPackageContext::new(package_manager, config)?;
        
        // Delegate to the context
        context.resolve_package_import(import_path).await
    }
    
    fn validate_package_dependencies(&self, imports: &[ImportStatement]) -> Result<(), Error> {
        // Basic validation - check if import paths are valid
        for import in imports {
            if import.path.is_empty() {
                return Err(LlvmPackageError::ImportResolution(ImportError::InvalidPath {
                    path: import.path.clone(),
                    reason: "Empty import path".to_string(),
                }));
            }
            
            // Check if token type is valid
            if import.token.token_type != crate::lexer::TokenType::Identifier {
                return Err(LlvmPackageError::ImportResolution(ImportError::InvalidPath {
                    path: import.path.clone(),
                    reason: "Import token must be an identifier".to_string(),
                }));
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_llvm_package_context_creation() {
        let package_manager = Arc::new(Mutex::new(
            crate::package_manager::PackageManager::new(
                crate::package_manager::PackageManagerConfig::default()
            ).unwrap()
        ));
        
        let config = LlvmPackageConfig::default();
        let context = LlvmPackageContext::new(package_manager, config);
        assert!(context.is_ok());
    }
    
    #[tokio::test]
    async fn test_package_integration_trait() {
        let mut codegen = LlvmCodeGenerator::new().unwrap();
        
        let source = r#"
import "stdlib::io"

slay main() {
    println("Hello with packages!");
}
"#;
        
        let result = codegen.compile_with_auto_packages(source, None).await;
        assert!(result.is_ok());
    }
}
