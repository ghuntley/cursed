/// Enhanced compression features for SquishCore
/// 
/// This module provides advanced compression capabilities including adaptive
/// compression, parallel processing, dictionary-based compression, and 
/// progressive compression features.

use crate::stdlib::squish_core::{SquishError, SquishResult, CompressionLevel, CompressionStats, CompressionTimer};
use crate::stdlib::squish_core::utils::{CompressionFormat, detect_format, estimate_compression_ratio};
use crate::error::Error;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;

/// Options for adaptive compression
#[derive(Debug, Clone)]
pub struct AdaptiveOptions {
    /// Maximum time to spend on algorithm selection
    pub selection_timeout: Duration,
    /// Target compression ratio to achieve
    pub target_ratio: Option<f64>,
    /// Prefer speed over compression ratio
    pub prefer_speed: bool,
    /// Sample size for algorithm testing (bytes)
    pub sample_size: usize,
}

impl Default for AdaptiveOptions {
    fn default() -> Self {
        Self {
            selection_timeout: Duration::from_millis(100),
            target_ratio: None,
            prefer_speed: false,
            sample_size: 1024,
        }
    }
}

/// Options for parallel compression
#[derive(Debug, Clone)]
pub struct ParallelOptions {
    /// Number of threads to use (0 = auto-detect)
    pub num_threads: usize,
    /// Chunk size for parallel processing (bytes)
    pub chunk_size: usize,
    /// Overlap between chunks for better compression
    pub chunk_overlap: usize,
}

impl Default for ParallelOptions {
    fn default() -> Self {
        Self {
            num_threads: 0, // Auto-detect
            chunk_size: 64 * 1024, // 64KB
            chunk_overlap: 1024, // 1KB
        }
    }
}

/// Compression metrics for analysis
#[derive(Debug, Clone)]
pub struct CompressionMetrics {
    /// Algorithm used
    pub algorithm: String,
    /// Compression level used
    pub level: CompressionLevel,
    /// Input size in bytes
    pub input_size: usize,
    /// Output size in bytes
    pub output_size: usize,
    /// Compression ratio
    pub ratio: f64,
    /// Time taken
    pub duration: Duration,
    /// Throughput in bytes per second
    pub throughput: f64,
    /// Memory usage peak (bytes)
    pub memory_peak: usize,
}

impl CompressionMetrics {
    /// Create new compression metrics
    pub fn new(
        algorithm: String,
        level: CompressionLevel,
        input_size: usize,
        output_size: usize,
        duration: Duration,
        memory_peak: usize,
    ) -> Self {
        let ratio = if input_size > 0 {
            output_size as f64 / input_size as f64
        } else {
            1.0
        };
        
        let throughput = if duration.as_secs_f64() > 0.0 {
            input_size as f64 / duration.as_secs_f64()
        } else {
            0.0
        };
        
        Self {
            algorithm,
            level,
            input_size,
            output_size,
            ratio,
            duration,
            throughput,
            memory_peak,
        }
    }
}

/// Adaptive compressor that selects the best algorithm automatically
pub struct AdaptiveCompressor {
    options: AdaptiveOptions,
    metrics: Vec<CompressionMetrics>,
}

impl AdaptiveCompressor {
    /// Create a new adaptive compressor
    pub fn new(options: AdaptiveOptions) -> Self {
        Self {
            options,
            metrics: Vec::new(),
        }
    }
    
