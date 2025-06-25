
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

use crate::error::{CursedError, Result};
use crate::optimization::benchmarks::{BenchmarkResult, BenchmarkSuiteResult};

/// Performance baseline stored for regression analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    /// Unique identifier for this baseline
    /// Human-readable name
    /// Baseline type (commit, release, manual)
    /// Timestamp when baseline was created
    /// Git commit hash (if applicable)
    /// Version or release tag (if applicable)
    /// Performance metrics for each benchmark
    /// Metadata about the baseline
    /// Confidence level of this baseline (0.0 to 1.0)
/// Type of performance baseline
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BaselineType {
    /// Baseline from a specific git commit
    /// Baseline from a release version
    /// Manually created baseline
    /// Continuous integration baseline
    /// Development branch baseline
/// Performance metrics for a single benchmark in a baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineBenchmark {
    /// Benchmark name
    /// Compilation time metrics
    /// Runtime performance metrics (if available)
    /// Binary size in bytes
    /// Peak memory usage in bytes
    /// Number of optimization passes
    /// Additional custom metrics
/// Statistical metrics for time measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeMetrics {
    /// Mean time
    /// Standard deviation
    /// Minimum time observed
    /// Maximum time observed
    /// Number of samples
    /// 95th percentile
/// Baseline storage manager
pub struct BaselineStorage {
    /// Directory where baselines are stored
    /// Currently loaded baselines
    /// Default baseline ID
/// Configuration for baseline storage
#[derive(Debug, Clone)]
pub struct BaselineStorageConfig {
    /// Storage directory path
    /// Maximum number of baselines to keep
    /// Automatically clean old baselines
    /// Minimum confidence level for baselines
impl Default for BaselineStorageConfig {
    fn default() -> Self {
        Self {
            storage_dir: PathBuf::from(".cursed/baselines"),
        }
    }
impl BaselineStorage {
    /// Create a new baseline storage manager
    #[instrument(skip(config))]
    pub fn new(config: BaselineStorageConfig) -> Result<Self> {
        info!("Initializing baseline storage at: {}", config.storage_dir.display());
        
        // Create storage directory if it doesn't exist
        std::fs::create_dir_all(&config.storage_dir)
            .map_err(|e| CursedError::General(format!("Failed to create baseline storage directory: {}", e)))?;

        let mut storage = Self {

        // Load existing baselines
        storage.load_all_baselines()?;

        // Auto-cleanup if enabled
        if config.auto_cleanup {
            storage.cleanup_old_baselines(config.max_baselines)?;
        Ok(storage)
    /// Create a new baseline from benchmark results
    #[instrument(skip(self, suite_result))]
    pub fn create_baseline(
    ) -> Result<String> {
        info!("Creating new baseline: {}", name);

        let baseline_id = self.generate_baseline_id(&name, &baseline_type);
        
        let mut benchmarks = HashMap::new();
        
        // Convert benchmark results to baseline benchmarks
        for result in &suite_result.results {
            let time_metrics = TimeMetrics {
                std_dev: Duration::from_millis(50), // TODO: Calculate from multiple runs

            let runtime_metrics = result.runtime_performance.map(|runtime| TimeMetrics {
            });

            benchmarks.insert(result.name.clone(), BaselineBenchmark {
            });
        let mut metadata = HashMap::new();
        metadata.insert("suite_name".to_string(), suite_result.suite_name.clone());
        metadata.insert("benchmark_count".to_string(), suite_result.results.len().to_string());
                               (suite_result.statistics.successful_benchmarks as f64 / 
                                suite_result.statistics.total_benchmarks as f64) * 100.0));

        let baseline = PerformanceBaseline {

        // Save to storage
        self.save_baseline(&baseline)?;
        self.loaded_baselines.insert(baseline_id.clone(), baseline);

        info!("Baseline created successfully: {}", baseline_id);
        Ok(baseline_id)
    /// Load a specific baseline by ID
    pub fn load_baseline(&mut self, baseline_id: &str) -> Result<Option<&PerformanceBaseline>> {
        if let Some(baseline) = self.loaded_baselines.get(baseline_id) {
            return Ok(Some(baseline));
        let baseline_path = self.get_baseline_path(baseline_id);
        if !baseline_path.exists() {
            return Ok(None);
        let content = std::fs::read_to_string(&baseline_path)
            .map_err(|e| CursedError::General(format!("Failed to read baseline file: {}", e)))?;
        
        let baseline: PerformanceBaseline = serde_json::from_str(&content)
            .map_err(|e| CursedError::General(format!("Failed to parse baseline: {}", e)))?;

        let baseline_id = baseline.baseline_id.clone();
        self.loaded_baselines.insert(baseline_id.clone(), baseline);
        
        Ok(self.loaded_baselines.get(&baseline_id))
    /// Load all baselines from storage
    fn load_all_baselines(&mut self) -> Result<()> {
        debug!("Loading all baselines from storage");
        
        let entries = std::fs::read_dir(&self.storage_dir)
            .map_err(|e| CursedError::General(format!("Failed to read baseline directory: {}", e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| CursedError::General(format!("Failed to read directory entry: {}", e)))?;
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
    /// Save a baseline to storage
    fn save_baseline(&self, baseline: &PerformanceBaseline) -> Result<()> {
        let baseline_path = self.get_baseline_path(&baseline.baseline_id);
        
        let json = serde_json::to_string_pretty(baseline)
            .map_err(|e| CursedError::General(format!("Failed to serialize baseline: {}", e)))?;
        
        std::fs::write(&baseline_path, json)
            .map_err(|e| CursedError::General(format!("Failed to write baseline file: {}", e)))?;
        
        debug!("Baseline saved: {}", baseline_path.display());
        Ok(())
    /// Get all available baselines
    pub fn list_baselines(&self) -> Vec<&PerformanceBaseline> {
        self.loaded_baselines.values().collect()
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
            return Err(CursedError::General(format!("Baseline not found: {}", baseline_id)));
        self.default_baseline_id = Some(baseline_id.clone());
        info!("Default baseline set to: {}", baseline_id);
        Ok(())
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
    /// Clean up old baselines
    fn cleanup_old_baselines(&mut self, max_baselines: usize) -> Result<()> {
        if self.loaded_baselines.len() <= max_baselines {
            return Ok(());
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
            let baseline_path = self.get_baseline_path(&baseline.baseline_id);
            if let Err(e) = std::fs::remove_file(&baseline_path) {
                warn!("Failed to remove baseline file {}: {}", baseline_path.display(), e);
            } else {
                debug!("Removed old baseline: {}", baseline.baseline_id);
            self.loaded_baselines.remove(&baseline.baseline_id);
        Ok(())
    /// Generate a unique baseline ID
    fn generate_baseline_id(&self, name: &str, baseline_type: &BaselineType) -> String {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let type_prefix = match baseline_type {
        
        let sanitized_name = name.chars()
            .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
            .collect::<String>();
        
        format!("{}_{}_{}", type_prefix, sanitized_name, timestamp)
    /// Get the file path for a baseline
    fn get_baseline_path(&self, baseline_id: &str) -> PathBuf {
        self.storage_dir.join(format!("{}.json", baseline_id))
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
        
        success_rate * benchmark_confidence
    /// Export baselines to a portable format
    pub fn export_baselines(&self, export_path: &Path, baseline_ids: Option<Vec<String>>) -> Result<()> {
        let baselines_to_export: Vec<_> = if let Some(ids) = baseline_ids {
            ids.iter()
                .filter_map(|id| self.loaded_baselines.get(id))
                .collect()
        } else {
            self.loaded_baselines.values().collect()

        let export_data = serde_json::to_string_pretty(&baselines_to_export)
            .map_err(|e| CursedError::General(format!("Failed to serialize export data: {}", e)))?;

        std::fs::write(export_path, export_data)
            .map_err(|e| CursedError::General(format!("Failed to write export file: {}", e)))?;

        info!("Exported {} baselines to: {}", baselines_to_export.len(), export_path.display());
        Ok(())
    /// Import baselines from a portable format
    pub fn import_baselines(&mut self, import_path: &Path, overwrite_existing: bool) -> Result<usize> {
        let content = std::fs::read_to_string(import_path)
            .map_err(|e| CursedError::General(format!("Failed to read import file: {}", e)))?;

        let imported_baselines: Vec<PerformanceBaseline> = serde_json::from_str(&content)
            .map_err(|e| CursedError::General(format!("Failed to parse import data: {}", e)))?;

        let mut imported_count = 0;
        for baseline in imported_baselines {
            if !overwrite_existing && self.loaded_baselines.contains_key(&baseline.baseline_id) {
                debug!("Skipping existing baseline: {}", baseline.baseline_id);
                continue;
            self.save_baseline(&baseline)?;
            self.loaded_baselines.insert(baseline.baseline_id.clone(), baseline);
            imported_count += 1;
        info!("Imported {} baselines from: {}", imported_count, import_path.display());
        Ok(imported_count)
    }
}

