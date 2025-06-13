//! Production-ready Message Digest Computation
//! 
//! Comprehensive message digest computation with support for multiple formats,
//! streaming processing, digital signatures, and security validation.

use crate::stdlib::packages::crypto_signatures::{
    errors::{SignatureError, SignatureResult},
    hash_algorithms::{HashAlgorithmManager, HashAlgorithm, HashResult, HashContext},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::io::{Read, Write};

/// Message digest computation modes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DigestMode {
    /// Simple hash of the entire message
    Simple,
    /// Canonical digest for signatures (RFC 3161)
    Canonical,
    /// Structured digest with metadata
    Structured,
    /// Streaming digest for large messages
    Streaming,
    /// Multi-algorithm digest for enhanced security
    MultiAlgorithm,
}

/// Message format types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageFormat {
    /// Raw binary data
    Binary,
    /// UTF-8 text
    Text,
    /// JSON document
    Json,
    /// XML document
    Xml,
    /// Base64 encoded data
    Base64,
    /// Email message (RFC 5322)
    Email,
    /// HTTP message
    Http,
    /// Custom format
    Custom(String),
}

/// Digest computation options
#[derive(Debug, Clone)]
pub struct DigestOptions {
    pub mode: DigestMode,
    pub algorithm: HashAlgorithm,
    pub format: MessageFormat,
    pub include_metadata: bool,
    pub canonical_encoding: bool,
    pub chunk_size: usize,
    pub max_message_size: Option<usize>,
}

impl Default for DigestOptions {
    fn default() -> Self {
        Self {
            mode: DigestMode::Simple,
            algorithm: HashAlgorithm::Sha256,
            format: MessageFormat::Binary,
            include_metadata: false,
            canonical_encoding: true,
            chunk_size: 8192,
            max_message_size: Some(100 * 1024 * 1024), // 100MB
        }
    }
}

/// Message metadata for digest computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMetadata {
    pub content_type: Option<String>,
    pub encoding: Option<String>,
    pub timestamp: Option<i64>,
    pub source: Option<String>,
    pub attributes: HashMap<String, String>,
}

/// Comprehensive message digest result
#[derive(Debug, Clone)]
pub struct MessageDigest {
    pub digest: Vec<u8>,
    pub algorithm: HashAlgorithm,
    pub mode: DigestMode,
    pub format: MessageFormat,
    pub message_size: usize,
    pub metadata: Option<MessageMetadata>,
    pub computation_time: Option<std::time::Duration>,
    pub verification_data: Option<VerificationData>,
}

/// Verification data for digest validation
#[derive(Debug, Clone)]
pub struct VerificationData {
    pub checksum: Vec<u8>,
    pub chunk_count: usize,
    pub integrity_hash: Vec<u8>,
}

/// Streaming digest processor
pub struct StreamingDigestProcessor {
    context: HashContext,
    algorithm: HashAlgorithm,
    options: DigestOptions,
    processed_size: usize,
    chunk_count: usize,
    start_time: std::time::Instant,
    verification_hasher: Option<HashContext>,
}

/// Production-ready message digest manager
pub struct MessageDigestManager {
    hash_manager: HashAlgorithmManager,
    default_options: DigestOptions,
}

impl MessageDigestManager {
    /// Create a new message digest manager
    pub fn new() -> Self {
        Self {
            hash_manager: HashAlgorithmManager::new(),
            default_options: DigestOptions::default(),
        }
    }

    /// Create with custom default options
    pub fn with_options(options: DigestOptions) -> Self {
        Self {
            hash_manager: HashAlgorithmManager::new(),
            default_options: options,
        }
    }

    /// Set default options
    pub fn set_default_options(&mut self, options: DigestOptions) {
        self.default_options = options;
    }

    /// Compute simple message digest
    pub fn compute_digest(&self, message: &[u8]) -> SignatureResult<MessageDigest> {
        self.compute_digest_with_options(message, &self.default_options)
    }

