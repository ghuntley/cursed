/// Production-ready response compression utilities (gzip, deflate, brotli, zstd)
use std::collections::HashMap;
use std::io::{Read, Write};
use flate2::{Compression, read::{GzDecoder, DeflateDecoder}, write::{GzEncoder, DeflateEncoder}};
use brotli::{CompressorWriter, Decompressor};

/// Compression types supported
#[derive(Debug, Clone, PartialEq)]
pub enum CompressionType {
    Gzip,
    Deflate,
    Brotli,
    Zstd,
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
                "zstd" => CompressionType::Zstd,
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
            CompressionType::Zstd => "zstd",
            CompressionType::Identity => "identity",
        }
    }

    /// Check if compression type is supported
    pub fn is_supported(&self) -> bool {
        !matches!(self, CompressionType::Identity)
    }

    /// Get compression priority (higher = better, 0 = not supported)
    pub fn priority(&self) -> u8 {
        match self {
            CompressionType::Brotli => 10,  // Best compression ratio
            CompressionType::Zstd => 9,     // Fast with good compression
            CompressionType::Gzip => 8,     // Widely supported, good compression
            CompressionType::Deflate => 7,  // Widely supported, basic compression
            CompressionType::Identity => 0, // No compression
        }
    }
}

/// Response compressor configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    /// Compression level (0-11, varies by algorithm)
    pub level: u8,
    /// Minimum size threshold for compression
    pub min_size_threshold: usize,
    /// Maximum size threshold for compression (to avoid memory issues)
    pub max_size_threshold: usize,
    /// Compressible content types
    pub compressible_types: Vec<String>,
    /// Buffer size for streaming compression
    pub buffer_size: usize,
    /// Enable/disable specific compression types
    pub enabled_types: Vec<CompressionType>,
    /// Quality threshold for accepting compression
    pub quality_threshold: f32,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            level: 6,
            min_size_threshold: 1024,          // 1KB
            max_size_threshold: 10 * 1024 * 1024, // 10MB
            compressible_types: vec![
                "text/html".to_string(),
                "text/plain".to_string(),
                "text/css".to_string(),
                "text/javascript".to_string(),
                "application/javascript".to_string(),
                "application/json".to_string(),
                "application/xml".to_string(),
                "text/xml".to_string(),
                "image/svg+xml".to_string(),
                "application/rss+xml".to_string(),
                "application/atom+xml".to_string(),
                "text/csv".to_string(),
                "application/x-font-ttf".to_string(),
                "font/opentype".to_string(),
            ],
            buffer_size: 64 * 1024, // 64KB
            enabled_types: vec![
                CompressionType::Brotli,
                CompressionType::Zstd,
                CompressionType::Gzip,
                CompressionType::Deflate,
            ],
            quality_threshold: 0.1,
        }
    }
}

/// Response compressor with production-ready implementations
pub struct ResponseCompressor {
    config: CompressionConfig,
    stats: CompressionStats,
}

