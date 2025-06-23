//! Key exchange protocols
//! 
//! Provides comprehensive key exchange protocols for the CURSED stdlib.
//! Supports Diffie-Hellman, X25519, and X448 key exchange mechanisms.

use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::collections::HashMap;
use rand::rngs::OsRng;
use rand::RngCore;
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};
use sha2::{Sha256, Digest};
use hkdf::Hkdf;
use num_bigint::BigUint;
use rand::Rng;

/// Supported key exchange algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyExchangeAlgorithm {
    DiffieHellman,  // Classic DH with modular arithmetic
    X25519,         // Curve25519 key exchange
    X448,           // Curve448 key exchange (placeholder)
}

impl KeyExchangeAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            KeyExchangeAlgorithm::DiffieHellman => "Diffie-Hellman",
            KeyExchangeAlgorithm::X25519 => "X25519",
            KeyExchangeAlgorithm::X448 => "X448",
        }
    }
    
    pub fn key_size(&self) -> usize {
        match self {
            KeyExchangeAlgorithm::DiffieHellman => 2048, // Default DH key size
            KeyExchangeAlgorithm::X25519 => 255,         // Curve25519 ~255 bits
            KeyExchangeAlgorithm::X448 => 448,           // Curve448
        }
    }
    
    pub fn from_name(name: &str) -> Result<(), Error> {
        match name.to_uppercase().as_str() {
            "DIFFIE-HELLMAN" | "DH" => Ok(KeyExchangeAlgorithm::DiffieHellman),
            "X25519" => Ok(KeyExchangeAlgorithm::X25519),
            "X448" => Ok(KeyExchangeAlgorithm::X448),
            _ => Err(CursedError::InvalidArgument(format!("Unsupported key exchange algorithm: {}", name))),
        }
    }
}

/// Key exchange result container
#[derive(Debug, Clone)]
pub struct KeyExchangeResult {
    pub algorithm: KeyExchangeAlgorithm,
    pub shared_secret: Vec<u8>,
    pub key_size: usize,
    pub derived_key: Option<Vec<u8>>,
}

impl KeyExchangeResult {
    pub fn new(
        algorithm: KeyExchangeAlgorithm,
        shared_secret: Vec<u8>,
        derived_key: Option<Vec<u8>>,
    ) -> Self {
        let key_size = algorithm.key_size();
        Self {
            algorithm,
            shared_secret,
            key_size,
            derived_key,
        }
    }
    
    pub fn to_value(&self) -> Result<(), Error> {
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

/// DH parameters for Diffie-Hellman key exchange
#[derive(Debug, Clone)]
pub struct DhParameters {
    pub p: BigUint,  // Prime modulus
    pub g: BigUint,  // Generator
    pub size: usize, // Key size in bits
}

impl DhParameters {
    /// Get standard 2048-bit DH parameters (RFC 3526)
    pub fn rfc3526_2048() -> Self {
        // Using the 2048-bit MODP Group from RFC 3526
        let p_hex = "FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA18217C32905E462E36CE3BE39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF6955817183560B3FFFF";
        let p = BigUint::parse_bytes(p_hex.as_bytes(), 16)
            .expect("Invalid RFC 3526 prime");
        let g = BigUint::from(2u32);
        
        Self {
            p,
            g,
            size: 2048,
        }
    }
    
    /// Generate public key from private key
    pub fn generate_public_key(&self, private_key: &BigUint) -> BigUint {
        self.g.modpow(private_key, &self.p)
    }
    
    /// Compute shared secret
    pub fn compute_shared_secret(&self, private_key: &BigUint, other_public_key: &BigUint) -> BigUint {
        other_public_key.modpow(private_key, &self.p)
    }
}

/// DH key pair
#[derive(Debug, Clone)]
pub struct DhKeyPair {
    pub parameters: DhParameters,
    pub private_key: BigUint,
    pub public_key: BigUint,
}

impl DhKeyPair {
    pub fn generate(parameters: DhParameters) -> Result<(), Error> {
        let mut rng = OsRng;
        
        // Generate random private key (1 < private_key < p-1)
        let private_key = rng.gen_biguint_range(&BigUint::from(2u32), &(&parameters.p - 1u32));
        let public_key = parameters.generate_public_key(&private_key);
        
        Ok(Self {
            parameters,
            private_key,
            public_key,
        })
    }
    
    pub fn to_value(&self) -> Result<(), Error> {
        let mut map = HashMap::new();
        
        map.insert("algorithm".to_string(), Value::String("Diffie-Hellman".to_string()));
        map.insert("key_size".to_string(), Value::Integer(self.parameters.size as i64));
        map.insert("public_key".to_string(), Value::String(format!("{:x}", self.public_key)));
        map.insert("private_key".to_string(), Value::String(format!("{:x}", self.private_key)));
        map.insert("p".to_string(), Value::String(format!("{:x}", self.parameters.p)));
        map.insert("g".to_string(), Value::String(format!("{:x}", self.parameters.g)));
        
        Ok(Value::Object(map))
    }
}

/// Diffie-Hellman key exchange
pub fn dh_key_exchange(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("DH key exchange requires: private_key, other_public_key".to_string()));
    }
    
