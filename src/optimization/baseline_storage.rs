
// Baseline Performance Storage and Management System
//
// Provides persistent storage and management of performance baselines for
// regression analysis and performance tracking over time.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn, instrument};
use chrono::{DateTime, Utc};

use crate::error::{Error, Result};
use crate::optimization::benchmarks::{BenchmarkResult, BenchmarkSuiteResult};

/// Performance baseline stored for regression analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    /// Unique identifier for this baseline
    pub baseline_id: String,
    /// Human-readable name
    pub name: String,
    /// Baseline type (commit, release, manual)
    pub baseline_type: BaselineType,
    /// Timestamp when baseline was created
    pub created_at: DateTime<Utc>,
    /// Git commit hash (if applicable)
    pub git_commit: Option<String>,
    /// Version or release tag (if applicable)
    pub version: Option<String>,
    /// Performance metrics for each benchmark
    pub benchmarks: HashMap<String, BaselineBenchmark>,
    /// Metadata about the baseline
    pub metadata: HashMap<String, String>,
    /// Confidence level of this baseline (0.0 to 1.0)
    pub confidence_level: f64,
}

/// Type of performance baseline
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BaselineType {
    /// Baseline from a specific git commit
    GitCommit,
    /// Baseline from a release version
    Release,
    /// Manually created baseline
    Manual,
    /// Continuous integration baseline
    CI,
    /// Development branch baseline
    Development,
}

/// Performance metrics for a single benchmark in a baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineBenchmark {
    /// Benchmark name
    pub name: String,
    /// Compilation time metrics
    pub compile_time_metrics: TimeMetrics,
    /// Runtime performance metrics (if available)
    pub runtime_metrics: Option<TimeMetrics>,
    /// Binary size in bytes
    pub binary_size: usize,
    /// Peak memory usage in bytes
    pub peak_memory_usage: usize,
    /// Number of optimization passes
    pub optimization_passes: usize,
    /// Additional custom metrics
    pub custom_metrics: HashMap<String, f64>,
}

/// Statistical metrics for time measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeMetrics {
    /// Mean time
    pub mean: Duration,
    /// Standard deviation
    pub std_dev: Duration,
    /// Minimum time observed
    pub min: Duration,
    /// Maximum time observed
    pub max: Duration,
    /// Number of samples
    pub sample_count: usize,
    /// 95th percentile
    pub percentile_95: Duration,
}

/// Baseline storage manager
pub struct BaselineStorage {
    /// Directory where baselines are stored
    storage_dir: PathBuf,
    /// Currently loaded baselines
    loaded_baselines: HashMap<String, PerformanceBaseline>,
    /// Default baseline ID
    default_baseline_id: Option<String>,
}

/// Configuration for baseline storage
#[derive(Debug, Clone)]
pub struct BaselineStorageConfig {
    /// Storage directory path
    pub storage_dir: PathBuf,
    /// Maximum number of baselines to keep
    pub max_baselines: usize,
    /// Automatically clean old baselines
    pub auto_cleanup: bool,
    /// Minimum confidence level for baselines
    pub min_confidence_level: f64,
}

impl Default for BaselineStorageConfig {
    fn default() -> Self {
        Self {
            storage_dir: PathBuf::from(".cursed/baselines"),
            max_baselines: 50,
            auto_cleanup: true,
            min_confidence_level: 0.7,
        }
    }
}

impl BaselineStorage {
    /// Create a new baseline storage manager
    #[instrument(skip(config))]
    pub fn new(config: BaselineStorageConfig) -> Result<Self> {
        info!("Initializing baseline storage at: {}", config.storage_dir.display());
        
        // Create storage directory if it doesn't exist
        std::fs::create_dir_all(&config.storage_dir)
            .map_err(|e| Error::General(format!("Failed to create baseline storage directory: {}", e)))?;

        let mut storage = Self {
            storage_dir: config.storage_dir,
            loaded_baselines: HashMap::new(),
            default_baseline_id: None,
        };

        // Load existing baselines
        storage.load_all_baselines()?;

        // Auto-cleanup if enabled
        if config.auto_cleanup {
            storage.cleanup_old_baselines(config.max_baselines)?;
        }

        Ok(storage)
    }

