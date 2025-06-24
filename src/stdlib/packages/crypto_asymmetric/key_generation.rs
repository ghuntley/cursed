// Key generation utilities
// 
// Provides comprehensive key generation functions for the CURSED stdlib.
// Supports RSA, ECC, Ed25519, and X25519 key generation with proper validation.

use crate::stdlib::value::Value;
use crate::error::CursedError;
use crate::error::Error;
use std::collections::HashMap;
use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};
use p256::{SecretKey as P256SecretKey, PublicKey as P256PublicKey};
use p384::{SecretKey as P384SecretKey, PublicKey as P384PublicKey};
use ed25519_dalek::{SigningKey, VerifyingKey};
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};
use sha2::{Sha256, Digest};

/// Supported asymmetric algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsymmetricAlgorithm {
    Rsa2048,
    Rsa3072,
    Rsa4096,
    EcdsaP256,
    EcdsaP384,
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
            AsymmetricAlgorithm::Ed25519 => "Ed25519",
            AsymmetricAlgorithm::X25519 => "X25519",
        }
    }
    
    pub fn from_name(name: &str) -> Result<(), Error> {
        match name.to_uppercase().as_str() {
            "RSA-2048" | "RSA2048" => Ok(AsymmetricAlgorithm::Rsa2048),
            "RSA-3072" | "RSA3072" => Ok(AsymmetricAlgorithm::Rsa3072),
            "RSA-4096" | "RSA4096" => Ok(AsymmetricAlgorithm::Rsa4096),
            "ECDSA-P256" | "P256" => Ok(AsymmetricAlgorithm::EcdsaP256),
            "ECDSA-P384" | "P384" => Ok(AsymmetricAlgorithm::EcdsaP384),
            "ED25519" => Ok(AsymmetricAlgorithm::Ed25519),
            "X25519" => Ok(AsymmetricAlgorithm::X25519),
            _ => Err(CursedError::InvalidArgument(format!("Unsupported algorithm: {}", name))),
        }
    }
}

/// Generated key pair container
#[derive(Debug, Clone)]
pub struct GeneratedKeyPair {
    pub algorithm: AsymmetricAlgorithm,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub key_size: Option<usize>,
    pub fingerprint: String,
}

impl GeneratedKeyPair {
    /// Create a new key pair container
    pub fn new(
        algorithm: AsymmetricAlgorithm,
        public_key: Vec<u8>,
        private_key: Vec<u8>,
        key_size: Option<usize>,
    ) -> Self {
        let fingerprint = Self::compute_fingerprint(&public_key);
        Self {
            algorithm,
            public_key,
            private_key,
            key_size,
            fingerprint,
        }
    }
    
    /// Compute SHA-256 fingerprint of public key
    fn compute_fingerprint(public_key: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(public_key);
        let result = hasher.finalize();
        hex::encode(result)
    }
    
    /// Convert to CURSED Value
    pub fn to_value(&self) -> Result<(), Error> {
        let mut map = HashMap::new();
        
        map.insert("algorithm".to_string(), Value::String(self.algorithm.name().to_string()));
        map.insert("public_key".to_string(), Value::String(hex::encode(&self.public_key)));
        map.insert("private_key".to_string(), Value::String(hex::encode(&self.private_key)));
        map.insert("fingerprint".to_string(), Value::String(self.fingerprint.clone()));
        
        if let Some(size) = self.key_size {
            map.insert("key_size".to_string(), Value::Integer(size as i64));
        }
        
        Ok(Value::Object(map))
    }
}

/// Key generation errors
#[derive(Debug, Clone)]
pub enum KeyGenerationError {
    InvalidAlgorithm(String),
    InvalidKeySize(usize),
    GenerationFailed(String),
    InsufficientEntropy,
    Internal(String),
}

impl std::fmt::Display for KeyGenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyGenerationError::InvalidAlgorithm(alg) => write!(f, "Invalid algorithm: {}", alg),
            KeyGenerationError::InvalidKeySize(size) => write!(f, "Invalid key size: {}", size),
            KeyGenerationError::GenerationFailed(msg) => write!(f, "Key generation failed: {}", msg),
            KeyGenerationError::InsufficientEntropy => write!(f, "Insufficient entropy for key generation"),
            KeyGenerationError::Internal(msg) => write!(f, "Internal key generation error: {}", msg),
        }
    }
}

/// Generate cryptographic key pair
pub fn generate_keypair(args: Vec<Value>) -> Result<(), Error> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("Algorithm name required".to_string()));
    }
    
    let algorithm_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Algorithm name must be a string".to_string())),
    };
    
    let algorithm = AsymmetricAlgorithm::from_name(&algorithm_name)?;
    
    // Optional key size for RSA
    let key_size = if args.len() > 1 {
        match &args[1] {
            Value::Integer(size) => Some(*size as usize),
            _ => None,
        }
    } else {
        None
    };
    
    generate_asymmetric_keypair(algorithm, key_size)
}

