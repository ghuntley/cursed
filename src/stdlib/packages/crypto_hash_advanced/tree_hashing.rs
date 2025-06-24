/// Production-ready tree hashing implementations for Merkle trees and hash trees
use crate::error::CursedError;
use crate::stdlib::packages::crypto_hash_advanced::hash_traits::*;
use std::collections::VecDeque;

/// Result type for tree operations
pub type TreeResult<T> = std::result::Result<T, CryptoError>;

/// Merkle tree implementation
#[derive(Debug, Clone)]
pub struct MerkleTree<H: Hasher + Clone> {
    hasher: H,
    leaves: Vec<Vec<u8>>,
    tree: Vec<Vec<Vec<u8>>>, // Level 0 = leaves, Level n = root
    root_hash: Option<Vec<u8>>,
}

impl<H: Hasher + Clone> MerkleTree<H> {
    /// Create new Merkle tree
    pub fn new(hasher: H) -> Self {
        Self {
            hasher,
            leaves: Vec::new(),
            tree: Vec::new(),
            root_hash: None,
        }
    }
    
    /// Add leaf data to the tree
    pub fn add_leaf(&mut self, data: &[u8]) {
        self.leaves.push(data.to_vec());
        self.root_hash = None; // Invalidate cached root
    }
    
    /// Add multiple leaves at once
    pub fn add_leaves(&mut self, data_items: &[&[u8]]) {
        for data in data_items {
            self.leaves.push(data.to_vec());
        }
        self.root_hash = None;
    }
    
    /// Build the Merkle tree and compute root hash
    pub fn build(&mut self) -> TreeResult<Vec<u8>> {
        if self.leaves.is_empty() {
            return Err(CursedError::InvalidArgument("Cannot build tree with no leaves".to_string()));
        }
        
        self.tree.clear();
        
        // Level 0: Hash all leaves
        let mut current_level = Vec::new();
        for leaf in &self.leaves {
            current_level.push(self.hasher.clone().hash(leaf));
        }
        self.tree.push(current_level);
        
        // Build tree levels bottom-up
        let mut level_index = 0;
        while self.tree[level_index].len() > 1 {
            let current_level = &self.tree[level_index];
            let mut next_level = Vec::new();
            
            // Process pairs
            for chunk in current_level.chunks(2) {
                if chunk.len() == 2 {
                    // Hash pair
                    let combined = [&chunk[0][..], &chunk[1][..]].concat();
                    next_level.push(self.hasher.clone().hash(&combined));
                } else {
                    // Odd number - duplicate last hash (Bitcoin-style)
                    let combined = [&chunk[0][..], &chunk[0][..]].concat();
                    next_level.push(self.hasher.clone().hash(&combined));
                }
            }
            
            self.tree.push(next_level);
            level_index += 1;
        }
        
        // Cache and return root hash
        let root = self.tree.last().unwrap()[0].clone();
        self.root_hash = Some(root.clone());
        Ok(root)
    }
    
    /// Get the root hash (builds tree if necessary)
    pub fn root(&mut self) -> TreeResult<Vec<u8>> {
        if let Some(ref root) = self.root_hash {
            Ok(root.clone())
        } else {
            self.build()
        }
    }
    
    /// Generate Merkle proof for a leaf at given index
    pub fn generate_proof(&mut self, leaf_index: usize) -> TreeResult<MerkleProof> {
        if leaf_index >= self.leaves.len() {
            return Err(CursedError::InvalidArgument(
                format!("Leaf index {} out of bounds ({})", leaf_index, self.leaves.len())
            ));
        }
        
        // Ensure tree is built
        self.build()?;
        
        let mut proof_hashes = Vec::new();
        let mut proof_directions = Vec::new();
        let mut current_index = leaf_index;
        
        // Traverse from leaf to root
        for level in 0..self.tree.len() - 1 {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };
            
            let current_level = &self.tree[level];
            
            if sibling_index < current_level.len() {
                proof_hashes.push(current_level[sibling_index].clone());
                proof_directions.push(current_index % 2 == 0); // true = right sibling, false = left
            } else {
                // No sibling (odd number case) - use self
                proof_hashes.push(current_level[current_index].clone());
                proof_directions.push(false);
            }
            
            current_index /= 2;
        }
        
