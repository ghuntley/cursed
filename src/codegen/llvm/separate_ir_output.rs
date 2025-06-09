//! IR output integration for separate compilation.
//!
//! This module extends the separate compilation system to support
//! generation of LLVM IR and bitcode files for individual packages
//! and linked modules.

use crate::codegen::llvm::{
    SeparateCompiler, PackageMetadata, IrOutputGenerator, IrOutputConfig, 
    IrOutputFormat, GeneratedFiles
};
use crate::error::Error;

use inkwell::context::Context;
use inkwell::module::Module;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

use tracing::{debug, error, info, instrument};

/// Configuration for separate compilation IR output
#[derive(Debug, Clone)]
pub struct SeparateIrOutputConfig {
    /// Base IR output configuration
    pub ir_config: IrOutputConfig,
    /// Whether to generate per-package output
    pub per_package: bool,
    /// Whether to generate linked output
    pub linked_output: bool,
    /// Package output subdirectory name
    pub package_subdir: String,
    /// Linked output subdirectory name
    pub linked_subdir: String,
}

impl Default for SeparateIrOutputConfig {
    fn default() -> Self {
        Self {
            ir_config: IrOutputConfig::default(),
            per_package: true,
            linked_output: true,
            package_subdir: "packages".to_string(),
            linked_subdir: "linked".to_string(),
        }
    }
}

/// IR output for separate compilation results
pub struct SeparateIrOutput<'ctx> {
    context: &'ctx Context,
    config: SeparateIrOutputConfig,
}

impl<'ctx> SeparateIrOutput<'ctx> {
    /// Create a new separate IR output generator
    pub fn new(context: &'ctx Context, config: SeparateIrOutputConfig) -> Self {
        Self { context, config }
    }

    /// Create with default configuration
    pub fn with_defaults(context: &'ctx Context) -> Self {
        Self::new(context, SeparateIrOutputConfig::default())
    }

    /// Generate IR output for all compiled packages
    #[instrument(skip(self, compiler, modules), fields(
        num_packages = modules.len(),
        format = ?self.config.ir_config.format
    ))]
    pub fn generate_package_output(
        &self,
        compiler: &SeparateCompiler<'ctx>,
        modules: &HashMap<String, Module<'ctx>>,
    ) -> Result<SeparateGeneratedFiles, Error> {
        info!("Generating IR output for {} packages", modules.len());

        let mut result = SeparateGeneratedFiles::new();

        // Generate per-package output if requested
        if self.config.per_package {
            result.package_files = self.generate_per_package_output(compiler, modules)?;
        }

        Ok(result)
    }

    /// Generate IR output for individual packages
    #[instrument(skip(self, compiler, modules))]
    fn generate_per_package_output(
        &self,
        compiler: &SeparateCompiler<'ctx>,
        modules: &HashMap<String, Module<'ctx>>,
    ) -> Result<HashMap<String, GeneratedFiles>, Error> {
        let mut package_files = HashMap::new();

        for (package_name, module) in modules {
            debug!("Generating IR for package: {}", package_name);

            // Create package-specific output directory
            let package_output_dir = self.config.ir_config.output_dir
                .join(&self.config.package_subdir)
                .join(package_name);

            // Configure IR generator for this package
            let mut package_config = self.config.ir_config.clone();
            package_config.output_dir = package_output_dir;
            package_config.base_name = Some(package_name.clone());

            let ir_generator = IrOutputGenerator::new(self.context, package_config);

            // Use package source path as input reference
            let input_path = compiler.get_package_source_path(package_name)
                .unwrap_or_else(|| PathBuf::from(format!("{}.csd", package_name)));

            let generated = ir_generator.generate_from_module(module, &input_path)?;
            package_files.insert(package_name.clone(), generated);

            info!("Generated IR for package: {}", package_name);
        }

        Ok(package_files)
    }

    /// Generate IR output for a linked module
    #[instrument(skip(self, linked_module), fields(output_name))]
    pub fn generate_linked_output(
        &self,
        linked_module: &Module<'ctx>,
        output_name: &str,
    ) -> Result<GeneratedFiles, Error> {
        tracing::Span::current().record("output_name", output_name);
        info!("Generating linked IR output: {}", output_name);

        // Create linked output directory
        let linked_output_dir = self.config.ir_config.output_dir
            .join(&self.config.linked_subdir);

        // Configure IR generator for linked output
        let mut linked_config = self.config.ir_config.clone();
        linked_config.output_dir = linked_output_dir;
        linked_config.base_name = Some(output_name.to_string());

        let ir_generator = IrOutputGenerator::new(self.context, linked_config);

        // Generate output
        let input_path = PathBuf::from(format!("{}.linked", output_name));
        ir_generator.generate_from_module(linked_module, &input_path)
    }

    /// Generate IR output for entire separate compilation workflow
    #[instrument(skip(self, compiler, modules, linked_module))]
    pub fn generate_complete_output(
        &self,
        compiler: &SeparateCompiler<'ctx>,
        modules: &HashMap<String, Module<'ctx>>,
        linked_module: Option<&Module<'ctx>>,
        output_name: &str,
    ) -> Result<SeparateGeneratedFiles, Error> {
        info!("Generating complete IR output for separate compilation");

        let mut result = self.generate_package_output(compiler, modules)?;

        // Generate linked output if requested and available
        if self.config.linked_output {
            if let Some(linked) = linked_module {
                result.linked_files = Some(self.generate_linked_output(linked, output_name)?);
            }
        }

        Ok(result)
    }
}

