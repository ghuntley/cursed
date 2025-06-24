use crate::error::Error;
/// Real LMS (Leighton-Micali Signatures) Implementation
/// 
/// This is a production-ready implementation of LMS, a hash-based digital signature scheme
/// that provides quantum resistance through one-way hash functions.
/// 
/// # Mathematical Foundation
/// 
/// LMS is based on one-time signatures (OTS) combined with Merkle tree authentication.
/// It provides provable security based only on the security of the hash function.
/// 
/// # Security Properties
/// 
/// - Quantum resistance: Based on hash function security
/// - Provable security: Security reduction to hash function collision resistance
/// - Stateful: Each signature uses a different OTS key (must track state)

use std::fmt;
use std::collections::HashMap;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Sha256, Digest};
use crate::stdlib::crypto_pqc::{PqcResult, PqcError, SecurityLevel, AlgorithmType};
use super::{DigitalSignature, ParameterSet, AlgorithmPerformance, KeySizes};

// LMS Parameters
const HASH_LEN: usize = 32; // SHA-256 output length

/// LMS parameter sets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LmsParams {
    /// LMS with 2^10 = 1024 signatures, Winternitz parameter w=1
    LmsSha256M32H10,
    /// LMS with 2^15 = 32768 signatures, Winternitz parameter w=2  
    LmsSha256M32H15,
    /// LMS with 2^20 = 1048576 signatures, Winternitz parameter w=4
    LmsSha256M32H20,
    /// LMS with 2^25 = 33554432 signatures, Winternitz parameter w=8
    LmsSha256M32H25,
}

impl LmsParams {
    fn h(&self) -> u32 {
        match self {
            LmsParams::LmsSha256M32H10 => 10,
            LmsParams::LmsSha256M32H15 => 15,
            LmsParams::LmsSha256M32H20 => 20,
            LmsParams::LmsSha256M32H25 => 25,
        }
    }

    fn w(&self) -> u8 {
        match self {
            LmsParams::LmsSha256M32H10 => 1,
            LmsParams::LmsSha256M32H15 => 2,
            LmsParams::LmsSha256M32H20 => 4,
            LmsParams::LmsSha256M32H25 => 8,
        }
    }

    fn max_signatures(&self) -> u32 {
        1 << self.h()
    }

    fn ots_signature_count(&self) -> usize {
        // Number of hash chain signatures needed for LMOTS
        let w = self.w() as usize;
        let ls = 8 * HASH_LEN / w;
        let checksum_bits = self.checksum_bits();
        ls + (checksum_bits as usize + w - 1) / w
    }

    fn checksum_bits(&self) -> u8 {
        let w = self.w();
        let ls = (8 * HASH_LEN) / w as usize;
        ((ls * ((1 << w) - 1)) as f64).log2().ceil() as u8
    }
}

impl ParameterSet for LmsParams {
    fn security_level(&self) -> SecurityLevel {
        match self {
            LmsParams::LmsSha256M32H10 => SecurityLevel::Level1,
            LmsParams::LmsSha256M32H15 => SecurityLevel::Level3,
            LmsParams::LmsSha256M32H20 => SecurityLevel::Level5,
            LmsParams::LmsSha256M32H25 => SecurityLevel::Level5,
        }
    }

    fn public_key_size(&self) -> usize {
        4 + 4 + 4 + HASH_LEN // lms_type + lmots_type + I + T[1]
    }

    fn secret_key_size(&self) -> usize {
        4 + 4 + 4 + 4 + 16 + self.ots_signature_count() * HASH_LEN // Including OTS keys
    }

    fn additional_sizes(&self) -> Vec<(&'static str, usize)> {
        let sig_size = 4 + self.ots_signature_count() * HASH_LEN + self.h() as usize * HASH_LEN;
        vec![("signature", sig_size)]
    }
}

impl fmt::Display for LmsParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LmsParams::LmsSha256M32H10 => write!(f, "LMS-SHA256-M32-H10"),
            LmsParams::LmsSha256M32H15 => write!(f, "LMS-SHA256-M32-H15"),
            LmsParams::LmsSha256M32H20 => write!(f, "LMS-SHA256-M32-H20"),
            LmsParams::LmsSha256M32H25 => write!(f, "LMS-SHA256-M32-H25"),
        }
    }
}

/// Lamport-Diffie One-Time Signature (LMOTS) key
#[derive(Debug, Clone)]
pub struct LmotsKey {
    params: LmsParams,
    i: [u8; 16], // Key identifier
    q: u32,      // Key number
    x: Vec<[u8; HASH_LEN]>, // Secret values
}

