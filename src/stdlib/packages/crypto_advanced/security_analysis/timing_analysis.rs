/// fr fr Timing attack detection and analysis module
/// 
/// This module provides comprehensive timing analysis capabilities to detect
/// potential timing vulnerabilities in cryptographic implementations.

use super::super::errors::*;
use std::time::{Duration, Instant};
use std::collections::VecDeque;

/// fr fr Result type for timing analysis
pub type TimingAnalysisResult<(), Error>;

/// fr fr Timing analysis result
#[derive(Debug, Clone)]
pub struct TimingResult {
    pub function_name: String,
    pub sample_count: usize,
    pub mean_time: Duration,
    pub variance: f64,
    pub standard_deviation: f64,
    pub min_time: Duration,
    pub max_time: Duration,
    pub confidence: f64,
    pub is_constant_time: bool,
    pub timing_samples: Vec<Duration>,
}

impl TimingResult {
    /// slay Check if timing vulnerability exists
    pub fn has_timing_vulnerability(&self) -> bool {
        !self.is_constant_time || self.variance > 0.1 || self.confidence < 0.95
    }

    /// slay Get timing variation coefficient
    pub fn variation_coefficient(&self) -> f64 {
        if self.mean_time.as_nanos() == 0 {
            return 0.0;
        }
        self.standard_deviation / (self.mean_time.as_nanos() as f64)
    }
}

/// fr fr Statistical timing analyzer
#[derive(Debug)]
pub struct TimingAnalyzer {
    warmup_iterations: usize,
    cooldown_duration: Duration,
    outlier_threshold: f64,
}

impl TimingAnalyzer {
    /// slay Create new timing analyzer
    pub fn new() -> Self {
        Self {
            warmup_iterations: 100,
            cooldown_duration: Duration::from_millis(1),
            outlier_threshold: 2.0, // Standard deviations
        }
    }

    /// slay Create with custom configuration
    pub fn with_config(warmup_iterations: usize, cooldown_duration: Duration, outlier_threshold: f64) -> Self {
        Self {
            warmup_iterations,
            cooldown_duration,
            outlier_threshold,
        }
    }

    /// slay Analyze timing characteristics of a function
    pub fn analyze_timing<F, R>(&self, name: &str, func: F, sample_count: usize) -> TimingAnalysisResult<TimingResult>
    where
        F: Fn() -> R + Send + Sync,
        R: Send,
    {
        if sample_count < 10 {
            return Err(SecurityAnalysisError::InsufficientData(
                "Need at least 10 samples for timing analysis".to_string()
            ));
        }

        // Warmup phase to stabilize CPU state
        for _ in 0..self.warmup_iterations {
            let _ = func();
        }

        let mut timings = Vec::with_capacity(sample_count);
        let mut min_time = Duration::from_secs(u64::MAX);
        let mut max_time = Duration::ZERO;

        // Collect timing samples
        for i in 0..sample_count {
            // Small cooldown between samples
            if i > 0 {
                std::thread::sleep(self.cooldown_duration);
            }

            let start = Instant::now();
            let _ = func();
            let elapsed = start.elapsed();

            timings.push(elapsed);
            min_time = min_time.min(elapsed);
            max_time = max_time.max(elapsed);
        }

        // Remove outliers using statistical filtering
        let filtered_timings = self.remove_outliers(&timings);
        
        if filtered_timings.len() < sample_count / 2 {
            return Err(SecurityAnalysisError::InsufficientData(
                "Too many outliers removed, insufficient data for analysis".to_string()
            ));
        }

        // Calculate statistics
        let mean_time = self.calculate_mean(&filtered_timings);
        let variance = self.calculate_variance(&filtered_timings, mean_time);
        let standard_deviation = variance.sqrt();
        
        // Determine if timing is constant-time
        let is_constant_time = self.is_constant_time_implementation(&filtered_timings, variance);
        
        // Calculate confidence level
        let confidence = self.calculate_confidence(&filtered_timings, variance);

        Ok(TimingResult {
            function_name: name.to_string(),
            sample_count: filtered_timings.len(),
            mean_time,
            variance,
            standard_deviation,
            min_time,
            max_time,
            confidence,
            is_constant_time,
            timing_samples: filtered_timings,
        })
    }

