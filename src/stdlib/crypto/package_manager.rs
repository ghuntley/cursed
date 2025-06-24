/// fr fr Crypto Package Manager - orchestrates all crypto functionality periodt
/// 
/// This module provides comprehensive package management for the CURSED crypto ecosystem,
/// including initialization, configuration, monitoring, and cross-package integration.
/// Think package management but make it cryptographically secure bestie!

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, SystemTime, Instant};
use serde::{Serialize, Deserialize};

use crate::error::CursedError;
use crate::stdlib::value::Value;
use super::unified_api::{
    UnifiedCryptoError, UnifiedCryptoResult, UnifiedCryptoManager, CryptoConfig, 
    PerformanceMetrics, SecurityAuditResult, CryptoOperation
};
use crate::error::Error;
use super::integration_manager::{CryptoIntegrationManager, IntegrationTestResult};

/// fr fr Package information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub algorithms: Vec<String>,
    pub dependencies: Vec<String>,
    pub features: Vec<String>,
    pub security_level: String,
    pub performance_tier: String,
    pub last_updated: SystemTime,
    pub initialization_time: Option<Duration>,
}

/// fr fr Package registry entry
#[derive(Debug, Clone)]
pub struct PackageRegistryEntry {
    pub info: PackageInfo,
    pub status: PackageStatus,
    pub init_function: Option<fn() -> Result<(), Error>>,
    pub test_function: Option<fn() -> UnifiedCryptoResult<HashMap<String, bool>>>,
    pub capabilities: PackageCapabilities,
}

/// fr fr Package status
#[derive(Debug, Clone, PartialEq)]
pub enum PackageStatus {
    NotRegistered,
    Registered,
    Initializing,
    Ready,
    Failed(String),
    Updating,
    Disabled,
}

/// fr fr Package capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageCapabilities {
    pub encryption: bool,
    pub decryption: bool,
    pub key_generation: bool,
    pub key_derivation: bool,
    pub digital_signatures: bool,
    pub hash_functions: bool,
    pub random_generation: bool,
    pub certificate_handling: bool,
    pub post_quantum: bool,
    pub zero_knowledge: bool,
    pub protocols: bool,
    pub hardware_support: bool,
}

impl Default for PackageCapabilities {
    fn default() -> Self {
        Self {
            encryption: false,
            decryption: false,
            key_generation: false,
            key_derivation: false,
            digital_signatures: false,
            hash_functions: false,
            random_generation: false,
            certificate_handling: false,
            post_quantum: false,
            zero_knowledge: false,
            protocols: false,
            hardware_support: false,
        }
    }
}

/// fr fr Crypto package statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageStatistics {
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub average_duration: Duration,
    pub peak_memory_usage: u64,
    pub last_operation: Option<SystemTime>,
    pub uptime: Duration,
    pub error_rate: f64,
}

impl Default for PackageStatistics {
    fn default() -> Self {
        Self {
            total_operations: 0,
            successful_operations: 0,
            failed_operations: 0,
            average_duration: Duration::from_millis(0),
            peak_memory_usage: 0,
            last_operation: None,
            uptime: Duration::from_secs(0),
            error_rate: 0.0,
        }
    }
}

/// fr fr Comprehensive crypto package manager
pub struct CryptoPackageManager {
    registry: Arc<RwLock<HashMap<String, PackageRegistryEntry>>>,
    unified_manager: Arc<UnifiedCryptoManager>,
    integration_manager: Arc<CryptoIntegrationManager>,
    package_stats: Arc<RwLock<HashMap<String, PackageStatistics>>>,
    global_config: Arc<RwLock<CryptoConfig>>,
    startup_time: Instant,
    monitoring_enabled: Arc<RwLock<bool>>,
}

