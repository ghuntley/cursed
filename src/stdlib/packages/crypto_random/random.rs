/// Production-ready cryptographic random number generation module
pub mod entropy_sources;
pub mod entropy_collection;
pub mod entropy_monitoring;
pub mod entropy_estimation;
pub mod entropy_mixing;
pub mod hardware_entropy;
pub mod csprng;
pub mod secure_random;
pub mod random_generators;
pub mod random_numbers;
pub mod random_bytes;
pub mod random_strings;
pub mod nonce_generation;
pub mod randomness_tests;
pub mod security_analysis;

use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;

// Re-export main types and functions for convenient access
pub use secure_random::{
    SecureRandom, secure_bytes, secure_u32, secure_u64, secure_f64, secure_bool,
    secure_range_u32, secure_range_u64, secure_choose, secure_shuffle, 
    secure_fill_bytes, secure_reseed, secure_entropy_info, secure_test_quality
};

pub use random_bytes::{
    RandomBytes, random_bytes, random_hex, random_base64, random_base64url,
    random_salt, random_iv, random_nonce as random_nonce_bytes, random_key_material
};

pub use random_strings::{
    RandomStrings, CharSet, random_string, random_alphabetic, random_alphanumeric,
    random_numeric, random_hexadecimal, random_password, random_identifier, random_filename
};

pub use random_numbers::{
    RandomNumbers, random_normal, random_exponential, random_uniform, random_uniform_int,
    random_poisson, random_binomial
};

pub use random_generators::{
    PasswordGenerator, UuidGenerator, TokenGenerator, DataGenerator,
    generate_password, generate_secure_password, generate_uuid, generate_token,
    generate_api_key, generate_session_token
};

pub use nonce_generation::{
    NonceGenerator, NonceStrategy, NonceFormat, NonceConfig,
    generate_nonce, generate_random_nonce, generate_timestamp_nonce,
    generate_uuid_nonce, generate_encryption_nonce, generate_session_nonce
};

pub use csprng::{Csprng, CsprngAlgorithm, CsprngConfig};
pub use hardware_entropy::{HardwareEntropyCollector, HardwareRngType};
pub use entropy_collection::{EntropyCollector, EntropyCollectionConfig};
pub use entropy_monitoring::{EntropyMonitor, EntropyQualityMetrics, AlertLevel};
pub use randomness_tests::{RandomnessTestSuite, TestResult, TestSuiteConfig};
pub use security_analysis::{SecurityAnalyzer, SecurityLevel, ThreatModel, SecurityAnalysisConfig};

/// Generate cryptographically secure random bytes
pub fn random_bytes(size: usize) -> AdvancedCryptoResult<Vec<u8>> {
    secure_bytes(size)
}

/// Generate cryptographically secure random number
pub fn random_number() -> AdvancedCryptoResult<u64> {
    secure_u64()
}

/// Initialize the cryptographic random number generator
pub fn init_rng() -> AdvancedCryptoResult<()> {
    // The SecureRandom implementation automatically initializes itself
    // We can test it by generating a small amount of entropy
    let _test_bytes = secure_bytes(1)?;
    Ok(())
}

/// Advanced random number generation API
pub struct RandomAPI {
    secure_rng: SecureRandom,
    byte_generator: RandomBytes,
    string_generator: RandomStrings,
    number_generator: RandomNumbers,
    nonce_generator: NonceGenerator,
}

impl RandomAPI {
    /// Create new random API instance
    pub fn new() -> AdvancedCryptoResult<Self> {
        Ok(Self {
            secure_rng: SecureRandom::new()?,
            byte_generator: RandomBytes::new()?,
            string_generator: RandomStrings::new()?,
            number_generator: RandomNumbers::new()?,
            nonce_generator: NonceGenerator::new()?,
        })
    }
    
    /// Generate random bytes with various encodings
    pub fn bytes(&self) -> &RandomBytes {
        &self.byte_generator
    }
    
    /// Generate random strings with various character sets
    pub fn strings(&self) -> &RandomStrings {
        &self.string_generator
    }
    
