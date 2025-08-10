
use crate::error::{Result, CursedError};

#[derive(Debug, Clone)]
pub enum ChainError {
    InvalidChain,
    TrustStoreError,
    ValidationFailed,
}

pub type ChainResult<T> = Result<T>;

#[derive(Debug, Clone)]
pub struct ChainValidationPolicy {
    pub require_root_ca: bool,
    pub max_chain_length: u32,
}

#[derive(Debug, Clone)]
pub struct ChainConstraints {
    pub allowed_key_usage: Vec<String>,
    pub required_extensions: Vec<String>,
}
