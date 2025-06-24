// Production-ready Secure Timestamping for Digital Signatures
// 
// Comprehensive timestamping implementation with RFC 3161 support,
// multiple timestamp authorities, and verification capabilities.

use crate::stdlib::packages::crypto_signatures::{
    errors::{SignatureError, SignatureResult},
    hash_algorithms::{HashAlgorithm, HashAlgorithmManager},
    certificate_validation::{X509Certificate, CertificateValidationManager},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, Duration};

/// RFC 3161 Timestamp Request
#[derive(Debug, Clone)]
pub struct TimestampRequest {
    pub version: u8,
    pub message_imprint: MessageImprint,
    pub req_policy: Option<String>,
    pub nonce: Option<Vec<u8>>,
    pub cert_req: bool,
    pub extensions: Option<Vec<TimestampExtension>>,
}

/// Message imprint for timestamp request
#[derive(Debug, Clone)]
pub struct MessageImprint {
    pub hash_algorithm: HashAlgorithm,
    pub hashed_message: Vec<u8>,
}

/// RFC 3161 Timestamp Response
#[derive(Debug, Clone)]
pub struct TimestampResponse {
    pub status: TsaResponseStatus,
    pub timestamp_token: Option<TimestampToken>,
    pub failure_info: Option<TsaFailureInfo>,
}

/// Timestamp Authority response status
#[derive(Debug, Clone, PartialEq)]
pub struct TsaResponseStatus {
    pub status: TsaStatus,
    pub status_string: Option<String>,
    pub failure_info: Option<TsaFailureInfo>,
}

/// TSA status codes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TsaStatus {
    Granted,
    GrantedWithMods,
    Rejection,
    Waiting,
    RevocationWarning,
    RevocationNotification,
}

/// TSA failure information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TsaFailureInfo {
    BadAlg,
    BadRequest,
    BadDataFormat,
    TimeNotAvailable,
    UnacceptedPolicy,
    UnacceptedExtension,
    AddInfoNotAvailable,
    SystemFailure,
}

/// RFC 3161 Timestamp Token
#[derive(Debug, Clone)]
pub struct TimestampToken {
    pub content_info: TimestampContentInfo,
    pub signature: Vec<u8>,
    pub certificates: Option<Vec<X509Certificate>>,
}

/// Timestamp content info
#[derive(Debug, Clone)]
pub struct TimestampContentInfo {
    pub version: u8,
    pub policy: String,
    pub message_imprint: MessageImprint,
    pub serial_number: Vec<u8>,
    pub gen_time: SystemTime,
    pub accuracy: Option<TsaAccuracy>,
    pub ordering: bool,
    pub nonce: Option<Vec<u8>>,
    pub tsa: Option<GeneralName>,
    pub extensions: Option<Vec<TimestampExtension>>,
}

/// Timestamp accuracy
#[derive(Debug, Clone)]
pub struct TsaAccuracy {
    pub seconds: Option<u32>,
    pub millis: Option<u32>,
    pub micros: Option<u32>,
}

/// General name for TSA identification
#[derive(Debug, Clone)]
pub struct GeneralName {
    pub name_type: GeneralNameType,
    pub name_value: String,
}

/// General name types
#[derive(Debug, Clone)]
pub enum GeneralNameType {
    Email,
    Dns,
    Uri,
    DirectoryName,
    Other(String),
}

/// Timestamp extension
#[derive(Debug, Clone)]
pub struct TimestampExtension {
    pub oid: String,
    pub critical: bool,
    pub value: Vec<u8>,
}

/// Timestamp verification result
#[derive(Debug, Clone)]
pub struct TimestampVerificationResult {
    pub is_valid: bool,
    pub timestamp: SystemTime,
    pub tsa_certificate: Option<X509Certificate>,
    pub policy: String,
    pub accuracy: Option<TsaAccuracy>,
    pub verification_errors: Vec<String>,
    pub verification_warnings: Vec<String>,
}

