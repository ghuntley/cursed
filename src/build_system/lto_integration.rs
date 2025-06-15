/// Build System LTO Integration for CURSED
/// 
/// Integrates Link-Time Optimization with the CURSED build system,
/// providing seamless LTO support across the entire compilation pipeline.

use crate::error::{Error, Result};
use crate::optimization::lto::{LtoOptimizer, LtoConfig, LtoLevel, LtoCompilationUnit, LtoStatistics};
use crate::build_system::{BuildConfig, BuildResult, BuildStatistics, BuildOrchestrator};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{info, instrument, warn};

/// LTO build integration configuration
#[derive(Debug, Clone)]
pub struct LtoBuildConfig {
    /// LTO configuration
    pub lto_config: LtoConfig,
    /// Enable LTO for release builds only
    pub release_only: bool,
    /// LTO output directory
    pub output_directory: PathBuf,
    /// Enable incremental LTO
    pub enable_incremental: bool,
    /// LTO cache size limit (in MB)
    pub cache_size_limit: usize,
    /// Enable LTO profiling and analysis
    pub enable_profiling: bool,
    /// Maximum time to spend on LTO (timeout)
    pub max_lto_time: Duration,
    /// Enable parallel LTO processing
    pub enable_parallel: bool,
}

impl Default for LtoBuildConfig {
    fn default() -> Self {
        Self {
            lto_config: LtoConfig::default(),
            release_only: true,
            output_directory: PathBuf::from("target/lto"),
            enable_incremental: true,
            cache_size_limit: 1024, // 1GB default
            enable_profiling: false,
            max_lto_time: Duration::from_secs(300), // 5 minutes default
            enable_parallel: true,
        }
    }
}

/// LTO build integration manager
pub struct LtoBuildIntegration {
    config: LtoBuildConfig,
    build_cache: Arc<Mutex<LtoBuildCache>>,
    statistics: Arc<Mutex<LtoBuildStatistics>>,
}

impl LtoBuildIntegration {
    /// Create new LTO build integration
    #[instrument(skip(config))]
    pub fn new(config: LtoBuildConfig) -> Result<Self> {
        info!("Initializing LTO build integration");

        // Create output directory
        std::fs::create_dir_all(&config.output_directory)
            .map_err(|e| Error::Other(format!("Failed to create LTO output directory: {}", e)))?;

        let cache = LtoBuildCache::new(&config)?;

        Ok(Self {
            config,
            build_cache: Arc::new(Mutex::new(cache)),
            statistics: Arc::new(Mutex::new(LtoBuildStatistics::default())),
        })
    }

    /// Integrate LTO into build orchestrator
    #[instrument(skip(self, orchestrator))]
    pub fn integrate_with_orchestrator(&self, orchestrator: &mut BuildOrchestrator) -> Result<()> {
        info!("Integrating LTO with build orchestrator");

        // Register LTO as a post-compilation step
        // In a real implementation, this would hook into the build pipeline
        // to run LTO after all compilation units are complete

        Ok(())
    }

    /// Run LTO optimization on build artifacts
    #[instrument(skip(self, compilation_units))]
    pub fn run_lto_optimization(&mut self, compilation_units: &[CompilationArtifact]) -> Result<LtoOptimizationResult> {
        let start_time = Instant::now();
        
        // Check if LTO should be enabled for this build
        if !self.should_run_lto(compilation_units)? {
            info!("LTO optimization skipped");
            return Ok(LtoOptimizationResult::skipped());
        }

        info!("Starting LTO optimization for {} compilation units", compilation_units.len());

        // Check cache for incremental builds
        if self.config.enable_incremental {
            if let Some(cached_result) = self.check_incremental_cache(compilation_units)? {
                info!("Using cached LTO results");
                return Ok(cached_result);
            }
        }

        // Create LTO optimizer
        let mut optimizer = LtoOptimizer::new(self.config.lto_config.clone())?;

        // Convert build artifacts to LTO compilation units
        let lto_units = self.convert_to_lto_units(compilation_units)?;

        // Add units to optimizer
        for unit in lto_units {
            optimizer.add_compilation_unit(unit);
        }

        // Run LTO optimization with timeout
        let lto_result = self.run_lto_with_timeout(&mut optimizer)?;

        // Generate output files
        let output_files = self.generate_lto_outputs(&lto_result)?;

        // Update cache
        if self.config.enable_incremental {
            self.update_incremental_cache(compilation_units, &lto_result)?;
        }

        // Update statistics
        let optimization_time = start_time.elapsed();
        let mut stats = self.statistics.lock().unwrap();
        stats.total_lto_time += optimization_time;
        stats.lto_runs += 1;
        stats.compilation_units_processed += compilation_units.len();

        info!("LTO optimization completed in {:?}", optimization_time);

        Ok(LtoOptimizationResult {
            lto_statistics: lto_result.statistics,
            output_files,
            optimization_time,
            cache_hit: false,
            size_reduction: lto_result.statistics.code_size_reduction_percent(),
        })
    }

