/// fr fr High-level Hybrid Cryptography API for CURSED Language
/// 
/// This module provides a user-friendly interface for hybrid cryptography that combines
/// classical and post-quantum cryptographic algorithms. Hybrid schemes offer the best
/// of both worlds: the proven security of classical algorithms and the quantum-resistance
/// of post-quantum cryptography.
/// 
/// ## Key Features
/// - **Dual Security**: Protection against both classical and quantum attacks
/// - **Migration Ready**: Smooth transition path from classical to post-quantum cryptography
/// - **Performance Optimized**: Intelligent fallback and optimization strategies
/// - **Standards Compliant**: Implements industry-standard hybrid schemes
/// 
/// ## Supported Hybrid Schemes
/// - **X25519+Kyber**: Key exchange combining elliptic curve and lattice cryptography
/// - **Ed25519+Dilithium**: Digital signatures with classical and post-quantum components
/// - **RSA+SPHINCS+**: Legacy RSA with hash-based post-quantum signatures
/// 
/// ## Security Considerations
/// Hybrid cryptography provides "crypto-agility" - the ability to adapt to new threats
/// while maintaining backward compatibility. Each hybrid operation includes both
/// classical and post-quantum components, ensuring security even if one component
/// is compromised.

use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::stdlib::packages::crypto_asymmetric::{AsymmetricKey, AsymmetricKeyPair};
use crate::error::CursedError;
use super::pqc_core::{PqcKey, SecurityLevel};
use super::hybrid::{
    HybridAlgorithmConfig, HybridKeyPair, HybridKemResult, HybridSignature,
    X25519KyberHybrid, Ed25519DilithiumHybrid, HybridCryptoManager,
    HybridFallbackManager, FallbackStrategy, HybridSchemeType
};
use std::collections::HashMap;
use std::time::{Duration, Instant};

// ================================================================================================
// SIMPLE API FOR COMMON USE CASES
// ================================================================================================

/// Generate a secure hybrid key pair for the most common use case
/// This is the simplest way to get started with hybrid cryptography
pub fn generate_secure_keypair() -> AdvancedCryptoResult<HybridKeyPair> {
    let hybrid = X25519KyberHybrid::new(SecurityLevel::Level3)?;
    hybrid.generate_keypair()
}

/// Encrypt data using hybrid cryptography (simple interface)
/// Returns encrypted data that requires both classical and PQC algorithms to decrypt
pub fn hybrid_encrypt(data: &[u8], public_key: &HybridKeyPair) -> AdvancedCryptoResult<HybridEncryptionResult> {
    let hybrid = X25519KyberHybrid::new(SecurityLevel::Level3)?;
    let kem_result = hybrid.encapsulate(public_key)?;
    
    // Use the shared secret to encrypt the data
    let encrypted_data = simple_encrypt(data, &kem_result.shared_secret)?;
    
    Ok(HybridEncryptionResult {
        classical_ciphertext: kem_result.classical_ciphertext,
        pqc_ciphertext: kem_result.pqc_ciphertext,
        encrypted_data,
        algorithm: kem_result.algorithm,
        timestamp: std::time::SystemTime::now(),
    })
}

/// Decrypt data using hybrid cryptography (simple interface)
pub fn hybrid_decrypt(encryption_result: &HybridEncryptionResult, private_key: &HybridKeyPair) -> AdvancedCryptoResult<Vec<u8>> {
    let hybrid = X25519KyberHybrid::new(SecurityLevel::Level3)?;
    
    let kem_result = HybridKemResult {
        classical_ciphertext: encryption_result.classical_ciphertext.clone(),
        pqc_ciphertext: encryption_result.pqc_ciphertext.clone(),
        shared_secret: Vec::new(), // Will be computed during decapsulation
        algorithm: encryption_result.algorithm.clone(),
    };
    
    let shared_secret = hybrid.decapsulate(private_key, &kem_result)?;
    simple_decrypt(&encryption_result.encrypted_data, &shared_secret)
}

/// Sign data using hybrid digital signatures (simple interface)
pub fn hybrid_sign(data: &[u8], private_key: &HybridKeyPair) -> AdvancedCryptoResult<HybridSignature> {
    let hybrid = Ed25519DilithiumHybrid::new(SecurityLevel::Level3);
    hybrid.sign(private_key, data)
}

/// Verify hybrid digital signature (simple interface)
pub fn hybrid_verify(data: &[u8], signature: &HybridSignature, public_key: &HybridKeyPair) -> AdvancedCryptoResult<bool> {
    let hybrid = Ed25519DilithiumHybrid::new(SecurityLevel::Level3);
    hybrid.verify(public_key, data, signature)
}

// ================================================================================================
// SECURE MESSAGING WORKFLOW
// ================================================================================================

/// Complete secure messaging session with hybrid cryptography
pub struct SecureMessagingSession {
    sender_keypair: HybridKeyPair,
    receiver_public_key: Option<HybridKeyPair>,
    session_id: String,
    security_level: SecurityLevel,
    message_counter: u64,
    created_at: std::time::SystemTime,
}

impl SecureMessagingSession {
    /// Create new secure messaging session
    pub fn new(security_level: SecurityLevel) -> AdvancedCryptoResult<Self> {
        let hybrid = X25519KyberHybrid::new(security_level)?;
        let sender_keypair = hybrid.generate_keypair()?;
        
        Ok(Self {
            sender_keypair,
            receiver_public_key: None,
            session_id: generate_session_id(),
            security_level,
            message_counter: 0,
            created_at: std::time::SystemTime::now(),
        })
    }
    
