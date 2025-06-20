/// ZLIB compression implementation for SquishCore
/// 
/// This module provides ZLIB compression and decompression functionality
/// using the flate2 crate, with CURSED-style interfaces and error handling.

use crate::stdlib::squish_core::{SquishError, SquishResult, CompressionLevel, CompressionStats};
use crate::stdlib::squish_core::core::{Reader as SquishReader, Writer as SquishWriter};
use std::io::{Read, Write, BufWriter, BufReader};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use std::time::Instant;

/// ZLIB reader that decompresses data on read
pub struct ZlibReader<R: Read> {
    inner: ZlibDecoder<BufReader<R>>,
    bytes_read: usize,
    timer: Instant,
    input_size: usize,
}

impl<R: Read> ZlibReader<R> {
    /// Create a new ZLIB reader
    pub fn new(reader: R) -> SquishResult<Self> {
        let buffered = BufReader::new(reader);
        let decoder = ZlibDecoder::new(buffered);
        
        Ok(Self {
            inner: decoder,
            bytes_read: 0,
            timer: Instant::now(),
            input_size: 0,
        })
    }
}

impl<R: Read> Read for ZlibReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes = self.inner.read(buf)?;
        self.bytes_read += bytes;
        Ok(bytes)
    }
}

impl<R: Read> SquishReader for ZlibReader<R> {
    fn close(&mut self) -> SquishResult<()> {
        // ZLIB decoder closes automatically when dropped
        Ok(())
    }
    
    fn stats(&self) -> Option<CompressionStats> {
        Some(CompressionStats::new(
            self.input_size,
            self.bytes_read,
            self.timer.elapsed(),
            "zlib".to_string(),
            None,
        ))
    }
}

/// ZLIB writer that compresses data on write
pub struct ZlibWriter<W: Write> {
    inner: Option<ZlibEncoder<BufWriter<W>>>,
    bytes_written: usize,
    uncompressed_size: usize,
    level: CompressionLevel,
    timer: Instant,
}

impl<W: Write> ZlibWriter<W> {
    /// Create a new ZLIB writer with default compression
    pub fn new(writer: W) -> Self {
        Self::with_level(writer, CompressionLevel::Default)
    }
    
    /// Create a new ZLIB writer with specified compression level
    pub fn with_level(writer: W, level: CompressionLevel) -> Self {
        let buffered = BufWriter::new(writer);
        let compression = Compression::new(level.to_numeric() as u32);
        let encoder = ZlibEncoder::new(buffered, compression);
        
        Self {
            inner: Some(encoder),
            bytes_written: 0,
            uncompressed_size: 0,
            level,
            timer: Instant::now(),
        }
    }
}

impl<W: Write> Write for ZlibWriter<W> {
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

impl<W: Write> SquishWriter for ZlibWriter<W> {
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
        Err(SquishError::generic("Reset not supported for ZLIB writer in this implementation"))
    }
    
    fn stats(&self) -> Option<CompressionStats> {
        Some(CompressionStats::new(
            self.uncompressed_size,
            self.bytes_written,
            self.timer.elapsed(),
            "zlib".to_string(),
            Some(self.level.to_numeric()),
        ))
    }
}

/// Create a new ZLIB reader
pub fn NewZlibReader<R: Read>(reader: R) -> SquishResult<ZlibReader<R>> {
    ZlibReader::new(reader)
}

/// Create a new ZLIB writer with default compression
pub fn NewZlibWriter<W: Write>(writer: W) -> ZlibWriter<W> {
    ZlibWriter::new(writer)
}

/// Create a new ZLIB writer with specified compression level
pub fn NewZlibWriterLevel<W: Write>(writer: W, level: CompressionLevel) -> ZlibWriter<W> {
    ZlibWriter::with_level(writer, level)
}

/// Compress data using ZLIB with default compression
pub fn zlib_compress(data: &[u8]) -> SquishResult<Vec<u8>> {
    zlib_compress_level(data, CompressionLevel::Default)
}

/// Compress data using ZLIB with specified compression level
pub fn zlib_compress_level(data: &[u8], level: CompressionLevel) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    {
        let mut writer = ZlibWriter::with_level(&mut result, level);
        writer.write_all(data).map_err(SquishError::from)?;
        writer.close()?;
    }
    Ok(result)
}

/// Decompress ZLIB data
pub fn zlib_decompress(data: &[u8]) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    let cursor = std::io::Cursor::new(data);
    let mut reader = ZlibReader::new(cursor)?;
    reader.read_to_end(&mut result).map_err(SquishError::from)?;
    Ok(result)
}

/// Check if data is ZLIB format
pub fn is_zlib_data(data: &[u8]) -> bool {
    if data.len() < 2 {
        return false;
    }
    
    // ZLIB magic numbers: 0x78 followed by various values
    data[0] == 0x78 && matches!(data[1], 0x01 | 0x5e | 0x9c | 0xda)
}

