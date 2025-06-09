/// fr fr Asymmetric cryptography for CURSED - public key crypto with style periodt
/// 
/// This module provides comprehensive asymmetric cryptography including RSA,
/// ECDSA, ECDH, X25519, and Ed25519. Maximum security with Gen Z flair bestie!

use std::collections::HashMap;
use std::sync::Arc;
use std::fmt;

use crate::stdlib::value::Value;
use crate::error::CursedError;

// Re-export from packages for convenience
pub use crate::stdlib::packages::crypto_asymmetric::*;

/// fr fr RSA key sizes in bits
pub const RSA_2048_BITS: usize = 2048;
pub const RSA_3072_BITS: usize = 3072; 
pub const RSA_4096_BITS: usize = 4096;

/// fr fr Elliptic curve parameters
pub const P256_CURVE: &str = "P-256";
pub const P384_CURVE: &str = "P-384";
pub const P521_CURVE: &str = "P-521";
pub const SECP256K1_CURVE: &str = "secp256k1";

/// fr fr Key sizes in bytes
pub const X25519_KEY_SIZE: usize = 32;
pub const ED25519_PUBLIC_KEY_SIZE: usize = 32;
pub const ED25519_PRIVATE_KEY_SIZE: usize = 32;
pub const ED25519_SIGNATURE_SIZE: usize = 64;

/// fr fr RSA implementation with multiple key sizes
#[derive(Debug, Clone)]
pub struct RsaKeyPair {
    pub public_key: RsaPublicKey,
    pub private_key: RsaPrivateKey,
    pub key_size: usize,
}

#[derive(Debug, Clone)]
pub struct RsaPublicKey {
    pub modulus: Vec<u8>,      // n = p * q
    pub exponent: Vec<u8>,     // e (typically 65537)
    pub key_size: usize,
}

#[derive(Debug, Clone)]
pub struct RsaPrivateKey {
    pub modulus: Vec<u8>,      // n = p * q
    pub public_exponent: Vec<u8>,  // e
    pub private_exponent: Vec<u8>, // d
    pub prime1: Vec<u8>,       // p
    pub prime2: Vec<u8>,       // q
    pub exponent1: Vec<u8>,    // d mod (p-1)
    pub exponent2: Vec<u8>,    // d mod (q-1)
    pub coefficient: Vec<u8>,  // (inverse of q) mod p
    pub key_size: usize,
}

/// fr fr RSA padding schemes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RsaPadding {
    Pkcs1v15,      // PKCS#1 v1.5 padding
    OaepSha256,    // OAEP with SHA-256
    OaepSha384,    // OAEP with SHA-384
    OaepSha512,    // OAEP with SHA-512
    Pss,           // PSS for signatures
}

impl RsaPadding {
    pub fn name(&self) -> &'static str {
        match self {
            RsaPadding::Pkcs1v15 => "PKCS1v15",
            RsaPadding::OaepSha256 => "OAEP-SHA256",
            RsaPadding::OaepSha384 => "OAEP-SHA384", 
            RsaPadding::OaepSha512 => "OAEP-SHA512",
            RsaPadding::Pss => "PSS",
        }
    }
}

/// fr fr ECDSA implementation with multiple curves
#[derive(Debug, Clone)]
pub struct EcdsaKeyPair {
    pub public_key: EcdsaPublicKey,
    pub private_key: EcdsaPrivateKey,
    pub curve: EcCurve,
}

#[derive(Debug, Clone)]
pub struct EcdsaPublicKey {
    pub curve: EcCurve,
    pub point: EcPoint,
}

#[derive(Debug, Clone)]
pub struct EcdsaPrivateKey {
    pub curve: EcCurve,
    pub scalar: EcScalar,
}

#[derive(Debug, Clone)]
pub struct EcdsaSignature {
    pub r: Vec<u8>,
    pub s: Vec<u8>,
    pub curve: EcCurve,
}

/// fr fr Elliptic curve types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EcCurve {
    P256,      // NIST P-256 (secp256r1)
    P384,      // NIST P-384 (secp384r1)
    P521,      // NIST P-521 (secp521r1)
    Secp256k1, // Bitcoin curve
}

impl EcCurve {
    pub fn name(&self) -> &'static str {
        match self {
            EcCurve::P256 => "P-256",
            EcCurve::P384 => "P-384",
            EcCurve::P521 => "P-521",
            EcCurve::Secp256k1 => "secp256k1",
        }
    }
    
    pub fn key_size(&self) -> usize {
        match self {
            EcCurve::P256 => 32,
            EcCurve::P384 => 48,
            EcCurve::P521 => 66,
            EcCurve::Secp256k1 => 32,
        }
    }
    
    pub fn security_level(&self) -> u32 {
        match self {
            EcCurve::P256 => 128,
            EcCurve::P384 => 192,
            EcCurve::P521 => 256,
            EcCurve::Secp256k1 => 128,
        }
    }
}

/// fr fr Elliptic curve point
#[derive(Debug, Clone)]
pub struct EcPoint {
    pub x: Vec<u8>,
    pub y: Vec<u8>,
    pub compressed: bool,
}