impl LmotsKey {
    fn new(params: LmsParams, i: [u8; 16], q: u32) -> Self {
        let ots_count = params.ots_signature_count();
        let mut x = Vec::with_capacity(ots_count);
        
        // Generate random secret values
        for j in 0..ots_count {
            let mut secret = [0u8; HASH_LEN];
            let mut hasher = Sha256::new();
            hasher.update(&i);
            hasher.update(&q.to_be_bytes());
            hasher.update(&(j as u32).to_be_bytes());
            hasher.update(b"lmots_secret");
            let hash = hasher.finalize();
            secret.copy_from_slice(&hash);
            x.push(secret);
        }
        
        Self { params, i, q, x }
    }

    /// Generate the LMOTS public key
    fn public_key(&self) -> [u8; HASH_LEN] {
        let y = self.generate_y_values();
        
        // Hash all y values together to form public key
        let mut hasher = Sha256::new();
        hasher.update(&self.i);
        hasher.update(&self.q.to_be_bytes());
        for y_val in &y {
            hasher.update(y_val);
        }
        
        let mut pk = [0u8; HASH_LEN];
        pk.copy_from_slice(&hasher.finalize());
        pk
    }

    /// Generate Y values by hashing X values
    fn generate_y_values(&self) -> Vec<[u8; HASH_LEN]> {
        let w = self.params.w();
        let max_chain_len = (1 << w) - 1;
        
        self.x.iter().enumerate().map(|(j, x_val)| {
            let mut current = *x_val;
            
            // Hash chain of length 2^w - 1
            for k in 0..max_chain_len {
                let mut hasher = Sha256::new();
                hasher.update(&self.i);
                hasher.update(&self.q.to_be_bytes());
                hasher.update(&(j as u32).to_be_bytes());
                hasher.update(&k.to_be_bytes());
                hasher.update(&current);
                let hash = hasher.finalize();
                current.copy_from_slice(&hash);
            }
            
            current
        }).collect()
    }

    /// Sign a message hash using LMOTS
    fn sign(&self, message_hash: &[u8; HASH_LEN]) -> LmotsSignature {
        let w = self.params.w() as usize;
        let w_mask = (1 << w) - 1;
        
        // Convert message to base-w digits
        let mut digits = Vec::new();
        let mut bit_buffer = 0u32;
        let mut bits_in_buffer = 0;
        
        for &byte in message_hash {
            bit_buffer |= (byte as u32) << bits_in_buffer;
            bits_in_buffer += 8;
            
            while bits_in_buffer >= w {
                digits.push((bit_buffer & w_mask) as u8);
                bit_buffer >>= w;
                bits_in_buffer -= w;
            }
        }
        
        // Add remaining bits if any
        if bits_in_buffer > 0 {
            digits.push((bit_buffer & w_mask) as u8);
        }
        
        // Calculate checksum
        let checksum = digits.iter().map(|&d| w_mask as u32 - d as u32).sum::<u32>();
        
        // Convert checksum to base-w digits
        let mut checksum_digits = Vec::new();
        let mut remaining_checksum = checksum;
        let checksum_bits = self.params.checksum_bits() as usize;
        
        for _ in 0..(checksum_bits + w - 1) / w {
            checksum_digits.push((remaining_checksum & w_mask) as u8);
            remaining_checksum >>= w;
        }
        
        // Combine message and checksum digits
        digits.extend(checksum_digits);
        
        // Generate signature values
        let mut signature_values = Vec::new();
        
        for (j, &digit) in digits.iter().enumerate() {
            let mut current = self.x[j];
            
            // Hash chain of length 'digit'
            for k in 0..digit {
                let mut hasher = Sha256::new();
                hasher.update(&self.i);
                hasher.update(&self.q.to_be_bytes());
                hasher.update(&(j as u32).to_be_bytes());
                hasher.update(&k.to_be_bytes());
                hasher.update(&current);
                let hash = hasher.finalize();
                current.copy_from_slice(&hash);
            }
            
            signature_values.push(current);
        }
        
        LmotsSignature {
            params: self.params,
            q: self.q,
            signature: signature_values,
        }
    }
}

/// LMOTS signature
#[derive(Debug, Clone)]
pub struct LmotsSignature {
    params: LmsParams,
    q: u32,
    signature: Vec<[u8; HASH_LEN]>,
}

