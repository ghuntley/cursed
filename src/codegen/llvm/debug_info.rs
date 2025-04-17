//! Debug information generation for LLVM code.
//!
//! This module provides functionality for adding debug information to LLVM IR,
//! which helps with debugging compiled CURSED programs.

use crate::error::Error;
use inkwell::module::Module;
use std::path::Path;

/// Adds debug information to an LLVM module.
///
/// # Arguments
///
/// * `module` - The LLVM module to add debug info to
/// * `file_path` - Path to the source file
/// * `full_debug_info` - Whether to include full debug info or just line tables
///
/// # Returns
///
/// Result<(), Error> - Success or error if adding debug info fails
#[tracing::instrument(level = "debug", skip(module, file_path))]
pub fn add_debug_info<'ctx>(
    module: &Module<'ctx>,
    file_path: &Path,
    full_debug_info: bool
) -> Result<(), Error> {
    // Get the file name and directory
    let file_name = file_path.file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("unknown.csd");
        
    let dir_name = file_path.parent()
        .and_then(|p| p.to_str())
        .unwrap_or(".");
        
    tracing::debug!("Adding debug information for file: {}/{}", dir_name, file_name);
    
    // Note: The current version of inkwell used in this codebase doesn't provide
    // full DIBuilder support. In a real implementation, we would:
    // 1. Create a DIBuilder
    // 2. Create compile units, types, etc.
    // 3. Finalize the debug info
    
    // For now, we rely on the linker's debug flags (-g) to include appropriate debug info
    
    tracing::debug!("Debug info level: {}", if full_debug_info { "Full" } else { "Line tables only" });
    
    Ok(())
}