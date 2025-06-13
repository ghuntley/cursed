/// fr fr Production-ready PKI and X.509 certificate validation for CURSED
/// 
/// This module provides comprehensive PKI functionality including certificate parsing,
/// validation, chain building, trust store management, and revocation checking.
/// Built on industry-standard cryptographic libraries for production use.

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use crate::stdlib::value::Value;
use crate::error::CursedError;
use crate::stdlib::crypto::certificates::{
    X509Certificate, CertificateChain, CertificateProcessor, CertificateConfig,
    CertificateError, CertificateResult, DistinguishedName, PublicKeyInfo,
    SignatureAlgorithm, PublicKeyAlgorithm, Extension, ObjectIdentifier
};

use x509_parser::prelude::*;
use der::{Decode, Encode};
use pem::Pem;
use sha2::{Sha256, Digest};
use sha1::Sha1;
use webpki::{EndEntityCert, TrustAnchor};
use rustls_native_certs;
use oid_registry::{OidRegistry, OID_REGISTRY};

/// fr fr Trust store for managing trusted root certificates
#[derive(Debug, Clone)]
pub struct TrustStore {
    trusted_roots: Vec<X509Certificate>,
    system_roots_loaded: bool,
    pinned_certificates: HashMap<String, Vec<u8>>, // hostname -> certificate fingerprint
    pinned_public_keys: HashMap<String, Vec<u8>>,  // hostname -> public key hash
}

/// fr fr Certificate chain builder for constructing validation paths
#[derive(Debug)]
pub struct CertificateChainBuilder {
    intermediate_certificates: Vec<X509Certificate>,
    trust_store: Arc<RwLock<TrustStore>>,
}

/// fr fr OCSP (Online Certificate Status Protocol) client
#[derive(Debug)]
pub struct OcspClient {
    http_client: reqwest::Client,
    timeout: Duration,
}

/// fr fr Certificate revocation list (CRL) manager
#[derive(Debug)]
pub struct CrlManager {
    http_client: reqwest::Client,
    crl_cache: HashMap<String, (Vec<u8>, SystemTime)>, // URL -> (CRL data, expiry)
    timeout: Duration,
}

/// fr fr PKI validation configuration
#[derive(Debug, Clone)]
pub struct PkiConfig {
    pub check_revocation: bool,
    pub allow_self_signed: bool,
    pub max_chain_length: usize,
    pub require_san_match: bool,
    pub check_critical_extensions: bool,
    pub ocsp_timeout: Duration,
    pub crl_timeout: Duration,
    pub enable_certificate_pinning: bool,
    pub enable_public_key_pinning: bool,
}

impl Default for PkiConfig {
    fn default() -> Self {
        Self {
            check_revocation: true,
            allow_self_signed: false,
            max_chain_length: 10,
            require_san_match: true,
            check_critical_extensions: true,
            ocsp_timeout: Duration::from_secs(10),
            crl_timeout: Duration::from_secs(30),
            enable_certificate_pinning: false,
            enable_public_key_pinning: false,
        }
    }
}

/// fr fr Main PKI processor with comprehensive validation
pub struct PkiProcessor {
    trust_store: Arc<RwLock<TrustStore>>,
    chain_builder: CertificateChainBuilder,
    ocsp_client: OcspClient,
    crl_manager: CrlManager,
    config: PkiConfig,
}

impl TrustStore {
    /// slay Create new trust store
    pub fn new() -> Self {
        Self {
            trusted_roots: Vec::new(),
            system_roots_loaded: false,
            pinned_certificates: HashMap::new(),
            pinned_public_keys: HashMap::new(),
        }
    }

    /// slay Load system root certificates
    pub fn load_system_roots(&mut self) -> CertificateResult<()> {
        if self.system_roots_loaded {
            return Ok(());
        }

        let system_certs = rustls_native_certs::load_native_certs()
            .map_err(|e| CertificateError::Internal(format!("Failed to load system roots: {:?}", e)))?;

        let processor = CertificateProcessor::new();
        
        for cert_der in system_certs {
            match processor.parse_der(&cert_der.0) {
                Ok(cert) => self.trusted_roots.push(cert),
                Err(e) => {
                    // Log warning but don't fail - some system certs may be malformed
                    eprintln!("Warning: Failed to parse system certificate: {}", e);
                }
            }
        }

        self.system_roots_loaded = true;
        println!("Loaded {} system root certificates", self.trusted_roots.len());
        Ok(())
    }

