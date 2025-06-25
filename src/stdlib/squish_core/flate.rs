/// Raw DEFLATE compression implementation for SquishCore
/// 
/// This module provides raw DEFLATE compression and decompression functionality
/// without GZIP or ZLIB headers, using the flate2 crate.

// use crate::stdlib::squish_core::{SquishError, SquishResult, CompressionLevel, CompressionStats};
// use crate::stdlib::squish_core::core::{Reader as SquishReader, Writer as SquishWriter};
use std::io::{Read, Write, BufWriter, BufReader};
use flate2::{Compression, read::DeflateDecoder, write::DeflateEncoder};
use std::time::Instant;
use crate::error::CursedError;

/// DEFLATE reader that decompresses raw deflate data on read
pub struct FlateReader<R: Read> {
impl<R: Read> FlateReader<R> {
    /// Create a new DEFLATE reader
    pub fn new(reader: R) -> SquishResult<Self> {
        let buffered = BufReader::new(reader);
        let decoder = DeflateDecoder::new(buffered);
        
        Ok(Self {
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
    fn stats(&self) -> Option<CompressionStats> {
        Some(CompressionStats::new(
        ))
    }
}

/// DEFLATE writer that compresses data to raw deflate format on write
pub struct FlateWriter<W: Write> {
impl<W: Write> FlateWriter<W> {
    /// Create a new DEFLATE writer with default compression
    pub fn new(writer: W) -> Self {
        Self::with_level(writer, CompressionLevel::Default)
    /// Create a new DEFLATE writer with specified compression level
    pub fn with_level(writer: W, level: CompressionLevel) -> Self {
        let buffered = BufWriter::new(writer);
        let compression = Compression::new(level.to_numeric() as u32);
        let encoder = DeflateEncoder::new(buffered, compression);
        
        Self {
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
impl<W: Write> SquishWriter for FlateWriter<W> {
    fn close(&mut self) -> SquishResult<()> {
        if let Some(encoder) = self.inner.take() {
            let writer = encoder.finish().map_err(SquishError::from)?;
            drop(writer);
        }
        Ok(())
    fn flush(&mut self) -> SquishResult<()> {
        Write::flush(self).map_err(SquishError::from)
    fn reset(&mut self, writer: Box<dyn Write>) -> SquishResult<()> {
        // Close current encoder
        self.close()?;
        
        // Create new encoder (simplified implementation)
        Err(SquishError::generic("Reset not supported for DEFLATE writer in this implementation"))
    fn stats(&self) -> Option<CompressionStats> {
        Some(CompressionStats::new(
        ))
    }
}

/// Create a new DEFLATE reader
pub fn NewFlateReader<R: Read>(reader: R) -> SquishResult<FlateReader<R>> {
    FlateReader::new(reader)
/// Create a new DEFLATE writer with default compression
pub fn NewFlateWriter<W: Write>(writer: W) -> FlateWriter<W> {
    FlateWriter::new(writer)
/// Create a new DEFLATE writer with specified compression level
pub fn NewFlateWriterLevel<W: Write>(writer: W, level: CompressionLevel) -> FlateWriter<W> {
    FlateWriter::with_level(writer, level)
/// Compress data using raw DEFLATE with default compression
pub fn flate_compress(data: &[u8]) -> SquishResult<Vec<u8>> {
    flate_compress_level(data, CompressionLevel::Default)
/// Compress data using raw DEFLATE with specified compression level
pub fn flate_compress_level(data: &[u8], level: CompressionLevel) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    {
        let mut writer = FlateWriter::with_level(&mut result, level);
        writer.write_all(data).map_err(SquishError::from)?;
        writer.close()?;
    }
    Ok(result)
/// Decompress raw DEFLATE data
pub fn flate_decompress(data: &[u8]) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    let cursor = std::io::Cursor::new(data);
    let mut reader = FlateReader::new(cursor)?;
    reader.read_to_end(&mut result).map_err(SquishError::from)?;
    Ok(result)
/// Get file extension for DEFLATE files
pub fn file_extension() -> &'static str {
    ".deflate"
/// Get MIME type for DEFLATE data
pub fn mime_type() -> &'static str {
    "application/deflate"
/// Check if compression level is valid for DEFLATE
pub fn is_valid_compression_level(level: i32) -> bool {
    level >= 0 && level <= 9 || level == -1
/// Initialize DEFLATE module
pub fn initialize() {
        // TODO: implement
    }
    // No specific initialization needed for DEFLATE

/// bestie Create new FLATE writer with default compression
pub fn new_writer<W: Write>(writer: W) -> SquishResult<FlateWriter<W>> {
    FlateWriter::new(writer)
}