    /// slay Compare timing between two functions
    pub fn compare_timing<F1, F2, R1, R2>(&self, 
        name1: &str, func1: F1,
        name2: &str, func2: F2,
        sample_count: usize
    ) -> TimingAnalysisResult<TimingComparison>
    where
        F1: Fn() -> R1 + Send + Sync,
        F2: Fn() -> R2 + Send + Sync,
        R1: Send,
        R2: Send,
    {
        let result1 = self.analyze_timing(name1, func1, sample_count)?;
        let result2 = self.analyze_timing(name2, func2, sample_count)?;

        let performance_ratio = result1.mean_time.as_nanos() as f64 / result2.mean_time.as_nanos() as f64;
        let variance_ratio = result1.variance / result2.variance;

        Ok(TimingComparison {
            result1,
            result2,
            performance_ratio,
            variance_ratio,
            timing_difference_significant: self.is_timing_difference_significant(&result1, &result2),
        })
    }

    /// slay Analyze timing with different input sizes for complexity analysis
    pub fn analyze_complexity<F, R>(&self, name: &str, func_generator: impl Fn(usize) -> F, 
                                   input_sizes: &[usize]) -> TimingAnalysisResult<ComplexityAnalysis>
    where
        F: Fn() -> R + Send + Sync,
        R: Send,
    {
        let mut results = Vec::new();
        
        for &size in input_sizes {
            let func = func_generator(size);
            let result = self.analyze_timing(&format!("{}_size_{}", name, size), func, 100)?;
            results.push((size, result));
        }

        // Analyze complexity pattern
        let complexity_pattern = self.determine_complexity_pattern(&results);
        
        Ok(ComplexityAnalysis {
            function_name: name.to_string(),
            size_results: results,
            complexity_pattern,
        })
    }

    /// slay Remove statistical outliers from timing data
    fn remove_outliers(&self, timings: &[Duration]) -> Vec<Duration> {
        if timings.len() < 3 {
            return timings.to_vec();
        }

        let mean = self.calculate_mean(timings);
        let variance = self.calculate_variance(timings, mean);
        let std_dev = variance.sqrt();
        
        let threshold = self.outlier_threshold * std_dev;
        let mean_nanos = mean.as_nanos() as f64;

        timings.iter()
            .filter(|&timing| {
                let diff = (timing.as_nanos() as f64 - mean_nanos).abs();
                diff <= threshold
            })
            .cloned()
            .collect()
    }

    /// slay Calculate mean timing
    fn calculate_mean(&self, timings: &[Duration]) -> Duration {
        let sum_nanos: u128 = timings.iter().map(|d| d.as_nanos()).sum();
        Duration::from_nanos((sum_nanos / timings.len() as u128) as u64)
    }

    /// slay Calculate timing variance
    fn calculate_variance(&self, timings: &[Duration], mean: Duration) -> f64 {
        let mean_nanos = mean.as_nanos() as f64;
        let variance_sum: f64 = timings.iter()
            .map(|timing| {
                let diff = timing.as_nanos() as f64 - mean_nanos;
                diff * diff
            })
            .sum();
        
        variance_sum / timings.len() as f64
    }

    /// slay Determine if implementation is constant-time
    fn is_constant_time_implementation(&self, timings: &[Duration], variance: f64) -> bool {
        // Check if variance is within acceptable bounds for constant-time
        let cv_threshold = 0.05; // 5% coefficient of variation
        let mean_nanos = self.calculate_mean(timings).as_nanos() as f64;
        
        if mean_nanos == 0.0 {
            return false;
        }
        
        let coefficient_of_variation = variance.sqrt() / mean_nanos;
        coefficient_of_variation <= cv_threshold
    }

    /// slay Calculate confidence level for timing analysis
    fn calculate_confidence(&self, timings: &[Duration], variance: f64) -> f64 {
        // Simple confidence calculation based on sample size and variance
        let sample_size = timings.len() as f64;
        let base_confidence = 0.6;
        let size_factor = (sample_size.ln() / 10.0).min(0.3);
        let variance_factor = (-variance / 1000.0).exp().min(0.3);
        
        (base_confidence + size_factor + variance_factor).min(1.0)
    }