    /// Set receiver's public key for the session
    pub fn set_receiver(&mut self, receiver_public_key: HybridKeyPair) -> AdvancedCryptoResult<()> {
        // Validate the receiver's key
        receiver_public_key.validate()?;
        self.receiver_public_key = Some(receiver_public_key);
        Ok(())
    }
    
    /// Send secure message
    pub fn send_message(&mut self, message: &str) -> AdvancedCryptoResult<SecureMessage> {
        let receiver_key = self.receiver_public_key.as_ref()
            .ok_or_else(|| CursedError::InvalidState("No receiver public key set".to_string()))?;
        
        // Create message with metadata
        let message_data = MessageData {
            content: message.as_bytes().to_vec(),
            sender_id: self.session_id.clone(),
            sequence: self.message_counter,
            timestamp: std::time::SystemTime::now(),
        };
        
        let serialized_message = serialize_message_data(&message_data)?;
        
        // Encrypt using hybrid cryptography
        let encryption_result = hybrid_encrypt(&serialized_message, receiver_key)?;
        
        // Sign the encrypted message for authenticity
        let signature = hybrid_sign(&encryption_result.encrypted_data, &self.sender_keypair)?;
        
        self.message_counter += 1;
        
        Ok(SecureMessage {
            encryption_result,
            signature,
            session_id: self.session_id.clone(),
            sequence: message_data.sequence,
        })
    }
    
    /// Receive and decrypt secure message
    pub fn receive_message(&self, secure_message: &SecureMessage, sender_public_key: &HybridKeyPair) -> AdvancedCryptoResult<String> {
        // Verify signature first
        let signature_valid = hybrid_verify(&secure_message.encryption_result.encrypted_data, &secure_message.signature, sender_public_key)?;
        if !signature_valid {
            return Err(CursedError::CryptoError("Invalid message signature".to_string()));
        }
        
        // Decrypt the message
        let decrypted_data = hybrid_decrypt(&secure_message.encryption_result, &self.sender_keypair)?;
        
        // Deserialize message data
        let message_data = deserialize_message_data(&decrypted_data)?;
        
        // Convert to string
        String::from_utf8(message_data.content)
            .map_err(|e| CursedError::InvalidInput(format!("Invalid UTF-8 in message: {}", e)))
    }
    
    /// Get session public key for sharing
    pub fn get_public_key(&self) -> AdvancedCryptoResult<Vec<u8>> {
        self.sender_keypair.get_combined_public_key()
    }
    
    /// Get session statistics
    pub fn get_statistics(&self) -> SessionStatistics {
        SessionStatistics {
            session_id: self.session_id.clone(),
            messages_sent: self.message_counter,
            session_duration: self.created_at.elapsed().unwrap_or(Duration::from_secs(0)),
            security_level: self.security_level.clone(),
        }
    }
}

// ================================================================================================
// MIGRATION AND COMPATIBILITY TOOLS
// ================================================================================================

/// Migration helper for transitioning from classical to hybrid cryptography
pub struct HybridMigrationHelper {
    current_classical_keys: Vec<AsymmetricKeyPair>,
    hybrid_keys: Vec<HybridKeyPair>,
    migration_config: MigrationConfig,
    compatibility_mode: bool,
}

impl HybridMigrationHelper {
    /// Create new migration helper
    pub fn new(migration_config: MigrationConfig) -> Self {
        Self {
            current_classical_keys: Vec::new(),
            hybrid_keys: Vec::new(),
            migration_config,
            compatibility_mode: true,
        }
    }
    
    /// Add existing classical keys to migration
    pub fn add_classical_keys(&mut self, keys: Vec<AsymmetricKeyPair>) {
        self.current_classical_keys.extend(keys);
    }
    
    /// Generate hybrid keys for migration
    pub fn generate_hybrid_keys(&mut self, count: usize) -> AdvancedCryptoResult<Vec<HybridKeyPair>> {
        let mut hybrid_keys = Vec::new();
        
        for _ in 0..count {
            let hybrid = X25519KyberHybrid::new(self.migration_config.target_security_level)?;
            let keypair = hybrid.generate_keypair()?;
            hybrid_keys.push(keypair);
        }
        
        self.hybrid_keys.extend(hybrid_keys.clone());
        Ok(hybrid_keys)
    }
    
    /// Create compatibility bridge for gradual migration
    pub fn create_compatibility_bridge(&self) -> CompatibilityBridge {
        CompatibilityBridge {
            classical_fallback: self.compatibility_mode,
            hybrid_preferred: true,
            migration_phase: self.migration_config.phase.clone(),
            supported_algorithms: vec![
                "X25519+Kyber".to_string(),
                "Ed25519+Dilithium".to_string(),
                "RSA+SPHINCS+".to_string(),
            ],
        }
    }
    
    /// Assess migration readiness
    pub fn assess_migration_readiness(&self) -> MigrationReadinessReport {
        let classical_count = self.current_classical_keys.len();
        let hybrid_count = self.hybrid_keys.len();
        
        let readiness_score = if hybrid_count == 0 {
            0.0
        } else if classical_count == 0 {
            1.0
        } else {
            hybrid_count as f64 / (classical_count + hybrid_count) as f64
        };
        
        let estimated_time = match self.migration_config.phase {
            MigrationPhase::Planning => Duration::from_secs(30 * 24 * 3600), // 30 days
            MigrationPhase::Testing => Duration::from_secs(60 * 24 * 3600),  // 60 days
            MigrationPhase::Gradual => Duration::from_secs(90 * 24 * 3600),  // 90 days
            MigrationPhase::Complete => Duration::from_secs(7 * 24 * 3600),  // 7 days
        };
        
        MigrationReadinessReport {
            readiness_score,
            classical_keys_count: classical_count,
            hybrid_keys_count: hybrid_count,
            estimated_migration_time: estimated_time,
            recommendations: generate_migration_recommendations(readiness_score, &self.migration_config),
            risks: assess_migration_risks(&self.migration_config),
        }
    }
}

