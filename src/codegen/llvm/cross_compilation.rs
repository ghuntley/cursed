//! Cross-compilation support for the CURSED compiler.
//!
//! This module provides functionality for compiling CURSED programs to
//! different target platforms using LLVM's cross-compilation capabilities.

use crate::error::Error;
use inkwell::module::Module;
use inkwell::targets::{Target, TargetMachine, TargetTriple, CodeModel, RelocMode};
use inkwell::OptimizationLevel;

/// Information about a compilation target.
#[derive(Debug, Clone)]
pub struct TargetInfo {
    /// The target triple (e.g., x86_64-unknown-linux-gnu)
    pub triple: String,
    
    /// The CPU name (e.g., generic, x86-64, core-avx2)
    pub cpu: String,
    
    /// CPU features (e.g., +avx2,+fma)
    pub features: String,
    
    /// System libraries specific to this target
    pub system_libraries: Vec<String>,
    
    /// C++ ABI to use (if applicable)
    pub cpp_abi: Option<String>,
}

impl TargetInfo {
    /// Creates a new target info with default settings.
    ///
    /// # Arguments
    ///
    /// * `triple` - The target triple
    ///
    /// # Returns
    ///
    /// A new TargetInfo instance
    pub fn new(triple: &str) -> Self {
        TargetInfo {
            triple: triple.to_string(),
            cpu: "generic".to_string(),
            features: "".to_string(),
            system_libraries: Vec::new(),
            cpp_abi: None,
        }
    }
    
    /// Sets the CPU for this target.
    ///
    /// # Arguments
    ///
    /// * `cpu` - The CPU name
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_cpu(mut self, cpu: &str) -> Self {
        self.cpu = cpu.to_string();
        self
    }
    
    /// Sets the features for this target.
    ///
    /// # Arguments
    ///
    /// * `features` - The features string
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_features(mut self, features: &str) -> Self {
        self.features = features.to_string();
        self
    }
    
    /// Adds a system library required for this target.
    ///
    /// # Arguments
    ///
    /// * `lib` - The library name
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_system_library(mut self, lib: &str) -> Self {
        self.system_libraries.push(lib.to_string());
        self
    }
    
    /// Sets the C++ ABI to use for this target.
    ///
    /// # Arguments
    ///
    /// * `abi` - The C++ ABI name
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_cpp_abi(mut self, abi: &str) -> Self {
        self.cpp_abi = Some(abi.to_string());
        self
    }
}

/// Configures an LLVM module for cross-compilation to the specified target.
///
/// # Arguments
///
/// * `module` - The LLVM module to configure
/// * `target_info` - Information about the target
/// * `opt_level` - Optimization level
///
/// # Returns
///
/// Result<TargetMachine, Error> - The created target machine or an error
#[tracing::instrument(level = "debug", skip(module))]
pub fn configure_target(
    module: &Module,
    target_info: &TargetInfo,
    opt_level: OptimizationLevel,
) -> Result<TargetMachine, Error> {
    // Create a TargetTriple from the string
    let target_triple = TargetTriple::create(&target_info.triple);
    
    // Set the triple on the module
    module.set_triple(&target_triple);
    
    // Get the target from the triple
    let target = Target::from_triple(&target_triple)
        .map_err(|e| Error::from_str(&format!("Failed to get target from triple: {}", e)))?;
    
    // Check if the target is valid for the given triple
    if !target.has_target_machine() {
        return Err(Error::from_str(&format!("Target {} does not support code generation", target_info.triple)));
    }
    
    // Check if the target supports JIT compilation
    if !target.has_asm_backend() {
        tracing::warn!("Target {} does not have full backend support", target_info.triple);
    }
    
    // Create a target machine
    let target_machine = target
        .create_target_machine(
            &target_triple,
            &target_info.cpu,
            &target_info.features,
            opt_level,
            RelocMode::Default,
            CodeModel::Default,
        )
        .ok_or_else(|| Error::from_str(&format!("Failed to create target machine for {}", target_info.triple)))?;
    
    // Set the data layout of the module to the target machine's data layout
    module.set_data_layout(&target_machine.get_target_data().get_data_layout());
    
    Ok(target_machine)
}

/// Gets default target information for the host platform.
///
/// # Returns
///
/// TargetInfo - Default target information for the current host
pub fn get_host_target_info() -> TargetInfo {
    let triple = TargetMachine::get_default_triple().to_string();
    let cpu = TargetMachine::get_host_cpu_name().to_string();
    let features = TargetMachine::get_host_cpu_features().to_string();
    
    TargetInfo {
        triple,
        cpu,
        features,
        system_libraries: Vec::new(),
        cpp_abi: None,
    }
}

/// Returns a list of all available targets supported by the current LLVM installation.
///
/// # Returns
///
/// Vec<String> - List of available target triples
pub fn get_available_targets() -> Vec<String> {
    // This is a simplification - in a real implementation, we would need to
    // query LLVM for available targets, which current inkwell doesn't expose directly
    vec![
        "x86_64-unknown-linux-gnu".to_string(),
        "x86_64-apple-darwin".to_string(),
        "x86_64-pc-windows-msvc".to_string(),
        "aarch64-unknown-linux-gnu".to_string(),
        "aarch64-apple-darwin".to_string(),
        "wasm32-unknown-unknown".to_string(),
    ]
}