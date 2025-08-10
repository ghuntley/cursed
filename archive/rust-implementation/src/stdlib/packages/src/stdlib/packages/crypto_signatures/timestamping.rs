use crate::error::{Result, CursedError};


#[derive(Debug, Clone)]
pub struct TimestampValidationPolicy {
    pub require_nonce: bool,
    pub max_age_seconds: u64,
}

#[derive(Debug, Clone)]
pub struct TimestampVerificationResult {
    pub is_valid: bool,
    pub timestamp: Option<u64>,
}