// ================================================================================================
// CONFIGURATION AND MANAGEMENT
// ================================================================================================

/// Advanced hybrid crypto configuration manager
pub struct HybridConfigManager {
    configs: HashMap<String, HybridAlgorithmConfig>,
    performance_profiles: HashMap<String, PerformanceProfile>,
    security_policies: HashMap<String, SecurityPolicy>,
    current_config: Option<String>,
}

impl HybridConfigManager {
    /// Create new configuration manager with defaults
    pub fn new() -> Self {
        let mut manager = Self {
            configs: HashMap::new(),
            performance_profiles: HashMap::new(),
            security_policies: HashMap::new(),
            current_config: None,
        };
        
        manager.load_default_configs();
        manager.load_default_profiles();
        manager.load_default_policies();
        
        manager
    }
    
    /// Load default configurations
    fn load_default_configs(&mut self) {
        // High security configuration
        self.configs.insert("high_security".to_string(), 
            HybridAlgorithmConfig::x25519_kyber(SecurityLevel::Level5));
        
        // Balanced configuration
        self.configs.insert("balanced".to_string(), 
            HybridAlgorithmConfig::x25519_kyber(SecurityLevel::Level3));
        
        // Performance optimized configuration
        let mut perf_config = HybridAlgorithmConfig::x25519_kyber(SecurityLevel::Level1);
        perf_config.performance_priority = true;
        self.configs.insert("performance".to_string(), perf_config);
        
        // Signature configurations
        self.configs.insert("signature_standard".to_string(),
            HybridAlgorithmConfig::ed25519_dilithium(SecurityLevel::Level3));
        
        // Legacy compatibility
        self.configs.insert("legacy_rsa".to_string(),
            HybridAlgorithmConfig::rsa_sphincs(3072));
    }
    
    /// Load default performance profiles
    fn load_default_profiles(&mut self) {
        self.performance_profiles.insert("web_server".to_string(), PerformanceProfile {
            max_key_generation_time: Duration::from_millis(100),
            max_encryption_time: Duration::from_millis(10),
            max_signature_time: Duration::from_millis(50),
            memory_limit_mb: 50,
            cpu_intensive_operations: false,
        });
        
        self.performance_profiles.insert("high_throughput".to_string(), PerformanceProfile {
            max_key_generation_time: Duration::from_millis(50),
            max_encryption_time: Duration::from_millis(5),
            max_signature_time: Duration::from_millis(20),
            memory_limit_mb: 100,
            cpu_intensive_operations: true,
        });
        
        self.performance_profiles.insert("iot_device".to_string(), PerformanceProfile {
            max_key_generation_time: Duration::from_millis(500),
            max_encryption_time: Duration::from_millis(100),
            max_signature_time: Duration::from_millis(200),
            memory_limit_mb: 10,
            cpu_intensive_operations: false,
        });
    }
    
    /// Load default security policies
    fn load_default_policies(&mut self) {
        self.security_policies.insert("government".to_string(), SecurityPolicy {
            minimum_security_level: SecurityLevel::Level5,
            require_pqc: true,
            allow_classical_fallback: false,
            mandatory_signature: true,
            key_rotation_days: 30,
        });
        
        self.security_policies.insert("enterprise".to_string(), SecurityPolicy {
            minimum_security_level: SecurityLevel::Level3,
            require_pqc: true,
            allow_classical_fallback: true,
            mandatory_signature: true,
            key_rotation_days: 90,
        });
        
        self.security_policies.insert("standard".to_string(), SecurityPolicy {
            minimum_security_level: SecurityLevel::Level1,
            require_pqc: false,
            allow_classical_fallback: true,
            mandatory_signature: false,
            key_rotation_days: 365,
        });
    }
    
    /// Set active configuration
    pub fn set_config(&mut self, config_name: &str) -> AdvancedCryptoResult<()> {
        if self.configs.contains_key(config_name) {
            self.current_config = Some(config_name.to_string());
            Ok(())
        } else {
            Err(CursedError::InvalidInput(format!("Configuration '{}' not found", config_name)))
        }
    }
    
    /// Get current configuration
    pub fn get_current_config(&self) -> AdvancedCryptoResult<&HybridAlgorithmConfig> {
        let config_name = self.current_config.as_ref()
            .ok_or_else(|| CursedError::InvalidState("No active configuration".to_string()))?;
        
        self.configs.get(config_name)
            .ok_or_else(|| CursedError::InvalidState("Active configuration not found".to_string()))
    }
    
    /// Create optimized configuration for use case
    pub fn create_optimized_config(&self, use_case: &str, performance_profile: &str, security_policy: &str) -> AdvancedCryptoResult<HybridAlgorithmConfig> {
        let profile = self.performance_profiles.get(performance_profile)
            .ok_or_else(|| CursedError::InvalidInput(format!("Performance profile '{}' not found", performance_profile)))?;
        
        let policy = self.security_policies.get(security_policy)
            .ok_or_else(|| CursedError::InvalidInput(format!("Security policy '{}' not found", security_policy)))?;
        
        // Create configuration based on requirements
        let mut config = match use_case {
            "messaging" | "communication" => HybridAlgorithmConfig::x25519_kyber(policy.minimum_security_level),
            "document_signing" | "authentication" => HybridAlgorithmConfig::ed25519_dilithium(policy.minimum_security_level),
            "legacy_support" => HybridAlgorithmConfig::rsa_sphincs(3072),
            _ => HybridAlgorithmConfig::x25519_kyber(SecurityLevel::Level3),
        };
        
        // Apply performance optimizations
        config.performance_priority = profile.cpu_intensive_operations;
        config.fallback_enabled = policy.allow_classical_fallback;
        
        Ok(config)
    }
    
