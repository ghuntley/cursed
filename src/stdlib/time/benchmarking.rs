/// Performance measurement and benchmarking utilities
use crate::stdlib::time::error::{TimeError, TimeResult, time_error};
use crate::stdlib::time::duration::Duration;
use crate::stdlib::time::datetime::Instant;
use crate::stdlib::time::sleep::system_time_nanos;
use std::collections::HashMap;

/// Benchmark result containing timing and statistical information
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_time: Duration,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub median_time: Duration,
    pub std_deviation: Duration,
    pub throughput: Option<f64>, // Operations per second
    pub all_times: Vec<Duration>,
}

impl BenchmarkResult {
    /// Create a new benchmark result
    pub fn new(name: String, times: Vec<Duration>) -> TimeResult<Self> {
        if times.is_empty() {
            return Err(time_error("Cannot create benchmark result with no timing data"));
        }
        
        let iterations = times.len();
        let total_time = times.iter().fold(Duration::from_seconds(0), |acc, t| {
            acc.add(t).unwrap_or(acc)
        });
        
        let average_time = total_time.divide(iterations as i64)?;
        let min_time = *times.iter().min().unwrap();
        let max_time = *times.iter().max().unwrap();
        
        // Calculate median
        let mut sorted_times = times.clone();
        sorted_times.sort();
        let median_time = if iterations % 2 == 0 {
            let mid1 = sorted_times[iterations / 2 - 1];
            let mid2 = sorted_times[iterations / 2];
            mid1.add(&mid2)?.divide(2)?
        } else {
            sorted_times[iterations / 2]
        };
        
        // Calculate standard deviation
        let avg_nanos = average_time.total_nanoseconds() as f64;
        let variance = times.iter()
            .map(|t| {
                let diff = t.total_nanoseconds() as f64 - avg_nanos;
                diff * diff
            })
            .sum::<f64>() / iterations as f64;
        let std_deviation = Duration::from_nanoseconds(variance.sqrt() as i64);
        
        // Calculate throughput (operations per second)
        let throughput = if total_time.total_seconds() > 0 {
            Some(iterations as f64 / total_time.total_seconds_f64())
        } else {
            None
        };
        
        Ok(BenchmarkResult {
            name,
            iterations,
            total_time,
            average_time,
            min_time,
            max_time,
            median_time,
            std_deviation,
            throughput,
            all_times: times,
        })
    }
    
    /// Format result as a string
    pub fn format(&self) -> String {
        let mut result = format!("Benchmark: {}\n", self.name);
        result.push_str(&format!("Iterations: {}\n", self.iterations));
        result.push_str(&format!("Total time: {}\n", self.total_time.humanize()));
        result.push_str(&format!("Average time: {}\n", self.average_time.humanize()));
        result.push_str(&format!("Min time: {}\n", self.min_time.humanize()));
        result.push_str(&format!("Max time: {}\n", self.max_time.humanize()));
        result.push_str(&format!("Median time: {}\n", self.median_time.humanize()));
        result.push_str(&format!("Std deviation: {}\n", self.std_deviation.humanize()));
        
        if let Some(throughput) = self.throughput {
            result.push_str(&format!("Throughput: {:.2} ops/sec\n", throughput));
        }
        
        result
    }
    
    /// Get percentile time
    pub fn percentile(&self, p: f64) -> TimeResult<Duration> {
        if !(0.0..=100.0).contains(&p) {
            return Err(time_error("Percentile must be between 0 and 100"));
        }
        
        let mut sorted = self.all_times.clone();
        sorted.sort();
        
        let index = (p / 100.0 * (sorted.len() - 1) as f64).round() as usize;
        Ok(sorted[index])
    }
    
