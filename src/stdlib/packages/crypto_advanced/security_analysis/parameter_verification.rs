/// fr fr Cryptographic parameter verification module
/// 
/// This module provides comprehensive verification of cryptographic parameters
/// to ensure they meet current security standards and best practices.

use super::super::errors::*;
use super::super::CryptoParameters;
use std::collections::HashMap;

/// fr fr Result type for parameter verification
pub type ParameterResult<T> = Result<T, SecurityAnalysisError>;

/// fr fr Parameter verification result
#[derive(Debug, Clone)]
pub struct ParameterVerificationResult {
    pub algorithm: String,
    pub compliance_score: f64,
    pub security_level: SecurityLevel,
    pub violations: Vec<ParameterViolation>,
    pub recommendations: Vec<String>,
    pub standards_compliance: StandardsCompliance,
    pub estimated_security_bits: u32,
}

impl ParameterVerificationResult {
    /// slay Check if parameters are compliant with security standards
    pub fn is_compliant(&self) -> bool {
        self.compliance_score >= 0.8 && 
        !self.violations.iter().any(|v| v.severity >= ViolationSeverity::High) &&
        self.estimated_security_bits >= 128
    }

    /// slay Get critical violations requiring immediate attention
    pub fn get_critical_violations(&self) -> Vec<&ParameterViolation> {
        self.violations.iter()
            .filter(|v| v.severity >= ViolationSeverity::Critical)
            .collect()
    }

    /// slay Get all high or critical violations
    pub fn get_serious_violations(&self) -> Vec<&ParameterViolation> {
        self.violations.iter()
            .filter(|v| v.severity >= ViolationSeverity::High)
            .collect()
    }
}

/// fr fr Parameter violation detected during verification
#[derive(Debug, Clone)]
pub struct ParameterViolation {
    pub parameter_name: String,
    pub violation_type: ViolationType,
    pub severity: ViolationSeverity,
    pub description: String,
    pub recommendation: String,
    pub standard_reference: Option<String>,
}

/// fr fr Types of parameter violations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationType {
    KeySizeTooSmall,
    KeySizeInvalid,
    BlockSizeInvalid,
    IvSizeTooSmall,
    TagSizeTooSmall,
    RoundsInsufficient,
    WeakParameter,
    DeprecatedParameter,
    NonStandardParameter,
    SecurityDowngrade,
}

/// fr fr Violation severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ViolationSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// fr fr Security level assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    Insecure,
    Weak,
    Acceptable,
    Strong,
    Excellent,
}

impl SecurityLevel {
    /// slay Get security level description
    pub fn description(&self) -> &'static str {
        match self {
            SecurityLevel::Insecure => "Insecure - Immediate replacement required",
            SecurityLevel::Weak => "Weak - Not recommended for new applications",
            SecurityLevel::Acceptable => "Acceptable - Minimum security requirements met",
            SecurityLevel::Strong => "Strong - Good security for most applications",
            SecurityLevel::Excellent => "Excellent - High security for sensitive applications",
        }
    }

    /// slay Check if security level is adequate for production use
    pub fn is_production_ready(&self) -> bool {
        matches!(self, SecurityLevel::Acceptable | SecurityLevel::Strong | SecurityLevel::Excellent)
    }
}

/// fr fr Standards compliance assessment
#[derive(Debug, Clone)]
pub struct StandardsCompliance {
    pub nist_compliant: bool,
    pub fips_approved: bool,
    pub enisa_recommended: bool,
    pub bsi_approved: bool,
    pub suite_b_compliant: bool,
    pub commercial_national_security_algorithm: bool,
    pub compliance_notes: Vec<String>,
}

/// fr fr Parameter verifier
#[derive(Debug)]
pub struct ParameterVerifier {
    security_year: u32,
    minimum_security_bits: u32,
    strict_mode: bool,
    algorithm_registry: HashMap<String, AlgorithmProfile>,
}

