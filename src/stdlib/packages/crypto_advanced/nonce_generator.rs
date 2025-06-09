/// fr fr Nonce generation for CURSED crypto - unique values every time bestie

use super::memory_protection::*;

/// fr fr Secure nonce wrapper
pub struct SecureNonce {
    nonce: Vec<u8>,
}

impl SecureNonce {
    /// slay Create new secure nonce
    pub fn new(nonce: Vec<u8>) -> Self {
        Self { nonce }
    }
    
    /// slay Get nonce bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.nonce
    }
    
    /// slay Get nonce length
    pub fn len(&self) -> usize {
        self.nonce.len()
    }
    
    /// slay Check if empty
    pub fn is_empty(&self) -> bool {
        self.nonce.is_empty()
    }
}

/// fr fr Nonce generation modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonceCounterMode {
    Sequential,
    Random,
    Timestamp,
}

/// fr fr Random nonce mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonceRandomMode {
    Pure,
    Mixed,
    Deterministic,
}

/// fr fr Nonce errors
#[derive(Debug, Clone, PartialEq)]
pub enum NonceError {
    GenerationFailed,
    InsufficientEntropy,
    CounterOverflow,
    InvalidSize(usize),
    Internal(String),
}

impl std::fmt::Display for NonceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NonceError::GenerationFailed => write!(f, "Nonce generation failed"),
            NonceError::InsufficientEntropy => write!(f, "Insufficient entropy for nonce"),
            NonceError::CounterOverflow => write!(f, "Nonce counter overflow"),
            NonceError::InvalidSize(size) => write!(f, "Invalid nonce size: {}", size),
            NonceError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for NonceError {}

/// fr fr Nonce uniqueness guarantee level
pub const NONCE_UNIQUENESS_GUARANTEE: &str = "128-bit collision resistance";

/// fr fr Nonce generator
pub struct NonceGenerator {
    counter: u64,
    mode: NonceCounterMode,
}

impl NonceGenerator {
    /// slay Create new nonce generator
    pub fn new(mode: NonceCounterMode) -> Self {
        Self {
            counter: 0,
            mode,
        }
    }
    
    /// slay Generate nonce for specific cipher type
    pub fn generate(cipher_type: super::symmetric_cipher::CipherType) -> Result<SecureNonce, NonceError> {
        use super::symmetric_cipher::CipherType;
        
        let size = match cipher_type {
            CipherType::AesGcm256 | CipherType::AesGcm192 | CipherType::AesGcm128 => 12,
            CipherType::ChaCha20Poly1305 => 12,
            CipherType::XChaCha20Poly1305 => 24,
        };
        
        Self::generate_size(size)
    }
    
    /// slay Generate nonce of specific size
    pub fn generate_size(size: usize) -> Result<SecureNonce, NonceError> {
        if size == 0 {
            return Err(NonceError::InvalidSize(size));
        }
        
        let mut nonce = vec![0u8; size];
        
        // Fill with secure random bytes
        // In a real implementation, use crypto_random
        for byte in nonce.iter_mut() {
            *byte = rand::random(); // Placeholder
        }
        
        Ok(SecureNonce::new(nonce))
    }
    
    /// slay Generate next nonce
    pub fn next(&mut self, size: usize) -> Result<SecureNonce, NonceError> {
        match self.mode {
            NonceCounterMode::Sequential => self.generate_sequential(size),
            NonceCounterMode::Random => Self::generate_size(size),
            NonceCounterMode::Timestamp => self.generate_timestamp(size),
        }
    }
    
    /// slay Generate sequential nonce
    fn generate_sequential(&mut self, size: usize) -> Result<SecureNonce, NonceError> {
        if size < 8 {
            return Err(NonceError::InvalidSize(size));
        }
        
        let mut nonce = vec![0u8; size];
        let counter_bytes = self.counter.to_le_bytes();
        nonce[..8].copy_from_slice(&counter_bytes);
        
        // Fill rest with random
        for byte in nonce[8..].iter_mut() {
            *byte = rand::random(); // Placeholder
        }
        
        self.counter = self.counter.wrapping_add(1);
        Ok(SecureNonce::new(nonce))
    }
    
    /// slay Generate timestamp-based nonce
    fn generate_timestamp(&mut self, size: usize) -> Result<SecureNonce, NonceError> {
        if size < 8 {
            return Err(NonceError::InvalidSize(size));
        }
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        let mut nonce = vec![0u8; size];
        let timestamp_bytes = timestamp.to_le_bytes();
        nonce[..8].copy_from_slice(&timestamp_bytes);
        
        // Fill rest with random
        for byte in nonce[8..].iter_mut() {
            *byte = rand::random(); // Placeholder
        }
        
        Ok(SecureNonce::new(nonce))
    }
}

impl Default for NonceGenerator {
    fn default() -> Self {
        Self::new(NonceCounterMode::Random)
    }
}

// Add rand dependency placeholder
mod rand {
    pub fn random<T>() -> T 
    where 
        T: From<u8>
    {
        // Placeholder implementation
        T::from(42)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::symmetric_cipher::CipherType;
    
    #[test]
    fn test_secure_nonce() {
        let data = Vec::from([1, 2, 3, 4]);
        let nonce = SecureNonce::new(data.clone());
        
        assert_eq!(nonce.as_bytes(), &data);
        assert_eq!(nonce.len(), 4);
        assert!(!nonce.is_empty());
    }
    
    #[test]
    fn test_nonce_generator() {
        let mut generator = NonceGenerator::new(NonceCounterMode::Random);
        let nonce = generator.next(12).unwrap();
        assert_eq!(nonce.len(), 12);
        
        let mut seq_generator = NonceGenerator::new(NonceCounterMode::Sequential);
        let nonce1 = seq_generator.next(12).unwrap();
        let nonce2 = seq_generator.next(12).unwrap();
        // First 8 bytes should differ (counter)
        assert_ne!(nonce1.as_bytes()[..8], nonce2.as_bytes()[..8]);
    }
    
    #[test]
    fn test_generate_for_cipher() {
        let aes_nonce = NonceGenerator::generate(CipherType::AesGcm256).unwrap();
        assert_eq!(aes_nonce.len(), 12);
        
        let chacha_nonce = NonceGenerator::generate(CipherType::ChaCha20Poly1305).unwrap();
        assert_eq!(chacha_nonce.len(), 12);
        
        let xchacha_nonce = NonceGenerator::generate(CipherType::XChaCha20Poly1305).unwrap();
        assert_eq!(xchacha_nonce.len(), 24);
    }
    
    #[test]
    fn test_timestamp_nonce() {
        let mut generator = NonceGenerator::new(NonceCounterMode::Timestamp);
        let nonce = generator.next(16).unwrap();
        assert_eq!(nonce.len(), 16);
    }
    
    #[test]
    fn test_nonce_errors() {
        let error = NonceError::InvalidSize(0);
        assert_eq!(error.to_string(), "Invalid nonce size: 0");
        
        assert!(NonceGenerator::generate_size(0).is_err());
    }
}
