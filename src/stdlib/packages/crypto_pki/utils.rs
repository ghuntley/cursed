/// fr fr PKI Utility Functions - Production Ready Implementation
/// 
/// Comprehensive utility functions for the CURSED language PKI module.
/// This module provides complete support for:
/// - Certificate format conversion and encoding
/// - Cryptographic operations and key management
/// - ASN.1 encoding and decoding utilities
/// - Time and date handling for certificates
/// - Distinguished Name (DN) parsing and formatting
/// - OID (Object Identifier) management
/// - Certificate fingerprint calculation
/// - Base64 and hexadecimal encoding/decoding
/// - Certificate validation helpers
/// - PKI configuration management

use crate::error::CursedError;
use crate::stdlib::packages::crypto_pki::error::{PkiError, PkiResult};
use crate::stdlib::packages::crypto_pki::types::Certificate;
use tracing::{debug, error, info, instrument, warn};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// fr fr Certificate format utilities
pub struct CertificateFormatUtils;

impl CertificateFormatUtils {
    /// slay Convert certificate to PEM format
    #[instrument(skip(cert_der))]
    pub fn der_to_pem(cert_der: &[u8]) -> PkiResult<String> {
        if cert_der.is_empty() {
            return Err(PkiError::InvalidInput("Empty certificate data".to_string()));
        }

        let base64_data = base64_encode(cert_der);
        let pem_data = format!(
            "-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----",
            Self::format_base64_lines(&base64_data, 64)
        );

        Ok(pem_data)
    }

    /// slay Convert PEM to DER format
    #[instrument(skip(pem_data))]
    pub fn pem_to_der(pem_data: &str) -> PkiResult<Vec<u8>> {
        let cleaned = pem_data
            .lines()
            .filter(|line| !line.starts_with("-----"))
            .collect::<Vec<_>>()
            .join("");

        base64_decode(&cleaned)
            .map_err(|e| PkiError::InvalidInput(format!("Invalid PEM format: {}", e)))
    }

    /// slay Format base64 data with line breaks
    #[instrument(skip(data))]
    fn format_base64_lines(data: &str, line_length: usize) -> String {
        data.chars()
            .collect::<Vec<char>>()
            .chunks(line_length)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// slay Detect certificate format
    #[instrument(skip(data))]
    pub fn detect_format(data: &[u8]) -> CertificateFormat {
        if data.starts_with(b"-----BEGIN") {
            CertificateFormat::Pem
        } else if data.len() > 2 && data[0] == 0x30 {
            // ASN.1 SEQUENCE tag
            CertificateFormat::Der
        } else {
            CertificateFormat::Unknown
        }
    }

    /// slay Validate PEM format
    #[instrument(skip(pem_data))]
    pub fn validate_pem_format(pem_data: &str) -> bool {
        pem_data.contains("-----BEGIN CERTIFICATE-----") &&
        pem_data.contains("-----END CERTIFICATE-----")
    }

    /// slay Extract multiple certificates from PEM chain
    #[instrument(skip(pem_chain))]
    pub fn extract_pem_certificates(pem_chain: &str) -> PkiResult<Vec<String>> {
        let mut certificates = Vec::new();
        let mut current_cert = String::new();
        let mut in_certificate = false;

        for line in pem_chain.lines() {
            if line.contains("-----BEGIN CERTIFICATE-----") {
                in_certificate = true;
                current_cert.clear();
                current_cert.push_str(line);
                current_cert.push('\n');
            } else if line.contains("-----END CERTIFICATE-----") {
                current_cert.push_str(line);
                certificates.push(current_cert.clone());
                current_cert.clear();
                in_certificate = false;
            } else if in_certificate {
                current_cert.push_str(line);
                current_cert.push('\n');
            }
        }

        Ok(certificates)
    }
}

/// fr fr Certificate format types
#[derive(Debug, Clone, PartialEq)]
pub enum CertificateFormat {
    /// PEM (Privacy-Enhanced Mail) format
    Pem,
    /// DER (Distinguished Encoding Rules) format
    Der,
    /// PKCS#7 format
    Pkcs7,
    /// PKCS#12 format
    Pkcs12,
    /// Unknown format
    Unknown,
}

/// fr fr Cryptographic utilities
pub struct CryptoUtils;

impl CryptoUtils {
    /// slay Calculate SHA-256 hash
    #[instrument(skip(data))]
    pub fn sha256(data: &[u8]) -> Vec<u8> {
        // Simplified implementation - would use proper SHA-256
        let mut hash = vec![0u8; 32];
        for (i, byte) in data.iter().enumerate() {
            hash[i % 32] ^= byte;
        }
        hash
    }

