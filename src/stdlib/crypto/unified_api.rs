/// fr fr Unified Crypto API for CURSED - one API to rule them all periodt
/// 
/// This module provides a unified interface to all cryptographic functionality
/// across symmetric, asymmetric, hash, PKI, PQC, ZK, and protocol implementations.
/// Maximum convenience with production-ready security bestie!

use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Instant, Duration};
use serde::{Serialize, Deserialize};

use crate::error::CursedError;
// use crate::stdlib::value::Value;

/// fr fr Unified crypto error type
#[derive(Debug, Clone, PartialEq)]
pub enum UnifiedCryptoError {
    /// Configuration error
    Configuration(String),
    /// Algorithm not supported
    UnsupportedAlgorithm(String),
    /// Invalid input
    InvalidInput(String),
    /// Operation failed
    OperationFailed(String),
    /// Security violation
    SecurityViolation(String),
    /// Performance threshold exceeded
    PerformanceThreshold(String),
    /// Package not initialized
    PackageNotInitialized(String),
    /// Compliance check failed
    ComplianceFailed(String),
    /// Integration error
    Integration(String),
}

// impl std::fmt::Display for UnifiedCryptoError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             UnifiedCryptoError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
//             UnifiedCryptoError::UnsupportedAlgorithm(msg) => write!(f, "Unsupported algorithm: {}", msg),
//             UnifiedCryptoError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
//             UnifiedCryptoError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
//             UnifiedCryptoError::SecurityViolation(msg) => write!(f, "Security violation: {}", msg),
//             UnifiedCryptoError::PerformanceThreshold(msg) => write!(f, "Performance threshold exceeded: {}", msg),
//             UnifiedCryptoError::PackageNotInitialized(msg) => write!(f, "Package not initialized: {}", msg),
//             UnifiedCryptoError::ComplianceFailed(msg) => write!(f, "Compliance check failed: {}", msg),
//             UnifiedCryptoError::Integration(msg) => write!(f, "Integration error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for UnifiedCryptoError {}
// 
pub type UnifiedCryptoResult<T> = std::result::Result<T, UnifiedCryptoError>;

/// fr fr Crypto operation types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CryptoOperation {
    SymmetricEncrypt,
    SymmetricDecrypt,
    AsymmetricEncrypt,
    AsymmetricDecrypt,
    Hash,
    Sign,
    Verify,
    KeyGeneration,
    KeyDerivation,
    CertificateValidation,
    ZeroKnowledgeProof,
    PostQuantumOperation,
    ProtocolExecution,
}

/// fr fr Performance metrics for crypto operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub operation: CryptoOperation,
    pub algorithm: String,
    pub duration: Duration,
    pub throughput_bytes_per_second: Option<u64>,
    pub memory_usage_bytes: Option<u64>,
    pub cpu_cycles: Option<u64>,
    pub timestamp: std::time::SystemTime,
}

/// fr fr Security compliance levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComplianceLevel {
    Development,
    Testing,
    Production,
    HighSecurity,
    Enterprise,
    Government,
}

/// fr fr Security audit results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditResult {
    pub compliance_level: ComplianceLevel,
    pub algorithm_compliance: HashMap<String, bool>,
    pub security_vulnerabilities: Vec<String>,
    pub recommendations: Vec<String>,
    pub overall_score: f64,
    pub timestamp: std::time::SystemTime,
}

/// fr fr Crypto package configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoConfig {
    pub enabled_packages: Vec<String>,
    pub default_algorithms: HashMap<String, String>,
    pub performance_monitoring: bool,
    pub security_audit_frequency: Duration,
    pub compliance_level: ComplianceLevel,
    pub max_operation_duration: Duration,
    pub enable_timing_protection: bool,
    pub memory_protection: bool,
}