    /// Check if LTO should run for this build
    fn should_run_lto(&self, compilation_units: &[CompilationArtifact]) -> Result<bool> {
        // Skip if no compilation units
        if compilation_units.is_empty() {
            return Ok(false);
        }

        // Skip if LTO is disabled
        if self.config.lto_config.level == LtoLevel::None {
            return Ok(false);
        }

        // Check if release-only is enabled
        if self.config.release_only {
            // In a real implementation, this would check the build profile
            // For now, we'll assume it's always enabled
        }

        // Check if we have valid bitcode files
        let has_bitcode = compilation_units.iter()
            .any(|unit| unit.bitcode_path.is_some());

        if !has_bitcode {
            warn!("No bitcode files found for LTO optimization");
            return Ok(false);
        }

        Ok(true)
    }

    /// Check incremental cache for existing LTO results
    fn check_incremental_cache(&self, compilation_units: &[CompilationArtifact]) -> Result<Option<LtoOptimizationResult>> {
        let cache = self.build_cache.lock().unwrap();
        
        // Generate cache key from compilation units
        let cache_key = self.generate_cache_key(compilation_units)?;
        
        if let Some(cached_entry) = cache.get(&cache_key) {
            // Check if cached entry is still valid
            if self.is_cache_entry_valid(cached_entry, compilation_units)? {
                info!("Found valid cached LTO result");
                return Ok(Some(cached_entry.result.clone()));
            }
        }

        Ok(None)
    }

    /// Generate cache key for compilation units
    fn generate_cache_key(&self, compilation_units: &[CompilationArtifact]) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        
        // Hash LTO configuration
        format!("{:?}", self.config.lto_config).hash(&mut hasher);
        
        // Hash compilation unit information
        for unit in compilation_units {
            unit.object_path.hash(&mut hasher);
            if let Some(ref bitcode_path) = unit.bitcode_path {
                bitcode_path.hash(&mut hasher);
            }
            unit.source_hash.hash(&mut hasher);
        }

