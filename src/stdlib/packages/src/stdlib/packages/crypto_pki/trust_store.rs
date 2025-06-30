
use crate::error::{Result, CursedError};

#[derive(Debug, Clone)]
pub enum TrustStoreError {
    CertificateNotFound,
    InvalidCertificate,
    AccessDenied,
}

pub type TrustStoreResult<T> = Result<T>;

pub fn remove_trusted_certificate(cert_id: &str) -> TrustStoreResult<()> {
    Ok(())
}

pub fn verify_trust(certificate: &[u8]) -> TrustStoreResult<bool> {
    Ok(true)
}