impl Default for CryptoConfig {
    fn default() -> Self {
        let mut default_algorithms = HashMap::new();
        default_algorithms.insert("symmetric".to_string(), "AES-256-GCM".to_string());
        default_algorithms.insert("asymmetric".to_string(), "RSA-4096".to_string());
        default_algorithms.insert("hash".to_string(), "SHA-256".to_string());
        default_algorithms.insert("signature".to_string(), "Ed25519".to_string());
        default_algorithms.insert("kdf".to_string(), "PBKDF2".to_string());
        default_algorithms.insert("pqc_kem".to_string(), "Kyber-1024".to_string());
        default_algorithms.insert("pqc_signature".to_string(), "Dilithium-5".to_string());

        Self {
            enabled_packages: vec![
                "crypto_advanced".to_string(),
                "crypto_asymmetric".to_string(),
                "crypto_hash_advanced".to_string(),
                "crypto_signatures".to_string(),
                "crypto_kdf".to_string(),
                "crypto_random".to_string(),
                "crypto_pki".to_string(),
                "crypto_zk".to_string(),
                "crypto_pqc".to_string(),
                "crypto_protocols".to_string(),
            ],
            default_algorithms,
            performance_monitoring: true,
            security_audit_frequency: Duration::from_secs(86400), // Daily
            compliance_level: ComplianceLevel::Production,
            max_operation_duration: Duration::from_secs(30),
            enable_timing_protection: true,
            memory_protection: true,
        }
    }
}

/// fr fr Package status tracking
#[derive(Debug, Clone, PartialEq)]
pub enum PackageStatus {
    NotInitialized,
    Initializing,
    Ready,
    CursedError(String),
    Disabled,
}

/// fr fr Unified crypto package manager
pub struct UnifiedCryptoManager {
    config: Arc<RwLock<CryptoConfig>>,
    package_status: Arc<RwLock<HashMap<String, PackageStatus>>>,
    performance_metrics: Arc<Mutex<Vec<PerformanceMetrics>>>,
    audit_results: Arc<Mutex<Vec<SecurityAuditResult>>>,
    initialized_packages: Arc<RwLock<HashMap<String, bool>>>,
}

