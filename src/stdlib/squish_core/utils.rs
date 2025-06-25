/// Utility functions for SquishCore compression
/// 
/// This module provides utilities for format detection, validation,
/// benchmarking, and other compression-related operations.

// use crate::stdlib::squish_core::{SquishError, SquishResult, CompressionStats};
use crate::error::CursedError;
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
        // TODO: implement
    }
    // No specific initialization needed for utilities
}


pub fn decompress(data: &[u8]) -> SquishResult<Vec<u8>> {
//     crate::stdlib::squish_core::gzip::gzip_decompress(data)
}

pub fn compress_with_level(data: &[u8], level: i32) -> SquishResult<Vec<u8>> {
//     crate::stdlib::squish_core::gzip::gzip_compress_level(data, level)
}

pub fn compress_adaptive(data: &[u8]) -> SquishResult<Vec<u8>> {
    let level = if data.len() < 1024 { 1 } else if data.len() < 1024 * 1024 { 6 } else { 9 };
    compress_with_level(data, level)
}

pub fn max_compressed_size(input_size: usize) -> usize {
    input_size + (input_size / 1000) + 12
}

pub fn validate_level_for_algorithm(level: i32, algorithm: &str) -> bool {
    match algorithm {
        "gzip" | "zlib" | "deflate" => level >= 0 && level <= 9,
        "bzip2" => level >= 1 && level <= 9,
        "lzw" => level == 0, // LZW doesn't use levels
        _ => false,
    }
}

pub fn get_file_extension(format: &CompressionFormat) -> &'static str {
    match format {
        CompressionFormat::Gzip => "gz",
        CompressionFormat::Zlib => "z",
        CompressionFormat::Deflate => "deflate",
        CompressionFormat::Bzip2 => "bz2",
        CompressionFormat::Lzw => "lzw",
        CompressionFormat::Unknown => "bin",
    }
}

pub fn get_mime_type(format: &CompressionFormat) -> &'static str {
    match format {
        CompressionFormat::Gzip => "application/gzip",
        CompressionFormat::Zlib => "application/zlib",
        CompressionFormat::Deflate => "application/deflate",
        CompressionFormat::Bzip2 => "application/x-bzip2",
        CompressionFormat::Lzw => "application/x-lzw",
        CompressionFormat::Unknown => "application/octet-stream",
    }
}

/// Validate compression level
pub fn validate_compression_level(level: i32) -> SquishResult<i32> {
    if level < 1 || level > 9 {
        Err(SquishError::InvalidCompressionLevel(level))
    } else {
        Ok(level)
    }
}

/// Convert quality to compression level
pub fn convert_quality_to_level(quality: f32) -> i32 {
    if quality <= 0.0 { 1 }
    else if quality >= 1.0 { 9 }
    else { (quality * 8.0).ceil() as i32 + 1 }
}

/// Determine if parallel compression should be used
pub fn use_parallel_compression(data_size: usize) -> bool {
    data_size > 1024 * 1024 // Use parallel for data > 1MB
}

/// Get optimal chunk size for parallel compression
pub fn get_optimal_chunk_size(data_size: usize, num_threads: usize) -> usize {
    if num_threads <= 1 {
        data_size
    } else {
        let base_chunk = data_size / num_threads;
        let min_chunk = 64 * 1024; // 64KB minimum
        let max_chunk = 16 * 1024 * 1024; // 16MB maximum
        base_chunk.max(min_chunk).min(max_chunk)
    }
}

pub fn supports_streaming(format: &CompressionFormat) -> bool {
    match format {
        CompressionFormat::Gzip | CompressionFormat::Zlib | CompressionFormat::Deflate => true,
        CompressionFormat::Bzip2 | CompressionFormat::Lzw => false,
        CompressionFormat::Unknown => false,
    }
}

pub fn get_recommended_buffer_size(format: &CompressionFormat) -> usize {
    match format {
        CompressionFormat::Gzip | CompressionFormat::Zlib | CompressionFormat::Deflate => 8192,
        CompressionFormat::Bzip2 => 16384,
        CompressionFormat::Lzw => 4096,
        CompressionFormat::Unknown => 8192,
    }
}
