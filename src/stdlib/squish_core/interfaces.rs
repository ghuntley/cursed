/// Core interfaces for compression and decompression operations
use crate::stdlib::squish_core::error::SquishResult;
use std::io;

/// Reader interface for reading from compressed data streams
pub trait Reader: io::Read {
    /// Read compressed data into the provided buffer
    /// Returns the number of bytes read and any error encountered
    fn read(&mut self, buf: &mut [u8]) -> SquishResult<usize>;
    
    /// Close the reader and release any resources
    fn close(&mut self) -> SquishResult<()>;
    
    /// Get compression statistics if available
    fn stats(&self) -> Option<crate::stdlib::squish_core::statistics::CompressionStats> {
        None
    }
    
    /// Reset the reader to its initial state
    fn reset(&mut self) -> SquishResult<()> {
        Err(crate::stdlib::squish_core::error::not_supported_error("Reset not supported"))
    }
}

/// Writer interface for writing to compressed data streams  
pub trait Writer: io::Write {
    /// Write data to the compressed stream
    /// Returns the number of bytes written and any error encountered
    fn write(&mut self, buf: &[u8]) -> SquishResult<usize>;
    
    /// Close the writer and finalize compression
    fn close(&mut self) -> SquishResult<()>;
    
    /// Flush any pending data to the underlying stream
    fn flush(&mut self) -> SquishResult<()>;
    
    /// Get compression statistics if available
    fn stats(&self) -> Option<crate::stdlib::squish_core::statistics::CompressionStats> {
        None
    }
    
    /// Reset the writer to use a new destination
    fn reset(&mut self, dst: Box<dyn io::Write>) -> SquishResult<()> {
        let _ = dst; // Suppress unused parameter warning
        Err(crate::stdlib::squish_core::error::not_supported_error("Reset not supported"))
    }
    
    /// Set compression level (if supported)
    fn set_level(&mut self, level: i32) -> SquishResult<()> {
        let _ = level; // Suppress unused parameter warning
        Err(crate::stdlib::squish_core::error::not_supported_error("Dynamic level setting not supported"))
    }
}

/// Generic compressor interface for one-shot compression operations
pub trait Compressor {
    /// Compress source data into destination buffer
    /// Returns the number of bytes written to the destination
    fn compress(&mut self, dst: &mut [u8], src: &[u8]) -> SquishResult<usize>;
    
    /// Compress with specific compression level
    /// Level typically ranges from 0 (no compression) to 9 (best compression)
    /// -1 indicates default compression level
    fn compress_level(&mut self, dst: &mut [u8], src: &[u8], level: i32) -> SquishResult<usize>;
    
    /// Get the maximum size needed for compression output
    fn max_compressed_size(&self, input_size: usize) -> usize;
    
    /// Get compressor statistics
    fn stats(&self) -> Option<crate::stdlib::squish_core::statistics::CompressionStats> {
        None
    }
    
    /// Reset compressor state
    fn reset(&mut self) -> SquishResult<()> {
        Ok(())
    }
}

/// Generic decompressor interface for one-shot decompression operations
pub trait Decompressor {
    /// Decompress source data into destination buffer
    /// Returns the number of bytes written to the destination
    fn decompress(&mut self, dst: &mut [u8], src: &[u8]) -> SquishResult<usize>;
    
    /// Get the expected decompressed size if known
    fn decompressed_size(&self, compressed_data: &[u8]) -> Option<usize> {
        let _ = compressed_data; // Suppress unused parameter warning
        None
    }
    
    /// Get decompressor statistics
    fn stats(&self) -> Option<crate::stdlib::squish_core::statistics::CompressionStats> {
        None
    }
    
    /// Reset decompressor state
    fn reset(&mut self) -> SquishResult<()> {
        Ok(())
    }
}

/// Extended reader interface with additional functionality
pub trait ExtendedReader: Reader {
    /// Skip ahead in the stream by the specified number of bytes
    fn skip(&mut self, n: usize) -> SquishResult<usize> {
        let mut buf = vec![0u8; std::cmp::min(n, 8192)];
        let mut total_skipped = 0;
        while total_skipped < n {
            let to_skip = std::cmp::min(n - total_skipped, buf.len());
            let skipped = self.read(&mut buf[..to_skip])?;
            if skipped == 0 {
                break;
            }
            total_skipped += skipped;
        }
        Ok(total_skipped)
    }
    
    /// Peek at upcoming data without consuming it
    fn peek(&mut self, buf: &mut [u8]) -> SquishResult<usize> {
        let _ = buf; // Suppress unused parameter warning
        Err(crate::stdlib::squish_core::error::not_supported_error("Peek not supported"))
    }
    
    /// Check if the reader has reached end of stream
    fn is_eof(&self) -> bool {
        false
    }
}

/// Extended writer interface with additional functionality
pub trait ExtendedWriter: Writer {
    /// Write all data or return an error
    fn write_all(&mut self, buf: &[u8]) -> SquishResult<()> {
        let mut written = 0;
        while written < buf.len() {
            let n = self.write(&buf[written..])?;
            if n == 0 {
                return Err(crate::stdlib::squish_core::error::io_error("Failed to write all data"));
            }
            written += n;
        }
        Ok(())
    }
    
    /// Sync all data to the underlying storage
    fn sync(&mut self) -> SquishResult<()> {
        self.flush()
    }
    
    /// Get the current position in the stream if supported
    fn position(&self) -> Option<u64> {
        None
    }
}

/// Seekable reader interface for random access
pub trait SeekableReader: Reader + io::Seek {
    /// Seek to a specific position in the uncompressed data
    fn seek_uncompressed(&mut self, pos: u64) -> SquishResult<u64> {
        let _ = pos; // Suppress unused parameter warning
        Err(crate::stdlib::squish_core::error::not_supported_error("Seek in uncompressed data not supported"))
    }
    
    /// Get the total uncompressed size if known
    fn uncompressed_size(&self) -> Option<u64> {
        None
    }
}

/// Configuration trait for compression parameters
pub trait Configurable {
    /// Configuration type for this compressor/decompressor
    type Config;
    
    /// Apply configuration
    fn configure(&mut self, config: Self::Config) -> SquishResult<()>;
    
    /// Get current configuration
    fn get_config(&self) -> Self::Config;
}

/// Trait for components that support dictionary-based compression
pub trait DictionarySupport {
    /// Set compression dictionary
    fn set_dictionary(&mut self, dict: &[u8]) -> SquishResult<()>;
    
    /// Get current dictionary if any
    fn get_dictionary(&self) -> Option<&[u8]>;
    
    /// Clear dictionary
    fn clear_dictionary(&mut self) -> SquishResult<()>;
}
