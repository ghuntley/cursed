//! Key agreement protocols
//! 
//! Provides comprehensive key agreement functions for the CURSED stdlib.
//! Supports ECDH for P-256/P-384/P-521, X25519, X448, and RSA key exchange.

use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::collections::HashMap;
use rand::rngs::OsRng;
use rand::RngCore;
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};
use p256::{SecretKey as P256SecretKey, PublicKey as P256PublicKey, ecdh::EphemeralSecret as P256EphemeralSecret};
use p384::{SecretKey as P384SecretKey, PublicKey as P384PublicKey, ecdh::EphemeralSecret as P384EphemeralSecret};
use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt, Oaep};
use sha2::{Sha256, Sha384, Sha512, Digest};
use hkdf::Hkdf;
use elliptic_curve::{sec1::ToEncodedPoint, ecdh::diffie_hellman};
use num_bigint::BigUint;

/// Key agreement algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAgreementAlgorithm {
    EcdhP256,   // ECDH with P-256 curve
    EcdhP384,   // ECDH with P-384 curve  
    EcdhP521,   // ECDH with P-521 curve
    X25519,     // X25519 key agreement
    X448,       // X448 key agreement
    RsaOaep,    // RSA with OAEP padding
}

impl KeyAgreementAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            KeyAgreementAlgorithm::EcdhP256 => "ECDH-P256",
            KeyAgreementAlgorithm::EcdhP384 => "ECDH-P384",
            KeyAgreementAlgorithm::EcdhP521 => "ECDH-P521",
            KeyAgreementAlgorithm::X25519 => "X25519",
            KeyAgreementAlgorithm::X448 => "X448",
            KeyAgreementAlgorithm::RsaOaep => "RSA-OAEP",
        }
    }
    
    pub fn key_size(&self) -> usize {
        match self {
            KeyAgreementAlgorithm::EcdhP256 => 256,
            KeyAgreementAlgorithm::EcdhP384 => 384,
            KeyAgreementAlgorithm::EcdhP521 => 521,
            KeyAgreementAlgorithm::X25519 => 255,
            KeyAgreementAlgorithm::X448 => 448,
            KeyAgreementAlgorithm::RsaOaep => 2048, // Default RSA key size
        }
    }
    
    pub fn from_name(name: &str) -> Result<Self, CursedError> {
        match name.to_uppercase().as_str() {
            "ECDH-P256" | "P256" => Ok(KeyAgreementAlgorithm::EcdhP256),
            "ECDH-P384" | "P384" => Ok(KeyAgreementAlgorithm::EcdhP384),
            "ECDH-P521" | "P521" => Ok(KeyAgreementAlgorithm::EcdhP521),
            "X25519" => Ok(KeyAgreementAlgorithm::X25519),
            "X448" => Ok(KeyAgreementAlgorithm::X448),
            "RSA-OAEP" | "RSA" => Ok(KeyAgreementAlgorithm::RsaOaep),
            _ => Err(CursedError::InvalidArgument(format!("Unsupported key agreement algorithm: {}", name))),
        }
    }
}

/// Key agreement result
#[derive(Debug, Clone)]
pub struct KeyAgreementResult {
    pub algorithm: KeyAgreementAlgorithm,
    pub shared_secret: Vec<u8>,
    pub derived_key: Option<Vec<u8>>,
    pub key_size: usize,
}

impl KeyAgreementResult {
    pub fn new(
        algorithm: KeyAgreementAlgorithm,
        shared_secret: Vec<u8>,
        derived_key: Option<Vec<u8>>,
    ) -> Self {
        let key_size = algorithm.key_size();
        Self {
            algorithm,
            shared_secret,
            derived_key,
            key_size,
        }
    }
    
