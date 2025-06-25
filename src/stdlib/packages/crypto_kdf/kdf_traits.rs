/// fr fr KDF Traits and common interfaces
/// 
/// This module provides common traits and interfaces for all KDF implementations
/// to ensure consistency and interoperability.

// use crate::stdlib::packages::crypto_kdf::{KdfResult, KdfError};
use crate::error::CursedError;

/// fr fr Common KDF trait for all key derivation functions
pub trait KeyDerivationFunction {
    /// Configuration type for this KDF
    type Config;
    
    /// Derive key from password and salt
    fn derive_key(&self, password: &[u8], salt: &[u8], output_length: usize) -> KdfResult<Vec<u8>>;
    
    /// Get configuration
    fn config(&self) -> &Self::Config;
    
    /// Validate inputs before processing
    fn validate_inputs(&self, password: &[u8], salt: &[u8], output_length: usize) -> KdfResult<()> {
        if password.is_empty() {
            return Err(KdfError::InvalidInput("Password cannot be empty".to_string()));
        }
        
        if salt.len() < 8 {
            return Err(KdfError::InvalidInput("Salt must be at least 8 bytes".to_string()));
        }
        
        if output_length == 0 || output_length > 1024 * 1024 {
            return Err(KdfError::InvalidInput("Output length must be between 1 and 1MB".to_string()));
        }
        
        Ok(())
    }
}

/// fr fr Trait for password hashing functions
pub trait PasswordHasher {
    /// Hash a password with automatic salt generation
    fn hash_password(&self, password: &[u8]) -> KdfResult<String>;
    
    /// Verify a password against a hash
    fn verify_password(&self, password: &[u8], hash: &str) -> KdfResult<bool>;
    
    /// Generate secure salt
    fn generate_salt(&self) -> KdfResult<Vec<u8>>;
}

/// fr fr Trait for configurable KDF parameters
pub trait Configurable {
    /// Configuration type
    type Config;
    
    /// Get current configuration
    fn config(&self) -> &Self::Config;
    
    /// Update configuration
    fn with_config(config: Self::Config) -> KdfResult<Self> where Self: Sized;
    
    /// Validate configuration
    fn validate_config(config: &Self::Config) -> KdfResult<()>;
}

/// fr fr Trait for memory-hard functions
pub trait MemoryHard {
    /// Get memory usage in bytes
    fn memory_usage(&self) -> usize;
    
    /// Get number of iterations
    fn iterations(&self) -> u32;
    
    /// Check if function is memory-hard
    fn is_memory_hard(&self) -> bool {
        self.memory_usage() > 1024 * 1024 // > 1MB
    }
    
    /// Estimate processing time in milliseconds
    fn estimate_time(&self) -> f64;
}

/// fr fr Trait for parallel processing support
pub trait ParallelSupport {
    /// Check if parallel processing is supported
    fn supports_parallel(&self) -> bool;
    
    /// Get optimal thread count
    fn optimal_threads(&self) -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    }
    
    /// Process with specified thread count
    fn process_parallel(&self, password: &[u8], salt: &[u8], threads: usize) -> KdfResult<Vec<u8>>;
}

/// fr fr Trait for security assessment
pub trait SecurityAssessment {
    /// Get security level in bits
    fn security_level(&self) -> u32;
    
    /// Check if configuration meets minimum security requirements
    fn meets_security_requirements(&self, min_level: u32) -> bool {
        self.security_level() >= min_level
    }
    
    /// Get resistance to various attack types
    fn attack_resistance(&self) -> AttackResistance;
}

/// fr fr Attack resistance information
#[derive(Debug, Clone)]
pub struct AttackResistance {
    pub brute_force: u32,      // Bits of resistance
    pub dictionary: u32,       // Bits of resistance
    pub rainbow_table: u32,    // Bits of resistance
    pub gpu_attack: u32,       // Bits of resistance
    pub asic_attack: u32,      // Bits of resistance
    pub side_channel: bool,    // Resistant to side-channel attacks
    pub timing_attack: bool,   // Resistant to timing attacks
}

