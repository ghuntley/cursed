//! Post-Quantum Cryptography Key Management
//! 
//! This module provides comprehensive key management functionality for PQC algorithms,
//! including key generation, serialization, deserialization, validation, and lifecycle management.

use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use serde::{Serialize, Deserialize};
use crate::stdlib::crypto_pqc::{PqcResult, PqcError, SecurityLevel, AlgorithmType, StandardizationStatus};

/// Universal key container for all PQC algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcKey {
    /// Algorithm type
    pub algorithm: AlgorithmType,
    /// Security level
    pub security_level: SecurityLevel,
    /// Key type (public or secret)
    pub key_type: KeyType,
    /// Key data
    pub key_data: Vec<u8>,
    /// Key metadata
    pub metadata: KeyMetadata,
}

/// Key type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    Public,
    Secret,
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyType::Public => write!(f, "public"),
            KeyType::Secret => write!(f, "secret"),
        }
    }
}

/// Key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Key identifier (UUID)
    pub key_id: String,
    /// Creation timestamp
    pub created_at: u64,
    /// Expiration timestamp (optional)
    pub expires_at: Option<u64>,
    /// Key usage flags
    pub usage: KeyUsage,
    /// Key format
    pub format: KeyFormat,
    /// Additional parameters specific to the algorithm
    pub algorithm_params: HashMap<String, String>,
    /// Key derivation information (if applicable)
    pub derivation_info: Option<KeyDerivationInfo>,
}

/// Key usage flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyUsage {
    /// Can be used for encryption/encapsulation
    pub encrypt: bool,
    /// Can be used for decryption/decapsulation
    pub decrypt: bool,
    /// Can be used for signing
    pub sign: bool,
    /// Can be used for verification
    pub verify: bool,
    /// Can be used for key agreement
    pub key_agreement: bool,
    /// Can be used for key derivation
    pub key_derivation: bool,
}

impl KeyUsage {
    /// Create usage flags for KEM public keys
    pub fn kem_public() -> Self {
        Self {
            encrypt: true,
            decrypt: false,
            sign: false,
            verify: false,
            key_agreement: true,
            key_derivation: false,
        }
    }

    /// Create usage flags for KEM secret keys
    pub fn kem_secret() -> Self {
        Self {
            encrypt: false,
            decrypt: true,
            sign: false,
            verify: false,
            key_agreement: true,
            key_derivation: true,
        }
    }

    /// Create usage flags for signature public keys
    pub fn signature_public() -> Self {
        Self {
            encrypt: false,
            decrypt: false,
            sign: false,
            verify: true,
            key_agreement: false,
            key_derivation: false,
        }
    }

    /// Create usage flags for signature secret keys
    pub fn signature_secret() -> Self {
        Self {
            encrypt: false,
            decrypt: false,
            sign: true,
            verify: false,
            key_agreement: false,
            key_derivation: false,
        }
    }

    /// Create usage flags for encryption public keys
    pub fn encryption_public() -> Self {
        Self {
            encrypt: true,
            decrypt: false,
            sign: false,
            verify: false,
            key_agreement: false,
            key_derivation: false,
        }
    }

    /// Create usage flags for encryption secret keys
    pub fn encryption_secret() -> Self {
        Self {
            encrypt: false,
            decrypt: true,
            sign: false,
            verify: false,
            key_agreement: false,
            key_derivation: true,
        }
    }
}

/// Key formats supported
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyFormat {
    /// Raw binary format
    Raw,
    /// DER encoding
    Der,
    /// PEM encoding
    Pem,
    /// JSON Web Key (JWK)
    Jwk,
    /// CURSED native format
    CursedNative,
}

impl fmt::Display for KeyFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyFormat::Raw => write!(f, "raw"),
            KeyFormat::Der => write!(f, "der"),
            KeyFormat::Pem => write!(f, "pem"),
            KeyFormat::Jwk => write!(f, "jwk"),
            KeyFormat::CursedNative => write!(f, "cursed-native"),
        }
    }
}

/// Key derivation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationInfo {
    /// Parent key ID (if derived)
    pub parent_key_id: Option<String>,
    /// Derivation method
    pub method: String,
    /// Derivation parameters
    pub params: HashMap<String, String>,
    /// Salt used in derivation
    pub salt: Option<Vec<u8>>,
}

