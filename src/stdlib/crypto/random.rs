//! Cryptographic random number generation

use crate::error::CursedError;

/// Result type for cryptographic random operations
pub type CryptoRandomResult<T> = Result<T, CursedError>;

/// Cryptographically secure random number generator
pub struct SecureRng {
    _phantom: std::marker::PhantomData<()>,
}

impl SecureRng {
    /// Create a new secure random number generator
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Generate a random u64
    pub fn gen_u64(&mut self) -> u64 {
        use rand::RngCore;
        rand::thread_rng().next_u64()
    }
    
    /// Generate a random u32
    pub fn gen_u32(&mut self) -> u32 {
        use rand::RngCore;
        rand::thread_rng().next_u32()
    }
    
    /// Fill bytes with random data
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        use rand::RngCore;
        rand::thread_rng().fill_bytes(dest);
    }
    
    /// Generate random bytes
    pub fn gen_bytes(&mut self, len: usize) -> Vec<u8> {
        use rand::RngCore;
        let mut bytes = vec![0u8; len];
        rand::thread_rng().fill_bytes(&mut bytes);
        bytes
    }
}

impl Default for SecureRng {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a cryptographically secure random number
pub fn secure_random() -> u64 {
    use rand::RngCore;
    rand::thread_rng().next_u64()
}

/// Generate cryptographically secure random bytes
pub fn secure_random_bytes(len: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut bytes = vec![0u8; len];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes
}

/// Generate a random string with specified length and character set
pub fn random_string(len: usize, charset: &str) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = charset.chars().collect();
    (0..len)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

/// Generate a random alphanumeric string
pub fn random_alphanumeric(len: usize) -> String {
    const CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    random_string(len, CHARSET)
}

/// Test random number generation functionality
pub fn test_crypto_random() -> CryptoRandomResult<()> {
    let mut rng = SecureRng::new();
    
    // Test basic functionality
    let _num = rng.gen_u64();
    let bytes = rng.gen_bytes(32);
    
    if bytes.len() != 32 {
        return Err(CursedError::runtime_error("Random byte generation failed"));
    }
    
    // Basic entropy check - bytes shouldn't all be the same
    if bytes.iter().all(|&b| b == bytes[0]) {
        return Err(CursedError::runtime_error("Random bytes lack entropy"));
    }
    
    Ok(())
}

/// Initialize the crypto random subsystem
pub fn init_crypto_random() -> CryptoRandomResult<()> {
    test_crypto_random()?;
    println!("🎲 Cryptographic random number generation initialized");
    Ok(())
}