impl AttackResistance {
    /// slay Create attack resistance with defaults
    pub fn new() -> Self {
        Self {
            brute_force: 128,
            dictionary: 80,
            rainbow_table: 80,
            gpu_attack: 64,
            asic_attack: 32,
            side_channel: false,
            timing_attack: false,
        }
    }
    
    /// bestie High-security resistance profile
    pub fn high_security() -> Self {
        Self {
            brute_force: 256,
            dictionary: 128,
            rainbow_table: 128,
            gpu_attack: 128,
            asic_attack: 80,
            side_channel: true,
            timing_attack: true,
        }
    }
}

impl Default for AttackResistance {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Trait for algorithm benchmarking
pub trait Benchmarkable {
    /// Benchmark configuration for target time
    fn benchmark_for_time(&self, target_ms: u64) -> KdfResult<Self> where Self: Sized;
    
    /// Run performance test
    fn performance_test(&self, iterations: usize) -> PerformanceResult;
}

/// fr fr Performance test results
#[derive(Debug, Clone)]
pub struct PerformanceResult {
    pub iterations: usize,
    pub total_time_ms: u64,
    pub average_time_ms: f64,
    pub min_time_ms: u64,
    pub max_time_ms: u64,
    pub memory_usage_bytes: usize,
    pub throughput_ops_per_sec: f64,
}

impl PerformanceResult {
    /// slay Create new performance result
    pub fn new(iterations: usize, times_ms: &[u64], memory_usage: usize) -> Self {
        let total_time = times_ms.iter().sum::<u64>();
        let average_time = total_time as f64 / iterations as f64;
        let min_time = *times_ms.iter().min().unwrap_or(&0);
        let max_time = *times_ms.iter().max().unwrap_or(&0);
        let throughput = if average_time > 0.0 {
            1000.0 / average_time // ops per second
        } else {
            0.0
        };
        
        Self {
            iterations,
            total_time_ms: total_time,
            average_time_ms: average_time,
            min_time_ms: min_time,
            max_time_ms: max_time,
            memory_usage_bytes: memory_usage,
            throughput_ops_per_sec: throughput,
        }
    }
}

/// fr fr Trait for constant-time operations
pub trait ConstantTime {
    /// Check if operations are constant-time
    fn is_constant_time(&self) -> bool;
    
    /// Perform constant-time comparison
    fn constant_time_eq(&self, a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        
        result == 0
    }
}

/// fr fr Unified KDF interface combining all traits
pub trait UnifiedKdf: 
    KeyDerivationFunction + 
    SecurityAssessment + 
    ConstantTime 
{
    /// Get algorithm name
    fn algorithm_name(&self) -> &'static str;
    
    /// Get algorithm version
    fn algorithm_version(&self) -> &'static str {
        "1.0"
    }
    
    /// Check compatibility with other implementations
    fn is_compatible_with(&self, other_name: &str, other_version: &str) -> bool {
        self.algorithm_name() == other_name && self.algorithm_version() == other_version
    }
    
    /// Get configuration as string
    fn get_config_string(&self) -> String;
    
    /// Update configuration from string
    fn update_config_string(&mut self, config: &str) -> KdfResult<()>;
}

/// fr fr KDF engine wrapper to unify different engine types
#[derive(Debug)]
pub struct KdfEngineWrapper {
    engine: KdfEngineType,
    config: String,
}

#[derive(Debug)]
enum KdfEngineType {
//     Pbkdf2(crate::stdlib::packages::crypto_kdf::pbkdf2::Pbkdf2Engine),
//     Argon2(crate::stdlib::packages::crypto_kdf::argon2::Argon2Engine),
//     Scrypt(crate::stdlib::packages::crypto_kdf::scrypt::ScryptEngine),
//     Hkdf(crate::stdlib::packages::crypto_kdf::hkdf::HkdfEngine),
}