impl ResponseCompressor {
    /// Create new response compressor with default configuration
    pub fn new() -> Self {
        Self {
            config: CompressionConfig::default(),
            stats: CompressionStats::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: CompressionConfig) -> Self {
        Self {
            config,
            stats: CompressionStats::default(),
        }
    }

    /// Set compression level (0-11, varies by algorithm)
    pub fn with_level(mut self, level: u8) -> Self {
        self.config.level = level.min(11);
        self
    }

    /// Set minimum size threshold for compression
    pub fn with_threshold(mut self, threshold: usize) -> Self {
        self.config.min_size_threshold = threshold;
        self
    }

    /// Set maximum size threshold for compression
    pub fn with_max_threshold(mut self, threshold: usize) -> Self {
        self.config.max_size_threshold = threshold;
        self
    }

    /// Add compressible content type
    pub fn add_compressible_type(mut self, content_type: String) -> Self {
        self.config.compressible_types.push(content_type);
        self
    }

    /// Enable specific compression types
    pub fn enable_types(mut self, types: Vec<CompressionType>) -> Self {
        self.config.enabled_types = types;
        self
    }

    /// Check if content should be compressed
    pub fn should_compress(&self, content: &[u8], content_type: &str) -> bool {
        let content_len = content.len();
        
        // Check size thresholds
        if content_len < self.config.min_size_threshold 
            || content_len > self.config.max_size_threshold {
            return false;
        }

        // Check if content type is compressible
        let content_type_base = content_type.split(';').next().unwrap_or("").trim().to_lowercase();
        self.config.compressible_types.iter()
            .any(|ct| ct.to_lowercase() == content_type_base)
    }

    /// Select best compression type from Accept-Encoding header
    pub fn select_compression(&self, accept_encoding: &str) -> CompressionType {
        let encodings = CompressionType::from_accept_encoding(accept_encoding);
        
        for (encoding, quality) in encodings {
            if quality >= self.config.quality_threshold 
                && self.config.enabled_types.contains(&encoding) 
                && encoding.is_supported() {
                return encoding;
            }
        }

        CompressionType::Identity
    }

    /// Compress content using specified compression type
    pub fn compress(
        &mut self,
        content: &[u8],
        compression_type: CompressionType,
    ) -> Result<Vec<u8>, CompressionError> {
        let start_time = std::time::Instant::now();
        
        let result = match compression_type {
            CompressionType::Gzip => self.gzip_compress(content),
            CompressionType::Deflate => self.deflate_compress(content),
            CompressionType::Brotli => self.brotli_compress(content),
            CompressionType::Zstd => self.zstd_compress(content),
            CompressionType::Identity => Ok(content.to_vec()),
        };

        // Update statistics
        let duration = start_time.elapsed();
        self.stats.update_compression_stats(
            compression_type.clone(),
            content.len(),
            result.as_ref().map(|r| r.len()).unwrap_or(0),
            duration,
            result.is_ok(),
        );

        result
    }

    /// Decompress content
    pub fn decompress(
        &mut self,
        content: &[u8],
        compression_type: CompressionType,
    ) -> Result<Vec<u8>, CompressionError> {
        let start_time = std::time::Instant::now();
        
        let result = match compression_type {
            CompressionType::Gzip => self.gzip_decompress(content),
            CompressionType::Deflate => self.deflate_decompress(content),
            CompressionType::Brotli => self.brotli_decompress(content),
            CompressionType::Zstd => self.zstd_decompress(content),
            CompressionType::Identity => Ok(content.to_vec()),
        };

        // Update statistics
        let duration = start_time.elapsed();
        self.stats.update_decompression_stats(
            compression_type,
            content.len(),
            result.as_ref().map(|r| r.len()).unwrap_or(0),
            duration,
            result.is_ok(),
        );

        result
    }

    /// Compress response with automatic compression type selection
    pub fn compress_response(
        &mut self,
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
                compression_time: std::time::Duration::ZERO,
            };
        }

        // Select compression type
        let compression_type = self.select_compression(accept_encoding);
        