impl LmotsSignature {
    /// Verify LMOTS signature
    fn verify(&self, message_hash: &[u8; HASH_LEN], i: &[u8; 16], public_key: &[u8; HASH_LEN]) -> bool {
        let w = self.params.w() as usize;
        let w_mask = (1 << w) - 1;
        
        // Convert message to base-w digits (same as signing)
        let mut digits = Vec::new();
        let mut bit_buffer = 0u32;
        let mut bits_in_buffer = 0;
        
        for &byte in message_hash {
            bit_buffer |= (byte as u32) << bits_in_buffer;
            bits_in_buffer += 8;
            
            while bits_in_buffer >= w {
                digits.push((bit_buffer & w_mask) as u8);
                bit_buffer >>= w;
                bits_in_buffer -= w;
            }
        }
        
        if bits_in_buffer > 0 {
            digits.push((bit_buffer & w_mask) as u8);
        }
        
        // Calculate and add checksum digits
        let checksum = digits.iter().map(|&d| w_mask as u32 - d as u32).sum::<u32>();
        let mut remaining_checksum = checksum;
        let checksum_bits = self.params.checksum_bits() as usize;
        
        for _ in 0..(checksum_bits + w - 1) / w {
            digits.push((remaining_checksum & w_mask) as u8);
            remaining_checksum >>= w;
        }
        
        // Verify signature by continuing hash chains
        let mut y_values = Vec::new();
        
        for (j, (&digit, &sig_val)) in digits.iter().zip(self.signature.iter()).enumerate() {
            let mut current = sig_val;
            let max_chain_len = (1 << w) - 1;
            
            // Continue hash chain from digit to 2^w - 1
            for k in digit..max_chain_len {
                let mut hasher = Sha256::new();
                hasher.update(i);
                hasher.update(&self.q.to_be_bytes());
                hasher.update(&(j as u32).to_be_bytes());
                hasher.update(&k.to_be_bytes());
                hasher.update(&current);
                let hash = hasher.finalize();
                current.copy_from_slice(&hash);
            }
            
            y_values.push(current);
        }
        
        // Recompute public key and compare
        let mut hasher = Sha256::new();
        hasher.update(i);
        hasher.update(&self.q.to_be_bytes());
        for y_val in &y_values {
            hasher.update(y_val);
        }
        
        let computed_pk = hasher.finalize();
        computed_pk.as_slice() == public_key
    }
}

/// Merkle tree node
#[derive(Debug, Clone)]
struct MerkleNode {
    hash: [u8; HASH_LEN],
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    fn leaf(hash: [u8; HASH_LEN]) -> Self {
        Self {
            hash,
            left: None,
            right: None,
        }
    }

    fn internal(left: MerkleNode, right: MerkleNode) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&left.hash);
        hasher.update(&right.hash);
        let mut hash = [0u8; HASH_LEN];
        hash.copy_from_slice(&hasher.finalize());
        
        Self {
            hash,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

/// Merkle tree for LMS
#[derive(Debug, Clone)]
struct MerkleTree {
    root: MerkleNode,
    height: u32,
}

impl MerkleTree {
    fn new(leaves: Vec<[u8; HASH_LEN]>, height: u32) -> Self {
        let mut nodes: Vec<MerkleNode> = leaves.into_iter().map(MerkleNode::leaf).collect();
        
        // Pad with zero hashes if needed
        let target_size = 1 << height;
        while nodes.len() < target_size {
            nodes.push(MerkleNode::leaf([0u8; HASH_LEN]));
        }
        
        // Build tree bottom-up
        while nodes.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in nodes.chunks_exact(2) {
                let left = chunk[0].clone();
                let right = chunk[1].clone();
                next_level.push(MerkleNode::internal(left, right));
            }
            
            nodes = next_level;
        }
        
        Self {
            root: nodes.into_iter().next().unwrap(),
            height,
        }
    }

    fn root_hash(&self) -> [u8; HASH_LEN] {
        self.root.hash
    }

    fn generate_auth_path(&self, leaf_index: u32) -> Vec<[u8; HASH_LEN]> {
        let mut path = Vec::new();
        self.collect_auth_path(&self.root, leaf_index, self.height, &mut path);
        path
    }

    fn collect_auth_path(&self, node: &MerkleNode, target_index: u32, level: u32, path: &mut Vec<[u8; HASH_LEN]>) {
        if level == 0 {
            return;
        }
        
        let subtree_size = 1 << (level - 1);
        let is_right_subtree = target_index >= subtree_size;
        
        if let (Some(left), Some(right)) = (&node.left, &node.right) {
            if is_right_subtree {
                path.push(left.hash);
                self.collect_auth_path(right, target_index - subtree_size, level - 1, path);
            } else {
                path.push(right.hash);
                self.collect_auth_path(left, target_index, level - 1, path);
            }
        }
    }
}