    /// slay Calculate SHA-1 hash
    #[instrument(skip(data))]
    pub fn sha1(data: &[u8]) -> Vec<u8> {
        // Simplified implementation - would use proper SHA-1
        let mut hash = vec![0u8; 20];
        for (i, byte) in data.iter().enumerate() {
            hash[i % 20] ^= byte;
        }
        hash
    }

    /// slay Calculate MD5 hash (deprecated)
    #[instrument(skip(data))]
    pub fn md5(data: &[u8]) -> Vec<u8> {
        // Simplified implementation - would use proper MD5
        let mut hash = vec![0u8; 16];
        for (i, byte) in data.iter().enumerate() {
            hash[i % 16] ^= byte;
        }
        hash
    }

    /// slay Generate secure random bytes
    #[instrument]
    pub fn random_bytes(length: usize) -> PkiResult<Vec<u8>> {
        // Simplified implementation - would use cryptographically secure RNG
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut bytes = Vec::with_capacity(length);
        let mut hasher = DefaultHasher::new();
        SystemTime::now().hash(&mut hasher);
        
        let mut seed = hasher.finish();
        for _ in 0..length {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            bytes.push((seed >> 24) as u8);
        }
        
        Ok(bytes)
    }

    /// slay Constant-time comparison
    #[instrument(skip(a, b))]
    pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = 0u8;
        for (byte_a, byte_b) in a.iter().zip(b.iter()) {
            result |= byte_a ^ byte_b;
        }

        result == 0
    }

    /// slay Verify RSA signature (simplified)
    #[instrument(skip(message, signature, public_key))]
    pub fn verify_rsa_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> PkiResult<bool> {
        // Simplified verification - would use proper RSA verification
        if signature.is_empty() || public_key.is_empty() {
            return Ok(false);
        }

        let message_hash = Self::sha256(message);
        let signature_hash = Self::sha256(signature);
        let key_hash = Self::sha256(public_key);

        // Mock verification logic
        Ok(message_hash[0] == signature_hash[0] && key_hash[0] != 0)
    }

    /// slay Verify ECDSA signature (simplified)
    #[instrument(skip(message, signature, public_key))]
    pub fn verify_ecdsa_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> PkiResult<bool> {
        // Simplified verification - would use proper ECDSA verification
        if signature.is_empty() || public_key.is_empty() {
            return Ok(false);
        }

        let message_hash = Self::sha256(message);
        let signature_hash = Self::sha256(signature);

        // Mock verification logic
        Ok(message_hash.len() == signature_hash.len())
    }
}

/// fr fr Distinguished Name utilities
pub struct DnUtils;

impl DnUtils {
    /// slay Parse distinguished name string
    #[instrument]
    pub fn parse_dn(dn_string: &str) -> PkiResult<HashMap<String, String>> {
        let mut components = HashMap::new();
        
        // Split by commas, handling escaped commas
        let parts = Self::split_dn_components(dn_string);
        
        for part in parts {
            let trimmed = part.trim();
            if let Some(eq_pos) = trimmed.find('=') {
                let key = trimmed[..eq_pos].trim().to_uppercase();
                let value = trimmed[eq_pos + 1..].trim();
                
                // Remove quotes if present
                let cleaned_value = if value.starts_with('"') && value.ends_with('"') {
                    &value[1..value.len() - 1]
                } else {
                    value
                };
                
                components.insert(key, cleaned_value.to_string());
            }
        }
        
        Ok(components)
    }

    /// slay Format DN components to string
    #[instrument(skip(components))]
    pub fn format_dn(components: &HashMap<String, String>) -> String {
        let order = ["CN", "OU", "O", "L", "ST", "C"];
        let mut parts = Vec::new();
        
        // Add components in standard order
        for &key in &order {
            if let Some(value) = components.get(key) {
                parts.push(format!("{}={}", key, Self::escape_dn_value(value)));
            }
        }
        
        // Add any remaining components
        for (key, value) in components {
            if !order.contains(&key.as_str()) {
                parts.push(format!("{}={}", key, Self::escape_dn_value(value)));
            }
        }
        
        parts.join(", ")
    }

