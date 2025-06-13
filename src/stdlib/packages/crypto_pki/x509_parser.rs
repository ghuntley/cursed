//! X.509 Certificate Parser
//! 
//! Comprehensive X.509 certificate parsing and validation functionality.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::stdlib::packages::crypto_pki::types::*;
use crate::stdlib::packages::crypto_pki::error::{PkiError, PkiResult, CertificateErrorCode};

/// X.509 certificate parser with support for DER and PEM formats
#[derive(Debug)]
pub struct X509Parser {
    /// Parser configuration
    config: ParserConfig,
    /// Supported extensions registry
    supported_extensions: HashMap<String, ExtensionParser>,
}

/// Parser configuration options
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Strict mode: fail on unsupported extensions
    pub strict_mode: bool,
    /// Parse extension values
    pub parse_extensions: bool,
    /// Validate signature during parsing
    pub validate_signature: bool,
    /// Maximum certificate size (bytes)
    pub max_certificate_size: usize,
    /// Decode text fields as UTF-8
    pub utf8_mode: bool,
}

/// Extension parser function type
type ExtensionParser = fn(&[u8]) -> PkiResult<ExtensionData>;

/// Temporary TBS certificate structure for parsing
#[derive(Debug)]
struct TbsCertificate {
    version: u8,
    serial_number: SerialNumber,
    signature_algorithm: SignatureAlgorithm,
    issuer: DistinguishedName,
    validity: Validity,
    subject: DistinguishedName,
    subject_public_key_info: SubjectPublicKeyInfo,
    extensions: Vec<X509Extension>,
}

/// ASN.1 parsing context for certificate data
#[derive(Debug)]
struct Asn1Context {
    /// Raw data being parsed
    data: Vec<u8>,
    /// Current position
    position: usize,
    /// Parsing stack depth (for recursion protection)
    depth: usize,
    /// Maximum parsing depth
    max_depth: usize,
}

/// ASN.1 tag types
#[derive(Debug, Clone, Copy, PartialEq)]
enum Asn1Tag {
    Boolean = 0x01,
    Integer = 0x02,
    BitString = 0x03,
    OctetString = 0x04,
    Null = 0x05,
    ObjectIdentifier = 0x06,
    Utf8String = 0x0C,
    PrintableString = 0x13,
    T61String = 0x14,
    Ia5String = 0x16,
    UtcTime = 0x17,
    GeneralizedTime = 0x18,
    Sequence = 0x30,
    Set = 0x31,
    ContextSpecific = 0x80,
}

impl X509Parser {
    /// Create a new X.509 parser with default configuration
    pub fn new() -> Self {
        let mut parser = Self {
            config: ParserConfig::default(),
            supported_extensions: HashMap::new(),
        };
        parser.register_default_extensions();
        parser
    }
    
    /// Create a parser with custom configuration
    pub fn with_config(config: ParserConfig) -> Self {
        let mut parser = Self {
            config,
            supported_extensions: HashMap::new(),
        };
        parser.register_default_extensions();
        parser
    }
    
    /// Parse a certificate from DER encoded bytes
    pub fn parse_der(&self, der_data: &[u8]) -> PkiResult<X509Certificate> {
        if der_data.len() > self.config.max_certificate_size {
            return Err(PkiError::certificate_error(
                format!("Certificate too large: {} bytes", der_data.len()),
                CertificateErrorCode::MalformedCertificate
            ));
        }
        
        let mut context = Asn1Context {
            data: der_data.to_vec(),
            position: 0,
            depth: 0,
            max_depth: 32,
        };
        
        self.parse_certificate(&mut context)
    }
    
    /// Parse a certificate from PEM encoded text
    pub fn parse_pem(&self, pem_data: &str) -> PkiResult<X509Certificate> {
        let der_data = self.pem_to_der(pem_data)?;
        self.parse_der(&der_data)
    }
    
