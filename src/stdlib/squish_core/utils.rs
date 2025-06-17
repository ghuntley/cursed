/// Utility functions for SquishCore compression
/// 
/// This module provides utilities for format detection, validation,
/// benchmarking, and other compression-related operations.

use crate::stdlib::squish_core::{SquishError, SquishResult, CompressionStats};
use std::time::{Duration, Instant};

/// Supported compression formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionFormat {
    Gzip,
    Zlib,
    Deflate,
    Bzip2,
    Lzw,
    Unknown,
}

impl CompressionFormat {
    /// Get the file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            CompressionFormat::Gzip => ".gz",
            CompressionFormat::Zlib => ".zlib",
            CompressionFormat::Deflate => ".deflate",
            CompressionFormat::Bzip2 => ".bz2",
            CompressionFormat::Lzw => ".lzw",
            CompressionFormat::Unknown => "",
        }
    }
    
    /// Get the MIME type for this format
    pub fn mime_type(&self) -> &'static str {
        match self {
            CompressionFormat::Gzip => "application/gzip",
            CompressionFormat::Zlib => "application/zlib",
            CompressionFormat::Deflate => "application/deflate",
            CompressionFormat::Bzip2 => "application/x-bzip2",
            CompressionFormat::Lzw => "application/x-lzw",
            CompressionFormat::Unknown => "application/octet-stream",
        }
    }
}

/// Detect compression format from data header
pub fn detect_format(data: &[u8]) -> SquishResult<CompressionFormat> {
    if data.is_empty() {
        return Err(SquishError::invalid_input("Empty data cannot be format-detected"));
    }
    
    // GZIP magic number: 0x1f, 0x8b
    if data.len() >= 2 && data[0] == 0x1f && data[1] == 0x8b {
        return Ok(CompressionFormat::Gzip);
    }
    
    // ZLIB magic numbers: 0x78 followed by various values
    if data.len() >= 2 && data[0] == 0x78 {
        match data[1] {
            0x01 | 0x5e | 0x9c | 0xda => return Ok(CompressionFormat::Zlib),
            _ => {}
        }
    }
    
    // BZIP2 magic number: "BZ" (0x42, 0x5a)
    if data.len() >= 2 && data[0] == 0x42 && data[1] == 0x5a {
        return Ok(CompressionFormat::Bzip2);
    }
    
    // For DEFLATE and LZW, we can't reliably detect from headers alone
    // These would require more complex heuristics or external context
    
    Ok(CompressionFormat::Unknown)
}

/// Estimate compression ratio for given data and algorithm
pub fn estimate_compression_ratio(data: &[u8], format: CompressionFormat) -> f64 {
    if data.is_empty() {
        return 1.0;
    }
    
    // Simple heuristics based on data entropy and format capabilities
    let entropy = calculate_entropy(data);
    
    match format {
        CompressionFormat::Gzip | CompressionFormat::Zlib | CompressionFormat::Deflate => {
            // DEFLATE-based algorithms, good for most data types
            if entropy < 0.5 {
                0.1 // Very compressible (repeated data)
            } else if entropy < 1.0 {
                0.3 // Moderately compressible (text)
            } else if entropy < 1.5 {
                0.6 // Somewhat compressible (structured data)
            } else {
                0.9 // Low compressibility (random/encrypted data)
            }
        },
        CompressionFormat::Bzip2 => {
            // BZIP2 generally better than DEFLATE but slower
            if entropy < 0.5 {
                0.08
            } else if entropy < 1.0 {
                0.25
            } else if entropy < 1.5 {
                0.55
            } else {
                0.85
            }
        },
        CompressionFormat::Lzw => {
            // LZW good for repetitive data
            if entropy < 0.5 {
                0.12
            } else if entropy < 1.0 {
                0.35
            } else {
                0.8
            }
        },
        CompressionFormat::Unknown => 1.0,
    }
}

/// Calculate simple entropy measure for data
fn calculate_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    
    let mut counts = [0u32; 256];
    for &byte in data {
        counts[byte as usize] += 1;
    }
    
    let len = data.len() as f64;
    let mut entropy = 0.0;
    
    for count in counts.iter() {
        if *count > 0 {
            let p = *count as f64 / len;
            entropy -= p * p.log2();
        }
    }
    
    entropy / 8.0 // Normalize to 0-1 range approximately
}

