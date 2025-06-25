/// Advanced cryptographic hash algorithms supported by the CURSED language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdvancedHashAlgorithm {
    /// SHA-256 (Secure Hash Algorithm 256-bit)
    Sha256,
    /// SHA-512 (Secure Hash Algorithm 512-bit)
    Sha512,
    /// BLAKE3 (Fast cryptographic hash function)
    Blake3,
    /// SHA-3-256 (Keccak-based hash function)
    Sha3_256,
    /// SHA-3-512 (Keccak-based hash function)
    Sha3_512,
    /// Keccak-256 (Original Keccak, used by Ethereum)
    Keccak256,
    /// xxHash64 (Fast non-cryptographic hash)
    XxHash64,
    /// SipHash (Keyed hash function for hash tables)
    SipHash,
}

impl AdvancedHashAlgorithm {
    /// Get the algorithm name
    pub fn name(&self) -> &str {
        match self {
            AdvancedHashAlgorithm::Sha256 => "SHA-256",
            AdvancedHashAlgorithm::Sha512 => "SHA-512",
            AdvancedHashAlgorithm::Blake3 => "BLAKE3",
            AdvancedHashAlgorithm::Sha3_256 => "SHA3-256",
            AdvancedHashAlgorithm::Sha3_512 => "SHA3-512",
            AdvancedHashAlgorithm::Keccak256 => "Keccak-256",
            AdvancedHashAlgorithm::XxHash64 => "xxHash64",
            AdvancedHashAlgorithm::SipHash => "SipHash",
        }
    }
    
    /// Get the digest size in bytes
    pub fn digest_size(&self) -> usize {
        match self {
            AdvancedHashAlgorithm::Sha256 => 32,
            AdvancedHashAlgorithm::Sha512 => 64,
            AdvancedHashAlgorithm::Blake3 => 32,
            AdvancedHashAlgorithm::Sha3_256 => 32,
            AdvancedHashAlgorithm::Sha3_512 => 64,
            AdvancedHashAlgorithm::Keccak256 => 32,
            AdvancedHashAlgorithm::XxHash64 => 8,
            AdvancedHashAlgorithm::SipHash => 8,
        }
    }
    
    /// Check if the algorithm is cryptographically secure
    pub fn is_cryptographic(&self) -> bool {
        match self {
            AdvancedHashAlgorithm::Sha256 |
            AdvancedHashAlgorithm::Sha512 |
            AdvancedHashAlgorithm::Blake3 |
            AdvancedHashAlgorithm::Sha3_256 |
            AdvancedHashAlgorithm::Sha3_512 |
            AdvancedHashAlgorithm::Keccak256 => true,
            AdvancedHashAlgorithm::XxHash64 |
            AdvancedHashAlgorithm::SipHash => false,
        }
    }
    
    /// Get the security level in bits
    pub fn security_level(&self) -> Option<usize> {
        match self {
            AdvancedHashAlgorithm::Sha256 |
            AdvancedHashAlgorithm::Blake3 |
            AdvancedHashAlgorithm::Sha3_256 |
            AdvancedHashAlgorithm::Keccak256 => Some(128),
            AdvancedHashAlgorithm::Sha512 |
            AdvancedHashAlgorithm::Sha3_512 => Some(256),
            AdvancedHashAlgorithm::XxHash64 |
            AdvancedHashAlgorithm::SipHash => None, // Non-cryptographic
        }
    }
    
    /// Get algorithm description
    pub fn description(&self) -> &str {
        match self {
            AdvancedHashAlgorithm::Sha256 => "NIST SHA-2 family, 256-bit digest, widely used",
            AdvancedHashAlgorithm::Sha512 => "NIST SHA-2 family, 512-bit digest, high security",
            AdvancedHashAlgorithm::Blake3 => "Modern cryptographic hash, very fast, secure",
            AdvancedHashAlgorithm::Sha3_256 => "NIST SHA-3 family, 256-bit, Keccak-based",
            AdvancedHashAlgorithm::Sha3_512 => "NIST SHA-3 family, 512-bit, Keccak-based",
            AdvancedHashAlgorithm::Keccak256 => "Original Keccak, used by Ethereum blockchain",
            AdvancedHashAlgorithm::XxHash64 => "Very fast non-cryptographic hash for checksums",
            AdvancedHashAlgorithm::SipHash => "Keyed hash for hash tables, DoS resistant",
        }
    }
    
    /// Parse algorithm from string name
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "sha-256" | "sha256" => Some(AdvancedHashAlgorithm::Sha256),
            "sha-512" | "sha512" => Some(AdvancedHashAlgorithm::Sha512),
            "blake3" => Some(AdvancedHashAlgorithm::Blake3),
            "sha3-256" | "sha3_256" => Some(AdvancedHashAlgorithm::Sha3_256),
            "sha3-512" | "sha3_512" => Some(AdvancedHashAlgorithm::Sha3_512),
            "keccak-256" | "keccak256" => Some(AdvancedHashAlgorithm::Keccak256),
            "xxhash64" | "xxhash-64" => Some(AdvancedHashAlgorithm::XxHash64),
            "siphash" => Some(AdvancedHashAlgorithm::SipHash),
            _ => None,
        }
    }
    
    /// Get all available algorithms
    pub fn all() -> Vec<AdvancedHashAlgorithm> {
        vec![
            AdvancedHashAlgorithm::Sha256,
            AdvancedHashAlgorithm::Sha512,
            AdvancedHashAlgorithm::Blake3,
            AdvancedHashAlgorithm::Sha3_256,
            AdvancedHashAlgorithm::Sha3_512,
            AdvancedHashAlgorithm::Keccak256,
            AdvancedHashAlgorithm::XxHash64,
            AdvancedHashAlgorithm::SipHash,
        ]
    }
    
    /// Get only cryptographic algorithms
    pub fn cryptographic() -> Vec<AdvancedHashAlgorithm> {
        Self::all().into_iter()
            .filter(|alg| alg.is_cryptographic())
            .collect()
    }
    
    /// Get only fast (non-cryptographic) algorithms
    pub fn fast() -> Vec<AdvancedHashAlgorithm> {
        Self::all().into_iter()
            .filter(|alg| !alg.is_cryptographic())
            .collect()
    }
}

impl std::fmt::Display for AdvancedHashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::str::FromStr for AdvancedHashAlgorithm {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_name(s).ok_or_else(|| format!("Unknown algorithm: {}", s))
    }
}