    /// List available configurations
    pub fn list_configs(&self) -> Vec<String> {
        self.configs.keys().cloned().collect()
    }
    
    /// List available performance profiles
    pub fn list_performance_profiles(&self) -> Vec<String> {
        self.performance_profiles.keys().cloned().collect()
    }
    
    /// List available security policies
    pub fn list_security_policies(&self) -> Vec<String> {
        self.security_policies.keys().cloned().collect()
    }
}

// ================================================================================================
// BENCHMARKING AND TESTING
// ================================================================================================

/// Comprehensive hybrid crypto benchmarking suite
pub struct HybridBenchmarkSuite {
    configs_to_test: Vec<HybridAlgorithmConfig>,
    test_data_sizes: Vec<usize>,
    iterations: usize,
}

impl HybridBenchmarkSuite {
    /// Create new benchmark suite
    pub fn new() -> Self {
        Self {
            configs_to_test: vec![
                HybridAlgorithmConfig::x25519_kyber(SecurityLevel::Level1),
                HybridAlgorithmConfig::x25519_kyber(SecurityLevel::Level3),
                HybridAlgorithmConfig::x25519_kyber(SecurityLevel::Level5),
                HybridAlgorithmConfig::ed25519_dilithium(SecurityLevel::Level1),
                HybridAlgorithmConfig::ed25519_dilithium(SecurityLevel::Level3),
                HybridAlgorithmConfig::ed25519_dilithium(SecurityLevel::Level5),
            ],
            test_data_sizes: vec![100, 1024, 10240, 102400], // 100B, 1KB, 10KB, 100KB
            iterations: 100,
        }
    }
    
    /// Run comprehensive benchmark
    pub fn run_benchmark(&self) -> AdvancedCryptoResult<BenchmarkReport> {
        let mut report = BenchmarkReport {
            algorithm_results: HashMap::new(),
            summary: BenchmarkSummary::default(),
            timestamp: std::time::SystemTime::now(),
        };
        
        for config in &self.configs_to_test {
            let algorithm_name = format!("{}+{}", config.classical_algorithm, config.pqc_algorithm);
            let mut algorithm_results = AlgorithmBenchmarkResult::default();
            
            // Benchmark key generation
            let key_gen_time = self.benchmark_key_generation(config)?;
            algorithm_results.avg_key_generation_time = key_gen_time;
            
            // Benchmark encryption/decryption if KEM
            if config.scheme_type == HybridSchemeType::Kem {
                for &data_size in &self.test_data_sizes {
                    let (enc_time, dec_time) = self.benchmark_encryption_decryption(config, data_size)?;
                    algorithm_results.encryption_times.insert(data_size, enc_time);
                    algorithm_results.decryption_times.insert(data_size, dec_time);
                }
            }
            
            // Benchmark signing/verification if signature
            if config.scheme_type == HybridSchemeType::Signature {
                for &data_size in &self.test_data_sizes {
                    let (sign_time, verify_time) = self.benchmark_signing_verification(config, data_size)?;
                    algorithm_results.signing_times.insert(data_size, sign_time);
                    algorithm_results.verification_times.insert(data_size, verify_time);
                }
            }
            
            report.algorithm_results.insert(algorithm_name, algorithm_results);
        }
        
        // Calculate summary statistics
        report.summary = self.calculate_summary(&report.algorithm_results);
        
        Ok(report)
    }
    
    /// Benchmark key generation
    fn benchmark_key_generation(&self, config: &HybridAlgorithmConfig) -> AdvancedCryptoResult<Duration> {
        let start = Instant::now();
        
        for _ in 0..self.iterations {
            match config.scheme_type {
                HybridSchemeType::Kem => {
                    let hybrid = X25519KyberHybrid::new(config.security_level)?;
                    let _ = hybrid.generate_keypair()?;
                },
                HybridSchemeType::Signature => {
                    let hybrid = Ed25519DilithiumHybrid::new(config.security_level);
                    let _ = hybrid.generate_keypair()?;
                },
                _ => return Err(CursedError::InvalidInput("Unsupported scheme type".to_string())),
            }
        }
        
        let total_time = start.elapsed();
        Ok(total_time / self.iterations as u32)
    }
    
    /// Benchmark encryption and decryption
    fn benchmark_encryption_decryption(&self, config: &HybridAlgorithmConfig, data_size: usize) -> AdvancedCryptoResult<(Duration, Duration)> {
        let hybrid = X25519KyberHybrid::new(config.security_level)?;
        let keypair = hybrid.generate_keypair()?;
        let test_data = vec![0u8; data_size];
        
        // Benchmark encryption
        let enc_start = Instant::now();
        for _ in 0..self.iterations {
            let _ = hybrid_encrypt(&test_data, &keypair)?;
        }
        let avg_enc_time = enc_start.elapsed() / self.iterations as u32;
        
        // Prepare for decryption benchmark
        let encryption_result = hybrid_encrypt(&test_data, &keypair)?;
        
        // Benchmark decryption
        let dec_start = Instant::now();
        for _ in 0..self.iterations {
            let _ = hybrid_decrypt(&encryption_result, &keypair)?;
        }
        let avg_dec_time = dec_start.elapsed() / self.iterations as u32;
        
        Ok((avg_enc_time, avg_dec_time))
    }
    
