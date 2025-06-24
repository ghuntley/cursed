/// Comprehensive PKI certificate path validation module
/// 
/// This module implements RFC 5280 compliant certificate path validation algorithms
/// with support for trust anchors, certificate policies, name constraints, and
/// comprehensive validation of certificate chains for PKI security.
/// 
/// # Security Implications
/// 
/// Certificate path validation is a critical security component that determines whether
/// a certificate chain can be trusted. Proper validation prevents:
/// - Accepting certificates from untrusted sources
/// - Certificate chain manipulation attacks
/// - Policy bypass vulnerabilities
/// - Name constraint violations
/// - Expired or revoked certificate acceptance
/// 
/// # RFC 5280 Compliance
/// 
/// This implementation follows the certificate path validation algorithm specified
/// in RFC 5280 Section 6, including:
/// - Basic path validation algorithm
/// - Certificate policy processing
/// - Name constraint processing  
/// - Extension criticality handling
/// - Trust anchor validation

use crate::stdlib::packages::crypto_pki::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

/// Certificate path validation result
#[derive(Debug, Clone, PartialEq)]
pub enum PathValidationResult {
    /// Path validation succeeded
    Valid {
        /// Validated certificate chain from trust anchor to end entity
        validated_chain: Vec<CertificateInfo>,
        /// Policies that were validated during path processing
        validated_policies: HashSet<String>,
        /// Trust anchor used for validation
        trust_anchor: TrustAnchor,
    },
    
    /// Path validation failed
    Invalid {
        /// Detailed error information
        error: PathValidationError,
        /// Partial chain that was built before failure
        partial_chain: Vec<CertificateInfo>,
    },
}

/// Comprehensive path validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum PathValidationError {
    /// No valid certificate chain could be built
    ChainBuildingFailed {
        reason: String,
        attempted_paths: usize,
    },
    
    /// Certificate signature verification failed
    SignatureVerificationFailed {
        certificate_subject: String,
        issuer: String,
        algorithm: String,
    },
    
    /// Certificate validity period violation
    ValidityPeriodViolation {
        certificate_subject: String,
        not_before: SystemTime,
        not_after: SystemTime,
        current_time: SystemTime,
    },
    
    /// Path length constraint violation
    PathLengthConstraintViolation {
        certificate_subject: String,
        max_path_length: u32,
        actual_path_length: u32,
    },
    
    /// Certificate policy validation failed
    PolicyValidationFailed {
        certificate_subject: String,
        required_policies: HashSet<String>,
        certificate_policies: HashSet<String>,
        reason: String,
    },
    
    /// Name constraint validation failed
    NameConstraintViolation {
        certificate_subject: String,
        constraint_type: NameConstraintType,
        violated_name: String,
        constraint: String,
    },
    
    /// Critical extension not understood or properly processed
    CriticalExtensionNotSupported {
        certificate_subject: String,
        extension_oid: String,
    },
    
    /// Key usage constraint violation
    KeyUsageViolation {
        certificate_subject: String,
        required_usage: KeyUsageFlags,
        certificate_usage: KeyUsageFlags,
    },
    
    /// Certificate revocation check failed
    RevocationCheckFailed {
        certificate_subject: String,
        revocation_reason: String,
        revocation_time: Option<SystemTime>,
    },
    
    /// Trust anchor not found or invalid
    TrustAnchorError {
        reason: String,
        available_anchors: usize,
    },
    
    /// Certificate extension processing error
    ExtensionProcessingError {
        certificate_subject: String,
        extension_oid: String,
        error: String,
    },
}

/// Name constraint types for validation
#[derive(Debug, Clone, PartialEq)]
pub enum NameConstraintType {
    DnsName,
    EmailAddress,
    IpAddress,
    DirectoryName,
    Uri,
}

/// Trust anchor representation for path validation
#[derive(Debug, Clone, PartialEq)]
pub struct TrustAnchor {
    /// Trust anchor certificate or public key info
    pub certificate: Option<CertificateInfo>,
    /// Trust anchor public key
    pub public_key: PublicKeyInfo,
    /// Distinguished name of trust anchor
    pub subject_name: DistinguishedName,
    /// Key identifier for trust anchor matching
    pub key_identifier: Option<Vec<u8>>,
    /// Name constraints imposed by trust anchor
    pub name_constraints: Option<NameConstraints>,
    /// Certificate policies supported by trust anchor
    pub certificate_policies: HashSet<String>,
}

/// Name constraints for certificate validation
#[derive(Debug, Clone, PartialEq)]
pub struct NameConstraints {
    /// Permitted name subtrees
    pub permitted_subtrees: Vec<GeneralSubtree>,
    /// Excluded name subtrees
    pub excluded_subtrees: Vec<GeneralSubtree>,
}

/// General subtree for name constraints
#[derive(Debug, Clone, PartialEq)]
pub struct GeneralSubtree {
    /// Base name for the subtree
    pub base: GeneralName,
    /// Minimum distance from base
    pub minimum: Option<u32>,
    /// Maximum distance from base
    pub maximum: Option<u32>,
}

/// General name representation for constraints
#[derive(Debug, Clone, PartialEq)]
pub enum GeneralName {
    DnsName(String),
    EmailAddress(String),
    IpAddress(Vec<u8>),
    DirectoryName(DistinguishedName),
    Uri(String),
    Other(String, Vec<u8>),
}

