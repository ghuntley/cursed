// Standard Library Module for CURSED
//
// This module provides the standard library functionality including:
// - Basic data types and operations
// - IO operations
// - String manipulation
// - Collections
// - Error handling
// - Testing utilities
// - Networking and web functionality
// - Cryptographic operations
// - Asynchronous support

// Core stdlib modules
pub mod string;
pub mod math;
pub mod mathz;
pub mod stringz;
pub mod vibez;

// TODO: Enable these modules once they are implemented
// pub mod prelude;
// pub mod collections;
// pub mod io;
// pub mod error;

// Conditional modules based on features
#[cfg(feature = "web")]
pub mod web_vibez;

#[cfg(feature = "crypto")]
pub mod crypto;

#[cfg(feature = "pqc")]
pub mod crypto_pqc;

#[cfg(feature = "async")]
pub mod async_runtime;

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "testing")]
pub mod testing;

// Core types for stdlib
pub use crate::common_types::*;

/// Web framework module (placeholder)
#[cfg(feature = "web")]
pub mod web_vibez {
    pub use crate::common_types::*;
    
    pub struct SecurityContext {
        pub auth_level: u8,
        pub session_id: String,
    }
    
    impl Default for SecurityContext {
        fn default() -> Self {
            Self {
                auth_level: 0,
                session_id: "anonymous".to_string(),
            }
        }
    }
}

/// Cryptographic operations (placeholder)
#[cfg(feature = "crypto")]  
pub mod crypto {
    pub struct CryptoContext {
        pub algorithm: String,
        pub key_size: u32,
    }
    
    impl Default for CryptoContext {
        fn default() -> Self {
            Self {
                algorithm: "AES-256".to_string(),
                key_size: 256,
            }
        }
    }
}

/// Post-quantum cryptography (placeholder)
#[cfg(feature = "pqc")]
pub mod crypto_pqc {
    pub struct PqcContext {
        pub algorithm: String,
        pub security_level: u8,
    }
    
    impl Default for PqcContext {
        fn default() -> Self {
            Self {
                algorithm: "Kyber".to_string(),
                security_level: 3,
            }
        }
    }
}

/// Asynchronous runtime support (placeholder)
#[cfg(feature = "async")]
pub mod async_runtime {
    pub struct AsyncRuntime {
        pub worker_threads: usize,
        pub max_blocking_threads: usize,
    }
    
    impl Default for AsyncRuntime {
        fn default() -> Self {
            Self {
                worker_threads: num_cpus::get(),
                max_blocking_threads: 512,
            }
        }
    }
}

/// Synchronization primitives (placeholder)
#[cfg(feature = "sync")]
pub mod sync {
    pub struct SyncPrimitive {
        pub sync_type: String,
        pub max_waiters: u32,
    }
    
    pub enum SyncError {
        General(String),
        Timeout,
        Deadlock,
    }
    
    impl std::fmt::Display for SyncError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SyncError::General(msg) => write!(f, "Sync error: {}", msg),
                SyncError::Timeout => write!(f, "Timeout error"),
                SyncError::Deadlock => write!(f, "Deadlock detected"),
            }
        }
    }
    
    impl std::error::Error for SyncError {}
    
    pub mod error {
        use super::SyncError;
        
        pub fn thread_pool_error(msg: &str) -> SyncError {
            SyncError::General(format!("Thread pool error: {}", msg))
        }
        
        pub fn timeout_error(msg: &str) -> SyncError {
            SyncError::General(format!("Timeout error: {}", msg))
        }
        
        pub fn channel_error(msg: &str) -> SyncError {
            SyncError::General(format!("Channel error: {}", msg))
        }
    }
}

/// Testing utilities (placeholder)
#[cfg(feature = "testing")]
pub mod testing {
    pub struct TestContext {
        pub test_name: String,
        pub timeout_ms: u64,
    }
    
    pub enum TestResult {
        Passed,
        Failed(String),
        Skipped(String),
    }
    
    impl Default for TestContext {
        fn default() -> Self {
            Self {
                test_name: "unknown".to_string(),
                timeout_ms: 5000,
            }
        }
    }
}

// Default implementations for when features are disabled
#[cfg(not(feature = "web"))]
pub mod web_vibez {
    pub struct SecurityContext;
    impl Default for SecurityContext {
        fn default() -> Self {
            Self
        }
    }
}

#[cfg(not(feature = "crypto"))]
pub mod crypto {
    pub struct CryptoContext;
    impl Default for CryptoContext {
        fn default() -> Self {
            Self
        }
    }
}

#[cfg(not(feature = "pqc"))]
pub mod crypto_pqc {
    pub struct PqcContext;
    impl Default for PqcContext {
        fn default() -> Self {
            Self
        }
    }
}

#[cfg(not(feature = "async"))]
pub mod async_runtime {
    pub struct AsyncRuntime;
    impl Default for AsyncRuntime {
        fn default() -> Self {
            Self
        }
    }
}

#[cfg(not(feature = "sync"))]
pub mod sync {
    pub struct SyncPrimitive;
    #[derive(Debug)]
    pub enum SyncError {
        General(String),
    }
    
    impl std::fmt::Display for SyncError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SyncError::General(msg) => write!(f, "Sync error: {}", msg),
            }
        }
    }
    
    impl std::error::Error for SyncError {}
    
    pub mod error {
        use super::SyncError;
        
        pub fn thread_pool_error(msg: &str) -> SyncError {
            SyncError::General(format!("Thread pool error: {}", msg))
        }
        
        pub fn timeout_error(msg: &str) -> SyncError {
            SyncError::General(format!("Timeout error: {}", msg))
        }
        
        pub fn channel_error(msg: &str) -> SyncError {
            SyncError::General(format!("Channel error: {}", msg))
        }
    }
}

#[cfg(not(feature = "testing"))]
pub mod testing {
    pub struct TestContext;
    pub enum TestResult {
        Passed,
        Failed(String),
    }
    
    impl Default for TestContext {
        fn default() -> Self {
            Self
        }
    }
}
