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
use crate::error::Error;

pub use utils::{
    validate_compression_level as is_valid_compression_level,
    convert_quality_to_level as quality_to_level,
    get_recommended_buffer_size as recommended_buffer_size,
    use_parallel_compression as should_use_parallel,
    get_optimal_chunk_size as optimal_chunk_size,
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
    validate_compression_level, convert_quality_to_level, get_recommended_buffer_size,
    use_parallel_compression, get_optimal_chunk_size
};
pub use statistics::{get_module_stats, cleanup};

// Core interfaces and types
pub use core::{
    Reader, Writer, Compressor, Decompressor,
    CompressionLevel, CompressionStats, CompressionMode, CompressionTimer,
    DEFAULT_COMPRESSION, NO_COMPRESSION, BEST_SPEED, BEST_COMPRESSION, HUFFMAN_ONLY,
    compress, decompress, compress_with_level, decompress_with_validation
};

// Format-specific implementations
pub use gzip::{
    GzipReader, GzipWriter, NewGzipReader, NewGzipWriter, NewGzipWriterLevel,
    gzip_compress, gzip_decompress, gzip_compress_level,
    new_reader as gzip_new_reader, new_writer as gzip_new_writer
};

pub use zlib::{
    ZlibReader, ZlibWriter, NewZlibReader, NewZlibWriter, NewZlibWriterLevel,
    zlib_compress, zlib_decompress, zlib_compress_level,
    new_reader as zlib_new_reader, new_writer as zlib_new_writer, new_writer_level as zlib_new_writer_level
};

pub use flate::{
    FlateReader, FlateWriter, NewFlateReader, NewFlateWriter,
    flate_compress, flate_decompress, flate_compress_level,
    new_reader as flate_new_reader, new_writer as flate_new_writer
};

pub use bzip2::{
    Bzip2Reader, Bzip2Writer, NewBzip2Reader, NewBzip2Writer, NewBzip2WriterLevel,
    bzip2_compress, bzip2_decompress, bzip2_compress_level,
    new_reader as bzip2_new_reader, new_writer as bzip2_new_writer, new_writer_level as bzip2_new_writer_level
};

pub use lzw::{
    LzwReader, LzwWriter, NewLzwReader, NewLzwWriter, LzwOrder,
    lzw_compress, lzw_decompress
};

// Enhanced features
pub use enhanced::{
    EnhancedCompressor, CompressionMode, CompressionOptions,
    fast_compressor, max_compressor, parallel_compressor,
    smart_compress, compress_with_mode, ultra_compress
};

// Utility functions
pub use utils::{
    detect_format, estimate_compression_ratio, benchmark_algorithm,
    validate_compressed_data, get_compression_info, format_compression_stats
};

use std::sync::Once;
static INIT: Once = Once::new();

/// Initialize the squish_core module
pub fn initialize() {
    INIT.call_once(|| {
        // Initialize compression subsystems
        gzip::initialize();
        zlib::initialize();
        flate::initialize();
        bzip2::initialize();
        lzw::initialize();
        enhanced::initialize();
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
    let format = detect_format(data)?;
    match format {
        utils::CompressionFormat::Gzip => gzip_decompress(data),
        utils::CompressionFormat::Zlib => zlib_decompress(data),
        utils::CompressionFormat::Deflate => flate_decompress(data),
        utils::CompressionFormat::Bzip2 => bzip2_decompress(data),
        _ => Err(SquishError::UnsupportedFormat(format!("Format not supported for decompression: {:?}", format)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_initialization() {
        initialize();
        assert_eq!(MODULE_NAME, "squish_core");
        assert_eq!(VERSION, "1.0.0");
    }

    #[test]
    fn test_module_info() {
        let info = module_info();
        assert!(info.contains("squish_core"));
        assert!(info.contains("1.0.0"));
        assert!(info.contains("Compression"));
    }

    #[test]
    fn test_quick_compression_decompression() {
        let test_data = b"Hello, World! This is a test of the compression system.";
        
        // Test compression
        let compressed = squish(test_data).expect("Compression should succeed");
        assert!(!compressed.is_empty());
        
        // Test decompression
        let decompressed = unsquish(&compressed).expect("Decompression should succeed");
        assert_eq!(decompressed, test_data);
    }
}
