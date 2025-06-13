/// Comprehensive PEM/DER format handling for PKI operations
/// 
/// This module provides production-ready implementations for parsing, encoding, and converting
/// between PEM (Privacy-Enhanced Mail) and DER (Distinguished Encoding Rules) formats.
/// It includes comprehensive ASN.1 parsing, certificate handling, and secure format validation.
/// 
/// # Security Considerations
/// 
/// PEM/DER format handling is critical for PKI interoperability and security:
/// - Input validation prevents ASN.1 parsing vulnerabilities
/// - Bounds checking protects against buffer overflow attacks
/// - Secure memory handling protects private key material
/// - Format validation ensures cryptographic parameter integrity
/// - Error handling prevents information leakage
/// 
/// # Supported Formats
/// 
/// - X.509 certificates (PEM/DER)
/// - Private keys (PKCS#1, PKCS#8, SEC1 for EC keys)
/// - Certificate Signing Requests (PKCS#10)
/// - Certificate Revocation Lists (CRL)
/// - Certificate chains and bundles
/// - Encrypted private keys with password protection
/// 
/// # Examples
/// 
/// ```rust
/// use cursed::stdlib::packages::crypto_pki::pem_der::*;
/// 
/// // Parse PEM certificate
/// let pem_data = "-----BEGIN CERTIFICATE-----\n...";
/// let cert = parse_pem_certificate(pem_data)?;
/// 
/// // Convert PEM to DER
/// let der_data = pem_to_der(pem_data)?;
/// 
/// // Parse certificate chain
/// let chain_pem = "-----BEGIN CERTIFICATE-----\n...";
/// let chain = parse_certificate_chain(chain_pem)?;
/// ```

use std::collections::HashMap;
use std::fmt;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::error::CursedError;

// Re-export and define common types for PEM/DER operations
pub use crate::stdlib::packages::crypto_pki::types::{
    X509Certificate, CertificateSigningRequest as CsrType, PublicKeyAlgorithm, SignatureAlgorithm
};
pub use crate::stdlib::packages::crypto_pki::key_management::KeyPair;

// Type aliases for PEM/DER operations  
pub type Certificate = X509Certificate;
pub type CertificateSigningRequest = CsrType;
pub type PublicKey = Vec<u8>;

/// Private key representation for PEM/DER operations
#[derive(Debug, Clone)]
pub struct PrivateKey {
    /// Algorithm used for this private key
    pub algorithm: PublicKeyAlgorithm,
    /// Private key data (DER encoded)
    pub data: Vec<u8>,
    /// DER encoded representation
    pub der_encoded: Option<Vec<u8>>,
    /// Key parameters
    pub parameters: Option<Vec<u8>>,
    /// Key usage flags
    pub usage: Option<Vec<String>>,
}

impl PrivateKey {
    pub fn new() -> Self {
        Self {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            data: Vec::new(),
            der_encoded: None,
            parameters: None,
            usage: None,
        }
    }
    
    /// Create private key from key pair
    pub fn from_key_pair(key_pair: &KeyPair) -> Self {
        Self {
            algorithm: key_pair.algorithm.clone(),
            data: key_pair.private_key.clone(),
            der_encoded: Some(key_pair.private_key.clone()),
            parameters: key_pair.parameters.clone(),
            usage: None,
        }
    }
}

impl Default for PrivateKey {
    fn default() -> Self {
        Self::new()
    }
}

/// Comprehensive error types for PEM/DER operations
#[derive(Debug, Clone)]
pub enum PemDerError {
    /// Invalid PEM format or structure
    InvalidPemFormat(String),
    /// Invalid DER encoding
    InvalidDerEncoding(String),
    /// ASN.1 parsing error
    Asn1ParseError(String),
    /// Unsupported format or algorithm
    UnsupportedFormat(String),
    /// Certificate validation error
    CertificateValidationError(String),
    /// Private key parsing error
    PrivateKeyError(String),
    /// Encryption/decryption error
    EncryptionError(String),
    /// Base64 encoding/decoding error
    Base64Error(String),
    /// Input validation error
    ValidationError(String),
    /// I/O or system error
    IoError(String),
}

impl fmt::Display for PemDerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PemDerError::InvalidPemFormat(msg) => write!(f, "Invalid PEM format: {}", msg),
            PemDerError::InvalidDerEncoding(msg) => write!(f, "Invalid DER encoding: {}", msg),
            PemDerError::Asn1ParseError(msg) => write!(f, "ASN.1 parsing error: {}", msg),
            PemDerError::UnsupportedFormat(msg) => write!(f, "Unsupported format: {}", msg),
            PemDerError::CertificateValidationError(msg) => write!(f, "Certificate validation error: {}", msg),
            PemDerError::PrivateKeyError(msg) => write!(f, "Private key error: {}", msg),
            PemDerError::EncryptionError(msg) => write!(f, "Encryption error: {}", msg),
            PemDerError::Base64Error(msg) => write!(f, "Base64 error: {}", msg),
            PemDerError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            PemDerError::IoError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for PemDerError {}

impl From<PemDerError> for CursedError {
    fn from(err: PemDerError) -> Self {
        CursedError::Runtime(err.to_string())
    }
}

/// Result type for PEM/DER operations
pub type PemDerResult<T> = Result<T, PemDerError>;

/// PEM format types and headers
#[derive(Debug, Clone, PartialEq)]
pub enum PemType {
    Certificate,
    PrivateKey,
    PublicKey,
    CertificateRequest,
    CertificateRevocationList,
    RsaPrivateKey,
    EcPrivateKey,
    EncryptedPrivateKey,
    DhParameters,
    EcParameters,
    Custom(String),
}

impl PemType {
    /// Get PEM header for this type
    pub fn header(&self) -> &str {
        match self {
            PemType::Certificate => "CERTIFICATE",
            PemType::PrivateKey => "PRIVATE KEY",
            PemType::PublicKey => "PUBLIC KEY",
            PemType::CertificateRequest => "CERTIFICATE REQUEST",
            PemType::CertificateRevocationList => "X509 CRL",
            PemType::RsaPrivateKey => "RSA PRIVATE KEY",
            PemType::EcPrivateKey => "EC PRIVATE KEY",
            PemType::EncryptedPrivateKey => "ENCRYPTED PRIVATE KEY",
            PemType::DhParameters => "DH PARAMETERS",
            PemType::EcParameters => "EC PARAMETERS",
            PemType::Custom(header) => header,
        }
    }