        Ok(MerkleProof {
            leaf_data: self.leaves[leaf_index].clone(),
            leaf_hash: self.tree[0][leaf_index].clone(),
            proof_hashes,
            proof_directions,
            root_hash: self.root_hash.clone().unwrap(),
        })
    }
    
    /// Verify a Merkle proof
    pub fn verify_proof(&self, proof: &MerkleProof) -> bool {
        let mut current_hash = self.hasher.clone().hash(&proof.leaf_data);
        
        // Check leaf hash matches
        if current_hash != proof.leaf_hash {
            return false;
        }
        
        // Traverse proof path
        for (sibling_hash, is_right_sibling) in proof.proof_hashes.iter().zip(&proof.proof_directions) {
            current_hash = if *is_right_sibling {
                // Current is left, sibling is right
                let combined = [&current_hash[..], &sibling_hash[..]].concat();
                self.hasher.clone().hash(&combined)
            } else {
                // Current is right, sibling is left
                let combined = [&sibling_hash[..], &current_hash[..]].concat();
                self.hasher.clone().hash(&combined)
            };
        }
        
        current_hash == proof.root_hash
    }
    
    /// Get tree depth
    pub fn depth(&self) -> usize {
        if self.tree.is_empty() {
            return 0;
        }
        self.tree.len() - 1
    }
    
    /// Get number of leaves
    pub fn leaf_count(&self) -> usize {
        self.leaves.len()
    }
    
    /// Update a leaf and rebuild affected path
    pub fn update_leaf(&mut self, index: usize, new_data: &[u8]) -> TreeResult<Vec<u8>> {
        if index >= self.leaves.len() {
            return Err(CursedError::InvalidArgument("Leaf index out of bounds".to_string()));
        }
        
        self.leaves[index] = new_data.to_vec();
        self.build() // Rebuild entire tree for simplicity
    }
}

/// Merkle proof for verifying leaf inclusion
#[derive(Debug, Clone)]
pub struct MerkleProof {
    pub leaf_data: Vec<u8>,
    pub leaf_hash: Vec<u8>,
    pub proof_hashes: Vec<Vec<u8>>,
    pub proof_directions: Vec<bool>, // true = right sibling, false = left sibling
    pub root_hash: Vec<u8>,
}

impl MerkleProof {
    /// Convert proof to compact binary format
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        // Leaf data length + data
        result.extend_from_slice(&(self.leaf_data.len() as u32).to_le_bytes());
        result.extend_from_slice(&self.leaf_data);
        
        // Hash size (assuming all hashes same size)
        let hash_size = self.leaf_hash.len();
        result.push(hash_size as u8);
        
        // Leaf hash
        result.extend_from_slice(&self.leaf_hash);
        
        // Proof hashes count
        result.push(self.proof_hashes.len() as u8);
        
        // Proof hashes and directions
        for (hash, direction) in self.proof_hashes.iter().zip(&self.proof_directions) {
            result.extend_from_slice(hash);
            result.push(if *direction { 1 } else { 0 });
        }
        
        // Root hash
        result.extend_from_slice(&self.root_hash);
        
        result
    }
    
    /// Parse proof from binary format
    pub fn from_bytes(data: &[u8]) -> TreeResult<Self> {
        let mut offset = 0;
        
        if data.len() < 4 {
            return Err(CursedError::InvalidArgument("Proof data too short".to_string()));
        }
        
        // Leaf data
        let leaf_len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        offset += 4;
        
        if offset + leaf_len >= data.len() {
            return Err(CursedError::InvalidArgument("Invalid leaf data length".to_string()));
        }
        
        let leaf_data = data[offset..offset + leaf_len].to_vec();
        offset += leaf_len;
        
        // Hash size
        let hash_size = data[offset] as usize;
        offset += 1;
        
        // Leaf hash
        if offset + hash_size >= data.len() {
            return Err(CursedError::InvalidArgument("Invalid leaf hash".to_string()));
        }
        
        let leaf_hash = data[offset..offset + hash_size].to_vec();
        offset += hash_size;
        
        // Proof hashes count
        let proof_count = data[offset] as usize;
        offset += 1;
        
        // Proof hashes and directions
        let mut proof_hashes = Vec::new();
        let mut proof_directions = Vec::new();
        
        for _ in 0..proof_count {
            if offset + hash_size + 1 > data.len() {
                return Err(CursedError::InvalidArgument("Invalid proof hash".to_string()));
            }
            
            proof_hashes.push(data[offset..offset + hash_size].to_vec());
            offset += hash_size;
            
            proof_directions.push(data[offset] != 0);
            offset += 1;
        }
        
        // Root hash
        if offset + hash_size != data.len() {
            return Err(CursedError::InvalidArgument("Invalid root hash".to_string()));
        }
        
        let root_hash = data[offset..offset + hash_size].to_vec();
        
        Ok(MerkleProof {
            leaf_data,
            leaf_hash,
            proof_hashes,
            proof_directions,
            root_hash,
        })
    }
}

/// Binary hash tree for efficient hash computation
#[derive(Debug, Clone)]
pub struct BinaryHashTree<H: Hasher + Clone> {
    hasher: H,
    nodes: Vec<Option<Vec<u8>>>,
    height: usize,
}

