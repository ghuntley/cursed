//! LLVM IR output functionality for CURSED programs.
//!
//! This module provides capabilities to generate human-readable LLVM IR (.ll files)
//! and LLVM bitcode (.bc files) from compiled CURSED modules. This enables:
//!
//! - Debugging and inspection of generated LLVM IR
//! - Alternative compilation workflows using external LLVM tools
//! - Binary distribution of compiled modules as bitcode
//! - Teaching and learning about LLVM IR generation
//!
//! Key features:
//! - Clean, well-formatted LLVM IR output
//! - Efficient bitcode generation for binary distribution
//! - Support for both single-module and separate compilation workflows
//! - Configurable output directories and file naming
//! - Integration with existing compilation pipeline
//! - Proper error handling and validation

use crate::ast::Program;
use crate::codegen::llvm::{LlvmCodeGenerator, create_optimization_manager};
use crate::error::Error;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::memory_buffer::MemoryBuffer;

use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

use tracing::{debug, error, info, instrument};

/// Output format for LLVM IR generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrOutputFormat {
    /// Human-readable LLVM IR (.ll files)
    LlvmIr,
    /// LLVM bitcode (.bc files)
    Bitcode,
    /// Both IR and bitcode
    Both,
}

/// Configuration for IR output generation
#[derive(Debug, Clone)]
pub struct IrOutputConfig {
    /// Output format (IR, bitcode, or both)
    pub format: IrOutputFormat,
    /// Output directory for generated files
    pub output_dir: PathBuf,
    /// Whether to preserve directory structure from input
    pub preserve_structure: bool,
    /// Whether to optimize the module before output
    pub optimize: bool,
    /// Optimization level (O0, O1, O2, O3, Os, Oz)
    pub optimization_level: String,
    /// Base name for output files (when not preserving structure)
    pub base_name: Option<String>,
    /// Whether to include debug comments in IR output
    pub include_debug_comments: bool,
    /// Whether to show optimization statistics
    pub show_optimization_stats: bool,
}

impl Default for IrOutputConfig {
    fn default() -> Self {
        Self {
            format: IrOutputFormat::LlvmIr,
            output_dir: PathBuf::from("output"),
            preserve_structure: true,
            optimize: false,
            optimization_level: "O0".to_string(),
            base_name: None,
            include_debug_comments: true,
            show_optimization_stats: false,
        }
    }
}

/// LLVM IR output generator
pub struct IrOutputGenerator<'ctx> {
    context: &'ctx Context,
    config: IrOutputConfig,
}

impl<'ctx> IrOutputGenerator<'ctx> {
    /// Create a new IR output generator with the specified configuration
    pub fn new(context: &'ctx Context, config: IrOutputConfig) -> Self {
        Self {
            context,
            config,
        }
    }

    /// Create a new IR output generator with default configuration
    pub fn with_defaults(context: &'ctx Context) -> Self {
        Self::new(context, IrOutputConfig::default())
    }