        Ok(format!("lto_cache_{:016x}", hasher.finish()))
    }

    /// Check if cache entry is still valid
    fn is_cache_entry_valid(&self, entry: &LtoCacheEntry, compilation_units: &[CompilationArtifact]) -> Result<bool> {
        // Check if any source files have been modified
        for unit in compilation_units {
            if let Some(ref bitcode_path) = unit.bitcode_path {
                let metadata = std::fs::metadata(bitcode_path)
                    .map_err(|e| Error::Other(format!("Failed to get file metadata: {}", e)))?;
                
                if metadata.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH) > entry.timestamp {
                    return Ok(false);
                }
            }
        }

        // Check if output files still exist
        for output_file in &entry.result.output_files {
            if !output_file.exists() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Convert build artifacts to LTO compilation units
    fn convert_to_lto_units(&self, artifacts: &[CompilationArtifact]) -> Result<Vec<LtoCompilationUnit>> {
        let mut units = Vec::new();

        for artifact in artifacts {
            if let Some(ref bitcode_path) = artifact.bitcode_path {
                let mut unit = LtoCompilationUnit::new(
                    artifact.unit_name.clone(),
                    bitcode_path.clone()
                );

                // Add source files
                unit.source_files = artifact.source_files.clone();

                // Add exported symbols (mock data for now)
                for i in 0..3 {
                    unit.exported_functions.insert(format!("{}_function_{}", artifact.unit_name, i));
                }
                unit.exported_globals.insert(format!("{}_globals", artifact.unit_name));

                // Estimate size based on file size
                if let Ok(metadata) = std::fs::metadata(bitcode_path) {
                    unit.size_estimate = metadata.len() as usize;
                }

                units.push(unit);
            }
        }

        Ok(units)
    }

    /// Run LTO with timeout protection
    fn run_lto_with_timeout(&self, optimizer: &mut LtoOptimizer) -> Result<crate::optimization::lto::LtoResult> {
        use std::sync::mpsc;
        use std::thread;

        let (sender, receiver) = mpsc::channel();
        let timeout = self.config.max_lto_time;

        // In a real implementation, we'd need to handle the optimizer move properly
        // For now, we'll run it directly and simulate timeout checking
        let start_time = Instant::now();
        
        let result = optimizer.optimize()?;
        
        let elapsed = start_time.elapsed();
        if elapsed > timeout {
            warn!("LTO optimization took {:?}, which exceeds timeout of {:?}", elapsed, timeout);
            // In a real implementation, we might want to terminate early or adjust parameters
        }

        Ok(result)
    }

    /// Generate LTO output files
    fn generate_lto_outputs(&self, lto_result: &crate::optimization::lto::LtoResult) -> Result<Vec<PathBuf>> {
        let mut output_files = Vec::new();

        match self.config.lto_config.level {
            LtoLevel::None => {
                // No LTO, return empty
            }
            LtoLevel::Thin => {
                // Generate object files for each optimization partition
                for i in 0..lto_result.optimization_results.inlining_results.functions_inlined.len().max(1) {
                    let output_path = self.config.output_directory.join(format!("lto_thin_{}.o", i));
                    
                    // In a real implementation, this would write actual object code
                    std::fs::write(&output_path, b"mock object code")
                        .map_err(|e| Error::Other(format!("Failed to write LTO output: {}", e)))?;
                    
                    output_files.push(output_path);
                }
            }
            LtoLevel::Full => {
                // Generate single merged object file
                let output_path = self.config.output_directory.join("lto_full.o");
                
                // In a real implementation, this would write actual merged object code
                std::fs::write(&output_path, b"mock merged object code")
                    .map_err(|e| Error::Other(format!("Failed to write LTO output: {}", e)))?;
                
                output_files.push(output_path);
            }
        }

        // Generate LTO report
        let report_path = self.config.output_directory.join("lto_report.md");
        let report_content = format!("# LTO Optimization Report\n\n{:?}", lto_result.statistics);
        std::fs::write(&report_path, report_content)
            .map_err(|e| Error::Other(format!("Failed to write LTO report: {}", e)))?;

        info!("Generated {} LTO output files", output_files.len());
        Ok(output_files)
    }

    /// Update incremental cache with new results
    fn update_incremental_cache(&self, compilation_units: &[CompilationArtifact], lto_result: &crate::optimization::lto::LtoResult) -> Result<()> {
        let mut cache = self.build_cache.lock().unwrap();
        
        let cache_key = self.generate_cache_key(compilation_units)?;
        
        let cache_entry = LtoCacheEntry {
            timestamp: std::time::SystemTime::now(),
            result: LtoOptimizationResult {
                lto_statistics: lto_result.statistics.clone(),
                output_files: Vec::new(), // Would be populated with actual output files
                optimization_time: Duration::from_secs(0), // Would use actual time
                cache_hit: false,
                size_reduction: lto_result.statistics.code_size_reduction_percent(),
            },
        };

        cache.put(cache_key, cache_entry);
        
        // Clean up old cache entries if needed
        cache.cleanup_old_entries(self.config.cache_size_limit)?;

        Ok(())
    }

    /// Get LTO build statistics
    pub fn get_statistics(&self) -> LtoBuildStatistics {
        self.statistics.lock().unwrap().clone()
    }

    /// Generate comprehensive LTO build report
    pub fn generate_build_report(&self) -> Result<String> {
        let stats = self.get_statistics();
        let mut report = String::new();

        report.push_str("# CURSED LTO Build Integration Report\n\n");
        report.push_str(&format!("**LTO Level**: {}\n", self.config.lto_config.level.as_str()));
        report.push_str(&format!("**Total LTO Runs**: {}\n", stats.lto_runs));
        report.push_str(&format!("**Total LTO Time**: {:?}\n", stats.total_lto_time));
        report.push_str(&format!("**Compilation Units Processed**: {}\n", stats.compilation_units_processed));

        if stats.lto_runs > 0 {
            let avg_time = stats.total_lto_time / stats.lto_runs as u32;
            report.push_str(&format!("**Average LTO Time**: {:?}\n", avg_time));
        }

        report.push_str(&format!("**Cache Hits**: {}\n", stats.cache_hits));
        report.push_str(&format!("**Cache Misses**: {}\n", stats.cache_misses));

        if stats.cache_hits + stats.cache_misses > 0 {
            let hit_rate = stats.cache_hits as f64 / (stats.cache_hits + stats.cache_misses) as f64 * 100.0;
            report.push_str(&format!("**Cache Hit Rate**: {:.1}%\n", hit_rate));
        }

        report.push_str(&format!("**Total Size Reduction**: {:.1}%\n", stats.total_size_reduction));

        Ok(report)
    }

    /// Clean up LTO build artifacts
    pub fn cleanup(&self) -> Result<()> {
        info!("Cleaning up LTO build artifacts");

        // Clean up output directory
        if self.config.output_directory.exists() {
            std::fs::remove_dir_all(&self.config.output_directory)
                .map_err(|e| Error::Other(format!("Failed to clean LTO output directory: {}", e)))?;
        }

        // Clean up cache
        let mut cache = self.build_cache.lock().unwrap();
        cache.clear();

        Ok(())
    }
}