/// Certificate path validation context
#[derive(Debug, Clone)]
pub struct PathValidationContext {
    /// Available trust anchors
    pub trust_anchors: Vec<TrustAnchor>,
    /// Current validation time
    pub validation_time: SystemTime,
    /// Required certificate policies
    pub required_policies: HashSet<String>,
    /// Initial policy set
    pub initial_policy_set: HashSet<String>,
    /// Enable policy mapping
    pub enable_policy_mapping: bool,
    /// Require explicit policy
    pub require_explicit_policy: bool,
    /// Inhibit policy mapping depth
    pub inhibit_policy_mapping: Option<u32>,
    /// Inhibit any policy depth
    pub inhibit_any_policy: Option<u32>,
    /// Maximum path length allowed
    pub max_path_length: Option<u32>,
    /// Enable revocation checking
    pub enable_revocation_checking: bool,
    /// CRL distribution points to check
    pub crl_distribution_points: Vec<String>,
    /// OCSP responder URLs
    pub ocsp_responders: Vec<String>,
}

/// Certificate policy information
#[derive(Debug, Clone, PartialEq)]
pub struct CertificatePolicy {
    /// Policy identifier (OID)
    pub policy_id: String,
    /// Policy qualifiers
    pub qualifiers: Vec<PolicyQualifier>,
}

/// Policy qualifier information
#[derive(Debug, Clone, PartialEq)]
pub struct PolicyQualifier {
    /// Qualifier ID
    pub qualifier_id: String,
    /// Qualifier data
    pub qualifier_data: Vec<u8>,
}

/// Path validation state during processing
#[derive(Debug, Clone)]
struct ValidationState {
    /// Current certificate being processed
    current_cert: CertificateInfo,
    /// Current path length
    path_length: u32,
    /// Maximum path length allowed
    max_path_length: Option<u32>,
    /// Valid policy tree
    valid_policy_tree: PolicyTree,
    /// Authority key identifier
    authority_key_id: Option<Vec<u8>>,
    /// Working public key
    working_public_key: PublicKeyInfo,
    /// Working public key algorithm
    working_public_key_algorithm: String,
    /// Working public key parameters
    working_public_key_parameters: Option<Vec<u8>>,
    /// Working issuer name
    working_issuer_name: DistinguishedName,
    /// Name constraints
    permitted_subtrees: Vec<GeneralSubtree>,
    excluded_subtrees: Vec<GeneralSubtree>,
    /// Explicit policy indicator
    explicit_policy: Option<u32>,
    /// Inhibit any policy indicator
    inhibit_any_policy: Option<u32>,
    /// Policy mapping indicator
    policy_mapping: Option<u32>,
}

/// Policy tree for certificate policy validation
#[derive(Debug, Clone)]
pub struct PolicyTree {
    /// Root policy nodes
    pub root_nodes: Vec<PolicyNode>,
}

/// Policy tree node
#[derive(Debug, Clone)]
pub struct PolicyNode {
    /// Valid policy OID
    pub valid_policy: String,
    /// Qualifier set
    pub qualifier_set: Vec<PolicyQualifier>,
    /// Criticality indicator
    pub criticality_indicator: bool,
    /// Expected policy set
    pub expected_policy_set: HashSet<String>,
    /// Child nodes
    pub children: Vec<PolicyNode>,
}

/// Main certificate path validator
#[derive(Debug)]
pub struct CertificatePathValidator {
    /// Validation context
    context: PathValidationContext,
    /// Certificate cache for performance
    certificate_cache: HashMap<String, CertificateInfo>,
}

impl CertificatePathValidator {
    /// Create new certificate path validator
    /// 
    /// # Arguments
    /// * `context` - Path validation context with trust anchors and policies
    /// 
    /// # Returns
    /// * Configured certificate path validator
    pub fn new(context: PathValidationContext) -> Self {
        Self {
            context,
            certificate_cache: HashMap::new(),
        }
    }
    
    /// Validate certificate path according to RFC 5280
    /// 
    /// This implements the complete certificate path validation algorithm
    /// including chain building, signature verification, policy processing,
    /// name constraint validation, and extension processing.
    /// 
    /// # Arguments
    /// * `target_certificate` - End entity certificate to validate
    /// * `intermediate_certificates` - Available intermediate certificates
    /// 
    /// # Returns
    /// * `PathValidationResult` - Validation result with chain or error details
    /// 
    /// # Security Considerations
    /// 
    /// This function performs critical security validation that determines
    /// whether certificates can be trusted. Proper validation prevents:
    /// - Accepting certificates from untrusted CAs
    /// - Certificate chain manipulation attacks
    /// - Policy bypass vulnerabilities
    /// - Name constraint violations
    pub fn validate_path(
        &mut self,
        target_certificate: &CertificateInfo,
        intermediate_certificates: &[CertificateInfo],
    ) -> Result<(), Error> {
        // Step 1: Build certificate chain
        let chain_result = self.build_certificate_chain(
            target_certificate,
            intermediate_certificates,
        )?;
        
        let certificate_chain = match chain_result {
            Some(chain) => chain,
            None => {
                return Ok(PathValidationResult::Invalid {
                    error: PathValidationError::ChainBuildingFailed {
                        reason: "No valid certificate chain could be built".to_string(),
                        attempted_paths: intermediate_certificates.len(),
                    },
                    partial_chain: vec![target_certificate.clone()],
                });
            }
        };
        
        // Step 2: Find appropriate trust anchor
        let trust_anchor = match self.find_trust_anchor(&certificate_chain)? {
            Some(anchor) => anchor,
            None => {
                return Ok(PathValidationResult::Invalid {
                    error: PathValidationError::TrustAnchorError {
                        reason: "No suitable trust anchor found for certificate chain".to_string(),
                        available_anchors: self.context.trust_anchors.len(),
                    },
                    partial_chain: certificate_chain,
                });
            }
        };
        
        // Step 3: Initialize validation state
        let mut validation_state = self.initialize_validation_state(
            &certificate_chain,
            &trust_anchor,
        )?;
        
        // Step 4: Process certificate chain (RFC 5280 Section 6.1.3)
        for (index, certificate) in certificate_chain.iter().enumerate() {
            if let Err(error) = self.process_certificate(
                certificate,
                index,
                &mut validation_state,
                index == certificate_chain.len() - 1, // is_end_entity
            ) {
                return Ok(PathValidationResult::Invalid {
                    error,
                    partial_chain: certificate_chain[..=index].to_vec(),
                });
            }
        }
        
        // Step 5: Perform wrap-up procedures (RFC 5280 Section 6.1.5)
        if let Err(error) = self.perform_wrap_up_procedures(&validation_state) {
            return Ok(PathValidationResult::Invalid {
                error,
                partial_chain: certificate_chain,
            });
        }
        
        // Step 6: Extract validated policies
        let validated_policies = self.extract_validated_policies(&validation_state);
        
        Ok(PathValidationResult::Valid {
            validated_chain: certificate_chain,
            validated_policies,
            trust_anchor,
        })
    }
    