    /// Parse multiple certificates from PEM data
    pub fn parse_pem_multiple(&self, pem_data: &str) -> PkiResult<Vec<X509Certificate>> {
        let mut certificates = Vec::new();
        let mut current_cert = String::new();
        let mut in_certificate = false;
        
        for line in pem_data.lines() {
            let line = line.trim();
            
            if line == "-----BEGIN CERTIFICATE-----" {
                in_certificate = true;
                current_cert.clear();
                current_cert.push_str(line);
                current_cert.push('\n');
            } else if line == "-----END CERTIFICATE-----" {
                current_cert.push_str(line);
                current_cert.push('\n');
                
                if in_certificate {
                    let cert = self.parse_pem(&current_cert)?;
                    certificates.push(cert);
                }
                
                in_certificate = false;
                current_cert.clear();
            } else if in_certificate {
                current_cert.push_str(line);
                current_cert.push('\n');
            }
        }
        
        Ok(certificates)
    }
    
    /// Convert PEM to DER format
    fn pem_to_der(&self, pem_data: &str) -> PkiResult<Vec<u8>> {
        let pem_data = pem_data.trim();
        
        // Find BEGIN and END markers
        let begin_marker = "-----BEGIN CERTIFICATE-----";
        let end_marker = "-----END CERTIFICATE-----";
        
        let start = pem_data.find(begin_marker)
            .ok_or_else(|| PkiError::encoding_error(
                "Missing BEGIN CERTIFICATE marker",
                "PEM"
            ))?;
        
        let end = pem_data.find(end_marker)
            .ok_or_else(|| PkiError::encoding_error(
                "Missing END CERTIFICATE marker", 
                "PEM"
            ))?;
        
        if start >= end {
            return Err(PkiError::encoding_error(
                "Invalid PEM structure",
                "PEM"
            ));
        }
        
        // Extract base64 data
        let base64_start = start + begin_marker.len();
        let base64_data = &pem_data[base64_start..end];
        
        // Remove whitespace and decode
        let base64_clean = base64_data.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        
        self.base64_decode(&base64_clean)
    }
    
    /// Base64 decode implementation
    fn base64_decode(&self, base64_data: &str) -> PkiResult<Vec<u8>> {
        const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        
        let mut result = Vec::new();
        let bytes = base64_data.as_bytes();
        
        // Process groups of 4 characters
        for chunk in bytes.chunks(4) {
            let mut group = [0u8; 4];
            let mut padding = 0;
            
            for (i, &byte) in chunk.iter().enumerate() {
                if byte == b'=' {
                    padding += 1;
                    group[i] = 0;
                } else {
                    let pos = BASE64_CHARS.iter().position(|&b| b == byte)
                        .ok_or_else(|| PkiError::encoding_error(
                            format!("Invalid base64 character: {}", byte as char),
                            "Base64"
                        ))?;
                    group[i] = pos as u8;
                }
            }
            
            // Decode 4 characters to 3 bytes
            let decoded = [
                (group[0] << 2) | (group[1] >> 4),
                (group[1] << 4) | (group[2] >> 2),
                (group[2] << 6) | group[3],
            ];
            
            // Add non-padding bytes
            let bytes_to_add = 3 - padding;
            result.extend_from_slice(&decoded[..bytes_to_add]);
        }
        
        Ok(result)
    }
    