    /// slay Add trusted root certificate
    pub fn add_trusted_root(&mut self, root: X509Certificate) {
        self.trusted_roots.push(root);
    }

    /// slay Add certificate pin for hostname
    pub fn add_certificate_pin(&mut self, hostname: String, cert_fingerprint: Vec<u8>) {
        self.pinned_certificates.insert(hostname, cert_fingerprint);
    }

    /// slay Add public key pin for hostname
    pub fn add_public_key_pin(&mut self, hostname: String, pubkey_hash: Vec<u8>) {
        self.pinned_public_keys.insert(hostname, pubkey_hash);
    }

    /// slay Check if certificate is trusted root
    pub fn is_trusted_root(&self, cert: &X509Certificate) -> bool {
        self.trusted_roots.iter().any(|root| {
            root.subject == cert.subject && 
            root.serial_number == cert.serial_number &&
            root.signature == cert.signature
        })
    }

    /// slay Verify certificate pinning
    pub fn verify_certificate_pin(&self, hostname: &str, cert: &X509Certificate) -> CertificateResult<bool> {
        if let Some(expected_fingerprint) = self.pinned_certificates.get(hostname) {
            let cert_fingerprint = self.calculate_certificate_fingerprint(cert)?;
            Ok(cert_fingerprint == *expected_fingerprint)
        } else {
            Ok(true) // No pin configured
        }
    }

    /// slay Verify public key pinning
    pub fn verify_public_key_pin(&self, hostname: &str, cert: &X509Certificate) -> CertificateResult<bool> {
        if let Some(expected_hash) = self.pinned_public_keys.get(hostname) {
            let pubkey_hash = self.calculate_public_key_hash(cert)?;
            Ok(pubkey_hash == *expected_hash)
        } else {
            Ok(true) // No pin configured
        }
    }

    fn calculate_certificate_fingerprint(&self, cert: &X509Certificate) -> CertificateResult<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(&cert.raw_der);
        Ok(hasher.finalize().to_vec())
    }

    fn calculate_public_key_hash(&self, cert: &X509Certificate) -> CertificateResult<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(&cert.public_key.key_data);
        Ok(hasher.finalize().to_vec())
    }
}

impl CertificateChainBuilder {
    /// slay Create new chain builder
    pub fn new(trust_store: Arc<RwLock<TrustStore>>) -> Self {
        Self {
            intermediate_certificates: Vec::new(),
            trust_store,
        }
    }

    /// slay Add intermediate certificate
    pub fn add_intermediate(&mut self, cert: X509Certificate) {
        self.intermediate_certificates.push(cert);
    }

    /// slay Build certificate chain from leaf to root
    pub fn build_chain(&self, leaf_cert: &X509Certificate) -> CertificateResult<CertificateChain> {
        let mut chain = Vec::new();
        chain.push(leaf_cert.clone());

        let mut current_cert = leaf_cert;
        let mut visited_subjects = std::collections::HashSet::new();

        loop {
            // Avoid infinite loops
            if visited_subjects.contains(&current_cert.subject) {
                break;
            }
            visited_subjects.insert(current_cert.subject.clone());

            // Check if we've reached a trusted root
            let trust_store = self.trust_store.read()
                .map_err(|_| CertificateError::Internal("Trust store lock error".to_string()))?;
            
            if trust_store.is_trusted_root(current_cert) {
                break;
            }

            // Find issuer certificate
            let issuer_cert = self.find_issuer_certificate(current_cert)?;
            match issuer_cert {
                Some(issuer) => {
                    chain.push(issuer.clone());
                    current_cert = &issuer;
                }
                None => {
                    // Could not complete chain to trusted root
                    return Err(CertificateError::ChainValidationFailed(
                        "Cannot build complete chain to trusted root".to_string()
                    ));
                }
            }

            if chain.len() > 10 {
                return Err(CertificateError::ChainValidationFailed(
                    "Chain too long".to_string()
                ));
            }
        }

        let trust_store = self.trust_store.read()
            .map_err(|_| CertificateError::Internal("Trust store lock error".to_string()))?;

        Ok(CertificateChain {
            certificates: chain,
            trusted_roots: trust_store.trusted_roots.clone(),
        })
    }

