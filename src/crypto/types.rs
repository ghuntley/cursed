// Common cryptographic types for CURSED

/// Cryptographic platform abstraction
#[derive(Debug, Clone, PartialEq)]
pub enum CryptoPlatform {
    /// Software implementation
    /// Hardware security module
    /// Trusted execution environment
    /// WebAssembly
impl Default for CryptoPlatform {
    fn default() -> Self {
        CryptoPlatform::Software
    }
}

/// Polynomial commitment scheme
#[derive(Debug, Clone)]
pub struct PolynomialCommitment {
impl PolynomialCommitment {
    /// Create new polynomial commitment
    pub fn new(commitment: Vec<u8>) -> Self {
        Self {
        }
    }
    
    /// Create with proof
    pub fn with_proof(mut self, proof: Vec<u8>) -> Self {
        self.proof = Some(proof);
        self
    /// Create with degree bound
    pub fn with_degree_bound(mut self, bound: usize) -> Self {
        self.degree_bound = Some(bound);
        self
    /// Verify the commitment
    pub fn verify(&self) -> Result<bool, CryptoError> {
        // Placeholder implementation
        Ok(true)
    /// Get commitment bytes
    pub fn commitment_bytes(&self) -> &[u8] {
        &self.commitment
    /// Get proof if available
    pub fn proof_bytes(&self) -> Option<&[u8]> {
        self.proof.as_ref().map(|p| p.as_slice())
    }
}

/// Cryptographic error types
#[derive(Debug, Clone)]
pub enum CryptoError {
    /// Invalid key format or content
    /// Invalid signature
    /// Invalid format
    /// Verification failed
    /// Key generation failed
    /// Encryption failed
    /// Decryption failed
    /// Hash computation failed
    /// Random number generation failed
    /// Platform not supported
    /// Generic crypto error
// impl std::fmt::Display for CryptoError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CryptoError::InvalidKey(msg) => write!(f, "Invalid key: {}", msg),
//             CryptoError::InvalidSignature(msg) => write!(f, "Invalid signature: {}", msg),
//             CryptoError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
//             CryptoError::VerificationFailed => write!(f, "Verification failed"),
//             CryptoError::KeyGenerationFailed(msg) => write!(f, "Key generation failed: {}", msg),
//             CryptoError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
//             CryptoError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
//             CryptoError::HashFailed(msg) => write!(f, "Hash computation failed: {}", msg),
//             CryptoError::RandomGenerationFailed => write!(f, "Random number generation failed"),
//             CryptoError::PlatformNotSupported(platform) => {
//                 write!(f, "Platform not supported: {:?}", platform)
//             },
//             CryptoError::Generic(msg) => write!(f, "Crypto error: {}", msg),
//         }
//     }
// }

// impl std::error::Error for CryptoError {}
// 
impl From<CryptoError> for crate::error::Error {
    fn from(err: CryptoError) -> Self {
        crate::error::Error::Generic(err.to_string())
    }
}

/// Hash algorithm types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HashAlgorithm {
impl HashAlgorithm {
    /// Get the output size in bytes
    pub fn output_size(&self) -> usize {
        match self {
        }
    }
    
    /// Get the algorithm name
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
/// Signature algorithm types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignatureAlgorithm {
impl SignatureAlgorithm {
    /// Get the algorithm name
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
    
    /// Get typical signature size in bytes
    pub fn signature_size(&self) -> usize {
        match self {
            SignatureAlgorithm::ECDSA => 64, // Depends on curve
            SignatureAlgorithm::RSA => 256, // Depends on key size
        }
    }
    
    /// Get typical public key size in bytes
    pub fn public_key_size(&self) -> usize {
        match self {
            SignatureAlgorithm::ECDSA => 33, // Compressed
            SignatureAlgorithm::RSA => 256, // Depends on key size
        }
    }
}
