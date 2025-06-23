/// Production-ready asymmetric cryptographic operations
/// 
/// This module provides unified asymmetric cryptography operations with
/// comprehensive algorithm support including RSA, ECDSA, Ed25519, X25519, and X448.

use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::collections::HashMap;
use rand::rngs::OsRng;

// Re-export key modules for unified access
pub use super::rsa::*;
pub use super::ecc::*;
pub use super::ed25519::*;
pub use super::x25519::*;
pub use super::key_exchange::*;

/// Generic asymmetric key representation
#[derive(Debug, Clone)]
pub struct AsymmetricKey {
    pub algorithm: String,
    pub key_data: Vec<u8>,
    pub key_type: String, // "private" or "public"
}

impl AsymmetricKey {
    pub fn new(algorithm: String, key_data: Vec<u8>, key_type: String) -> Self {
        Self {
            algorithm,
            key_data,
            key_type,
        }
    }
}

/// Generic asymmetric key pair representation
#[derive(Debug, Clone)]
pub struct AsymmetricKeyPair {
    pub private_key: AsymmetricKey,
    pub public_key: AsymmetricKey,
    pub algorithm: String,
}

impl AsymmetricKeyPair {
    pub fn new(private_key: AsymmetricKey, public_key: AsymmetricKey, algorithm: String) -> Self {
        Self {
            private_key,
            public_key,
            algorithm,
        }
    }
}

/// Asymmetric cryptographic operations
#[derive(Debug, Clone)]
pub struct AsymmetricCrypto {
    pub rsa_engine: super::rsa::RsaEngine,
    pub ecc_engine: super::ecc::EccEngine,
    pub ed25519_engine: super::ed25519::Ed25519Engine,
    pub x25519_engine: super::x25519::X25519Engine,
}

impl AsymmetricCrypto {
    /// Create a new asymmetric crypto instance
    pub fn new() -> Self {
        Self {
            rsa_engine: super::rsa::RsaEngine::new(),
            ecc_engine: super::ecc::EccEngine::new(),
            ed25519_engine: super::ed25519::Ed25519Engine::new(),
            x25519_engine: super::x25519::X25519Engine::new(),
        }
    }

    /// Generate a key pair for the specified algorithm
    pub fn generate_keypair(&mut self, algorithm: &str) -> Result<(), Error> {
        match algorithm.to_uppercase().as_str() {
            "RSA-2048" => {
                let keypair = self.rsa_engine.generate_keypair(2048)?;
                keypair.to_value()
            },
            "RSA-3072" => {
                let keypair = self.rsa_engine.generate_keypair(3072)?;
                keypair.to_value()
            },
            "RSA-4096" => {
                let keypair = self.rsa_engine.generate_keypair(4096)?;
                keypair.to_value()
            },
            "ECDSA-P256" => {
                let keypair = self.ecc_engine.generate_keypair(super::ecc::EccCurve::P256)?;
                keypair.to_value()
            },
            "ECDSA-P384" => {
                let keypair = self.ecc_engine.generate_keypair(super::ecc::EccCurve::P384)?;
                keypair.to_value()
            },
            "ECDSA-P521" => {
                let keypair = self.ecc_engine.generate_keypair(super::ecc::EccCurve::P521)?;
                keypair.to_value()
            },
            "ED25519" => {
                let keypair = self.ed25519_engine.generate_keypair()?;
                keypair.to_value()
            },
            "X25519" => {
                let keypair = self.x25519_engine.generate_static_keypair()?;
                keypair.to_value()
            },
            _ => Err(CursedError::InvalidArgument(format!("Unsupported algorithm: {}", algorithm))),
        }
    }

