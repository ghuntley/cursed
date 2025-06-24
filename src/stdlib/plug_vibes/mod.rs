/// PlugVibes - Plugin system for CURSED programming language
/// 
/// Provides functionality for loading and using plugins to extend applications at runtime
/// with good vibes. Inspired by Go's plugin package but with enhanced features, improved
/// usability, and stronger safety guarantees.
/// 
/// Features:
/// - Dynamic plugin loading and unloading with hot reload support
/// - Plugin registry and management system with lifecycle control
/// - Security features including sandboxing and signature verification
/// - Version compatibility checking and dependency management
/// - Plugin hooks and extension points for host applications
/// - Plugin development tools and utilities
/// - Cross-platform compatibility (Linux, macOS, Windows)
/// - Thread-safe operations with comprehensive error handling
/// - LLVM integration for runtime plugin compilation

pub mod error;
pub mod plug;
pub mod registry;
pub mod manager;
pub mod sandbox;
pub mod version;
pub mod security;
pub mod hooks;
pub mod distribution;
pub mod development;
pub mod llvm_integration;
use crate::error::Error;

// Re-export core functionality
pub use error::*;
pub use plug::*;
pub use registry::*;
pub use manager::*;
pub use sandbox::*;
pub use version::*;
pub use security::*;
pub use hooks::*;
pub use distribution::*;
pub use development::*;
pub use llvm_integration::*;

#[cfg(test)]
pub mod tests;