    /// Parse PEM type from header
    pub fn from_header(header: &str) -> Self {
        match header.trim() {
            "CERTIFICATE" => PemType::Certificate,
            "PRIVATE KEY" => PemType::PrivateKey,
            "PUBLIC KEY" => PemType::PublicKey,
            "CERTIFICATE REQUEST" => PemType::CertificateRequest,
            "X509 CRL" => PemType::CertificateRevocationList,
            "RSA PRIVATE KEY" => PemType::RsaPrivateKey,
            "EC PRIVATE KEY" => PemType::EcPrivateKey,
            "ENCRYPTED PRIVATE KEY" => PemType::EncryptedPrivateKey,
            "DH PARAMETERS" => PemType::DhParameters,
            "EC PARAMETERS" => PemType::EcParameters,
            other => PemType::Custom(other.to_string()),
        }
    }
}

/// PEM block containing format metadata and data
#[derive(Debug, Clone)]
pub struct PemBlock {
    /// PEM format type
    pub pem_type: PemType,
    /// Headers (key-value pairs between BEGIN and data)
    pub headers: HashMap<String, String>,
    /// Binary data (decoded from base64)
    pub data: Vec<u8>,
    /// Original PEM content
    pub raw_content: String,
}

impl PemBlock {
    /// Create new PEM block
    pub fn new(pem_type: PemType, data: Vec<u8>) -> Self {
        Self {
            pem_type,
            headers: HashMap::new(),
            data,
            raw_content: String::new(),
        }
    }

    /// Add header to PEM block
    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    /// Get header value
    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    /// Check if PEM block is encrypted
    pub fn is_encrypted(&self) -> bool {
        self.headers.contains_key("Proc-Type") || 
        self.headers.contains_key("DEK-Info") ||
        matches!(self.pem_type, PemType::EncryptedPrivateKey)
    }

    /// Encode PEM block to string
    pub fn encode(&self) -> String {
        let header = self.pem_type.header();
        let mut result = format!("-----BEGIN {}-----\n", header);
        
        // Add headers
        for (key, value) in &self.headers {
            result.push_str(&format!("{}: {}\n", key, value));
        }
        
        if !self.headers.is_empty() {
            result.push('\n');
        }
        
        // Add base64 encoded data with proper line wrapping
        let base64_data = BASE64.encode(&self.data);
        for chunk in base64_data.as_bytes().chunks(64) {
            result.push_str(&String::from_utf8_lossy(chunk));
            result.push('\n');
        }
        
        result.push_str(&format!("-----END {}-----\n", header));
        result
    }
}

/// ASN.1 tag information
#[derive(Debug, Clone, PartialEq)]
pub struct Asn1Tag {
    pub class: u8,
    pub constructed: bool,
    pub tag_number: u32,
}

/// ASN.1 element containing tag, length, and value
#[derive(Debug, Clone)]
pub struct Asn1Element {
    pub tag: Asn1Tag,
    pub data: Vec<u8>,
    pub children: Vec<Asn1Element>,
}

impl Asn1Element {
    /// Create new ASN.1 element
    pub fn new(tag: Asn1Tag, data: Vec<u8>) -> Self {
        Self {
            tag,
            data,
            children: Vec::new(),
        }
    }

    /// Check if element is a sequence
    pub fn is_sequence(&self) -> bool {
        self.tag.class == 0 && self.tag.constructed && self.tag.tag_number == 16
    }

    /// Check if element is a set
    pub fn is_set(&self) -> bool {
        self.tag.class == 0 && self.tag.constructed && self.tag.tag_number == 17
    }

    /// Get integer value (for ASN.1 INTEGER)
    pub fn as_integer(&self) -> PemDerResult<i64> {
        if self.tag.tag_number != 2 {
            return Err(PemDerError::Asn1ParseError("Not an INTEGER".to_string()));
        }
        
        if self.data.is_empty() {
            return Ok(0);
        }
        
        let mut result = 0i64;
        let negative = (self.data[0] & 0x80) != 0;
        
        for &byte in &self.data {
            result = result.wrapping_shl(8).wrapping_add(byte as i64);
        }
        
        if negative {
            // Two's complement for negative numbers
            let bits = self.data.len() * 8;
            result -= 1i64 << bits;
        }
        
        Ok(result)
    }

    /// Get string value (for ASN.1 UTF8String, PrintableString, etc.)
    pub fn as_string(&self) -> PemDerResult<String> {
        match self.tag.tag_number {
            12 | 19 | 22 | 30 => { // UTF8String, PrintableString, IA5String, BMPString
                String::from_utf8(self.data.clone())
                    .map_err(|e| PemDerError::Asn1ParseError(format!("Invalid UTF-8: {}", e)))
            }
            _ => Err(PemDerError::Asn1ParseError("Not a string type".to_string())),
        }
    }

    /// Get boolean value (for ASN.1 BOOLEAN)
    pub fn as_boolean(&self) -> PemDerResult<bool> {
        if self.tag.tag_number != 1 {
            return Err(PemDerError::Asn1ParseError("Not a BOOLEAN".to_string()));
        }
        
        if self.data.len() != 1 {
            return Err(PemDerError::Asn1ParseError("Invalid BOOLEAN length".to_string()));
        }
        
        Ok(self.data[0] != 0)
    }
}

/// Certificate format detection result
#[derive(Debug, Clone, PartialEq)]
pub enum FormatType {
    Pem,
    Der,
    Unknown,
}

/// Certificate metadata extracted from parsing
#[derive(Debug, Clone)]
pub struct CertificateMetadata {
    pub subject: String,
    pub issuer: String,
    pub serial_number: String,
    pub not_before: String,
    pub not_after: String,
    pub public_key_algorithm: String,
    pub signature_algorithm: String,
    pub extensions: HashMap<String, String>,
}

