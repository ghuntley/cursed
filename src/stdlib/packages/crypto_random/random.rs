use rand::{RngCore, rngs::OsRng, Rng};
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;

/// Cryptographically secure random number generation errors
#[derive(Debug, Clone)]
pub enum CryptoRandomError {
    /// System entropy source is unavailable
    EntropyUnavailable,
    /// Invalid parameter provided
    InvalidParameter(String),
    /// Random number generation failure
    GenerationFailed(String),
    /// Insufficient entropy available
    InsufficientEntropy,
}

impl std::fmt::Display for CryptoRandomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoRandomError::EntropyUnavailable => write!(f, "System entropy source is unavailable"),
            CryptoRandomError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            CryptoRandomError::GenerationFailed(msg) => write!(f, "Random generation failed: {}", msg),
            CryptoRandomError::InsufficientEntropy => write!(f, "Insufficient entropy available"),
        }
    }
}

impl std::error::Error for CryptoRandomError {}

pub type CryptoRandomResult<T> = Result<T, CryptoRandomError>;

/// Global entropy statistics for monitoring
static RNG_STATS: Lazy<Arc<Mutex<RngStatistics>>> = Lazy::new(|| {
    Arc::new(Mutex::new(RngStatistics::new()))
});

/// Statistics tracking for random number generation
#[derive(Debug, Clone)]
pub struct RngStatistics {
    pub bytes_generated: u64,
    pub generation_count: u64,
    pub errors_count: u64,
    pub entropy_sources: HashMap<String, u64>,
}

impl RngStatistics {
    pub fn new() -> Self {
        Self {
            bytes_generated: 0,
            generation_count: 0,
            errors_count: 0,
            entropy_sources: HashMap::new(),
        }
    }

    pub fn record_generation(&mut self, bytes: usize) {
        self.bytes_generated += bytes as u64;
        self.generation_count += 1;
        *self.entropy_sources.entry("OsRng".to_string()).or_insert(0) += 1;
    }

    pub fn record_error(&mut self) {
        self.errors_count += 1;
    }
}

/// Thread-safe cryptographically secure random number generator
pub struct SecureRng {
    rng: Mutex<OsRng>,
}

impl SecureRng {
    pub fn new() -> CryptoRandomResult<Self> {
        Ok(Self {
            rng: Mutex::new(OsRng),
        })
    }

    /// Fill buffer with cryptographically secure random bytes
    pub fn fill_bytes(&self, buffer: &mut [u8]) -> CryptoRandomResult<()> {
        if buffer.is_empty() {
            return Ok(());
        }

        let mut rng = self.rng.lock().map_err(|_| {
            CryptoRandomError::GenerationFailed("Failed to acquire RNG lock".to_string())
        })?;

        rng.try_fill_bytes(buffer).map_err(|e| {
            if let Ok(mut stats) = RNG_STATS.lock() {
                stats.record_error();
            }
            CryptoRandomError::GenerationFailed(format!("OS RNG failed: {}", e))
        })?;

        // Record successful generation
        if let Ok(mut stats) = RNG_STATS.lock() {
            stats.record_generation(buffer.len());
        }

        Ok(())
    }

    /// Generate random bytes of specified length
    pub fn generate_bytes(&self, length: usize) -> CryptoRandomResult<Vec<u8>> {
        if length == 0 {
            return Ok(Vec::new());
        }

        if length > 1024 * 1024 {
            return Err(CryptoRandomError::InvalidParameter(
                "Requested length exceeds maximum (1MB)".to_string()
            ));
        }

        let mut buffer = vec![0u8; length];
        self.fill_bytes(&mut buffer)?;
        Ok(buffer)
    }

    /// Generate a random u32
    pub fn next_u32(&self) -> CryptoRandomResult<u32> {
        let mut rng = self.rng.lock().map_err(|_| {
            CryptoRandomError::GenerationFailed("Failed to acquire RNG lock".to_string())
        })?;

        let value = rng.next_u32();

        if let Ok(mut stats) = RNG_STATS.lock() {
            stats.record_generation(4);
        }

        Ok(value)
    }

    /// Generate a random u64
    pub fn next_u64(&self) -> CryptoRandomResult<u64> {
        let mut rng = self.rng.lock().map_err(|_| {
            CryptoRandomError::GenerationFailed("Failed to acquire RNG lock".to_string())
        })?;

        let value = rng.next_u64();

        if let Ok(mut stats) = RNG_STATS.lock() {
            stats.record_generation(8);
        }

        Ok(value)
    }

    /// Generate a random boolean
    pub fn next_bool(&self) -> CryptoRandomResult<bool> {
        let mut rng = self.rng.lock().map_err(|_| {
            CryptoRandomError::GenerationFailed("Failed to acquire RNG lock".to_string())
        })?;

        let value = rng.gen::<bool>();

        if let Ok(mut stats) = RNG_STATS.lock() {
            stats.record_generation(1);
        }

        Ok(value)
    }

    /// Generate random integer in range [min, max)
    pub fn gen_range_u32(&self, min: u32, max: u32) -> CryptoRandomResult<u32> {
        if min >= max {
            return Err(CryptoRandomError::InvalidParameter(
                "min must be less than max".to_string()
            ));
        }

        let mut rng = self.rng.lock().map_err(|_| {
            CryptoRandomError::GenerationFailed("Failed to acquire RNG lock".to_string())
        })?;

        let value = rng.gen_range(min..max);

        if let Ok(mut stats) = RNG_STATS.lock() {
            stats.record_generation(4);
        }

        Ok(value)
    }
}