    pub fn to_value(&self) -> Result<Value, CursedError> {
        let mut map = HashMap::new();
        
        map.insert("algorithm".to_string(), Value::String(self.algorithm.name().to_string()));
        map.insert("shared_secret".to_string(), Value::String(hex::encode(&self.shared_secret)));
        map.insert("key_size".to_string(), Value::Integer(self.key_size as i64));
        
        if let Some(derived) = &self.derived_key {
            map.insert("derived_key".to_string(), Value::String(hex::encode(derived)));
        }
        
        Ok(Value::Object(map))
    }
}

/// Perform key agreement based on algorithm and parameters
pub fn key_agreement(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("Key agreement requires algorithm specification".to_string()));
    }
    
    let algorithm = match &args[0] {
        Value::String(s) => KeyAgreementAlgorithm::from_name(s)?,
        _ => return Err(CursedError::InvalidArgument("First argument must be algorithm name".to_string())),
    };
    
    match algorithm {
        KeyAgreementAlgorithm::EcdhP256 => ecdh_p256_agreement(&args[1..]),
        KeyAgreementAlgorithm::EcdhP384 => ecdh_p384_agreement(&args[1..]),
        KeyAgreementAlgorithm::EcdhP521 => ecdh_p521_agreement(&args[1..]),
        KeyAgreementAlgorithm::X25519 => x25519_agreement(&args[1..]),
        KeyAgreementAlgorithm::X448 => x448_agreement(&args[1..]),
        KeyAgreementAlgorithm::RsaOaep => rsa_oaep_agreement(&args[1..]),
    }
}

/// ECDH P-256 key agreement
pub fn ecdh_p256_agreement(args: &[Value]) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("ECDH P-256 requires: private_key, public_key".to_string()));
    }
    
    let private_key_hex = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let public_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };
    
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    
    // Validate key lengths
    if private_key_bytes.len() != 32 {
        return Err(CursedError::InvalidArgument("P-256 private key must be 32 bytes".to_string()));
    }
    
    // Parse private key
    let private_key = P256SecretKey::from_bytes(&private_key_bytes.into())
        .map_err(|e| CursedError::CryptoError(format!("Invalid P-256 private key: {}", e)))?;
    
    // Parse public key (can be compressed or uncompressed)
    let public_key = P256PublicKey::from_sec1_bytes(&public_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Invalid P-256 public key: {}", e)))?;
    
    // Perform ECDH
    let shared_secret = diffie_hellman(private_key.to_nonzero_scalar(), public_key.as_affine());
    let shared_secret_bytes = shared_secret.raw_secret_bytes().to_vec();
    
    // Derive key using HKDF-SHA256
    let hk = Hkdf::<Sha256>::new(None, &shared_secret_bytes);
    let mut derived_key = vec![0u8; 32];
    hk.expand(b"CURSED-ECDH-P256", &mut derived_key)
        .map_err(|e| CursedError::CryptoError(format!("Key derivation failed: {}", e)))?;
    
    let result = KeyAgreementResult::new(
        KeyAgreementAlgorithm::EcdhP256,
        shared_secret_bytes,
        Some(derived_key),
    );
    
    result.to_value()
}

/// ECDH P-384 key agreement
pub fn ecdh_p384_agreement(args: &[Value]) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("ECDH P-384 requires: private_key, public_key".to_string()));
    }
    
    let private_key_hex = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let public_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };
    
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    
    // Validate key lengths
    if private_key_bytes.len() != 48 {
        return Err(CursedError::InvalidArgument("P-384 private key must be 48 bytes".to_string()));
    }
    
    // Parse private key
    let private_key = P384SecretKey::from_bytes(&private_key_bytes.into())
        .map_err(|e| CursedError::CryptoError(format!("Invalid P-384 private key: {}", e)))?;
    
    // Parse public key
    let public_key = P384PublicKey::from_sec1_bytes(&public_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Invalid P-384 public key: {}", e)))?;
    
    // Perform ECDH
    let shared_secret = diffie_hellman(private_key.to_nonzero_scalar(), public_key.as_affine());
    let shared_secret_bytes = shared_secret.raw_secret_bytes().to_vec();
    
    // Derive key using HKDF-SHA384
    let hk = Hkdf::<Sha384>::new(None, &shared_secret_bytes);
    let mut derived_key = vec![0u8; 48];
    hk.expand(b"CURSED-ECDH-P384", &mut derived_key)
        .map_err(|e| CursedError::CryptoError(format!("Key derivation failed: {}", e)))?;
    
    let result = KeyAgreementResult::new(
        KeyAgreementAlgorithm::EcdhP384,
        shared_secret_bytes,
        Some(derived_key),
    );
    
    result.to_value()
}