impl KdfEngineWrapper {
    /// Create PBKDF2 wrapper
//     pub fn new_pbkdf2(engine: crate::stdlib::packages::crypto_kdf::pbkdf2::Pbkdf2Engine) -> Self {
        Self {
            engine: KdfEngineType::Pbkdf2(engine),
            config: "pbkdf2_default".to_string(),
        }
    }
    
    /// Create Argon2 wrapper  
//     pub fn new_argon2(engine: crate::stdlib::packages::crypto_kdf::argon2::Argon2Engine) -> Self {
        Self {
            engine: KdfEngineType::Argon2(engine),
            config: "argon2_default".to_string(),
        }
    }
    
    /// Create Scrypt wrapper
//     pub fn new_scrypt(engine: crate::stdlib::packages::crypto_kdf::scrypt::ScryptEngine) -> Self {
        Self {
            engine: KdfEngineType::Scrypt(engine),
            config: "scrypt_default".to_string(),
        }
    }
    
    /// Create HKDF wrapper
//     pub fn new_hkdf(engine: crate::stdlib::packages::crypto_kdf::hkdf::HkdfEngine) -> Self {
        Self {
            engine: KdfEngineType::Hkdf(engine),
            config: "hkdf_default".to_string(),
        }
    }
}

impl KeyDerivationFunction for KdfEngineWrapper {
    type Config = String;
    
    fn derive_key(&self, password: &[u8], salt: &[u8], output_length: usize) -> KdfResult<Vec<u8>> {
        match &self.engine {
            KdfEngineType::Pbkdf2(engine) => {
                engine.derive_key_custom(password, salt, 100_000, output_length)
                    .map_err(|e| KdfError::CryptographicError(format!("PBKDF2 error: {}", e)))
            }
            KdfEngineType::Argon2(engine) => {
                let mut key = engine.derive_key(password, salt)
                    .map_err(|e| KdfError::CryptographicError(format!("Argon2 error: {}", e)))?;
                key.truncate(output_length);
                key.resize(output_length, 0);
                Ok(key)
            }
            KdfEngineType::Scrypt(engine) => {
                let mut key = engine.derive_key(password, salt)
                    .map_err(|e| KdfError::CryptographicError(format!("Scrypt error: {}", e)))?;
                key.truncate(output_length);
                key.resize(output_length, 0);
                Ok(key)
            }
            KdfEngineType::Hkdf(engine) => {
                // For HKDF, we need to first extract, then expand
                // Use salt as IKM (input key material) and derive PRK first
                let prk = engine.extract(Some(salt), password)
                    .map_err(|e| KdfError::CryptographicError(format!("HKDF extract error: {}", e)))?;
                
                let info: Option<&[u8]> = None; // Empty info for basic key derivation
                engine.expand(&prk, info, output_length)
                    .map_err(|e| KdfError::CryptographicError(format!("HKDF expand error: {}", e)))
            }
        }
    }
    
    fn config(&self) -> &Self::Config {
        &self.config
    }
}

impl Configurable for KdfEngineWrapper {
    type Config = String;
    
    fn config(&self) -> &Self::Config {
        &self.config
    }
    
    fn with_config(config: Self::Config) -> KdfResult<Self> where Self: Sized {
        Self::create_engine_from_config(&config)
    }
    