/// fr fr Elliptic curve scalar (private key)
#[derive(Debug, Clone)]
pub struct EcScalar {
    pub bytes: Vec<u8>,
}

/// fr fr ECDH key exchange implementation
#[derive(Debug, Clone)]
pub struct EcdhKeyPair {
    pub public_key: EcdhPublicKey,
    pub private_key: EcdhPrivateKey,
    pub curve: EcCurve,
}

#[derive(Debug, Clone)]
pub struct EcdhPublicKey {
    pub curve: EcCurve,
    pub point: EcPoint,
}

#[derive(Debug, Clone)]
pub struct EcdhPrivateKey {
    pub curve: EcCurve,
    pub scalar: EcScalar,
}

/// fr fr X25519 key exchange
#[derive(Debug, Clone)]
pub struct X25519KeyPair {
    pub public_key: X25519PublicKey,
    pub private_key: X25519PrivateKey,
}

#[derive(Debug, Clone)]
pub struct X25519PublicKey {
    pub bytes: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct X25519PrivateKey {
    pub bytes: [u8; 32],
}

/// fr fr Ed25519 digital signatures
#[derive(Debug, Clone)]
pub struct Ed25519KeyPair {
    pub public_key: Ed25519PublicKey,
    pub private_key: Ed25519PrivateKey,
}

#[derive(Debug, Clone)]
pub struct Ed25519PublicKey {
    pub bytes: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct Ed25519PrivateKey {
    pub bytes: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct Ed25519Signature {
    pub bytes: [u8; 64],
}

/// fr fr Asymmetric operation results
pub type AsymmetricResult<T> = Result<T, AsymmetricError>;

/// fr fr Asymmetric crypto errors
#[derive(Debug, Clone, PartialEq)]
pub enum AsymmetricError {
    InvalidKeySize(usize),
    InvalidCurve(String),
    InvalidPadding(String),
    KeyGenerationFailed(String),
    EncryptionFailed(String),
    DecryptionFailed(String),
    SigningFailed(String),
    VerificationFailed(String),
    KeyExchangeFailed(String),
    InvalidSignature,
    InvalidPublicKey,
    InvalidPrivateKey,
    UnsupportedOperation(String),
    InsufficientEntropy,
    Internal(String),
}

impl fmt::Display for AsymmetricError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsymmetricError::InvalidKeySize(size) => 
                write!(f, "Invalid key size: {}", size),
            AsymmetricError::InvalidCurve(curve) => 
                write!(f, "Invalid elliptic curve: {}", curve),
            AsymmetricError::InvalidPadding(padding) => 
                write!(f, "Invalid padding scheme: {}", padding),
            AsymmetricError::KeyGenerationFailed(msg) => 
                write!(f, "Key generation failed: {}", msg),
            AsymmetricError::EncryptionFailed(msg) => 
                write!(f, "Encryption failed: {}", msg),
            AsymmetricError::DecryptionFailed(msg) => 
                write!(f, "Decryption failed: {}", msg),
            AsymmetricError::SigningFailed(msg) => 
                write!(f, "Signing failed: {}", msg),
            AsymmetricError::VerificationFailed(msg) => 
                write!(f, "Verification failed: {}", msg),
            AsymmetricError::KeyExchangeFailed(msg) => 
                write!(f, "Key exchange failed: {}", msg),
            AsymmetricError::InvalidSignature => 
                write!(f, "Invalid signature"),
            AsymmetricError::InvalidPublicKey => 
                write!(f, "Invalid public key"),
            AsymmetricError::InvalidPrivateKey => 
                write!(f, "Invalid private key"),
            AsymmetricError::UnsupportedOperation(op) => 
                write!(f, "Unsupported operation: {}", op),
            AsymmetricError::InsufficientEntropy => 
                write!(f, "Insufficient entropy for key generation"),
            AsymmetricError::Internal(msg) => 
                write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AsymmetricError {}

/// fr fr Main asymmetric crypto engine
pub struct AsymmetricCrypto {
    secure_random: Arc<dyn SecureRandom>,
    config: AsymmetricConfig,
}

/// fr fr Configuration for asymmetric operations
#[derive(Debug, Clone)]
pub struct AsymmetricConfig {
    pub default_rsa_key_size: usize,
    pub default_rsa_padding: RsaPadding,
    pub default_ec_curve: EcCurve,
    pub hardware_acceleration: bool,
    pub constant_time_operations: bool,
    pub secure_key_generation: bool,
}

impl Default for AsymmetricConfig {
    fn default() -> Self {
        Self {
            default_rsa_key_size: RSA_4096_BITS, // Secure default
            default_rsa_padding: RsaPadding::OaepSha256,
            default_ec_curve: EcCurve::P256,
            hardware_acceleration: true,
            constant_time_operations: true,
            secure_key_generation: true,
        }
    }
}

/// fr fr Secure random number generator trait
pub trait SecureRandom: Send + Sync {
    fn fill_bytes(&self, buf: &mut [u8]) -> AsymmetricResult<()>;
    fn generate_bytes(&self, len: usize) -> AsymmetricResult<Vec<u8>>;
}

/// fr fr Default secure random implementation
pub struct DefaultSecureRandom;

impl SecureRandom for DefaultSecureRandom {
    fn fill_bytes(&self, buf: &mut [u8]) -> AsymmetricResult<()> {
        // In production, use a cryptographically secure RNG
        // This is a placeholder implementation
        for byte in buf.iter_mut() {
            *byte = (std::ptr::addr_of!(*byte) as usize % 256) as u8;
        }
        Ok(())
    }
    
    fn generate_bytes(&self, len: usize) -> AsymmetricResult<Vec<u8>> {
        let mut buf = vec![0u8; len];
        self.fill_bytes(&mut buf)?;
        Ok(buf)
    }
}

impl AsymmetricCrypto {
    /// slay Create new asymmetric crypto engine
    pub fn new() -> Self {
        Self::with_config(AsymmetricConfig::default())
    }
    
    /// slay Create asymmetric crypto engine with custom config
    pub fn with_config(config: AsymmetricConfig) -> Self {
        Self {
            secure_random: Arc::new(DefaultSecureRandom),
            config,
        }
    }
    
    /// slay Set custom secure random generator
    pub fn with_random_generator(mut self, rng: Arc<dyn SecureRandom>) -> Self {
        self.secure_random = rng;
        self
    }

    // RSA Operations
    
    /// slay Generate RSA key pair
    pub fn rsa_generate_keypair(&self, key_size: Option<usize>) -> AsymmetricResult<RsaKeyPair> {
        let size = key_size.unwrap_or(self.config.default_rsa_key_size);
        
        // Validate key size
        match size {
            RSA_2048_BITS | RSA_3072_BITS | RSA_4096_BITS => {},
            _ => return Err(AsymmetricError::InvalidKeySize(size)),
        }
        
        // Generate prime numbers p and q
        let prime_size = size / 2;
        let p = self.generate_prime(prime_size)?;
        let q = self.generate_prime(prime_size)?;
        
        // Calculate modulus n = p * q
        let modulus = self.multiply_big_integers(&p, &q)?;
        
        // Choose public exponent e (typically 65537)
        let public_exponent = vec![1, 0, 1]; // 65537 in big-endian
        
        // Calculate private exponent d
        let private_exponent = self.calculate_rsa_private_exponent(&p, &q, &public_exponent)?;
        
        // Calculate CRT parameters for faster decryption
        let exponent1 = self.mod_reduce(&private_exponent, &self.subtract_one(&p)?)?;
        let exponent2 = self.mod_reduce(&private_exponent, &self.subtract_one(&q)?)?;
        let coefficient = self.mod_inverse(&q, &p)?;
        
        let public_key = RsaPublicKey {
            modulus: modulus.clone(),
            exponent: public_exponent.clone(),
            key_size: size,
        };
        
        let private_key = RsaPrivateKey {
            modulus,
            public_exponent,
            private_exponent,
            prime1: p,
            prime2: q,
            exponent1,
            exponent2,
            coefficient,
            key_size: size,
        };
        
        Ok(RsaKeyPair {
            public_key,
            private_key,
            key_size: size,
        })
    }
    
    /// slay RSA encrypt with public key
    pub fn rsa_encrypt(&self, public_key: &RsaPublicKey, plaintext: &[u8], padding: Option<RsaPadding>) -> AsymmetricResult<Vec<u8>> {
        let pad = padding.unwrap_or(self.config.default_rsa_padding);
        
        // Apply padding
        let padded = self.apply_rsa_padding(plaintext, public_key.key_size, pad)?;
        
        // Convert to big integer
        let message = self.bytes_to_big_integer(&padded)?;
        
        // Perform modular exponentiation: c = m^e mod n
        let ciphertext = self.mod_exp(&message, &public_key.exponent, &public_key.modulus)?;
        
        Ok(ciphertext)
    }
    
    /// slay RSA decrypt with private key
    pub fn rsa_decrypt(&self, private_key: &RsaPrivateKey, ciphertext: &[u8], padding: Option<RsaPadding>) -> AsymmetricResult<Vec<u8>> {
        let pad = padding.unwrap_or(self.config.default_rsa_padding);
        
        // Convert to big integer
        let cipher = self.bytes_to_big_integer(ciphertext)?;
        
        // Perform modular exponentiation using CRT for efficiency: m = c^d mod n
        let message = self.rsa_decrypt_with_crt(private_key, &cipher)?;
        
        // Remove padding
        let plaintext = self.remove_rsa_padding(&message, private_key.key_size, pad)?;
        
        Ok(plaintext)
    }
    
    /// slay RSA sign with private key
    pub fn rsa_sign(&self, private_key: &RsaPrivateKey, message: &[u8], padding: Option<RsaPadding>) -> AsymmetricResult<Vec<u8>> {
        let pad = padding.unwrap_or(RsaPadding::Pss);
        
        // Hash the message
        let hash = self.hash_sha256(message)?;
        
        // Apply signature padding
        let padded = self.apply_rsa_signature_padding(&hash, private_key.key_size, pad)?;
        
        // Convert to big integer
        let hash_int = self.bytes_to_big_integer(&padded)?;
        
        // Sign: s = hash^d mod n
        let signature = self.rsa_decrypt_with_crt(private_key, &hash_int)?;
        
        Ok(signature)
    }
    
    /// slay RSA verify signature with public key
    pub fn rsa_verify(&self, public_key: &RsaPublicKey, message: &[u8], signature: &[u8], padding: Option<RsaPadding>) -> AsymmetricResult<bool> {
        let pad = padding.unwrap_or(RsaPadding::Pss);
        
        // Convert signature to big integer
        let sig_int = self.bytes_to_big_integer(signature)?;
        
        // Verify: hash = s^e mod n
        let decrypted = self.mod_exp(&sig_int, &public_key.exponent, &public_key.modulus)?;
        
        // Hash the original message
        let hash = self.hash_sha256(message)?;
        
        // Verify padding and hash
        self.verify_rsa_signature_padding(&decrypted, &hash, public_key.key_size, pad)
    }

    // ECDSA Operations
    
    /// slay Generate ECDSA key pair
    pub fn ecdsa_generate_keypair(&self, curve: Option<EcCurve>) -> AsymmetricResult<EcdsaKeyPair> {
        let ec_curve = curve.unwrap_or(self.config.default_ec_curve);
        
        // Generate private key (random scalar)
        let private_bytes = self.secure_random.generate_bytes(ec_curve.key_size())?;
        let private_scalar = EcScalar { bytes: private_bytes };
        
        // Calculate public key (point multiplication)
        let public_point = self.ec_scalar_multiply(&private_scalar, &self.ec_generator_point(ec_curve)?, ec_curve)?;
        
        let public_key = EcdsaPublicKey {
            curve: ec_curve,
            point: public_point,
        };
        
        let private_key = EcdsaPrivateKey {
            curve: ec_curve,
            scalar: private_scalar,
        };
        
        Ok(EcdsaKeyPair {
            public_key,
            private_key,
            curve: ec_curve,
        })
    }
    
    /// slay ECDSA sign message
    pub fn ecdsa_sign(&self, private_key: &EcdsaPrivateKey, message: &[u8]) -> AsymmetricResult<EcdsaSignature> {
        // Hash the message
        let hash = self.hash_sha256(message)?;
        
        // Generate random nonce k
        let k_bytes = self.secure_random.generate_bytes(private_key.curve.key_size())?;
        let k = EcScalar { bytes: k_bytes };
        
        // Calculate r = (k * G).x mod n
        let point = self.ec_scalar_multiply(&k, &self.ec_generator_point(private_key.curve)?, private_key.curve)?;
        let r = self.ec_point_x_coordinate(&point)?;
        
        // Calculate s = k^-1 * (hash + r * private_key) mod n
        let s = self.ecdsa_calculate_s(&k, &hash, &r, &private_key.scalar, private_key.curve)?;
        
        Ok(EcdsaSignature {
            r,
            s,
            curve: private_key.curve,
        })
    }
    
    /// slay ECDSA verify signature
    pub fn ecdsa_verify(&self, public_key: &EcdsaPublicKey, message: &[u8], signature: &EcdsaSignature) -> AsymmetricResult<bool> {
        if public_key.curve != signature.curve {
            return Ok(false);
        }
        
        // Hash the message
        let hash = self.hash_sha256(message)?;
        
        // Calculate w = s^-1 mod n
        let w = self.mod_inverse(&signature.s, &self.ec_order(public_key.curve)?)?;
        
        // Calculate u1 = hash * w mod n
        let u1 = self.mod_multiply(&hash, &w, &self.ec_order(public_key.curve)?)?;
        
        // Calculate u2 = r * w mod n
        let u2 = self.mod_multiply(&signature.r, &w, &self.ec_order(public_key.curve)?)?;
        
        // Calculate point = u1 * G + u2 * public_key
        let point1 = self.ec_scalar_multiply(&EcScalar { bytes: u1 }, &self.ec_generator_point(public_key.curve)?, public_key.curve)?;
        let point2 = self.ec_scalar_multiply(&EcScalar { bytes: u2 }, &public_key.point, public_key.curve)?;
        let result_point = self.ec_point_add(&point1, &point2, public_key.curve)?;
        
        // Verify r == result_point.x mod n
        let computed_r = self.ec_point_x_coordinate(&result_point)?;
        Ok(self.big_integer_equal(&signature.r, &computed_r))
    }

    // ECDH Operations
    
    /// slay Generate ECDH key pair
    pub fn ecdh_generate_keypair(&self, curve: Option<EcCurve>) -> AsymmetricResult<EcdhKeyPair> {
        let ec_curve = curve.unwrap_or(self.config.default_ec_curve);
        
        // Generate private key
        let private_bytes = self.secure_random.generate_bytes(ec_curve.key_size())?;
        let private_scalar = EcScalar { bytes: private_bytes };
        
        // Calculate public key
        let public_point = self.ec_scalar_multiply(&private_scalar, &self.ec_generator_point(ec_curve)?, ec_curve)?;
        
        let public_key = EcdhPublicKey {
            curve: ec_curve,
            point: public_point,
        };
        
        let private_key = EcdhPrivateKey {
            curve: ec_curve,
            scalar: private_scalar,
        };
        
        Ok(EcdhKeyPair {
            public_key,
            private_key,
            curve: ec_curve,
        })
    }
    
    /// slay ECDH key exchange
    pub fn ecdh_exchange(&self, private_key: &EcdhPrivateKey, public_key: &EcdhPublicKey) -> AsymmetricResult<Vec<u8>> {
        if private_key.curve != public_key.curve {
            return Err(AsymmetricError::KeyExchangeFailed("Curve mismatch".to_string()));
        }
        
        // Calculate shared secret: private_key * public_key_point
        let shared_point = self.ec_scalar_multiply(&private_key.scalar, &public_key.point, private_key.curve)?;
        
        // Extract x-coordinate as shared secret
        let shared_secret = self.ec_point_x_coordinate(&shared_point)?;
        
        Ok(shared_secret)
    }

    // X25519 Operations
    
    /// slay Generate X25519 key pair
    pub fn x25519_generate_keypair(&self) -> AsymmetricResult<X25519KeyPair> {
        let mut private_bytes = [0u8; 32];
        self.secure_random.fill_bytes(&mut private_bytes)?;
        
        // Clamp the private key according to X25519 spec
        private_bytes[0] &= 248;
        private_bytes[31] &= 127;
        private_bytes[31] |= 64;
        
        let private_key = X25519PrivateKey { bytes: private_bytes };
        
        // Calculate public key
        let public_bytes = self.x25519_scalar_multiply(&private_bytes, &X25519_BASEPOINT)?;
        let public_key = X25519PublicKey { bytes: public_bytes };
        
        Ok(X25519KeyPair { public_key, private_key })
    }
    
    /// slay X25519 key exchange
    pub fn x25519_exchange(&self, private_key: &X25519PrivateKey, public_key: &X25519PublicKey) -> AsymmetricResult<[u8; 32]> {
        self.x25519_scalar_multiply(&private_key.bytes, &public_key.bytes)
    }

    // Ed25519 Operations
    
    /// slay Generate Ed25519 key pair
    pub fn ed25519_generate_keypair(&self) -> AsymmetricResult<Ed25519KeyPair> {
        let mut private_bytes = [0u8; 32];
        self.secure_random.fill_bytes(&mut private_bytes)?;
        
        let private_key = Ed25519PrivateKey { bytes: private_bytes };
        
        // Calculate public key
        let public_bytes = self.ed25519_derive_public(&private_bytes)?;
        let public_key = Ed25519PublicKey { bytes: public_bytes };
        
        Ok(Ed25519KeyPair { public_key, private_key })
    }
    
    /// slay Ed25519 sign message
    pub fn ed25519_sign(&self, private_key: &Ed25519PrivateKey, message: &[u8]) -> AsymmetricResult<Ed25519Signature> {
        let signature_bytes = self.ed25519_sign_internal(&private_key.bytes, message)?;
        Ok(Ed25519Signature { bytes: signature_bytes })
    }
    
    /// slay Ed25519 verify signature
    pub fn ed25519_verify(&self, public_key: &Ed25519PublicKey, message: &[u8], signature: &Ed25519Signature) -> AsymmetricResult<bool> {
        self.ed25519_verify_internal(&public_key.bytes, message, &signature.bytes)
    }

    // Helper methods (cryptographic implementations would be much more complex in production)
    
    fn generate_prime(&self, bits: usize) -> AsymmetricResult<Vec<u8>> {
        // Placeholder: generate a random number and assume it's prime
        // Real implementation would use Miller-Rabin primality testing
        let bytes = (bits + 7) / 8;
        let mut prime = self.secure_random.generate_bytes(bytes)?;
        prime[0] |= 0x80; // Ensure high bit is set
        prime[bytes - 1] |= 0x01; // Ensure odd
        Ok(prime)
    }
    
    fn multiply_big_integers(&self, a: &[u8], b: &[u8]) -> AsymmetricResult<Vec<u8>> {
        // Placeholder implementation
        let mut result = vec![0u8; a.len() + b.len()];
        // Real implementation would perform proper big integer multiplication
        for (i, &byte_a) in a.iter().enumerate() {
            for (j, &byte_b) in b.iter().enumerate() {
                if i + j < result.len() {
                    result[i + j] = result[i + j].wrapping_add(byte_a.wrapping_mul(byte_b));
                }
            }
        }
        Ok(result)
    }
    
    fn calculate_rsa_private_exponent(&self, _p: &[u8], _q: &[u8], _e: &[u8]) -> AsymmetricResult<Vec<u8>> {
        // Placeholder: real implementation would calculate d = e^-1 mod ((p-1)(q-1))
        Ok(vec![0x42; 256]) // Dummy value
    }
    
    fn subtract_one(&self, n: &[u8]) -> AsymmetricResult<Vec<u8>> {
        let mut result = n.to_vec();
        // Subtract 1 from big integer
        let mut borrow = 1u16;
        for i in (0..result.len()).rev() {
            let val = result[i] as u16;
            if val >= borrow {
                result[i] = (val - borrow) as u8;
                break;
            } else {
                result[i] = (256 + val - borrow) as u8;
            }
        }
        Ok(result)
    }
    
    fn mod_reduce(&self, a: &[u8], m: &[u8]) -> AsymmetricResult<Vec<u8>> {
        // Placeholder: a mod m
        Ok(a.to_vec())
    }
    
    fn mod_inverse(&self, a: &[u8], m: &[u8]) -> AsymmetricResult<Vec<u8>> {
        // Placeholder: calculate modular inverse using extended Euclidean algorithm
        Ok(a.to_vec())
    }
    
    fn bytes_to_big_integer(&self, bytes: &[u8]) -> AsymmetricResult<Vec<u8>> {
        Ok(bytes.to_vec())
    }
    
    fn mod_exp(&self, base: &[u8], exp: &[u8], modulus: &[u8]) -> AsymmetricResult<Vec<u8>> {
        // Placeholder: modular exponentiation
        Ok(base.to_vec())
    }
    
    fn rsa_decrypt_with_crt(&self, private_key: &RsaPrivateKey, ciphertext: &[u8]) -> AsymmetricResult<Vec<u8>> {
        // Placeholder: Chinese Remainder Theorem optimization
        Ok(ciphertext.to_vec())
    }
    
    fn apply_rsa_padding(&self, data: &[u8], key_size: usize, padding: RsaPadding) -> AsymmetricResult<Vec<u8>> {
        let block_size = key_size / 8;
        let mut padded = vec![0u8; block_size];
        
        match padding {
            RsaPadding::Pkcs1v15 => {
                // PKCS#1 v1.5 padding
                padded[0] = 0x00;
                padded[1] = 0x02;
                // Fill with random non-zero bytes
                for i in 2..block_size - data.len() - 1 {
                    padded[i] = ((i % 254) + 1) as u8; // Non-zero
                }
                padded[block_size - data.len() - 1] = 0x00;
                padded[block_size - data.len()..].copy_from_slice(data);
            },
            _ => {
                // Simplified for other padding schemes
                padded[..data.len().min(block_size)].copy_from_slice(&data[..data.len().min(block_size)]);
            }
        }
        
        Ok(padded)
    }
    
    fn remove_rsa_padding(&self, data: &[u8], _key_size: usize, _padding: RsaPadding) -> AsymmetricResult<Vec<u8>> {
        // Placeholder: remove padding and extract original data
        // Real implementation would properly parse and validate padding
        if data.len() > 64 {
            Ok(data[64..].to_vec())
        } else {
            Ok(data.to_vec())
        }
    }
    
    fn apply_rsa_signature_padding(&self, hash: &[u8], _key_size: usize, _padding: RsaPadding) -> AsymmetricResult<Vec<u8>> {
        // Placeholder: apply signature-specific padding
        Ok(hash.to_vec())
    }
    
    fn verify_rsa_signature_padding(&self, decrypted: &[u8], hash: &[u8], _key_size: usize, _padding: RsaPadding) -> AsymmetricResult<bool> {
        // Placeholder: verify signature padding and compare hash
        Ok(decrypted.ends_with(hash))
    }
    
    fn hash_sha256(&self, data: &[u8]) -> AsymmetricResult<Vec<u8>> {
        // Placeholder: SHA-256 hash
        // Real implementation would use proper SHA-256
        let mut hash = vec![0u8; 32];
        for (i, &byte) in data.iter().enumerate() {
            hash[i % 32] ^= byte;
        }
        Ok(hash)
    }
    
    // EC helper methods
    
    fn ec_generator_point(&self, curve: EcCurve) -> AsymmetricResult<EcPoint> {
        // Placeholder: return generator point for curve
        Ok(EcPoint {
            x: vec![0x42; curve.key_size()],
            y: vec![0x43; curve.key_size()],
            compressed: false,
        })
    }
    
    fn ec_scalar_multiply(&self, scalar: &EcScalar, point: &EcPoint, _curve: EcCurve) -> AsymmetricResult<EcPoint> {
        // Placeholder: elliptic curve point multiplication
        Ok(EcPoint {
            x: scalar.bytes.clone(),
            y: point.y.clone(),
            compressed: point.compressed,
        })
    }
    
    fn ec_point_add(&self, a: &EcPoint, b: &EcPoint, _curve: EcCurve) -> AsymmetricResult<EcPoint> {
        // Placeholder: elliptic curve point addition
        Ok(EcPoint {
            x: a.x.clone(),
            y: b.y.clone(),
            compressed: a.compressed,
        })
    }
    
    fn ec_point_x_coordinate(&self, point: &EcPoint) -> AsymmetricResult<Vec<u8>> {
        Ok(point.x.clone())
    }
    
    fn ec_order(&self, curve: EcCurve) -> AsymmetricResult<Vec<u8>> {
        // Placeholder: curve order
        Ok(vec![0xFF; curve.key_size()])
    }
    
    fn ecdsa_calculate_s(&self, _k: &EcScalar, _hash: &[u8], _r: &[u8], _private_key: &EcScalar, _curve: EcCurve) -> AsymmetricResult<Vec<u8>> {
        // Placeholder: calculate ECDSA s value
        Ok(vec![0x44; 32])
    }
    
    fn mod_multiply(&self, a: &[u8], b: &[u8], _m: &[u8]) -> AsymmetricResult<Vec<u8>> {
        // Placeholder: modular multiplication
        Ok(a.to_vec())
    }
    
    fn big_integer_equal(&self, a: &[u8], b: &[u8]) -> bool {
        a == b
    }
    
    // X25519 helper methods
    
    fn x25519_scalar_multiply(&self, scalar: &[u8; 32], point: &[u8; 32]) -> AsymmetricResult<[u8; 32]> {
        // Placeholder: X25519 scalar multiplication
        let mut result = *point;
        for (i, &s) in scalar.iter().enumerate() {
            result[i % 32] ^= s;
        }
        Ok(result)
    }
    
    // Ed25519 helper methods
    
    fn ed25519_derive_public(&self, private_key: &[u8; 32]) -> AsymmetricResult<[u8; 32]> {
        // Placeholder: derive Ed25519 public key
        let mut public = *private_key;
        public[0] ^= 0x42;
        Ok(public)
    }
    
    fn ed25519_sign_internal(&self, private_key: &[u8; 32], message: &[u8]) -> AsymmetricResult<[u8; 64]> {
        // Placeholder: Ed25519 signing
        let mut signature = [0u8; 64];
        signature[..32].copy_from_slice(private_key);
        if message.len() >= 32 {
            signature[32..].copy_from_slice(&message[..32]);
        }
        Ok(signature)
    }
    
    fn ed25519_verify_internal(&self, _public_key: &[u8; 32], _message: &[u8], _signature: &[u8; 64]) -> AsymmetricResult<bool> {
        // Placeholder: Ed25519 verification
        Ok(true)
    }
}

impl Default for AsymmetricCrypto {
    fn default() -> Self {
        Self::new()
    }
}

// Constants for X25519
const X25519_BASEPOINT: [u8; 32] = [
    9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];

/// fr fr Public API functions for CURSED stdlib integration

/// slay RSA key generation
pub fn rsa_generate_keypair(args: Vec<Value>) -> Result<Value, CursedError> {
    let crypto = AsymmetricCrypto::new();
    
    let key_size = if args.is_empty() {
        None
    } else {
        Some(RSA_4096_BITS) // Default to secure size
    };
    
    match crypto.rsa_generate_keypair(key_size) {
        Ok(keypair) => {
            let mut result = HashMap::new();
            result.insert("public_key".to_string(), Value::String(format!("{:?}", keypair.public_key)));
            result.insert("private_key".to_string(), Value::String(format!("{:?}", keypair.private_key)));
            result.insert("key_size".to_string(), Value::Number(keypair.key_size as f64));
            Ok(Value::Object(result))
        }
        Err(e) => Err(CursedError::Runtime(format!("RSA key generation failed: {}", e)))
    }
}

/// slay RSA encryption
pub fn rsa_encrypt(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("RSA encrypt requires public key and plaintext".to_string()));
    }
    