    /// Parse certificate from ASN.1 context
    fn parse_certificate(&self, context: &mut Asn1Context) -> PkiResult<X509Certificate> {
        // Certificate ::= SEQUENCE {
        //     tbsCertificate       TBSCertificate,
        //     signatureAlgorithm   AlgorithmIdentifier,
        //     signature            BIT STRING
        // }
        
        let cert_start = context.position;
        self.expect_tag(context, Asn1Tag::Sequence)?;
        let cert_length = self.parse_length(context)?;
        let cert_end = context.position + cert_length;
        
        // Parse TBSCertificate
        let tbs_cert = self.parse_tbs_certificate(context)?;
        
        // Parse signatureAlgorithm
        let signature_algorithm = self.parse_algorithm_identifier(context)?;
        
        // Parse signature
        self.expect_tag(context, Asn1Tag::BitString)?;
        let sig_length = self.parse_length(context)?;
        let _unused_bits = self.read_byte(context)?; // Should be 0
        let signature = self.read_bytes(context, sig_length - 1)?;
        
        if context.position != cert_end {
            return Err(PkiError::certificate_error(
                "Certificate parsing didn't consume all data",
                CertificateErrorCode::MalformedCertificate
            ));
        }
        
        // Calculate fingerprint
        let raw_data = context.data[cert_start..cert_end].to_vec();
        let fingerprint = Some(self.calculate_sha256(&raw_data));
        
        // Build certificate
        let mut cert = X509Certificate {
            version: tbs_cert.version,
            serial_number: tbs_cert.serial_number,
            signature_algorithm,
            issuer: tbs_cert.issuer,
            validity: tbs_cert.validity,
            subject: tbs_cert.subject,
            subject_public_key_info: tbs_cert.subject_public_key_info,
            extensions: tbs_cert.extensions,
            raw_data,
            fingerprint,
            key_usage: KeyUsage::default(),
            extended_key_usage: ExtendedKeyUsage::default(),
        };
        
        // Extract key usage from extensions
        self.extract_key_usage(&mut cert);
        
        // Validate signature if configured
        if self.config.validate_signature {
            self.validate_certificate_signature(&cert)?;
        }
        
        Ok(cert)
    }
    

    
    /// Parse TBSCertificate structure
    fn parse_tbs_certificate(&self, context: &mut Asn1Context) -> PkiResult<TbsCertificate> {
        self.expect_tag(context, Asn1Tag::Sequence)?;
        let _tbs_length = self.parse_length(context)?;
        
        // Parse version (optional, default v1)
        let version = if self.peek_tag(context) == Some(0xA0) {
            self.read_byte(context)?; // consume tag
            let _length = self.parse_length(context)?;
            self.expect_tag(context, Asn1Tag::Integer)?;
            let _int_length = self.parse_length(context)?;
            let version_byte = self.read_byte(context)?;
            version_byte + 1 // X.509 versions are 0-indexed
        } else {
            1 // Default to version 1
        };
        
        // Parse serial number
        self.expect_tag(context, Asn1Tag::Integer)?;
        let serial_length = self.parse_length(context)?;
        let serial_bytes = self.read_bytes(context, serial_length)?;
        let serial_number = SerialNumber::from_bytes(serial_bytes);
        
        // Parse signature algorithm
        let signature_algorithm = self.parse_algorithm_identifier(context)?;
        
        // Parse issuer
        let issuer = self.parse_distinguished_name(context)?;
        
        // Parse validity
        let validity = self.parse_validity(context)?;
        
        // Parse subject
        let subject = self.parse_distinguished_name(context)?;
        
        // Parse subject public key info
        let subject_public_key_info = self.parse_subject_public_key_info(context)?;
        
        // Parse extensions (optional, v3 only)
        let extensions = if version >= 3 && self.peek_tag(context) == Some(0xA3) {
            self.read_byte(context)?; // consume tag
            let _length = self.parse_length(context)?;
            self.parse_extensions(context)?
        } else {
            Vec::new()
        };
        
        Ok(TbsCertificate {
            version,
            serial_number,
            signature_algorithm,
            issuer,
            validity,
            subject,
            subject_public_key_info,
            extensions,
        })
    }
    
    /// Parse algorithm identifier
    fn parse_algorithm_identifier(&self, context: &mut Asn1Context) -> PkiResult<SignatureAlgorithm> {
        self.expect_tag(context, Asn1Tag::Sequence)?;
        let _seq_length = self.parse_length(context)?;
        
        // Parse algorithm OID
        self.expect_tag(context, Asn1Tag::ObjectIdentifier)?;
        let oid_length = self.parse_length(context)?;
        let oid_bytes = self.read_bytes(context, oid_length)?;
        let oid = self.decode_oid(&oid_bytes)?;
        
        // Parse parameters (optional)
        if self.peek_tag(context) == Some(Asn1Tag::Null as u8) {
            self.read_byte(context)?; // consume NULL tag
            let null_length = self.parse_length(context)?;
            if null_length != 0 {
                return Err(PkiError::certificate_error(
                    "Invalid NULL parameter length",
                    CertificateErrorCode::MalformedCertificate
                ));
            }
        }
        
        // Map OID to signature algorithm
        self.oid_to_signature_algorithm(&oid)
    }
    
