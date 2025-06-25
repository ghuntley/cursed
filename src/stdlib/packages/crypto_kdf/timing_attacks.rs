/// fr fr Timing attack resistance for KDF operations
/// 
/// This module provides utilities to prevent timing attacks in cryptographic
/// key derivation functions.

// use crate::stdlib::packages::crypto_kdf::KdfResult;

/// slay Constant-time comparison for KDF outputs
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (byte_a, byte_b) in a.iter().zip(b.iter()) {
        result |= byte_a ^ byte_b;
    }
    
    result == 0
}

/// slay Timing-safe KDF verification
pub fn timing_safe_verify(derived: &[u8], expected: &[u8]) -> KdfResult<bool> {
    Ok(constant_time_compare(derived, expected))
}
