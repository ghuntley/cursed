/// GZIP compression implementation for SquishCore
/// 
/// This module provides GZIP compression and decompression functionality
/// using the flate2 crate, with CURSED-style interfaces and error handling.

use crate::stdlib::squish_core::{SquishError, SquishResult, CompressionLevel, CompressionStats, CompressionTimer};
use crate::stdlib::squish_core::core::{Reader as SquishReader, Writer as SquishWriter};
use std::io::{Read, Write, BufWriter, BufReader};
use flate2::{Compression, GzBuilder, read::GzDecoder, write::GzEncoder};
use std::time::Instant;

/// GZIP reader that decompresses data on read
pub struct GzipReader<R: Read> {
    inner: GzDecoder<BufReader<R>>,
    bytes_read: usize,
    timer: Instant,
    input_size: usize,
}

impl<R: Read> GzipReader<R> {
    /// Create a new GZIP reader
    pub fn new(reader: R) -> SquishResult<Self> {
        let buffered = BufReader::new(reader);
        let decoder = GzDecoder::new(buffered);
        
        Ok(Self {
            inner: decoder,
            bytes_read: 0,
            timer: Instant::now(),
            input_size: 0,
        })
    }
}

impl<R: Read> Read for GzipReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes = self.inner.read(buf)?;
        self.bytes_read += bytes;
        Ok(bytes)
    }
}

impl<R: Read> SquishReader for GzipReader<R> {
    fn close(&mut self) -> SquishResult<()> {
        // GZIP decoder closes automatically when dropped
        Ok(())
    }
    
    fn stats(&self) -> Option<CompressionStats> {
        Some(CompressionStats::new(
            self.input_size,
            self.bytes_read,
            self.timer.elapsed(),
            "gzip".to_string(),
            None,
        ))
    }
}

/// GZIP writer that compresses data on write
pub struct GzipWriter<W: Write> {
    inner: Option<GzEncoder<BufWriter<W>>>,
    bytes_written: usize,
    uncompressed_size: usize,
    level: CompressionLevel,
    timer: Instant,
}

impl<W: Write> GzipWriter<W> {
    /// Create a new GZIP writer with default compression
    pub fn new(writer: W) -> Self {
        Self::with_level(writer, CompressionLevel::Default)
    }
    
    /// Create a new GZIP writer with specified compression level
    pub fn with_level(writer: W, level: CompressionLevel) -> Self {
        let buffered = BufWriter::new(writer);
        let compression = Compression::new(level.to_numeric() as u32);
        let encoder = GzBuilder::new()
            .write(buffered, compression);
        
        Self {
            inner: Some(encoder),
            bytes_written: 0,
            uncompressed_size: 0,
            level,
            timer: Instant::now(),
        }
    }
}

impl<W: Write> Write for GzipWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(ref mut encoder) = self.inner {
            let bytes = encoder.write(buf)?;
            self.uncompressed_size += bytes;
            Ok(bytes)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "Writer has been closed"
            ))
        }
    }
    
    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(ref mut encoder) = self.inner {
            encoder.flush()
        } else {
            Ok(())
        }
    }
}

impl<W: Write> SquishWriter for GzipWriter<W> {
    fn close(&mut self) -> SquishResult<()> {
        if let Some(encoder) = self.inner.take() {
            let writer = encoder.finish().map_err(SquishError::from)?;
            drop(writer);
        }
        Ok(())
    }
    
    fn flush(&mut self) -> SquishResult<()> {
        Write::flush(self).map_err(SquishError::from)
    }
    
    fn reset(&mut self, writer: Box<dyn Write>) -> SquishResult<()> {
        // Close current encoder
        self.close()?;
        
        // Create new encoder (this is a simplified implementation)
        Err(SquishError::generic("Reset not supported for GZIP writer in this implementation"))
    }
    
    fn stats(&self) -> Option<CompressionStats> {
        Some(CompressionStats::new(
            self.uncompressed_size,
            self.bytes_written,
            self.timer.elapsed(),
            "gzip".to_string(),
            Some(self.level.to_numeric()),
        ))
    }
}

/// Create a new GZIP reader
pub fn NewGzipReader<R: Read>(reader: R) -> SquishResult<GzipReader<R>> {
    GzipReader::new(reader)
}

/// Create a new GZIP writer with default compression
pub fn NewGzipWriter<W: Write>(writer: W) -> GzipWriter<W> {
    GzipWriter::new(writer)
}

/// Create a new GZIP writer with specified compression level
pub fn NewGzipWriterLevel<W: Write>(writer: W, level: CompressionLevel) -> GzipWriter<W> {
    GzipWriter::with_level(writer, level)
}

/// Compress data using GZIP with default compression
pub fn gzip_compress(data: &[u8]) -> SquishResult<Vec<u8>> {
    gzip_compress_level(data, CompressionLevel::Default)
}

/// Compress data using GZIP with specified compression level
pub fn gzip_compress_level(data: &[u8], level: CompressionLevel) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    {
        let mut writer = GzipWriter::with_level(&mut result, level);
        writer.write_all(data).map_err(SquishError::from)?;
        writer.close()?;
    }
    Ok(result)
}

/// Decompress GZIP data
pub fn gzip_decompress(data: &[u8]) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    let cursor = std::io::Cursor::new(data);
    let mut reader = GzipReader::new(cursor)?;
    reader.read_to_end(&mut result).map_err(SquishError::from)?;
    Ok(result)
}

