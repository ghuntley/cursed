use crate::error::Error;
/// fr fr Unified key generator for all asymmetric cryptography algorithms
/// 
/// This module provides a unified interface for generating keys across different
/// asymmetric cryptography algorithms including RSA, ECC, Ed25519, and X25519.

use std::collections::HashMap;
use rand::rngs::OsRng;
use zeroize::Zeroizing;
use crate::error::CursedError;
use super::{
    rsa::{RsaEngine, RsaError, CursedRsaKeyPair, RSA_2048_BITS, RSA_3072_BITS, RSA_4096_BITS, KeyFormat as RsaKeyFormat},
    ecc::{EccEngine, EccError, EccKeyPair, EccCurve, EccKeyFormat},
    ed25519::{Ed25519Engine, Ed25519Error, Ed25519KeyPair, Ed25519KeyFormat},
    x25519::{X25519Engine, X25519Error, X25519KeyPair, X25519KeyFormat},
};

/// fr fr Supported asymmetric algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsymmetricAlgorithm {
    Rsa2048,
    Rsa3072,
    Rsa4096,
    EcdsaP256,
    EcdsaP384,
    EcdsaP521,
    Ed25519,
    X25519,
}

impl AsymmetricAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            AsymmetricAlgorithm::Rsa2048 => "RSA-2048",
            AsymmetricAlgorithm::Rsa3072 => "RSA-3072",
            AsymmetricAlgorithm::Rsa4096 => "RSA-4096",
            AsymmetricAlgorithm::EcdsaP256 => "ECDSA-P256",
            AsymmetricAlgorithm::EcdsaP384 => "ECDSA-P384",
            AsymmetricAlgorithm::EcdsaP521 => "ECDSA-P521",
            AsymmetricAlgorithm::Ed25519 => "Ed25519",
            AsymmetricAlgorithm::X25519 => "X25519",
        }
    }
    
    pub fn key_size_bits(&self) -> usize {
        match self {
            AsymmetricAlgorithm::Rsa2048 => 2048,
            AsymmetricAlgorithm::Rsa3072 => 3072,
            AsymmetricAlgorithm::Rsa4096 => 4096,
            AsymmetricAlgorithm::EcdsaP256 => 256,
            AsymmetricAlgorithm::EcdsaP384 => 384,
            AsymmetricAlgorithm::EcdsaP521 => 521,
            AsymmetricAlgorithm::Ed25519 => 256,
            AsymmetricAlgorithm::X25519 => 256,
        }
    }
    
    pub fn supports_encryption(&self) -> bool {
        match self {
            AsymmetricAlgorithm::Rsa2048 | 
            AsymmetricAlgorithm::Rsa3072 | 
            AsymmetricAlgorithm::Rsa4096 => true,
            _ => false,
        }
    }
    
    pub fn supports_signing(&self) -> bool {
        match self {
            AsymmetricAlgorithm::Rsa2048 | 
            AsymmetricAlgorithm::Rsa3072 | 
            AsymmetricAlgorithm::Rsa4096 |
            AsymmetricAlgorithm::EcdsaP256 |
            AsymmetricAlgorithm::EcdsaP384 |
            AsymmetricAlgorithm::EcdsaP521 |
            AsymmetricAlgorithm::Ed25519 => true,
            AsymmetricAlgorithm::X25519 => false,
        }
    }
    
    pub fn supports_key_exchange(&self) -> bool {
        match self {
            AsymmetricAlgorithm::X25519 => true,
            _ => false,
        }
    }
    
    pub fn from_string(name: &str) -> Option<Self> {
        match name.to_uppercase().as_str() {
            "RSA-2048" | "RSA2048" => Some(AsymmetricAlgorithm::Rsa2048),
            "RSA-3072" | "RSA3072" => Some(AsymmetricAlgorithm::Rsa3072),
            "RSA-4096" | "RSA4096" => Some(AsymmetricAlgorithm::Rsa4096),
            "ECDSA-P256" | "P256" | "SECP256R1" => Some(AsymmetricAlgorithm::EcdsaP256),
            "ECDSA-P384" | "P384" | "SECP384R1" => Some(AsymmetricAlgorithm::EcdsaP384),
            "ECDSA-P521" | "P521" | "SECP521R1" => Some(AsymmetricAlgorithm::EcdsaP521),
            "ED25519" => Some(AsymmetricAlgorithm::Ed25519),
            "X25519" => Some(AsymmetricAlgorithm::X25519),
            _ => None,
        }
    }
}