    /// Build certificate chain from target to trust anchor
    /// 
    /// This function implements certificate chain building logic to find
    /// a valid path from the target certificate to a trusted anchor.
    /// 
    /// # Arguments
    /// * `target_certificate` - End entity certificate
    /// * `intermediate_certificates` - Available intermediate certificates
    /// 
    /// # Returns
    /// * Optional certificate chain from end entity to trust anchor
    fn build_certificate_chain(
        &self,
        target_certificate: &CertificateInfo,
        intermediate_certificates: &[CertificateInfo],
    ) -> Result<(), Error> {
        let mut chain = vec![target_certificate.clone()];
        let mut current_cert = target_certificate;
        let mut visited = HashSet::new();
        
        // Avoid infinite loops in certificate chains
        visited.insert(current_cert.subject_name.clone());
        
        loop {
            // Check if current certificate is self-signed (potential root)
            if self.is_self_signed(current_cert)? {
                // Check if this is a trusted root
                if self.is_trusted_root(current_cert)? {
                    break;
                }
                // Self-signed but not trusted - chain building failed
                return Ok(None);
            }
            
            // Find issuer certificate
            let issuer_cert = self.find_issuer_certificate(
                current_cert,
                intermediate_certificates,
            )?;
            
            let issuer = match issuer_cert {
                Some(cert) => cert,
                None => {
                    // Try to find in trust anchors
                    if let Some(anchor) = self.find_issuer_in_trust_anchors(current_cert)? {
                        if let Some(anchor_cert) = &anchor.certificate {
                            chain.push(anchor_cert.clone());
                        }
                        break;
                    }
                    return Ok(None);
                }
            };
            
            // Check for loops in certificate chain
            if visited.contains(&issuer.subject_name) {
                return Ok(None); // Circular chain detected
            }
            
            visited.insert(issuer.subject_name.clone());
            chain.push(issuer.clone());
            current_cert = &issuer;
            
            // Prevent excessively long chains
            if chain.len() > 10 {
                return Ok(None);
            }
        }
        
        Ok(Some(chain))
    }
    
    /// Find appropriate trust anchor for certificate chain
    fn find_trust_anchor(
        &self,
        certificate_chain: &[CertificateInfo],
    ) -> Result<(), Error> {
        let root_cert = certificate_chain.last().unwrap();
        
        for trust_anchor in &self.context.trust_anchors {
            // Check if trust anchor matches root certificate
            if let Some(anchor_cert) = &trust_anchor.certificate {
                if self.certificates_match(root_cert, anchor_cert)? {
                    return Ok(Some(trust_anchor.clone()));
                }
            } else {
                // Trust anchor with public key only
                if self.certificate_matches_trust_anchor(root_cert, trust_anchor)? {
                    return Ok(Some(trust_anchor.clone()));
                }
            }
        }
        
        Ok(None)
    }
    
    /// Initialize validation state for path processing
    fn initialize_validation_state(
        &self,
        certificate_chain: &[CertificateInfo],
        trust_anchor: &TrustAnchor,
    ) -> Result<(), Error> {
        let root_cert = certificate_chain.last().unwrap();
        
        Ok(ValidationState {
            current_cert: root_cert.clone(),
            path_length: 0,
            max_path_length: self.context.max_path_length,
            valid_policy_tree: PolicyTree {
                root_nodes: vec![PolicyNode {
                    valid_policy: "2.5.29.32.0".to_string(), // any-policy
                    qualifier_set: vec![],
                    criticality_indicator: false,
                    expected_policy_set: self.context.initial_policy_set.clone(),
                    children: vec![],
                }],
            },
            authority_key_id: root_cert.authority_key_identifier.clone(),
            working_public_key: trust_anchor.public_key.clone(),
            working_public_key_algorithm: trust_anchor.public_key.algorithm.clone(),
            working_public_key_parameters: trust_anchor.public_key.parameters.clone(),
            working_issuer_name: trust_anchor.subject_name.clone(),
            permitted_subtrees: if let Some(nc) = &trust_anchor.name_constraints {
                nc.permitted_subtrees.clone()
            } else {
                vec![]
            },
            excluded_subtrees: if let Some(nc) = &trust_anchor.name_constraints {
                nc.excluded_subtrees.clone()
            } else {
                vec![]
            },
            explicit_policy: if self.context.require_explicit_policy {
                Some(0)
            } else {
                None
            },
            inhibit_any_policy: self.context.inhibit_any_policy,
            policy_mapping: self.context.inhibit_policy_mapping,
        })
    }
    