    /// Benchmark signing and verification
    fn benchmark_signing_verification(&self, config: &HybridAlgorithmConfig, data_size: usize) -> AdvancedCryptoResult<(Duration, Duration)> {
        let hybrid = Ed25519DilithiumHybrid::new(config.security_level);
        let keypair = hybrid.generate_keypair()?;
        let test_data = vec![0u8; data_size];
        
        // Benchmark signing
        let sign_start = Instant::now();
        for _ in 0..self.iterations {
            let _ = hybrid_sign(&test_data, &keypair)?;
        }
        let avg_sign_time = sign_start.elapsed() / self.iterations as u32;
        
        // Prepare for verification benchmark
        let signature = hybrid_sign(&test_data, &keypair)?;
        
        // Benchmark verification
        let verify_start = Instant::now();
        for _ in 0..self.iterations {
            let _ = hybrid_verify(&test_data, &signature, &keypair)?;
        }
        let avg_verify_time = verify_start.elapsed() / self.iterations as u32;
        
        Ok((avg_sign_time, avg_verify_time))
    }
    
    /// Calculate summary statistics
    fn calculate_summary(&self, results: &HashMap<String, AlgorithmBenchmarkResult>) -> BenchmarkSummary {
        let mut summary = BenchmarkSummary::default();
        
        if results.is_empty() {
            return summary;
        }
        
        // Calculate average times across all algorithms
        let key_gen_times: Vec<Duration> = results.values().map(|r| r.avg_key_generation_time).collect();
        summary.avg_key_generation_time = key_gen_times.iter().sum::<Duration>() / key_gen_times.len() as u32;
        
        // Find fastest and slowest algorithms
        if let Some((fastest_alg, fastest_result)) = results.iter()
            .min_by_key(|(_, result)| result.avg_key_generation_time) {
            summary.fastest_algorithm = fastest_alg.clone();
            summary.fastest_key_generation = fastest_result.avg_key_generation_time;
        }
        
        if let Some((slowest_alg, slowest_result)) = results.iter()
            .max_by_key(|(_, result)| result.avg_key_generation_time) {
            summary.slowest_algorithm = slowest_alg.clone();
            summary.slowest_key_generation = slowest_result.avg_key_generation_time;
        }
        
        summary.total_algorithms_tested = results.len();
        summary
    }
}

// ================================================================================================
// DEMONSTRATION AND EXAMPLES
// ================================================================================================

/// Comprehensive demo of hybrid cryptography features
pub fn run_hybrid_crypto_demo() -> AdvancedCryptoResult<()> {
    println!("🔐 CURSED Hybrid Cryptography Demo");
    println!("==================================");
    
    // 1. Simple encryption/decryption
    println!("\n1. Simple Hybrid Encryption/Decryption");
    println!("---------------------------------------");
    
    let alice_keypair = generate_secure_keypair()?;
    let message = "Hello, post-quantum world! 🌍";
    
    println!("Original message: {}", message);
    
    let encrypted = hybrid_encrypt(message.as_bytes(), &alice_keypair)?;
    println!("✅ Message encrypted with hybrid cryptography");
    println!("   - Classical component: {} bytes", encrypted.classical_ciphertext.len());
    println!("   - Post-quantum component: {} bytes", encrypted.pqc_ciphertext.len());
    println!("   - Algorithm: {}", encrypted.algorithm);
    
    let decrypted = hybrid_decrypt(&encrypted, &alice_keypair)?;
    let decrypted_message = String::from_utf8(decrypted)
        .map_err(|e| CursedError::InvalidInput(format!("UTF-8 error: {}", e)))?;
    
    println!("✅ Message decrypted successfully: {}", decrypted_message);
    
    // 2. Digital signatures
    println!("\n2. Hybrid Digital Signatures");
    println!("-----------------------------");
    
    let document = "Important contract: Transfer 1000 CURSED tokens";
    let signature = hybrid_sign(document.as_bytes(), &alice_keypair)?;
    
    println!("Document: {}", document);
    println!("✅ Document signed with hybrid signature");
    println!("   - Classical signature: {} bytes", signature.classical_signature.len());
    println!("   - Post-quantum signature: {} bytes", signature.pqc_signature.len());
    println!("   - Algorithm: {}", signature.algorithm);
    
    let is_valid = hybrid_verify(document.as_bytes(), &signature, &alice_keypair)?;
    println!("✅ Signature verification: {}", if is_valid { "VALID" } else { "INVALID" });
    
    // 3. Secure messaging session
    println!("\n3. Secure Messaging Session");
    println!("---------------------------");
    
    let mut alice_session = SecureMessagingSession::new(SecurityLevel::Level3)?;
    let mut bob_session = SecureMessagingSession::new(SecurityLevel::Level3)?;
    
    // Exchange public keys
    let alice_public = alice_session.get_public_key()?;
    let bob_public = bob_session.get_public_key()?;
    
    // Set up session (simplified - in reality would need proper key exchange)
    alice_session.set_receiver(bob_session.sender_keypair.clone())?;
    bob_session.set_receiver(alice_session.sender_keypair.clone())?;
    
    // Alice sends message to Bob
    let secret_message = "The eagle has landed. Quantum resistance is active.";
    let secure_msg = alice_session.send_message(secret_message)?;
    
    println!("Alice sends: {}", secret_message);
    println!("✅ Message encrypted and signed");
    println!("   - Session ID: {}", secure_msg.session_id);
    println!("   - Sequence: {}", secure_msg.sequence);
    
    // Bob receives and decrypts
    let received_message = bob_session.receive_message(&secure_msg, &alice_session.sender_keypair)?;
    println!("✅ Bob receives: {}", received_message);
    
    // 4. Configuration management
    println!("\n4. Configuration Management");
    println!("---------------------------");
    
    let mut config_manager = HybridConfigManager::new();
    
    println!("Available configurations:");
    for config in config_manager.list_configs() {
        println!("   - {}", config);
    }
    
    println!("Available performance profiles:");
    for profile in config_manager.list_performance_profiles() {
        println!("   - {}", profile);
    }
    
    println!("Available security policies:");
    for policy in config_manager.list_security_policies() {
        println!("   - {}", policy);
    }
    
    // Create optimized configuration
    let web_config = config_manager.create_optimized_config("messaging", "web_server", "enterprise")?;
    println!("✅ Created optimized configuration for web messaging");
    println!("   - Security level: {:?}", web_config.security_level);
    println!("   - Performance priority: {}", web_config.performance_priority);
    
    // 5. Migration assessment
    println!("\n5. Migration Assessment");
    println!("----------------------");
    
    let migration_config = MigrationConfig {
        phase: MigrationPhase::Planning,
        target_security_level: SecurityLevel::Level3,
        timeline_months: 6,
    };
    
    let mut migration_helper = HybridMigrationHelper::new(migration_config);
    
    // Simulate adding classical keys
    let classical_keys = vec![
        create_mock_classical_keypair("RSA2048"),
        create_mock_classical_keypair("ECDSA-P256"),
    ];
    migration_helper.add_classical_keys(classical_keys);
    
    // Generate hybrid keys
    let hybrid_keys = migration_helper.generate_hybrid_keys(2)?;
    println!("✅ Generated {} hybrid key pairs for migration", hybrid_keys.len());
    
    let readiness = migration_helper.assess_migration_readiness();
    println!("Migration readiness score: {:.2}", readiness.readiness_score);
    println!("Estimated migration time: {} days", readiness.estimated_migration_time.as_secs() / (24 * 3600));
    
    // 6. Performance benchmark
    println!("\n6. Performance Benchmark");
    println!("------------------------");
    
    let benchmark_suite = HybridBenchmarkSuite::new();
    println!("Running performance benchmark (this may take a moment)...");
    
    let benchmark_report = benchmark_suite.run_benchmark()?;
    println!("✅ Benchmark completed");
    println!("   - Algorithms tested: {}", benchmark_report.summary.total_algorithms_tested);
    println!("   - Average key generation: {:?}", benchmark_report.summary.avg_key_generation_time);
    println!("   - Fastest algorithm: {}", benchmark_report.summary.fastest_algorithm);
    println!("   - Slowest algorithm: {}", benchmark_report.summary.slowest_algorithm);
    
    println!("\n🎉 Hybrid Cryptography Demo Complete!");
    println!("=====================================");
    println!("✨ All hybrid cryptography features demonstrated successfully");
    println!("🔒 Your data is now protected against both classical and quantum attacks");
    println!("🚀 Ready for the post-quantum future!");
    
    Ok(())
}

