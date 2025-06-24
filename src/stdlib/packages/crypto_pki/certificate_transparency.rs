/// Certificate Transparency Support

use crate::stdlib::packages::crypto_pki::crate::types::{PkiResult, PkiError};
use crate::error::Error;

pub struct CertificateTransparency;
pub struct SignedCertificateTimestamp;
pub struct SctList;
pub struct CtLog;
pub struct CtLogList;
pub type CtError = PkiError;
pub type CtResult<T> = PkiResult<T>;

pub fn parse_scts(_data: &[u8]) -> CtResult<SctList> {
    Ok(SctList)
}

pub fn verify_sct(_sct: &SignedCertificateTimestamp, _log: &CtLog) -> CtResult<bool> {
    Ok(true)
}