        // Compress content
        let start_time = std::time::Instant::now();
        match self.compress(content, compression_type.clone()) {
            Ok(compressed) => {
                let compression_time = start_time.elapsed();
                CompressionResult {
                    content: compressed.clone(),
                    compression_type,
                    original_size: content.len(),
                    compressed_size: compressed.len(),
                    compression_ratio: compressed.len() as f64 / content.len() as f64,
                    compression_time,
                }
            },
            Err(_) => CompressionResult {
                content: content.to_vec(),
                compression_type: CompressionType::Identity,
                original_size: content.len(),
                compressed_size: content.len(),
                compression_ratio: 1.0,
                compression_time: start_time.elapsed(),
            },
        }
    }

    /// GZIP compression using flate2
    fn gzip_compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let mut encoder = GzEncoder::new(
            Vec::new(),
            Compression::new(self.config.level.min(9) as u32)
        );
        
        encoder.write_all(data)
            .map_err(|e| CompressionError::CompressionFailed(format!("GZIP compression failed: {}", e)))?;
        
        encoder.finish()
            .map_err(|e| CompressionError::CompressionFailed(format!("GZIP finalization failed: {}", e)))
    }

    /// GZIP decompression using flate2
    fn gzip_decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| CompressionError::DecompressionFailed(format!("GZIP decompression failed: {}", e)))?;
        
        Ok(decompressed)
    }

    /// Deflate compression using flate2
    fn deflate_compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let mut encoder = DeflateEncoder::new(
            Vec::new(),
            Compression::new(self.config.level.min(9) as u32)
        );
        
        encoder.write_all(data)
            .map_err(|e| CompressionError::CompressionFailed(format!("Deflate compression failed: {}", e)))?;
        
        encoder.finish()
            .map_err(|e| CompressionError::CompressionFailed(format!("Deflate finalization failed: {}", e)))
    }

    /// Deflate decompression using flate2
    fn deflate_decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let mut decoder = DeflateDecoder::new(data);
        let mut decompressed = Vec::new();
        
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| CompressionError::DecompressionFailed(format!("Deflate decompression failed: {}", e)))?;
        
        Ok(decompressed)
    }

    /// Brotli compression using brotli crate
    fn brotli_compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let mut compressed = Vec::new();
        let params = brotli::enc::BrotliEncoderParams {
            quality: self.config.level.min(11) as i32,
            ..Default::default()
        };
        
        {
            let mut compressor = CompressorWriter::with_params(
                &mut compressed,
                self.config.buffer_size,
                &params
            );
            
            compressor.write_all(data)
                .map_err(|e| CompressionError::CompressionFailed(format!("Brotli compression failed: {}", e)))?;
            
            compressor.flush()
                .map_err(|e| CompressionError::CompressionFailed(format!("Brotli flush failed: {}", e)))?;
        }
        
        Ok(compressed)
    }

    /// Brotli decompression using brotli crate
    fn brotli_decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let mut decompressed = Vec::new();
        let mut decompressor = Decompressor::new(data, self.config.buffer_size);
        
        loop {
            let mut buffer = [0u8; 4096];
            match decompressor.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => decompressed.extend_from_slice(&buffer[..n]),
                Err(e) => return Err(CompressionError::DecompressionFailed(
                    format!("Brotli decompression failed: {}", e)
                )),
            }
        }
        
        Ok(decompressed)
    }

    /// Zstandard compression using zstd crate
    fn zstd_compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        zstd::encode_all(data, self.config.level.min(22) as i32)
            .map_err(|e| CompressionError::CompressionFailed(format!("Zstd compression failed: {}", e)))
    }

    /// Zstandard decompression using zstd crate
    fn zstd_decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        zstd::decode_all(data)
            .map_err(|e| CompressionError::DecompressionFailed(format!("Zstd decompression failed: {}", e)))
    }

    /// Get compression statistics
    pub fn get_stats(&self) -> &CompressionStats {
        &self.stats
    }

    /// Reset compression statistics
    pub fn reset_stats(&mut self) {
        self.stats = CompressionStats::default();
    }

    /// Get configuration
    pub fn get_config(&self) -> &CompressionConfig {
        &self.config
    }
}

impl Default for ResponseCompressor {
    fn default() -> Self {
        Self::new()
    }
}

/// Compression result with detailed metrics
#[derive(Debug)]
pub struct CompressionResult {
    pub content: Vec<u8>,
    pub compression_type: CompressionType,
    pub original_size: usize,
    pub compressed_size: usize,
    pub compression_ratio: f64,
    pub compression_time: std::time::Duration,
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

    /// Check if compression was effective (default: at least 10% reduction)
    pub fn is_effective(&self) -> bool {
        self.is_effective_with_threshold(0.9)
    }