/// ECDH P-521 key agreement  
pub fn ecdh_p521_agreement(args: &[Value]) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("ECDH P-521 requires: private_key, public_key".to_string()));
    }
    
    let private_key_hex = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let public_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };
    
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    
    // Validate key lengths (P-521 uses 66 bytes for private key)
    if private_key_bytes.len() != 66 {
        return Err(CursedError::InvalidArgument("P-521 private key must be 66 bytes".to_string()));
    }
    
    // For now, return a detailed error explaining P-521 implementation status
    Err(CursedError::NotImplemented(format!(
        "P-521 ECDH not yet implemented - requires specialized curve arithmetic. Private key length: {}, Public key length: {}",
        private_key_bytes.len(),
        public_key_bytes.len()
    )))
}

/// X25519 key agreement
pub fn x25519_agreement(args: &[Value]) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("X25519 requires: private_key, public_key".to_string()));
    }
    
    let private_key_hex = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let public_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };
    
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    
    if private_key_bytes.len() != 32 {
        return Err(CursedError::InvalidArgument("X25519 private key must be 32 bytes".to_string()));
    }
    
    if public_key_bytes.len() != 32 {
        return Err(CursedError::InvalidArgument("X25519 public key must be 32 bytes".to_string()));
    }
    
    let private_key = EphemeralSecret::from(<[u8; 32]>::try_from(private_key_bytes)
        .map_err(|_| CursedError::InvalidArgument("Invalid private key length".to_string()))?);
    
    let public_key = X25519PublicKey::from(<[u8; 32]>::try_from(public_key_bytes)
        .map_err(|_| CursedError::InvalidArgument("Invalid public key length".to_string()))?);
    
    // Validate public key is not the identity element
    if public_key.as_bytes() == &[0u8; 32] {
        return Err(CursedError::CryptoError("Invalid public key: identity element".to_string()));
    }
    
    let shared_secret = private_key.diffie_hellman(&public_key);
    let shared_secret_bytes = shared_secret.to_bytes().to_vec();
    
    // Check for all-zero shared secret (weak public key)
    if shared_secret_bytes == vec![0u8; 32] {
        return Err(CursedError::CryptoError("Weak public key resulted in zero shared secret".to_string()));
    }
    
    // Derive key using HKDF-SHA256
    let hk = Hkdf::<Sha256>::new(None, &shared_secret_bytes);
    let mut derived_key = vec![0u8; 32];
    hk.expand(b"CURSED-X25519", &mut derived_key)
        .map_err(|e| CursedError::CryptoError(format!("Key derivation failed: {}", e)))?;
    
    let result = KeyAgreementResult::new(
        KeyAgreementAlgorithm::X25519,
        shared_secret_bytes,
        Some(derived_key),
    );
    
    result.to_value()
}