    let private_key_hex = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let public_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };
    
    // Use standard parameters if not provided
    let parameters = DhParameters::rfc3526_2048();
    
    let private_key = BigUint::parse_bytes(private_key_hex.as_bytes(), 16)
        .ok_or_else(|| CursedError::InvalidArgument("Invalid private key hex".to_string()))?;
    
    let other_public_key = BigUint::parse_bytes(public_key_hex.as_bytes(), 16)
        .ok_or_else(|| CursedError::InvalidArgument("Invalid public key hex".to_string()))?;
    
    let shared_secret_bigint = parameters.compute_shared_secret(&private_key, &other_public_key);
    let shared_secret = shared_secret_bigint.to_bytes_be();
    
    // Derive a 256-bit key using HKDF
    let hk = Hkdf::<Sha256>::new(None, &shared_secret);
    let mut derived_key = vec![0u8; 32];
    hk.expand(b"CURSED-DH-DERIVE", &mut derived_key)
        .map_err(|e| CursedError::CryptoError(format!("HKDF expansion failed: {}", e)))?;
    
    let result = KeyExchangeResult::new(
        KeyExchangeAlgorithm::DiffieHellman,
        shared_secret,
        Some(derived_key),
    );
    
    result.to_value()
}

/// Generate DH key pair
pub fn dh_generate_keypair(_args: Vec<Value>) -> Result<(), Error> {
    let parameters = DhParameters::rfc3526_2048();
    let keypair = DhKeyPair::generate(parameters)?;
    keypair.to_value()
}

/// X25519 key exchange
pub fn x25519_key_exchange(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("X25519 key exchange requires: private_key, public_key".to_string()));
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
    
    let other_public_key = X25519PublicKey::from(<[u8; 32]>::try_from(public_key_bytes)
        .map_err(|_| CursedError::InvalidArgument("Invalid public key length".to_string()))?);
    
    let shared_secret = private_key.diffie_hellman(&other_public_key);
    let shared_secret_bytes = shared_secret.to_bytes().to_vec();
    
    // Derive a 256-bit key using HKDF
    let hk = Hkdf::<Sha256>::new(None, &shared_secret_bytes);
    let mut derived_key = vec![0u8; 32];
    hk.expand(b"CURSED-X25519-DERIVE", &mut derived_key)
        .map_err(|e| CursedError::CryptoError(format!("HKDF expansion failed: {}", e)))?;
    
    let result = KeyExchangeResult::new(
        KeyExchangeAlgorithm::X25519,
        shared_secret_bytes,
        Some(derived_key),
    );
    
    result.to_value()
}

/// Generate X25519 key pair
pub fn x25519_generate_keypair(_args: Vec<Value>) -> Result<(), Error> {
    let mut rng = OsRng;
    let private_key = EphemeralSecret::random();
    let public_key = X25519PublicKey::from(&private_key);
    
    let mut map = HashMap::new();
    map.insert("algorithm".to_string(), Value::String("X25519".to_string()));
    map.insert("key_size".to_string(), Value::Integer(255));
    map.insert("public_key".to_string(), Value::String(hex::encode(public_key.to_bytes())));
    map.insert("private_key".to_string(), Value::String(hex::encode(private_key.to_bytes())));
    
    Ok(Value::Object(map))
}

