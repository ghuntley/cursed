//! ChaCha20-Poly1305 authenticated encryption
//! 
//! Provides ChaCha20-Poly1305 AEAD implementation for the CURSED stdlib.

use crate::stdlib::value::Value;
use crate::error::CursedError;

/// ChaCha20-Poly1305 encryption function
pub fn chacha20_poly1305_encrypt(_args: Vec<Value>) -> Result<Value, CursedError> {
    // TODO: Implement ChaCha20-Poly1305 encryption
    Err(CursedError::NotImplemented("ChaCha20-Poly1305 encryption not yet implemented".to_string()))
}

/// ChaCha20-Poly1305 decryption function
pub fn chacha20_poly1305_decrypt(_args: Vec<Value>) -> Result<Value, CursedError> {
    // TODO: Implement ChaCha20-Poly1305 decryption
    Err(CursedError::NotImplemented("ChaCha20-Poly1305 decryption not yet implemented".to_string()))
}
