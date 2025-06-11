/// System utilities module for CURSED
/// 
/// This module provides system-level functionality.
/// Process management is available through the main stdlib::process module.

pub mod error;

// Re-export error types for compatibility
pub use error::{
    ProcessError, ProcessResult, ProcessErrorKind,
    spawn_error, wait_error, signal_error, permission_error,
};