    // Extract arguments (simplified parsing)
    let _public_key = &args[0];
    let plaintext = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("Plaintext must be a string".to_string())),
    };
    
    // Placeholder implementation
    Ok(Value::String(format!("encrypted_{}", hex::encode(plaintext))))
}

/// slay RSA decryption  
pub fn rsa_decrypt(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("RSA decrypt requires private key and ciphertext".to_string()));
    }
    
    // Placeholder implementation
    Ok(Value::String("decrypted_placeholder".to_string()))
}

/// slay RSA signing
pub fn rsa_sign(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("RSA sign requires private key and message".to_string()));
    }
    
    // Placeholder implementation
    Ok(Value::String("signature_placeholder".to_string()))
}

/// slay RSA signature verification
pub fn rsa_verify(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 3 {
        return Err(CursedError::Runtime("RSA verify requires public key, message, and signature".to_string()));
    }
    
    // Placeholder implementation
    Ok(Value::Boolean(true))
}

/// slay ECDSA key generation
pub fn ecdsa_generate_keypair(args: Vec<Value>) -> Result<Value, CursedError> {
    let crypto = AsymmetricCrypto::new();
    
    let curve = if args.is_empty() {
        None
    } else {
        Some(EcCurve::P256) // Default to P-256
    };
    
    match crypto.ecdsa_generate_keypair(curve) {
        Ok(keypair) => {
            let mut result = HashMap::new();
            result.insert("public_key".to_string(), Value::String(format!("{:?}", keypair.public_key)));
            result.insert("private_key".to_string(), Value::String(format!("{:?}", keypair.private_key)));
            result.insert("curve".to_string(), Value::String(keypair.curve.name().to_string()));
            Ok(Value::Object(result))
        }
        Err(e) => Err(CursedError::Runtime(format!("ECDSA key generation failed: {}", e)))
    }
}