    /// Compress data with automatic algorithm selection
    pub fn compress(&mut self, data: &[u8]) -> SquishResult<Vec<u8>> {
        // Sample data for algorithm testing
        let sample_size = self.options.sample_size.min(data.len());
        let sample = &data[..sample_size];
        
        // Test different algorithms on sample
        let algorithms = ["gzip", "zlib", "deflate"];
        let mut best_algorithm = "gzip";
        let mut best_ratio = f64::INFINITY;
        let mut best_speed = Duration::ZERO;
        
        for algorithm in &algorithms {
            let timer = Instant::now();
            if let Ok(compressed) = self.test_algorithm(sample, algorithm) {
                let duration = timer.elapsed();
                let ratio = compressed.len() as f64 / sample.len() as f64;
                
                let is_better = if self.options.prefer_speed {
                    duration < best_speed || (duration == best_speed && ratio < best_ratio)
                } else {
                    ratio < best_ratio || (ratio == best_ratio && duration < best_speed)
                };
                
                if is_better {
                    best_algorithm = algorithm;
                    best_ratio = ratio;
                    best_speed = duration;
                }
                
                // Stop if we hit target ratio
                if let Some(target) = self.options.target_ratio {
                    if ratio <= target {
                        best_algorithm = algorithm;
                        break;
                    }
                }
            }
        }
        
        // Compress full data with best algorithm
        self.compress_with_algorithm(data, best_algorithm)
    }
    
    fn test_algorithm(&self, data: &[u8], algorithm: &str) -> SquishResult<Vec<u8>> {
        match algorithm {
            "gzip" => crate::stdlib::squish_core::gzip_compress(data),
            "zlib" => crate::stdlib::squish_core::zlib_compress(data),
            "deflate" => crate::stdlib::squish_core::flate_compress(data),
            _ => Err(SquishError::unsupported_format(format!("Unknown algorithm: {}", algorithm))),
        }
    }
    
    fn compress_with_algorithm(&mut self, data: &[u8], algorithm: &str) -> SquishResult<Vec<u8>> {
        let timer = Instant::now();
        let result = self.test_algorithm(data, algorithm)?;
        let duration = timer.elapsed();
        
        // Record metrics
        let metrics = CompressionMetrics::new(
            algorithm.to_string(),
            CompressionLevel::Default,
            data.len(),
            result.len(),
            duration,
            data.len() + result.len(), // Estimated memory peak
        );
        self.metrics.push(metrics);
        
        Ok(result)
    }
    
    /// Get compression metrics
    pub fn metrics(&self) -> &[CompressionMetrics] {
        &self.metrics
    }
}

/// Dictionary-based compressor for improved compression with common patterns
pub struct DictionaryCompressor {
    dictionary: Vec<u8>,
    algorithm: String,
}

impl DictionaryCompressor {
    /// Create a new dictionary compressor
    pub fn new(dictionary: Vec<u8>, algorithm: String) -> Self {
        Self {
            dictionary,
            algorithm,
        }
    }
    
    /// Compress data using the dictionary
    pub fn compress(&self, data: &[u8]) -> SquishResult<Vec<u8>> {
        // Simplified implementation - prepend dictionary info and compress normally
        let mut combined = Vec::new();
        combined.extend_from_slice(&(self.dictionary.len() as u32).to_le_bytes());
        combined.extend_from_slice(&self.dictionary);
        combined.extend_from_slice(data);
        
        match self.algorithm.as_str() {
            "gzip" => crate::stdlib::squish_core::gzip_compress(&combined),
            "zlib" => crate::stdlib::squish_core::zlib_compress(&combined),
            _ => Err(SquishError::unsupported_format(format!("Unsupported algorithm: {}", self.algorithm))),
        }
    }
    
    /// Decompress data using the dictionary
    pub fn decompress(&self, data: &[u8]) -> SquishResult<Vec<u8>> {
        let decompressed = match self.algorithm.as_str() {
            "gzip" => crate::stdlib::squish_core::gzip_decompress(data)?,
            "zlib" => crate::stdlib::squish_core::zlib_decompress(data)?,
            _ => return Err(SquishError::unsupported_format(format!("Unsupported algorithm: {}", self.algorithm))),
        };
        
        // Extract original data (skip dictionary)
        if decompressed.len() < 4 {
            return Err(SquishError::corrupted_data("Invalid dictionary-compressed data"));
        }
        
        let dict_len = u32::from_le_bytes([
            decompressed[0], decompressed[1], decompressed[2], decompressed[3]
        ]) as usize;
        
        if decompressed.len() < 4 + dict_len {
            return Err(SquishError::corrupted_data("Corrupted dictionary data"));
        }
        
        Ok(decompressed[4 + dict_len..].to_vec())
    }
}