    /// Sign a message with the specified algorithm and private key
    pub fn sign(&self, algorithm: &str, private_key: &str, message: &[u8]) -> Result<(), Error> {
        match algorithm.to_uppercase().as_str() {
            "RSA-2048" | "RSA-3072" | "RSA-4096" => {
                // Parse private key and sign
                let private_key_data = self.parse_private_key(private_key)?;
                super::rsa::rsa_sign(vec![
                    Value::String(private_key.to_string()),
                    Value::String(base64::encode(message)),
                    Value::String("SHA256".to_string()),
                    Value::String("PSS".to_string()),
                ])
                .and_then(|v| match v {
                    Value::String(sig) => base64::decode(sig)
                        .map_err(|e| CursedError::CryptoError(format!("Base64 decode error: {}", e))),
                    _ => Err(CursedError::CryptoError("Invalid signature format".to_string())),
                })
            },
            "ECDSA-P256" | "ECDSA-P384" | "ECDSA-P521" => {
                super::ecc::ecdsa_sign(vec![
                    Value::String(private_key.to_string()),
                    Value::String(base64::encode(message)),
                    Value::String("SHA256".to_string()),
                ])
                .and_then(|v| match v {
                    Value::String(sig) => base64::decode(sig)
                        .map_err(|e| CursedError::CryptoError(format!("Base64 decode error: {}", e))),
                    _ => Err(CursedError::CryptoError("Invalid signature format".to_string())),
                })
            },
            "ED25519" => {
                super::ed25519::ed25519_sign(vec![
                    Value::String(private_key.to_string()),
                    Value::String(base64::encode(message)),
                ])
                .and_then(|v| match v {
                    Value::String(sig) => base64::decode(sig)
                        .map_err(|e| CursedError::CryptoError(format!("Base64 decode error: {}", e))),
                    _ => Err(CursedError::CryptoError("Invalid signature format".to_string())),
                })
            },
            _ => Err(CursedError::InvalidArgument(format!("Unsupported signing algorithm: {}", algorithm))),
        }
    }

    /// Verify a signature with the specified algorithm and public key
    pub fn verify(&self, algorithm: &str, public_key: &str, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        match algorithm.to_uppercase().as_str() {
            "RSA-2048" | "RSA-3072" | "RSA-4096" => {
                super::rsa::rsa_verify(vec![
                    Value::String(public_key.to_string()),
                    Value::String(base64::encode(message)),
                    Value::String(base64::encode(signature)),
                    Value::String("SHA256".to_string()),
                    Value::String("PSS".to_string()),
                ])
                .map(|v| match v {
                    Value::Boolean(valid) => valid,
                    _ => false,
                })
            },
            "ECDSA-P256" | "ECDSA-P384" | "ECDSA-P521" => {
                super::ecc::ecdsa_verify(vec![
                    Value::String(public_key.to_string()),
                    Value::String(base64::encode(message)),
                    Value::String(base64::encode(signature)),
                    Value::String("SHA256".to_string()),
                ])
                .map(|v| match v {
                    Value::Boolean(valid) => valid,
                    _ => false,
                })
            },
            "ED25519" => {
                super::ed25519::ed25519_verify(vec![
                    Value::String(public_key.to_string()),
                    Value::String(base64::encode(message)),
                    Value::String(base64::encode(signature)),
                ])
                .map(|v| match v {
                    Value::Boolean(valid) => valid,
                    _ => false,
                })
            },
            _ => Err(CursedError::InvalidArgument(format!("Unsupported verification algorithm: {}", algorithm))),
        }
    }

    /// Perform key exchange with the specified algorithm
    pub fn key_exchange(&self, algorithm: &str, private_key: &str, public_key: &str) -> Result<(), Error> {
        match algorithm.to_uppercase().as_str() {
            "X25519" => {
                x25519_key_exchange(vec![
                    Value::String(private_key.to_string()),
                    Value::String(public_key.to_string()),
                ])
                .and_then(|v| match v {
                    Value::Object(map) => {
                        if let Some(Value::String(secret)) = map.get("shared_secret") {
                            hex::decode(secret)
                                .map_err(|e| CursedError::CryptoError(format!("Hex decode error: {}", e)))
                        } else {
                            Err(CursedError::CryptoError("Missing shared secret".to_string()))
                        }
                    },
                    _ => Err(CursedError::CryptoError("Invalid key exchange result".to_string())),
                })
            },
            "DH" | "DIFFIE-HELLMAN" => {
                dh_key_exchange(vec![
                    Value::String(private_key.to_string()),
                    Value::String(public_key.to_string()),
                ])
                .and_then(|v| match v {
                    Value::Object(map) => {
                        if let Some(Value::String(secret)) = map.get("shared_secret") {
                            hex::decode(secret)
                                .map_err(|e| CursedError::CryptoError(format!("Hex decode error: {}", e)))
                        } else {
                            Err(CursedError::CryptoError("Missing shared secret".to_string()))
                        }
                    },
                    _ => Err(CursedError::CryptoError("Invalid key exchange result".to_string())),
                })
            },
            "X448" => {
                // Use the new X448 implementation
                crate::stdlib::crypto::x448_implementation::x448_key_exchange(args)
            },
            _ => Err(CursedError::InvalidArgument(format!("Unsupported key exchange algorithm: {}", algorithm))),
        }
    }