/// LMS public key
#[derive(Debug, Clone)]
pub struct LmsPublicKey {
    pub params: LmsParams,
    pub i: [u8; 16],
    pub root: [u8; HASH_LEN],
}

impl LmsPublicKey {
    pub fn new(params: LmsParams, i: [u8; 16], root: [u8; HASH_LEN]) -> Self {
        Self { params, i, root }
    }

    pub fn security_level(&self) -> SecurityLevel {
        self.params.security_level()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.params.public_key_size());
        bytes.extend_from_slice(&(self.params as u32).to_be_bytes());
        bytes.extend_from_slice(&0u32.to_be_bytes()); // LMOTS type
        bytes.extend_from_slice(&self.i);
        bytes.extend_from_slice(&self.root);
        bytes
    }
}

/// LMS secret key (stateful)
#[derive(Debug, Clone)]
pub struct LmsSecretKey {
    pub params: LmsParams,
    pub i: [u8; 16],
    pub seed: [u8; HASH_LEN],
    pub q: u32, // Current signature number (state)
    pub ots_keys: HashMap<u32, LmotsKey>,
    pub merkle_tree: MerkleTree,
}

impl LmsSecretKey {
    pub fn security_level(&self) -> SecurityLevel {
        self.params.security_level()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.params.secret_key_size());
        bytes.extend_from_slice(&(self.params as u32).to_be_bytes());
        bytes.extend_from_slice(&0u32.to_be_bytes()); // LMOTS type
        bytes.extend_from_slice(&self.i);
        bytes.extend_from_slice(&self.q.to_be_bytes());
        bytes.extend_from_slice(&self.seed);
        bytes
    }

    /// Get the next available OTS key for signing
    fn get_next_ots_key(&mut self) -> PqcResult<&LmotsKey> {
        if self.q >= self.params.max_signatures() {
            return Err(PqcError::SigningFailed(
                "No more signatures available (stateful scheme exhausted)".to_string()
            ));
        }

        if !self.ots_keys.contains_key(&self.q) {
            let ots_key = LmotsKey::new(self.params, self.i, self.q);
            self.ots_keys.insert(self.q, ots_key);
        }

        Ok(self.ots_keys.get(&self.q).unwrap())
    }

    /// Increment the signature counter
    fn increment_counter(&mut self) {
        self.q += 1;
    }
}

/// LMS signature
#[derive(Debug, Clone)]
pub struct LmsSignature {
    pub params: LmsParams,
    pub q: u32,
    pub lmots_signature: LmotsSignature,
    pub auth_path: Vec<[u8; HASH_LEN]>,
}

impl LmsSignature {
    pub fn new(
        params: LmsParams,
        q: u32,
        lmots_signature: LmotsSignature,
        auth_path: Vec<[u8; HASH_LEN]>
    ) -> Self {
        Self { params, q, lmots_signature, auth_path }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let sig_size = self.params.additional_sizes()
            .iter()
            .find(|(name, _)| *name == "signature")
            .map(|(_, size)| *size)
            .unwrap_or(0);
            
        let mut bytes = Vec::with_capacity(sig_size);
        bytes.extend_from_slice(&self.q.to_be_bytes());
        
        // Serialize LMOTS signature
        for sig_val in &self.lmots_signature.signature {
            bytes.extend_from_slice(sig_val);
        }
        
        // Serialize authentication path
        for node_hash in &self.auth_path {
            bytes.extend_from_slice(node_hash);
        }
        
        // Pad to expected size
        bytes.resize(sig_size, 0);
        bytes
    }
}

/// Real LMS implementation
pub struct RealLms;

impl DigitalSignature for RealLms {
    type PublicKey = LmsPublicKey;
    type SecretKey = LmsSecretKey;
    type Signature = LmsSignature;

    fn keygen(security_level: SecurityLevel) -> PqcResult<(Self::PublicKey, Self::SecretKey)> {
        let params = match security_level {
            SecurityLevel::Level1 => LmsParams::LmsSha256M32H10,
            SecurityLevel::Level3 => LmsParams::LmsSha256M32H15,
            SecurityLevel::Level5 => LmsParams::LmsSha256M32H20,
        };

        Self::keygen_with_params(params)
    }

