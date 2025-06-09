//! Package-level compilation pipeline for CURSED
//!
//! This module provides a higher-level interface for compiling CURSED packages,
//! handling the entire compilation pipeline from source code to linked executables.

use crate::codegen::llvm::separate_compilation::{SeparateCompiler, PackageMetadata};
use crate::error::Error;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info, instrument, warn};

/// Configuration for package compilation
#[derive(Debug, Clone)]
pub struct PackageCompilationConfig {
    /// Optimization level for compilation
    pub optimization_level: OptimizationLevel,
    /// Target triple for code generation
    pub target_triple: Option<String>,
    /// Output directory for compiled modules
    pub output_dir: PathBuf,
    /// Whether to generate debug information
    pub debug_info: bool,
    /// Whether to emit LLVM IR files
    pub emit_ir: bool,
    /// Whether to emit object files
    pub emit_object: bool,
}

impl Default for PackageCompilationConfig {
    fn default() -> Self {
        Self {
            optimization_level: OptimizationLevel::Default,
            target_triple: None,
            output_dir: PathBuf::from("./build"),
            debug_info: true,
            emit_ir: true,
            emit_object: true,
        }
    }
}

/// Represents a compiled package with all outputs
#[derive(Debug, Clone)]
pub struct CompiledPackage {
    /// Package metadata
    pub metadata: PackageMetadata,
    /// Path to generated LLVM IR file
    pub ir_file: Option<PathBuf>,
    /// Path to generated object file
    pub object_file: Option<PathBuf>,
    /// LLVM module (if kept in memory)
    pub module_name: String,
}

/// High-level package compilation pipeline
pub struct PackageCompilationPipeline<'ctx> {
    /// LLVM context
    context: &'ctx Context,
    /// Separate compiler instance
    compiler: SeparateCompiler<'ctx>,
    /// Compilation configuration
    config: PackageCompilationConfig,
    /// Target machine for code generation
    target_machine: Option<TargetMachine>,
    /// Compiled packages
    compiled_packages: HashMap<String, CompiledPackage>,
}

impl<'ctx> PackageCompilationPipeline<'ctx> {
    /// Create a new compilation pipeline
    pub fn new(context: &'ctx Context, config: PackageCompilationConfig) -> Result<Self, Error> {
        // Initialize LLVM targets
        Target::initialize_all(&InitializationConfig::default());

        // Create target machine if target is specified
        let target_machine = if let Some(ref triple) = config.target_triple {
            let target_triple = inkwell::targets::TargetTriple::create(triple);
            let target = Target::from_triple(&target_triple)
                .map_err(|e| Error::from_str(&format!("Invalid target triple '{}': {}", triple, e)))?;
            
            Some(target.create_target_machine(
                &target_triple,
                "",
                "",
                config.optimization_level,
                RelocMode::Default,
                CodeModel::Default,
            ).ok_or_else(|| Error::from_str("Failed to create target machine"))?)
        } else {
            None
        };

        Ok(Self {
            context,
            compiler: SeparateCompiler::new(context),
            config,
            target_machine,
            compiled_packages: HashMap::new(),
        })
    }

    /// Add a package source file to the compilation pipeline
    #[instrument(skip(self), level = "debug")]
    pub fn add_package(&mut self, package_path: &Path) -> Result<(), Error> {
        debug!(path = ?package_path, "Adding package to compilation pipeline");

        // Read and analyze the package
        let input = std::fs::read_to_string(package_path)
            .map_err(|e| Error::from_str(&format!("Failed to read package file: {}", e)))?;

        let metadata = self.compiler.analyze_package(&input, package_path.to_path_buf())?;
        self.compiler.add_package_source(&metadata.name, package_path.to_path_buf())?;

        info!(package = metadata.name, dependencies = ?metadata.dependencies, "Package added to pipeline");
        Ok(())
    }