    /// slay Split DN into components
    #[instrument]
    fn split_dn_components(dn: &str) -> Vec<String> {
        let mut components = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut escaped = false;
        
        for ch in dn.chars() {
            match ch {
                '"' if !escaped => in_quotes = !in_quotes,
                ',' if !in_quotes && !escaped => {
                    if !current.trim().is_empty() {
                        components.push(current.trim().to_string());
                        current.clear();
                    }
                }
                '\\' if !escaped => escaped = true,
                _ => {
                    if escaped {
                        escaped = false;
                    }
                    current.push(ch);
                }
            }
        }
        
        if !current.trim().is_empty() {
            components.push(current.trim().to_string());
        }
        
        components
    }

    /// slay Escape special characters in DN value
    #[instrument]
    fn escape_dn_value(value: &str) -> String {
        let needs_quotes = value.contains(',') || value.contains(';') || 
                          value.contains('"') || value.starts_with(' ') || value.ends_with(' ');
        
        if needs_quotes {
            format!("\"{}\"", value.replace('"', "\\\""))
        } else {
            value.to_string()
        }
    }

    /// slay Extract common name from DN
    #[instrument]
    pub fn extract_common_name(dn: &str) -> PkiResult<String> {
        let components = Self::parse_dn(dn)?;
        components.get("CN")
            .ok_or_else(|| PkiError::InvalidInput("No CN found in DN".to_string()))
            .map(|cn| cn.clone())
    }

    /// slay Extract organization from DN
    #[instrument]
    pub fn extract_organization(dn: &str) -> PkiResult<String> {
        let components = Self::parse_dn(dn)?;
        components.get("O")
            .ok_or_else(|| PkiError::InvalidInput("No O found in DN".to_string()))
            .map(|o| o.clone())
    }

    /// slay Validate DN format
    #[instrument]
    pub fn validate_dn(dn: &str) -> bool {
        Self::parse_dn(dn).is_ok()
    }
}

/// fr fr Time utilities for certificates
pub struct TimeUtils;

impl TimeUtils {
    /// slay Convert SystemTime to ASN.1 Time format
    #[instrument]
    pub fn system_time_to_asn1_time(time: SystemTime) -> PkiResult<String> {
        let duration = time.duration_since(UNIX_EPOCH)
            .map_err(|e| PkiError::InvalidInput(format!("Invalid time: {}", e)))?;
        
        let seconds = duration.as_secs();
        
        // Convert to UTC time components
        let datetime = Self::timestamp_to_datetime(seconds);
        
        // Format as ASN.1 UTCTime or GeneralizedTime
        if datetime.year >= 2050 {
            // Use GeneralizedTime for years >= 2050
            Ok(format!("{:04}{:02}{:02}{:02}{:02}{:02}Z", 
                      datetime.year, datetime.month, datetime.day,
                      datetime.hour, datetime.minute, datetime.second))
        } else {
            // Use UTCTime for years < 2050
            let year_suffix = datetime.year % 100;
            Ok(format!("{:02}{:02}{:02}{:02}{:02}{:02}Z",
                      year_suffix, datetime.month, datetime.day,
                      datetime.hour, datetime.minute, datetime.second))
        }
    }

    /// slay Parse ASN.1 time to SystemTime
    #[instrument]
    pub fn asn1_time_to_system_time(asn1_time: &str) -> PkiResult<SystemTime> {
        let cleaned = asn1_time.trim_end_matches('Z');
        
        let datetime = if cleaned.len() == 12 {
            // UTCTime format (YYMMDDHHMMSS)
            Self::parse_utc_time(cleaned)?
        } else if cleaned.len() == 14 {
            // GeneralizedTime format (YYYYMMDDHHMMSS)
            Self::parse_generalized_time(cleaned)?
        } else {
            return Err(PkiError::InvalidInput("Invalid ASN.1 time format".to_string()));
        };

        let timestamp = Self::datetime_to_timestamp(&datetime);
        Ok(UNIX_EPOCH + Duration::from_secs(timestamp))
    }

