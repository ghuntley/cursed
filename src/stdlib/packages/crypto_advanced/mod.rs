/// fr fr Advanced symmetric encryption for CURSED - maximum security periodt
/// 
/// This module provides state-of-the-art symmetric encryption algorithms
/// with authenticated encryption, constant-time operations, and security-first design.
/// Think authenticated encryption but make it Gen Z bestie!

// Core symmetric encryption implementations
pub mod aes_gcm;
pub mod chacha20_poly1305;
pub mod xchacha20_poly1305;
pub mod symmetric_cipher;
pub mod authenticated_encryption;
pub mod key_management;
pub mod nonce_generator;

// Security and utility modules
pub mod constant_time;
pub mod memory_protection;
pub mod security_analysis;

// Re-export main types for convenience
pub use aes_gcm::{
    AesGcm256, AesGcm192, AesGcm128, AesGcmCipher, AesGcmKey, AesGcmNonce,
    AesGcmResult, AesGcmError, AES_GCM_KEY_SIZE_256, AES_GCM_NONCE_SIZE
};
pub use chacha20_poly1305::{
    ChaCha20Poly1305, ChaCha20Key, ChaCha20Nonce, ChaCha20Result,
    ChaCha20Error, CHACHA20_KEY_SIZE, CHACHA20_NONCE_SIZE
};
pub use xchacha20_poly1305::{
    XChaCha20Poly1305, XChaCha20Key, XChaCha20Nonce, XChaCha20Result,
    XChaCha20Error, XCHACHA20_KEY_SIZE, XCHACHA20_NONCE_SIZE
};
pub use symmetric_cipher::{
    SymmetricCipher, CipherResult, CipherError, CipherType,
    EncryptionContext, DecryptionContext, CipherCapabilities
};
pub use authenticated_encryption::{
    AuthenticatedEncryption, AuthenticationTag, EncryptionResult,
    DecryptionResult, AuthenticationError, TagMismatchError
};
pub use key_management::{
    KeyManager, SecureKey, KeyDerivation, KeyRotation, KeyStorage,
    KeyBackup, KeyRecovery, DerivedKey
};
pub use nonce_generator::{
    NonceGenerator, SecureNonce, NonceCounterMode, NonceRandomMode,
    NonceError, NONCE_UNIQUENESS_GUARANTEE
};
pub use constant_time::{
    ConstantTimeOps, constant_time_compare, constant_time_select,
    constant_time_copy, timing_safe_equal
};
pub use memory_protection::{
    SecureMemory, ZeroOnDrop, ProtectedBytes, MemoryBarrier,
    clear_sensitive_data, memory_lock, memory_unlock
};

use std::sync::Arc;
use std::collections::HashMap;

/// fr fr Global cipher registry for managing available ciphers
static CIPHER_REGISTRY: std::sync::LazyLock<Arc<std::sync::RwLock<CipherRegistry>>> = 
    std::sync::LazyLock::new(|| Arc::new(std::sync::RwLock::new(CipherRegistry::new())));

/// fr fr Cipher registry for managing symmetric ciphers
#[derive(Debug, Default)]
pub struct CipherRegistry {
    ciphers: HashMap<String, Box<dyn SymmetricCipher + Send + Sync>>,
}

impl CipherRegistry {
    /// slay Create a new cipher registry
    pub fn new() -> Self {
        Self {
            ciphers: HashMap::new(),
        }
    }

    /// slay Register a symmetric cipher
    pub fn register_cipher<T>(&mut self, name: String, cipher: T) 
    where
        T: SymmetricCipher + Send + Sync + 'static,
    {
        self.ciphers.insert(name, Box::new(cipher));
    }

    /// slay Get a cipher by name
    pub fn get_cipher(&self, name: &str) -> Option<&(dyn SymmetricCipher + Send + Sync)> {
        self.ciphers.get(name).map(|c| c.as_ref())
    }

    /// slay List all available ciphers
    pub fn list_ciphers(&self) -> Vec<String> {
        self.ciphers.keys().cloned().collect()
    }
}

/// slay Register a cipher globally
pub fn register_cipher<T>(name: &str, cipher: T) -> CipherResult<()>
where
    T: SymmetricCipher + Send + Sync + 'static,
{
    let mut registry = CIPHER_REGISTRY.write()
        .map_err(|_| CipherError::Internal("Failed to acquire cipher registry lock".to_string()))?;
    
    registry.register_cipher(name.to_string(), cipher);
    Ok(())
}

/// slay Get a cipher by name from global registry
pub fn get_cipher(name: &str) -> CipherResult<Arc<dyn SymmetricCipher + Send + Sync>> {
    let registry = CIPHER_REGISTRY.read()
        .map_err(|_| CipherError::Internal("Failed to acquire cipher registry lock".to_string()))?;
    
    registry.get_cipher(name)
        .map(|cipher| {
            // Create a new Arc from the boxed trait object
            unsafe {
                let ptr = cipher as *const (dyn SymmetricCipher + Send + Sync);
                Arc::from_raw(ptr)
            }
        })
        .ok_or_else(|| CipherError::UnsupportedCipher(format!("Cipher '{}' not found", name)))
}

/// slay List all available ciphers globally
pub fn list_ciphers() -> Vec<String> {
    CIPHER_REGISTRY.read()
        .map(|registry| registry.list_ciphers())
        .unwrap_or_default()
}

/// fr fr Crypto utilities and helper functions
pub mod utils {
    use super::*;
    
    /// slay Quick AES-256-GCM encryption (recommended default)
    pub fn quick_encrypt(key: &[u8], plaintext: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        let cipher = AesGcm256::new(key)?;
        cipher.encrypt(plaintext)
    }
    
    /// slay Quick AES-256-GCM decryption
    pub fn quick_decrypt(key: &[u8], ciphertext: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        let cipher = AesGcm256::new(key)?;
        cipher.decrypt(ciphertext)
    }
}

/// fr fr Initialize the crypto_advanced package
pub fn init_crypto_advanced() -> AdvancedCryptoResult<()> {
    println!("🔐 crypto_advanced package initialized - advanced crypto ready bestie!");
    Ok(())
}
