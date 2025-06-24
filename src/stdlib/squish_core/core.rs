/// Core types and interfaces for SquishCore compression
/// 
/// This module provides the fundamental types, traits, and constants used
/// throughout the SquishCore compression system.

use crate::stdlib::squish_core::{SquishError, SquishResult};
use crate::error::Error;
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
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

/// Compression level specification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// No compression (level 0)
    None,
    /// Fastest compression (level 1)
    Fastest,
    /// Fast compression (level 2-3)
    Fast,
    /// Default/balanced compression (level 4-6)
    Default,
    /// Best compression (level 7-9)
    Best,
    /// Custom level (specific numeric value)
    Custom(i32),
}

impl CompressionLevel {
    /// Convert to numeric level for underlying libraries
    pub fn to_numeric(&self) -> i32 {
        match self {
            CompressionLevel::None => 0,
            CompressionLevel::Fastest => 1,
            CompressionLevel::Fast => 3,
            CompressionLevel::Default => 6,
            CompressionLevel::Best => 9,
            CompressionLevel::Custom(level) => *level,
        }
    }
    
    /// Create from numeric level
    pub fn from_numeric(level: i32) -> SquishResult<Self> {
        match level {
            0 => Ok(CompressionLevel::None),
            1 => Ok(CompressionLevel::Fastest),
            2..=3 => Ok(CompressionLevel::Fast),
            4..=6 => Ok(CompressionLevel::Default),
            7..=9 => Ok(CompressionLevel::Best),
            -2..=-1 => Ok(CompressionLevel::Custom(level)), // Special levels
            10..=22 => Ok(CompressionLevel::Custom(level)), // Extended levels
            _ => Err(SquishError::invalid_level(level)),
        }
    }
}

/// Compression statistics and metrics
#[derive(Debug, Clone)]
pub struct CompressionStats {
    /// Input size in bytes
    pub input_size: usize,
    /// Output size in bytes
    pub output_size: usize,
    /// Compression ratio (output_size / input_size)
    pub compression_ratio: f64,
    /// Space saved as percentage
    pub space_saved_percent: f64,
    /// Time taken for compression/decompression
    pub duration: Duration,
    /// Throughput in bytes per second
    pub throughput_bps: f64,
    /// Algorithm used
    pub algorithm: String,
    /// Compression level used
    pub level: Option<i32>,
}

impl CompressionStats {
    /// Create new compression statistics
    pub fn new(
        input_size: usize,
        output_size: usize,
        duration: Duration,
        algorithm: String,
        level: Option<i32>,
    ) -> Self {
        let compression_ratio = if input_size > 0 {
            output_size as f64 / input_size as f64
        } else {
            0.0
        };
        
        let space_saved_percent = if input_size > 0 {
            (1.0 - compression_ratio) * 100.0
        } else {
            0.0
        };
        
        let throughput_bps = if duration.as_secs_f64() > 0.0 {
            input_size as f64 / duration.as_secs_f64()
        } else {
            0.0
        };
        
        Self {
            input_size,
            output_size,
            compression_ratio,
            space_saved_percent,
            duration,
            throughput_bps,
            algorithm,
            level,
        }
    }
    
    /// Format statistics as a human-readable string
    pub fn format_summary(&self) -> String {
        format!(
            "{} compression: {} → {} bytes ({:.1}% saved, {:.2}x ratio) in {:.2}ms ({:.1} MB/s)",
            self.algorithm,
            self.input_size,
            self.output_size,
            self.space_saved_percent,
            self.compression_ratio,
            self.duration.as_millis(),
            self.throughput_bps / 1_000_000.0
        )
    }
}

/// Compression mode for different use cases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMode {
    /// Optimize for speed
    Speed,
    /// Optimize for compression ratio
    Ratio,
    /// Balanced speed and ratio
    Balanced,
    /// Optimize for memory usage
    Memory,
}