    /// Generate IR/bitcode output for a compiled CURSED program
    #[instrument(skip(self, program, input_path), fields(
        input_path = ?input_path.as_ref(),
        format = ?self.config.format,
        output_dir = ?self.config.output_dir
    ))]
    pub fn generate_from_program<P: AsRef<Path>>(
        &self,
        program: &Program,
        input_path: P,
    ) -> Result<GeneratedFiles, Error> {
        info!("Generating IR output for CURSED program");

        // Create code generator and compile the program
        let input_path_ref = input_path.as_ref();
        let input_path_buf = input_path_ref.to_path_buf();
        let module_name = input_path_ref.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("program")
            .to_string();
        
        let mut code_generator = LlvmCodeGenerator::new(self.context, &module_name, input_path_buf);
        code_generator.compile_program(program)
            .map_err(|e| Error::CodeGenError(format!("Failed to compile program: {}", e)))?;

        let module = code_generator.module();
        self.generate_from_module(module, input_path)
    }

    /// Generate IR/bitcode output from a pre-compiled LLVM module
    #[instrument(skip(self, module), fields(
        input_path = ?input_path.as_ref(),
        format = ?self.config.format
    ))]
    pub fn generate_from_module<P: AsRef<Path>>(
        &self,
        module: &Module<'ctx>,
        input_path: P,
    ) -> Result<GeneratedFiles, Error> {
        let input_path = input_path.as_ref();
        debug!("Generating output from LLVM module for input: {:?}", input_path);

        // Optimize module if requested
        if self.config.optimize {
            debug!(
                optimization_level = %self.config.optimization_level,
                "Optimizing module before output"
            );
            
            let mut manager = create_optimization_manager(&self.config.optimization_level)
                .map_err(|e| Error::CodeGenError(format!("Failed to create optimization manager: {}", e)))?;
            
            manager.optimize_module(module)
                .map_err(|e| Error::CodeGenError(format!("Optimization failed: {}", e)))?;
            
            if self.config.show_optimization_stats {
                let stats = manager.get_stats();
                info!(
                    total_time = ?stats.total_time,
                    functions_optimized = stats.functions_optimized,
                    passes_applied = stats.passes_applied,
                    size_reduction = %format!("{:.2}%", stats.size_reduction_percentage()),
                    "Optimization completed"
                );
            }
        }

        // Ensure output directory exists
        fs::create_dir_all(&self.config.output_dir)
            .map_err(Error::IO)?;

        // Determine output file paths
        let (ir_path, bc_path) = self.compute_output_paths(input_path)?;

        let mut generated_files = GeneratedFiles::new();

        // Generate LLVM IR if requested
        if matches!(self.config.format, IrOutputFormat::LlvmIr | IrOutputFormat::Both) {
            if let Some(ref path) = ir_path {
                debug!("Generating LLVM IR to: {:?}", path);
                self.write_llvm_ir(module, path)?;
                generated_files.ir_file = Some(path.clone());
                info!("Generated LLVM IR: {:?}", path);
            }
        }

        // Generate bitcode if requested
        if matches!(self.config.format, IrOutputFormat::Bitcode | IrOutputFormat::Both) {
            if let Some(ref path) = bc_path {
                debug!("Generating LLVM bitcode to: {:?}", path);
                self.write_bitcode(module, path)?;
                generated_files.bitcode_file = Some(path.clone());
                info!("Generated LLVM bitcode: {:?}", path);
            }
        }

        Ok(generated_files)
    }

    /// Compute output file paths based on input path and configuration
    fn compute_output_paths<P: AsRef<Path>>(
        &self,
        input_path: P,
    ) -> Result<(Option<PathBuf>, Option<PathBuf>), Error> {
        let input_path = input_path.as_ref();

        let base_name = if let Some(ref name) = self.config.base_name {
            name.clone()
        } else {
            input_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output")
                .to_string()
        };

        let output_dir = if self.config.preserve_structure {
            // Try to preserve the directory structure
            let relative_dir = input_path
                .parent()
                .unwrap_or(Path::new("."));
            self.config.output_dir.join(relative_dir)
        } else {
            self.config.output_dir.clone()
        };

        // Ensure output directory exists
        fs::create_dir_all(&output_dir)
            .map_err(Error::IO)?;

        let ir_path = if matches!(self.config.format, IrOutputFormat::LlvmIr | IrOutputFormat::Both) {
            Some(output_dir.join(format!("{}.ll", base_name)))
        } else {
            None
        };

        let bc_path = if matches!(self.config.format, IrOutputFormat::Bitcode | IrOutputFormat::Both) {
            Some(output_dir.join(format!("{}.bc", base_name)))
        } else {
            None
        };

        Ok((ir_path, bc_path))
    }

    /// Write LLVM IR to a file
    #[instrument(skip(self, module), fields(output_path = ?output_path.as_ref()))]
    fn write_llvm_ir<P: AsRef<Path>>(
        &self,
        module: &Module<'ctx>,
        output_path: P,
    ) -> Result<(), Error> {
        let output_path = output_path.as_ref();
        
        // Get the LLVM IR string
        let ir_string = module.print_to_string().to_string();

        // Optionally add debug comments
        let final_content = if self.config.include_debug_comments {
            self.add_debug_comments(&ir_string)
        } else {
            ir_string
        };

        // Write to file
        fs::write(output_path, final_content)
            .map_err(Error::IO)?;

        debug!("Successfully wrote LLVM IR to: {:?}", output_path);
        Ok(())
    }

    /// Write LLVM bitcode to a file
    #[instrument(skip(self, module), fields(output_path = ?output_path.as_ref()))]
    fn write_bitcode<P: AsRef<Path>>(
        &self,
        module: &Module<'ctx>,
        output_path: P,
    ) -> Result<(), Error> {
        let output_path = output_path.as_ref();

        // Write bitcode to memory buffer
        let memory_buffer = module.write_bitcode_to_memory();
        let data = memory_buffer.as_slice();

        // Write to file
        fs::write(output_path, data)
            .map_err(Error::IO)?;

        debug!("Successfully wrote LLVM bitcode to: {:?}", output_path);
        Ok(())
    }

    /// Add debug comments to LLVM IR for better readability
    fn add_debug_comments(&self, ir_content: &str) -> String {
        let mut result = String::new();
        
        // Add header comment
        result.push_str("; Generated by CURSED compiler\n");
        result.push_str("; LLVM IR output for debugging and inspection\n");
        result.push_str(";\n");

        // Add the original IR content
        result.push_str(ir_content);

        result
    }
}