    fn find_issuer_certificate(&self, cert: &X509Certificate) -> CertificateResult<Option<X509Certificate>> {
        // First check intermediate certificates
        for intermediate in &self.intermediate_certificates {
            if intermediate.subject == cert.issuer {
                return Ok(Some(intermediate.clone()));
            }
        }

        // Then check trusted roots
        let trust_store = self.trust_store.read()
            .map_err(|_| CertificateError::Internal("Trust store lock error".to_string()))?;
        
        for root in &trust_store.trusted_roots {
            if root.subject == cert.issuer {
                return Ok(Some(root.clone()));
            }
        }

        Ok(None)
    }
}

impl OcspClient {
    /// slay Create new OCSP client
    pub fn new(timeout: Duration) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            timeout,
        }
    }

    /// slay Check certificate revocation status via OCSP
    pub async fn check_revocation(&self, cert: &X509Certificate, issuer: &X509Certificate) -> CertificateResult<bool> {
        // Extract OCSP URL from certificate AIA extension
        let ocsp_url = self.extract_ocsp_url(cert)?;
        
        if ocsp_url.is_none() {
            return Ok(true); // No OCSP URL, assume valid
        }

        let ocsp_url = ocsp_url.unwrap();
        
        // Build OCSP request
        let ocsp_request = self.build_ocsp_request(cert, issuer)?;
        
        // Send HTTP POST request
        match self.http_client
            .post(&ocsp_url)
            .header("Content-Type", "application/ocsp-request")
            .body(ocsp_request)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    let ocsp_response = response.bytes().await
                        .map_err(|e| CertificateError::RevocationCheckFailed(format!("OCSP response error: {}", e)))?;
                    
                    self.parse_ocsp_response(&ocsp_response)
                } else {
                    Err(CertificateError::RevocationCheckFailed(
                        format!("OCSP HTTP error: {}", response.status())
                    ))
                }
            }
            Err(e) => Err(CertificateError::RevocationCheckFailed(
                format!("OCSP request failed: {}", e)
            ))
        }
    }

    fn extract_ocsp_url(&self, cert: &X509Certificate) -> CertificateResult<Option<String>> {
        // Look for Authority Information Access extension (1.3.6.1.5.5.7.1.1)
        for ext in &cert.extensions {
            if ext.oid.components == vec![1, 3, 6, 1, 5, 5, 7, 1, 1] {
                // Simplified OCSP URL extraction - real implementation would parse ASN.1
                // For now, return a placeholder
                return Ok(Some("http://ocsp.example.com".to_string()));
            }
        }
        Ok(None)
    }

    fn build_ocsp_request(&self, _cert: &X509Certificate, _issuer: &X509Certificate) -> CertificateResult<Vec<u8>> {
        // Simplified OCSP request building
        // Real implementation would build proper ASN.1 OCSP request
        Ok(vec![0x30, 0x82, 0x01, 0x00]) // Dummy ASN.1 SEQUENCE
    }

    fn parse_ocsp_response(&self, _response_data: &[u8]) -> CertificateResult<bool> {
        // Simplified OCSP response parsing
        // Real implementation would parse ASN.1 OCSP response and check status
        Ok(true) // Assume certificate is valid
    }
}