    /// bestie Create engine from configuration string
    fn create_engine_from_config(config: &Self::Config) -> KdfResult<Self> where Self: Sized {
        // Parse configuration format: "algorithm:param1=value1,param2=value2"
        let parts: Vec<&str> = config.splitn(2, ':').collect();
        let algorithm = parts[0].trim();
        let params = if parts.len() > 1 { parts[1] } else { "" };
        
        match algorithm.to_lowercase().as_str() {
            "pbkdf2" => {
                let pbkdf2_config = KdfFactory::parse_pbkdf2_config(params)?;
//                 let engine = crate::stdlib::packages::crypto_kdf::pbkdf2::Pbkdf2Engine::new(pbkdf2_config)
                    .map_err(|e| KdfError::InvalidConfig(format!("PBKDF2 engine creation failed: {}", e)))?;
                Ok(KdfEngineWrapper::new_pbkdf2(engine))
            }
            "argon2" | "argon2i" | "argon2d" | "argon2id" => {
                let argon2_config = KdfFactory::parse_argon2_config(algorithm, params)?;
//                 let engine = crate::stdlib::packages::crypto_kdf::argon2::Argon2Engine::new(argon2_config);
                Ok(KdfEngineWrapper::new_argon2(engine))
            }
            "scrypt" => {
                let scrypt_config = KdfFactory::parse_scrypt_config(params)?;
//                 let engine = crate::stdlib::packages::crypto_kdf::scrypt::ScryptEngine::new(scrypt_config)
                    .map_err(|e| KdfError::InvalidConfig(format!("Scrypt engine creation failed: {}", e)))?;
                Ok(KdfEngineWrapper::new_scrypt(engine))
            }
            "hkdf" => {
//                 let engine = crate::stdlib::packages::crypto_kdf::hkdf::HkdfEngine::new();
                Ok(KdfEngineWrapper::new_hkdf(engine))
            }
            _ => Err(KdfError::InvalidConfig(format!("Unknown algorithm in config: {}", algorithm))),
        }
    }
    
    fn validate_config(config: &Self::Config) -> KdfResult<()> {
        // Basic validation of config format
        if config.is_empty() {
            return Err(KdfError::InvalidConfig("Empty configuration string".to_string()));
        }
        
        let parts: Vec<&str> = config.splitn(2, ':').collect();
        let algorithm = parts[0].trim();
        
        match algorithm.to_lowercase().as_str() {
            "pbkdf2" | "argon2" | "argon2i" | "argon2d" | "argon2id" | "scrypt" | "hkdf" => Ok(()),
            _ => Err(KdfError::InvalidConfig(format!("Unsupported algorithm: {}", algorithm))),
        }
    }
}

impl SecurityAssessment for KdfEngineWrapper {
    fn security_level(&self) -> u32 {
        match &self.engine {
            KdfEngineType::Pbkdf2(_) => 128,
            KdfEngineType::Argon2(_) => 256,
            KdfEngineType::Scrypt(_) => 192,
            KdfEngineType::Hkdf(_) => 128,
        }
    }
    
    fn attack_resistance(&self) -> AttackResistance {
        match &self.engine {
            KdfEngineType::Pbkdf2(_) => AttackResistance::new(),
            KdfEngineType::Argon2(_) => AttackResistance::high_security(),
            KdfEngineType::Scrypt(_) => {
                let mut resistance = AttackResistance::new();
                resistance.gpu_attack = 128;
                resistance.asic_attack = 80;
                resistance
            }
            KdfEngineType::Hkdf(_) => AttackResistance::new(),
        }
    }
}

impl ConstantTime for KdfEngineWrapper {
    fn is_constant_time(&self) -> bool {
        match &self.engine {
            KdfEngineType::Pbkdf2(_) => true,
            KdfEngineType::Argon2(_) => false, // Argon2d is not constant-time
            KdfEngineType::Scrypt(_) => true,
            KdfEngineType::Hkdf(_) => true,
        }
    }
}

impl UnifiedKdf for KdfEngineWrapper {
    fn algorithm_name(&self) -> &'static str {
        match &self.engine {
            KdfEngineType::Pbkdf2(_) => "PBKDF2",
            KdfEngineType::Argon2(_) => "Argon2", 
            KdfEngineType::Scrypt(_) => "scrypt",
            KdfEngineType::Hkdf(_) => "HKDF",
        }
    }
    
    fn algorithm_version(&self) -> &'static str {
        match &self.engine {
            KdfEngineType::Pbkdf2(_) => "RFC2898",
            KdfEngineType::Argon2(_) => "v1.3",
            KdfEngineType::Scrypt(_) => "RFC7914", 
            KdfEngineType::Hkdf(_) => "RFC5869",
        }
    }
    
    fn get_config_string(&self) -> String {
        self.config.clone()
    }
    
    fn update_config_string(&mut self, config: &str) -> KdfResult<()> {
        self.config = config.to_string();
        Ok(())
    }
}

