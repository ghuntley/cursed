/// fr fr Core symmetric cipher trait and types - the foundation of all encryption bestie

use std::fmt::Debug;

/// fr fr Supported cipher types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CipherType {
    AesGcm256,
    AesGcm192,
    AesGcm128,
    ChaCha20Poly1305,
    XChaCha20Poly1305,
}

/// fr fr Cipher capabilities structure
#[derive(Debug, Clone)]
pub struct CipherCapabilities {
    pub key_size: usize,
    pub nonce_size: usize,
    pub tag_size: usize,
    pub authenticated: bool,
    pub constant_time: bool,
    pub quantum_resistant: bool,
}

/// fr fr Encryption context for additional security parameters
#[derive(Debug, Clone)]
pub struct EncryptionContext {
    pub additional_data: Vec<u8>,
    pub nonce: Option<Vec<u8>>,
    pub key_derivation_salt: Option<Vec<u8>>,
}

impl Default for EncryptionContext {
    fn default() -> Self {
        Self {
            additional_data: Vec::new(),
            nonce: None,
            key_derivation_salt: None,
        }
    }
}

/// fr fr Decryption context for additional security parameters
#[derive(Debug, Clone)]
pub struct DecryptionContext {
    pub additional_data: Vec<u8>,
    pub expected_tag: Option<Vec<u8>>,
    pub verify_integrity: bool,
}

impl Default for DecryptionContext {
    fn default() -> Self {
        Self {
            additional_data: Vec::new(),
            expected_tag: None,
            verify_integrity: true,
        }
    }
}

/// fr fr Cipher errors
#[derive(Debug, Clone, PartialEq)]
pub enum CipherError {
    InvalidKeySize(usize, usize), // (provided, expected)
    InvalidNonceSize(usize, usize), // (provided, expected)
    InvalidCiphertext,
    AuthenticationFailed,
    UnsupportedCipher(String),
    KeyDerivationFailed,
    NonceGenerationFailed,
    MemoryProtectionFailed,
    Internal(String),
}

impl std::fmt::Display for CipherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CipherError::InvalidKeySize(provided, expected) => 
                write!(f, "Invalid key size: provided {}, expected {}", provided, expected),
            CipherError::InvalidNonceSize(provided, expected) => 
                write!(f, "Invalid nonce size: provided {}, expected {}", provided, expected),
            CipherError::InvalidCiphertext => write!(f, "Invalid ciphertext format"),
            CipherError::AuthenticationFailed => write!(f, "Authentication tag verification failed"),
            CipherError::UnsupportedCipher(name) => write!(f, "Unsupported cipher: {}", name),
            CipherError::KeyDerivationFailed => write!(f, "Key derivation failed"),
            CipherError::NonceGenerationFailed => write!(f, "Nonce generation failed"),
            CipherError::MemoryProtectionFailed => write!(f, "Memory protection failed"),
            CipherError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for CipherError {}

/// fr fr Cipher result type
pub type CipherResult<T> = Result<T, CipherError>;

/// fr fr Main symmetric cipher trait - all ciphers must implement this periodt
pub trait SymmetricCipher: Debug + Send + Sync {
    /// slay Get cipher type
    fn cipher_type(&self) -> CipherType;
    
    /// slay Get cipher capabilities
    fn capabilities(&self) -> CipherCapabilities;
    
    /// slay Encrypt plaintext with optional additional authenticated data
    fn encrypt(&self, plaintext: &[u8], additional_data: &[u8]) -> CipherResult<Vec<u8>>;
    
    /// slay Encrypt with full context
    fn encrypt_with_context(&self, plaintext: &[u8], context: &EncryptionContext) -> CipherResult<Vec<u8>>;
    
    /// slay Decrypt ciphertext with optional additional authenticated data
    fn decrypt(&self, ciphertext: &[u8], additional_data: &[u8]) -> CipherResult<Vec<u8>>;
    
    /// slay Decrypt with full context
    fn decrypt_with_context(&self, ciphertext: &[u8], context: &DecryptionContext) -> CipherResult<Vec<u8>>;
    
    /// slay Generate a new key for this cipher
    fn generate_key(&self) -> CipherResult<Vec<u8>>;
    
