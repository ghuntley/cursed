use crate::error::{Result, CursedError};


#[derive(Debug, Clone)]
pub enum SaltLength {
    Auto,
    Fixed(usize),
    MaxLength,
}

#[derive(Debug, Clone)]
pub struct RsaPssSignature {
    pub signature: Vec<u8>,
    pub salt_length: SaltLength,
}
