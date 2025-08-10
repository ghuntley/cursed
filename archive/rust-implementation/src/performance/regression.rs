//! Performance regression detection system
//! 
//! Automatically detects performance regressions by comparing current
//! performance metrics against historical baselines and alerts on significant changes.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime};
use std::thread;
use crate::error::CursedError;
use crate::performance::{PerformanceConfig, BenchmarkResults, RegressionAlert, RegressionSeverity};

/// Regression detection system
pub struct RegressionDetector {
    config: PerformanceConfig,
    baseline_data: Arc<RwLock<HashMap<String, PerformanceBaseline>>>,
    historical_data: Arc<RwLock<Vec<PerformanceSnapshot>>>,
    alerts: Arc<RwLock<Vec<RegressionAlert>>>,
    thresholds: RegressionThresholds,
    is_running: Arc<RwLock<bool>>,
    monitor_thread: Arc<RwLock<Option<thread::JoinHandle<()>>>>,
}

/// Performance baseline for comparison
#[derive(Debug, Clone)]
pub struct PerformanceBaseline {
    pub metric_name: String,
    pub baseline_value: f64,
    pub baseline_timestamp: SystemTime,
    pub statistical_data: BaselineStatistics,
    pub confidence_interval: ConfidenceInterval,
    pub sample_size: usize,
}

/// Statistical data for baseline
#[derive(Debug, Clone)]
pub struct BaselineStatistics {
    pub mean: f64,
    pub median: f64,
    pub std_deviation: f64,
    pub min: f64,
    pub max: f64,
    pub p95: f64,
    pub p99: f64,
    pub coefficient_of_variation: f64,
}

/// Confidence interval for statistical significance
#[derive(Debug, Clone)]
pub struct ConfidenceInterval {
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64,
}

/// Performance snapshot for historical tracking
#[derive(Debug, Clone)]
pub struct PerformanceSnapshot {
    pub timestamp: SystemTime,
    pub metrics: HashMap<String, f64>,
    pub system_info: SystemInfo,
    pub build_info: BuildInfo,
}

/// System information at time of measurement
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub cpu_model: String,
    pub memory_total: usize,
    pub os_version: String,
    pub load_average: f64,
    pub available_memory: usize,
}

/// Build information for correlation with performance
#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub commit_hash: String,
    pub branch: String,
    pub build_timestamp: SystemTime,
    pub compiler_flags: Vec<String>,
    pub dependencies: HashMap<String, String>,
}

/// Regression detection thresholds
#[derive(Debug)]
pub struct RegressionThresholds {
    pub minor_threshold: f64,      // 5% change
    pub moderate_threshold: f64,   // 10% change
    pub major_threshold: f64,      // 20% change
    pub critical_threshold: f64,   // 50% change
    pub statistical_significance: f64, // p-value threshold
    pub minimum_sample_size: usize,
    pub lookback_period: Duration,
}

/// Regression analysis result
#[derive(Debug, Clone)]
pub struct RegressionAnalysis {
    pub metric_name: String,
    pub current_value: f64,
    pub baseline_value: f64,
    pub change_percentage: f64,
    pub trend: RegressionTrend,
    pub statistical_significance: f64,
    pub confidence_interval: ConfidenceInterval,
    pub recommendation: String,
    pub severity: RegressionSeverity,
}

/// Trend analysis for regression detection
#[derive(Debug, Clone)]
pub enum RegressionTrend {
    Improving,
    Stable,
    Degrading,
    Volatile,
    InsufficientData,
}

/// Regression detection strategy
#[derive(Debug, Clone)]
pub enum DetectionStrategy {
    SimpleThreshold,
    StatisticalSignificance,
    TrendAnalysis,
    MovingAverage,
    Exponential,
}