impl Default for CryptoPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptoPackageManager {
    /// slay Create a new crypto package manager
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(HashMap::new())),
            unified_manager: Arc::new(UnifiedCryptoManager::new()),
            integration_manager: Arc::new(CryptoIntegrationManager::new()),
            package_stats: Arc::new(RwLock::new(HashMap::new())),
            global_config: Arc::new(RwLock::new(CryptoConfig::default())),
            startup_time: Instant::now(),
            monitoring_enabled: Arc::new(RwLock::new(true)),
        }
    }

    /// slay Initialize the crypto package manager
    pub fn initialize(&self) -> UnifiedCryptoResult<()> {
        println!("🔐 Initializing CURSED Crypto Package Manager...");

        // Register all crypto packages
        self.register_all_packages()?;

        // Initialize unified crypto manager
        self.unified_manager.initialize()?;

        // Initialize integration manager
        self.integration_manager.initialize_integration_testing()?;

        // Initialize all registered packages
        self.initialize_all_packages()?;

        // Run comprehensive tests
        self.run_comprehensive_tests()?;

        // Perform initial security audit
        self.perform_security_audit()?;

        println!("🔐 Crypto Package Manager initialized successfully - all systems ready bestie!");
        Ok(())
    }

    /// slay Register all crypto packages
    fn register_all_packages(&self) -> UnifiedCryptoResult<()> {
        let packages = vec![
            self.create_crypto_advanced_entry(),
            self.create_crypto_asymmetric_entry(),
            self.create_crypto_hash_advanced_entry(),
            self.create_crypto_signatures_entry(),
            self.create_crypto_kdf_entry(),
            self.create_crypto_random_entry(),
            self.create_crypto_pki_entry(),
            self.create_crypto_zk_entry(),
            self.create_crypto_pqc_entry(),
            self.create_crypto_protocols_entry(),
        ];

        let mut registry = self.registry.write()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to write registry".to_string()))?;

        for entry in packages {
            let name = entry.info.name.clone();
            registry.insert(name.clone(), entry);
            
            // Initialize package statistics
            let mut stats = self.package_stats.write()
                .map_err(|_| UnifiedCryptoError::Configuration("Failed to write stats".to_string()))?;
            stats.insert(name, PackageStatistics::default());
        }

        Ok(())
    }

    /// slay Create crypto_advanced package entry
    fn create_crypto_advanced_entry(&self) -> PackageRegistryEntry {
        let mut capabilities = PackageCapabilities::default();
        capabilities.encryption = true;
        capabilities.decryption = true;
        capabilities.key_generation = true;

        PackageRegistryEntry {
            info: PackageInfo {
                name: "crypto_advanced".to_string(),
                version: "1.0.0".to_string(),
                description: "Advanced symmetric encryption algorithms".to_string(),
                algorithms: vec![
                    "AES-128-GCM".to_string(),
                    "AES-192-GCM".to_string(),
                    "AES-256-GCM".to_string(),
                    "ChaCha20-Poly1305".to_string(),
                    "XChaCha20-Poly1305".to_string(),
                ],
                dependencies: vec!["crypto_random".to_string()],
                features: vec![
                    "authenticated_encryption".to_string(),
                    "constant_time".to_string(),
                    "memory_protection".to_string(),
                ],
                security_level: "High".to_string(),
                performance_tier: "Optimized".to_string(),
                last_updated: SystemTime::now(),
                initialization_time: None,
            },
            status: PackageStatus::Registered,
            init_function: Some(|| {
                crate::stdlib::packages::crypto_advanced::init_crypto_advanced()
                    .map_err(|e| CursedError::Runtime(e.to_string()))
            }),
            test_function: None,
            capabilities,
        }
    }

    /// slay Create crypto_asymmetric package entry
    fn create_crypto_asymmetric_entry(&self) -> PackageRegistryEntry {
        let mut capabilities = PackageCapabilities::default();
        capabilities.encryption = true;
        capabilities.decryption = true;
        capabilities.key_generation = true;
        capabilities.digital_signatures = true;

        PackageRegistryEntry {
            info: PackageInfo {
                name: "crypto_asymmetric".to_string(),
                version: "1.0.0".to_string(),
                description: "Asymmetric cryptography and key exchange".to_string(),
                algorithms: vec![
                    "RSA-2048".to_string(),
                    "RSA-3072".to_string(),
                    "RSA-4096".to_string(),
                    "ECC-P256".to_string(),
                    "ECC-P384".to_string(),
                    "ECC-P521".to_string(),
                    "X25519".to_string(),
                    "Ed25519".to_string(),
                ],
                dependencies: vec!["crypto_random".to_string(), "crypto_hash_advanced".to_string()],
                features: vec![
                    "key_exchange".to_string(),
                    "digital_signatures".to_string(),
                    "elliptic_curves".to_string(),
                ],
                security_level: "High".to_string(),
                performance_tier: "Standard".to_string(),
                last_updated: SystemTime::now(),
                initialization_time: None,
            },
            status: PackageStatus::Registered,
            init_function: Some(|| {
                crate::stdlib::packages::crypto_asymmetric::init_crypto_asymmetric()
                    .map_err(|e| CursedError::Runtime(e.to_string()))
            }),
            test_function: None,
            capabilities,
        }
    }

    /// slay Create other package entries (simplified for brevity)
    fn create_crypto_hash_advanced_entry(&self) -> PackageRegistryEntry {
        let mut capabilities = PackageCapabilities::default();
        capabilities.hash_functions = true;

        PackageRegistryEntry {
            info: PackageInfo {
                name: "crypto_hash_advanced".to_string(),
                version: "1.0.0".to_string(),
                description: "Advanced cryptographic hash functions".to_string(),
                algorithms: vec![
                    "SHA-256".to_string(), "SHA-384".to_string(), "SHA-512".to_string(),
                    "SHA-3-256".to_string(), "SHA-3-384".to_string(), "SHA-3-512".to_string(),
                    "BLAKE3".to_string(), "HMAC".to_string(),
                ],
                dependencies: vec![],
                features: vec!["hmac".to_string(), "sha3".to_string(), "blake3".to_string()],
                security_level: "High".to_string(),
                performance_tier: "Optimized".to_string(),
                last_updated: SystemTime::now(),
                initialization_time: None,
            },
            status: PackageStatus::Registered,
            init_function: Some(|| {
                crate::stdlib::packages::crypto_hash_advanced::init_crypto_hash_advanced()
                    .map_err(|e| CursedError::Runtime(e.to_string()))
            }),
            test_function: None,
            capabilities,
        }
    }

    fn create_crypto_signatures_entry(&self) -> PackageRegistryEntry {
        let mut capabilities = PackageCapabilities::default();
        capabilities.digital_signatures = true;

        PackageRegistryEntry {
            info: PackageInfo {
                name: "crypto_signatures".to_string(),
                version: "1.0.0".to_string(),
                description: "Digital signature algorithms".to_string(),
                algorithms: vec!["Ed25519".to_string(), "ECDSA".to_string(), "RSA-PSS".to_string()],
                dependencies: vec!["crypto_asymmetric".to_string(), "crypto_hash_advanced".to_string()],
                features: vec!["verification".to_string(), "batch_verification".to_string()],
                security_level: "High".to_string(),
                performance_tier: "Standard".to_string(),
                last_updated: SystemTime::now(),
                initialization_time: None,
            },
            status: PackageStatus::Registered,
            init_function: Some(|| {
                crate::stdlib::packages::crypto_signatures::init_crypto_signatures()
                    .map_err(|e| CursedError::Runtime(e.to_string()))
            }),
            test_function: None,
            capabilities,
        }
    }

    fn create_crypto_kdf_entry(&self) -> PackageRegistryEntry {
        let mut capabilities = PackageCapabilities::default();
        capabilities.key_derivation = true;

        PackageRegistryEntry {
            info: PackageInfo {
                name: "crypto_kdf".to_string(),
                version: "1.0.0".to_string(),
                description: "Key derivation functions".to_string(),
                algorithms: vec!["PBKDF2".to_string(), "Argon2".to_string(), "HKDF".to_string(), "scrypt".to_string()],
                dependencies: vec!["crypto_hash_advanced".to_string()],
                features: vec!["pbkdf2".to_string(), "argon2".to_string(), "hkdf".to_string()],
                security_level: "High".to_string(),
                performance_tier: "Standard".to_string(),
                last_updated: SystemTime::now(),
                initialization_time: None,
            },
            status: PackageStatus::Registered,
            init_function: Some(|| {
                crate::stdlib::packages::crypto_kdf::init_crypto_kdf()
                    .map_err(|e| CursedError::Runtime(e.to_string()))
            }),
            test_function: None,
            capabilities,
        }
    }

    fn create_crypto_random_entry(&self) -> PackageRegistryEntry {
        let mut capabilities = PackageCapabilities::default();
        capabilities.random_generation = true;

        PackageRegistryEntry {
            info: PackageInfo {
                name: "crypto_random".to_string(),
                version: "1.0.0".to_string(),
                description: "Cryptographically secure random number generation".to_string(),
                algorithms: vec!["ChaCha20Rng".to_string(), "SystemRandom".to_string()],
                dependencies: vec![],
                features: vec!["secure_random".to_string(), "hardware_random".to_string()],
                security_level: "Critical".to_string(),
                performance_tier: "Optimized".to_string(),
                last_updated: SystemTime::now(),
                initialization_time: None,
            },
            status: PackageStatus::Registered,
            init_function: Some(|| {
                crate::stdlib::packages::crypto_random::init_crypto_random()
                    .map_err(|e| CursedError::Runtime(e.to_string()))
            }),
            test_function: None,
            capabilities,
        }
    }

    fn create_crypto_pki_entry(&self) -> PackageRegistryEntry {
        let mut capabilities = PackageCapabilities::default();
        capabilities.certificate_handling = true;
        capabilities.digital_signatures = true;

        PackageRegistryEntry {
            info: PackageInfo {
                name: "crypto_pki".to_string(),
                version: "1.0.0".to_string(),
                description: "Public Key Infrastructure and certificates".to_string(),
                algorithms: vec!["X.509".to_string(), "PKCS#10".to_string(), "PKCS#12".to_string()],
                dependencies: vec!["crypto_asymmetric".to_string(), "crypto_signatures".to_string()],
                features: vec!["certificate_validation".to_string(), "csr_processing".to_string()],
                security_level: "High".to_string(),
                performance_tier: "Standard".to_string(),
                last_updated: SystemTime::now(),
                initialization_time: None,
            },
            status: PackageStatus::Registered,
            init_function: Some(|| {
                crate::stdlib::packages::crypto_pki::init_crypto_pki()
                    .map_err(|e| CursedError::Runtime(e.to_string()))
            }),
            test_function: None,
            capabilities,
        }
    }

    fn create_crypto_zk_entry(&self) -> PackageRegistryEntry {
        let mut capabilities = PackageCapabilities::default();
        capabilities.zero_knowledge = true;

        PackageRegistryEntry {
            info: PackageInfo {
                name: "crypto_zk".to_string(),
                version: "1.0.0".to_string(),
                description: "Zero-knowledge proof systems".to_string(),
                algorithms: vec!["ZK-SNARKs".to_string(), "ZK-STARKs".to_string(), "Bulletproofs".to_string()],
                dependencies: vec!["crypto_random".to_string(), "crypto_hash_advanced".to_string()],
                features: vec!["zkp_verification".to_string(), "proof_generation".to_string()],
                security_level: "Research".to_string(),
                performance_tier: "Experimental".to_string(),
                last_updated: SystemTime::now(),
                initialization_time: None,
            },
            status: PackageStatus::Registered,
            init_function: Some(|| {
                crate::stdlib::packages::crypto_zk::init_crypto_zk()
                    .map_err(|e| CursedError::Runtime(e.to_string()))
            }),
            test_function: None,
            capabilities,
        }
    }

    fn create_crypto_pqc_entry(&self) -> PackageRegistryEntry {
        let mut capabilities = PackageCapabilities::default();
        capabilities.post_quantum = true;
        capabilities.encryption = true;
        capabilities.digital_signatures = true;

        PackageRegistryEntry {
            info: PackageInfo {
                name: "crypto_pqc".to_string(),
                version: "1.0.0".to_string(),
                description: "Post-quantum cryptography".to_string(),
                algorithms: vec![
                    "Kyber-512".to_string(), "Kyber-768".to_string(), "Kyber-1024".to_string(),
                    "Dilithium-2".to_string(), "Dilithium-3".to_string(), "Dilithium-5".to_string(),
                ],
                dependencies: vec!["crypto_random".to_string(), "crypto_hash_advanced".to_string()],
                features: vec!["quantum_resistant".to_string(), "hybrid_modes".to_string()],
                security_level: "Future".to_string(),
                performance_tier: "Standard".to_string(),
                last_updated: SystemTime::now(),
                initialization_time: None,
            },
            status: PackageStatus::Registered,
            init_function: Some(|| {
                crate::stdlib::packages::crypto_pqc::init_crypto_pqc()
                    .map_err(|e| CursedError::Runtime(e.to_string()))
            }),
            test_function: None,
            capabilities,
        }
    }

    fn create_crypto_protocols_entry(&self) -> PackageRegistryEntry {
        let mut capabilities = PackageCapabilities::default();
        capabilities.protocols = true;
        capabilities.key_generation = true;

        PackageRegistryEntry {
            info: PackageInfo {
                name: "crypto_protocols".to_string(),
                version: "1.0.0".to_string(),
                description: "Cryptographic protocols and schemes".to_string(),
                algorithms: vec![
                    "TLS".to_string(), "Noise".to_string(), "Signal".to_string(),
                    "Diffie-Hellman".to_string(), "ECDH".to_string(),
                ],
                dependencies: vec![
                    "crypto_advanced".to_string(), "crypto_asymmetric".to_string(), "crypto_kdf".to_string()
                ],
                features: vec!["key_exchange".to_string(), "secure_channels".to_string()],
                security_level: "High".to_string(),
                performance_tier: "Standard".to_string(),
                last_updated: SystemTime::now(),
                initialization_time: None,
            },
            status: PackageStatus::Registered,
            init_function: Some(|| {
                crate::stdlib::packages::crypto_protocols::init_crypto_protocols()
                    .map_err(|e| CursedError::Runtime(e.to_string()))
            }),
            test_function: None,
            capabilities,
        }
    }

    /// slay Initialize all registered packages
    fn initialize_all_packages(&self) -> UnifiedCryptoResult<()> {
        let package_names: Vec<String> = {
            let registry = self.registry.read()
                .map_err(|_| UnifiedCryptoError::Configuration("Failed to read registry".to_string()))?;
            registry.keys().cloned().collect()
        };

        for package_name in package_names {
            self.initialize_package(&package_name)?;
        }

        Ok(())
    }

    /// slay Initialize a specific package
    pub fn initialize_package(&self, package_name: &str) -> UnifiedCryptoResult<()> {
        let start_time = Instant::now();

        println!("📦 Initializing package: {}", package_name);

        // Update status to initializing
        {
            let mut registry = self.registry.write()
                .map_err(|_| UnifiedCryptoError::Configuration("Failed to write registry".to_string()))?;
            
            if let Some(entry) = registry.get_mut(package_name) {
                entry.status = PackageStatus::Initializing;
            }
        }

        // Run initialization function
        let init_result = {
            let registry = self.registry.read()
                .map_err(|_| UnifiedCryptoError::Configuration("Failed to read registry".to_string()))?;
            
            if let Some(entry) = registry.get(package_name) {
                if let Some(init_fn) = entry.init_function {
                    init_fn().map_err(|e| UnifiedCryptoError::Integration(e.to_string()))
                } else {
                    Ok(())
                }
            } else {
                Err(UnifiedCryptoError::PackageNotInitialized(format!("Package not found: {}", package_name)))
            }
        };

        let initialization_time = start_time.elapsed();

        // Update status based on result
        {
            let mut registry = self.registry.write()
                .map_err(|_| UnifiedCryptoError::Configuration("Failed to write registry".to_string()))?;
            
            if let Some(entry) = registry.get_mut(package_name) {
                match init_result {
                    Ok(_) => {
                        entry.status = PackageStatus::Ready;
                        entry.info.initialization_time = Some(initialization_time);
                        println!("✅ Package {} initialized in {:?}", package_name, initialization_time);
                    },
                    Err(ref e) => {
                        entry.status = PackageStatus::Failed(e.to_string());
                        println!("❌ Package {} failed to initialize: {}", package_name, e);
                    },
                }
            }
        }

        init_result
    }

    /// slay Run comprehensive tests across all packages
    fn run_comprehensive_tests(&self) -> UnifiedCryptoResult<()> {
        println!("🧪 Running comprehensive crypto package tests...");

        // Test individual packages
        let package_names: Vec<String> = {
            let registry = self.registry.read()
                .map_err(|_| UnifiedCryptoError::Configuration("Failed to read registry".to_string()))?;
            registry.keys().cloned().collect()
        };

        for package_name in &package_names {
            if let Err(e) = self.test_package(package_name) {
                println!("⚠️  Package {} test failed: {}", package_name, e);
            }
        }

        // Run integration tests
        let integration_results = self.integration_manager.run_basic_integration_tests();
        match integration_results {
            Ok(_) => println!("✅ Integration tests completed successfully"),
            Err(e) => println!("⚠️  Integration tests failed: {}", e),
        }

        Ok(())
    }

    /// slay Test a specific package
    pub fn test_package(&self, package_name: &str) -> UnifiedCryptoResult<HashMap<String, bool>> {
        let registry = self.registry.read()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to read registry".to_string()))?;

        if let Some(entry) = registry.get(package_name) {
            if let Some(test_fn) = entry.test_function {
                test_fn()
            } else {
                // Default test - check if package is ready
                let mut results = HashMap::new();
                results.insert("status_check".to_string(), entry.status == PackageStatus::Ready);
                Ok(results)
            }
        } else {
            Err(UnifiedCryptoError::PackageNotInitialized(format!("Package not found: {}", package_name)))
        }
    }

    /// slay Perform security audit
    pub fn perform_security_audit(&self) -> UnifiedCryptoResult<SecurityAuditResult> {
        println!("🔍 Performing comprehensive security audit...");
        self.unified_manager.perform_security_audit()
    }

    /// slay Get package information
    pub fn get_package_info(&self, package_name: &str) -> UnifiedCryptoResult<PackageInfo> {
        let registry = self.registry.read()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to read registry".to_string()))?;

        registry.get(package_name)
            .map(|entry| entry.info.clone())
            .ok_or_else(|| UnifiedCryptoError::PackageNotInitialized(format!("Package not found: {}", package_name)))
    }

    /// slay List all packages
    pub fn list_packages(&self) -> UnifiedCryptoResult<Vec<PackageInfo>> {
        let registry = self.registry.read()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to read registry".to_string()))?;

        Ok(registry.values().map(|entry| entry.info.clone()).collect())
    }

    /// slay Get package statistics
    pub fn get_package_statistics(&self, package_name: &str) -> UnifiedCryptoResult<PackageStatistics> {
        let stats = self.package_stats.read()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to read stats".to_string()))?;

        stats.get(package_name)
            .cloned()
            .ok_or_else(|| UnifiedCryptoError::PackageNotInitialized(format!("Package not found: {}", package_name)))
    }

    /// slay Get system overview
    pub fn get_system_overview(&self) -> UnifiedCryptoResult<HashMap<String, Value>> {
        let mut overview = HashMap::new();

        // Basic info
        overview.insert("uptime".to_string(), Value::Number(self.startup_time.elapsed().as_secs() as f64));
        overview.insert("total_packages".to_string(), Value::Number(self.list_packages()?.len() as f64));

        // Package status summary
        let registry = self.registry.read()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to read registry".to_string()))?;

        let mut status_counts = HashMap::new();
        for entry in registry.values() {
            let status_name = match &entry.status {
                PackageStatus::NotRegistered => "not_registered",
                PackageStatus::Registered => "registered",
                PackageStatus::Initializing => "initializing",
                PackageStatus::Ready => "ready",
                PackageStatus::Failed(_) => "failed",
                PackageStatus::Updating => "updating",
                PackageStatus::Disabled => "disabled",
            };
            *status_counts.entry(status_name.to_string()).or_insert(0) += 1;
        }

        let mut status_obj = HashMap::new();
        for (status, count) in status_counts {
            status_obj.insert(status, Value::Number(count as f64));
        }
        overview.insert("package_status".to_string(), Value::Object(status_obj));

        // Performance metrics
        if let Ok(perf_stats) = self.unified_manager.get_performance_statistics() {
            overview.insert("performance".to_string(), Value::Object(perf_stats));
        }

        Ok(overview)
    }

    /// slay Get global configuration
    pub fn get_global_config(&self) -> UnifiedCryptoResult<CryptoConfig> {
        let config = self.global_config.read()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to read config".to_string()))?;
        Ok(config.clone())
    }

    /// slay Update global configuration
    pub fn update_global_config(&self, new_config: CryptoConfig) -> UnifiedCryptoResult<()> {
        let mut config = self.global_config.write()
            .map_err(|_| UnifiedCryptoError::Configuration("Failed to write config".to_string()))?;
        *config = new_config.clone();

        // Update unified manager config
        self.unified_manager.update_config(new_config)?;

        Ok(())
    }
}