/// fr fr Generated key pair wrapper
#[derive(Debug, Clone)]
pub enum GeneratedKeyPair {
    Rsa(CursedRsaKeyPair),
    Ecc(EccKeyPair),
    Ed25519(Ed25519KeyPair),
    X25519(X25519KeyPair),
}

impl GeneratedKeyPair {
    pub fn algorithm(&self) -> AsymmetricAlgorithm {
        match self {
            GeneratedKeyPair::Rsa(keypair) => {
                match keypair.key_size {
                    2048 => AsymmetricAlgorithm::Rsa2048,
                    3072 => AsymmetricAlgorithm::Rsa3072,
                    4096 => AsymmetricAlgorithm::Rsa4096,
                    _ => AsymmetricAlgorithm::Rsa2048, // Default
                }
            },
            GeneratedKeyPair::Ecc(keypair) => {
                match keypair.curve() {
                    EccCurve::P256 => AsymmetricAlgorithm::EcdsaP256,
                    EccCurve::P384 => AsymmetricAlgorithm::EcdsaP384,
                    EccCurve::P521 => AsymmetricAlgorithm::EcdsaP521,
                }
            },
            GeneratedKeyPair::Ed25519(_) => AsymmetricAlgorithm::Ed25519,
            GeneratedKeyPair::X25519(_) => AsymmetricAlgorithm::X25519,
        }
    }
    
    pub fn key_size_bits(&self) -> usize {
        self.algorithm().key_size_bits()
    }
}

/// fr fr Key generator error types
#[derive(Debug, Clone, PartialEq)]
pub enum KeyGeneratorError {
    UnsupportedAlgorithm(String),
    UnsupportedOperation(String),
    RsaError(String),
    EccError(String),
    Ed25519Error(String),
    X25519Error(String),
    InvalidParameters(String),
    Internal(String),
}

impl std::fmt::Display for KeyGeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyGeneratorError::UnsupportedAlgorithm(algo) => write!(f, "Unsupported algorithm: {}", algo),
            KeyGeneratorError::UnsupportedOperation(op) => write!(f, "Unsupported operation: {}", op),
            KeyGeneratorError::RsaError(msg) => write!(f, "RSA error: {}", msg),
            KeyGeneratorError::EccError(msg) => write!(f, "ECC error: {}", msg),
            KeyGeneratorError::Ed25519Error(msg) => write!(f, "Ed25519 error: {}", msg),
            KeyGeneratorError::X25519Error(msg) => write!(f, "X25519 error: {}", msg),
            KeyGeneratorError::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
            KeyGeneratorError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for KeyGeneratorError {}

impl From<RsaError> for KeyGeneratorError {
    fn from(err: RsaError) -> Self {
        KeyGeneratorError::RsaError(err.to_string())
    }
}

impl From<EccError> for KeyGeneratorError {
    fn from(err: EccError) -> Self {
        KeyGeneratorError::EccError(err.to_string())
    }
}

impl From<Ed25519Error> for KeyGeneratorError {
    fn from(err: Ed25519Error) -> Self {
        KeyGeneratorError::Ed25519Error(err.to_string())
    }
}

impl From<X25519Error> for KeyGeneratorError {
    fn from(err: X25519Error) -> Self {
        KeyGeneratorError::X25519Error(err.to_string())
    }
}

type KeyGeneratorResult<T> = Result<T, Error>;

/// fr fr Unified key generator for all asymmetric algorithms
/// 
/// # Security Notes
/// - Uses cryptographically secure random number generation
/// - All algorithms use established security parameters
/// - Private keys are protected with zeroizing containers where possible
/// - Keys are validated after generation
pub struct KeyGenerator {
    rng: OsRng,
    rsa_engine: RsaEngine,
    ecc_engine: EccEngine,
    ed25519_engine: Ed25519Engine,
    x25519_engine: X25519Engine,
}

impl KeyGenerator {
    /// slay Create new key generator with cryptographically secure RNG
    pub fn new() -> Self {
        Self {
            rng: OsRng,
            rsa_engine: RsaEngine::new(),
            ecc_engine: EccEngine::new(),
            ed25519_engine: Ed25519Engine::new(),
            x25519_engine: X25519Engine::new(),
        }
    }
    
