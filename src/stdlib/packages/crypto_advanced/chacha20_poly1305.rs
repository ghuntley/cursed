/// fr fr ChaCha20-Poly1305 authenticated encryption implementation
use super::errors::*;
use super::symmetric_cipher::*;
use super::memory_protection::*;
use super::constant_time::*;
use super::nonce_generator::*;
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    ChaCha20Poly1305 as ChaCha20Poly1305Impl,
    Nonce as ChaChaLib20Nonce,
    Key as ChaChaLib20Key,
};

/// ChaCha20-Poly1305 authenticated encryption cipher
#[derive(Debug)]
pub struct ChaCha20Poly1305 {
    key: SecureMemory,
    cipher: ChaCha20Poly1305Impl,
    nonce_generator: NonceGenerator,
}

/// Encryption result with authentication tag
#[derive(Debug, Clone)]
pub struct EncryptionResult {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub tag: Vec<u8>,
}

/// Decryption result with verification status
#[derive(Debug, Clone)]
pub struct DecryptionResult {
    pub plaintext: Vec<u8>,
    pub verified: bool,
}

impl ChaCha20Poly1305 {
    /// Create new ChaCha20-Poly1305 cipher
    pub fn new(key: &[u8]) -> AdvancedCryptoResult<Self> {
        if key.len() != CHACHA20_KEY_SIZE {
            return Err(AdvancedCryptoError::InvalidKey(
                format!("ChaCha20-Poly1305 requires {}-byte key, got {}", CHACHA20_KEY_SIZE, key.len())
            ));
        }
        
        let secure_key = SecureMemory::new(key.to_vec())?;
        let chacha_key = ChaChaLib20Key::from_slice(key);
        let cipher = ChaCha20Poly1305Impl::new(chacha_key);
        let nonce_generator = NonceGenerator::new()?;
        
        Ok(Self {
            key: secure_key,
            cipher,
            nonce_generator,
        })
    }
    
    /// Generate new ChaCha20-Poly1305 cipher with random key
    pub fn generate() -> AdvancedCryptoResult<Self> {
        let mut key_bytes = vec![0u8; CHACHA20_KEY_SIZE];
        let mut nonce_gen = NonceGenerator::new()?;
        nonce_gen.fill_random(&mut key_bytes)?;
        
        Self::new(&key_bytes)
    }
    
    /// Encrypt plaintext with associated data
    pub fn encrypt_with_aad(&self, plaintext: &[u8], associated_data: &[u8]) -> AdvancedCryptoResult<EncryptionResult> {
        // Generate random nonce
        let nonce_bytes = self.generate_nonce()?;
        let nonce = ChaChaLib20Nonce::from_slice(&nonce_bytes);
        
        // Prepare payload
        let payload = if associated_data.is_empty() {
            Payload::from(plaintext)
        } else {
            Payload {
                msg: plaintext,
                aad: associated_data,
            }
        };
        
        // Perform encryption
        let ciphertext_with_tag = self.cipher.encrypt(nonce, payload)
            .map_err(|e| AdvancedCryptoError::EncryptionFailed(format!("ChaCha20-Poly1305 encryption failed: {:?}", e)))?;
        
        // Split ciphertext and tag (tag is last 16 bytes)
        if ciphertext_with_tag.len() < POLY1305_TAG_SIZE {
            return Err(AdvancedCryptoError::EncryptionFailed("Invalid ciphertext length".to_string()));
        }
        
        let (ciphertext, tag) = ciphertext_with_tag.split_at(ciphertext_with_tag.len() - POLY1305_TAG_SIZE);
        
        Ok(EncryptionResult {
            ciphertext: ciphertext.to_vec(),
            nonce: nonce_bytes,
            tag: tag.to_vec(),
        })
    }
    
    /// Encrypt plaintext without associated data
    pub fn encrypt(&self, plaintext: &[u8]) -> AdvancedCryptoResult<EncryptionResult> {
        self.encrypt_with_aad(plaintext, &[])
    }
    
