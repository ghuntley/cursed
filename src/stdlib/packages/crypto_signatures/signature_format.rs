//! Production-ready Signature Format Utilities
//! 
//! Comprehensive signature format handling with support for multiple encoding formats,
//! ASN.1 DER encoding/decoding, and standard signature formats.

use crate::stdlib::packages::crypto_signatures::errors::{SignatureError, SignatureResult};
use crate::error::Error;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported signature formats
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SignatureFormat {
    /// Raw binary signature
    Raw,
    /// Base64 encoded signature
    Base64,
    /// Hexadecimal encoded signature
    Hex,
    /// ASN.1 DER encoded signature
    Der,
    /// PEM encoded signature
    Pem,
    /// PKCS#7 signature format
    Pkcs7,
    /// Compact signature format (for compact curves)
    Compact,
}

/// Signature encoding options
#[derive(Debug, Clone)]
pub struct EncodingOptions {
    pub format: SignatureFormat,
    pub line_breaks: bool,
    pub header_footer: bool,
    pub mime_type: Option<String>,
}

impl Default for EncodingOptions {
    fn default() -> Self {
        Self {
            format: SignatureFormat::Base64,
            line_breaks: false,
            header_footer: false,
            mime_type: None,
        }
    }
}

/// Signature metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureMetadata {
    pub algorithm: String,
    pub key_id: Option<String>,
    pub timestamp: Option<i64>,
    pub digest_algorithm: Option<String>,
    pub format: SignatureFormat,
    pub size: usize,
}

/// Encoded signature with metadata
#[derive(Debug, Clone)]
pub struct EncodedSignature {
    pub data: Vec<u8>,
    pub encoding: String,
    pub metadata: SignatureMetadata,
}

/// Production-ready signature format utilities
pub struct SignatureFormatHandler {
    default_options: EncodingOptions,
}

impl SignatureFormatHandler {
    /// Create a new signature format handler
    pub fn new() -> Self {
        Self {
            default_options: EncodingOptions::default(),
        }
    }

    /// Create with custom default options
    pub fn with_options(options: EncodingOptions) -> Self {
        Self {
            default_options: options,
        }
    }

    /// Encode signature using default options
    pub fn encode(&self, signature: &[u8]) -> SignatureResult<String> {
        self.encode_with_options(signature, &self.default_options)
    }

    /// Encode signature with specific options
    pub fn encode_with_options(&self, signature: &[u8], options: &EncodingOptions) -> SignatureResult<String> {
        match options.format {
            SignatureFormat::Raw => {
                // Raw format returns hex for string representation
                Ok(hex::encode(signature))
            }
            SignatureFormat::Base64 => {
                let encoded = BASE64_STANDARD.encode(signature);
                if options.line_breaks {
                    Ok(self.add_line_breaks(&encoded, 64))
                } else {
                    Ok(encoded)
                }
            }
            SignatureFormat::Hex => {
                let encoded = hex::encode(signature);
                if options.line_breaks {
                    Ok(self.add_line_breaks(&encoded, 32))
                } else {
                    Ok(encoded)
                }
            }
            SignatureFormat::Der => {
                // For DER format, we wrap the signature in a simple ASN.1 structure
                let der_encoded = self.encode_der_signature(signature)?;
                Ok(BASE64_STANDARD.encode(der_encoded))
            }
            SignatureFormat::Pem => {
                let base64_data = BASE64_STANDARD.encode(signature);
                let formatted = self.add_line_breaks(&base64_data, 64);
                
                if options.header_footer {
                    Ok(format!(
                        "-----BEGIN SIGNATURE-----\n{}\n-----END SIGNATURE-----",
                        formatted
                    ))
                } else {
                    Ok(formatted)
                }
            }
            SignatureFormat::Pkcs7 => {
                // Basic PKCS#7 structure (simplified)
                let pkcs7_data = self.encode_pkcs7_signature(signature)?;
                Ok(BASE64_STANDARD.encode(pkcs7_data))
            }
            SignatureFormat::Compact => {
                // Compact format for elliptic curve signatures
                if signature.len() == 64 {
                    // Assume it's an ECDSA/EdDSA signature
                    Ok(BASE64_STANDARD.encode(signature))
                } else {
                    Err(SignatureError::Format("Compact format only supports 64-byte signatures".to_string()))
                }
            }
        }
    }

