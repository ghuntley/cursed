/// Merkle tree implementations for zero-knowledge proofs
use std::collections::HashMap;
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::stdlib::error::CryptoError;
use crate::value::Value;
use sha3::{Digest, Sha3_256};

/// Merkle tree node
#[derive(Debug, Clone, PartialEq)]
pub struct MerkleNode {
    pub hash: Vec<u8>,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    /// Create a new leaf node
    pub fn new_leaf(data: &[u8]) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(b"leaf:");
        hasher.update(data);
        let hash = hasher.finalize().to_vec();

        Self {
            hash,
            left: None,
            right: None,
        }
    }

    /// Create a new internal node
    pub fn new_internal(left: MerkleNode, right: MerkleNode) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(b"internal:");
        hasher.update(&left.hash);
        hasher.update(&right.hash);
        let hash = hasher.finalize().to_vec();

        Self {
            hash,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    /// Check if node is a leaf
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    /// Get node height
    pub fn height(&self) -> usize {
        if self.is_leaf() {
            0
        } else {
            let left_height = self.left.as_ref().map_or(0, |n| n.height());
            let right_height = self.right.as_ref().map_or(0, |n| n.height());
            1 + left_height.max(right_height)
        }
    }
}

/// Merkle proof element
#[derive(Debug, Clone)]
pub struct MerkleProofElement {
    pub hash: Vec<u8>,
    pub is_right: bool,
}

/// Merkle proof
#[derive(Debug, Clone)]
pub struct MerkleProof {
    pub leaf_index: usize,
    pub proof_elements: Vec<MerkleProofElement>,
    pub root_hash: Vec<u8>,
}

impl MerkleProof {
    /// Verify the Merkle proof
    pub fn verify(&self, leaf_data: &[u8]) -> bool {
        let mut current_hash = {
            let mut hasher = Sha3_256::new();
            hasher.update(b"leaf:");
            hasher.update(leaf_data);
            hasher.finalize().to_vec()
        };

        for element in &self.proof_elements {
            let mut hasher = Sha3_256::new();
            hasher.update(b"internal:");
            
            if element.is_right {
                hasher.update(&current_hash);
                hasher.update(&element.hash);
            } else {
                hasher.update(&element.hash);
                hasher.update(&current_hash);
            }
            
            current_hash = hasher.finalize().to_vec();
        }

        current_hash == self.root_hash
    }

    /// Convert to CURSED Value representation
    pub fn to_value(&self) -> Value {
        let mut proof_map = HashMap::new();
        proof_map.insert("leaf_index".to_string(), Value::Integer(self.leaf_index as i64));
        proof_map.insert("root_hash".to_string(), Value::String(hex::encode(&self.root_hash)));
        
        let proof_elements: Vec<Value> = self.proof_elements.iter().map(|elem| {
            let mut elem_map = HashMap::new();
            elem_map.insert("hash".to_string(), Value::String(hex::encode(&elem.hash)));
            elem_map.insert("is_right".to_string(), Value::Boolean(elem.is_right));
            Value::Object(elem_map)
        }).collect();
        
        proof_map.insert("proof_elements".to_string(), Value::Array(proof_elements));
        Value::Object(proof_map)
    }

    /// Create from CURSED Value representation
    pub fn from_value(value: &Value) -> AdvancedCryptoResult<Self> {
        let obj = match value {
            Value::Object(map) => map,
            _ => return Err(CryptoError::InvalidInput("Expected object for Merkle proof".to_string())),
        };

        let leaf_index = match obj.get("leaf_index") {
            Some(Value::Integer(i)) => *i as usize,
            _ => return Err(CryptoError::InvalidInput("Invalid leaf_index".to_string())),
        };

        let root_hash = match obj.get("root_hash") {
            Some(Value::String(s)) => hex::decode(s)
                .map_err(|_| CryptoError::InvalidInput("Invalid root_hash hex".to_string()))?,
            _ => return Err(CryptoError::InvalidInput("Invalid root_hash".to_string())),
        };

        let proof_elements = match obj.get("proof_elements") {
            Some(Value::Array(arr)) => {
                let mut elements = Vec::new();
                for elem_val in arr {
                    if let Value::Object(elem_map) = elem_val {
                        let hash = match elem_map.get("hash") {
                            Some(Value::String(s)) => hex::decode(s)
                                .map_err(|_| CryptoError::InvalidInput("Invalid proof element hash".to_string()))?,
                            _ => return Err(CryptoError::InvalidInput("Invalid proof element hash".to_string())),
                        };
                        
                        let is_right = match elem_map.get("is_right") {
                            Some(Value::Boolean(b)) => *b,
                            _ => return Err(CryptoError::InvalidInput("Invalid is_right value".to_string())),
                        };
                        
                        elements.push(MerkleProofElement { hash, is_right });
                    } else {
                        return Err(CryptoError::InvalidInput("Invalid proof element format".to_string()));
                    }
                }
                elements
            }
            _ => return Err(CryptoError::InvalidInput("Invalid proof_elements".to_string())),
        };

        Ok(Self {
            leaf_index,
            proof_elements,
            root_hash,
        })
    }
}

