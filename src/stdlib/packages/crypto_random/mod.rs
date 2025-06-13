/// Cryptographically secure random number generation package
pub mod random;

// Re-export all random functionality
pub use random::*;

use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;

/// Initialize the crypto_random package with comprehensive functionality
pub fn init_crypto_random() -> AdvancedCryptoResult<()> {
    // Test that the random number generator is working properly
    let _ = random::init_rng()?;
    
    // Perform basic health check
    let test_bytes = random::random_bytes(32)?;
    if test_bytes.len() != 32 {
        return Err("Failed to generate test random bytes".into());
    }
    
    // Test entropy quality
    let quality_report = random::secure_test_quality(1000)?;
    if quality_report.is_empty() {
        return Err("Failed to generate entropy quality report".into());
    }
    
    println!("🔐 crypto_random package initialized - enterprise-grade secure random ready!");
    println!("📊 Entropy sources: {}", random::secure_entropy_info());
    
    Ok(())
}

/// Quick access to generate secure random bytes
pub fn generate_random_bytes(size: usize) -> AdvancedCryptoResult<Vec<u8>> {
    random::random_bytes(size)
}

/// Quick access to generate secure random number
pub fn generate_random_number() -> AdvancedCryptoResult<u64> {
    random::random_number()
}

/// Quick access to generate UUID
pub fn generate_uuid() -> AdvancedCryptoResult<String> {
    random::uuid()
}

/// Quick access to generate password
pub fn generate_password(length: usize) -> AdvancedCryptoResult<String> {
    random::password(length)
}

/// Quick access to generate API key
pub fn generate_api_key() -> AdvancedCryptoResult<String> {
    random::api_key()
}

/// Quick access to generate nonce
pub fn generate_nonce() -> AdvancedCryptoResult<String> {
    random::nonce()
}

/// Get comprehensive entropy and security information
pub fn get_entropy_info() -> String {
    random::secure_entropy_info()
}

/// Test randomness quality
pub fn test_randomness_quality(sample_size: usize) -> AdvancedCryptoResult<String> {
    random::secure_test_quality(sample_size)
}

/// Force reseed of all random generators
pub fn reseed_generators() -> AdvancedCryptoResult<()> {
    random::secure_reseed()
}
