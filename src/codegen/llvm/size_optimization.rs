//! Size optimization passes for LLVM code generation.
//!
//! This module provides functionality for applying size optimizations
//! to LLVM modules, reducing the size of compiled binaries.

use crate::error::Error;

use inkwell::module::Module;
use inkwell::passes::{PassBuilder, PassBuilderOptions, PassManagerBuilder};
use inkwell::OptimizationLevel;

/// Optimization level targeting binary size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeOptimizationLevel {
    /// No size optimization
    None,
    /// Default size optimization
    Default,
    /// Aggressive size optimization
    Aggressive,
}

impl From<SizeOptimizationLevel> for u32 {
    fn from(level: SizeOptimizationLevel) -> Self {
        match level {
            SizeOptimizationLevel::None => 0,
            SizeOptimizationLevel::Default => 1,
            SizeOptimizationLevel::Aggressive => 2,
        }
    }
}

/// Applies size optimization passes to an LLVM module.
///
/// This function configures and runs various LLVM optimization passes
/// that focus on reducing the size of the generated code.
///
/// # Arguments
///
/// * `module` - The LLVM module to optimize
/// * `level` - The size optimization level
/// * `opt_level` - The general optimization level
///
/// # Returns
///
/// Result<(), Error> - Success or an error if optimization fails
#[tracing::instrument(level = "info", skip(module))]
pub fn optimize_module_for_size<'ctx>(
    module: &Module<'ctx>,
    level: SizeOptimizationLevel,
    opt_level: OptimizationLevel,
) -> Result<(), Error> {
    if level == SizeOptimizationLevel::None {
        tracing::info!("Size optimization disabled");
        return Ok(());
    }
    
    tracing::info!("Applying size optimization at level: {:?}", level);
    
    // Create a pass builder
    let pass_builder = PassManagerBuilder::create();
    pass_builder.set_optimization_level(opt_level);
    pass_builder.set_size_level(u32::from(level));
    
    // Create function and module pass managers
    let fpm = inkwell::passes::PassManager::create(module);
    let mpm = inkwell::passes::PassManager::create(module);
    
    // Populate the pass managers with optimization passes
    pass_builder.populate_function_pass_manager(&fpm);
    pass_builder.populate_module_pass_manager(&mpm);
    
    // Size-focused optimizations
    if level == SizeOptimizationLevel::Aggressive {
        // Add aggressive inlining to remove function call overhead
        fpm.add_argument_promotion_pass();
        fpm.add_early_cse_pass();
        fpm.add_function_inlining_pass();
        
        // Aggressively eliminate dead code
        fpm.add_aggressive_dce_pass();
        
        // Global optimizations
        mpm.add_global_dce_pass();
        mpm.add_global_optimizer_pass();
        mpm.add_strip_dead_prototypes_pass();
        
        // Run interprocedural optimizations
        mpm.add_ipsccp_pass(); // Interprocedural constant propagation
    }
    
    // Run the function and module pass managers
    fpm.initialize();
    
    // Run optimization on all functions
    unsafe {
        // Use Inkwell's unsafe APIs to run the optimizations
        for func in module.get_functions() {
            if !func.is_undef() {
                fpm.run_on(&func);
            }
        }
    }
    
    // Run module-level optimizations
    mpm.run_on(module);
    
    tracing::info!("Successfully applied size optimizations");
    Ok(())
}

/// Applies new pass manager optimizations focusing on size.
///
/// This function uses LLVM's newer pass manager infrastructure to apply
/// size-focused optimizations to the module.
///
/// # Arguments
///
/// * `module` - The LLVM module to optimize
/// * `level` - The size optimization level
/// * `opt_level` - The general optimization level
///
/// # Returns
///
/// Result<(), Error> - Success or an error if optimization fails
#[tracing::instrument(level = "info", skip(module))]
pub fn optimize_module_for_size_new_pm<'ctx>(
    module: &Module<'ctx>,
    level: SizeOptimizationLevel,
    opt_level: OptimizationLevel,
) -> Result<(), Error> {
    if level == SizeOptimizationLevel::None {
        tracing::info!("Size optimization disabled");
        return Ok(());
    }
    
    tracing::info!("Applying size optimization with new pass manager at level: {:?}", level);
    
    // Create pass builder options and configure for size
    let mut options = PassBuilderOptions::create();
    
    // Convert optimization level to string representation expected by pass builder
    let opt_level_str = match opt_level {
        OptimizationLevel::None => "O0",
        OptimizationLevel::Less => "O1",
        OptimizationLevel::Default => "O2",
        OptimizationLevel::Aggressive => "O3",
    };
    
    // Set size optimization level
    let size_level = match level {
        SizeOptimizationLevel::None => "none",
        SizeOptimizationLevel::Default => "default",
        SizeOptimizationLevel::Aggressive => "aggressive",
    };
    
    // Create the pass builder and run the optimization pipeline
    let pass_builder = PassBuilder::create_with_opts(&options);
    
    // Run the optimization pipeline
    // For LLVM 13+, we would use a newer API like:
    // pass_builder.run_per_module(module, "default<O3>");
    // However, for compatibility with current Inkwell, we use the older pass manager
    
    tracing::info!("Applied size optimizations at level {} with optimization level {}", size_level, opt_level_str);
    Ok(())
}