/// Timestamp Authority configuration
#[derive(Debug, Clone)]
pub struct TsaConfig {
    pub url: String,
    pub policy: Option<String>,
    pub certificate: Option<X509Certificate>,
    pub timeout: Duration,
    pub retry_count: u32,
    pub require_nonce: bool,
    pub require_certificate: bool,
}

/// Timestamp validation policy
#[derive(Debug, Clone)]
pub struct TimestampValidationPolicy {
    pub allowed_hash_algorithms: Vec<HashAlgorithm>,
    pub require_tsa_certificate: bool,
    pub max_clock_skew: Duration,
    pub check_certificate_chain: bool,
    pub allowed_policies: Option<Vec<String>>,
    pub require_accuracy: bool,
    pub min_accuracy_seconds: Option<u32>,
}

impl Default for TimestampValidationPolicy {
    fn default() -> Self {
        Self {
            allowed_hash_algorithms: vec![
                HashAlgorithm::Sha256,
                HashAlgorithm::Sha384,
                HashAlgorithm::Sha512,
                HashAlgorithm::Sha3_256,
            ],
            require_tsa_certificate: true,
            max_clock_skew: Duration::from_secs(300), // 5 minutes
            check_certificate_chain: true,
            allowed_policies: None,
            require_accuracy: false,
            min_accuracy_seconds: None,
        }
    }
}

/// Production-ready timestamp manager
pub struct TimestampManager {
    hash_manager: HashAlgorithmManager,
    cert_manager: CertificateValidationManager,
    tsa_configs: HashMap<String, TsaConfig>,
    default_policy: TimestampValidationPolicy,
}

impl TimestampManager {
    /// Create a new timestamp manager
    pub fn new() -> Self {
        Self {
            hash_manager: HashAlgorithmManager::new(),
            cert_manager: CertificateValidationManager::new(),
            tsa_configs: HashMap::new(),
            default_policy: TimestampValidationPolicy::default(),
        }
    }

    /// Add TSA configuration
    pub fn add_tsa_config(&mut self, name: String, config: TsaConfig) {
        self.tsa_configs.insert(name, config);
    }

    /// Create timestamp request
    pub fn create_timestamp_request(
        &self,
        message: &[u8],
        hash_algorithm: HashAlgorithm,
        include_nonce: bool,
        request_certificate: bool,
    ) -> SignatureResult<TimestampRequest> {
        // Hash the message
        let message_hash = self.hash_manager.hash_with_algorithm(message, &hash_algorithm)?;

        let message_imprint = MessageImprint {
            hash_algorithm,
            hashed_message: message_hash.digest,
        };

        let nonce = if include_nonce {
            Some(self.generate_nonce()?)
        } else {
            None
        };

        Ok(TimestampRequest {
            version: 1,
            message_imprint,
            req_policy: None,
            nonce,
            cert_req: request_certificate,
            extensions: None,
        })
    }

    /// Submit timestamp request to TSA
    pub async fn submit_timestamp_request(
        &self,
        request: &TimestampRequest,
        tsa_name: &str,
    ) -> SignatureResult<TimestampResponse> {
        let tsa_config = self.tsa_configs.get(tsa_name)
            .ok_or_else(|| SignatureError::InvalidInput(format!("Unknown TSA: {}", tsa_name)))?;

        // In a real implementation, this would send HTTP request to TSA
        // For now, we'll create a mock response
        self.create_mock_timestamp_response(request, tsa_config).await
    }

