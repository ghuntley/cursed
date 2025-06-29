/// ZLIB compression implementation for SquishCore
/// 
/// This module provides ZLIB compression and decompression functionality
/// using the flate2 crate, with CURSED-style interfaces and error handling.

use crate::stdlib::squish_core::{SquishError, SquishResult, constants::CompressionLevel, core::CompressionStats};
use crate::stdlib::squish_core::core::{Reader as SquishReader, Writer as SquishWriter};
use std::io::{Read, Write, BufWriter, BufReader};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use std::time::Instant;
use crate::error_types::CursedError;

/// ZLIB reader that decompresses data on read
pub struct ZlibReader<R: Read> {
    inner: ZlibDecoder<BufReader<R>>,
    bytes_read: usize,
    start_time: Instant,
}

impl<R: Read> ZlibReader<R> {
    /// Create a new ZLIB reader
    pub fn new(reader: R) -> SquishResult<Self> {
        let buffered = BufReader::new(reader);
        let decoder = ZlibDecoder::new(buffered);
        
        Ok(Self {
            inner: decoder,
            bytes_read: 0,
            start_time: Instant::now(),
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
            self.bytes_read as u64,
            0, // We don't track compressed size here
            self.start_time.elapsed(),
        ))
    }
}

/// ZLIB writer that compresses data on write
pub struct ZlibWriter<W: Write> {
    inner: Option<ZlibEncoder<BufWriter<W>>>,
    uncompressed_size: usize,
    start_time: Instant,
}

impl<W: Write> ZlibWriter<W> {
    /// Create a new ZLIB writer with default compression
    pub fn new(writer: W) -> Self {
        Self::with_level(writer, CompressionLevel::Default)
    /// Create a new ZLIB writer with specified compression level
    pub fn with_level(writer: W, level: CompressionLevel) -> Self {
        let buffered = BufWriter::new(writer);
        let compression = Compression::new(level.to_numeric() as u32);
        let encoder = ZlibEncoder::new(buffered, compression);
        
        Self {
            inner: Some(encoder),
            uncompressed_size: 0,
            start_time: Instant::now(),
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
impl<W: Write> SquishWriter for ZlibWriter<W> {
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
        Err(SquishError::generic("Reset not supported for ZLIB writer in this implementation"))
    fn stats(&self) -> Option<CompressionStats> {
        Some(CompressionStats::new(
            self.uncompressed_size as u64,
            0, // We don't track compressed size here
            self.start_time.elapsed(),
        ))
    }
}

/// Create a new ZLIB reader
pub fn NewZlibReader<R: Read>(reader: R) -> SquishResult<ZlibReader<R>> {
    ZlibReader::new(reader)
/// Create a new ZLIB writer with default compression
pub fn NewZlibWriter<W: Write>(writer: W) -> ZlibWriter<W> {
    ZlibWriter::new(writer)
/// Create a new ZLIB writer with specified compression level
pub fn NewZlibWriterLevel<W: Write>(writer: W, level: CompressionLevel) -> ZlibWriter<W> {
    ZlibWriter::with_level(writer, level)
/// Compress data using ZLIB with default compression
pub fn zlib_compress(data: &[u8]) -> SquishResult<Vec<u8>> {
    zlib_compress_level(data, CompressionLevel::Default)
/// Compress data using ZLIB with specified compression level
pub fn zlib_compress_level(data: &[u8], level: CompressionLevel) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    {
        let mut writer = ZlibWriter::with_level(&mut result, level);
        writer.write_all(data).map_err(SquishError::from)?;
        writer.close()?;
    }
    Ok(result)
/// Decompress ZLIB data
pub fn zlib_decompress(data: &[u8]) -> SquishResult<Vec<u8>> {
    let mut result = Vec::new();
    let cursor = std::io::Cursor::new(data);
    let mut reader = ZlibReader::new(cursor)?;
    reader.read_to_end(&mut result).map_err(SquishError::from)?;
    Ok(result)
/// Check if data is ZLIB format
pub fn is_zlib_data(data: &[u8]) -> bool {
    if data.len() < 2 {
        return false;
    // ZLIB magic numbers: 0x78 followed by various values
    data[0] == 0x78 && matches!(data[1], 0x01 | 0x5e | 0x9c | 0xda)
/// Get file extension for ZLIB files
pub fn file_extension() -> &'static str {
    ".zlib"
/// Get MIME type for ZLIB data
pub fn mime_type() -> &'static str {
    "application/zlib"
/// Check if compression level is valid for ZLIB
pub fn is_valid_compression_level(level: i32) -> bool {
    level >= 0 && level <= 9 || level == -1
/// Initialize ZLIB module
pub fn initialize() {
    // No specific initialization needed for ZLIB
}

/// bestie Create new ZLIB writer with default compression
pub fn new_writer<W: Write>(writer: W) -> SquishResult<ZlibWriter<W>> {
    Ok(ZlibWriter::new(writer))
/// periodt Create new ZLIB writer with specified compression level
pub fn new_writer_level<W: Write>(writer: W, level: CompressionLevel) -> SquishResult<ZlibWriter<W>> {
    Ok(ZlibWriter::with_level(writer, level))
}