    /// Create a new baseline from benchmark results
    #[instrument(skip(self, suite_result))]
    pub fn create_baseline(
        &mut self,
        name: String,
        baseline_type: BaselineType,
        suite_result: &BenchmarkSuiteResult,
        git_commit: Option<String>,
        version: Option<String>,
    ) -> Result<String> {
        info!("Creating new baseline: {}", name);

        let baseline_id = self.generate_baseline_id(&name, &baseline_type);
        
        let mut benchmarks = HashMap::new();
        
        // Convert benchmark results to baseline benchmarks
        for result in &suite_result.results {
            let time_metrics = TimeMetrics {
                mean: result.compile_time,
                std_dev: Duration::from_millis(50), // TODO: Calculate from multiple runs
                min: result.compile_time,
                max: result.compile_time,
                sample_count: 1,
                percentile_95: result.compile_time,
            };

            let runtime_metrics = result.runtime_performance.map(|runtime| TimeMetrics {
                mean: runtime,
                std_dev: Duration::from_millis(10),
                min: runtime,
                max: runtime,
                sample_count: 1,
                percentile_95: runtime,
            });

            benchmarks.insert(result.name.clone(), BaselineBenchmark {
                name: result.name.clone(),
                compile_time_metrics: time_metrics,
                runtime_metrics,
                binary_size: result.binary_size,
                peak_memory_usage: result.peak_memory_usage,
                optimization_passes: result.optimization_passes,
                custom_metrics: HashMap::new(),
            });
        }

        let mut metadata = HashMap::new();
        metadata.insert("suite_name".to_string(), suite_result.suite_name.clone());
        metadata.insert("benchmark_count".to_string(), suite_result.results.len().to_string());
        metadata.insert("success_rate".to_string(), 
                        format!("{:.1}%", 
                               (suite_result.statistics.successful_benchmarks as f64 / 
                                suite_result.statistics.total_benchmarks as f64) * 100.0));

        let baseline = PerformanceBaseline {
            baseline_id: baseline_id.clone(),
            name,
            baseline_type,
            created_at: Utc::now(),
            git_commit,
            version,
            benchmarks,
            metadata,
            confidence_level: self.calculate_confidence_level(suite_result),
        };

        // Save to storage
        self.save_baseline(&baseline)?;
        self.loaded_baselines.insert(baseline_id.clone(), baseline);

        info!("Baseline created successfully: {}", baseline_id);
        Ok(baseline_id)
    }

    /// Load a specific baseline by ID
    pub fn load_baseline(&mut self, baseline_id: &str) -> Result<Option<&PerformanceBaseline>> {
        if let Some(baseline) = self.loaded_baselines.get(baseline_id) {
            return Ok(Some(baseline));
        }

        let baseline_path = self.get_baseline_path(baseline_id);
        if !baseline_path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&baseline_path)
            .map_err(|e| Error::General(format!("Failed to read baseline file: {}", e)))?;
        
        let baseline: PerformanceBaseline = serde_json::from_str(&content)
            .map_err(|e| Error::General(format!("Failed to parse baseline: {}", e)))?;

        let baseline_id = baseline.baseline_id.clone();
        self.loaded_baselines.insert(baseline_id.clone(), baseline);
        