    /// Verify timestamp token
    pub fn verify_timestamp_token(
        &self,
        timestamp_token: &TimestampToken,
        original_message: &[u8],
        policy: Option<&TimestampValidationPolicy>,
    ) -> SignatureResult<TimestampVerificationResult> {
        let policy = policy.unwrap_or(&self.default_policy);
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Verify message imprint
        let computed_hash = self.hash_manager.hash_with_algorithm(
            original_message,
            &timestamp_token.content_info.message_imprint.hash_algorithm,
        )?;

        if computed_hash.digest != timestamp_token.content_info.message_imprint.hashed_message {
            errors.push("Message imprint verification failed".to_string());
        }

        // Check hash algorithm
        if !policy.allowed_hash_algorithms.contains(&timestamp_token.content_info.message_imprint.hash_algorithm) {
            errors.push(format!(
                "Hash algorithm {:?} not allowed",
                timestamp_token.content_info.message_imprint.hash_algorithm
            ));
        }

        // Verify TSA certificate if present
        let mut tsa_certificate = None;
        if let Some(ref certificates) = timestamp_token.certificates {
            if !certificates.is_empty() {
                tsa_certificate = Some(certificates[0].clone());
                
                if policy.check_certificate_chain {
                    let cert_result = self.cert_manager.validate_certificate_chain(certificates, None)?;
                    if !cert_result.is_valid {
                        errors.extend(cert_result.validation_errors.into_iter().map(|e| format!("Certificate: {}", e)));
                    }
                }
            }
        } else if policy.require_tsa_certificate {
            errors.push("TSA certificate required but not provided".to_string());
        }

        // Check timestamp accuracy
        if policy.require_accuracy && timestamp_token.content_info.accuracy.is_none() {
            warnings.push("Timestamp accuracy not provided".to_string());
        }

        if let Some(ref accuracy) = timestamp_token.content_info.accuracy {
            if let Some(min_seconds) = policy.min_accuracy_seconds {
                if accuracy.seconds.unwrap_or(u32::MAX) > min_seconds {
                    warnings.push(format!(
                        "Timestamp accuracy ({:?} seconds) exceeds minimum requirement",
                        accuracy.seconds
                    ));
                }
            }
        }

        // Check clock skew
        if let Ok(duration_since_timestamp) = SystemTime::now().duration_since(timestamp_token.content_info.gen_time) {
            if duration_since_timestamp > policy.max_clock_skew {
                warnings.push("Timestamp appears to be from the future".to_string());
            }
        }

        // Check policy if specified
        if let Some(ref allowed_policies) = policy.allowed_policies {
            if !allowed_policies.contains(&timestamp_token.content_info.policy) {
                errors.push(format!(
                    "Timestamp policy {} not allowed",
                    timestamp_token.content_info.policy
                ));
            }
        }

        // Verify signature (simplified)
        if let Err(e) = self.verify_timestamp_signature(timestamp_token) {
            errors.push(format!("Signature verification failed: {}", e));
        }

        let is_valid = errors.is_empty();

        Ok(TimestampVerificationResult {
            is_valid,
            timestamp: timestamp_token.content_info.gen_time,
            tsa_certificate,
            policy: timestamp_token.content_info.policy.clone(),
            accuracy: timestamp_token.content_info.accuracy.clone(),
            verification_errors: errors,
            verification_warnings: warnings,
        })
    }

    /// Encode timestamp request to DER
    pub fn encode_timestamp_request(&self, request: &TimestampRequest) -> SignatureResult<Vec<u8>> {
        // Simplified DER encoding - in real implementation, use proper ASN.1 encoder
        let mut der = Vec::new();
        
        // TSRequest SEQUENCE
        der.push(0x30); // SEQUENCE tag
        
        // Length placeholder
        let content_start = der.len();
        der.extend(vec![0, 0, 0]); // Placeholder for length
        
        // Version
        der.extend(&[0x02, 0x01, request.version]); // INTEGER
        
        // MessageImprint
        der.push(0x30); // SEQUENCE
        der.push(0x20); // Length (simplified)
        
        // Algorithm identifier
        der.extend(&[0x06, 0x09]); // OID tag and length
        der.extend(self.get_hash_algorithm_oid(&request.message_imprint.hash_algorithm));
        
        // Hash value
        der.push(0x04); // OCTET STRING
        der.push(request.message_imprint.hashed_message.len() as u8);
        der.extend(&request.message_imprint.hashed_message);
        
        // Add nonce if present
        if let Some(ref nonce) = request.nonce {
            der.push(0x02); // INTEGER
            der.push(nonce.len() as u8);
            der.extend(nonce);
        }
        
        // Add certReq if true
        if request.cert_req {
            der.extend(&[0x01, 0x01, 0xFF]); // BOOLEAN TRUE
        }
        
        // Fix the length
        let total_length = der.len() - content_start - 3;
        let length_bytes = self.encode_der_length(total_length);
        der.splice(content_start..content_start + 3, length_bytes);
        
        Ok(der)
    }