    /// Parse distinguished name
    fn parse_distinguished_name(&self, context: &mut Asn1Context) -> PkiResult<DistinguishedName> {
        self.expect_tag(context, Asn1Tag::Sequence)?;
        let _seq_length = self.parse_length(context)?;
        
        let mut dn = DistinguishedName::new();
        
        // Parse RDNs (Relative Distinguished Names)
        while context.position < context.data.len() && 
              self.peek_tag(context) == Some(Asn1Tag::Set as u8) {
            
            self.expect_tag(context, Asn1Tag::Set)?;
            let _set_length = self.parse_length(context)?;
            
            // Parse attribute type and value
            self.expect_tag(context, Asn1Tag::Sequence)?;
            let _attr_length = self.parse_length(context)?;
            
            // Parse attribute type (OID)
            self.expect_tag(context, Asn1Tag::ObjectIdentifier)?;
            let oid_length = self.parse_length(context)?;
            let oid_bytes = self.read_bytes(context, oid_length)?;
            let oid = self.decode_oid(&oid_bytes)?;
            
            // Parse attribute value
            let value_tag = self.read_byte(context)?;
            let value_length = self.parse_length(context)?;
            let value_bytes = self.read_bytes(context, value_length)?;
            let value = if self.config.utf8_mode {
                String::from_utf8_lossy(&value_bytes).into_owned()
            } else {
                // Try to decode as ASCII/UTF-8, fallback to hex
                String::from_utf8(value_bytes.clone())
                    .unwrap_or_else(|_| hex::encode(&value_bytes))
            };
            
            // Map OID to DN field
            match oid.as_str() {
                "2.5.4.3" => dn.common_name = Some(value),
                "2.5.4.10" => dn.organization = Some(value),
                "2.5.4.11" => dn.organizational_unit = Some(value),
                "2.5.4.6" => dn.country = Some(value),
                "2.5.4.8" => dn.state_or_province = Some(value),
                "2.5.4.7" => dn.locality = Some(value),
                "1.2.840.113549.1.9.1" => dn.email_address = Some(value),
                _ => {
                    dn.additional_attributes.insert(oid, value);
                }
            }
        }
        
        Ok(dn)
    }
    
    /// Parse validity period
    fn parse_validity(&self, context: &mut Asn1Context) -> PkiResult<Validity> {
        self.expect_tag(context, Asn1Tag::Sequence)?;
        let _seq_length = self.parse_length(context)?;
        
        // Parse notBefore
        let not_before = self.parse_time(context)?;
        
        // Parse notAfter  
        let not_after = self.parse_time(context)?;
        
        Ok(Validity { not_before, not_after })
    }
    
    /// Parse time (UTCTime or GeneralizedTime)
    fn parse_time(&self, context: &mut Asn1Context) -> PkiResult<SystemTime> {
        let tag = self.read_byte(context)?;
        let length = self.parse_length(context)?;
        let time_bytes = self.read_bytes(context, length)?;
        let time_str = String::from_utf8(time_bytes)
            .map_err(|_| PkiError::certificate_error(
                "Invalid time encoding",
                CertificateErrorCode::MalformedCertificate
            ))?;
        
        match tag {
            0x17 => self.parse_utc_time(&time_str), // UTCTime
            0x18 => self.parse_generalized_time(&time_str), // GeneralizedTime
            _ => Err(PkiError::certificate_error(
                format!("Unsupported time tag: 0x{:02x}", tag),
                CertificateErrorCode::MalformedCertificate
            )),
        }
    }
    
    /// Parse UTC time format (YYMMDDHHMMSSZ)
    fn parse_utc_time(&self, time_str: &str) -> PkiResult<SystemTime> {
        if time_str.len() != 13 || !time_str.ends_with('Z') {
            return Err(PkiError::certificate_error(
                format!("Invalid UTC time format: {}", time_str),
                CertificateErrorCode::MalformedCertificate
            ));
        }
        
        let year: u32 = time_str[0..2].parse()
            .map_err(|_| PkiError::certificate_error(
                "Invalid year in UTC time", 
                CertificateErrorCode::MalformedCertificate
            ))?;
        let year = if year >= 50 { 1900 + year } else { 2000 + year };
        
        let month: u32 = time_str[2..4].parse()
            .map_err(|_| PkiError::certificate_error(
                "Invalid month in UTC time",
                CertificateErrorCode::MalformedCertificate
            ))?;
        let day: u32 = time_str[4..6].parse()
            .map_err(|_| PkiError::certificate_error(
                "Invalid day in UTC time",
                CertificateErrorCode::MalformedCertificate
            ))?;
        let hour: u32 = time_str[6..8].parse()
            .map_err(|_| PkiError::certificate_error(
                "Invalid hour in UTC time",
                CertificateErrorCode::MalformedCertificate
            ))?;
        let minute: u32 = time_str[8..10].parse()
            .map_err(|_| PkiError::certificate_error(
                "Invalid minute in UTC time",
                CertificateErrorCode::MalformedCertificate
            ))?;
        let second: u32 = time_str[10..12].parse()
            .map_err(|_| PkiError::certificate_error(
                "Invalid second in UTC time",
                CertificateErrorCode::MalformedCertificate
            ))?;
        
        // Convert to Unix timestamp (simplified calculation)
        let days_since_epoch = self.days_since_unix_epoch(year, month, day)?;
        let seconds_since_epoch = (days_since_epoch as u64) * 86400 + 
                                 (hour as u64) * 3600 + 
                                 (minute as u64) * 60 + 
                                 (second as u64);
        
        Ok(UNIX_EPOCH + std::time::Duration::from_secs(seconds_since_epoch))
    }
    
