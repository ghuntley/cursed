/// Compilation Pipeline Performance Optimization
/// 
/// Optimizes the compilation pipeline itself with parallel compilation support,
/// incremental compilation cache improvements, and compilation progress reporting.

use crate::error::{CursedError, Result};
// use crate::profiling::performance::{PerformanceMonitor, CompilationPhase, ReportConfig, ReportFormat};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
// use tokio::sync::mpsc; // Disabled - tokio causes E0753 errors
use std::sync::mpsc;
use rayon::prelude::*;

/// Parallel compilation configuration
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    /// Number of worker threads (0 = auto-detect)
    pub num_threads: usize,
    /// Maximum files per worker
    pub max_files_per_worker: usize,
    /// Enable work stealing between workers
    pub enable_work_stealing: bool,
    /// Enable dependency-aware scheduling
    pub dependency_aware: bool,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            num_threads: 0, // Auto-detect
            max_files_per_worker: 50,
            enable_work_stealing: true,
            dependency_aware: true,
        }
    }
}

/// Incremental compilation configuration
#[derive(Debug, Clone)]
pub struct IncrementalConfig {
    /// Enable incremental compilation
    pub enabled: bool,
    /// Cache directory path
    pub cache_dir: PathBuf,
    /// Maximum cache size in MB
    pub max_cache_size_mb: usize,
    /// Cache expiration time in hours
    pub cache_expiration_hours: u64,
    /// Enable dependency tracking
    pub track_dependencies: bool,
}

impl Default for IncrementalConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_dir: PathBuf::from(".cursed_cache"),
            max_cache_size_mb: 1024, // 1GB
            cache_expiration_hours: 24,
            track_dependencies: true,
        }
    }
}

/// Progress reporting configuration
#[derive(Debug, Clone)]
pub struct ProgressConfig {
    /// Enable progress reporting
    pub enabled: bool,
    /// Update interval in milliseconds
    pub update_interval_ms: u64,
    /// Show file names being processed
    pub show_file_names: bool,
    /// Show phase transitions
    pub show_phases: bool,
    /// Show estimated time remaining
    pub show_eta: bool,
}

impl Default for ProgressConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            update_interval_ms: 100,
            show_file_names: false,
            show_phases: true,
            show_eta: true,
        }
    }
}

/// Compilation job for parallel processing
#[derive(Debug, Clone)]
pub struct CompilationJob {
    pub id: usize,
    pub file_path: PathBuf,
    pub source_code: String,
    pub dependencies: Vec<PathBuf>,
    pub priority: u8, // 0 = highest, 255 = lowest
}

/// Compilation result
#[derive(Debug, Clone)]
pub struct CompilationResult {
    pub job_id: usize,
    pub file_path: PathBuf,
    pub success: bool,
    pub duration: Duration,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub output_size: usize,
}

/// Cache entry for incremental compilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub file_path: PathBuf,
    pub source_hash: u64,
    pub dependency_hashes: HashMap<PathBuf, u64>,
    pub compiled_output: Vec<u8>,
    pub compile_time: SystemTime,
    pub metadata: HashMap<String, String>,
}

/// Progress reporter
pub struct ProgressReporter {
    config: ProgressConfig,
    total_files: usize,
    completed_files: Arc<Mutex<usize>>,
    current_phase: Arc<Mutex<CompilationPhase>>,
    start_time: Instant,
}

impl ProgressReporter {
    pub fn new(config: ProgressConfig, total_files: usize) -> Self {
        Self {
            config,
            total_files,
            completed_files: Arc::new(Mutex::new(0)),
            current_phase: Arc::new(Mutex::new(CompilationPhase::Total)),
            start_time: Instant::now(),
        }
    }
    
    pub fn update_progress(&self, completed: usize) {
        if !self.config.enabled {
            return;
        }
        
        *self.completed_files.lock().unwrap() = completed;
        self.print_progress();
    }
    
    pub fn set_phase(&self, phase: CompilationPhase) {
        if !self.config.enabled {
            return;
        }
        
        *self.current_phase.lock().unwrap() = phase;
        if self.config.show_phases {
            println!("📝 Phase: {}", phase);
        }
    }
    
    pub fn report_file(&self, file_path: &Path) {
        if !self.config.enabled || !self.config.show_file_names {
            return;
        }
        
        println!("   📄 {}", file_path.display());
    }
    
