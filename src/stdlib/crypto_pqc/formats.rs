use crate::error::CursedError;
/// Key Format Conversion and Serialization
/// 
/// This module provides functionality for converting PQC keys between different
/// formats including PEM, DER, JWK, and custom CURSED formats.

use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Serialize, Deserialize};
// use crate::stdlib::crypto_pqc::{PqcResult, PqcError, AlgorithmType, SecurityLevel};
// use crate::stdlib::crypto_pqc::key_management::{PqcKey, KeyType, KeyFormat};

/// Key format converter
pub struct KeyFormatConverter;

impl KeyFormatConverter {
    /// Convert key to PEM format
    pub fn to_pem(key: &PqcKey) -> PqcResult<String> {
        let label = match key.key_type {

        let encoded = BASE64.encode(&key.key_data);
        let mut pem = format!("-----BEGIN {}-----\n", label);
        
        // Split into 64-character lines
        for chunk in encoded.as_bytes().chunks(64) {
            pem.push_str(&String::from_utf8_lossy(chunk));
            pem.push('\n');
        pem.push_str(&format!("-----END {}-----\n", label));
        Ok(pem)
    /// Parse key from PEM format
    pub fn from_pem(pem_data: &str) -> PqcResult<PqcKey> {
        let lines: Vec<&str> = pem_data.split("\n").collect();
        
        if lines.len() < 3 {
            return Err(PqcError::FormatError("Invalid PEM format".to_string()));
        let begin_line = lines[0];
        let end_line = lines[lines.len() - 1];

        // Parse header
        let (algorithm, key_type) = Self::parse_pem_header(begin_line)?;
        
        // Verify footer matches header
        let expected_end = begin_line.replace("BEGIN", "END");
        if end_line != expected_end {
            return Err(PqcError::FormatError("PEM header/footer mismatch".to_string()));
        // Extract and decode base64 content
        let mut base64_content = String::new();
        for line in &lines[1..lines.len()-1] {
            base64_content.push_str(line.trim());
        let key_data = BASE64.decode(&base64_content)
            .map_err(|e| PqcError::FormatError(format!("Base64 decode error: {}", e)))?;

        let mut key = PqcKey::new(algorithm, SecurityLevel::Level1, key_type, key_data);
        key.metadata.format = KeyFormat::Pem;
        
        Ok(key)
    /// Parse PEM header to extract algorithm and key type
    fn parse_pem_header(header: &str) -> PqcResult<(AlgorithmType, KeyType)> {
        if !header.starts_with("-----BEGIN ") || !header.ends_with("-----") {
            return Err(PqcError::FormatError("Invalid PEM header format".to_string()));
        let content = &header[11..header.len()-5]; // Remove "-----BEGIN " and "-----"
        
        let (algorithm, key_type) = if content.ends_with(" PUBLIC KEY") {
            let alg_str = &content[..content.len()-11]; // Remove " PUBLIC KEY"
            (Self::parse_algorithm(alg_str)?, KeyType::Public)
        } else if content.ends_with(" PRIVATE KEY") {
            let alg_str = &content[..content.len()-12]; // Remove " PRIVATE KEY"
            (Self::parse_algorithm(alg_str)?, KeyType::Secret)
        } else {
            return Err(PqcError::FormatError("Unrecognized PEM key type".to_string()));

        Ok((algorithm, key_type))
    /// Parse algorithm from string
    fn parse_algorithm(alg_str: &str) -> PqcResult<AlgorithmType> {
        match alg_str {
        }
    }

    /// Convert key to DER format
    pub fn to_der(key: &PqcKey) -> PqcResult<Vec<u8>> {
        // Simplified DER encoding - in a real implementation, this would use proper ASN.1 DER encoding
        let mut der = Vec::new();
        
        // Algorithm identifier (simplified)
        der.push(key.algorithm as u8);
        der.push(key.security_level as u8);
        der.push(key.key_type as u8);
        
        // Key data length (big-endian u32)
        let len = key.key_data.len() as u32;
        der.extend_from_slice(&len.to_be_bytes());
        
        // Key data
        der.extend_from_slice(&key.key_data);
        
        Ok(der)
    /// Parse key from DER format
    pub fn from_der(der_data: &[u8]) -> PqcResult<PqcKey> {
        if der_data.len() < 7 {
            return Err(PqcError::FormatError("DER data too short".to_string()));
        let algorithm = match der_data[0] {

        let security_level = match der_data[1] {

        let key_type = match der_data[2] {

        let key_len = u32::from_be_bytes([der_data[3], der_data[4], der_data[5], der_data[6]]) as usize;
        
        if der_data.len() < 7 + key_len {
            return Err(PqcError::FormatError("DER key data truncated".to_string()));
        let key_data = der_data[7..7 + key_len].to_vec();
        
        let mut key = PqcKey::new(algorithm, security_level, key_type, key_data);
        key.metadata.format = KeyFormat::Der;
        
        Ok(key)
    /// Convert key to JWK format
    pub fn to_jwk(key: &PqcKey) -> PqcResult<String> {
        let jwk = PqcJwk {

        serde_json::to_string_pretty(&jwk)
            .map_err(|e| PqcError::FormatError(format!("JWK serialization error: {}", e)))
    /// Parse key from JWK format
    pub fn from_jwk(jwk_data: &str) -> PqcResult<PqcKey> {
        let jwk: PqcJwk = serde_json::from_str(jwk_data)
            .map_err(|e| PqcError::FormatError(format!("JWK parse error: {}", e)))?;

        if jwk.kty != "PQC" {
            return Err(PqcError::FormatError("Not a PQC JWK".to_string()));
        let algorithm = Self::parse_algorithm(&jwk.alg)?;
        
        let security_level = match jwk.security_level.as_str() {

        let key_type = match jwk.key_type.as_str() {

        let key_data = BASE64.decode(&jwk.key_data)
            .map_err(|e| PqcError::FormatError(format!("JWK key data decode error: {}", e)))?;

        let mut key = PqcKey::new(algorithm, security_level, key_type, key_data);
        key.metadata.format = KeyFormat::Jwk;
        
        if let Some(key_id) = jwk.key_id {
            key.metadata.key_id = key_id;
        Ok(key)
    /// Convert key to CURSED native format
    pub fn to_cursed_native(key: &PqcKey) -> PqcResult<String> {
        serde_json::to_string_pretty(key)
            .map_err(|e| PqcError::FormatError(format!("CURSED native serialization error: {}", e)))
    /// Parse key from CURSED native format
    pub fn from_cursed_native(data: &str) -> PqcResult<PqcKey> {
        let mut key: PqcKey = serde_json::from_str(data)
            .map_err(|e| PqcError::FormatError(format!("CURSED native parse error: {}", e)))?;
        
        key.metadata.format = KeyFormat::CursedNative;
        Ok(key)
    }
}

/// JWK representation for PQC keys
#[derive(Debug, Serialize, Deserialize)]
struct PqcJwk {
    #[serde(skip_serializing_if = "Option::is_none")]
/// Batch format converter for multiple keys
pub struct BatchFormatConverter;

impl BatchFormatConverter {
    /// Convert multiple keys to a format
    pub fn convert_batch(keys: &[PqcKey], target_format: KeyFormat) -> PqcResult<Vec<String>> {
        let mut results = Vec::new();
        
        for key in keys {
            let converted = match target_format {
                KeyFormat::Der => {
                    let der_bytes = KeyFormatConverter::to_der(key)?;
                    BASE64.encode(&der_bytes)
            results.push(converted);
        Ok(results)
    /// Create a key bundle with multiple formats
    pub fn create_key_bundle(key: &PqcKey) -> PqcResult<KeyBundle> {
        Ok(KeyBundle {
        })
    }
}

/// Key bundle containing multiple formats
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyBundle {
/// Format validation utilities
pub struct FormatValidator;

impl FormatValidator {
    /// Validate PEM format
    pub fn validate_pem(pem_data: &str) -> bool {
        let lines: Vec<&str> = pem_data.split("\n").collect();
        
        if lines.len() < 3 {
            return false;
        let begin_line = lines[0];
        let end_line = lines[lines.len() - 1];

        begin_line.starts_with("-----BEGIN ") && 
        begin_line.ends_with("-----") &&
        end_line.starts_with("-----END ") &&
        end_line.ends_with("-----") &&
        begin_line.replace("BEGIN", "END") == end_line
    /// Validate JWK format
    pub fn validate_jwk(jwk_data: &str) -> bool {
        match serde_json::from_str::<PqcJwk>(jwk_data) {
        }
    }

    /// Validate CURSED native format
    pub fn validate_cursed_native(data: &str) -> bool {
        serde_json::from_str::<PqcKey>(data).is_ok()
    /// Detect format from content
    pub fn detect_format(data: &str) -> Option<KeyFormat> {
        if Self::validate_pem(data) {
            Some(KeyFormat::Pem)
        } else if Self::validate_jwk(data) {
            Some(KeyFormat::Jwk)
        } else if Self::validate_cursed_native(data) {
            Some(KeyFormat::CursedNative)
        } else {
            None
        }
    }
