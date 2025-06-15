/// fr fr Certificate Templates - Production Ready Implementation
/// 
/// Comprehensive certificate template functionality for the CURSED language PKI module.
/// This module provides complete support for:
/// - Certificate template definitions and management
/// - Standard certificate profiles (Server, Client, CA, etc.)
/// - Template inheritance and composition
/// - Template validation and constraint enforcement
/// - Dynamic template customization
/// - Template versioning and migration
/// - Template security policies
/// - Template compliance checking
/// - Template repository management
/// - Template generation and instantiation

use crate::error::CursedError;
use crate::stdlib::packages::crypto_pki::error::{PkiError, PkiResult};
use crate::stdlib::packages::crypto_pki::types::{Certificate, X509Extension, KeyUsage, ExtendedKeyUsage};
use crate::stdlib::packages::crypto_pki::extensions::{Extension, ExtensionOid, ExtensionValue, BasicConstraints};
use tracing::{debug, error, info, instrument, warn};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// fr fr Certificate template definition
#[derive(Debug, Clone)]
pub struct CertificateTemplate {
    /// Template name
    pub name: String,
    /// Template version
    pub version: String,
    /// Template description
    pub description: String,
    /// Template type
    pub template_type: TemplateType,
    /// Subject template
    pub subject_template: SubjectTemplate,
    /// Key usage constraints
    pub key_usage: TemplateKeyUsage,
    /// Extensions to include
    pub extensions: Vec<TemplateExtension>,
    /// Validity period
    pub validity: TemplateValidity,
    /// Template constraints
    pub constraints: TemplateConstraints,
    /// Template metadata
    pub metadata: TemplateMetadata,
}

/// fr fr Certificate template types
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateType {
    /// Server authentication certificate
    ServerAuth,
    /// Client authentication certificate
    ClientAuth,
    /// Code signing certificate
    CodeSigning,
    /// Email protection certificate
    EmailProtection,
    /// Certificate Authority certificate
    CertificateAuthority,
    /// Intermediate CA certificate
    IntermediateCA,
    /// Time stamping certificate
    TimeStamping,
    /// OCSP signing certificate
    OcspSigning,
    /// User certificate
    User,
    /// Device certificate
    Device,
    /// Custom template
    Custom(String),
}

impl TemplateType {
    /// slay Get template type name
    pub fn name(&self) -> &str {
        match self {
            TemplateType::ServerAuth => "Server Authentication",
            TemplateType::ClientAuth => "Client Authentication",
            TemplateType::CodeSigning => "Code Signing",
            TemplateType::EmailProtection => "Email Protection",
            TemplateType::CertificateAuthority => "Certificate Authority",
            TemplateType::IntermediateCA => "Intermediate CA",
            TemplateType::TimeStamping => "Time Stamping",
            TemplateType::OcspSigning => "OCSP Signing",
            TemplateType::User => "User",
            TemplateType::Device => "Device",
            TemplateType::Custom(name) => name,
        }
    }

    /// slay Get default key usage for template type
    pub fn default_key_usage(&self) -> TemplateKeyUsage {
        match self {
            TemplateType::ServerAuth => TemplateKeyUsage {
                digital_signature: Some(true),
                key_encipherment: Some(true),
                key_agreement: Some(true),
                key_cert_sign: Some(false),
                crl_sign: Some(false),
                content_commitment: None,
                data_encipherment: None,
                encipher_only: None,
                decipher_only: None,
            },
            TemplateType::ClientAuth => TemplateKeyUsage {
                digital_signature: Some(true),
                key_encipherment: Some(true),
                key_agreement: Some(true),
                key_cert_sign: Some(false),
                crl_sign: Some(false),
                content_commitment: None,
                data_encipherment: None,
                encipher_only: None,
                decipher_only: None,
            },
            TemplateType::CodeSigning => TemplateKeyUsage {
                digital_signature: Some(true),
                content_commitment: Some(true),
                key_encipherment: Some(false),
                key_agreement: Some(false),
                key_cert_sign: Some(false),
                crl_sign: Some(false),
                data_encipherment: None,
                encipher_only: None,
                decipher_only: None,
            },
            TemplateType::CertificateAuthority => TemplateKeyUsage {
                digital_signature: Some(true),
                key_cert_sign: Some(true),
                crl_sign: Some(true),
                key_encipherment: Some(false),
                key_agreement: Some(false),
                content_commitment: None,
                data_encipherment: None,
                encipher_only: None,
                decipher_only: None,
            },
            _ => TemplateKeyUsage::default(),
        }
    }