/// Benchmark compression algorithm performance
pub fn benchmark_algorithm<F>(name: &str, data: &[u8], operation: F) -> SquishResult<CompressionStats>
where
    F: FnOnce(&[u8]) -> SquishResult<Vec<u8>>,
{
    let start = Instant::now();
    let result = operation(data)?;
    let duration = start.elapsed();
    
    Ok(CompressionStats::new(
        data.len(),
        result.len(),
        duration,
        name.to_string(),
        None,
    ))
}

/// Validate compressed data integrity without full decompression
pub fn validate_compressed_data(data: &[u8], format: CompressionFormat) -> SquishResult<bool> {
    // Check magic numbers and basic structure
    match format {
        CompressionFormat::Gzip => {
            validate_gzip_header(data)
        },
        CompressionFormat::Zlib => {
            validate_zlib_header(data)
        },
        CompressionFormat::Bzip2 => {
            validate_bzip2_header(data)
        },
        _ => {
            // For other formats, just check if we can detect the format
            match detect_format(data) {
                Ok(detected) => Ok(detected == format),
                Err(_) => Ok(false),
            }
        }
    }
}

fn validate_gzip_header(data: &[u8]) -> SquishResult<bool> {
    if data.len() < 10 {
        return Ok(false);
    }
    
    // Check magic number
    if data[0] != 0x1f || data[1] != 0x8b {
        return Ok(false);
    }
    
    // Check compression method (should be 8 for deflate)
    if data[2] != 8 {
        return Ok(false);
    }
    
    // Basic header structure validation passed
    Ok(true)
}

fn validate_zlib_header(data: &[u8]) -> SquishResult<bool> {
    if data.len() < 2 {
        return Ok(false);
    }
    
    // Check magic number combinations
    if data[0] == 0x78 {
        match data[1] {
            0x01 | 0x5e | 0x9c | 0xda => Ok(true),
            _ => Ok(false),
        }
    } else {
        Ok(false)
    }
}

fn validate_bzip2_header(data: &[u8]) -> SquishResult<bool> {
    if data.len() < 4 {
        return Ok(false);
    }
    
    // Check magic number "BZh"
    Ok(data[0] == 0x42 && data[1] == 0x5a && data[2] == b'h')
}

/// Get compression information from data
pub fn get_compression_info(data: &[u8]) -> SquishResult<CompressionInfo> {
    let format = detect_format(data)?;
    let estimated_ratio = estimate_compression_ratio(data, format);
    let is_valid = validate_compressed_data(data, format)?;
    
    Ok(CompressionInfo {
        format,
        estimated_ratio,
        is_valid,
        original_size: data.len(),
        header_info: extract_header_info(data, format)?,
    })
}

/// Compression information structure
#[derive(Debug, Clone)]
pub struct CompressionInfo {
    pub format: CompressionFormat,
    pub estimated_ratio: f64,
    pub is_valid: bool,
    pub original_size: usize,
    pub header_info: Option<HeaderInfo>,
}

/// Header information for compressed data
#[derive(Debug, Clone)]
pub struct HeaderInfo {
    pub compression_method: Option<u8>,
    pub flags: Option<u8>,
    pub timestamp: Option<u32>,
    pub filename: Option<String>,
    pub comment: Option<String>,
}

fn extract_header_info(data: &[u8], format: CompressionFormat) -> SquishResult<Option<HeaderInfo>> {
    match format {
        CompressionFormat::Gzip => extract_gzip_header_info(data),
        _ => Ok(None), // Other formats don't have extractable header info in this implementation
    }
}

fn extract_gzip_header_info(data: &[u8]) -> SquishResult<Option<HeaderInfo>> {
    if data.len() < 10 {
        return Ok(None);
    }
    
    let compression_method = Some(data[2]);
    let flags = Some(data[3]);
    
    // Extract timestamp (bytes 4-7, little endian)
    let timestamp = if data.len() >= 8 {
        Some(u32::from_le_bytes([data[4], data[5], data[6], data[7]]))
    } else {
        None
    };
    
    // For simplicity, we're not extracting filename/comment in this implementation
    // Real implementation would parse variable-length fields based on flags
    
    Ok(Some(HeaderInfo {
        compression_method,
        flags,
        timestamp,
        filename: None,
        comment: None,
    }))
}