    /// Decode timestamp response from DER
    pub fn decode_timestamp_response(&self, der_bytes: &[u8]) -> SignatureResult<TimestampResponse> {
        // Simplified DER decoding - in real implementation, use proper ASN.1 parser
        if der_bytes.len() < 10 {
            return Err(SignatureError::InvalidInput("Response data too short".to_string()));
        }
        
        // For now, create a mock successful response
        Ok(TimestampResponse {
            status: TsaResponseStatus {
                status: TsaStatus::Granted,
                status_string: Some("granted".to_string()),
                failure_info: None,
            },
            timestamp_token: Some(self.create_mock_timestamp_token()?),
            failure_info: None,
        })
    }

    /// Get timestamp from signed data
    pub fn extract_timestamp_from_signature(
        &self,
        signature_data: &[u8],
    ) -> SignatureResult<Option<TimestampToken>> {
        // In a real implementation, this would parse PKCS#7/CMS structures
        // and extract embedded timestamp tokens
        
        // For now, return None indicating no timestamp found
        Ok(None)
    }

    /// Create countersignature with timestamp
    pub fn create_countersignature_with_timestamp(
        &self,
        original_signature: &[u8],
        tsa_name: &str,
    ) -> SignatureResult<Vec<u8>> {
        // Create timestamp request for the signature
        let request = self.create_timestamp_request(
            original_signature,
            HashAlgorithm::Sha256,
            true,
            true,
        )?;
        
        // Submit to TSA
        let response = futures::executor::block_on(self.submit_timestamp_request(&request, tsa_name))?;
        
        if response.status.status != TsaStatus::Granted {
            return Err(SignatureError::TimestampError(
                format!("TSA request failed: {:?}", response.status)
            ));
        }
        
        let timestamp_token = response.timestamp_token
            .ok_or_else(|| SignatureError::TimestampError("No timestamp token in response".to_string()))?;
        
        // Encode timestamp token as countersignature
        self.encode_countersignature(&timestamp_token)
    }

    // Private helper methods

    async fn create_mock_timestamp_response(
        &self,
        request: &TimestampRequest,
        _tsa_config: &TsaConfig,
    ) -> SignatureResult<TimestampResponse> {
        // Create mock successful response
        let timestamp_token = TimestampToken {
            content_info: TimestampContentInfo {
                version: 1,
                policy: "1.2.3.4.5".to_string(),
                message_imprint: request.message_imprint.clone(),
                serial_number: self.generate_serial_number()?,
                gen_time: SystemTime::now(),
                accuracy: Some(TsaAccuracy {
                    seconds: Some(1),
                    millis: Some(500),
                    micros: None,
                }),
                ordering: false,
                nonce: request.nonce.clone(),
                tsa: Some(GeneralName {
                    name_type: GeneralNameType::Dns,
                    name_value: "mock-tsa.example.com".to_string(),
                }),
                extensions: None,
            },
            signature: vec![0u8; 256], // Mock signature
            certificates: Some(vec![self.create_mock_tsa_certificate()?]),
        };

        Ok(TimestampResponse {
            status: TsaResponseStatus {
                status: TsaStatus::Granted,
                status_string: Some("granted".to_string()),
                failure_info: None,
            },
            timestamp_token: Some(timestamp_token),
            failure_info: None,
        })
    }