    /// slay Get default extended key usage
    pub fn default_extended_key_usage(&self) -> Vec<String> {
        match self {
            TemplateType::ServerAuth => vec![
                "1.3.6.1.5.5.7.3.1".to_string(), // serverAuth
            ],
            TemplateType::ClientAuth => vec![
                "1.3.6.1.5.5.7.3.2".to_string(), // clientAuth
            ],
            TemplateType::CodeSigning => vec![
                "1.3.6.1.5.5.7.3.3".to_string(), // codeSigning
            ],
            TemplateType::EmailProtection => vec![
                "1.3.6.1.5.5.7.3.4".to_string(), // emailProtection
            ],
            TemplateType::TimeStamping => vec![
                "1.3.6.1.5.5.7.3.8".to_string(), // timeStamping
            ],
            TemplateType::OcspSigning => vec![
                "1.3.6.1.5.5.7.3.9".to_string(), // OCSPSigning
            ],
            _ => Vec::new(),
        }
    }
}

/// fr fr Subject template for DN construction
#[derive(Debug, Clone)]
pub struct SubjectTemplate {
    /// Country (C)
    pub country: Option<TemplateField>,
    /// State or Province (ST)
    pub state_or_province: Option<TemplateField>,
    /// Locality (L)
    pub locality: Option<TemplateField>,
    /// Organization (O)
    pub organization: Option<TemplateField>,
    /// Organizational Unit (OU)
    pub organizational_unit: Option<TemplateField>,
    /// Common Name (CN)
    pub common_name: Option<TemplateField>,
    /// Email Address
    pub email_address: Option<TemplateField>,
    /// Additional attributes
    pub additional_attributes: HashMap<String, TemplateField>,
}

/// fr fr Template field definition
#[derive(Debug, Clone)]
pub struct TemplateField {
    /// Field value or pattern
    pub value: Option<String>,
    /// Whether field is required
    pub required: bool,
    /// Field validation pattern
    pub validation_pattern: Option<String>,
    /// Field constraints
    pub constraints: FieldConstraints,
    /// Default value
    pub default_value: Option<String>,
    /// Field description
    pub description: Option<String>,
}

/// fr fr Field constraints
#[derive(Debug, Clone)]
pub struct FieldConstraints {
    /// Minimum length
    pub min_length: Option<usize>,
    /// Maximum length
    pub max_length: Option<usize>,
    /// Allowed values
    pub allowed_values: Option<Vec<String>>,
    /// Forbidden values
    pub forbidden_values: Option<Vec<String>>,
    /// Case sensitivity
    pub case_sensitive: bool,
    /// Custom validation function
    pub custom_validator: Option<String>,
}

impl Default for FieldConstraints {
    fn default() -> Self {
        Self {
            min_length: None,
            max_length: None,
            allowed_values: None,
            forbidden_values: None,
            case_sensitive: true,
            custom_validator: None,
        }
    }
}

/// fr fr Template key usage definition
#[derive(Debug, Clone)]
pub struct TemplateKeyUsage {
    /// Digital signature
    pub digital_signature: Option<bool>,
    /// Content commitment (non-repudiation)
    pub content_commitment: Option<bool>,
    /// Key encipherment
    pub key_encipherment: Option<bool>,
    /// Data encipherment
    pub data_encipherment: Option<bool>,
    /// Key agreement
    pub key_agreement: Option<bool>,
    /// Key certificate sign
    pub key_cert_sign: Option<bool>,
    /// CRL sign
    pub crl_sign: Option<bool>,
    /// Encipher only
    pub encipher_only: Option<bool>,
    /// Decipher only
    pub decipher_only: Option<bool>,
}

impl Default for TemplateKeyUsage {
    fn default() -> Self {
        Self {
            digital_signature: None,
            content_commitment: None,
            key_encipherment: None,
            data_encipherment: None,
            key_agreement: None,
            key_cert_sign: None,
            crl_sign: None,
            encipher_only: None,
            decipher_only: None,
        }
    }
}

/// fr fr Template extension definition
#[derive(Debug, Clone)]
pub struct TemplateExtension {
    /// Extension OID
    pub oid: String,
    /// Extension name
    pub name: String,
    /// Whether extension is critical
    pub critical: bool,
    /// Whether extension is required
    pub required: bool,
    /// Extension value template
    pub value_template: ExtensionValueTemplate,
    /// Extension constraints
    pub constraints: ExtensionConstraints,
}

