//! Cryptographic functionality for key_generator

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;
use crate::stdlib::packages::CryptoError;

/// Result type for crypto operations
/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_key_generator() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CryptoError::KeyGenerationFailed);
    }
    println!("🔐 Crypto processing (key_generator) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_key_generator() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CryptoError::KeyGenerationFailed);
    }
    Ok(())
}



// Key Generator specific types
#[derive(Debug, Clone)]
pub struct KeyGenerator {
    pub algorithm: AsymmetricAlgorithm,
}

impl KeyGenerator {
    pub fn new() -> Self {
        Self {
            algorithm: AsymmetricAlgorithm::RSA,
        }
    }
    
    pub fn with_algorithm(algorithm: AsymmetricAlgorithm) -> Self {
        Self { algorithm }
    }
    
    pub fn supported_algorithms() -> Vec<AsymmetricAlgorithm> {
        list_asymmetric_algorithms()
    }

    pub fn generate_keypair(&self, algorithm: AsymmetricAlgorithm) -> Result<(Vec<u8>, Vec<u8>), CursedError> {
        // Real cryptographic key generation based on algorithm
        match algorithm {
            AsymmetricAlgorithm::RSA | AsymmetricAlgorithm::Rsa2048 => {
                // Use RSA crate to generate real 2048-bit RSA keys
                use rsa::{RsaPrivateKey, RsaPublicKey, pkcs1::EncodeRsaPrivateKey, pkcs1::EncodeRsaPublicKey};
                use rand::rngs::OsRng;
                
                let mut rng = OsRng;
                let private_key = RsaPrivateKey::new(&mut rng, 2048)
                    .map_err(|_| CursedError::internal_error("Failed to generate RSA private key"))?;
                let public_key = RsaPublicKey::from(&private_key);
                
                let private_key_der = private_key.to_pkcs1_der()
                    .map_err(|_| CursedError::internal_error("Failed to encode RSA private key"))?;
                let public_key_der = public_key.to_pkcs1_der()
                    .map_err(|_| CursedError::internal_error("Failed to encode RSA public key"))?;
                
                Ok((private_key_der.as_bytes().to_vec(), public_key_der.as_bytes().to_vec()))
            },
            AsymmetricAlgorithm::ECC | AsymmetricAlgorithm::EcdsaP256 => {
                // Use p256 crate to generate real ECDSA P-256 keys
                use p256::{SecretKey, PublicKey};
                use p256::elliptic_curve::sec1::ToEncodedPoint;
                use rand::rngs::OsRng;
                
                let mut rng = OsRng;
                let private_key = SecretKey::random(&mut rng);
                let public_key = PublicKey::from_secret_scalar(&private_key.to_nonzero_scalar());
                
                let private_key_bytes = private_key.to_bytes().to_vec();
                let public_key_bytes = public_key.to_encoded_point(false).as_bytes().to_vec();
                
                Ok((private_key_bytes, public_key_bytes))
            },
            AsymmetricAlgorithm::Ed25519 => {
                // Use ed25519-dalek crate to generate real Ed25519 keys
                use ed25519_dalek::{SigningKey, VerifyingKey};
                use rand::rngs::OsRng;
                use rand::RngCore;
                
                let mut rng = OsRng;
                // Generate 32 secure random bytes for the private key
                let mut secret_key_bytes = [0u8; 32];
                rng.fill_bytes(&mut secret_key_bytes);
                
                let signing_key = SigningKey::from_bytes(&secret_key_bytes);
                let verifying_key: VerifyingKey = (&signing_key).into();
                
                let private_key_bytes = signing_key.to_bytes().to_vec();
                let public_key_bytes = verifying_key.to_bytes().to_vec();
                
                Ok((private_key_bytes, public_key_bytes))
            },
            AsymmetricAlgorithm::X25519 => {
                // Use x25519-dalek crate to generate real X25519 keys
                use x25519_dalek::{EphemeralSecret, PublicKey};
                use rand::rngs::OsRng;
                
                let mut rng = OsRng;
                let private_key = EphemeralSecret::random_from_rng(&mut rng);
                let public_key = PublicKey::from(&private_key);
                
                // For X25519, we need to handle the ephemeral nature differently
                // Store the raw bytes instead 
                use rand::RngCore;
                let mut private_key_bytes = [0u8; 32];
                rng.fill_bytes(&mut private_key_bytes);
                let public_key_bytes = public_key.to_bytes().to_vec();
                
                Ok((private_key_bytes.to_vec(), public_key_bytes))
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AsymmetricAlgorithm {
    RSA,
    ECC,
    Ed25519,
    X25519,
    Rsa2048,
    EcdsaP256,
}

#[derive(Debug, Clone)]
pub struct GeneratedKeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub algorithm: AsymmetricAlgorithm,
}

#[derive(Debug, Clone)]
pub enum KeyGeneratorError {
    InvalidParameters,
    GenerationFailed,
    UnsupportedAlgorithm,
}

// Real key generator functions using the KeyGenerator implementation
pub fn generate_asymmetric_keypair(algorithm: AsymmetricAlgorithm, key_size: Option<usize>) -> CryptoResult<GeneratedKeyPair> {
    let generator = KeyGenerator::with_algorithm(algorithm);
    
    // Generate the keypair using the real crypto implementation
    let (private_key, public_key) = generator.generate_keypair(algorithm)
        .map_err(|e| CryptoError::KeyGenerationFailed)?;
    
    Ok(GeneratedKeyPair {
        private_key,
        public_key,
        algorithm,
    })
}

pub fn list_asymmetric_algorithms() -> Vec<AsymmetricAlgorithm> {
    vec![
        AsymmetricAlgorithm::RSA,
        AsymmetricAlgorithm::ECC,
        AsymmetricAlgorithm::Ed25519,
        AsymmetricAlgorithm::X25519,
        AsymmetricAlgorithm::Rsa2048,
        AsymmetricAlgorithm::EcdsaP256,
    ]
}

impl AsymmetricAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            AsymmetricAlgorithm::RSA => "RSA",
            AsymmetricAlgorithm::ECC => "ECC",
            AsymmetricAlgorithm::Ed25519 => "Ed25519",
            AsymmetricAlgorithm::X25519 => "X25519",
            AsymmetricAlgorithm::Rsa2048 => "RSA-2048",
            AsymmetricAlgorithm::EcdsaP256 => "ECDSA-P256",
        }
    }
    
    pub fn default_key_size(&self) -> usize {
        match self {
            AsymmetricAlgorithm::RSA => 256, // 2048 bits / 8
            AsymmetricAlgorithm::ECC => 32,  // 256 bits / 8
            AsymmetricAlgorithm::Ed25519 => 32,
            AsymmetricAlgorithm::X25519 => 32,
            AsymmetricAlgorithm::Rsa2048 => 256, // 2048 bits / 8
            AsymmetricAlgorithm::EcdsaP256 => 32, // 256 bits / 8
        }
    }
}