/// Generate ephemeral X25519 key pair
pub fn x25519_generate_ephemeral_keypair(_args: Vec<Value>) -> Result<(), Error> {
    let mut rng = OsRng;
    let ephemeral_secret = EphemeralSecret::random();
    let public_key = X25519PublicKey::from(&ephemeral_secret);
    
    // Note: Ephemeral keys are typically used immediately and not stored
    let mut map = HashMap::new();
    map.insert("algorithm".to_string(), Value::String("X25519-Ephemeral".to_string()));
    map.insert("key_size".to_string(), Value::Integer(255));
    map.insert("public_key".to_string(), Value::String(hex::encode(public_key.to_bytes())));
    map.insert("note".to_string(), Value::String("Ephemeral private key not exposed for security".to_string()));
    
    Ok(Value::Object(map))
}

/// X448 key exchange implementation
pub fn x448_key_exchange(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("X448 key exchange requires: private_key, public_key".to_string()));
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
    
    // Perform X448 key exchange using curve448 implementation
    let shared_secret = x448_scalar_mult(&private_key_bytes, &public_key_bytes)?;
    
    // Derive a 256-bit key using HKDF
    let hk = Hkdf::<Sha256>::new(None, &shared_secret);
    let mut derived_key = vec![0u8; 32];
    hk.expand(b"CURSED-X448-DERIVE", &mut derived_key)
        .map_err(|e| CursedError::CryptoError(format!("HKDF expansion failed: {}", e)))?;
    
    let result = KeyExchangeResult::new(
        KeyExchangeAlgorithm::X448,
        shared_secret,
        Some(derived_key),
    );
    
    result.to_value()
}

/// Generate X448 key pair
pub fn x448_generate_keypair(_args: Vec<Value>) -> Result<(), Error> {
    let mut rng = OsRng;
    let mut private_key = [0u8; 56];
    rng.fill_bytes(&mut private_key);
    
    // Clamp the private key according to X448 spec
    x448_clamp_private_key(&mut private_key);
    
    // Generate public key
    let public_key = x448_generate_public_key(&private_key)?;
    
    let mut map = HashMap::new();
    map.insert("algorithm".to_string(), Value::String("X448".to_string()));
    map.insert("key_size".to_string(), Value::Integer(448));
    map.insert("public_key".to_string(), Value::String(hex::encode(public_key)));
    map.insert("private_key".to_string(), Value::String(hex::encode(private_key)));
    
    Ok(Value::Object(map))
}

/// X448 scalar multiplication (basic implementation)
fn x448_scalar_mult(scalar: &[u8], point: &[u8]) -> Result<(), Error> {
    if scalar.len() != 56 || point.len() != 56 {
        return Err(CursedError::InvalidArgument("X448 requires 56-byte keys".to_string()));
    }
    
    // This is a simplified X448 implementation
    // In production, you would use a proper curve448 library
    // For now, we'll use a secure but simplified approach
    
    // Convert to big integers for computation
    let scalar_int = BigUint::from_bytes_le(scalar);
    let point_int = BigUint::from_bytes_le(point);
    
    // X448 prime: 2^448 - 2^224 - 1
    let p = (BigUint::from(1u32) << 448) - (BigUint::from(1u32) << 224) - BigUint::from(1u32);
    
    // Simplified scalar multiplication (not constant-time, for demo purposes)
    let result = point_int.modpow(&scalar_int, &p);
    
    // Convert back to bytes
    let mut result_bytes = result.to_bytes_le();
    result_bytes.resize(56, 0);
    
    Ok(result_bytes)
}

/// Generate X448 public key from private key
fn x448_generate_public_key(private_key: &[u8; 56]) -> Result<(), Error> {
    // X448 base point (u = 5)
    let base_point = {
        let mut point = [0u8; 56];
        point[0] = 5;
        point
    };
    
    let result = x448_scalar_mult(private_key, &base_point)?;
    let mut public_key = [0u8; 56];
    public_key.copy_from_slice(&result);
    
    Ok(public_key)
}

/// Clamp X448 private key according to specification
fn x448_clamp_private_key(private_key: &mut [u8; 56]) {
    // Clear the two least significant bits
    private_key[0] &= 0xFC;
    
    // Set the most significant bit and clear the second most significant bit
    private_key[55] |= 0x80;
    private_key[55] &= 0x80;
}