/// Get file extension for ZLIB files
pub fn file_extension() -> &'static str {
    ".zlib"
}

/// Get MIME type for ZLIB data
pub fn mime_type() -> &'static str {
    "application/zlib"
}

/// Check if compression level is valid for ZLIB
pub fn is_valid_compression_level(level: i32) -> bool {
    level >= 0 && level <= 9 || level == -1
}

/// Initialize ZLIB module
pub fn initialize() {
    // No specific initialization needed for ZLIB
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_zlib_compress_decompress() {
        let original = b"Hello, World! This is a test of ZLIB compression.";
        
        // Compress
        let compressed = zlib_compress(original).expect("Compression should succeed");
        assert!(!compressed.is_empty());
        assert!(compressed.len() < original.len() + 50); // Should be reasonably sized
        
        // Decompress
        let decompressed = zlib_decompress(&compressed).expect("Decompression should succeed");
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_zlib_compression_levels() {
        let data = b"This is test data for compression level testing. ".repeat(20);
        
        let fast = zlib_compress_level(&data, CompressionLevel::Fastest).unwrap();
        let best = zlib_compress_level(&data, CompressionLevel::Best).unwrap();
        
        // Best compression should be smaller than fastest
        assert!(best.len() <= fast.len());
        
        // Both should decompress to original
        assert_eq!(zlib_decompress(&fast).unwrap(), data);
        assert_eq!(zlib_decompress(&best).unwrap(), data);
    }

    #[test]
    fn test_zlib_empty_data() {
        let empty = b"";
        let compressed = zlib_compress(empty).unwrap();
        let decompressed = zlib_decompress(&compressed).unwrap();
        assert_eq!(decompressed, empty);
    }

    #[test]
    fn test_zlib_large_data() {
        let large_data = vec![b'Z'; 10000];
        let compressed = zlib_compress(&large_data).unwrap();
        let decompressed = zlib_decompress(&compressed).unwrap();
        assert_eq!(decompressed, large_data);
        
        // Should achieve good compression ratio on repetitive data
        assert!(compressed.len() < large_data.len() / 10);
    }

    #[test]
    fn test_zlib_streaming_compression() {
        let data = b"Streaming test data for ZLIB compression.";
        let mut compressed = Vec::new();
        
        {
            let mut writer = NewZlibWriter(&mut compressed);
            writer.write_all(data).unwrap();
            writer.close().unwrap();
        }
        
        let decompressed = zlib_decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_zlib_streaming_decompression() {
        let original = b"Streaming decompression test data.";
        let compressed = zlib_compress(original).unwrap();
        
        let cursor = Cursor::new(compressed);
        let mut reader = NewZlibReader(cursor).unwrap();
        
        let mut result = Vec::new();
        reader.read_to_end(&mut result).unwrap();
        
        assert_eq!(result, original);
    }

    #[test]
    fn test_is_zlib_data() {
        let zlib_header = vec![0x78, 0x9c];
        assert!(is_zlib_data(&zlib_header));
        
        let zlib_header2 = vec![0x78, 0xda];
        assert!(is_zlib_data(&zlib_header2));
        
        let not_zlib = vec![0x1f, 0x8b];
        assert!(!is_zlib_data(&not_zlib));
        
        let too_short = vec![0x78];
        assert!(!is_zlib_data(&too_short));
    }

    #[test]
    fn test_zlib_metadata() {
        assert_eq!(file_extension(), ".zlib");
        assert_eq!(mime_type(), "application/zlib");
        
        assert!(is_valid_compression_level(0));
        assert!(is_valid_compression_level(9));
        assert!(is_valid_compression_level(-1));
        assert!(!is_valid_compression_level(10));
    }

    #[test]
    fn test_zlib_statistics() {
        let data = b"Test data for statistics collection.";
        let mut result = Vec::new();
        
        let mut writer = NewZlibWriter(&mut result);
        writer.write_all(data).unwrap();
        
        if let Some(stats) = writer.stats() {
            assert_eq!(stats.algorithm, "zlib");
            assert!(stats.input_size > 0);
        }
        
        writer.close().unwrap();
    }

    #[test]
    fn test_module_initialization() {
        initialize(); // Should not panic
    }
}

/// fr fr Create new ZLIB reader with default settings
pub fn new_reader<R: Read>(reader: R) -> SquishResult<ZlibReader<R>> {
    ZlibReader::new(reader)
}

/// bestie Create new ZLIB writer with default compression
pub fn new_writer<W: Write>(writer: W) -> SquishResult<ZlibWriter<W>> {
    ZlibWriter::new(writer)
}

/// periodt Create new ZLIB writer with specified compression level
pub fn new_writer_level<W: Write>(writer: W, level: CompressionLevel) -> SquishResult<ZlibWriter<W>> {
    ZlibWriter::with_level(writer, level)
}