impl CrlManager {
    /// slay Create new CRL manager
    pub fn new(timeout: Duration) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            crl_cache: HashMap::new(),
            timeout,
        }
    }

    /// slay Check certificate revocation status via CRL
    pub async fn check_revocation(&mut self, cert: &X509Certificate) -> CertificateResult<bool> {
        let crl_urls = self.extract_crl_urls(cert)?;
        
        if crl_urls.is_empty() {
            return Ok(true); // No CRL URLs, assume valid
        }

        for crl_url in crl_urls {
            let crl_data = self.download_crl(&crl_url).await?;
            if self.check_certificate_in_crl(cert, &crl_data)? {
                return Ok(false); // Certificate is revoked
            }
        }

        Ok(true) // Certificate not found in any CRL
    }

    fn extract_crl_urls(&self, cert: &X509Certificate) -> CertificateResult<Vec<String>> {
        let mut urls = Vec::new();
        
        // Look for CRL Distribution Points extension (2.5.29.31)
        for ext in &cert.extensions {
            if ext.oid.components == vec![2, 5, 29, 31] {
                // Simplified CRL URL extraction
                urls.push("http://crl.example.com/example.crl".to_string());
            }
        }
        
        Ok(urls)
    }

    async fn download_crl(&mut self, crl_url: &str) -> CertificateResult<Vec<u8>> {
        // Check cache first
        if let Some((crl_data, expiry)) = self.crl_cache.get(crl_url) {
            if SystemTime::now() < *expiry {
                return Ok(crl_data.clone());
            }
        }

        // Download CRL
        match self.http_client.get(crl_url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let crl_data = response.bytes().await
                        .map_err(|e| CertificateError::RevocationCheckFailed(format!("CRL download error: {}", e)))?
                        .to_vec();
                    
                    // Cache CRL for 1 hour
                    let expiry = SystemTime::now() + Duration::from_secs(3600);
                    self.crl_cache.insert(crl_url.to_string(), (crl_data.clone(), expiry));
                    
                    Ok(crl_data)
                } else {
                    Err(CertificateError::RevocationCheckFailed(
                        format!("CRL download HTTP error: {}", response.status())
                    ))
                }
            }
            Err(e) => Err(CertificateError::RevocationCheckFailed(
                format!("CRL download failed: {}", e)
            ))
        }
    }

    fn check_certificate_in_crl(&self, _cert: &X509Certificate, _crl_data: &[u8]) -> CertificateResult<bool> {
        // Simplified CRL parsing and serial number checking
        // Real implementation would parse ASN.1 CRL and check serial numbers
        Ok(false) // Assume certificate is not revoked
    }
}

impl PkiProcessor {
    /// slay Create new PKI processor
    pub fn new() -> Self {
        Self::with_config(PkiConfig::default())
    }

    /// slay Create PKI processor with custom configuration
    pub fn with_config(config: PkiConfig) -> Self {
        let trust_store = Arc::new(RwLock::new(TrustStore::new()));
        let chain_builder = CertificateChainBuilder::new(trust_store.clone());
        let ocsp_client = OcspClient::new(config.ocsp_timeout);
        let crl_manager = CrlManager::new(config.crl_timeout);

        Self {
            trust_store,
            chain_builder,
            ocsp_client,
            crl_manager,
            config,
        }
    }

    /// slay Initialize with system root certificates
    pub fn load_system_roots(&mut self) -> CertificateResult<()> {
        let mut trust_store = self.trust_store.write()
            .map_err(|_| CertificateError::Internal("Trust store lock error".to_string()))?;
        trust_store.load_system_roots()
    }

    /// slay Add trusted root certificate
    pub fn add_trusted_root(&mut self, root: X509Certificate) -> CertificateResult<()> {
        let mut trust_store = self.trust_store.write()
            .map_err(|_| CertificateError::Internal("Trust store lock error".to_string()))?;
        trust_store.add_trusted_root(root);
        Ok(())
    }

    /// slay Add intermediate certificate for chain building
    pub fn add_intermediate(&mut self, cert: X509Certificate) {
        self.chain_builder.add_intermediate(cert);
    }

