//! Optimization passes for LLVM modules.
//!
//! This module provides the implementation for the optimize_module method
//! of the BinaryCompiler struct.

use crate::error::Error;
use super::size_optimization;

use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::OptimizationLevel;

/// Optimizes an LLVM module.
///
/// This function applies various LLVM optimization passes to the module
/// based on the configured optimization level and size optimization setting.
///
/// # Arguments
///
/// * `module` - The LLVM module to optimize
/// * `optimization_level` - The optimization level to apply
/// * `optimize_for_size` - Whether to optimize for size
///
/// # Returns
///
/// Result<(), Error> - Success or an error if optimization fails
#[tracing::instrument(level = "debug", skip(module))]
pub fn optimize_module<'ctx>(
    module: &Module<'ctx>,
    optimization_level: OptimizationLevel,
    optimize_for_size: bool,
) -> Result<(), Error> {
    tracing::debug!("Running optimization passes at level: {:?}", optimization_level);
    
    // Apply standard optimizations based on the optimization level
    let pass_manager = PassManager::create(module);
    
    // Run different passes based on optimization level
    match optimization_level {
        OptimizationLevel::None => {
            // No optimizations
        },
        OptimizationLevel::Less => {
            // Basic optimizations
            pass_manager.add_promote_memory_to_register_pass();
            pass_manager.add_instruction_combining_pass();
            pass_manager.add_reassociate_pass();
        },
        OptimizationLevel::Default | OptimizationLevel::Aggressive => {
            // Standard optimizations
            pass_manager.add_promote_memory_to_register_pass();
            pass_manager.add_instruction_combining_pass();
            pass_manager.add_reassociate_pass();
            pass_manager.add_gvn_pass();
            pass_manager.add_cfg_simplification_pass();
            
            if optimization_level == OptimizationLevel::Aggressive {
                // Additional aggressive optimizations
                pass_manager.add_function_inlining_pass();
                pass_manager.add_tail_call_elimination_pass();
            }
        }
    }
    
    // Run the pass manager
    pass_manager.run_on(module);
    
    // Apply size-specific optimizations if requested
    if optimize_for_size {
        tracing::info!("Applying size optimizations");
        let size_level = if optimization_level == OptimizationLevel::Aggressive {
            size_optimization::SizeOptimizationLevel::Aggressive
        } else {
            size_optimization::SizeOptimizationLevel::Default
        };
        
        size_optimization::optimize_module_for_size(module, size_level, optimization_level)?;
    }
    
    tracing::debug!("Successfully ran optimization passes");
    Ok(())
}