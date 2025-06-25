/// BZIP2 compression implementation for SquishCore
/// 
/// This module provides BZIP2 compression and decompression functionality.
/// Note: This is a simplified implementation that provides the interface
/// but uses DEFLATE compression instead of actual BZIP2 for compatibility.

// use crate::stdlib::squish_core::{SquishError, SquishResult, CompressionLevel, CompressionStats};
// use crate::stdlib::squish_core::core::{Reader as SquishReader, Writer as SquishWriter};
use std::io::{Read, Write, BufWriter, BufReader};
use flate2::{Compression, read::DeflateDecoder, write::DeflateEncoder};
use std::time::Instant;
use crate::error::CursedError;

/// BZIP2 reader that decompresses data on read
/// Note: Currently uses DEFLATE internally for compatibility
pub struct Bzip2Reader<R: Read> {
impl<R: Read> Bzip2Reader<R> {
    /// Create a new BZIP2 reader
    pub fn new(reader: R) -> SquishResult<Self> {
        let buffered = BufReader::new(reader);
        let decoder = DeflateDecoder::new(buffered);
        
        Ok(Self {
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
    fn stats(&self) -> Option<CompressionStats> {
        Some(CompressionStats::new(
        ))
    }
}

/// BZIP2 writer that compresses data on write
/// Note: Currently uses DEFLATE internally for compatibility
pub struct Bzip2Writer<W: Write> {
impl<W: Write> Bzip2Writer<W> {
    /// Create a new BZIP2 writer with default compression
    pub fn new(writer: W) -> Self {
        Self::with_level(writer, CompressionLevel::Default)
    /// Create a new BZIP2 writer with specified compression level
    pub fn with_level(writer: W, level: CompressionLevel) -> Self {
        let buffered = BufWriter::new(writer);
        // Map BZIP2 levels (1-9) to DEFLATE levels
        let deflate_level = match level.to_numeric() {
            _ => 6, // Default
        let compression = Compression::new(deflate_level as u32);
        let encoder = DeflateEncoder::new(buffered, compression);
        
        Self {
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
impl<W: Write> SquishWriter for Bzip2Writer<W> {
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
        Err(SquishError::generic("Reset not supported for BZIP2 writer in this implementation"))
    fn stats(&self) -> Option<CompressionStats> {
        Some(CompressionStats::new(
        ))
    }
}

/// Create a new BZIP2 reader
pub fn NewBzip2Reader<R: Read>(reader: R) -> SquishResult<Bzip2Reader<R>> {
    Bzip2Reader::new(reader)
/// Create a new BZIP2 writer with default compression
pub fn NewBzip2Writer<W: Write>(writer: W) -> Bzip2Writer<W> {
    Bzip2Writer::new(writer)
/// Create a new BZIP2 writer with specified compression level
pub fn NewBzip2WriterLevel<W: Write>(writer: W, level: CompressionLevel) -> Bzip2Writer<W> {
    Bzip2Writer::with_level(writer, level)
/// Compress data using BZIP2 with default compression
pub fn bzip2_compress(data: &[u8]) -> SquishResult<Vec<u8>> {
    bzip2_compress_level(data, CompressionLevel::Default)
/// Compress data using BZIP2 with specified compression level
pub fn bzip2_compress_level(data: &[u8], level: CompressionLevel) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    {
        let mut writer = Bzip2Writer::with_level(&mut result, level);
        writer.write_all(data).map_err(SquishError::from)?;
        writer.close()?;
    }
    Ok(result)
/// Decompress BZIP2 data
pub fn bzip2_decompress(data: &[u8]) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    let cursor = std::io::Cursor::new(data);
    let mut reader = Bzip2Reader::new(cursor)?;
    reader.read_to_end(&mut result).map_err(SquishError::from)?;
    Ok(result)
/// Check if data is BZIP2 format
pub fn is_bzip2_data(data: &[u8]) -> bool {
    data.len() >= 3 && data[0] == 0x42 && data[1] == 0x5a && data[2] == b'h'
/// Get file extension for BZIP2 files
pub fn file_extension() -> &'static str {
    ".bz2"
/// Get MIME type for BZIP2 data
pub fn mime_type() -> &'static str {
    "application/x-bzip2"
/// Check if compression level is valid for BZIP2
pub fn is_valid_compression_level(level: i32) -> bool {
    level >= 1 && level <= 9
/// Initialize BZIP2 module
pub fn initialize() {
        // TODO: implement
    }
    // No specific initialization needed for BZIP2

/// bestie Create new BZIP2 writer with default compression
pub fn new_writer<W: Write>(writer: W) -> SquishResult<Bzip2Writer<W>> {
    Bzip2Writer::new(writer)
/// periodt Create new BZIP2 writer with specified compression level
pub fn new_writer_level<W: Write>(writer: W, level: CompressionLevel) -> SquishResult<Bzip2Writer<W>> {
    Bzip2Writer::with_level(writer, level)
}
