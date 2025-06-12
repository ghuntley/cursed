/// Response compression utilities (gzip, deflate, brotli)
use std::collections::HashMap;

/// Compression types supported
#[derive(Debug, Clone, PartialEq)]
pub enum CompressionType {
    Gzip,
    Deflate,
    Brotli,
    Identity, // No compression
}

impl CompressionType {
    /// Parse compression type from Accept-Encoding header
    pub fn from_accept_encoding(accept_encoding: &str) -> Vec<(CompressionType, f32)> {
        let mut encodings = Vec::new();
        
        for encoding in accept_encoding.split(',') {
            let encoding = encoding.trim();
            let parts: Vec<&str> = encoding.split(';').collect();
            let name = parts[0].trim();
            
            // Parse quality value
            let quality = if parts.len() > 1 {
                parts[1].trim()
                    .strip_prefix("q=")
                    .and_then(|q| q.parse().ok())
                    .unwrap_or(1.0)
            } else {
                1.0
            };

            let compression_type = match name.to_lowercase().as_str() {
                "gzip" => CompressionType::Gzip,
                "deflate" => CompressionType::Deflate,
                "br" => CompressionType::Brotli,
                "*" => CompressionType::Identity,
                "identity" => CompressionType::Identity,
                _ => continue,
            };

            encodings.push((compression_type, quality));
        }

        // Sort by quality (descending)
        encodings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        encodings
    }

    /// Get the encoding name for HTTP headers
    pub fn header_name(&self) -> &'static str {
        match self {
            CompressionType::Gzip => "gzip",
            CompressionType::Deflate => "deflate",
            CompressionType::Brotli => "br",
            CompressionType::Identity => "identity",
        }
    }

    /// Check if compression type is supported
    pub fn is_supported(&self) -> bool {
        matches!(self, CompressionType::Gzip | CompressionType::Deflate)
    }
}

/// Response compressor
pub struct ResponseCompressor {
    compression_level: u8,
    min_size_threshold: usize,
    compressible_types: Vec<String>,
}

impl ResponseCompressor {
    /// Create new response compressor
    pub fn new() -> Self {
        Self {
            compression_level: 6,
            min_size_threshold: 1024, // 1KB
            compressible_types: vec![
                "text/html".to_string(),
                "text/plain".to_string(),
                "text/css".to_string(),
                "application/javascript".to_string(),
                "application/json".to_string(),
                "application/xml".to_string(),
                "text/xml".to_string(),
                "image/svg+xml".to_string(),
            ],
        }
    }

    /// Set compression level (0-9)
    pub fn with_level(mut self, level: u8) -> Self {
        self.compression_level = level.min(9);
        self
    }

    /// Set minimum size threshold for compression
    pub fn with_threshold(mut self, threshold: usize) -> Self {
        self.min_size_threshold = threshold;
        self
    }

    /// Add compressible content type
    pub fn add_compressible_type(mut self, content_type: String) -> Self {
        self.compressible_types.push(content_type);
        self
    }

    /// Check if content should be compressed
    pub fn should_compress(&self, content: &[u8], content_type: &str) -> bool {
        // Check size threshold
        if content.len() < self.min_size_threshold {
            return false;
        }

        // Check if content type is compressible
        let content_type_base = content_type.split(';').next().unwrap_or("").trim();
        self.compressible_types.iter().any(|ct| ct == content_type_base)
    }

    /// Select best compression type from Accept-Encoding header
    pub fn select_compression(&self, accept_encoding: &str) -> CompressionType {
        let encodings = CompressionType::from_accept_encoding(accept_encoding);
        
        for (encoding, _quality) in encodings {
            if encoding.is_supported() {
                return encoding;
            }
        }

        CompressionType::Identity
    }

    /// Compress content using specified compression type
    pub fn compress(
        &self,
        content: &[u8],
        compression_type: CompressionType,
    ) -> Result<Vec<u8>, CompressionError> {
        match compression_type {
            CompressionType::Gzip => self.gzip_compress(content),
            CompressionType::Deflate => self.deflate_compress(content),
            CompressionType::Brotli => Err(CompressionError::UnsupportedType),
            CompressionType::Identity => Ok(content.to_vec()),
        }
    }

    /// Decompress content
    pub fn decompress(
        &self,
        content: &[u8],
        compression_type: CompressionType,
    ) -> Result<Vec<u8>, CompressionError> {
        match compression_type {
            CompressionType::Gzip => self.gzip_decompress(content),
            CompressionType::Deflate => self.deflate_decompress(content),
            CompressionType::Brotli => Err(CompressionError::UnsupportedType),
            CompressionType::Identity => Ok(content.to_vec()),
        }
    }