/// Merkle tree implementation
#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: MerkleNode,
    pub leaves: Vec<Vec<u8>>,
}

impl MerkleTree {
    /// Create a new Merkle tree from data
    pub fn new(data: Vec<Vec<u8>>) -> AdvancedCryptoResult<Self> {
        if data.is_empty() {
            return Err(CryptoError::InvalidInput("Cannot create tree from empty data".to_string()));
        }

        let leaves = data.clone();
        
        // Create leaf nodes
        let mut nodes: Vec<MerkleNode> = data.iter()
            .map(|d| MerkleNode::new_leaf(d))
            .collect();

        // Build tree bottom-up
        while nodes.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in nodes.chunks(2) {
                if chunk.len() == 2 {
                    let left = chunk[0].clone();
                    let right = chunk[1].clone();
                    next_level.push(MerkleNode::new_internal(left, right));
                } else {
                    // Odd number of nodes - duplicate the last one
                    let left = chunk[0].clone();
                    let right = chunk[0].clone();
                    next_level.push(MerkleNode::new_internal(left, right));
                }
            }
            
            nodes = next_level;
        }

        Ok(Self {
            root: nodes.into_iter().next().unwrap(),
            leaves,
        })
    }

    /// Get the root hash
    pub fn root_hash(&self) -> Vec<u8> {
        self.root.hash.clone()
    }

    /// Generate a Merkle proof for a leaf at the given index
    pub fn generate_proof(&self, index: usize) -> AdvancedCryptoResult<MerkleProof> {
        if index >= self.leaves.len() {
            return Err(CryptoError::InvalidInput("Index out of bounds".to_string()));
        }

        let mut proof_elements = Vec::new();
        let mut current_index = index;
        let mut nodes = self.create_level_nodes();

        // Work our way up the tree
        for level in 0..nodes.len() - 1 {
            let level_nodes = &nodes[level];
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < level_nodes.len() {
                let sibling_hash = level_nodes[sibling_index].hash.clone();
                let is_right = current_index % 2 == 0;
                proof_elements.push(MerkleProofElement {
                    hash: sibling_hash,
                    is_right,
                });
            }

            current_index /= 2;
        }

        Ok(MerkleProof {
            leaf_index: index,
            proof_elements,
            root_hash: self.root_hash(),
        })
    }

    /// Create all level nodes for proof generation
    fn create_level_nodes(&self) -> Vec<Vec<MerkleNode>> {
        let mut all_levels = Vec::new();
        
        // Start with leaf nodes
        let mut current_level: Vec<MerkleNode> = self.leaves.iter()
            .map(|d| MerkleNode::new_leaf(d))
            .collect();
        
        all_levels.push(current_level.clone());

        // Build all levels
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in current_level.chunks(2) {
                if chunk.len() == 2 {
                    let left = chunk[0].clone();
                    let right = chunk[1].clone();
                    next_level.push(MerkleNode::new_internal(left, right));
                } else {
                    let left = chunk[0].clone();
                    let right = chunk[0].clone();
                    next_level.push(MerkleNode::new_internal(left, right));
                }
            }
            
            all_levels.push(next_level.clone());
            current_level = next_level;
        }

        all_levels
    }

    /// Verify that a piece of data is in the tree
    pub fn verify_inclusion(&self, data: &[u8], proof: &MerkleProof) -> bool {
        proof.verify(data) && proof.root_hash == self.root_hash()
    }

    /// Get tree height
    pub fn height(&self) -> usize {
        self.root.height()
    }

    /// Get number of leaves
    pub fn leaf_count(&self) -> usize {
        self.leaves.len()
    }
}

/// Sparse Merkle tree for more efficient proofs
#[derive(Debug, Clone)]
pub struct SparseMerkleTree {
    root_hash: Vec<u8>,
    nodes: HashMap<Vec<u8>, Vec<u8>>,
    depth: usize,
}

impl SparseMerkleTree {
    /// Create a new sparse Merkle tree
    pub fn new(depth: usize) -> Self {
        let empty_hash = Self::empty_hash();
        Self {
            root_hash: empty_hash,
            nodes: HashMap::new(),
            depth,
        }
    }

    /// Empty hash for sparse tree
    fn empty_hash() -> Vec<u8> {
        vec![0u8; 32]
    }

    /// Hash two nodes
    fn hash_nodes(left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().to_vec()
    }