/// Compilation artifact for LTO processing
#[derive(Debug, Clone)]
pub struct CompilationArtifact {
    /// Unit name
    pub unit_name: String,
    /// Source files for this unit
    pub source_files: Vec<PathBuf>,
    /// Object file path
    pub object_path: PathBuf,
    /// Bitcode file path (for LTO)
    pub bitcode_path: Option<PathBuf>,
    /// Source hash for change detection
    pub source_hash: String,
    /// Dependencies
    pub dependencies: Vec<String>,
}

/// LTO optimization result
#[derive(Debug, Clone)]
pub struct LtoOptimizationResult {
    /// LTO statistics
    pub lto_statistics: LtoStatistics,
    /// Generated output files
    pub output_files: Vec<PathBuf>,
    /// Time spent on optimization
    pub optimization_time: Duration,
    /// Whether this was a cache hit
    pub cache_hit: bool,
    /// Code size reduction percentage
    pub size_reduction: f64,
}

impl LtoOptimizationResult {
    /// Create a skipped LTO result
    pub fn skipped() -> Self {
        Self {
            lto_statistics: LtoStatistics::default(),
            output_files: Vec::new(),
            optimization_time: Duration::from_secs(0),
            cache_hit: false,
            size_reduction: 0.0,
        }
    }
}

/// LTO build statistics
#[derive(Debug, Clone, Default)]
pub struct LtoBuildStatistics {
    /// Number of LTO optimization runs
    pub lto_runs: usize,
    /// Total time spent on LTO
    pub total_lto_time: Duration,
    /// Number of compilation units processed
    pub compilation_units_processed: usize,
    /// Cache hits
    pub cache_hits: usize,
    /// Cache misses
    pub cache_misses: usize,
    /// Total size reduction achieved
    pub total_size_reduction: f64,
}

/// LTO build cache
struct LtoBuildCache {
    entries: HashMap<String, LtoCacheEntry>,
    max_size: usize,
}

impl LtoBuildCache {
    fn new(config: &LtoBuildConfig) -> Result<Self> {
        Ok(Self {
            entries: HashMap::new(),
            max_size: config.cache_size_limit,
        })
    }

    fn get(&self, key: &str) -> Option<&LtoCacheEntry> {
        self.entries.get(key)
    }

    fn put(&mut self, key: String, entry: LtoCacheEntry) {
        self.entries.insert(key, entry);
    }

    fn clear(&mut self) {
        self.entries.clear();
    }

    fn cleanup_old_entries(&mut self, _size_limit: usize) -> Result<()> {
        // In a real implementation, this would remove old entries based on size/age
        Ok(())
    }
}

/// LTO cache entry
#[derive(Debug, Clone)]
struct LtoCacheEntry {
    timestamp: std::time::SystemTime,
    result: LtoOptimizationResult,
}

/// LTO build configuration factory
pub struct LtoBuildConfigFactory;

impl LtoBuildConfigFactory {
    /// Create configuration for development builds
    pub fn development() -> LtoBuildConfig {
        LtoBuildConfig {
            lto_config: LtoConfig {
                level: LtoLevel::None,
                ..Default::default()
            },
            release_only: false,
            enable_incremental: true,
            enable_profiling: false,
            max_lto_time: Duration::from_secs(60), // 1 minute for dev
            ..Default::default()
        }
    }

    /// Create configuration for release builds
    pub fn release() -> LtoBuildConfig {
        LtoBuildConfig {
            lto_config: LtoConfig {
                level: LtoLevel::Full,
                enable_cross_module_inlining: true,
                enable_whole_program_dce: true,
                enable_global_variable_optimization: true,
                enable_cross_module_constant_propagation: true,
                enable_devirtualization: true,
                ..Default::default()
            },
            release_only: true,
            enable_incremental: true,
            enable_profiling: true,
            max_lto_time: Duration::from_secs(600), // 10 minutes for release
            ..Default::default()
        }
    }