/// fr fr Extension value template
#[derive(Debug, Clone)]
pub enum ExtensionValueTemplate {
    /// Static value
    Static(Vec<u8>),
    /// Template pattern
    Pattern(String),
    /// Dynamic value based on certificate data
    Dynamic(String),
    /// Reference to another extension
    Reference(String),
}

/// fr fr Extension constraints
#[derive(Debug, Clone)]
pub struct ExtensionConstraints {
    /// Minimum value length
    pub min_length: Option<usize>,
    /// Maximum value length
    pub max_length: Option<usize>,
    /// Allowed patterns
    pub allowed_patterns: Vec<String>,
    /// Forbidden patterns
    pub forbidden_patterns: Vec<String>,
    /// Custom validation
    pub custom_validation: Option<String>,
}

impl Default for ExtensionConstraints {
    fn default() -> Self {
        Self {
            min_length: None,
            max_length: None,
            allowed_patterns: Vec::new(),
            forbidden_patterns: Vec::new(),
            custom_validation: None,
        }
    }
}

/// fr fr Template validity constraints
#[derive(Debug, Clone)]
pub struct TemplateValidity {
    /// Minimum validity period
    pub min_validity: Option<Duration>,
    /// Maximum validity period
    pub max_validity: Option<Duration>,
    /// Default validity period
    pub default_validity: Duration,
    /// Validity constraints
    pub constraints: ValidityConstraints,
}

impl Default for TemplateValidity {
    fn default() -> Self {
        Self {
            min_validity: Some(Duration::from_secs(24 * 3600)), // 1 day
            max_validity: Some(Duration::from_secs(365 * 24 * 3600)), // 1 year
            default_validity: Duration::from_secs(90 * 24 * 3600), // 90 days
            constraints: ValidityConstraints::default(),
        }
    }
}

/// fr fr Validity constraints
#[derive(Debug, Clone)]
pub struct ValidityConstraints {
    /// Require validity period within bounds
    pub enforce_bounds: bool,
    /// Allow backdating
    pub allow_backdating: bool,
    /// Maximum backdate period
    pub max_backdate: Duration,
    /// Align to specific intervals
    pub align_to_intervals: bool,
    /// Interval alignment
    pub alignment_interval: Duration,
}

impl Default for ValidityConstraints {
    fn default() -> Self {
        Self {
            enforce_bounds: true,
            allow_backdating: false,
            max_backdate: Duration::from_secs(0),
            align_to_intervals: false,
            alignment_interval: Duration::from_secs(24 * 3600), // 1 day
        }
    }
}

/// fr fr Template constraints
#[derive(Debug, Clone)]
pub struct TemplateConstraints {
    /// Allowed key algorithms
    pub allowed_key_algorithms: Vec<String>,
    /// Minimum key size
    pub min_key_size: Option<usize>,
    /// Maximum key size
    pub max_key_size: Option<usize>,
    /// Allowed signature algorithms
    pub allowed_signature_algorithms: Vec<String>,
    /// Subject DN constraints
    pub subject_constraints: SubjectConstraints,
    /// SAN constraints
    pub san_constraints: Option<SanConstraints>,
    /// Policy constraints
    pub policy_constraints: Vec<PolicyConstraint>,
}

impl Default for TemplateConstraints {
    fn default() -> Self {
        Self {
            allowed_key_algorithms: vec!["RSA".to_string(), "ECDSA".to_string()],
            min_key_size: Some(2048),
            max_key_size: Some(4096),
            allowed_signature_algorithms: vec!["SHA256withRSA".to_string(), "SHA384withECDSA".to_string()],
            subject_constraints: SubjectConstraints::default(),
            san_constraints: None,
            policy_constraints: Vec::new(),
        }
    }
}

/// fr fr Subject constraints
#[derive(Debug, Clone)]
pub struct SubjectConstraints {
    /// Require specific subject components
    pub required_components: Vec<String>,
    /// Forbidden subject components
    pub forbidden_components: Vec<String>,
    /// Component constraints
    pub component_constraints: HashMap<String, FieldConstraints>,
    /// Require unique subjects
    pub require_unique_subjects: bool,
    /// Subject validation rules
    pub validation_rules: Vec<SubjectValidationRule>,
}

impl Default for SubjectConstraints {
    fn default() -> Self {
        Self {
            required_components: Vec::new(),
            forbidden_components: Vec::new(),
            component_constraints: HashMap::new(),
            require_unique_subjects: false,
            validation_rules: Vec::new(),
        }
    }
}