/// Results of separate compilation IR generation
#[derive(Debug, Default)]
pub struct SeparateGeneratedFiles {
    /// Generated files per package
    pub package_files: HashMap<String, GeneratedFiles>,
    /// Generated files for linked output
    pub linked_files: Option<GeneratedFiles>,
}

impl SeparateGeneratedFiles {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any files were generated
    pub fn has_files(&self) -> bool {
        !self.package_files.is_empty() || self.linked_files.is_some()
    }

    /// Get total number of generated files
    pub fn total_file_count(&self) -> usize {
        let package_count: usize = self.package_files
            .values()
            .map(|files| files.all_files().len())
            .sum();

        let linked_count = self.linked_files
            .as_ref()
            .map(|files| files.all_files().len())
            .unwrap_or(0);

        package_count + linked_count
    }

    /// Get all generated file paths
    pub fn all_files(&self) -> Vec<&PathBuf> {
        let mut all_files = Vec::new();

        for generated in self.package_files.values() {
            all_files.extend(generated.all_files());
        }

        if let Some(ref linked) = self.linked_files {
            all_files.extend(linked.all_files());
        }

        all_files
    }

    /// Print summary of generated files
    pub fn print_summary(&self) {
        println!("Generated IR/Bitcode Files:");
        
        if !self.package_files.is_empty() {
            println!("  Packages:");
            for (package_name, files) in &self.package_files {
                println!("    {}:", package_name);
                for file in files.all_files() {
                    println!("      {:?}", file);
                }
            }
        }

        if let Some(ref linked) = self.linked_files {
            println!("  Linked:");
            for file in linked.all_files() {
                println!("    {:?}", file);
            }
        }

        println!("Total files: {}", self.total_file_count());
    }
}

/// Extension trait for SeparateCompiler to add IR output capabilities
pub trait SeparateCompilerIrExt<'ctx> {
    /// Get source path for a package
    fn get_package_source_path(&self, package_name: &str) -> Option<PathBuf>;
}

impl<'ctx> SeparateCompilerIrExt<'ctx> for SeparateCompiler<'ctx> {
    fn get_package_source_path(&self, package_name: &str) -> Option<PathBuf> {
        // This would need to be implemented in the actual SeparateCompiler
        // For now, return a default path
        Some(PathBuf::from(format!("{}.csd", package_name)))
    }
}

/// Convenience function for generating IR output from separate compilation
pub fn generate_separate_ir_output<'ctx>(
    context: &'ctx Context,
    compiler: &SeparateCompiler<'ctx>,
    modules: &HashMap<String, Module<'ctx>>,
    linked_module: Option<&Module<'ctx>>,
    config: SeparateIrOutputConfig,
    output_name: &str,
) -> Result<SeparateGeneratedFiles, Error> {
    let ir_output = SeparateIrOutput::new(context, config);
    ir_output.generate_complete_output(compiler, modules, linked_module, output_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_separate_ir_output_config_default() {
        let config = SeparateIrOutputConfig::default();
        assert!(config.per_package);
        assert!(config.linked_output);
        assert_eq!(config.package_subdir, "packages");
        assert_eq!(config.linked_subdir, "linked");
    }

    #[test]
    fn test_separate_generated_files() {
        let mut files = SeparateGeneratedFiles::new();
        assert!(!files.has_files());
        assert_eq!(files.total_file_count(), 0);

        let mut package_files = GeneratedFiles::new();
        package_files.ir_file = Some(PathBuf::from("test.ll"));
        files.package_files.insert("test".to_string(), package_files);

        assert!(files.has_files());
        assert_eq!(files.total_file_count(), 1);
    }
}