/// fr fr Algorithm security profile
#[derive(Debug, Clone)]
pub struct AlgorithmProfile {
    pub min_key_size: usize,
    pub recommended_key_size: usize,
    pub max_key_size: usize,
    pub valid_key_sizes: Vec<usize>,
    pub min_block_size: Option<usize>,
    pub min_iv_size: Option<usize>,
    pub min_tag_size: Option<usize>,
    pub min_rounds: Option<usize>,
    pub security_bits_per_key_bit: f64,
    pub deprecated: bool,
    pub standards_approved: Vec<String>,
}

impl ParameterVerifier {
    /// slay Create new parameter verifier
    pub fn new() -> Self {
        let mut verifier = Self {
            security_year: 2024,
            minimum_security_bits: 128,
            strict_mode: false,
            algorithm_registry: HashMap::new(),
        };
        
        verifier.initialize_algorithm_profiles();
        verifier
    }

    /// slay Create with custom configuration
    pub fn with_config(security_year: u32, min_security_bits: u32, strict: bool) -> Self {
        let mut verifier = Self {
            security_year,
            minimum_security_bits: min_security_bits,
            strict_mode: strict,
            algorithm_registry: HashMap::new(),
        };
        
        verifier.initialize_algorithm_profiles();
        verifier
    }

    /// slay Verify cryptographic parameters
    pub fn verify_parameters(&self, params: &CryptoParameters) -> ParameterResult<ParameterVerificationResult> {
        let mut violations = Vec::new();
        let mut recommendations = Vec::new();

        // Get algorithm profile
        let profile = self.algorithm_registry.get(&params.algorithm.to_lowercase())
            .ok_or_else(|| SecurityAnalysisError::ParameterError(
                format!("Unknown algorithm: {}", params.algorithm)
            ))?;

        // Verify key size
        self.verify_key_size(params, profile, &mut violations, &mut recommendations);

        // Verify block size if applicable
        if let Some(block_size) = params.block_size {
            self.verify_block_size(block_size, profile, &mut violations, &mut recommendations);
        }

        // Verify IV size if applicable
        if let Some(iv_size) = params.iv_size {
            self.verify_iv_size(iv_size, profile, &mut violations, &mut recommendations);
        }

        // Verify tag size if applicable
        if let Some(tag_size) = params.tag_size {
            self.verify_tag_size(tag_size, profile, &mut violations, &mut recommendations);
        }

        // Verify rounds if applicable
        if let Some(rounds) = params.rounds {
            self.verify_rounds(rounds, profile, &mut violations, &mut recommendations);
        }

        // Check for deprecated algorithms
        if profile.deprecated {
            violations.push(ParameterViolation {
                parameter_name: "algorithm".to_string(),
                violation_type: ViolationType::DeprecatedParameter,
                severity: ViolationSeverity::High,
                description: format!("Algorithm {} is deprecated", params.algorithm),
                recommendation: "Use a modern, approved algorithm".to_string(),
                standard_reference: None,
            });
        }

        // Calculate compliance score
        let compliance_score = self.calculate_compliance_score(&violations, profile);
        
        // Determine security level
        let security_level = self.determine_security_level(compliance_score, &violations, profile);
        
        // Estimate security bits
        let estimated_security_bits = self.estimate_security_bits(params, profile);
        
        // Check standards compliance
        let standards_compliance = self.check_standards_compliance(params, profile, &violations);

        // Generate additional recommendations
        self.generate_general_recommendations(params, profile, &mut recommendations);

        Ok(ParameterVerificationResult {
            algorithm: params.algorithm.clone(),
            compliance_score,
            security_level,
            violations,
            recommendations,
            standards_compliance,
            estimated_security_bits,
        })
    }

    /// slay Verify multiple parameter sets for comparison
    pub fn verify_multiple_parameters(&self, param_sets: &[CryptoParameters]) 
        -> ParameterResult<Vec<ParameterVerificationResult>> {
        param_sets.iter()
            .map(|params| self.verify_parameters(params))
            .collect()
    }

