// Note: Ed25519PublicKey is defined in this module
/// Ed25519 cryptographic key types for CURSED

use crate::error::{CursedError, Result};

use serde::{Serialize, Deserialize};
use std::fmt;

/// Ed25519 public key
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ed25519PublicKey {
    /// Raw 32-byte public key
    pub bytes: [u8; 32],
}

impl Ed25519PublicKey {
    /// Create a new public key from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }

    /// Get the raw bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.bytes)
    }

    /// Create from hex string
    pub fn from_hex(hex_str: &str) -> Result<Self> {
        let bytes = hex::decode(hex_str)
            .map_err(|e| CursedError::General(format!("Invalid hex: {}", e)))?;
        
        if bytes.len() != 32 {
            return Err(CursedError::General("Ed25519 public key must be 32 bytes".to_string()));
        }

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&bytes);
        Ok(Self::from_bytes(key_bytes))
    }

    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &Ed25519Signature) -> bool {
        // TODO: Implement actual Ed25519 verification
        // For now, return false as a placeholder
        false
    }
}

impl fmt::Display for Ed25519PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ed25519PublicKey({})", self.to_hex())
    }
}

/// Ed25519 private key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ed25519PrivateKey {
    /// Raw 32-byte private key (seed)
    bytes: [u8; 32],
}

impl Ed25519PrivateKey {
    /// Create a new private key from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }

    /// Generate a random private key
    pub fn generate() -> Result<Self> {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        Ok(Self::from_bytes(bytes))
    }

    /// Get the corresponding public key
    pub fn public_key(&self) -> Ed25519PublicKey {
        // TODO: Implement actual Ed25519 public key derivation
        // For now, use a placeholder
        Ed25519PublicKey::from_bytes([0u8; 32])
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Result<Ed25519Signature> {
        // TODO: Implement actual Ed25519 signing
        // For now, return a placeholder signature
        Ok(Ed25519Signature::from_bytes([0u8; 64]))
    }

    /// Convert to hex string (be careful with this!)
    pub fn to_hex(&self) -> String {
        hex::encode(self.bytes)
    }

    /// Create from hex string
    pub fn from_hex(hex_str: &str) -> Result<Self> {
        let bytes = hex::decode(hex_str)
            .map_err(|e| CursedError::General(format!("Invalid hex: {}", e)))?;
        
        if bytes.len() != 32 {
            return Err(CursedError::General("Ed25519 private key must be 32 bytes".to_string()));
        }

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&bytes);
        Ok(Self::from_bytes(key_bytes))
    }
}

impl fmt::Display for Ed25519PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ed25519PrivateKey(***REDACTED***)")
    }
}

impl Drop for Ed25519PrivateKey {
    fn drop(&mut self) {
        // Zero out the private key when dropped
        self.bytes.fill(0);
    }
}

/// Ed25519 keypair
#[derive(Debug, Clone)]
pub struct Ed25519Keypair {
    /// Private key
    pub private_key: Ed25519PrivateKey,
    /// Public key
    pub public_key: Ed25519PublicKey,
}

impl Ed25519Keypair {
    /// Generate a new random keypair
    pub fn generate() -> Result<Self> {
        let private_key = Ed25519PrivateKey::generate()?;
        let public_key = private_key.public_key();
        Ok(Self {
            private_key,
            public_key,
        })
    }

    /// Create from private key
    pub fn from_private_key(private_key: Ed25519PrivateKey) -> Self {
        let public_key = private_key.public_key();
        Self {
            private_key,
            public_key,
        }
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Result<Ed25519Signature> {
        self.private_key.sign(message)
    }

    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &Ed25519Signature) -> bool {
        self.public_key.verify(message, signature)
    }
}

/// Ed25519 signature
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ed25519Signature {
    /// Raw 64-byte signature
    pub bytes: [u8; 64],
}

impl Ed25519Signature {
    /// Create a new signature from bytes
    pub fn from_bytes(bytes: [u8; 64]) -> Self {
        Self { bytes }
    }

    /// Get the raw bytes
    pub fn as_bytes(&self) -> &[u8; 64] {
        &self.bytes
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.bytes)
    }

    /// Create from hex string
    pub fn from_hex(hex_str: &str) -> Result<Self> {
        let bytes = hex::decode(hex_str)
            .map_err(|e| CursedError::General(format!("Invalid hex: {}", e)))?;
        
        if bytes.len() != 64 {
            return Err(CursedError::General("Ed25519 signature must be 64 bytes".to_string()));
        }

        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(&bytes);
        Ok(Self::from_bytes(sig_bytes))
    }
}

impl fmt::Display for Ed25519Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ed25519Signature({})", self.to_hex())
    }
}
