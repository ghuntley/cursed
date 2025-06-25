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
    ThreadError { message: String, thread_id: Option<String> },
    
    /// Lock acquisition timeout or failure
    LockError { lock_type: String, operation: String, timeout: Option<Duration> },
    
    /// Operation timed out
    TimeoutError { operation: String, duration: Duration },
    
    /// Deadlock detected
    DeadlockError { thread_ids: Vec<String>, resource: String },
    
    /// Channel operation error
    ChannelError { operation: String, message: String },
    
    /// Thread pool error
    ThreadPoolError { pool_id: String, message: String },
    
    /// Atomic operation error
    AtomicError { operation: String, message: String },
    
    /// Barrier synchronization error
    BarrierError { expected: usize, actual: usize },
    
    /// Semaphore operation error
    SemaphoreError { operation: String, permits: usize },
    
    /// Thread-local storage error
    ThreadLocalError { key: String, message: String },
    
    /// Resource exhaustion error
    ResourceExhausted { resource_type: String, limit: usize },
    
    /// Invalid state for operation
    InvalidState { expected: String, actual: String },
    
    /// Permission denied for operation
    PermissionDenied { operation: String },
    
    /// System-level error
    SystemError { code: i32, message: String },
    
    /// General sync error
    General { message: String },
}

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
        message: message.to_string(),
        thread_id: None,
    }
}

pub fn thread_error_with_id(message: &str, thread_id: &str) -> SyncError {
    SyncError::ThreadError {
        message: message.to_string(),
        thread_id: Some(thread_id.to_string()),
    }
}

pub fn lock_error(lock_type: &str, operation: &str) -> SyncError {
    SyncError::LockError {
        lock_type: lock_type.to_string(),
        operation: operation.to_string(),
        timeout: None,
    }
}

pub fn lock_timeout_error(lock_type: &str, operation: &str, timeout: Duration) -> SyncError {
    SyncError::LockError {
        lock_type: lock_type.to_string(),
        operation: operation.to_string(),
        timeout: Some(timeout),
    }
}

pub fn timeout_error(operation: &str, duration: Duration) -> SyncError {
    SyncError::TimeoutError {
        operation: operation.to_string(),
        duration,
    }
}

pub fn deadlock_error(thread_ids: Vec<String>, resource: &str) -> SyncError {
    SyncError::DeadlockError {
        thread_ids,
        resource: resource.to_string(),
    }
}

pub fn channel_error(operation: &str, message: &str) -> SyncError {
    SyncError::ChannelError {
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

pub fn thread_pool_error(pool_id: &str, message: &str) -> SyncError {
    SyncError::ThreadPoolError {
        pool_id: pool_id.to_string(),
        message: message.to_string(),
    }
}

pub fn atomic_error(operation: &str, message: &str) -> SyncError {
    SyncError::AtomicError {
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

pub fn barrier_error(expected: usize, actual: usize) -> SyncError {
    SyncError::BarrierError { expected, actual }
}

pub fn semaphore_error(operation: &str, permits: usize) -> SyncError {
    SyncError::SemaphoreError {
        operation: operation.to_string(),
        permits,
    }
}

pub fn thread_local_error(key: &str, message: &str) -> SyncError {
    SyncError::ThreadLocalError {
        key: key.to_string(),
        message: message.to_string(),
    }
}

pub fn resource_exhausted_error(resource_type: &str, limit: usize) -> SyncError {
    SyncError::ResourceExhausted {
        resource_type: resource_type.to_string(),
        limit,
    }
}

pub fn invalid_state_error(expected: &str, actual: &str) -> SyncError {
    SyncError::InvalidState {
        expected: expected.to_string(),
        actual: actual.to_string(),
    }
}

pub fn permission_denied_error(operation: &str) -> SyncError {
    SyncError::PermissionDenied {
        operation: operation.to_string(),
    }
}

pub fn system_error(code: i32, message: &str) -> SyncError {
    SyncError::SystemError {
        code,
        message: message.to_string(),
    }
}

pub fn general_error(message: &str) -> SyncError {
    SyncError::General {
        message: message.to_string(),
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

