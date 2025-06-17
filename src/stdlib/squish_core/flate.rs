/// Raw DEFLATE compression implementation for SquishCore
/// 
/// This module provides raw DEFLATE compression and decompression functionality
/// without GZIP or ZLIB headers, using the flate2 crate.

use crate::stdlib::squish_core::{SquishError, SquishResult, CompressionLevel, CompressionStats};
use crate::stdlib::squish_core::core::{Reader as SquishReader, Writer as SquishWriter};
use std::io::{Read, Write, BufWriter, BufReader};
use flate2::{Compression, read::DeflateDecoder, write::DeflateEncoder};
use std::time::Instant;

/// DEFLATE reader that decompresses raw deflate data on read
pub struct FlateReader<R: Read> {
    inner: DeflateDecoder<BufReader<R>>,
    bytes_read: usize,
    timer: Instant,
    input_size: usize,
}

impl<R: Read> FlateReader<R> {
    /// Create a new DEFLATE reader
    pub fn new(reader: R) -> SquishResult<Self> {
        let buffered = BufReader::new(reader);
        let decoder = DeflateDecoder::new(buffered);
        
        Ok(Self {
            inner: decoder,
            bytes_read: 0,
            timer: Instant::now(),
            input_size: 0,
        })
    }
}

impl<R: Read> Read for FlateReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes = self.inner.read(buf)?;
        self.bytes_read += bytes;
        Ok(bytes)
    }
}

impl<R: Read> SquishReader for FlateReader<R> {
    fn close(&mut self) -> SquishResult<()> {
        // DEFLATE decoder closes automatically when dropped
        Ok(())
    }
    
    fn stats(&self) -> Option<CompressionStats> {
        Some(CompressionStats::new(
            self.input_size,
            self.bytes_read,
            self.timer.elapsed(),
            "deflate".to_string(),
            None,
        ))
    }
}

/// DEFLATE writer that compresses data to raw deflate format on write
pub struct FlateWriter<W: Write> {
    inner: Option<DeflateEncoder<BufWriter<W>>>,
    bytes_written: usize,
    uncompressed_size: usize,
    level: CompressionLevel,
    timer: Instant,
}

impl<W: Write> FlateWriter<W> {
    /// Create a new DEFLATE writer with default compression
    pub fn new(writer: W) -> Self {
        Self::with_level(writer, CompressionLevel::Default)
    }
    
    /// Create a new DEFLATE writer with specified compression level
    pub fn with_level(writer: W, level: CompressionLevel) -> Self {
        let buffered = BufWriter::new(writer);
        let compression = Compression::new(level.to_numeric() as u32);
        let encoder = DeflateEncoder::new(buffered, compression);
        
        Self {
            inner: Some(encoder),
            bytes_written: 0,
            uncompressed_size: 0,
            level,
            timer: Instant::now(),
        }
    }
}

impl<W: Write> Write for FlateWriter<W> {
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

impl<W: Write> SquishWriter for FlateWriter<W> {
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
        
        // Create new encoder (simplified implementation)
        Err(SquishError::generic("Reset not supported for DEFLATE writer in this implementation"))
    }
    
    fn stats(&self) -> Option<CompressionStats> {
        Some(CompressionStats::new(
            self.uncompressed_size,
            self.bytes_written,
            self.timer.elapsed(),
            "deflate".to_string(),
            Some(self.level.to_numeric()),
        ))
    }
}

/// Create a new DEFLATE reader
pub fn NewFlateReader<R: Read>(reader: R) -> SquishResult<FlateReader<R>> {
    FlateReader::new(reader)
}

/// Create a new DEFLATE writer with default compression
pub fn NewFlateWriter<W: Write>(writer: W) -> FlateWriter<W> {
    FlateWriter::new(writer)
}

/// Create a new DEFLATE writer with specified compression level
pub fn NewFlateWriterLevel<W: Write>(writer: W, level: CompressionLevel) -> FlateWriter<W> {
    FlateWriter::with_level(writer, level)
}

/// Compress data using raw DEFLATE with default compression
pub fn flate_compress(data: &[u8]) -> SquishResult<Vec<u8>> {
    flate_compress_level(data, CompressionLevel::Default)
}

