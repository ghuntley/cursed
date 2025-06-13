/// OCSP Operations - Production Implementation

use crate::stdlib::packages::crypto_pki::{
    error::{PkiError, PkiResult},
    types::*,
    ocsp_client::{OcspClient, OcspConfig},
};

/// OCSP operations wrapper
pub struct OcspOperations;

impl OcspOperations {
    /// Check certificate status via OCSP
    pub fn check_status(
        cert: &X509Certificate,
        issuer: &X509Certificate,
        responder_url: Option<&str>,
    ) -> PkiResult<RevocationStatus> {
        let client = OcspClient::new(OcspConfig::default());
        let status_info = client.check_certificate_status(cert, issuer, responder_url)?;
        Ok(status_info.status)
    }
    
    /// Create OCSP client with custom config
    pub fn create_client(config: OcspConfig) -> OcspClient {
        OcspClient::new(config)
    }
}

/// Re-export for convenience
pub use OcspOperations as Ocsp;
