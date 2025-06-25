use crate::error::CursedError;
/// Production-ready response compression utilities (gzip, deflate, brotli, zstd)
use std::collections::HashMap;
use std::io::{Read, Write};
use flate2::{Compression, read::{GzDecoder, DeflateDecoder}, write::{GzEncoder, DeflateEncoder}};
use brotli::{CompressorWriter, Decompressor};

/// Compression types supported
#[derive(Debug, Clone, PartialEq)]
pub enum CompressionType {
    Identity, // No compression
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

            let compression_type = match name.to_lowercase().as_str() {

            encodings.push((compression_type, quality));
        // Sort by quality (descending)
        encodings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        encodings
    /// Get the encoding name for HTTP headers
    pub fn header_name(&self) -> &'static str {
        match self {
        }
    }

    /// Check if compression type is supported
    pub fn is_supported(&self) -> bool {
        !matches!(self, CompressionType::Identity)
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
/// Response compressor configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    /// Compression level (0-11, varies by algorithm)
    /// Minimum size threshold for compression
    /// Maximum size threshold for compression (to avoid memory issues)
    /// Compressible content types
    /// Buffer size for streaming compression
    /// Enable/disable specific compression types
    /// Quality threshold for accepting compression
impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
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
            buffer_size: 64 * 1024, // 64KB
            enabled_types: vec![
        }
    }
/// Response compressor with production-ready implementations
pub struct ResponseCompressor {
impl ResponseCompressor {
    /// Create new response compressor with default configuration
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: CompressionConfig) -> Self {
        Self {
        }
    }

