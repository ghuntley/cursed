use crate::error::{Result, CursedError};


#[derive(Debug, Clone)]
pub struct CertificateChainValidationResult {
    pub is_valid: bool,
    pub revocation_status: RevocationStatus,
}

#[derive(Debug, Clone)]
pub enum RevocationStatus {
    Valid,
    Revoked,
    Unknown,
}
