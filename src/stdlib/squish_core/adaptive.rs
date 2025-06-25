/// Adaptive compression that automatically selects the best algorithm
// use crate::stdlib::squish_core::{
    error::{SquishError, SquishResult},
    constants::*,
    statistics::{CompressionStats, OperationTimer},
};

/// Strategy for adaptive compression
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionStrategy {
    /// Optimize for best compression ratio
    BestRatio,
    /// Optimize for fastest compression
    BestSpeed,
    /// Balance between ratio and speed
    Balanced,
    /// Automatically determine best strategy
    Auto,
}

impl Default for CompressionStrategy {
    fn default() -> Self {
        CompressionStrategy::Balanced
    }
}

/// Adaptive compressor that selects the best algorithm based on data characteristics
pub struct AdaptiveCompressor {
    strategy: CompressionStrategy,
    sample_size: usize,
    ratio_threshold: f64,
    speed_threshold_ms: u64,
}

impl Default for AdaptiveCompressor {
    fn default() -> Self {
        AdaptiveCompressor {
            strategy: CompressionStrategy::default(),
            sample_size: ADAPTIVE_SAMPLE_SIZE,
            ratio_threshold: COMPRESSION_RATIO_THRESHOLD,
            speed_threshold_ms: SPEED_THRESHOLD_MS,
        }
    }
}

impl AdaptiveCompressor {
    /// Create a new adaptive compressor
    pub fn new(strategy: CompressionStrategy) -> Self {
        AdaptiveCompressor {
            strategy,
            ..Default::default()
        }
    }
    
    /// Create adaptive compressor with custom parameters
    pub fn with_config(
        strategy: CompressionStrategy,
        sample_size: usize,
        ratio_threshold: f64,
        speed_threshold_ms: u64,
    ) -> Self {
        AdaptiveCompressor {
            strategy,
            sample_size,
            ratio_threshold,
            speed_threshold_ms,
        }
    }
    
    /// Analyze data and recommend the best compression algorithm
    pub fn analyze_data(&self, data: &[u8]) -> SquishResult<String> {
        let sample = if data.len() > self.sample_size {
            &data[..self.sample_size]
        } else {
            data
        };
        
        // Analyze data characteristics
        let characteristics = self.analyze_characteristics(sample);
        
        // Select algorithm based on strategy and characteristics
        let algorithm = match self.strategy {
            CompressionStrategy::BestRatio => self.select_for_best_ratio(&characteristics),
            CompressionStrategy::BestSpeed => self.select_for_best_speed(&characteristics),
            CompressionStrategy::Balanced => self.select_balanced(&characteristics),
            CompressionStrategy::Auto => self.select_auto(&characteristics),
        };
        
        Ok(algorithm)
    }
    
    /// Compress data using adaptive algorithm selection
    pub fn compress(&mut self, data: &[u8]) -> SquishResult<Vec<u8>> {
        let algorithm = self.analyze_data(data)?;
        let timer = OperationTimer::start(&algorithm);
        
        // TODO: Implement actual compression using selected algorithm
        // For now, return a placeholder
        let compressed = data.to_vec(); // Placeholder
        
        let (algo, time_ms) = timer.finish();
        let stats = CompressionStats {
            compressed_size: compressed.len() as u64,
            decompressed_size: data.len() as u64,
            compression_ratio: compressed.len() as f64 / data.len() as f64,
            algorithm: algo,
            level: None,
            processing_time_ms: Some(time_ms),
        };
        
//         crate::stdlib::squish_core::statistics::update_global_stats(&algorithm, &stats);
        
        Ok(compressed)
    }
    
    /// Get current strategy
    pub fn strategy(&self) -> CompressionStrategy {
        self.strategy
    }
    
    /// Set compression strategy
    pub fn set_strategy(&mut self, strategy: CompressionStrategy) {
        self.strategy = strategy;
    }
    
    /// Analyze data characteristics for algorithm selection
    fn analyze_characteristics(&self, data: &[u8]) -> DataCharacteristics {
        let mut characteristics = DataCharacteristics::default();
        
        if data.is_empty() {
            return characteristics;
        }
        
        // Calculate entropy (simplified)
        let mut byte_counts = [0u32; 256];
        for &byte in data {
            byte_counts[byte as usize] += 1;
        }
        
        let len = data.len() as f64;
        let mut entropy = 0.0;
        for &count in &byte_counts {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
            }
        }
        characteristics.entropy = entropy;
        
        // Detect patterns
        characteristics.has_repeated_patterns = self.detect_repeated_patterns(data);
        characteristics.has_long_runs = self.detect_long_runs(data);
        characteristics.is_text_like = self.is_text_like(data);
        characteristics.is_binary = !characteristics.is_text_like;
        
