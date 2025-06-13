/// fr fr Nonce generation for cryptographic operations
use super::errors::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

/// Cryptographically secure nonce generator
#[derive(Debug)]
pub struct NonceGenerator {
    rng: Mutex<ChaCha20Rng>,
    counter: AtomicU64,
    instance_id: u64,
}

/// Secure nonce container
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecureNonce {
    data: Vec<u8>,
    generation_time: u64,
    instance_id: u64,
    counter: u64,
}

/// Counter-based nonce generation mode
#[derive(Debug, Clone)]
pub struct NonceCounterMode {
    base_value: Vec<u8>,
    counter: AtomicU64,
    max_count: u64,
}

/// Random nonce generation mode  
#[derive(Debug, Clone)]
pub struct NonceRandomMode {
    entropy_source: NonceEntropySource,
}

/// Entropy sources for nonce generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonceEntropySource {
    SystemRandom,
    ChaCha20Rng,
    CombinedSources,
}

/// Nonce generation error types
#[derive(Debug, Clone, PartialEq)]
pub enum NonceError {
    InsufficientEntropy(String),
    CounterOverflow,
    InvalidSize(String),
    GenerationFailed(String),
    UniquenessBreach(String),
}

impl std::fmt::Display for NonceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NonceError::InsufficientEntropy(msg) => write!(f, "Insufficient entropy: {}", msg),
            NonceError::CounterOverflow => write!(f, "Counter overflow"),
            NonceError::InvalidSize(msg) => write!(f, "Invalid size: {}", msg),
            NonceError::GenerationFailed(msg) => write!(f, "Generation failed: {}", msg),
            NonceError::UniquenessBreach(msg) => write!(f, "Uniqueness breach: {}", msg),
        }
    }
}

impl std::error::Error for NonceError {}

impl From<NonceError> for AdvancedCryptoError {
    fn from(err: NonceError) -> Self {
        AdvancedCryptoError::NonceGenerationFailed(err.to_string())
    }
}

impl NonceGenerator {
    /// Create new nonce generator with system entropy
    pub fn new() -> AdvancedCryptoResult<Self> {
        let mut entropy = [0u8; 32];
        getrandom::getrandom(&mut entropy)
            .map_err(|e| AdvancedCryptoError::NonceGenerationFailed(format!("System entropy failed: {}", e)))?;
        
        let rng = ChaCha20Rng::from_seed(entropy);
        let instance_id = Self::generate_instance_id();
        
        Ok(Self {
            rng: Mutex::new(rng),
            counter: AtomicU64::new(0),
            instance_id,
        })
    }
    
    /// Create nonce generator with specific seed (for testing)
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let rng = ChaCha20Rng::from_seed(seed);
        let instance_id = Self::generate_instance_id();
        
        Self {
            rng: Mutex::new(rng),
            counter: AtomicU64::new(0),
            instance_id,
        }
    }
    
    /// Generate secure nonce of specified size
    pub fn generate_nonce(&self, size: usize) -> AdvancedCryptoResult<SecureNonce> {
        if size == 0 || size > 1024 {
            return Err(AdvancedCryptoError::InvalidParameters(
                format!("Invalid nonce size: {} (must be 1-1024 bytes)", size)
            ));
        }
        
        let mut data = vec![0u8; size];
        self.fill_random(&mut data)?;
        
        let counter = self.counter.fetch_add(1, Ordering::SeqCst);
        let generation_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        Ok(SecureNonce {
            data,
            generation_time,
            instance_id: self.instance_id,
            counter,
        })
    }
    
    /// Fill buffer with cryptographically secure random bytes
    pub fn fill_random(&self, buffer: &mut [u8]) -> AdvancedCryptoResult<()> {
        let mut rng = self.rng.lock()
            .map_err(|_| AdvancedCryptoError::InternalError("RNG lock poisoned".to_string()))?;
        
        rng.fill_bytes(buffer);
        Ok(())
    }
    
    /// Generate nonce with timestamp and counter for uniqueness
    pub fn generate_timestamped_nonce(&self, size: usize) -> AdvancedCryptoResult<SecureNonce> {
        if size < 12 {
            return Err(AdvancedCryptoError::InvalidParameters(
                "Timestamped nonce requires at least 12 bytes".to_string()
            ));
        }
        
        let mut data = vec![0u8; size];
        
        // Add timestamp (8 bytes)
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        data[0..8].copy_from_slice(&timestamp.to_le_bytes());
        
        // Add counter (4 bytes)
        let counter = self.counter.fetch_add(1, Ordering::SeqCst);
        data[8..12].copy_from_slice(&(counter as u32).to_le_bytes());
        
        // Fill remaining with random data
        if size > 12 {
            self.fill_random(&mut data[12..])?;
        }
        
        Ok(SecureNonce {
            data,
            generation_time: timestamp,
            instance_id: self.instance_id,
            counter,
        })
    }
    
    /// Generate counter-based nonce
    pub fn generate_counter_nonce(&self, base: &[u8], counter_value: u64) -> AdvancedCryptoResult<SecureNonce> {
        if base.len() < 8 {
            return Err(AdvancedCryptoError::InvalidParameters(
                "Base value must be at least 8 bytes".to_string()
            ));
        }
        
        let mut data = base.to_vec();
        let counter_bytes = counter_value.to_le_bytes();
        
        // XOR counter into the first 8 bytes
        for (i, &counter_byte) in counter_bytes.iter().enumerate() {
            data[i] ^= counter_byte;
        }
        
        let generation_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        Ok(SecureNonce {
            data,
            generation_time,
            instance_id: self.instance_id,
            counter: counter_value,
        })
    }
    
    /// Generate unique instance ID
    fn generate_instance_id() -> u64 {
        let mut bytes = [0u8; 8];
        getrandom::getrandom(&mut bytes).unwrap_or_else(|_| {
            // Fallback to timestamp if getrandom fails
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64;
            bytes = timestamp.to_le_bytes();
        });
        
        u64::from_le_bytes(bytes)
    }
    
    /// Get current counter value
    pub fn current_counter(&self) -> u64 {
        self.counter.load(Ordering::Acquire)
    }
    
    /// Reset counter (use with caution)
    pub fn reset_counter(&self) {
        self.counter.store(0, Ordering::Release);
    }
    
    /// Get instance ID
    pub fn instance_id(&self) -> u64 {
        self.instance_id
    }
}