impl RegressionDetector {
    /// Create a new regression detector
    pub fn new(config: PerformanceConfig) -> Result<Self, CursedError> {
        let thresholds = RegressionThresholds {
            minor_threshold: config.performance_threshold, // 5%
            moderate_threshold: config.performance_threshold * 2.0, // 10%
            major_threshold: config.performance_threshold * 4.0, // 20%
            critical_threshold: config.performance_threshold * 10.0, // 50%
            statistical_significance: 0.05, // 5% p-value
            minimum_sample_size: 30,
            lookback_period: Duration::from_secs(86400 * 7), // 1 week
        };

        Ok(Self {
            config,
            baseline_data: Arc::new(RwLock::new(HashMap::new())),
            historical_data: Arc::new(RwLock::new(Vec::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
            thresholds,
            is_running: Arc::new(RwLock::new(false)),
            monitor_thread: Arc::new(RwLock::new(None)),
        })
    }

    /// Start regression monitoring
    pub fn start(&self) -> Result<(), CursedError> {
        let mut is_running = self.is_running.write().unwrap();
        if *is_running {
            return Err(CursedError::runtime_error("Regression detector already running"));
        }
        *is_running = true;

        let config = self.config.clone();
        let baseline_data = Arc::clone(&self.baseline_data);
        let historical_data = Arc::clone(&self.historical_data);
        let alerts = Arc::clone(&self.alerts);
        let thresholds = self.thresholds.clone();
        let is_running_clone = Arc::clone(&self.is_running);

        let handle = thread::Builder::new()
            .name("regression-detector".to_string())
            .spawn(move || {
                let mut last_check = Instant::now();
                let check_interval = Duration::from_secs(300); // Check every 5 minutes
                
                while *is_running_clone.read().unwrap() {
                    thread::sleep(Duration::from_secs(60)); // Check every minute
                    
                    if last_check.elapsed() >= check_interval {
                        // Perform regression analysis
                        if let Err(e) = Self::perform_regression_analysis(
                            &baseline_data,
                            &historical_data,
                            &alerts,
                            &thresholds,
                        ) {
                            eprintln!("Regression analysis failed: {}", e);
                        }
                        
                        // Clean up old data
                        Self::cleanup_old_data(&historical_data, &thresholds);
                        
                        last_check = Instant::now();
                    }
                }
            })
            .map_err(|e| CursedError::runtime_error(&format!("Failed to start regression detector: {}", e)))?;

        *self.monitor_thread.write().unwrap() = Some(handle);
        Ok(())
    }

    /// Stop regression monitoring
    pub fn stop(&self) -> Result<(), CursedError> {
        *self.is_running.write().unwrap() = false;
        
        if let Some(handle) = self.monitor_thread.write().unwrap().take() {
            handle.join().map_err(|_| CursedError::runtime_error("Failed to join regression detector thread"))?;
        }
        
        Ok(())
    }

    /// Add performance data point
    pub fn add_data_point(&self, metric_name: &str, value: f64) -> Result<(), CursedError> {
        let mut historical_data = self.historical_data.write().unwrap();
        
        // Create or update current snapshot
        if let Some(snapshot) = historical_data.last_mut() {
            if snapshot.timestamp.elapsed().unwrap_or(Duration::MAX) < Duration::from_secs(60) {
                // Update existing snapshot if it's recent
                snapshot.metrics.insert(metric_name.to_string(), value);
                return Ok(());
            }
        }
        
        // Create new snapshot
        let mut metrics = HashMap::new();
        metrics.insert(metric_name.to_string(), value);
        
        let snapshot = PerformanceSnapshot {
            timestamp: SystemTime::now(),
            metrics,
            system_info: Self::collect_system_info(),
            build_info: Self::collect_build_info(),
        };
        
        historical_data.push(snapshot);
        
        // Limit history size
        if historical_data.len() > 10000 {
            historical_data.remove(0);
        }
        
        Ok(())
    }

    /// Update baseline for metric
    pub fn update_baseline(&self, metric_name: &str, values: &[f64]) -> Result<(), CursedError> {
        if values.len() < self.thresholds.minimum_sample_size {
            return Err(CursedError::runtime_error(&format!(
                "Insufficient data points for baseline: {} < {}",
                values.len(),
                self.thresholds.minimum_sample_size
            )));
        }

        let stats = self.calculate_statistics(values);
        let confidence_interval = self.calculate_confidence_interval(values, 0.95);

        let baseline = PerformanceBaseline {
            metric_name: metric_name.to_string(),
            baseline_value: stats.mean,
            baseline_timestamp: SystemTime::now(),
            statistical_data: stats,
            confidence_interval,
            sample_size: values.len(),
        };

        self.baseline_data.write().unwrap().insert(metric_name.to_string(), baseline);
        Ok(())
    }

    /// Detect regressions for all metrics
    pub fn detect_regressions(&self) -> Result<Vec<RegressionAlert>, CursedError> {
        let mut alerts = Vec::new();
        let baseline_data = self.baseline_data.read().unwrap();
        let historical_data = self.historical_data.read().unwrap();

        if historical_data.is_empty() {
            return Ok(alerts);
        }

        let recent_data = &historical_data[historical_data.len().saturating_sub(10)..];

        for (metric_name, baseline) in baseline_data.iter() {
            let recent_values: Vec<f64> = recent_data.iter()
                .filter_map(|snapshot| snapshot.metrics.get(metric_name))
                .cloned()
                .collect();

            if recent_values.is_empty() {
                continue;
            }

            let current_value = recent_values[recent_values.len() - 1];
            let analysis = self.analyze_regression(metric_name, current_value, baseline);

            if analysis.severity != RegressionSeverity::Low {
                alerts.push(RegressionAlert {
                    metric: metric_name.clone(),
                    current_value,
                    baseline_value: baseline.baseline_value,
                    regression_percentage: analysis.change_percentage,
                    severity: analysis.severity,
                    recommendation: analysis.recommendation,
                });
            }
        }

        // Store alerts
        self.alerts.write().unwrap().extend(alerts.clone());

        Ok(alerts)
    }

    /// Analyze regression for a specific metric
    fn analyze_regression(&self, metric_name: &str, current_value: f64, baseline: &PerformanceBaseline) -> RegressionAnalysis {
        let change_percentage = ((current_value - baseline.baseline_value) / baseline.baseline_value) * 100.0;
        let abs_change = change_percentage.abs();

        let severity = if abs_change < self.thresholds.minor_threshold {
            RegressionSeverity::Low
        } else if abs_change < self.thresholds.moderate_threshold {
            RegressionSeverity::Medium
        } else if abs_change < self.thresholds.major_threshold {
            RegressionSeverity::High
        } else {
            RegressionSeverity::Critical
        };

        let trend = if change_percentage > self.thresholds.minor_threshold {
            RegressionTrend::Degrading
        } else if change_percentage < -self.thresholds.minor_threshold {
            RegressionTrend::Improving
        } else {
            RegressionTrend::Stable
        };

        let recommendation = self.generate_recommendation(&trend, severity, change_percentage);

        // Calculate statistical significance (simplified)
        let z_score = (current_value - baseline.baseline_value) / baseline.statistical_data.std_deviation;
        let p_value = 2.0 * (1.0 - self.standard_normal_cdf(z_score.abs()));

        RegressionAnalysis {
            metric_name: metric_name.to_string(),
            current_value,
            baseline_value: baseline.baseline_value,
            change_percentage,
            trend,
            statistical_significance: p_value,
            confidence_interval: baseline.confidence_interval.clone(),
            recommendation,
            severity,
        }
    }

    /// Generate recommendation based on analysis
    fn generate_recommendation(&self, trend: &RegressionTrend, severity: RegressionSeverity, change_percentage: f64) -> String {
        match (trend, severity) {
            (RegressionTrend::Improving, _) => {
                "Performance improved! Consider analyzing recent changes to maintain this improvement.".to_string()
            }
            (RegressionTrend::Stable, RegressionSeverity::Low) => {
                "Performance is stable within acceptable bounds.".to_string()
            }
            (RegressionTrend::Degrading, RegressionSeverity::Medium) => {
                format!("Performance degraded by {:.1}%. Consider reviewing recent changes.", change_percentage)
            }
            (RegressionTrend::Degrading, RegressionSeverity::High) => {
                format!("Significant performance regression detected ({:.1}%). Investigate immediately.", change_percentage)
            }
            (RegressionTrend::Degrading, RegressionSeverity::Critical) => {
                format!("CRITICAL: Performance degraded by {:.1}%. This requires immediate attention!", change_percentage)
            }
            (RegressionTrend::Volatile, _) => {
                "Performance is highly variable. Consider increasing measurement frequency or checking for external factors.".to_string()
            }
            (RegressionTrend::InsufficientData, _) => {
                "Insufficient data for reliable analysis. Continue monitoring.".to_string()
            }
            _ => "Performance monitoring active.".to_string(),
        }
    }

    /// Calculate statistical data for values
    fn calculate_statistics(&self, values: &[f64]) -> BaselineStatistics {
        if values.is_empty() {
            return BaselineStatistics::default();
        }

        let mut sorted_values = values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let median = if sorted_values.len() % 2 == 0 {
            let mid = sorted_values.len() / 2;
            (sorted_values[mid - 1] + sorted_values[mid]) / 2.0
        } else {
            sorted_values[sorted_values.len() / 2]
        };

        let variance = values.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        let std_deviation = variance.sqrt();

        let min = sorted_values[0];
        let max = sorted_values[sorted_values.len() - 1];

        let p95_index = (sorted_values.len() as f64 * 0.95) as usize;
        let p99_index = (sorted_values.len() as f64 * 0.99) as usize;
        let p95 = sorted_values[p95_index.min(sorted_values.len() - 1)];
        let p99 = sorted_values[p99_index.min(sorted_values.len() - 1)];

        let coefficient_of_variation = if mean != 0.0 {
            std_deviation / mean.abs()
        } else {
            0.0
        };

        BaselineStatistics {
            mean,
            median,
            std_deviation,
            min,
            max,
            p95,
            p99,
            coefficient_of_variation,
        }
    }

    /// Calculate confidence interval
    fn calculate_confidence_interval(&self, values: &[f64], confidence_level: f64) -> ConfidenceInterval {
        let stats = self.calculate_statistics(values);
        let n = values.len() as f64;
        let t_value = 1.96; // Approximate t-value for 95% confidence
        let margin_of_error = t_value * (stats.std_deviation / n.sqrt());

        ConfidenceInterval {
            lower_bound: stats.mean - margin_of_error,
            upper_bound: stats.mean + margin_of_error,
            confidence_level,
        }
    }

    /// Standard normal CDF approximation
    fn standard_normal_cdf(&self, x: f64) -> f64 {
        0.5 * (1.0 + self.error_function(x / 2.0_f64.sqrt()))
    }

    /// Error function approximation
    fn error_function(&self, x: f64) -> f64 {
        let a1 = 0.254829592;
        let a2 = -0.284496736;
        let a3 = 1.421413741;
        let a4 = -1.453152027;
        let a5 = 1.061405429;
        let p = 0.3275911;

        let sign = if x < 0.0 { -1.0 } else { 1.0 };
        let x = x.abs();

        let t = 1.0 / (1.0 + p * x);
        let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

        sign * y
    }

    /// Perform regression analysis (static method for thread)
    fn perform_regression_analysis(
        baseline_data: &Arc<RwLock<HashMap<String, PerformanceBaseline>>>,
        historical_data: &Arc<RwLock<Vec<PerformanceSnapshot>>>,
        alerts: &Arc<RwLock<Vec<RegressionAlert>>>,
        thresholds: &RegressionThresholds,
    ) -> Result<(), CursedError> {
        let baseline_data = baseline_data.read().unwrap();
        let historical_data = historical_data.read().unwrap();

        if historical_data.is_empty() {
            return Ok(());
        }

        let recent_data = &historical_data[historical_data.len().saturating_sub(10)..];
        let mut new_alerts = Vec::new();

        for (metric_name, baseline) in baseline_data.iter() {
            let recent_values: Vec<f64> = recent_data.iter()
                .filter_map(|snapshot| snapshot.metrics.get(metric_name))
                .cloned()
                .collect();

            if recent_values.is_empty() {
                continue;
            }

            let current_value = recent_values[recent_values.len() - 1];
            let change_percentage = ((current_value - baseline.baseline_value) / baseline.baseline_value) * 100.0;
            let abs_change = change_percentage.abs();

            if abs_change > thresholds.minor_threshold {
                let severity = if abs_change < thresholds.moderate_threshold {
                    RegressionSeverity::Medium
                } else if abs_change < thresholds.major_threshold {
                    RegressionSeverity::High
                } else {
                    RegressionSeverity::Critical
                };

                let recommendation = if change_percentage > 0.0 {
                    format!("Performance degraded by {:.1}%. Investigate recent changes.", change_percentage)
                } else {
                    format!("Performance improved by {:.1}%. Consider analyzing what changed.", change_percentage.abs())
                };

                new_alerts.push(RegressionAlert {
                    metric: metric_name.clone(),
                    current_value,
                    baseline_value: baseline.baseline_value,
                    regression_percentage: change_percentage,
                    severity,
                    recommendation,
                });
            }
        }

        // Store new alerts
        if !new_alerts.is_empty() {
            alerts.write().unwrap().extend(new_alerts);
        }

        Ok(())
    }

    /// Clean up old data
    fn cleanup_old_data(
        historical_data: &Arc<RwLock<Vec<PerformanceSnapshot>>>,
        thresholds: &RegressionThresholds,
    ) {
        let mut data = historical_data.write().unwrap();
        let cutoff_time = SystemTime::now() - thresholds.lookback_period;

        data.retain(|snapshot| snapshot.timestamp >= cutoff_time);
    }

    /// Collect system information
    fn collect_system_info() -> SystemInfo {
        SystemInfo {
            cpu_model: "Unknown CPU".to_string(),
            memory_total: 1024 * 1024 * 1024 * 8, // 8GB
            os_version: std::env::consts::OS.to_string(),
            load_average: 1.0,
            available_memory: 1024 * 1024 * 1024 * 4, // 4GB
        }
    }

    /// Collect build information
    fn collect_build_info() -> BuildInfo {
        BuildInfo {
            commit_hash: "unknown".to_string(),
            branch: "main".to_string(),
            build_timestamp: SystemTime::now(),
            compiler_flags: vec!["-O2".to_string()],
            dependencies: HashMap::new(),
        }
    }

    /// Get all alerts
    pub fn get_alerts(&self) -> Vec<RegressionAlert> {
        self.alerts.read().unwrap().clone()
    }

    /// Clear all alerts
    pub fn clear_alerts(&self) {
        self.alerts.write().unwrap().clear();
    }

    /// Get baseline data
    pub fn get_baselines(&self) -> HashMap<String, PerformanceBaseline> {
        self.baseline_data.read().unwrap().clone()
    }

    /// Get historical data
    pub fn get_historical_data(&self) -> Vec<PerformanceSnapshot> {
        self.historical_data.read().unwrap().clone()
    }
}

impl Default for BaselineStatistics {
    fn default() -> Self {
        Self {
            mean: 0.0,
            median: 0.0,
            std_deviation: 0.0,
            min: 0.0,
            max: 0.0,
            p95: 0.0,
            p99: 0.0,
            coefficient_of_variation: 0.0,
        }
    }
}

impl Clone for RegressionThresholds {
    fn clone(&self) -> Self {
        Self {
            minor_threshold: self.minor_threshold,
            moderate_threshold: self.moderate_threshold,
            major_threshold: self.major_threshold,
            critical_threshold: self.critical_threshold,
            statistical_significance: self.statistical_significance,
            minimum_sample_size: self.minimum_sample_size,
            lookback_period: self.lookback_period,
        }
    }
}