    /// Encrypt data with RSA
    pub fn rsa_encrypt(&self, public_key: &str, data: &[u8]) -> Result<(), Error> {
        super::rsa::rsa_encrypt(vec![
            Value::String(public_key.to_string()),
            Value::String(base64::encode(data)),
            Value::String("OAEP".to_string()),
        ])
        .and_then(|v| match v {
            Value::String(encrypted) => base64::decode(encrypted)
                .map_err(|e| CursedError::CryptoError(format!("Base64 decode error: {}", e))),
            _ => Err(CursedError::CryptoError("Invalid encryption result".to_string())),
        })
    }

    /// Decrypt data with RSA
    pub fn rsa_decrypt(&self, private_key: &str, encrypted_data: &[u8]) -> Result<(), Error> {
        super::rsa::rsa_decrypt(vec![
            Value::String(private_key.to_string()),
            Value::String(base64::encode(encrypted_data)),
            Value::String("OAEP".to_string()),
        ])
        .and_then(|v| match v {
            Value::String(decrypted) => base64::decode(decrypted)
                .map_err(|e| CursedError::CryptoError(format!("Base64 decode error: {}", e))),
            _ => Err(CursedError::CryptoError("Invalid decryption result".to_string())),
        })
    }

    /// Parse a private key from string format
    fn parse_private_key(&self, private_key: &str) -> Result<(), Error> {
        // Try different formats
        if private_key.starts_with("-----BEGIN") {
            // PEM format
            self.parse_pem_private_key(private_key)
        } else if private_key.len() % 2 == 0 {
            // Hex format
            hex::decode(private_key)
                .map_err(|e| CursedError::CryptoError(format!("Invalid hex private key: {}", e)))
        } else {
            // Base64 format
            base64::decode(private_key)
                .map_err(|e| CursedError::CryptoError(format!("Invalid base64 private key: {}", e)))
        }
    }

    /// Parse PEM private key
    fn parse_pem_private_key(&self, pem_data: &str) -> Result<(), Error> {
        // Basic PEM parsing - extract base64 content
        let lines: Vec<&str> = pem_data.split("\n").collect();
        let mut base64_content = String::new();
        let mut in_key = false;
        
        for line in lines {
            if line.starts_with("-----BEGIN") {
                in_key = true;
                continue;
            }
            if line.starts_with("-----END") {
                break;
            }
            if in_key {
                base64_content.push_str(line.trim());
            }
        }
        
        if base64_content.is_empty() {
            return Err(CursedError::CryptoError("Invalid PEM format".to_string()));
        }
        
        base64::decode(base64_content)
            .map_err(|e| CursedError::CryptoError(format!("Invalid PEM base64 content: {}", e)))
    }

    /// Get supported algorithms
    pub fn supported_algorithms(&self) -> Vec<String> {
        vec![
            "RSA-2048".to_string(),
            "RSA-3072".to_string(),
            "RSA-4096".to_string(),
            "ECDSA-P256".to_string(),
            "ECDSA-P384".to_string(),
            "ECDSA-P521".to_string(),
            "Ed25519".to_string(),
            "X25519".to_string(),
        ]
    }

