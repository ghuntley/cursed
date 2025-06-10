/// fr fr Hash algorithms
#[derive(Debug, Clone)]
pub enum AdvancedHashAlgorithm {
    Sha256,
    Sha512,
    Blake3,
}

impl AdvancedHashAlgorithm {
    pub fn name(&self) -> &str {
        match self {
            AdvancedHashAlgorithm::Sha256 => "SHA-256",
            AdvancedHashAlgorithm::Sha512 => "SHA-512",
            AdvancedHashAlgorithm::Blake3 => "BLAKE3",
        }
    }
}
