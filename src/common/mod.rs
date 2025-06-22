/// Common types and utilities used across the CURSED codebase
/// 
/// This module provides shared types, constants, and utilities that are used
/// throughout different parts of the CURSED compiler and runtime.

pub mod optimization_level;

// Re-export the canonical optimization level for easy access
pub use optimization_level::OptimizationLevel;

/// Version information for the CURSED compiler
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Target triple information
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
pub const TARGET_TRIPLE: &str = "x86_64-unknown-linux-gnu";

#[cfg(all(target_arch = "x86_64", target_os = "macos"))]
pub const TARGET_TRIPLE: &str = "x86_64-apple-darwin";

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
pub const TARGET_TRIPLE: &str = "x86_64-pc-windows-msvc";

#[cfg(not(any(
    all(target_arch = "x86_64", target_os = "linux"),
    all(target_arch = "x86_64", target_os = "macos"),
    all(target_arch = "x86_64", target_os = "windows")
)))]
pub const TARGET_TRIPLE: &str = "unknown-target";

/// Build timestamp
pub const BUILD_TIMESTAMP: &str = "unknown";

/// Common result type used throughout the codebase
pub type Result<T> = std::result::Result<T, crate::error::Error>;
