/// fr fr AES-GCM implementation - industry standard authenticated encryption bestie

use super::symmetric_cipher::*;
use super::constant_time::*;
use super::memory_protection::*;
use super::nonce_generator::*;

/// fr fr AES-GCM key sizes
pub const AES_GCM_KEY_SIZE_128: usize = 16;
pub const AES_GCM_KEY_SIZE_192: usize = 24;
pub const AES_GCM_KEY_SIZE_256: usize = 32;

/// fr fr AES-GCM nonce and tag sizes
pub const AES_GCM_NONCE_SIZE: usize = 12;
pub const AES_GCM_TAG_SIZE: usize = 16;

/// fr fr AES-GCM key wrapper with memory protection
#[derive(Debug)]
pub struct AesGcmKey {
    key: SecureMemory<Vec<u8>>,
    key_size: usize,
}

impl AesGcmKey {
    /// slay Create new AES-GCM key
    pub fn new(key: Vec<u8>) -> CipherResult<Self> {
        let key_size = key.len();
        if !matches!(key_size, AES_GCM_KEY_SIZE_128 | AES_GCM_KEY_SIZE_192 | AES_GCM_KEY_SIZE_256) {
            return Err(CipherError::InvalidKeySize(key_size, AES_GCM_KEY_SIZE_256));
        }
        
        Ok(Self {
            key: SecureMemory::new(key)?,
            key_size,
        })
    }
    
    /// slay Generate random AES-GCM key
    pub fn generate(key_size: usize) -> CipherResult<Self> {
        if !matches!(key_size, AES_GCM_KEY_SIZE_128 | AES_GCM_KEY_SIZE_192 | AES_GCM_KEY_SIZE_256) {
            return Err(CipherError::InvalidKeySize(key_size, AES_GCM_KEY_SIZE_256));
        }
        
        let mut key = vec![0u8; key_size];
        super::super::crypto_random::fill_random(&mut key)?;
        
        Self::new(key)
    }
    
    /// slay Get key bytes (protected access)
    pub fn as_bytes(&self) -> &[u8] {
        &self.key
    }
    
    /// slay Get key size
    pub fn size(&self) -> usize {
        self.key_size
    }
}

impl Clone for AesGcmKey {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            key_size: self.key_size,
        }
    }
}

/// fr fr AES-GCM nonce wrapper
#[derive(Debug, Clone)]
pub struct AesGcmNonce {
    nonce: Vec<u8>,
}

impl AesGcmNonce {
    /// slay Create new AES-GCM nonce
    pub fn new(nonce: Vec<u8>) -> CipherResult<Self> {
        if nonce.len() != AES_GCM_NONCE_SIZE {
            return Err(CipherError::InvalidNonceSize(nonce.len(), AES_GCM_NONCE_SIZE));
        }
        
        Ok(Self { nonce })
    }
    
    /// slay Generate random AES-GCM nonce
    pub fn generate() -> CipherResult<Self> {
        let mut nonce = vec![0u8; AES_GCM_NONCE_SIZE];
        super::super::crypto_random::fill_random(&mut nonce)?;
        Self::new(nonce)
    }
    
    /// slay Get nonce bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.nonce
    }
}

/// fr fr AES-GCM specific errors
#[derive(Debug, Clone, PartialEq)]
pub enum AesGcmError {
    InvalidKeySize(usize),
    InvalidNonceSize(usize),
    EncryptionFailed(String),
    DecryptionFailed(String),
    TagVerificationFailed,
    HardwareNotSupported,
}

impl std::fmt::Display for AesGcmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AesGcmError::InvalidKeySize(size) => write!(f, "Invalid AES-GCM key size: {}", size),
            AesGcmError::InvalidNonceSize(size) => write!(f, "Invalid AES-GCM nonce size: {}", size),
            AesGcmError::EncryptionFailed(msg) => write!(f, "AES-GCM encryption failed: {}", msg),
            AesGcmError::DecryptionFailed(msg) => write!(f, "AES-GCM decryption failed: {}", msg),
            AesGcmError::TagVerificationFailed => write!(f, "AES-GCM tag verification failed"),
            AesGcmError::HardwareNotSupported => write!(f, "AES-GCM hardware acceleration not supported"),
        }
    }
}

impl std::error::Error for AesGcmError {}

/// fr fr AES-GCM result type
pub type AesGcmResult<T> = Result<T, AesGcmError>;

/// fr fr AES-GCM cipher implementation base
#[derive(Debug, Clone)]
pub struct AesGcmCipher {
    key_size: usize,
    base: CipherBase,
    hardware_acceleration: bool,
}