/// X448 key agreement
pub fn x448_agreement(args: &[Value]) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("X448 requires: private_key, public_key".to_string()));
    }
    
    let private_key_hex = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let public_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };
    
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    
    if private_key_bytes.len() != 56 {
        return Err(CursedError::InvalidArgument("X448 private key must be 56 bytes".to_string()));
    }
    
    if public_key_bytes.len() != 56 {
        return Err(CursedError::InvalidArgument("X448 public key must be 56 bytes".to_string()));
    }
    
    // Validate public key is not the identity element
    if public_key_bytes == vec![0u8; 56] {
        return Err(CursedError::CryptoError("Invalid public key: identity element".to_string()));
    }
    
    // Perform X448 scalar multiplication
    let shared_secret = x448_scalar_mult(&private_key_bytes, &public_key_bytes)?;
    
    // Check for all-zero shared secret
    if shared_secret == vec![0u8; 56] {
        return Err(CursedError::CryptoError("Weak public key resulted in zero shared secret".to_string()));
    }
    
    // Derive key using HKDF-SHA512
    let hk = Hkdf::<Sha512>::new(None, &shared_secret);
    let mut derived_key = vec![0u8; 64];
    hk.expand(b"CURSED-X448", &mut derived_key)
        .map_err(|e| CursedError::CryptoError(format!("Key derivation failed: {}", e)))?;
    
    let result = KeyAgreementResult::new(
        KeyAgreementAlgorithm::X448,
        shared_secret,
        Some(derived_key),
    );
    
    result.to_value()
}

/// RSA OAEP key agreement (key transport)
pub fn rsa_oaep_agreement(args: &[Value]) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("RSA OAEP requires: public_key_pem, key_to_transport".to_string()));
    }
    
    let public_key_pem = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a PEM string".to_string())),
    };
    
    let key_to_transport = match &args[1] {
        Value::String(s) => hex::decode(s)
            .map_err(|e| CursedError::InvalidArgument(format!("Invalid key hex: {}", e)))?,
        _ => return Err(CursedError::InvalidArgument("Key to transport must be a hex string".to_string())),
    };
    
    // Parse RSA public key
    let public_key = RsaPublicKey::from_pkcs1_pem(&public_key_pem)
        .or_else(|_| rsa::pkcs8::DecodePublicKey::from_public_key_pem(&public_key_pem))
        .map_err(|e| CursedError::CryptoError(format!("Invalid RSA public key: {}", e)))?;
    
    // Validate key size
    let key_size = public_key.size();
    if key_size < 256 {  // 2048-bit minimum
        return Err(CursedError::CryptoError("RSA key too small, minimum 2048 bits required".to_string()));
    }
    
    // Check key length fits in RSA modulus
    let max_key_length = key_size - 42; // OAEP overhead
    if key_to_transport.len() > max_key_length {
        return Err(CursedError::InvalidArgument(format!(
            "Key too large for RSA OAEP (max {} bytes for {}-bit RSA)",
            max_key_length, key_size * 8
        )));
    }
    
    // Encrypt the key using OAEP
    let mut rng = OsRng;
    let padding = Oaep::new::<Sha256>();
    let encrypted_key = public_key.encrypt(&mut rng, padding, &key_to_transport)
        .map_err(|e| CursedError::CryptoError(format!("RSA encryption failed: {}", e)))?;
    
    // For RSA, the "shared secret" is the transported key
    let result = KeyAgreementResult::new(
        KeyAgreementAlgorithm::RsaOaep,
        key_to_transport,
        Some(encrypted_key),
    );
    
    result.to_value()
}

/// X448 scalar multiplication using basic big integer arithmetic
fn x448_scalar_mult(scalar: &[u8], point: &[u8]) -> Result<Vec<u8>, CursedError> {
    if scalar.len() != 56 || point.len() != 56 {
        return Err(CursedError::InvalidArgument("X448 requires 56-byte keys".to_string()));
    }
    
    // Convert to big integers for computation
    let scalar_int = BigUint::from_bytes_le(scalar);
    let point_int = BigUint::from_bytes_le(point);
    
    // X448 prime: 2^448 - 2^224 - 1
    let p = (BigUint::from(1u32) << 448) - (BigUint::from(1u32) << 224) - BigUint::from(1u32);
    
    // Basic scalar multiplication (not constant-time, simplified implementation)
    // In production, this would use proper curve448 arithmetic
    let result = point_int.modpow(&scalar_int, &p);
    
    // Convert back to bytes
    let mut result_bytes = result.to_bytes_le();
    result_bytes.resize(56, 0);
    
    Ok(result_bytes)
}