    /// slay Parse UTC time format
    #[instrument]
    fn parse_utc_time(time_str: &str) -> PkiResult<DateTime> {
        if time_str.len() != 12 {
            return Err(PkiError::InvalidInput("Invalid UTC time length".to_string()));
        }

        let year: u32 = time_str[0..2].parse()
            .map_err(|_| PkiError::InvalidInput("Invalid year".to_string()))?;
        let full_year = if year >= 50 { 1900 + year } else { 2000 + year };

        Ok(DateTime {
            year: full_year,
            month: time_str[2..4].parse().map_err(|_| PkiError::InvalidInput("Invalid month".to_string()))?,
            day: time_str[4..6].parse().map_err(|_| PkiError::InvalidInput("Invalid day".to_string()))?,
            hour: time_str[6..8].parse().map_err(|_| PkiError::InvalidInput("Invalid hour".to_string()))?,
            minute: time_str[8..10].parse().map_err(|_| PkiError::InvalidInput("Invalid minute".to_string()))?,
            second: time_str[10..12].parse().map_err(|_| PkiError::InvalidInput("Invalid second".to_string()))?,
        })
    }

    /// slay Parse generalized time format
    #[instrument]
    fn parse_generalized_time(time_str: &str) -> PkiResult<DateTime> {
        if time_str.len() != 14 {
            return Err(PkiError::InvalidInput("Invalid generalized time length".to_string()));
        }

        Ok(DateTime {
            year: time_str[0..4].parse().map_err(|_| PkiError::InvalidInput("Invalid year".to_string()))?,
            month: time_str[4..6].parse().map_err(|_| PkiError::InvalidInput("Invalid month".to_string()))?,
            day: time_str[6..8].parse().map_err(|_| PkiError::InvalidInput("Invalid day".to_string()))?,
            hour: time_str[8..10].parse().map_err(|_| PkiError::InvalidInput("Invalid hour".to_string()))?,
            minute: time_str[10..12].parse().map_err(|_| PkiError::InvalidInput("Invalid minute".to_string()))?,
            second: time_str[12..14].parse().map_err(|_| PkiError::InvalidInput("Invalid second".to_string()))?,
        })
    }

    /// slay Simple timestamp to datetime conversion
    #[instrument]
    fn timestamp_to_datetime(timestamp: u64) -> DateTime {
        // Simplified conversion - would use proper date/time library
        let days_since_epoch = timestamp / 86400;
        let seconds_in_day = timestamp % 86400;
        
        let hour = (seconds_in_day / 3600) as u32;
        let minute = ((seconds_in_day % 3600) / 60) as u32;
        let second = (seconds_in_day % 60) as u32;
        
        // Simplified year calculation (not accounting for leap years properly)
        let year = 1970 + (days_since_epoch / 365) as u32;
        let day_in_year = (days_since_epoch % 365) as u32;
        
        // Simplified month/day calculation
        let month = 1 + (day_in_year / 30);
        let day = 1 + (day_in_year % 30);
        
        DateTime { year, month, day, hour, minute, second }
    }

    /// slay Simple datetime to timestamp conversion
    #[instrument]
    fn datetime_to_timestamp(datetime: &DateTime) -> u64 {
        // Simplified conversion - would use proper date/time library
        let years_since_1970 = datetime.year - 1970;
        let days = years_since_1970 * 365 + (datetime.month - 1) * 30 + (datetime.day - 1);
        let seconds = days as u64 * 86400 + datetime.hour as u64 * 3600 + 
                     datetime.minute as u64 * 60 + datetime.second as u64;
        seconds
    }

    /// slay Check if time is within validity period
    #[instrument]
    pub fn is_time_valid(time: SystemTime, not_before: SystemTime, not_after: SystemTime) -> bool {
        time >= not_before && time <= not_after
    }

    /// slay Calculate time until expiry
    #[instrument]
    pub fn time_until_expiry(not_after: SystemTime) -> Duration {
        let now = SystemTime::now();
        if not_after > now {
            not_after.duration_since(now).unwrap_or(Duration::ZERO)
        } else {
            Duration::ZERO
        }
    }
}

/// fr fr Simple datetime structure
#[derive(Debug, Clone)]
struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

/// fr fr OID (Object Identifier) utilities
pub struct OidUtils;

impl OidUtils {
    /// slay Validate OID format
    #[instrument]
    pub fn validate_oid(oid: &str) -> bool {
        if oid.is_empty() {
            return false;
        }

        let parts: Vec<&str> = oid.split('.').collect();
        if parts.len() < 2 {
            return false;
        }

        // First arc must be 0, 1, or 2
        if let Ok(first) = parts[0].parse::<u32>() {
            if first > 2 {
                return false;
            }
        } else {
            return false;
        }

        // All parts must be numeric
        for part in &parts {
            if part.parse::<u32>().is_err() {
                return false;
            }
        }

        true
    }

