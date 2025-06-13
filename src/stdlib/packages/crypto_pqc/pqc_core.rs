/// fr fr Post-quantum cryptography core infrastructure
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::error::CursedError;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// NIST security levels for post-quantum cryptography
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecurityLevel {
    /// Level 1: At least as hard to break as AES-128
    Level1,
    /// Level 3: At least as hard to break as AES-192
    Level3,
    /// Level 5: At least as hard to break as AES-256
    Level5,
}

impl SecurityLevel {
    /// Get the classical security equivalent in bits
    pub fn classical_equivalent_bits(&self) -> u32 {
        match self {
            SecurityLevel::Level1 => 128,
            SecurityLevel::Level3 => 192,
            SecurityLevel::Level5 => 256,
        }
    }
    
    /// Get the quantum security bits
    pub fn quantum_security_bits(&self) -> u32 {
        match self {
            SecurityLevel::Level1 => 64,
            SecurityLevel::Level3 => 96,
            SecurityLevel::Level5 => 128,
        }
    }
}

/// Post-quantum cryptographic algorithm types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PqcAlgorithmType {
    /// Key Encapsulation Mechanism
    Kem,
    /// Digital Signature
    Signature,
    /// Hash-based signature
    HashBasedSignature,
    /// Symmetric encryption
    SymmetricEncryption,
}

/// PQC algorithm information
#[derive(Debug, Clone)]
pub struct PqcAlgorithm {
    pub name: String,
    pub algorithm_type: PqcAlgorithmType,
    pub security_level: SecurityLevel,
    pub key_size_bytes: usize,
    pub signature_size_bytes: Option<usize>,
    pub ciphertext_size_bytes: Option<usize>,
    pub is_standardized: bool,
    pub implementation_available: bool,
}

/// Algorithm registry for PQC algorithms
#[derive(Debug)]
pub struct PqcAlgorithmRegistry {
    algorithms: HashMap<String, PqcAlgorithm>,
    performance_data: HashMap<String, AlgorithmPerformance>,
}

/// Performance data for algorithms
#[derive(Debug, Clone)]
pub struct AlgorithmPerformance {
    pub keygen_time_ms: f64,
    pub sign_time_ms: f64,
    pub verify_time_ms: f64,
    pub encrypt_time_ms: f64,
    pub decrypt_time_ms: f64,
    pub operations_per_second: u64,
    pub memory_usage_bytes: usize,
}

impl Default for AlgorithmPerformance {
    fn default() -> Self {
        Self {
            keygen_time_ms: 0.0,
            sign_time_ms: 0.0,
            verify_time_ms: 0.0,
            encrypt_time_ms: 0.0,
            decrypt_time_ms: 0.0,
            operations_per_second: 0,
            memory_usage_bytes: 0,
        }
    }
}

impl PqcAlgorithmRegistry {
    /// Create a new registry with default algorithms
    pub fn new() -> Self {
        let mut registry = Self {
            algorithms: HashMap::new(),
            performance_data: HashMap::new(),
        };
        
        registry.register_default_algorithms();
        registry
    }
    