/// fr fr KDF factory for creating instances
#[derive(Debug)]
pub struct KdfFactory;

impl KdfFactory {
    /// slay Create KDF by algorithm name
    pub fn create_kdf(algorithm: &str) -> KdfResult<Box<dyn UnifiedKdf<Config = String>>> {
        Self::create_kdf_with_config(algorithm, "")
    }
    
    /// bestie Create KDF by algorithm name with configuration
    pub fn create_kdf_with_config(algorithm: &str, config: &str) -> KdfResult<Box<dyn UnifiedKdf<Config = String>>> {
        match algorithm.to_lowercase().as_str() {
            "pbkdf2" => {
                let pbkdf2_config = Self::parse_pbkdf2_config(config)?;
//                 let engine = crate::stdlib::packages::crypto_kdf::pbkdf2::Pbkdf2Engine::new(pbkdf2_config)
                    .map_err(|e| KdfError::InvalidConfig(format!("PBKDF2 config error: {}", e)))?;
                Ok(Box::new(KdfEngineWrapper::new_pbkdf2(engine)))
            }
            "argon2" | "argon2i" | "argon2d" | "argon2id" => {
                let argon2_config = Self::parse_argon2_config(algorithm, config)?;
//                 let engine = crate::stdlib::packages::crypto_kdf::argon2::Argon2Engine::new(argon2_config);
                Ok(Box::new(KdfEngineWrapper::new_argon2(engine)))
            }
            "scrypt" => {
                let scrypt_config = Self::parse_scrypt_config(config)?;
//                 let engine = crate::stdlib::packages::crypto_kdf::scrypt::ScryptEngine::new(scrypt_config)
                    .map_err(|e| KdfError::InvalidConfig(format!("Scrypt config error: {}", e)))?;
                Ok(Box::new(KdfEngineWrapper::new_scrypt(engine)))
            }
            "hkdf" => {
//                 let engine = crate::stdlib::packages::crypto_kdf::hkdf::HkdfEngine::new();
                Ok(Box::new(KdfEngineWrapper::new_hkdf(engine)))
            }
            _ => Err(KdfError::InvalidConfig(format!("Unknown algorithm: {}", algorithm))),
        }
    }
    
    /// facts Parse PBKDF2 configuration from string
//     fn parse_pbkdf2_config(config: &str) -> KdfResult<crate::stdlib::packages::crypto_kdf::pbkdf2::Pbkdf2Config> {
        if config.is_empty() {
//             return Ok(crate::stdlib::packages::crypto_kdf::pbkdf2::Pbkdf2Config::new());
        }
        
//         let mut pbkdf2_config = crate::stdlib::packages::crypto_kdf::pbkdf2::Pbkdf2Config::new();
        
        // Parse configuration string format: "iterations=100000,output_len=32,hash=sha256"
        for pair in config.split(',') {
            let parts: Vec<&str> = pair.split('=').collect();
            if parts.len() != 2 {
                continue;
            }
            
            match parts[0].trim() {
                "iterations" => {
                    pbkdf2_config.iterations = parts[1].trim().parse()
                        .map_err(|_| KdfError::InvalidConfig("Invalid iterations value".to_string()))?;
                }
                "output_len" => {
                    pbkdf2_config.output_len = parts[1].trim().parse()
                        .map_err(|_| KdfError::InvalidConfig("Invalid output_len value".to_string()))?;
                }
                "hash" => {
//                     use crate::stdlib::packages::crypto_hash_advanced::hmac::HmacAlgorithm;
                    pbkdf2_config.hash_algorithm = match parts[1].trim().to_lowercase().as_str() {
                        "sha256" => HmacAlgorithm::Sha256,
                        "sha512" => HmacAlgorithm::Sha512,
                        _ => return Err(KdfError::InvalidConfig("Invalid hash algorithm".to_string())),
                    };
                }
                _ => {} // Ignore unknown parameters
            }
        }
        