    fn print_progress(&self) {
        let completed = *self.completed_files.lock().unwrap();
        let phase = self.current_phase.lock().unwrap().clone();
        
        let percentage = if self.total_files > 0 {
            (completed as f64 / self.total_files as f64 * 100.0) as u8
        } else {
            0
        };
        
        let elapsed = self.start_time.elapsed();
        let eta = if completed > 0 && completed < self.total_files {
            let avg_time_per_file = elapsed.as_secs_f64() / completed as f64;
            let remaining_files = self.total_files - completed;
            Duration::from_secs_f64(avg_time_per_file * remaining_files as f64)
        } else {
            Duration::from_secs(0)
        };
        
        let progress_bar = self.create_progress_bar(percentage);
        
        if self.config.show_eta && eta.as_secs() > 0 {
            println!("\r🚀 {} [{}] {}% ({}/{}) ETA: {:.1}s", 
                     phase, progress_bar, percentage, completed, self.total_files, eta.as_secs_f64());
        } else {
            println!("\r🚀 {} [{}] {}% ({}/{})", 
                     phase, progress_bar, percentage, completed, self.total_files);
        }
    }
    
    fn create_progress_bar(&self, percentage: u8) -> String {
        let width = 20;
        let filled = (percentage as usize * width / 100).min(width);
        let empty = width - filled;
        
        format!("{}{}", "█".repeat(filled), "░".repeat(empty))
    }
    
    pub fn finish(&self) {
        if !self.config.enabled {
            return;
        }
        
        let total_time = self.start_time.elapsed();
        println!("\n✅ Compilation completed in {:.2}s", total_time.as_secs_f64());
    }
}

/// Performance-optimized compilation pipeline
pub struct PerformancePipeline {
    parallel_config: ParallelConfig,
    incremental_config: IncrementalConfig,
    progress_config: ProgressConfig,
    performance_monitor: PerformanceMonitor,
    cache: Arc<Mutex<HashMap<PathBuf, CacheEntry>>>,
}

