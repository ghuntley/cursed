/// fr fr Authenticated Encryption with Associated Data (AEAD) implementations
use super::errors::*;
use super::memory_protection::*;
use super::constant_time::*;
use super::nonce_generator::*;

/// Authenticated Encryption trait for AEAD ciphers
pub trait AuthenticatedEncryption {
    /// Encrypt plaintext with authentication
    fn encrypt_with_auth(&self, plaintext: &[u8], associated_data: &[u8]) -> AdvancedCryptoResult<AeadResult>;
    
    /// Decrypt and verify ciphertext
    fn decrypt_with_auth(&self, ciphertext: &[u8], nonce: &[u8], tag: &[u8], associated_data: &[u8]) -> AdvancedCryptoResult<Vec<u8>>;
    
    /// Get algorithm name
    fn algorithm_name(&self) -> &str;
    
    /// Get key size in bytes
    fn key_size(&self) -> usize;
    
    /// Get nonce size in bytes  
    fn nonce_size(&self) -> usize;
    
    /// Get authentication tag size in bytes
    fn tag_size(&self) -> usize;
    
    /// Verify authentication tag
    fn verify_tag(&self, expected: &[u8], actual: &[u8]) -> bool {
        constant_time_compare(expected, actual)
    }
}

/// AEAD encryption result
#[derive(Debug, Clone)]
pub struct AeadResult {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub tag: AuthenticationTag,
    pub algorithm: String,
}

/// Authentication tag with metadata
#[derive(Debug, Clone, PartialEq)]
pub struct AuthenticationTag {
    data: Vec<u8>,
    algorithm: String,
    key_id: Option<String>,
}

impl AuthenticationTag {
    /// Create new authentication tag
    pub fn new(data: Vec<u8>, algorithm: String) -> Self {
        Self {
            data,
            algorithm,
            key_id: None,
        }
    }
    
    /// Create tag with key identifier
    pub fn with_key_id(data: Vec<u8>, algorithm: String, key_id: String) -> Self {
        Self {
            data,
            algorithm,
            key_id: Some(key_id),
        }
    }
    
    /// Get tag data
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
    
    /// Get tag data as vec
    pub fn to_vec(&self) -> Vec<u8> {
        self.data.clone()
    }
    
    /// Get tag length
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if tag is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Get algorithm name
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }
    
    /// Get key ID if present
    pub fn key_id(&self) -> Option<&str> {
        self.key_id.as_deref()
    }
    
    /// Verify tag against another in constant time
    pub fn verify(&self, other: &AuthenticationTag) -> bool {
        if self.algorithm != other.algorithm || self.data.len() != other.data.len() {
            return false;
        }
        constant_time_compare(&self.data, &other.data)
    }
    
    /// Verify tag against raw bytes in constant time
    pub fn verify_bytes(&self, other: &[u8]) -> bool {
        constant_time_compare(&self.data, other)
    }
}

/// Generic AEAD cipher wrapper
#[derive(Debug)]
pub struct AeadCipher {
    algorithm: String,
    key: SecureMemory,
    nonce_generator: NonceGenerator,
}

impl AeadCipher {
    /// Create new AEAD cipher
    pub fn new(algorithm: String, key: Vec<u8>) -> AdvancedCryptoResult<Self> {
        let key_size = Self::get_key_size_for_algorithm(&algorithm)?;
        if key.len() != key_size {
            return Err(AdvancedCryptoError::InvalidKey(
                format!("Invalid key size for {}: expected {}, got {}", algorithm, key_size, key.len())
            ));
        }
        
        Ok(Self {
            algorithm,
            key: SecureMemory::new(key)?,
            nonce_generator: NonceGenerator::new()?,
        })
    }
    
    /// Get key size for algorithm
    fn get_key_size_for_algorithm(algorithm: &str) -> AdvancedCryptoResult<usize> {
        match algorithm {
            "AES-256-GCM" => Ok(32),
            "ChaCha20-Poly1305" => Ok(32),
            "XChaCha20-Poly1305" => Ok(32),
            _ => Err(AdvancedCryptoError::UnsupportedAlgorithm(algorithm.to_string())),
        }
    }
    