/// Check if data is GZIP format
pub fn is_gzip_data(data: &[u8]) -> bool {
    data.len() >= 2 && data[0] == 0x1f && data[1] == 0x8b
}

/// Get file extension for GZIP files
pub fn file_extension() -> &'static str {
    ".gz"
}

/// Get MIME type for GZIP data
pub fn mime_type() -> &'static str {
    "application/gzip"
}

/// Check if compression level is valid for GZIP
pub fn is_valid_compression_level(level: i32) -> bool {
    level >= 0 && level <= 9 || level == -1
}

/// Initialize GZIP module
pub fn initialize() {
    // No specific initialization needed for GZIP
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_gzip_compress_decompress() {
        let original = b"Hello, World! This is a test of GZIP compression.";
        
        // Compress
        let compressed = gzip_compress(original).expect("Compression should succeed");
        assert!(!compressed.is_empty());
        assert!(compressed.len() < original.len() + 50); // Should be reasonably sized
        
        // Decompress
        let decompressed = gzip_decompress(&compressed).expect("Decompression should succeed");
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_gzip_compression_levels() {
        let data = b"This is test data for compression level testing. ".repeat(20);
        
        let fast = gzip_compress_level(&data, CompressionLevel::Fastest).unwrap();
        let best = gzip_compress_level(&data, CompressionLevel::Best).unwrap();
        
        // Best compression should be smaller than fastest
        assert!(best.len() <= fast.len());
        
        // Both should decompress to original
        assert_eq!(gzip_decompress(&fast).unwrap(), data);
        assert_eq!(gzip_decompress(&best).unwrap(), data);
    }

    #[test]
    fn test_gzip_empty_data() {
        let empty = b"";
        let compressed = gzip_compress(empty).unwrap();
        let decompressed = gzip_decompress(&compressed).unwrap();
        assert_eq!(decompressed, empty);
    }

    #[test]
    fn test_gzip_large_data() {
        let large_data = vec![b'X'; 10000];
        let compressed = gzip_compress(&large_data).unwrap();
        let decompressed = gzip_decompress(&compressed).unwrap();
        assert_eq!(decompressed, large_data);
        
        // Should achieve good compression ratio on repetitive data
        assert!(compressed.len() < large_data.len() / 10);
    }

    #[test]
    fn test_gzip_streaming_compression() {
        let data = b"Streaming test data for GZIP compression.";
        let mut compressed = Vec::new();
        
        {
            let mut writer = NewGzipWriter(&mut compressed);
            writer.write_all(data).unwrap();
            writer.close().unwrap();
        }
        
        let decompressed = gzip_decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_gzip_streaming_decompression() {
        let original = b"Streaming decompression test data.";
        let compressed = gzip_compress(original).unwrap();
        
        let cursor = Cursor::new(compressed);
        let mut reader = NewGzipReader(cursor).unwrap();
        
        let mut result = Vec::new();
        reader.read_to_end(&mut result).unwrap();
        
        assert_eq!(result, original);
    }

    #[test]
    fn test_gzip_writer_with_level() {
        let data = b"Test data for level-specific compression.";
        let mut result = Vec::new();
        
        {
            let mut writer = NewGzipWriterLevel(&mut result, CompressionLevel::Best);
            writer.write_all(data).unwrap();
            writer.close().unwrap();
        }
        
        let decompressed = gzip_decompress(&result).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_is_gzip_data() {
        let gzip_header = vec![0x1f, 0x8b, 0x08, 0x00];
        assert!(is_gzip_data(&gzip_header));
        
        let not_gzip = vec![0x78, 0x9c, 0x00, 0x00];
        assert!(!is_gzip_data(&not_gzip));
        
        let too_short = vec![0x1f];
        assert!(!is_gzip_data(&too_short));
    }

    #[test]
    fn test_gzip_metadata() {
        assert_eq!(file_extension(), ".gz");
        assert_eq!(mime_type(), "application/gzip");
        
        assert!(is_valid_compression_level(0));
        assert!(is_valid_compression_level(9));
        assert!(is_valid_compression_level(-1)); // Default compression
        assert!(!is_valid_compression_level(10));
        assert!(!is_valid_compression_level(-3));
    }

    #[test]
    fn test_gzip_statistics() {
        let data = b"Test data for statistics collection.";
        let mut result = Vec::new();
        
        let mut writer = NewGzipWriter(&mut result);
        writer.write_all(data).unwrap();
        
        if let Some(stats) = writer.stats() {
            assert_eq!(stats.algorithm, "gzip");
            assert!(stats.input_size > 0);
            assert!(stats.duration.as_nanos() > 0);
        }
        
        writer.close().unwrap();
    }

    #[test]
    fn test_gzip_invalid_data() {
        let invalid_data = vec![0x00, 0x01, 0x02, 0x03];
        assert!(gzip_decompress(&invalid_data).is_err());
    }

    #[test]
    fn test_compression_levels() {
        let level_none = CompressionLevel::None;
        let level_fast = CompressionLevel::Fastest;
        let level_best = CompressionLevel::Best;
        
        assert_eq!(level_none.to_numeric(), 0);
        assert_eq!(level_fast.to_numeric(), 1);
        assert_eq!(level_best.to_numeric(), 9);
    }

    #[test]
    fn test_module_initialization() {
        initialize(); // Should not panic
    }
}