    /// Decrypt ciphertext with associated data verification
    pub fn decrypt_with_aad(&self, 
                           ciphertext: &[u8], 
                           nonce: &[u8], 
                           tag: &[u8], 
                           associated_data: &[u8]) -> AdvancedCryptoResult<DecryptionResult> {
        
        if nonce.len() != CHACHA20_NONCE_SIZE {
            return Err(AdvancedCryptoError::InvalidNonce(
                format!("Expected {}-byte nonce, got {}", CHACHA20_NONCE_SIZE, nonce.len())
            ));
        }
        
        if tag.len() != POLY1305_TAG_SIZE {
            return Err(AdvancedCryptoError::InvalidTag(
                format!("Expected {}-byte tag, got {}", POLY1305_TAG_SIZE, tag.len())
            ));
        }
        
        let chacha_nonce = ChaChaLib20Nonce::from_slice(nonce);
        
        // Combine ciphertext and tag for ChaCha20-Poly1305 decryption
        let mut ciphertext_with_tag = ciphertext.to_vec();
        ciphertext_with_tag.extend_from_slice(tag);
        
        // Prepare payload
        let payload = if associated_data.is_empty() {
            Payload::from(ciphertext_with_tag.as_slice())
        } else {
            Payload {
                msg: &ciphertext_with_tag,
                aad: associated_data,
            }
        };
        
        // Perform decryption and authentication verification
        match self.cipher.decrypt(chacha_nonce, payload) {
            Ok(plaintext) => Ok(DecryptionResult {
                plaintext,
                verified: true,
            }),
            Err(e) => Err(AdvancedCryptoError::DecryptionFailed(
                format!("ChaCha20-Poly1305 decryption/verification failed: {:?}", e)
            )),
        }
    }
    
    /// Decrypt ciphertext without associated data
    pub fn decrypt(&self, ciphertext: &[u8], nonce: &[u8], tag: &[u8]) -> AdvancedCryptoResult<DecryptionResult> {
        self.decrypt_with_aad(ciphertext, nonce, tag, &[])
    }
    
    /// Generate cryptographically secure nonce
    pub fn generate_nonce(&self) -> AdvancedCryptoResult<Vec<u8>> {
        let mut nonce = vec![0u8; CHACHA20_NONCE_SIZE];
        self.nonce_generator.generate_nonce(&mut nonce)?;
        Ok(nonce)
    }
    
    /// Get key size in bytes
    pub fn key_size(&self) -> usize {
        CHACHA20_KEY_SIZE
    }
    
    /// Get nonce size in bytes
    pub fn nonce_size(&self) -> usize {
        CHACHA20_NONCE_SIZE
    }
    
    /// Get authentication tag size in bytes
    pub fn tag_size(&self) -> usize {
        POLY1305_TAG_SIZE
    }
    
    /// Verify authentication tag in constant time
    pub fn verify_tag(&self, expected: &[u8], actual: &[u8]) -> bool {
        constant_time_compare(expected, actual)
    }
    
    /// Get algorithm name
    pub fn algorithm_name(&self) -> &str {
        "ChaCha20-Poly1305"
    }
    
    /// Check if key is properly initialized
    pub fn is_key_valid(&self) -> bool {
        self.key.len() == CHACHA20_KEY_SIZE
    }
}

impl SymmetricCipher for ChaCha20Poly1305 {
    fn encrypt(&self, plaintext: &[u8]) -> Result<(), Error> {
        let result = Self::encrypt(self, plaintext)
            .map_err(|e| CipherError::OperationFailed(e.to_string()))?;
        
        // Combine nonce + ciphertext + tag for compatibility
        let mut combined = result.nonce;
        combined.extend_from_slice(&result.ciphertext);
        combined.extend_from_slice(&result.tag);
        
        Ok(combined)
    }
    
    fn decrypt(&self, ciphertext: &[u8]) -> Result<(), Error> {
        if ciphertext.len() < CHACHA20_NONCE_SIZE + POLY1305_TAG_SIZE {
            return Err(CipherError::OperationFailed("Ciphertext too short".to_string()));
        }
        
        // Split combined data: nonce + ciphertext + tag
        let nonce = &ciphertext[..CHACHA20_NONCE_SIZE];
        let tag_start = ciphertext.len() - POLY1305_TAG_SIZE;
        let payload = &ciphertext[CHACHA20_NONCE_SIZE..tag_start];
        let tag = &ciphertext[tag_start..];
        
        let result = Self::decrypt(self, payload, nonce, tag)
            .map_err(|e| CipherError::OperationFailed(e.to_string()))?;
        
        Ok(result.plaintext)
    }
    