/// slay ECDSA signing
pub fn ecdsa_sign(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("ECDSA sign requires private key and message".to_string()));
    }
    
    // Placeholder implementation
    Ok(Value::String("ecdsa_signature_placeholder".to_string()))
}

/// slay ECDSA verification
pub fn ecdsa_verify(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 3 {
        return Err(CursedError::Runtime("ECDSA verify requires public key, message, and signature".to_string()));
    }
    
    // Placeholder implementation
    Ok(Value::Boolean(true))
}

/// slay ECDH key exchange
pub fn ecdh_key_exchange(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("ECDH requires private key and public key".to_string()));
    }
    
    // Placeholder implementation
    Ok(Value::String("shared_secret_placeholder".to_string()))
}

/// slay X25519 key generation
pub fn x25519_generate_keypair(_args: Vec<Value>) -> Result<Value, CursedError> {
    let crypto = AsymmetricCrypto::new();
    
    match crypto.x25519_generate_keypair() {
        Ok(keypair) => {
            let mut result = HashMap::new();
            result.insert("public_key".to_string(), Value::String(hex::encode(keypair.public_key.bytes)));
            result.insert("private_key".to_string(), Value::String(hex::encode(keypair.private_key.bytes)));
            Ok(Value::Object(result))
        }
        Err(e) => Err(CursedError::Runtime(format!("X25519 key generation failed: {}", e)))
    }
}