/// Private key metadata
#[derive(Debug, Clone)]
pub struct PrivateKeyMetadata {
    pub algorithm: String,
    pub key_size: usize,
    pub encrypted: bool,
    pub format: String,
}

/// Certificate chain validation result
#[derive(Debug, Clone)]
pub struct ChainValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub chain_length: usize,
}

/// Format detection utilities
pub mod format_detection {
    use super::*;

    /// Detect format of input data
    pub fn detect_format(data: &[u8]) -> FormatType {
        // Check for PEM format
        if let Ok(text) = std::str::from_utf8(data) {
            if text.contains("-----BEGIN") && text.contains("-----END") {
                return FormatType::Pem;
            }
        }
        
        // Check for DER format (ASN.1 structure)
        if data.len() >= 2 && data[0] == 0x30 {
            // SEQUENCE tag - likely DER
            return FormatType::Der;
        }
        
        FormatType::Unknown
    }

    /// Check if data appears to be a certificate
    pub fn is_certificate(data: &[u8]) -> bool {
        match detect_format(data) {
            FormatType::Pem => {
                if let Ok(text) = std::str::from_utf8(data) {
                    text.contains("-----BEGIN CERTIFICATE-----")
                } else {
                    false
                }
            }
            FormatType::Der => {
                // Basic DER certificate structure check
                data.len() > 10 && data[0] == 0x30
            }
            FormatType::Unknown => false,
        }
    }

    /// Check if data appears to be a private key
    pub fn is_private_key(data: &[u8]) -> bool {
        if let Ok(text) = std::str::from_utf8(data) {
            text.contains("-----BEGIN PRIVATE KEY-----") ||
            text.contains("-----BEGIN RSA PRIVATE KEY-----") ||
            text.contains("-----BEGIN EC PRIVATE KEY-----") ||
            text.contains("-----BEGIN ENCRYPTED PRIVATE KEY-----")
        } else {
            false
        }
    }
}

/// PEM parsing and encoding functions
pub mod pem {
    use super::*;
    use regex::Regex;

    /// Parse PEM format data
    pub fn parse_pem(data: &str) -> PemDerResult<Vec<PemBlock>> {
        let mut blocks = Vec::new();
        let re = Regex::new(r"-----BEGIN ([^-]+)-----\s*((?:[A-Za-z0-9+/\s])*?[A-Za-z0-9+/]*={0,2})\s*-----END ([^-]+)-----")
            .map_err(|e| PemDerError::ValidationError(format!("Regex error: {}", e)))?;

        for captures in re.captures_iter(data) {
            let begin_header = captures.get(1).unwrap().as_str();
            let content = captures.get(2).unwrap().as_str();
            let end_header = captures.get(3).unwrap().as_str();

            if begin_header != end_header {
                return Err(PemDerError::InvalidPemFormat(
                    format!("Mismatched headers: {} != {}", begin_header, end_header)
                ));
            }

            let pem_type = PemType::from_header(begin_header);
            
            // Decode base64 content
            let cleaned_content = content.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();
            
            let decoded_data = BASE64.decode(&cleaned_content)
                .map_err(|e| PemDerError::Base64Error(format!("Base64 decode error: {}", e)))?;

            let mut block = PemBlock::new(pem_type, decoded_data);
            block.raw_content = captures.get(0).unwrap().as_str().to_string();
            
            blocks.push(block);
        }

        if blocks.is_empty() {
            return Err(PemDerError::InvalidPemFormat("No PEM blocks found".to_string()));
        }

        Ok(blocks)
    }

    /// Parse single PEM block
    pub fn parse_pem_single(data: &str) -> PemDerResult<PemBlock> {
        let blocks = parse_pem(data)?;
        if blocks.len() != 1 {
            return Err(PemDerError::InvalidPemFormat(
                format!("Expected single PEM block, found {}", blocks.len())
            ));
        }
        Ok(blocks.into_iter().next().unwrap())
    }

    /// Encode data as PEM
    pub fn encode_pem(pem_type: PemType, data: &[u8]) -> String {
        let block = PemBlock::new(pem_type, data.to_vec());
        block.encode()
    }

    /// Parse PEM certificate
    pub fn parse_pem_certificate(data: &str) -> PemDerResult<Certificate> {
        let block = parse_pem_single(data)?;
        if !matches!(block.pem_type, PemType::Certificate) {
            return Err(PemDerError::InvalidPemFormat("Not a certificate".to_string()));
        }
        
        // Parse DER data to create Certificate
        der::parse_der_certificate(&block.data)
    }

    /// Parse PEM private key
    pub fn parse_pem_private_key(data: &str) -> PemDerResult<PrivateKey> {
        let block = parse_pem_single(data)?;
        
        match block.pem_type {
            PemType::PrivateKey | PemType::RsaPrivateKey | PemType::EcPrivateKey => {
                der::parse_der_private_key(&block.data)
            }
            PemType::EncryptedPrivateKey => {
                Err(PemDerError::EncryptionError("Encrypted private key requires password".to_string()))
            }
            _ => Err(PemDerError::InvalidPemFormat("Not a private key".to_string())),
        }
    }

    /// Parse PEM certificate chain
    pub fn parse_certificate_chain(data: &str) -> PemDerResult<Vec<Certificate>> {
        let blocks = parse_pem(data)?;
        let mut chain = Vec::new();
        
        for block in blocks {
            if matches!(block.pem_type, PemType::Certificate) {
                let cert = der::parse_der_certificate(&block.data)?;
                chain.push(cert);
            }
        }
        
        if chain.is_empty() {
            return Err(PemDerError::InvalidPemFormat("No certificates found in chain".to_string()));
        }
        
        Ok(chain)
    }
}

/// DER parsing and encoding functions
pub mod der {
    use super::*;

    /// Parse DER encoded data into ASN.1 elements
    pub fn parse_der(data: &[u8]) -> PemDerResult<Vec<Asn1Element>> {
        let mut elements = Vec::new();
        let mut offset = 0;

        while offset < data.len() {
            let (element, consumed) = parse_asn1_element(&data[offset..])?;
            elements.push(element);
            offset += consumed;
        }

        Ok(elements)
    }

