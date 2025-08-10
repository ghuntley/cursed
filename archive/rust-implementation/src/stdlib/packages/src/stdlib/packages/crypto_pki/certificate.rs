
use crate::error::{Result, CursedError};

#[derive(Debug, Clone)]
pub enum CertificateFormat {
    X509,
    Pem,
    Der,
}

#[derive(Debug, Clone)]
pub struct CertificateParser {
    pub format: CertificateFormat,
}

#[derive(Debug, Clone)]
pub struct CertificateValidator {
    pub strict_mode: bool,
}

impl CertificateParser {
    pub fn new(format: CertificateFormat) -> Self {
        Self { format }
    }
}