    /// Get nonce size for algorithm
    fn get_nonce_size_for_algorithm(algorithm: &str) -> AdvancedCryptoResult<usize> {
        match algorithm {
            "AES-256-GCM" | "ChaCha20-Poly1305" => Ok(12),
            "XChaCha20-Poly1305" => Ok(24),
            _ => Err(AdvancedCryptoError::UnsupportedAlgorithm(algorithm.to_string())),
        }
    }
    
    /// Get tag size for algorithm
    fn get_tag_size_for_algorithm(algorithm: &str) -> AdvancedCryptoResult<usize> {
        match algorithm {
            "AES-256-GCM" | "ChaCha20-Poly1305" | "XChaCha20-Poly1305" => Ok(16),
            _ => Err(AdvancedCryptoError::UnsupportedAlgorithm(algorithm.to_string())),
        }
    }
}

impl AuthenticatedEncryption for AeadCipher {
    fn encrypt_with_auth(&self, plaintext: &[u8], associated_data: &[u8]) -> AdvancedCryptoResult<AeadResult> {
        let nonce_size = Self::get_nonce_size_for_algorithm(&self.algorithm)?;
        let nonce = self.nonce_generator.generate_nonce(nonce_size)?;
        
        // For this implementation, we'll use the ChaCha20-Poly1305 as the primary cipher
        // In a full implementation, this would dispatch to the appropriate algorithm
        match self.algorithm.as_str() {
            "ChaCha20-Poly1305" => {
                use super::chacha20_poly1305::ChaCha20Poly1305;
                let cipher = ChaCha20Poly1305::new(self.key.as_bytes())?;
                let result = cipher.encrypt_with_aad(plaintext, associated_data)?;
                
                Ok(AeadResult {
                    ciphertext: result.ciphertext,
                    nonce: result.nonce,
                    tag: AuthenticationTag::new(result.tag, self.algorithm.clone()),
                    algorithm: self.algorithm.clone(),
                })
            },
            _ => {
                // Placeholder for other algorithms
                Err(AdvancedCryptoError::UnsupportedAlgorithm(
                    format!("Algorithm {} not yet implemented", self.algorithm)
                ))
            }
        }
    }
    
    fn decrypt_with_auth(&self, ciphertext: &[u8], nonce: &[u8], tag: &[u8], associated_data: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        match self.algorithm.as_str() {
            "ChaCha20-Poly1305" => {
                use super::chacha20_poly1305::ChaCha20Poly1305;
                let cipher = ChaCha20Poly1305::new(self.key.as_bytes())?;
                let result = cipher.decrypt_with_aad(ciphertext, nonce, tag, associated_data)?;
                Ok(result.plaintext)
            },
            _ => {
                Err(AdvancedCryptoError::UnsupportedAlgorithm(
                    format!("Algorithm {} not yet implemented", self.algorithm)
                ))
            }
        }
    }
    
    fn algorithm_name(&self) -> &str {
        &self.algorithm
    }
    
    fn key_size(&self) -> usize {
        self.key.len()
    }
    
    fn nonce_size(&self) -> usize {
        Self::get_nonce_size_for_algorithm(&self.algorithm).unwrap_or(0)
    }
    
    fn tag_size(&self) -> usize {
        Self::get_tag_size_for_algorithm(&self.algorithm).unwrap_or(0)
    }
}

/// AEAD cipher factory
pub struct AeadCipherFactory;

impl AeadCipherFactory {
    /// Create AEAD cipher for specific algorithm
    pub fn create_cipher(algorithm: &str, key: &[u8]) -> AdvancedCryptoResult<Box<dyn AuthenticatedEncryption + Send + Sync>> {
        match algorithm {
            "ChaCha20-Poly1305" => {
                let cipher = AeadCipher::new(algorithm.to_string(), key.to_vec())?;
                Ok(Box::new(cipher))
            },
            "AES-256-GCM" => {
                // Would implement AES-GCM here
                Err(AdvancedCryptoError::UnsupportedAlgorithm("AES-256-GCM not yet implemented".to_string()))
            },
            "XChaCha20-Poly1305" => {
                // Would implement XChaCha20-Poly1305 here
                Err(AdvancedCryptoError::UnsupportedAlgorithm("XChaCha20-Poly1305 not yet implemented".to_string()))
            },
            _ => Err(AdvancedCryptoError::UnsupportedAlgorithm(algorithm.to_string())),
        }
    }
    
