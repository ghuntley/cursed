//! Platform Abstraction Layer (PAL) for CURSED Runtime
//! 
//! This module provides platform-specific abstractions for:
//! - ARM64 (aarch64) - macOS and Linux
//! - x86_64 - macOS, Linux, and Windows  
//! - WebAssembly (WASM) - Browser and standalone runtimes
//!
//! The PAL handles:
//! - Memory management and allocation
//! - System calls and OS interfaces
//! - Thread synchronization primitives
//! - Platform-specific optimizations
//! - Architecture-specific register handling

pub mod arm64;
pub mod x86_64; 
pub mod wasm;
pub mod common;

use crate::runtime::memory::MemoryManager;
use crate::runtime::goroutine::Scheduler;
use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;

/// Platform abstraction trait for runtime operations
pub trait PlatformAbstraction: Send + Sync {
    /// Initialize platform-specific runtime components
    fn initialize(&self) -> Result<(), PlatformError>;
    
    /// Get platform-specific memory manager
    fn memory_manager(&self) -> Arc<dyn MemoryManager>;
    
    /// Get platform-specific scheduler
    fn scheduler(&self) -> Arc<dyn Scheduler>;
    
    /// Platform-specific stack size for goroutines
    fn default_stack_size(&self) -> usize;
    
    /// Platform-specific memory page size
    fn page_size(&self) -> usize;
    
    /// Number of hardware threads available
    fn hardware_concurrency(&self) -> usize;
    
    /// Platform name for debugging
    fn platform_name(&self) -> &'static str;
    
    /// Architecture name
    fn architecture(&self) -> Architecture;
    
    /// Operating system
    fn operating_system(&self) -> OperatingSystem;

    // Enhanced platform capabilities
    
    /// Get platform version information
    fn get_version_info(&self, verbose: bool) -> String {
        common::VersionReporter::generate_version_report(verbose)
    }
    
    /// Get basic platform statistics
    fn get_basic_stats(&self) -> BasicStats {
        BasicStats {
            uptime: Duration::from_secs(0),
            memory_usage: 0,
            cpu_cores: self.hardware_concurrency(),
            platform_name: self.platform_name().to_string(),
        }
    }
}

/// Basic platform statistics
#[derive(Debug, Clone)]
pub struct BasicStats {
    pub uptime: Duration,
    pub memory_usage: u64,
    pub cpu_cores: usize,
    pub platform_name: String,
}

impl BasicStats {
    pub fn format_human_readable(&self) -> String {
        format!(
            "Platform: {}\n\
            CPU Cores: {}\n\
            Memory Usage: {} MB\n\
            Uptime: {:?}\n",
            self.platform_name,
            self.cpu_cores,
            self.memory_usage / 1024 / 1024,
            self.uptime
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    Arm64,
    X86_64,
    Wasm32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingSystem {
    MacOS,
    Linux,
    Windows,
    Browser,
    WasmRuntime,
}

#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    #[error("Platform initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Unsupported platform: {arch:?} on {os:?}")]
    UnsupportedPlatform { arch: Architecture, os: OperatingSystem },
    
    #[error("Memory allocation failed: {0}")]
    MemoryAllocationFailed(String),
    
    #[error("System call failed: {0}")]
    SystemCallFailed(String),
}

/// Enhanced factory function to create the appropriate PAL for the current platform
pub fn create_platform_abstraction() -> Result<Arc<dyn PlatformAbstraction>, PlatformError> {
    // Detect platform capabilities first
    let platform_info = common::PlatformDetector::detect();
    
    cfg_if::cfg_if! {
        if #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {
            let pal = Arc::new(arm64::Arm64MacOSPal::new()?);
            log::info!("Initialized ARM64 macOS PAL with {} cores", 
                      pal.hardware_concurrency());
            Ok(pal)
        } else if #[cfg(all(target_arch = "aarch64", target_os = "linux"))] {
            let pal = Arc::new(arm64::Arm64LinuxPal::new()?);
            log::info!("Initialized ARM64 Linux PAL with {} cores", 
                      pal.hardware_concurrency());
            Ok(pal)
        } else if #[cfg(all(target_arch = "x86_64", target_os = "macos"))] {
            let pal = Arc::new(x86_64::X86_64MacOSPal::new()?);
            log::info!("Initialized x86_64 macOS PAL with {} cores", pal.hardware_concurrency());
            Ok(pal)
        } else if #[cfg(all(target_arch = "x86_64", target_os = "linux"))] {
            let pal = Arc::new(x86_64::X86_64LinuxPal::new()?);
            log::info!("Initialized x86_64 Linux PAL with {} cores", pal.hardware_concurrency());
            Ok(pal)
        } else if #[cfg(all(target_arch = "x86_64", target_os = "windows"))] {
            let pal = Arc::new(x86_64::X86_64WindowsPal::new()?);
            log::info!("Initialized x86_64 Windows PAL with {} cores", pal.hardware_concurrency());
            Ok(pal)
        } else if #[cfg(target_arch = "wasm32")] {
            let pal = Arc::new(wasm::WasmPal::new()?);
            log::info!("Initialized WebAssembly PAL");
            Ok(pal)
        } else {
            Err(PlatformError::UnsupportedPlatform {
                arch: platform_info.architecture,
                os: platform_info.operating_system,
            })
        }
    }
}

/// Initialize the platform abstraction layer with comprehensive setup
pub fn initialize_pal() -> Result<Arc<dyn PlatformAbstraction>, PlatformError> {
    let pal = create_platform_abstraction()?;
    
    // Initialize platform-specific components
    pal.initialize()?;
    
    log::info!("Platform initialized: {}", pal.platform_name());
    
    Ok(pal)
}

/// Get platform version information
pub fn get_platform_version_info(verbose: bool) -> String {
    common::VersionReporter::generate_version_report(verbose)
}

/// Get current platform statistics
pub fn get_platform_stats() -> Result<BasicStats, PlatformError> {
    let pal = create_platform_abstraction()?;
    Ok(pal.get_basic_stats())
}

fn detect_architecture() -> Architecture {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            Architecture::Arm64
        } else if #[cfg(target_arch = "x86_64")] {
            Architecture::X86_64
        } else if #[cfg(target_arch = "wasm32")] {
            Architecture::Wasm32
        } else {
            // Fallback detection
            Architecture::X86_64
        }
    }
}

fn detect_operating_system() -> OperatingSystem {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            OperatingSystem::MacOS
        } else if #[cfg(target_os = "linux")] {
            OperatingSystem::Linux
        } else if #[cfg(target_os = "windows")] {
            OperatingSystem::Windows
        } else if #[cfg(target_arch = "wasm32")] {
            // Distinguish between browser and standalone WASM runtime
            if cfg!(target_feature = "atomics") {
                OperatingSystem::WasmRuntime
            } else {
                OperatingSystem::Browser
            }
        } else {
            OperatingSystem::Linux
        }
    }
}

/// Alias for create_platform_abstraction for backward compatibility
pub fn detect_platform() -> Result<Arc<dyn PlatformAbstraction>, PlatformError> {
    create_platform_abstraction()
}
