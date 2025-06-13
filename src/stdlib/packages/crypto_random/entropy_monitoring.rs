/// Entropy monitoring and quality assessment for cryptographic random number generation
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use super::entropy_sources::EntropySource;

/// Entropy quality metrics
#[derive(Debug, Clone)]
pub struct EntropyQualityMetrics {
    pub shannon_entropy: f64,       // Shannon entropy (0.0 to 8.0 for bytes)
    pub min_entropy: f64,           // Conservative min-entropy estimate
    pub compression_ratio: f64,     // Compression ratio (lower is better)
    pub chi_square_p_value: f64,    // Chi-square test p-value
    pub runs_test_p_value: f64,     // Runs test p-value
    pub longest_run: usize,         // Longest run of identical bits
    pub autocorrelation: f64,       // Autocorrelation coefficient
    pub frequency_deviation: f64,   // Deviation from uniform frequency
    pub quality_score: f64,         // Overall quality score (0.0 to 1.0)
}

/// Entropy source monitoring information
#[derive(Debug, Clone)]
pub struct EntropySourceMonitoring {
    pub source: EntropySource,
    pub last_metrics: Option<EntropyQualityMetrics>,
    pub historical_quality: VecDeque<f64>,
    pub total_bytes_analyzed: u64,
    pub quality_trend: f64,         // Positive = improving, negative = degrading
    pub alert_level: AlertLevel,
    pub last_analysis: Option<SystemTime>,
    pub failure_count: usize,
    pub consecutive_failures: usize,
}

/// Alert levels for entropy quality
#[derive(Debug, Clone, PartialEq)]
pub enum AlertLevel {
    Normal,     // Quality is good
    Warning,    // Quality is degraded but acceptable
    Critical,   // Quality is poor, source should be avoided
    Failed,     // Source is failing consistently
}

/// Entropy monitoring configuration
#[derive(Debug, Clone)]
pub struct EntropyMonitoringConfig {
    pub min_shannon_entropy: f64,       // Minimum acceptable Shannon entropy
    pub min_quality_score: f64,         // Minimum acceptable quality score
    pub max_compression_ratio: f64,     // Maximum acceptable compression ratio
    pub max_autocorrelation: f64,       // Maximum acceptable autocorrelation
    pub min_chi_square_p_value: f64,    // Minimum acceptable chi-square p-value
    pub max_longest_run: usize,         // Maximum acceptable longest run
    pub history_size: usize,            // Number of quality scores to keep
    pub analysis_interval: Duration,    // How often to analyze entropy
    pub alert_threshold_warning: f64,   // Quality score threshold for warnings
    pub alert_threshold_critical: f64,  // Quality score threshold for critical alerts
    pub max_consecutive_failures: usize, // Max failures before marking as failed
}

impl Default for EntropyMonitoringConfig {
    fn default() -> Self {
        Self {
            min_shannon_entropy: 7.0,      // Require good entropy
            min_quality_score: 0.7,        // Require 70% quality
            max_compression_ratio: 0.9,    // Should compress to <90%
            max_autocorrelation: 0.1,      // Low autocorrelation
            min_chi_square_p_value: 0.01,  // 1% significance level
            max_longest_run: 20,           // Max 20 consecutive identical bits
            history_size: 100,             // Keep 100 quality scores
            analysis_interval: Duration::from_secs(5),
            alert_threshold_warning: 0.6,  // Warning at 60%
            alert_threshold_critical: 0.4, // Critical at 40%
            max_consecutive_failures: 3,   // 3 failures = failed source
        }
    }
}

/// Entropy monitor that tracks quality and performance of entropy sources
pub struct EntropyMonitor {
    config: EntropyMonitoringConfig,
    source_monitoring: Arc<Mutex<HashMap<EntropySource, EntropySourceMonitoring>>>,
    global_metrics: Arc<Mutex<Option<EntropyQualityMetrics>>>,
    alert_history: Arc<Mutex<VecDeque<(SystemTime, EntropySource, AlertLevel, String)>>>,
}