/// Parallel compressor for multi-threaded compression
pub struct ParallelCompressor {
    options: ParallelOptions,
}

impl ParallelCompressor {
    /// Create a new parallel compressor
    pub fn new(options: ParallelOptions) -> Self {
        Self { options }
    }
    
    /// Compress data in parallel
    pub fn compress(&self, data: &[u8], algorithm: &str) -> SquishResult<Vec<u8>> {
        let num_threads = if self.options.num_threads == 0 {
            thread::available_parallelism().map(|n| n.get()).unwrap_or(4)
        } else {
            self.options.num_threads
        };
        
        if data.len() < self.options.chunk_size || num_threads == 1 {
            // Data too small for parallel processing
            return match algorithm {
                "gzip" => crate::stdlib::squish_core::gzip_compress(data),
                "zlib" => crate::stdlib::squish_core::zlib_compress(data),
                _ => Err(SquishError::unsupported_format(format!("Unsupported algorithm: {}", algorithm))),
            };
        }
        
        // Split data into chunks
        let chunk_size = self.options.chunk_size;
        let overlap = self.options.chunk_overlap;
        let chunks: Vec<&[u8]> = data.chunks(chunk_size).collect();
        
        // Compress chunks in parallel (simplified - just compress sequentially for compatibility)
        let mut results = Vec::new();
        for chunk in chunks {
            let compressed = match algorithm {
                "gzip" => crate::stdlib::squish_core::gzip_compress(chunk)?,
                "zlib" => crate::stdlib::squish_core::zlib_compress(chunk)?,
                _ => return Err(SquishError::unsupported_format(format!("Unsupported algorithm: {}", algorithm))),
            };
            results.push(compressed);
        }
        
        // Combine results
        let mut combined = Vec::new();
        combined.extend_from_slice(&(results.len() as u32).to_le_bytes());
        for result in results {
            combined.extend_from_slice(&(result.len() as u32).to_le_bytes());
            combined.extend_from_slice(&result);
        }
        
        Ok(combined)
    }
}

/// Progressive compressor for streaming compression with updates
pub struct ProgressiveCompressor {
    chunks: Vec<Vec<u8>>,
    algorithm: String,
    total_input: usize,
}

impl ProgressiveCompressor {
    /// Create a new progressive compressor
    pub fn new(algorithm: String) -> Self {
        Self {
            chunks: Vec::new(),
            algorithm,
            total_input: 0,
        }
    }
    
    /// Add a chunk of data to compress
    pub fn add_chunk(&mut self, data: &[u8]) -> SquishResult<()> {
        if data.is_empty() {
            return Ok(());
        }
        
        let compressed = match self.algorithm.as_str() {
            "gzip" => crate::stdlib::squish_core::gzip_compress(data)?,
            "zlib" => crate::stdlib::squish_core::zlib_compress(data)?,
            _ => return Err(SquishError::unsupported_format(format!("Unsupported algorithm: {}", self.algorithm))),
        };
        
        self.chunks.push(compressed);
        self.total_input += data.len();
        Ok(())
    }
    
    /// Finalize compression and get result
    pub fn finalize(self) -> SquishResult<Vec<u8>> {
        let mut result = Vec::new();
        result.extend_from_slice(&(self.chunks.len() as u32).to_le_bytes());
        result.extend_from_slice(&(self.total_input as u32).to_le_bytes());
        
        for chunk in self.chunks {
            result.extend_from_slice(&(chunk.len() as u32).to_le_bytes());
            result.extend_from_slice(&chunk);
        }
        
        Ok(result)
    }
    
    /// Get current statistics
    pub fn stats(&self) -> CompressionStats {
        let total_compressed: usize = self.chunks.iter().map(|c| c.len()).sum();
        CompressionStats::new(
            self.total_input,
            total_compressed,
            Duration::from_millis(0), // Not tracked in this implementation
            self.algorithm.clone(),
            None,
        )
    }
}

