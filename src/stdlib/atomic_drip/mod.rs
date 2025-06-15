use crate::error::{CursedError, Result as CursedResult};

/// Core atomic types module
pub mod core;
/// Generic atomic value container
pub mod value;
/// Memory ordering types and constants
pub mod memory_order;

// Re-export all public types and functions
pub use core::*;
pub use value::*;
pub use memory_order::*;

/// atomic_drip module provides low-level atomic memory operations for synchronization across goroutines.
/// These operations ensure that concurrent modifications to shared memory are performed without race conditions.

/// Type alias for atomic operation results
pub type AtomicResult<T> = CursedResult<T>;

/// Common atomic operation errors
#[derive(Debug, Clone, PartialEq)]
pub enum AtomicError {
    /// Operation failed due to concurrent modification
    ConcurrentModification,
    /// Invalid memory ordering specified
    InvalidMemoryOrder,
    /// Operation not supported on this platform
    UnsupportedOperation,
    /// Alignment error for 64-bit operations on 32-bit platforms
    AlignmentError,
    /// General atomic operation failure
    OperationFailed(String),
}

impl std::fmt::Display for AtomicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtomicError::ConcurrentModification => write!(f, "Atomic operation failed due to concurrent modification"),
            AtomicError::InvalidMemoryOrder => write!(f, "Invalid memory ordering specified"),
            AtomicError::UnsupportedOperation => write!(f, "Atomic operation not supported on this platform"),
            AtomicError::AlignmentError => write!(f, "Alignment error for 64-bit atomic operation on 32-bit platform"),
            AtomicError::OperationFailed(msg) => write!(f, "Atomic operation failed: {}", msg),
        }
    }
}

impl std::error::Error for AtomicError {}

impl From<AtomicError> for CursedError {
    fn from(err: AtomicError) -> Self {
        CursedError::Runtime(err.to_string())
    }
}

/// Helper function to create atomic operation error
pub fn atomic_error(msg: &str) -> CursedError {
    AtomicError::OperationFailed(msg.to_string()).into()
}

/// Helper function to create concurrent modification error
pub fn concurrent_modification_error() -> CursedError {
    AtomicError::ConcurrentModification.into()
}

/// Helper function to create alignment error
pub fn alignment_error() -> CursedError {
    AtomicError::AlignmentError.into()
}

/// Initialize atomic_drip module
pub fn init() -> CursedResult<()> {
    // Check platform capabilities and set up optimizations
    #[cfg(target_pointer_width = "32")]
    {
        // On 32-bit platforms, 64-bit atomics may require special handling
        log::debug!("atomic_drip: Initializing on 32-bit platform, 64-bit atomics may have alignment requirements");
    }
    
    #[cfg(target_pointer_width = "64")]
    {
        log::debug!("atomic_drip: Initializing on 64-bit platform, all atomic operations supported");
    }
    
    log::info!("atomic_drip module initialized successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_initialization() {
        assert!(init().is_ok());
    }

    #[test]
    fn test_error_conversion() {
        let atomic_err = AtomicError::ConcurrentModification;
        let cursed_err: CursedError = atomic_err.into();
        assert!(matches!(cursed_err, CursedError::Runtime(_)));
    }

    #[test]
    fn test_error_helpers() {
        let err = atomic_error("test error");
        assert!(matches!(err, CursedError::Runtime(_)));
        
        let err = concurrent_modification_error();
        assert!(matches!(err, CursedError::Runtime(_)));
        
        let err = alignment_error();
        assert!(matches!(err, CursedError::Runtime(_)));
    }
}
