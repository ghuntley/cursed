//! Certificate Chain Validation
//! 
//! Comprehensive certificate chain validation with path building and trust verification.

use std::time::SystemTime;
use std::collections::{HashMap, HashSet};
use crate::stdlib::packages::crypto_pki::types::*;
use crate::stdlib::packages::crypto_pki::error::{PkiError, PkiResult, CertificateErrorCode};

/// Certificate chain validator with comprehensive validation logic
#[derive(Debug, Clone)]
pub struct ChainValidator {
    /// Validation policy
    policy: ValidationPolicy,
    /// Validation cache for performance
    validation_cache: HashMap<String, CachedValidationResult>,
    /// Statistics
    stats: ValidationStatistics,
}

/// Validation policy configuration
#[derive(Debug, Clone)]
pub struct ValidationPolicy {
    /// Check certificate validity dates
    pub check_validity_dates: bool,
    /// Check certificate revocation status
    pub check_revocation: bool,
    /// Maximum chain length allowed
    pub max_chain_length: u32,
    /// Require basic constraints extension
    pub require_basic_constraints: bool,
    /// Allow self-signed certificates
    pub allow_self_signed: bool,
    /// Check key usage constraints
    pub check_key_usage: bool,
    /// Check extended key usage
    pub check_extended_key_usage: bool,
    /// Check name constraints
    pub check_name_constraints: bool,
    /// Check policy constraints
    pub check_policy_constraints: bool,
    /// Allow weak signature algorithms
    pub allow_weak_signatures: bool,
    /// Minimum RSA key size
    pub min_rsa_key_size: u32,
    /// Allowed signature algorithms
    pub allowed_signature_algorithms: Vec<SignatureAlgorithm>,
    /// Trust anchor constraints
    pub trust_anchor_constraints: TrustAnchorConstraints,
}

/// Trust anchor constraints
#[derive(Debug, Clone)]
pub struct TrustAnchorConstraints {
    /// Require explicit trust anchors
    pub require_explicit_trust: bool,
    /// Allow intermediate CAs as trust anchors
    pub allow_intermediate_trust_anchors: bool,
    /// Maximum trust anchor age (days)
    pub max_trust_anchor_age_days: Option<u32>,
}

/// Validation context for certificate chain validation
#[derive(Debug)]
pub struct ValidationContext<'a> {
    /// Trust store
    pub trust_store: &'a TrustStore,
    /// Validation policy
    pub policy: &'a ValidationPolicy,
    /// Current validation time
    pub validation_time: SystemTime,
    /// Additional intermediate certificates
    pub additional_intermediates: Vec<&'a X509Certificate>,
    /// Validation depth (for recursion protection)
    pub depth: u32,
}

/// Cached validation result
#[derive(Debug, Clone)]
struct CachedValidationResult {
    /// Validation result
    result: ValidationResult,
    /// Cache timestamp
    cached_at: SystemTime,
    /// Cache TTL (time to live)
    ttl_seconds: u64,
}

/// Validation statistics
#[derive(Debug, Clone, Default)]
pub struct ValidationStatistics {
    /// Total validations performed
    pub total_validations: u64,
    /// Successful validations
    pub successful_validations: u64,
    /// Failed validations
    pub failed_validations: u64,
    /// Cache hits
    pub cache_hits: u64,
    /// Average validation time (milliseconds)
    pub avg_validation_time_ms: f64,
    /// Validation errors by type
    pub errors_by_type: HashMap<String, u64>,
}

/// Path building result
#[derive(Debug, Clone)]
pub struct PathBuildingResult {
    /// Successfully built certificate chains
    pub chains: Vec<CertificateChain>,
    /// Path building errors encountered
    pub errors: Vec<String>,
    /// Whether path building was successful
    pub success: bool,
}

impl ChainValidator {
    /// Create a new chain validator
    pub fn new(policy: ValidationPolicy) -> Self {
        Self {
            policy,
            validation_cache: HashMap::new(),
            stats: ValidationStatistics::default(),
        }
    }
    
