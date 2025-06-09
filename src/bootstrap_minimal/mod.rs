//! Minimal CURSED Bootstrap Compiler Subset Implementation
//!
//! This module provides a working implementation of the bootstrap subset
//! that can be integrated once the main bootstrap module compilation issues
//! are resolved.

pub mod subset;
pub mod validator;
pub mod config;

pub use subset::BootstrapSubset;
pub use validator::SubsetValidator;
pub use config::BootstrapConfig;

/// Re-export the main components for easy access
pub use config::BootstrapConfigBuilder;
