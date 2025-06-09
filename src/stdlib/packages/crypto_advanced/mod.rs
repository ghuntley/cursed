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
    
    /// slay Quick encryption with AES-256-GCM (recommended default)
    pub fn quick_encrypt(key: &[u8], plaintext: &[u8], additional_data: Option<&[u8]>) -> CipherResult<Vec<u8>> {
        let cipher = AesGcm256::new(key)?;
        cipher.encrypt(plaintext, additional_data.unwrap_or(&[]))
    }
    
    /// slay Quick decryption with AES-256-GCM
    pub fn quick_decrypt(key: &[u8], ciphertext: &[u8], additional_data: Option<&[u8]>) -> CipherResult<Vec<u8>> {
        let cipher = AesGcm256::new(key)?;
        cipher.decrypt(ciphertext, additional_data.unwrap_or(&[]))
    }
    
    /// slay Generate a secure encryption key
    pub fn generate_key(cipher_type: CipherType) -> CipherResult<SecureKey> {
        KeyManager::generate_key(cipher_type)
    }
    
    /// slay Generate a secure nonce
    pub fn generate_nonce(cipher_type: CipherType) -> CipherResult<SecureNonce> {
        NonceGenerator::generate(cipher_type)
    }
    
    /// slay Check if a cipher is available
    pub fn is_cipher_available(name: &str) -> bool {
        list_ciphers().contains(&name.to_string())
    }
    
    /// slay Get cipher capabilities
    pub fn get_cipher_capabilities(cipher_type: CipherType) -> CipherCapabilities {
        match cipher_type {
            CipherType::AesGcm256 => CipherCapabilities {
                key_size: AES_GCM_KEY_SIZE_256,
                nonce_size: AES_GCM_NONCE_SIZE,
                tag_size: 16,
                authenticated: true,
                constant_time: true,
                quantum_resistant: false,
            },
            CipherType::ChaCha20Poly1305 => CipherCapabilities {
                key_size: CHACHA20_KEY_SIZE,
                nonce_size: CHACHA20_NONCE_SIZE,
                tag_size: 16,
                authenticated: true,
                constant_time: true,
                quantum_resistant: false,
            },
            CipherType::XChaCha20Poly1305 => CipherCapabilities {
                key_size: XCHACHA20_KEY_SIZE,
                nonce_size: XCHACHA20_NONCE_SIZE,
                tag_size: 16,
                authenticated: true,
                constant_time: true,
                quantum_resistant: false,
            },
        }
    }
}

/// fr fr Security configuration for crypto operations
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub constant_time_operations: bool,
    pub memory_protection: bool,
    pub timing_attack_protection: bool,
    pub side_channel_protection: bool,
    pub quantum_resistance: bool,
    pub key_rotation_enabled: bool,
    pub secure_random_enabled: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            constant_time_operations: true,
            memory_protection: true,
            timing_attack_protection: true,
            side_channel_protection: true,
            quantum_resistance: false, // Not yet available
            key_rotation_enabled: true,
            secure_random_enabled: true,
        }
    }
}

/// fr fr Initialize the crypto_advanced package
pub fn init_crypto_advanced() -> CipherResult<()> {
    // Register default ciphers
    register_cipher("aes-256-gcm", AesGcm256::default())?;
    register_cipher("chacha20-poly1305", ChaCha20Poly1305::default())?;
    register_cipher("xchacha20-poly1305", XChaCha20Poly1305::default())?;
    
    println!("🔒 crypto_advanced package initialized - maximum security engaged bestie!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cipher_registry() {
        let mut registry = CipherRegistry::new();
        assert_eq!(registry.list_ciphers().len(), 0);
        assert!(registry.get_cipher("nonexistent").is_none());
    }
    
    #[test]
    fn test_init_crypto_advanced() {
        assert!(init_crypto_advanced().is_ok());
    }
    
    #[test]
    fn test_security_config() {
        let config = SecurityConfig::default();
        assert!(config.constant_time_operations);
        assert!(config.memory_protection);
        assert!(config.timing_attack_protection);
    }
    
    #[test]
    fn test_utils() {
        assert!(!utils::is_cipher_available("nonexistent"));
        
        let caps = utils::get_cipher_capabilities(CipherType::AesGcm256);
        assert_eq!(caps.key_size, 32);
        assert!(caps.authenticated);
        assert!(caps.constant_time);
    }
}