impl SecureNonce {
    /// Create new secure nonce from bytes
    pub fn from_bytes(data: Vec<u8>) -> Self {
        let generation_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        Self {
            data,
            generation_time,
            instance_id: 0,
            counter: 0,
        }
    }
    
    /// Get nonce data as slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
    
    /// Get nonce data as vec
    pub fn to_vec(&self) -> Vec<u8> {
        self.data.clone()
    }
    
    /// Get nonce length
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if nonce is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Get generation timestamp
    pub fn generation_time(&self) -> u64 {
        self.generation_time
    }
    
    /// Get instance ID
    pub fn instance_id(&self) -> u64 {
        self.instance_id
    }
    
    /// Get counter value
    pub fn counter(&self) -> u64 {
        self.counter
    }
    
    /// Check if nonce is fresh (generated within specified duration)
    pub fn is_fresh(&self, max_age_seconds: u64) -> bool {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        let age_nanos = current_time.saturating_sub(self.generation_time);
        let age_seconds = age_nanos / 1_000_000_000;
        
        age_seconds <= max_age_seconds
    }
    
    /// Verify nonce uniqueness properties
    pub fn verify_uniqueness(&self, other: &SecureNonce) -> bool {
        // Different if any uniqueness property differs
        self.instance_id != other.instance_id || 
        self.counter != other.counter ||
        self.generation_time != other.generation_time ||
        self.data != other.data
    }
}

impl NonceCounterMode {
    /// Create new counter mode with base value
    pub fn new(base_value: Vec<u8>, max_count: u64) -> AdvancedCryptoResult<Self> {
        if base_value.len() < 8 {
            return Err(AdvancedCryptoError::InvalidParameters(
                "Base value must be at least 8 bytes".to_string()
            ));
        }
        
        Ok(Self {
            base_value,
            counter: AtomicU64::new(0),
            max_count,
        })
    }
    
    /// Generate next nonce in sequence
    pub fn next_nonce(&self) -> AdvancedCryptoResult<SecureNonce> {
        let current = self.counter.fetch_add(1, Ordering::SeqCst);
        
        if current >= self.max_count {
            return Err(NonceError::CounterOverflow.into());
        }
        
        let mut data = self.base_value.clone();
        let counter_bytes = current.to_le_bytes();
        
        // XOR counter into the first 8 bytes
        for (i, &counter_byte) in counter_bytes.iter().enumerate() {
            data[i] ^= counter_byte;
        }
        
        let generation_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        Ok(SecureNonce {
            data,
            generation_time,
            instance_id: 0,
            counter: current,
        })
    }
    
    /// Get current counter value
    pub fn current_counter(&self) -> u64 {
        self.counter.load(Ordering::Acquire)
    }
    
    /// Check if counter can generate more nonces
    pub fn can_generate_more(&self) -> bool {
        self.counter.load(Ordering::Acquire) < self.max_count
    }
}