impl PerformancePipeline {
    /// Create a new performance pipeline
    pub fn new(
        parallel_config: ParallelConfig,
        incremental_config: IncrementalConfig,
        progress_config: ProgressConfig,
    ) -> Self {
        Self {
            parallel_config,
            incremental_config,
            progress_config,
            performance_monitor: PerformanceMonitor::new(),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Initialize the pipeline
    pub fn initialize(&mut self) -> Result<()> {
        // Setup thread pool
        let num_threads = if self.parallel_config.num_threads == 0 {
            num_cpus::get()
        } else {
            self.parallel_config.num_threads
        };
        
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .map_err(|e| CursedError::General(format!("Failed to initialize thread pool: {}", e)))?;
        
        // Initialize incremental compilation cache
        if self.incremental_config.enabled {
            self.load_cache()?;
        }
        
        println!("🔧 Performance pipeline initialized with {} threads", num_threads);
        
        Ok(())
    }
    
    /// Compile multiple files in parallel
    pub async fn compile_files(&mut self, jobs: Vec<CompilationJob>) -> Result<Vec<CompilationResult>> {
        let total_files = jobs.len();
        let progress_reporter = Arc::new(ProgressReporter::new(
            self.progress_config.clone(),
            total_files,
        ));
        
        self.performance_monitor.start_phase(CompilationPhase::Total)?;
        progress_reporter.set_phase(CompilationPhase::Total);
        
        // Check cache for incremental compilation
        let (cached_jobs, compile_jobs) = if self.incremental_config.enabled {
            self.filter_cached_jobs(jobs)?
        } else {
            (Vec::new(), jobs)
        };
        
        let mut results = Vec::new();
        
        // Add cached results
        for job in cached_jobs {
            results.push(CompilationResult {
                job_id: job.id,
                file_path: job.file_path,
                success: true,
                duration: Duration::from_millis(1), // Cache hit is nearly instant
                errors: Vec::new(),
                warnings: Vec::new(),
                output_size: 0, // Would be from cache
            });
        }
        
        if !compile_jobs.is_empty() {
            // Sort jobs by priority and dependencies
            let sorted_jobs = self.sort_jobs_by_priority(compile_jobs);
            
            // Compile in parallel
            let parallel_results = if self.parallel_config.dependency_aware {
                self.compile_with_dependencies(sorted_jobs, progress_reporter.clone()).await?
            } else {
                self.compile_parallel(sorted_jobs, progress_reporter.clone()).await?
            };
            
            results.extend(parallel_results);
        }
        
        progress_reporter.finish();
        self.performance_monitor.end_phase(CompilationPhase::Total)?;
        
        // Update cache with new results
        if self.incremental_config.enabled {
            self.update_cache(&results)?;
        }
        
        Ok(results)
    }
    
    /// Filter jobs that can be served from cache
    fn filter_cached_jobs(&self, jobs: Vec<CompilationJob>) -> Result<(Vec<CompilationJob>, Vec<CompilationJob>)> {
        let cache = self.cache.lock().unwrap();
        let mut cached = Vec::new();
        let mut to_compile = Vec::new();
        
        for job in jobs {
            if let Some(cache_entry) = cache.get(&job.file_path) {
                let source_hash = self.compute_file_hash(&job.source_code);
                
                if cache_entry.source_hash == source_hash && self.check_dependencies_unchanged(&job, cache_entry) {
                    cached.push(job);
                    continue;
                }
            }
            
            to_compile.push(job);
        }
        
        println!("📦 Cache: {} hits, {} misses", cached.len(), to_compile.len());
        
        Ok((cached, to_compile))
    }
    
    /// Sort jobs by priority and dependency order
    fn sort_jobs_by_priority(&self, mut jobs: Vec<CompilationJob>) -> Vec<CompilationJob> {
        // Sort by priority first (lower number = higher priority)
        jobs.sort_by_key(|job| job.priority);
        
        // If dependency-aware scheduling is enabled, topologically sort
        if self.parallel_config.dependency_aware {
            // This is a simplified implementation - a real system would need
            // proper dependency graph analysis
            jobs
        } else {
            jobs
        }
    }
    
    /// Compile jobs in parallel without dependency awareness
    async fn compile_parallel(
        &self,
        jobs: Vec<CompilationJob>,
        progress_reporter: Arc<ProgressReporter>,
    ) -> Result<Vec<CompilationResult>> {
        let (tx, mut rx) = mpsc::channel(1000);
        let completed_count = Arc::new(Mutex::new(0));
        
        // Process jobs in parallel using rayon
        let results = jobs.into_par_iter().map(|job| {
            let start_time = Instant::now();
            progress_reporter.report_file(&job.file_path);
            
            // Simulate compilation (replace with actual compilation logic)
            let result = self.simulate_compilation(&job);
            
            let duration = start_time.elapsed();
            
            // Update progress
            {
                let mut count = completed_count.lock().unwrap();
                *count += 1;
                progress_reporter.update_progress(*count);
            }
            
            CompilationResult {
                job_id: job.id,
                file_path: job.file_path,
                success: result.is_ok(),
                duration,
                errors: if result.is_err() { vec![result.unwrap_err().to_string()] } else { Vec::new() },
                warnings: Vec::new(),
                output_size: 1024, // Placeholder
            }
        }).collect();
        
        Ok(results)
    }
    
    /// Compile jobs with dependency awareness
    async fn compile_with_dependencies(
        &self,
        jobs: Vec<CompilationJob>,
        progress_reporter: Arc<ProgressReporter>,
    ) -> Result<Vec<CompilationResult>> {
        // For now, fall back to parallel compilation
        // A real implementation would build a dependency graph and schedule accordingly
        self.compile_parallel(jobs, progress_reporter).await
    }
    
    /// Simulate compilation (replace with actual compilation logic)
    fn simulate_compilation(&self, job: &CompilationJob) -> Result<()> {
        // Simulate compilation time based on file size
        let compile_time_ms = (job.source_code.len() / 100).max(10).min(1000);
        thread::sleep(Duration::from_millis(compile_time_ms as u64));
        
        // Simulate random failures for testing
        if job.file_path.to_string_lossy().contains("error") {
            return Err(CursedError::General("Simulated compilation error".to_string()));
        }
        
        Ok(())
    }
    
    /// Load cache from disk
    fn load_cache(&self) -> Result<()> {
        let cache_file = self.incremental_config.cache_dir.join("compilation_cache.json");
        
        if cache_file.exists() {
            let cache_data = std::fs::read_to_string(&cache_file)
                .map_err(|e| CursedError::General(format!("Failed to read cache file: {}", e)))?;
            
            let cache_entries: HashMap<PathBuf, CacheEntry> = serde_json::from_str(&cache_data)
                .map_err(|e| CursedError::General(format!("Failed to parse cache file: {}", e)))?;
            
            *self.cache.lock().unwrap() = cache_entries;
            println!("📦 Loaded compilation cache with {} entries", self.cache.lock().unwrap().len());
        }
        
        Ok(())
    }
    
    /// Save cache to disk
    fn save_cache(&self) -> Result<()> {
        if !self.incremental_config.cache_dir.exists() {
            std::fs::create_dir_all(&self.incremental_config.cache_dir)
                .map_err(|e| CursedError::General(format!("Failed to create cache directory: {}", e)))?;
        }
        
        let cache_file = self.incremental_config.cache_dir.join("compilation_cache.json");
        let cache = self.cache.lock().unwrap();
        let cache_data = serde_json::to_string_pretty(&*cache)
            .map_err(|e| CursedError::General(format!("Failed to serialize cache: {}", e)))?;
        
        std::fs::write(&cache_file, cache_data)
            .map_err(|e| CursedError::General(format!("Failed to write cache file: {}", e)))?;
        
        Ok(())
    }
    
    /// Update cache with new compilation results
    fn update_cache(&self, results: &[CompilationResult]) -> Result<()> {
        let mut cache = self.cache.lock().unwrap();
        
        for result in results {
            if result.success {
                let source_hash = 12345; // Placeholder - would compute actual hash
                
                cache.insert(result.file_path.clone(), CacheEntry {
                    file_path: result.file_path.clone(),
                    source_hash,
                    dependency_hashes: HashMap::new(),
                    compiled_output: vec![0; result.output_size], // Placeholder
                    compile_time: SystemTime::now(),
                    metadata: HashMap::new(),
                });
            }
        }
        
        self.save_cache()?;
        
        Ok(())
    }
    
    /// Compute hash of source code
    fn compute_file_hash(&self, source: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        hasher.finish()
    }
    
    /// Check if dependencies haven't changed
    fn check_dependencies_unchanged(&self, job: &CompilationJob, cache_entry: &CacheEntry) -> bool {
        // Simplified implementation - would check actual dependency timestamps/hashes
        job.dependencies.len() == cache_entry.dependency_hashes.len()
    }
    
    /// Get performance report
    pub fn get_performance_report(&self, format: ReportFormat) -> Result<String> {
        let mut config = ReportConfig::default();
        config.format = format;
        
        let mut monitor = self.performance_monitor.clone();
        monitor.update_config(config);
        monitor.generate_report()
    }
    
    /// Clean old cache entries
    pub fn clean_cache(&self) -> Result<()> {
        let mut cache = self.cache.lock().unwrap();
        let expiration_time = SystemTime::now() - Duration::from_secs(
            self.incremental_config.cache_expiration_hours * 3600
        );
        
        let initial_count = cache.len();
        cache.retain(|_, entry| entry.compile_time >= expiration_time);
        let cleaned_count = initial_count - cache.len();
        
        if cleaned_count > 0 {
            println!("🧹 Cleaned {} expired cache entries", cleaned_count);
            self.save_cache()?;
        }
        
        Ok(())
    }
    
    /// Get cache statistics
    pub fn get_cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.lock().unwrap();
        let entry_count = cache.len();
        let total_size = cache.values()
            .map(|entry| entry.compiled_output.len())
            .sum::<usize>();
        
        (entry_count, total_size)
    }
}

impl Drop for PerformancePipeline {
    fn drop(&mut self) {
        if self.incremental_config.enabled {
            let _ = self.save_cache();
        }
    }
}

/// Utilities for performance pipeline
pub mod utils {
    use super::*;
    