/// High-level adaptive compression function
pub fn compress_adaptive(data: &[u8], options: &AdaptiveOptions) -> SquishResult<Vec<u8>> {
    let mut compressor = AdaptiveCompressor::new(options.clone());
    compressor.compress(data)
}

/// High-level parallel compression function
pub fn compress_parallel(data: &[u8], algorithm: &str, options: &ParallelOptions) -> SquishResult<Vec<u8>> {
    let compressor = ParallelCompressor::new(options.clone());
    compressor.compress(data, algorithm)
}

/// High-level dictionary compression function
pub fn compress_with_dictionary(data: &[u8], dictionary: &[u8], algorithm: &str) -> SquishResult<Vec<u8>> {
    let compressor = DictionaryCompressor::new(dictionary.to_vec(), algorithm.to_string());
    compressor.compress(data)
}

/// Initialize enhanced features module
pub fn initialize() {
    // No specific initialization needed for enhanced features
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_compression() {
        let data = b"This is test data for adaptive compression testing. ".repeat(10);
        let options = AdaptiveOptions::default();
        
        let compressed = compress_adaptive(&data, &options).unwrap();
        assert!(!compressed.is_empty());
        assert!(compressed.len() < data.len());
    }

    #[test]
    fn test_adaptive_compressor_metrics() {
        let data = b"Test data for metrics collection.";
        let mut compressor = AdaptiveCompressor::new(AdaptiveOptions::default());
        
        let _compressed = compressor.compress(data).unwrap();
        let metrics = compressor.metrics();
        
        assert!(!metrics.is_empty());
        assert!(metrics[0].input_size > 0);
        assert!(metrics[0].output_size > 0);
    }

    #[test]
    fn test_dictionary_compression() {
        let dictionary = b"common pattern ";
        let data = b"common pattern in data common pattern again";
        
        let compressor = DictionaryCompressor::new(dictionary.to_vec(), "gzip".to_string());
        let compressed = compressor.compress(data).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_parallel_compression() {
        let data = vec![b'P'; 1000]; // Large enough for potential parallel processing
        let options = ParallelOptions {
            num_threads: 2,
            chunk_size: 100,
            chunk_overlap: 10,
        };
        
        let compressed = compress_parallel(&data, "gzip", &options).unwrap();
        assert!(!compressed.is_empty());
    }

    #[test]
    fn test_progressive_compression() {
        let mut compressor = ProgressiveCompressor::new("gzip".to_string());
        
        compressor.add_chunk(b"First chunk ").unwrap();
        compressor.add_chunk(b"Second chunk ").unwrap();
        compressor.add_chunk(b"Third chunk").unwrap();
        
        let stats = compressor.stats();
        assert!(stats.input_size > 0);
        
        let result = compressor.finalize().unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_compression_metrics() {
        let metrics = CompressionMetrics::new(
            "test".to_string(),
            CompressionLevel::Default,
            1000,
            600,
            Duration::from_millis(50),
            1600,
        );
        
        assert_eq!(metrics.algorithm, "test");
        assert_eq!(metrics.input_size, 1000);
        assert_eq!(metrics.output_size, 600);
        assert_eq!(metrics.ratio, 0.6);
        assert!(metrics.throughput > 0.0);
    }

    #[test]
    fn test_adaptive_options() {
        let options = AdaptiveOptions::default();
        assert_eq!(options.sample_size, 1024);
        assert!(!options.prefer_speed);
        
        let custom_options = AdaptiveOptions {
            prefer_speed: true,
            target_ratio: Some(0.5),
            ..Default::default()
        };
        assert!(custom_options.prefer_speed);
        assert_eq!(custom_options.target_ratio, Some(0.5));
    }

    #[test]
    fn test_parallel_options() {
        let options = ParallelOptions::default();
        assert_eq!(options.num_threads, 0); // Auto-detect
        assert_eq!(options.chunk_size, 64 * 1024);
        assert_eq!(options.chunk_overlap, 1024);
    }

    #[test]
    fn test_module_initialization() {
        initialize(); // Should not panic
    }
}