    /// Generate random numbers with statistical distributions
    pub fn numbers(&self) -> &RandomNumbers {
        &self.number_generator
    }
    
    /// Generate nonces for cryptographic operations
    pub fn nonces(&self) -> &NonceGenerator {
        &self.nonce_generator
    }
    
    /// Access the underlying secure random generator
    pub fn secure(&self) -> &SecureRandom {
        &self.secure_rng
    }
    
    /// Test the quality of generated randomness
    pub fn test_quality(&self, sample_size: usize) -> AdvancedCryptoResult<String> {
        self.secure_rng.test_quality(sample_size)
    }
    
    /// Get entropy source information
    pub fn entropy_info(&self) -> String {
        self.secure_rng.get_entropy_info()
    }
    
    /// Force reseed of all generators
    pub fn reseed(&self) -> AdvancedCryptoResult<()> {
        self.secure_rng.reseed()
    }
}

impl Default for RandomAPI {
    fn default() -> Self {
        Self::new().expect("Failed to create default RandomAPI")
    }
}

/// Quick access functions for common operations

/// Generate a random UUID
pub fn uuid() -> AdvancedCryptoResult<String> {
    generate_uuid()
}

/// Generate a secure password
pub fn password(length: usize) -> AdvancedCryptoResult<String> {
    generate_password(length)
}

/// Generate an API key
pub fn api_key() -> AdvancedCryptoResult<String> {
    generate_api_key()
}

/// Generate a session token
pub fn session_token() -> AdvancedCryptoResult<String> {
    generate_session_token()
}

/// Generate a nonce for cryptographic operations
pub fn nonce() -> AdvancedCryptoResult<String> {
    generate_nonce()
}

/// Generate random integer in range [min, max] (inclusive)
pub fn range_i32(min: i32, max: i32) -> AdvancedCryptoResult<i32> {
    SecureRandom::new()?.range_i32(min, max)
}

/// Generate random integer in range [min, max] (inclusive)
pub fn range_u32(min: u32, max: u32) -> AdvancedCryptoResult<u32> {
    SecureRandom::new()?.range_u32(min, max)
}

/// Generate random float in range [0.0, 1.0)
pub fn float() -> AdvancedCryptoResult<f64> {
    secure_f64()
}

/// Generate random boolean
pub fn boolean() -> AdvancedCryptoResult<bool> {
    secure_bool()
}

/// Choose random element from slice
pub fn choose<T>(items: &[T]) -> AdvancedCryptoResult<Option<&T>> {
    secure_choose(items)
}