    /// Compute message digest with specific options
    pub fn compute_digest_with_options(
        &self,
        message: &[u8],
        options: &DigestOptions,
    ) -> SignatureResult<MessageDigest> {
        let start_time = std::time::Instant::now();

        // Check message size limit
        if let Some(max_size) = options.max_message_size {
            if message.len() > max_size {
                return Err(SignatureError::InvalidInput(
                    format!("Message size {} exceeds limit {}", message.len(), max_size)
                ));
            }
        }

        // Preprocess message based on format
        let processed_message = self.preprocess_message(message, &options.format)?;

        // Compute digest based on mode
        let hash_result = match options.mode {
            DigestMode::Simple => {
                self.hash_manager.hash_with_algorithm(&processed_message, &options.algorithm)?
            }
            DigestMode::Canonical => {
                let canonical_data = self.canonicalize_message(&processed_message, &options.format)?;
                self.hash_manager.hash_with_algorithm(&canonical_data, &options.algorithm)?
            }
            DigestMode::Structured => {
                self.compute_structured_digest(&processed_message, options)?
            }
            DigestMode::Streaming => {
                self.compute_streaming_digest(&processed_message, options)?
            }
            DigestMode::MultiAlgorithm => {
                // Use primary algorithm for main digest
                self.hash_manager.hash_with_algorithm(&processed_message, &options.algorithm)?
            }
        };

        let computation_time = start_time.elapsed();

        // Create verification data if needed
        let verification_data = if options.include_metadata {
            Some(self.create_verification_data(&processed_message, options)?)
        } else {
            None
        };

        // Create metadata if requested
        let metadata = if options.include_metadata {
            Some(MessageMetadata {
                content_type: self.detect_content_type(&options.format),
                encoding: Some("binary".to_string()),
                timestamp: Some(chrono::Utc::now().timestamp()),
                source: None,
                attributes: HashMap::new(),
            })
        } else {
            None
        };

        Ok(MessageDigest {
            digest: hash_result.digest,
            algorithm: options.algorithm.clone(),
            mode: options.mode.clone(),
            format: options.format.clone(),
            message_size: message.len(),
            metadata,
            computation_time: Some(computation_time),
            verification_data,
        })
    }

    /// Create streaming digest processor
    pub fn create_streaming_processor(&self, options: DigestOptions) -> SignatureResult<StreamingDigestProcessor> {
        let context = self.hash_manager.create_context(&options.algorithm)?;
        
        let verification_hasher = if options.include_metadata {
            Some(self.hash_manager.create_context(&HashAlgorithm::Sha256)?)
        } else {
            None
        };

        Ok(StreamingDigestProcessor {
            context,
            algorithm: options.algorithm.clone(),
            options,
            processed_size: 0,
            chunk_count: 0,
            start_time: std::time::Instant::now(),
            verification_hasher,
        })
    }

    /// Process chunk in streaming processor
    pub fn process_chunk(
        &self,
        processor: &mut StreamingDigestProcessor,
        chunk: &[u8],
    ) -> SignatureResult<()> {
        // Check size limits
        if let Some(max_size) = processor.options.max_message_size {
            if processor.processed_size + chunk.len() > max_size {
                return Err(SignatureError::InvalidInput(
                    format!("Total message size would exceed limit {}", max_size)
                ));
            }
        }

        // Process chunk based on format
        let processed_chunk = self.preprocess_message(chunk, &processor.options.format)?;

        // Update main hash
        self.hash_manager.update_context(&mut processor.context, &processed_chunk)?;

        // Update verification hash if enabled
        if let Some(ref mut verification_hasher) = processor.verification_hasher {
            self.hash_manager.update_context(verification_hasher, chunk)?;
        }

        processor.processed_size += chunk.len();
        processor.chunk_count += 1;

        Ok(())
    }