    /// Set compression level (0-11, varies by algorithm)
    pub fn with_level(mut self, level: u8) -> Self {
        self.config.level = level.min(11);
        self
    /// Set minimum size threshold for compression
    pub fn with_threshold(mut self, threshold: usize) -> Self {
        self.config.min_size_threshold = threshold;
        self
    /// Set maximum size threshold for compression
    pub fn with_max_threshold(mut self, threshold: usize) -> Self {
        self.config.max_size_threshold = threshold;
        self
    /// Add compressible content type
    pub fn add_compressible_type(mut self, content_type: String) -> Self {
        self.config.compressible_types.push(content_type);
        self
    /// Enable specific compression types
    pub fn enable_types(mut self, types: Vec<CompressionType>) -> Self {
        self.config.enabled_types = types;
        self
    /// Check if content should be compressed
    pub fn should_compress(&self, content: &[u8], content_type: &str) -> bool {
        let content_len = content.len();
        
        // Check size thresholds
        if content_len < self.config.min_size_threshold 
            || content_len > self.config.max_size_threshold {
            return false;
        // Check if content type is compressible
        let content_type_base = content_type.split(';').next().unwrap_or("").trim().to_lowercase();
        self.config.compressible_types.iter()
            .any(|ct| ct.to_lowercase() == content_type_base)
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
    /// Compress content using specified compression type
    pub fn compress(
    ) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        
        let result = match compression_type {

        // Update statistics
        let duration = start_time.elapsed();
        self.stats.update_compression_stats(
        );

        result
    /// Decompress content
    pub fn decompress(
    ) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        
        let result = match compression_type {

        // Update statistics
        let duration = start_time.elapsed();
        self.stats.update_decompression_stats(
        );

        result
    /// Compress response with automatic compression type selection
    pub fn compress_response(
    ) -> CompressionResult {
        // Check if compression should be applied
        if !self.should_compress(content, content_type) {
            return CompressionResult {
        // Select compression type
        let compression_type = self.select_compression(accept_encoding);
        
        // Compress content
        let start_time = std::time::Instant::now();
        match self.compress(content, compression_type.clone()) {
            Ok(compressed) => {
                let compression_time = start_time.elapsed();
                CompressionResult {
                    compression_ratio: compressed.len() as f64 / content.len() as f64,
                }
            Err(_) => CompressionResult {
        }
    }

    /// GZIP compression using flate2
    fn gzip_compress(&self, data: &[u8]) -> crate::error::Result<()> {
        let mut encoder = GzEncoder::new(
            Compression::new(self.config.level.min(9) as u32)
        );
        
        encoder.write_all(data)
            .map_err(|e| CompressionError::CompressionFailed(format!("GZIP compression failed: {}", e)))?;
        
        encoder.finish()
            .map_err(|e| CompressionError::CompressionFailed(format!("GZIP finalization failed: {}", e)))
    /// GZIP decompression using flate2
    fn gzip_decompress(&self, data: &[u8]) -> crate::error::Result<()> {
        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| CompressionError::DecompressionFailed(format!("GZIP decompression failed: {}", e)))?;
        
        Ok(decompressed)
    /// Deflate compression using flate2
    fn deflate_compress(&self, data: &[u8]) -> crate::error::Result<()> {
        let mut encoder = DeflateEncoder::new(
            Compression::new(self.config.level.min(9) as u32)
        );
        
        encoder.write_all(data)
            .map_err(|e| CompressionError::CompressionFailed(format!("Deflate compression failed: {}", e)))?;
        
        encoder.finish()
            .map_err(|e| CompressionError::CompressionFailed(format!("Deflate finalization failed: {}", e)))
    /// Deflate decompression using flate2
    fn deflate_decompress(&self, data: &[u8]) -> crate::error::Result<()> {
        let mut decoder = DeflateDecoder::new(data);
        let mut decompressed = Vec::new();
        
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| CompressionError::DecompressionFailed(format!("Deflate decompression failed: {}", e)))?;
        
        Ok(decompressed)
    /// Brotli compression using brotli crate
    fn brotli_compress(&self, data: &[u8]) -> crate::error::Result<()> {
        let mut compressed = Vec::new();
        let params = brotli::enc::BrotliEncoderParams {
            ..Default::default()
        
        {
            let mut compressor = CompressorWriter::with_params(
                &params
            );
            
            compressor.write_all(data)
                .map_err(|e| CompressionError::CompressionFailed(format!("Brotli compression failed: {}", e)))?;
            
            compressor.flush()
                .map_err(|e| CompressionError::CompressionFailed(format!("Brotli flush failed: {}", e)))?;
        Ok(compressed)
    /// Brotli decompression using brotli crate
    fn brotli_decompress(&self, data: &[u8]) -> crate::error::Result<()> {
        let mut decompressed = Vec::new();
        let mut decompressor = Decompressor::new(data, self.config.buffer_size);
        
        loop {
            let mut buffer = [0u8; 4096];
            match decompressor.read(&mut buffer) {
                Ok(0) => break, // EOF
                Err(e) => return Err(CompressionError::DecompressionFailed(
                    format!("Brotli decompression failed: {}", e)
            }
        }
        
        Ok(decompressed)
    /// Zstandard compression using zstd crate
    fn zstd_compress(&self, data: &[u8]) -> crate::error::Result<()> {
        zstd::encode_all(data, self.config.level.min(22) as i32)
            .map_err(|e| CompressionError::CompressionFailed(format!("Zstd compression failed: {}", e)))
    /// Zstandard decompression using zstd crate
    fn zstd_decompress(&self, data: &[u8]) -> crate::error::Result<()> {
        zstd::decode_all(data)
            .map_err(|e| CompressionError::DecompressionFailed(format!("Zstd decompression failed: {}", e)))
    /// Get compression statistics
    pub fn get_stats(&self) -> &CompressionStats {
        &self.stats
    /// Reset compression statistics
    pub fn reset_stats(&mut self) {
        self.stats = CompressionStats::default();
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
    /// Check if compression was effective with custom threshold
    pub fn is_effective_with_threshold(&self, threshold: f64) -> bool {
        self.compression_ratio < threshold
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
            ));
            headers.push((
            ));
        headers.push((
        ));

        headers
    }
}

/// Compression errors
#[derive(Debug)]
pub enum CompressionError {
// impl std::fmt::Display for CompressionError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             CompressionError::UnsupportedType => write!(f, "Unsupported compression type"),
//             CompressionError::InvalidData => write!(f, "Invalid compressed data"),
//             CompressionError::CompressionFailed(msg) => write!(f, "Compression failed: {}", msg),
//             CompressionError::DecompressionFailed(msg) => write!(f, "Decompression failed: {}", msg),
//             CompressionError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
//             CompressionError::BufferTooSmall => write!(f, "Buffer too small for operation"),
//             CompressionError::SizeLimitExceeded => write!(f, "Size limit exceeded"),
//         }
//     }
// }

// impl std::error::CursedError for CompressionError {}
// 
/// Comprehensive compression statistics
#[derive(Debug, Default)]
pub struct CompressionStats {
#[derive(Debug, Default)]
pub struct AlgorithmStats {
impl CompressionStats {
    fn update_compression_stats(
    ) {
        self.total_compressions += 1;
        self.total_compression_time += duration;
        
        if success {
            self.total_bytes_compressed += input_size as u64;
        } else {
            self.compression_failures += 1;
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
    ) {
        self.total_decompressions += 1;
        self.total_decompression_time += duration;
        
        if success {
            self.total_bytes_decompressed += output_size as u64;
        } else {
            self.decompression_failures += 1;
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
/// Streaming compressor for large responses with memory efficiency
pub struct StreamingCompressor {
impl StreamingCompressor {
    /// Create new streaming compressor
    pub fn new(compression_type: CompressionType, buffer_size: usize) -> Self {
        Self {
        }
    }

    /// Create with custom configuration
    pub fn with_config(compression_type: CompressionType, config: CompressionConfig) -> Self {
        let buffer_size = config.buffer_size;
        Self {
        }
    }

    /// Add data to compression buffer
    pub fn write(&mut self, data: &[u8]) -> crate::error::Result<()> {
        self.buffer.extend_from_slice(data);
        self.total_input += data.len();
        
        if self.buffer.len() >= self.buffer_size {
            self.flush()
        } else {
            Ok(Vec::new())
        }
    }

    /// Flush remaining data
    pub fn flush(&mut self) -> crate::error::Result<()> {
        if self.buffer.is_empty() {
            return Ok(Vec::new());
        let compressed = self.compressor.compress(&self.buffer, self.compression_type.clone())?;
        self.total_output += compressed.len();
        self.buffer.clear();
        Ok(compressed)
    /// Finish compression and return final data
    pub fn finish(mut self) -> crate::error::Result<()> {
        self.flush()
    /// Get compression statistics
    pub fn get_stats(&self) -> (usize, usize, f64) {
        let ratio = if self.total_input == 0 {
            1.0
        } else {
            self.total_output as f64 / self.total_input as f64
        (self.total_input, self.total_output, ratio)
    }
}

/// Middleware for automatic response compression with advanced features
pub struct CompressionMiddleware {
impl CompressionMiddleware {
    /// Create new compression middleware
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: CompressionConfig) -> Self {
        Self {
        }
    }

    /// Enable or disable compression
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    /// Force compression even for small content
    pub fn force_compression(mut self, force: bool) -> Self {
        self.force_compression = force;
        self
    /// Control Vary header addition
    pub fn vary_header(mut self, vary: bool) -> Self {
        self.vary_header = vary;
        self
    /// Set compression level
    pub fn with_level(mut self, level: u8) -> Self {
        self.compressor = self.compressor.with_level(level);
        self
    /// Process response for compression
    pub fn process_response(
    ) -> Vec<u8> {
        if !self.enabled {
            return content.to_vec();
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

        if !should_compress {
            if self.vary_header {
                headers.insert("Vary".to_string(), "Accept-Encoding".to_string());
            }
            return content.to_vec();
        let result = self.compressor.compress_response(content, &content_type, &accept_encoding);

        // Update headers
        for (key, value) in result.get_headers() {
            headers.insert(key, value);
        // Add performance metrics as headers (optional)
        if result.compression_type != CompressionType::Identity {
            headers.insert(
                format!("{:.3}", result.compression_ratio)
            );
            headers.insert(
                format!("{}ms", result.compression_time.as_millis())
            );
        result.content
    /// Get compression statistics
    pub fn get_stats(&self) -> &CompressionStats {
        self.compressor.get_stats()
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
                }
            }

            if successful_iterations > 0 {
                let avg_time = total_time / successful_iterations as u32;
                let avg_size = total_compressed_size / successful_iterations;
                let ratio = avg_size as f64 / data.len() as f64;
                let throughput = data.len() as f64 / avg_time.as_secs_f64() / (1024.0 * 1024.0);

                results.algorithm_results.insert(
                    AlgorithmBenchmark {
                        success_rate: successful_iterations as f64 / iterations as f64,
                    }
                );
            }
        }

        results
    #[derive(Debug, Default)]
    pub struct BenchmarkResults {
    #[derive(Debug)]
    pub struct AlgorithmBenchmark {
    impl BenchmarkResults {
        /// Get the best algorithm for compression ratio
        pub fn best_compression_ratio(&self) -> Option<(&String, &AlgorithmBenchmark)> {
            self.algorithm_results.iter()
                .min_by(|a, b| a.1.compression_ratio.partial_cmp(&b.1.compression_ratio).unwrap())
        /// Get the fastest algorithm
        pub fn fastest_algorithm(&self) -> Option<(&String, &AlgorithmBenchmark)> {
            self.algorithm_results.iter()
                .max_by(|a, b| a.1.throughput_mbps.partial_cmp(&b.1.throughput_mbps).unwrap())
        }
    }

impl CompressionManager {
    /// Create a new compression manager with default settings
    pub fn new() -> Self {
        Self {
            buffer_size: 32768, // 32KB buffer
        }
    }

    /// Create a compression manager with custom level
    pub fn with_level(level: CompressionLevel) -> Self {
        Self {
        }
    }

    /// Set the buffer size for compression operations
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    /// Compress data using the specified algorithm
    pub fn compress(&mut self, data: &[u8], compression_type: CompressionType) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        let original_size = data.len();

        let result = match compression_type {

        match result {
            Ok(compressed) => {
                let compression_time = start_time.elapsed();
                self.stats.update_compression_stats(compression_type, original_size, compressed.len(), compression_time, true);
                Ok(compressed)
            }
            Err(e) => {
                self.stats.update_compression_stats(compression_type, original_size, 0, start_time.elapsed(), false);
                Err(e)
            }
        }
    /// Decompress data using the specified algorithm
    pub fn decompress(&mut self, data: &[u8], compression_type: CompressionType) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();

        let result = match compression_type {

        match result {
            Ok(decompressed) => {
                let decompression_time = start_time.elapsed();
                self.stats.update_decompression_stats(compression_type, data.len(), decompressed.len(), decompression_time, true);
                Ok(decompressed)
            }
            Err(e) => {
                self.stats.update_decompression_stats(compression_type, data.len(), 0, start_time.elapsed(), false);
                Err(e)
            }
        }
    /// Get compression statistics
    pub fn get_stats(&self) -> &CompressionStats {
        &self.stats
    /// Reset compression statistics
    pub fn reset_stats(&mut self) {
        self.stats = CompressionStats::default();
    // Private compression methods
    fn compress_gzip(&self, data: &[u8]) -> crate::error::Result<()> {
        let mut encoder = GzEncoder::new(Vec::new(), self.compression_level.into());
        encoder.write_all(data).map_err(|e| CompressionError::CompressionFailed(e.to_string()))?;
        encoder.finish().map_err(|e| CompressionError::CompressionFailed(e.to_string()))
    fn compress_deflate(&self, data: &[u8]) -> crate::error::Result<()> {
        let mut encoder = DeflateEncoder::new(Vec::new(), self.compression_level.into());
        encoder.write_all(data).map_err(|e| CompressionError::CompressionFailed(e.to_string()))?;
        encoder.finish().map_err(|e| CompressionError::CompressionFailed(e.to_string()))
    fn compress_brotli(&self, data: &[u8]) -> crate::error::Result<()> {
        let mut output = Vec::new();
        let mut compressor = CompressorWriter::new(&mut output, self.buffer_size, 6, 22);
        compressor.write_all(data).map_err(|e| CompressionError::CompressionFailed(e.to_string()))?;
        compressor.flush().map_err(|e| CompressionError::CompressionFailed(e.to_string()))?;
        drop(compressor);
        Ok(output)
    fn compress_zstd(&self, data: &[u8]) -> crate::error::Result<()> {
        // Mock implementation for zstd - in production would use actual zstd library
        let mut output = Vec::with_capacity(data.len());
        output.extend_from_slice(b"ZSTD");
        output.extend_from_slice(&(data.len() as u32).to_le_bytes());
        output.extend_from_slice(data);
        Ok(output)
    fn decompress_gzip(&self, data: &[u8]) -> crate::error::Result<()> {
        let mut decoder = GzDecoder::new(data);
        let mut output = Vec::new();
        decoder.read_to_end(&mut output).map_err(|e| CompressionError::DecompressionFailed(e.to_string()))?;
        Ok(output)
    fn decompress_deflate(&self, data: &[u8]) -> crate::error::Result<()> {
        let mut decoder = DeflateDecoder::new(data);
        let mut output = Vec::new();
        decoder.read_to_end(&mut output).map_err(|e| CompressionError::DecompressionFailed(e.to_string()))?;
        Ok(output)
    fn decompress_brotli(&self, data: &[u8]) -> crate::error::Result<()> {
        let mut decoder = Decompressor::new(data, self.buffer_size);
        let mut output = Vec::new();
        decoder.read_to_end(&mut output).map_err(|e| CompressionError::DecompressionFailed(e.to_string()))?;
        Ok(output)
    fn decompress_zstd(&self, data: &[u8]) -> crate::error::Result<()> {
        // Mock implementation for zstd decompression
        if data.len() < 8 || &data[0..4] != b"ZSTD" {
            return Err(CompressionError::DecompressionFailed("Invalid ZSTD format".to_string()));
        }
        let original_size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
        if data.len() < 8 + original_size {
            return Err(CompressionError::DecompressionFailed("Truncated ZSTD data".to_string()));
        }
        Ok(data[8..8 + original_size].to_vec())
    }
}

impl Default for CompressionManager {
    fn default() -> Self {
        Self::new()
    }
}



/// High-level compression engine for web requests
#[derive(Debug, Clone)]
pub struct CompressionEngine {
impl CompressionEngine {
    /// Create a new compression engine with configuration
    pub fn new(config: CompressionConfig) -> Self {
        Self {
        }
    }

    /// Compress data using configured compression type
    pub fn compress(&mut self, data: &[u8]) -> crate::error::Result<()> {
        self.compressor.compress(data, self.config.compression_type)
    /// Decompress data using detected compression type  
    pub fn decompress(&mut self, data: &[u8]) -> crate::error::Result<()> {
        self.compressor.decompress(data, self.config.compression_type)
    /// Get compression statistics
    pub fn stats(&self) -> &CompressionStats {
        &self.compressor.stats
    }
}