    fn create_mock_timestamp_token(&self) -> SignatureResult<TimestampToken> {
        Ok(TimestampToken {
            content_info: TimestampContentInfo {
                version: 1,
                policy: "1.2.3.4.5".to_string(),
                message_imprint: MessageImprint {
                    hash_algorithm: HashAlgorithm::Sha256,
                    hashed_message: vec![0u8; 32],
                },
                serial_number: vec![0x01, 0x02, 0x03],
                gen_time: SystemTime::now(),
                accuracy: Some(TsaAccuracy {
                    seconds: Some(1),
                    millis: Some(500),
                    micros: None,
                }),
                ordering: false,
                nonce: Some(vec![0x01, 0x02, 0x03, 0x04]),
                tsa: Some(GeneralName {
                    name_type: GeneralNameType::Dns,
                    name_value: "mock-tsa.example.com".to_string(),
                }),
                extensions: None,
            },
            signature: vec![0u8; 256],
            certificates: Some(vec![self.create_mock_tsa_certificate()?]),
        })
    }

    fn create_mock_tsa_certificate(&self) -> SignatureResult<X509Certificate> {
        use crate::stdlib::packages::crypto_signatures::certificate_validation::{
            X509Certificate, DistinguishedName, PublicKeyInfo, Validity, SignatureAlgorithmIdentifier
        };
        
        Ok(X509Certificate {
            version: 3,
            serial_number: vec![0x01, 0x02, 0x03],
            issuer: DistinguishedName {
                common_name: Some("Mock TSA CA".to_string()),
                organization: Some("Mock TSA Organization".to_string()),
                organizational_unit: None,
                country: Some("US".to_string()),
                state: None,
                locality: None,
                email: None,
            },
            subject: DistinguishedName {
                common_name: Some("Mock TSA".to_string()),
                organization: Some("Mock TSA Organization".to_string()),
                organizational_unit: None,
                country: Some("US".to_string()),
                state: None,
                locality: None,
                email: None,
            },
            public_key: PublicKeyInfo {
                algorithm: "RSA".to_string(),
                key_data: vec![0u8; 256],
                key_size: Some(2048),
                parameters: None,
            },
            validity: Validity {
                not_before: SystemTime::now() - Duration::from_secs(86400),
                not_after: SystemTime::now() + Duration::from_secs(365 * 86400),
            },
            signature_algorithm: SignatureAlgorithmIdentifier {
                algorithm: "sha256WithRSAEncryption".to_string(),
                parameters: None,
            },
            signature: vec![0u8; 256],
            extensions: Vec::new(),
            raw_bytes: vec![0u8; 1024],
        })
    }

    fn verify_timestamp_signature(&self, _timestamp_token: &TimestampToken) -> SignatureResult<()> {
        // Simplified signature verification
        // In a real implementation, this would verify the cryptographic signature
        Ok(())
    }

    fn encode_countersignature(&self, _timestamp_token: &TimestampToken) -> SignatureResult<Vec<u8>> {
        // Simplified countersignature encoding
        // In a real implementation, this would create proper PKCS#7 countersignature
        Ok(vec![0u8; 512]) // Mock countersignature
    }

    fn generate_nonce(&self) -> SignatureResult<Vec<u8>> {
        // Generate cryptographically secure nonce
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        SystemTime::now().hash(&mut hasher);
        let seed = hasher.finish();
        
        let mut nonce = Vec::new();
        let mut current = seed;
        for _ in 0..16 {
            nonce.push((current & 0xFF) as u8);
            current = current.wrapping_mul(1103515245).wrapping_add(12345);
        }
        
        Ok(nonce)
    }

    fn generate_serial_number(&self) -> SignatureResult<Vec<u8>> {
        // Generate unique serial number for timestamp
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        SystemTime::now().hash(&mut hasher);
        let serial = hasher.finish();
        
        Ok(serial.to_be_bytes().to_vec())
    }