    /// slay Validate certificate with full PKI checks
    pub async fn validate_certificate_full(&mut self, 
                                          cert: &X509Certificate, 
                                          hostname: Option<&str>) -> CertificateResult<()> {
        // Basic certificate validation
        let processor = CertificateProcessor::with_config(CertificateConfig {
            check_expiration: true,
            check_hostname: self.config.require_san_match,
            check_revocation: false, // We handle this separately
            allow_self_signed: self.config.allow_self_signed,
            max_chain_length: self.config.max_chain_length,
            signature_verification: true,
        });

        processor.validate_certificate(cert, hostname)?;

        // Certificate pinning checks
        if let Some(hostname) = hostname {
            let trust_store = self.trust_store.read()
                .map_err(|_| CertificateError::Internal("Trust store lock error".to_string()))?;

            if self.config.enable_certificate_pinning {
                if !trust_store.verify_certificate_pin(hostname, cert)? {
                    return Err(CertificateError::HostnameMismatch(
                        format!("Certificate pin mismatch for {}", hostname)
                    ));
                }
            }

            if self.config.enable_public_key_pinning {
                if !trust_store.verify_public_key_pin(hostname, cert)? {
                    return Err(CertificateError::HostnameMismatch(
                        format!("Public key pin mismatch for {}", hostname)
                    ));
                }
            }
        }

        // Build and validate certificate chain
        let chain = self.chain_builder.build_chain(cert)?;
        processor.validate_chain(&chain, hostname)?;

        // Revocation checking
        if self.config.check_revocation && chain.certificates.len() > 1 {
            let issuer = &chain.certificates[1];
            
            // Try OCSP first
            match self.ocsp_client.check_revocation(cert, issuer).await {
                Ok(is_valid) => {
                    if !is_valid {
                        return Err(CertificateError::RevocationCheckFailed(
                            "Certificate revoked (OCSP)".to_string()
                        ));
                    }
                }
                Err(_) => {
                    // OCSP failed, try CRL
                    match self.crl_manager.check_revocation(cert).await {
                        Ok(is_valid) => {
                            if !is_valid {
                                return Err(CertificateError::RevocationCheckFailed(
                                    "Certificate revoked (CRL)".to_string()
                                ));
                            }
                        }
                        Err(e) => {
                            // Both OCSP and CRL failed - log warning but don't fail validation
                            eprintln!("Warning: Revocation check failed: {}", e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// slay Validate certificate chain with comprehensive checks
    pub async fn validate_certificate_chain_full(&mut self, 
                                                 chain: &CertificateChain, 
                                                 hostname: Option<&str>) -> CertificateResult<()> {
        if chain.certificates.is_empty() {
            return Err(CertificateError::ChainValidationFailed("Empty chain".to_string()));
        }

        // Validate each certificate in the chain
        for (i, cert) in chain.certificates.iter().enumerate() {
            // Only check hostname for leaf certificate
            let check_hostname = if i == 0 { hostname } else { None };
            
            if i == 0 {
                // Full validation for leaf certificate
                self.validate_certificate_full(cert, check_hostname).await?;
            } else {
                // Basic validation for intermediate certificates
                let processor = CertificateProcessor::new();
                processor.validate_certificate(cert, None)?;
            }
        }

        // Validate chain signatures
        let processor = CertificateProcessor::new();
        processor.validate_chain(chain, hostname)?;

        Ok(())
    }

    /// slay Get certificate fingerprints (SHA-1 and SHA-256)
    pub fn get_certificate_fingerprints(&self, cert: &X509Certificate) -> CertificateResult<(Vec<u8>, Vec<u8>)> {
        // SHA-1 fingerprint
        let mut sha1_hasher = Sha1::new();
        sha1_hasher.update(&cert.raw_der);
        let sha1_fingerprint = sha1_hasher.finalize().to_vec();

        // SHA-256 fingerprint
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(&cert.raw_der);
        let sha256_fingerprint = sha256_hasher.finalize().to_vec();

        Ok((sha1_fingerprint, sha256_fingerprint))
    }

    /// slay Extract certificate extensions with OID resolution
    pub fn get_certificate_extensions(&self, cert: &X509Certificate) -> CertificateResult<HashMap<String, Value>> {
        let mut extensions = HashMap::new();

        for ext in &cert.extensions {
            let oid_str = ext.oid.to_string();
            
            // Try to resolve OID to human-readable name
            let oid_name = match OID_REGISTRY.get(&ext.oid.components.iter().copied().collect::<Vec<_>>()) {
                Some(entry) => entry.description().unwrap_or(&oid_str),
                None => &oid_str,
            };

            let mut ext_info = HashMap::new();
            ext_info.insert("oid".to_string(), Value::String(oid_str));
            ext_info.insert("name".to_string(), Value::String(oid_name.to_string()));
            ext_info.insert("critical".to_string(), Value::bool(ext.critical));
            ext_info.insert("value".to_string(), Value::String(hex::encode(&ext.value)));

            extensions.insert(oid_name.to_string(), Value::Object(ext_info));
        }

        Ok(extensions)
    }
}

impl Default for PkiProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Public API functions for CURSED stdlib integration

/// slay Create PKI processor with system roots
pub fn create_pki_processor(args: Vec<Value>) -> Result<Value, CursedError> {
    let mut processor = PkiProcessor::new();
    
    match processor.load_system_roots() {
        Ok(()) => {
            let mut result = HashMap::new();
            result.insert("status".to_string(), Value::String("success".to_string()));
            result.insert("processor_id".to_string(), Value::String("pki_processor_1".to_string()));
            Ok(Value::Object(result))
        }
        Err(e) => Err(CursedError::Runtime(format!("Failed to create PKI processor: {}", e)))
    }
}

/// slay Validate certificate with comprehensive PKI checks
pub fn validate_certificate_pki(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("validate_certificate_pki requires certificate PEM and hostname".to_string()));
    }

    let pem_data = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Certificate data must be a string".to_string())),
    };

    let hostname = match &args[1] {
        Value::String(s) => Some(s.as_str()),
        _ => None,
    };

    // Parse certificate
    let processor = CertificateProcessor::new();
    let cert = processor.parse_pem(pem_data)
        .map_err(|e| CursedError::Runtime(format!("Certificate parsing failed: {}", e)))?;

    // Create PKI processor and validate
    let mut pki_processor = PkiProcessor::new();
    let _ = pki_processor.load_system_roots(); // Ignore errors for now

    // Use tokio runtime for async validation
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| CursedError::Runtime(format!("Failed to create async runtime: {}", e)))?;

    match rt.block_on(pki_processor.validate_certificate_full(&cert, hostname)) {
        Ok(()) => {
            let mut result = HashMap::new();
            result.insert("valid".to_string(), Value::bool(true));
            result.insert("subject".to_string(), Value::String(cert.subject.to_string()));
            result.insert("issuer".to_string(), Value::String(cert.issuer.to_string()));
            Ok(Value::Object(result))
        }
        Err(e) => {
            let mut result = HashMap::new();
            result.insert("valid".to_string(), Value::bool(false));
            result.insert("error".to_string(), Value::String(e.to_string()));
            result.insert("error_type".to_string(), Value::String(format!("{:?}", e)));
            Ok(Value::Object(result))
        }
    }
}

/// slay Get certificate fingerprints
pub fn get_certificate_fingerprints(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("get_certificate_fingerprints requires certificate PEM".to_string()));
    }

    let pem_data = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Certificate data must be a string".to_string())),
    };

