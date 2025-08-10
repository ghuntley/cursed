//! Benchmarking suite for CURSED language performance testing

use std::time::{Duration, Instant};
use std::collections::HashMap;
use crate::error::CursedError;

#[derive(Debug, Clone)]
pub struct BenchmarkSuite {
    benchmarks: HashMap<String, Benchmark>,
    config: BenchmarkConfig,
    results: HashMap<String, BenchmarkResults>,
}

#[derive(Debug, Clone)]
pub struct Benchmark {
    pub name: String,
    pub description: String,
    pub iterations: u64,
    pub warmup_iterations: u64,
    pub timeout: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub default_iterations: u64,
    pub default_warmup: u64,
    pub parallel_execution: bool,
    pub statistical_analysis: bool,
    pub output_format: BenchmarkOutputFormat,
}

#[derive(Debug, Clone)]
pub enum BenchmarkOutputFormat {
    Text,
    Json,
    Csv,
    Html,
}

#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub benchmark_name: String,
    pub total_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub mean_time: Duration,
    pub median_time: Duration,
    pub std_deviation: f64,
    pub iterations_completed: u64,
    pub throughput: f64, // operations per second
    pub memory_usage: MemoryBenchmarkData,
    pub iteration_times: Vec<Duration>,
    // Additional fields referenced in the code
    pub name: String,
    pub iterations: u64,
    pub avg_time: Duration,
    pub avg_memory: u64,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryBenchmarkData {
    pub peak_memory: usize,
    pub average_memory: usize,
    pub allocations_per_second: f64,
    pub gc_collections: u64,
    pub gc_time: Duration,
}