    /// Check if the result is within expected performance bounds
    pub fn is_within_bounds(&self, max_average: Duration, max_std_dev: Duration) -> bool {
        self.average_time <= max_average && self.std_deviation <= max_std_dev
    }
}

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub warmup_iterations: usize,
    pub measurement_iterations: usize,
    pub min_execution_time: Duration,
    pub max_execution_time: Duration,
    pub collect_all_samples: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        BenchmarkConfig {
            warmup_iterations: 10,
            measurement_iterations: 100,
            min_execution_time: Duration::from_milliseconds(1),
            max_execution_time: Duration::from_seconds(30),
            collect_all_samples: true,
        }
    }
}

/// Main benchmark runner
#[derive(Debug)]
pub struct Benchmark {
    config: BenchmarkConfig,
    results: HashMap<String, BenchmarkResult>,
}

impl Benchmark {
    /// Create a new benchmark runner
    pub fn new() -> Self {
        Benchmark {
            config: BenchmarkConfig::default(),
            results: HashMap::new(),
        }
    }
    
    /// Create a benchmark runner with custom config
    pub fn with_config(config: BenchmarkConfig) -> Self {
        Benchmark {
            config,
            results: HashMap::new(),
        }
    }
    
    /// Run a benchmark
    pub fn bench<F, T>(&mut self, name: &str, mut f: F) -> TimeResult<BenchmarkResult>
    where
        F: FnMut() -> T,
    {
        // Warmup phase
        for _ in 0..self.config.warmup_iterations {
            let _ = f();
        }
        
        // Measurement phase
        let mut times = Vec::with_capacity(self.config.measurement_iterations);
        let benchmark_start = Instant::now();
        
        for _ in 0..self.config.measurement_iterations {
            let start = Instant::now();
            let _ = f();
            let elapsed = start.elapsed();
            
            // Check execution time bounds
            if elapsed < self.config.min_execution_time {
                return Err(time_error(&format!(
                    "Execution time {} is below minimum threshold {}",
                    elapsed.humanize(),
                    self.config.min_execution_time.humanize()
                )));
            }
            
            if elapsed > self.config.max_execution_time {
                return Err(time_error(&format!(
                    "Execution time {} exceeds maximum threshold {}",
                    elapsed.humanize(),
                    self.config.max_execution_time.humanize()
                )));
            }
            
            times.push(elapsed);
        }
        
        let result = BenchmarkResult::new(name.to_string(), times)?;
        self.results.insert(name.to_string(), result.clone());
        
        Ok(result)
    }
    
    /// Get benchmark result by name
    pub fn get_result(&self, name: &str) -> Option<&BenchmarkResult> {
        self.results.get(name)
    }
    
    /// Get all benchmark results
    pub fn get_all_results(&self) -> &HashMap<String, BenchmarkResult> {
        &self.results
    }
    
    /// Clear all results
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
    
    /// Generate a summary report
    pub fn summary(&self) -> String {
        if self.results.is_empty() {
            return "No benchmark results available.".to_string();
        }
        
        let mut summary = String::from("Benchmark Summary\n");
        summary.push_str("=================\n\n");
        
        for (name, result) in &self.results {
            summary.push_str(&format!("{}: {} avg, {} min, {} max\n",
                name,
                result.average_time.humanize(),
                result.min_time.humanize(),
                result.max_time.humanize()
            ));
        }
        
        summary
    }
}

impl Default for Benchmark {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple benchmark function
pub fn benchmark<F, T>(name: &str, iterations: usize, mut f: F) -> TimeResult<BenchmarkResult>
where
    F: FnMut() -> T,
{
    let mut times = Vec::with_capacity(iterations);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = f();
        times.push(start.elapsed());
    }
    
    BenchmarkResult::new(name.to_string(), times)
}

/// Time a single execution
pub fn time_it<F, T>(f: F) -> TimeResult<(T, Duration)>
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    Ok((result, elapsed))
}

/// Measure time of a function call
pub fn measure_time<F, T>(f: F) -> Duration
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let _ = f();
    start.elapsed()
}