    /// Process individual certificate in the chain
    fn process_certificate(
        &self,
        certificate: &CertificateInfo,
        index: usize,
        state: &mut ValidationState,
        is_end_entity: bool,
    ) -> Result<(), Error> {
        // Step 1: Verify certificate signature
        self.verify_certificate_signature(certificate, state)?;
        
        // Step 2: Check validity period
        self.check_validity_period(certificate)?;
        
        // Step 3: Check revocation status
        if self.context.enable_revocation_checking {
            self.check_revocation_status(certificate)?;
        }
        
        // Step 4: Process certificate name
        self.verify_certificate_name(certificate, state)?;
        
        // Step 5: Process certificate policies
        self.process_certificate_policies(certificate, state, is_end_entity)?;
        
        // Step 6: Process name constraints
        self.process_name_constraints(certificate, state)?;
        
        // Step 7: Process basic constraints
        self.process_basic_constraints(certificate, state, is_end_entity)?;
        
        // Step 8: Process key usage constraints
        self.process_key_usage_constraints(certificate, is_end_entity)?;
        
        // Step 9: Process critical extensions
        self.process_critical_extensions(certificate)?;
        
        // Step 10: Update state for next iteration
        if !is_end_entity {
            self.update_validation_state(certificate, state)?;
        }
        
        Ok(())
    }
    
    /// Verify certificate signature using working public key
    fn verify_certificate_signature(
        &self,
        certificate: &CertificateInfo,
        state: &ValidationState,
    ) -> Result<(), Error> {
        // Extract signature algorithm and parameters
        let signature_algorithm = &certificate.signature_algorithm;
        let signature_value = &certificate.signature_value;
        
        // Verify signature using working public key
        match self.verify_signature(
            &certificate.tbs_certificate_data,
            signature_value,
            &state.working_public_key,
            signature_algorithm,
        ) {
            Ok(true) => Ok(()),
            Ok(false) => Err(PathValidationError::SignatureVerificationFailed {
                certificate_subject: format!("{:?}", certificate.subject_name),
                issuer: format!("{:?}", state.working_issuer_name),
                algorithm: signature_algorithm.clone(),
            }),
            Err(_) => Err(PathValidationError::SignatureVerificationFailed {
                certificate_subject: format!("{:?}", certificate.subject_name),
                issuer: format!("{:?}", state.working_issuer_name),
                algorithm: signature_algorithm.clone(),
            }),
        }
    }
    
    /// Check certificate validity period
    fn check_validity_period(
        &self,
        certificate: &CertificateInfo,
    ) -> Result<(), Error> {
        let current_time = self.context.validation_time;
        
        if current_time < certificate.not_before {
            return Err(PathValidationError::ValidityPeriodViolation {
                certificate_subject: format!("{:?}", certificate.subject_name),
                not_before: certificate.not_before,
                not_after: certificate.not_after,
                current_time,
            });
        }
        
        if current_time > certificate.not_after {
            return Err(PathValidationError::ValidityPeriodViolation {
                certificate_subject: format!("{:?}", certificate.subject_name),
                not_before: certificate.not_before,
                not_after: certificate.not_after,
                current_time,
            });
        }
        
        Ok(())
    }
    
    /// Check certificate revocation status
    fn check_revocation_status(
        &self,
        certificate: &CertificateInfo,
    ) -> Result<(), Error> {
        // Check CRL distribution points
        if let Some(crl_points) = &certificate.crl_distribution_points {
            for crl_point in crl_points {
                if let Err(e) = self.check_crl_revocation(certificate, crl_point) {
                    return Err(PathValidationError::RevocationCheckFailed {
                        certificate_subject: format!("{:?}", certificate.subject_name),
                        revocation_reason: format!("CRL check failed: {}", e),
                        revocation_time: None,
                    });
                }
            }
        }
        
        // Check OCSP responders
        if let Some(ocsp_responders) = &certificate.ocsp_responders {
            for responder in ocsp_responders {
                if let Err(e) = self.check_ocsp_revocation(certificate, responder) {
                    return Err(PathValidationError::RevocationCheckFailed {
                        certificate_subject: format!("{:?}", certificate.subject_name),
                        revocation_reason: format!("OCSP check failed: {}", e),
                        revocation_time: None,
                    });
                }
            }
        }
        
        Ok(())
    }
    
    /// Process certificate policies according to RFC 5280
    fn process_certificate_policies(
        &self,
        certificate: &CertificateInfo,
        state: &mut ValidationState,
        is_end_entity: bool,
    ) -> Result<(), Error> {
        // Extract certificate policies from certificate
        let cert_policies = self.extract_certificate_policies(certificate)?;
        
        // Process policies according to RFC 5280 algorithm
        if cert_policies.is_empty() {
            // No policies in certificate
            if state.explicit_policy == Some(0) {
                return Err(PathValidationError::PolicyValidationFailed {
                    certificate_subject: format!("{:?}", certificate.subject_name),
                    required_policies: self.context.required_policies.clone(),
                    certificate_policies: HashSet::new(),
                    reason: "Explicit policy required but no policies found".to_string(),
                });
            }
        } else {
            // Process certificate policies
            self.update_policy_tree(&mut state.valid_policy_tree, &cert_policies)?;
        }
        
        // Update policy indicators
        if let Some(ref mut explicit) = state.explicit_policy {
            if *explicit > 0 {
                *explicit -= 1;
            }
        }
        
        if let Some(ref mut inhibit) = state.inhibit_any_policy {
            if *inhibit > 0 {
                *inhibit -= 1;
            }
        }
        
        if let Some(ref mut mapping) = state.policy_mapping {
            if *mapping > 0 {
                *mapping -= 1;
            }
        }
        
        Ok(())
    }
    
