
use crate::error::{Result, CursedError};

#[derive(Debug, Clone)]
pub enum CrlError {
    FetchFailed,
    ParseError,
    Expired,
}

pub type CrlResult<T> = Result<T>;

#[derive(Debug, Clone)]
pub enum RevocationStatus {
    Valid,
    Revoked,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct CrlCache {
    pub entries: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CrlValidator {
    pub cache: CrlCache,
}