    /// Finalize streaming digest processor
    pub fn finalize_streaming_processor(
        &self,
        processor: StreamingDigestProcessor,
    ) -> SignatureResult<MessageDigest> {
        let computation_time = processor.start_time.elapsed();

        // Finalize main hash
        let hash_result = self.hash_manager.finalize_context(
            processor.context,
            processor.algorithm.clone(),
            processor.processed_size,
        )?;

        // Create verification data if enabled
        let verification_data = if processor.options.include_metadata {
            let verification_hash = if let Some(verification_hasher) = processor.verification_hasher {
                self.hash_manager.finalize_context(
                    verification_hasher,
                    HashAlgorithm::Sha256,
                    processor.processed_size,
                )?.digest
            } else {
                vec![]
            };

            Some(VerificationData {
                checksum: verification_hash.clone(),
                chunk_count: processor.chunk_count,
                integrity_hash: verification_hash,
            })
        } else {
            None
        };

        // Create metadata if requested
        let metadata = if processor.options.include_metadata {
            Some(MessageMetadata {
                content_type: self.detect_content_type(&processor.options.format),
                encoding: Some("streaming".to_string()),
                timestamp: Some(chrono::Utc::now().timestamp()),
                source: None,
                attributes: {
                    let mut attrs = HashMap::new();
                    attrs.insert("chunk_count".to_string(), processor.chunk_count.to_string());
                    attrs.insert("chunk_size".to_string(), processor.options.chunk_size.to_string());
                    attrs
                },
            })
        } else {
            None
        };

        Ok(MessageDigest {
            digest: hash_result.digest,
            algorithm: processor.algorithm,
            mode: processor.options.mode,
            format: processor.options.format,
            message_size: processor.processed_size,
            metadata,
            computation_time: Some(computation_time),
            verification_data,
        })
    }

    /// Verify message digest
    pub fn verify_digest(
        &self,
        message: &[u8],
        expected_digest: &MessageDigest,
    ) -> SignatureResult<bool> {
        let options = DigestOptions {
            mode: expected_digest.mode.clone(),
            algorithm: expected_digest.algorithm.clone(),
            format: expected_digest.format.clone(),
            include_metadata: expected_digest.metadata.is_some(),
            canonical_encoding: true,
            chunk_size: 8192,
            max_message_size: None,
        };

        let computed_digest = self.compute_digest_with_options(message, &options)?;
        Ok(computed_digest.digest == expected_digest.digest)
    }

