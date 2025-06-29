//! Core compression interfaces and basic implementations

use super::error::{SquishError, SquishResult};
use super::constants::CompressionLevel;
use std::io::{Read, Write};

/// Basic compression function
pub fn compress(data: &[u8]) -> SquishResult<Vec<u8>> {
    compress_with_level(data, CompressionLevel::Default)
}

/// Compression with specified level
pub fn compress_with_level(data: &[u8], level: CompressionLevel) -> SquishResult<Vec<u8>> {
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::new(level.to_numeric() as u32));
    encoder.write_all(data)?;
    let compressed = encoder.finish()?;
    
    Ok(compressed)
}

/// Basic decompression function
pub fn decompress(data: &[u8]) -> SquishResult<Vec<u8>> {
    decompress_with_validation(data, true)
}

/// Decompression with optional validation
pub fn decompress_with_validation(data: &[u8], _validate: bool) -> SquishResult<Vec<u8>> {
    use flate2::read::ZlibDecoder;
    
    let mut decoder = ZlibDecoder::new(data);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result)?;
    
    Ok(result)
}

/// Reader trait for compression readers
pub trait Reader: Read {
    /// Close the reader
    fn close(&mut self) -> SquishResult<()>;
    
    /// Get compression statistics
    fn stats(&self) -> Option<CompressionStats> {
        None
    }
}

/// Writer trait for compression writers
pub trait Writer: Write {
    /// Close the writer and flush all data
    fn close(&mut self) -> SquishResult<()>;
    
    /// Flush the writer
    fn flush(&mut self) -> SquishResult<()>;
    
    /// Reset writer with new output
    fn reset(&mut self, writer: Box<dyn Write>) -> SquishResult<()>;
    
    /// Get compression statistics
    fn stats(&self) -> Option<CompressionStats> {
        None
    }
}

/// Compression statistics
#[derive(Debug, Clone)]
pub struct CompressionStats {
    pub bytes_in: u64,
    pub bytes_out: u64,
    pub compression_ratio: f64,
    pub time_taken: std::time::Duration,
}

impl CompressionStats {
    pub fn new(bytes_in: u64, bytes_out: u64, time_taken: std::time::Duration) -> Self {
        let compression_ratio = if bytes_in > 0 {
            bytes_out as f64 / bytes_in as f64
        } else {
            0.0
        };
        
        Self {
            bytes_in,
            bytes_out,
            compression_ratio,
            time_taken,
        }
    }
}
