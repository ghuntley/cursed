/// Statistics and performance metrics for compression operations
use std::time::{Duration, Instant};

/// Compression statistics for individual operations
#[derive(Debug, Clone)]
pub struct CompressionStats {
    /// Size of compressed data in bytes
    pub compressed_size: u64,
    
    /// Size of original data in bytes
    pub decompressed_size: u64,
    
    /// Compression ratio (compressed_size / decompressed_size)
    pub compression_ratio: f64,
    
    /// Algorithm used for compression
    pub algorithm: String,
    
    /// Compression level used (if applicable)
    pub level: Option<i32>,
    
    /// Time taken for processing in milliseconds
    pub processing_time_ms: Option<u64>,
}

impl CompressionStats {
    /// Create new compression statistics
    pub fn new(algorithm: &str) -> Self {
        CompressionStats {
            compressed_size: 0,
            decompressed_size: 0,
            compression_ratio: 0.0,
            algorithm: algorithm.to_string(),
            level: None,
            processing_time_ms: None,
        }
    }
    
    /// Calculate compression ratio
    pub fn calculate_ratio(&mut self) {
        if self.decompressed_size > 0 {
            self.compression_ratio = self.compressed_size as f64 / self.decompressed_size as f64;
        }
    }
    
    /// Get space savings as a percentage
    pub fn space_savings_percent(&self) -> f64 {
        if self.decompressed_size > 0 {
            (1.0 - self.compression_ratio) * 100.0
        } else {
            0.0
        }
    }
    
    /// Get compression speed in bytes per millisecond
    pub fn compression_speed_bytes_per_ms(&self) -> Option<f64> {
        if let Some(time_ms) = self.processing_time_ms {
            if time_ms > 0 {
                Some(self.decompressed_size as f64 / time_ms as f64)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Get compression throughput in MB/s
    pub fn throughput_mbps(&self) -> Option<f64> {
        if let Some(speed) = self.compression_speed_bytes_per_ms() {
            Some(speed / 1000.0) // Convert bytes/ms to MB/s
        } else {
            None
        }
    }
    
    /// Check if compression was effective (ratio < 1.0)
    pub fn is_effective(&self) -> bool {
        self.compression_ratio < 1.0
    }
}

/// Performance metrics for compression operations
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Total number of compression operations
    pub total_compressions: u64,
    
    /// Total number of decompression operations
    pub total_decompressions: u64,
    
    /// Total bytes compressed
    pub total_bytes_compressed: u64,
    
    /// Total bytes decompressed
    pub total_bytes_decompressed: u64,
    
    /// Total time spent compressing (milliseconds)
    pub total_compression_time_ms: u64,
    
    /// Total time spent decompressing (milliseconds)
    pub total_decompression_time_ms: u64,
    
    /// Average compression ratio
    pub average_compression_ratio: f64,
    
    /// Best compression ratio achieved
    pub best_compression_ratio: f64,
    
    /// Worst compression ratio achieved
    pub worst_compression_ratio: f64,
    
    /// Number of failed operations
    pub failed_operations: u64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        PerformanceMetrics {
            total_compressions: 0,
            total_decompressions: 0,
            total_bytes_compressed: 0,
            total_bytes_decompressed: 0,
            total_compression_time_ms: 0,
            total_decompression_time_ms: 0,
            average_compression_ratio: 0.0,
            best_compression_ratio: f64::INFINITY,
            worst_compression_ratio: 0.0,
            failed_operations: 0,
        }
    }
}

impl PerformanceMetrics {
    /// Update metrics with compression statistics
    pub fn update_compression(&mut self, stats: &CompressionStats) {
        self.total_compressions += 1;
        self.total_bytes_compressed += stats.compressed_size;
        self.total_bytes_decompressed += stats.decompressed_size;
        
        if let Some(time_ms) = stats.processing_time_ms {
            self.total_compression_time_ms += time_ms;
        }
        
        // Update compression ratio statistics
        if stats.compression_ratio > 0.0 {
            if stats.compression_ratio < self.best_compression_ratio {
                self.best_compression_ratio = stats.compression_ratio;
            }
            if stats.compression_ratio > self.worst_compression_ratio {
                self.worst_compression_ratio = stats.compression_ratio;
            }
            
            // Recalculate average compression ratio
            self.recalculate_average_ratio();
        }
    }
    
    /// Update metrics for decompression
    pub fn update_decompression(&mut self, decompressed_size: u64, time_ms: Option<u64>) {
        self.total_decompressions += 1;
        self.total_bytes_decompressed += decompressed_size;
        
        if let Some(time) = time_ms {
            self.total_decompression_time_ms += time;
        }
    }
    
    /// Record a failed operation
    pub fn record_failure(&mut self) {
        self.failed_operations += 1;
    }
    
    /// Get average compression throughput in MB/s
    pub fn average_compression_throughput_mbps(&self) -> f64 {
        if self.total_compression_time_ms > 0 {
            (self.total_bytes_decompressed as f64 / 1_000_000.0) / 
            (self.total_compression_time_ms as f64 / 1000.0)
        } else {
            0.0
        }
    }
    
    /// Get average decompression throughput in MB/s
    pub fn average_decompression_throughput_mbps(&self) -> f64 {
        if self.total_decompression_time_ms > 0 {
            (self.total_bytes_decompressed as f64 / 1_000_000.0) / 
            (self.total_decompression_time_ms as f64 / 1000.0)
        } else {
            0.0
        }
    }
    