    /// Validate a certificate chain
    pub fn validate_chain(
        &self, 
        chain: &CertificateChain, 
        context: &ValidationContext
    ) -> PkiResult<ValidationResult> {
        let start_time = SystemTime::now();
        
        // Check cache first
        let cache_key = self.generate_cache_key(chain, context);
        if let Some(cached) = self.get_cached_result(&cache_key) {
            return Ok(cached.result);
        }
        
        // Perform validation
        let mut result = ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            trust_chain: Some(chain.clone()),
            validated_at: context.validation_time,
        };
        
        // Basic chain structure validation
        self.validate_chain_structure(chain, &mut result)?;
        
        // Certificate-by-certificate validation
        self.validate_certificates_in_chain(chain, context, &mut result)?;
        
        // Trust chain validation
        self.validate_trust_chain(chain, context, &mut result)?;
        
        // Policy compliance validation
        self.validate_policy_compliance(chain, context, &mut result)?;
        
        // Set overall validity
        result.is_valid = result.errors.is_empty();
        
        // Cache the result
        self.cache_result(cache_key, &result);
        
        // Update statistics
        self.update_statistics(&result, start_time);
        
        Ok(result)
    }
    
    /// Build certificate paths from end entity to trust anchor
    pub fn build_paths(
        &self,
        end_entity: &X509Certificate,
        context: &ValidationContext
    ) -> PkiResult<PathBuildingResult> {
        let mut result = PathBuildingResult {
            chains: Vec::new(),
            errors: Vec::new(),
            success: false,
        };
        
        // Start path building from end entity
        let mut current_cert = end_entity.clone();
        let mut chain = vec![current_cert.clone()];
        let mut visited = HashSet::new();
        
        // Build paths recursively
        self.build_paths_recursive(
            &mut current_cert,
            &mut chain,
            &mut visited,
            context,
            &mut result,
            0
        )?;
        
        result.success = !result.chains.is_empty();
        Ok(result)
    }
    
    /// Validate basic chain structure
    fn validate_chain_structure(
        &self, 
        chain: &CertificateChain, 
        result: &mut ValidationResult
    ) -> PkiResult<()> {
        // Check chain length
        let total_length = 1 + chain.intermediates.len() + if chain.root.is_some() { 1 } else { 0 };
        if total_length > self.policy.max_chain_length as usize {
            result.errors.push(format!(
                "Certificate chain too long: {} certificates (max: {})",
                total_length, self.policy.max_chain_length
            ));
        }
        
        // Check end entity certificate
        if chain.end_entity.is_ca() && !self.policy.allow_self_signed {
            result.warnings.push(
                "End entity certificate has CA flag set".to_string()
            );
        }
        
        // Check intermediate certificates
        for (i, intermediate) in chain.intermediates.iter().enumerate() {
            if !intermediate.is_ca() {
                result.errors.push(format!(
                    "Intermediate certificate {} is not a CA certificate",
                    i
                ));
            }
        }
        
        // Check root certificate
        if let Some(root) = &chain.root {
            if !root.is_ca() {
                result.errors.push(
                    "Root certificate is not a CA certificate".to_string()
                );
            }
            
            // Check if root is self-signed
            if !self.is_self_signed(root) && !self.policy.allow_self_signed {
                result.warnings.push(
                    "Root certificate is not self-signed".to_string()
                );
            }
        }
        
        Ok(())
    }
    
    /// Validate individual certificates in the chain
    fn validate_certificates_in_chain(
        &self,
        chain: &CertificateChain,
        context: &ValidationContext,
        result: &mut ValidationResult
    ) -> PkiResult<()> {
        // Validate end entity certificate
        self.validate_individual_certificate(&chain.end_entity, context, result, "end entity")?;
        
        // Validate intermediate certificates
        for (i, intermediate) in chain.intermediates.iter().enumerate() {
            self.validate_individual_certificate(
                intermediate, 
                context, 
                result, 
                &format!("intermediate {}", i)
            )?;
        }
        
        // Validate root certificate
        if let Some(root) = &chain.root {
            self.validate_individual_certificate(root, context, result, "root")?;
        }
        
        Ok(())
    }
    
    /// Validate an individual certificate
    fn validate_individual_certificate(
        &self,
        cert: &X509Certificate,
        context: &ValidationContext,
        result: &mut ValidationResult,
        cert_type: &str
    ) -> PkiResult<()> {
        // Check validity dates
        if self.policy.check_validity_dates {
            self.validate_certificate_dates(cert, context, result, cert_type)?;
        }
        
        // Check signature algorithm
        self.validate_signature_algorithm(cert, result, cert_type)?;
        
        // Check key size
        self.validate_key_size(cert, result, cert_type)?;
        
        // Check extensions
        if self.policy.require_basic_constraints {
            self.validate_basic_constraints(cert, result, cert_type)?;
        }
        
        // Check key usage
        if self.policy.check_key_usage {
            self.validate_key_usage_constraints(cert, result, cert_type)?;
        }
        
        Ok(())
    }
    
    /// Validate certificate dates
    fn validate_certificate_dates(
        &self,
        cert: &X509Certificate,
        context: &ValidationContext,
        result: &mut ValidationResult,
        cert_type: &str
    ) -> PkiResult<()> {
        let now = context.validation_time;
        
        if now < cert.validity.not_before {
            result.errors.push(format!(
                "{} certificate is not yet valid (not before: {:?})",
                cert_type, cert.validity.not_before
            ));
        }
        
        if now > cert.validity.not_after {
            result.errors.push(format!(
                "{} certificate has expired (not after: {:?})",
                cert_type, cert.validity.not_after
            ));
        }
        
        Ok(())
    }
    
    /// Validate signature algorithm
    fn validate_signature_algorithm(
        &self,
        cert: &X509Certificate,
        result: &mut ValidationResult,
        cert_type: &str
    ) -> PkiResult<()> {
        if !self.policy.allowed_signature_algorithms.is_empty() {
            if !self.policy.allowed_signature_algorithms.contains(&cert.signature_algorithm) {
                result.errors.push(format!(
                    "{} certificate uses disallowed signature algorithm: {:?}",
                    cert_type, cert.signature_algorithm
                ));
            }
        }
        
        // Check for weak signature algorithms
        if !self.policy.allow_weak_signatures {
            match &cert.signature_algorithm {
                SignatureAlgorithm::Custom { name, .. } if name.contains("MD5") || name.contains("SHA1") => {
                    result.errors.push(format!(
                        "{} certificate uses weak signature algorithm: {:?}",
                        cert_type, cert.signature_algorithm
                    ));
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Validate key size constraints
    fn validate_key_size(
        &self,
        cert: &X509Certificate,
        result: &mut ValidationResult,
        cert_type: &str
    ) -> PkiResult<()> {
        match &cert.subject_public_key_info.algorithm {
            PublicKeyAlgorithm::Rsa { key_size } => {
                if *key_size < self.policy.min_rsa_key_size {
                    result.errors.push(format!(
                        "{} certificate RSA key size {} is below minimum {}",
                        cert_type, key_size, self.policy.min_rsa_key_size
                    ));
                }
            }
            _ => {
                // Other algorithms are generally considered strong enough
            }
        }
        
        Ok(())
    }
    
    /// Validate basic constraints extension
    fn validate_basic_constraints(
        &self,
        cert: &X509Certificate,
        result: &mut ValidationResult,
        cert_type: &str
    ) -> PkiResult<()> {
        let has_basic_constraints = cert.extensions.iter().any(|ext| {
            ext.oid == "2.5.29.19" // Basic Constraints OID
        });
        
        if !has_basic_constraints {
            result.errors.push(format!(
                "{} certificate missing required Basic Constraints extension",
                cert_type
            ));
        }
        
        Ok(())
    }
    
    /// Validate key usage constraints
    fn validate_key_usage_constraints(
        &self,
        cert: &X509Certificate,
        result: &mut ValidationResult,
        cert_type: &str
    ) -> PkiResult<()> {
        // Check if CA certificates have the key cert sign usage
        if cert.is_ca() && !cert.key_usage.key_cert_sign {
            result.errors.push(format!(
                "{} CA certificate missing keyCertSign usage",
                cert_type
            ));
        }
        
        // Check if CA certificates have CRL sign usage (if they issue CRLs)
        if cert.is_ca() && !cert.key_usage.crl_sign {
            result.warnings.push(format!(
                "{} CA certificate missing cRLSign usage",
                cert_type
            ));
        }
        
        Ok(())
    }
    
    /// Validate trust chain
    fn validate_trust_chain(
        &self,
        chain: &CertificateChain,
        context: &ValidationContext,
        result: &mut ValidationResult
    ) -> PkiResult<()> {
        // Check if we have a trust anchor
        let trust_anchor = if let Some(root) = &chain.root {
            if context.trust_store.is_trusted(root) {
                Some(root)
            } else {
                result.errors.push("Root certificate is not trusted".to_string());
                None
            }
        } else {
            // Try to find trust anchor in intermediates
            chain.intermediates.iter().find(|cert| {
                context.trust_store.is_trusted(cert)
            })
        };
        
        if trust_anchor.is_none() && !self.policy.allow_self_signed {
            result.errors.push("No trusted certificate found in chain".to_string());
        }
        
        // Validate signature chain
        self.validate_signature_chain(chain, result)?;
        
        Ok(())
    }
    
    /// Validate signature chain (each certificate signed by next)
    fn validate_signature_chain(
        &self,
        chain: &CertificateChain,
        result: &mut ValidationResult
    ) -> PkiResult<()> {
        let mut current = &chain.end_entity;
        
        // Check end entity signed by first intermediate (or root)
        if let Some(issuer) = chain.intermediates.first().or(chain.root.as_ref()) {
            if !self.verify_certificate_signature(current, issuer) {
                result.errors.push(
                    "End entity certificate signature verification failed".to_string()
                );
            }
            current = issuer;
        }
        
        // Check intermediate certificates
        for (i, intermediate) in chain.intermediates.iter().enumerate().skip(1) {
            if !self.verify_certificate_signature(current, intermediate) {
                result.errors.push(format!(
                    "Intermediate certificate {} signature verification failed", i
                ));
            }
            current = intermediate;
        }
        
        // Check root certificate (if present and not self-signed)
        if let Some(root) = &chain.root {
            if !self.is_self_signed(root) {
                // Root should be verified against a trust anchor
                // For now, we'll assume it's valid if it's in the trust store
            } else {
                // Self-signed root - verify self-signature
                if !self.verify_certificate_signature(root, root) {
                    result.errors.push(
                        "Root certificate self-signature verification failed".to_string()
                    );
                }
            }
        }
        
        Ok(())
    }
    
    /// Validate policy compliance
    fn validate_policy_compliance(
        &self,
        _chain: &CertificateChain,
        _context: &ValidationContext,
        _result: &mut ValidationResult
    ) -> PkiResult<()> {
        // TODO: Implement certificate policy validation
        // This would check:
        // - Certificate policies extension
        // - Policy mappings
        // - Policy constraints
        // - Name constraints
        // - Inhibit any policy
        
        Ok(())
    }
    
    /// Verify certificate signature (simplified implementation)
    fn verify_certificate_signature(&self, cert: &X509Certificate, issuer: &X509Certificate) -> bool {
        // TODO: Implement actual signature verification
        // This would:
        // 1. Extract the signature algorithm from the certificate
        // 2. Hash the TBSCertificate portion
        // 3. Verify the signature using the issuer's public key
        
        // For now, just check that the issuer matches
        cert.issuer.to_string() == issuer.subject.to_string()
    }
    
    /// Check if certificate is self-signed
    fn is_self_signed(&self, cert: &X509Certificate) -> bool {
        cert.issuer.to_string() == cert.subject.to_string()
    }
    
    /// Build certificate paths recursively
    fn build_paths_recursive(
        &self,
        current_cert: &mut X509Certificate,
        chain: &mut Vec<X509Certificate>,
        visited: &mut HashSet<String>,
        context: &ValidationContext,
        result: &mut PathBuildingResult,
        depth: u32
    ) -> PkiResult<()> {
        // Prevent infinite recursion
        if depth > self.policy.max_chain_length {
            result.errors.push("Maximum chain depth exceeded during path building".to_string());
            return Ok(());
        }
        
        // Check if we've seen this certificate before (loop detection)
        let cert_key = current_cert.fingerprint.as_ref()
            .map(|f| hex::encode(f))
            .unwrap_or_else(|| current_cert.serial_number.to_hex_string());
        
        if visited.contains(&cert_key) {
            result.errors.push("Certificate loop detected during path building".to_string());
            return Ok(());
        }
        visited.insert(cert_key.clone());
        
        // Check if current certificate is a trust anchor
        if context.trust_store.is_trusted(current_cert) {
            // Found a complete path
            let complete_chain = CertificateChain {
                end_entity: chain[0].clone(),
                intermediates: chain[1..].to_vec(),
                root: Some(current_cert.clone()),
            };
            result.chains.push(complete_chain);
            visited.remove(&cert_key);
            return Ok(());
        }
        
        // Look for issuer certificates
        let potential_issuers = self.find_potential_issuers(current_cert, context);
        
        for issuer in potential_issuers {
            chain.push(issuer.clone());
            let mut issuer_copy = issuer;
            
            self.build_paths_recursive(
                &mut issuer_copy,
                chain,
                visited,
                context,
                result,
                depth + 1
            )?;
            
            chain.pop();
        }
        
        visited.remove(&cert_key);
        Ok(())
    }
    
    /// Find potential issuer certificates
    fn find_potential_issuers(
        &self,
        cert: &X509Certificate,
        context: &ValidationContext
    ) -> Vec<X509Certificate> {
        let mut issuers = Vec::new();
        
        // Check trust store certificates
        for root_cert in &context.trust_store.root_certificates {
            if self.could_be_issuer(cert, root_cert) {
                issuers.push(root_cert.clone());
            }
        }
        
        for intermediate_cert in &context.trust_store.intermediate_certificates {
            if self.could_be_issuer(cert, intermediate_cert) {
                issuers.push(intermediate_cert.clone());
            }
        }
        
        // Check additional intermediates
        for additional_cert in &context.additional_intermediates {
            if self.could_be_issuer(cert, additional_cert) {
                issuers.push((*additional_cert).clone());
            }
        }
        
        issuers
    }
    
    /// Check if one certificate could be the issuer of another
    fn could_be_issuer(&self, cert: &X509Certificate, potential_issuer: &X509Certificate) -> bool {
        // Basic checks
        if cert.issuer.to_string() != potential_issuer.subject.to_string() {
            return false;
        }
        
        // Check if potential issuer is a CA
        if !potential_issuer.is_ca() {
            return false;
        }
        
        // Check key usage
        if !potential_issuer.key_usage.key_cert_sign {
            return false;
        }
        
        // TODO: Additional checks like:
        // - Authority Key Identifier vs Subject Key Identifier
        // - Path length constraints
        // - Name constraints
        
        true
    }
    
    /// Generate cache key for validation result
    fn generate_cache_key(&self, chain: &CertificateChain, context: &ValidationContext) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        
        // Hash chain structure
        chain.end_entity.serial_number.to_hex_string().hash(&mut hasher);
        for intermediate in &chain.intermediates {
            intermediate.serial_number.to_hex_string().hash(&mut hasher);
        }
        if let Some(root) = &chain.root {
            root.serial_number.to_hex_string().hash(&mut hasher);
        }
        
        // Hash validation time (rounded to hour for caching)
        let hour_timestamp = context.validation_time.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default().as_secs() / 3600;
        hour_timestamp.hash(&mut hasher);
        
        format!("chain_{:x}", hasher.finish())
    }
    
    /// Get cached validation result
    fn get_cached_result(&self, cache_key: &str) -> Option<&CachedValidationResult> {
        if let Some(cached) = self.validation_cache.get(cache_key) {
            let now = SystemTime::now();
            let elapsed = now.duration_since(cached.cached_at).unwrap_or_default().as_secs();
            
            if elapsed < cached.ttl_seconds {
                return Some(cached);
            }
        }
        None
    }
    
    /// Cache validation result
    fn cache_result(&self, cache_key: String, result: &ValidationResult) {
        // Implementation would use internal mutability or separate cache management
        // For now, this is a placeholder
        let _cached = CachedValidationResult {
            result: result.clone(),
            cached_at: SystemTime::now(),
            ttl_seconds: 3600, // 1 hour TTL
        };
        
        // In a real implementation:
        // self.validation_cache.insert(cache_key, cached);
    }
    
    /// Update validation statistics
    fn update_statistics(&self, result: &ValidationResult, start_time: SystemTime) {
        // Implementation would use internal mutability
        // For now, this is a placeholder
        
        let _elapsed = start_time.elapsed().unwrap_or_default().as_millis() as f64;
        
        // In a real implementation:
        // self.stats.total_validations += 1;
        // if result.is_valid { self.stats.successful_validations += 1; }
        // else { self.stats.failed_validations += 1; }
        // Update timing statistics, etc.
    }
}

impl<'a> ValidationContext<'a> {
    /// Create a new validation context
    pub fn new(trust_store: &'a TrustStore, policy: &'a ValidationPolicy) -> Self {
        Self {
            trust_store,
            policy,
            validation_time: SystemTime::now(),
            additional_intermediates: Vec::new(),
            depth: 0,
        }
    }
    
    /// Create validation context with custom time
    pub fn with_time(
        trust_store: &'a TrustStore, 
        policy: &'a ValidationPolicy, 
        validation_time: SystemTime
    ) -> Self {
        Self {
            trust_store,
            policy,
            validation_time,
            additional_intermediates: Vec::new(),
            depth: 0,
        }
    }
    
    /// Add additional intermediate certificates
    pub fn with_intermediates(mut self, intermediates: Vec<&'a X509Certificate>) -> Self {
        self.additional_intermediates = intermediates;
        self
    }
}

impl Default for ValidationPolicy {
    fn default() -> Self {
        Self {
            check_validity_dates: true,
            check_revocation: false, // Expensive, off by default
            max_chain_length: 10,
            require_basic_constraints: true,
            allow_self_signed: false,
            check_key_usage: true,
            check_extended_key_usage: false,
            check_name_constraints: false,
            check_policy_constraints: false,
            allow_weak_signatures: false,
            min_rsa_key_size: 2048,
            allowed_signature_algorithms: vec![
                SignatureAlgorithm::RsaWithSha256,
                SignatureAlgorithm::RsaWithSha384,
                SignatureAlgorithm::RsaWithSha512,
                SignatureAlgorithm::EcdsaWithSha256,
                SignatureAlgorithm::EcdsaWithSha384,
                SignatureAlgorithm::EcdsaWithSha512,
                SignatureAlgorithm::Ed25519,
                SignatureAlgorithm::Ed448,
            ],
            trust_anchor_constraints: TrustAnchorConstraints::default(),
        }
    }
}

impl Default for TrustAnchorConstraints {
    fn default() -> Self {
        Self {
            require_explicit_trust: true,
            allow_intermediate_trust_anchors: false,
            max_trust_anchor_age_days: Some(3650), // 10 years
        }
    }
}

/// Hex encoding utility module
mod hex {
    pub fn encode(data: &[u8]) -> String {
        data.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