/// Global secure RNG instance
static GLOBAL_RNG: Lazy<SecureRng> = Lazy::new(|| {
    SecureRng::new().expect("Failed to initialize secure RNG")
});

/// Fill buffer with cryptographically secure random bytes
/// 
/// This function uses the operating system's cryptographically secure
/// random number generator to fill the provided buffer with random bytes.
/// 
/// # Arguments
/// * `buffer` - Mutable byte slice to fill with random data
/// 
/// # Returns
/// * `Ok(())` on success
/// * `Err(String)` if random generation fails
/// 
/// # Security
/// This function is cryptographically secure and suitable for:
/// - Key generation
/// - Nonce generation
/// - Salt generation
/// - Any security-critical randomness
pub fn fill_random(buffer: &mut [u8]) -> Result<(), String> {
    GLOBAL_RNG.fill_bytes(buffer)
        .map_err(|e| e.to_string())
}

/// Generate cryptographically secure random bytes
/// 
/// This function generates a vector of random bytes using the operating
/// system's cryptographically secure random number generator.
/// 
/// # Arguments
/// * `length` - Number of random bytes to generate
/// 
/// # Returns
/// * `Ok(Vec<u8>)` containing random bytes on success
/// * `Err(String)` if random generation fails or length is invalid
/// 
/// # Security
/// This function is cryptographically secure and suitable for:
/// - Key generation
/// - Token generation
/// - Challenge generation
/// - Any security-critical randomness
/// 
/// # Limits
/// Maximum length is 1MB (1,048,576 bytes) to prevent memory exhaustion
pub fn generate_random_bytes(length: usize) -> Result<Vec<u8>, String> {
    GLOBAL_RNG.generate_bytes(length)
        .map_err(|e| e.to_string())
}

/// Generate a cryptographically secure random u32
pub fn generate_random_u32() -> Result<u32, String> {
    GLOBAL_RNG.next_u32()
        .map_err(|e| e.to_string())
}

/// Generate a cryptographically secure random u64
pub fn generate_random_u64() -> Result<u64, String> {
    GLOBAL_RNG.next_u64()
        .map_err(|e| e.to_string())
}

/// Generate a cryptographically secure random boolean
pub fn generate_random_bool() -> Result<bool, String> {
    GLOBAL_RNG.next_bool()
        .map_err(|e| e.to_string())
}

/// Generate a random u32 in the specified range [min, max)
pub fn generate_random_range(min: u32, max: u32) -> Result<u32, String> {
    GLOBAL_RNG.gen_range_u32(min, max)
        .map_err(|e| e.to_string())
}

/// Generate a cryptographically secure random hex string
pub fn generate_random_hex(length: usize) -> Result<String, String> {
    let bytes = generate_random_bytes(length)?;
    Ok(hex::encode(bytes))
}

/// Generate a cryptographically secure random base64 string
pub fn generate_random_base64(length: usize) -> Result<String, String> {
    let bytes = generate_random_bytes(length)?;
    Ok(base64::encode(bytes))
}

/// Get statistics about random number generation
pub fn get_rng_statistics() -> Result<RngStatistics, String> {
    RNG_STATS.lock()
        .map(|stats| stats.clone())
        .map_err(|_| "Failed to acquire statistics lock".to_string())
}

/// Reset random number generation statistics
pub fn reset_rng_statistics() -> Result<(), String> {
    RNG_STATS.lock()
        .map(|mut stats| *stats = RngStatistics::new())
        .map_err(|_| "Failed to acquire statistics lock".to_string())
}

/// Test the entropy quality of the random number generator
/// Returns true if the RNG appears to be working correctly
pub fn test_entropy_quality() -> Result<bool, String> {
    const TEST_SIZE: usize = 1024;
    let data = generate_random_bytes(TEST_SIZE)?;
    
    // Basic entropy test: ensure we don't have too many repeated bytes
    let mut byte_counts = [0u32; 256];
    for &byte in &data {
        byte_counts[byte as usize] += 1;
    }
    
    // Check if any single byte value appears more than 25% of the time
    // This is a very basic test - in practice, more sophisticated tests would be used
    let max_count = byte_counts.iter().max().unwrap_or(&0);
    let threshold = TEST_SIZE / 4; // 25% threshold
    
    if *max_count > threshold as u32 {
        return Ok(false);
    }
    
    // Test that we don't have all zeros (our old stub behavior)
    let zero_count = byte_counts[0];
    if zero_count == TEST_SIZE as u32 {
        return Ok(false);
    }
    
    Ok(true)
}

/// Verify that the RNG is properly initialized and working
pub fn verify_rng_health() -> Result<(), String> {
    // Test basic functionality
    let _bytes = generate_random_bytes(32)?;
    let _u32_val = generate_random_u32()?;
    let _u64_val = generate_random_u64()?;
    let _bool_val = generate_random_bool()?;
    let _range_val = generate_random_range(1, 100)?;
    
    // Test entropy quality
    if !test_entropy_quality()? {
        return Err("RNG failed entropy quality test".to_string());
    }
    
    Ok(())
}