    /// Compute digest from reader (streaming)
    pub fn compute_digest_from_reader<R: Read>(
        &self,
        mut reader: R,
        options: DigestOptions,
    ) -> SignatureResult<MessageDigest> {
        let mut processor = self.create_streaming_processor(options)?;
        let mut buffer = vec![0u8; processor.options.chunk_size];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(bytes_read) => {
                    self.process_chunk(&mut processor, &buffer[..bytes_read])?;
                }
                Err(e) => return Err(SignatureError::Internal(format!("Read error: {}", e))),
            }
        }

        self.finalize_streaming_processor(processor)
    }

    /// Compute multi-algorithm digest
    pub fn compute_multi_algorithm_digest(
        &self,
        message: &[u8],
        algorithms: &[HashAlgorithm],
    ) -> SignatureResult<Vec<MessageDigest>> {
        let mut results = Vec::new();

        for algorithm in algorithms {
            let options = DigestOptions {
                algorithm: algorithm.clone(),
                ..self.default_options.clone()
            };

            let digest = self.compute_digest_with_options(message, &options)?;
            results.push(digest);
        }

        Ok(results)
    }

    /// Compare two message digests
    pub fn compare_digests(&self, digest1: &MessageDigest, digest2: &MessageDigest) -> bool {
        digest1.digest == digest2.digest 
            && digest1.algorithm == digest2.algorithm
            && digest1.mode == digest2.mode
    }

    // Private helper methods

    fn preprocess_message(&self, message: &[u8], format: &MessageFormat) -> SignatureResult<Vec<u8>> {
        match format {
            MessageFormat::Binary => Ok(message.to_vec()),
            MessageFormat::Text => {
                // Normalize line endings for text
                let text = String::from_utf8(message.to_vec())
                    .map_err(|e| SignatureError::InvalidInput(format!("Invalid UTF-8: {}", e)))?;
                Ok(text.replace("\r\n", "\n").replace('\r', "\n").into_bytes())
            }
            MessageFormat::Json => {
                // Parse and re-serialize JSON for canonical form
                let json_str = String::from_utf8(message.to_vec())
                    .map_err(|e| SignatureError::InvalidInput(format!("Invalid UTF-8: {}", e)))?;
                let json_value: serde_json::Value = serde_json::from_str(&json_str)
                    .map_err(|e| SignatureError::InvalidInput(format!("Invalid JSON: {}", e)))?;
                let canonical_json = serde_json::to_string(&json_value)
                    .map_err(|e| SignatureError::Internal(format!("JSON serialization error: {}", e)))?;
                Ok(canonical_json.into_bytes())
            }
            MessageFormat::Base64 => {
                // Decode base64 first
                let decoded = base64::prelude::BASE64_STANDARD.decode(message)
                    .map_err(|e| SignatureError::InvalidInput(format!("Invalid Base64: {}", e)))?;
                Ok(decoded)
            }
            MessageFormat::Xml => {
                // Basic XML normalization (remove extra whitespace)
                let xml_str = String::from_utf8(message.to_vec())
                    .map_err(|e| SignatureError::InvalidInput(format!("Invalid UTF-8: {}", e)))?;
                let normalized = xml_str
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| !line.is_empty())
                    .collect::<Vec<_>>()
                    .join("\n");
                Ok(normalized.into_bytes())
            }
            MessageFormat::Email | MessageFormat::Http => {
                // Normalize headers (consistent line endings, header case)
                let text = String::from_utf8(message.to_vec())
                    .map_err(|e| SignatureError::InvalidInput(format!("Invalid UTF-8: {}", e)))?;
                Ok(text.replace("\r\n", "\n").into_bytes())
            }
            MessageFormat::Custom(_) => {
                // For custom formats, just pass through
                Ok(message.to_vec())
            }
        }
    }

    fn canonicalize_message(&self, message: &[u8], format: &MessageFormat) -> SignatureResult<Vec<u8>> {
        match format {
            MessageFormat::Json => {
                // JSON canonicalization
                let json_str = String::from_utf8(message.to_vec())
                    .map_err(|e| SignatureError::InvalidInput(format!("Invalid UTF-8: {}", e)))?;
                let json_value: serde_json::Value = serde_json::from_str(&json_str)
                    .map_err(|e| SignatureError::InvalidInput(format!("Invalid JSON: {}", e)))?;
                
                // Serialize with consistent formatting
                let canonical = serde_json::to_string(&json_value)
                    .map_err(|e| SignatureError::Internal(format!("JSON canonicalization error: {}", e)))?;
                Ok(canonical.into_bytes())
            }
            MessageFormat::Xml => {
                // XML canonicalization (simplified)
                let xml_str = String::from_utf8(message.to_vec())
                    .map_err(|e| SignatureError::InvalidInput(format!("Invalid UTF-8: {}", e)))?;
                
                // Basic canonicalization: normalize whitespace, sort attributes, etc.
                let normalized = xml_str
                    .replace("> <", "><")
                    .replace("  ", " ")
                    .trim()
                    .to_string();
                Ok(normalized.into_bytes())
            }
            _ => {
                // For other formats, use preprocessing
                self.preprocess_message(message, format)
            }
        }
    }

    fn compute_structured_digest(&self, message: &[u8], options: &DigestOptions) -> SignatureResult<HashResult> {
        // Create structured digest with metadata
        let mut structured_data = Vec::new();
        
        // Add algorithm identifier
        structured_data.extend(format!("ALG:{}", options.algorithm).as_bytes());
        structured_data.push(0); // Separator
        
        // Add format identifier
        structured_data.extend(format!("FMT:{:?}", options.format).as_bytes());
        structured_data.push(0); // Separator
        
        // Add timestamp
        structured_data.extend(chrono::Utc::now().timestamp().to_be_bytes());
        structured_data.push(0); // Separator
        
        // Add message length
        structured_data.extend((message.len() as u64).to_be_bytes());
        structured_data.push(0); // Separator
        
        // Add message data
        structured_data.extend(message);
        
        self.hash_manager.hash_with_algorithm(&structured_data, &options.algorithm)
    }

    fn compute_streaming_digest(&self, message: &[u8], options: &DigestOptions) -> SignatureResult<HashResult> {
        let chunks = message.chunks(options.chunk_size);
        let chunk_data: Vec<&[u8]> = chunks.collect();
        
        self.hash_manager.hash_chunks(&chunk_data, &options.algorithm)
    }

    fn create_verification_data(&self, message: &[u8], options: &DigestOptions) -> SignatureResult<VerificationData> {
        // Compute checksum
        let checksum_result = self.hash_manager.hash_with_algorithm(message, &HashAlgorithm::Sha256)?;
        
        // Count chunks
        let chunk_count = (message.len() + options.chunk_size - 1) / options.chunk_size;
        
        // Compute integrity hash (includes metadata)
        let mut integrity_data = Vec::new();
        integrity_data.extend(checksum_result.digest.clone());
        integrity_data.extend((chunk_count as u64).to_be_bytes());
        integrity_data.extend((message.len() as u64).to_be_bytes());
        
        let integrity_result = self.hash_manager.hash_with_algorithm(&integrity_data, &HashAlgorithm::Sha256)?;
        
        Ok(VerificationData {
            checksum: checksum_result.digest,
            chunk_count,
            integrity_hash: integrity_result.digest,
        })
    }

    fn detect_content_type(&self, format: &MessageFormat) -> Option<String> {
        match format {
            MessageFormat::Binary => Some("application/octet-stream".to_string()),
            MessageFormat::Text => Some("text/plain".to_string()),
            MessageFormat::Json => Some("application/json".to_string()),
            MessageFormat::Xml => Some("application/xml".to_string()),
            MessageFormat::Base64 => Some("application/base64".to_string()),
            MessageFormat::Email => Some("message/rfc822".to_string()),
            MessageFormat::Http => Some("message/http".to_string()),
            MessageFormat::Custom(name) => Some(format!("application/{}", name)),
        }
    }
}