impl<H: Hasher + Clone> BinaryHashTree<H> {
    /// Create tree with specified height (2^height leaves)
    pub fn new(hasher: H, height: usize) -> Self {
        let node_count = (1 << (height + 1)) - 1; // 2^(h+1) - 1 nodes
        Self {
            hasher,
            nodes: vec![None; node_count],
            height,
        }
    }
    
    /// Set leaf value at given index
    pub fn set_leaf(&mut self, index: usize, data: &[u8]) -> TreeResult<()> {
        let max_leaves = 1 << self.height;
        if index >= max_leaves {
            return Err(CursedError::InvalidArgument(
                format!("Leaf index {} exceeds maximum {}", index, max_leaves - 1)
            ));
        }
        
        let leaf_node_index = self.leaf_node_index(index);
        self.nodes[leaf_node_index] = Some(self.hasher.clone().hash(data));
        
        // Update path to root
        self.update_path_to_root(leaf_node_index);
        
        Ok(())
    }
    
    /// Get root hash
    pub fn root(&self) -> Option<Vec<u8>> {
        self.nodes[0].clone()
    }
    
    /// Update path from leaf to root
    fn update_path_to_root(&mut self, mut node_index: usize) {
        while node_index > 0 {
            let parent_index = (node_index - 1) / 2;
            let sibling_index = if node_index % 2 == 1 {
                node_index + 1
            } else {
                node_index - 1
            };
            
            // Compute parent hash from children
            if let (Some(left), Some(right)) = (
                &self.nodes[if node_index % 2 == 1 { node_index } else { sibling_index }],
                &self.nodes[if node_index % 2 == 1 { sibling_index } else { node_index }],
            ) {
                let combined = [&left[..], &right[..]].concat();
                self.nodes[parent_index] = Some(self.hasher.clone().hash(&combined));
            } else {
                self.nodes[parent_index] = None;
            }
            
            node_index = parent_index;
        }
    }
    
    fn leaf_node_index(&self, leaf_index: usize) -> usize {
        // Leaves start at index 2^height - 1
        (1 << self.height) - 1 + leaf_index
    }
}

/// Streaming Merkle tree for large datasets
pub struct StreamingMerkleTree<H: Hasher + Clone> {
    hasher: H,
    buffer: VecDeque<Vec<u8>>,
    tree_levels: Vec<VecDeque<Vec<u8>>>,
    finalized: bool,
}

impl<H: Hasher + Clone> StreamingMerkleTree<H> {
    pub fn new(hasher: H) -> Self {
        Self {
            hasher,
            buffer: VecDeque::new(),
            tree_levels: vec![VecDeque::new()],
            finalized: false,
        }
    }
    
    /// Add data to the streaming tree
    pub fn add_data(&mut self, data: &[u8]) -> TreeResult<()> {
        if self.finalized {
            return Err(CursedError::InvalidArgument("Cannot add data to finalized tree".to_string()));
        }
        
        let leaf_hash = self.hasher.clone().hash(data);
        self.buffer.push_back(leaf_hash);
        
        self.process_buffer();
        Ok(())
    }
    
    /// Finalize tree and get root hash
    pub fn finalize(&mut self) -> TreeResult<Vec<u8>> {
        if self.finalized {
            return Err(CursedError::InvalidArgument("Tree already finalized".to_string()));
        }
        
        // Process any remaining items in buffer
        while !self.buffer.is_empty() {
            self.process_buffer();
        }
        
        // Combine all levels to get final root
        let mut current_level = 0;
        while self.tree_levels.len() > 1 {
            if self.tree_levels[current_level].len() >= 2 {
                let left = self.tree_levels[current_level].pop_front().unwrap();
                let right = self.tree_levels[current_level].pop_front().unwrap();
                
                let combined = [&left[..], &right[..]].concat();
                let parent_hash = self.hasher.clone().hash(&combined);
                
                if self.tree_levels.len() <= current_level + 1 {
                    self.tree_levels.push(VecDeque::new());
                }
                self.tree_levels[current_level + 1].push_back(parent_hash);
            } else if self.tree_levels[current_level].len() == 1 {
                // Move single item up
                let item = self.tree_levels[current_level].pop_front().unwrap();
                if self.tree_levels.len() <= current_level + 1 {
                    self.tree_levels.push(VecDeque::new());
                }
                self.tree_levels[current_level + 1].push_back(item);
                current_level += 1;
            } else {
                current_level += 1;
            }
            
            if current_level >= self.tree_levels.len() {
                break;
            }
        }
        
        self.finalized = true;
        
        // Find the root
        for level in self.tree_levels.iter().rev() {
            if !level.is_empty() {
                return Ok(level[0].clone());
            }
        }
        
        Err(CursedError::InvalidArgument("Empty tree".to_string()))
    }
    