    /// Add multiple package source files
    #[instrument(skip(self, package_paths), fields(package_count = package_paths.len()), level = "info")]
    pub fn add_packages(&mut self, package_paths: &[PathBuf]) -> Result<(), Error> {
        info!("Adding {} packages to compilation pipeline", package_paths.len());

        for path in package_paths {
            self.add_package(path)?;
        }

        Ok(())
    }

    /// Compile all packages in the pipeline
    #[instrument(skip(self), level = "info")]
    pub fn compile_all(&mut self) -> Result<(), Error> {
        info!("Compiling all packages in pipeline");

        // Ensure output directory exists
        std::fs::create_dir_all(&self.config.output_dir)
            .map_err(|e| Error::from_str(&format!("Failed to create output directory: {}", e)))?;

        // Compile all packages
        let modules = self.compiler.compile_all_packages()?;
        let compilation_order = self.compiler.get_compilation_order().to_vec();

        // Process each compiled module
        for (i, module) in modules.iter().enumerate() {
            let package_name = &compilation_order[i];
            
            if let Some(metadata) = self.compiler.get_package_metadata(package_name) {
                let compiled_package = self.process_compiled_module(module, metadata.clone())?;
                self.compiled_packages.insert(package_name.clone(), compiled_package);
            }
        }

        info!(packages_compiled = self.compiled_packages.len(), "All packages compiled successfully");
        Ok(())
    }

    /// Link all compiled packages into a single executable
    #[instrument(skip(self), level = "info")]
    pub fn link_executable(&self, output_path: &Path) -> Result<(), Error> {
        info!(output = ?output_path, "Linking executable");

        // Collect all object files
        let object_files: Vec<PathBuf> = self.compiled_packages
            .values()
            .filter_map(|pkg| pkg.object_file.as_ref())
            .cloned()
            .collect();

        if object_files.is_empty() {
            return Err(Error::from_str("No object files to link"));
        }

        // Use system linker to create executable
        self.link_object_files(&object_files, output_path)?;

        info!("Executable linked successfully");
        Ok(())
    }

    /// Compile and link a complete program
    #[instrument(skip(self, package_paths), fields(package_count = package_paths.len()), level = "info")]
    pub fn compile_and_link(&mut self, package_paths: &[PathBuf], output_path: &Path) -> Result<(), Error> {
        info!("Compiling and linking complete program");

        self.add_packages(package_paths)?;
        self.compile_all()?;
        self.link_executable(output_path)?;

        info!("Program compilation and linking completed successfully");
        Ok(())
    }

    /// Process a compiled LLVM module and generate output files
    #[instrument(skip(self, module), fields(package = metadata.name), level = "debug")]
    fn process_compiled_module(&self, module: &Module<'ctx>, metadata: PackageMetadata) -> Result<CompiledPackage, Error> {
        debug!(package = metadata.name, "Processing compiled module");

        let mut compiled_package = CompiledPackage {
            metadata: metadata.clone(),
            ir_file: None,
            object_file: None,
            module_name: module.get_name().to_string_lossy().to_string(),
        };

        // Generate IR file if requested
        if self.config.emit_ir {
            let ir_path = self.config.output_dir.join(format!("{}.ll", metadata.name));
            compiled_package.ir_file = Some(ir_path.clone());
            
            let ir_string = module.print_to_string().to_string();
            std::fs::write(&ir_path, ir_string)
                .map_err(|e| Error::from_str(&format!("Failed to write IR file: {}", e)))?;
            
            debug!(path = ?ir_path, "Generated LLVM IR file");
        }

        // Generate object file if requested
        if self.config.emit_object {
            if let Some(ref target_machine) = self.target_machine {
                let object_path = self.config.output_dir.join(format!("{}.o", metadata.name));
                compiled_package.object_file = Some(object_path.clone());

                target_machine.write_to_file(module, FileType::Object, &object_path)
                    .map_err(|e| Error::from_str(&format!("Failed to write object file: {}", e)))?;

                debug!(path = ?object_path, "Generated object file");
            } else {
                warn!("Cannot generate object file without target machine");
            }
        }

        Ok(compiled_package)
    }