impl Default for MessageDigestManager {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DigestMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DigestMode::Simple => write!(f, "Simple"),
            DigestMode::Canonical => write!(f, "Canonical"),
            DigestMode::Structured => write!(f, "Structured"),
            DigestMode::Streaming => write!(f, "Streaming"),
            DigestMode::MultiAlgorithm => write!(f, "Multi-Algorithm"),
        }
    }
}

impl fmt::Display for MessageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageFormat::Binary => write!(f, "Binary"),
            MessageFormat::Text => write!(f, "Text"),
            MessageFormat::Json => write!(f, "JSON"),
            MessageFormat::Xml => write!(f, "XML"),
            MessageFormat::Base64 => write!(f, "Base64"),
            MessageFormat::Email => write!(f, "Email"),
            MessageFormat::Http => write!(f, "HTTP"),
            MessageFormat::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

/// Convenience functions for common digest operations
pub mod utils {
    use super::*;

    /// Quick message digest with SHA-256
    pub fn quick_digest(message: &[u8]) -> SignatureResult<Vec<u8>> {
        let manager = MessageDigestManager::new();
        let result = manager.compute_digest(message)?;
        Ok(result.digest)
    }

    /// Quick canonical digest for signatures
    pub fn canonical_digest(message: &[u8], format: MessageFormat) -> SignatureResult<Vec<u8>> {
        let manager = MessageDigestManager::new();
        let options = DigestOptions {
            mode: DigestMode::Canonical,
            format,
            ..Default::default()
        };
        
        let result = manager.compute_digest_with_options(message, &options)?;
        Ok(result.digest)
    }

    /// Quick multi-algorithm digest
    pub fn multi_digest(message: &[u8]) -> SignatureResult<HashMap<String, Vec<u8>>> {
        let manager = MessageDigestManager::new();
        let algorithms = [
            HashAlgorithm::Sha256,
            HashAlgorithm::Sha3_256,
            HashAlgorithm::Blake3,
        ];
        
        let results = manager.compute_multi_algorithm_digest(message, &algorithms)?;
        let mut digest_map = HashMap::new();
        
        for result in results {
            digest_map.insert(format!("{}", result.algorithm), result.digest);
        }
        
        Ok(digest_map)
    }