    /// slay Get recommended parameters for an algorithm
    pub fn get_recommended_parameters(&self, algorithm: &str) -> ParameterResult<CryptoParameters> {
        let profile = self.algorithm_registry.get(&algorithm.to_lowercase())
            .ok_or_else(|| SecurityAnalysisError::ParameterError(
                format!("Unknown algorithm: {}", algorithm)
            ))?;

        Ok(CryptoParameters {
            algorithm: algorithm.to_string(),
            key_size: profile.recommended_key_size,
            block_size: profile.min_block_size,
            iv_size: profile.min_iv_size,
            tag_size: profile.min_tag_size,
            rounds: profile.min_rounds,
            custom_params: HashMap::new(),
        })
    }

    /// slay Initialize algorithm security profiles
    fn initialize_algorithm_profiles(&mut self) {
        // AES profiles
        self.algorithm_registry.insert("aes".to_string(), AlgorithmProfile {
            min_key_size: 128,
            recommended_key_size: 256,
            max_key_size: 256,
            valid_key_sizes: vec![128, 192, 256],
            min_block_size: Some(128),
            min_iv_size: Some(96), // For GCM mode
            min_tag_size: Some(96), // For authenticated encryption
            min_rounds: Some(10), // AES-128
            security_bits_per_key_bit: 1.0,
            deprecated: false,
            standards_approved: vec!["FIPS-197".to_string(), "NIST".to_string()],
        });

        // ChaCha20 profiles
        self.algorithm_registry.insert("chacha20".to_string(), AlgorithmProfile {
            min_key_size: 256,
            recommended_key_size: 256,
            max_key_size: 256,
            valid_key_sizes: vec![256],
            min_block_size: Some(512),
            min_iv_size: Some(96),
            min_tag_size: Some(128), // For Poly1305
            min_rounds: Some(20),
            security_bits_per_key_bit: 1.0,
            deprecated: false,
            standards_approved: vec!["RFC-8439".to_string()],
        });

        // RSA profiles
        self.algorithm_registry.insert("rsa".to_string(), AlgorithmProfile {
            min_key_size: 2048,
            recommended_key_size: 3072,
            max_key_size: 8192,
            valid_key_sizes: vec![2048, 3072, 4096, 8192],
            min_block_size: None,
            min_iv_size: None,
            min_tag_size: None,
            min_rounds: None,
            security_bits_per_key_bit: 0.5, // RSA has lower security per bit
            deprecated: false,
            standards_approved: vec!["FIPS-186-4".to_string(), "PKCS#1".to_string()],
        });

        // ECC profiles
        self.algorithm_registry.insert("ecdsa".to_string(), AlgorithmProfile {
            min_key_size: 256,
            recommended_key_size: 384,
            max_key_size: 521,
            valid_key_sizes: vec![256, 384, 521],
            min_block_size: None,
            min_iv_size: None,
            min_tag_size: None,
            min_rounds: None,
            security_bits_per_key_bit: 0.5, // ECC efficiency
            deprecated: false,
            standards_approved: vec!["FIPS-186-4".to_string(), "SEC1".to_string()],
        });

        // Ed25519 profiles
        self.algorithm_registry.insert("ed25519".to_string(), AlgorithmProfile {
            min_key_size: 256,
            recommended_key_size: 256,
            max_key_size: 256,
            valid_key_sizes: vec![256],
            min_block_size: None,
            min_iv_size: None,
            min_tag_size: None,
            min_rounds: None,
            security_bits_per_key_bit: 0.5,
            deprecated: false,
            standards_approved: vec!["RFC-8032".to_string()],
        });

        // SHA family
        self.algorithm_registry.insert("sha256".to_string(), AlgorithmProfile {
            min_key_size: 256,
            recommended_key_size: 256,
            max_key_size: 256,
            valid_key_sizes: vec![256],
            min_block_size: Some(512),
            min_iv_size: None,
            min_tag_size: Some(256),
            min_rounds: Some(64),
            security_bits_per_key_bit: 0.5,
            deprecated: false,
            standards_approved: vec!["FIPS-180-4".to_string()],
        });

        self.algorithm_registry.insert("sha384".to_string(), AlgorithmProfile {
            min_key_size: 384,
            recommended_key_size: 384,
            max_key_size: 384,
            valid_key_sizes: vec![384],
            min_block_size: Some(1024),
            min_iv_size: None,
            min_tag_size: Some(384),
            min_rounds: Some(80),
            security_bits_per_key_bit: 0.67,
            deprecated: false,
            standards_approved: vec!["FIPS-180-4".to_string()],
        });

        self.algorithm_registry.insert("sha512".to_string(), AlgorithmProfile {
            min_key_size: 512,
            recommended_key_size: 512,
            max_key_size: 512,
            valid_key_sizes: vec![512],
            min_block_size: Some(1024),
            min_iv_size: None,
            min_tag_size: Some(512),
            min_rounds: Some(80),
            security_bits_per_key_bit: 0.5,
            deprecated: false,
            standards_approved: vec!["FIPS-180-4".to_string()],
        });

        // Deprecated algorithms
        self.algorithm_registry.insert("md5".to_string(), AlgorithmProfile {
            min_key_size: 128,
            recommended_key_size: 128,
            max_key_size: 128,
            valid_key_sizes: vec![128],
            min_block_size: Some(512),
            min_iv_size: None,
            min_tag_size: Some(128),
            min_rounds: Some(64),
            security_bits_per_key_bit: 0.0, // Broken
            deprecated: true,
            standards_approved: vec![],
        });

        self.algorithm_registry.insert("sha1".to_string(), AlgorithmProfile {
            min_key_size: 160,
            recommended_key_size: 160,
            max_key_size: 160,
            valid_key_sizes: vec![160],
            min_block_size: Some(512),
            min_iv_size: None,
            min_tag_size: Some(160),
            min_rounds: Some(80),
            security_bits_per_key_bit: 0.0, // Effectively broken
            deprecated: true,
            standards_approved: vec![],
        });

        self.algorithm_registry.insert("des".to_string(), AlgorithmProfile {
            min_key_size: 56,
            recommended_key_size: 56,
            max_key_size: 56,
            valid_key_sizes: vec![56],
            min_block_size: Some(64),
            min_iv_size: Some(64),
            min_tag_size: None,
            min_rounds: Some(16),
            security_bits_per_key_bit: 0.0, // Broken
            deprecated: true,
            standards_approved: vec![],
        });

        self.algorithm_registry.insert("3des".to_string(), AlgorithmProfile {
            min_key_size: 112,
            recommended_key_size: 168,
            max_key_size: 168,
            valid_key_sizes: vec![112, 168],
            min_block_size: Some(64),
            min_iv_size: Some(64),
            min_tag_size: None,
            min_rounds: Some(48),
            security_bits_per_key_bit: 0.5,
            deprecated: true, // Being phased out
            standards_approved: vec!["FIPS-46-3".to_string()],
        });
    }