    /// Register default PQC algorithms
    fn register_default_algorithms(&mut self) {
        // Kyber KEM algorithms
        self.register_algorithm(PqcAlgorithm {
            name: "Kyber512".to_string(),
            algorithm_type: PqcAlgorithmType::Kem,
            security_level: SecurityLevel::Level1,
            key_size_bytes: 800,
            signature_size_bytes: None,
            ciphertext_size_bytes: Some(768),
            is_standardized: true,
            implementation_available: true,
        });
        
        self.register_algorithm(PqcAlgorithm {
            name: "Kyber768".to_string(),
            algorithm_type: PqcAlgorithmType::Kem,
            security_level: SecurityLevel::Level3,
            key_size_bytes: 1184,
            signature_size_bytes: None,
            ciphertext_size_bytes: Some(1088),
            is_standardized: true,
            implementation_available: true,
        });
        
        self.register_algorithm(PqcAlgorithm {
            name: "Kyber1024".to_string(),
            algorithm_type: PqcAlgorithmType::Kem,
            security_level: SecurityLevel::Level5,
            key_size_bytes: 1568,
            signature_size_bytes: None,
            ciphertext_size_bytes: Some(1568),
            is_standardized: true,
            implementation_available: true,
        });
        
        // Dilithium signature algorithms
        self.register_algorithm(PqcAlgorithm {
            name: "Dilithium2".to_string(),
            algorithm_type: PqcAlgorithmType::Signature,
            security_level: SecurityLevel::Level1,
            key_size_bytes: 1312,
            signature_size_bytes: Some(2420),
            ciphertext_size_bytes: None,
            is_standardized: true,
            implementation_available: true,
        });
        
        self.register_algorithm(PqcAlgorithm {
            name: "Dilithium3".to_string(),
            algorithm_type: PqcAlgorithmType::Signature,
            security_level: SecurityLevel::Level3,
            key_size_bytes: 1952,
            signature_size_bytes: Some(3293),
            ciphertext_size_bytes: None,
            is_standardized: true,
            implementation_available: true,
        });
        
        self.register_algorithm(PqcAlgorithm {
            name: "Dilithium5".to_string(),
            algorithm_type: PqcAlgorithmType::Signature,
            security_level: SecurityLevel::Level5,
            key_size_bytes: 2592,
            signature_size_bytes: Some(4595),
            ciphertext_size_bytes: None,
            is_standardized: true,
            implementation_available: true,
        });
        
        // SPHINCS+ hash-based signatures
        self.register_algorithm(PqcAlgorithm {
            name: "SPHINCS+128s".to_string(),
            algorithm_type: PqcAlgorithmType::HashBasedSignature,
            security_level: SecurityLevel::Level1,
            key_size_bytes: 32,
            signature_size_bytes: Some(7856),
            ciphertext_size_bytes: None,
            is_standardized: true,
            implementation_available: false, // Stub implementation
        });
        
        self.register_algorithm(PqcAlgorithm {
            name: "SPHINCS+192s".to_string(),
            algorithm_type: PqcAlgorithmType::HashBasedSignature,
            security_level: SecurityLevel::Level3,
            key_size_bytes: 48,
            signature_size_bytes: Some(16224),
            ciphertext_size_bytes: None,
            is_standardized: true,
            implementation_available: false, // Stub implementation
        });
        
        self.register_algorithm(PqcAlgorithm {
            name: "SPHINCS+256s".to_string(),
            algorithm_type: PqcAlgorithmType::HashBasedSignature,
            security_level: SecurityLevel::Level5,
            key_size_bytes: 64,
            signature_size_bytes: Some(29792),
            ciphertext_size_bytes: None,
            is_standardized: true,
            implementation_available: false, // Stub implementation
        });
    }
    
    /// Register a new algorithm
    pub fn register_algorithm(&mut self, algorithm: PqcAlgorithm) {
        let name = algorithm.name.clone();
        self.algorithms.insert(name.clone(), algorithm);
        self.performance_data.insert(name, AlgorithmPerformance::default());
    }
    
    /// Get algorithm by name
    pub fn get_algorithm(&self, name: &str) -> Option<&PqcAlgorithm> {
        self.algorithms.get(name)
    }
    
    /// Get algorithms by type and security level
    pub fn get_algorithms_by_criteria(
        &self,
        algorithm_type: PqcAlgorithmType,
        min_security_level: SecurityLevel,
    ) -> Vec<&PqcAlgorithm> {
        self.algorithms
            .values()
            .filter(|alg| {
                alg.algorithm_type == algorithm_type &&
                alg.security_level.classical_equivalent_bits() >= min_security_level.classical_equivalent_bits() &&
                alg.implementation_available
            })
            .collect()
    }
    