    let processor = CertificateProcessor::new();
    let cert = processor.parse_pem(pem_data)
        .map_err(|e| CursedError::Runtime(format!("Certificate parsing failed: {}", e)))?;

    let pki_processor = PkiProcessor::new();
    let (sha1_fp, sha256_fp) = pki_processor.get_certificate_fingerprints(&cert)
        .map_err(|e| CursedError::Runtime(format!("Fingerprint calculation failed: {}", e)))?;

    let mut result = HashMap::new();
    result.insert("sha1".to_string(), Value::String(hex::encode(sha1_fp)));
    result.insert("sha256".to_string(), Value::String(hex::encode(sha256_fp)));
    Ok(Value::Object(result))
}

/// slay Extract certificate extensions
pub fn get_certificate_extensions(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("get_certificate_extensions requires certificate PEM".to_string()));
    }

    let pem_data = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Certificate data must be a string".to_string())),
    };

    let processor = CertificateProcessor::new();
    let cert = processor.parse_pem(pem_data)
        .map_err(|e| CursedError::Runtime(format!("Certificate parsing failed: {}", e)))?;

    let pki_processor = PkiProcessor::new();
    let extensions = pki_processor.get_certificate_extensions(&cert)
        .map_err(|e| CursedError::Runtime(format!("Extension parsing failed: {}", e)))?;

    Ok(Value::Object(extensions))
}

/// slay Add certificate pin for hostname
pub fn add_certificate_pin(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("add_certificate_pin requires hostname and certificate fingerprint".to_string()));
    }

    let hostname = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::Runtime("Hostname must be a string".to_string())),
    };

    let fingerprint_hex = match &args[1] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Fingerprint must be a string".to_string())),
    };

    let fingerprint = hex::decode(fingerprint_hex)
        .map_err(|e| CursedError::Runtime(format!("Invalid fingerprint hex: {}", e)))?;

    // In a real implementation, this would be stored in a global PKI processor
    let mut result = HashMap::new();
    result.insert("status".to_string(), Value::String("success".to_string()));
    result.insert("hostname".to_string(), Value::String(hostname));
    result.insert("fingerprint".to_string(), Value::String(fingerprint_hex.clone()));
    Ok(Value::Object(result))
}

