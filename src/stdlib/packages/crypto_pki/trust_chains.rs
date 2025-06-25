// Trust Chain Building and Validation System
// 
// Comprehensive trust chain building and validation infrastructure for certificate path
// discovery, trust store management, and complete chain validation.

// Placeholder imports disabled
    types::{
        KeyUsage, ExtendedKeyUsage, BasicConstraints
    }
// };
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{SystemTime, Duration};
use std::sync::{Arc, Mutex, RwLock};
use std::path::Path;

/// Trust chain builder for constructing certificate paths
pub struct TrustChainBuilder {
    /// Trust store manager
    /// Certificate cache for efficient lookups
    /// Chain building configuration
    /// Statistics tracking
/// Configuration for trust chain building
#[derive(Debug, Clone)]
pub struct ChainBuildingConfig {
    /// Maximum chain length to consider
    /// Maximum time to spend building chains (seconds)
    /// Enable certificate caching
    /// Allow cross-certification between CAs
    /// Prefer shorter chains over longer ones
    /// Maximum number of intermediate certificates to fetch
    /// Network timeout for certificate fetching
/// Certificate path representing a complete certification chain
#[derive(Debug, Clone)]
pub struct CertificatePath {
    /// Certificates in the path (leaf to root)
    /// Trust anchor used for validation
    /// Path validation status
    /// Path building metadata
/// Path validation status
#[derive(Debug, Clone)]
pub enum PathValidationStatus {
    /// Path is valid and trusted
    /// Path is invalid with specific errors
    /// Path validation is pending
    /// Path could not be validated (missing information)
/// Path building metadata
#[derive(Debug, Clone)]
pub struct PathMetadata {
    /// Time taken to build the path
    /// Number of certificates fetched during building
    /// Whether the path uses cross-certification
    /// Path building algorithm used
/// Path building algorithms
#[derive(Debug, Clone)]
pub enum PathBuildingAlgorithm {
    /// Forward chaining (leaf to root)
    /// Reverse chaining (root to leaf)
    /// Bidirectional chaining
/// Enhanced trust store with advanced management capabilities
pub struct EnhancedTrustStore {
    /// Core trust store
    /// Certificate pinning registry
    /// Trust overrides (hostname -> certificate)
    /// Inheritance rules for trust delegation
    /// Trust anchor management
    /// Store configuration
/// Trust inheritance rule
#[derive(Debug, Clone)]
pub struct TrustInheritanceRule {
    /// Source trust anchor
    /// Target certificate subject pattern
    /// Inheritance conditions
    /// Rule priority
/// Inheritance condition
#[derive(Debug, Clone)]
pub enum InheritanceCondition {
    /// Subject DN matches pattern
    /// Issuer DN matches pattern
    /// Key usage includes specific usage
    /// Extended key usage includes specific usage
    /// Certificate validity period
/// Trust anchor with metadata
#[derive(Debug, Clone)]
pub struct TrustAnchor {
    /// Root certificate
    /// Trust constraints
    /// Last validation time
    /// Trust level
/// Trust constraints applied to trust anchors
#[derive(Debug, Clone)]
pub struct TrustConstraints {
    /// Permitted subtrees for name constraints
    /// Excluded subtrees for name constraints
    /// Maximum path length
    /// Required policy identifiers
    /// Permitted key usage flags
/// Trust level classification
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    /// Maximum trust (system root CAs)
    /// High trust (manually added trusted CAs)
    /// Conditional trust (with constraints)
    /// Temporary trust (time-limited)
/// Enhanced trust store configuration
#[derive(Debug, Clone)]
pub struct EnhancedTrustStoreConfig {
    /// Base trust store configuration
    /// Enable certificate pinning
    /// Enable trust overrides
    /// Maximum number of pinned certificates per hostname
    /// Trust inheritance configuration
/// Trust inheritance configuration
#[derive(Debug, Clone)]
pub struct TrustInheritanceConfig {
    /// Enable trust inheritance
    /// Maximum inheritance depth
    /// Default inheritance conditions
/// Complete chain validator with advanced validation logic
pub struct ComprehensiveChainValidator {
    /// Validation configuration
    /// Cache for validation results
    /// Revocation checker
    /// Policy engine
/// Chain validation configuration
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Check signature validity
    /// Check certificate validity periods
    /// Check certificate revocation status
    /// Check name constraints
    /// Check policy constraints
    /// Check basic constraints
    /// Check key usage constraints
    /// Maximum acceptable clock skew (seconds)
    /// Cache validation results
    /// Cache TTL for validation results (seconds)
/// Cached validation result
#[derive(Debug, Clone)]
pub struct CachedValidationResult {
    /// Validation result
    /// Cache timestamp
    /// Cache TTL
/// Revocation checker interface
pub trait RevocationChecker: Send + Sync {
    /// Check if a certificate is revoked
    fn is_revoked(&self, certificate: &X509Certificate) -> PkiResult<bool>;
    