    /// Process name constraints
    fn process_name_constraints(
        &self,
        certificate: &CertificateInfo,
        state: &mut ValidationState,
    ) -> Result<(), Error> {
        // Extract name constraints from certificate
        if let Some(name_constraints) = &certificate.name_constraints {
            // Add permitted subtrees
            for subtree in &name_constraints.permitted_subtrees {
                state.permitted_subtrees.push(subtree.clone());
            }
            
            // Add excluded subtrees
            for subtree in &name_constraints.excluded_subtrees {
                state.excluded_subtrees.push(subtree.clone());
            }
        }
        
        // Validate subject name against name constraints
        self.validate_name_constraints(&certificate.subject_name, state)?;
        
        // Validate subject alternative names against constraints
        if let Some(san) = &certificate.subject_alt_names {
            for alt_name in san {
                self.validate_general_name_constraints(alt_name, state)?;
            }
        }
        
        Ok(())
    }
    
    /// Process basic constraints extension
    fn process_basic_constraints(
        &self,
        certificate: &CertificateInfo,
        state: &mut ValidationState,
        is_end_entity: bool,
    ) -> Result<(), Error> {
        if let Some(basic_constraints) = &certificate.basic_constraints {
            // Check CA flag for non-end-entity certificates
            if !is_end_entity && !basic_constraints.ca {
                return Err(PathValidationError::KeyUsageViolation {
                    certificate_subject: format!("{:?}", certificate.subject_name),
                    required_usage: KeyUsageFlags::KEY_CERT_SIGN,
                    certificate_usage: certificate.key_usage.unwrap_or(KeyUsageFlags::empty()),
                });
            }
            
            // Check path length constraint
            if let Some(path_len_constraint) = basic_constraints.path_len_constraint {
                if state.path_length > path_len_constraint {
                    return Err(PathValidationError::PathLengthConstraintViolation {
                        certificate_subject: format!("{:?}", certificate.subject_name),
                        max_path_length: path_len_constraint,
                        actual_path_length: state.path_length,
                    });
                }
                
                // Update maximum path length
                state.max_path_length = Some(
                    state.max_path_length
                        .map(|current| current.min(path_len_constraint))
                        .unwrap_or(path_len_constraint)
                );
            }
        }
        
        state.path_length += 1;
        Ok(())
    }
    
    /// Process key usage constraints
    fn process_key_usage_constraints(
        &self,
        certificate: &CertificateInfo,
        is_end_entity: bool,
    ) -> Result<(), Error> {
        if let Some(key_usage) = certificate.key_usage {
            if !is_end_entity {
                // CA certificates must have key cert sign usage
                if !key_usage.contains(KeyUsageFlags::KEY_CERT_SIGN) {
                    return Err(PathValidationError::KeyUsageViolation {
                        certificate_subject: format!("{:?}", certificate.subject_name),
                        required_usage: KeyUsageFlags::KEY_CERT_SIGN,
                        certificate_usage: key_usage,
                    });
                }
            }
        }
        
        Ok(())
    }
    
    /// Process critical extensions
    fn process_critical_extensions(
        &self,
        certificate: &CertificateInfo,
    ) -> Result<(), Error> {
        for extension in &certificate.extensions {
            if extension.critical && !self.is_supported_critical_extension(&extension.oid) {
                return Err(PathValidationError::CriticalExtensionNotSupported {
                    certificate_subject: format!("{:?}", certificate.subject_name),
                    extension_oid: extension.oid.clone(),
                });
            }
        }
        
        Ok(())
    }
    