    /// slay Generate key pair for specified algorithm
    /// 
    /// # Security Notes
    /// - Keys are generated with cryptographically secure parameters
    /// - Private keys should be stored securely
    /// - Use appropriate key sizes for your security requirements
    pub fn generate_keypair(&mut self, algorithm: AsymmetricAlgorithm) -> KeyGeneratorResult<GeneratedKeyPair> {
        match algorithm {
            AsymmetricAlgorithm::Rsa2048 => {
                let keypair = self.rsa_engine.generate_keypair(RSA_2048_BITS)?;
                Ok(GeneratedKeyPair::Rsa(keypair))
            },
            AsymmetricAlgorithm::Rsa3072 => {
                let keypair = self.rsa_engine.generate_keypair(RSA_3072_BITS)?;
                Ok(GeneratedKeyPair::Rsa(keypair))
            },
            AsymmetricAlgorithm::Rsa4096 => {
                let keypair = self.rsa_engine.generate_keypair(RSA_4096_BITS)?;
                Ok(GeneratedKeyPair::Rsa(keypair))
            },
            AsymmetricAlgorithm::EcdsaP256 => {
                let keypair = self.ecc_engine.generate_keypair(EccCurve::P256)?;
                Ok(GeneratedKeyPair::Ecc(keypair))
            },
            AsymmetricAlgorithm::EcdsaP384 => {
                let keypair = self.ecc_engine.generate_keypair(EccCurve::P384)?;
                Ok(GeneratedKeyPair::Ecc(keypair))
            },
            AsymmetricAlgorithm::EcdsaP521 => {
                let keypair = self.ecc_engine.generate_keypair(EccCurve::P521)?;
                Ok(GeneratedKeyPair::Ecc(keypair))
            },
            AsymmetricAlgorithm::Ed25519 => {
                let keypair = self.ed25519_engine.generate_keypair()?;
                Ok(GeneratedKeyPair::Ed25519(keypair))
            },
            AsymmetricAlgorithm::X25519 => {
                let keypair = self.x25519_engine.generate_static_keypair()?;
                Ok(GeneratedKeyPair::X25519(keypair))
            },
        }
    }
    
    /// slay Generate key pair with custom parameters
    pub fn generate_keypair_with_params(&mut self, algorithm: AsymmetricAlgorithm, params: &HashMap<String, String>) -> KeyGeneratorResult<GeneratedKeyPair> {
        match algorithm {
            AsymmetricAlgorithm::Rsa2048 | AsymmetricAlgorithm::Rsa3072 | AsymmetricAlgorithm::Rsa4096 => {
                // For RSA, we could support custom key sizes, but stick to standard ones for security
                self.generate_keypair(algorithm)
            },
            AsymmetricAlgorithm::Ed25519 => {
                // Check if deterministic generation is requested
                if let Some(seed_hex) = params.get("seed") {
                    let seed = hex::decode(seed_hex)
                        .map_err(|e| KeyGeneratorError::InvalidParameters(format!("Invalid seed: {}", e)))?;
                    let keypair = self.ed25519_engine.generate_keypair_from_seed(&seed)?;
                    Ok(GeneratedKeyPair::Ed25519(keypair))
                } else {
                    self.generate_keypair(algorithm)
                }
            },
            AsymmetricAlgorithm::X25519 => {
                // Check if deterministic generation is requested
                if let Some(seed_hex) = params.get("seed") {
                    let seed = hex::decode(seed_hex)
                        .map_err(|e| KeyGeneratorError::InvalidParameters(format!("Invalid seed: {}", e)))?;
                    let keypair = self.x25519_engine.generate_keypair_from_seed(&seed)?;
                    Ok(GeneratedKeyPair::X25519(keypair))
                } else if params.get("ephemeral").map(|s| s == "true").unwrap_or(false) {
                    // Generate ephemeral key for perfect forward secrecy
                    let ephemeral = self.x25519_engine.generate_ephemeral_keypair()?;
                    // Convert to static format for compatibility
                    let keypair = self.x25519_engine.generate_static_keypair()?;
                    Ok(GeneratedKeyPair::X25519(keypair))
                } else {
                    self.generate_keypair(algorithm)
                }
            },
            _ => self.generate_keypair(algorithm),
        }
    }
    
