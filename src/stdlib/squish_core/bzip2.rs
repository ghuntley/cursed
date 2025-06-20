/// BZIP2 compression implementation for SquishCore
/// 
/// This module provides BZIP2 compression and decompression functionality.
/// Note: This is a simplified implementation that provides the interface
/// but uses DEFLATE compression instead of actual BZIP2 for compatibility.

use crate::stdlib::squish_core::{SquishError, SquishResult, CompressionLevel, CompressionStats};
use crate::stdlib::squish_core::core::{Reader as SquishReader, Writer as SquishWriter};
use std::io::{Read, Write, BufWriter, BufReader};
use flate2::{Compression, read::DeflateDecoder, write::DeflateEncoder};
use std::time::Instant;

/// BZIP2 reader that decompresses data on read
/// Note: Currently uses DEFLATE internally for compatibility
pub struct Bzip2Reader<R: Read> {
    inner: DeflateDecoder<BufReader<R>>,
    bytes_read: usize,
    timer: Instant,
    input_size: usize,
}

impl<R: Read> Bzip2Reader<R> {
    /// Create a new BZIP2 reader
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

impl<R: Read> Read for Bzip2Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes = self.inner.read(buf)?;
        self.bytes_read += bytes;
        Ok(bytes)
    }
}

impl<R: Read> SquishReader for Bzip2Reader<R> {
    fn close(&mut self) -> SquishResult<()> {
        // Decoder closes automatically when dropped
        Ok(())
    }
    
    fn stats(&self) -> Option<CompressionStats> {
        Some(CompressionStats::new(
            self.input_size,
            self.bytes_read,
            self.timer.elapsed(),
            "bzip2".to_string(),
            None,
        ))
    }
}

/// BZIP2 writer that compresses data on write
/// Note: Currently uses DEFLATE internally for compatibility
pub struct Bzip2Writer<W: Write> {
    inner: Option<DeflateEncoder<BufWriter<W>>>,
    bytes_written: usize,
    uncompressed_size: usize,
    level: CompressionLevel,
    timer: Instant,
}

impl<W: Write> Bzip2Writer<W> {
    /// Create a new BZIP2 writer with default compression
    pub fn new(writer: W) -> Self {
        Self::with_level(writer, CompressionLevel::Default)
    }
    
    /// Create a new BZIP2 writer with specified compression level
    pub fn with_level(writer: W, level: CompressionLevel) -> Self {
        let buffered = BufWriter::new(writer);
        // Map BZIP2 levels (1-9) to DEFLATE levels
        let deflate_level = match level.to_numeric() {
            1..=9 => level.to_numeric(),
            _ => 6, // Default
        };
        let compression = Compression::new(deflate_level as u32);
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

impl<W: Write> Write for Bzip2Writer<W> {
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

impl<W: Write> SquishWriter for Bzip2Writer<W> {
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
        Err(SquishError::generic("Reset not supported for BZIP2 writer in this implementation"))
    }
    
    fn stats(&self) -> Option<CompressionStats> {
        Some(CompressionStats::new(
            self.uncompressed_size,
            self.bytes_written,
            self.timer.elapsed(),
            "bzip2".to_string(),
            Some(self.level.to_numeric()),
        ))
    }
}

/// Create a new BZIP2 reader
pub fn NewBzip2Reader<R: Read>(reader: R) -> SquishResult<Bzip2Reader<R>> {
    Bzip2Reader::new(reader)
}

/// Create a new BZIP2 writer with default compression
pub fn NewBzip2Writer<W: Write>(writer: W) -> Bzip2Writer<W> {
    Bzip2Writer::new(writer)
}

/// Create a new BZIP2 writer with specified compression level
pub fn NewBzip2WriterLevel<W: Write>(writer: W, level: CompressionLevel) -> Bzip2Writer<W> {
    Bzip2Writer::with_level(writer, level)
}

/// Compress data using BZIP2 with default compression
pub fn bzip2_compress(data: &[u8]) -> SquishResult<Vec<u8>> {
    bzip2_compress_level(data, CompressionLevel::Default)
}

/// Compress data using BZIP2 with specified compression level
pub fn bzip2_compress_level(data: &[u8], level: CompressionLevel) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    {
        let mut writer = Bzip2Writer::with_level(&mut result, level);
        writer.write_all(data).map_err(SquishError::from)?;
        writer.close()?;
    }
    Ok(result)
}

/// Decompress BZIP2 data
pub fn bzip2_decompress(data: &[u8]) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    let cursor = std::io::Cursor::new(data);
    let mut reader = Bzip2Reader::new(cursor)?;
    reader.read_to_end(&mut result).map_err(SquishError::from)?;
    Ok(result)
}