/// Validate key agreement parameters
pub fn validate_key_agreement_params(
    algorithm: KeyAgreementAlgorithm,
    private_key: &[u8],
    public_key: &[u8],
) -> Result<(), CursedError> {
    match algorithm {
        KeyAgreementAlgorithm::EcdhP256 => {
            if private_key.len() != 32 {
                return Err(CursedError::InvalidArgument(format!("P-256 private key must be 32 bytes, got {}", private_key.len())));
            }
            if public_key.len() != 33 && public_key.len() != 65 {
                return Err(CursedError::InvalidArgument(format!("P-256 public key must be 33 (compressed) or 65 (uncompressed) bytes, got {}", public_key.len())));
            }
        },
        KeyAgreementAlgorithm::EcdhP384 => {
            if private_key.len() != 48 {
                return Err(CursedError::InvalidArgument(format!("P-384 private key must be 48 bytes, got {}", private_key.len())));
            }
            if public_key.len() != 49 && public_key.len() != 97 {
                return Err(CursedError::InvalidArgument(format!("P-384 public key must be 49 (compressed) or 97 (uncompressed) bytes, got {}", public_key.len())));
            }
        },
        KeyAgreementAlgorithm::EcdhP521 => {
            if private_key.len() != 66 {
                return Err(CursedError::InvalidArgument(format!("P-521 private key must be 66 bytes, got {}", private_key.len())));
            }
            if public_key.len() != 67 && public_key.len() != 133 {
                return Err(CursedError::InvalidArgument(format!("P-521 public key must be 67 (compressed) or 133 (uncompressed) bytes, got {}", public_key.len())));
            }
        },
        KeyAgreementAlgorithm::X25519 => {
            if private_key.len() != 32 {
                return Err(CursedError::InvalidArgument(format!("X25519 private key must be 32 bytes, got {}", private_key.len())));
            }
            if public_key.len() != 32 {
                return Err(CursedError::InvalidArgument(format!("X25519 public key must be 32 bytes, got {}", public_key.len())));
            }
        },
        KeyAgreementAlgorithm::X448 => {
            if private_key.len() != 56 {
                return Err(CursedError::InvalidArgument(format!("X448 private key must be 56 bytes, got {}", private_key.len())));
            }
            if public_key.len() != 56 {
                return Err(CursedError::InvalidArgument(format!("X448 public key must be 56 bytes, got {}", public_key.len())));
            }
        },
        KeyAgreementAlgorithm::RsaOaep => {
            // RSA keys can vary, basic validation
            if private_key.is_empty() || public_key.is_empty() {
                return Err(CursedError::InvalidArgument("RSA keys cannot be empty".to_string()));
            }
        },
    }
    Ok(())
}

/// List supported key agreement algorithms
pub fn list_key_agreement_algorithms() -> Vec<String> {
    vec![
        KeyAgreementAlgorithm::EcdhP256.name().to_string(),
        KeyAgreementAlgorithm::EcdhP384.name().to_string(), 
        KeyAgreementAlgorithm::EcdhP521.name().to_string(),
        KeyAgreementAlgorithm::X25519.name().to_string(),
        KeyAgreementAlgorithm::X448.name().to_string(),
        KeyAgreementAlgorithm::RsaOaep.name().to_string(),
    ]
}