/// fr fr Global crypto package manager instance
static GLOBAL_PACKAGE_MANAGER: std::sync::LazyLock<CryptoPackageManager> = 
    std::sync::LazyLock::new(|| CryptoPackageManager::new());

/// slay Get the global package manager
pub fn global_package_manager() -> &'static CryptoPackageManager {
    &GLOBAL_PACKAGE_MANAGER
}

/// slay Initialize the crypto package ecosystem
pub fn initialize_crypto_ecosystem() -> UnifiedCryptoResult<()> {
    global_package_manager().initialize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_manager_creation() {
        let manager = CryptoPackageManager::new();
        assert!(manager.get_global_config().is_ok());
    }

    #[test]
    fn test_package_registration() {
        let manager = CryptoPackageManager::new();
        let result = manager.register_all_packages();
        assert!(result.is_ok());

        let packages = manager.list_packages().unwrap();
        assert!(!packages.is_empty());
        assert!(packages.iter().any(|p| p.name == "crypto_advanced"));
    }

    #[test]
    fn test_package_capabilities() {
        let capabilities = PackageCapabilities::default();
        assert!(!capabilities.encryption);
        assert!(!capabilities.digital_signatures);
    }

    #[test]
    fn test_package_statistics() {
        let stats = PackageStatistics::default();
        assert_eq!(stats.total_operations, 0);
        assert_eq!(stats.error_rate, 0.0);
    }

    #[test]
    fn test_system_overview() {
        let manager = CryptoPackageManager::new();
        let _ = manager.register_all_packages();
        
        let overview = manager.get_system_overview();
        assert!(overview.is_ok());

        let overview_data = overview.unwrap();
        assert!(overview_data.contains_key("uptime"));
        assert!(overview_data.contains_key("total_packages"));
    }
}