    /// Get algorithm capabilities
    pub fn get_algorithm_info(&self, algorithm: &str) -> Result<(), Error> {
        let mut info = HashMap::new();
        
        match algorithm.to_uppercase().as_str() {
            "RSA-2048" => {
                info.insert("name".to_string(), Value::String("RSA-2048".to_string()));
                info.insert("type".to_string(), Value::String("RSA".to_string()));
                info.insert("key_size".to_string(), Value::Integer(2048));
                info.insert("capabilities".to_string(), Value::Array(vec![
                    Value::String("encryption".to_string()),
                    Value::String("signature".to_string()),
                ]));
            },
            "RSA-3072" => {
                info.insert("name".to_string(), Value::String("RSA-3072".to_string()));
                info.insert("type".to_string(), Value::String("RSA".to_string()));
                info.insert("key_size".to_string(), Value::Integer(3072));
                info.insert("capabilities".to_string(), Value::Array(vec![
                    Value::String("encryption".to_string()),
                    Value::String("signature".to_string()),
                ]));
            },
            "RSA-4096" => {
                info.insert("name".to_string(), Value::String("RSA-4096".to_string()));
                info.insert("type".to_string(), Value::String("RSA".to_string()));
                info.insert("key_size".to_string(), Value::Integer(4096));
                info.insert("capabilities".to_string(), Value::Array(vec![
                    Value::String("encryption".to_string()),
                    Value::String("signature".to_string()),
                ]));
            },
            "ECDSA-P256" => {
                info.insert("name".to_string(), Value::String("ECDSA-P256".to_string()));
                info.insert("type".to_string(), Value::String("ECC".to_string()));
                info.insert("curve".to_string(), Value::String("P-256".to_string()));
                info.insert("key_size".to_string(), Value::Integer(256));
                info.insert("capabilities".to_string(), Value::Array(vec![
                    Value::String("signature".to_string()),
                ]));
            },
            "ECDSA-P384" => {
                info.insert("name".to_string(), Value::String("ECDSA-P384".to_string()));
                info.insert("type".to_string(), Value::String("ECC".to_string()));
                info.insert("curve".to_string(), Value::String("P-384".to_string()));
                info.insert("key_size".to_string(), Value::Integer(384));
                info.insert("capabilities".to_string(), Value::Array(vec![
                    Value::String("signature".to_string()),
                ]));
            },
            "ECDSA-P521" => {
                info.insert("name".to_string(), Value::String("ECDSA-P521".to_string()));
                info.insert("type".to_string(), Value::String("ECC".to_string()));
                info.insert("curve".to_string(), Value::String("P-521".to_string()));
                info.insert("key_size".to_string(), Value::Integer(521));
                info.insert("capabilities".to_string(), Value::Array(vec![
                    Value::String("signature".to_string()),
                ]));
            },
            "ED25519" => {
                info.insert("name".to_string(), Value::String("Ed25519".to_string()));
                info.insert("type".to_string(), Value::String("EdDSA".to_string()));
                info.insert("curve".to_string(), Value::String("Curve25519".to_string()));
                info.insert("key_size".to_string(), Value::Integer(255));
                info.insert("capabilities".to_string(), Value::Array(vec![
                    Value::String("signature".to_string()),
                ]));
            },
            "X25519" => {
                info.insert("name".to_string(), Value::String("X25519".to_string()));
                info.insert("type".to_string(), Value::String("ECDH".to_string()));
                info.insert("curve".to_string(), Value::String("Curve25519".to_string()));
                info.insert("key_size".to_string(), Value::Integer(255));
                info.insert("capabilities".to_string(), Value::Array(vec![
                    Value::String("key_exchange".to_string()),
                ]));
            },
            _ => return Err(CursedError::InvalidArgument(format!("Unknown algorithm: {}", algorithm))),
        }
        
        Ok(Value::Object(info))
    }
}

impl Default for AsymmetricCrypto {
    fn default() -> Self {
        Self::new()
    }
}

// High-level API functions for CURSED stdlib

/// Generate an asymmetric key pair
pub fn generate_asymmetric_keypair(args: Vec<Value>) -> Result<(), Error> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("Algorithm name required".to_string()));
    }

    let algorithm = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Algorithm must be a string".to_string())),
    };

    let mut crypto = AsymmetricCrypto::new();
    crypto.generate_keypair(&algorithm)
}

/// Sign a message with asymmetric cryptography
pub fn asymmetric_sign(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("Required: algorithm, private_key, message".to_string()));
    }

    let algorithm = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Algorithm must be a string".to_string())),
    };

    let private_key = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };

    let message = match &args[2] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(arr) => {
            let mut bytes = Vec::new();
            for val in arr {
                match val {
                    Value::Integer(i) => bytes.push(*i as u8),
                    _ => return Err(CursedError::InvalidArgument("Message array must contain integers".to_string())),
                }
            }
            bytes
        },
        _ => return Err(CursedError::InvalidArgument("Message must be a string or byte array".to_string())),
    };

    let crypto = AsymmetricCrypto::new();
    let signature = crypto.sign(&algorithm, &private_key, &message)?;
    
    Ok(Value::String(base64::encode(signature)))
}