    /// Select best algorithm for criteria
    pub fn select_best_algorithm(
        &self,
        algorithm_type: PqcAlgorithmType,
        min_security_level: SecurityLevel,
        optimize_for_speed: bool,
    ) -> Option<&PqcAlgorithm> {
        let candidates = self.get_algorithms_by_criteria(algorithm_type, min_security_level);
        
        if candidates.is_empty() {
            return None;
        }
        
        if optimize_for_speed {
            // Select algorithm with best performance
            candidates.into_iter().min_by(|a, b| {
                let perf_a = self.performance_data.get(&a.name).unwrap();
                let perf_b = self.performance_data.get(&b.name).unwrap();
                perf_a.operations_per_second.cmp(&perf_b.operations_per_second).reverse()
            })
        } else {
            // Select algorithm with smallest key/signature sizes
            candidates.into_iter().min_by_key(|alg| {
                alg.key_size_bytes + alg.signature_size_bytes.unwrap_or(0) + alg.ciphertext_size_bytes.unwrap_or(0)
            })
        }
    }
    
    /// Update performance data for an algorithm
    pub fn update_performance(&mut self, algorithm_name: &str, performance: AlgorithmPerformance) {
        self.performance_data.insert(algorithm_name.to_string(), performance);
    }
    
    /// Get performance data
    pub fn get_performance(&self, algorithm_name: &str) -> Option<&AlgorithmPerformance> {
        self.performance_data.get(algorithm_name)
    }
}

/// Algorithm negotiation for protocol selection
#[derive(Debug)]
pub struct AlgorithmNegotiator {
    registry: PqcAlgorithmRegistry,
    preferences: HashMap<PqcAlgorithmType, Vec<String>>,
}

impl AlgorithmNegotiator {
    /// Create new negotiator
    pub fn new() -> Self {
        Self {
            registry: PqcAlgorithmRegistry::new(),
            preferences: HashMap::new(),
        }
    }
    
    /// Set algorithm preferences for a type
    pub fn set_preferences(&mut self, algorithm_type: PqcAlgorithmType, preferences: Vec<String>) {
        self.preferences.insert(algorithm_type, preferences);
    }
    
    /// Negotiate algorithm based on client and server preferences
    pub fn negotiate_algorithm(
        &self,
        algorithm_type: PqcAlgorithmType,
        client_algorithms: &[String],
        server_algorithms: &[String],
    ) -> Option<String> {
        // Find intersection of supported algorithms
        for client_alg in client_algorithms {
            if server_algorithms.contains(client_alg) {
                if let Some(alg) = self.registry.get_algorithm(client_alg) {
                    if alg.implementation_available {
                        return Some(client_alg.clone());
                    }
                }
            }
        }
        None
    }
}

/// Key format standardization
#[derive(Debug, Clone)]
pub enum PqcKeyFormat {
    /// Raw binary format
    Raw,
    /// PEM encoded format
    Pem,
    /// DER encoded format
    Der,
    /// PKCS#8 format
    Pkcs8,
    /// X.509 SubjectPublicKeyInfo
    X509,
}

/// Standardized key container
#[derive(Debug, Clone)]
pub struct PqcKey {
    pub algorithm: String,
    pub format: PqcKeyFormat,
    pub key_data: Vec<u8>,
    pub is_private: bool,
    pub metadata: HashMap<String, String>,
}

impl PqcKey {
    /// Create new PQC key
    pub fn new(
        algorithm: String,
        format: PqcKeyFormat,
        key_data: Vec<u8>,
        is_private: bool,
    ) -> Self {
        Self {
            algorithm,
            format,
            key_data,
            is_private,
            metadata: HashMap::new(),
        }
    }
    
    /// Convert key to different format
    pub fn convert_format(&self, target_format: PqcKeyFormat) -> AdvancedCryptoResult<PqcKey> {
        // In real implementation, this would convert between formats
        // For now, return a simple conversion
        Ok(PqcKey {
            algorithm: self.algorithm.clone(),
            format: target_format,
            key_data: self.key_data.clone(),
            is_private: self.is_private,
            metadata: self.metadata.clone(),
        })
    }
    
    /// Validate key format and data
    pub fn validate(&self) -> AdvancedCryptoResult<()> {
        if self.key_data.is_empty() {
            return Err(CursedError::InvalidInput("Empty key data".to_string()));
        }
        
        // Additional validation would go here
        Ok(())
    }
}