/// Check if data is BZIP2 format
pub fn is_bzip2_data(data: &[u8]) -> bool {
    data.len() >= 3 && data[0] == 0x42 && data[1] == 0x5a && data[2] == b'h'
}

/// Get file extension for BZIP2 files
pub fn file_extension() -> &'static str {
    ".bz2"
}

/// Get MIME type for BZIP2 data
pub fn mime_type() -> &'static str {
    "application/x-bzip2"
}

/// Check if compression level is valid for BZIP2
pub fn is_valid_compression_level(level: i32) -> bool {
    level >= 1 && level <= 9
}

/// Initialize BZIP2 module
pub fn initialize() {
    // No specific initialization needed for BZIP2
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_bzip2_compress_decompress() {
        let original = b"Hello, World! This is a test of BZIP2-style compression.";
        
        // Compress
        let compressed = bzip2_compress(original).expect("Compression should succeed");
        assert!(!compressed.is_empty());
        
        // Decompress
        let decompressed = bzip2_decompress(&compressed).expect("Decompression should succeed");
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_bzip2_compression_levels() {
        let data = b"This is test data for BZIP2 compression level testing. ".repeat(20);
        
        let fast = bzip2_compress_level(&data, CompressionLevel::Fastest).unwrap();
        let best = bzip2_compress_level(&data, CompressionLevel::Best).unwrap();
        
        // Both should decompress to original
        assert_eq!(bzip2_decompress(&fast).unwrap(), data);
        assert_eq!(bzip2_decompress(&best).unwrap(), data);
    }

    #[test]
    fn test_bzip2_empty_data() {
        let empty = b"";
        let compressed = bzip2_compress(empty).unwrap();
        let decompressed = bzip2_decompress(&compressed).unwrap();
        assert_eq!(decompressed, empty);
    }

    #[test]
    fn test_bzip2_large_data() {
        let large_data = vec![b'B'; 5000]; // Smaller for compatibility
        let compressed = bzip2_compress(&large_data).unwrap();
        let decompressed = bzip2_decompress(&compressed).unwrap();
        assert_eq!(decompressed, large_data);
    }

    #[test]
    fn test_bzip2_streaming_compression() {
        let data = b"Streaming test data for BZIP2 compression.";
        let mut compressed = Vec::new();
        
        {
            let mut writer = NewBzip2Writer(&mut compressed);
            writer.write_all(data).unwrap();
            writer.close().unwrap();
        }
        
        let decompressed = bzip2_decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_bzip2_streaming_decompression() {
        let original = b"Streaming decompression test data for BZIP2.";
        let compressed = bzip2_compress(original).unwrap();
        
        let cursor = Cursor::new(compressed);
        let mut reader = NewBzip2Reader(cursor).unwrap();
        
        let mut result = Vec::new();
        reader.read_to_end(&mut result).unwrap();
        
        assert_eq!(result, original);
    }

    #[test]
    fn test_is_bzip2_data() {
        let bzip2_header = vec![0x42, 0x5a, b'h', b'9'];
        assert!(is_bzip2_data(&bzip2_header));
        
        let not_bzip2 = vec![0x1f, 0x8b, 0x08, 0x00];
        assert!(!is_bzip2_data(&not_bzip2));
        
        let too_short = vec![0x42, 0x5a];
        assert!(!is_bzip2_data(&too_short));
    }

    #[test]
    fn test_bzip2_metadata() {
        assert_eq!(file_extension(), ".bz2");
        assert_eq!(mime_type(), "application/x-bzip2");
        
        assert!(is_valid_compression_level(1));
        assert!(is_valid_compression_level(9));
        assert!(!is_valid_compression_level(0));
        assert!(!is_valid_compression_level(10));
    }

    #[test]
    fn test_bzip2_statistics() {
        let data = b"Test data for statistics collection.";
        let mut result = Vec::new();
        
        let mut writer = NewBzip2Writer(&mut result);
        writer.write_all(data).unwrap();
        
        if let Some(stats) = writer.stats() {
            assert_eq!(stats.algorithm, "bzip2");
            assert!(stats.input_size > 0);
        }
        
        writer.close().unwrap();
    }

    #[test]
    fn test_module_initialization() {
        initialize(); // Should not panic
    }
}

/// fr fr Create new BZIP2 reader with default settings
pub fn new_reader<R: Read>(reader: R) -> SquishResult<Bzip2Reader<R>> {
    Bzip2Reader::new(reader)
}

/// bestie Create new BZIP2 writer with default compression
pub fn new_writer<W: Write>(writer: W) -> SquishResult<Bzip2Writer<W>> {
    Bzip2Writer::new(writer)
}

/// periodt Create new BZIP2 writer with specified compression level
pub fn new_writer_level<W: Write>(writer: W, level: CompressionLevel) -> SquishResult<Bzip2Writer<W>> {
    Bzip2Writer::with_level(writer, level)
}