    /// Decode signature using default format
    pub fn decode(&self, encoded: &str) -> SignatureResult<Vec<u8>> {
        self.decode_with_format(encoded, &self.default_options.format)
    }

    /// Decode signature with specified format
    pub fn decode_with_format(&self, encoded: &str, format: &SignatureFormat) -> SignatureResult<Vec<u8>> {
        match format {
            SignatureFormat::Raw | SignatureFormat::Hex => {
                let cleaned = encoded.replace([' ', '\n', '\r', '\t'], "");
                hex::decode(cleaned)
                    .map_err(|e| SignatureError::Format(format!("Hex decode error: {}", e)))
            }
            SignatureFormat::Base64 | SignatureFormat::Compact => {
                let cleaned = encoded.replace([' ', '\n', '\r', '\t'], "");
                BASE64_STANDARD.decode(cleaned)
                    .map_err(|e| SignatureError::Format(format!("Base64 decode error: {}", e)))
            }
            SignatureFormat::Der => {
                let cleaned = encoded.replace([' ', '\n', '\r', '\t'], "");
                let der_data = BASE64_STANDARD.decode(cleaned)
                    .map_err(|e| SignatureError::Format(format!("Base64 decode error: {}", e)))?;
                self.decode_der_signature(&der_data)
            }
            SignatureFormat::Pem => {
                let cleaned = self.extract_pem_data(encoded)?;
                BASE64_STANDARD.decode(cleaned)
                    .map_err(|e| SignatureError::Format(format!("PEM decode error: {}", e)))
            }
            SignatureFormat::Pkcs7 => {
                let cleaned = encoded.replace([' ', '\n', '\r', '\t'], "");
                let pkcs7_data = BASE64_STANDARD.decode(cleaned)
                    .map_err(|e| SignatureError::Format(format!("Base64 decode error: {}", e)))?;
                self.decode_pkcs7_signature(&pkcs7_data)
            }
        }
    }

    /// Auto-detect signature format and decode
    pub fn auto_decode(&self, encoded: &str) -> SignatureResult<(Vec<u8>, SignatureFormat)> {
        // Try different formats in order of likelihood
        let formats = [
            SignatureFormat::Base64,
            SignatureFormat::Hex,
            SignatureFormat::Pem,
            SignatureFormat::Der,
            SignatureFormat::Pkcs7,
        ];

        for format in &formats {
            if let Ok(decoded) = self.decode_with_format(encoded, format) {
                return Ok((decoded, format.clone()));
            }
        }

        Err(SignatureError::Format("Unable to auto-detect signature format".to_string()))
    }

    /// Create signature metadata
    pub fn create_metadata(
        &self,
        signature: &[u8],
        algorithm: &str,
        format: SignatureFormat,
    ) -> SignatureMetadata {
        SignatureMetadata {
            algorithm: algorithm.to_string(),
            key_id: None,
            timestamp: Some(chrono::Utc::now().timestamp()),
            digest_algorithm: None,
            format,
            size: signature.len(),
        }
    }

    /// Encode signature with metadata
    pub fn encode_with_metadata(
        &self,
        signature: &[u8],
        metadata: &SignatureMetadata,
    ) -> SignatureResult<EncodedSignature> {
        let options = EncodingOptions {
            format: metadata.format.clone(),
            ..self.default_options.clone()
        };

        let encoding = self.encode_with_options(signature, &options)?;

        Ok(EncodedSignature {
            data: signature.to_vec(),
            encoding,
            metadata: metadata.clone(),
        })
    }

    /// Validate signature format
    pub fn validate_format(&self, signature: &[u8], expected_format: &SignatureFormat) -> SignatureResult<bool> {
        match expected_format {
            SignatureFormat::Raw => Ok(true), // Raw format accepts any bytes
            SignatureFormat::Compact => {
                // Compact signatures should be exactly 64 bytes
                Ok(signature.len() == 64)
            }
            SignatureFormat::Der => {
                // Basic DER validation
                self.validate_der_signature(signature)
            }
            SignatureFormat::Pkcs7 => {
                // Basic PKCS#7 validation
                self.validate_pkcs7_signature(signature)
            }
            _ => {
                // For encoded formats, try to decode and see if it works
                let encoded = self.encode_with_options(signature, &EncodingOptions {
                    format: expected_format.clone(),
                    ..Default::default()
                })?;
                
                match self.decode_with_format(&encoded, expected_format) {
                    Ok(decoded) => Ok(decoded == signature),
                    Err(_) => Ok(false),
                }
            }
        }
    }