impl NonceRandomMode {
    /// Create new random mode with entropy source
    pub fn new(entropy_source: NonceEntropySource) -> Self {
        Self { entropy_source }
    }
    
    /// Generate random nonce
    pub fn generate_nonce(&self, size: usize) -> AdvancedCryptoResult<SecureNonce> {
        let mut data = vec![0u8; size];
        
        match self.entropy_source {
            NonceEntropySource::SystemRandom => {
                getrandom::getrandom(&mut data)
                    .map_err(|e| AdvancedCryptoError::NonceGenerationFailed(
                        format!("System random failed: {}", e)
                    ))?;
            },
            NonceEntropySource::ChaCha20Rng => {
                let mut rng = ChaCha20Rng::from_entropy();
                rng.fill_bytes(&mut data);
            },
            NonceEntropySource::CombinedSources => {
                // Use both system random and ChaCha20
                let mut sys_data = vec![0u8; size];
                getrandom::getrandom(&mut sys_data)
                    .map_err(|e| AdvancedCryptoError::NonceGenerationFailed(
                        format!("System random failed: {}", e)
                    ))?;
                
                let mut rng = ChaCha20Rng::from_entropy();
                rng.fill_bytes(&mut data);
                
                // XOR both sources
                for (i, &sys_byte) in sys_data.iter().enumerate() {
                    data[i] ^= sys_byte;
                }
            },
        }
        
        Ok(SecureNonce::from_bytes(data))
    }
}

/// Nonce utilities and validation
pub struct NonceUtils;

impl NonceUtils {
    /// Validate nonce size for specific algorithm
    pub fn validate_nonce_size(algorithm: &str, size: usize) -> AdvancedCryptoResult<()> {
        let expected = match algorithm {
            "ChaCha20" | "ChaCha20-Poly1305" => 12,
            "AES-GCM" => 12,
            "AES-CTR" => 16,
            "XSalsa20" => 24,
            _ => return Err(AdvancedCryptoError::UnsupportedAlgorithm(algorithm.to_string())),
        };
        
        if size != expected {
            return Err(AdvancedCryptoError::InvalidNonce(
                format!("{} requires {}-byte nonce, got {}", algorithm, expected, size)
            ));
        }
        
        Ok(())
    }
    
    /// Check nonce collision
    pub fn check_collision(nonces: &[SecureNonce]) -> bool {
        for i in 0..nonces.len() {
            for j in (i + 1)..nonces.len() {
                if !nonces[i].verify_uniqueness(&nonces[j]) {
                    return true; // Collision found
                }
            }
        }
        false
    }
    
    /// Generate nonce with specific pattern
    pub fn generate_patterned_nonce(pattern: &[u8], random_bytes: usize) -> AdvancedCryptoResult<SecureNonce> {
        if pattern.len() + random_bytes > 256 {
            return Err(AdvancedCryptoError::InvalidParameters(
                "Pattern + random bytes too large".to_string()
            ));
        }
        
        let mut data = pattern.to_vec();
        let mut random_data = vec![0u8; random_bytes];
        getrandom::getrandom(&mut random_data)
            .map_err(|e| AdvancedCryptoError::NonceGenerationFailed(
                format!("Random generation failed: {}", e)
            ))?;
        
        data.extend_from_slice(&random_data);
        Ok(SecureNonce::from_bytes(data))
    }
}