    /// Check if compression was effective with custom threshold
    pub fn is_effective_with_threshold(&self, threshold: f64) -> bool {
        self.compression_ratio < threshold
    }

    /// Get compression throughput in MB/s
    pub fn throughput_mbps(&self) -> f64 {
        if self.compression_time.is_zero() {
            0.0
        } else {
            let seconds = self.compression_time.as_secs_f64();
            let mb = self.original_size as f64 / (1024.0 * 1024.0);
            mb / seconds
        }
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
    ConfigurationError(String),
    BufferTooSmall,
    SizeLimitExceeded,
}

impl std::fmt::Display for CompressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompressionError::UnsupportedType => write!(f, "Unsupported compression type"),
            CompressionError::InvalidData => write!(f, "Invalid compressed data"),
            CompressionError::CompressionFailed(msg) => write!(f, "Compression failed: {}", msg),
            CompressionError::DecompressionFailed(msg) => write!(f, "Decompression failed: {}", msg),
            CompressionError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            CompressionError::BufferTooSmall => write!(f, "Buffer too small for operation"),
            CompressionError::SizeLimitExceeded => write!(f, "Size limit exceeded"),
        }
    }
}

impl std::error::Error for CompressionError {}

/// Comprehensive compression statistics
#[derive(Debug, Default)]
pub struct CompressionStats {
    pub total_compressions: u64,
    pub total_decompressions: u64,
    pub total_bytes_compressed: u64,
    pub total_bytes_decompressed: u64,
    pub total_compression_time: std::time::Duration,
    pub total_decompression_time: std::time::Duration,
    pub compression_failures: u64,
    pub decompression_failures: u64,
    pub algorithm_stats: HashMap<String, AlgorithmStats>,
}

#[derive(Debug, Default)]
pub struct AlgorithmStats {
    pub compressions: u64,
    pub decompressions: u64,
    pub input_bytes: u64,
    pub output_bytes: u64,
    pub total_time: std::time::Duration,
    pub failures: u64,
}

impl CompressionStats {
    fn update_compression_stats(
        &mut self,
        algorithm: CompressionType,
        input_size: usize,
        output_size: usize,
        duration: std::time::Duration,
        success: bool,
    ) {
        self.total_compressions += 1;
        self.total_compression_time += duration;
        
        if success {
            self.total_bytes_compressed += input_size as u64;
        } else {
            self.compression_failures += 1;
        }

        let algo_name = algorithm.header_name().to_string();
        let stats = self.algorithm_stats.entry(algo_name).or_default();
        stats.compressions += 1;
        stats.total_time += duration;
        
        if success {
            stats.input_bytes += input_size as u64;
            stats.output_bytes += output_size as u64;
        } else {
            stats.failures += 1;
        }
    }

    fn update_decompression_stats(
        &mut self,
        algorithm: CompressionType,
        input_size: usize,
        output_size: usize,
        duration: std::time::Duration,
        success: bool,
    ) {
        self.total_decompressions += 1;
        self.total_decompression_time += duration;
        
        if success {
            self.total_bytes_decompressed += output_size as u64;
        } else {
            self.decompression_failures += 1;
        }

        let algo_name = algorithm.header_name().to_string();
        let stats = self.algorithm_stats.entry(algo_name).or_default();
        stats.decompressions += 1;
        stats.total_time += duration;
        
        if success {
            stats.input_bytes += input_size as u64;
            stats.output_bytes += output_size as u64;
        } else {
            stats.failures += 1;
        }
    }

    /// Get average compression ratio for all operations
    pub fn average_compression_ratio(&self) -> f64 {
        if self.total_bytes_compressed == 0 {
            1.0
        } else {
            let total_output: u64 = self.algorithm_stats.values()
                .map(|s| s.output_bytes)
                .sum();
            total_output as f64 / self.total_bytes_compressed as f64
        }
    }