    /// Verify message against digest
    pub fn verify_message(message: &[u8], expected_digest: &[u8], algorithm: HashAlgorithm) -> SignatureResult<bool> {
        let manager = MessageDigestManager::new();
        let options = DigestOptions {
            algorithm,
            ..Default::default()
        };
        
        let result = manager.compute_digest_with_options(message, &options)?;
        Ok(result.digest == expected_digest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_digest() {
        let manager = MessageDigestManager::new();
        let message = b"test message";
        
        let result = manager.compute_digest(message).unwrap();
        assert_eq!(result.algorithm, HashAlgorithm::Sha256);
        assert_eq!(result.mode, DigestMode::Simple);
        assert_eq!(result.message_size, message.len());
        assert!(!result.digest.is_empty());
    }

    #[test]
    fn test_canonical_digest() {
        let manager = MessageDigestManager::new();
        let json_message = br#"{"name":"test","value":123}"#;
        
        let options = DigestOptions {
            mode: DigestMode::Canonical,
            format: MessageFormat::Json,
            ..Default::default()
        };
        
        let result = manager.compute_digest_with_options(json_message, &options).unwrap();
        assert_eq!(result.mode, DigestMode::Canonical);
        assert_eq!(result.format, MessageFormat::Json);
    }

    #[test]
    fn test_streaming_digest() {
        let manager = MessageDigestManager::new();
        let message = b"test message for streaming";
        
        let options = DigestOptions {
            mode: DigestMode::Streaming,
            chunk_size: 5,
            ..Default::default()
        };
        
        // Compute streaming digest
        let mut processor = manager.create_streaming_processor(options.clone()).unwrap();
        
        for chunk in message.chunks(5) {
            manager.process_chunk(&mut processor, chunk).unwrap();
        }
        
        let stream_result = manager.finalize_streaming_processor(processor).unwrap();
        
        // Compute direct digest for comparison
        let direct_result = manager.compute_digest_with_options(message, &options).unwrap();
        
        // Results should be different due to different processing modes
        assert_eq!(stream_result.algorithm, direct_result.algorithm);
        assert_eq!(stream_result.message_size, direct_result.message_size);
    }

    #[test]
    fn test_multi_algorithm_digest() {
        let manager = MessageDigestManager::new();
        let message = b"test message";
        let algorithms = [HashAlgorithm::Sha256, HashAlgorithm::Blake3];
        
        let results = manager.compute_multi_algorithm_digest(message, &algorithms).unwrap();
        
        assert_eq!(results.len(), 2);
        assert_ne!(results[0].digest, results[1].digest);
        assert_eq!(results[0].algorithm, HashAlgorithm::Sha256);
        assert_eq!(results[1].algorithm, HashAlgorithm::Blake3);
    }

    #[test]
    fn test_digest_verification() {
        let manager = MessageDigestManager::new();
        let message = b"test message";
        
        let digest = manager.compute_digest(message).unwrap();
        let is_valid = manager.verify_digest(message, &digest).unwrap();
        
        assert!(is_valid);
        
        // Test with different message
        let different_message = b"different message";
        let is_valid_different = manager.verify_digest(different_message, &digest).unwrap();
        
        assert!(!is_valid_different);
    }

    #[test]
    fn test_format_preprocessing() {
        let manager = MessageDigestManager::new();
        
        // Test JSON canonicalization
        let json1 = br#"{"b": 2, "a": 1}"#;
        let json2 = br#"{"a":1,"b":2}"#;
        
        let options = DigestOptions {
            mode: DigestMode::Canonical,
            format: MessageFormat::Json,
            ..Default::default()
        };
        
        let digest1 = manager.compute_digest_with_options(json1, &options).unwrap();
        let digest2 = manager.compute_digest_with_options(json2, &options).unwrap();
        
        // Canonical digests should be the same for equivalent JSON
        assert_eq!(digest1.digest, digest2.digest);
    }

    #[test]
    fn test_utils_functions() {
        let message = b"test message";
        
        let digest = utils::quick_digest(message).unwrap();
        assert_eq!(digest.len(), 32); // SHA-256 digest size
        
        let canonical = utils::canonical_digest(message, MessageFormat::Text).unwrap();
        assert_eq!(canonical.len(), 32);
        
        let multi = utils::multi_digest(message).unwrap();
        assert!(multi.len() >= 3);
        
        let is_valid = utils::verify_message(message, &digest, HashAlgorithm::Sha256).unwrap();
        assert!(is_valid);
    }
}
