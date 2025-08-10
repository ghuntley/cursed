
use crate::error::{Result, CursedError};

#[derive(Debug, Clone)]
pub enum CaError {
    InvalidRequest,
    KeyGenerationFailed,
    SigningFailed,
}

pub type CaResult<T> = Result<T>;

#[derive(Debug, Clone)]
pub enum CaStatus {
    Active,
    Revoked,
    Suspended,
}

#[derive(Debug, Clone)]
pub struct CaMetadata {
    pub issuer: String,
    pub serial_number: String,
    pub status: CaStatus,
}