    /// Get list of supported algorithms
    pub fn supported_algorithms() -> Vec<&'static str> {
        vec![
            "ChaCha20-Poly1305",
            // Future: "AES-256-GCM", "XChaCha20-Poly1305"
        ]
    }
    
    /// Get recommended algorithm for specific use case
    pub fn recommended_algorithm(use_case: &str) -> &'static str {
        match use_case {
            "general" | "high_performance" => "ChaCha20-Poly1305",
            "government" | "fips" => "AES-256-GCM", // Future
            "long_nonce" => "XChaCha20-Poly1305", // Future
            _ => "ChaCha20-Poly1305",
        }
    }
}

/// AEAD utilities for common operations
pub struct AeadUtils;

impl AeadUtils {
    /// Encrypt multiple messages with the same key but different nonces
    pub fn encrypt_batch(
        cipher: &dyn AuthenticatedEncryption,
        messages: &[(&[u8], &[u8])], // (plaintext, associated_data)
    ) -> AdvancedCryptoResult<Vec<AeadResult>> {
        let mut results = Vec::new();
        
        for (plaintext, associated_data) in messages {
            let result = cipher.encrypt_with_auth(plaintext, associated_data)?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// Decrypt multiple messages
    pub fn decrypt_batch(
        cipher: &dyn AuthenticatedEncryption,
        messages: &[(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)], // (ciphertext, nonce, tag, aad)
    ) -> AdvancedCryptoResult<Vec<Vec<u8>>> {
        let mut results = Vec::new();
        
        for (ciphertext, nonce, tag, aad) in messages {
            let plaintext = cipher.decrypt_with_auth(ciphertext, nonce, tag, aad)?;
            results.push(plaintext);
        }
        
        Ok(results)
    }
    
    /// Validate AEAD parameters
    pub fn validate_parameters(
        algorithm: &str,
        key_size: usize,
        nonce_size: usize,
        tag_size: usize,
    ) -> AdvancedCryptoResult<()> {
        let expected_key = AeadCipher::get_key_size_for_algorithm(algorithm)?;
        let expected_nonce = AeadCipher::get_nonce_size_for_algorithm(algorithm)?;
        let expected_tag = AeadCipher::get_tag_size_for_algorithm(algorithm)?;
        
        if key_size != expected_key {
            return Err(AdvancedCryptoError::InvalidKey(
                format!("Invalid key size for {}: expected {}, got {}", algorithm, expected_key, key_size)
            ));
        }
        
        if nonce_size != expected_nonce {
            return Err(AdvancedCryptoError::InvalidNonce(
                format!("Invalid nonce size for {}: expected {}, got {}", algorithm, expected_nonce, nonce_size)
            ));
        }
        
        if tag_size != expected_tag {
            return Err(AdvancedCryptoError::InvalidTag(
                format!("Invalid tag size for {}: expected {}, got {}", algorithm, expected_tag, tag_size)
            ));
        }
        
        Ok(())
    }
}

// Type aliases for compatibility
pub type EncryptionResult<T> = Result<T, AdvancedCryptoError>;
pub type DecryptionResult<T> = Result<T, AdvancedCryptoError>;
pub type AuthenticationError = AdvancedCryptoError;
pub type TagMismatchError = AdvancedCryptoError;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_authentication_tag() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let tag1 = AuthenticationTag::new(data.clone(), "ChaCha20-Poly1305".to_string());
        let tag2 = AuthenticationTag::new(data.clone(), "ChaCha20-Poly1305".to_string());
        let tag3 = AuthenticationTag::new(vec![1, 2, 3, 4], "ChaCha20-Poly1305".to_string());
        
        assert_eq!(tag1.len(), 16);
        assert!(!tag1.is_empty());
        assert_eq!(tag1.algorithm(), "ChaCha20-Poly1305");
        assert!(tag1.verify(&tag2));
        assert!(!tag1.verify(&tag3));
        assert!(tag1.verify_bytes(&data));
    }
    
    #[test]
    fn test_authentication_tag_with_key_id() {
        let data = vec![1, 2, 3, 4];
        let tag = AuthenticationTag::with_key_id(data, "ChaCha20-Poly1305".to_string(), "key123".to_string());
        
        assert_eq!(tag.key_id(), Some("key123"));
        assert_eq!(tag.algorithm(), "ChaCha20-Poly1305");
    }
    