    /// Create default parallel configuration based on system capabilities
    pub fn auto_parallel_config() -> ParallelConfig {
        let num_cpus = num_cpus::get();
        
        ParallelConfig {
            num_threads: num_cpus,
            max_files_per_worker: if num_cpus > 4 { 25 } else { 50 },
            enable_work_stealing: true,
            dependency_aware: num_cpus > 2,
        }
    }
    
    /// Create development-friendly configuration
    pub fn dev_config() -> (ParallelConfig, IncrementalConfig, ProgressConfig) {
        (
            ParallelConfig {
                num_threads: 2,
                ..Default::default()
            },
            IncrementalConfig {
                enabled: true,
                cache_dir: PathBuf::from(".cursed_dev_cache"),
                ..Default::default()
            },
            ProgressConfig {
                enabled: true,
                show_file_names: true,
                ..Default::default()
            },
        )
    }
    
    /// Create production-friendly configuration
    pub fn production_config() -> (ParallelConfig, IncrementalConfig, ProgressConfig) {
        (
            auto_parallel_config(),
            IncrementalConfig {
                enabled: true,
                cache_dir: PathBuf::from(".cursed_cache"),
                max_cache_size_mb: 2048, // 2GB
                ..Default::default()
            },
            ProgressConfig {
                enabled: false, // Less verbose in production
                ..Default::default()
            },
        )
    }
}