    /// slay Verify key size parameters
    fn verify_key_size(&self, params: &CryptoParameters, profile: &AlgorithmProfile,
                      violations: &mut Vec<ParameterViolation>, recommendations: &mut Vec<String>) {
        let key_size_bits = params.key_size;

        // Check if key size is too small
        if key_size_bits < profile.min_key_size {
            violations.push(ParameterViolation {
                parameter_name: "key_size".to_string(),
                violation_type: ViolationType::KeySizeTooSmall,
                severity: if key_size_bits < 128 { ViolationSeverity::Critical } else { ViolationSeverity::High },
                description: format!("Key size {} bits is below minimum {} bits for {}",
                                   key_size_bits, profile.min_key_size, params.algorithm),
                recommendation: format!("Use at least {} bits, {} bits recommended",
                                      profile.min_key_size, profile.recommended_key_size),
                standard_reference: Some("NIST SP 800-57".to_string()),
            });
        }

        // Check if key size is invalid
        if !profile.valid_key_sizes.is_empty() && !profile.valid_key_sizes.contains(&key_size_bits) {
            violations.push(ParameterViolation {
                parameter_name: "key_size".to_string(),
                violation_type: ViolationType::KeySizeInvalid,
                severity: ViolationSeverity::Medium,
                description: format!("Key size {} bits is not a standard size for {}",
                                   key_size_bits, params.algorithm),
                recommendation: format!("Use standard key sizes: {:?}", profile.valid_key_sizes),
                standard_reference: None,
            });
        }

        // Recommend stronger key sizes
        if key_size_bits < profile.recommended_key_size {
            recommendations.push(format!("Consider using {} bits for enhanced security",
                                        profile.recommended_key_size));
        }

        // Check against current year security requirements
        let required_bits = self.get_required_security_bits_for_year(self.security_year);
        let effective_security = (key_size_bits as f64 * profile.security_bits_per_key_bit) as u32;
        
        if effective_security < required_bits {
            violations.push(ParameterViolation {
                parameter_name: "key_size".to_string(),
                violation_type: ViolationType::SecurityDowngrade,
                severity: ViolationSeverity::Medium,
                description: format!("Effective security {} bits insufficient for year {}",
                                   effective_security, self.security_year),
                recommendation: format!("Use key size providing at least {} bits of security",
                                      required_bits),
                standard_reference: Some("NIST SP 800-57".to_string()),
            });
        }
    }

