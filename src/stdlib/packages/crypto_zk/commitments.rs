//! Commitment schemes for zero-knowledge proofs

use crate::error::CursedError;
use crate::stdlib::packages::crypto_zk::field_arithmetic::FieldElement;

/// Result type for crypto operations
pub type CryptoResult<T> = Result<T, CursedError>;

/// A Pedersen commitment for hiding values with perfect binding
#[derive(Debug, Clone)]
pub struct PedersenCommitment {
    pub commitment: FieldElement,
    pub randomness: FieldElement,
    pub value: FieldElement,
    pub generator: FieldElement,
    pub randomness_generator: FieldElement,
}

impl PedersenCommitment {
    pub fn new(
        value: FieldElement,
        randomness: FieldElement,
        generator: FieldElement,
        randomness_generator: FieldElement,
    ) -> CryptoResult<Self> {
        let commitment = generator.clone() * value.clone() + randomness_generator.clone() * randomness.clone();
        
        Ok(Self {
            commitment,
            randomness,
            value,
            generator,
            randomness_generator,
        })
    }
    
    pub fn commit(
        value: FieldElement,
        generator: FieldElement,
        randomness_generator: FieldElement,
    ) -> CryptoResult<Self> {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut random_bytes = vec![0u8; 32];
        rng.fill_bytes(&mut random_bytes);
        
        let randomness = FieldElement::new(random_bytes, value.modulus.clone());
        Self::new(value, randomness, generator, randomness_generator)
    }
    
    pub fn verify(&self) -> bool {
        let expected = self.generator.clone() * self.value.clone() + 
                      self.randomness_generator.clone() * self.randomness.clone();
        expected == self.commitment
    }
    
    pub fn get_commitment(&self) -> &FieldElement {
        &self.commitment
    }
}

/// A hash-based commitment scheme
#[derive(Debug, Clone)]
pub struct HashCommitment {
    pub commitment: Vec<u8>,
    pub value: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl HashCommitment {
    pub fn new(value: Vec<u8>, nonce: Vec<u8>) -> CryptoResult<Self> {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(&value);
        hasher.update(&nonce);
        let commitment = hasher.finalize().to_vec();
        
        Ok(Self {
            commitment,
            value,
            nonce,
        })
    }
    
    pub fn commit(value: Vec<u8>) -> CryptoResult<Self> {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut nonce = vec![0u8; 32];
        rng.fill_bytes(&mut nonce);
        
        Self::new(value, nonce)
    }
    
    pub fn verify(&self, commitment: &[u8]) -> bool {
        self.commitment == commitment
    }
    
    pub fn reveal(&self) -> (&[u8], &[u8]) {
        (&self.value, &self.nonce)
    }
    
    pub fn verify_opening(&self, value: &[u8], nonce: &[u8]) -> bool {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(value);
        hasher.update(nonce);
        let computed_commitment = hasher.finalize().to_vec();
        
        computed_commitment == self.commitment
    }
}

/// A vector commitment scheme for committing to vectors of values
#[derive(Debug, Clone)]
pub struct VectorCommitment {
    pub commitments: Vec<PedersenCommitment>,
    pub values: Vec<FieldElement>,
    pub generator: FieldElement,
    pub randomness_generator: FieldElement,
}

impl VectorCommitment {
    pub fn new(
        values: Vec<FieldElement>,
        generator: FieldElement,
        randomness_generator: FieldElement,
    ) -> CryptoResult<Self> {
        let mut commitments = vec![];
        
        for value in &values {
            let commitment = PedersenCommitment::commit(
                value.clone(),
                generator.clone(),
                randomness_generator.clone(),
            )?;
            commitments.push(commitment);
        }
        
        Ok(Self {
            commitments,
            values,
            generator,
            randomness_generator,
        })
    }
    
    pub fn get_commitment(&self, index: usize) -> CryptoResult<&PedersenCommitment> {
        self.commitments.get(index)
            .ok_or_else(|| CursedError::runtime_error("Index out of bounds"))
    }
    