/// Compare multiple benchmark results
pub fn compare_benchmarks(results: &[BenchmarkResult]) -> String {
    if results.is_empty() {
        return "No benchmark results to compare.".to_string();
    }
    
    if results.len() == 1 {
        return results[0].format();
    }
    
    let mut comparison = String::from("Benchmark Comparison\n");
    comparison.push_str("====================\n\n");
    
    // Find the fastest benchmark for relative comparison
    let fastest = results.iter().min_by_key(|r| r.average_time).unwrap();
    
    comparison.push_str(&format!("Baseline (fastest): {} - {}\n\n", 
        fastest.name, fastest.average_time.humanize()));
    
    for result in results {
        let relative_speed = if result.name == fastest.name {
            1.0
        } else {
            result.average_time.total_seconds_f64() / fastest.average_time.total_seconds_f64()
        };
        
        comparison.push_str(&format!(
            "{}: {} ({:.2}x slower than baseline)\n",
            result.name,
            result.average_time.humanize(),
            relative_speed
        ));
        
        if let (Some(throughput), Some(fastest_throughput)) = (result.throughput, fastest.throughput) {
            comparison.push_str(&format!(
                "  Throughput: {:.2} ops/sec vs {:.2} ops/sec (baseline)\n",
                throughput, fastest_throughput
            ));
        }
    }
    
    comparison
}

/// Benchmark multiple functions against each other
pub fn benchmark_multiple<T>(benchmarks: Vec<(&str, Box<dyn FnMut() -> T>)>, iterations: usize) -> TimeResult<Vec<BenchmarkResult>>
where
    T: 'static,
{
    let mut results = Vec::new();
    
    for (name, mut func) in benchmarks {
        let result = benchmark(name, iterations, &mut func)?;
        results.push(result);
    }
    
    Ok(results)
}

/// Performance counter for tracking multiple metrics
#[derive(Debug)]
pub struct PerformanceCounter {
    counters: HashMap<String, u64>,
    timers: HashMap<String, Duration>,
    start_times: HashMap<String, Instant>,
}

impl PerformanceCounter {
    /// Create a new performance counter
    pub fn new() -> Self {
        PerformanceCounter {
            counters: HashMap::new(),
            timers: HashMap::new(),
            start_times: HashMap::new(),
        }
    }
    
    /// Increment a counter
    pub fn increment(&mut self, name: &str) {
        *self.counters.entry(name.to_string()).or_insert(0) += 1;
    }
    
    /// Add to a counter
    pub fn add(&mut self, name: &str, value: u64) {
        *self.counters.entry(name.to_string()).or_insert(0) += value;
    }
    
    /// Start timing an operation
    pub fn start_timer(&mut self, name: &str) {
        self.start_times.insert(name.to_string(), Instant::now());
    }
    
    /// Stop timing an operation and record the duration
    pub fn stop_timer(&mut self, name: &str) -> Option<Duration> {
        if let Some(start_time) = self.start_times.remove(name) {
            let duration = start_time.elapsed();
            let entry = self.timers.entry(name.to_string()).or_insert(Duration::from_seconds(0));
            *entry = entry.add(&duration).unwrap_or(*entry);
            Some(duration)
        } else {
            None
        }
    }
    
    /// Get counter value
    pub fn get_counter(&self, name: &str) -> u64 {
        self.counters.get(name).copied().unwrap_or(0)
    }
    
    /// Get timer duration
    pub fn get_timer(&self, name: &str) -> Duration {
        self.timers.get(name).copied().unwrap_or(Duration::from_seconds(0))
    }
    
    /// Get all counters
    pub fn get_all_counters(&self) -> &HashMap<String, u64> {
        &self.counters
    }
    
    /// Get all timers
    pub fn get_all_timers(&self) -> &HashMap<String, Duration> {
        &self.timers
    }
    
    /// Reset all counters and timers
    pub fn reset(&mut self) {
        self.counters.clear();
        self.timers.clear();
        self.start_times.clear();
    }
    