impl Default for UnifiedCryptoManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UnifiedCryptoManager {
    /// slay Create a new unified crypto manager
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(CryptoConfig::default())),
            package_status: Arc::new(RwLock::new(HashMap::new())),
            performance_metrics: Arc::new(Mutex::new(Vec::new())),
            audit_results: Arc::new(Mutex::new(Vec::new())),
            initialized_packages: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// slay Initialize all enabled crypto packages
    pub fn initialize(&self) -> UnifiedCryptoResult<()> {
        let config = self.config.read()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to read config".to_string()))?;

        for package_name in &config.enabled_packages {
            self.initialize_package(package_name)?;
        }

        // Run initial security audit
        if config.compliance_level != ComplianceLevel::Development {
            self.perform_security_audit()?;
        }

        println!("🔐 Unified crypto manager initialized - all packages ready bestie!");
        Ok(())
    }

    /// slay Initialize a specific package
    pub fn initialize_package(&self, package_name: &str) -> UnifiedCryptoResult<()> {
        {
            let mut status = self.package_status.write()
                .map_err(|_| UnifiedCryptoError::Configuration("Failed to write package status".to_string()))?;
            status.insert(package_name.to_string(), PackageStatus::Initializing);
        }

        let result = match package_name {
            "crypto_advanced" => {
//                 crate::stdlib::packages::crypto_advanced::init_crypto_advanced()
                    .map_err(|e| UnifiedCryptoError::Integration(format!("crypto_advanced: {}", e)))
            },
            "crypto_asymmetric" => {
//                 crate::stdlib::packages::crypto_asymmetric::init_crypto_asymmetric()
                    .map_err(|e| UnifiedCryptoError::Integration(format!("crypto_asymmetric: {}", e)))
            },
            "crypto_hash_advanced" => {
//                 crate::stdlib::packages::crypto_hash_advanced::init_crypto_hash_advanced()
                    .map_err(|e| UnifiedCryptoError::Integration(format!("crypto_hash_advanced: {}", e)))
            },
            "crypto_signatures" => {
//                 crate::stdlib::packages::crypto_signatures::init_crypto_signatures()
                    .map_err(|e| UnifiedCryptoError::Integration(format!("crypto_signatures: {}", e)))
            },
            "crypto_kdf" => {
//                 crate::stdlib::packages::crypto_kdf::init_crypto_kdf()
                    .map_err(|e| UnifiedCryptoError::Integration(format!("crypto_kdf: {}", e)))
            },
            "crypto_random" => {
//                 crate::stdlib::packages::crypto_random::init_crypto_random()
                    .map_err(|e| UnifiedCryptoError::Integration(format!("crypto_random: {}", e)))
            },
            "crypto_pki" => {
//                 crate::stdlib::packages::crypto_pki::init_crypto_pki()
                    .map_err(|e| UnifiedCryptoError::Integration(format!("crypto_pki: {}", e)))
            },
            "crypto_zk" => {
//                 crate::stdlib::packages::crypto_zk::init_crypto_zk()
                    .map_err(|e| UnifiedCryptoError::Integration(format!("crypto_zk: {}", e)))
            },
            "crypto_pqc" => {
//                 crate::stdlib::packages::crypto_pqc::init_crypto_pqc()
                    .map_err(|e| UnifiedCryptoError::Integration(format!("crypto_pqc: {}", e)))
            },
            "crypto_protocols" => {
//                 crate::stdlib::packages::crypto_protocols::init_crypto_protocols()
                    .map_err(|e| UnifiedCryptoError::Integration(format!("crypto_protocols: {}", e)))
            },
            _ => Err(UnifiedCryptoError::UnsupportedAlgorithm(format!("Unknown package: {}", package_name))),
        };

        let mut status = self.package_status.write()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to write package status".to_string()))?;

        match result {
            Ok(_) => {
                status.insert(package_name.to_string(), PackageStatus::Ready);
                let mut initialized = self.initialized_packages.write()
                    .map_err(|_| UnifiedCryptoError::Configuration("Failed to write initialized packages".to_string()))?;
                initialized.insert(package_name.to_string(), true);
                Ok(())
            },
            Err(e) => {
                status.insert(package_name.to_string(), PackageStatus::CursedError(e.to_string()));
                Err(e)
            },
        }
    }

    /// slay Check if a package is ready
    pub fn is_package_ready(&self, package_name: &str) -> bool {
        self.package_status.read()
            .map(|status| {
                matches!(status.get(package_name), Some(PackageStatus::Ready))
            })
            .unwrap_or(false)
    }

    /// slay Get package status
    pub fn get_package_status(&self, package_name: &str) -> PackageStatus {
        self.package_status.read()
            .map(|status| status.get(package_name).cloned().unwrap_or(PackageStatus::NotInitialized))
            .unwrap_or(PackageStatus::NotInitialized)
    }

    /// slay Record performance metrics
    pub fn record_performance(&self, metrics: PerformanceMetrics) -> UnifiedCryptoResult<()> {
        let config = self.config.read()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to read config".to_string()))?;

        if !config.performance_monitoring {
            return Ok(());
        }

        // Check performance thresholds
        if metrics.duration > config.max_operation_duration {
            return Err(UnifiedCryptoError::PerformanceThreshold(
                format!("Operation took {:?}, maximum allowed: {:?}", 
                    metrics.duration, config.max_operation_duration)
            ));
        }

        let mut perf_metrics = self.performance_metrics.lock()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to lock performance metrics".to_string()))?;

        perf_metrics.push(metrics);

        // Keep only recent metrics (last 1000 operations)
        if perf_metrics.len() > 1000 {
            perf_metrics.drain(0..perf_metrics.len() - 1000);
        }

        Ok(())
    }

    /// slay Perform security audit
    pub fn perform_security_audit(&self) -> UnifiedCryptoResult<SecurityAuditResult> {
        let config = self.config.read()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to read config".to_string()))?;

        let mut algorithm_compliance = HashMap::new();
        let mut vulnerabilities = Vec::new();
        let mut recommendations = Vec::new();

        // Check algorithm compliance based on compliance level
        for (category, algorithm) in &config.default_algorithms {
            let is_compliant = match config.compliance_level {
                ComplianceLevel::Development => true, // All algorithms allowed in dev
                ComplianceLevel::Testing => !["MD5", "SHA-1", "DES", "3DES"].contains(&algorithm.as_str()),
                ComplianceLevel::Production => {
                    !["MD5", "SHA-1", "DES", "3DES", "RSA-1024"].contains(&algorithm.as_str())
                },
                ComplianceLevel::HighSecurity => {
                    matches!(algorithm.as_str(), 
                        "AES-256-GCM" | "ChaCha20-Poly1305" | "RSA-4096" | "Ed25519" | 
                        "SHA-256" | "SHA-384" | "SHA-512" | "BLAKE3")
                },
                ComplianceLevel::Enterprise => {
                    matches!(algorithm.as_str(),
                        "AES-256-GCM" | "ChaCha20-Poly1305" | "RSA-4096" | "Ed25519" |
                        "SHA-384" | "SHA-512" | "BLAKE3" | "Kyber-1024" | "Dilithium-5")
                },
                ComplianceLevel::Government => {
                    matches!(algorithm.as_str(),
                        "AES-256-GCM" | "RSA-4096" | "Ed25519" | "SHA-384" | "SHA-512" |
                        "Kyber-1024" | "Dilithium-5")
                },
            };

            algorithm_compliance.insert(format!("{}:{}", category, algorithm), is_compliant);

            if !is_compliant {
                vulnerabilities.push(format!("Non-compliant algorithm: {} for {}", algorithm, category));
                recommendations.push(format!("Upgrade {} algorithm for category {}", algorithm, category));
            }
        }

        // Check timing protection
        if !config.enable_timing_protection {
            vulnerabilities.push("Timing protection disabled".to_string());
            recommendations.push("Enable timing protection for production use".to_string());
        }

        // Check memory protection
        if !config.memory_protection {
            vulnerabilities.push("Memory protection disabled".to_string());
            recommendations.push("Enable memory protection for production use".to_string());
        }

        // Calculate overall score
        let compliant_count = algorithm_compliance.values().filter(|&&v| v).count();
        let total_count = algorithm_compliance.len();
        let base_score = if total_count > 0 {
            (compliant_count as f64 / total_count as f64) * 100.0
        } else {
            0.0
        };

        // Adjust score for security features
        let mut overall_score = base_score;
        if config.enable_timing_protection {
            overall_score += 5.0;
        }
        if config.memory_protection {
            overall_score += 5.0;
        }
        overall_score = overall_score.min(100.0);

        let audit_result = SecurityAuditResult {
            compliance_level: config.compliance_level.clone(),
            algorithm_compliance,
            security_vulnerabilities: vulnerabilities,
            recommendations,
            overall_score,
            timestamp: std::time::SystemTime::now(),
        };

        // Store audit result
        let mut audit_results = self.audit_results.lock()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to lock audit results".to_string()))?;
        audit_results.push(audit_result.clone());

        // Keep only recent audits (last 100)
        if audit_results.len() > 100 {
            audit_results.drain(0..audit_results.len() - 100);
        }

        Ok(audit_result)
    }

    /// slay Get performance statistics
    pub fn get_performance_statistics(&self) -> UnifiedCryptoResult<HashMap<String, Value>> {
        let metrics = self.performance_metrics.lock()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to lock performance metrics".to_string()))?;

        let mut stats = HashMap::new();
        stats.insert("total_operations".to_string(), Value::Number(metrics.len() as f64));

        if !metrics.is_empty() {
            let avg_duration = metrics.iter()
                .map(|m| m.duration.as_millis() as f64)
                .sum::<f64>() / metrics.len() as f64;
            stats.insert("average_duration_ms".to_string(), Value::Number(avg_duration));

            let max_duration = metrics.iter()
                .map(|m| m.duration.as_millis())
                .max()
                .unwrap_or(0) as f64;
            stats.insert("max_duration_ms".to_string(), Value::Number(max_duration));

            // Group by operation type
            let mut operation_counts: HashMap<String, u32> = HashMap::new();
            for metric in metrics.iter() {
                let key = format!("{:?}", metric.operation);
                *operation_counts.entry(key).or_insert(0) += 1;
            }

            let mut op_stats = HashMap::new();
            for (op, count) in operation_counts {
                op_stats.insert(op, Value::Number(count as f64));
            }
            stats.insert("operations_by_type".to_string(), Value::Object(op_stats));
        }

        Ok(stats)
    }

    /// slay Update configuration
    pub fn update_config(&self, new_config: CryptoConfig) -> UnifiedCryptoResult<()> {
        let mut config = self.config.write()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to write config".to_string()))?;
        *config = new_config;
        Ok(())
    }

    /// slay Get current configuration
    pub fn get_config(&self) -> UnifiedCryptoResult<CryptoConfig> {
        let config = self.config.read()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to read config".to_string()))?;
        Ok(config.clone())
    }

    /// slay Get latest security audit result
    pub fn get_latest_audit(&self) -> UnifiedCryptoResult<Option<SecurityAuditResult>> {
        let audit_results = self.audit_results.lock()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to lock audit results".to_string()))?;
        Ok(audit_results.last().cloned())
    }

    /// slay List all available algorithms across packages
    pub fn list_available_algorithms(&self) -> HashMap<String, Vec<String>> {
        let mut algorithms = HashMap::new();

        // Symmetric algorithms
        algorithms.insert("symmetric".to_string(), vec![
            "AES-128-GCM".to_string(),
            "AES-192-GCM".to_string(),
            "AES-256-GCM".to_string(),
            "ChaCha20-Poly1305".to_string(),
            "XChaCha20-Poly1305".to_string(),
        ]);

        // Asymmetric algorithms
        algorithms.insert("asymmetric".to_string(), vec![
            "RSA-2048".to_string(),
            "RSA-3072".to_string(),
            "RSA-4096".to_string(),
            "ECC-P256".to_string(),
            "ECC-P384".to_string(),
            "ECC-P521".to_string(),
            "X25519".to_string(),
            "Ed25519".to_string(),
        ]);

        // Hash algorithms
        algorithms.insert("hash".to_string(), vec![
            "SHA-256".to_string(),
            "SHA-384".to_string(),
            "SHA-512".to_string(),
            "SHA-3-256".to_string(),
            "SHA-3-384".to_string(),
            "SHA-3-512".to_string(),
            "BLAKE3".to_string(),
        ]);

        // Post-quantum algorithms
        algorithms.insert("pqc_kem".to_string(), vec![
            "Kyber-512".to_string(),
            "Kyber-768".to_string(),
            "Kyber-1024".to_string(),
        ]);

        algorithms.insert("pqc_signature".to_string(), vec![
            "Dilithium-2".to_string(),
            "Dilithium-3".to_string(),
            "Dilithium-5".to_string(),
            "Falcon-512".to_string(),
            "Falcon-1024".to_string(),
        ]);

        algorithms
    }
}