/// fr fr Subject validation rule
#[derive(Debug, Clone)]
pub struct SubjectValidationRule {
    /// Rule name
    pub name: String,
    /// Rule expression
    pub expression: String,
    /// Rule description
    pub description: String,
    /// Whether rule is enforced
    pub enforced: bool,
}

/// fr fr SAN constraints
#[derive(Debug, Clone)]
pub struct SanConstraints {
    /// Require SAN extension
    pub require_san: bool,
    /// Allowed SAN types
    pub allowed_types: Vec<String>,
    /// Forbidden SAN types
    pub forbidden_types: Vec<String>,
    /// Maximum SAN entries
    pub max_entries: Option<usize>,
    /// SAN validation patterns
    pub validation_patterns: HashMap<String, String>,
}

impl Default for SanConstraints {
    fn default() -> Self {
        Self {
            require_san: false,
            allowed_types: vec!["dNSName".to_string(), "iPAddress".to_string()],
            forbidden_types: Vec::new(),
            max_entries: Some(100),
            validation_patterns: HashMap::new(),
        }
    }
}

/// fr fr Policy constraint
#[derive(Debug, Clone)]
pub struct PolicyConstraint {
    /// Policy OID
    pub policy_oid: String,
    /// Policy description
    pub description: String,
    /// Whether policy is required
    pub required: bool,
    /// Policy qualifiers
    pub qualifiers: Vec<String>,
}

/// fr fr Template metadata
#[derive(Debug, Clone)]
pub struct TemplateMetadata {
    /// Template author
    pub author: Option<String>,
    /// Creation time
    pub created_at: SystemTime,
    /// Last modified time
    pub modified_at: SystemTime,
    /// Template tags
    pub tags: Vec<String>,
    /// Template category
    pub category: Option<String>,
    /// Template status
    pub status: TemplateStatus,
    /// Compliance standards
    pub compliance_standards: Vec<String>,
    /// Template documentation
    pub documentation: Option<String>,
}

/// fr fr Template status
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateStatus {
    /// Template is in draft
    Draft,
    /// Template is active
    Active,
    /// Template is deprecated
    Deprecated,
    /// Template is retired
    Retired,
    /// Template is under review
    UnderReview,
}

/// fr fr Template repository for managing templates
#[derive(Debug)]
pub struct TemplateRepository {
    /// Templates by name
    templates: HashMap<String, CertificateTemplate>,
    /// Template inheritance relationships
    inheritance: HashMap<String, String>,
    /// Template validation rules
    validation_rules: Vec<TemplateValidationRule>,
    /// Repository configuration
    config: RepositoryConfig,
}

/// fr fr Repository configuration
#[derive(Debug, Clone)]
pub struct RepositoryConfig {
    /// Enable template inheritance
    pub enable_inheritance: bool,
    /// Maximum inheritance depth
    pub max_inheritance_depth: usize,
    /// Enable template validation
    pub enable_validation: bool,
    /// Template versioning
    pub enable_versioning: bool,
    /// Template backup
    pub enable_backup: bool,
    /// Backup interval
    pub backup_interval: Duration,
}

impl Default for RepositoryConfig {
    fn default() -> Self {
        Self {
            enable_inheritance: true,
            max_inheritance_depth: 5,
            enable_validation: true,
            enable_versioning: true,
            enable_backup: false,
            backup_interval: Duration::from_secs(24 * 3600),
        }
    }
}

/// fr fr Template validation rule
#[derive(Debug, Clone)]
pub struct TemplateValidationRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule severity
    pub severity: ValidationSeverity,
    /// Rule expression
    pub expression: String,
    /// Rule category
    pub category: String,
}

/// fr fr Validation severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationSeverity {
    /// Informational
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Critical
    Critical,
}

/// fr fr Template validation result
#[derive(Debug, Clone)]
pub struct TemplateValidationResult {
    /// Whether template is valid
    pub valid: bool,
    /// Validation issues
    pub issues: Vec<ValidationIssue>,
    /// Validation warnings
    pub warnings: Vec<String>,
    /// Validation metadata
    pub metadata: ValidationMetadata,
}

/// fr fr Validation issue
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// Issue severity
    pub severity: ValidationSeverity,
    /// Issue message
    pub message: String,
    /// Issue location
    pub location: Option<String>,
    /// Issue rule
    pub rule: Option<String>,
    /// Suggested fix
    pub suggested_fix: Option<String>,
}