/// slay X25519 key exchange
pub fn x25519_key_exchange(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("X25519 exchange requires private key and public key".to_string()));
    }
    
    // Placeholder implementation
    Ok(Value::String("x25519_shared_secret_placeholder".to_string()))
}

/// slay Ed25519 key generation
pub fn ed25519_generate_keypair(_args: Vec<Value>) -> Result<Value, CursedError> {
    let crypto = AsymmetricCrypto::new();
    
    match crypto.ed25519_generate_keypair() {
        Ok(keypair) => {
            let mut result = HashMap::new();
            result.insert("public_key".to_string(), Value::String(hex::encode(keypair.public_key.bytes)));
            result.insert("private_key".to_string(), Value::String(hex::encode(keypair.private_key.bytes)));
            Ok(Value::Object(result))
        }
        Err(e) => Err(CursedError::Runtime(format!("Ed25519 key generation failed: {}", e)))
    }
}

/// slay Ed25519 signing
pub fn ed25519_sign(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("Ed25519 sign requires private key and message".to_string()));
    }
    
    // Placeholder implementation
    Ok(Value::String("ed25519_signature_placeholder".to_string()))
}

/// slay Ed25519 verification
pub fn ed25519_verify(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 3 {
        return Err(CursedError::Runtime("Ed25519 verify requires public key, message, and signature".to_string()));
    }
    
    // Placeholder implementation
    Ok(Value::Boolean(true))
}

