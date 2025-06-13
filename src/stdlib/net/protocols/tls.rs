/// TLS/SSL configuration and utilities

/// TLS protocol versions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsVersion {
    Tls10,
    Tls11,
    Tls12,
    Tls13,
}

/// Cipher suites
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherSuite {
    TlsAes128GcmSha256,
    TlsAes256GcmSha384,
    TlsChaCha20Poly1305Sha256,
    TlsAes128CcmSha256,
    TlsAes128Ccm8Sha256,
}

/// TLS configuration
#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub min_version: TlsVersion,
    pub max_version: TlsVersion,
    pub cipher_suites: Vec<CipherSuite>,
    pub verify_certificates: bool,
    pub ca_file: Option<String>,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            min_version: TlsVersion::Tls12,
            max_version: TlsVersion::Tls13,
            cipher_suites: vec![
                CipherSuite::TlsAes256GcmSha384,
                CipherSuite::TlsAes128GcmSha256,
                CipherSuite::TlsChaCha20Poly1305Sha256,
            ],
            verify_certificates: true,
            ca_file: None,
            cert_file: None,
            key_file: None,
        }
    }
}

impl TlsConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn min_version(mut self, version: TlsVersion) -> Self {
        self.min_version = version;
        self
    }
    
    pub fn max_version(mut self, version: TlsVersion) -> Self {
        self.max_version = version;
        self
    }
    
    pub fn verify_certificates(mut self, verify: bool) -> Self {
        self.verify_certificates = verify;
        self
    }
    
    pub fn ca_file<P: Into<String>>(mut self, path: P) -> Self {
        self.ca_file = Some(path.into());
        self
    }
    
    pub fn cert_file<P: Into<String>>(mut self, path: P) -> Self {
        self.cert_file = Some(path.into());
        self
    }
    
    pub fn key_file<P: Into<String>>(mut self, path: P) -> Self {
        self.key_file = Some(path.into());
        self
    }
}

impl std::fmt::Display for TlsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TlsVersion::Tls10 => write!(f, "TLSv1.0"),
            TlsVersion::Tls11 => write!(f, "TLSv1.1"),
            TlsVersion::Tls12 => write!(f, "TLSv1.2"),
            TlsVersion::Tls13 => write!(f, "TLSv1.3"),
        }
    }
}

impl std::fmt::Display for CipherSuite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CipherSuite::TlsAes128GcmSha256 => write!(f, "TLS_AES_128_GCM_SHA256"),
            CipherSuite::TlsAes256GcmSha384 => write!(f, "TLS_AES_256_GCM_SHA384"),
            CipherSuite::TlsChaCha20Poly1305Sha256 => write!(f, "TLS_CHACHA20_POLY1305_SHA256"),
            CipherSuite::TlsAes128CcmSha256 => write!(f, "TLS_AES_128_CCM_SHA256"),
            CipherSuite::TlsAes128Ccm8Sha256 => write!(f, "TLS_AES_128_CCM_8_SHA256"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_config_default() {
        let config = TlsConfig::default();
        assert_eq!(config.min_version, TlsVersion::Tls12);
        assert_eq!(config.max_version, TlsVersion::Tls13);
        assert!(config.verify_certificates);
        assert!(!config.cipher_suites.is_empty());
    }

    #[test]
    fn test_tls_config_builder() {
        let config = TlsConfig::new()
            .min_version(TlsVersion::Tls13)
            .verify_certificates(false)
            .ca_file("/path/to/ca.pem")
            .cert_file("/path/to/cert.pem")
            .key_file("/path/to/key.pem");
        
        assert_eq!(config.min_version, TlsVersion::Tls13);
        assert!(!config.verify_certificates);
        assert_eq!(config.ca_file, Some("/path/to/ca.pem".to_string()));
        assert_eq!(config.cert_file, Some("/path/to/cert.pem".to_string()));
        assert_eq!(config.key_file, Some("/path/to/key.pem".to_string()));
    }

    #[test]
    fn test_tls_version_display() {
        assert_eq!(TlsVersion::Tls12.to_string(), "TLSv1.2");
        assert_eq!(TlsVersion::Tls13.to_string(), "TLSv1.3");
    }

    #[test]
    fn test_cipher_suite_display() {
        assert_eq!(CipherSuite::TlsAes128GcmSha256.to_string(), "TLS_AES_128_GCM_SHA256");
        assert_eq!(CipherSuite::TlsAes256GcmSha384.to_string(), "TLS_AES_256_GCM_SHA384");
    }
}