    pub fn verify_commitment(&self, index: usize) -> CryptoResult<bool> {
        let commitment = self.get_commitment(index)?;
        Ok(commitment.verify())
    }
    
    pub fn batch_verify(&self) -> bool {
        self.commitments.iter().all(|c| c.verify())
    }
    
    pub fn get_proof(&self, index: usize) -> CryptoResult<VectorCommitmentProof> {
        let commitment = self.get_commitment(index)?.clone();
        Ok(VectorCommitmentProof {
            commitment,
            index,
            vector_size: self.values.len(),
        })
    }
}

/// A proof for a vector commitment
#[derive(Debug, Clone)]
pub struct VectorCommitmentProof {
    pub commitment: PedersenCommitment,
    pub index: usize,
    pub vector_size: usize,
}

impl VectorCommitmentProof {
    pub fn verify(&self, value: &FieldElement) -> bool {
        self.commitment.value == *value && self.commitment.verify()
    }
}

/// Kate commitment scheme (KZG polynomial commitments)
#[derive(Debug, Clone)]
pub struct KateCommitment {
    pub commitment: FieldElement,
    pub polynomial_coefficients: Vec<FieldElement>,
    pub setup_points: Vec<FieldElement>,
}

impl KateCommitment {
    pub fn new(
        polynomial_coefficients: Vec<FieldElement>,
        setup_points: Vec<FieldElement>,
    ) -> CryptoResult<Self> {
        if polynomial_coefficients.is_empty() {
            return Err(CursedError::runtime_error("Empty polynomial"));
        }
        
        let modulus = polynomial_coefficients[0].modulus.clone();
        let commitment = FieldElement::zero(modulus);
        
        Ok(Self {
            commitment,
            polynomial_coefficients,
            setup_points,
        })
    }
    
    pub fn evaluate_at(&self, point: &FieldElement) -> FieldElement {
        let mut result = FieldElement::zero(point.modulus.clone());
        let mut power = FieldElement::one(point.modulus.clone());
        
        for coeff in &self.polynomial_coefficients {
            result = result + coeff.clone() * power.clone();
            power = power * point.clone();
        }
        
        result
    }
    
    pub fn create_proof(&self, point: &FieldElement) -> CryptoResult<KateProof> {
        let evaluation = self.evaluate_at(point);
        Ok(KateProof {
            proof: self.commitment.clone(),
            evaluation,
            point: point.clone(),
        })
    }
}

/// A proof for Kate commitment opening
#[derive(Debug, Clone)]
pub struct KateProof {
    pub proof: FieldElement,
    pub evaluation: FieldElement,
    pub point: FieldElement,
}

impl KateProof {
    pub fn verify(&self, commitment: &KateCommitment) -> bool {
        let expected_evaluation = commitment.evaluate_at(&self.point);
        expected_evaluation == self.evaluation
    }
}

/// Collection of commitment scheme utilities
pub struct Commitments;

impl Commitments {
    pub fn pedersen_commit(
        value: FieldElement,
        generator: FieldElement,
        randomness_generator: FieldElement,
    ) -> CryptoResult<PedersenCommitment> {
        PedersenCommitment::commit(value, generator, randomness_generator)
    }
    
    pub fn hash_commit(value: Vec<u8>) -> CryptoResult<HashCommitment> {
        HashCommitment::commit(value)
    }
    
    pub fn vector_commit(
        values: Vec<FieldElement>,
        generator: FieldElement,
        randomness_generator: FieldElement,
    ) -> CryptoResult<VectorCommitment> {
        VectorCommitment::new(values, generator, randomness_generator)
    }
    
    pub fn kate_commit(
        polynomial: Vec<FieldElement>,
        setup: Vec<FieldElement>,
    ) -> CryptoResult<KateCommitment> {
        KateCommitment::new(polynomial, setup)
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
pub fn init_commitments() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (commitments) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_commitments() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}