    /// Get average compression throughput in MB/s
    pub fn average_compression_throughput(&self) -> f64 {
        if self.total_compression_time.is_zero() {
            0.0
        } else {
            let seconds = self.total_compression_time.as_secs_f64();
            let mb = self.total_bytes_compressed as f64 / (1024.0 * 1024.0);
            mb / seconds
        }
    }

    /// Get compression success rate
    pub fn compression_success_rate(&self) -> f64 {
        if self.total_compressions == 0 {
            0.0
        } else {
            (self.total_compressions - self.compression_failures) as f64 / self.total_compressions as f64
        }
    }
}

/// Streaming compressor for large responses with memory efficiency
pub struct StreamingCompressor {
    compression_type: CompressionType,
    buffer: Vec<u8>,
    buffer_size: usize,
    compressor: ResponseCompressor,
    total_input: usize,
    total_output: usize,
}

impl StreamingCompressor {
    /// Create new streaming compressor
    pub fn new(compression_type: CompressionType, buffer_size: usize) -> Self {
        Self {
            compression_type,
            buffer: Vec::with_capacity(buffer_size),
            buffer_size,
            compressor: ResponseCompressor::new(),
            total_input: 0,
            total_output: 0,
        }
    }

    /// Create with custom configuration
    pub fn with_config(compression_type: CompressionType, config: CompressionConfig) -> Self {
        let buffer_size = config.buffer_size;
        Self {
            compression_type,
            buffer: Vec::with_capacity(buffer_size),
            buffer_size,
            compressor: ResponseCompressor::with_config(config),
            total_input: 0,
            total_output: 0,
        }
    }

    /// Add data to compression buffer
    pub fn write(&mut self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        self.buffer.extend_from_slice(data);
        self.total_input += data.len();
        
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
        self.total_output += compressed.len();
        self.buffer.clear();
        Ok(compressed)
    }

    /// Finish compression and return final data
    pub fn finish(mut self) -> Result<Vec<u8>, CompressionError> {
        self.flush()
    }

    /// Get compression statistics
    pub fn get_stats(&self) -> (usize, usize, f64) {
        let ratio = if self.total_input == 0 {
            1.0
        } else {
            self.total_output as f64 / self.total_input as f64
        };
        (self.total_input, self.total_output, ratio)
    }
}

/// Middleware for automatic response compression with advanced features
pub struct CompressionMiddleware {
    compressor: ResponseCompressor,
    enabled: bool,
    force_compression: bool,
    vary_header: bool,
}

impl CompressionMiddleware {
    /// Create new compression middleware
    pub fn new() -> Self {
        Self {
            compressor: ResponseCompressor::new(),
            enabled: true,
            force_compression: false,
            vary_header: true,
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: CompressionConfig) -> Self {
        Self {
            compressor: ResponseCompressor::with_config(config),
            enabled: true,
            force_compression: false,
            vary_header: true,
        }
    }

    /// Enable or disable compression
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Force compression even for small content
    pub fn force_compression(mut self, force: bool) -> Self {
        self.force_compression = force;
        self
    }

    /// Control Vary header addition
    pub fn vary_header(mut self, vary: bool) -> Self {
        self.vary_header = vary;
        self
    }

    /// Set compression level
    pub fn with_level(mut self, level: u8) -> Self {
        self.compressor = self.compressor.with_level(level);
        self
    }

    /// Process response for compression
    pub fn process_response(
        &mut self,
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

        // Override size check if force compression is enabled
        let should_compress = if self.force_compression {
            true
        } else {
            self.compressor.should_compress(content, &content_type)
        };

        if !should_compress {
            if self.vary_header {
                headers.insert("Vary".to_string(), "Accept-Encoding".to_string());
            }
            return content.to_vec();
        }

        let result = self.compressor.compress_response(content, &content_type, &accept_encoding);

        // Update headers
        for (key, value) in result.get_headers() {
            headers.insert(key, value);
        }

        // Add performance metrics as headers (optional)
        if result.compression_type != CompressionType::Identity {
            headers.insert(
                "X-Compression-Ratio".to_string(),
                format!("{:.3}", result.compression_ratio)
            );
            headers.insert(
                "X-Compression-Time".to_string(),
                format!("{}ms", result.compression_time.as_millis())
            );
        }

        result.content
    }

    /// Get compression statistics
    pub fn get_stats(&self) -> &CompressionStats {
        self.compressor.get_stats()
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.compressor.reset_stats();
    }
}

impl Default for CompressionMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

/// Compression benchmark utilities
pub mod benchmark {
    use super::*;
    use std::time::Instant;