    /// Get revocation reason if certificate is revoked
    fn get_revocation_reason(&self, certificate: &X509Certificate) -> PkiResult<Option<RevocationReason>>;
/// Revocation reason codes
#[derive(Debug, Clone)]
pub enum RevocationReason {
/// Policy engine for certificate policies
pub struct PolicyEngine {
    /// Policy mappings
    /// Policy constraints
    /// Initial policy set
/// Policy mapping
#[derive(Debug, Clone)]
pub struct PolicyMapping {
    /// Source policy OID
    /// Target policy OID
    /// Mapping conditions
/// Policy constraint
#[derive(Debug, Clone)]
pub struct PolicyConstraint {
    /// Policy OID
    /// Constraint type
    /// Constraint value
/// Policy constraint types
#[derive(Debug, Clone)]
pub enum PolicyConstraintType {
/// Chain building statistics
#[derive(Debug, Clone, Default)]
pub struct ChainBuildingStatistics {
    /// Total chains built
    /// Successful chain builds
    /// Failed chain builds
    /// Average build time (milliseconds)
    /// Total certificates fetched
    /// Cache hit ratio
    /// Cross-certification usage
impl TrustChainBuilder {
    /// Create a new trust chain builder
    pub fn new(trust_store: TrustStore, config: ChainBuildingConfig) -> Self {
        Self {
        }
    }
    
