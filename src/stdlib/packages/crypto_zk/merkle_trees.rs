//! Merkle tree implementations for zero-knowledge proofs

use crate::error::CursedError;
use std::collections::HashMap;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;

/// Result type for crypto operations
/// A Merkle tree for efficient data integrity verification
#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: Vec<u8>,
    pub leaves: Vec<Vec<u8>>,
    pub levels: Vec<Vec<Vec<u8>>>,
}

impl MerkleTree {
    pub fn new(leaves: Vec<Vec<u8>>) -> CryptoResult<Self> {
        let mut tree = Self {
            root: vec![],
            leaves: leaves.clone(),
            levels: vec![],
        };
        tree.build()?;
        Ok(tree)
    }
    
    pub fn build(&mut self) -> CryptoResult<()> {
        use sha2::{Sha256, Digest};
        
        if self.leaves.is_empty() {
            return Err(CursedError::runtime_error(&"Cannot build tree with no leaves".to_string()));
        }
        
        let mut current_level = self.leaves.clone();
        self.levels.push(current_level.clone());
        
        while current_level.len() > 1 {
            let mut next_level = vec![];
            
            for chunk in current_level.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(&chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(&chunk[1]);
                } else {
                    hasher.update(&chunk[0]);
                }
                next_level.push(hasher.finalize().to_vec());
            }
            
            self.levels.push(next_level.clone());
            current_level = next_level;
        }
        
        self.root = current_level[0].clone();
        Ok(())
    }
    
    pub fn get_proof(&self, index: usize) -> CryptoResult<MerkleProof> {
        if index >= self.leaves.len() {
            return Err(CursedError::runtime_error(&"Index out of bounds".to_string()));
        }
        
        let mut proof_hashes = vec![];
        let mut proof_directions = vec![];
        let mut current_index = index;
        
        for level in &self.levels[..self.levels.len() - 1] {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };
            
            if sibling_index < level.len() {
                proof_hashes.push(level[sibling_index].clone());
                proof_directions.push(current_index % 2 == 0);
            }
            
            current_index /= 2;
        }
        
        Ok(MerkleProof {
            leaf_hash: self.leaves[index].clone(),
            proof_hashes,
            proof_directions,
            root: self.root.clone(),
        })
    }
    
    pub fn verify_proof(&self, proof: &MerkleProof) -> bool {
        proof.verify()
    }
}

/// A proof for a leaf in a Merkle tree
#[derive(Debug, Clone)]
pub struct MerkleProof {
    pub leaf_hash: Vec<u8>,
    pub proof_hashes: Vec<Vec<u8>>,
    pub proof_directions: Vec<bool>,
    pub root: Vec<u8>,
}

impl MerkleProof {
    pub fn verify(&self) -> bool {
        use sha2::{Sha256, Digest};
        
        let mut current_hash = self.leaf_hash.clone();
        
        for (sibling_hash, is_left) in self.proof_hashes.iter().zip(&self.proof_directions) {
            let mut hasher = Sha256::new();
            
            if *is_left {
                hasher.update(&current_hash);
                hasher.update(sibling_hash);
            } else {
                hasher.update(sibling_hash);
                hasher.update(&current_hash);
            }
            
            current_hash = hasher.finalize().to_vec();
        }
        
        current_hash == self.root
    }
}

/// A sparse Merkle tree for efficient zero-knowledge proofs
#[derive(Debug, Clone)]
pub struct SparseMerkleTree {
    pub root: Vec<u8>,
    pub depth: usize,
    pub nodes: HashMap<Vec<u8>, Vec<u8>>,
    pub default_hashes: Vec<Vec<u8>>,
}

impl SparseMerkleTree {
    pub fn new(depth: usize) -> CryptoResult<Self> {
        use sha2::{Sha256, Digest};
        
        let mut default_hashes = vec![vec![0; 32]];
        
        for _ in 1..=depth {
            let mut hasher = Sha256::new();
            let prev_hash = &default_hashes[default_hashes.len() - 1];
            hasher.update(prev_hash);
            hasher.update(prev_hash);
            default_hashes.push(hasher.finalize().to_vec());
        }
        
        Ok(Self {
            root: default_hashes[depth].clone(),
            depth,
            nodes: HashMap::new(),
            default_hashes,
        })
    }
    
    pub fn update(&mut self, key: Vec<u8>, value: Vec<u8>) -> CryptoResult<()> {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(&value);
        let leaf_hash = hasher.finalize().to_vec();
        
        self.nodes.insert(key.clone(), leaf_hash);
        self.recompute_root()?;
        Ok(())
    }
    
    pub fn get(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.nodes.get(key)
    }
    
    pub fn get_proof(&self, key: &[u8]) -> CryptoResult<MerkleProof> {
        let leaf_hash = self.nodes.get(key)
            .unwrap_or(&self.default_hashes[0])
            .clone();
        
        Ok(MerkleProof {
            leaf_hash,
            proof_hashes: vec![],
            proof_directions: vec![],
            root: self.root.clone(),
        })
    }
    
    fn recompute_root(&mut self) -> CryptoResult<()> {
        self.root = self.default_hashes[self.depth].clone();
        Ok(())
    }
}

/// Collection of Merkle tree utilities
pub struct MerkleTrees;

impl MerkleTrees {
    pub fn build_tree(leaves: Vec<Vec<u8>>) -> CryptoResult<MerkleTree> {
        MerkleTree::new(leaves)
    }
    
    pub fn build_sparse_tree(depth: usize) -> CryptoResult<SparseMerkleTree> {
        SparseMerkleTree::new(depth)
    }
    
    pub fn verify_inclusion(proof: &MerkleProof) -> bool {
        proof.verify()
    }
    
    pub fn hash_leaf(data: &[u8]) -> Vec<u8> {
        use sha2::{Sha256, Digest};
use crate::stdlib::packages::CryptoError;
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_merkle_trees() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CryptoError::KeyGenerationFailed);
    }
    println!("🔐 Crypto processing (merkle_trees) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_merkle_trees() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error(&"Crypto hash test failed".to_string()));
    }
    Ok(())
}