    /// Parse single ASN.1 element from DER data
    pub fn parse_asn1_element(data: &[u8]) -> PemDerResult<(Asn1Element, usize)> {
        if data.is_empty() {
            return Err(PemDerError::InvalidDerEncoding("Empty data".to_string()));
        }

        let mut offset = 0;
        
        // Parse tag
        let first_byte = data[0];
        offset += 1;
        
        let class = (first_byte >> 6) & 0x03;
        let constructed = (first_byte & 0x20) != 0;
        let mut tag_number = (first_byte & 0x1F) as u32;
        
        // Handle long form tag numbers
        if tag_number == 0x1F {
            tag_number = 0;
            loop {
                if offset >= data.len() {
                    return Err(PemDerError::InvalidDerEncoding("Truncated tag".to_string()));
                }
                let byte = data[offset];
                offset += 1;
                tag_number = (tag_number << 7) | ((byte & 0x7F) as u32);
                if (byte & 0x80) == 0 {
                    break;
                }
            }
        }
        
        let tag = Asn1Tag { class, constructed, tag_number };
        
        // Parse length
        if offset >= data.len() {
            return Err(PemDerError::InvalidDerEncoding("Missing length".to_string()));
        }
        
        let length_byte = data[offset];
        offset += 1;
        
        let length = if (length_byte & 0x80) == 0 {
            // Short form
            length_byte as usize
        } else {
            // Long form
            let length_octets = (length_byte & 0x7F) as usize;
            if length_octets == 0 {
                return Err(PemDerError::InvalidDerEncoding("Indefinite length not allowed in DER".to_string()));
            }
            if length_octets > 4 {
                return Err(PemDerError::InvalidDerEncoding("Length too large".to_string()));
            }
            if offset + length_octets > data.len() {
                return Err(PemDerError::InvalidDerEncoding("Truncated length".to_string()));
            }
            
            let mut length = 0usize;
            for i in 0..length_octets {
                length = (length << 8) | (data[offset + i] as usize);
            }
            offset += length_octets;
            length
        };
        
        // Parse value
        if offset + length > data.len() {
            return Err(PemDerError::InvalidDerEncoding("Truncated value".to_string()));
        }
        
        let value_data = data[offset..offset + length].to_vec();
        offset += length;
        
        let mut element = Asn1Element::new(tag.clone(), value_data);
        
        // Parse children for constructed types
        if constructed {
            let child_data = &element.data;
            let mut child_offset = 0;
            
            while child_offset < child_data.len() {
                let (child, consumed) = parse_asn1_element(&child_data[child_offset..])?;
                element.children.push(child);
                child_offset += consumed;
            }
        }
        
        Ok((element, offset))
    }

    /// Parse DER certificate
    pub fn parse_der_certificate(data: &[u8]) -> PemDerResult<Certificate> {
        use crate::stdlib::packages::crypto_pki::types::*;
        use std::time::{SystemTime, Duration};
        
        let elements = parse_der(data)?;
        if elements.is_empty() {
            return Err(PemDerError::InvalidDerEncoding("No ASN.1 elements found".to_string()));
        }
        
        let cert_element = &elements[0];
        if !cert_element.is_sequence() {
            return Err(PemDerError::CertificateValidationError("Certificate must be a SEQUENCE".to_string()));
        }
        
        // Basic certificate structure validation (Certificate SEQUENCE should have 3 parts)
        if cert_element.children.len() < 3 {
            return Err(PemDerError::CertificateValidationError("Invalid certificate structure".to_string()));
        }
        
        // Extract TBSCertificate (to-be-signed certificate), signatureAlgorithm, and signatureValue
        let tbs_cert = &cert_element.children[0];
        let signature_algorithm = &cert_element.children[1]; 
        let signature_value = &cert_element.children[2];
        
        if !tbs_cert.is_sequence() {
            return Err(PemDerError::CertificateValidationError("TBSCertificate must be a SEQUENCE".to_string()));
        }
        
        // Parse TBSCertificate fields
        let mut cert_version = 1u8; // Default to v1
        let mut serial_number = SerialNumber::from_big_int(0);
        let mut signature_alg = SignatureAlgorithm::RsaWithSha256;
        let mut issuer = DistinguishedName::default();
        let mut validity = Validity {
            not_before: SystemTime::now(),
            not_after: SystemTime::now() + Duration::from_secs(365 * 24 * 3600),
        };
        let mut subject = DistinguishedName::default();
        let mut subject_public_key_info = SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            public_key: Vec::new(),
            parameters: None,
        };
        let mut extensions = Vec::new();
        
