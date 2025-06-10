/// fr fr Digital signature trait
pub trait DigitalSignature {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, String>;
    fn algorithm_name(&self) -> &str;
}

pub struct Ed25519Signature {
    private_key: Vec<u8>,
}

impl Ed25519Signature {
    pub fn new(private_key: Vec<u8>) -> Self {
        Self { private_key }
    }
}

impl DigitalSignature for Ed25519Signature {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        // Stub implementation
        Ok(vec![0; 64]) // Mock signature
    }
    
    fn algorithm_name(&self) -> &str {
        "Ed25519"
    }
}