/// Compress data using raw DEFLATE with specified compression level
pub fn flate_compress_level(data: &[u8], level: CompressionLevel) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    {
        let mut writer = FlateWriter::with_level(&mut result, level);
        writer.write_all(data).map_err(SquishError::from)?;
        writer.close()?;
    }
    Ok(result)
}

/// Decompress raw DEFLATE data
pub fn flate_decompress(data: &[u8]) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    let cursor = std::io::Cursor::new(data);
    let mut reader = FlateReader::new(cursor)?;
    reader.read_to_end(&mut result).map_err(SquishError::from)?;
    Ok(result)
}

/// Get file extension for DEFLATE files
pub fn file_extension() -> &'static str {
    ".deflate"
}

/// Get MIME type for DEFLATE data
pub fn mime_type() -> &'static str {
    "application/deflate"
}

/// Check if compression level is valid for DEFLATE
pub fn is_valid_compression_level(level: i32) -> bool {
    level >= 0 && level <= 9 || level == -1
}

/// Initialize DEFLATE module
pub fn initialize() {
    // No specific initialization needed for DEFLATE
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_flate_compress_decompress() {
        let original = b"Hello, World! This is a test of raw DEFLATE compression.";
        
        // Compress
        let compressed = flate_compress(original).expect("Compression should succeed");
        assert!(!compressed.is_empty());
        
        // Decompress
        let decompressed = flate_decompress(&compressed).expect("Decompression should succeed");
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_flate_compression_levels() {
        let data = b"This is test data for DEFLATE compression level testing. ".repeat(20);
        
        let fast = flate_compress_level(&data, CompressionLevel::Fastest).unwrap();
        let best = flate_compress_level(&data, CompressionLevel::Best).unwrap();
        
        // Best compression should be smaller than or equal to fastest
        assert!(best.len() <= fast.len());
        
        // Both should decompress to original
        assert_eq!(flate_decompress(&fast).unwrap(), data);
        assert_eq!(flate_decompress(&best).unwrap(), data);
    }

    #[test]
    fn test_flate_empty_data() {
        let empty = b"";
        let compressed = flate_compress(empty).unwrap();
        let decompressed = flate_decompress(&compressed).unwrap();
        assert_eq!(decompressed, empty);
    }

    #[test]
    fn test_flate_large_data() {
        let large_data = vec![b'D'; 10000];
        let compressed = flate_compress(&large_data).unwrap();
        let decompressed = flate_decompress(&compressed).unwrap();
        assert_eq!(decompressed, large_data);
        
        // Should achieve good compression ratio on repetitive data
        assert!(compressed.len() < large_data.len() / 10);
    }

    #[test]
    fn test_flate_streaming_compression() {
        let data = b"Streaming test data for DEFLATE compression.";
        let mut compressed = Vec::new();
        
        {
            let mut writer = NewFlateWriter(&mut compressed);
            writer.write_all(data).unwrap();
            writer.close().unwrap();
        }
        
        let decompressed = flate_decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_flate_streaming_decompression() {
        let original = b"Streaming decompression test data for DEFLATE.";
        let compressed = flate_compress(original).unwrap();
        
        let cursor = Cursor::new(compressed);
        let mut reader = NewFlateReader(cursor).unwrap();
        
        let mut result = Vec::new();
        reader.read_to_end(&mut result).unwrap();
        
        assert_eq!(result, original);
    }

    #[test]
    fn test_flate_metadata() {
        assert_eq!(file_extension(), ".deflate");
        assert_eq!(mime_type(), "application/deflate");
        
        assert!(is_valid_compression_level(0));
        assert!(is_valid_compression_level(9));
        assert!(is_valid_compression_level(-1));
        assert!(!is_valid_compression_level(10));
    }

    #[test]
    fn test_flate_statistics() {
        let data = b"Test data for statistics collection.";
        let mut result = Vec::new();
        
        let mut writer = NewFlateWriter(&mut result);
        writer.write_all(data).unwrap();
        
        if let Some(stats) = writer.stats() {
            assert_eq!(stats.algorithm, "deflate");
            assert!(stats.input_size > 0);
        }
        
        writer.close().unwrap();
    }

    #[test]
    fn test_module_initialization() {
        initialize(); // Should not panic
    }
}