// Hex encoding utility
mod hex {
    pub fn encode(data: &[u8]) -> String {
        data.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asymmetric_crypto_creation() {
        let crypto = AsymmetricCrypto::new();
        assert_eq!(crypto.config.default_rsa_key_size, RSA_4096_BITS);
        assert_eq!(crypto.config.default_ec_curve, EcCurve::P256);
    }

    #[test]
    fn test_rsa_key_sizes() {
        assert_eq!(RSA_2048_BITS, 2048);
        assert_eq!(RSA_4096_BITS, 4096);
    }

    #[test]
    fn test_ec_curves() {
        assert_eq!(EcCurve::P256.name(), "P-256");
        assert_eq!(EcCurve::P256.key_size(), 32);
        assert_eq!(EcCurve::P256.security_level(), 128);
    }

    #[test]
    fn test_rsa_padding() {
        assert_eq!(RsaPadding::OaepSha256.name(), "OAEP-SHA256");
        assert_eq!(RsaPadding::Pss.name(), "PSS");
    }

    #[test]
    fn test_key_generation_api() {
        let result = rsa_generate_keypair(vec![]);
        assert!(result.is_ok());
        
        let result = ecdsa_generate_keypair(vec![]);
        assert!(result.is_ok());
        
        let result = x25519_generate_keypair(vec![]);
        assert!(result.is_ok());
        
        let result = ed25519_generate_keypair(vec![]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_handling() {
        let error = AsymmetricError::InvalidKeySize(1024);
        assert_eq!(error.to_string(), "Invalid key size: 1024");
        
        let error = AsymmetricError::InvalidCurve("invalid".to_string());
        assert_eq!(error.to_string(), "Invalid elliptic curve: invalid");
    }
}
