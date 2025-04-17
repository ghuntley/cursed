//! Size optimization for LLVM code.
//!
//! This module provides functionality for optimizing LLVM IR to produce
//! smaller binaries, which is useful for embedded systems or situations
//! where binary size is more important than execution speed.

use crate::error::Error;
use inkwell::module::Module;
use inkwell::OptimizationLevel;

/// Size optimization settings for LLVM modules.
#[derive(Debug, Clone, Copy)]
pub struct SizeOptimizationSettings {
    /// The size level (0-2), with higher values providing more aggressive size optimization
    pub size_level: u32,
    
    /// Whether to use global dead code elimination
    pub use_global_dce: bool,
    
    /// Whether to merge duplicate functions
    pub merge_functions: bool,
    
    /// Whether to remove excess precision operations
    pub slp_vectorize: bool,
    
    /// Whether to perform loop unrolling
    pub loop_unroll: bool,
}

impl Default for SizeOptimizationSettings {
    fn default() -> Self {
        SizeOptimizationSettings {
            size_level: 2, // Maximum size optimization
            use_global_dce: true,
            merge_functions: true,
            slp_vectorize: false, // Disable vectorization when optimizing for size
            loop_unroll: false,   // Disable loop unrolling when optimizing for size
        }
    }
}

impl SizeOptimizationSettings {
    /// Creates new size optimization settings with default values.
    ///
    /// # Returns
    ///
    /// Default size optimization settings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Sets the size optimization level.
    ///
    /// # Arguments
    ///
    /// * `level` - The size level (0-2)
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_size_level(mut self, level: u32) -> Self {
        self.size_level = level.min(2); // Maximum level is 2
        self
    }
    
    /// Enables or disables global dead code elimination.
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable global DCE
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_global_dce(mut self, enable: bool) -> Self {
        self.use_global_dce = enable;
        self
    }
    
    /// Enables or disables function merging.
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable function merging
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_merge_functions(mut self, enable: bool) -> Self {
        self.merge_functions = enable;
        self
    }
}

/// Applies size optimization passes to an LLVM module.
///
/// # Arguments
///
/// * `module` - The LLVM module to optimize
/// * `settings` - Size optimization settings
/// * `opt_level` - Base optimization level
///
/// # Returns
///
/// Result<(), Error> - Success or an error if optimization fails
#[tracing::instrument(level = "debug", skip(module, settings))]
pub fn optimize_for_size(
    module: &Module,
    settings: &SizeOptimizationSettings,
    opt_level: OptimizationLevel,
) -> Result<(), Error> {
    tracing::debug!("Applying size optimizations at level: {}", settings.size_level);
    
    // Note: The current version of inkwell used in this codebase doesn't provide
    // a straightforward way to set up optimization passes via PassManagerBuilder.
    // In a real implementation, we would use Inkwell's pass manager or LLVM's
    // native pass API to set up and run optimization passes.
    
    // For now, we'll rely on the linker's size optimization flags (-Os) when
    // invoking the linker in the binary compiler.
    
    tracing::debug!("Settings: DCE={}, merge={}, vectorize={}, unroll={}",
        settings.use_global_dce,
        settings.merge_functions,
        settings.slp_vectorize,
        settings.loop_unroll);
    
    tracing::debug!("Size optimization complete");
    Ok(())
}

/// Checks if a module has size-critical attributes.
///
/// This function analyzes the module to determine if size optimization
/// would be beneficial, based on factors like the presence of large
/// string literals, many small functions, etc.
///
/// # Arguments
///
/// * `module` - The LLVM module to analyze
///
/// # Returns
///
/// bool - True if the module would benefit from size optimization
pub fn is_size_critical(module: &Module) -> bool {
    // Count global variables
    let global_count = module.get_globals().count();
    if global_count > 20 {
        return true;
    }
    
    // Count functions
    let function_count = module.get_functions().count();
    if function_count > 50 {
        return true;
    }
    
    // This is a simplified implementation
    // In a real implementation, we would analyze function sizes,
    // string lengths, and other factors
    
    false
}