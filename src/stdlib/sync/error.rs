use crate::error::CursedError;
/// CursedError handling for sync operations
use std::fmt;
use std::time::Duration;

/// Result type for sync operations
pub type SyncResult<T> = std::result::Result<T, SyncError>;

/// Comprehensive error types for synchronization operations
#[derive(Debug, Clone, PartialEq)]
pub enum SyncError {
    /// Thread creation or management error
    
    /// Lock acquisition timeout or failure
    
    /// Operation timed out
    
    /// Deadlock detected
    
    /// Channel operation error
    
    /// Thread pool error
    
    /// Atomic operation error
    
    /// Barrier synchronization error
    
    /// Semaphore operation error
    
    /// Thread-local storage error
    
    /// Resource exhaustion error
    
    /// Invalid state for operation
    
    /// Permission denied for operation
    
    /// System-level error
    
    /// General sync error
// impl fmt::Display for SyncError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             SyncError::ThreadError { message, thread_id } => {
//                 if let Some(id) = thread_id {
//                     write!(f, "Thread error in {}: {}", id, message)
//                 } else {
//                     write!(f, "Thread error: {}", message)
//                 }
//             }
//             SyncError::LockError { lock_type, operation, timeout } => {
//                 if let Some(duration) = timeout {
//                     write!(f, "{} lock {} failed after timeout of {:?}", lock_type, operation, duration)
//                 } else {
//                     write!(f, "{} lock {} failed", lock_type, operation)
//                 }
//             }
//             SyncError::TimeoutError { operation, duration } => {
//                 write!(f, "Operation '{}' timed out after {:?}", operation, duration)
//             }
//             SyncError::DeadlockError { thread_ids, resource } => {
//                 write!(f, "Deadlock detected on resource '{}' involving threads: {:?}", resource, thread_ids)
//             }
//             SyncError::ChannelError { operation, message } => {
//                 write!(f, "Channel {} error: {}", operation, message)
//             }
//             SyncError::ThreadPoolError { pool_id, message } => {
//                 write!(f, "Thread pool '{}' error: {}", pool_id, message)
//             }
//             SyncError::AtomicError { operation, message } => {
//                 write!(f, "Atomic {} error: {}", operation, message)
//             }
//             SyncError::BarrierError { expected, actual } => {
//                 write!(f, "Barrier error: expected {} threads, got {}", expected, actual)
//             }
//             SyncError::SemaphoreError { operation, permits } => {
//                 write!(f, "Semaphore {} error with {} permits", operation, permits)
//             }
//             SyncError::ThreadLocalError { key, message } => {
//                 write!(f, "Thread-local storage error for key '{}': {}", key, message)
//             }
//             SyncError::ResourceExhausted { resource_type, limit } => {
//                 write!(f, "Resource exhausted: {} limit of {} exceeded", resource_type, limit)
//             }
//             SyncError::InvalidState { expected, actual } => {
//                 write!(f, "Invalid state: expected '{}', found '{}'", expected, actual)
//             }
//             SyncError::PermissionDenied { operation } => {
//                 write!(f, "Permission denied for operation: {}", operation)
//             }
//             SyncError::SystemError { code, message } => {
//                 write!(f, "System error (code {}): {}", code, message)
//             }
//             SyncError::General { message } => {
//                 write!(f, "Sync error: {}", message)
//             }
//         }
//     }
// }

impl StdError for SyncError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

// Helper functions for creating specific error types
pub fn thread_error(message: &str) -> SyncError {
    SyncError::ThreadError {
    }
}

pub fn thread_error_with_id(message: &str, thread_id: &str) -> SyncError {
    SyncError::ThreadError {
    }
}

pub fn lock_error(lock_type: &str, operation: &str) -> SyncError {
    SyncError::LockError {
    }
}

pub fn lock_timeout_error(lock_type: &str, operation: &str, timeout: Duration) -> SyncError {
    SyncError::LockError {
    }
}

pub fn timeout_error(operation: &str, duration: Duration) -> SyncError {
    SyncError::TimeoutError {
    }
}

pub fn deadlock_error(thread_ids: Vec<String>, resource: &str) -> SyncError {
    SyncError::DeadlockError {
    }
}

pub fn channel_error(operation: &str, message: &str) -> SyncError {
    SyncError::ChannelError {
    }
}

pub fn thread_pool_error(pool_id: &str, message: &str) -> SyncError {
    SyncError::ThreadPoolError {
    }
}

pub fn atomic_error(operation: &str, message: &str) -> SyncError {
    SyncError::AtomicError {
    }
}

pub fn barrier_error(expected: usize, actual: usize) -> SyncError {
    SyncError::BarrierError { expected, actual }
}

pub fn semaphore_error(operation: &str, permits: usize) -> SyncError {
    SyncError::SemaphoreError {
    }
}

pub fn thread_local_error(key: &str, message: &str) -> SyncError {
    SyncError::ThreadLocalError {
    }
}

pub fn resource_exhausted_error(resource_type: &str, limit: usize) -> SyncError {
    SyncError::ResourceExhausted {
    }
}

pub fn invalid_state_error(expected: &str, actual: &str) -> SyncError {
    SyncError::InvalidState {
    }
}

pub fn permission_denied_error(operation: &str) -> SyncError {
    SyncError::PermissionDenied {
    }
}

pub fn system_error(code: i32, message: &str) -> SyncError {
    SyncError::SystemError {
    }
}

pub fn general_error(message: &str) -> SyncError {
    SyncError::General {
    }
}

// Conversion from standard library errors
// impl From<std::io::Error> for SyncError {
//     fn from(err: std::io::Error) -> Self {
//         system_error(err.raw_os_error().unwrap_or(-1), &err.to_string())
//     }
// }

impl<T> From<std::sync::PoisonError<T>> for SyncError {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        lock_error("poisoned", &err.to_string())
    }
}

impl From<std::sync::mpsc::RecvError> for SyncError {
    fn from(err: std::sync::mpsc::RecvError) -> Self {
        channel_error("receive", &err.to_string())
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for SyncError {
    fn from(err: std::sync::mpsc::SendError<T>) -> Self {
        channel_error("send", &format!("Send error: {}", err))
    }
}

impl From<std::sync::mpsc::RecvTimeoutError> for SyncError {
    fn from(err: std::sync::mpsc::RecvTimeoutError) -> Self {
        match err {
            std::sync::mpsc::RecvTimeoutError::Timeout => {
                timeout_error("channel receive", Duration::from_secs(0))
            }
            std::sync::mpsc::RecvTimeoutError::Disconnected => {
                channel_error("receive", "channel disconnected")
            }
        }
    }
}