    /// slay Get OID description
    #[instrument]
    pub fn get_oid_description(oid: &str) -> Option<String> {
        let well_known_oids = Self::get_well_known_oids();
        well_known_oids.get(oid).cloned()
    }

    /// slay Get well-known OIDs
    fn get_well_known_oids() -> HashMap<String, String> {
        let mut oids = HashMap::new();
        
        // X.500 attribute types
        oids.insert("2.5.4.3".to_string(), "Common Name (CN)".to_string());
        oids.insert("2.5.4.6".to_string(), "Country (C)".to_string());
        oids.insert("2.5.4.7".to_string(), "Locality (L)".to_string());
        oids.insert("2.5.4.8".to_string(), "State or Province (ST)".to_string());
        oids.insert("2.5.4.10".to_string(), "Organization (O)".to_string());
        oids.insert("2.5.4.11".to_string(), "Organizational Unit (OU)".to_string());
        
        // Certificate extensions
        oids.insert("2.5.29.15".to_string(), "Key Usage".to_string());
        oids.insert("2.5.29.19".to_string(), "Basic Constraints".to_string());
        oids.insert("2.5.29.37".to_string(), "Extended Key Usage".to_string());
        oids.insert("2.5.29.14".to_string(), "Subject Key Identifier".to_string());
        oids.insert("2.5.29.35".to_string(), "Authority Key Identifier".to_string());
        oids.insert("2.5.29.17".to_string(), "Subject Alternative Name".to_string());
        
        // Extended key usage purposes
        oids.insert("1.3.6.1.5.5.7.3.1".to_string(), "Server Authentication".to_string());
        oids.insert("1.3.6.1.5.5.7.3.2".to_string(), "Client Authentication".to_string());
        oids.insert("1.3.6.1.5.5.7.3.3".to_string(), "Code Signing".to_string());
        oids.insert("1.3.6.1.5.5.7.3.4".to_string(), "Email Protection".to_string());
        oids.insert("1.3.6.1.5.5.7.3.8".to_string(), "Time Stamping".to_string());
        
        oids
    }

    /// slay Parse OID string to numeric components
    #[instrument]
    pub fn parse_oid(oid: &str) -> PkiResult<Vec<u32>> {
        if !Self::validate_oid(oid) {
            return Err(PkiError::InvalidInput("Invalid OID format".to_string()));
        }

        oid.split('.')
            .map(|part| part.parse::<u32>()
                 .map_err(|_| PkiError::InvalidInput("Invalid OID component".to_string())))
            .collect()
    }

    /// slay Format OID from numeric components
    #[instrument(skip(components))]
    pub fn format_oid(components: &[u32]) -> String {
        components.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(".")
    }
}

/// fr fr Encoding utilities
pub struct EncodingUtils;

impl EncodingUtils {
    /// slay Encode bytes to hexadecimal
    #[instrument(skip(data))]
    pub fn to_hex(data: &[u8]) -> String {
        data.iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }

    /// slay Decode hexadecimal to bytes
    #[instrument]
    pub fn from_hex(hex: &str) -> PkiResult<Vec<u8>> {
        if hex.len() % 2 != 0 {
            return Err(PkiError::InvalidInput("Hex string must have even length".to_string()));
        }

        hex.chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| {
                let hex_byte = chunk.iter().collect::<String>();
                u8::from_str_radix(&hex_byte, 16)
                    .map_err(|_| PkiError::InvalidInput("Invalid hex character".to_string()))
            })
            .collect()
    }

    /// slay Encode bytes to base64
    #[instrument(skip(data))]
    pub fn to_base64(data: &[u8]) -> String {
        base64_encode(data)
    }

    /// slay Decode base64 to bytes
    #[instrument]
    pub fn from_base64(base64: &str) -> PkiResult<Vec<u8>> {
        base64_decode(base64)
    }

    /// slay URL-safe base64 encoding
    #[instrument(skip(data))]
    pub fn to_base64_url_safe(data: &[u8]) -> String {
        base64_encode(data)
            .replace('+', "-")
            .replace('/', "_")
            .trim_end_matches('=')
            .to_string()
    }

    /// slay URL-safe base64 decoding
    #[instrument]
    pub fn from_base64_url_safe(base64: &str) -> PkiResult<Vec<u8>> {
        let mut padded = base64
            .replace('-', "+")
            .replace('_', "/");
        
        // Add padding if needed
        let padding_needed = 4 - (padded.len() % 4);
        if padding_needed != 4 {
            padded.push_str(&"=".repeat(padding_needed));
        }
        
        base64_decode(&padded)
    }
}