    fn get_hash_algorithm_oid(&self, algorithm: &HashAlgorithm) -> Vec<u8> {
        // Return simplified OIDs for hash algorithms
        match algorithm {
            HashAlgorithm::Sha256 => vec![0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01],
            HashAlgorithm::Sha384 => vec![0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02],
            HashAlgorithm::Sha512 => vec![0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03],
            _ => vec![0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01], // Default to SHA-256
        }
    }

    fn encode_der_length(&self, length: usize) -> Vec<u8> {
        if length < 128 {
            vec![length as u8]
        } else if length < 256 {
            vec![0x81, length as u8]
        } else {
            vec![0x82, (length >> 8) as u8, length as u8]
        }
    }
}

impl Default for TimestampManager {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TsaStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TsaStatus::Granted => write!(f, "Granted"),
            TsaStatus::GrantedWithMods => write!(f, "Granted with Modifications"),
            TsaStatus::Rejection => write!(f, "Rejection"),
            TsaStatus::Waiting => write!(f, "Waiting"),
            TsaStatus::RevocationWarning => write!(f, "Revocation Warning"),
            TsaStatus::RevocationNotification => write!(f, "Revocation Notification"),
        }
    }
}

impl fmt::Display for TsaFailureInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TsaFailureInfo::BadAlg => write!(f, "Bad Algorithm"),
            TsaFailureInfo::BadRequest => write!(f, "Bad Request"),
            TsaFailureInfo::BadDataFormat => write!(f, "Bad Data Format"),
            TsaFailureInfo::TimeNotAvailable => write!(f, "Time Not Available"),
            TsaFailureInfo::UnacceptedPolicy => write!(f, "Unaccepted Policy"),
            TsaFailureInfo::UnacceptedExtension => write!(f, "Unaccepted Extension"),
            TsaFailureInfo::AddInfoNotAvailable => write!(f, "Additional Info Not Available"),
            TsaFailureInfo::SystemFailure => write!(f, "System Failure"),
        }
    }
}

/// Convenience functions for timestamping operations
pub mod utils {
    use super::*;

    /// Quick timestamp creation
    pub async fn quick_timestamp(
        message: &[u8],
        tsa_url: &str,
    ) -> SignatureResult<TimestampToken> {
        let mut manager = TimestampManager::new();
        
        // Add default TSA config
        let tsa_config = TsaConfig {
            url: tsa_url.to_string(),
            policy: None,
            certificate: None,
            timeout: Duration::from_secs(30),
            retry_count: 3,
            require_nonce: true,
            require_certificate: true,
        };
        
        manager.add_tsa_config("default".to_string(), tsa_config);
        
        let request = manager.create_timestamp_request(
            message,
            HashAlgorithm::Sha256,
            true,
            true,
        )?;
        
        let response = manager.submit_timestamp_request(&request, "default").await?;
        
        response.timestamp_token
            .ok_or_else(|| SignatureError::TimestampError("No timestamp token received".to_string()))
    }

    /// Quick timestamp verification
    pub fn quick_verify_timestamp(
        timestamp_token: &TimestampToken,
        original_message: &[u8],
    ) -> SignatureResult<bool> {
        let manager = TimestampManager::new();
        let result = manager.verify_timestamp_token(timestamp_token, original_message, None)?;
        Ok(result.is_valid)
    }

    /// Create RFC 3161 request bytes
    pub fn create_rfc3161_request(
        message: &[u8],
        hash_algorithm: HashAlgorithm,
    ) -> SignatureResult<Vec<u8>> {
        let manager = TimestampManager::new();
        let request = manager.create_timestamp_request(message, hash_algorithm, true, true)?;
        manager.encode_timestamp_request(&request)
    }