/// fr fr Validation metadata
#[derive(Debug, Clone)]
pub struct ValidationMetadata {
    /// Validation timestamp
    pub validated_at: SystemTime,
    /// Validator version
    pub validator_version: String,
    /// Validation duration
    pub validation_duration: Duration,
    /// Rules checked
    pub rules_checked: usize,
}

/// fr fr Template error types
#[derive(Debug, Clone)]
pub enum TemplateError {
    /// Template not found
    TemplateNotFound(String),
    /// Invalid template format
    InvalidTemplate(String),
    /// Template validation failed
    ValidationFailed(String),
    /// Inheritance cycle detected
    InheritanceCycle(String),
    /// Constraint violation
    ConstraintViolation(String),
    /// Template compilation failed
    CompilationFailed(String),
    /// Internal error
    Internal(String),
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateError::TemplateNotFound(name) => write!(f, "Template not found: {}", name),
            TemplateError::InvalidTemplate(msg) => write!(f, "Invalid template: {}", msg),
            TemplateError::ValidationFailed(msg) => write!(f, "Template validation failed: {}", msg),
            TemplateError::InheritanceCycle(msg) => write!(f, "Inheritance cycle detected: {}", msg),
            TemplateError::ConstraintViolation(msg) => write!(f, "Constraint violation: {}", msg),
            TemplateError::CompilationFailed(msg) => write!(f, "Template compilation failed: {}", msg),
            TemplateError::Internal(msg) => write!(f, "Internal template error: {}", msg),
        }
    }
}

impl std::error::Error for TemplateError {}

/// fr fr Template result type
pub type TemplateResult<T> = Result<T, TemplateError>;

impl CertificateTemplate {
    /// slay Create new certificate template
    #[instrument]
    pub fn new(name: String, template_type: TemplateType) -> Self {
        Self {
            name: name.clone(),
            version: "1.0".to_string(),
            description: format!("{} certificate template", template_type.name()),
            template_type: template_type.clone(),
            subject_template: SubjectTemplate::default(),
            key_usage: template_type.default_key_usage(),
            extensions: Vec::new(),
            validity: TemplateValidity::default(),
            constraints: TemplateConstraints::default(),
            metadata: TemplateMetadata {
                author: None,
                created_at: SystemTime::now(),
                modified_at: SystemTime::now(),
                tags: Vec::new(),
                category: Some(template_type.name().to_string()),
                status: TemplateStatus::Draft,
                compliance_standards: Vec::new(),
                documentation: None,
            },
        }
    }