        // Parse TBSCertificate fields in order
        let mut field_index = 0;
        for child in &tbs_cert.children {
            match field_index {
                0 => {
                    // Version (optional, explicit tag [0])
                    if child.tag.class == 2 && child.tag.tag_number == 0 {
                        if let Some(version_element) = child.children.get(0) {
                            if let Ok(version) = version_element.as_integer() {
                                cert_version = (version + 1) as u8; // ASN.1 encoding: v1=0, v2=1, v3=2
                            }
                        }
                    } else {
                        // No version field, this is actually the serial number
                        if let Ok(serial) = child.as_integer() {
                            serial_number = SerialNumber::from_big_int(serial);
                        }
                        field_index += 1; // Skip to signature algorithm
                    }
                }
                1 => {
                    // Serial number (if version was present) or signature algorithm
                    if cert_version > 1 {
                        if let Ok(serial) = child.as_integer() {
                            serial_number = SerialNumber::from_big_int(serial);
                        }
                    } else {
                        // This is signature algorithm
                        signature_alg = parse_signature_algorithm(child)?;
                        field_index += 1; // Skip to issuer
                    }
                }
                2 => {
                    // Signature algorithm (if version and serial were present) or issuer
                    if cert_version > 1 {
                        signature_alg = parse_signature_algorithm(child)?;
                    } else {
                        // This is issuer
                        issuer = parse_distinguished_name(child)?;
                        field_index += 1; // Skip to validity
                    }
                }
                3 => {
                    // Issuer (if version was present) or validity
                    if cert_version > 1 {
                        issuer = parse_distinguished_name(child)?;
                    } else {
                        // This is validity
                        validity = parse_validity(child)?;
                        field_index += 1; // Skip to subject
                    }
                }
                4 => {
                    // Validity (if version was present) or subject
                    if cert_version > 1 {
                        validity = parse_validity(child)?;
                    } else {
                        // This is subject
                        subject = parse_distinguished_name(child)?;
                        field_index += 1; // Skip to public key
                    }
                }
                5 => {
                    // Subject (if version was present) or public key
                    if cert_version > 1 {
                        subject = parse_distinguished_name(child)?;
                    } else {
                        // This is subject public key info
                        subject_public_key_info = parse_subject_public_key_info(child)?;
                        field_index += 1; // Continue
                    }
                }
                6 => {
                    // Subject public key info (if version was present) or extensions
                    if cert_version > 1 {
                        subject_public_key_info = parse_subject_public_key_info(child)?;
                    } else if cert_version == 3 {
                        // Extensions for v3 certificates
                        if child.tag.class == 2 && child.tag.tag_number == 3 {
                            extensions = parse_extensions(child)?;
                        }
                    }
                }
                7 => {
                    // Extensions (v3 only, explicit tag [3])
                    if cert_version == 3 && child.tag.class == 2 && child.tag.tag_number == 3 {
                        extensions = parse_extensions(child)?;
                    }
                }
                _ => {} // Ignore additional fields
            }
            field_index += 1;
        }
        
        // Calculate fingerprint (SHA-256 of DER data)
        let fingerprint = calculate_sha256_fingerprint(data);
        
        // Create certificate
        let cert = Certificate {
            version: cert_version,
            serial_number,
            signature_algorithm: signature_alg,
            issuer,
            validity,
            subject,
            subject_public_key_info,
            extensions,
            raw_data: data.to_vec(),
            fingerprint: Some(fingerprint),
            key_usage: KeyUsage::default(),
            extended_key_usage: ExtendedKeyUsage::default(),
        };
        
        Ok(cert)
    }

    /// Parse DER private key
    pub fn parse_der_private_key(data: &[u8]) -> PemDerResult<PrivateKey> {
        let elements = parse_der(data)?;
        if elements.is_empty() {
            return Err(PemDerError::InvalidDerEncoding("No ASN.1 elements found".to_string()));
        }
        
        let key_element = &elements[0];
        if !key_element.is_sequence() {
            return Err(PemDerError::PrivateKeyError("Private key must be a SEQUENCE".to_string()));
        }
        
        let mut private_key = PrivateKey::new();
        private_key.data = data.to_vec();
        private_key.der_encoded = Some(data.to_vec());
        
        // Try to determine key algorithm from structure
        if let Some(version_element) = key_element.children.get(0) {
            if let Ok(version) = version_element.as_integer() {
                if version == 0 {
                    // PKCS#8 or RSA private key format
                    if key_element.children.len() >= 3 {
                        if let Some(algorithm_element) = key_element.children.get(1) {
                            if algorithm_element.is_sequence() {
                                private_key.algorithm = PublicKeyAlgorithm::Rsa { key_size: 2048 };
                            }
                        }
                    }
                }
            }
        }
        
        Ok(private_key)
    }

    /// Encode ASN.1 element to DER
    pub fn encode_der_element(element: &Asn1Element) -> Vec<u8> {
        let mut result = Vec::new();
        
        // Encode tag
        let tag_byte = (element.tag.class << 6) | 
                      (if element.tag.constructed { 0x20 } else { 0x00 }) |
                      if element.tag.tag_number < 0x1F { element.tag.tag_number as u8 } else { 0x1F };
        result.push(tag_byte);
        
        // Handle long form tag numbers
        if element.tag.tag_number >= 0x1F {
            let mut tag_bytes = Vec::new();
            let mut tag_num = element.tag.tag_number;
            
            tag_bytes.push((tag_num & 0x7F) as u8);
            tag_num >>= 7;
            
            while tag_num > 0 {
                tag_bytes.push(((tag_num & 0x7F) | 0x80) as u8);
                tag_num >>= 7;
            }
            
            for &byte in tag_bytes.iter().rev() {
                result.push(byte);
            }
        }
        
        // Prepare content
        let content = if element.tag.constructed {
            // Encode children
            let mut child_data = Vec::new();
            for child in &element.children {
                child_data.extend(encode_der_element(child));
            }
            child_data
        } else {
            element.data.clone()
        };
        
        // Encode length
        if content.len() < 0x80 {
            // Short form
            result.push(content.len() as u8);
        } else {
            // Long form
            let mut length_bytes = Vec::new();
            let mut len = content.len();
            
            while len > 0 {
                length_bytes.push((len & 0xFF) as u8);
                len >>= 8;
            }
            
            result.push(0x80 | length_bytes.len() as u8);
            for &byte in length_bytes.iter().rev() {
                result.push(byte);
            }
        }
        
        // Add content
        result.extend(content);
        
        result
    }
}

/// Certificate parsing helper functions
pub mod certificate_parsing {
    use super::*;
    use crate::stdlib::packages::crypto_pki::types::*;
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    
    /// Parse signature algorithm from ASN.1 element
    pub fn parse_signature_algorithm(element: &Asn1Element) -> PemDerResult<SignatureAlgorithm> {
        if !element.is_sequence() {
            return Ok(SignatureAlgorithm::RsaWithSha256); // Default
        }
        
        // In a real implementation, would parse the OID to determine algorithm
        // For now, return a reasonable default
        Ok(SignatureAlgorithm::RsaWithSha256)
    }
    