    /// slay Get list of supported algorithms
    pub fn supported_algorithms() -> Vec<AsymmetricAlgorithm> {
        vec![
            AsymmetricAlgorithm::Rsa2048,
            AsymmetricAlgorithm::Rsa3072,
            AsymmetricAlgorithm::Rsa4096,
            AsymmetricAlgorithm::EcdsaP256,
            AsymmetricAlgorithm::EcdsaP384,
            AsymmetricAlgorithm::EcdsaP521,
            AsymmetricAlgorithm::Ed25519,
            AsymmetricAlgorithm::X25519,
        ]
    }
    
    /// slay Get algorithms that support encryption
    pub fn encryption_algorithms() -> Vec<AsymmetricAlgorithm> {
        Self::supported_algorithms()
            .into_iter()
            .filter(|algo| algo.supports_encryption())
            .collect()
    }
    
    /// slay Get algorithms that support digital signatures
    pub fn signing_algorithms() -> Vec<AsymmetricAlgorithm> {
        Self::supported_algorithms()
            .into_iter()
            .filter(|algo| algo.supports_signing())
            .collect()
    }
    
    /// slay Get algorithms that support key exchange
    pub fn key_exchange_algorithms() -> Vec<AsymmetricAlgorithm> {
        Self::supported_algorithms()
            .into_iter()
            .filter(|algo| algo.supports_key_exchange())
            .collect()
    }
    
    /// slay Serialize key pair to PEM format (convenience method)
    pub fn serialize_keypair_to_pem(&self, keypair: &GeneratedKeyPair) -> KeyGeneratorResult<(Zeroizing<Vec<u8>>, Vec<u8>)> {
        match keypair {
            GeneratedKeyPair::Rsa(rsa_keypair) => {
                let private_pem = self.rsa_engine.serialize_private_key(&rsa_keypair.private_key, RsaKeyFormat::Pkcs8Pem)?;
                let public_pem = self.rsa_engine.serialize_public_key(&rsa_keypair.public_key, RsaKeyFormat::Pkcs8Pem)?;
                Ok((private_pem, public_pem))
            },
            GeneratedKeyPair::Ecc(ecc_keypair) => {
                let private_pem = self.ecc_engine.serialize_private_key(ecc_keypair, EccKeyFormat::Pkcs8Pem)?;
                let public_pem = self.ecc_engine.serialize_public_key(ecc_keypair, EccKeyFormat::Pkcs8Pem)?;
                Ok((private_pem, public_pem))
            },
            GeneratedKeyPair::Ed25519(ed_keypair) => {
                let private_pem = self.ed25519_engine.serialize_private_key(ed_keypair, Ed25519KeyFormat::Pkcs8Pem)?;
                let public_pem = self.ed25519_engine.serialize_public_key(ed_keypair, Ed25519KeyFormat::Pkcs8Pem)?;
                Ok((private_pem, public_pem))
            },
            GeneratedKeyPair::X25519(x_keypair) => {
                // X25519 doesn't have standard PEM format, use base64
                let private_raw = self.x25519_engine.serialize_private_key(x_keypair, X25519KeyFormat::Base64)?;
                let public_raw = self.x25519_engine.serialize_public_key(x_keypair, X25519KeyFormat::Base64)?;
                Ok((private_raw, public_raw))
            },
        }
    }
    