impl PqcKey {
    /// Create a new PQC key
    pub fn new(
        algorithm: AlgorithmType,
        security_level: SecurityLevel,
        key_type: KeyType,
        key_data: Vec<u8>,
    ) -> Self {
        let key_id = Self::generate_key_id();
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let usage = match (algorithm, key_type) {
            (AlgorithmType::Kyber | AlgorithmType::FrodoKem | 
             AlgorithmType::ClassicMcEliece | AlgorithmType::Bike | 
             AlgorithmType::Hqc, KeyType::Public) => KeyUsage::kem_public(),
            (AlgorithmType::Kyber | AlgorithmType::FrodoKem | 
             AlgorithmType::ClassicMcEliece | AlgorithmType::Bike | 
             AlgorithmType::Hqc, KeyType::Secret) => KeyUsage::kem_secret(),
            (AlgorithmType::Dilithium | AlgorithmType::Sphincs | 
             AlgorithmType::Lms | AlgorithmType::Xmss | 
             AlgorithmType::Rainbow | AlgorithmType::GeMSS, KeyType::Public) => KeyUsage::signature_public(),
            (AlgorithmType::Dilithium | AlgorithmType::Sphincs | 
             AlgorithmType::Lms | AlgorithmType::Xmss | 
             AlgorithmType::Rainbow | AlgorithmType::GeMSS, KeyType::Secret) => KeyUsage::signature_secret(),
            (AlgorithmType::Ntru, KeyType::Public) => KeyUsage::encryption_public(),
            (AlgorithmType::Ntru, KeyType::Secret) => KeyUsage::encryption_secret(),
            (AlgorithmType::Sike, _) => KeyUsage::kem_public(), // Deprecated
        };

        Self {
            algorithm,
            security_level,
            key_type,
            key_data,
            metadata: KeyMetadata {
                key_id,
                created_at,
                expires_at: None,
                usage,
                format: KeyFormat::Raw,
                algorithm_params: HashMap::new(),
                derivation_info: None,
            },
        }
    }

    /// Generate a unique key ID
    fn generate_key_id() -> String {
        use rand::RngCore;
        use rand::rngs::OsRng;
        
        let mut bytes = [0u8; 16];
        OsRng.fill_bytes(&mut bytes);
        
        // Format as UUID v4
        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-4{:01x}{:02x}-{:01x}{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6] & 0x0f, bytes[7],
            (bytes[8] & 0x3f) | 0x80, bytes[9], bytes[10],
            bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        )
    }

    /// Set expiration time
    pub fn set_expiration(&mut self, duration: Duration) {
        let expires_at = self.metadata.created_at + duration.as_secs();
        self.metadata.expires_at = Some(expires_at);
    }

    /// Check if key is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.metadata.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            now > expires_at
        } else {
            false
        }
    }

    /// Get key age in seconds
    pub fn age_seconds(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.metadata.created_at)
    }

    /// Validate the key
    pub fn validate(&self) -> PqcResult<()> {
        // Check if expired
        if self.is_expired() {
            return Err(PqcError::InvalidKey("Key has expired".to_string()));
        }

        // Check key data size constraints
        if self.key_data.is_empty() {
            return Err(PqcError::InvalidKey("Key data is empty".to_string()));
        }

        // Check if algorithm is deprecated
        if self.algorithm == AlgorithmType::Sike {
            return Err(PqcError::InvalidKey("SIKE algorithm is deprecated and broken".to_string()));
        }

        // Validate usage flags match key type
        match self.key_type {
            KeyType::Public => {
                if self.metadata.usage.decrypt || self.metadata.usage.sign || self.metadata.usage.key_derivation {
                    return Err(PqcError::InvalidKey("Public key cannot have private key usage flags".to_string()));
                }
            },
            KeyType::Secret => {
                if !self.metadata.usage.decrypt && !self.metadata.usage.sign && !self.metadata.usage.key_derivation {
                    return Err(PqcError::InvalidKey("Secret key must have at least one private operation usage flag".to_string()));
                }
            },
        }

        Ok(())
    }

    /// Get standardization status for this key's algorithm
    pub fn standardization_status(&self) -> StandardizationStatus {
        StandardizationStatus::for_algorithm(self.algorithm)
    }

    /// Check if this key is production ready
    pub fn is_production_ready(&self) -> bool {
        self.standardization_status().is_production_ready()
    }

    /// Add algorithm-specific parameter
    pub fn set_algorithm_param(&mut self, key: String, value: String) {
        self.metadata.algorithm_params.insert(key, value);
    }

    /// Get algorithm-specific parameter
    pub fn get_algorithm_param(&self, key: &str) -> Option<&String> {
        self.metadata.algorithm_params.get(key)
    }

    /// Set key derivation information
    pub fn set_derivation_info(&mut self, info: KeyDerivationInfo) {
        self.metadata.derivation_info = Some(info);
    }

    /// Clone the key with a new ID (for key rotation)
    pub fn clone_with_new_id(&self) -> Self {
        let mut new_key = self.clone();
        new_key.metadata.key_id = Self::generate_key_id();
        new_key.metadata.created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        new_key
    }
}

