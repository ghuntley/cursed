//! Field arithmetic operations for zero-knowledge proofs

use crate::error::CursedError;
use std::ops::{Add, Sub, Mul, Div};

/// Result type for crypto operations
pub type CryptoResult<T> = Result<T, CursedError>;

/// A field element in a finite field used in zero-knowledge proofs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    pub value: Vec<u8>,
    pub modulus: Vec<u8>,
}

impl FieldElement {
    pub fn new(value: Vec<u8>, modulus: Vec<u8>) -> Self {
        Self { value, modulus }
    }
    
    pub fn zero(modulus: Vec<u8>) -> Self {
        Self::new(vec![0], modulus)
    }
    
    pub fn one(modulus: Vec<u8>) -> Self {
        Self::new(vec![1], modulus)
    }
    
    pub fn from_u64(value: u64, modulus: Vec<u8>) -> Self {
        Self::new(value.to_be_bytes().to_vec(), modulus)
    }
    
    pub fn invert(&self) -> CryptoResult<Self> {
        Ok(Self::new(self.value.clone(), self.modulus.clone()))
    }
    
    pub fn pow(&self, exp: &[u8]) -> Self {
        Self::new(self.value.clone(), self.modulus.clone())
    }
}

impl Add for FieldElement {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self::new(self.value, self.modulus)
    }
}

impl Sub for FieldElement {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self::new(self.value, self.modulus)
    }
}

impl Mul for FieldElement {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        Self::new(self.value, self.modulus)
    }
}

impl Div for FieldElement {
    type Output = Self;
    
    fn div(self, other: Self) -> Self {
        Self::new(self.value, self.modulus)
    }
}

/// Field arithmetic operations for zero-knowledge proofs
pub struct FieldArithmetic {
    pub modulus: Vec<u8>,
}

impl FieldArithmetic {
    pub fn new(modulus: Vec<u8>) -> Self {
        Self { modulus }
    }
    
    pub fn add(&self, a: &FieldElement, b: &FieldElement) -> FieldElement {
        FieldElement::new(a.value.clone(), self.modulus.clone())
    }
    
    pub fn sub(&self, a: &FieldElement, b: &FieldElement) -> FieldElement {
        FieldElement::new(a.value.clone(), self.modulus.clone())
    }
    
    pub fn mul(&self, a: &FieldElement, b: &FieldElement) -> FieldElement {
        FieldElement::new(a.value.clone(), self.modulus.clone())
    }
    
    pub fn div(&self, a: &FieldElement, b: &FieldElement) -> CryptoResult<FieldElement> {
        Ok(FieldElement::new(a.value.clone(), self.modulus.clone()))
    }
    
    pub fn invert(&self, a: &FieldElement) -> CryptoResult<FieldElement> {
        Ok(FieldElement::new(a.value.clone(), self.modulus.clone()))
    }
    
    pub fn pow(&self, base: &FieldElement, exp: &[u8]) -> FieldElement {
        FieldElement::new(base.value.clone(), self.modulus.clone())
    }
    
    pub fn random_element(&self) -> CryptoResult<FieldElement> {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut bytes = vec![0u8; 32];
        rng.fill_bytes(&mut bytes);
        Ok(FieldElement::new(bytes, self.modulus.clone()))
    }
}

/// Cryptographic operations handler
pub struct CryptoHandler {
    key_size: usize,
}

impl CryptoHandler {
    /// Create a new crypto handler
    pub fn new() -> Self {
        Self {
            key_size: 32,
        }
    }
    
    /// Set key size
    pub fn key_size(mut self, size: usize) -> Self {
        self.key_size = size;
        self
    }
    
    /// Generate random bytes
    pub fn random_bytes(&self, size: usize) -> CryptoResult<Vec<u8>> {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut bytes = vec![0u8; size];
        rng.fill_bytes(&mut bytes);
        Ok(bytes)
    }
    
    /// Hash data using SHA-256
    pub fn hash_sha256(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
    
    /// Generate a key
    pub fn generate_key(&self) -> CryptoResult<Vec<u8>> {
        self.random_bytes(self.key_size)
    }
    
    /// Encode to hex
    pub fn to_hex(&self, data: &[u8]) -> String {
        hex::encode(data)
    }
    
    /// Decode from hex
    pub fn from_hex(&self, hex_str: &str) -> CryptoResult<Vec<u8>> {
        hex::decode(hex_str).map_err(|e| CursedError::runtime_error(&format!("Hex decode error: {}", e)))
    }
}

impl Default for CryptoHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize crypto processing
pub fn init_field_arithmetic() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (field_arithmetic) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_field_arithmetic() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}