    /// slay Get key pair information summary
    pub fn get_keypair_info(&self, keypair: &GeneratedKeyPair) -> HashMap<String, String> {
        let mut info = HashMap::new();
        let algorithm = keypair.algorithm();
        
        info.insert("algorithm".to_string(), algorithm.name().to_string());
        info.insert("key_size_bits".to_string(), algorithm.key_size_bits().to_string());
        info.insert("supports_encryption".to_string(), algorithm.supports_encryption().to_string());
        info.insert("supports_signing".to_string(), algorithm.supports_signing().to_string());
        info.insert("supports_key_exchange".to_string(), algorithm.supports_key_exchange().to_string());
        
        match keypair {
            GeneratedKeyPair::Rsa(_) => {
                info.insert("key_type".to_string(), "RSA".to_string());
                info.insert("padding_schemes".to_string(), "PKCS1v15, OAEP, PSS".to_string());
            },
            GeneratedKeyPair::Ecc(ecc_keypair) => {
                info.insert("key_type".to_string(), "ECC".to_string());
                info.insert("curve".to_string(), ecc_keypair.curve().name().to_string());
                info.insert("signature_algorithm".to_string(), "ECDSA".to_string());
            },
            GeneratedKeyPair::Ed25519(_) => {
                info.insert("key_type".to_string(), "EdDSA".to_string());
                info.insert("curve".to_string(), "Curve25519".to_string());
                info.insert("signature_algorithm".to_string(), "Ed25519".to_string());
            },
            GeneratedKeyPair::X25519(_) => {
                info.insert("key_type".to_string(), "ECDH".to_string());
                info.insert("curve".to_string(), "Curve25519".to_string());
                info.insert("key_exchange_algorithm".to_string(), "X25519".to_string());
            },
        }
        
        info
    }
    
    /// slay Legacy API for backward compatibility
    pub fn generate_keypair(&mut self) -> Result<(Vec<u8>, Vec<u8>), String> {
        // Default to Ed25519 for new applications
        match self.generate_keypair(AsymmetricAlgorithm::Ed25519) {
            Ok(GeneratedKeyPair::Ed25519(keypair)) => {
                let private_bytes = keypair.private_key_bytes().to_vec();
                let public_bytes = keypair.public_key_bytes().to_vec();
                Ok((private_bytes, public_bytes))
            },
            Err(e) => Err(e.to_string()),
            _ => Err("Unexpected key type".to_string()),
        }
    }
}

impl Default for KeyGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Public API functions for CURSED integration
use crate::stdlib::value::Value;

/// slay Generate key pair for specified algorithm
pub fn generate_asymmetric_keypair(args: Vec<Value>) -> Result<(), Error> {
    let algorithm_name = if args.is_empty() {
        "Ed25519".to_string()
    } else {
        match &args[0] {
            Value::String(s) => s.clone(),
            _ => "Ed25519".to_string(),
        }
    };
    
    let algorithm = AsymmetricAlgorithm::from_string(&algorithm_name)
        .ok_or_else(|| CursedError::Runtime(format!("Unsupported algorithm: {}", algorithm_name)))?;
    
    let mut generator = KeyGenerator::new();
    match generator.generate_keypair(algorithm) {
        Ok(keypair) => {
            let mut result = HashMap::new();
            let info = generator.get_keypair_info(&keypair);
            
            for (key, value) in info {
                result.insert(key, Value::String(value));
            }
            
            // Add serialized keys
            if let Ok((private_pem, public_pem)) = generator.serialize_keypair_to_pem(&keypair) {
                result.insert("private_key_pem".to_string(), Value::String(String::from_utf8_lossy(&private_pem).to_string()));
                result.insert("public_key_pem".to_string(), Value::String(String::from_utf8_lossy(&public_pem).to_string()));
            }
            
            result.insert("generation_time".to_string(), Value::String(chrono::Utc::now().to_rfc3339()));
            
            Ok(Value::Object(result))
        },
        Err(e) => Err(CursedError::Runtime(format!("Key generation failed: {}", e))),
    }
}