/// Format compression statistics for display
pub fn format_compression_stats(stats: &CompressionStats) -> String {
    format!(
        "Algorithm: {}\n\
         Input size: {} bytes\n\
         Output size: {} bytes\n\
         Compression ratio: {:.3}\n\
         Space saved: {:.1}%\n\
         Time taken: {:.2}ms\n\
         Throughput: {:.1} MB/s",
        stats.algorithm,
        stats.input_size,
        stats.output_size,
        stats.compression_ratio,
        stats.space_saved_percent,
        stats.duration.as_millis(),
        stats.throughput_bps / 1_000_000.0
    )
}

/// Initialize utilities module
pub fn initialize() {
    // No specific initialization needed for utilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_gzip_format() {
        let gzip_header = vec![0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(detect_format(&gzip_header).unwrap(), CompressionFormat::Gzip);
    }

    #[test]
    fn test_detect_zlib_format() {
        let zlib_header = vec![0x78, 0x9c];
        assert_eq!(detect_format(&zlib_header).unwrap(), CompressionFormat::Zlib);
    }

    #[test]
    fn test_detect_bzip2_format() {
        let bzip2_header = vec![0x42, 0x5a, b'h', b'9'];
        assert_eq!(detect_format(&bzip2_header).unwrap(), CompressionFormat::Bzip2);
    }

    #[test]
    fn test_detect_unknown_format() {
        let unknown_data = vec![0x00, 0x01, 0x02, 0x03];
        assert_eq!(detect_format(&unknown_data).unwrap(), CompressionFormat::Unknown);
    }

    #[test]
    fn test_empty_data_detection() {
        assert!(detect_format(&[]).is_err());
    }

    #[test]
    fn test_compression_format_extensions() {
        assert_eq!(CompressionFormat::Gzip.extension(), ".gz");
        assert_eq!(CompressionFormat::Zlib.extension(), ".zlib");
        assert_eq!(CompressionFormat::Bzip2.extension(), ".bz2");
    }

    #[test]
    fn test_compression_format_mime_types() {
        assert_eq!(CompressionFormat::Gzip.mime_type(), "application/gzip");
        assert_eq!(CompressionFormat::Zlib.mime_type(), "application/zlib");
        assert_eq!(CompressionFormat::Bzip2.mime_type(), "application/x-bzip2");
    }

    #[test]
    fn test_estimate_compression_ratio() {
        // Highly repetitive data should compress well
        let repetitive_data = vec![b'A'; 1000];
        let ratio = estimate_compression_ratio(&repetitive_data, CompressionFormat::Gzip);
        assert!(ratio < 0.5); // Should compress to less than 50%
        
        // Random data should not compress well
        let random_data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        let ratio = estimate_compression_ratio(&random_data, CompressionFormat::Gzip);
        assert!(ratio > 0.5); // Should not compress much
    }

    #[test]
    fn test_validate_gzip_header() {
        let valid_gzip = vec![0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff];
        assert!(validate_compressed_data(&valid_gzip, CompressionFormat::Gzip).unwrap());
        
        let invalid_gzip = vec![0x1f, 0x8b, 0x09]; // Wrong compression method
        assert!(!validate_compressed_data(&invalid_gzip, CompressionFormat::Gzip).unwrap());
    }

    #[test]
    fn test_calculate_entropy() {
        // All same bytes should have low entropy
        let uniform_data = vec![0u8; 100];
        let entropy = calculate_entropy(&uniform_data);
        assert!(entropy < 0.1);
        
        // Mixed data should have higher entropy
        let mixed_data: Vec<u8> = (0..256).map(|i| i as u8).collect();
        let entropy = calculate_entropy(&mixed_data);
        assert!(entropy > 0.5);
    }

    #[test]
    fn test_compression_info() {
        let gzip_data = vec![0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff];
        let info = get_compression_info(&gzip_data).unwrap();
        
        assert_eq!(info.format, CompressionFormat::Gzip);
        assert!(info.is_valid);
        assert_eq!(info.original_size, gzip_data.len());
        assert!(info.header_info.is_some());
    }

    #[test]
    fn test_format_compression_stats() {
        let stats = CompressionStats::new(
            1000,
            600,
            Duration::from_millis(50),
            "test".to_string(),
            Some(6),
        );
        
        let formatted = format_compression_stats(&stats);
        assert!(formatted.contains("test"));
        assert!(formatted.contains("1000"));
        assert!(formatted.contains("600"));
        assert!(formatted.contains("40.0%"));
    }
}