    /// slay Verify block size parameters
    fn verify_block_size(&self, block_size: usize, profile: &AlgorithmProfile,
                        violations: &mut Vec<ParameterViolation>, recommendations: &mut Vec<String>) {
        if let Some(min_block_size) = profile.min_block_size {
            if block_size < min_block_size {
                violations.push(ParameterViolation {
                    parameter_name: "block_size".to_string(),
                    violation_type: ViolationType::BlockSizeInvalid,
                    severity: ViolationSeverity::Medium,
                    description: format!("Block size {} bits is below minimum {} bits",
                                       block_size, min_block_size),
                    recommendation: format!("Use block size of at least {} bits", min_block_size),
                    standard_reference: None,
                });
            }
        }

        // Block size should be a power of 2 for most algorithms
        if block_size > 0 && (block_size & (block_size - 1)) != 0 {
            recommendations.push("Consider using a power-of-2 block size for optimal performance".to_string());
        }
    }

    /// slay Verify IV size parameters
    fn verify_iv_size(&self, iv_size: usize, profile: &AlgorithmProfile,
                     violations: &mut Vec<ParameterViolation>, _recommendations: &mut Vec<String>) {
        if let Some(min_iv_size) = profile.min_iv_size {
            if iv_size < min_iv_size {
                violations.push(ParameterViolation {
                    parameter_name: "iv_size".to_string(),
                    violation_type: ViolationType::IvSizeTooSmall,
                    severity: ViolationSeverity::High,
                    description: format!("IV size {} bits is below minimum {} bits",
                                       iv_size, min_iv_size),
                    recommendation: format!("Use IV size of at least {} bits", min_iv_size),
                    standard_reference: Some("NIST SP 800-38D".to_string()),
                });
            }
        }

        // IV should not be too small for security
        if iv_size < 64 {
            violations.push(ParameterViolation {
                parameter_name: "iv_size".to_string(),
                violation_type: ViolationType::IvSizeTooSmall,
                severity: ViolationSeverity::Critical,
                description: "IV size is dangerously small and may lead to security vulnerabilities".to_string(),
                recommendation: "Use IV size of at least 96 bits for authenticated encryption".to_string(),
                standard_reference: Some("NIST SP 800-38D".to_string()),
            });
        }
    }

    /// slay Verify tag size parameters
    fn verify_tag_size(&self, tag_size: usize, profile: &AlgorithmProfile,
                      violations: &mut Vec<ParameterViolation>, _recommendations: &mut Vec<String>) {
        if let Some(min_tag_size) = profile.min_tag_size {
            if tag_size < min_tag_size {
                violations.push(ParameterViolation {
                    parameter_name: "tag_size".to_string(),
                    violation_type: ViolationType::TagSizeTooSmall,
                    severity: ViolationSeverity::High,
                    description: format!("Authentication tag size {} bits is below minimum {} bits",
                                       tag_size, min_tag_size),
                    recommendation: format!("Use tag size of at least {} bits", min_tag_size),
                    standard_reference: Some("NIST SP 800-38D".to_string()),
                });
            }
        }

        // Authentication tags should provide sufficient security
        if tag_size < 96 {
            violations.push(ParameterViolation {
                parameter_name: "tag_size".to_string(),
                violation_type: ViolationType::TagSizeTooSmall,
                severity: ViolationSeverity::Critical,
                description: "Authentication tag is too small and vulnerable to forgery attacks".to_string(),
                recommendation: "Use authentication tag of at least 96 bits, 128 bits recommended".to_string(),
                standard_reference: Some("NIST SP 800-38D".to_string()),
            });
        }
    }