impl EntropyMonitor {
    /// Create new entropy monitor
    pub fn new(config: EntropyMonitoringConfig) -> Self {
        Self {
            config,
            source_monitoring: Arc::new(Mutex::new(HashMap::new())),
            global_metrics: Arc::new(Mutex::new(None)),
            alert_history: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    
    /// Analyze entropy quality from a specific source
    pub fn analyze_entropy(&self, source: &EntropySource, data: &[u8]) -> AdvancedCryptoResult<EntropyQualityMetrics> {
        if data.is_empty() {
            return Err("Cannot analyze empty entropy data".into());
        }
        
        let metrics = self.calculate_quality_metrics(data)?;
        
        // Update source monitoring
        self.update_source_monitoring(source, &metrics);
        
        // Check for alerts
        self.check_alerts(source, &metrics);
        
        Ok(metrics)
    }
    
    /// Calculate comprehensive quality metrics for entropy data
    fn calculate_quality_metrics(&self, data: &[u8]) -> AdvancedCryptoResult<EntropyQualityMetrics> {
        let shannon_entropy = self.calculate_shannon_entropy(data);
        let min_entropy = self.estimate_min_entropy(data);
        let compression_ratio = self.calculate_compression_ratio(data)?;
        let chi_square_p_value = self.chi_square_test(data);
        let runs_test_p_value = self.runs_test(data);
        let longest_run = self.find_longest_run(data);
        let autocorrelation = self.calculate_autocorrelation(data);
        let frequency_deviation = self.calculate_frequency_deviation(data);
        
        // Calculate overall quality score
        let quality_score = self.calculate_quality_score(
            shannon_entropy,
            min_entropy,
            compression_ratio,
            chi_square_p_value,
            runs_test_p_value,
            longest_run,
            autocorrelation,
            frequency_deviation,
        );
        
        Ok(EntropyQualityMetrics {
            shannon_entropy,
            min_entropy,
            compression_ratio,
            chi_square_p_value,
            runs_test_p_value,
            longest_run,
            autocorrelation,
            frequency_deviation,
            quality_score,
        })
    }
    
    /// Calculate Shannon entropy
    fn calculate_shannon_entropy(&self, data: &[u8]) -> f64 {
        let mut frequencies = [0u32; 256];
        for &byte in data {
            frequencies[byte as usize] += 1;
        }
        
        let len = data.len() as f64;
        let mut entropy = 0.0;
        
        for &freq in &frequencies {
            if freq > 0 {
                let p = freq as f64 / len;
                entropy -= p * p.log2();
            }
        }
        
        entropy
    }
    
    /// Estimate min-entropy (most conservative entropy measure)
    fn estimate_min_entropy(&self, data: &[u8]) -> f64 {
        let mut frequencies = [0u32; 256];
        for &byte in data {
            frequencies[byte as usize] += 1;
        }
        
        // Find the most frequent symbol
        let max_freq = frequencies.iter().max().unwrap_or(&0);
        
        if *max_freq == 0 {
            return 0.0;
        }
        
        let p_max = *max_freq as f64 / data.len() as f64;
        -p_max.log2() * data.len() as f64
    }
    
    /// Calculate compression ratio using simple RLE
    fn calculate_compression_ratio(&self, data: &[u8]) -> AdvancedCryptoResult<f64> {
        if data.is_empty() {
            return Ok(1.0);
        }
        
        // Simple run-length encoding
        let mut compressed_size = 0;
        let mut i = 0;
        
        while i < data.len() {
            let current = data[i];
            let mut run_length = 1;
            
            while i + run_length < data.len() && data[i + run_length] == current {
                run_length += 1;
            }
            
            // Each run takes 2 bytes (value + length)
            compressed_size += 2;
            i += run_length;
        }
        
        Ok(compressed_size as f64 / data.len() as f64)
    }
    
    /// Chi-square goodness-of-fit test
    fn chi_square_test(&self, data: &[u8]) -> f64 {
        let mut frequencies = [0u32; 256];
        for &byte in data {
            frequencies[byte as usize] += 1;
        }
        
        let expected = data.len() as f64 / 256.0;
        let mut chi_square = 0.0;
        
        for &freq in &frequencies {
            let observed = freq as f64;
            let diff = observed - expected;
            chi_square += diff * diff / expected;
        }
        
        // Convert chi-square statistic to p-value (simplified)
        // For 255 degrees of freedom, this is a rough approximation
        let p_value = if chi_square > 300.0 {
            0.0
        } else if chi_square < 200.0 {
            1.0
        } else {
            (300.0 - chi_square) / 100.0
        };
        
        p_value.max(0.0).min(1.0)
    }
    
    /// Runs test for randomness
    fn runs_test(&self, data: &[u8]) -> f64 {
        if data.len() < 2 {
            return 1.0;
        }
        
        // Convert to binary and count runs
        let mut runs = 1;
        let mut prev_bit = data[0] & 1;
        
        for &byte in &data[1..] {
            let bit = byte & 1;
            if bit != prev_bit {
                runs += 1;
                prev_bit = bit;
            }
        }
        
        // Expected number of runs for random data
        let n = data.len() as f64;
        let expected_runs = (n + 1.0) / 2.0;
        let variance = (n - 1.0) / 4.0;
        
        if variance <= 0.0 {
            return 1.0;
        }
        
        // Z-score
        let z = (runs as f64 - expected_runs) / variance.sqrt();
        
        // Convert to p-value (two-tailed test)
        let p_value = 2.0 * (1.0 - self.normal_cdf(z.abs()));
        p_value.max(0.0).min(1.0)
    }
    
    /// Normal CDF approximation
    fn normal_cdf(&self, x: f64) -> f64 {
        0.5 * (1.0 + (x / std::f64::consts::SQRT_2).tanh())
    }
    
    /// Find longest run of identical bits
    fn find_longest_run(&self, data: &[u8]) -> usize {
        if data.is_empty() {
            return 0;
        }
        
        let mut max_run = 1;
        let mut current_run = 1;
        let mut prev_bit = data[0] & 1;
        
        for &byte in &data[1..] {
            let bit = byte & 1;
            if bit == prev_bit {
                current_run += 1;
                max_run = max_run.max(current_run);
            } else {
                current_run = 1;
                prev_bit = bit;
            }
        }
        
        max_run
    }
    
    /// Calculate autocorrelation
    fn calculate_autocorrelation(&self, data: &[u8]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }
        
        let n = data.len();
        let lag = std::cmp::min(n / 4, 8); // Use lag of 1/4 length or 8, whichever is smaller
        
        if lag == 0 {
            return 0.0;
        }
        
        // Calculate mean
        let mean = data.iter().map(|&x| x as f64).sum::<f64>() / n as f64;
        
        // Calculate autocorrelation at lag
        let mut numerator = 0.0;
        let mut denominator = 0.0;
        
        for i in 0..(n - lag) {
            let x1 = data[i] as f64 - mean;
            let x2 = data[i + lag] as f64 - mean;
            numerator += x1 * x2;
        }
        
        for &byte in data {
            let x = byte as f64 - mean;
            denominator += x * x;
        }
        
        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }
    
    /// Calculate frequency deviation from uniform distribution
    fn calculate_frequency_deviation(&self, data: &[u8]) -> f64 {
        let mut frequencies = [0u32; 256];
        for &byte in data {
            frequencies[byte as usize] += 1;
        }
        
        let expected = data.len() as f64 / 256.0;
        let mut total_deviation = 0.0;
        
        for &freq in &frequencies {
            let deviation = (freq as f64 - expected).abs();
            total_deviation += deviation;
        }
        
        total_deviation / (data.len() as f64)
    }
    
    /// Calculate overall quality score
    fn calculate_quality_score(
        &self,
        shannon_entropy: f64,
        min_entropy: f64,
        compression_ratio: f64,
        chi_square_p_value: f64,
        runs_test_p_value: f64,
        longest_run: usize,
        autocorrelation: f64,
        frequency_deviation: f64,
    ) -> f64 {
        // Normalize each metric to 0-1 scale
        let shannon_score = (shannon_entropy / 8.0).min(1.0);
        let min_entropy_score = (min_entropy / (8.0 * 100.0)).min(1.0); // Assume 100 bytes
        let compression_score = (1.0 - compression_ratio).max(0.0);
        let chi_square_score = chi_square_p_value;
        let runs_score = runs_test_p_value;
        let longest_run_score = (1.0 - (longest_run as f64 / 50.0)).max(0.0);
        let autocorr_score = (1.0 - autocorrelation.abs()).max(0.0);
        let freq_score = (1.0 - frequency_deviation).max(0.0);
        
        // Weighted average
        let weights = [0.2, 0.15, 0.15, 0.15, 0.1, 0.1, 0.1, 0.05];
        let scores = [
            shannon_score,
            min_entropy_score,
            compression_score,
            chi_square_score,
            runs_score,
            longest_run_score,
            autocorr_score,
            freq_score,
        ];
        
        weights.iter()
            .zip(scores.iter())
            .map(|(w, s)| w * s)
            .sum()
    }
    
    /// Update source monitoring information
    fn update_source_monitoring(&self, source: &EntropySource, metrics: &EntropyQualityMetrics) {
        let mut monitoring = self.source_monitoring.lock().unwrap();
        
        let entry = monitoring.entry(source.clone()).or_insert_with(|| {
            EntropySourceMonitoring {
                source: source.clone(),
                last_metrics: None,
                historical_quality: VecDeque::new(),
                total_bytes_analyzed: 0,
                quality_trend: 0.0,
                alert_level: AlertLevel::Normal,
                last_analysis: None,
                failure_count: 0,
                consecutive_failures: 0,
            }
        });
        
        entry.last_metrics = Some(metrics.clone());
        entry.last_analysis = Some(SystemTime::now());
        entry.total_bytes_analyzed += 100; // Assume 100 bytes analyzed
        
        // Update quality history
        entry.historical_quality.push_back(metrics.quality_score);
        if entry.historical_quality.len() > self.config.history_size {
            entry.historical_quality.pop_front();
        }
        
        // Calculate quality trend
        if entry.historical_quality.len() >= 2 {
            let recent = entry.historical_quality.iter().rev().take(10).collect::<Vec<_>>();
            let older = entry.historical_quality.iter().take(10).collect::<Vec<_>>();
            
            if !recent.is_empty() && !older.is_empty() {
                let recent_avg = recent.iter().map(|&&x| x).sum::<f64>() / recent.len() as f64;
                let older_avg = older.iter().map(|&&x| x).sum::<f64>() / older.len() as f64;
                entry.quality_trend = recent_avg - older_avg;
            }
        }
        
        // Update failure tracking
        if metrics.quality_score < self.config.alert_threshold_critical {
            entry.consecutive_failures += 1;
            entry.failure_count += 1;
        } else {
            entry.consecutive_failures = 0;
        }
    }
    
    /// Check for alerts based on quality metrics
    fn check_alerts(&self, source: &EntropySource, metrics: &EntropyQualityMetrics) {
        let mut monitoring = self.source_monitoring.lock().unwrap();
        let mut alerts = self.alert_history.lock().unwrap();
        
        if let Some(entry) = monitoring.get_mut(source) {
            let new_alert_level = if entry.consecutive_failures >= self.config.max_consecutive_failures {
                AlertLevel::Failed
            } else if metrics.quality_score < self.config.alert_threshold_critical {
                AlertLevel::Critical
            } else if metrics.quality_score < self.config.alert_threshold_warning {
                AlertLevel::Warning
            } else {
                AlertLevel::Normal
            };
            
            // Only log alert if level changed
            if new_alert_level != entry.alert_level {
                let message = match new_alert_level {
                    AlertLevel::Normal => "Entropy quality returned to normal".to_string(),
                    AlertLevel::Warning => format!("Entropy quality degraded (score: {:.2})", metrics.quality_score),
                    AlertLevel::Critical => format!("Entropy quality critical (score: {:.2})", metrics.quality_score),
                    AlertLevel::Failed => format!("Entropy source failed after {} consecutive failures", entry.consecutive_failures),
                };
                
                alerts.push_back((SystemTime::now(), source.clone(), new_alert_level.clone(), message));
                
                // Limit alert history
                while alerts.len() > 1000 {
                    alerts.pop_front();
                }
            }
            
            entry.alert_level = new_alert_level;
        }
    }
    
    /// Get monitoring information for a source
    pub fn get_source_monitoring(&self, source: &EntropySource) -> Option<EntropySourceMonitoring> {
        let monitoring = self.source_monitoring.lock().unwrap();
        monitoring.get(source).cloned()
    }
    
    /// Get all monitored sources
    pub fn get_all_sources_monitoring(&self) -> HashMap<EntropySource, EntropySourceMonitoring> {
        self.source_monitoring.lock().unwrap().clone()
    }
    
    /// Get recent alerts
    pub fn get_recent_alerts(&self, count: usize) -> Vec<(SystemTime, EntropySource, AlertLevel, String)> {
        let alerts = self.alert_history.lock().unwrap();
        alerts.iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }
    
    /// Get sources by alert level
    pub fn get_sources_by_alert_level(&self, level: AlertLevel) -> Vec<EntropySource> {
        let monitoring = self.source_monitoring.lock().unwrap();
        monitoring.values()
            .filter(|entry| entry.alert_level == level)
            .map(|entry| entry.source.clone())
            .collect()
    }
    
    /// Get overall system entropy health
    pub fn get_system_health(&self) -> f64 {
        let monitoring = self.source_monitoring.lock().unwrap();
        
        if monitoring.is_empty() {
            return 0.0;
        }
        
        let total_quality: f64 = monitoring.values()
            .filter_map(|entry| entry.last_metrics.as_ref())
            .map(|metrics| metrics.quality_score)
            .sum();
        
        total_quality / monitoring.len() as f64
    }
    
    /// Check if any sources are in critical state
    pub fn has_critical_alerts(&self) -> bool {
        let monitoring = self.source_monitoring.lock().unwrap();
        monitoring.values().any(|entry| {
            matches!(entry.alert_level, AlertLevel::Critical | AlertLevel::Failed)
        })
    }
    
    /// Set global metrics for mixed entropy
    pub fn set_global_metrics(&self, metrics: EntropyQualityMetrics) {
        let mut global = self.global_metrics.lock().unwrap();
        *global = Some(metrics);
    }
    
    /// Get global entropy metrics
    pub fn get_global_metrics(&self) -> Option<EntropyQualityMetrics> {
        self.global_metrics.lock().unwrap().clone()
    }
}

impl Default for EntropyMonitor {
    fn default() -> Self {
        Self::new(EntropyMonitoringConfig::default())
    }
}