    /// Get failure rate as a percentage
    pub fn failure_rate_percent(&self) -> f64 {
        let total_operations = self.total_compressions + self.total_decompressions;
        if total_operations > 0 {
            (self.failed_operations as f64 / total_operations as f64) * 100.0
        } else {
            0.0
        }
    }
    
    /// Recalculate average compression ratio
    fn recalculate_average_ratio(&mut self) {
        if self.total_bytes_decompressed > 0 {
            self.average_compression_ratio = 
                self.total_bytes_compressed as f64 / self.total_bytes_decompressed as f64;
        }
    }
}

/// Module-level statistics
#[derive(Debug, Clone)]
pub struct ModuleStats {
    /// Performance metrics for each algorithm
    pub algorithm_metrics: std::collections::HashMap<String, PerformanceMetrics>,
    
    /// Overall module performance
    pub overall_metrics: PerformanceMetrics,
    
    /// Module initialization time
    pub initialization_time: Instant,
    
    /// Number of active compression operations
    pub active_operations: u64,
    
    /// Peak memory usage in bytes
    pub peak_memory_usage: u64,
    
    /// Current memory usage in bytes
    pub current_memory_usage: u64,
}

impl Default for ModuleStats {
    fn default() -> Self {
        ModuleStats {
            algorithm_metrics: std::collections::HashMap::new(),
            overall_metrics: PerformanceMetrics::default(),
            initialization_time: Instant::now(),
            active_operations: 0,
            peak_memory_usage: 0,
            current_memory_usage: 0,
        }
    }
}

impl ModuleStats {
    /// Update statistics for a specific algorithm
    pub fn update_algorithm_stats(&mut self, algorithm: &str, stats: &CompressionStats) {
        let metrics = self.algorithm_metrics.entry(algorithm.to_string())
            .or_insert_with(PerformanceMetrics::default);
        metrics.update_compression(stats);
        self.overall_metrics.update_compression(stats);
    }
    
    /// Get uptime since module initialization
    pub fn uptime(&self) -> Duration {
        self.initialization_time.elapsed()
    }
    
    /// Update memory usage statistics
    pub fn update_memory_usage(&mut self, current: u64) {
        self.current_memory_usage = current;
        if current > self.peak_memory_usage {
            self.peak_memory_usage = current;
        }
    }
    
    /// Increment active operations counter
    pub fn increment_active_operations(&mut self) {
        self.active_operations += 1;
    }
    
    /// Decrement active operations counter
    pub fn decrement_active_operations(&mut self) {
        if self.active_operations > 0 {
            self.active_operations -= 1;
        }
    }
}

/// Timer for measuring operation performance
pub struct OperationTimer {
    start_time: Instant,
    algorithm: String,
}

impl OperationTimer {
    /// Start timing an operation
    pub fn start(algorithm: &str) -> Self {
        OperationTimer {
            start_time: Instant::now(),
            algorithm: algorithm.to_string(),
        }
    }
    
    /// Finish timing and get elapsed milliseconds
    pub fn finish(self) -> (String, u64) {
        let elapsed = self.start_time.elapsed();
        (self.algorithm, elapsed.as_millis() as u64)
    }
    
    /// Get elapsed time without finishing
    pub fn elapsed_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }
}

// Global statistics tracking
static mut GLOBAL_STATS: Option<ModuleStats> = None;
static mut STATS_INITIALIZED: bool = false;

/// Initialize global statistics tracking
pub fn initialize_stats() {
        // TODO: implement
    }
    unsafe {
        if !STATS_INITIALIZED {
            GLOBAL_STATS = Some(ModuleStats::default());
            STATS_INITIALIZED = true;
        }
    }
}

/// Get global module statistics
// pub fn get_module_stats() -> crate::stdlib::squish_core::error::SquishResult<ModuleStats> {
    unsafe {
        if let Some(ref stats) = GLOBAL_STATS {
            Ok(stats.clone())
        } else {
//             Err(crate::stdlib::squish_core::error::general_error("Statistics not initialized"))
        }
    }
}

/// Update global statistics with operation results
pub fn update_global_stats(algorithm: &str, stats: &CompressionStats) {
    unsafe {
        if let Some(ref mut global_stats) = GLOBAL_STATS {
            global_stats.update_algorithm_stats(algorithm, stats);
        }
    }
}

/// Record a failed operation in global statistics
pub fn record_global_failure() {
        // TODO: implement
    }
    unsafe {
        if let Some(ref mut global_stats) = GLOBAL_STATS {
            global_stats.overall_metrics.record_failure();
        }
    }
}

/// Start operation tracking
pub fn start_operation() {
        // TODO: implement
    }
    unsafe {
        if let Some(ref mut global_stats) = GLOBAL_STATS {
            global_stats.increment_active_operations();
        }
    }
}

/// End operation tracking
pub fn end_operation() {
        // TODO: implement
    }
    unsafe {
        if let Some(ref mut global_stats) = GLOBAL_STATS {
            global_stats.decrement_active_operations();
        }
    }
}


pub fn cleanup() -> Result<(), String> {
    // Clear global statistics
    Ok(())
}
