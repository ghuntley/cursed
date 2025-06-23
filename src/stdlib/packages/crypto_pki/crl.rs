/// CRL Operations - Production Implementation

use crate::stdlib::packages::crypto_pki::{
    error::{PkiError, PkiResult},
    crate::types::*,
    crl_manager::{CrlManager, CrlConfig},
};

/// CRL operations wrapper
pub struct CrlOperations;

impl CrlOperations {
    /// Check if certificate is revoked via CRL
    pub fn check_revocation(
        cert: &X509Certificate,
        issuer: Option<&X509Certificate>,
    ) -> PkiResult<RevocationStatus> {
        let manager = CrlManager::new(CrlConfig::default());
        manager.check_revocation_status(cert, issuer)
    }
    
    /// Generate a new CRL
    pub fn generate_crl(
        issuer: &X509Certificate,
        revoked_certs: Vec<RevokedCertificate>,
        validity_hours: Option<u32>,
    ) -> PkiResult<CertificateRevocationList> {
        let manager = CrlManager::new(CrlConfig::default());
        manager.generate_crl(issuer, revoked_certs, validity_hours)
    }
    
    /// Create CRL manager with custom config
    pub fn create_manager(config: CrlConfig) -> CrlManager {
        CrlManager::new(config)
    }
}

/// Re-export for convenience
pub use CrlOperations as Crl;