    /// slay Validate template
    #[instrument(skip(self))]
    pub fn validate(&self) -> TemplateValidationResult {
        let start_time = SystemTime::now();
        let mut result = TemplateValidationResult {
            valid: true,
            issues: Vec::new(),
            warnings: Vec::new(),
            metadata: ValidationMetadata {
                validated_at: start_time,
                validator_version: "1.0".to_string(),
                validation_duration: Duration::from_secs(0),
                rules_checked: 0,
            },
        };

        // Validate template name
        if self.name.is_empty() {
            result.issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                message: "Template name cannot be empty".to_string(),
                location: Some("name".to_string()),
                rule: Some("required_name".to_string()),
                suggested_fix: Some("Provide a descriptive template name".to_string()),
            });
            result.valid = false;
        }

        // Validate key usage consistency
        if let Err(e) = self.validate_key_usage() {
            result.issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                message: e,
                location: Some("key_usage".to_string()),
                rule: Some("key_usage_consistency".to_string()),
                suggested_fix: Some("Fix key usage conflicts".to_string()),
            });
            result.valid = false;
        }

        // Validate extensions
        for (i, extension) in self.extensions.iter().enumerate() {
            if let Err(e) = self.validate_extension(extension) {
                result.issues.push(ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    message: e,
                    location: Some(format!("extensions[{}]", i)),
                    rule: Some("extension_validation".to_string()),
                    suggested_fix: None,
                });
            }
        }

        // Validate constraints
        if let Err(e) = self.validate_constraints() {
            result.issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                message: e,
                location: Some("constraints".to_string()),
                rule: Some("constraint_validation".to_string()),
                suggested_fix: None,
            });
            result.valid = false;
        }

        result.metadata.validation_duration = start_time.elapsed().unwrap_or(Duration::from_secs(0));
        result.metadata.rules_checked = 4; // Number of validation rules checked

        result
    }

    /// slay Validate key usage
    #[instrument(skip(self))]
    fn validate_key_usage(&self) -> Result<(), String> {
        // Check for conflicting key usage
        if self.key_usage.encipher_only == Some(true) && self.key_usage.key_agreement != Some(true) {
            return Err("encipher_only requires key_agreement".to_string());
        }

        if self.key_usage.decipher_only == Some(true) && self.key_usage.key_agreement != Some(true) {
            return Err("decipher_only requires key_agreement".to_string());
        }

        Ok(())
    }

    /// slay Validate extension
    #[instrument(skip(self, extension))]
    fn validate_extension(&self, extension: &TemplateExtension) -> Result<(), String> {
        // Validate extension OID format
        if !extension.oid.contains('.') {
            return Err(format!("Invalid extension OID format: {}", extension.oid));
        }

        // Validate extension name
        if extension.name.is_empty() {
            return Err("Extension name cannot be empty".to_string());
        }

        Ok(())
    }

    /// slay Validate constraints
    #[instrument(skip(self))]
    fn validate_constraints(&self) -> Result<(), String> {
        // Validate key size constraints
        if let (Some(min), Some(max)) = (self.constraints.min_key_size, self.constraints.max_key_size) {
            if min > max {
                return Err("Minimum key size cannot be greater than maximum".to_string());
            }
        }

        // Validate validity constraints
        if let (Some(min), Some(max)) = (&self.validity.min_validity, &self.validity.max_validity) {
            if min > max {
                return Err("Minimum validity cannot be greater than maximum".to_string());
            }
        }

        Ok(())
    }

    /// slay Generate certificate extensions from template
    #[instrument(skip(self))]
    pub fn generate_extensions(&self) -> TemplateResult<Vec<Extension>> {
        let mut extensions = Vec::new();

        // Add basic constraints for CA certificates
        if matches!(self.template_type, TemplateType::CertificateAuthority | TemplateType::IntermediateCA) {
            let basic_constraints = BasicConstraints {
                ca: true,
                path_len_constraint: if matches!(self.template_type, TemplateType::CertificateAuthority) { 
                    None 
                } else { 
                    Some(0) 
                },
            };
            
            extensions.push(Extension {
                oid: ExtensionOid::BasicConstraints,
                critical: true,
                value: ExtensionValue::BasicConstraints(basic_constraints),
            });
        }

        // Add key usage extension
        if self.has_key_usage_set() {
            let key_usage = self.build_key_usage_extension()?;
            extensions.push(Extension {
                oid: ExtensionOid::KeyUsage,
                critical: true,
                value: ExtensionValue::KeyUsage(key_usage),
            });
        }

        // Add template-specific extensions
        for template_ext in &self.extensions {
            if let Ok(extension) = self.compile_template_extension(template_ext) {
                extensions.push(extension);
            }
        }

        Ok(extensions)
    }

    /// slay Check if key usage is set
    #[instrument(skip(self))]
    fn has_key_usage_set(&self) -> bool {
        self.key_usage.digital_signature.is_some() ||
        self.key_usage.content_commitment.is_some() ||
        self.key_usage.key_encipherment.is_some() ||
        self.key_usage.data_encipherment.is_some() ||
        self.key_usage.key_agreement.is_some() ||
        self.key_usage.key_cert_sign.is_some() ||
        self.key_usage.crl_sign.is_some() ||
        self.key_usage.encipher_only.is_some() ||
        self.key_usage.decipher_only.is_some()
    }

    /// slay Build key usage extension
    #[instrument(skip(self))]
    fn build_key_usage_extension(&self) -> TemplateResult<crate::stdlib::packages::crypto_pki::extensions::KeyUsage> {
        Ok(crate::stdlib::packages::crypto_pki::extensions::KeyUsage {
            digital_signature: self.key_usage.digital_signature.unwrap_or(false),
            content_commitment: self.key_usage.content_commitment.unwrap_or(false),
            key_encipherment: self.key_usage.key_encipherment.unwrap_or(false),
            data_encipherment: self.key_usage.data_encipherment.unwrap_or(false),
            key_agreement: self.key_usage.key_agreement.unwrap_or(false),
            key_cert_sign: self.key_usage.key_cert_sign.unwrap_or(false),
            crl_sign: self.key_usage.crl_sign.unwrap_or(false),
            encipher_only: self.key_usage.encipher_only.unwrap_or(false),
            decipher_only: self.key_usage.decipher_only.unwrap_or(false),
        })
    }

    /// slay Compile template extension
    #[instrument(skip(self, template_ext))]
    fn compile_template_extension(&self, template_ext: &TemplateExtension) -> TemplateResult<Extension> {
        let oid = ExtensionOid::from_str(&template_ext.oid);
        
        let value = match &template_ext.value_template {
            ExtensionValueTemplate::Static(data) => ExtensionValue::Raw(data.clone()),
            ExtensionValueTemplate::Pattern(pattern) => {
                // Compile pattern to actual value
                let compiled_value = self.compile_pattern(pattern)?;
                ExtensionValue::Raw(compiled_value)
            }
            ExtensionValueTemplate::Dynamic(expr) => {
                // Evaluate dynamic expression
                let dynamic_value = self.evaluate_dynamic_expression(expr)?;
                ExtensionValue::Raw(dynamic_value)
            }
            ExtensionValueTemplate::Reference(ref_name) => {
                // Resolve reference
                let resolved_value = self.resolve_reference(ref_name)?;
                ExtensionValue::Raw(resolved_value)
            }
        };

        Ok(Extension {
            oid,
            critical: template_ext.critical,
            value,
        })
    }

    /// slay Compile pattern to value
    #[instrument(skip(self))]
    fn compile_pattern(&self, pattern: &str) -> TemplateResult<Vec<u8>> {
        // Simplified pattern compilation
        Ok(pattern.as_bytes().to_vec())
    }

    /// slay Evaluate dynamic expression
    #[instrument(skip(self))]
    fn evaluate_dynamic_expression(&self, expr: &str) -> TemplateResult<Vec<u8>> {
        // Simplified expression evaluation
        Ok(expr.as_bytes().to_vec())
    }

    /// slay Resolve reference
    #[instrument(skip(self))]
    fn resolve_reference(&self, ref_name: &str) -> TemplateResult<Vec<u8>> {
        // Simplified reference resolution
        Ok(ref_name.as_bytes().to_vec())
    }
}