/// Constants for nonce generation
pub const NONCE_UNIQUENESS_GUARANTEE: bool = true;
pub const MAX_NONCE_SIZE: usize = 1024;
pub const MIN_NONCE_SIZE: usize = 1;
pub const DEFAULT_NONCE_SIZE: usize = 12;
pub const TIMESTAMP_NONCE_MIN_SIZE: usize = 12;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_nonce_generator_creation() {
        let generator = NonceGenerator::new();
        assert!(generator.is_ok());
        
        let generator = generator.unwrap();
        assert_eq!(generator.current_counter(), 0);
        assert_ne!(generator.instance_id(), 0);
    }
    
    #[test]
    fn test_nonce_generation() {
        let generator = NonceGenerator::new().unwrap();
        
        let nonce1 = generator.generate_nonce(12).unwrap();
        let nonce2 = generator.generate_nonce(12).unwrap();
        
        assert_eq!(nonce1.len(), 12);
        assert_eq!(nonce2.len(), 12);
        assert_ne!(nonce1.as_bytes(), nonce2.as_bytes());
        assert!(nonce1.verify_uniqueness(&nonce2));
    }
    
    #[test]
    fn test_timestamped_nonce() {
        let generator = NonceGenerator::new().unwrap();
        
        let nonce = generator.generate_timestamped_nonce(16).unwrap();
        assert_eq!(nonce.len(), 16);
        assert_ne!(nonce.generation_time(), 0);
        assert!(nonce.is_fresh(10)); // Should be fresh within 10 seconds
    }
    
    #[test]
    fn test_counter_nonce() {
        let generator = NonceGenerator::new().unwrap();
        let base = vec![0x42; 12];
        
        let nonce1 = generator.generate_counter_nonce(&base, 1).unwrap();
        let nonce2 = generator.generate_counter_nonce(&base, 2).unwrap();
        
        assert_eq!(nonce1.len(), 12);
        assert_eq!(nonce2.len(), 12);
        assert_ne!(nonce1.as_bytes(), nonce2.as_bytes());
        assert_eq!(nonce1.counter(), 1);
        assert_eq!(nonce2.counter(), 2);
    }
    
    #[test]
    fn test_nonce_counter_mode() {
        let base = vec![0x42; 12];
        let counter_mode = NonceCounterMode::new(base, 1000).unwrap();
        
        let nonce1 = counter_mode.next_nonce().unwrap();
        let nonce2 = counter_mode.next_nonce().unwrap();
        
        assert_eq!(nonce1.counter(), 0);
        assert_eq!(nonce2.counter(), 1);
        assert_ne!(nonce1.as_bytes(), nonce2.as_bytes());
        assert!(counter_mode.can_generate_more());
    }
    
    #[test]
    fn test_nonce_random_mode() {
        let random_mode = NonceRandomMode::new(NonceEntropySource::ChaCha20Rng);
        
        let nonce1 = random_mode.generate_nonce(12).unwrap();
        let nonce2 = random_mode.generate_nonce(12).unwrap();
        
        assert_eq!(nonce1.len(), 12);
        assert_eq!(nonce2.len(), 12);
        assert_ne!(nonce1.as_bytes(), nonce2.as_bytes());
    }
    
    #[test]
    fn test_secure_nonce() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let nonce = SecureNonce::from_bytes(data.clone());
        
        assert_eq!(nonce.as_bytes(), &data);
        assert_eq!(nonce.len(), 12);
        assert!(!nonce.is_empty());
        assert_ne!(nonce.generation_time(), 0);
        assert!(nonce.is_fresh(60));
    }
    
    #[test]
    fn test_nonce_uniqueness() {
        let nonce1 = SecureNonce::from_bytes(vec![1, 2, 3]);
        let nonce2 = SecureNonce::from_bytes(vec![1, 2, 4]);
        let nonce3 = SecureNonce::from_bytes(vec![1, 2, 3]);
        
        assert!(nonce1.verify_uniqueness(&nonce2));
        assert!(!nonce1.verify_uniqueness(&nonce3)); // Same data
    }
    
    #[test]
    fn test_nonce_utils() {
        assert!(NonceUtils::validate_nonce_size("ChaCha20", 12).is_ok());
        assert!(NonceUtils::validate_nonce_size("ChaCha20", 8).is_err());
        assert!(NonceUtils::validate_nonce_size("AES-GCM", 12).is_ok());
        assert!(NonceUtils::validate_nonce_size("Unknown", 12).is_err());
        
        let nonces = vec![
            SecureNonce::from_bytes(vec![1, 2, 3]),
            SecureNonce::from_bytes(vec![4, 5, 6]),
        ];
        assert!(!NonceUtils::check_collision(&nonces));
        
        let pattern_nonce = NonceUtils::generate_patterned_nonce(b"prefix", 8).unwrap();
        assert_eq!(pattern_nonce.len(), 14);
        assert!(pattern_nonce.as_bytes().starts_with(b"prefix"));
    }
    
    #[test]
    fn test_invalid_nonce_sizes() {
        let generator = NonceGenerator::new().unwrap();
        
        assert!(generator.generate_nonce(0).is_err());
        assert!(generator.generate_nonce(2000).is_err());
        assert!(generator.generate_timestamped_nonce(8).is_err());
    }
    
    #[test]
    fn test_from_seed() {
        let seed = [42u8; 32];
        let generator1 = NonceGenerator::from_seed(seed);
        let generator2 = NonceGenerator::from_seed(seed);
        
        // Same seed should produce deterministic results for testing
        let mut buffer1 = [0u8; 16];
        let mut buffer2 = [0u8; 16];
        
        generator1.fill_random(&mut buffer1).unwrap();
        generator2.fill_random(&mut buffer2).unwrap();
        
        assert_eq!(buffer1, buffer2);
    }
}
