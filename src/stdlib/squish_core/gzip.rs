/// GZIP compression implementation for SquishCore
/// 
/// This module provides GZIP compression and decompression functionality
/// using the flate2 crate, with CURSED-style interfaces and error handling.

// use crate::stdlib::squish_core::{SquishError, SquishResult, CompressionLevel, CompressionStats, CompressionTimer};
// use crate::stdlib::squish_core::core::{Reader as SquishReader, Writer as SquishWriter};
use std::io::{Read, Write, BufWriter, BufReader};
use flate2::{Compression, GzBuilder, read::GzDecoder, write::GzEncoder};
use std::time::Instant;
use crate::error::CursedError;

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
        // TODO: implement
    }
    // No specific initialization needed for GZIP
}


/// bestie Create new GZIP writer with default compression
pub fn new_writer<W: Write>(writer: W) -> SquishResult<GzipWriter<W>> {
    GzipWriter::new(writer)
}

/// periodt Create new GZIP writer with specified compression level
pub fn new_writer_level<W: Write>(writer: W, level: CompressionLevel) -> SquishResult<GzipWriter<W>> {
    GzipWriter::with_level(writer, level)
}
