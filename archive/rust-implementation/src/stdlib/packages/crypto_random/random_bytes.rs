//! Cryptographically secure random byte generation

use crate::error::CursedError;
use crate::stdlib::packages::CryptoError;

/// Result type for random byte operations
pub type RandomBytesResult<T> = Result<T, CryptoError>;

/// Generate cryptographically secure random bytes
pub fn random_bytes(size: usize) -> RandomBytesResult<Vec<u8>> {
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; size];
    rng.fill_bytes(&mut bytes);
    Ok(bytes)
}

/// Fill a buffer with cryptographically secure random bytes
pub fn fill_bytes(buffer: &mut [u8]) -> RandomBytesResult<()> {
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    rng.fill_bytes(buffer);
    Ok(())
}

/// Generate cryptographically secure random bytes using system entropy
pub fn secure_random_bytes(size: usize) -> RandomBytesResult<Vec<u8>> {
    use getrandom::getrandom;
    let mut bytes = vec![0u8; size];
    getrandom(&mut bytes).map_err(|e| CryptoError::Other(format!("Failed to get secure random bytes: {}", "placeholder")))?;
    Ok(bytes)
}

/// Fill a buffer with cryptographically secure random bytes using system entropy
pub fn secure_fill_bytes(buffer: &mut [u8]) -> RandomBytesResult<()> {
    use getrandom::getrandom;
use crate::stdlib::packages::CryptoError;
    getrandom(buffer).map_err(|e| CryptoError::Other(format!("Failed to get secure random bytes: {}", "placeholder")))?;
    Ok(())
}

/// Generate random bytes with specific entropy requirements
pub fn entropy_random_bytes(size: usize, min_entropy_bits: usize) -> RandomBytesResult<Vec<u8>> {
    // For now, use secure random bytes and assume they meet entropy requirements
    // In a real implementation, we would measure and verify entropy
    if min_entropy_bits > size * 8 {
        return Err(CryptoError::InvalidInput);
    }
    secure_random_bytes(size)
}

/// Test for proper initialization of random byte generation
pub fn test_random_bytes() -> RandomBytesResult<()> {
    let test_bytes = random_bytes(32)?;
    if test_bytes.len() != 32 {
        return Err(CryptoError::Other("Random byte generation test failed".to_string()));
    }
    
    // Check that bytes are not all zeros (extremely unlikely with good RNG)
    if test_bytes.iter().all(|&b| b == 0) {
        return Err(CryptoError::Other("Random bytes appear to be all zeros - RNG may be broken".to_string()));
    }
    
    Ok(())
}