    fn process_buffer(&mut self) {
        // Process pairs from buffer
        while self.buffer.len() >= 2 {
            let left = self.buffer.pop_front().unwrap();
            let right = self.buffer.pop_front().unwrap();
            
            let combined = [&left[..], &right[..]].concat();
            let parent_hash = self.hasher.clone().hash(&combined);
            
            self.tree_levels[0].push_back(parent_hash);
        }
        
        // Process tree levels
        let mut level = 0;
        while level < self.tree_levels.len() && self.tree_levels[level].len() >= 2 {
            let left = self.tree_levels[level].pop_front().unwrap();
            let right = self.tree_levels[level].pop_front().unwrap();
            
            let combined = [&left[..], &right[..]].concat();
            let parent_hash = self.hasher.clone().hash(&combined);
            
            if self.tree_levels.len() <= level + 1 {
                self.tree_levels.push(VecDeque::new());
            }
            self.tree_levels[level + 1].push_back(parent_hash);
            level += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::packages::crypto_hash_advanced::xxhash::XxHash64;

    #[test]
    fn test_merkle_tree_basic() {
        let hasher = XxHash64::new();
        let mut tree = MerkleTree::new(hasher);
        
        tree.add_leaf(b"leaf1");
        tree.add_leaf(b"leaf2");
        tree.add_leaf(b"leaf3");
        tree.add_leaf(b"leaf4");
        
        let root = tree.build().unwrap();
        assert_eq!(root.len(), 8); // xxHash64 produces 8 bytes
        assert_eq!(tree.leaf_count(), 4);
        assert_eq!(tree.depth(), 2); // 4 leaves -> depth 2
    }

    #[test]
    fn test_merkle_proof() {
        let hasher = XxHash64::new();
        let mut tree = MerkleTree::new(hasher.clone());
        
        tree.add_leaf(b"data1");
        tree.add_leaf(b"data2");
        tree.add_leaf(b"data3");
        tree.add_leaf(b"data4");
        
        tree.build().unwrap();
        
        // Generate and verify proof for leaf 1
        let proof = tree.generate_proof(1).unwrap();
        assert!(tree.verify_proof(&proof));
        
        // Proof should serialize/deserialize correctly
        let proof_bytes = proof.to_bytes();
        let parsed_proof = MerkleProof::from_bytes(&proof_bytes).unwrap();
        assert!(tree.verify_proof(&parsed_proof));
    }

    #[test]
    fn test_merkle_tree_update() {
        let hasher = XxHash64::new();
        let mut tree = MerkleTree::new(hasher);
        
        tree.add_leaf(b"original");
        let original_root = tree.build().unwrap();
        
        tree.update_leaf(0, b"updated").unwrap();
        let updated_root = tree.root().unwrap();
        
        assert_ne!(original_root, updated_root);
    }

    #[test]
    fn test_binary_hash_tree() {
        let hasher = XxHash64::new();
        let mut tree = BinaryHashTree::new(hasher, 2); // 4 leaves
        
        tree.set_leaf(0, b"leaf0").unwrap();
        tree.set_leaf(1, b"leaf1").unwrap();
        tree.set_leaf(2, b"leaf2").unwrap();
        tree.set_leaf(3, b"leaf3").unwrap();
        
        let root = tree.root().unwrap();
        assert_eq!(root.len(), 8);
    }

    #[test]
    fn test_streaming_merkle_tree() {
        let hasher = XxHash64::new();
        let mut tree = StreamingMerkleTree::new(hasher);
        
        tree.add_data(b"data1").unwrap();
        tree.add_data(b"data2").unwrap();
        tree.add_data(b"data3").unwrap();
        tree.add_data(b"data4").unwrap();
        
        let root = tree.finalize().unwrap();
        assert_eq!(root.len(), 8);
    }

    #[test]
    fn test_merkle_tree_empty() {
        let hasher = XxHash64::new();
        let mut tree = MerkleTree::new(hasher);
        
        assert!(tree.build().is_err());
    }

    #[test]
    fn test_merkle_proof_invalid_index() {
        let hasher = XxHash64::new();
        let mut tree = MerkleTree::new(hasher);
        
        tree.add_leaf(b"data");
        tree.build().unwrap();
        
        assert!(tree.generate_proof(1).is_err());
    }

    #[test]
    fn test_binary_tree_bounds() {
        let hasher = XxHash64::new();
        let mut tree = BinaryHashTree::new(hasher, 1); // 2 leaves
        
        assert!(tree.set_leaf(0, b"ok").is_ok());
        assert!(tree.set_leaf(1, b"ok").is_ok());
        assert!(tree.set_leaf(2, b"error").is_err());
    }
}
