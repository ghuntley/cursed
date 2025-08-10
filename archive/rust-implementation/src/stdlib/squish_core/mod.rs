/// SquishCore - Compression utilities with Gen Z efficiency 🗜️
/// 
/// This module provides comprehensive compression and decompression functionality
/// for the CURSED programming language, supporting multiple algorithms and formats
/// with both ease of use and high performance.
/// 
/// # Why SquishCore matters:
/// - Essential for data storage and transmission efficiency
/// - Provides type-safe compression operations with CURSED semantics
/// - Includes modern compression patterns with optimized implementations
/// - Supports both streaming and block compression modes

pub mod error;
pub mod core;
pub mod gzip;
pub mod zlib;
pub mod flate;
pub mod bzip2;
pub mod lzw;
pub mod utils;
use crate::error::CursedError;

pub use utils::{
    CompressionUtils, DecompressionUtils
};
pub mod enhanced;
pub mod adaptive;
pub mod constants;
pub mod interfaces;
pub mod statistics;
pub mod parallel;
pub mod progressive;
pub mod dictionary;

// Re-export all public APIs for easy access
pub use error::{SquishError, SquishResult, CompressionError, DecompressionError};

// Constants and types needed by stdlib
pub use constants::{MIN_COMPRESSION_LEVEL, MAX_COMPRESSION_LEVEL, CompressionQuality, CompressionStrategy, FlushMode};
pub use utils::{
    use_parallel_compression, get_optimal_chunk_size
};
pub use statistics::{get_module_stats, cleanup};

// Core interfaces and types
pub use core::{
    compress, decompress, compress_with_level, decompress_with_validation
};

// Format-specific implementations
pub use gzip::{
    new_reader as gzip_new_reader, new_writer as gzip_new_writer
};

pub use zlib::{
    new_reader as zlib_new_reader, new_writer as zlib_new_writer, new_writer_level as zlib_new_writer_level
};

pub use flate::{
    new_reader as flate_new_reader, new_writer as flate_new_writer
};

pub use bzip2::{
    new_reader as bzip2_new_reader, new_writer as bzip2_new_writer, new_writer_level as bzip2_new_writer_level
};

pub use lzw::{
    lzw_compress, lzw_decompress
};

// Enhanced features
pub use enhanced::{
    smart_compress, compress_with_mode, ultra_compress
};

// Utility functions
pub use utils::{
    validate_compressed_data, get_compression_info, format_compression_stats
};

use std::sync::Once;
static INIT: Once = Once::new();

/// Initialize the squish_core module
pub fn initialize() {
    INIT.call_once(|| {
        initialize_compression_system();
    });
}
/// Module version information
pub const VERSION: &str = "1.0.0";
pub const MODULE_NAME: &str = "squish_core";

/// Get module information
pub fn module_info() -> String {
    format!("{} v{} - Compression utilities for CURSED", MODULE_NAME, VERSION)
}

/// Quick compression function with automatic format detection
pub fn squish(data: &[u8]) -> SquishResult<Vec<u8>> {
    smart_compress(data)
}

/// Quick decompression function with automatic format detection  
pub fn unsquish(data: &[u8]) -> SquishResult<Vec<u8>> {
    detect_and_decompress(data)
}

/// Initialize compression system
fn initialize_compression_system() {
    // Initialize internal state for compression modules
    println!("SquishCore compression system initialized");
}

/// Detect compression format and decompress
fn detect_and_decompress(data: &[u8]) -> SquishResult<Vec<u8>> {
    if data.len() < 2 {
        return Err(SquishError::InvalidData("Data too short to determine format".to_string()));
    }
    
    // Try ZLIB first (most common)  
    if zlib::is_zlib_data(data) {
        return zlib::zlib_decompress(data);
    }
    
    // Try GZIP
    if data.starts_with(&[0x1f, 0x8b]) {
        return gzip_decompress(data);
    }
    
    // Try to decompress as raw deflate
    match flate_decompress(data) {
        Ok(result) => return Ok(result),
        Err(_) => {}
    }
    
    Err(SquishError::UnsupportedFormat("Unknown compression format".to_string()))
}

/// Basic GZIP decompression
fn gzip_decompress(data: &[u8]) -> SquishResult<Vec<u8>> {
    use flate2::read::GzDecoder;
    use std::io::Read;
    
    let mut decoder = GzDecoder::new(data);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result)
        .map_err(|e| SquishError::DecompressionError(e.to_string()))?;
    
    Ok(result)
}

/// Basic FLATE decompression
fn flate_decompress(data: &[u8]) -> SquishResult<Vec<u8>> {
    use flate2::read::DeflateDecoder;
    use std::io::Read;
    
    let mut decoder = DeflateDecoder::new(data);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result)
        .map_err(|e| SquishError::DecompressionError(e.to_string()))?;
    
    Ok(result)
}