        pbkdf2_config.validate()
            .map_err(|e| KdfError::InvalidConfig(format!("PBKDF2 validation error: {}", e)))?;
        
        Ok(pbkdf2_config)
    }
    
    /// vibes Parse Argon2 configuration from string
//     fn parse_argon2_config(algorithm: &str, config: &str) -> KdfResult<crate::stdlib::packages::crypto_kdf::argon2::Argon2Config> {
//         let mut argon2_config = crate::stdlib::packages::crypto_kdf::argon2::Argon2Config::new();
        
        // Set variant based on algorithm name
//         use crate::stdlib::packages::crypto_kdf::argon2::Argon2Variant;
        argon2_config.variant = match algorithm.to_lowercase().as_str() {
            "argon2i" => Argon2Variant::Argon2i,
            "argon2d" => Argon2Variant::Argon2d,
            "argon2id" | "argon2" => Argon2Variant::Argon2id,
            _ => Argon2Variant::Argon2id,
        };
        
        if config.is_empty() {
            return Ok(argon2_config);
        }
        
        // Parse configuration string format: "memory=65536,time=3,parallelism=4,output_len=32"
        for pair in config.split(',') {
            let parts: Vec<&str> = pair.split('=').collect();
            if parts.len() != 2 {
                continue;
            }
            
            match parts[0].trim() {
                "memory" => {
                    argon2_config.memory_cost = parts[1].trim().parse()
                        .map_err(|_| KdfError::InvalidConfig("Invalid memory value".to_string()))?;
                    argon2_config.memory_size = argon2_config.memory_cost as usize * 1024;
                }
                "time" => {
                    argon2_config.time_cost = parts[1].trim().parse()
                        .map_err(|_| KdfError::InvalidConfig("Invalid time value".to_string()))?;
                    argon2_config.iterations = argon2_config.time_cost;
                }
                "parallelism" => {
                    argon2_config.parallelism = parts[1].trim().parse()
                        .map_err(|_| KdfError::InvalidConfig("Invalid parallelism value".to_string()))?;
                }
                "output_len" => {
                    let output_len = parts[1].trim().parse()
                        .map_err(|_| KdfError::InvalidConfig("Invalid output_len value".to_string()))?;
                    argon2_config.output_len = output_len;
                    argon2_config.output_length = output_len;
                }
                _ => {} // Ignore unknown parameters
            }
        }
        
        Ok(argon2_config)
    }
    
    /// periodt Parse Scrypt configuration from string
//     fn parse_scrypt_config(config: &str) -> KdfResult<crate::stdlib::packages::crypto_kdf::scrypt::ScryptConfig> {
        if config.is_empty() {
//             return Ok(crate::stdlib::packages::crypto_kdf::scrypt::ScryptConfig::new());
        }
        
