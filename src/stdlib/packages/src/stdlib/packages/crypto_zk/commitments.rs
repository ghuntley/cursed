use crate::error::{Result, CursedError};


#[derive(Debug, Clone)]
pub struct PedersenCommitment {
    pub commitment: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct HashCommitment {
    pub commitment: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct VectorCommitment {
    pub commitment: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct KateCommitment {
    pub commitment: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Commitments;
