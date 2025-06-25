// Common cryptographic types for CURSED

/// Cryptographic platform abstraction
#[derive(Debug, Clone, PartialEq)]
pub enum CryptoPlatform {
    /// Software implementation
    Software,
    /// Hardware security module
    HSM,
    /// Trusted execution environment
    TEE,
    /// WebAssembly
    WASM,
}

impl Default for CryptoPlatform {
    fn default() -> Self {
        CryptoPlatform::Software
    }
}

/// Polynomial commitment scheme
#[derive(Debug, Clone)]
pub struct PolynomialCommitment {
    pub commitment: Vec<u8>,
    pub proof: Option<Vec<u8>>,
    pub degree_bound: Option<usize>,
}

impl PolynomialCommitment {
    /// Create new polynomial commitment
    pub fn new(commitment: Vec<u8>) -> Self {
        Self {
            commitment,
            proof: None,
            degree_bound: None,
        }
    }
    
    /// Create with proof
    pub fn with_proof(mut self, proof: Vec<u8>) -> Self {
        self.proof = Some(proof);
        self
    }
    
    /// Create with degree bound
    pub fn with_degree_bound(mut self, bound: usize) -> Self {
        self.degree_bound = Some(bound);
        self
    }
    
    /// Verify the commitment
    pub fn verify(&self) -> Result<bool, CryptoError> {
        // Placeholder implementation
        Ok(true)
    }
    
    /// Get commitment bytes
    pub fn commitment_bytes(&self) -> &[u8] {
        &self.commitment
    }
    
    /// Get proof if available
    pub fn proof_bytes(&self) -> Option<&[u8]> {
        self.proof.as_ref().map(|p| p.as_slice())
    }
}

/// Cryptographic error types
#[derive(Debug, Clone)]
pub enum CryptoError {
    /// Invalid key format or content
    InvalidKey(String),
    /// Invalid signature
    InvalidSignature(String),
    /// Invalid format
    InvalidFormat(String),
    /// Verification failed
    VerificationFailed,
    /// Key generation failed
    KeyGenerationFailed(String),
    /// Encryption failed
    EncryptionFailed(String),
    /// Decryption failed
    DecryptionFailed(String),
    /// Hash computation failed
    HashFailed(String),
    /// Random number generation failed
    RandomGenerationFailed,
    /// Platform not supported
    PlatformNotSupported(CryptoPlatform),
    /// Generic crypto error
    Generic(String),
}

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
    SHA256,
    SHA512,
    BLAKE3,
    Keccak256,
}

impl HashAlgorithm {
    /// Get the output size in bytes
    pub fn output_size(&self) -> usize {
        match self {
            HashAlgorithm::SHA256 => 32,
            HashAlgorithm::SHA512 => 64,
            HashAlgorithm::BLAKE3 => 32,
            HashAlgorithm::Keccak256 => 32,
        }
    }
    
    /// Get the algorithm name
    pub fn name(&self) -> &'static str {
        match self {
            HashAlgorithm::SHA256 => "SHA256",
            HashAlgorithm::SHA512 => "SHA512",
            HashAlgorithm::BLAKE3 => "BLAKE3",
            HashAlgorithm::Keccak256 => "Keccak256",
        }
    }
}

/// Signature algorithm types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignatureAlgorithm {
    Ed25519,
    ECDSA,
    RSA,
    BLS,
}

impl SignatureAlgorithm {
    /// Get the algorithm name
    pub fn name(&self) -> &'static str {
        match self {
            SignatureAlgorithm::Ed25519 => "Ed25519",
            SignatureAlgorithm::ECDSA => "ECDSA",
            SignatureAlgorithm::RSA => "RSA",
            SignatureAlgorithm::BLS => "BLS",
        }
    }
    
    /// Get typical signature size in bytes
    pub fn signature_size(&self) -> usize {
        match self {
            SignatureAlgorithm::Ed25519 => 64,
            SignatureAlgorithm::ECDSA => 64, // Depends on curve
            SignatureAlgorithm::RSA => 256, // Depends on key size
            SignatureAlgorithm::BLS => 48,
        }
    }
    
    /// Get typical public key size in bytes
    pub fn public_key_size(&self) -> usize {
        match self {
            SignatureAlgorithm::Ed25519 => 32,
            SignatureAlgorithm::ECDSA => 33, // Compressed
            SignatureAlgorithm::RSA => 256, // Depends on key size
            SignatureAlgorithm::BLS => 48,
        }
    }
}