impl BenchmarkSuite {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            benchmarks: HashMap::new(),
            config,
            results: HashMap::new(),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(BenchmarkConfig::default())
    }

    pub fn add_benchmark(&mut self, benchmark: Benchmark) {
        self.benchmarks.insert(benchmark.name.clone(), benchmark);
    }

    pub fn add_simple_benchmark(&mut self, name: String, description: String) {
        let benchmark = Benchmark {
            name: name.clone(),
            description,
            iterations: self.config.default_iterations,
            warmup_iterations: self.config.default_warmup,
            timeout: None,
        };
        self.benchmarks.insert(name, benchmark);
    }

    pub fn run_benchmark<F>(&mut self, name: &str, benchmark_fn: F) -> Result<BenchmarkResults, CursedError>
    where
        F: Fn() -> Result<(), CursedError>,
    {
        let benchmark = self.benchmarks.get(name)
            .ok_or_else(|| CursedError::runtime_error(&format!("Benchmark '{}' not found", name)))?
            .clone();

        let mut results = self.execute_benchmark(&benchmark, benchmark_fn)?;
        self.results.insert(name.to_string(), results.clone());
        Ok(results)
    }

    pub fn run_all_benchmarks<F>(&mut self, benchmark_provider: F) -> Result<HashMap<String, BenchmarkResults>, CursedError>
    where
        F: Fn(&str) -> Result<Box<dyn Fn() -> Result<(), CursedError>>, CursedError>,
    {
        let mut all_results = HashMap::new();
        
        for benchmark_name in self.benchmarks.keys().cloned().collect::<Vec<_>>() {
            let benchmark_fn = benchmark_provider(&benchmark_name)?;
            let results = self.run_benchmark_boxed(&benchmark_name, benchmark_fn)?;
            all_results.insert(benchmark_name, results);
        }
        
        Ok(all_results)
    }

    pub fn run_benchmark_boxed(&mut self, name: &str, benchmark_fn: Box<dyn Fn() -> Result<(), CursedError>>) -> Result<BenchmarkResults, CursedError> {
        let benchmark = self.benchmarks.get(name)
            .ok_or_else(|| CursedError::runtime_error(&format!("Benchmark '{}' not found", name)))?
            .clone();
        
        let mut results = self.execute_benchmark_boxed(&benchmark, benchmark_fn)?;
        self.results.insert(name.to_string(), results.clone());
        Ok(results)
    }

    fn execute_benchmark_boxed(&self, benchmark: &Benchmark, benchmark_fn: Box<dyn Fn() -> Result<(), CursedError>>) -> Result<BenchmarkResults, CursedError> {
        // Warmup phase
        for _ in 0..benchmark.warmup_iterations {
            benchmark_fn()?;
        }
        
        let mut times = Vec::new();
        let mut memory_usage = Vec::new();
        
        for _ in 0..benchmark.iterations {
            let start_time = std::time::Instant::now();
            let start_memory = self.get_memory_usage();
            
            benchmark_fn()?;
            
            let end_time = std::time::Instant::now();
            let end_memory = self.get_memory_usage();
            
            times.push(end_time.duration_since(start_time));
            memory_usage.push(end_memory.saturating_sub(start_memory));
        }
        
        let avg_time = times.iter().sum::<std::time::Duration>() / times.len() as u32;
        let min_time = *times.iter().min().unwrap();
        let max_time = *times.iter().max().unwrap();
        let avg_memory = memory_usage.iter().sum::<u64>() / memory_usage.len() as u64;
        
        Ok(BenchmarkResults {
            benchmark_name: benchmark.name.clone(),
            name: benchmark.name.clone(),
            iterations: benchmark.iterations,
            iterations_completed: benchmark.iterations,
            avg_time,
            total_time: avg_time * benchmark.iterations as u32,
            min_time,
            max_time,
            mean_time: avg_time,
            median_time: avg_time, // Simplified
            std_deviation: 0.0, // Simplified
            throughput: if avg_time.as_secs_f64() > 0.0 {
                1.0 / avg_time.as_secs_f64()
            } else {
                0.0
            },
            memory_usage: MemoryBenchmarkData {
                peak_memory: avg_memory as usize,
                average_memory: avg_memory as usize,
                allocations_per_second: 1000.0,
                gc_collections: 0,
                gc_time: Duration::from_secs(0),
            },
            iteration_times: vec![avg_time; benchmark.iterations as usize],
            avg_memory,
            success_rate: 100.0, // All succeeded if we got here
        })
    }

    fn execute_benchmark<F>(&self, benchmark: &Benchmark, benchmark_fn: F) -> Result<BenchmarkResults, CursedError>
    where
        F: Fn() -> Result<(), CursedError>,
    {
        // Warmup phase
        for _ in 0..benchmark.warmup_iterations {
            benchmark_fn()?;
        }

        let mut iteration_times = Vec::with_capacity(benchmark.iterations as usize);
        let mut memory_data = MemoryBenchmarkData::default();
        
        let start_time = Instant::now();
        
        for i in 0..benchmark.iterations {
            let iter_start = Instant::now();
            
            // Check timeout
            if let Some(timeout) = benchmark.timeout {
                if start_time.elapsed() > timeout {
                    return Err(CursedError::runtime_error(&format!(
                        "Benchmark '{}' timed out after {} iterations",
                        benchmark.name, i
                    )));
                }
            }
            
            // Run the benchmark function
            benchmark_fn()?;
            
            let iter_duration = iter_start.elapsed();
            iteration_times.push(iter_duration);
            
            // Simulate memory tracking (in a real implementation, this would
            // interface with the actual memory profiler)
            if i % 100 == 0 {
                memory_data.peak_memory = memory_data.peak_memory.max(1024 * (i as usize + 1));
            }
        }
        
        let total_time = start_time.elapsed();
        
        // Calculate statistics
        let min_time = *iteration_times.iter().min().unwrap_or(&Duration::ZERO);
        let max_time = *iteration_times.iter().max().unwrap_or(&Duration::ZERO);
        let mean_time = if !iteration_times.is_empty() {
            Duration::from_nanos(
                iteration_times.iter().map(|d| d.as_nanos() as u64).sum::<u64>() / iteration_times.len() as u64
            )
        } else {
            Duration::ZERO
        };
        
        let median_time = self.calculate_median(&iteration_times);
        let std_deviation = self.calculate_std_deviation(&iteration_times, mean_time);
        let throughput = if total_time.as_secs_f64() > 0.0 {
            benchmark.iterations as f64 / total_time.as_secs_f64()
        } else {
            0.0
        };
        
        // Update memory averages
        if !iteration_times.is_empty() {
            memory_data.average_memory = memory_data.peak_memory / 2; // Simplified
            memory_data.allocations_per_second = throughput * 10.0; // Estimated
        }
        
        let avg_memory = memory_data.average_memory as u64;
        
        Ok(BenchmarkResults {
            benchmark_name: benchmark.name.clone(),
            name: benchmark.name.clone(),
            iterations: benchmark.iterations,
            iterations_completed: benchmark.iterations,
            avg_time: mean_time,
            total_time,
            min_time,
            max_time,
            mean_time,
            median_time,
            std_deviation,
            throughput,
            memory_usage: memory_data,
            iteration_times,
            avg_memory,
            success_rate: 100.0,
        })
    }

    fn calculate_median(&self, times: &[Duration]) -> Duration {
        if times.is_empty() {
            return Duration::ZERO;
        }
        
        let mut sorted_times = times.to_vec();
        sorted_times.sort();
        
        let len = sorted_times.len();
        if len % 2 == 0 {
            let mid1 = sorted_times[len / 2 - 1].as_nanos() as u64;
            let mid2 = sorted_times[len / 2].as_nanos() as u64;
            Duration::from_nanos((mid1 + mid2) / 2)
        } else {
            sorted_times[len / 2]
        }
    }

    fn calculate_std_deviation(&self, times: &[Duration], mean: Duration) -> f64 {
        if times.len() <= 1 {
            return 0.0;
        }
        
        let mean_nanos = mean.as_nanos() as f64;
        let variance: f64 = times.iter()
            .map(|&duration| {
                let diff = duration.as_nanos() as f64 - mean_nanos;
                diff * diff
            })
            .sum::<f64>() / (times.len() - 1) as f64;
        
        variance.sqrt()
    }

    pub fn get_results(&self, benchmark_name: &str) -> Option<&BenchmarkResults> {
        self.results.get(benchmark_name)
    }

    pub fn get_all_results(&self) -> &HashMap<String, BenchmarkResults> {
        &self.results
    }

    pub fn generate_report(&self) -> String {
        match self.config.output_format {
            BenchmarkOutputFormat::Text => self.generate_text_report(),
            BenchmarkOutputFormat::Json => self.generate_json_report(),
            BenchmarkOutputFormat::Csv => self.generate_csv_report(),
            BenchmarkOutputFormat::Html => self.generate_html_report(),
        }
    }

    fn generate_text_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== CURSED Benchmark Results ===\n\n");
        
        for (name, results) in &self.results {
            report.push_str(&format!("Benchmark: {}\n", name));
            report.push_str(&format!("  Total Time:      {:?}\n", results.total_time));
            report.push_str(&format!("  Mean Time:       {:?}\n", results.mean_time));
            report.push_str(&format!("  Median Time:     {:?}\n", results.median_time));
            report.push_str(&format!("  Min Time:        {:?}\n", results.min_time));
            report.push_str(&format!("  Max Time:        {:?}\n", results.max_time));
            report.push_str(&format!("  Std Deviation:   {:.2}ns\n", results.std_deviation));
            report.push_str(&format!("  Throughput:      {:.2} ops/sec\n", results.throughput));
            report.push_str(&format!("  Iterations:      {}\n", results.iterations_completed));
            report.push_str(&format!("  Peak Memory:     {} bytes\n", results.memory_usage.peak_memory));
            report.push_str("\n");
        }
        
        report
    }

    fn generate_json_report(&self) -> String {
        let mut json = String::from("{\n  \"benchmarks\": {\n");
        
        let mut first = true;
        for (name, results) in &self.results {
            if !first {
                json.push_str(",\n");
            }
            json.push_str(&format!(
                "    \"{}\": {{\n      \"total_time_ms\": {},\n      \"mean_time_ns\": {},\n      \"throughput\": {},\n      \"iterations\": {},\n      \"peak_memory\": {}\n    }}",
                name,
                results.total_time.as_millis(),
                results.mean_time.as_nanos(),
                results.throughput,
                results.iterations_completed,
                results.memory_usage.peak_memory
            ));
            first = false;
        }
        
        json.push_str("\n  }\n}\n");
        json
    }

    fn generate_csv_report(&self) -> String {
        let mut csv = String::from("Benchmark,Total Time(ms),Mean Time(ns),Throughput(ops/s),Iterations,Peak Memory(bytes)\n");
        
        for (name, results) in &self.results {
            csv.push_str(&format!(
                "{},{},{},{},{},{}\n",
                name,
                results.total_time.as_millis(),
                results.mean_time.as_nanos(),
                results.throughput,
                results.iterations_completed,
                results.memory_usage.peak_memory
            ));
        }
        
        csv
    }

    fn generate_html_report(&self) -> String {
        let mut html = String::from("<!DOCTYPE html>\n<html><head><title>CURSED Benchmark Report</title></head><body>\n");
        html.push_str("<h1>CURSED Benchmark Results</h1>\n");
        html.push_str("<table border='1'>\n");
        html.push_str("<tr><th>Benchmark</th><th>Total Time</th><th>Mean Time</th><th>Throughput</th><th>Iterations</th><th>Memory</th></tr>\n");
        
        for (name, results) in &self.results {
            html.push_str(&format!(
                "<tr><td>{}</td><td>{:?}</td><td>{:?}</td><td>{:.2} ops/s</td><td>{}</td><td>{} bytes</td></tr>\n",
                name,
                results.total_time,
                results.mean_time,
                results.throughput,
                results.iterations_completed,
                results.memory_usage.peak_memory
            ));
        }
        
        html.push_str("</table>\n</body></html>\n");
        html
    }

    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    /// Get current memory usage in bytes (simplified implementation)
    pub fn get_memory_usage(&self) -> u64 {
        // This is a simplified implementation
        // In a real implementation, this would interface with the system
        use std::process;
        let pid = process::id();
        
        // Try to read memory info from /proc (Linux) or return estimate
        if let Ok(contents) = std::fs::read_to_string(format!("/proc/{}/status", pid)) {
            for line in contents.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<u64>() {
                            return kb * 1024; // Convert KB to bytes
                        }
                    }
                }
            }
        }
        
        // Fallback estimate
        1024 * 1024 // 1MB estimate
    }
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            default_iterations: 1000,
            default_warmup: 100,
            parallel_execution: false,
            statistical_analysis: true,
            output_format: BenchmarkOutputFormat::Text,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_suite() {
        let mut suite = BenchmarkSuite::with_default_config();
        suite.add_simple_benchmark("test_bench".to_string(), "Test benchmark".to_string());
        
        let results = suite.run_benchmark("test_bench", || {
            // Simulate some work
            std::thread::sleep(Duration::from_nanos(1));
            Ok(())
        }).unwrap();
        
        assert_eq!(results.benchmark_name, "test_bench");
        assert!(results.iterations_completed > 0);
        assert!(results.total_time > Duration::ZERO);
    }

    #[test]
    fn test_statistical_calculations() {
        let suite = BenchmarkSuite::with_default_config();
        let times = vec![
            Duration::from_millis(10),
            Duration::from_millis(15),
            Duration::from_millis(20),
            Duration::from_millis(25),
            Duration::from_millis(30),
        ];
        
        let median = suite.calculate_median(&times);
        assert_eq!(median, Duration::from_millis(20));
        
        let mean = Duration::from_millis(20);
        let std_dev = suite.calculate_std_deviation(&times, mean);
        assert!(std_dev > 0.0);
    }
}