    /// Generate a report
    pub fn report(&self) -> String {
        let mut report = String::from("Performance Report\n");
        report.push_str("==================\n\n");
        
        if !self.counters.is_empty() {
            report.push_str("Counters:\n");
            for (name, value) in &self.counters {
                report.push_str(&format!("  {}: {}\n", name, value));
            }
            report.push('\n');
        }
        
        if !self.timers.is_empty() {
            report.push_str("Timers:\n");
            for (name, duration) in &self.timers {
                report.push_str(&format!("  {}: {}\n", name, duration.humanize()));
            }
        }
        
        report
    }
}

impl Default for PerformanceCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro for easy benchmarking
#[macro_export]
macro_rules! bench {
    ($name:expr, $iterations:expr, $code:block) => {
        $crate::stdlib::time::benchmarking::benchmark($name, $iterations, || $code)
    };
}

/// Macro for timing code blocks
#[macro_export]
macro_rules! time_block {
    ($name:expr, $code:block) => {
        {
            let start = $crate::stdlib::time::datetime::Instant::now();
            let result = $code;
            let elapsed = start.elapsed();
            println!("{}: {}", $name, elapsed.humanize());
            result
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    
    #[test]
    fn test_benchmark_result() {
        let times = vec![
            Duration::from_milliseconds(100),
            Duration::from_milliseconds(110),
            Duration::from_milliseconds(90),
            Duration::from_milliseconds(105),
            Duration::from_milliseconds(95),
        ];
        
        let result = BenchmarkResult::new("test".to_string(), times).unwrap();
        
        assert_eq!(result.iterations, 5);
        assert_eq!(result.min_time, Duration::from_milliseconds(90));
        assert_eq!(result.max_time, Duration::from_milliseconds(110));
        assert!(result.throughput.is_some());
    }
    
    #[test]
    fn test_simple_benchmark() {
        let result = benchmark("sleep_test", 5, || {
            thread::sleep(std::time::Duration::from_millis(10));
        }).unwrap();
        
        assert_eq!(result.iterations, 5);
        assert!(result.average_time.total_milliseconds() >= 8);
        assert!(result.average_time.total_milliseconds() <= 50);
    }
    
    #[test]
    fn test_benchmark_runner() {
        let mut bench = Benchmark::new();
        
        let result = bench.bench("test_function", || {
            // Simple computation
            let mut sum = 0;
            for i in 1..=100 {
                sum += i;
            }
            sum
        }).unwrap();
        
        assert_eq!(result.name, "test_function");
        assert!(result.iterations > 0);
    }
    
    #[test]
    fn test_time_it() {
        let (result, duration) = time_it(|| {
            thread::sleep(std::time::Duration::from_millis(10));
            42
        }).unwrap();
        
        assert_eq!(result, 42);
        assert!(duration.total_milliseconds() >= 8);
    }
    
    #[test]
    fn test_performance_counter() {
        let mut counter = PerformanceCounter::new();
        
        counter.increment("operations");
        counter.increment("operations");
        counter.add("bytes", 1024);
        
        counter.start_timer("processing");
        thread::sleep(std::time::Duration::from_millis(10));
        counter.stop_timer("processing");
        
        assert_eq!(counter.get_counter("operations"), 2);
        assert_eq!(counter.get_counter("bytes"), 1024);
        assert!(counter.get_timer("processing").total_milliseconds() >= 8);
    }
    
    #[test]
    fn test_benchmark_comparison() {
        let result1 = BenchmarkResult::new("fast".to_string(), vec![
            Duration::from_milliseconds(10),
            Duration::from_milliseconds(12),
            Duration::from_milliseconds(11),
        ]).unwrap();
        
        let result2 = BenchmarkResult::new("slow".to_string(), vec![
            Duration::from_milliseconds(20),
            Duration::from_milliseconds(22),
            Duration::from_milliseconds(21),
        ]).unwrap();
        
        let comparison = compare_benchmarks(&[result1, result2]);
        assert!(comparison.contains("Benchmark Comparison"));
        assert!(comparison.contains("fast"));
        assert!(comparison.contains("slow"));
    }
}