    /// Create configuration for size-optimized builds
    pub fn size_optimized() -> LtoBuildConfig {
        LtoBuildConfig {
            lto_config: LtoConfig {
                level: LtoLevel::Full,
                enable_cross_module_inlining: true,
                enable_whole_program_dce: true,
                enable_global_variable_optimization: true,
                enable_cross_module_constant_propagation: true,
                enable_devirtualization: true,
                ..Default::default()
            },
            release_only: true,
            enable_incremental: true,
            enable_profiling: false,
            max_lto_time: Duration::from_secs(900), // 15 minutes for max size optimization
            ..Default::default()
        }
    }

    /// Create configuration for fast builds (Thin LTO)
    pub fn fast_build() -> LtoBuildConfig {
        LtoBuildConfig {
            lto_config: LtoConfig {
                level: LtoLevel::Thin,
                max_worker_threads: num_cpus::get(),
                thin_lto_partition_threshold: 500,
                enable_cross_module_inlining: true,
                enable_whole_program_dce: true,
                ..Default::default()
            },
            release_only: false,
            enable_incremental: true,
            enable_profiling: false,
            enable_parallel: true,
            max_lto_time: Duration::from_secs(120), // 2 minutes for fast builds
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lto_build_config_creation() {
        let config = LtoBuildConfig::default();
        assert_eq!(config.lto_config.level, LtoLevel::None);
        assert!(config.release_only);
        assert!(config.enable_incremental);
    }

    #[test]
    fn test_lto_build_integration_creation() {
        let config = LtoBuildConfig::default();
        let integration = LtoBuildIntegration::new(config);
        assert!(integration.is_ok());
    }

    #[test]
    fn test_compilation_artifact_creation() {
        let artifact = CompilationArtifact {
            unit_name: "test_unit".to_string(),
            source_files: vec![PathBuf::from("test.csd")],
            object_path: PathBuf::from("test.o"),
            bitcode_path: Some(PathBuf::from("test.bc")),
            source_hash: "abc123".to_string(),
            dependencies: vec!["stdlib".to_string()],
        };

        assert_eq!(artifact.unit_name, "test_unit");
        assert_eq!(artifact.source_files.len(), 1);
        assert!(artifact.bitcode_path.is_some());
    }

    #[test]
    fn test_lto_optimization_result() {
        let result = LtoOptimizationResult::skipped();
        assert_eq!(result.lto_statistics.modules_processed, 0);
        assert_eq!(result.optimization_time, Duration::from_secs(0));
        assert!(!result.cache_hit);
    }

    #[test]
    fn test_config_factory() {
        let dev_config = LtoBuildConfigFactory::development();
        let release_config = LtoBuildConfigFactory::release();
        let size_config = LtoBuildConfigFactory::size_optimized();
        let fast_config = LtoBuildConfigFactory::fast_build();

        assert_eq!(dev_config.lto_config.level, LtoLevel::None);
        assert_eq!(release_config.lto_config.level, LtoLevel::Full);
        assert_eq!(size_config.lto_config.level, LtoLevel::Full);
        assert_eq!(fast_config.lto_config.level, LtoLevel::Thin);

        assert!(!dev_config.release_only);
        assert!(release_config.release_only);
        assert!(fast_config.enable_parallel);
    }

    #[test]
    fn test_cache_key_generation() {
        let config = LtoBuildConfig::default();
        let integration = LtoBuildIntegration::new(config).unwrap();

        let artifacts = vec![
            CompilationArtifact {
                unit_name: "unit1".to_string(),
                source_files: vec![PathBuf::from("unit1.csd")],
                object_path: PathBuf::from("unit1.o"),
                bitcode_path: Some(PathBuf::from("unit1.bc")),
                source_hash: "hash1".to_string(),
                dependencies: Vec::new(),
            },
            CompilationArtifact {
                unit_name: "unit2".to_string(),
                source_files: vec![PathBuf::from("unit2.csd")],
                object_path: PathBuf::from("unit2.o"),
                bitcode_path: Some(PathBuf::from("unit2.bc")),
                source_hash: "hash2".to_string(),
                dependencies: Vec::new(),
            },
        ];

        let key1 = integration.generate_cache_key(&artifacts);
        let key2 = integration.generate_cache_key(&artifacts);

        assert!(key1.is_ok());
        assert!(key2.is_ok());
        assert_eq!(key1.unwrap(), key2.unwrap()); // Same inputs should generate same key
    }
}