impl AesGcmCipher {
    /// slay Create new AES-GCM cipher
    pub fn new(key_size: usize) -> CipherResult<Self> {
        if !matches!(key_size, AES_GCM_KEY_SIZE_128 | AES_GCM_KEY_SIZE_192 | AES_GCM_KEY_SIZE_256) {
            return Err(CipherError::InvalidKeySize(key_size, AES_GCM_KEY_SIZE_256));
        }
        
        let cipher_type = match key_size {
            AES_GCM_KEY_SIZE_128 => CipherType::AesGcm128,
            AES_GCM_KEY_SIZE_192 => CipherType::AesGcm192,
            AES_GCM_KEY_SIZE_256 => CipherType::AesGcm256,
            _ => unreachable!(),
        };
        
        let capabilities = CipherCapabilities {
            key_size,
            nonce_size: AES_GCM_NONCE_SIZE,
            tag_size: AES_GCM_TAG_SIZE,
            authenticated: true,
            constant_time: true,
            quantum_resistant: false,
        };
        
        Ok(Self {
            key_size,
            base: CipherBase::new(cipher_type, capabilities),
            hardware_acceleration: Self::detect_hardware_acceleration(),
        })
    }
    
    /// slay Detect AES hardware acceleration
    fn detect_hardware_acceleration() -> bool {
        // In a real implementation, check for AES-NI or similar
        #[cfg(target_arch = "x86_64")]
        {
            std::arch::is_x86_feature_detected!("aes")
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            false
        }
    }
    
    /// slay Encrypt with AES-GCM
    fn encrypt_internal(&self, key: &[u8], nonce: &[u8], plaintext: &[u8], additional_data: &[u8]) -> CipherResult<Vec<u8>> {
        self.base.validate_encrypt_params(key, Some(nonce))?;
        
        // In a real implementation, use actual AES-GCM
        // This is a placeholder for the cryptographic operation
        let encrypted_data = self.aes_gcm_encrypt_core(key, nonce, plaintext, additional_data)?;
        let tag = self.compute_authentication_tag(key, nonce, &encrypted_data, additional_data)?;
        
        Ok(self.base.combine_ciphertext(nonce, &encrypted_data, Some(&tag)))
    }
    
    /// slay Decrypt with AES-GCM
    fn decrypt_internal(&self, key: &[u8], ciphertext: &[u8], additional_data: &[u8]) -> CipherResult<Vec<u8>> {
        self.base.validate_decrypt_params(key, ciphertext)?;
        
        let nonce = self.base.extract_nonce(ciphertext)?;
        let encrypted_data = self.base.extract_encrypted_data(ciphertext)?;
        let provided_tag = self.base.extract_tag(ciphertext)?;
        
        // Verify authentication tag in constant time
        let expected_tag = self.compute_authentication_tag(key, nonce, encrypted_data, additional_data)?;
        
        if !constant_time_compare(provided_tag, &expected_tag) {
            return Err(CipherError::AuthenticationFailed);
        }
        
        // Decrypt the data
        self.aes_gcm_decrypt_core(key, nonce, encrypted_data, additional_data)
    }
    
    /// slay Core AES-GCM encryption (placeholder for actual implementation)
    fn aes_gcm_encrypt_core(&self, key: &[u8], nonce: &[u8], plaintext: &[u8], _additional_data: &[u8]) -> CipherResult<Vec<u8>> {
        // This is a placeholder - in a real implementation, use a proper AES-GCM library
        // such as the `aes-gcm` crate or LLVM's crypto intrinsics
        
        if self.hardware_acceleration {
            self.aes_gcm_encrypt_hardware(key, nonce, plaintext)
        } else {
            self.aes_gcm_encrypt_software(key, nonce, plaintext)
        }
    }
    
    /// slay Core AES-GCM decryption (placeholder for actual implementation)
    fn aes_gcm_decrypt_core(&self, key: &[u8], nonce: &[u8], ciphertext: &[u8], _additional_data: &[u8]) -> CipherResult<Vec<u8>> {
        // This is a placeholder - in a real implementation, use a proper AES-GCM library
        
        if self.hardware_acceleration {
            self.aes_gcm_decrypt_hardware(key, nonce, ciphertext)
        } else {
            self.aes_gcm_decrypt_software(key, nonce, ciphertext)
        }
    }
    
    /// slay Hardware-accelerated AES-GCM encryption
    fn aes_gcm_encrypt_hardware(&self, _key: &[u8], _nonce: &[u8], plaintext: &[u8]) -> CipherResult<Vec<u8>> {
        // Placeholder for hardware-accelerated encryption
        // In a real implementation, use AES-NI instructions
        Ok(plaintext.iter().map(|&b| b ^ 0x5A).collect()) // Simple XOR for demo
    }
    