    /// Benchmark compression algorithms
    pub fn benchmark_algorithms(data: &[u8], iterations: usize) -> BenchmarkResults {
        let mut results = BenchmarkResults::default();
        
        for algorithm in &[
            CompressionType::Gzip,
            CompressionType::Deflate,
            CompressionType::Brotli,
            CompressionType::Zstd,
        ] {
            let mut compressor = ResponseCompressor::new();
            let mut total_time = std::time::Duration::ZERO;
            let mut total_compressed_size = 0;
            let mut successful_iterations = 0;

            for _ in 0..iterations {
                let start = Instant::now();
                match compressor.compress(data, algorithm.clone()) {
                    Ok(compressed) => {
                        total_time += start.elapsed();
                        total_compressed_size += compressed.len();
                        successful_iterations += 1;
                    },
                    Err(_) => continue,
                }
            }

            if successful_iterations > 0 {
                let avg_time = total_time / successful_iterations as u32;
                let avg_size = total_compressed_size / successful_iterations;
                let ratio = avg_size as f64 / data.len() as f64;
                let throughput = data.len() as f64 / avg_time.as_secs_f64() / (1024.0 * 1024.0);

                results.algorithm_results.insert(
                    algorithm.header_name().to_string(),
                    AlgorithmBenchmark {
                        compression_ratio: ratio,
                        avg_compression_time: avg_time,
                        throughput_mbps: throughput,
                        success_rate: successful_iterations as f64 / iterations as f64,
                    }
                );
            }
        }

        results
    }

    #[derive(Debug, Default)]
    pub struct BenchmarkResults {
        pub algorithm_results: HashMap<String, AlgorithmBenchmark>,
    }

    #[derive(Debug)]
    pub struct AlgorithmBenchmark {
        pub compression_ratio: f64,
        pub avg_compression_time: std::time::Duration,
        pub throughput_mbps: f64,
        pub success_rate: f64,
    }

    impl BenchmarkResults {
        /// Get the best algorithm for compression ratio
        pub fn best_compression_ratio(&self) -> Option<(&String, &AlgorithmBenchmark)> {
            self.algorithm_results.iter()
                .min_by(|a, b| a.1.compression_ratio.partial_cmp(&b.1.compression_ratio).unwrap())
        }