    /// Set a value at a specific index
    pub fn set(&mut self, index: u64, value: &[u8]) -> AdvancedCryptoResult<()> {
        if index >= (1 << self.depth) {
            return Err(CryptoError::InvalidInput("Index exceeds tree capacity".to_string()));
        }

        let leaf_hash = {
            let mut hasher = Sha3_256::new();
            hasher.update(value);
            hasher.finalize().to_vec()
        };

        self.update_path(index, leaf_hash)?;
        Ok(())
    }

    /// Update the path from leaf to root
    fn update_path(&mut self, index: u64, leaf_hash: Vec<u8>) -> AdvancedCryptoResult<()> {
        let mut current_hash = leaf_hash;
        let mut current_index = index;

        for level in 0..self.depth {
            let is_right = (current_index & 1) == 1;
            let sibling_index = current_index ^ 1;
            
            let sibling_hash = self.get_hash_at_index(sibling_index, self.depth - level - 1);
            
            let parent_hash = if is_right {
                Self::hash_nodes(&sibling_hash, &current_hash)
            } else {
                Self::hash_nodes(&current_hash, &sibling_hash)
            };

            current_index >>= 1;
            current_hash = parent_hash;
        }

        self.root_hash = current_hash;
        Ok(())
    }

    /// Get hash at specific index and level
    fn get_hash_at_index(&self, index: u64, level: usize) -> Vec<u8> {
        let key = format!("{}:{}", level, index);
        self.nodes.get(key.as_bytes())
            .cloned()
            .unwrap_or_else(Self::empty_hash)
    }

    /// Generate inclusion proof
    pub fn generate_proof(&self, index: u64) -> AdvancedCryptoResult<Vec<Vec<u8>>> {
        if index >= (1 << self.depth) {
            return Err(CryptoError::InvalidInput("Index exceeds tree capacity".to_string()));
        }

        let mut proof = Vec::new();
        let mut current_index = index;

        for level in 0..self.depth {
            let sibling_index = current_index ^ 1;
            let sibling_hash = self.get_hash_at_index(sibling_index, level);
            proof.push(sibling_hash);
            current_index >>= 1;
        }

        Ok(proof)
    }

    /// Verify inclusion proof
    pub fn verify_proof(&self, index: u64, value: &[u8], proof: &[Vec<u8>]) -> bool {
        if proof.len() != self.depth {
            return false;
        }

        let mut current_hash = {
            let mut hasher = Sha3_256::new();
            hasher.update(value);
            hasher.finalize().to_vec()
        };

        let mut current_index = index;

        for sibling_hash in proof {
            let is_right = (current_index & 1) == 1;
            
            current_hash = if is_right {
                Self::hash_nodes(sibling_hash, &current_hash)
            } else {
                Self::hash_nodes(&current_hash, sibling_hash)
            };

            current_index >>= 1;
        }

        current_hash == self.root_hash
    }

    /// Get root hash
    pub fn root_hash(&self) -> Vec<u8> {
        self.root_hash.clone()
    }
}

/// Public API for CURSED language
pub struct MerkleTrees;

impl MerkleTrees {
    /// Create a new Merkle tree
    pub fn create_tree(data: &Value) -> AdvancedCryptoResult<Value> {
        let data_vec = match data {
            Value::Array(arr) => {
                let mut result = Vec::new();
                for item in arr {
                    match item {
                        Value::String(s) => result.push(s.as_bytes().to_vec()),
                        Value::Array(bytes_arr) => {
                            let mut bytes = Vec::new();
                            for byte_val in bytes_arr {
                                if let Value::Integer(b) = byte_val {
                                    bytes.push(*b as u8);
                                }
                            }
                            result.push(bytes);
                        }
                        _ => return Err(CryptoError::InvalidInput("Invalid data format".to_string())),
                    }
                }
                result
            }
            _ => return Err(CryptoError::InvalidInput("Expected array of data".to_string())),
        };

        let tree = MerkleTree::new(data_vec)?;
        let mut tree_map = HashMap::new();
        tree_map.insert("root_hash".to_string(), Value::String(hex::encode(tree.root_hash())));
        tree_map.insert("leaf_count".to_string(), Value::Integer(tree.leaf_count() as i64));
        tree_map.insert("height".to_string(), Value::Integer(tree.height() as i64));
        
        Ok(Value::Object(tree_map))
    }

    /// Generate Merkle proof
    pub fn generate_proof(tree_data: &Value, index: i64) -> AdvancedCryptoResult<Value> {
        let data_vec = Self::extract_data_from_tree_value(tree_data)?;
        let tree = MerkleTree::new(data_vec)?;
        let proof = tree.generate_proof(index as usize)?;
        Ok(proof.to_value())
    }