        Ok(self.loaded_baselines.get(&baseline_id))
    }

    /// Load all baselines from storage
    fn load_all_baselines(&mut self) -> Result<()> {
        debug!("Loading all baselines from storage");
        
        let entries = std::fs::read_dir(&self.storage_dir)
            .map_err(|e| Error::General(format!("Failed to read baseline directory: {}", e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| Error::General(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Err(e) = self.load_baseline(stem) {
                        warn!("Failed to load baseline {}: {}", stem, e);
                    }
                }
            }
        }

        info!("Loaded {} baselines", self.loaded_baselines.len());
        Ok(())
    }

    /// Save a baseline to storage
    fn save_baseline(&self, baseline: &PerformanceBaseline) -> Result<()> {
        let baseline_path = self.get_baseline_path(&baseline.baseline_id);
        
        let json = serde_json::to_string_pretty(baseline)
            .map_err(|e| Error::General(format!("Failed to serialize baseline: {}", e)))?;
        
        std::fs::write(&baseline_path, json)
            .map_err(|e| Error::General(format!("Failed to write baseline file: {}", e)))?;
        
        debug!("Baseline saved: {}", baseline_path.display());
        Ok(())
    }

    /// Get all available baselines
    pub fn list_baselines(&self) -> Vec<&PerformanceBaseline> {
        self.loaded_baselines.values().collect()
    }

    /// Get the default baseline
    pub fn get_default_baseline(&self) -> Option<&PerformanceBaseline> {
        if let Some(ref default_id) = self.default_baseline_id {
            self.loaded_baselines.get(default_id)
        } else {
            // Return the most recent release baseline, or most recent baseline
            self.loaded_baselines.values()
                .filter(|b| b.baseline_type == BaselineType::Release)
                .max_by_key(|b| b.created_at)
                .or_else(|| self.loaded_baselines.values().max_by_key(|b| b.created_at))
        }
    }

    /// Set the default baseline
    pub fn set_default_baseline(&mut self, baseline_id: String) -> Result<()> {
        if !self.loaded_baselines.contains_key(&baseline_id) {
            return Err(Error::General(format!("Baseline not found: {}", baseline_id)));
        }
        
        self.default_baseline_id = Some(baseline_id.clone());
        info!("Default baseline set to: {}", baseline_id);
        Ok(())
    }

    /// Find the most suitable baseline for comparison
    pub fn find_comparison_baseline(&self, benchmark_name: &str) -> Option<&PerformanceBaseline> {
        // Priority order:
        // 1. Default baseline if it has this benchmark
        // 2. Most recent release baseline with this benchmark
        // 3. Most recent baseline with this benchmark
        
        if let Some(default) = self.get_default_baseline() {
            if default.benchmarks.contains_key(benchmark_name) {
                return Some(default);
            }
        }

        self.loaded_baselines.values()
            .filter(|b| b.baseline_type == BaselineType::Release && b.benchmarks.contains_key(benchmark_name))
            .max_by_key(|b| b.created_at)
            .or_else(|| {
                self.loaded_baselines.values()
                    .filter(|b| b.benchmarks.contains_key(benchmark_name))
                    .max_by_key(|b| b.created_at)
            })
    }

    /// Clean up old baselines
    fn cleanup_old_baselines(&mut self, max_baselines: usize) -> Result<()> {
        if self.loaded_baselines.len() <= max_baselines {
            return Ok(());
        }

        info!("Cleaning up old baselines, keeping {} most recent", max_baselines);

        // Sort baselines by creation time, keeping the most recent
        let mut baselines: Vec<_> = self.loaded_baselines.values().collect();
        baselines.sort_by_key(|b| b.created_at);
        
        let to_remove = baselines.len() - max_baselines;
        for baseline in baselines.iter().take(to_remove) {
            // Don't remove release baselines or the default baseline
            if baseline.baseline_type == BaselineType::Release ||
               Some(&baseline.baseline_id) == self.default_baseline_id.as_ref() {
                continue;
            }

            let baseline_path = self.get_baseline_path(&baseline.baseline_id);
            if let Err(e) = std::fs::remove_file(&baseline_path) {
                warn!("Failed to remove baseline file {}: {}", baseline_path.display(), e);
            } else {
                debug!("Removed old baseline: {}", baseline.baseline_id);
            }
            
            self.loaded_baselines.remove(&baseline.baseline_id);
        }

        Ok(())
    }

    /// Generate a unique baseline ID
    fn generate_baseline_id(&self, name: &str, baseline_type: &BaselineType) -> String {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let type_prefix = match baseline_type {
            BaselineType::GitCommit => "commit",
            BaselineType::Release => "release",
            BaselineType::Manual => "manual",
            BaselineType::CI => "ci",
            BaselineType::Development => "dev",
        };
        
        let sanitized_name = name.chars()
            .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
            .collect::<String>();
        
        format!("{}_{}_{}", type_prefix, sanitized_name, timestamp)
    }

    /// Get the file path for a baseline
    fn get_baseline_path(&self, baseline_id: &str) -> PathBuf {
        self.storage_dir.join(format!("{}.json", baseline_id))
    }

    /// Calculate confidence level for a baseline
    fn calculate_confidence_level(&self, suite_result: &BenchmarkSuiteResult) -> f64 {
        let success_rate = suite_result.statistics.successful_benchmarks as f64 / 
                          suite_result.statistics.total_benchmarks as f64;
        
        // Base confidence on success rate and number of benchmarks
        let benchmark_confidence = if suite_result.statistics.total_benchmarks >= 10 {
            0.9
        } else if suite_result.statistics.total_benchmarks >= 5 {
            0.8
        } else {
            0.7
        };
        
        success_rate * benchmark_confidence
    }

    /// Export baselines to a portable format
    pub fn export_baselines(&self, export_path: &Path, baseline_ids: Option<Vec<String>>) -> Result<()> {
        let baselines_to_export: Vec<_> = if let Some(ids) = baseline_ids {
            ids.iter()
                .filter_map(|id| self.loaded_baselines.get(id))
                .collect()
        } else {
            self.loaded_baselines.values().collect()
        };

        let export_data = serde_json::to_string_pretty(&baselines_to_export)
            .map_err(|e| Error::General(format!("Failed to serialize export data: {}", e)))?;

        std::fs::write(export_path, export_data)
            .map_err(|e| Error::General(format!("Failed to write export file: {}", e)))?;

        info!("Exported {} baselines to: {}", baselines_to_export.len(), export_path.display());
        Ok(())
    }

    /// Import baselines from a portable format
    pub fn import_baselines(&mut self, import_path: &Path, overwrite_existing: bool) -> Result<usize> {
        let content = std::fs::read_to_string(import_path)
            .map_err(|e| Error::General(format!("Failed to read import file: {}", e)))?;

        let imported_baselines: Vec<PerformanceBaseline> = serde_json::from_str(&content)
            .map_err(|e| Error::General(format!("Failed to parse import data: {}", e)))?;

        let mut imported_count = 0;
        for baseline in imported_baselines {
            if !overwrite_existing && self.loaded_baselines.contains_key(&baseline.baseline_id) {
                debug!("Skipping existing baseline: {}", baseline.baseline_id);
                continue;
            }

            self.save_baseline(&baseline)?;
            self.loaded_baselines.insert(baseline.baseline_id.clone(), baseline);
            imported_count += 1;
        }

        info!("Imported {} baselines from: {}", imported_count, import_path.display());
        Ok(imported_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::optimization::benchmarks::{BenchmarkStatistics, BenchmarkResult};
    use crate::common::optimization_level::OptimizationLevel;

    fn create_test_suite_result() -> BenchmarkSuiteResult {
        let results = vec![
            BenchmarkResult {
                name: "test_benchmark".to_string(),
                optimization_level: OptimizationLevel::O2,
                compile_time: Duration::from_secs(2),
                runtime_performance: Some(Duration::from_millis(500)),
                binary_size: 1024,
                peak_memory_usage: 8192,
                optimization_passes: 10,
                success: true,
                error_message: None,
            }
        ];

        BenchmarkSuiteResult {
            suite_name: "test_suite".to_string(),
            timestamp: Utc::now(),
            results,
            statistics: BenchmarkStatistics {
                total_benchmarks: 1,
                successful_benchmarks: 1,
                avg_compile_time: Duration::from_secs(2),
                avg_performance_improvement: 15.0,
                avg_size_change: -5.0,
                best_optimization_level: OptimizationLevel::O2,
            },
            regression_analysis: None,
        }
    }

    #[test]
    fn test_baseline_storage_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = BaselineStorageConfig {
            storage_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let storage = BaselineStorage::new(config);
        assert!(storage.is_ok());
    }

    #[test]
    fn test_baseline_creation_and_loading() {
        let temp_dir = TempDir::new().unwrap();
        let config = BaselineStorageConfig {
            storage_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let mut storage = BaselineStorage::new(config).unwrap();
        let suite_result = create_test_suite_result();

        let baseline_id = storage.create_baseline(
            "test_baseline".to_string(),
            BaselineType::Manual,
            &suite_result,
            Some("abc123".to_string()),
            None,
        ).unwrap();

        let loaded_baseline = storage.load_baseline(&baseline_id).unwrap();
        assert!(loaded_baseline.is_some());
        
        let baseline = loaded_baseline.unwrap();
        assert_eq!(baseline.name, "test_baseline");
        assert_eq!(baseline.baseline_type, BaselineType::Manual);
        assert!(baseline.benchmarks.contains_key("test_benchmark"));
    }

    #[test]
    fn test_default_baseline_management() {
        let temp_dir = TempDir::new().unwrap();
        let config = BaselineStorageConfig {
            storage_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let mut storage = BaselineStorage::new(config).unwrap();
        let suite_result = create_test_suite_result();

        let baseline_id = storage.create_baseline(
            "test_baseline".to_string(),
            BaselineType::Release,
            &suite_result,
            None,
            Some("v1.0.0".to_string()),
        ).unwrap();

        storage.set_default_baseline(baseline_id.clone()).unwrap();
        
        let default_baseline = storage.get_default_baseline();
        assert!(default_baseline.is_some());
        assert_eq!(default_baseline.unwrap().baseline_id, baseline_id);
    }
}