/// Key manager for handling multiple keys
#[derive(Debug, Clone)]
pub struct KeyManager {
    /// Storage for keys indexed by key ID
    keys: HashMap<String, PqcKey>,
    /// Default algorithms for each security level
    default_algorithms: HashMap<SecurityLevel, AlgorithmType>,
}

impl KeyManager {
    /// Create a new key manager
    pub fn new() -> Self {
        let mut default_algorithms = HashMap::new();
        default_algorithms.insert(SecurityLevel::Level1, AlgorithmType::Kyber);
        default_algorithms.insert(SecurityLevel::Level3, AlgorithmType::Kyber);
        default_algorithms.insert(SecurityLevel::Level5, AlgorithmType::Kyber);

        Self {
            keys: HashMap::new(),
            default_algorithms,
        }
    }

    /// Add a key to the manager
    pub fn add_key(&mut self, key: PqcKey) -> PqcResult<String> {
        key.validate()?;
        let key_id = key.metadata.key_id.clone();
        self.keys.insert(key_id.clone(), key);
        Ok(key_id)
    }

    /// Get a key by ID
    pub fn get_key(&self, key_id: &str) -> Option<&PqcKey> {
        self.keys.get(key_id)
    }

    /// Remove a key by ID
    pub fn remove_key(&mut self, key_id: &str) -> Option<PqcKey> {
        self.keys.remove(key_id)
    }

    /// List all key IDs
    pub fn list_key_ids(&self) -> Vec<String> {
        self.keys.keys().cloned().collect()
    }

    /// List keys by algorithm
    pub fn list_keys_by_algorithm(&self, algorithm: AlgorithmType) -> Vec<&PqcKey> {
        self.keys.values().filter(|key| key.algorithm == algorithm).collect()
    }

    /// List keys by security level
    pub fn list_keys_by_security_level(&self, level: SecurityLevel) -> Vec<&PqcKey> {
        self.keys.values().filter(|key| key.security_level == level).collect()
    }

    /// List expired keys
    pub fn list_expired_keys(&self) -> Vec<&PqcKey> {
        self.keys.values().filter(|key| key.is_expired()).collect()
    }

    /// Remove expired keys
    pub fn remove_expired_keys(&mut self) -> Vec<PqcKey> {
        let expired_ids: Vec<String> = self.keys
            .iter()
            .filter(|(_, key)| key.is_expired())
            .map(|(id, _)| id.clone())
            .collect();

        expired_ids.into_iter()
            .filter_map(|id| self.keys.remove(&id))
            .collect()
    }

    /// Set default algorithm for a security level
    pub fn set_default_algorithm(&mut self, level: SecurityLevel, algorithm: AlgorithmType) {
        self.default_algorithms.insert(level, algorithm);
    }

    /// Get default algorithm for a security level
    pub fn get_default_algorithm(&self, level: SecurityLevel) -> Option<AlgorithmType> {
        self.default_algorithms.get(&level).copied()
    }

    /// Validate all keys
    pub fn validate_all_keys(&self) -> Vec<(String, PqcError)> {
        let mut errors = Vec::new();
        for (key_id, key) in &self.keys {
            if let Err(err) = key.validate() {
                errors.push((key_id.clone(), err));
            }
        }
        errors
    }

    /// Get statistics about managed keys
    pub fn get_statistics(&self) -> KeyManagerStatistics {
        let total_keys = self.keys.len();
        let mut by_algorithm = HashMap::new();
        let mut by_security_level = HashMap::new();
        let mut by_key_type = HashMap::new();
        let mut expired_count = 0;

        for key in self.keys.values() {
            *by_algorithm.entry(key.algorithm).or_insert(0) += 1;
            *by_security_level.entry(key.security_level).or_insert(0) += 1;
            *by_key_type.entry(key.key_type).or_insert(0) += 1;
            
            if key.is_expired() {
                expired_count += 1;
            }
        }

        KeyManagerStatistics {
            total_keys,
            by_algorithm,
            by_security_level,
            by_key_type,
            expired_count,
        }
    }

    /// Export keys to a serializable format
    pub fn export_keys(&self) -> PqcResult<Vec<PqcKey>> {
        Ok(self.keys.values().cloned().collect())
    }

    /// Import keys from a serializable format
    pub fn import_keys(&mut self, keys: Vec<PqcKey>) -> PqcResult<Vec<String>> {
        let mut imported_ids = Vec::new();
        for key in keys {
            let key_id = self.add_key(key)?;
            imported_ids.push(key_id);
        }
        Ok(imported_ids)
    }
}

