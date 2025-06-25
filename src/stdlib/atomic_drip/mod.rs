use crate::error::{CursedError, Result as CursedResult};

/// Core atomic types module
pub mod core;
/// Generic atomic value container
pub mod value;
/// Memory ordering types and constants
pub mod memory_order;
/// Wait group for goroutine synchronization
pub mod wait_group;
/// Atomic bitfield operations
pub mod bitfield;
/// Atomic collections (queue, stack, counter)
pub mod collections;
/// Specialized atomic flags with extended operations
pub mod flags;

// Re-export all public types and functions
pub use core::*;
pub use value::*;
pub use memory_order::*;
pub use wait_group::*;
pub use bitfield::*;
pub use collections::*;
pub use flags::*;

/// atomic_drip module provides low-level atomic memory operations for synchronization across goroutines.
/// These operations ensure that concurrent modifications to shared memory are performed without race conditions.

/// Type alias for atomic operation results
pub type AtomicResult<T> = CursedResult<T>;

/// Common atomic operation errors
#[derive(Debug, Clone, PartialEq)]
pub enum AtomicError {
    /// Operation failed due to concurrent modification
    /// Invalid memory ordering specified
    /// Operation not supported on this platform
    /// Alignment error for 64-bit operations on 32-bit platforms
    /// General atomic operation failure
// impl std::fmt::Display for AtomicError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             AtomicError::ConcurrentModification => write!(f, "Atomic operation failed due to concurrent modification"),
//             AtomicError::InvalidMemoryOrder => write!(f, "Invalid memory ordering specified"),
//             AtomicError::UnsupportedOperation => write!(f, "Atomic operation not supported on this platform"),
//             AtomicError::AlignmentError => write!(f, "Alignment error for 64-bit atomic operation on 32-bit platform"),
//             AtomicError::OperationFailed(msg) => write!(f, "Atomic operation failed: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for AtomicError {}
// 
// impl From<AtomicError> for CursedError {
//     fn from(err: AtomicError) -> Self {
//         CursedError::Runtime(err.to_string())
//     }
// }

/// Helper function to create atomic operation error
pub fn atomic_error(msg: &str) -> CursedError {
    AtomicError::OperationFailed(msg.to_string()).into()
/// Helper function to create concurrent modification error
pub fn concurrent_modification_error() -> CursedError {
    AtomicError::ConcurrentModification.into()
/// Helper function to create alignment error
pub fn alignment_error() -> CursedError {
    AtomicError::AlignmentError.into()
/// Initialize atomic_drip module
pub fn init() -> CursedResult<()> {
    // Check platform capabilities and set up optimizations
    #[cfg(target_pointer_width = "32")]
    {
        // On 32-bit platforms, 64-bit atomics may require special handling
        log::debug!("atomic_drip: Initializing on 32-bit platform, 64-bit atomics may have alignment requirements");
    #[cfg(target_pointer_width = "64")]
    {
        log::debug!("atomic_drip: Initializing on 64-bit platform, all atomic operations supported");
    log::info!("atomic_drip module initialized successfully");
    Ok(())