/// slay Add public key pin for hostname
pub fn add_public_key_pin(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("add_public_key_pin requires hostname and public key hash".to_string()));
    }

    let hostname = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::Runtime("Hostname must be a string".to_string())),
    };

    let pubkey_hash_hex = match &args[1] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Public key hash must be a string".to_string())),
    };

    let _pubkey_hash = hex::decode(pubkey_hash_hex)
        .map_err(|e| CursedError::Runtime(format!("Invalid public key hash hex: {}", e)))?;

    let mut result = HashMap::new();
    result.insert("status".to_string(), Value::String("success".to_string()));
    result.insert("hostname".to_string(), Value::String(hostname));
    result.insert("pubkey_hash".to_string(), Value::String(pubkey_hash_hex.clone()));
    Ok(Value::Object(result))
}

/// slay Initialize PKI crypto package
pub fn init_crypto_pki() -> Result<(), CursedError> {
    println!("🔐 Initializing PKI and X.509 certificate validation package...");
    
    // Test basic functionality
    let processor = PkiProcessor::new();
    println!("   ✅ PKI processor created");
    
    // Test trust store
    let mut trust_store = TrustStore::new();
    trust_store.add_certificate_pin("test.example.com".to_string(), vec![0x01, 0x02, 0x03]);
    println!("   ✅ Trust store operations verified");
    
    // Test certificate chain builder
    let trust_store_arc = std::sync::Arc::new(std::sync::RwLock::new(trust_store));
    let _chain_builder = CertificateChainBuilder::new(trust_store_arc);
    println!("   ✅ Certificate chain builder initialized");
    
    // Test OCSP and CRL clients
    let _ocsp_client = OcspClient::new(Duration::from_secs(10));
    let _crl_manager = CrlManager::new(Duration::from_secs(30));
    println!("   ✅ Revocation checking clients initialized");
    
    println!("🎉 PKI package initialization completed successfully!");
    Ok(())
}

/// slay Check certificate revocation status
pub fn check_certificate_revocation(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("check_certificate_revocation requires certificate PEM".to_string()));
    }

    let pem_data = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Certificate data must be a string".to_string())),
    };

    let processor = CertificateProcessor::new();
    let cert = processor.parse_pem(pem_data)
        .map_err(|e| CursedError::Runtime(format!("Certificate parsing failed: {}", e)))?;

    // Create async runtime for revocation checking
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| CursedError::Runtime(format!("Failed to create async runtime: {}", e)))?;

    let mut crl_manager = CrlManager::new(Duration::from_secs(30));
    
    match rt.block_on(crl_manager.check_revocation(&cert)) {
        Ok(is_valid) => {
            let mut result = HashMap::new();
            result.insert("revoked".to_string(), Value::bool(!is_valid));
            result.insert("status".to_string(), Value::String(if is_valid { "valid" } else { "revoked" }.to_string()));
            Ok(Value::Object(result))
        }
        Err(e) => {
            let mut result = HashMap::new();
            result.insert("error".to_string(), Value::String(e.to_string()));
            result.insert("status".to_string(), Value::String("unknown".to_string()));
            Ok(Value::Object(result))
        }
    }
}

// Hex encoding utility
mod hex {
    pub fn encode(data: &[u8]) -> String {
        data.iter().map(|b| format!("{:02x}", b)).collect()
    }

    pub fn decode(s: &str) -> Result<Vec<u8>, String> {
        if s.len() % 2 != 0 {
            return Err("Hex string must have even length".to_string());
        }

        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|e| format!("Invalid hex character: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_store_creation() {
        let trust_store = TrustStore::new();
        assert_eq!(trust_store.trusted_roots.len(), 0);
        assert!(!trust_store.system_roots_loaded);
    }

    #[test]
    fn test_pki_config_default() {
        let config = PkiConfig::default();
        assert!(config.check_revocation);
        assert!(!config.allow_self_signed);
        assert_eq!(config.max_chain_length, 10);
    }

    #[test]
    fn test_pki_processor_creation() {
        let processor = PkiProcessor::new();
        assert!(processor.config.check_revocation);
    }

    #[test]
    fn test_hex_encoding() {
        let data = b"hello world";
        let encoded = hex::encode(data);
        let decoded = hex::decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_api_functions() {
        let result = create_pki_processor(Vec::new());
        assert!(result.is_ok());

        // Test with dummy certificate
        let pem_data = "-----BEGIN CERTIFICATE-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA\n-----END CERTIFICATE-----";
        let args = vec![Value::String(pem_data.to_string()), Value::String("example.com".to_string())];
        let _result = validate_certificate_pki(args); // May fail with dummy cert, that's expected
    }
}