    /// Parse generalized time format (YYYYMMDDHHMMSSZ)
    fn parse_generalized_time(&self, time_str: &str) -> PkiResult<SystemTime> {
        if time_str.len() != 15 || !time_str.ends_with('Z') {
            return Err(PkiError::certificate_error(
                format!("Invalid generalized time format: {}", time_str),
                CertificateErrorCode::MalformedCertificate
            ));
        }
        
        let year: u32 = time_str[0..4].parse()
            .map_err(|_| PkiError::certificate_error(
                "Invalid year in generalized time",
                CertificateErrorCode::MalformedCertificate
            ))?;
        let month: u32 = time_str[4..6].parse()
            .map_err(|_| PkiError::certificate_error(
                "Invalid month in generalized time",
                CertificateErrorCode::MalformedCertificate
            ))?;
        let day: u32 = time_str[6..8].parse()
            .map_err(|_| PkiError::certificate_error(
                "Invalid day in generalized time",
                CertificateErrorCode::MalformedCertificate
            ))?;
        let hour: u32 = time_str[8..10].parse()
            .map_err(|_| PkiError::certificate_error(
                "Invalid hour in generalized time",
                CertificateErrorCode::MalformedCertificate
            ))?;
        let minute: u32 = time_str[10..12].parse()
            .map_err(|_| PkiError::certificate_error(
                "Invalid minute in generalized time",
                CertificateErrorCode::MalformedCertificate
            ))?;
        let second: u32 = time_str[12..14].parse()
            .map_err(|_| PkiError::certificate_error(
                "Invalid second in generalized time",
                CertificateErrorCode::MalformedCertificate
            ))?;
        
        // Convert to Unix timestamp
        let days_since_epoch = self.days_since_unix_epoch(year, month, day)?;
        let seconds_since_epoch = (days_since_epoch as u64) * 86400 + 
                                 (hour as u64) * 3600 + 
                                 (minute as u64) * 60 + 
                                 (second as u64);
        
        Ok(UNIX_EPOCH + std::time::Duration::from_secs(seconds_since_epoch))
    }
    
    /// Calculate days since Unix epoch
    fn days_since_unix_epoch(&self, year: u32, month: u32, day: u32) -> PkiResult<i32> {
        // Simplified calendar calculation
        if month < 1 || month > 12 || day < 1 || day > 31 {
            return Err(PkiError::certificate_error(
                "Invalid date",
                CertificateErrorCode::MalformedCertificate
            ));
        }
        
        let mut days = 0i32;
        
        // Add days for complete years since 1970
        for y in 1970..year {
            days += if self.is_leap_year(y) { 366 } else { 365 };
        }
        
        // Add days for complete months in current year
        let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for m in 1..month {
            days += days_in_month[(m - 1) as usize];
            if m == 2 && self.is_leap_year(year) {
                days += 1; // February in leap year
            }
        }
        
        // Add days in current month
        days += (day - 1) as i32;
        
        Ok(days)
    }
    
