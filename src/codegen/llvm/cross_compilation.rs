//! Cross-compilation support for LLVM code generation.
//!
//! This module provides functionality for configuring LLVM modules
//! to target different platforms and architectures.

use crate::error::Error;

use inkwell::module::Module;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple};
use inkwell::OptimizationLevel;

use std::sync::Once;

// Ensure LLVM targets are initialized only once
static LLVM_INIT: Once = Once::new();

/// Represents a supported target platform for cross-compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetPlatform {
    /// The host platform (native compilation)
    Host,
    /// Windows x86_64
    WindowsX86_64,
    /// Linux x86_64
    LinuxX86_64,
    /// macOS x86_64
    MacOSX86_64,
    /// Linux ARM 64-bit (AArch64)
    LinuxAarch64,
    /// macOS ARM 64-bit (Apple Silicon)
    MacOSAarch64,
    /// WebAssembly
    Wasm32,
}

impl TargetPlatform {
    /// Converts a platform to its LLVM target triple string.
    pub fn to_target_triple(&self) -> String {
        match self {
            Self::Host => TargetMachine::get_default_triple().to_string(),
            Self::WindowsX86_64 => "x86_64-pc-windows-msvc".to_string(),
            Self::LinuxX86_64 => "x86_64-unknown-linux-gnu".to_string(),
            Self::MacOSX86_64 => "x86_64-apple-darwin".to_string(),
            Self::LinuxAarch64 => "aarch64-unknown-linux-gnu".to_string(),
            Self::MacOSAarch64 => "aarch64-apple-darwin".to_string(),
            Self::Wasm32 => "wasm32-unknown-unknown".to_string(),
        }
    }
    
    /// Returns additional compiler/linker flags needed for the platform.
    pub fn get_platform_flags(&self) -> Vec<String> {
        match self {
            Self::WindowsX86_64 => vec!["-lmsvcrt".to_string()],
            Self::LinuxX86_64 | Self::LinuxAarch64 => vec!["-ldl".to_string(), "-lpthread".to_string()],
            Self::MacOSX86_64 | Self::MacOSAarch64 => vec![
                "-framework".to_string(), "Foundation".to_string(),
                "-framework".to_string(), "CoreFoundation".to_string(),
            ],
            Self::Wasm32 => vec!["--no-entry".to_string(), "--export-all".to_string()],
            Self::Host => {
                #[cfg(target_os = "linux")]
                return vec!["-ldl".to_string(), "-lpthread".to_string()];
                
                #[cfg(target_os = "macos")]
                return vec![
                    "-framework".to_string(), "Foundation".to_string(),
                    "-framework".to_string(), "CoreFoundation".to_string(),
                ];
                
                #[cfg(target_os = "windows")]
                return vec!["-lmsvcrt".to_string()];
                
                #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
                return vec![];
            }
        }
    }
    
    /// Returns the appropriate linker for the platform.
    pub fn get_linker(&self) -> String {
        match self {
            Self::WindowsX86_64 => "link.exe".to_string(),
            Self::LinuxX86_64 | Self::LinuxAarch64 => "gcc".to_string(),
            Self::MacOSX86_64 | Self::MacOSAarch64 => "clang".to_string(),
            Self::Wasm32 => "wasm-ld".to_string(),
            Self::Host => {
                #[cfg(target_os = "linux")]
                return "gcc".to_string();
                
                #[cfg(target_os = "macos")]
                return "clang".to_string();
                
                #[cfg(target_os = "windows")]
                return "link.exe".to_string();
                
                #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
                return "gcc".to_string();
            }
        }
    }
    
    /// Returns the file extension for executables on this platform.
    pub fn executable_extension(&self) -> &'static str {
        match self {
            Self::WindowsX86_64 => ".exe",
            Self::LinuxX86_64 | Self::LinuxAarch64 | Self::MacOSX86_64 | Self::MacOSAarch64 => "",
            Self::Wasm32 => ".wasm",
            Self::Host => {
                #[cfg(target_os = "windows")]
                return ".exe";
                
                #[cfg(not(target_os = "windows"))]
                return "";
            }
        }
    }
}

/// Initializes LLVM target support.
///
/// This function ensures LLVM targets are initialized for cross-compilation.
/// It only needs to be called once per program execution.
#[tracing::instrument(level = "debug")]
pub fn initialize_targets() {
    LLVM_INIT.call_once(|| {
        tracing::debug!("Initializing LLVM targets");
        let config = InitializationConfig {
            asm_parser: true,
            asm_printer: true,
            base: true,
            disassembler: true,
            info: true,
            machine_code: true,
        };
        Target::initialize_all(&config);
    });
}

/// Configures an LLVM module for cross-compilation to a target platform.
///
/// # Arguments
///
/// * `module` - The LLVM module to configure
/// * `target` - The target platform
/// * `optimization_level` - The optimization level
///
/// # Returns
///
/// Result<TargetMachine, Error> - The target machine for the target platform
#[tracing::instrument(level = "info", skip(module))]
pub fn configure_module_for_target(
    module: &Module,
    target: TargetPlatform,
    optimization_level: OptimizationLevel,
) -> Result<TargetMachine, Error> {
    // Ensure targets are initialized
    initialize_targets();
    
    // Get the target triple
    let target_triple_str = target.to_target_triple();
    tracing::info!("Configuring for target: {}", target_triple_str);
    let target_triple = TargetTriple::create(&target_triple_str);
    
    // Get the target from the triple
    let target = Target::from_triple(&target_triple)
        .map_err(|e| Error::from_str(&format!("Failed to get target from triple: {}", e)))?;
    
    // Create a target machine
    let target_machine = target
        .create_target_machine(
            &target_triple,
            &TargetMachine::get_host_cpu_name().to_string(),
            &TargetMachine::get_host_cpu_features().to_string(),
            optimization_level,
            RelocMode::Default,
            CodeModel::Default,
        )
        .ok_or_else(|| Error::from_str("Failed to create target machine"))?;
    
    // Set the data layout of the module to the target machine's data layout
    module.set_data_layout(&target_machine.get_target_data().get_data_layout());
    module.set_triple(&target_triple);
    
    tracing::info!("Successfully configured module for target: {}", target_triple_str);
    Ok(target_machine)
}