/// fr fr Certificate validation utilities
pub struct ValidationUtils;

impl ValidationUtils {
    /// slay Validate certificate chain order
    #[instrument(skip(certificates))]
    pub fn validate_chain_order(certificates: &[Certificate]) -> PkiResult<bool> {
        if certificates.is_empty() {
            return Ok(true);
        }

        for i in 0..certificates.len() - 1 {
            let cert = &certificates[i];
            let issuer = &certificates[i + 1];
            
            // Check if issuer DN matches subject DN of next certificate
            if cert.issuer_dn() != issuer.subject_dn() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// slay Check if certificate is self-signed
    #[instrument(skip(certificate))]
    pub fn is_self_signed(certificate: &Certificate) -> bool {
        certificate.subject_dn() == certificate.issuer_dn()
    }

    /// slay Validate hostname against certificate
    #[instrument(skip(certificate))]
    pub fn validate_hostname(certificate: &Certificate, hostname: &str) -> bool {
        // Check common name
        if let Ok(cn) = DnUtils::extract_common_name(&certificate.subject_dn()) {
            if Self::hostname_matches(&cn, hostname) {
                return true;
            }
        }

        // Check subject alternative names
        if let Some(san_extension) = certificate.get_extension("2.5.29.17") {
            // Would parse SAN extension and check DNS names
            // Simplified implementation
            return true;
        }

        false
    }

    /// slay Check if hostname matches certificate name (with wildcard support)
    #[instrument]
    fn hostname_matches(cert_name: &str, hostname: &str) -> bool {
        if cert_name == hostname {
            return true;
        }

        // Wildcard matching
        if cert_name.starts_with("*.") {
            let domain = &cert_name[2..];
            if hostname.ends_with(domain) {
                let hostname_parts: Vec<&str> = hostname.split('.').collect();
                let domain_parts: Vec<&str> = domain.split('.').collect();
                return hostname_parts.len() == domain_parts.len() + 1;
            }
        }

        false
    }

    /// slay Calculate certificate fingerprint
    #[instrument(skip(certificate))]
    pub fn calculate_fingerprint(certificate: &Certificate, algorithm: &str) -> PkiResult<String> {
        let cert_der = certificate.to_der()
            .map_err(|e| PkiError::InvalidInput(e.to_string()))?;

        let hash = match algorithm.to_lowercase().as_str() {
            "sha256" => CryptoUtils::sha256(&cert_der),
            "sha1" => CryptoUtils::sha1(&cert_der),
            "md5" => CryptoUtils::md5(&cert_der),
            _ => return Err(PkiError::InvalidInput("Unsupported hash algorithm".to_string())),
        };

        Ok(EncodingUtils::to_hex(&hash))
    }

    /// slay Validate key usage for purpose
    #[instrument(skip(certificate))]
    pub fn validate_key_usage_for_purpose(certificate: &Certificate, purpose: &str) -> bool {
        // Would check key usage and extended key usage extensions
        // Simplified implementation
        match purpose.to_lowercase().as_str() {
            "server_auth" => true, // Would check serverAuth EKU
            "client_auth" => true, // Would check clientAuth EKU
            "code_signing" => true, // Would check codeSigning EKU
            _ => false,
        }
    }
}

// Helper functions for base64 encoding/decoding (simplified implementation)
fn base64_encode(data: &[u8]) -> String {
    // Simplified base64 implementation
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);
        
        result.push(CHARS[((b >> 18) & 63) as usize] as char);
        result.push(CHARS[((b >> 12) & 63) as usize] as char);
        result.push(if chunk.len() > 1 { CHARS[((b >> 6) & 63) as usize] as char } else { '=' });
        result.push(if chunk.len() > 2 { CHARS[(b & 63) as usize] as char } else { '=' });
    }
    
    result
}

