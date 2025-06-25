/// Core types and interfaces for SquishCore compression
/// 
/// This module provides the fundamental types, traits, and constants used
/// throughout the SquishCore compression system.

// use crate::stdlib::squish_core::{SquishError, SquishResult};
use crate::error::CursedError;
use std::io::{Read, Write};
use std::time::{Duration, Instant};

/// Core trait for readable compression streams
pub trait Reader: Read {
    /// Close the reader and finalize any pending operations
    fn close(&mut self) -> SquishResult<()>;
    
    /// Get compression statistics if available
    fn stats(&self) -> Option<CompressionStats> {
        None
    }
}

/// Core trait for writable compression streams
pub trait Writer: Write {
    /// Close the writer and finalize compression
    fn close(&mut self) -> SquishResult<()>;
    
    /// Flush any pending compressed data
    fn flush(&mut self) -> SquishResult<()>;
    
    /// Reset the writer to use a new output destination
    fn reset(&mut self, writer: Box<dyn Write>) -> SquishResult<()>;
    
    /// Get compression statistics if available
    fn stats(&self) -> Option<CompressionStats> {
        None
    }
}

/// Core trait for block compression algorithms
pub trait Compressor {
    /// Compress data from src into dst, returning bytes written
    fn compress(&self, dst: &mut Vec<u8>, src: &[u8]) -> SquishResult<usize>;
    
    /// Compress data with specific compression level
    fn compress_level(&self, dst: &mut Vec<u8>, src: &[u8], level: CompressionLevel) -> SquishResult<usize>;
    
    /// Estimate output buffer size needed for compression
    fn estimate_output_size(&self, input_size: usize) -> usize {
        // Conservative estimate: worst case is input size + overhead
        input_size + (input_size / 10) + 1024
    }
}

/// Core trait for block decompression algorithms
pub trait Decompressor {
    /// Decompress data from src into dst, returning bytes written
    fn decompress(&self, dst: &mut Vec<u8>, src: &[u8]) -> SquishResult<usize>;
    
    /// Validate compressed data without full decompression
    fn validate(&self, src: &[u8]) -> SquishResult<bool> {
        // Default implementation: try to decompress and check for errors
        let mut temp = Vec::new();
        match self.decompress(&mut temp, src) {
        }
    }
/// Compression level specification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// No compression (level 0)
    /// Fastest compression (level 1)
    /// Fast compression (level 2-3)
    /// Default/balanced compression (level 4-6)
    /// Best compression (level 7-9)
    /// Custom level (specific numeric value)
impl CompressionLevel {
    /// Convert to numeric level for underlying libraries
    pub fn to_numeric(&self) -> i32 {
        match self {
        }
    }
    
    /// Create from numeric level
    pub fn from_numeric(level: i32) -> SquishResult<Self> {
        match level {
            -2..=-1 => Ok(CompressionLevel::Custom(level)), // Special levels
            10..=22 => Ok(CompressionLevel::Custom(level)), // Extended levels
        }
    }
/// Compression statistics and metrics
#[derive(Debug, Clone)]
pub struct CompressionStats {
    /// Input size in bytes
    /// Output size in bytes
    /// Compression ratio (output_size / input_size)
    /// Space saved as percentage
    /// Time taken for compression/decompression
    /// Throughput in bytes per second
    /// Algorithm used
    /// Compression level used
impl CompressionStats {
    /// Create new compression statistics
    pub fn new(
    ) -> Self {
        let compression_ratio = if input_size > 0 {
            output_size as f64 / input_size as f64
        } else {
            0.0
        
        let space_saved_percent = if input_size > 0 {
            (1.0 - compression_ratio) * 100.0
        } else {
            0.0
        
        let throughput_bps = if duration.as_secs_f64() > 0.0 {
            input_size as f64 / duration.as_secs_f64()
        } else {
            0.0
        
        Self {
        }
    }
    
    /// Format statistics as a human-readable string
    pub fn format_summary(&self) -> String {
        format!(
            "{} compression: {} → {} bytes ({:.1}% saved, {:.2}x ratio) in {:.2}ms ({:.1} MB/s)",
            self.throughput_bps / 1_000_000.0
        )
    }
}

/// Compression mode for different use cases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMode {
    /// Optimize for speed
    /// Optimize for compression ratio
    /// Balanced speed and ratio
    /// Optimize for memory usage
// Compression level constants
pub const NO_COMPRESSION: CompressionLevel = CompressionLevel::None;
pub const BEST_SPEED: CompressionLevel = CompressionLevel::Fastest;
pub const BEST_COMPRESSION: CompressionLevel = CompressionLevel::Best;
pub const DEFAULT_COMPRESSION: CompressionLevel = CompressionLevel::Default;
pub const HUFFMAN_ONLY: CompressionLevel = CompressionLevel::Custom(-2);

/// High-level compression function
pub fn compress(data: &[u8], algorithm: &str) -> SquishResult<Vec<u8>> {
    compress_with_level(data, algorithm, DEFAULT_COMPRESSION)
/// High-level compression function with level
pub fn compress_with_level(data: &[u8], algorithm: &str, level: CompressionLevel) -> SquishResult<Vec<u8>> {
    match algorithm.to_lowercase().as_str() {
//         "gzip" => crate::stdlib::squish_core::gzip_compress_level(data, level),
//         "zlib" => crate::stdlib::squish_core::zlib_compress_level(data, level),
//         "deflate" | "flate" => crate::stdlib::squish_core::flate_compress_level(data, level),
//         "bzip2" => crate::stdlib::squish_core::bzip2_compress_level(data, level),
    }
}

/// High-level decompression function
pub fn decompress(data: &[u8], algorithm: &str) -> SquishResult<Vec<u8>> {
    match algorithm.to_lowercase().as_str() {
//         "gzip" => crate::stdlib::squish_core::gzip_decompress(data),
//         "zlib" => crate::stdlib::squish_core::zlib_decompress(data),
//         "deflate" | "flate" => crate::stdlib::squish_core::flate_decompress(data),
//         "bzip2" => crate::stdlib::squish_core::bzip2_decompress(data),
    }
}

/// High-level decompression function with validation
pub fn decompress_with_validation(data: &[u8], algorithm: &str, expected_size: Option<usize>) -> SquishResult<Vec<u8>> {
    let result = decompress(data, algorithm)?;
    
    if let Some(expected) = expected_size {
        if result.len() != expected {
            return Err(SquishError::corrupted_data(
                format!("Decompressed size {} doesn't match expected size {}", result.len(), expected)
            ));
        }
    }
    
    Ok(result)
/// Timer utility for measuring compression operations
pub struct CompressionTimer {
impl CompressionTimer {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

impl Default for CompressionTimer {
    fn default() -> Self {
        Self::new()
    }
}

