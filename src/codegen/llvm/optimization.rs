//! LLVM optimization support
//!
//! This module provides backward compatibility for the existing optimization API
//! while integrating with the new comprehensive optimization system.

use inkwell::module::Module;
use super::optimization_passes::{OptimizationManager, create_optimization_manager};

/// Apply standard optimizations to the LLVM module.
/// 
/// This function provides backward compatibility with the existing API while
/// using the new comprehensive optimization system internally.
#[tracing::instrument(skip(module), fields(module_name = ?module.get_name().to_string_lossy(), optimization_level = optimization_level), level = "debug")]
pub fn apply_optimizations<'ctx>(module: &Module<'ctx>, optimization_level: u32) -> Result<(), String> {
    tracing::info!(optimization_level = optimization_level, "Applying LLVM optimizations");
    
    // Create optimization manager for the specified level
    let mut manager = OptimizationManager::for_level(optimization_level as u8)?;
    
    // Apply optimizations using the new system
    manager.optimize_module(module)?;
    
    // Log optimization statistics
    let stats = manager.get_stats();
    tracing::info!(
        total_time = ?stats.total_time,
        functions_optimized = stats.functions_optimized,
        passes_applied = stats.passes_applied,
        size_reduction = %format!("{:.2}%", stats.size_reduction_percentage()),
        "LLVM optimization complete"
    );
    
    Ok(())
}

/// Apply optimizations with a specific optimization level string (O0, O1, O2, O3, Os, Oz)
#[tracing::instrument(skip(module), fields(module_name = ?module.get_name().to_string_lossy(), optimization_level = %level), level = "debug")]
pub fn apply_optimizations_with_level<'ctx>(module: &Module<'ctx>, level: &str) -> Result<(), String> {
    tracing::info!(optimization_level = %level, "Applying LLVM optimizations");
    
    // Create optimization manager for the specified level
    let mut manager = create_optimization_manager(level)?;
    
    // Apply optimizations
    manager.optimize_module(module)?;
    
    // Log optimization statistics
    let stats = manager.get_stats();
    tracing::info!(
        total_time = ?stats.total_time,
        functions_optimized = stats.functions_optimized,
        passes_applied = stats.passes_applied,
        size_reduction = %format!("{:.2}%", stats.size_reduction_percentage()),
        "LLVM optimization complete"
    );
    
    Ok(())
}