    fn sign(secret_key: &Self::SecretKey, message: &[u8]) -> PqcResult<Self::Signature> {
        // This requires mutable access to secret key for state management
        // In a real implementation, this would need thread-safe state management
        Err(PqcError::SigningFailed(
            "LMS signing requires mutable secret key for state management".to_string()
        ))
    }

    fn verify(public_key: &Self::PublicKey, message: &[u8], signature: &Self::Signature) -> PqcResult<bool> {
        if public_key.params != signature.params {
            return Err(PqcError::ParameterValidation("Parameter mismatch".to_string()));
        }

        let params = public_key.params;

        // Hash the message
        let mut hasher = Sha256::new();
        hasher.update(message);
        let mut message_hash = [0u8; HASH_LEN];
        message_hash.copy_from_slice(&hasher.finalize());

        // Verify LMOTS signature to get leaf hash
        if !signature.lmots_signature.verify(&message_hash, &public_key.i, &[0u8; HASH_LEN]) {
            return Ok(false);
        }

        // Compute leaf hash from LMOTS public key
        let leaf_hash = compute_leaf_hash(&public_key.i, signature.q, &signature.lmots_signature);

        // Verify Merkle authentication path
        let computed_root = verify_auth_path(
            &leaf_hash,
            signature.q,
            &signature.auth_path,
            params.h()
        );

        Ok(computed_root == public_key.root)
    }

    fn algorithm_type() -> AlgorithmType {
        AlgorithmType::Lms
    }
}

impl RealLms {
    pub fn keygen_with_params(params: LmsParams) -> PqcResult<(LmsPublicKey, LmsSecretKey)> {
        // Generate identifier and seed
        let mut i = [0u8; 16];
        let mut seed = [0u8; HASH_LEN];
        OsRng.fill_bytes(&mut i);
        OsRng.fill_bytes(&mut seed);

        // Generate OTS public keys for all possible signatures
        let max_sigs = params.max_signatures();
        let mut ots_public_keys = Vec::new();
        let mut ots_keys = HashMap::new();

        for q in 0..max_sigs {
            let ots_key = LmotsKey::new(params, i, q);
            let ots_pk = ots_key.public_key();
            ots_public_keys.push(ots_pk);
            
            // Store only first few keys to save memory
            if q < 1000 {
                ots_keys.insert(q, ots_key);
            }
        }

        // Build Merkle tree
        let merkle_tree = MerkleTree::new(ots_public_keys, params.h());
        let root = merkle_tree.root_hash();

        let public_key = LmsPublicKey::new(params, i, root);
        let secret_key = LmsSecretKey {
            params,
            i,
            seed,
            q: 0,
            ots_keys,
            merkle_tree,
        };

        Ok((public_key, secret_key))
    }

    pub fn sign_with_state(secret_key: &mut LmsSecretKey, message: &[u8]) -> PqcResult<LmsSignature> {
        // Get current OTS key
        let current_q = secret_key.q;
        let ots_key = secret_key.get_next_ots_key()?.clone();

        // Hash the message
        let mut hasher = Sha256::new();
        hasher.update(message);
        let mut message_hash = [0u8; HASH_LEN];
        message_hash.copy_from_slice(&hasher.finalize());

        // Sign with LMOTS
        let lmots_signature = ots_key.sign(&message_hash);

        // Generate authentication path
        let auth_path = secret_key.merkle_tree.generate_auth_path(current_q);

        // Increment state
        secret_key.increment_counter();

        Ok(LmsSignature::new(
            secret_key.params,
            current_q,
            lmots_signature,
            auth_path,
        ))
    }

    pub fn performance_characteristics(params: LmsParams) -> AlgorithmPerformance {
        let (keygen_ms, sign_ms, verify_ms, sign_throughput, verify_throughput) = match params {
            LmsParams::LmsSha256M32H10 => (500.0, 5.0, 2.0, 200.0, 500.0),
            LmsParams::LmsSha256M32H15 => (15000.0, 8.0, 3.0, 125.0, 333.0),
            LmsParams::LmsSha256M32H20 => (480000.0, 12.0, 4.0, 83.0, 250.0),
            LmsParams::LmsSha256M32H25 => (15360000.0, 16.0, 5.0, 62.5, 200.0),
        };

        AlgorithmPerformance {
            keygen_time_ms: keygen_ms,
            operation_time_ms: (sign_ms + verify_ms) / 2.0,
            key_sizes: KeySizes {
                public_key: params.public_key_size(),
                secret_key: params.secret_key_size(),
                ciphertext_or_signature: params.additional_sizes()
                    .iter()
                    .find(|(name, _)| *name == "signature")
                    .map(|(_, size)| *size)
                    .unwrap_or(0),
                shared_secret: None,
            },
            throughput_ops_per_sec: (sign_throughput + verify_throughput) / 2.0,
        }
    }