/// Generate asymmetric key pair for specified algorithm
pub fn generate_asymmetric_keypair(
    algorithm: AsymmetricAlgorithm,
    custom_key_size: Option<usize>,
) -> Result<(), Error> {
    match algorithm {
        AsymmetricAlgorithm::Rsa2048 => generate_rsa_keypair(2048),
        AsymmetricAlgorithm::Rsa3072 => generate_rsa_keypair(3072),
        AsymmetricAlgorithm::Rsa4096 => generate_rsa_keypair(4096),
        AsymmetricAlgorithm::EcdsaP256 => generate_ecdsa_p256_keypair(),
        AsymmetricAlgorithm::EcdsaP384 => generate_ecdsa_p384_keypair(),
        AsymmetricAlgorithm::Ed25519 => generate_ed25519_keypair(),
        AsymmetricAlgorithm::X25519 => generate_x25519_keypair(),
    }
}

/// Generate RSA key pair
fn generate_rsa_keypair(key_size: usize) -> Result<(), Error> {
    if key_size < 2048 {
        return Err(CursedError::InvalidArgument(format!("RSA key size {} too small (minimum 2048)", key_size)));
    }
    
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, key_size)
        .map_err(|e| CursedError::CryptoError(format!("RSA key generation failed: {}", e)))?;
    
    let public_key = RsaPublicKey::from(&private_key);
    
    // Serialize keys to DER format
    use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
    
    let private_der = private_key.to_pkcs8_der()
        .map_err(|e| CursedError::CryptoError(format!("Private key serialization failed: {}", e)))?;
    
    let public_der = public_key.to_public_key_der()
        .map_err(|e| CursedError::CryptoError(format!("Public key serialization failed: {}", e)))?;
    
    let algorithm = match key_size {
        2048 => AsymmetricAlgorithm::Rsa2048,
        3072 => AsymmetricAlgorithm::Rsa3072,
        4096 => AsymmetricAlgorithm::Rsa4096,
        _ => return Err(CursedError::InvalidArgument(format!("Unsupported RSA key size: {}", key_size))),
    };
    
    let keypair = GeneratedKeyPair::new(
        algorithm,
        public_der.as_bytes().to_vec(),
        private_der.as_bytes().to_vec(),
        Some(key_size),
    );
    
    keypair.to_value()
}

/// Generate ECDSA P-256 key pair
fn generate_ecdsa_p256_keypair() -> Result<(), Error> {
    let mut rng = OsRng;
    let private_key = P256SecretKey::random(&mut rng);
    let public_key = P256PublicKey::from(&private_key);
    
    // Serialize keys
    use elliptic_curve::sec1::{ToEncodedPoint, FromEncodedPoint};
    use elliptic_curve::pkcs8::{EncodePrivateKey, EncodePublicKey};
    
    let private_der = private_key.to_pkcs8_der()
        .map_err(|e| CursedError::CryptoError(format!("P-256 private key serialization failed: {}", e)))?;
    
    let public_der = public_key.to_public_key_der()
        .map_err(|e| CursedError::CryptoError(format!("P-256 public key serialization failed: {}", e)))?;
    
    let keypair = GeneratedKeyPair::new(
        AsymmetricAlgorithm::EcdsaP256,
        public_der.as_bytes().to_vec(),
        private_der.as_bytes().to_vec(),
        Some(256),
    );
    
    keypair.to_value()
}

/// Generate ECDSA P-384 key pair
fn generate_ecdsa_p384_keypair() -> Result<(), Error> {
    let mut rng = OsRng;
    let private_key = P384SecretKey::random(&mut rng);
    let public_key = P384PublicKey::from(&private_key);
    
    use elliptic_curve::pkcs8::{EncodePrivateKey, EncodePublicKey};
    
    let private_der = private_key.to_pkcs8_der()
        .map_err(|e| CursedError::CryptoError(format!("P-384 private key serialization failed: {}", e)))?;
    
    let public_der = public_key.to_public_key_der()
        .map_err(|e| CursedError::CryptoError(format!("P-384 public key serialization failed: {}", e)))?;
    
    let keypair = GeneratedKeyPair::new(
        AsymmetricAlgorithm::EcdsaP384,
        public_der.as_bytes().to_vec(),
        private_der.as_bytes().to_vec(),
        Some(384),
    );
    
    keypair.to_value()
}