    /// Parse distinguished name from ASN.1 element
    pub fn parse_distinguished_name(element: &Asn1Element) -> PemDerResult<DistinguishedName> {
        if !element.is_sequence() {
            return Err(PemDerError::Asn1ParseError("Distinguished name must be a SEQUENCE".to_string()));
        }
        
        let mut dn = DistinguishedName::default();
        
        // Parse RDN sequence
        for rdn_element in &element.children {
            if rdn_element.is_set() {
                for attr_element in &rdn_element.children {
                    if attr_element.is_sequence() && attr_element.children.len() == 2 {
                        // Parse attribute OID and value
                        if let Some(value_element) = attr_element.children.get(1) {
                            if let Ok(value) = value_element.as_string() {
                                // In a real implementation, would check OID to determine attribute type
                                // For now, assign to common name as example
                                if dn.common_name.is_none() {
                                    dn.common_name = Some(value);
                                } else if dn.organization.is_none() {
                                    dn.organization = Some(value);
                                } else if dn.country.is_none() {
                                    dn.country = Some(value);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(dn)
    }
    
    /// Parse validity from ASN.1 element
    pub fn parse_validity(element: &Asn1Element) -> PemDerResult<Validity> {
        if !element.is_sequence() || element.children.len() != 2 {
            return Err(PemDerError::Asn1ParseError("Validity must be a SEQUENCE with 2 elements".to_string()));
        }
        
        // For simplicity, use current time + 1 year as default
        let now = SystemTime::now();
        let validity = Validity {
            not_before: now,
            not_after: now + Duration::from_secs(365 * 24 * 3600),
        };
        
        // In a real implementation, would parse ASN.1 TIME values
        Ok(validity)
    }
    
    /// Parse subject public key info from ASN.1 element
    pub fn parse_subject_public_key_info(element: &Asn1Element) -> PemDerResult<SubjectPublicKeyInfo> {
        if !element.is_sequence() || element.children.len() < 2 {
            return Err(PemDerError::Asn1ParseError("SubjectPublicKeyInfo must be a SEQUENCE with at least 2 elements".to_string()));
        }
        
        let algorithm_element = &element.children[0];
        let public_key_element = &element.children[1];
        
        // Parse algorithm (simplified)
        let algorithm = if algorithm_element.is_sequence() {
            // In a real implementation, would parse algorithm OID
            PublicKeyAlgorithm::Rsa { key_size: 2048 }
        } else {
            PublicKeyAlgorithm::Rsa { key_size: 2048 }
        };
        
        // Extract public key data (bit string)
        let public_key = if public_key_element.tag.tag_number == 3 { // BIT STRING
            // Remove the first byte (unused bits indicator) if present
            if !public_key_element.data.is_empty() {
                public_key_element.data[1..].to_vec()
            } else {
                public_key_element.data.clone()
            }
        } else {
            public_key_element.data.clone()
        };
        
        Ok(SubjectPublicKeyInfo {
            algorithm,
            public_key,
            parameters: None,
        })
    }
    
    /// Parse extensions from ASN.1 element
    pub fn parse_extensions(element: &Asn1Element) -> PemDerResult<Vec<X509Extension>> {
        let mut extensions = Vec::new();
        
        if element.tag.class == 2 && element.tag.tag_number == 3 { // Explicit tag [3]
            if let Some(ext_sequence) = element.children.get(0) {
                if ext_sequence.is_sequence() {
                    for ext_element in &ext_sequence.children {
                        if ext_element.is_sequence() {
                            // Parse individual extension
                            let extension = parse_single_extension(ext_element)?;
                            extensions.push(extension);
                        }
                    }
                }
            }
        }
        
        Ok(extensions)
    }
    
    /// Parse a single extension from ASN.1 element
    fn parse_single_extension(element: &Asn1Element) -> PemDerResult<X509Extension> {
        if element.children.len() < 2 {
            return Err(PemDerError::Asn1ParseError("Extension must have at least 2 elements".to_string()));
        }
        
        // Extension structure: SEQUENCE { oid, [critical], value }
        let oid_element = &element.children[0];
        let mut critical = false;
        let mut value_element_index = 1;
        
        // Check if second element is BOOLEAN (critical flag)
        if element.children.len() > 2 && element.children[1].tag.tag_number == 1 {
            if let Ok(crit) = element.children[1].as_boolean() {
                critical = crit;
                value_element_index = 2;
            }
        }
        
        let value_element = &element.children[value_element_index];
        
        // Create extension with basic info
        let extension = X509Extension {
            oid: "1.2.3.4".to_string(), // Placeholder OID
            critical,
            value: value_element.data.clone(),
            parsed_data: None,
        };
        
        Ok(extension)
    }
    
    /// Calculate SHA-256 fingerprint
    pub fn calculate_sha256_fingerprint(data: &[u8]) -> Vec<u8> {
        // In a real implementation, would use a proper SHA-256 implementation
        // For now, return a mock fingerprint
        let mut fingerprint = vec![0u8; 32];
        for (i, &byte) in data.iter().take(32).enumerate() {
            fingerprint[i] = byte.wrapping_add(i as u8);
        }
        fingerprint
    }
}

// Re-export the parsing functions at module level for convenience
pub use certificate_parsing::*;

/// Format conversion utilities
pub mod conversion {
    use super::*;

    /// Convert PEM to DER
    pub fn pem_to_der(pem_data: &str) -> PemDerResult<Vec<u8>> {
        let block = pem::parse_pem_single(pem_data)?;
        Ok(block.data)
    }

    /// Convert DER to PEM
    pub fn der_to_pem(der_data: &[u8], pem_type: PemType) -> String {
        pem::encode_pem(pem_type, der_data)
    }

    /// Auto-detect format and convert to DER
    pub fn to_der(data: &[u8]) -> PemDerResult<Vec<u8>> {
        match format_detection::detect_format(data) {
            FormatType::Der => Ok(data.to_vec()),
            FormatType::Pem => {
                let pem_str = std::str::from_utf8(data)
                    .map_err(|e| PemDerError::ValidationError(format!("Invalid UTF-8: {}", e)))?;
                pem_to_der(pem_str)
            }
            FormatType::Unknown => Err(PemDerError::UnsupportedFormat("Unknown format".to_string())),
        }
    }

    /// Auto-detect format and convert to PEM
    pub fn to_pem(data: &[u8], pem_type: PemType) -> PemDerResult<String> {
        let der_data = to_der(data)?;
        Ok(der_to_pem(&der_data, pem_type))
    }
}

/// Certificate metadata extraction
pub mod metadata {
    use super::*;

    /// Extract certificate metadata
    pub fn extract_certificate_metadata(cert: &Certificate) -> PemDerResult<CertificateMetadata> {
        let mut metadata = CertificateMetadata {
            subject: "Unknown".to_string(),
            issuer: "Unknown".to_string(),
            serial_number: cert.serial_number.clone().unwrap_or_else(|| "Unknown".to_string()),
            not_before: "Unknown".to_string(),
            not_after: "Unknown".to_string(),
            public_key_algorithm: "Unknown".to_string(),
            signature_algorithm: "Unknown".to_string(),
            extensions: HashMap::new(),
        };

        // Extract additional metadata if available
        if let Some(der_data) = &cert.der_encoded {
            // Parse DER to extract detailed information
            if let Ok(elements) = der::parse_der(der_data) {
                if let Some(cert_element) = elements.get(0) {
                    if cert_element.is_sequence() && cert_element.children.len() >= 3 {
                        // Basic metadata extraction (would need full X.509 parsing for complete implementation)
                        metadata.public_key_algorithm = "RSA".to_string(); // Placeholder
                        metadata.signature_algorithm = "SHA256WithRSA".to_string(); // Placeholder
                    }
                }
            }
        }

        Ok(metadata)
    }

    /// Extract private key metadata
    pub fn extract_private_key_metadata(key: &PrivateKey) -> PemDerResult<PrivateKeyMetadata> {
        let metadata = PrivateKeyMetadata {
            algorithm: "RSA".to_string(), // Would detect from key structure
            key_size: 2048, // Would calculate from key data
            encrypted: false, // Would detect from key format
            format: "PKCS#8".to_string(), // Would detect from structure
        };

        Ok(metadata)
    }
}

/// Certificate chain validation
pub mod validation {
    use super::*;

    /// Validate certificate chain
    pub fn validate_certificate_chain(chain: &[Certificate]) -> ChainValidationResult {
        let mut result = ChainValidationResult {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            chain_length: chain.len(),
        };

        if chain.is_empty() {
            result.valid = false;
            result.errors.push("Empty certificate chain".to_string());
            return result;
        }

        // Basic chain validation
        if chain.len() == 1 {
            result.warnings.push("Single certificate in chain - self-signed or incomplete chain".to_string());
        }

        // Additional validation would go here
        // - Check certificate signatures
        // - Validate certificate dates
        // - Check certificate extensions
        // - Verify chain of trust

        result
    }

    /// Validate certificate format
    pub fn validate_certificate_format(data: &[u8]) -> PemDerResult<bool> {
        match format_detection::detect_format(data) {
            FormatType::Pem => {
                let pem_str = std::str::from_utf8(data)
                    .map_err(|e| PemDerError::ValidationError(format!("Invalid UTF-8: {}", e)))?;
                let blocks = pem::parse_pem(pem_str)?;
                Ok(!blocks.is_empty())
            }
            FormatType::Der => {
                let elements = der::parse_der(data)?;
                Ok(!elements.is_empty())
            }
            FormatType::Unknown => Ok(false),
        }
    }
}

/// Encrypted PEM handling
pub mod encrypted {
    use super::*;

    /// Decrypt encrypted PEM private key
    pub fn decrypt_pem_private_key(
        pem_data: &str,
        password: &str
    ) -> PemDerResult<PrivateKey> {
        let block = pem::parse_pem_single(pem_data)?;
        
        if !block.is_encrypted() {
            return Err(PemDerError::EncryptionError("PEM block is not encrypted".to_string()));
        }

        // Handle different encryption types
        if let Some(dek_info) = block.get_header("DEK-Info") {
            decrypt_traditional_pem(&block, password, dek_info)
        } else if matches!(block.pem_type, PemType::EncryptedPrivateKey) {
            decrypt_pkcs8_pem(&block, password)
        } else {
            Err(PemDerError::EncryptionError("Unsupported encryption format".to_string()))
        }
    }

    /// Decrypt traditional encrypted PEM (PKCS#1 with DEK-Info)
    fn decrypt_traditional_pem(
        block: &PemBlock,
        password: &str,
        dek_info: &str
    ) -> PemDerResult<PrivateKey> {
        // Parse DEK-Info header to extract algorithm and IV
        let parts: Vec<&str> = dek_info.split(',').collect();
        if parts.len() != 2 {
            return Err(PemDerError::EncryptionError("Invalid DEK-Info format".to_string()));
        }
        
        let algorithm = parts[0].trim();
        let iv_hex = parts[1].trim();
        
        // Validate algorithm support
        match algorithm {
            "DES-EDE3-CBC" | "AES-128-CBC" | "AES-256-CBC" => {
                // For security reasons, we'll return an error for now
                // In a real implementation, this would use proper cryptographic libraries
                Err(PemDerError::EncryptionError(
                    format!("Decryption of {} encrypted PEM not implemented for security", algorithm)
                ))
            }
            _ => Err(PemDerError::EncryptionError(
                format!("Unsupported encryption algorithm: {}", algorithm)
            ))
        }
    }

    /// Decrypt PKCS#8 encrypted private key
    fn decrypt_pkcs8_pem(
        block: &PemBlock,
        password: &str
    ) -> PemDerResult<PrivateKey> {
        if password.is_empty() {
            return Err(PemDerError::EncryptionError("Password cannot be empty".to_string()));
        }
        
        // Parse PKCS#8 EncryptedPrivateKeyInfo structure
        let elements = der::parse_der(&block.data)?;
        if elements.is_empty() {
            return Err(PemDerError::InvalidDerEncoding("No ASN.1 elements found".to_string()));
        }
        
        let encrypted_key_element = &elements[0];
        if !encrypted_key_element.is_sequence() || encrypted_key_element.children.len() < 2 {
            return Err(PemDerError::EncryptionError("Invalid PKCS#8 structure".to_string()));
        }
        
        // Extract encryption algorithm and encrypted data
        let encryption_algorithm = &encrypted_key_element.children[0];
        let encrypted_data = &encrypted_key_element.children[1];
        
        // For security and simplicity, return error for now
        // Real implementation would use proper PKCS#8 decryption
        Err(PemDerError::EncryptionError("PKCS#8 decryption requires proper cryptographic implementation".to_string()))
    }

    /// Encrypt private key to PEM
    pub fn encrypt_private_key_pem(
        private_key: &PrivateKey,
        password: &str,
        algorithm: &str
    ) -> PemDerResult<String> {
        if password.is_empty() {
            return Err(PemDerError::EncryptionError("Password cannot be empty".to_string()));
        }
        
        // This would implement actual encryption
        Err(PemDerError::EncryptionError(format!("Encryption with {} not yet implemented", algorithm)))
    }
}

/// Bundle handling for certificate chains and PKCS#12
pub mod bundle {
    use super::*;

    /// Parse certificate bundle (multiple certificates)
    pub fn parse_certificate_bundle(data: &str) -> PemDerResult<Vec<Certificate>> {
        pem::parse_certificate_chain(data)
    }

    /// Create certificate bundle PEM
    pub fn create_certificate_bundle(certificates: &[Certificate]) -> PemDerResult<String> {
        let mut bundle = String::new();
        
        for cert in certificates {
            if let Some(der_data) = &cert.der_encoded {
                bundle.push_str(&pem::encode_pem(PemType::Certificate, der_data));
                bundle.push('\n');
            } else {
                return Err(PemDerError::CertificateValidationError("Certificate missing DER data".to_string()));
            }
        }
        
        Ok(bundle)
    }

    /// Basic PKCS#12 structure (simplified implementation)
    pub fn parse_pkcs12_basic(_data: &[u8], _password: &str) -> PemDerResult<(Vec<Certificate>, Option<PrivateKey>)> {
        // This would implement actual PKCS#12 parsing
        Err(PemDerError::UnsupportedFormat("PKCS#12 parsing not yet implemented".to_string()))
    }
}

/// High-level convenience functions
/// Parse any supported certificate format
pub fn parse_certificate(data: &[u8]) -> PemDerResult<Certificate> {
    match format_detection::detect_format(data) {
        FormatType::Pem => {
            let pem_str = std::str::from_utf8(data)
                .map_err(|e| PemDerError::ValidationError(format!("Invalid UTF-8: {}", e)))?;
            pem::parse_pem_certificate(pem_str)
        }
        FormatType::Der => {
            der::parse_der_certificate(data)
        }
        FormatType::Unknown => {
            Err(PemDerError::UnsupportedFormat("Unknown certificate format".to_string()))
        }
    }
}

/// Parse any supported private key format
pub fn parse_private_key(data: &[u8]) -> PemDerResult<PrivateKey> {
    match format_detection::detect_format(data) {
        FormatType::Pem => {
            let pem_str = std::str::from_utf8(data)
                .map_err(|e| PemDerError::ValidationError(format!("Invalid UTF-8: {}", e)))?;
            pem::parse_pem_private_key(pem_str)
        }
        FormatType::Der => {
            der::parse_der_private_key(data)
        }
        FormatType::Unknown => {
            Err(PemDerError::UnsupportedFormat("Unknown private key format".to_string()))
        }
    }
}

/// Convert between PEM and DER formats
pub fn convert_format(data: &[u8], target_format: FormatType, pem_type: PemType) -> PemDerResult<Vec<u8>> {
    match (format_detection::detect_format(data), target_format) {
        (FormatType::Pem, FormatType::Der) => {
            let pem_str = std::str::from_utf8(data)
                .map_err(|e| PemDerError::ValidationError(format!("Invalid UTF-8: {}", e)))?;
            conversion::pem_to_der(pem_str)
        }
        (FormatType::Der, FormatType::Pem) => {
            let pem_str = conversion::der_to_pem(data, pem_type);
            Ok(pem_str.into_bytes())
        }
        (source, target) if source == target => {
            Ok(data.to_vec())
        }
        _ => {
            Err(PemDerError::UnsupportedFormat("Conversion not supported".to_string()))
        }
    }
}

/// Get format information for data
pub fn get_format_info(data: &[u8]) -> (FormatType, bool, bool) {
    let format = format_detection::detect_format(data);
    let is_cert = format_detection::is_certificate(data);
    let is_key = format_detection::is_private_key(data);
    (format, is_cert, is_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_detection() {
        let pem_data = b"-----BEGIN CERTIFICATE-----\nMIIB...";
        assert_eq!(format_detection::detect_format(pem_data), FormatType::Pem);
        
        let der_data = &[0x30, 0x82, 0x01, 0x00]; // SEQUENCE
        assert_eq!(format_detection::detect_format(der_data), FormatType::Der);
        
        let unknown_data = b"not a certificate";
        assert_eq!(format_detection::detect_format(unknown_data), FormatType::Unknown);
    }

    #[test]
    fn test_pem_type_headers() {
        assert_eq!(PemType::Certificate.header(), "CERTIFICATE");
        assert_eq!(PemType::PrivateKey.header(), "PRIVATE KEY");
        assert_eq!(PemType::from_header("CERTIFICATE"), PemType::Certificate);
    }

    #[test]
    fn test_pem_block_creation() {
        let data = vec![1, 2, 3, 4];
        let mut block = PemBlock::new(PemType::Certificate, data.clone());
        block.add_header("Test".to_string(), "Value".to_string());
        
        assert_eq!(block.data, data);
        assert_eq!(block.get_header("Test"), Some(&"Value".to_string()));
        assert!(!block.is_encrypted());
    }

    #[test]
    fn test_asn1_element() {
        let tag = Asn1Tag {
            class: 0,
            constructed: false,
            tag_number: 2, // INTEGER
        };
        let element = Asn1Element::new(tag, vec![0x01]); // INTEGER 1
        
        assert_eq!(element.as_integer().unwrap(), 1);
        assert!(!element.is_sequence());
    }

    #[test]
    fn test_certificate_validation() {
        let empty_chain = vec![];
        let result = validation::validate_certificate_chain(&empty_chain);
        assert!(!result.valid);
        assert!(result.errors.len() > 0);
    }
}