    // Private helper methods

    fn add_line_breaks(&self, input: &str, line_length: usize) -> String {
        input
            .chars()
            .collect::<Vec<char>>()
            .chunks(line_length)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn extract_pem_data(&self, pem: &str) -> SignatureResult<String> {
        let lines: Vec<&str> = pem.split("\n").collect();
        let mut data_lines = Vec::new();
        let mut in_data = false;

        for line in lines {
            if line.starts_with("-----BEGIN") {
                in_data = true;
                continue;
            }
            if line.starts_with("-----END") {
                break;
            }
            if in_data {
                data_lines.push(line.trim());
            }
        }

        if data_lines.is_empty() {
            // No PEM headers found, assume it's just base64 data
            Ok(pem.replace([' ', '\n', '\r', '\t'], ""))
        } else {
            Ok(data_lines.join(""))
        }
    }

    fn encode_der_signature(&self, signature: &[u8]) -> SignatureResult<Vec<u8>> {
        // Simple DER encoding: SEQUENCE { BIT STRING signature }
        let mut der = Vec::new();
        
        // SEQUENCE tag
        der.push(0x30);
        
        // Length of content (BIT STRING tag + length + content)
        let content_len = 1 + self.encode_der_length(signature.len() + 1).len() + signature.len() + 1;
        der.extend(self.encode_der_length(content_len));
        
        // BIT STRING tag
        der.push(0x03);
        
        // Length of bit string (signature + unused bits indicator)
        der.extend(self.encode_der_length(signature.len() + 1));
        
        // Unused bits (0 for signature data)
        der.push(0x00);
        
        // Signature data
        der.extend(signature);
        
        Ok(der)
    }

    fn decode_der_signature(&self, der_data: &[u8]) -> SignatureResult<Vec<u8>> {
        if der_data.len() < 4 {
            return Err(SignatureError::Format("DER data too short".to_string()));
        }

        let mut pos = 0;
        
        // Check SEQUENCE tag
        if der_data[pos] != 0x30 {
            return Err(SignatureError::Format("Invalid DER SEQUENCE tag".to_string()));
        }
        pos += 1;
        
        // Skip sequence length
        let (_, new_pos) = self.decode_der_length(der_data, pos)?;
        pos = new_pos;
        
        // Check BIT STRING tag
        if pos >= der_data.len() || der_data[pos] != 0x03 {
            return Err(SignatureError::Format("Invalid DER BIT STRING tag".to_string()));
        }
        pos += 1;
        
        // Get bit string length
        let (length, new_pos) = self.decode_der_length(der_data, pos)?;
        pos = new_pos;
        
        // Skip unused bits indicator
        if pos >= der_data.len() {
            return Err(SignatureError::Format("Missing unused bits indicator".to_string()));
        }
        pos += 1;
        
        // Extract signature data
        let signature_len = length - 1; // Subtract 1 for unused bits indicator
        if pos + signature_len > der_data.len() {
            return Err(SignatureError::Format("Invalid DER signature length".to_string()));
        }
        
        Ok(der_data[pos..pos + signature_len].to_vec())
    }

    fn encode_der_length(&self, length: usize) -> Vec<u8> {
        if length < 128 {
            vec![length as u8]
        } else if length < 256 {
            vec![0x81, length as u8]
        } else if length < 65536 {
            vec![0x82, (length >> 8) as u8, length as u8]
        } else {
            // For larger lengths, use 3 bytes
            vec![0x83, (length >> 16) as u8, (length >> 8) as u8, length as u8]
        }
    }

    fn decode_der_length(&self, data: &[u8], pos: usize) -> SignatureResult<(usize, usize)> {
        if pos >= data.len() {
            return Err(SignatureError::Format("Unexpected end of DER data".to_string()));
        }

        let first_byte = data[pos];
        if first_byte & 0x80 == 0 {
            // Short form
            Ok((first_byte as usize, pos + 1))
        } else {
            // Long form
            let length_bytes = (first_byte & 0x7f) as usize;
            if length_bytes == 0 || pos + length_bytes >= data.len() {
                return Err(SignatureError::Format("Invalid DER length encoding".to_string()));
            }

            let mut length = 0usize;
            for i in 1..=length_bytes {
                length = (length << 8) | (data[pos + i] as usize);
            }

            Ok((length, pos + length_bytes + 1))
        }
    }

    fn encode_pkcs7_signature(&self, signature: &[u8]) -> SignatureResult<Vec<u8>> {
        // Simplified PKCS#7 structure
        // In a real implementation, this would be much more complex
        let mut pkcs7 = Vec::new();
        
        // PKCS#7 ContentInfo SEQUENCE
        pkcs7.push(0x30); // SEQUENCE tag
        
        // Length placeholder (we'll fill this in)
        let content_start = pkcs7.len();
        pkcs7.extend(vec![0, 0, 0]); // Placeholder for length
        
        // signedData OID (1.2.840.113549.1.7.2)
        pkcs7.extend(&[
            0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x02
        ]);
        
        // Signature data (simplified)
        pkcs7.push(0x04); // OCTET STRING
        pkcs7.extend(self.encode_der_length(signature.len()));
        pkcs7.extend(signature);
        
        // Fix the length
        let total_length = pkcs7.len() - content_start - 3;
        let length_bytes = self.encode_der_length(total_length);
        
        // Replace placeholder with actual length
        pkcs7.splice(content_start..content_start + 3, length_bytes);
        
        Ok(pkcs7)
    }

    fn decode_pkcs7_signature(&self, pkcs7_data: &[u8]) -> SignatureResult<Vec<u8>> {
        // Simplified PKCS#7 decoding
        // This is a basic implementation - real PKCS#7 is much more complex
        
        if pkcs7_data.len() < 20 {
            return Err(SignatureError::Format("PKCS#7 data too short".to_string()));
        }

        // Look for OCTET STRING containing the signature
        for i in 0..pkcs7_data.len().saturating_sub(2) {
            if pkcs7_data[i] == 0x04 { // OCTET STRING tag
                let (length, pos) = self.decode_der_length(pkcs7_data, i + 1)?;
                if pos + length <= pkcs7_data.len() {
                    return Ok(pkcs7_data[pos..pos + length].to_vec());
                }
            }
        }

        Err(SignatureError::Format("No signature found in PKCS#7 data".to_string()))
    }

    fn validate_der_signature(&self, signature: &[u8]) -> SignatureResult<bool> {
        // Basic DER validation
        if signature.len() < 4 {
            return Ok(false);
        }

        // Check if it starts with SEQUENCE tag
        if signature[0] != 0x30 {
            return Ok(false);
        }

        // Try to decode and see if it's valid
        match self.decode_der_signature(signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn validate_pkcs7_signature(&self, signature: &[u8]) -> SignatureResult<bool> {
        // Basic PKCS#7 validation
        if signature.len() < 20 {
            return Ok(false);
        }

        // Check if it starts with SEQUENCE tag
        if signature[0] != 0x30 {
            return Ok(false);
        }

        // Try to decode and see if it's valid
        match self.decode_pkcs7_signature(signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

impl Default for SignatureFormatHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SignatureFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignatureFormat::Raw => write!(f, "Raw"),
            SignatureFormat::Base64 => write!(f, "Base64"),
            SignatureFormat::Hex => write!(f, "Hex"),
            SignatureFormat::Der => write!(f, "DER"),
            SignatureFormat::Pem => write!(f, "PEM"),
            SignatureFormat::Pkcs7 => write!(f, "PKCS#7"),
            SignatureFormat::Compact => write!(f, "Compact"),
        }
    }
}

/// Convenience functions for common operations
pub mod utils {
    use super::*;

    /// Quick base64 encoding
    pub fn encode_base64(signature: &[u8]) -> String {
        BASE64_STANDARD.encode(signature)
    }

    /// Quick base64 decoding
    pub fn decode_base64(encoded: &str) -> SignatureResult<Vec<u8>> {
        BASE64_STANDARD.decode(encoded)
            .map_err(|e| SignatureError::Format(format!("Base64 decode error: {}", e)))
    }

    /// Quick hex encoding
    pub fn encode_hex(signature: &[u8]) -> String {
        hex::encode(signature)
    }

    /// Quick hex decoding
    pub fn decode_hex(encoded: &str) -> SignatureResult<Vec<u8>> {
        hex::decode(encoded)
            .map_err(|e| SignatureError::Format(format!("Hex decode error: {}", e)))
    }

    /// Convert signature to PEM format
    pub fn to_pem(signature: &[u8]) -> String {
        let base64_data = encode_base64(signature);
        let formatted = base64_data
            .chars()
            .collect::<Vec<char>>()
            .chunks(64)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        
        format!(
            "-----BEGIN SIGNATURE-----\n{}\n-----END SIGNATURE-----",
            formatted
        )
    }

    /// Parse PEM format signature
    pub fn from_pem(pem: &str) -> SignatureResult<Vec<u8>> {
        let handler = SignatureFormatHandler::new();
        handler.decode_with_format(pem, &SignatureFormat::Pem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encoding() {
        let handler = SignatureFormatHandler::new();
        let signature = b"test signature data";
        
        let encoded = handler.encode(signature).unwrap();
        let decoded = handler.decode(&encoded).unwrap();
        
        assert_eq!(decoded, signature);
    }

    #[test]
    fn test_hex_encoding() {
        let handler = SignatureFormatHandler::with_options(EncodingOptions {
            format: SignatureFormat::Hex,
            ..Default::default()
        });
        
        let signature = b"test signature data";
        let encoded = handler.encode(signature).unwrap();
        let decoded = handler.decode(&encoded).unwrap();
        
        assert_eq!(decoded, signature);
    }

    #[test]
    fn test_pem_encoding() {
        let handler = SignatureFormatHandler::new();
        let signature = b"test signature data";
        
        let options = EncodingOptions {
            format: SignatureFormat::Pem,
            header_footer: true,
            line_breaks: true,
        };
        
        let encoded = handler.encode_with_options(signature, &options).unwrap();
        assert!(encoded.contains("-----BEGIN SIGNATURE-----"));
        assert!(encoded.contains("-----END SIGNATURE-----"));
        
        let decoded = handler.decode_with_format(&encoded, &SignatureFormat::Pem).unwrap();
        assert_eq!(decoded, signature);
    }

    #[test]
    fn test_der_encoding() {
        let handler = SignatureFormatHandler::new();
        let signature = b"test signature data";
        
        let options = EncodingOptions {
            format: SignatureFormat::Der,
            ..Default::default()
        };
        
        let encoded = handler.encode_with_options(signature, &options).unwrap();
        let decoded = handler.decode_with_format(&encoded, &SignatureFormat::Der).unwrap();
        
        assert_eq!(decoded, signature);
    }

    #[test]
    fn test_auto_detect() {
        let handler = SignatureFormatHandler::new();
        let signature = b"test signature data";
        
        // Test with different formats
        let base64_encoded = utils::encode_base64(signature);
        let (decoded, format) = handler.auto_decode(&base64_encoded).unwrap();
        assert_eq!(decoded, signature);
        assert_eq!(format, SignatureFormat::Base64);
        
        let hex_encoded = utils::encode_hex(signature);
        let (decoded, format) = handler.auto_decode(&hex_encoded).unwrap();
        assert_eq!(decoded, signature);
        assert_eq!(format, SignatureFormat::Hex);
    }

    #[test]
    fn test_metadata_creation() {
        let handler = SignatureFormatHandler::new();
        let signature = b"test signature data";
        
        let metadata = handler.create_metadata(
            signature,
            "Ed25519",
            SignatureFormat::Base64,
        );
        
        assert_eq!(metadata.algorithm, "Ed25519");
        assert_eq!(metadata.format, SignatureFormat::Base64);
        assert_eq!(metadata.size, signature.len());
        assert!(metadata.timestamp.is_some());
    }
}