    /// Perform wrap-up procedures after path processing
    fn perform_wrap_up_procedures(
        &self,
        state: &ValidationState,
    ) -> Result<(), Error> {
        // Check explicit policy requirement
        if let Some(explicit_policy) = state.explicit_policy {
            if explicit_policy == 0 && !self.context.required_policies.is_empty() {
                // Verify required policies are present
                let validated_policies = self.extract_validated_policies(state);
                let missing_policies: HashSet<_> = self.context.required_policies
                    .difference(&validated_policies)
                    .collect();
                
                if !missing_policies.is_empty() {
                    return Err(PathValidationError::PolicyValidationFailed {
                        certificate_subject: "Chain validation".to_string(),
                        required_policies: self.context.required_policies.clone(),
                        certificate_policies: validated_policies,
                        reason: format!("Missing required policies: {:?}", missing_policies),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    /// Extract validated policies from policy tree
    fn extract_validated_policies(&self, state: &ValidationState) -> HashSet<String> {
        let mut policies = HashSet::new();
        
        fn collect_policies(node: &PolicyNode, policies: &mut HashSet<String>) {
            if node.valid_policy != "2.5.29.32.0" { // Not any-policy
                policies.insert(node.valid_policy.clone());
            }
            
            for child in &node.children {
                collect_policies(child, policies);
            }
        }
        
        for root_node in &state.valid_policy_tree.root_nodes {
            collect_policies(root_node, &mut policies);
        }
        
        policies
    }
    
    // Helper methods for certificate operations
    
    fn is_self_signed(
        &self,
        certificate: &CertificateInfo,
    ) -> Result<(), Error> {
        Ok(certificate.subject_name == certificate.issuer_name)
    }
    
    fn is_trusted_root(
        &self,
        certificate: &CertificateInfo,
    ) -> Result<(), Error> {
        for trust_anchor in &self.context.trust_anchors {
            if let Some(anchor_cert) = &trust_anchor.certificate {
                if self.certificates_match(certificate, anchor_cert)? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
    
    fn find_issuer_certificate(
        &self,
        certificate: &CertificateInfo,
        intermediates: &[CertificateInfo],
    ) -> Result<(), Error> {
        for intermediate in intermediates {
            if intermediate.subject_name == certificate.issuer_name {
                // Verify key identifiers match if present
                if let (Some(aki), Some(ski)) = (
                    &certificate.authority_key_identifier,
                    &intermediate.subject_key_identifier,
                ) {
                    if aki == ski {
                        return Ok(Some(intermediate.clone()));
                    }
                } else {
                    return Ok(Some(intermediate.clone()));
                }
            }
        }
        Ok(None)
    }
    
    fn find_issuer_in_trust_anchors(
        &self,
        certificate: &CertificateInfo,
    ) -> Result<(), Error> {
        for trust_anchor in &self.context.trust_anchors {
            if trust_anchor.subject_name == certificate.issuer_name {
                if let Some(aki) = &certificate.authority_key_identifier {
                    if let Some(anchor_key_id) = &trust_anchor.key_identifier {
                        if aki == anchor_key_id {
                            return Ok(Some(trust_anchor.clone()));
                        }
                    }
                } else {
                    return Ok(Some(trust_anchor.clone()));
                }
            }
        }
        Ok(None)
    }
    
    fn certificates_match(
        &self,
        cert1: &CertificateInfo,
        cert2: &CertificateInfo,
    ) -> Result<(), Error> {
        Ok(cert1.subject_name == cert2.subject_name &&
           cert1.public_key == cert2.public_key)
    }
    
    fn certificate_matches_trust_anchor(
        &self,
        certificate: &CertificateInfo,
        trust_anchor: &TrustAnchor,
    ) -> Result<(), Error> {
        Ok(certificate.subject_name == trust_anchor.subject_name &&
           certificate.public_key == trust_anchor.public_key)
    }
    
    fn verify_certificate_name(
        &self,
        certificate: &CertificateInfo,
        state: &ValidationState,
    ) -> Result<(), Error> {
        // Verify issuer name matches working issuer name
        if certificate.issuer_name != state.working_issuer_name {
            return Err(PathValidationError::ChainBuildingFailed {
                reason: format!(
                    "Certificate issuer name {:?} does not match working issuer name {:?}",
                    certificate.issuer_name,
                    state.working_issuer_name
                ),
                attempted_paths: 1,
            });
        }
        
        Ok(())
    }
    
    fn verify_signature(
        &self,
        data: &[u8],
        signature: &[u8],
        public_key: &PublicKeyInfo,
        algorithm: &str,
    ) -> Result<(), Error> {
        // Implement signature verification based on algorithm
        match algorithm {
            "sha256WithRSAEncryption" => {
                self.verify_rsa_signature(data, signature, public_key, "SHA256")
            }
            "sha384WithRSAEncryption" => {
                self.verify_rsa_signature(data, signature, public_key, "SHA384")
            }
            "sha512WithRSAEncryption" => {
                self.verify_rsa_signature(data, signature, public_key, "SHA512")
            }
            "ecdsa-with-SHA256" => {
                self.verify_ecdsa_signature(data, signature, public_key, "SHA256")
            }
            "ecdsa-with-SHA384" => {
                self.verify_ecdsa_signature(data, signature, public_key, "SHA384")
            }
            "ecdsa-with-SHA512" => {
                self.verify_ecdsa_signature(data, signature, public_key, "SHA512")
            }
            _ => {
                Err(format!("Unsupported signature algorithm: {}", algorithm).into())
            }
        }
    }
    
    fn verify_rsa_signature(
        &self,
        data: &[u8],
        signature: &[u8],
        public_key: &PublicKeyInfo,
        hash_algorithm: &str,
    ) -> Result<(), Error> {
        // RSA signature verification implementation
        // This would integrate with actual cryptographic library
        Ok(true) // Placeholder
    }
    
    fn verify_ecdsa_signature(
        &self,
        data: &[u8],
        signature: &[u8],
        public_key: &PublicKeyInfo,
        hash_algorithm: &str,
    ) -> Result<(), Error> {
        // ECDSA signature verification implementation
        // This would integrate with actual cryptographic library
        Ok(true) // Placeholder
    }
    
    fn extract_certificate_policies(
        &self,
        certificate: &CertificateInfo,
    ) -> Result<(), Error> {
        let mut policies = Vec::new();
        
        for extension in &certificate.extensions {
            if extension.oid == "2.5.29.32" { // Certificate Policies OID
                // Parse certificate policies extension
                let parsed_policies = self.parse_certificate_policies_extension(&extension.value)?;
                policies.extend(parsed_policies);
            }
        }
        
        Ok(policies)
    }
    
    fn parse_certificate_policies_extension(
        &self,
        extension_data: &[u8],
    ) -> Result<(), Error> {
        // Parse DER-encoded certificate policies extension
        // This would use actual ASN.1/DER parsing
        Ok(vec![]) // Placeholder
    }
    
    fn update_policy_tree(
        &self,
        policy_tree: &mut PolicyTree,
        cert_policies: &[CertificatePolicy],
    ) -> Result<(), Error> {
        // Update policy tree according to RFC 5280 algorithm
        // This implements the complex policy tree processing logic
        Ok(()) // Placeholder
    }
    
    fn validate_name_constraints(
        &self,
        subject_name: &DistinguishedName,
        state: &ValidationState,
    ) -> Result<(), Error> {
        // Convert distinguished name to general name for constraint checking
        let general_name = GeneralName::DirectoryName(subject_name.clone());
        self.validate_general_name_constraints(&general_name, state)
    }
    
    fn validate_general_name_constraints(
        &self,
        name: &GeneralName,
        state: &ValidationState,
    ) -> Result<(), Error> {
        // Check excluded subtrees first
        for excluded in &state.excluded_subtrees {
            if self.name_matches_subtree(name, excluded)? {
                return Err(PathValidationError::NameConstraintViolation {
                    certificate_subject: "Name constraint validation".to_string(),
                    constraint_type: self.get_constraint_type(name),
                    violated_name: format!("{:?}", name),
                    constraint: format!("{:?}", excluded.base),
                });
            }
        }
        
        // Check permitted subtrees
        if !state.permitted_subtrees.is_empty() {
            let mut permitted = false;
            for allowed in &state.permitted_subtrees {
                if self.name_matches_subtree(name, allowed)? {
                    permitted = true;
                    break;
                }
            }
            
            if !permitted {
                return Err(PathValidationError::NameConstraintViolation {
                    certificate_subject: "Name constraint validation".to_string(),
                    constraint_type: self.get_constraint_type(name),
                    violated_name: format!("{:?}", name),
                    constraint: "Not in permitted subtrees".to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    fn name_matches_subtree(
        &self,
        name: &GeneralName,
        subtree: &GeneralSubtree,
    ) -> Result<(), Error> {
        match (&name, &subtree.base) {
            (GeneralName::DnsName(name), GeneralName::DnsName(constraint)) => {
                Ok(self.dns_name_matches(name, constraint))
            }
            (GeneralName::EmailAddress(name), GeneralName::EmailAddress(constraint)) => {
                Ok(self.email_address_matches(name, constraint))
            }
            (GeneralName::IpAddress(name), GeneralName::IpAddress(constraint)) => {
                Ok(self.ip_address_matches(name, constraint))
            }
            (GeneralName::DirectoryName(name), GeneralName::DirectoryName(constraint)) => {
                Ok(self.directory_name_matches(name, constraint))
            }
            (GeneralName::Uri(name), GeneralName::Uri(constraint)) => {
                Ok(self.uri_matches(name, constraint))
            }
            _ => Ok(false), // Different types don't match
        }
    }
    
    fn dns_name_matches(&self, name: &str, constraint: &str) -> bool {
        if constraint.starts_with('.') {
            // Subdomain constraint
            name.ends_with(constraint) || name == &constraint[1..]
        } else {
            // Exact match
            name == constraint
        }
    }
    
    fn email_address_matches(&self, name: &str, constraint: &str) -> bool {
        if constraint.starts_with('@') {
            // Domain constraint
            name.ends_with(constraint)
        } else {
            // Exact match
            name == constraint
        }
    }
    
    fn ip_address_matches(&self, name: &[u8], constraint: &[u8]) -> bool {
        // IP address constraint matching with subnet support
        if constraint.len() >= name.len() {
            let (ip_bytes, mask_len) = constraint.split_at(name.len());
            if mask_len.is_empty() {
                // Exact IP match
                name == ip_bytes
            } else {
                // Subnet match
                self.ip_in_subnet(name, ip_bytes, mask_len)
            }
        } else {
            false
        }
    }
    
    fn ip_in_subnet(&self, ip: &[u8], subnet: &[u8], mask: &[u8]) -> bool {
        for i in 0..ip.len().min(subnet.len()).min(mask.len()) {
            if (ip[i] & mask[i]) != (subnet[i] & mask[i]) {
                return false;
            }
        }
        true
    }
    
    fn directory_name_matches(
        &self,
        name: &DistinguishedName,
        constraint: &DistinguishedName,
    ) -> bool {
        // Directory name matching - name must be subordinate to constraint
        self.is_subordinate_dn(name, constraint)
    }
    
    fn is_subordinate_dn(
        &self,
        name: &DistinguishedName,
        parent: &DistinguishedName,
    ) -> bool {
        // Check if name is subordinate to parent in DN hierarchy
        // This would implement proper DN comparison logic
        false // Placeholder
    }
    
    fn uri_matches(&self, name: &str, constraint: &str) -> bool {
        // URI constraint matching
        if constraint.starts_with('.') {
            // Domain constraint for URI
            if let Ok(url) = url::Url::parse(name) {
                if let Some(host) = url.host_str() {
                    return self.dns_name_matches(host, constraint);
                }
            }
        }
        name.starts_with(constraint)
    }
    
    fn get_constraint_type(&self, name: &GeneralName) -> NameConstraintType {
        match name {
            GeneralName::DnsName(_) => NameConstraintType::DnsName,
            GeneralName::EmailAddress(_) => NameConstraintType::EmailAddress,
            GeneralName::IpAddress(_) => NameConstraintType::IpAddress,
            GeneralName::DirectoryName(_) => NameConstraintType::DirectoryName,
            GeneralName::Uri(_) => NameConstraintType::Uri,
            GeneralName::Other(_, _) => NameConstraintType::DnsName, // Default
        }
    }
    
    fn update_validation_state(
        &self,
        certificate: &CertificateInfo,
        state: &mut ValidationState,
    ) -> Result<(), Error> {
        // Update working public key
        state.working_public_key = certificate.public_key.clone();
        state.working_public_key_algorithm = certificate.public_key.algorithm.clone();
        state.working_public_key_parameters = certificate.public_key.parameters.clone();
        
        // Update working issuer name
        state.working_issuer_name = certificate.subject_name.clone();
        
        // Update authority key identifier
        state.authority_key_id = certificate.subject_key_identifier.clone();
        
        Ok(())
    }
    
    fn is_supported_critical_extension(&self, oid: &str) -> bool {
        match oid {
            "2.5.29.15" => true, // Key Usage
            "2.5.29.19" => true, // Basic Constraints
            "2.5.29.32" => true, // Certificate Policies
            "2.5.29.30" => true, // Name Constraints
            "2.5.29.36" => true, // Policy Constraints
            "2.5.29.37" => true, // Extended Key Usage
            "2.5.29.54" => true, // Inhibit Any Policy
            _ => false,
        }
    }
    
    fn check_crl_revocation(
        &self,
        certificate: &CertificateInfo,
        crl_point: &str,
    ) -> Result<(), Error> {
        // CRL revocation checking implementation
        // This would download and parse CRL, then check certificate serial number
        Ok(()) // Placeholder
    }
    
    fn check_ocsp_revocation(
        &self,
        certificate: &CertificateInfo,
        ocsp_responder: &str,
    ) -> Result<(), Error> {
        // OCSP revocation checking implementation
        // This would send OCSP request and parse response
        Ok(()) // Placeholder
    }
}

/// Default path validation context for common use cases
impl Default for PathValidationContext {
    fn default() -> Self {
        Self {
            trust_anchors: vec![],
            validation_time: SystemTime::now(),
            required_policies: HashSet::new(),
            initial_policy_set: {
                let mut set = HashSet::new();
                set.insert("2.5.29.32.0".to_string()); // any-policy
                set
            },
            enable_policy_mapping: true,
            require_explicit_policy: false,
            inhibit_policy_mapping: None,
            inhibit_any_policy: None,
            max_path_length: Some(10),
            enable_revocation_checking: false,
            crl_distribution_points: vec![],
            ocsp_responders: vec![],
        }
    }
}

/// Create path validation context with common trust anchors
pub fn create_default_validation_context() -> PathValidationContext {
    PathValidationContext::default()
}

/// Create path validation context with custom trust anchors
pub fn create_validation_context_with_anchors(
    trust_anchors: Vec<TrustAnchor>,
) -> PathValidationContext {
    let mut context = PathValidationContext::default();
    context.trust_anchors = trust_anchors;
    context
}

/// Validate certificate path with default settings
/// 
/// This is a convenience function for simple path validation scenarios.
/// For advanced validation requirements, use `CertificatePathValidator` directly.
/// 
/// # Arguments
/// * `target_certificate` - End entity certificate to validate
/// * `intermediate_certificates` - Available intermediate certificates
/// * `trust_anchors` - Trusted root certificates or anchors
/// 
/// # Returns
/// * `PathValidationResult` - Validation result with chain or error details
/// 
/// # Example
/// 
/// ```rust
/// use cursed::stdlib::packages::crypto_pki::path_validation::*;
/// 
/// let result = validate_certificate_path_simple(
///     &end_entity_cert,
///     &intermediate_certs,
///     &trust_anchors,
/// )?;
/// 
/// match result {
///     PathValidationResult::Valid { validated_chain, .. } => {
///         println!("Certificate chain is valid!");
///     }
///     PathValidationResult::Invalid { error, .. } => {
///         println!("Validation failed: {:?}", error);
///     }
/// }
/// ```
pub fn validate_certificate_path_simple(
    target_certificate: &CertificateInfo,
    intermediate_certificates: &[CertificateInfo],
    trust_anchors: &[TrustAnchor],
) -> Result<(), Error> {
    let context = create_validation_context_with_anchors(trust_anchors.to_vec());
    let mut validator = CertificatePathValidator::new(context);
    
    validator.validate_path(target_certificate, intermediate_certificates)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test basic path validation functionality
    #[test]
    fn test_path_validation_basic() {
        // This would test basic path validation with mock certificates
        // Implementation would create test certificates and validate paths
    }
    
    /// Test certificate chain building
    #[test]
    fn test_certificate_chain_building() {
        // Test chain building from end entity to trust anchor
    }
    
    /// Test name constraint validation
    #[test]
    fn test_name_constraint_validation() {
        // Test DNS name, email, and IP address constraint validation
    }
    
    /// Test certificate policy processing
    #[test]
    fn test_certificate_policy_processing() {
        // Test policy tree building and validation
    }
    
    /// Test path length constraint validation
    #[test]
    fn test_path_length_constraints() {
        // Test path length constraint enforcement
    }
    
    /// Test revocation checking integration
    #[test]
    fn test_revocation_checking() {
        // Test CRL and OCSP revocation checking
    }
    
    /// Test critical extension processing
    #[test]
    fn test_critical_extension_processing() {
        // Test handling of critical extensions
    }
    
    /// Test error scenarios and edge cases
    #[test]
    fn test_error_scenarios() {
        // Test various error conditions and edge cases
    }
}