        /// Get the fastest algorithm
        pub fn fastest_algorithm(&self) -> Option<(&String, &AlgorithmBenchmark)> {
            self.algorithm_results.iter()
                .max_by(|a, b| a.1.throughput_mbps.partial_cmp(&b.1.throughput_mbps).unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_type_parsing() {
        let accept_encoding = "gzip, deflate, br;q=0.8, zstd;q=0.9, *;q=0.1";
        let encodings = CompressionType::from_accept_encoding(accept_encoding);

        assert_eq!(encodings.len(), 5);
        assert_eq!(encodings[0].0, CompressionType::Gzip);
        assert_eq!(encodings[0].1, 1.0);
        assert_eq!(encodings[1].0, CompressionType::Deflate);
        assert_eq!(encodings[1].1, 1.0);
        assert_eq!(encodings[2].0, CompressionType::Zstd);
        assert_eq!(encodings[2].1, 0.9);
    }

    #[test]
    fn test_compression_selection() {
        let compressor = ResponseCompressor::new();
        
        // Test Brotli selection (highest priority)
        let compression = compressor.select_compression("gzip, deflate, br");
        assert_eq!(compression, CompressionType::Brotli);
        
        // Test fallback to GZIP when Brotli not available
        let compression = compressor.select_compression("gzip, deflate");
        assert_eq!(compression, CompressionType::Gzip);
        
        // Test quality-based selection
        let compression = compressor.select_compression("gzip;q=0.5, deflate;q=0.8");
        assert_eq!(compression, CompressionType::Deflate);
    }

    #[test]
    fn test_should_compress() {
        let compressor = ResponseCompressor::new();
        
        // Should compress large HTML
        let large_html = vec![b'a'; 2048];
        assert!(compressor.should_compress(&large_html, "text/html"));
        
        // Should not compress small content
        let small_html = vec![b'a'; 100];
        assert!(!compressor.should_compress(&small_html, "text/html"));
        
        // Should not compress non-compressible content
        let large_binary = vec![b'a'; 2048];
        assert!(!compressor.should_compress(&large_binary, "image/png"));
        
        // Should not compress extremely large content
        let huge_content = vec![b'a'; 20 * 1024 * 1024]; // 20MB
        assert!(!compressor.should_compress(&huge_content, "text/html"));
    }

    #[test]
    fn test_gzip_compression() {
        let mut compressor = ResponseCompressor::new();
        let test_data = b"hello world hello world hello world hello world hello world";
        
        let compressed = compressor.compress(test_data, CompressionType::Gzip).unwrap();
        let decompressed = compressor.decompress(&compressed, CompressionType::Gzip).unwrap();
        
        assert_eq!(decompressed, test_data);
        assert!(compressed.len() < test_data.len());
    }

    #[test]
    fn test_deflate_compression() {
        let mut compressor = ResponseCompressor::new();
        let test_data = b"hello world hello world hello world hello world hello world";
        
        let compressed = compressor.compress(test_data, CompressionType::Deflate).unwrap();
        let decompressed = compressor.decompress(&compressed, CompressionType::Deflate).unwrap();
        
        assert_eq!(decompressed, test_data);
        assert!(compressed.len() < test_data.len());
    }

    #[test]
    fn test_brotli_compression() {
        let mut compressor = ResponseCompressor::new();
        let test_data = b"hello world hello world hello world hello world hello world";
        
        let compressed = compressor.compress(test_data, CompressionType::Brotli).unwrap();
        let decompressed = compressor.decompress(&compressed, CompressionType::Brotli).unwrap();
        
        assert_eq!(decompressed, test_data);
        assert!(compressed.len() < test_data.len());
    }

    #[test]
    fn test_zstd_compression() {
        let mut compressor = ResponseCompressor::new();
        let test_data = b"hello world hello world hello world hello world hello world";
        
        let compressed = compressor.compress(test_data, CompressionType::Zstd).unwrap();
        let decompressed = compressor.decompress(&compressed, CompressionType::Zstd).unwrap();
        
        assert_eq!(decompressed, test_data);
        assert!(compressed.len() < test_data.len());
    }

    #[test]
    fn test_compression_result() {
        let result = CompressionResult {
            content: vec![1, 2, 3],
            compression_type: CompressionType::Brotli,
            original_size: 100,
            compressed_size: 70,
            compression_ratio: 0.7,
            compression_time: std::time::Duration::from_millis(10),
        };

        assert_eq!(result.savings_percentage(), 30.0);
        assert!(result.is_effective());
        assert!(result.throughput_mbps() > 0.0);
        
        let headers = result.get_headers();
        assert!(headers.iter().any(|(k, v)| k == "Content-Encoding" && v == "br"));
        assert!(headers.iter().any(|(k, v)| k == "Content-Length" && v == "70"));
    }

    #[test]
    fn test_streaming_compressor() {
        let mut compressor = StreamingCompressor::new(CompressionType::Gzip, 1024);
        
        // Write small chunks
        let result1 = compressor.write(b"hello ").unwrap();
        assert!(result1.is_empty()); // Should not flush yet
        
        let result2 = compressor.write(b"world ").unwrap();
        assert!(result2.is_empty()); // Still not enough data
        
        // Add enough data to trigger flush
        let large_data = vec![b'x'; 1024];
        let result3 = compressor.write(&large_data).unwrap();
        assert!(!result3.is_empty()); // Should flush now
        
        // Finish compression
        let final_data = compressor.finish().unwrap();
        
        let (input, output, ratio) = compressor.get_stats();
        assert!(input > 0);
        assert!(output > 0);
        assert!(ratio > 0.0);
    }

    #[test]
    fn test_compression_middleware() {
        let mut middleware = CompressionMiddleware::new();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        headers.insert("Accept-Encoding".to_string(), "br, gzip".to_string());

        let content = vec![b'a'; 2048]; // Large content
        let processed = middleware.process_response(&content, &mut headers);

        // Should be compressed
        assert!(processed.len() < content.len());
        assert!(headers.contains_key("Content-Encoding"));
        assert!(headers.contains_key("Content-Length"));
        assert!(headers.contains_key("Vary"));
    }

    #[test]
    fn test_compression_stats() {
        let mut compressor = ResponseCompressor::new();
        let test_data = b"hello world hello world hello world";
        
        // Perform several compressions
        for _ in 0..3 {
            let _ = compressor.compress(test_data, CompressionType::Gzip);
        }
        
        let stats = compressor.get_stats();
        assert_eq!(stats.total_compressions, 3);
        assert!(stats.total_bytes_compressed > 0);
        assert!(stats.total_compression_time > std::time::Duration::ZERO);
        assert!(stats.compression_success_rate() > 0.0);
    }

    #[test]
    fn test_compression_levels() {
        let mut compressor_low = ResponseCompressor::new().with_level(1);
        let mut compressor_high = ResponseCompressor::new().with_level(9);
        
        let test_data = vec![b'a'; 10000]; // Large repeated data
        
        let compressed_low = compressor_low.compress(&test_data, CompressionType::Gzip).unwrap();
        let compressed_high = compressor_high.compress(&test_data, CompressionType::Gzip).unwrap();
        
        // Higher compression level should produce smaller output
        assert!(compressed_high.len() <= compressed_low.len());
    }

    #[test]
    fn test_error_handling() {
        let mut compressor = ResponseCompressor::new();
        
        // Test with invalid/corrupted data
        let invalid_gzip = vec![0x1f, 0x8b, 0x08]; // Truncated GZIP header
        let result = compressor.decompress(&invalid_gzip, CompressionType::Gzip);
        assert!(result.is_err());
        
        // Test with empty data
        let empty_data = vec![];
        let result = compressor.compress(&empty_data, CompressionType::Gzip);
        assert!(result.is_ok());
    }

    #[test]
    fn test_benchmark_functionality() {
        let test_data = vec![b'x'; 1000];
        let results = benchmark::benchmark_algorithms(&test_data, 3);
        
        // Should have results for supported algorithms
        assert!(!results.algorithm_results.is_empty());
        
        // Check if we can find best algorithms
        let best_compression = results.best_compression_ratio();
        let fastest = results.fastest_algorithm();
        
        assert!(best_compression.is_some() || fastest.is_some());
    }
}

/// High-level compression engine for web requests
#[derive(Debug, Clone)]
pub struct CompressionEngine {
    compressor: CompressionManager,
    config: CompressionConfig,
}

impl CompressionEngine {
    /// Create a new compression engine with configuration
    pub fn new(config: CompressionConfig) -> Self {
        Self {
            compressor: CompressionManager::new(),
            config,
        }
    }

    /// Compress data using configured compression type
    pub fn compress(&mut self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        self.compressor.compress(data, self.config.compression_type)
    }

    /// Decompress data using detected compression type  
    pub fn decompress(&mut self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        self.compressor.decompress(data, self.config.compression_type)
    }

    /// Get compression statistics
    pub fn stats(&self) -> &CompressionStats {
        &self.compressor.stats
    }
}