    /// Link object files using system linker
    #[instrument(skip(self, object_files), fields(object_count = object_files.len()), level = "debug")]
    fn link_object_files(&self, object_files: &[PathBuf], output_path: &Path) -> Result<(), Error> {
        debug!("Linking {} object files", object_files.len());

        // For now, use a simple approach with system linker
        // In a full implementation, this would be more sophisticated
        let mut linker_args = vec![];

        // Add object files
        for obj_file in object_files {
            linker_args.push(obj_file.to_string_lossy().to_string());
        }

        // Add output specification
        linker_args.push("-o".to_string());
        linker_args.push(output_path.to_string_lossy().to_string());

        // Execute linker
        let output = std::process::Command::new("ld")
            .args(&linker_args)
            .output()
            .map_err(|e| Error::from_str(&format!("Failed to execute linker: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::from_str(&format!("Linker failed: {}", stderr)));
        }

        debug!("Object files linked successfully");
        Ok(())
    }

    /// Get information about a compiled package
    pub fn get_compiled_package(&self, package_name: &str) -> Option<&CompiledPackage> {
        self.compiled_packages.get(package_name)
    }

    /// Get all compiled packages
    pub fn get_all_compiled_packages(&self) -> &HashMap<String, CompiledPackage> {
        &self.compiled_packages
    }

    /// Get compilation configuration
    pub fn get_config(&self) -> &PackageCompilationConfig {
        &self.config
    }
}

/// Convenience function to compile a single package
#[instrument(level = "info")]
pub fn compile_single_package(
    package_path: &Path,
    config: PackageCompilationConfig,
) -> Result<CompiledPackage, Error> {
    info!(path = ?package_path, "Compiling single package");

    let context = Context::create();
    let mut pipeline = PackageCompilationPipeline::new(&context, config)?;
    
    pipeline.add_package(package_path)?;
    pipeline.compile_all()?;

    // Get the compiled package
    let input = std::fs::read_to_string(package_path)
        .map_err(|e| Error::from_str(&format!("Failed to read package file: {}", e)))?;

    let metadata = pipeline.compiler.analyze_package(&input, package_path.to_path_buf())?;
    
    pipeline.get_compiled_package(&metadata.name)
        .cloned()
        .ok_or_else(|| Error::from_str("Package not found after compilation"))
}

/// Convenience function to compile multiple packages and link them
#[instrument(skip(package_paths), fields(package_count = package_paths.len()), level = "info")]
pub fn compile_and_link_packages(
    package_paths: &[PathBuf],
    output_path: &Path,
    config: PackageCompilationConfig,
) -> Result<(), Error> {
    info!("Compiling and linking {} packages", package_paths.len());

    let context = Context::create();
    let mut pipeline = PackageCompilationPipeline::new(&context, config)?;
    
    pipeline.compile_and_link(package_paths, output_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_package_compilation_pipeline() {
        let context = Context::create();
        let temp_dir = TempDir::new().unwrap();
        
        let config = PackageCompilationConfig {
            output_dir: temp_dir.path().to_path_buf(),
            emit_ir: true,
            emit_object: false, // Skip object files for testing
            ..Default::default()
        };

        let mut pipeline = PackageCompilationPipeline::new(&context, config).unwrap();

        // Create a test package
        let package_content = r#"
vibe testpkg;

slay main() {
    vibez.spill("Hello from package!")
}
"#;

        let package_path = temp_dir.path().join("test.csd");
        fs::write(&package_path, package_content).unwrap();

        // Add and compile package
        pipeline.add_package(&package_path).unwrap();
        pipeline.compile_all().unwrap();

        // Check that IR file was generated
        let ir_path = temp_dir.path().join("testpkg.ll");
        assert!(ir_path.exists());

        // Check compiled package info
        let compiled = pipeline.get_compiled_package("testpkg").unwrap();
        assert_eq!(compiled.metadata.name, "testpkg");
        assert!(compiled.ir_file.is_some());
    }
}