/// Generate Ed25519 key pair
fn generate_ed25519_keypair() -> Result<(), Error> {
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();
    
    let private_bytes = signing_key.to_bytes();
    let public_bytes = verifying_key.to_bytes();
    
    let keypair = GeneratedKeyPair::new(
        AsymmetricAlgorithm::Ed25519,
        public_bytes.to_vec(),
        private_bytes.to_vec(),
        Some(255), // Ed25519 uses Curve25519 which is ~255 bits
    );
    
    keypair.to_value()
}

/// Generate X25519 key pair
fn generate_x25519_keypair() -> Result<(), Error> {
    let mut rng = OsRng;
    let private_key = EphemeralSecret::random();
    let public_key = X25519PublicKey::from(&private_key);
    
    let private_bytes = private_key.to_bytes();
    let public_bytes = public_key.to_bytes();
    
    let keypair = GeneratedKeyPair::new(
        AsymmetricAlgorithm::X25519,
        public_bytes.to_vec(),
        private_bytes.to_vec(),
        Some(255), // X25519 uses Curve25519 which is ~255 bits
    );
    
    keypair.to_value()
}

/// List all supported asymmetric algorithms
pub fn list_asymmetric_algorithms() -> Vec<String> {
    vec![
        AsymmetricAlgorithm::Rsa2048.name().to_string(),
        AsymmetricAlgorithm::Rsa3072.name().to_string(),
        AsymmetricAlgorithm::Rsa4096.name().to_string(),
        AsymmetricAlgorithm::EcdsaP256.name().to_string(),
        AsymmetricAlgorithm::EcdsaP384.name().to_string(),
        AsymmetricAlgorithm::Ed25519.name().to_string(),
        AsymmetricAlgorithm::X25519.name().to_string(),
    ]
}

/// Validate algorithm and key size combination
pub fn validate_algorithm_key_size(algorithm: AsymmetricAlgorithm, key_size: Option<usize>) -> Result<(), Error> {
    match algorithm {
        AsymmetricAlgorithm::Rsa2048 => {
            if let Some(size) = key_size {
                if size != 2048 {
                    return Err(CursedError::InvalidArgument(format!("RSA-2048 requires key size 2048, got {}", size)));
                }
            }
        },
        AsymmetricAlgorithm::Rsa3072 => {
            if let Some(size) = key_size {
                if size != 3072 {
                    return Err(CursedError::InvalidArgument(format!("RSA-3072 requires key size 3072, got {}", size)));
                }
            }
        },
        AsymmetricAlgorithm::Rsa4096 => {
            if let Some(size) = key_size {
                if size != 4096 {
                    return Err(CursedError::InvalidArgument(format!("RSA-4096 requires key size 4096, got {}", size)));
                }
            }
        },
        AsymmetricAlgorithm::EcdsaP256 => {
            if let Some(size) = key_size {
                if size != 256 {
                    return Err(CursedError::InvalidArgument(format!("ECDSA-P256 has fixed key size 256, got {}", size)));
                }
            }
        },
        AsymmetricAlgorithm::EcdsaP384 => {
            if let Some(size) = key_size {
                if size != 384 {
                    return Err(CursedError::InvalidArgument(format!("ECDSA-P384 has fixed key size 384, got {}", size)));
                }
            }
        },
        AsymmetricAlgorithm::Ed25519 => {
            if let Some(size) = key_size {
                if size != 255 {
                    return Err(CursedError::InvalidArgument(format!("Ed25519 has fixed key size ~255, got {}", size)));
                }
            }
        },
        AsymmetricAlgorithm::X25519 => {
            if let Some(size) = key_size {
                if size != 255 {
                    return Err(CursedError::InvalidArgument(format!("X25519 has fixed key size ~255, got {}", size)));
                }
            }
        },
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_from_name() {
        assert_eq!(AsymmetricAlgorithm::from_name("RSA-2048").unwrap(), AsymmetricAlgorithm::Rsa2048);
        assert_eq!(AsymmetricAlgorithm::from_name("ed25519").unwrap(), AsymmetricAlgorithm::Ed25519);
        assert!(AsymmetricAlgorithm::from_name("invalid").is_err());
    }

    #[test]
    fn test_generate_rsa_keypair() {
        let result = generate_rsa_keypair(2048);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_ed25519_keypair() {
        let result = generate_ed25519_keypair();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_algorithm_key_size() {
        assert!(validate_algorithm_key_size(AsymmetricAlgorithm::Rsa2048, Some(2048)).is_ok());
        assert!(validate_algorithm_key_size(AsymmetricAlgorithm::Rsa2048, Some(1024)).is_err());
        assert!(validate_algorithm_key_size(AsymmetricAlgorithm::Ed25519, None).is_ok());
    }
}