    /// slay Verify rounds parameters
    fn verify_rounds(&self, rounds: usize, profile: &AlgorithmProfile,
                    violations: &mut Vec<ParameterViolation>, _recommendations: &mut Vec<String>) {
        if let Some(min_rounds) = profile.min_rounds {
            if rounds < min_rounds {
                violations.push(ParameterViolation {
                    parameter_name: "rounds".to_string(),
                    violation_type: ViolationType::RoundsInsufficient,
                    severity: ViolationSeverity::High,
                    description: format!("Number of rounds {} is below minimum {} for security",
                                       rounds, min_rounds),
                    recommendation: format!("Use at least {} rounds", min_rounds),
                    standard_reference: None,
                });
            }
        }
    }

    /// slay Calculate compliance score
    fn calculate_compliance_score(&self, violations: &[ParameterViolation], _profile: &AlgorithmProfile) -> f64 {
        let mut score = 1.0;

        for violation in violations {
            let penalty = match violation.severity {
                ViolationSeverity::Critical => 0.5,
                ViolationSeverity::High => 0.3,
                ViolationSeverity::Medium => 0.15,
                ViolationSeverity::Low => 0.05,
                ViolationSeverity::Info => 0.01,
            };
            score -= penalty;
        }

        score.max(0.0).min(1.0)
    }

    /// slay Determine security level
    fn determine_security_level(&self, compliance_score: f64, violations: &[ParameterViolation],
                               profile: &AlgorithmProfile) -> SecurityLevel {
        let has_critical = violations.iter().any(|v| v.severity >= ViolationSeverity::Critical);
        let has_high = violations.iter().any(|v| v.severity >= ViolationSeverity::High);

        if profile.deprecated || has_critical {
            SecurityLevel::Insecure
        } else if has_high || compliance_score < 0.6 {
            SecurityLevel::Weak
        } else if compliance_score < 0.8 {
            SecurityLevel::Acceptable
        } else if compliance_score < 0.95 {
            SecurityLevel::Strong
        } else {
            SecurityLevel::Excellent
        }
    }

    /// slay Estimate effective security bits
    fn estimate_security_bits(&self, params: &CryptoParameters, profile: &AlgorithmProfile) -> u32 {
        let base_security = (params.key_size as f64 * profile.security_bits_per_key_bit) as u32;
        
        // Apply penalties for various issues
        let mut effective_security = base_security;

        // Penalty for deprecated algorithms
        if profile.deprecated {
            effective_security = effective_security / 4; // Severe penalty
        }

        // Penalty for small IV
        if let Some(iv_size) = params.iv_size {
            if iv_size < 96 {
                effective_security = effective_security.saturating_sub(32);
            }
        }

        // Penalty for small authentication tag
        if let Some(tag_size) = params.tag_size {
            if tag_size < 96 {
                effective_security = effective_security.saturating_sub(16);
            }
        }

        effective_security
    }

    /// slay Check standards compliance
    fn check_standards_compliance(&self, params: &CryptoParameters, profile: &AlgorithmProfile,
                                 violations: &[ParameterViolation]) -> StandardsCompliance {
        let has_serious_violations = violations.iter().any(|v| v.severity >= ViolationSeverity::High);
        
        StandardsCompliance {
            nist_compliant: !has_serious_violations && profile.standards_approved.iter()
                .any(|s| s.contains("NIST") || s.contains("FIPS")),
            fips_approved: !profile.deprecated && profile.standards_approved.iter()
                .any(|s| s.contains("FIPS")),
            enisa_recommended: params.key_size >= 256 && !profile.deprecated,
            bsi_approved: params.key_size >= 256 && !has_serious_violations,
            suite_b_compliant: self.is_suite_b_compliant(params, profile),
            commercial_national_security_algorithm: self.is_cnsa_compliant(params, profile),
            compliance_notes: self.generate_compliance_notes(params, profile, violations),
        }
    }