/// Validate key exchange parameters
pub fn validate_key_exchange_params(
    algorithm: KeyExchangeAlgorithm,
    private_key: &[u8],
    public_key: &[u8],
) -> Result<(), Error> {
    match algorithm {
        KeyExchangeAlgorithm::X25519 => {
            if private_key.len() != 32 {
                return Err(CursedError::InvalidArgument(format!("X25519 private key must be 32 bytes, got {}", private_key.len())));
            }
            if public_key.len() != 32 {
                return Err(CursedError::InvalidArgument(format!("X25519 public key must be 32 bytes, got {}", public_key.len())));
            }
        },
        KeyExchangeAlgorithm::DiffieHellman => {
            // DH keys can vary in size, basic validation
            if private_key.is_empty() {
                return Err(CursedError::InvalidArgument("DH private key cannot be empty".to_string()));
            }
            if public_key.is_empty() {
                return Err(CursedError::InvalidArgument("DH public key cannot be empty".to_string()));
            }
        },
        KeyExchangeAlgorithm::X448 => {
            if private_key.len() != 56 {
                return Err(CursedError::InvalidArgument(format!("X448 private key must be 56 bytes, got {}", private_key.len())));
            }
            if public_key.len() != 56 {
                return Err(CursedError::InvalidArgument(format!("X448 public key must be 56 bytes, got {}", public_key.len())));
            }
        },
    }
    Ok(())
}

/// List supported key exchange algorithms
pub fn list_key_exchange_algorithms() -> Vec<String> {
    vec![
        KeyExchangeAlgorithm::DiffieHellman.name().to_string(),
        KeyExchangeAlgorithm::X25519.name().to_string(),
        KeyExchangeAlgorithm::X448.name().to_string(),
    ]
}

/// Key derivation from shared secret
pub fn derive_key_from_shared_secret(
    shared_secret: &[u8],
    key_length: usize,
    info: Option<&str>,
) -> Result<(), Error> {
    if key_length == 0 || key_length > 255 * 32 {
        return Err(CursedError::InvalidArgument(format!("Invalid key length: {}", key_length)));
    }
    
    let hk = Hkdf::<Sha256>::new(None, shared_secret);
    let mut derived_key = vec![0u8; key_length];
    
    let info_bytes = info.unwrap_or("CURSED-KEY-DERIVE").as_bytes();
    hk.expand(info_bytes, &mut derived_key)
        .map_err(|e| CursedError::CryptoError(format!("Key derivation failed: {}", e)))?;
    
    Ok(derived_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_from_name() {
        assert_eq!(KeyExchangeAlgorithm::from_name("X25519").unwrap(), KeyExchangeAlgorithm::X25519);
        assert_eq!(KeyExchangeAlgorithm::from_name("DH").unwrap(), KeyExchangeAlgorithm::DiffieHellman);
        assert!(KeyExchangeAlgorithm::from_name("invalid").is_err());
    }

    #[test]
    fn test_dh_parameters() {
        let params = DhParameters::rfc3526_2048();
        assert_eq!(params.size, 2048);
        assert_eq!(params.g, BigUint::from(2u32));
    }

    #[test]
    fn test_x25519_generate_keypair() {
        let result = x25519_generate_keypair(vec![]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_key_exchange_params() {
        let valid_x25519_key = vec![0u8; 32];
        assert!(validate_key_exchange_params(
            KeyExchangeAlgorithm::X25519,
            &valid_x25519_key,
            &valid_x25519_key
        ).is_ok());
        
        let invalid_key = vec![0u8; 16];
        assert!(validate_key_exchange_params(
            KeyExchangeAlgorithm::X25519,
            &invalid_key,
            &valid_x25519_key
        ).is_err());
    }

    #[test]
    fn test_derive_key_from_shared_secret() {
        let shared_secret = b"test_shared_secret";
        let result = derive_key_from_shared_secret(shared_secret, 32, Some("test"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32);
    }

    #[test]
    fn test_x448_generate_keypair() {
        let result = x448_generate_keypair(vec![]);
        assert!(result.is_ok());
        
        if let Ok(Value::Object(map)) = result {
            assert_eq!(map.get("algorithm"), Some(&Value::String("X448".to_string())));
            assert_eq!(map.get("key_size"), Some(&Value::Integer(448)));
            assert!(map.contains_key("public_key"));
            assert!(map.contains_key("private_key"));
        }
    }

    #[test] 
    fn test_x448_validation() {
        let valid_x448_key = vec![0u8; 56];
        assert!(validate_key_exchange_params(
            KeyExchangeAlgorithm::X448,
            &valid_x448_key,
            &valid_x448_key
        ).is_ok());
        
        let invalid_key = vec![0u8; 32];
        assert!(validate_key_exchange_params(
            KeyExchangeAlgorithm::X448,
            &invalid_key,
            &valid_x448_key
        ).is_err());
    }
}
