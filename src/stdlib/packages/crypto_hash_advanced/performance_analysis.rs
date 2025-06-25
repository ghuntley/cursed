/// Production-ready hash function performance analysis and benchmarking
use crate::error::CursedError;
// use crate::stdlib::packages::crypto_hash_advanced::hash_traits::*;
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Result type for performance operations
pub type PerformanceResult<T> = std::result::Result<T, CryptoError>;

/// Performance metrics for a hash function
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
/// Performance metrics for small inputs (< 1KB)
#[derive(Debug, Clone)]
pub struct SmallInputMetrics {
/// Performance metrics for large inputs (> 1MB)
#[derive(Debug, Clone)]
pub struct LargeInputMetrics {
/// Comprehensive benchmark suite for hash functions
pub struct HashBenchmark {
impl HashBenchmark {
    pub fn new() -> Self {
        Self {
            test_data_sizes: vec![
                8, 16, 32, 64, 128, 256, 512, 1024,           // Small sizes
                4096, 8192, 16384, 32768,                      // Medium sizes  
                65536, 131072, 262144, 524288, 1048576,       // Large sizes
                4194304, 8388608, 16777216,                   // Very large sizes
        }
    }
    
    pub fn with_custom_sizes(sizes: Vec<usize>) -> Self {
        Self {
        }
    }
    
    /// Run comprehensive benchmark for a hash function
    pub fn benchmark<H: Hasher + Clone>(&self, hasher: H) -> PerformanceResult<PerformanceMetrics> {
        println!("🚀 Starting comprehensive benchmark for {}", hasher.algorithm());
        
        let start_time = Instant::now();
        
        // Warm up
        self.warmup(hasher.clone());
        
        // Benchmark different input sizes
        let mut size_results = HashMap::new();
        let mut total_bytes_processed = 0u64;
        let mut total_time = Duration::new(0, 0);
        
        for &size in &self.test_data_sizes {
            let result = self.benchmark_size(hasher.clone(), size)?;
            size_results.insert(size, result);
            
            total_bytes_processed += (size * self.iterations_per_size) as u64;
            total_time += result.total_time;
        // Calculate overall metrics
        let overall_throughput = if total_time.as_nanos() > 0 {
            (total_bytes_processed as f64) / total_time.as_secs_f64()
        } else {
            0.0
        
        // Calculate small input metrics
        let small_metrics = self.calculate_small_input_metrics(&size_results)?;
        
        // Calculate large input metrics  
        let large_metrics = self.calculate_large_input_metrics(&size_results)?;
        
        // Estimate memory usage
        let memory_usage = self.estimate_memory_usage(hasher.clone());
        
        // Calculate efficiency score
        let efficiency_score = self.calculate_efficiency_score(
        );
        
        let total_hashes = (self.test_data_sizes.len() * self.iterations_per_size) as f64;
        let hashes_per_second = total_hashes / total_time.as_secs_f64();
        let avg_latency = total_time / total_hashes as u32;
        
        println!("✅ Benchmark completed in {:?}", start_time.elapsed());
        
        Ok(PerformanceMetrics {
            cpu_cycles_per_byte: None, // Would require platform-specific profiling
        })
    fn warmup<H: Hasher + Clone>(&self, mut hasher: H) {
        let warmup_data = vec![42u8; 1024];
        for _ in 0..self.warmup_iterations {
            hasher.reset();
            let _ = hasher.hash(&warmup_data);
        }
    }
    
    fn benchmark_size<H: Hasher + Clone>(&self, hasher: H, size: usize) -> PerformanceResult<SizeBenchmarkResult> {
        let test_data = self.generate_test_data(size);
        let mut times = Vec::with_capacity(self.iterations_per_size);
        
        // Run iterations
        for _ in 0..self.iterations_per_size {
            let mut test_hasher = hasher.clone();
            let start = Instant::now();
            let _ = test_hasher.hash(&test_data);
            times.push(start.elapsed());
        // Calculate statistics
        times.sort();
        let total_time: Duration = times.iter().sum();
        let average_time = total_time / times.len() as u32;
        let median_time = times[times.len() / 2];
        let p95_time = times[(times.len() as f64 * 0.95) as usize];
        let min_time = times[0];
        let max_time = times[times.len() - 1];
        
        let throughput = if average_time.as_nanos() > 0 {
            (size as f64) / average_time.as_secs_f64()
        } else {
            0.0
        
        Ok(SizeBenchmarkResult {
        })
    fn calculate_small_input_metrics(&self, results: &HashMap<usize, SizeBenchmarkResult>) -> PerformanceResult<SmallInputMetrics> {
        let small_results: Vec<_> = results.iter()
            .filter(|(&size, _)| size <= 1024)
            .map(|(_, result)| result)
            .collect();
        
        if small_results.is_empty() {
            return Err(CursedError::InvalidArgument("No small input results available".to_string()));
        let avg_latency = small_results.iter()
            .map(|r| r.average_time)
            .sum::<Duration>() / small_results.len() as u32;
        
        let median_latency = {
            let mut medians: Vec<_> = small_results.iter().map(|r| r.median_time).collect();
            medians.sort();
            medians[medians.len() / 2]
        
        let p95_latency = small_results.iter()
            .map(|r| r.p95_time)
            .max()
            .unwrap_or(Duration::new(0, 0));
        
        let throughput = small_results.iter()
            .map(|r| r.throughput)
            .sum::<f64>() / small_results.len() as f64;
        
        // Calculate overhead (comparing smallest vs baseline)
        let smallest_result = small_results.iter().min_by_key(|r| r.size).unwrap();
        let baseline_ns_per_byte = 0.1; // Assume 0.1ns baseline per byte
        let actual_ns_per_byte = smallest_result.average_time.as_nanos() as f64 / smallest_result.size as f64;
        let overhead_percentage = ((actual_ns_per_byte - baseline_ns_per_byte) / baseline_ns_per_byte) * 100.0;
        
        Ok(SmallInputMetrics {
        })
    fn calculate_large_input_metrics(&self, results: &HashMap<usize, SizeBenchmarkResult>) -> PerformanceResult<LargeInputMetrics> {
        let large_results: Vec<_> = results.iter()
            .filter(|(&size, _)| size >= 1048576) // 1MB+
            .map(|(_, result)| result)
            .collect();
        
        if large_results.is_empty() {
            return Err(CursedError::InvalidArgument("No large input results available".to_string()));
        let throughput_mb_per_second = large_results.iter()
            .map(|r| r.throughput / 1048576.0) // Convert to MB/s
            .sum::<f64>() / large_results.len() as f64;
        
        // Calculate streaming efficiency (how well performance scales with size)
        let streaming_efficiency = if large_results.len() >= 2 {
            let sorted_results: Vec<_> = large_results.iter()
                .collect::<Vec<_>>();
            
            let throughput_variance = self.calculate_variance(
                sorted_results.iter().map(|r| r.throughput).collect()
            );
            
            // Lower variance = better streaming efficiency
            let max_throughput = sorted_results.iter()
                .map(|r| r.throughput)
                .fold(0.0f64, f64::max);
            
            if max_throughput > 0.0 {
                1.0 - (throughput_variance.sqrt() / max_throughput)
            } else {
                0.0
            }
        } else {
            1.0
        
        // Memory efficiency (inverse of memory usage per throughput)
        let memory_efficiency = 0.8; // Placeholder - would need actual memory measurements
        
        // Scalability factor (how throughput scales with input size)
        let scalability_factor = if large_results.len() >= 2 {
            let mut sorted: Vec<_> = large_results.iter().collect();
            sorted.sort_by_key(|r| r.size);
            
            let first = sorted[0];
            let last = sorted[sorted.len() - 1];
            
            let size_ratio = last.size as f64 / first.size as f64;
            let throughput_ratio = last.throughput / first.throughput;
            
            throughput_ratio / size_ratio
        } else {
            1.0
        
        Ok(LargeInputMetrics {
        })
    fn calculate_variance(&self, values: Vec<f64>) -> f64 {
        if values.is_empty() {
            return 0.0;
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        
        variance
    fn estimate_memory_usage<H: Hasher + Clone>(&self, hasher: H) -> usize {
        // Estimate based on hash state size and algorithm characteristics
        let base_size = std::mem::size_of_val(&hasher);
        let digest_size = hasher.digest_size();
        let algorithm_overhead = match hasher.algorithm() {
            "Keccak-256" => 200, // State size
            _ => 256, // Default estimate
        
        base_size + digest_size + algorithm_overhead
                                large: &LargeInputMetrics, memory_usage: usize) -> f64 {
        // Weighted score combining different performance aspects
        let throughput_score = (throughput / 1_000_000.0).min(100.0); // Normalize to reasonable range
        let latency_score = (1.0 / small.average_latency.as_secs_f64()).min(1000.0);
        let memory_score = (1_000_000.0 / memory_usage as f64).min(100.0);
        let streaming_score = large.streaming_efficiency * 100.0;
        let scalability_score = large.scalability_factor.min(2.0) * 50.0;
        
        // Weighted average
        (throughput_score * 0.3 + 
         latency_score * 0.2 + 
         memory_score * 0.2 + 
         streaming_score * 0.15 + 
         scalability_score * 0.15).min(100.0)
    fn generate_test_data(&self, size: usize) -> Vec<u8> {
        // Generate deterministic but varied test data
        let mut data = Vec::with_capacity(size);
        for i in 0..size {
            data.push((i as u64 * 1103515245 + 12345) as u8);
        }
        data
    }
}

#[derive(Debug, Clone)]
struct SizeBenchmarkResult {
/// Comparative benchmark between multiple hash functions
pub struct ComparativeBenchmark {
impl ComparativeBenchmark {
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Compare multiple hash functions
    pub fn compare_hashers<H: Hasher + Clone>(&self, hashers: Vec<H>) -> PerformanceResult<ComparisonReport> {
        let mut results = Vec::new();
        
        for hasher in hashers {
            let metrics = self.benchmark.benchmark(hasher)?;
            results.push(metrics);
        Ok(ComparisonReport::new(results))
    }
}

/// Performance comparison report
#[derive(Debug, Clone)]
pub struct ComparisonReport {
impl ComparisonReport {
    fn new(mut results: Vec<PerformanceMetrics>) -> Self {
        results.sort_by(|a, b| b.efficiency_score.partial_cmp(&a.efficiency_score).unwrap());
        
        let rankings = PerformanceRankings {
            lowest_latency: Self::find_best_by(&results, |m| 1.0 / m.latency_per_hash.as_secs_f64()),
            lowest_memory: Self::find_best_by(&results, |m| 1.0 / m.memory_usage_bytes as f64),
        
        Self { results, rankings }
    }
    
    fn find_best_by<F>(results: &[PerformanceMetrics], score_fn: F) -> String
    where
    {
        results.iter()
            .max_by(|a, b| score_fn(a).partial_cmp(&score_fn(b)).unwrap())
            .map(|m| m.algorithm.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    /// Generate markdown report
    pub fn to_markdown(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Hash Function Performance Comparison\n\n");
        
        report.push_str("## Overall Rankings\n\n");
        report.push_str(&format!("- **Fastest Throughput**: {}\n", self.rankings.fastest_throughput));
        report.push_str(&format!("- **Lowest Latency**: {}\n", self.rankings.lowest_latency));
        report.push_str(&format!("- **Most Efficient**: {}\n", self.rankings.most_efficient));
        report.push_str(&format!("- **Best Small Input**: {}\n", self.rankings.best_small_input));
        report.push_str(&format!("- **Best Large Input**: {}\n", self.rankings.best_large_input));
        report.push_str(&format!("- **Lowest Memory**: {}\n", self.rankings.lowest_memory));
        
        report.push_str("\n## Detailed Results\n\n");
        report.push_str("| Algorithm | Throughput (MB/s) | Latency (μs) | Efficiency Score | Memory (KB) |\n");
        report.push_str("|-----------|------------------|--------------|------------------|-------------|\n");
        
        for metrics in &self.results {
            report.push_str(&format!(
                metrics.throughput_bytes_per_second / 1_048_576.0,
                metrics.memory_usage_bytes as f64 / 1024.0
            ));
        report
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceRankings {
/// Real-time performance monitor
pub struct PerformanceMonitor<H: Hasher + Clone> {
impl<H: Hasher + Clone> PerformanceMonitor<H> {
    pub fn new(hasher: H) -> Self {
        Self {
        }
    }
    
    /// Record a hash operation and its timing
    pub fn record_operation(&mut self, data: &[u8]) -> Vec<u8> {
        let start = Instant::now();
        let result = self.hasher.clone().hash(data);
        let elapsed = start.elapsed();
        
        self.samples.push(elapsed);
        if self.samples.len() > self.max_samples {
            self.samples.remove(0);
        result
    /// Get current performance statistics
    pub fn get_stats(&self) -> Option<RealtimeStats> {
        if self.samples.is_empty() {
            return None;
        let mut sorted_samples = self.samples.clone();
        sorted_samples.sort();
        
        let average = sorted_samples.iter().sum::<Duration>() / sorted_samples.len() as u32;
        let median = sorted_samples[sorted_samples.len() / 2];
        let p95 = sorted_samples[(sorted_samples.len() as f64 * 0.95) as usize];
        let min = sorted_samples[0];
        let max = sorted_samples[sorted_samples.len() - 1];
        
        Some(RealtimeStats {
        })
    }
}

#[derive(Debug, Clone)]
pub struct RealtimeStats {