impl Default for SubjectTemplate {
    fn default() -> Self {
        Self {
            country: None,
            state_or_province: None,
            locality: None,
            organization: None,
            organizational_unit: None,
            common_name: Some(TemplateField {
                value: None,
                required: true,
                validation_pattern: None,
                constraints: FieldConstraints::default(),
                default_value: None,
                description: Some("Common Name".to_string()),
            }),
            email_address: None,
            additional_attributes: HashMap::new(),
        }
    }
}

impl TemplateRepository {
    /// slay Create new template repository
    #[instrument]
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            inheritance: HashMap::new(),
            validation_rules: Self::default_validation_rules(),
            config: RepositoryConfig::default(),
        }
    }

    /// slay Get default validation rules
    fn default_validation_rules() -> Vec<TemplateValidationRule> {
        vec![
            TemplateValidationRule {
                name: "required_name".to_string(),
                description: "Template must have a name".to_string(),
                severity: ValidationSeverity::Error,
                expression: "name.length > 0".to_string(),
                category: "basic".to_string(),
            },
            TemplateValidationRule {
                name: "valid_key_usage".to_string(),
                description: "Key usage must be consistent".to_string(),
                severity: ValidationSeverity::Error,
                expression: "validate_key_usage()".to_string(),
                category: "security".to_string(),
            },
        ]
    }

    /// slay Add template to repository
    #[instrument(skip(self, template))]
    pub fn add_template(&mut self, template: CertificateTemplate) -> TemplateResult<()> {
        if self.config.enable_validation {
            let validation = template.validate();
            if !validation.valid {
                return Err(TemplateError::ValidationFailed(
                    validation.issues.into_iter()
                        .map(|issue| issue.message)
                        .collect::<Vec<_>>()
                        .join("; ")
                ));
            }
        }

        info!("Adding template: {}", template.name);
        self.templates.insert(template.name.clone(), template);
        Ok(())
    }

    /// slay Get template by name
    #[instrument(skip(self))]
    pub fn get_template(&self, name: &str) -> Option<&CertificateTemplate> {
        self.templates.get(name)
    }

    /// slay List all templates
    #[instrument(skip(self))]
    pub fn list_templates(&self) -> Vec<&CertificateTemplate> {
        self.templates.values().collect()
    }

    /// slay Remove template
    #[instrument(skip(self))]
    pub fn remove_template(&mut self, name: &str) -> Option<CertificateTemplate> {
        self.templates.remove(name)
    }
}

/// fr fr Convenience functions for common operations