// Compression level constants
pub const NO_COMPRESSION: CompressionLevel = CompressionLevel::None;
pub const BEST_SPEED: CompressionLevel = CompressionLevel::Fastest;
pub const BEST_COMPRESSION: CompressionLevel = CompressionLevel::Best;
pub const DEFAULT_COMPRESSION: CompressionLevel = CompressionLevel::Default;
pub const HUFFMAN_ONLY: CompressionLevel = CompressionLevel::Custom(-2);

/// High-level compression function
pub fn compress(data: &[u8], algorithm: &str) -> SquishResult<Vec<u8>> {
    compress_with_level(data, algorithm, DEFAULT_COMPRESSION)
}

/// High-level compression function with level
pub fn compress_with_level(data: &[u8], algorithm: &str, level: CompressionLevel) -> SquishResult<Vec<u8>> {
    match algorithm.to_lowercase().as_str() {
        "gzip" => crate::stdlib::squish_core::gzip_compress_level(data, level),
        "zlib" => crate::stdlib::squish_core::zlib_compress_level(data, level),
        "deflate" | "flate" => crate::stdlib::squish_core::flate_compress_level(data, level),
        "bzip2" => crate::stdlib::squish_core::bzip2_compress_level(data, level),
        _ => Err(SquishError::unsupported_format(format!("Unknown algorithm: {}", algorithm))),
    }
}

/// High-level decompression function
pub fn decompress(data: &[u8], algorithm: &str) -> SquishResult<Vec<u8>> {
    match algorithm.to_lowercase().as_str() {
        "gzip" => crate::stdlib::squish_core::gzip_decompress(data),
        "zlib" => crate::stdlib::squish_core::zlib_decompress(data),
        "deflate" | "flate" => crate::stdlib::squish_core::flate_decompress(data),
        "bzip2" => crate::stdlib::squish_core::bzip2_decompress(data),
        _ => Err(SquishError::unsupported_format(format!("Unknown algorithm: {}", algorithm))),
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
}

/// Timer utility for measuring compression operations
pub struct CompressionTimer {
    start: Instant,
}

impl CompressionTimer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_level_conversion() {
        assert_eq!(CompressionLevel::None.to_numeric(), 0);
        assert_eq!(CompressionLevel::Fastest.to_numeric(), 1);
        assert_eq!(CompressionLevel::Default.to_numeric(), 6);
        assert_eq!(CompressionLevel::Best.to_numeric(), 9);
        assert_eq!(CompressionLevel::Custom(5).to_numeric(), 5);
    }

    #[test]
    fn test_compression_level_from_numeric() {
        assert_eq!(CompressionLevel::from_numeric(0).unwrap(), CompressionLevel::None);
        assert_eq!(CompressionLevel::from_numeric(1).unwrap(), CompressionLevel::Fastest);
        assert_eq!(CompressionLevel::from_numeric(6).unwrap(), CompressionLevel::Default);
        assert_eq!(CompressionLevel::from_numeric(9).unwrap(), CompressionLevel::Best);
        
        assert!(CompressionLevel::from_numeric(-10).is_err());
        assert!(CompressionLevel::from_numeric(100).is_err());
    }

    #[test]
    fn test_compression_stats() {
        let stats = CompressionStats::new(
            1000,
            600,
            Duration::from_millis(100),
            "gzip".to_string(),
            Some(6),
        );
        
        assert_eq!(stats.input_size, 1000);
        assert_eq!(stats.output_size, 600);
        assert_eq!(stats.compression_ratio, 0.6);
        assert_eq!(stats.space_saved_percent, 40.0);
        assert!(stats.throughput_bps > 0.0);
        
        let summary = stats.format_summary();
        assert!(summary.contains("gzip"));
        assert!(summary.contains("1000"));
        assert!(summary.contains("600"));
        assert!(summary.contains("40.0%"));
    }

    #[test]
    fn test_compression_timer() {
        let timer = CompressionTimer::new();
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = timer.elapsed();
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn test_constants() {
        assert_eq!(NO_COMPRESSION, CompressionLevel::None);
        assert_eq!(BEST_SPEED, CompressionLevel::Fastest);
        assert_eq!(BEST_COMPRESSION, CompressionLevel::Best);
        assert_eq!(DEFAULT_COMPRESSION, CompressionLevel::Default);
    }
}