    /// Compress response with automatic compression type selection
    pub fn compress_response(
        &self,
        content: &[u8],
        content_type: &str,
        accept_encoding: &str,
    ) -> CompressionResult {
        // Check if compression should be applied
        if !self.should_compress(content, content_type) {
            return CompressionResult {
                content: content.to_vec(),
                compression_type: CompressionType::Identity,
                original_size: content.len(),
                compressed_size: content.len(),
                compression_ratio: 1.0,
            };
        }

        // Select compression type
        let compression_type = self.select_compression(accept_encoding);
        
        // Compress content
        match self.compress(content, compression_type.clone()) {
            Ok(compressed) => CompressionResult {
                content: compressed.clone(),
                compression_type,
                original_size: content.len(),
                compressed_size: compressed.len(),
                compression_ratio: compressed.len() as f64 / content.len() as f64,
            },
            Err(_) => CompressionResult {
                content: content.to_vec(),
                compression_type: CompressionType::Identity,
                original_size: content.len(),
                compressed_size: content.len(),
                compression_ratio: 1.0,
            },
        }
    }

    /// GZIP compression (simplified implementation)
    fn gzip_compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        // Simplified GZIP implementation
        // In a real implementation, this would use proper GZIP compression
        let mut compressed = Vec::new();
        
        // GZIP header
        compressed.extend_from_slice(&[0x1f, 0x8b, 0x08, 0x00]); // Magic + compression method + flags
        compressed.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Timestamp
        compressed.push(0x00); // Extra flags
        compressed.push(0xff); // OS

        // Compress data using simple run-length encoding
        compressed.extend_from_slice(&self.simple_compress(data));

        // Add CRC32 and original size (simplified)
        let crc = self.calculate_crc32(data);
        compressed.extend_from_slice(&crc.to_le_bytes());
        compressed.extend_from_slice(&(data.len() as u32).to_le_bytes());

        Ok(compressed)
    }

    /// GZIP decompression (simplified implementation)
    fn gzip_decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        if data.len() < 18 {
            return Err(CompressionError::InvalidData);
        }

        // Verify GZIP header
        if data[0] != 0x1f || data[1] != 0x8b {
            return Err(CompressionError::InvalidData);
        }

        // Extract compressed data (skip header and trailer)
        let compressed_data = &data[10..data.len() - 8];
        self.simple_decompress(compressed_data)
    }

    /// Deflate compression (simplified implementation)
    fn deflate_compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        // Simplified deflate implementation
        Ok(self.simple_compress(data))
    }

    /// Deflate decompression (simplified implementation)
    fn deflate_decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        self.simple_decompress(data)
    }

    /// LZ77-inspired compression algorithm
    fn simple_compress(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut compressed = Vec::new();
        let mut i = 0;
        let window_size = 256.min(data.len());

        while i < data.len() {
            let mut best_length = 0;
            let mut best_distance = 0;

            // Look for matches in the sliding window
            let search_start = if i >= window_size { i - window_size } else { 0 };
            
            for j in search_start..i {
                let mut length = 0;
                
                // Find longest matching sequence
                while i + length < data.len() 
                    && j + length < i 
                    && data[j + length] == data[i + length] 
                    && length < 255 {
                    length += 1;
                }
                
                if length > best_length && length >= 3 {
                    best_length = length;
                    best_distance = i - j;
                }
            }

            if best_length >= 3 {
                // Encode as (distance, length) pair
                compressed.push(0x00); // Marker for compressed data
                compressed.push(best_distance as u8);
                compressed.push(best_length as u8);
                i += best_length;
            } else {
                // Literal byte
                let byte = data[i];
                if byte == 0x00 {
                    compressed.push(0x00);
                    compressed.push(0xff); // Escape for literal 0x00
                } else {
                    compressed.push(byte);
                }
                i += 1;
            }
        }

        compressed
    }

    /// LZ77-inspired decompression algorithm  
    fn simple_decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let mut decompressed = Vec::new();
        let mut i = 0;

        while i < data.len() {
            if data[i] == 0x00 {
                if i + 1 < data.len() && data[i + 1] == 0xff {
                    // Literal 0x00
                    decompressed.push(0x00);
                    i += 2;
                } else if i + 2 < data.len() {
                    // Compressed data: (distance, length)
                    let distance = data[i + 1] as usize;
                    let length = data[i + 2] as usize;
                    
                    if distance == 0 || length == 0 {
                        return Err(CompressionError::InvalidData);
                    }
                    
                    if decompressed.len() < distance {
                        return Err(CompressionError::InvalidData);
                    }
                    
                    // Copy data from earlier in the output
                    let start_pos = decompressed.len() - distance;
                    for j in 0..length {
                        let byte = decompressed[start_pos + (j % distance)];
                        decompressed.push(byte);
                    }
                    i += 3;
                } else {
                    return Err(CompressionError::InvalidData);
                }
            } else {
                // Literal byte
                decompressed.push(data[i]);
                i += 1;
            }
        }

        Ok(decompressed)
    }

    /// Calculate CRC32 (simplified implementation)
    fn calculate_crc32(&self, data: &[u8]) -> u32 {
        let mut crc: u32 = 0xffffffff;
        
        for &byte in data {
            crc ^= byte as u32;
            for _ in 0..8 {
                if crc & 1 == 1 {
                    crc = (crc >> 1) ^ 0xedb88320;
                } else {
                    crc >>= 1;
                }
            }
        }
        
        !crc
    }

    /// Get compression statistics
    pub fn get_stats(&self) -> CompressionStats {
        CompressionStats {
            compression_level: self.compression_level,
            min_size_threshold: self.min_size_threshold,
            compressible_types: self.compressible_types.clone(),
        }
    }
}

