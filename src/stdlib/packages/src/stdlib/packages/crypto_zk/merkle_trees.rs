use crate::error::{Result, CursedError};


#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct MerkleProof {
    pub proof: Vec<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct SparseMerkleTree {
    pub root: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct MerkleTrees;