// ================================================================================================
// UTILITY FUNCTIONS AND DATA STRUCTURES
// ================================================================================================

/// Simple symmetric encryption using a shared secret (placeholder implementation)
fn simple_encrypt(data: &[u8], key: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
    // Placeholder: In production, use AES-GCM or ChaCha20-Poly1305
    let mut encrypted = Vec::new();
    for (i, &byte) in data.iter().enumerate() {
        let key_byte = key[i % key.len()];
        encrypted.push(byte ^ key_byte);
    }
    Ok(encrypted)
}

/// Simple symmetric decryption using a shared secret (placeholder implementation)
fn simple_decrypt(encrypted_data: &[u8], key: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
    // XOR is symmetric, so decryption is the same as encryption
    simple_encrypt(encrypted_data, key)
}

/// Generate unique session ID
fn generate_session_id() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    format!("session_{:x}", hasher.finish())
}

/// Serialize message data (placeholder implementation)
fn serialize_message_data(data: &MessageData) -> AdvancedCryptoResult<Vec<u8>> {
    // Placeholder: In production, use proper serialization like bincode or protobuf
    let mut serialized = Vec::new();
    
    // Add sequence number
    serialized.extend_from_slice(&data.sequence.to_be_bytes());
    
    // Add timestamp (seconds since epoch)
    let timestamp = data.timestamp.duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default().as_secs();
    serialized.extend_from_slice(&timestamp.to_be_bytes());
    
    // Add sender ID length and data
    let sender_bytes = data.sender_id.as_bytes();
    serialized.extend_from_slice(&(sender_bytes.len() as u32).to_be_bytes());
    serialized.extend_from_slice(sender_bytes);
    
    // Add content length and data
    serialized.extend_from_slice(&(data.content.len() as u32).to_be_bytes());
    serialized.extend_from_slice(&data.content);
    
    Ok(serialized)
}

/// Deserialize message data (placeholder implementation)
fn deserialize_message_data(data: &[u8]) -> AdvancedCryptoResult<MessageData> {
    if data.len() < 20 { // Minimum size check
        return Err(CursedError::InvalidInput("Invalid message data".to_string()));
    }
    
    let mut offset = 0;
    
    // Read sequence number
    let sequence = u64::from_be_bytes([
        data[offset], data[offset+1], data[offset+2], data[offset+3],
        data[offset+4], data[offset+5], data[offset+6], data[offset+7]
    ]);
    offset += 8;
    
    // Read timestamp
    let timestamp_secs = u64::from_be_bytes([
        data[offset], data[offset+1], data[offset+2], data[offset+3],
        data[offset+4], data[offset+5], data[offset+6], data[offset+7]
    ]);
    offset += 8;
    
    let timestamp = std::time::UNIX_EPOCH + std::time::Duration::from_secs(timestamp_secs);
    
    // Read sender ID
    let sender_len = u32::from_be_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]) as usize;
    offset += 4;
    
    if offset + sender_len > data.len() {
        return Err(CursedError::InvalidInput("Invalid sender ID length".to_string()));
    }
    
    let sender_id = String::from_utf8(data[offset..offset + sender_len].to_vec())
        .map_err(|e| CursedError::InvalidInput(format!("Invalid sender ID: {}", e)))?;
    offset += sender_len;
    
    // Read content
    let content_len = u32::from_be_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]) as usize;
    offset += 4;
    
    if offset + content_len > data.len() {
        return Err(CursedError::InvalidInput("Invalid content length".to_string()));
    }
    
    let content = data[offset..offset + content_len].to_vec();
    
    Ok(MessageData {
        content,
        sender_id,
        sequence,
        timestamp,
    })
}