    /// slay Check Suite B compliance
    fn is_suite_b_compliant(&self, params: &CryptoParameters, _profile: &AlgorithmProfile) -> bool {
        // Suite B algorithms: AES (128/256), ECDH/ECDSA (P-256/P-384), SHA-256/384
        match params.algorithm.to_lowercase().as_str() {
            "aes" => params.key_size >= 128,
            "ecdsa" | "ecdh" => params.key_size >= 256,
            "sha256" | "sha384" => true,
            _ => false,
        }
    }

    /// slay Check CNSA compliance
    fn is_cnsa_compliant(&self, params: &CryptoParameters, _profile: &AlgorithmProfile) -> bool {
        // Commercial National Security Algorithm requirements
        match params.algorithm.to_lowercase().as_str() {
            "aes" => params.key_size >= 256,
            "ecdsa" | "ecdh" => params.key_size >= 384,
            "rsa" => params.key_size >= 3072,
            "sha384" | "sha512" => true,
            _ => false,
        }
    }

    /// slay Generate compliance notes
    fn generate_compliance_notes(&self, _params: &CryptoParameters, profile: &AlgorithmProfile,
                                violations: &[ParameterViolation]) -> Vec<String> {
        let mut notes = Vec::new();

        if profile.deprecated {
            notes.push("Algorithm is deprecated and should be replaced".to_string());
        }

        if !profile.standards_approved.is_empty() {
            notes.push(format!("Standards approved: {}", profile.standards_approved.join(", ")));
        }

        if !violations.is_empty() {
            notes.push(format!("{} violations detected", violations.len()));
        }

        notes
    }

    /// slay Generate general recommendations
    fn generate_general_recommendations(&self, params: &CryptoParameters, profile: &AlgorithmProfile,
                                       recommendations: &mut Vec<String>) {
        if params.key_size < profile.recommended_key_size {
            recommendations.push(format!("Consider upgrading to {} bit keys for future-proofing",
                                        profile.recommended_key_size));
        }

        if self.security_year >= 2030 && params.key_size < 256 {
            recommendations.push("Consider post-quantum cryptography for long-term security".to_string());
        }

        if self.strict_mode {
            recommendations.push("Strict mode enabled - consider using maximum security parameters".to_string());
        }
    }

    /// slay Get required security bits for a given year
    fn get_required_security_bits_for_year(&self, year: u32) -> u32 {
        match year {
            2024..=2029 => 112,
            2030..=2039 => 128,
            2040..=2049 => 192,
            2050.. => 256,
            _ => 112, // Default for past years
        }
    }

    /// slay List all supported algorithms
    pub fn list_supported_algorithms(&self) -> Vec<String> {
        self.algorithm_registry.keys().cloned().collect()
    }