    /// slay Check if timing difference between two results is significant
    fn is_timing_difference_significant(&self, result1: &TimingResult, result2: &TimingResult) -> bool {
        // T-test approximation for significance
        let mean1 = result1.mean_time.as_nanos() as f64;
        let mean2 = result2.mean_time.as_nanos() as f64;
        let var1 = result1.variance;
        let var2 = result2.variance;
        let n1 = result1.sample_count as f64;
        let n2 = result2.sample_count as f64;

        let pooled_variance = ((n1 - 1.0) * var1 + (n2 - 1.0) * var2) / (n1 + n2 - 2.0);
        let standard_error = (pooled_variance * (1.0/n1 + 1.0/n2)).sqrt();
        
        if standard_error == 0.0 {
            return false;
        }
        
        let t_statistic = (mean1 - mean2).abs() / standard_error;
        t_statistic > 2.0 // Approximate critical value for 95% confidence
    }

    /// slay Determine algorithmic complexity pattern
    fn determine_complexity_pattern(&self, results: &[(usize, TimingResult)]) -> ComplexityPattern {
        if results.len() < 3 {
            return ComplexityPattern::Unknown;
        }

        // Simple heuristic to determine complexity
        let ratios: Vec<f64> = results.windows(2)
            .map(|window| {
                let (size1, result1) = &window[0];
                let (size2, result2) = &window[1];
                let time_ratio = result2.mean_time.as_nanos() as f64 / result1.mean_time.as_nanos() as f64;
                let size_ratio = *size2 as f64 / *size1 as f64;
                time_ratio / size_ratio
            })
            .collect();

        let avg_ratio = ratios.iter().sum::<f64>() / ratios.len() as f64;

        if avg_ratio < 1.2 {
            ComplexityPattern::Constant
        } else if avg_ratio < 2.0 {
            ComplexityPattern::Linear
        } else if avg_ratio < 4.0 {
            ComplexityPattern::Quadratic
        } else {
            ComplexityPattern::Exponential
        }
    }
}

/// fr fr Timing comparison result
#[derive(Debug, Clone)]
pub struct TimingComparison {
    pub result1: TimingResult,
    pub result2: TimingResult,
    pub performance_ratio: f64,
    pub variance_ratio: f64,
    pub timing_difference_significant: bool,
}

/// fr fr Complexity analysis result
#[derive(Debug, Clone)]
pub struct ComplexityAnalysis {
    pub function_name: String,
    pub size_results: Vec<(usize, TimingResult)>,
    pub complexity_pattern: ComplexityPattern,
}

/// fr fr Algorithmic complexity patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityPattern {
    Constant,
    Linear,
    Quadratic,
    Exponential,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_timing_analyzer_creation() {
        let analyzer = TimingAnalyzer::new();
        assert_eq!(analyzer.warmup_iterations, 100);
        assert_eq!(analyzer.cooldown_duration, Duration::from_millis(1));
        assert_eq!(analyzer.outlier_threshold, 2.0);
    }

    #[test]
    fn test_constant_time_analysis() {
        let analyzer = TimingAnalyzer::new();
        
        // Constant time function
        let result = analyzer.analyze_timing("constant", || {
            42 // Constant operation
        }, 50);
        
        assert!(result.is_ok());
        let timing_result = result.unwrap();
        assert!(timing_result.is_constant_time);
        assert!(timing_result.variance < 1000.0); // Should be very low variance
    }

    #[test]
    fn test_variable_time_analysis() {
        let analyzer = TimingAnalyzer::new();
        let mut counter = 0;
        
        // Variable time function
        let result = analyzer.analyze_timing("variable", || {
            counter += 1;
            if counter % 2 == 0 {
                thread::sleep(Duration::from_nanos(100));
            }
            counter
        }, 20);
        
        assert!(result.is_ok());
        let timing_result = result.unwrap();
        // Should detect timing variation
        assert!(timing_result.variance > 0.0);
    }

    #[test]
    fn test_timing_comparison() {
        let analyzer = TimingAnalyzer::new();
        
        let comparison = analyzer.compare_timing(
            "fast", || 42,
            "slow", || {
                thread::sleep(Duration::from_nanos(50));
                42
            },
            20
        );
        
        assert!(comparison.is_ok());
        let comp_result = comparison.unwrap();
        assert!(comp_result.performance_ratio < 1.0); // First should be faster
    }

    #[test]
    fn test_outlier_removal() {
        let analyzer = TimingAnalyzer::new();
        let timings = vec![
            Duration::from_nanos(100),
            Duration::from_nanos(101),
            Duration::from_nanos(102),
            Duration::from_nanos(1000), // Outlier
            Duration::from_nanos(103),
        ];
        
        let filtered = analyzer.remove_outliers(&timings);
        assert!(filtered.len() < timings.len()); // Should remove outlier
    }
}