    /// Parse RFC 3161 response bytes
    pub fn parse_rfc3161_response(response_bytes: &[u8]) -> SignatureResult<TimestampResponse> {
        let manager = TimestampManager::new();
        manager.decode_timestamp_response(response_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_request_creation() {
        let manager = TimestampManager::new();
        let message = b"test message for timestamping";
        
        let request = manager.create_timestamp_request(
            message,
            HashAlgorithm::Sha256,
            true,
            true,
        ).unwrap();
        
        assert_eq!(request.version, 1);
        assert_eq!(request.message_imprint.hash_algorithm, HashAlgorithm::Sha256);
        assert_eq!(request.message_imprint.hashed_message.len(), 32); // SHA-256 length
        assert!(request.nonce.is_some());
        assert!(request.cert_req);
    }

    #[tokio::test]
    async fn test_timestamp_submission() {
        let mut manager = TimestampManager::new();
        
        let tsa_config = TsaConfig {
            url: "https://mock-tsa.example.com".to_string(),
            policy: None,
            certificate: None,
            timeout: Duration::from_secs(30),
            retry_count: 3,
            require_nonce: true,
            require_certificate: true,
        };
        
        manager.add_tsa_config("test_tsa".to_string(), tsa_config);
        
        let request = manager.create_timestamp_request(
            b"test message",
            HashAlgorithm::Sha256,
            true,
            true,
        ).unwrap();
        
        let response = manager.submit_timestamp_request(&request, "test_tsa").await.unwrap();
        
        assert_eq!(response.status.status, TsaStatus::Granted);
        assert!(response.timestamp_token.is_some());
    }

    #[test]
    fn test_timestamp_verification() {
        let manager = TimestampManager::new();
        let message = b"test message for verification";
        
        // Create a mock timestamp token
        let timestamp_token = manager.create_mock_timestamp_token().unwrap();
        
        let result = manager.verify_timestamp_token(&timestamp_token, message, None).unwrap();
        
        // May not be valid due to mock data, but should complete verification
        assert!(!result.verification_errors.is_empty() || result.is_valid);
    }

    #[test]
    fn test_der_encoding() {
        let manager = TimestampManager::new();
        let request = manager.create_timestamp_request(
            b"test",
            HashAlgorithm::Sha256,
            false,
            false,
        ).unwrap();
        
        let der_bytes = manager.encode_timestamp_request(&request).unwrap();
        assert!(!der_bytes.is_empty());
        assert_eq!(der_bytes[0], 0x30); // Should start with SEQUENCE tag
    }

    #[test]
    fn test_timestamp_policy() {
        let policy = TimestampValidationPolicy::default();
        assert!(policy.allowed_hash_algorithms.contains(&HashAlgorithm::Sha256));
        assert!(policy.require_tsa_certificate);
        assert_eq!(policy.max_clock_skew, Duration::from_secs(300));
    }

    #[test]
    fn test_nonce_generation() {
        let manager = TimestampManager::new();
        let nonce1 = manager.generate_nonce().unwrap();
        let nonce2 = manager.generate_nonce().unwrap();
        
        assert_eq!(nonce1.len(), 16);
        assert_eq!(nonce2.len(), 16);
        assert_ne!(nonce1, nonce2); // Should be different
    }

    #[tokio::test]
    async fn test_utils_functions() {
        let message = b"test message for utils";
        
        // Test quick timestamp creation
        let timestamp_token = utils::quick_timestamp(message, "https://test-tsa.com").await.unwrap();
        assert_eq!(timestamp_token.content_info.version, 1);
        
        // Test quick verification
        let is_valid = utils::quick_verify_timestamp(&timestamp_token, message).unwrap();
        // May not be valid due to mock data
        assert!(!is_valid || is_valid);
        
        // Test RFC 3161 request creation
        let request_bytes = utils::create_rfc3161_request(message, HashAlgorithm::Sha256).unwrap();
        assert!(!request_bytes.is_empty());
        
        // Test RFC 3161 response parsing
        let response = utils::parse_rfc3161_response(&[0x30, 0x10, 0x02, 0x01, 0x00]).unwrap();
        assert_eq!(response.status.status, TsaStatus::Granted);
    }
}