/// fr fr Global unified crypto manager instance
static GLOBAL_CRYPTO_MANAGER: std::sync::LazyLock<UnifiedCryptoManager> = 
    std::sync::LazyLock::new(|| UnifiedCryptoManager::new());

/// slay Get the global crypto manager
pub fn global_crypto_manager() -> &'static UnifiedCryptoManager {
    &GLOBAL_CRYPTO_MANAGER
}

/// slay Initialize unified crypto system
pub fn initialize_unified_crypto() -> UnifiedCryptoResult<()> {
    global_crypto_manager().initialize()
}

/// slay Quick crypto operation with performance monitoring
pub fn quick_crypto_operation<F, R>(
    operation: CryptoOperation,
    algorithm: &str,
    operation_fn: F,
) -> UnifiedCryptoResult<R>
where
    F: FnOnce() -> UnifiedCryptoResult<R>,
{
    let start_time = Instant::now();
    let result = operation_fn()?;
    let duration = start_time.elapsed();

    let metrics = PerformanceMetrics {
        operation,
        algorithm: algorithm.to_string(),
        duration,
        throughput_bytes_per_second: None,
        memory_usage_bytes: None,
        cpu_cycles: None,
        timestamp: std::time::SystemTime::now(),
    };

    global_crypto_manager().record_performance(metrics)?;
    Ok(result)
}