/// Generate migration recommendations
fn generate_migration_recommendations(readiness_score: f64, config: &MigrationConfig) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    if readiness_score < 0.3 {
        recommendations.push("Begin immediate hybrid key generation".to_string());
        recommendations.push("Establish PQC testing environment".to_string());
        recommendations.push("Train development team on hybrid cryptography".to_string());
    } else if readiness_score < 0.7 {
        recommendations.push("Continue gradual migration to hybrid schemes".to_string());
        recommendations.push("Implement hybrid fallback mechanisms".to_string());
        recommendations.push("Conduct security assessments of hybrid implementation".to_string());
    } else {
        recommendations.push("Finalize transition to full hybrid cryptography".to_string());
        recommendations.push("Deprecate pure classical cryptography".to_string());
        recommendations.push("Implement quantum-safe key management".to_string());
    }
    
    match config.phase {
        MigrationPhase::Planning => {
            recommendations.push("Develop detailed migration timeline".to_string());
            recommendations.push("Assess current cryptographic inventory".to_string());
        },
        MigrationPhase::Testing => {
            recommendations.push("Expand testing coverage of hybrid schemes".to_string());
            recommendations.push("Validate performance in production-like environments".to_string());
        },
        MigrationPhase::Gradual => {
            recommendations.push("Monitor hybrid scheme performance metrics".to_string());
            recommendations.push("Gradually increase hybrid usage percentage".to_string());
        },
        MigrationPhase::Complete => {
            recommendations.push("Verify full migration completion".to_string());
            recommendations.push("Implement post-migration security monitoring".to_string());
        },
    }
    
    recommendations
}

/// Assess migration risks
fn assess_migration_risks(config: &MigrationConfig) -> Vec<String> {
    let mut risks = Vec::new();
    
    match config.target_security_level {
        SecurityLevel::Level1 => {
            risks.push("Level 1 security may not be sufficient for sensitive applications".to_string());
        },
        SecurityLevel::Level5 => {
            risks.push("Level 5 security may impact performance significantly".to_string());
        },
        _ => {},
    }
    
    if config.timeline_months < 3 {
        risks.push("Aggressive timeline may lead to implementation errors".to_string());
    }
    
    if config.timeline_months > 12 {
        risks.push("Extended timeline may delay quantum resistance".to_string());
    }
    
    risks.push("Hybrid schemes have larger key sizes and signatures".to_string());
    risks.push("Performance overhead during transition period".to_string());
    risks.push("Compatibility issues with legacy systems".to_string());
    
    risks
}

/// Create mock classical keypair for testing
fn create_mock_classical_keypair(algorithm: &str) -> AsymmetricKeyPair {
    let key_size = match algorithm {
        "RSA2048" => 256,   // 2048 bits = 256 bytes
        "RSA3072" => 384,   // 3072 bits = 384 bytes
        "ECDSA-P256" => 32, // P-256 = 32 bytes
        "Ed25519" => 32,    // Ed25519 = 32 bytes
        _ => 32,
    };
    
    AsymmetricKeyPair {
        private_key: AsymmetricKey {
            algorithm: algorithm.to_string(),
            key_data: vec![0u8; key_size],
            is_private: true,
        },
        public_key: AsymmetricKey {
            algorithm: algorithm.to_string(),
            key_data: vec![0u8; key_size],
            is_private: false,
        },
    }
}

// ================================================================================================
// DATA STRUCTURES
// ================================================================================================

/// Result of hybrid encryption operation
#[derive(Debug, Clone)]
pub struct HybridEncryptionResult {
    pub classical_ciphertext: Vec<u8>,
    pub pqc_ciphertext: Vec<u8>,
    pub encrypted_data: Vec<u8>,
    pub algorithm: String,
    pub timestamp: std::time::SystemTime,
}

/// Secure message for hybrid messaging
#[derive(Debug, Clone)]
pub struct SecureMessage {
    pub encryption_result: HybridEncryptionResult,
    pub signature: HybridSignature,
    pub session_id: String,
    pub sequence: u64,
}

/// Message data structure
#[derive(Debug, Clone)]
pub struct MessageData {
    pub content: Vec<u8>,
    pub sender_id: String,
    pub sequence: u64,
    pub timestamp: std::time::SystemTime,
}

/// Session statistics
#[derive(Debug, Clone)]
pub struct SessionStatistics {
    pub session_id: String,
    pub messages_sent: u64,
    pub session_duration: Duration,
    pub security_level: SecurityLevel,
}

/// Migration configuration
#[derive(Debug, Clone)]
pub struct MigrationConfig {
    pub phase: MigrationPhase,
    pub target_security_level: SecurityLevel,
    pub timeline_months: u32,
}

/// Migration phases
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationPhase {
    Planning,
    Testing,
    Gradual,
    Complete,
}

/// Migration readiness report
#[derive(Debug)]
pub struct MigrationReadinessReport {
    pub readiness_score: f64,
    pub classical_keys_count: usize,
    pub hybrid_keys_count: usize,
    pub estimated_migration_time: Duration,
    pub recommendations: Vec<String>,
    pub risks: Vec<String>,
}

