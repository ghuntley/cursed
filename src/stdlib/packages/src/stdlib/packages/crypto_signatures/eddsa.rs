use crate::error::{Result, CursedError};


#[derive(Debug, Clone)]
pub struct EdDsaContext {
    pub context: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct EdDsaVerificationResult {
    pub is_valid: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EdDsaBatchVerificationResult {
    pub results: Vec<EdDsaVerificationResult>,
    pub all_valid: bool,
}