    fn name(&self) -> &str {
        "ChaCha20-Poly1305"
    }
    
    fn key_size(&self) -> usize {
        CHACHA20_KEY_SIZE
    }
}

/// Secure operations for ChaCha20-Poly1305
impl ChaCha20Poly1305 {
    /// Derive key from password using key stretching
    pub fn from_password(password: &[u8], salt: &[u8], iterations: u32) -> AdvancedCryptoResult<Self> {
        use pbkdf2::pbkdf2_hmac;
        use sha2::Sha256;
        
        if password.is_empty() {
            return Err(AdvancedCryptoError::InvalidKey("Password cannot be empty".to_string()));
        }
        
        if salt.len() < 8 {
            return Err(AdvancedCryptoError::InvalidParameters("Salt must be at least 8 bytes".to_string()));
        }
        
        let mut derived_key = vec![0u8; CHACHA20_KEY_SIZE];
        pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut derived_key);
        
        Self::new(&derived_key)
    }
    
    /// Encrypt data with automatic nonce generation and serialization
    pub fn encrypt_and_serialize(&self, plaintext: &[u8], associated_data: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        let result = self.encrypt_with_aad(plaintext, associated_data)?;
        
        // Serialize as: nonce_size(1) + nonce + tag_size(1) + tag + ciphertext
        let mut serialized = Vec::new();
        serialized.push(result.nonce.len() as u8);
        serialized.extend_from_slice(&result.nonce);
        serialized.push(result.tag.len() as u8);
        serialized.extend_from_slice(&result.tag);
        serialized.extend_from_slice(&result.ciphertext);
        
        Ok(serialized)
    }
    
    /// Decrypt serialized data with automatic parsing
    pub fn decrypt_and_deserialize(&self, serialized: &[u8], associated_data: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        if serialized.len() < 2 {
            return Err(AdvancedCryptoError::DecryptionFailed("Serialized data too short".to_string()));
        }
        
        let mut pos = 0;
        
        // Parse nonce
        let nonce_size = serialized[pos] as usize;
        pos += 1;
        
        if pos + nonce_size >= serialized.len() {
            return Err(AdvancedCryptoError::DecryptionFailed("Invalid nonce size".to_string()));
        }
        
        let nonce = &serialized[pos..pos + nonce_size];
        pos += nonce_size;
        
        // Parse tag
        let tag_size = serialized[pos] as usize;
        pos += 1;
        
        if pos + tag_size >= serialized.len() {
            return Err(AdvancedCryptoError::DecryptionFailed("Invalid tag size".to_string()));
        }
        
        let tag = &serialized[pos..pos + tag_size];
        pos += tag_size;
        
        // Remaining bytes are ciphertext
        let ciphertext = &serialized[pos..];
        
        let result = self.decrypt_with_aad(ciphertext, nonce, tag, associated_data)?;
        Ok(result.plaintext)
    }
}

// Type aliases and constants
pub type ChaCha20Key = SecureMemory;
pub type ChaCha20Nonce = Vec<u8>;
pub type ChaCha20Result<(), Error>;
pub type ChaCha20Error = AdvancedCryptoError;

/// Constants
pub const CHACHA20_KEY_SIZE: usize = 32;
pub const CHACHA20_NONCE_SIZE: usize = 12;
pub const POLY1305_TAG_SIZE: usize = 16;
pub const CHACHA20_BLOCK_SIZE: usize = 64;

/// ChaCha20-Poly1305 utility functions
pub struct ChaCha20Poly1305Util;

impl ChaCha20Poly1305Util {
    /// Validate key size
    pub fn validate_key_size(key_size: usize) -> AdvancedCryptoResult<()> {
        if key_size != CHACHA20_KEY_SIZE {
            return Err(AdvancedCryptoError::InvalidKey(
                format!("Invalid key size: expected {}, got {}", CHACHA20_KEY_SIZE, key_size)
            ));
        }
        Ok(())
    }
    