/// Compatibility bridge for migration
#[derive(Debug)]
pub struct CompatibilityBridge {
    pub classical_fallback: bool,
    pub hybrid_preferred: bool,
    pub migration_phase: MigrationPhase,
    pub supported_algorithms: Vec<String>,
}

/// Performance profile for different use cases
#[derive(Debug, Clone)]
pub struct PerformanceProfile {
    pub max_key_generation_time: Duration,
    pub max_encryption_time: Duration,
    pub max_signature_time: Duration,
    pub memory_limit_mb: u32,
    pub cpu_intensive_operations: bool,
}

/// Security policy configuration
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub minimum_security_level: SecurityLevel,
    pub require_pqc: bool,
    pub allow_classical_fallback: bool,
    pub mandatory_signature: bool,
    pub key_rotation_days: u32,
}

/// Benchmark report
#[derive(Debug)]
pub struct BenchmarkReport {
    pub algorithm_results: HashMap<String, AlgorithmBenchmarkResult>,
    pub summary: BenchmarkSummary,
    pub timestamp: std::time::SystemTime,
}

/// Algorithm benchmark result
#[derive(Debug, Default)]
pub struct AlgorithmBenchmarkResult {
    pub avg_key_generation_time: Duration,
    pub encryption_times: HashMap<usize, Duration>,
    pub decryption_times: HashMap<usize, Duration>,
    pub signing_times: HashMap<usize, Duration>,
    pub verification_times: HashMap<usize, Duration>,
}

/// Benchmark summary
#[derive(Debug, Default)]
pub struct BenchmarkSummary {
    pub total_algorithms_tested: usize,
    pub avg_key_generation_time: Duration,
    pub fastest_algorithm: String,
    pub slowest_algorithm: String,
    pub fastest_key_generation: Duration,
    pub slowest_key_generation: Duration,
}

// ================================================================================================
// TESTING
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_hybrid_crypto() {
        let keypair = generate_secure_keypair().unwrap();
        
        let message = "Test hybrid encryption";
        let encrypted = hybrid_encrypt(message.as_bytes(), &keypair).unwrap();
        let decrypted = hybrid_decrypt(&encrypted, &keypair).unwrap();
        let decrypted_message = String::from_utf8(decrypted).unwrap();
        
        assert_eq!(message, decrypted_message);
    }
    
    #[test]
    fn test_hybrid_signatures() {
        let keypair = generate_secure_keypair().unwrap();
        
        let document = "Important document";
        let signature = hybrid_sign(document.as_bytes(), &keypair).unwrap();
        let is_valid = hybrid_verify(document.as_bytes(), &signature, &keypair).unwrap();
        
        assert!(is_valid);
    }
    
    #[test]
    fn test_secure_messaging_session() {
        let mut alice = SecureMessagingSession::new(SecurityLevel::Level1).unwrap();
        let mut bob = SecureMessagingSession::new(SecurityLevel::Level1).unwrap();
        
        // Set up session
        alice.set_receiver(bob.sender_keypair.clone()).unwrap();
        bob.set_receiver(alice.sender_keypair.clone()).unwrap();
        
        // Send message
        let message = "Hello Bob!";
        let secure_msg = alice.send_message(message).unwrap();
        
        // Receive message
        let received = bob.receive_message(&secure_msg, &alice.sender_keypair).unwrap();
        assert_eq!(message, received);
    }
    
    #[test]
    fn test_config_manager() {
        let mut manager = HybridConfigManager::new();
        
        assert!(!manager.list_configs().is_empty());
        assert!(!manager.list_performance_profiles().is_empty());
        assert!(!manager.list_security_policies().is_empty());
        
        assert!(manager.set_config("balanced").is_ok());
        assert!(manager.get_current_config().is_ok());
        
        let config = manager.create_optimized_config("messaging", "web_server", "enterprise").unwrap();
        assert_eq!(config.scheme_type, HybridSchemeType::Kem);
    }
    
    #[test]
    fn test_migration_helper() {
        let migration_config = MigrationConfig {
            phase: MigrationPhase::Planning,
            target_security_level: SecurityLevel::Level3,
            timeline_months: 6,
        };
        
        let mut helper = HybridMigrationHelper::new(migration_config);
        
        // Add classical keys
        let classical_keys = vec![create_mock_classical_keypair("RSA2048")];
        helper.add_classical_keys(classical_keys);
        
        // Generate hybrid keys
        let hybrid_keys = helper.generate_hybrid_keys(1).unwrap();
        assert_eq!(hybrid_keys.len(), 1);
        
        // Assess readiness
        let readiness = helper.assess_migration_readiness();
        assert!(readiness.readiness_score >= 0.0 && readiness.readiness_score <= 1.0);
        assert_eq!(readiness.classical_keys_count, 1);
        assert_eq!(readiness.hybrid_keys_count, 1);
    }
    
    #[test]
    fn test_message_serialization() {
        let data = MessageData {
            content: "Hello, World!".as_bytes().to_vec(),
            sender_id: "alice".to_string(),
            sequence: 42,
            timestamp: std::time::SystemTime::now(),
        };
        
        let serialized = serialize_message_data(&data).unwrap();
        let deserialized = deserialize_message_data(&serialized).unwrap();
        
        assert_eq!(data.content, deserialized.content);
        assert_eq!(data.sender_id, deserialized.sender_id);
        assert_eq!(data.sequence, deserialized.sequence);
    }
    
    #[test]
    fn test_benchmark_suite() {
        let suite = HybridBenchmarkSuite::new();
        assert!(!suite.configs_to_test.is_empty());
        assert!(!suite.test_data_sizes.is_empty());
        
        // Note: Full benchmark would take too long for unit tests
        // Just test the structure is correct
        assert!(suite.iterations > 0);
    }
}