    /// slay Generate a new nonce for this cipher
    fn generate_nonce(&self) -> CipherResult<Vec<u8>>;
    
    /// slay Validate key size
    fn validate_key(&self, key: &[u8]) -> CipherResult<()> {
        let expected_size = self.capabilities().key_size;
        if key.len() != expected_size {
            return Err(CipherError::InvalidKeySize(key.len(), expected_size));
        }
        Ok(())
    }
    
    /// slay Validate nonce size
    fn validate_nonce(&self, nonce: &[u8]) -> CipherResult<()> {
        let expected_size = self.capabilities().nonce_size;
        if nonce.len() != expected_size {
            return Err(CipherError::InvalidNonceSize(nonce.len(), expected_size));
        }
        Ok(())
    }
    
    /// slay Check if cipher supports authenticated encryption
    fn is_authenticated(&self) -> bool {
        self.capabilities().authenticated
    }
    
    /// slay Check if cipher operations are constant-time
    fn is_constant_time(&self) -> bool {
        self.capabilities().constant_time
    }
    
    /// slay Clone the cipher (for thread safety)
    fn clone_cipher(&self) -> Box<dyn SymmetricCipher>;
}

/// fr fr Default implementation structure for common cipher functionality
#[derive(Debug, Clone)]
pub struct CipherBase {
    pub cipher_type: CipherType,
    pub capabilities: CipherCapabilities,
}

impl CipherBase {
    /// slay Create a new cipher base
    pub fn new(cipher_type: CipherType, capabilities: CipherCapabilities) -> Self {
        Self {
            cipher_type,
            capabilities,
        }
    }
    
    /// slay Validate encryption parameters
    pub fn validate_encrypt_params(&self, key: &[u8], nonce: Option<&[u8]>) -> CipherResult<()> {
        // Validate key size
        if key.len() != self.capabilities.key_size {
            return Err(CipherError::InvalidKeySize(key.len(), self.capabilities.key_size));
        }
        
        // Validate nonce size if provided
        if let Some(nonce) = nonce {
            if nonce.len() != self.capabilities.nonce_size {
                return Err(CipherError::InvalidNonceSize(nonce.len(), self.capabilities.nonce_size));
            }
        }
        
        Ok(())
    }
    
    /// slay Validate decryption parameters
    pub fn validate_decrypt_params(&self, key: &[u8], ciphertext: &[u8]) -> CipherResult<()> {
        // Validate key size
        if key.len() != self.capabilities.key_size {
            return Err(CipherError::InvalidKeySize(key.len(), self.capabilities.key_size));
        }
        
        // For authenticated ciphers, ciphertext must be at least nonce + tag size
        if self.capabilities.authenticated {
            let min_size = self.capabilities.nonce_size + self.capabilities.tag_size;
            if ciphertext.len() < min_size {
                return Err(CipherError::InvalidCiphertext);
            }
        }
        
        Ok(())
    }
    
    /// slay Extract nonce from ciphertext (assuming nonce is prepended)
    pub fn extract_nonce(&self, ciphertext: &[u8]) -> CipherResult<&[u8]> {
        if ciphertext.len() < self.capabilities.nonce_size {
            return Err(CipherError::InvalidCiphertext);
        }
        Ok(&ciphertext[..self.capabilities.nonce_size])
    }
    
    /// slay Extract encrypted data from ciphertext (nonce + data + tag)
    pub fn extract_encrypted_data(&self, ciphertext: &[u8]) -> CipherResult<&[u8]> {
        let nonce_size = self.capabilities.nonce_size;
        let tag_size = if self.capabilities.authenticated { self.capabilities.tag_size } else { 0 };
        
        if ciphertext.len() < nonce_size + tag_size {
            return Err(CipherError::InvalidCiphertext);
        }
        
        let end = ciphertext.len() - tag_size;
        Ok(&ciphertext[nonce_size..end])
    }
    
