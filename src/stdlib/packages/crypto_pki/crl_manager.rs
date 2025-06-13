//! Certificate Revocation List (CRL) Manager
//! 
//! Manage Certificate Revocation Lists for tracking revoked certificates.

use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use crate::stdlib::packages::crypto_pki::types::*;
use crate::stdlib::packages::crypto_pki::error::{PkiError, PkiResult};

/// CRL manager for handling certificate revocation lists
#[derive(Debug)]
pub struct CrlManager {
    /// CRL configuration
    config: CrlConfig,
    /// CRL cache
    crl_cache: HashMap<String, CachedCrl>,
}

/// CRL configuration
#[derive(Debug, Clone)]
pub struct CrlConfig {
    /// CRL validity period
    pub validity_period: Duration,
    /// Signature algorithm for CRLs
    pub signature_algorithm: SignatureAlgorithm,
    /// CRL number sequence
    pub crl_number: u64,
    /// Enable delta CRLs
    pub enable_delta_crls: bool,
}

/// Cached CRL entry
#[derive(Debug, Clone)]
struct CachedCrl {
    /// The CRL
    crl: CertificateRevocationList,
    /// Cache timestamp
    cached_at: SystemTime,
    /// CRL source URL
    source_url: Option<String>,
}

impl CrlManager {
    /// Create a new CRL manager
    pub fn new(config: CrlConfig) -> Self {
        Self {
            config,
            crl_cache: HashMap::new(),
        }
    }
    
    /// Check if a certificate is revoked
    pub fn is_certificate_revoked(&self, serial_number: &SerialNumber, issuer: &DistinguishedName) -> PkiResult<bool> {
        // Look for CRL for this issuer
        let issuer_key = issuer.to_string();
        if let Some(cached_crl) = self.crl_cache.get(&issuer_key) {
            // Check if certificate is in revoked list
            let is_revoked = cached_crl.crl.revoked_certificates.iter()
                .any(|revoked| revoked.serial_number == *serial_number);
            Ok(is_revoked)
        } else {
            // No CRL available, assume not revoked
            Ok(false)
        }
    }
    
    /// Add CRL to cache
    pub fn add_crl(&mut self, crl: CertificateRevocationList, source_url: Option<String>) -> PkiResult<()> {
        let issuer_key = crl.issuer.to_string();
        let cached_crl = CachedCrl {
            crl,
            cached_at: SystemTime::now(),
            source_url,
        };
        
        self.crl_cache.insert(issuer_key, cached_crl);
        Ok(())
    }
}

impl Default for CrlConfig {
    fn default() -> Self {
        Self {
            validity_period: Duration::from_secs(7 * 24 * 3600), // 7 days
            signature_algorithm: SignatureAlgorithm::RsaWithSha256,
            crl_number: 1,
            enable_delta_crls: false,
        }
    }
}
