//! Cryptographic functionality for parallel_processing

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;

/// Result type for crypto operations
/// Cryptographic operations handler
/// Parallel processing configuration
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    thread_count: usize,
    chunk_size: usize,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            thread_count: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
            chunk_size: 1024,
        }
    }
}

/// Initialize crypto processing
pub fn init_parallel_processing() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (parallel_processing) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_parallel_processing() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}