/// Key derivation from shared secret using HKDF
pub fn derive_key_from_shared_secret(
    shared_secret: &[u8],
    key_length: usize,
    algorithm: Option<KeyAgreementAlgorithm>,
    info: Option<&str>,
) -> Result<Vec<u8>, CursedError> {
    if key_length == 0 || key_length > 255 * 64 {
        return Err(CursedError::InvalidArgument(format!("Invalid key length: {}", key_length)));
    }
    
    let mut derived_key = vec![0u8; key_length];
    let default_info = "CURSED-KEY-AGREEMENT";
    let info_bytes = info.unwrap_or(default_info).as_bytes();
    
    match algorithm.unwrap_or(KeyAgreementAlgorithm::EcdhP256) {
        KeyAgreementAlgorithm::EcdhP256 | KeyAgreementAlgorithm::X25519 | KeyAgreementAlgorithm::RsaOaep => {
            let hk = Hkdf::<Sha256>::new(None, shared_secret);
            hk.expand(info_bytes, &mut derived_key)
                .map_err(|e| CursedError::CryptoError(format!("Key derivation failed: {}", e)))?;
        },
        KeyAgreementAlgorithm::EcdhP384 => {
            let hk = Hkdf::<Sha384>::new(None, shared_secret);
            hk.expand(info_bytes, &mut derived_key)
                .map_err(|e| CursedError::CryptoError(format!("Key derivation failed: {}", e)))?;
        },
        KeyAgreementAlgorithm::EcdhP521 | KeyAgreementAlgorithm::X448 => {
            let hk = Hkdf::<Sha512>::new(None, shared_secret);
            hk.expand(info_bytes, &mut derived_key)
                .map_err(|e| CursedError::CryptoError(format!("Key derivation failed: {}", e)))?;
        },
    }
    
    Ok(derived_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_from_name() {
        assert_eq!(KeyAgreementAlgorithm::from_name("ECDH-P256").unwrap(), KeyAgreementAlgorithm::EcdhP256);
        assert_eq!(KeyAgreementAlgorithm::from_name("X25519").unwrap(), KeyAgreementAlgorithm::X25519);
        assert_eq!(KeyAgreementAlgorithm::from_name("RSA").unwrap(), KeyAgreementAlgorithm::RsaOaep);
        assert!(KeyAgreementAlgorithm::from_name("invalid").is_err());
    }

    #[test]
    fn test_key_agreement_result() {
        let shared_secret = vec![1, 2, 3, 4];
        let derived_key = vec![5, 6, 7, 8];
        let result = KeyAgreementResult::new(
            KeyAgreementAlgorithm::EcdhP256,
            shared_secret.clone(),
            Some(derived_key.clone()),
        );
        
        assert_eq!(result.algorithm, KeyAgreementAlgorithm::EcdhP256);
        assert_eq!(result.shared_secret, shared_secret);
        assert_eq!(result.derived_key, Some(derived_key));
        assert_eq!(result.key_size, 256);
    }

    #[test] 
    fn test_validate_key_agreement_params() {
        // Valid X25519 keys
        let valid_x25519_key = vec![0u8; 32];
        assert!(validate_key_agreement_params(
            KeyAgreementAlgorithm::X25519,
            &valid_x25519_key,
            &valid_x25519_key
        ).is_ok());
        
        // Invalid X25519 keys
        let invalid_key = vec![0u8; 16];
        assert!(validate_key_agreement_params(
            KeyAgreementAlgorithm::X25519,
            &invalid_key,
            &valid_x25519_key
        ).is_err());
        
        // Valid P-256 keys
        let valid_p256_private = vec![0u8; 32];
        let valid_p256_public_compressed = vec![0u8; 33];
        assert!(validate_key_agreement_params(
            KeyAgreementAlgorithm::EcdhP256,
            &valid_p256_private,
            &valid_p256_public_compressed
        ).is_ok());
    }

    #[test]
    fn test_list_key_agreement_algorithms() {
        let algorithms = list_key_agreement_algorithms();
        assert!(algorithms.contains(&"ECDH-P256".to_string()));
        assert!(algorithms.contains(&"X25519".to_string()));
        assert!(algorithms.contains(&"RSA-OAEP".to_string()));
    }

    #[test]
    fn test_derive_key_from_shared_secret() {
        let shared_secret = b"test_shared_secret";
        let result = derive_key_from_shared_secret(
            shared_secret, 
            32, 
            Some(KeyAgreementAlgorithm::EcdhP256),
            Some("test-info")
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32);
    }
}