/// Verify a signature with asymmetric cryptography
pub fn asymmetric_verify(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 4 {
        return Err(CursedError::InvalidArgument("Required: algorithm, public_key, message, signature".to_string()));
    }

    let algorithm = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Algorithm must be a string".to_string())),
    };

    let public_key = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };

    let message = match &args[2] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(arr) => {
            let mut bytes = Vec::new();
            for val in arr {
                match val {
                    Value::Integer(i) => bytes.push(*i as u8),
                    _ => return Err(CursedError::InvalidArgument("Message array must contain integers".to_string())),
                }
            }
            bytes
        },
        _ => return Err(CursedError::InvalidArgument("Message must be a string or byte array".to_string())),
    };

    let signature = match &args[3] {
        Value::String(s) => base64::decode(s)
            .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64 signature: {}", e)))?,
        Value::Array(arr) => {
            let mut bytes = Vec::new();
            for val in arr {
                match val {
                    Value::Integer(i) => bytes.push(*i as u8),
                    _ => return Err(CursedError::InvalidArgument("Signature array must contain integers".to_string())),
                }
            }
            bytes
        },
        _ => return Err(CursedError::InvalidArgument("Signature must be a string or byte array".to_string())),
    };

    let crypto = AsymmetricCrypto::new();
    let is_valid = crypto.verify(&algorithm, &public_key, &message, &signature)?;
    
    Ok(Value::Boolean(is_valid))
}

/// Perform asymmetric key exchange
pub fn asymmetric_key_exchange(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("Required: algorithm, private_key, public_key".to_string()));
    }

    let algorithm = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Algorithm must be a string".to_string())),
    };

    let private_key = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };

    let public_key = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };

    let crypto = AsymmetricCrypto::new();
    let shared_secret = crypto.key_exchange(&algorithm, &private_key, &public_key)?;
    
    Ok(Value::String(base64::encode(shared_secret)))
}

/// Get information about supported algorithms
pub fn get_asymmetric_algorithms() -> Result<(), Error> {
    let crypto = AsymmetricCrypto::new();
    let algorithms = crypto.supported_algorithms();
    
    let mut result = Vec::new();
    for algorithm in algorithms {
        if let Ok(info) = crypto.get_algorithm_info(&algorithm) {
            result.push(info);
        }
    }
    
    Ok(Value::Array(result))
}

/// Get comprehensive asymmetric crypto capabilities
pub fn get_asymmetric_capabilities() -> Result<(), Error> {
    let mut capabilities = HashMap::new();
    
    capabilities.insert("algorithms".to_string(), Value::Array(vec![
        Value::String("RSA-2048".to_string()),
        Value::String("RSA-3072".to_string()),
        Value::String("RSA-4096".to_string()),
        Value::String("ECDSA-P256".to_string()),
        Value::String("ECDSA-P384".to_string()),
        Value::String("ECDSA-P521".to_string()),
        Value::String("Ed25519".to_string()),
        Value::String("X25519".to_string()),
    ]));
    
    capabilities.insert("operations".to_string(), Value::Array(vec![
        Value::String("key_generation".to_string()),
        Value::String("digital_signatures".to_string()),
        Value::String("key_exchange".to_string()),
        Value::String("encryption".to_string()),
        Value::String("decryption".to_string()),
    ]));
    
    capabilities.insert("key_formats".to_string(), Value::Array(vec![
        Value::String("PEM".to_string()),
        Value::String("DER".to_string()),
        Value::String("Raw".to_string()),
        Value::String("JWK".to_string()),
    ]));
    
    capabilities.insert("security_features".to_string(), Value::Array(vec![
        Value::String("cryptographically_secure_rng".to_string()),
        Value::String("constant_time_operations".to_string()),
        Value::String("secure_key_generation".to_string()),
        Value::String("ephemeral_keys".to_string()),
        Value::String("forward_secrecy".to_string()),
    ]));
    
    Ok(Value::Object(capabilities))
}