        // Calculate compression hint based on characteristics
        if characteristics.entropy < 4.0 {
            characteristics.compression_hint = CompressionHint::HighlyCompressible;
        } else if characteristics.entropy < 6.0 {
            characteristics.compression_hint = CompressionHint::ModeratelyCompressible;
        } else {
            characteristics.compression_hint = CompressionHint::PoorlyCompressible;
        }
        
        characteristics
    }
    
    /// Select algorithm optimized for best compression ratio
    fn select_for_best_ratio(&self, characteristics: &DataCharacteristics) -> String {
        match characteristics.compression_hint {
            CompressionHint::HighlyCompressible => {
                if characteristics.has_long_runs {
                    "bzip2".to_string()
                } else {
                    "gzip".to_string()
                }
            }
            CompressionHint::ModeratelyCompressible => "gzip".to_string(),
            CompressionHint::PoorlyCompressible => "deflate".to_string(),
        }
    }
    
    /// Select algorithm optimized for speed
    fn select_for_best_speed(&self, characteristics: &DataCharacteristics) -> String {
        if characteristics.has_repeated_patterns {
            "lzw".to_string()
        } else {
            "deflate".to_string()
        }
    }
    
    /// Select balanced algorithm
    fn select_balanced(&self, characteristics: &DataCharacteristics) -> String {
        if characteristics.is_text_like {
            "gzip".to_string()
        } else if characteristics.has_long_runs {
            "bzip2".to_string()
        } else {
            "zlib".to_string()
        }
    }
    
    /// Auto-select algorithm based on comprehensive analysis
    fn select_auto(&self, characteristics: &DataCharacteristics) -> String {
        // Use balanced selection as baseline for auto mode
        self.select_balanced(characteristics)
    }
    
    /// Detect repeated patterns in data
    fn detect_repeated_patterns(&self, data: &[u8]) -> bool {
        if data.len() < 16 {
            return false;
        }
        
        // Simple pattern detection - look for repeated sequences
        let mut pattern_count = 0;
        let window_size = 8;
        
        for i in 0..data.len().saturating_sub(window_size * 2) {
            let pattern = &data[i..i + window_size];
            for j in (i + window_size)..data.len().saturating_sub(window_size) {
                if &data[j..j + window_size] == pattern {
                    pattern_count += 1;
                    if pattern_count > 3 {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    /// Detect long runs of similar bytes
    fn detect_long_runs(&self, data: &[u8]) -> bool {
        if data.len() < 8 {
            return false;
        }
        
        let mut current_run = 1;
        let mut max_run = 1;
        
        for i in 1..data.len() {
            if data[i] == data[i - 1] {
                current_run += 1;
                max_run = max_run.max(current_run);
            } else {
                current_run = 1;
            }
        }
        
        max_run >= 8
    }
    
    /// Check if data appears to be text-like
    fn is_text_like(&self, data: &[u8]) -> bool {
        if data.is_empty() {
            return false;
        }
        
        let mut printable_count = 0;
        for &byte in data {
            if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
                printable_count += 1;
            }
        }
        
        (printable_count as f64 / data.len() as f64) > 0.8
    }
}

/// Data characteristics used for algorithm selection
#[derive(Debug, Clone)]
struct DataCharacteristics {
    entropy: f64,
    has_repeated_patterns: bool,
    has_long_runs: bool,
    is_text_like: bool,
    is_binary: bool,
    compression_hint: CompressionHint,
}

impl Default for DataCharacteristics {
    fn default() -> Self {
        DataCharacteristics {
            entropy: 0.0,
            has_repeated_patterns: false,
            has_long_runs: false,
            is_text_like: false,
            is_binary: true,
            compression_hint: CompressionHint::ModeratelyCompressible,
        }
    }
}

/// Hint about expected compression effectiveness
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompressionHint {
    HighlyCompressible,
    ModeratelyCompressible,
    PoorlyCompressible,
}

/// Initialize adaptive compression module
pub fn initialize() -> SquishResult<()> {
//     crate::stdlib::squish_core::statistics::initialize_stats();
    Ok(())
}

/// Cleanup adaptive compression module
pub fn cleanup() -> SquishResult<()> {
    // Nothing to cleanup for now
    Ok(())
}

/// Create a new adaptive compressor with default settings
pub fn new_adaptive_compressor() -> AdaptiveCompressor {
    AdaptiveCompressor::default()
}

/// Create adaptive compressor with specified strategy
pub fn new_compressor_with_strategy(strategy: CompressionStrategy) -> AdaptiveCompressor {
    AdaptiveCompressor::new(strategy)
}