    /// slay Extract authentication tag from ciphertext (assuming tag is appended)
    pub fn extract_tag(&self, ciphertext: &[u8]) -> CipherResult<&[u8]> {
        if !self.capabilities.authenticated {
            return Err(CipherError::Internal("Cipher does not support authentication".to_string()));
        }
        
        if ciphertext.len() < self.capabilities.tag_size {
            return Err(CipherError::InvalidCiphertext);
        }
        
        let start = ciphertext.len() - self.capabilities.tag_size;
        Ok(&ciphertext[start..])
    }
    
    /// slay Combine nonce, encrypted data, and tag into final ciphertext
    pub fn combine_ciphertext(&self, nonce: &[u8], encrypted_data: &[u8], tag: Option<&[u8]>) -> Vec<u8> {
        let mut result = Vec::with_capacity(
            nonce.len() + encrypted_data.len() + tag.map_or(0, |t| t.len())
        );
        
        result.extend_from_slice(nonce);
        result.extend_from_slice(encrypted_data);
        
        if let Some(tag) = tag {
            result.extend_from_slice(tag);
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cipher_type() {
        assert_ne!(CipherType::AesGcm256, CipherType::ChaCha20Poly1305);
        assert_eq!(CipherType::AesGcm256, CipherType::AesGcm256);
    }
    
    #[test]
    fn test_cipher_capabilities() {
        let caps = CipherCapabilities {
            key_size: 32,
            nonce_size: 12,
            tag_size: 16,
            authenticated: true,
            constant_time: true,
            quantum_resistant: false,
        };
        
        assert_eq!(caps.key_size, 32);
        assert!(caps.authenticated);
        assert!(caps.constant_time);
        assert!(!caps.quantum_resistant);
    }
    
    #[test]
    fn test_encryption_context() {
        let context = EncryptionContext::default();
        assert!(context.additional_data.is_empty());
        assert!(context.nonce.is_none());
        assert!(context.key_derivation_salt.is_none());
    }
    
    #[test]
    fn test_decryption_context() {
        let context = DecryptionContext::default();
        assert!(context.additional_data.is_empty());
        assert!(context.expected_tag.is_none());
        assert!(context.verify_integrity);
    }
    
    #[test]
    fn test_cipher_error() {
        let error = CipherError::InvalidKeySize(16, 32);
        assert_eq!(error.to_string(), "Invalid key size: provided 16, expected 32");
        
        let error = CipherError::AuthenticationFailed;
        assert_eq!(error.to_string(), "Authentication tag verification failed");
    }
    
    #[test]
    fn test_cipher_base() {
        let caps = CipherCapabilities {
            key_size: 32,
            nonce_size: 12,
            tag_size: 16,
            authenticated: true,
            constant_time: true,
            quantum_resistant: false,
        };
        
        let base = CipherBase::new(CipherType::AesGcm256, caps);
        assert_eq!(base.cipher_type, CipherType::AesGcm256);
        assert_eq!(base.capabilities.key_size, 32);
        
        // Test validation
        let key = Vec::from([0u8; 32]);
        let nonce = Vec::from([0u8; 12]);
        assert!(base.validate_encrypt_params(&key, Some(&nonce)).is_ok());
        
        let wrong_key = Vec::from([0u8; 16]);
        assert!(base.validate_encrypt_params(&wrong_key, Some(&nonce)).is_err());
    }
    
    #[test]
    fn test_ciphertext_operations() {
        let caps = CipherCapabilities {
            key_size: 32,
            nonce_size: 12,
            tag_size: 16,
            authenticated: true,
            constant_time: true,
            quantum_resistant: false,
        };
        
        let base = CipherBase::new(CipherType::AesGcm256, caps);
        
        let nonce = Vec::from([1u8; 12]);
        let data = Vec::from([2u8; 32]);
        let tag = Vec::from([3u8; 16]);
        
        let combined = base.combine_ciphertext(&nonce, &data, Some(&tag));
        assert_eq!(combined.len(), 12 + 32 + 16);
        
        let extracted_nonce = base.extract_nonce(&combined).unwrap();
        assert_eq!(extracted_nonce, &nonce);
        
        let extracted_data = base.extract_encrypted_data(&combined).unwrap();
        assert_eq!(extracted_data, &data);
        
        let extracted_tag = base.extract_tag(&combined).unwrap();
        assert_eq!(extracted_tag, &tag);
    }
}