impl Default for ResponseCompressor {
    fn default() -> Self {
        Self::new()
    }
}

/// Compression result
#[derive(Debug)]
pub struct CompressionResult {
    pub content: Vec<u8>,
    pub compression_type: CompressionType,
    pub original_size: usize,
    pub compressed_size: usize,
    pub compression_ratio: f64,
}

impl CompressionResult {
    /// Get compression savings percentage
    pub fn savings_percentage(&self) -> f64 {
        if self.original_size == 0 {
            0.0
        } else {
            (1.0 - self.compression_ratio) * 100.0
        }
    }

    /// Check if compression was effective
    pub fn is_effective(&self) -> bool {
        self.compression_ratio < 0.9 // At least 10% reduction
    }

    /// Get headers to add to HTTP response
    pub fn get_headers(&self) -> Vec<(String, String)> {
        let mut headers = Vec::new();
        
        if self.compression_type != CompressionType::Identity {
            headers.push((
                "Content-Encoding".to_string(),
                self.compression_type.header_name().to_string(),
            ));
            headers.push((
                "Vary".to_string(),
                "Accept-Encoding".to_string(),
            ));
        }

        headers.push((
            "Content-Length".to_string(),
            self.compressed_size.to_string(),
        ));

        headers
    }
}

/// Compression errors
#[derive(Debug)]
pub enum CompressionError {
    UnsupportedType,
    InvalidData,
    CompressionFailed(String),
    DecompressionFailed(String),
}

impl std::fmt::Display for CompressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompressionError::UnsupportedType => write!(f, "Unsupported compression type"),
            CompressionError::InvalidData => write!(f, "Invalid compressed data"),
            CompressionError::CompressionFailed(msg) => write!(f, "Compression failed: {}", msg),
            CompressionError::DecompressionFailed(msg) => write!(f, "Decompression failed: {}", msg),
        }
    }
}

impl std::error::Error for CompressionError {}

/// Compression statistics
#[derive(Debug)]
pub struct CompressionStats {
    pub compression_level: u8,
    pub min_size_threshold: usize,
    pub compressible_types: Vec<String>,
}

/// Streaming compressor for large responses
pub struct StreamingCompressor {
    compression_type: CompressionType,
    buffer: Vec<u8>,
    buffer_size: usize,
    compressor: ResponseCompressor,
}

impl StreamingCompressor {
    /// Create new streaming compressor
    pub fn new(compression_type: CompressionType, buffer_size: usize) -> Self {
        Self {
            compression_type,
            buffer: Vec::with_capacity(buffer_size),
            buffer_size,
            compressor: ResponseCompressor::new(),
        }
    }

    /// Add data to compression buffer
    pub fn write(&mut self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        self.buffer.extend_from_slice(data);
        
        if self.buffer.len() >= self.buffer_size {
            self.flush()
        } else {
            Ok(Vec::new())
        }
    }

    /// Flush remaining data
    pub fn flush(&mut self) -> Result<Vec<u8>, CompressionError> {
        if self.buffer.is_empty() {
            return Ok(Vec::new());
        }

        let compressed = self.compressor.compress(&self.buffer, self.compression_type.clone())?;
        self.buffer.clear();
        Ok(compressed)
    }