    /// Check if year is a leap year
    fn is_leap_year(&self, year: u32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
    
    /// Parse subject public key info
    fn parse_subject_public_key_info(&self, context: &mut Asn1Context) -> PkiResult<SubjectPublicKeyInfo> {
        self.expect_tag(context, Asn1Tag::Sequence)?;
        let _seq_length = self.parse_length(context)?;
        
        // Parse algorithm
        self.expect_tag(context, Asn1Tag::Sequence)?;
        let _algo_length = self.parse_length(context)?;
        
        self.expect_tag(context, Asn1Tag::ObjectIdentifier)?;
        let oid_length = self.parse_length(context)?;
        let oid_bytes = self.read_bytes(context, oid_length)?;
        let oid = self.decode_oid(&oid_bytes)?;
        
        // Parse parameters (optional)
        let parameters = if self.peek_tag(context) == Some(Asn1Tag::Null as u8) {
            self.read_byte(context)?; // consume NULL
            let _null_length = self.parse_length(context)?;
            None
        } else if self.peek_tag(context) == Some(Asn1Tag::Sequence as u8) {
            // EC parameters
            let param_start = context.position;
            self.expect_tag(context, Asn1Tag::Sequence)?;
            let param_length = self.parse_length(context)?;
            let param_data = self.read_bytes(context, param_length)?;
            context.position = param_start;
            Some(param_data)
        } else {
            None
        };
        
        // Parse public key
        self.expect_tag(context, Asn1Tag::BitString)?;
        let key_length = self.parse_length(context)?;
        let _unused_bits = self.read_byte(context)?;
        let public_key = self.read_bytes(context, key_length - 1)?;
        
        let algorithm = self.oid_to_public_key_algorithm(&oid)?;
        
        Ok(SubjectPublicKeyInfo {
            algorithm,
            public_key,
            parameters,
        })
    }
    
    /// Parse certificate extensions
    fn parse_extensions(&self, context: &mut Asn1Context) -> PkiResult<Vec<X509Extension>> {
        self.expect_tag(context, Asn1Tag::Sequence)?;
        let _seq_length = self.parse_length(context)?;
        
        let mut extensions = Vec::new();
        
        while context.position < context.data.len() && 
              self.peek_tag(context) == Some(Asn1Tag::Sequence as u8) {
            
            self.expect_tag(context, Asn1Tag::Sequence)?;
            let _ext_length = self.parse_length(context)?;
            
            // Parse extension OID
            self.expect_tag(context, Asn1Tag::ObjectIdentifier)?;
            let oid_length = self.parse_length(context)?;
            let oid_bytes = self.read_bytes(context, oid_length)?;
            let oid = self.decode_oid(&oid_bytes)?;
            
            // Parse critical flag (optional)
            let critical = if self.peek_tag(context) == Some(Asn1Tag::Boolean as u8) {
                self.expect_tag(context, Asn1Tag::Boolean)?;
                let _bool_length = self.parse_length(context)?;
                let critical_byte = self.read_byte(context)?;
                critical_byte != 0
            } else {
                false
            };
            
            // Parse extension value
            self.expect_tag(context, Asn1Tag::OctetString)?;
            let value_length = self.parse_length(context)?;
            let value = self.read_bytes(context, value_length)?;
            
            // Parse extension data if supported
            let parsed_data = if self.config.parse_extensions {
                if let Some(parser) = self.supported_extensions.get(&oid) {
                    parser(&value).ok()
                } else {
                    None
                }
            } else {
                None
            };
            
            extensions.push(X509Extension {
                oid,
                critical,
                value,
                parsed_data,
            });
        }
        
        Ok(extensions)
    }
    
    /// Extract key usage information from certificate extensions
    fn extract_key_usage(&self, cert: &mut X509Certificate) {
        for extension in &cert.extensions {
            match &extension.parsed_data {
                Some(ExtensionData::KeyUsage(usage)) => {
                    cert.key_usage = usage.clone();
                }
                Some(ExtensionData::ExtendedKeyUsage(ext_usage)) => {
                    cert.extended_key_usage = ext_usage.clone();
                }
                _ => {}
            }
        }
    }
    
    /// Validate certificate signature (simplified implementation)
    fn validate_certificate_signature(&self, _cert: &X509Certificate) -> PkiResult<()> {
        // TODO: Implement actual signature validation
        // This would require:
        // 1. Extract the signature algorithm
        // 2. Get the issuer's public key
        // 3. Verify the signature over the TBSCertificate
        
        // For now, just return Ok as a placeholder
        Ok(())
    }
    
    /// Calculate SHA-256 hash
    fn calculate_sha256(&self, data: &[u8]) -> Vec<u8> {
        // Simplified SHA-256 implementation
        // In a real implementation, use a proper crypto library
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let hash = hasher.finish();
        hash.to_be_bytes().to_vec()
    }
    
    /// Register default extension parsers
    fn register_default_extensions(&mut self) {
        // Basic Constraints
        self.supported_extensions.insert(
            "2.5.29.19".to_string(),
            Self::parse_basic_constraints
        );
        
        // Key Usage
        self.supported_extensions.insert(
            "2.5.29.15".to_string(),
            Self::parse_key_usage
        );
        
        // Extended Key Usage
        self.supported_extensions.insert(
            "2.5.29.37".to_string(),
            Self::parse_extended_key_usage
        );
        
        // Subject Alternative Name
        self.supported_extensions.insert(
            "2.5.29.17".to_string(),
            Self::parse_subject_alternative_name
        );
    }
    
    /// Parse Basic Constraints extension
    fn parse_basic_constraints(data: &[u8]) -> PkiResult<ExtensionData> {
        // Simplified parser - would need full ASN.1 parsing
        let is_ca = data.len() > 0 && data[0] != 0;
        let path_length_constraint = None; // Would parse from ASN.1
        
        Ok(ExtensionData::BasicConstraints {
            is_ca,
            path_length_constraint,
        })
    }
    
    /// Parse Key Usage extension
    fn parse_key_usage(data: &[u8]) -> PkiResult<ExtensionData> {
        if data.is_empty() {
            return Ok(ExtensionData::KeyUsage(KeyUsage::default()));
        }
        
        // Parse bit string
        let mut usage = KeyUsage::default();
        
        if data.len() >= 3 {
            let bits = data[2]; // First byte of actual bit data
            usage.digital_signature = (bits & 0x80) != 0;
            usage.non_repudiation = (bits & 0x40) != 0;
            usage.key_encipherment = (bits & 0x20) != 0;
            usage.data_encipherment = (bits & 0x10) != 0;
            usage.key_agreement = (bits & 0x08) != 0;
            usage.key_cert_sign = (bits & 0x04) != 0;
            usage.crl_sign = (bits & 0x02) != 0;
            usage.encipher_only = (bits & 0x01) != 0;
        }
        
        if data.len() >= 4 {
            let bits2 = data[3];
            usage.decipher_only = (bits2 & 0x80) != 0;
        }
        
        Ok(ExtensionData::KeyUsage(usage))
    }
    
    /// Parse Extended Key Usage extension
    fn parse_extended_key_usage(_data: &[u8]) -> PkiResult<ExtensionData> {
        // Simplified implementation
        Ok(ExtensionData::ExtendedKeyUsage(ExtendedKeyUsage::default()))
    }
    
    /// Parse Subject Alternative Name extension
    fn parse_subject_alternative_name(_data: &[u8]) -> PkiResult<ExtensionData> {
        // Simplified implementation
        Ok(ExtensionData::SubjectAlternativeName(Vec::new()))
    }
    
    /// Map OID to signature algorithm
    fn oid_to_signature_algorithm(&self, oid: &str) -> PkiResult<SignatureAlgorithm> {
        match oid {
            "1.2.840.113549.1.1.11" => Ok(SignatureAlgorithm::RsaWithSha256),
            "1.2.840.113549.1.1.12" => Ok(SignatureAlgorithm::RsaWithSha384),
            "1.2.840.113549.1.1.13" => Ok(SignatureAlgorithm::RsaWithSha512),
            "1.2.840.10045.4.3.2" => Ok(SignatureAlgorithm::EcdsaWithSha256),
            "1.2.840.10045.4.3.3" => Ok(SignatureAlgorithm::EcdsaWithSha384),
            "1.2.840.10045.4.3.4" => Ok(SignatureAlgorithm::EcdsaWithSha512),
            "1.3.101.112" => Ok(SignatureAlgorithm::Ed25519),
            "1.3.101.113" => Ok(SignatureAlgorithm::Ed448),
            _ => Ok(SignatureAlgorithm::Custom {
                oid: oid.to_string(),
                name: format!("Unknown({})", oid),
            }),
        }
    }
    
    /// Map OID to public key algorithm
    fn oid_to_public_key_algorithm(&self, oid: &str) -> PkiResult<PublicKeyAlgorithm> {
        match oid {
            "1.2.840.113549.1.1.1" => Ok(PublicKeyAlgorithm::Rsa { key_size: 2048 }), // Default size
            "1.2.840.10045.2.1" => Ok(PublicKeyAlgorithm::EllipticCurve { curve: EllipticCurve::P256 }),
            "1.3.101.112" => Ok(PublicKeyAlgorithm::Ed25519),
            "1.3.101.113" => Ok(PublicKeyAlgorithm::Ed448),
            _ => Ok(PublicKeyAlgorithm::Custom {
                oid: oid.to_string(),
                name: format!("Unknown({})", oid),
            }),
        }
    }
    
    /// ASN.1 parsing helper functions
    fn expect_tag(&self, context: &mut Asn1Context, expected: Asn1Tag) -> PkiResult<()> {
        let tag = self.read_byte(context)?;
        if tag != expected as u8 {
            return Err(PkiError::certificate_error(
                format!("Expected tag 0x{:02x}, got 0x{:02x}", expected as u8, tag),
                CertificateErrorCode::MalformedCertificate
            ));
        }
        Ok(())
    }
    
    fn peek_tag(&self, context: &Asn1Context) -> Option<u8> {
        if context.position < context.data.len() {
            Some(context.data[context.position])
        } else {
            None
        }
    }
    
    fn read_byte(&self, context: &mut Asn1Context) -> PkiResult<u8> {
        if context.position >= context.data.len() {
            return Err(PkiError::certificate_error(
                "Unexpected end of data",
                CertificateErrorCode::MalformedCertificate
            ));
        }
        let byte = context.data[context.position];
        context.position += 1;
        Ok(byte)
    }
    
    fn read_bytes(&self, context: &mut Asn1Context, count: usize) -> PkiResult<Vec<u8>> {
        if context.position + count > context.data.len() {
            return Err(PkiError::certificate_error(
                "Unexpected end of data",
                CertificateErrorCode::MalformedCertificate
            ));
        }
        let bytes = context.data[context.position..context.position + count].to_vec();
        context.position += count;
        Ok(bytes)
    }
    
    fn parse_length(&self, context: &mut Asn1Context) -> PkiResult<usize> {
        let first_byte = self.read_byte(context)?;
        
        if first_byte & 0x80 == 0 {
            // Short form
            Ok(first_byte as usize)
        } else {
            // Long form
            let length_bytes = (first_byte & 0x7F) as usize;
            if length_bytes == 0 || length_bytes > 4 {
                return Err(PkiError::certificate_error(
                    "Invalid length encoding",
                    CertificateErrorCode::MalformedCertificate
                ));
            }
            
            let mut length = 0usize;
            for _ in 0..length_bytes {
                length = (length << 8) | (self.read_byte(context)? as usize);
            }
            Ok(length)
        }
    }
    
    fn decode_oid(&self, bytes: &[u8]) -> PkiResult<String> {
        if bytes.is_empty() {
            return Err(PkiError::certificate_error(
                "Empty OID",
                CertificateErrorCode::MalformedCertificate
            ));
        }
        
        let mut result = Vec::new();
        
        // First byte encodes first two components
        let first_byte = bytes[0];
        result.push((first_byte / 40).to_string());
        result.push((first_byte % 40).to_string());
        
        // Decode remaining components
        let mut i = 1;
        while i < bytes.len() {
            let mut value = 0u64;
            
            loop {
                if i >= bytes.len() {
                    return Err(PkiError::certificate_error(
                        "Truncated OID",
                        CertificateErrorCode::MalformedCertificate
                    ));
                }
                
                let byte = bytes[i];
                i += 1;
                
                value = (value << 7) | ((byte & 0x7F) as u64);
                
                if (byte & 0x80) == 0 {
                    break;
                }
            }
            
            result.push(value.to_string());
        }
        
        Ok(result.join("."))
    }
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            parse_extensions: true,
            validate_signature: false,
            max_certificate_size: 1024 * 1024, // 1MB
            utf8_mode: true,
        }
    }
}

/// Hex encoding utility
mod hex {
    pub fn encode(data: &[u8]) -> String {
        data.iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}