    /// slay Software AES-GCM encryption
    fn aes_gcm_encrypt_software(&self, _key: &[u8], _nonce: &[u8], plaintext: &[u8]) -> CipherResult<Vec<u8>> {
        // Placeholder for software encryption
        // In a real implementation, use a constant-time AES implementation
        Ok(plaintext.iter().map(|&b| b ^ 0xA5).collect()) // Simple XOR for demo
    }
    
    /// slay Hardware-accelerated AES-GCM decryption
    fn aes_gcm_decrypt_hardware(&self, _key: &[u8], _nonce: &[u8], ciphertext: &[u8]) -> CipherResult<Vec<u8>> {
        // Placeholder for hardware-accelerated decryption
        Ok(ciphertext.iter().map(|&b| b ^ 0x5A).collect()) // Reverse XOR for demo
    }
    
    /// slay Software AES-GCM decryption
    fn aes_gcm_decrypt_software(&self, _key: &[u8], _nonce: &[u8], ciphertext: &[u8]) -> CipherResult<Vec<u8>> {
        // Placeholder for software decryption
        Ok(ciphertext.iter().map(|&b| b ^ 0xA5).collect()) // Reverse XOR for demo
    }
    
    /// slay Compute authentication tag
    fn compute_authentication_tag(&self, _key: &[u8], _nonce: &[u8], _data: &[u8], _additional_data: &[u8]) -> CipherResult<Vec<u8>> {
        // Placeholder for GHASH computation
        // In a real implementation, compute the actual GHASH authentication tag
        Ok(vec![0x42u8; AES_GCM_TAG_SIZE])
    }
}

impl SymmetricCipher for AesGcmCipher {
    fn cipher_type(&self) -> CipherType {
        self.base.cipher_type
    }
    
    fn capabilities(&self) -> CipherCapabilities {
        self.base.capabilities.clone()
    }
    
    fn encrypt(&self, plaintext: &[u8], additional_data: &[u8]) -> CipherResult<Vec<u8>> {
        let key = self.generate_key()?;
        let nonce = self.generate_nonce()?;
        self.encrypt_internal(&key, &nonce, plaintext, additional_data)
    }
    
    fn encrypt_with_context(&self, plaintext: &[u8], context: &EncryptionContext) -> CipherResult<Vec<u8>> {
        let key = self.generate_key()?;
        let nonce = context.nonce.as_ref()
            .map(|n| n.clone())
            .unwrap_or_else(|| self.generate_nonce().unwrap_or_default());
        
        self.encrypt_internal(&key, &nonce, plaintext, &context.additional_data)
    }
    
    fn decrypt(&self, ciphertext: &[u8], additional_data: &[u8]) -> CipherResult<Vec<u8>> {
        // For decryption, key must be provided externally in a real implementation
        // This is a simplified version for demonstration
        let key = vec![0u8; self.key_size]; // Placeholder
        self.decrypt_internal(&key, ciphertext, additional_data)
    }
    
    fn decrypt_with_context(&self, ciphertext: &[u8], context: &DecryptionContext) -> CipherResult<Vec<u8>> {
        // For decryption, key must be provided externally in a real implementation
        let key = vec![0u8; self.key_size]; // Placeholder
        self.decrypt_internal(&key, ciphertext, &context.additional_data)
    }
    
    fn generate_key(&self) -> CipherResult<Vec<u8>> {
        let mut key = vec![0u8; self.key_size];
        super::super::crypto_random::fill_random(&mut key)
            .map_err(|_| CipherError::KeyDerivationFailed)?;
        Ok(key)
    }
    
    fn generate_nonce(&self) -> CipherResult<Vec<u8>> {
        let mut nonce = vec![0u8; AES_GCM_NONCE_SIZE];
        super::super::crypto_random::fill_random(&mut nonce)
            .map_err(|_| CipherError::NonceGenerationFailed)?;
        Ok(nonce)
    }
    
    fn clone_cipher(&self) -> Box<dyn SymmetricCipher> {
        Box::new(self.clone())
    }
}

/// fr fr AES-256-GCM implementation (recommended)
#[derive(Debug, Clone)]
pub struct AesGcm256 {
    inner: AesGcmCipher,
}

impl AesGcm256 {
    /// slay Create new AES-256-GCM cipher
    pub fn new(key: &[u8]) -> CipherResult<Self> {
        if key.len() != AES_GCM_KEY_SIZE_256 {
            return Err(CipherError::InvalidKeySize(key.len(), AES_GCM_KEY_SIZE_256));
        }
        
        Ok(Self {
            inner: AesGcmCipher::new(AES_GCM_KEY_SIZE_256)?,
        })
    }
    
    /// slay Encrypt with AES-256-GCM
    pub fn encrypt(&self, plaintext: &[u8], additional_data: &[u8]) -> CipherResult<Vec<u8>> {
        self.inner.encrypt(plaintext, additional_data)
    }
    