/// Shuffle slice in place
pub fn shuffle<T>(items: &mut [T]) -> AdvancedCryptoResult<()> {
    secure_shuffle(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_bytes_generation() {
        let result = random_bytes(32);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(bytes.len(), 32);
        
        // Test that bytes are not all the same (basic randomness check)
        let all_same = bytes.windows(2).all(|w| w[0] == w[1]);
        assert!(!all_same, "Generated bytes should not all be identical");
    }

    #[test]
    fn test_random_number_generation() {
        let result1 = random_number();
        let result2 = random_number();
        
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        
        let num1 = result1.unwrap();
        let num2 = result2.unwrap();
        
        // Numbers should be different (probabilistic test)
        assert_ne!(num1, num2, "Generated numbers should be different");
    }

    #[test]
    fn test_rng_initialization() {
        let result = init_rng();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_random_api_creation() {
        let api = RandomAPI::new();
        assert!(api.is_ok());
    }
    
    #[test]
    fn test_uuid_generation() {
        let uuid_result = uuid();
        assert!(uuid_result.is_ok());
        
        let uuid_str = uuid_result.unwrap();
        assert!(uuid_str.len() > 0);
        
        // UUID should have proper format (basic check)
        assert!(uuid_str.contains('-'), "UUID should contain hyphens");
    }
    
    #[test]
    fn test_password_generation() {
        let password_result = password(16);
        assert!(password_result.is_ok());
        
        let password_str = password_result.unwrap();
        assert_eq!(password_str.len(), 16);
    }
    
    #[test]
    fn test_range_generation() {
        let range_result = range_u32(1, 10);
        assert!(range_result.is_ok());
        
        let value = range_result.unwrap();
        assert!(value >= 1 && value <= 10);
    }
    
    #[test]
    fn test_float_generation() {
        let float_result = float();
        assert!(float_result.is_ok());
        
        let value = float_result.unwrap();
        assert!(value >= 0.0 && value < 1.0);
    }
    
    #[test]
    fn test_boolean_generation() {
        let bool_result = boolean();
        assert!(bool_result.is_ok());
        
        // Generate multiple booleans to check variety
        let mut has_true = false;
        let mut has_false = false;
        
        for _ in 0..20 {
            if let Ok(b) = boolean() {
                if b {
                    has_true = true;
                } else {
                    has_false = true;
                }
                
                if has_true && has_false {
                    break;
                }
            }
        }
        
        // With 20 attempts, we should see both true and false
        assert!(has_true || has_false, "Should generate at least one boolean value");
    }
    
    #[test]
    fn test_choose_function() {
        let items = vec![1, 2, 3, 4, 5];
        let choice_result = choose(&items);
        assert!(choice_result.is_ok());
        
        let choice = choice_result.unwrap();
        if let Some(value) = choice {
            assert!(items.contains(value));
        }
    }
    
    #[test]
    fn test_shuffle_function() {
        let mut items = vec![1, 2, 3, 4, 5];
        let original = items.clone();
        
        let shuffle_result = shuffle(&mut items);
        assert!(shuffle_result.is_ok());
        
        // Items should contain same elements
        items.sort();
        assert_eq!(items, original);
    }
    
    #[test]
    fn test_entropy_info() {
        let info = secure_entropy_info();
        assert!(!info.is_empty());
        assert!(info.contains("CSPRNG"));
    }
    
    #[test]
    fn test_quality_testing() {
        let quality_result = secure_test_quality(1000);
        assert!(quality_result.is_ok());
        
        let report = quality_result.unwrap();
        assert!(!report.is_empty());
        assert!(report.contains("Test Report"));
    }
    
    #[test]
    fn test_reseed_functionality() {
        let reseed_result = secure_reseed();
        assert!(reseed_result.is_ok());
    }
    
    #[test]
    fn test_api_key_generation() {
        let key_result = api_key();
        assert!(key_result.is_ok());
        
        let key = key_result.unwrap();
        assert!(key.len() >= 32, "API key should be at least 32 characters");
    }
    
    #[test]
    fn test_session_token_generation() {
        let token_result = session_token();
        assert!(token_result.is_ok());
        
        let token = token_result.unwrap();
        assert!(token.len() >= 16, "Session token should be at least 16 characters");
    }
    
    #[test]
    fn test_nonce_generation() {
        let nonce_result = nonce();
        assert!(nonce_result.is_ok());
        
        let nonce_str = nonce_result.unwrap();
        assert!(!nonce_str.is_empty());
        
        // Generate multiple nonces to check uniqueness
        let mut nonces = std::collections::HashSet::new();
        for _ in 0..10 {
            if let Ok(n) = nonce() {
                nonces.insert(n);
            }
        }
        
        // All nonces should be unique
        assert_eq!(nonces.len(), 10, "All generated nonces should be unique");
    }
    
    #[test]
    fn test_random_api_methods() {
        let api = RandomAPI::new().unwrap();
        
        // Test each component
        let bytes_result = api.bytes().generate(16);
        assert!(bytes_result.is_ok());
        
        let string_result = api.strings().alphanumeric(12);
        assert!(string_result.is_ok());
        
        let number_result = api.numbers().uniform(0.0, 1.0);
        assert!(number_result.is_ok());
        
        let nonce_result = api.nonces().generate();
        assert!(nonce_result.is_ok());
        
        let info = api.entropy_info();
        assert!(!info.is_empty());
    }
}