    /// Validate nonce size
    pub fn validate_nonce_size(nonce_size: usize) -> AdvancedCryptoResult<()> {
        if nonce_size != CHACHA20_NONCE_SIZE {
            return Err(AdvancedCryptoError::InvalidNonce(
                format!("Invalid nonce size: expected {}, got {}", CHACHA20_NONCE_SIZE, nonce_size)
            ));
        }
        Ok(())
    }
    
    /// Validate tag size
    pub fn validate_tag_size(tag_size: usize) -> AdvancedCryptoResult<()> {
        if tag_size != POLY1305_TAG_SIZE {
            return Err(AdvancedCryptoError::InvalidTag(
                format!("Invalid tag size: expected {}, got {}", POLY1305_TAG_SIZE, tag_size)
            ));
        }
        Ok(())
    }
    
    /// Generate random key
    pub fn generate_key() -> AdvancedCryptoResult<Vec<u8>> {
        let mut key = vec![0u8; CHACHA20_KEY_SIZE];
        let mut nonce_gen = NonceGenerator::new()?;
        nonce_gen.fill_random(&mut key)?;
        Ok(key)
    }
    
    /// Generate random nonce
    pub fn generate_nonce() -> AdvancedCryptoResult<Vec<u8>> {
        let mut nonce = vec![0u8; CHACHA20_NONCE_SIZE];
        let mut nonce_gen = NonceGenerator::new()?;
        nonce_gen.fill_random(&mut nonce)?;
        Ok(nonce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chacha20_poly1305_creation() {
        let key = vec![42u8; CHACHA20_KEY_SIZE];
        let cipher = ChaCha20Poly1305::new(&key);
        assert!(cipher.is_ok());
        
        let cipher = cipher.unwrap();
        assert_eq!(cipher.key_size(), CHACHA20_KEY_SIZE);
        assert_eq!(cipher.nonce_size(), CHACHA20_NONCE_SIZE);
        assert_eq!(cipher.tag_size(), POLY1305_TAG_SIZE);
        assert_eq!(cipher.algorithm_name(), "ChaCha20-Poly1305");
        assert!(cipher.is_key_valid());
    }
    
    #[test]
    fn test_invalid_key_size() {
        let short_key = vec![42u8; 16];
        let result = ChaCha20Poly1305::new(&short_key);
        assert!(result.is_err());
        
        let long_key = vec![42u8; 64];
        let result = ChaCha20Poly1305::new(&long_key);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = vec![42u8; CHACHA20_KEY_SIZE];
        let cipher = ChaCha20Poly1305::new(&key).unwrap();
        
        let plaintext = b"Hello, ChaCha20-Poly1305 world!";
        let associated_data = b"additional authenticated data";
        
        // Encrypt
        let encrypted = cipher.encrypt_with_aad(plaintext, associated_data).unwrap();
        assert_eq!(encrypted.nonce.len(), CHACHA20_NONCE_SIZE);
        assert_eq!(encrypted.tag.len(), POLY1305_TAG_SIZE);
        assert_ne!(encrypted.ciphertext, plaintext);
        
        // Decrypt
        let decrypted = cipher.decrypt_with_aad(
            &encrypted.ciphertext,
            &encrypted.nonce,
            &encrypted.tag,
            associated_data
        ).unwrap();
        
        assert!(decrypted.verified);
        assert_eq!(decrypted.plaintext, plaintext);
    }
    
    #[test]
    fn test_encrypt_decrypt_without_aad() {
        let key = vec![42u8; CHACHA20_KEY_SIZE];
        let cipher = ChaCha20Poly1305::new(&key).unwrap();
        
        let plaintext = b"Hello, world!";
        
        let encrypted = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted.ciphertext, &encrypted.nonce, &encrypted.tag).unwrap();
        
        assert_eq!(decrypted.plaintext, plaintext);
        assert!(decrypted.verified);
    }
    
    #[test]
    fn test_authentication_failure() {
        let key = vec![42u8; CHACHA20_KEY_SIZE];
        let cipher = ChaCha20Poly1305::new(&key).unwrap();
        
        let plaintext = b"Hello, world!";
        let encrypted = cipher.encrypt(plaintext).unwrap();
        
        // Tamper with ciphertext
        let mut tampered_ciphertext = encrypted.ciphertext.clone();
        tampered_ciphertext[0] ^= 1;
        
        let result = cipher.decrypt(&tampered_ciphertext, &encrypted.nonce, &encrypted.tag);
        assert!(result.is_err());
        
        // Tamper with tag
        let mut tampered_tag = encrypted.tag.clone();
        tampered_tag[0] ^= 1;
        
        let result = cipher.decrypt(&encrypted.ciphertext, &encrypted.nonce, &tampered_tag);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_symmetric_cipher_interface() {
        let key = vec![42u8; CHACHA20_KEY_SIZE];
        let cipher = ChaCha20Poly1305::new(&key).unwrap();
        
        let plaintext = b"Test symmetric cipher interface";
        
        let encrypted = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();
        
        assert_eq!(decrypted, plaintext);
        assert_eq!(cipher.name(), "ChaCha20-Poly1305");
        assert_eq!(cipher.key_size(), CHACHA20_KEY_SIZE);
    }
    
    #[test]
    fn test_serialize_deserialize() {
        let key = vec![42u8; CHACHA20_KEY_SIZE];
        let cipher = ChaCha20Poly1305::new(&key).unwrap();
        
        let plaintext = b"Serialization test data";
        let aad = b"metadata";
        
        let serialized = cipher.encrypt_and_serialize(plaintext, aad).unwrap();
        let deserialized = cipher.decrypt_and_deserialize(&serialized, aad).unwrap();
        
        assert_eq!(deserialized, plaintext);
    }
    
    #[test]
    fn test_from_password() {
        let password = b"strong_password";
        let salt = b"unique_salt_12345";
        let iterations = 10000;
        
        let cipher = ChaCha20Poly1305::from_password(password, salt, iterations).unwrap();
        assert!(cipher.is_key_valid());
        
        // Same password and salt should produce same cipher
        let cipher2 = ChaCha20Poly1305::from_password(password, salt, iterations).unwrap();
        
        let plaintext = b"Test deterministic key derivation";
        let encrypted1 = cipher.encrypt(plaintext).unwrap();
        let decrypted2 = cipher2.decrypt(&encrypted1.ciphertext, &encrypted1.nonce, &encrypted1.tag).unwrap();
        
        assert_eq!(decrypted2.plaintext, plaintext);
    }
    
    #[test]
    fn test_nonce_generation() {
        let key = vec![42u8; CHACHA20_KEY_SIZE];
        let cipher = ChaCha20Poly1305::new(&key).unwrap();
        
        let nonce1 = cipher.generate_nonce().unwrap();
        let nonce2 = cipher.generate_nonce().unwrap();
        
        assert_eq!(nonce1.len(), CHACHA20_NONCE_SIZE);
        assert_eq!(nonce2.len(), CHACHA20_NONCE_SIZE);
        assert_ne!(nonce1, nonce2); // Should be different
    }
    
    #[test]
    fn test_tag_verification() {
        let key = vec![42u8; CHACHA20_KEY_SIZE];
        let cipher = ChaCha20Poly1305::new(&key).unwrap();
        
        let tag1 = vec![1, 2, 3, 4];
        let tag2 = vec![1, 2, 3, 4];
        let tag3 = vec![1, 2, 3, 5];
        
        assert!(cipher.verify_tag(&tag1, &tag2));
        assert!(!cipher.verify_tag(&tag1, &tag3));
    }
    
    #[test]
    fn test_utilities() {
        assert!(ChaCha20Poly1305Util::validate_key_size(CHACHA20_KEY_SIZE).is_ok());
        assert!(ChaCha20Poly1305Util::validate_key_size(16).is_err());
        
        assert!(ChaCha20Poly1305Util::validate_nonce_size(CHACHA20_NONCE_SIZE).is_ok());
        assert!(ChaCha20Poly1305Util::validate_nonce_size(8).is_err());
        
        assert!(ChaCha20Poly1305Util::validate_tag_size(POLY1305_TAG_SIZE).is_ok());
        assert!(ChaCha20Poly1305Util::validate_tag_size(8).is_err());
        
        let key = ChaCha20Poly1305Util::generate_key().unwrap();
        assert_eq!(key.len(), CHACHA20_KEY_SIZE);
        
        let nonce = ChaCha20Poly1305Util::generate_nonce().unwrap();
        assert_eq!(nonce.len(), CHACHA20_NONCE_SIZE);
    }
}
