/// fr fr Ed25519 implementation stub
#[derive(Debug, Clone)]
pub struct Ed25519PublicKey {
    bytes: Vec<u8>,
}

impl Ed25519PublicKey {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
    
    pub fn to_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug, Clone)]
pub struct Ed25519PrivateKey {
    bytes: Vec<u8>,
}

impl Ed25519PrivateKey {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
    
    pub fn to_bytes(&self) -> &[u8] {
        &self.bytes
    }
}