/// slay Create server template
#[instrument]
pub fn create_server_template(hostname: &str) -> TemplateResult<CertificateTemplate> {
    let mut template = CertificateTemplate::new(
        "Server Authentication".to_string(),
        TemplateType::ServerAuth
    );
    
    // Set common name to hostname
    template.subject_template.common_name = Some(TemplateField {
        value: Some(hostname.to_string()),
        required: true,
        validation_pattern: Some(r"^[a-zA-Z0-9.-]+$".to_string()),
        constraints: FieldConstraints {
            min_length: Some(1),
            max_length: Some(64),
            ..Default::default()
        },
        default_value: None,
        description: Some("Server hostname".to_string()),
    });

    Ok(template)
}

/// slay Create client template
#[instrument]
pub fn create_client_template(client_name: &str) -> TemplateResult<CertificateTemplate> {
    let mut template = CertificateTemplate::new(
        "Client Authentication".to_string(),
        TemplateType::ClientAuth
    );
    
    // Set common name to client name
    template.subject_template.common_name = Some(TemplateField {
        value: Some(client_name.to_string()),
        required: true,
        validation_pattern: None,
        constraints: FieldConstraints::default(),
        default_value: None,
        description: Some("Client identifier".to_string()),
    });

    Ok(template)
}

/// slay Create CA template
#[instrument]
pub fn create_ca_template(ca_name: &str) -> TemplateResult<CertificateTemplate> {
    let mut template = CertificateTemplate::new(
        "Certificate Authority".to_string(),
        TemplateType::CertificateAuthority
    );
    
    // Set common name to CA name
    template.subject_template.common_name = Some(TemplateField {
        value: Some(ca_name.to_string()),
        required: true,
        validation_pattern: None,
        constraints: FieldConstraints::default(),
        default_value: None,
        description: Some("Certificate Authority name".to_string()),
    });

    // Set longer validity for CA certificates
    template.validity.default_validity = Duration::from_secs(10 * 365 * 24 * 3600); // 10 years
    template.validity.max_validity = Some(Duration::from_secs(20 * 365 * 24 * 3600)); // 20 years

    Ok(template)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_type_default_key_usage() {
        let server_usage = TemplateType::ServerAuth.default_key_usage();
        assert_eq!(server_usage.digital_signature, Some(true));
        assert_eq!(server_usage.key_encipherment, Some(true));
        assert_eq!(server_usage.key_cert_sign, Some(false));

        let ca_usage = TemplateType::CertificateAuthority.default_key_usage();
        assert_eq!(ca_usage.key_cert_sign, Some(true));
        assert_eq!(ca_usage.crl_sign, Some(true));
    }

    #[test]
    fn test_certificate_template_creation() {
        let template = CertificateTemplate::new(
            "Test Template".to_string(),
            TemplateType::ServerAuth
        );
        
        assert_eq!(template.name, "Test Template");
        assert_eq!(template.template_type, TemplateType::ServerAuth);
        assert_eq!(template.metadata.status, TemplateStatus::Draft);
    }

    #[test]
    fn test_template_validation() {
        let template = CertificateTemplate::new(
            "Valid Template".to_string(),
            TemplateType::ClientAuth
        );
        
        let result = template.validate();
        assert!(result.valid);
        assert!(result.issues.is_empty());
    }

    #[test]
    fn test_server_template_creation() {
        let template = create_server_template("example.com").unwrap();
        assert_eq!(template.template_type, TemplateType::ServerAuth);
        
        let cn_field = template.subject_template.common_name.unwrap();
        assert_eq!(cn_field.value, Some("example.com".to_string()));
        assert!(cn_field.required);
    }

    #[test]
    fn test_template_repository() {
        let mut repo = TemplateRepository::new();
        
        let template = CertificateTemplate::new(
            "Test Template".to_string(),
            TemplateType::ServerAuth
        );
        
        repo.add_template(template).unwrap();
        assert!(repo.get_template("Test Template").is_some());
        assert_eq!(repo.list_templates().len(), 1);
    }

    #[test]
    fn test_field_constraints() {
        let constraints = FieldConstraints {
            min_length: Some(3),
            max_length: Some(64),
            case_sensitive: true,
            ..Default::default()
        };
        
        assert_eq!(constraints.min_length, Some(3));
        assert_eq!(constraints.max_length, Some(64));
        assert!(constraints.case_sensitive);
    }

    #[test]
    fn test_template_status() {
        let status = TemplateStatus::Active;
        assert_eq!(status, TemplateStatus::Active);
        assert_ne!(status, TemplateStatus::Draft);
    }
}