    #[test]
    fn test_aead_cipher_creation() {
        let key = vec![42u8; 32];
        let cipher = AeadCipher::new("ChaCha20-Poly1305".to_string(), key);
        assert!(cipher.is_ok());
        
        let cipher = cipher.unwrap();
        assert_eq!(cipher.algorithm_name(), "ChaCha20-Poly1305");
        assert_eq!(cipher.key_size(), 32);
        assert_eq!(cipher.nonce_size(), 12);
        assert_eq!(cipher.tag_size(), 16);
    }
    
    #[test]
    fn test_aead_cipher_invalid_key() {
        let short_key = vec![42u8; 16];
        let result = AeadCipher::new("ChaCha20-Poly1305".to_string(), short_key);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_aead_encrypt_decrypt() {
        let key = vec![42u8; 32];
        let cipher = AeadCipher::new("ChaCha20-Poly1305".to_string(), key).unwrap();
        
        let plaintext = b"Hello, AEAD world!";
        let associated_data = b"metadata";
        
        // Encrypt
        let result = cipher.encrypt_with_auth(plaintext, associated_data).unwrap();
        assert_eq!(result.algorithm, "ChaCha20-Poly1305");
        assert_eq!(result.nonce.len(), 12);
        assert_eq!(result.tag.len(), 16);
        assert_ne!(result.ciphertext, plaintext);
        
        // Decrypt
        let decrypted = cipher.decrypt_with_auth(
            &result.ciphertext,
            &result.nonce,
            result.tag.as_bytes(),
            associated_data
        ).unwrap();
        
        assert_eq!(decrypted, plaintext);
    }
    
    #[test]
    fn test_aead_cipher_factory() {
        let key = vec![42u8; 32];
        
        let cipher = AeadCipherFactory::create_cipher("ChaCha20-Poly1305", &key);
        assert!(cipher.is_ok());
        
        let cipher = cipher.unwrap();
        assert_eq!(cipher.algorithm_name(), "ChaCha20-Poly1305");
        
        let supported = AeadCipherFactory::supported_algorithms();
        assert!(supported.contains(&"ChaCha20-Poly1305"));
        
        let recommended = AeadCipherFactory::recommended_algorithm("general");
        assert_eq!(recommended, "ChaCha20-Poly1305");
    }
    
    #[test]
    fn test_aead_utils() {
        let key = vec![42u8; 32];
        let cipher = AeadCipher::new("ChaCha20-Poly1305".to_string(), key).unwrap();
        
        // Test parameter validation
        let validation = AeadUtils::validate_parameters("ChaCha20-Poly1305", 32, 12, 16);
        assert!(validation.is_ok());
        
        let invalid_validation = AeadUtils::validate_parameters("ChaCha20-Poly1305", 16, 12, 16);
        assert!(invalid_validation.is_err());
        
        // Test batch encryption
        let messages = vec![
            (b"message1".as_ref(), b"aad1".as_ref()),
            (b"message2".as_ref(), b"aad2".as_ref()),
        ];
        
        let encrypted_batch = AeadUtils::encrypt_batch(&cipher, &messages).unwrap();
        assert_eq!(encrypted_batch.len(), 2);
        
        // Test batch decryption
        let decrypt_messages = vec![
            (encrypted_batch[0].ciphertext.clone(), encrypted_batch[0].nonce.clone(), 
             encrypted_batch[0].tag.to_vec(), b"aad1".to_vec()),
            (encrypted_batch[1].ciphertext.clone(), encrypted_batch[1].nonce.clone(), 
             encrypted_batch[1].tag.to_vec(), b"aad2".to_vec()),
        ];
        
        let decrypted_batch = AeadUtils::decrypt_batch(&cipher, &decrypt_messages).unwrap();
        assert_eq!(decrypted_batch.len(), 2);
        assert_eq!(decrypted_batch[0], b"message1");
        assert_eq!(decrypted_batch[1], b"message2");
    }
    
    #[test]
    fn test_unsupported_algorithm() {
        let key = vec![42u8; 32];
        let result = AeadCipher::new("UnsupportedAlgorithm".to_string(), key);
        assert!(result.is_err());
        
        let factory_result = AeadCipherFactory::create_cipher("UnsupportedAlgorithm", &[42u8; 32]);
        assert!(factory_result.is_err());
    }
}