    /// Build certificate chain from leaf certificate to trust anchor
    pub fn build_chain(&self, leaf_certificate: &X509Certificate) -> PkiResult<Vec<CertificatePath>> {
        let start_time = SystemTime::now();
        
        // Track statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.chains_built += 1;
        let paths = match self.config.prefer_shorter_chains {
        
        // Update statistics
        let build_time = start_time.elapsed().unwrap_or(Duration::ZERO).as_millis() as u64;
        {
            let mut stats = self.statistics.lock().unwrap();
            if !paths.is_empty() {
                stats.successful_builds += 1;
            } else {
                stats.failed_builds += 1;
            // Update average build time
            let total_builds = stats.successful_builds + stats.failed_builds;
            stats.average_build_time_ms = 
                (stats.average_build_time_ms * (total_builds - 1) as f64 + build_time as f64) / total_builds as f64;
        Ok(paths)
    /// Build shortest certificate paths
    fn build_shortest_paths(&self, leaf_certificate: &X509Certificate) -> PkiResult<Vec<CertificatePath>> {
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Start with the leaf certificate
        queue.push_back((vec![leaf_certificate.clone()], 0));
        
        while let Some((current_path, depth)) = queue.pop_front() {
            if depth >= self.config.max_chain_length {
                continue;
            let current_cert = current_path.last().unwrap();
            let cert_key = self.get_certificate_key(current_cert);
            
            if visited.contains(&cert_key) {
                continue;
            }
            visited.insert(cert_key);
            
            // Check if we've reached a trust anchor
            if self.is_trust_anchor(current_cert)? {
                let path = CertificatePath {
                    metadata: PathMetadata {
                        build_time_ms: 0, // Will be set later
                paths.push(path);
                continue;
            // Find potential issuers
            let issuers = self.find_potential_issuers(current_cert)?;
            
            for issuer in issuers {
                let mut new_path = current_path.clone();
                new_path.push(issuer);
                queue.push_back((new_path, depth + 1));
            }
        }
        
        Ok(paths)
    /// Build all possible certificate paths
    fn build_all_paths(&self, leaf_certificate: &X509Certificate) -> PkiResult<Vec<CertificatePath>> {
        let mut all_paths = Vec::new();
        let mut visited = HashSet::new();
        
        self.build_paths_recursive(
            &mut visited
        )?;
        
        Ok(all_paths)
    /// Recursive path building
    fn build_paths_recursive(
        visited: &mut HashSet<String>
    ) -> PkiResult<()> {
        if depth >= self.config.max_chain_length {
            return Ok(());
        let current_cert = current_path.last().unwrap();
        let cert_key = self.get_certificate_key(current_cert);
        
        if visited.contains(&cert_key) {
            return Ok(());
        }
        visited.insert(cert_key.clone());
        
        // Check if we've reached a trust anchor
        if self.is_trust_anchor(current_cert)? {
            let path = CertificatePath {
                metadata: PathMetadata {
            all_paths.push(path);
            visited.remove(&cert_key);
            return Ok(());
        // Find potential issuers
        let issuers = self.find_potential_issuers(current_cert)?;
        
        for issuer in issuers {
            let mut new_path = current_path.clone();
            new_path.push(issuer);
            self.build_paths_recursive(new_path, depth + 1, all_paths, visited)?;
        visited.remove(&cert_key);
        Ok(())
    /// Find potential issuer certificates
    fn find_potential_issuers(&self, certificate: &X509Certificate) -> PkiResult<Vec<X509Certificate>> {
        let mut issuers = Vec::new();
        
        // Check trust store for issuers
        let trust_store = self.trust_store.read().unwrap();
        
        // Look for certificates with matching subject DN to certificate's issuer DN
        for cert in trust_store.get_all_certificates() {
            if self.is_potential_issuer(&cert, certificate) {
                issuers.push(cert);
            }
        }
        
        // If configured, try to fetch intermediate certificates
        if self.config.max_intermediate_fetch > 0 {
            let fetched_intermediates = self.fetch_intermediate_certificates(certificate)?;
            issuers.extend(fetched_intermediates);
        Ok(issuers)
    /// Check if a certificate is a potential issuer for another certificate
    fn is_potential_issuer(&self, potential_issuer: &X509Certificate, certificate: &X509Certificate) -> bool {
        // Basic checks
        if potential_issuer.subject != certificate.issuer {
            return false;
        // Check if potential issuer is a CA
        if !self.is_certificate_authority(potential_issuer) {
            return false;
        // Check key usage
        if !potential_issuer.key_usage.key_cert_sign {
            return false;
        // Check validity period (issuer should be valid when issued certificate was signed)
        // This is a simplified check - real implementation would check signature timestamp
        if potential_issuer.validity.not_after < certificate.validity.not_before {
            return false;
        true
    /// Check if certificate is a Certificate Authority
    fn is_certificate_authority(&self, certificate: &X509Certificate) -> bool {
        // Check basic constraints extension
        for extension in &certificate.extensions {
            if extension.oid == "2.5.29.19" { // Basic Constraints OID
                if let Some(ref data) = extension.parsed_data {
//                     if let crate::stdlib::packages::crypto_pki::types::ExtensionData::BasicConstraints { is_ca, .. } = data {
                        return *is_ca;
                    }
                }
            }
        }
        false
    /// Fetch intermediate certificates from network
    fn fetch_intermediate_certificates(&self, certificate: &X509Certificate) -> PkiResult<Vec<X509Certificate>> {
        // This would implement actual certificate fetching from Authority Information Access
        // For now, return empty vector
        Ok(Vec::new())
    /// Check if certificate is a trust anchor
    fn is_trust_anchor(&self, certificate: &X509Certificate) -> PkiResult<bool> {
        let trust_store = self.trust_store.read().unwrap();
        Ok(trust_store.contains_certificate(certificate))
    /// Generate unique key for certificate (for deduplication)
    fn get_certificate_key(&self, certificate: &X509Certificate) -> String {
            hex::encode(&certificate.serial_number.bytes)
        )
    /// Convert Distinguished Name to string
    fn dn_to_string(&self, dn: &DistinguishedName) -> String {
        let mut parts = Vec::new();
        
        if let Some(ref cn) = dn.common_name {
            parts.push(format!("CN={}", cn));
        }
        if let Some(ref o) = dn.organization {
            parts.push(format!("O={}", o));
        }
        if let Some(ref ou) = dn.organizational_unit {
            parts.push(format!("OU={}", ou));
        }
        if let Some(ref c) = dn.country {
            parts.push(format!("C={}", c));
        parts.join(", ")
    /// Get chain building statistics
    pub fn get_statistics(&self) -> ChainBuildingStatistics {
        self.statistics.lock().unwrap().clone()
    }
}

impl EnhancedTrustStore {
    /// Create a new enhanced trust store
    pub fn new(config: EnhancedTrustStoreConfig) -> Self {
        Self {
        }
    }
    
    /// Add certificate pinning for a hostname
    pub fn pin_certificate(&mut self, hostname: String, certificate: X509Certificate) -> PkiResult<()> {
        if !self.config.enable_pinning {
            return Err(PkiError::general("Certificate pinning is disabled"));
        let pinned = self.pinned_certificates.entry(hostname.clone()).or_insert_with(Vec::new);
        
        if pinned.len() >= self.config.max_pinned_per_host {
            return Err(PkiError::general(format!(
                "Maximum pinned certificates reached for hostname: {}", hostname
            )));
        pinned.push(certificate);
        Ok(())
    /// Check if certificate is pinned for hostname
    pub fn is_certificate_pinned(&self, hostname: &str, certificate: &X509Certificate) -> bool {
        if let Some(pinned_certs) = self.pinned_certificates.get(hostname) {
            pinned_certs.iter().any(|pinned| self.certificates_match(pinned, certificate))
        } else {
            false
        }
    }
    
    /// Add trust override for hostname
    pub fn add_trust_override(&mut self, hostname: String, certificate: X509Certificate) -> PkiResult<()> {
        if !self.config.enable_trust_overrides {
            return Err(PkiError::general("Trust overrides are disabled"));
        self.trust_overrides.insert(hostname, certificate);
        Ok(())
    /// Get trust override for hostname
    pub fn get_trust_override(&self, hostname: &str) -> Option<&X509Certificate> {
        self.trust_overrides.get(hostname)
    /// Add trust inheritance rule
    pub fn add_trust_inheritance_rule(&mut self, rule: TrustInheritanceRule) {
        // Insert maintaining priority order
        let insert_pos = self.trust_inheritance_rules
            .iter()
            .position(|r| r.priority > rule.priority)
            .unwrap_or(self.trust_inheritance_rules.len());
        
        self.trust_inheritance_rules.insert(insert_pos, rule);
    /// Check if trust can be inherited for a certificate
    pub fn can_inherit_trust(&self, certificate: &X509Certificate, source_anchor: &str) -> bool {
        if !self.config.inheritance_config.enabled {
            return false;
        for rule in &self.trust_inheritance_rules {
            if rule.source_anchor == source_anchor && self.rule_matches(rule, certificate) {
                return true;
            }
        }
        
        false
    /// Check if trust inheritance rule matches certificate
    fn rule_matches(&self, rule: &TrustInheritanceRule, certificate: &X509Certificate) -> bool {
        for condition in &rule.conditions {
            match condition {
                InheritanceCondition::SubjectMatches(pattern) => {
                    let subject_str = self.dn_to_string(&certificate.subject);
                    if !subject_str.contains(pattern) {
                        return false;
                    }
                }
                InheritanceCondition::IssuerMatches(pattern) => {
                    let issuer_str = self.dn_to_string(&certificate.issuer);
                    if !issuer_str.contains(pattern) {
                        return false;
                    }
                }
                InheritanceCondition::KeyUsageIncludes(usage) => {
                    if !self.key_usage_includes(&certificate.key_usage, usage) {
                        return false;
                    }
                }
                InheritanceCondition::ValidityPeriod { min_days, max_days } => {
                    let validity_duration = certificate.validity.not_after
                        .duration_since(certificate.validity.not_before)
                        .unwrap_or(Duration::ZERO);
                    let validity_days = validity_duration.as_secs() / (24 * 3600);
                    
                    if validity_days < *min_days as u64 || validity_days > *max_days as u64 {
                        return false;
                    }
                }
                _ => {} // Handle other conditions
            }
        }
        
        true
    /// Convert Distinguished Name to string
    fn dn_to_string(&self, dn: &DistinguishedName) -> String {
        let mut parts = Vec::new();
        
        if let Some(ref cn) = dn.common_name {
            parts.push(format!("CN={}", cn));
        }
        if let Some(ref o) = dn.organization {
            parts.push(format!("O={}", o));
        parts.join(", ")
    /// Check if key usage includes another key usage
    fn key_usage_includes(&self, current: &KeyUsage, required: &KeyUsage) -> bool {
        // Simplified key usage check - real implementation would check all flags
        current.key_cert_sign || required.key_cert_sign
    /// Check if two certificates match (for pinning)
    fn certificates_match(&self, cert1: &X509Certificate, cert2: &X509Certificate) -> bool {
        cert1.serial_number == cert2.serial_number && 
        cert1.subject == cert2.subject &&
        cert1.issuer == cert2.issuer
    }
}

impl ComprehensiveChainValidator {
    /// Create a new comprehensive chain validator
    pub fn new(config: ValidationConfig) -> Self {
        Self {
        }
    }
    
    /// Validate a complete certificate path
    pub fn validate_path(&self, path: &CertificatePath, trust_store: &EnhancedTrustStore) -> PkiResult<ValidationResult> {
        let mut errors = Vec::new();
        
        // Check if result is cached
        if self.config.cache_validation_results {
            let cache_key = self.generate_cache_key(path);
            if let Some(cached_result) = self.get_cached_result(&cache_key) {
                return Ok(cached_result.result);
            }
        }
        
        // Validate chain length
        if path.certificates.is_empty() {
            errors.push(ValidationError::EmptyChain);
        if path.certificates.len() > self.config.max_chain_length() {
            errors.push(ValidationError::ChainTooLong {
            });
        // Validate each certificate in the chain
        for (i, certificate) in path.certificates.iter().enumerate() {
            // Check validity period
            if self.config.check_validity_periods {
                self.validate_validity_period(certificate, &mut errors)?;
            // Check signature (except for root certificates)
            if self.config.check_signatures && i < path.certificates.len() - 1 {
                let issuer = &path.certificates[i + 1];
                self.validate_signature(certificate, issuer, &mut errors)?;
            // Check basic constraints
            if self.config.check_basic_constraints {
                self.validate_basic_constraints(certificate, i, &mut errors)?;
            // Check key usage
            if self.config.check_key_usage {
                self.validate_key_usage(certificate, i, &mut errors)?;
            // Check revocation status
            if self.config.check_revocation_status {
                self.validate_revocation_status(certificate, &mut errors)?;
            }
        }
        
        // Validate trust anchor
        if let Some(ref trust_anchor) = path.trust_anchor {
            self.validate_trust_anchor(trust_anchor, trust_store, &mut errors)?;
        let result = ValidationResult {
            validation_time_ms: 0, // Will be set by caller
        
        // Cache result if configured
        if self.config.cache_validation_results {
            let cache_key = self.generate_cache_key(path);
            self.cache_result(cache_key, result.clone());
        Ok(result)
    /// Validate certificate validity period
    fn validate_validity_period(&self, certificate: &X509Certificate, errors: &mut Vec<ValidationError>) -> PkiResult<()> {
        let now = SystemTime::now();
        let max_skew = Duration::from_secs(self.config.max_clock_skew_seconds);
        
        if now < certificate.validity.not_before - max_skew {
            errors.push(ValidationError::CertificateNotYetValid {
            });
        if now > certificate.validity.not_after + max_skew {
            errors.push(ValidationError::CertificateExpired {
            });
        Ok(())
    /// Validate certificate signature
    fn validate_signature(&self, certificate: &X509Certificate, issuer: &X509Certificate, errors: &mut Vec<ValidationError>) -> PkiResult<()> {
        // This would implement actual signature verification using cryptographic libraries
        // For now, just check that the issuer subject matches certificate issuer
        if issuer.subject != certificate.issuer {
            errors.push(ValidationError::InvalidSignature {
            });
        Ok(())
    /// Validate basic constraints
    fn validate_basic_constraints(&self, certificate: &X509Certificate, position: usize, errors: &mut Vec<ValidationError>) -> PkiResult<()> {
        // Find basic constraints extension
        for extension in &certificate.extensions {
            if extension.oid == "2.5.29.19" { // Basic Constraints OID
                if let Some(ref data) = extension.parsed_data {
//                     if let crate::stdlib::packages::crypto_pki::types::ExtensionData::BasicConstraints { is_ca, path_length_constraint } = data {
                        // If this is not the end entity certificate, it must be a CA
                        if position > 0 && !is_ca {
                            errors.push(ValidationError::InvalidBasicConstraints {
                            });
                        // Check path length constraint
                        if let Some(max_path_length) = path_length_constraint {
                            if position > *max_path_length as usize {
                                errors.push(ValidationError::PathLengthConstraintViolated {
                                });
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    /// Validate key usage
    fn validate_key_usage(&self, certificate: &X509Certificate, position: usize, errors: &mut Vec<ValidationError>) -> PkiResult<()> {
        // CA certificates must have keyCertSign
        if position > 0 && !certificate.key_usage.key_cert_sign {
            errors.push(ValidationError::InvalidKeyUsage {
            });
        Ok(())
    /// Validate revocation status
    fn validate_revocation_status(&self, certificate: &X509Certificate, errors: &mut Vec<ValidationError>) -> PkiResult<()> {
        if let Some(ref checker) = self.revocation_checker {
            match checker.is_revoked(certificate) {
                Ok(true) => {
                    let reason = checker.get_revocation_reason(certificate)
                        .unwrap_or(Ok(Some(RevocationReason::Unspecified)))
                        .unwrap_or(Some(RevocationReason::Unspecified));
                    
                    errors.push(ValidationError::CertificateRevoked {
                    });
                }
                Ok(false) => {} // Certificate is not revoked
                Err(_) => {
                    errors.push(ValidationError::RevocationCheckFailed {
                    });
                }
            }
        Ok(())
    /// Validate trust anchor
    fn validate_trust_anchor(&self, trust_anchor: &X509Certificate, trust_store: &EnhancedTrustStore, errors: &mut Vec<ValidationError>) -> PkiResult<()> {
        // Check if trust anchor is in the trust store
        if !trust_store.core_store.contains_certificate(trust_anchor) {
            errors.push(ValidationError::UntrustedRoot {
            });
        Ok(())
    /// Generate cache key for validation result
    fn generate_cache_key(&self, path: &CertificatePath) -> String {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        for cert in &path.certificates {
            cert.serial_number.hash(&mut hasher);
            cert.subject.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    /// Get cached validation result
    fn get_cached_result(&self, cache_key: &str) -> Option<CachedValidationResult> {
        let cache = self.validation_cache.read().unwrap();
        
        if let Some(cached) = cache.get(cache_key) {
            let now = SystemTime::now();
            if now < cached.cached_at + cached.ttl {
                return Some(cached.clone());
            }
        }
        
        None
    /// Cache validation result
    fn cache_result(&self, cache_key: String, result: ValidationResult) {
        let mut cache = self.validation_cache.write().unwrap();
        let cached_result = CachedValidationResult {
        
        cache.insert(cache_key, cached_result);
    }
}

impl PolicyEngine {
    /// Create a new policy engine
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Add policy mapping
    pub fn add_policy_mapping(&mut self, mapping: PolicyMapping) {
        self.policy_mappings.insert(mapping.source_policy.clone(), mapping);
    /// Add policy constraint
    pub fn add_policy_constraint(&mut self, constraint: PolicyConstraint) {
        self.policy_constraints.push(constraint);
    /// Set initial policy set
    pub fn set_initial_policy_set(&mut self, policies: HashSet<String>) {
        self.initial_policy_set = policies;
    }
}

impl Default for ChainBuildingConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_clock_skew_seconds: 300, // 5 minutes
            validation_cache_ttl_seconds: 3600, // 1 hour
        }
    }
impl ValidationConfig {
    /// Get maximum chain length
    pub fn max_chain_length(&self) -> usize {
        10 // Default maximum chain length
    }
}

impl Default for EnhancedTrustStoreConfig {
    fn default() -> Self {
        Self {
            enable_trust_overrides: false, // Disabled by default for security
            inheritance_config: TrustInheritanceConfig {
        }
    }
// Helper trait implementations
use std::hash::{Hash, Hasher};

impl Hash for DistinguishedName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.common_name.hash(state);
        self.organization.hash(state);
        self.country.hash(state);
    }
}

impl Hash for SerialNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bytes.hash(state);
    }
}

/// Public API for trust chain operations
pub fn build_certificate_chain(
    config: Option<ChainBuildingConfig>
) -> PkiResult<Vec<CertificatePath>> {
    let config = config.unwrap_or_default();
    let builder = TrustChainBuilder::new(trust_store.clone(), config);
    builder.build_chain(leaf_certificate)
/// Validate a certificate path
pub fn validate_certificate_path(
    config: Option<ValidationConfig>
) -> PkiResult<ValidationResult> {
    let config = config.unwrap_or_default();
    let validator = ComprehensiveChainValidator::new(config);
    validator.validate_path(path, trust_store)
/// Create enhanced trust store with default configuration
pub fn create_enhanced_trust_store() -> EnhancedTrustStore {
    EnhancedTrustStore::new(EnhancedTrustStoreConfig::default())
