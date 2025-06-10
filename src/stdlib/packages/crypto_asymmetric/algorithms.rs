/// fr fr Asymmetric algorithms stub  
#[derive(Debug, Clone)]
pub enum AsymmetricAlgorithm {
    Ed25519,
    Rsa2048,
    EcdsaP256,
}

impl AsymmetricAlgorithm {
    pub fn name(&self) -> &str {
        match self {
            AsymmetricAlgorithm::Ed25519 => "Ed25519",
            AsymmetricAlgorithm::Rsa2048 => "RSA-2048", 
            AsymmetricAlgorithm::EcdsaP256 => "ECDSA-P256",
        }
    }
}