impl Default for KeyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about managed keys
#[derive(Debug, Clone)]
pub struct KeyManagerStatistics {
    pub total_keys: usize,
    pub by_algorithm: HashMap<AlgorithmType, usize>,
    pub by_security_level: HashMap<SecurityLevel, usize>,
    pub by_key_type: HashMap<KeyType, usize>,
    pub expired_count: usize,
}

impl fmt::Display for KeyManagerStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Key Manager Statistics:")?;
        writeln!(f, "  Total keys: {}", self.total_keys)?;
        writeln!(f, "  Expired keys: {}", self.expired_count)?;
        
        writeln!(f, "  By algorithm:")?;
        for (alg, count) in &self.by_algorithm {
            writeln!(f, "    {}: {}", alg, count)?;
        }
        
        writeln!(f, "  By security level:")?;
        for (level, count) in &self.by_security_level {
            writeln!(f, "    {:?}: {}", level, count)?;
        }
        
        writeln!(f, "  By key type:")?;
        for (key_type, count) in &self.by_key_type {
            writeln!(f, "    {}: {}", key_type, count)?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pqc_key_creation() {
        let key = PqcKey::new(
            AlgorithmType::Kyber,
            SecurityLevel::Level3,
            KeyType::Public,
            vec![1, 2, 3, 4],
        );

        assert_eq!(key.algorithm, AlgorithmType::Kyber);
        assert_eq!(key.security_level, SecurityLevel::Level3);
        assert_eq!(key.key_type, KeyType::Public);
        assert_eq!(key.key_data, vec![1, 2, 3, 4]);
        assert!(!key.metadata.key_id.is_empty());
    }

    #[test]
    fn test_key_expiration() {
        let mut key = PqcKey::new(
            AlgorithmType::Kyber,
            SecurityLevel::Level1,
            KeyType::Public,
            vec![1, 2, 3, 4],
        );

        assert!(!key.is_expired());
        
        key.set_expiration(Duration::from_secs(1));
        assert!(!key.is_expired()); // Should not be expired immediately
        
        // Manually set expiration in the past
        key.metadata.expires_at = Some(key.metadata.created_at - 1);
        assert!(key.is_expired());
    }

    #[test]
    fn test_key_validation() {
        let key = PqcKey::new(
            AlgorithmType::Kyber,
            SecurityLevel::Level1,
            KeyType::Public,
            vec![1, 2, 3, 4],
        );

        assert!(key.validate().is_ok());

        // Test deprecated algorithm
        let sike_key = PqcKey::new(
            AlgorithmType::Sike,
            SecurityLevel::Level1,
            KeyType::Public,
            vec![1, 2, 3, 4],
        );

        assert!(sike_key.validate().is_err());
    }

    #[test]
    fn test_key_manager() {
        let mut manager = KeyManager::new();

        let key1 = PqcKey::new(
            AlgorithmType::Kyber,
            SecurityLevel::Level1,
            KeyType::Public,
            vec![1, 2, 3, 4],
        );

        let key2 = PqcKey::new(
            AlgorithmType::Dilithium,
            SecurityLevel::Level3,
            KeyType::Secret,
            vec![5, 6, 7, 8],
        );

        let key1_id = manager.add_key(key1).unwrap();
        let key2_id = manager.add_key(key2).unwrap();

        assert_eq!(manager.list_key_ids().len(), 2);
        assert!(manager.get_key(&key1_id).is_some());
        assert!(manager.get_key(&key2_id).is_some());

        let kyber_keys = manager.list_keys_by_algorithm(AlgorithmType::Kyber);
        assert_eq!(kyber_keys.len(), 1);

        let level1_keys = manager.list_keys_by_security_level(SecurityLevel::Level1);
        assert_eq!(level1_keys.len(), 1);
    }

    #[test]
    fn test_key_manager_statistics() {
        let mut manager = KeyManager::new();

        let key1 = PqcKey::new(AlgorithmType::Kyber, SecurityLevel::Level1, KeyType::Public, vec![1]);
        let key2 = PqcKey::new(AlgorithmType::Kyber, SecurityLevel::Level3, KeyType::Secret, vec![2]);
        let key3 = PqcKey::new(AlgorithmType::Dilithium, SecurityLevel::Level1, KeyType::Public, vec![3]);

        manager.add_key(key1).unwrap();
        manager.add_key(key2).unwrap();
        manager.add_key(key3).unwrap();

        let stats = manager.get_statistics();
        assert_eq!(stats.total_keys, 3);
        assert_eq!(stats.by_algorithm[&AlgorithmType::Kyber], 2);
        assert_eq!(stats.by_algorithm[&AlgorithmType::Dilithium], 1);
        assert_eq!(stats.by_security_level[&SecurityLevel::Level1], 2);
        assert_eq!(stats.by_security_level[&SecurityLevel::Level3], 1);
    }
}