/// Information about generated files
#[derive(Debug, Clone, Default)]
pub struct GeneratedFiles {
    /// Path to generated LLVM IR file (.ll)
    pub ir_file: Option<PathBuf>,
    /// Path to generated bitcode file (.bc)
    pub bitcode_file: Option<PathBuf>,
}

impl GeneratedFiles {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any files were generated
    pub fn has_files(&self) -> bool {
        self.ir_file.is_some() || self.bitcode_file.is_some()
    }

    /// Get all generated file paths
    pub fn all_files(&self) -> Vec<&PathBuf> {
        let mut files = Vec::new();
        if let Some(ref ir) = self.ir_file {
            files.push(ir);
        }
        if let Some(ref bc) = self.bitcode_file {
            files.push(bc);
        }
        files
    }
}

/// Convenience function to generate IR output from a CURSED program
pub fn generate_ir_output<P: AsRef<Path>>(
    context: &Context,
    program: &Program,
    input_path: P,
    config: IrOutputConfig,
) -> Result<GeneratedFiles, Error> {
    let generator = IrOutputGenerator::new(context, config);
    generator.generate_from_program(program, input_path)
}

/// Convenience function to generate IR output with default configuration
pub fn generate_ir_output_default<P: AsRef<Path>>(
    context: &Context,
    program: &Program,
    input_path: P,
) -> Result<GeneratedFiles, Error> {
    let generator = IrOutputGenerator::with_defaults(context);
    generator.generate_from_program(program, input_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_ir_output_config_default() {
        let config = IrOutputConfig::default();
        assert_eq!(config.format, IrOutputFormat::LlvmIr);
        assert_eq!(config.output_dir, PathBuf::from("output"));
        assert!(config.preserve_structure);
        assert!(!config.optimize);
        assert!(config.base_name.is_none());
        assert!(config.include_debug_comments);
    }

    #[test]
    fn test_compute_output_paths() {
        let temp_dir = TempDir::new().unwrap();
        let config = IrOutputConfig {
            format: IrOutputFormat::Both,
            output_dir: temp_dir.path().to_path_buf(),
            preserve_structure: false,
            optimize: false,
            optimization_level: "O0".to_string(),
            base_name: Some("test".to_string()),
            include_debug_comments: true,
            show_optimization_stats: false,
        };

        let context = Context::create();
        let generator = IrOutputGenerator::new(&context, config);

        let input_path = Path::new("input/test.csd");
        let (ir_path, bc_path) = generator.compute_output_paths(input_path).unwrap();

        assert!(ir_path.is_some());
        assert!(bc_path.is_some());

        let ir_path = ir_path.unwrap();
        let bc_path = bc_path.unwrap();

        assert_eq!(ir_path.file_name().unwrap(), "test.ll");
        assert_eq!(bc_path.file_name().unwrap(), "test.bc");
    }

    #[test]
    fn test_generated_files() {
        let mut files = GeneratedFiles::new();
        assert!(!files.has_files());
        assert!(files.all_files().is_empty());

        files.ir_file = Some(PathBuf::from("test.ll"));
        assert!(files.has_files());
        assert_eq!(files.all_files().len(), 1);

        files.bitcode_file = Some(PathBuf::from("test.bc"));
        assert!(files.has_files());
        assert_eq!(files.all_files().len(), 2);
    }
}