/// Performance benchmarking infrastructure
pub struct PqcBenchmark {
    registry: PqcAlgorithmRegistry,
}

impl PqcBenchmark {
    /// Create new benchmark runner
    pub fn new() -> Self {
        Self {
            registry: PqcAlgorithmRegistry::new(),
        }
    }
    
    /// Benchmark key generation for an algorithm
    pub fn benchmark_keygen(&mut self, algorithm_name: &str, iterations: usize) -> AdvancedCryptoResult<f64> {
        let start = Instant::now();
        
        // Simulate key generation (in real implementation, call actual keygen)
        for _ in 0..iterations {
            // Placeholder: actual key generation would happen here
            std::thread::sleep(Duration::from_micros(100)); // Simulate work
        }
        
        let elapsed = start.elapsed();
        let avg_time_ms = elapsed.as_secs_f64() * 1000.0 / iterations as f64;
        
        // Update performance data
        if let Some(mut perf) = self.registry.get_performance(algorithm_name).cloned() {
            perf.keygen_time_ms = avg_time_ms;
            self.registry.update_performance(algorithm_name, perf);
        }
        
        Ok(avg_time_ms)
    }
    
    /// Benchmark signing operations
    pub fn benchmark_signing(&mut self, algorithm_name: &str, iterations: usize) -> AdvancedCryptoResult<f64> {
        let start = Instant::now();
        
        // Simulate signing operations
        for _ in 0..iterations {
            std::thread::sleep(Duration::from_micros(50)); // Simulate work
        }
        
        let elapsed = start.elapsed();
        let avg_time_ms = elapsed.as_secs_f64() * 1000.0 / iterations as f64;
        
        // Update performance data
        if let Some(mut perf) = self.registry.get_performance(algorithm_name).cloned() {
            perf.sign_time_ms = avg_time_ms;
            perf.operations_per_second = (1000.0 / avg_time_ms) as u64;
            self.registry.update_performance(algorithm_name, perf);
        }
        
        Ok(avg_time_ms)
    }
    
    /// Run comprehensive benchmark suite
    pub fn run_comprehensive_benchmark(&mut self, algorithm_name: &str) -> AdvancedCryptoResult<AlgorithmPerformance> {
        let keygen_time = self.benchmark_keygen(algorithm_name, 100)?;
        let sign_time = self.benchmark_signing(algorithm_name, 1000)?;
        
        let performance = AlgorithmPerformance {
            keygen_time_ms: keygen_time,
            sign_time_ms: sign_time,
            verify_time_ms: sign_time * 0.8, // Verification typically faster
            encrypt_time_ms: sign_time * 1.2, // Encryption typically slower
            decrypt_time_ms: sign_time * 0.9, // Decryption typically faster than encryption
            operations_per_second: (1000.0 / sign_time) as u64,
            memory_usage_bytes: 1024 * 1024, // Placeholder: 1MB
        };
        
        self.registry.update_performance(algorithm_name, performance.clone());
        Ok(performance)
    }
}

/// Migration tools from classical to post-quantum
pub struct PqcMigrationTool {
    algorithm_mappings: HashMap<String, String>,
}

impl PqcMigrationTool {
    /// Create new migration tool
    pub fn new() -> Self {
        let mut mappings = HashMap::new();
        
        // Map classical algorithms to PQC equivalents
        mappings.insert("RSA2048".to_string(), "Dilithium2".to_string());
        mappings.insert("RSA3072".to_string(), "Dilithium3".to_string());
        mappings.insert("RSA4096".to_string(), "Dilithium5".to_string());
        mappings.insert("ECDSA-P256".to_string(), "Dilithium2".to_string());
        mappings.insert("ECDSA-P384".to_string(), "Dilithium3".to_string());
        mappings.insert("ECDSA-P521".to_string(), "Dilithium5".to_string());
        mappings.insert("Ed25519".to_string(), "Dilithium2".to_string());
        mappings.insert("X25519".to_string(), "Kyber512".to_string());
        mappings.insert("DH2048".to_string(), "Kyber768".to_string());
        mappings.insert("ECDH-P256".to_string(), "Kyber512".to_string());
        mappings.insert("ECDH-P384".to_string(), "Kyber768".to_string());
        mappings.insert("ECDH-P521".to_string(), "Kyber1024".to_string());
        
        Self { algorithm_mappings: mappings }
    }
    