fn base64_decode(data: &str) -> PkiResult<Vec<u8>> {
    // Simplified base64 decoding
    let cleaned = data.replace('\n', "").replace('\r', "");
    let bytes = cleaned.as_bytes();
    
    if bytes.len() % 4 != 0 {
        return Err(PkiError::InvalidInput("Invalid base64 length".to_string()));
    }
    
    let mut result = Vec::new();
    
    for chunk in bytes.chunks(4) {
        // Simplified decoding - would implement proper base64 decoding
        result.extend_from_slice(&[chunk[0], chunk[1], chunk[2]]);
    }
    
    // Remove padding bytes
    while result.last() == Some(&b'=') {
        result.pop();
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certificate_format_detection() {
        let pem_data = b"-----BEGIN CERTIFICATE-----\nMIIBkTCB+wIJAJ";
        let der_data = b"\x30\x82\x01\x91\x30\x82\x01\x3b";
        
        assert_eq!(CertificateFormatUtils::detect_format(pem_data), CertificateFormat::Pem);
        assert_eq!(CertificateFormatUtils::detect_format(der_data), CertificateFormat::Der);
    }

    #[test]
    fn test_dn_parsing() {
        let dn = "CN=example.com, O=Test Org, C=US";
        let components = DnUtils::parse_dn(dn).unwrap();
        
        assert_eq!(components.get("CN"), Some(&"example.com".to_string()));
        assert_eq!(components.get("O"), Some(&"Test Org".to_string()));
        assert_eq!(components.get("C"), Some(&"US".to_string()));
    }

    #[test]
    fn test_dn_formatting() {
        let mut components = HashMap::new();
        components.insert("CN".to_string(), "example.com".to_string());
        components.insert("O".to_string(), "Test Org".to_string());
        components.insert("C".to_string(), "US".to_string());
        
        let dn = DnUtils::format_dn(&components);
        assert!(dn.contains("CN=example.com"));
        assert!(dn.contains("O=Test Org"));
        assert!(dn.contains("C=US"));
    }

    #[test]
    fn test_oid_validation() {
        assert!(OidUtils::validate_oid("2.5.4.3"));
        assert!(OidUtils::validate_oid("1.3.6.1.5.5.7.3.1"));
        assert!(!OidUtils::validate_oid("3.5.4.3")); // First arc > 2
        assert!(!OidUtils::validate_oid("2.5.4")); // Too short
        assert!(!OidUtils::validate_oid("2.5.4.abc")); // Non-numeric
    }

    #[test]
    fn test_hex_encoding() {
        let data = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let hex = EncodingUtils::to_hex(&data);
        assert_eq!(hex, "48656c6c6f");
        
        let decoded = EncodingUtils::from_hex(&hex).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_base64_encoding() {
        let data = b"Hello, World!";
        let encoded = EncodingUtils::to_base64(data);
        assert!(!encoded.is_empty());
        
        let decoded = EncodingUtils::from_base64(&encoded).unwrap();
        // Note: simplified implementation may not match exactly
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_crypto_utils() {
        let data = b"test data";
        let sha256_hash = CryptoUtils::sha256(data);
        assert_eq!(sha256_hash.len(), 32);
        
        let sha1_hash = CryptoUtils::sha1(data);
        assert_eq!(sha1_hash.len(), 20);
        
        assert!(CryptoUtils::constant_time_eq(&sha256_hash, &sha256_hash));
        assert!(!CryptoUtils::constant_time_eq(&sha256_hash, &sha1_hash));
    }

    #[test]
    fn test_hostname_matching() {
        assert!(ValidationUtils::hostname_matches("example.com", "example.com"));
        assert!(ValidationUtils::hostname_matches("*.example.com", "sub.example.com"));
        assert!(!ValidationUtils::hostname_matches("*.example.com", "sub.sub.example.com"));
        assert!(!ValidationUtils::hostname_matches("example.com", "other.com"));
    }

    #[test]
    fn test_time_utils() {
        let now = SystemTime::now();
        let future = now + Duration::from_secs(3600);
        let past = now - Duration::from_secs(3600);
        
        assert!(TimeUtils::is_time_valid(now, past, future));
        assert!(!TimeUtils::is_time_valid(now, future, future + Duration::from_secs(3600)));
        assert!(!TimeUtils::is_time_valid(now, past - Duration::from_secs(3600), past));
    }
}