    /// Verify Merkle proof
    pub fn verify_proof(data: &Value, proof: &Value) -> AdvancedCryptoResult<Value> {
        let data_bytes = match data {
            Value::String(s) => s.as_bytes().to_vec(),
            Value::Array(arr) => {
                let mut bytes = Vec::new();
                for byte_val in arr {
                    if let Value::Integer(b) = byte_val {
                        bytes.push(*b as u8);
                    }
                }
                bytes
            }
            _ => return Err(CryptoError::InvalidInput("Invalid data format".to_string())),
        };

        let merkle_proof = MerkleProof::from_value(proof)?;
        let is_valid = merkle_proof.verify(&data_bytes);
        Ok(Value::Boolean(is_valid))
    }

    /// Create sparse Merkle tree
    pub fn create_sparse_tree(depth: i64) -> AdvancedCryptoResult<Value> {
        let tree = SparseMerkleTree::new(depth as usize);
        let mut tree_map = HashMap::new();
        tree_map.insert("root_hash".to_string(), Value::String(hex::encode(tree.root_hash())));
        tree_map.insert("depth".to_string(), Value::Integer(depth));
        
        Ok(Value::Object(tree_map))
    }

    /// Helper to extract data from tree value representation
    fn extract_data_from_tree_value(tree_data: &Value) -> AdvancedCryptoResult<Vec<Vec<u8>>> {
        match tree_data {
            Value::Array(arr) => {
                let mut result = Vec::new();
                for item in arr {
                    match item {
                        Value::String(s) => result.push(s.as_bytes().to_vec()),
                        Value::Array(bytes_arr) => {
                            let mut bytes = Vec::new();
                            for byte_val in bytes_arr {
                                if let Value::Integer(b) = byte_val {
                                    bytes.push(*b as u8);
                                }
                            }
                            result.push(bytes);
                        }
                        _ => return Err(CryptoError::InvalidInput("Invalid data format".to_string())),
                    }
                }
                Ok(result)
            }
            _ => Err(CryptoError::InvalidInput("Expected array of data for tree".to_string())),
        }
    }

    /// Get root of empty tree
    pub fn empty_root() -> Value {
        Value::String(hex::encode(SparseMerkleTree::empty_hash()))
    }

    /// Hash two values
    pub fn hash_pair(left: &Value, right: &Value) -> AdvancedCryptoResult<Value> {
        let left_bytes = Self::value_to_bytes(left)?;
        let right_bytes = Self::value_to_bytes(right)?;
        
        let hash = SparseMerkleTree::hash_nodes(&left_bytes, &right_bytes);
        Ok(Value::String(hex::encode(hash)))
    }

    /// Convert value to bytes
    fn value_to_bytes(value: &Value) -> AdvancedCryptoResult<Vec<u8>> {
        match value {
            Value::String(s) => {
                if s.starts_with("0x") {
                    hex::decode(&s[2..])
                        .map_err(|_| CryptoError::InvalidInput("Invalid hex string".to_string()))
                } else {
                    Ok(s.as_bytes().to_vec())
                }
            }
            Value::Array(arr) => {
                let mut bytes = Vec::new();
                for byte_val in arr {
                    if let Value::Integer(b) = byte_val {
                        bytes.push(*b as u8);
                    }
                }
                Ok(bytes)
            }
            _ => Err(CryptoError::InvalidInput("Cannot convert value to bytes".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree_creation() {
        let data = vec![
            b"data1".to_vec(),
            b"data2".to_vec(),
            b"data3".to_vec(),
            b"data4".to_vec(),
        ];

        let tree = MerkleTree::new(data).unwrap();
        assert!(!tree.root_hash().is_empty());
        assert_eq!(tree.leaf_count(), 4);
    }

    #[test]
    fn test_merkle_proof_generation_and_verification() {
        let data = vec![
            b"data1".to_vec(),
            b"data2".to_vec(),
            b"data3".to_vec(),
            b"data4".to_vec(),
        ];

        let tree = MerkleTree::new(data.clone()).unwrap();
        let proof = tree.generate_proof(1).unwrap();
        
        assert!(proof.verify(b"data2"));
        assert!(tree.verify_inclusion(b"data2", &proof));
    }

    #[test]
    fn test_sparse_merkle_tree() {
        let mut tree = SparseMerkleTree::new(8);
        tree.set(5, b"test_data").unwrap();
        
        let proof = tree.generate_proof(5).unwrap();
        assert!(tree.verify_proof(5, b"test_data", &proof));
    }

    #[test]
    fn test_merkle_trees_api() {
        let data = Value::Array(vec![
            Value::String("data1".to_string()),
            Value::String("data2".to_string()),
            Value::String("data3".to_string()),
        ]);

        let tree = MerkleTrees::create_tree(&data).unwrap();
        assert!(matches!(tree, Value::Object(_)));

        let proof = MerkleTrees::generate_proof(&data, 1).unwrap();
        assert!(matches!(proof, Value::Object(_)));
    }
}