    /// Get PQC equivalent for classical algorithm
    pub fn get_pqc_equivalent(&self, classical_algorithm: &str) -> Option<&String> {
        self.algorithm_mappings.get(classical_algorithm)
    }
    
    /// Generate migration plan
    pub fn generate_migration_plan(&self, current_algorithms: &[String]) -> Vec<(String, String)> {
        current_algorithms
            .iter()
            .filter_map(|alg| {
                self.get_pqc_equivalent(alg).map(|pqc_alg| (alg.clone(), pqc_alg.clone()))
            })
            .collect()
    }
    
    /// Assess migration complexity
    pub fn assess_migration_complexity(&self, current_algorithms: &[String]) -> MigrationComplexity {
        let total_algorithms = current_algorithms.len();
        let mappable_algorithms = current_algorithms
            .iter()
            .filter(|alg| self.algorithm_mappings.contains_key(*alg))
            .count();
        
        let percentage = if total_algorithms > 0 {
            (mappable_algorithms as f64 / total_algorithms as f64) * 100.0
        } else {
            0.0
        };
        
        match percentage {
            p if p >= 90.0 => MigrationComplexity::Low,
            p if p >= 70.0 => MigrationComplexity::Medium,
            p if p >= 50.0 => MigrationComplexity::High,
            _ => MigrationComplexity::VeryHigh,
        }
    }
}