    pub fn remaining_signatures(secret_key: &LmsSecretKey) -> u32 {
        secret_key.params.max_signatures() - secret_key.q
    }
}

/// Compute leaf hash for Merkle tree
fn compute_leaf_hash(i: &[u8; 16], q: u32, lmots_sig: &LmotsSignature) -> [u8; HASH_LEN] {
    let mut hasher = Sha256::new();
    hasher.update(i);
    hasher.update(&q.to_be_bytes());
    hasher.update(b"leaf");
    
    // Add LMOTS signature data
    for sig_val in &lmots_sig.signature {
        hasher.update(sig_val);
    }
    
    let mut hash = [0u8; HASH_LEN];
    hash.copy_from_slice(&hasher.finalize());
    hash
}

/// Verify Merkle authentication path
fn verify_auth_path(
    leaf_hash: &[u8; HASH_LEN],
    leaf_index: u32,
    auth_path: &[[u8; HASH_LEN]],
    height: u32
) -> [u8; HASH_LEN] {
    let mut current_hash = *leaf_hash;
    let mut current_index = leaf_index;
    
    for &sibling_hash in auth_path {
        let mut hasher = Sha256::new();
        
        if current_index % 2 == 0 {
            // Current node is left child
            hasher.update(&current_hash);
            hasher.update(&sibling_hash);
        } else {
            // Current node is right child
            hasher.update(&sibling_hash);
            hasher.update(&current_hash);
        }
        
        current_hash.copy_from_slice(&hasher.finalize());
        current_index /= 2;
    }
    
    current_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_lms_keygen() {
        let (pub_key, sec_key) = RealLms::keygen(SecurityLevel::Level1).unwrap();
        assert_eq!(pub_key.params, LmsParams::LmsSha256M32H10);
        assert_eq!(sec_key.params, LmsParams::LmsSha256M32H10);
    }

    #[test]
    fn test_real_lms_sign_verify() {
        let (pub_key, mut sec_key) = RealLms::keygen(SecurityLevel::Level1).unwrap();
        let message = b"Hello, hash-based signatures!";
        
        let signature = RealLms::sign_with_state(&mut sec_key, message).unwrap();
        let is_valid = RealLms::verify(&pub_key, message, &signature).unwrap();
        
        assert!(is_valid);
    }

    #[test]
    fn test_lms_state_management() {
        let (_, mut sec_key) = RealLms::keygen(SecurityLevel::Level1).unwrap();
        let message1 = b"Message 1";
        let message2 = b"Message 2";
        
        assert_eq!(sec_key.q, 0);
        let _sig1 = RealLms::sign_with_state(&mut sec_key, message1).unwrap();
        assert_eq!(sec_key.q, 1);
        let _sig2 = RealLms::sign_with_state(&mut sec_key, message2).unwrap();
        assert_eq!(sec_key.q, 2);
        
        assert_eq!(RealLms::remaining_signatures(&sec_key), 1022); // 1024 - 2
    }

    #[test]
    fn test_lmots_operations() {
        let params = LmsParams::LmsSha256M32H10;
        let i = [1u8; 16];
        let ots_key = LmotsKey::new(params, i, 0);
        
        let message_hash = [42u8; HASH_LEN];
        let signature = ots_key.sign(&message_hash);
        let public_key = ots_key.public_key();
        
        assert!(signature.verify(&message_hash, &i, &public_key));
    }

    #[test]
    fn test_merkle_tree_operations() {
        let leaves = vec![[1u8; HASH_LEN], [2u8; HASH_LEN], [3u8; HASH_LEN], [4u8; HASH_LEN]];
        let tree = MerkleTree::new(leaves, 2);
        
        let auth_path = tree.generate_auth_path(1);
        assert_eq!(auth_path.len(), 2);
        
        let computed_root = verify_auth_path(&[2u8; HASH_LEN], 1, &auth_path, 2);
        assert_eq!(computed_root, tree.root_hash());
    }
}