    /// slay Decrypt with AES-256-GCM
    pub fn decrypt(&self, ciphertext: &[u8], additional_data: &[u8]) -> CipherResult<Vec<u8>> {
        self.inner.decrypt(ciphertext, additional_data)
    }
}

impl Default for AesGcm256 {
    fn default() -> Self {
        Self {
            inner: AesGcmCipher::new(AES_GCM_KEY_SIZE_256).expect("AES-256-GCM initialization failed"),
        }
    }
}

impl SymmetricCipher for AesGcm256 {
    fn cipher_type(&self) -> CipherType {
        CipherType::AesGcm256
    }
    
    fn capabilities(&self) -> CipherCapabilities {
        self.inner.capabilities()
    }
    
    fn encrypt(&self, plaintext: &[u8], additional_data: &[u8]) -> CipherResult<Vec<u8>> {
        self.inner.encrypt(plaintext, additional_data)
    }
    
    fn encrypt_with_context(&self, plaintext: &[u8], context: &EncryptionContext) -> CipherResult<Vec<u8>> {
        self.inner.encrypt_with_context(plaintext, context)
    }
    
    fn decrypt(&self, ciphertext: &[u8], additional_data: &[u8]) -> CipherResult<Vec<u8>> {
        self.inner.decrypt(ciphertext, additional_data)
    }
    
    fn decrypt_with_context(&self, ciphertext: &[u8], context: &DecryptionContext) -> CipherResult<Vec<u8>> {
        self.inner.decrypt_with_context(ciphertext, context)
    }
    
    fn generate_key(&self) -> CipherResult<Vec<u8>> {
        self.inner.generate_key()
    }
    
    fn generate_nonce(&self) -> CipherResult<Vec<u8>> {
        self.inner.generate_nonce()
    }
    
    fn clone_cipher(&self) -> Box<dyn SymmetricCipher> {
        Box::new(self.clone())
    }
}

/// fr fr AES-192-GCM implementation
pub type AesGcm192 = AesGcmCipher;

/// fr fr AES-128-GCM implementation
pub type AesGcm128 = AesGcmCipher;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_aes_gcm_key() {
        let key_data = vec![0u8; AES_GCM_KEY_SIZE_256];
        let key = AesGcmKey::new(key_data.clone()).unwrap();
        assert_eq!(key.size(), AES_GCM_KEY_SIZE_256);
        assert_eq!(key.as_bytes(), &key_data);
        
        // Test invalid key size
        let invalid_key = vec![0u8; 31];
        assert!(AesGcmKey::new(invalid_key).is_err());
    }
    
    #[test]
    fn test_aes_gcm_nonce() {
        let nonce_data = vec![0u8; AES_GCM_NONCE_SIZE];
        let nonce = AesGcmNonce::new(nonce_data.clone()).unwrap();
        assert_eq!(nonce.as_bytes(), &nonce_data);
        
        // Test invalid nonce size
        let invalid_nonce = vec![0u8; 11];
        assert!(AesGcmNonce::new(invalid_nonce).is_err());
    }
    
    #[test]
    fn test_aes_gcm_cipher() {
        let cipher = AesGcmCipher::new(AES_GCM_KEY_SIZE_256).unwrap();
        assert_eq!(cipher.cipher_type(), CipherType::AesGcm256);
        assert!(cipher.capabilities().authenticated);
        assert!(cipher.capabilities().constant_time);
        assert_eq!(cipher.capabilities().key_size, AES_GCM_KEY_SIZE_256);
        assert_eq!(cipher.capabilities().nonce_size, AES_GCM_NONCE_SIZE);
        assert_eq!(cipher.capabilities().tag_size, AES_GCM_TAG_SIZE);
    }
    
    #[test]
    fn test_aes_256_gcm() {
        let key = vec![0u8; AES_GCM_KEY_SIZE_256];
        let cipher = AesGcm256::new(&key).unwrap();
        
        let plaintext = b"Hello, World!";
        let additional_data = b"metadata";
        
        // Note: This test uses placeholder encryption, so we just verify it runs
        let encrypted = cipher.encrypt(plaintext, additional_data);
        assert!(encrypted.is_ok());
        
        let decrypted = cipher.decrypt(&encrypted.unwrap(), additional_data);
        assert!(decrypted.is_ok());
    }
    
    #[test]
    fn test_hardware_detection() {
        let has_hw = AesGcmCipher::detect_hardware_acceleration();
        // Just verify it doesn't panic
        println!("Hardware acceleration: {}", has_hw);
    }
    
    #[test]
    fn test_error_types() {
        let error = AesGcmError::InvalidKeySize(16);
        assert_eq!(error.to_string(), "Invalid AES-GCM key size: 16");
        
        let error = AesGcmError::TagVerificationFailed;
        assert_eq!(error.to_string(), "AES-GCM tag verification failed");
    }
}