/// Migration complexity assessment
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationComplexity {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// PQC readiness assessment
pub struct PqcReadinessAssessment {
    pub current_algorithms: Vec<String>,
    pub pqc_equivalents: Vec<(String, String)>,
    pub migration_complexity: MigrationComplexity,
    pub estimated_migration_time_days: u32,
    pub performance_impact: PerformanceImpact,
    pub recommendations: Vec<String>,
}

/// Performance impact assessment
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceImpact {
    Minimal,     // < 10% performance change
    Moderate,    // 10-50% performance change
    Significant, // 50-100% performance change
    Major,       // > 100% performance change
}

/// Conduct PQC readiness assessment
pub fn assess_pqc_readiness(current_algorithms: &[String]) -> PqcReadinessAssessment {
    let migration_tool = PqcMigrationTool::new();
    let pqc_equivalents = migration_tool.generate_migration_plan(current_algorithms);
    let migration_complexity = migration_tool.assess_migration_complexity(current_algorithms);
    
    let estimated_migration_time_days = match migration_complexity {
        MigrationComplexity::Low => 30,
        MigrationComplexity::Medium => 90,
        MigrationComplexity::High => 180,
        MigrationComplexity::VeryHigh => 365,
    };
    
    let performance_impact = match migration_complexity {
        MigrationComplexity::Low => PerformanceImpact::Minimal,
        MigrationComplexity::Medium => PerformanceImpact::Moderate,
        MigrationComplexity::High => PerformanceImpact::Significant,
        MigrationComplexity::VeryHigh => PerformanceImpact::Major,
    };
    
    let mut recommendations = Vec::new();
    recommendations.push("Start with hybrid cryptography for gradual transition".to_string());
    recommendations.push("Benchmark performance impact in test environment".to_string());
    recommendations.push("Update key management infrastructure".to_string());
    
    if migration_complexity == MigrationComplexity::VeryHigh {
        recommendations.push("Consider phased migration approach".to_string());
        recommendations.push("Evaluate custom PQC implementations".to_string());
    }
    
    PqcReadinessAssessment {
        current_algorithms: current_algorithms.to_vec(),
        pqc_equivalents,
        migration_complexity,
        estimated_migration_time_days,
        performance_impact,
        recommendations,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_security_levels() {
        assert_eq!(SecurityLevel::Level1.classical_equivalent_bits(), 128);
        assert_eq!(SecurityLevel::Level3.classical_equivalent_bits(), 192);
        assert_eq!(SecurityLevel::Level5.classical_equivalent_bits(), 256);
        
        assert_eq!(SecurityLevel::Level1.quantum_security_bits(), 64);
        assert_eq!(SecurityLevel::Level3.quantum_security_bits(), 96);
        assert_eq!(SecurityLevel::Level5.quantum_security_bits(), 128);
    }
    
    #[test]
    fn test_algorithm_registry() {
        let registry = PqcAlgorithmRegistry::new();
        
        // Test Kyber algorithms
        let kyber512 = registry.get_algorithm("Kyber512").unwrap();
        assert_eq!(kyber512.algorithm_type, PqcAlgorithmType::Kem);
        assert_eq!(kyber512.security_level, SecurityLevel::Level1);
        assert!(kyber512.implementation_available);
        
        // Test Dilithium algorithms
        let dilithium2 = registry.get_algorithm("Dilithium2").unwrap();
        assert_eq!(dilithium2.algorithm_type, PqcAlgorithmType::Signature);
        assert_eq!(dilithium2.security_level, SecurityLevel::Level1);
    }
    
    #[test]
    fn test_algorithm_selection() {
        let registry = PqcAlgorithmRegistry::new();
        
        let kem_algorithms = registry.get_algorithms_by_criteria(
            PqcAlgorithmType::Kem,
            SecurityLevel::Level1,
        );
        assert!(!kem_algorithms.is_empty());
        
        let best_kem = registry.select_best_algorithm(
            PqcAlgorithmType::Kem,
            SecurityLevel::Level1,
            false, // Optimize for size
        );
        assert!(best_kem.is_some());
    }
    
    #[test]
    fn test_algorithm_negotiation() {
        let mut negotiator = AlgorithmNegotiator::new();
        
        let client_algorithms = vec!["Kyber512".to_string(), "Dilithium2".to_string()];
        let server_algorithms = vec!["Kyber768".to_string(), "Kyber512".to_string()];
        
        let result = negotiator.negotiate_algorithm(
            PqcAlgorithmType::Kem,
            &client_algorithms,
            &server_algorithms,
        );
        assert_eq!(result, Some("Kyber512".to_string()));
    }
    
    #[test]
    fn test_migration_tool() {
        let migration_tool = PqcMigrationTool::new();
        
        let classical_algorithms = vec![
            "RSA2048".to_string(),
            "ECDSA-P256".to_string(),
            "X25519".to_string(),
        ];
        
        let migration_plan = migration_tool.generate_migration_plan(&classical_algorithms);
        assert_eq!(migration_plan.len(), 3);
        
        let complexity = migration_tool.assess_migration_complexity(&classical_algorithms);
        assert_eq!(complexity, MigrationComplexity::Low);
    }
    
    #[test]
    fn test_pqc_key() {
        let key_data = vec![1, 2, 3, 4, 5];
        let key = PqcKey::new(
            "Kyber512".to_string(),
            PqcKeyFormat::Raw,
            key_data.clone(),
            false,
        );
        
        assert_eq!(key.algorithm, "Kyber512");
        assert_eq!(key.key_data, key_data);
        assert!(!key.is_private);
        
        assert!(key.validate().is_ok());
        
        let converted = key.convert_format(PqcKeyFormat::Pem).unwrap();
        assert_eq!(converted.format, PqcKeyFormat::Pem);
    }
    
    #[test]
    fn test_pqc_readiness_assessment() {
        let current_algorithms = vec![
            "RSA2048".to_string(),
            "ECDSA-P256".to_string(),
            "AES256".to_string(), // No PQC equivalent
        ];
        
        let assessment = assess_pqc_readiness(&current_algorithms);
        assert_eq!(assessment.current_algorithms.len(), 3);
        assert_eq!(assessment.pqc_equivalents.len(), 2); // Only 2 have equivalents
        assert!(!assessment.recommendations.is_empty());
        assert!(assessment.estimated_migration_time_days > 0);
    }
}