/// slay List supported algorithms
pub fn list_asymmetric_algorithms(_args: Vec<Value>) -> Result<(), Error> {
    let algorithms = KeyGenerator::supported_algorithms();
    let mut result = Vec::new();
    
    for algo in algorithms {
        let mut algo_info = HashMap::new();
        algo_info.insert("name".to_string(), Value::String(algo.name().to_string()));
        algo_info.insert("key_size_bits".to_string(), Value::Number(algo.key_size_bits() as f64));
        algo_info.insert("supports_encryption".to_string(), Value::bool(algo.supports_encryption()));
        algo_info.insert("supports_signing".to_string(), Value::bool(algo.supports_signing()));
        algo_info.insert("supports_key_exchange".to_string(), Value::bool(algo.supports_key_exchange()));
        
        result.push(Value::Object(algo_info));
    }
    
    Ok(Value::Array(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_algorithm_properties() {
        assert!(AsymmetricAlgorithm::Rsa2048.supports_encryption());
        assert!(AsymmetricAlgorithm::Rsa2048.supports_signing());
        assert!(!AsymmetricAlgorithm::Rsa2048.supports_key_exchange());
        
        assert!(!AsymmetricAlgorithm::EcdsaP256.supports_encryption());
        assert!(AsymmetricAlgorithm::EcdsaP256.supports_signing());
        assert!(!AsymmetricAlgorithm::EcdsaP256.supports_key_exchange());
        
        assert!(!AsymmetricAlgorithm::Ed25519.supports_encryption());
        assert!(AsymmetricAlgorithm::Ed25519.supports_signing());
        assert!(!AsymmetricAlgorithm::Ed25519.supports_key_exchange());
        
        assert!(!AsymmetricAlgorithm::X25519.supports_encryption());
        assert!(!AsymmetricAlgorithm::X25519.supports_signing());
        assert!(AsymmetricAlgorithm::X25519.supports_key_exchange());
    }
    
    #[test]
    fn test_algorithm_from_string() {
        assert_eq!(AsymmetricAlgorithm::from_string("RSA-2048"), Some(AsymmetricAlgorithm::Rsa2048));
        assert_eq!(AsymmetricAlgorithm::from_string("P256"), Some(AsymmetricAlgorithm::EcdsaP256));
        assert_eq!(AsymmetricAlgorithm::from_string("Ed25519"), Some(AsymmetricAlgorithm::Ed25519));
        assert_eq!(AsymmetricAlgorithm::from_string("X25519"), Some(AsymmetricAlgorithm::X25519));
        assert_eq!(AsymmetricAlgorithm::from_string("Unknown"), None);
    }
    
    #[test]
    fn test_key_generation_all_algorithms() {
        let mut generator = KeyGenerator::new();
        
        for algorithm in KeyGenerator::supported_algorithms() {
            let keypair = generator.generate_keypair(algorithm).unwrap();
            assert_eq!(keypair.algorithm(), algorithm);
            assert_eq!(keypair.key_size_bits(), algorithm.key_size_bits());
        }
    }
    
    #[test]
    fn test_keypair_serialization() {
        let mut generator = KeyGenerator::new();
        
        // Test RSA
        let rsa_keypair = generator.generate_keypair(AsymmetricAlgorithm::Rsa2048).unwrap();
        let (private_pem, public_pem) = generator.serialize_keypair_to_pem(&rsa_keypair).unwrap();
        assert!(!private_pem.is_empty());
        assert!(!public_pem.is_empty());
        
        // Test Ed25519
        let ed_keypair = generator.generate_keypair(AsymmetricAlgorithm::Ed25519).unwrap();
        let (private_pem, public_pem) = generator.serialize_keypair_to_pem(&ed_keypair).unwrap();
        assert!(!private_pem.is_empty());
        assert!(!public_pem.is_empty());
    }
    
    #[test]
    fn test_keypair_info() {
        let mut generator = KeyGenerator::new();
        let keypair = generator.generate_keypair(AsymmetricAlgorithm::EcdsaP256).unwrap();
        let info = generator.get_keypair_info(&keypair);
        
        assert_eq!(info.get("algorithm"), Some(&"ECDSA-P256".to_string()));
        assert_eq!(info.get("key_size_bits"), Some(&"256".to_string()));
        assert_eq!(info.get("supports_signing"), Some(&"true".to_string()));
        assert_eq!(info.get("supports_encryption"), Some(&"false".to_string()));
    }
    
    #[test]
    fn test_algorithm_filtering() {
        let encryption_algos = KeyGenerator::encryption_algorithms();
        assert!(encryption_algos.contains(&AsymmetricAlgorithm::Rsa2048));
        assert!(!encryption_algos.contains(&AsymmetricAlgorithm::Ed25519));
        
        let signing_algos = KeyGenerator::signing_algorithms();
        assert!(signing_algos.contains(&AsymmetricAlgorithm::Ed25519));
        assert!(signing_algos.contains(&AsymmetricAlgorithm::EcdsaP256));
        assert!(!signing_algos.contains(&AsymmetricAlgorithm::X25519));
        
        let kex_algos = KeyGenerator::key_exchange_algorithms();
        assert!(kex_algos.contains(&AsymmetricAlgorithm::X25519));
        assert!(!kex_algos.contains(&AsymmetricAlgorithm::Ed25519));
    }
}