    /// slay Get algorithm profile
    pub fn get_algorithm_profile(&self, algorithm: &str) -> Option<&AlgorithmProfile> {
        self.algorithm_registry.get(&algorithm.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_verifier_creation() {
        let verifier = ParameterVerifier::new();
        assert_eq!(verifier.security_year, 2024);
        assert_eq!(verifier.minimum_security_bits, 128);
        assert!(!verifier.strict_mode);
        assert!(!verifier.algorithm_registry.is_empty());
    }

    #[test]
    fn test_aes_parameter_verification() {
        let verifier = ParameterVerifier::new();
        let params = CryptoParameters {
            algorithm: "AES".to_string(),
            key_size: 256,
            block_size: Some(128),
            iv_size: Some(96),
            tag_size: Some(128),
            rounds: Some(14),
            custom_params: HashMap::new(),
        };

        let result = verifier.verify_parameters(&params);
        assert!(result.is_ok());

        let verification = result.unwrap();
        assert!(verification.is_compliant());
        assert!(verification.security_level.is_production_ready());
    }

    #[test]
    fn test_weak_parameter_detection() {
        let verifier = ParameterVerifier::new();
        let params = CryptoParameters {
            algorithm: "AES".to_string(),
            key_size: 64, // Too small
            block_size: Some(128),
            iv_size: Some(32), // Too small
            tag_size: Some(64), // Too small
            rounds: Some(5), // Too few
            custom_params: HashMap::new(),
        };

        let result = verifier.verify_parameters(&params);
        assert!(result.is_ok());

        let verification = result.unwrap();
        assert!(!verification.is_compliant());
        assert!(!verification.violations.is_empty());
        assert!(verification.violations.iter().any(|v| v.severity >= ViolationSeverity::High));
    }

    #[test]
    fn test_deprecated_algorithm_detection() {
        let verifier = ParameterVerifier::new();
        let params = CryptoParameters {
            algorithm: "MD5".to_string(),
            key_size: 128,
            block_size: Some(512),
            iv_size: None,
            tag_size: Some(128),
            rounds: Some(64),
            custom_params: HashMap::new(),
        };

        let result = verifier.verify_parameters(&params);
        assert!(result.is_ok());

        let verification = result.unwrap();
        assert!(!verification.is_compliant());
        assert_eq!(verification.security_level, SecurityLevel::Insecure);
        assert!(verification.violations.iter().any(|v| v.violation_type == ViolationType::DeprecatedParameter));
    }

    #[test]
    fn test_recommended_parameters() {
        let verifier = ParameterVerifier::new();
        let recommended = verifier.get_recommended_parameters("AES");
        
        assert!(recommended.is_ok());
        let params = recommended.unwrap();
        assert_eq!(params.algorithm, "AES");
        assert_eq!(params.key_size, 256);
    }

    #[test]
    fn test_security_level_descriptions() {
        assert_eq!(SecurityLevel::Insecure.description(), "Insecure - Immediate replacement required");
        assert_eq!(SecurityLevel::Excellent.description(), "Excellent - High security for sensitive applications");
        
        assert!(!SecurityLevel::Insecure.is_production_ready());
        assert!(!SecurityLevel::Weak.is_production_ready());
        assert!(SecurityLevel::Acceptable.is_production_ready());
        assert!(SecurityLevel::Strong.is_production_ready());
        assert!(SecurityLevel::Excellent.is_production_ready());
    }

    #[test]
    fn test_standards_compliance() {
        let verifier = ParameterVerifier::new();
        let good_aes_params = CryptoParameters {
            algorithm: "AES".to_string(),
            key_size: 256,
            block_size: Some(128),
            iv_size: Some(96),
            tag_size: Some(128),
            rounds: Some(14),
            custom_params: HashMap::new(),
        };

        let result = verifier.verify_parameters(&good_aes_params).unwrap();
        assert!(result.standards_compliance.nist_compliant);
        assert!(result.standards_compliance.fips_approved);
    }

    #[test]
    fn test_security_bits_estimation() {
        let verifier = ParameterVerifier::new();
        
        // AES-256 should provide 256 bits of security
        let aes_params = CryptoParameters {
            algorithm: "AES".to_string(),
            key_size: 256,
            block_size: Some(128),
            iv_size: Some(96),
            tag_size: Some(128),
            rounds: Some(14),
            custom_params: HashMap::new(),
        };

        let result = verifier.verify_parameters(&aes_params).unwrap();
        assert_eq!(result.estimated_security_bits, 256);

        // RSA-2048 should provide less security per bit
        let rsa_params = CryptoParameters {
            algorithm: "RSA".to_string(),
            key_size: 2048,
            block_size: None,
            iv_size: None,
            tag_size: None,
            rounds: None,
            custom_params: HashMap::new(),
        };

        let result = verifier.verify_parameters(&rsa_params).unwrap();
        assert!(result.estimated_security_bits < 2048);
        assert!(result.estimated_security_bits >= 112); // Should be around 112 bits
    }
}