//         let mut scrypt_config = crate::stdlib::packages::crypto_kdf::scrypt::ScryptConfig::new();
        
        // Parse configuration string format: "n=32768,r=8,p=1,output_len=32"
        for pair in config.split(',') {
            let parts: Vec<&str> = pair.split('=').collect();
            if parts.len() != 2 {
                continue;
            }
            
            match parts[0].trim() {
                "n" => {
                    scrypt_config.n = parts[1].trim().parse()
                        .map_err(|_| KdfError::InvalidConfig("Invalid N value".to_string()))?;
                }
                "r" => {
                    scrypt_config.r = parts[1].trim().parse()
                        .map_err(|_| KdfError::InvalidConfig("Invalid r value".to_string()))?;
                }
                "p" => {
                    scrypt_config.p = parts[1].trim().parse()
                        .map_err(|_| KdfError::InvalidConfig("Invalid p value".to_string()))?;
                }
                "output_len" => {
                    scrypt_config.output_len = parts[1].trim().parse()
                        .map_err(|_| KdfError::InvalidConfig("Invalid output_len value".to_string()))?;
                }
                _ => {} // Ignore unknown parameters
            }
        }
        
        scrypt_config.validate()
            .map_err(|e| KdfError::InvalidConfig(format!("Scrypt validation error: {}", e)))?;
        
        Ok(scrypt_config)
    }
    
    /// bestie List available algorithms
    pub fn available_algorithms() -> Vec<&'static str> {
        vec!["pbkdf2", "argon2", "argon2i", "argon2d", "argon2id", "scrypt", "hkdf"]
    }
    
    /// vibes Get algorithm recommendations for use case
    pub fn recommend_algorithm(use_case: &str) -> &'static str {
        match use_case.to_lowercase().as_str() {
            "password" | "authentication" => "argon2id",
            "key_derivation" | "encryption" => "hkdf",
            "legacy" | "compatibility" => "pbkdf2",
            "memory_hard" | "slow" => "scrypt",
            _ => "argon2id", // Default recommendation
        }
    }
}

/// fr fr KDF utilities and helpers
#[derive(Debug)]
pub struct KdfUtils;

impl KdfUtils {
    /// bestie Generate cryptographically secure salt
    pub fn generate_salt(length: usize) -> KdfResult<Vec<u8>> {
        use rand::RngCore;
        
        if length == 0 || length > 1024 {
            return Err(KdfError::InvalidInput("Salt length must be between 1 and 1024 bytes".to_string()));
        }
        
        let mut salt = vec![0u8; length];
        rand::thread_rng().fill_bytes(&mut salt);
        Ok(salt)
    }
    
    /// vibes Validate password strength
    pub fn validate_password_strength(password: &[u8]) -> PasswordStrength {
        let length = password.len();
        let has_upper = password.iter().any(|&b| b >= b'A' && b <= b'Z');
        let has_lower = password.iter().any(|&b| b >= b'a' && b <= b'z');
        let has_digit = password.iter().any(|&b| b >= b'0' && b <= b'9');
        let has_special = password.iter().any(|&b| !b.is_ascii_alphanumeric());
        
        let complexity_score = [has_upper, has_lower, has_digit, has_special]
            .iter()
            .filter(|&&x| x)
            .count();
        
        let strength = if length < 8 {
            PasswordStrength::Weak
        } else if length < 12 || complexity_score < 3 {
            PasswordStrength::Fair
        } else if length < 16 || complexity_score < 4 {
            PasswordStrength::Good
        } else {
            PasswordStrength::Strong
        };
        
        strength
    }
    
    /// periodt Calculate entropy of input
    pub fn calculate_entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut counts = [0u32; 256];
        for &byte in data {
            counts[byte as usize] += 1;
        }
        
        let length = data.len() as f64;
        let mut entropy = 0.0;
        
        for &count in &counts {
            if count > 0 {
                let probability = count as f64 / length;
                entropy -= probability * probability.log2();
            }
        }
        
        entropy
    }
    
    /// facts Secure memory comparison
    pub fn secure_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        
        result == 0
    }
}

/// fr fr Password strength assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordStrength {
    Weak,
    Fair,
    Good,
    Strong,
}

impl PasswordStrength {
    pub fn score(&self) -> u32 {
        match self {
            PasswordStrength::Weak => 1,
            PasswordStrength::Fair => 2,
            PasswordStrength::Good => 3,
            PasswordStrength::Strong => 4,
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            PasswordStrength::Weak => "Weak - Use longer password with mixed characters",
            PasswordStrength::Fair => "Fair - Add more character types or length",
            PasswordStrength::Good => "Good - Strong enough for most purposes",
            PasswordStrength::Strong => "Strong - Excellent password strength",
        }
    }
}

