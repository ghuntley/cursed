//! Cryptographic functionality for ed25519

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;
use crate::stdlib::packages::CryptoError;

/// Result type for crypto operations
/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_ed25519() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CryptoError::KeyGenerationFailed);
    }
    println!("🔐 Crypto processing (ed25519) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_ed25519() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CryptoError::KeyGenerationFailed);
    }
    Ok(())
}



// Ed25519 specific types
#[derive(Debug, Clone)]
pub struct Ed25519Engine {
    pub context: Option<Vec<u8>>,
}

impl Ed25519Engine {
    pub fn new() -> Self {
        Self { context: None }
    }
    
    pub fn with_context(context: Vec<u8>) -> Self {
        Self { context: Some(context) }
    }

    pub fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), CursedError> {
        // Real Ed25519 keypair generation using cryptographically secure random keys
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
    }
}

#[derive(Debug, Clone)]
pub struct Ed25519KeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Ed25519Error {
    InvalidKey,
    InvalidSignature,
    SignatureFailed,
    VerificationFailed,
}

#[derive(Debug, Clone)]
pub enum Ed25519KeyFormat {
    Raw,
    PKCS8,
    SubjectPublicKeyInfo,
}

// Real ED25519 functions
pub fn ed25519_verify(public_key: &[u8], data: &[u8], signature: &[u8]) -> CryptoResult<bool> {
    // Real Ed25519 signature verification
    use ed25519_dalek::{VerifyingKey, Signature, Verifier};
    
    if public_key.len() != 32 {
        return Err(CryptoError::Other("Invalid public key length for Ed25519".to_string()));
    }
    
    if signature.len() != 64 {
        return Err(CryptoError::Other("Invalid signature length for Ed25519".to_string()));
    }
    
    let public_key_array: [u8; 32] = public_key.try_into()
        .map_err(|_| CryptoError::Other("Failed to convert public key".to_string()))?;
    let signature_array: [u8; 64] = signature.try_into()
        .map_err(|_| CryptoError::Other("Failed to convert signature".to_string()))?;
    
    let verifying_key = VerifyingKey::try_from(&public_key_array[..])
        .map_err(|_| CryptoError::Other("Invalid Ed25519 public key".to_string()))?;
    let signature = Signature::try_from(&signature_array[..])
        .map_err(|_| CryptoError::Other("Invalid Ed25519 signature".to_string()))?;
    
    match verifying_key.verify(data, &signature) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn ed25519_verify_raw(public_key: &[u8], message: &[u8], signature: &[u8]) -> CryptoResult<bool> {
    // Real Ed25519 raw signature verification (same as regular verification for Ed25519)
    ed25519_verify(public_key, message, signature)
}

pub fn ed25519_derive_public_key(private_key: &[u8]) -> CryptoResult<Vec<u8>> {
    // Real Ed25519 public key derivation from private key
    use ed25519_dalek::{SigningKey, VerifyingKey};
    
    if private_key.len() != 32 {
        return Err(CryptoError::Other("Invalid private key length for Ed25519".to_string()));
    }
    
    let private_key_array: [u8; 32] = private_key.try_into()
        .map_err(|_| CryptoError::Other("Failed to convert private key".to_string()))?;
    
    let signing_key = SigningKey::from_bytes(&private_key_array);
    let verifying_key: VerifyingKey = (&signing_key).into();
    
    Ok(verifying_key.to_bytes().to_vec())
}