    /// Finish compression and return final data
    pub fn finish(mut self) -> Result<Vec<u8>, CompressionError> {
        self.flush()
    }
}

/// Middleware for automatic response compression
pub struct CompressionMiddleware {
    compressor: ResponseCompressor,
    enabled: bool,
}

impl CompressionMiddleware {
    /// Create new compression middleware
    pub fn new() -> Self {
        Self {
            compressor: ResponseCompressor::new(),
            enabled: true,
        }
    }

    /// Enable or disable compression
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set compression level
    pub fn with_level(mut self, level: u8) -> Self {
        self.compressor = self.compressor.with_level(level);
        self
    }

    /// Process response for compression
    pub fn process_response(
        &self,
        content: &[u8],
        headers: &mut HashMap<String, String>,
    ) -> Vec<u8> {
        if !self.enabled {
            return content.to_vec();
        }

        let content_type = headers.get("Content-Type")
            .cloned()
            .unwrap_or_else(|| "text/plain".to_string());

        let accept_encoding = headers.get("Accept-Encoding")
            .cloned()
            .unwrap_or_else(|| "identity".to_string());

        let result = self.compressor.compress_response(content, &content_type, &accept_encoding);

        // Update headers
        for (key, value) in result.get_headers() {
            headers.insert(key, value);
        }

        result.content
    }
}

impl Default for CompressionMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_type_parsing() {
        let accept_encoding = "gzip, deflate, br;q=0.8, *;q=0.1";
        let encodings = CompressionType::from_accept_encoding(accept_encoding);

        assert_eq!(encodings.len(), 4);
        assert_eq!(encodings[0].0, CompressionType::Gzip);
        assert_eq!(encodings[0].1, 1.0);
        assert_eq!(encodings[1].0, CompressionType::Deflate);
        assert_eq!(encodings[1].1, 1.0);
    }

    #[test]
    fn test_compression_selection() {
        let compressor = ResponseCompressor::new();
        
        // Test GZIP selection
        let compression = compressor.select_compression("gzip, deflate");
        assert_eq!(compression, CompressionType::Gzip);
        
        // Test fallback to identity
        let compression = compressor.select_compression("br");
        assert_eq!(compression, CompressionType::Identity);
    }

    #[test]
    fn test_should_compress() {
        let compressor = ResponseCompressor::new();
        
        // Should compress large HTML
        let large_html = Vec::from([b'a'; 2048]);
        assert!(compressor.should_compress(&large_html, "text/html"));
        
        // Should not compress small content
        let small_html = Vec::from([b'a'; 100]);
        assert!(!compressor.should_compress(&small_html, "text/html"));
        
        // Should not compress non-compressible content
        let large_binary = Vec::from([b'a'; 2048]);
        assert!(!compressor.should_compress(&large_binary, "image/png"));
    }

    #[test]
    fn test_simple_compression() {
        let compressor = ResponseCompressor::new();
        let test_data = b"hello world hello world hello world";
        
        let compressed = compressor.simple_compress(test_data);
        let decompressed = compressor.simple_decompress(&compressed).unwrap();
        
        assert_eq!(decompressed, test_data);
    }

    #[test]
    fn test_compression_result() {
        let result = CompressionResult {
            content: Vec::from([1, 2, 3]),
            compression_type: CompressionType::Gzip,
            original_size: 100,
            compressed_size: 80,
            compression_ratio: 0.8,
        };

        assert_eq!(result.savings_percentage(), 20.0);
        assert!(result.is_effective());
        
        let headers = result.get_headers();
        assert!(headers.iter().any(|(k, v)| k == "Content-Encoding" && v == "gzip"));
    }

    #[test]
    fn test_streaming_compressor() {
        let mut compressor = StreamingCompressor::new(CompressionType::Gzip, 1024);
        
        // Write small chunk (should not flush)
        let result = compressor.write(b"hello").unwrap();
        assert!(result.is_empty());
        
        // Finish compression
        let final_data = compressor.finish().unwrap();
        assert!(!final_data.is_empty());
    }

    #[test]
    fn test_compression_middleware() {
        let middleware = CompressionMiddleware::new();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        headers.insert("Accept-Encoding".to_string(), "gzip".to_string());

        let content = Vec::from([b'a'; 2048]); // Large content
        let processed = middleware.process_response(&content, &mut headers);

        // Should be compressed if it's effective
        assert!(headers.contains_key("Content-Length"));
    }
}
