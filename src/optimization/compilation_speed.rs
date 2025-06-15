/// Compilation Speed Optimization for CURSED Compiler
/// 
/// Provides parallel compilation of independent modules, incremental compilation 
/// with dependency tracking, and build cache optimization.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::fs;
use std::io::{self, Read, Write};
use tracing::{debug, info, instrument, warn, error};
use crossbeam_channel::{bounded, Receiver, Sender};
use rayon::prelude::*;
use parking_lot::{RwLock as ParkingLotRwLock, Mutex as ParkingLotMutex};
use sha2::{Sha256, Digest};
use bincode;
use serde::{Deserialize, Serialize};
use tempfile::TempDir;
use lru::LruCache;

use crate::ast::Program;
use crate::error::{Error, Result};
use crate::lexer::Lexer;
use crate::parser::Parser;

/// Compilation speed optimization configuration
#[derive(Debug, Clone)]
pub struct CompilationSpeedConfig {
    /// Enable parallel compilation
    pub enable_parallel_compilation: bool,
    /// Enable incremental compilation
    pub enable_incremental_compilation: bool,
    /// Enable build caching
    pub enable_build_caching: bool,
    /// Maximum number of parallel compilation threads
    pub max_parallel_threads: usize,
    /// Cache directory
    pub cache_directory: PathBuf,
    /// Dependency analysis depth
    pub dependency_analysis_depth: usize,
    /// Enable AST caching
    pub enable_ast_caching: bool,
    /// Enable type checking parallelization
    pub enable_parallel_type_checking: bool,
}

impl Default for CompilationSpeedConfig {
    fn default() -> Self {
        Self {
            enable_parallel_compilation: true,
            enable_incremental_compilation: true,
            enable_build_caching: true,
            max_parallel_threads: num_cpus::get().max(1),
            cache_directory: PathBuf::from(".cursed_cache"),
            dependency_analysis_depth: 10,
            enable_ast_caching: true,
            enable_parallel_type_checking: true,
        }
    }
}

/// Compilation unit representing a single module or file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationUnit {
    /// Unique identifier for this unit
    pub id: String,
    /// Source file path
    pub source_path: PathBuf,
    /// Module name
    pub module_name: String,
    /// Source code
    pub source_code: String,
    /// Dependencies (other compilation units this depends on)
    pub dependencies: Vec<String>,
    /// Last modification time
    pub last_modified: SystemTime,
    /// Compilation status
    pub status: CompilationStatus,
    /// Compilation priority (higher = more urgent)
    pub priority: u32,
    /// Content hash for cache validation
    pub content_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompilationStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
    Cached,
}

/// Dependency graph for tracking module relationships
#[derive(Debug)]
pub struct DependencyGraph {
    /// Adjacency list representation
    graph: HashMap<String, Vec<String>>,
    /// Reverse dependencies (dependents)
    reverse_graph: HashMap<String, Vec<String>>,
    /// Topological order cache
    topological_order: Option<Vec<String>>,
}

impl DependencyGraph {
    /// Create a new dependency graph
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
            reverse_graph: HashMap::new(),
            topological_order: None,
        }
    }

    /// Add a dependency relationship
    pub fn add_dependency(&mut self, dependent: &str, dependency: &str) {
        self.graph
            .entry(dependent.to_string())
            .or_insert_with(Vec::new)
            .push(dependency.to_string());
        
        self.reverse_graph
            .entry(dependency.to_string())
            .or_insert_with(Vec::new)
            .push(dependent.to_string());
        
        // Invalidate topological order cache
        self.topological_order = None;
    }

    /// Get dependencies for a module
    pub fn get_dependencies(&self, module: &str) -> Vec<String> {
        self.graph.get(module).cloned().unwrap_or_default()
    }

    /// Get dependents for a module
    pub fn get_dependents(&self, module: &str) -> Vec<String> {
        self.reverse_graph.get(module).cloned().unwrap_or_default()
    }

    /// Compute topological order for compilation
    pub fn topological_sort(&mut self) -> Result<Vec<String>> {
        if let Some(ref order) = self.topological_order {
            return Ok(order.clone());
        }

        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Initialize in-degree count
        for (node, deps) in &self.graph {
            in_degree.entry(node.clone()).or_insert(0);
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }

        // Find nodes with no incoming edges
        for (node, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node.clone());
            }
        }

        // Process nodes
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());

            // Update in-degree for dependents
            if let Some(dependents) = self.reverse_graph.get(&node) {
                for dependent in dependents {
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dependent.clone());
                        }
                    }
                }
            }
        }

        // Check for cycles
        if result.len() != in_degree.len() {
            return Err(Error::Runtime("Circular dependency detected".to_string()));
        }

        self.topological_order = Some(result.clone());
        Ok(result)
    }

    /// Check if there are any cycles in the dependency graph
    pub fn has_cycles(&mut self) -> bool {
        self.topological_sort().is_err()
    }

    /// Get all modules that need to be recompiled when a module changes
    pub fn get_affected_modules(&self, changed_module: &str) -> HashSet<String> {
        let mut affected = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(changed_module.to_string());
        affected.insert(changed_module.to_string());

        while let Some(module) = queue.pop_front() {
            if let Some(dependents) = self.reverse_graph.get(&module) {
                for dependent in dependents {
                    if affected.insert(dependent.clone()) {
                        queue.push_back(dependent.clone());
                    }
                }
            }
        }

        affected
    }
}

/// Parallel AST processor for parsing multiple files concurrently
pub struct ParallelAstProcessor {
    /// Configuration
    config: CompilationSpeedConfig,
    /// Thread pool for parsing
    thread_pool: Vec<thread::JoinHandle<()>>,
    /// Work queue
    work_sender: Option<Sender<CompilationUnit>>,
    /// Result queue
    result_receiver: Option<Receiver<(String, Result<Program>)>>,
    /// Active workers
    active_workers: Arc<Mutex<usize>>,
}

impl ParallelAstProcessor {
    /// Create a new parallel AST processor
    pub fn new(config: CompilationSpeedConfig) -> Result<Self> {
        let (work_sender, work_receiver) = bounded(config.max_parallel_threads * 2);
        let (result_sender, result_receiver) = bounded(config.max_parallel_threads * 2);
        let active_workers = Arc::new(Mutex::new(0));

        let mut thread_pool = Vec::new();

        // Spawn worker threads
        for i in 0..config.max_parallel_threads {
            let work_receiver = work_receiver.clone();
            let result_sender = result_sender.clone();
            let active_workers = active_workers.clone();

            let handle = thread::Builder::new()
                .name(format!("ast-worker-{}", i))
                .spawn(move || {
                    Self::worker_thread(work_receiver, result_sender, active_workers);
                })
                .map_err(|e| Error::Runtime(format!("Failed to spawn worker thread: {}", e)))?;

            thread_pool.push(handle);
        }

        Ok(Self {
            config,
            thread_pool,
            work_sender: Some(work_sender),
            result_receiver: Some(result_receiver),
            active_workers,
        })
    }

    /// Worker thread function
    fn worker_thread(
        work_receiver: Receiver<CompilationUnit>,
        result_sender: Sender<(String, Result<Program>)>,
        active_workers: Arc<Mutex<usize>>,
    ) {
        while let Ok(unit) = work_receiver.recv() {
            {
                let mut workers = active_workers.lock().unwrap();
                *workers += 1;
            }

            debug!("Processing compilation unit: {}", unit.id);
            let start_time = Instant::now();

            // Parse the source code
            let result = Self::parse_unit(&unit);
            
            let elapsed = start_time.elapsed();
            match &result {
                Ok(_) => debug!("Successfully parsed {} in {}μs", unit.id, elapsed.as_micros()),
                Err(e) => warn!("Failed to parse {}: {} ({}μs)", unit.id, e, elapsed.as_micros()),
            }

            // Send result
            if result_sender.send((unit.id.clone(), result)).is_err() {
                break; // Channel closed
            }

            {
                let mut workers = active_workers.lock().unwrap();
                *workers -= 1;
            }
        }
    }

    /// Parse a single compilation unit
    fn parse_unit(unit: &CompilationUnit) -> Result<Program> {
        let lexer = Lexer::new(unit.source_code.clone());
        let mut parser = Parser::new(lexer)?;
        
        let program = parser.parse_program()?;
        
        // Check for parse errors
        let errors = parser.errors();
        if !errors.is_empty() {
            return Err(Error::Parse(format!("Parse errors in {}: {}", 
                                           unit.module_name, errors.join(", "))));
        }

        Ok(program)
    }

    /// Submit compilation unit for processing
    pub fn submit_unit(&self, unit: CompilationUnit) -> Result<()> {
        if let Some(ref sender) = self.work_sender {
            sender.send(unit)
                .map_err(|_| Error::Runtime("Failed to submit compilation unit".to_string()))?;
        }
        Ok(())
    }

    /// Get completed results
    pub fn get_results(&self) -> Vec<(String, Result<Program>)> {
        let mut results = Vec::new();
        
        if let Some(ref receiver) = self.result_receiver {
            while let Ok(result) = receiver.try_recv() {
                results.push(result);
            }
        }
        
        results
    }

    /// Wait for all workers to complete
    pub fn wait_for_completion(&self) -> Result<()> {
        loop {
            let active = {
                let workers = self.active_workers.lock().unwrap();
                *workers
            };
            
            if active == 0 {
                break;
            }
            
            thread::sleep(Duration::from_millis(10));
        }
        
        Ok(())
    }

    /// Get processing statistics
    pub fn get_statistics(&self) -> ProcessingStatistics {
        let active_workers = {
            let workers = self.active_workers.lock().unwrap();
            *workers
        };

        ProcessingStatistics {
            total_workers: self.config.max_parallel_threads,
            active_workers,
            queue_length: if let Some(ref sender) = self.work_sender {
                sender.len()
            } else {
                0
            },
            completed_results: if let Some(ref receiver) = self.result_receiver {
                receiver.len()
            } else {
                0
            },
        }
    }
}

impl Drop for ParallelAstProcessor {
    fn drop(&mut self) {
        // Close channels
        self.work_sender.take();
        self.result_receiver.take();
        
        // Wait for threads to finish
        for handle in self.thread_pool.drain(..) {
            let _ = handle.join();
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessingStatistics {
    pub total_workers: usize,
    pub active_workers: usize,
    pub queue_length: usize,
    pub completed_results: usize,
}

/// Type checking optimizer for parallel type analysis
pub struct TypeCheckingOptimizer {
    /// Configuration
    config: CompilationSpeedConfig,
    /// Cached type information
    type_cache: Arc<RwLock<HashMap<String, TypeCheckResult>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeCheckResult {
    pub module_name: String,
    pub type_errors: Vec<String>,
    pub exported_types: HashMap<String, String>, // Type name -> Type signature
    pub imported_types: HashMap<String, String>,
    pub last_checked: SystemTime,
}

impl TypeCheckingOptimizer {
    /// Create a new type checking optimizer
    pub fn new(config: CompilationSpeedConfig) -> Self {
        Self {
            config,
            type_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Perform type checking for a module
    #[instrument(skip(self, program))]
    pub fn check_types(&self, module_name: &str, program: &Program) -> Result<TypeCheckResult> {
        // Check cache first
        if self.config.enable_build_caching {
            let cache = self.type_cache.read().unwrap();
            if let Some(cached_result) = cache.get(module_name) {
                debug!("Using cached type check result for {}", module_name);
                return Ok(cached_result.clone());
            }
        }

        debug!("Performing type checking for {}", module_name);
        let start_time = Instant::now();

        // Simplified type checking (would integrate with actual type checker)
        let mut type_errors = Vec::new();
        let mut exported_types = HashMap::new();
        let mut imported_types = HashMap::new();

        // Analyze program for type information
        for statement in &program.statements {
            // This would be much more complex in a real implementation
            // For now, just simulate some type checking work
            match statement.to_string().as_str() {
                s if s.contains("facts ") => {
                    // Variable declaration
                    exported_types.insert("variable".to_string(), "unknown".to_string());
                }
                s if s.contains("slay ") => {
                    // Function declaration
                    exported_types.insert("function".to_string(), "unknown".to_string());
                }
                _ => {}
            }
        }

        let result = TypeCheckResult {
            module_name: module_name.to_string(),
            type_errors,
            exported_types,
            imported_types,
            last_checked: SystemTime::now(),
        };

        // Cache the result
        if self.config.enable_build_caching {
            let mut cache = self.type_cache.write().unwrap();
            cache.insert(module_name.to_string(), result.clone());
        }

        let elapsed = start_time.elapsed();
        debug!("Type checking for {} completed in {}μs", module_name, elapsed.as_micros());

        Ok(result)
    }

    /// Get cached type information
    pub fn get_cached_types(&self, module_name: &str) -> Option<TypeCheckResult> {
        let cache = self.type_cache.read().unwrap();
        cache.get(module_name).cloned()
    }

    /// Invalidate cached type information
    pub fn invalidate_cache(&self, module_name: &str) {
        let mut cache = self.type_cache.write().unwrap();
        cache.remove(module_name);
        debug!("Invalidated type cache for {}", module_name);
    }

    /// Clear all cached type information
    pub fn clear_cache(&self) {
        let mut cache = self.type_cache.write().unwrap();
        cache.clear();
        info!("Cleared all type checking cache");
    }
}

/// File-based AST cache for incremental compilation
#[derive(Debug)]
pub struct AstCache {
    /// Cache directory
    cache_dir: PathBuf,
    /// In-memory LRU cache for recent ASTs
    memory_cache: Arc<ParkingLotMutex<LruCache<String, CachedAst>>>,
    /// Cache statistics
    stats: Arc<ParkingLotRwLock<CacheStatistics>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedAst {
    /// The actual AST
    pub program: Program,
    /// Content hash when cached
    pub content_hash: String,
    /// File modification time when cached
    pub cached_at: SystemTime,
    /// Compilation time in microseconds
    pub compilation_time_us: u64,
    /// Dependencies at cache time
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CacheStatistics {
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub cache_invalidations: u64,
    pub total_cache_size_bytes: u64,
    pub average_hit_time_us: u64,
    pub average_miss_time_us: u64,
}

impl AstCache {
    /// Create a new AST cache
    pub fn new(cache_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&cache_dir)
            .map_err(|e| Error::Runtime(format!("Failed to create cache directory: {}", e)))?;

        Ok(Self {
            cache_dir,
            memory_cache: Arc::new(ParkingLotMutex::new(LruCache::new(std::num::NonZeroUsize::new(100).unwrap()))),
            stats: Arc::new(ParkingLotRwLock::new(CacheStatistics::default())),
        })
    }

    /// Calculate content hash for a compilation unit
    pub fn calculate_content_hash(unit: &CompilationUnit) -> String {
        let mut hasher = Sha256::new();
        hasher.update(unit.source_code.as_bytes());
        hasher.update(unit.module_name.as_bytes());
        
        // Include dependency hashes to detect transitive changes
        let mut deps_sorted = unit.dependencies.clone();
        deps_sorted.sort();
        for dep in deps_sorted {
            hasher.update(dep.as_bytes());
        }
        
        // Include modification time
        if let Ok(duration) = unit.last_modified.duration_since(UNIX_EPOCH) {
            hasher.update(&duration.as_secs().to_le_bytes());
        }
        
        format!("{:x}", hasher.finalize())
    }

    /// Get cached AST if valid
    #[instrument(skip(self, unit))]
    pub fn get_cached_ast(&self, unit: &CompilationUnit) -> Option<CachedAst> {
        let start_time = Instant::now();
        let cache_key = format!("{}_{}", unit.module_name, unit.content_hash);

        // Check memory cache first
        {
            let mut memory_cache = self.memory_cache.lock();
            if let Some(cached) = memory_cache.get(&cache_key) {
                if cached.content_hash == unit.content_hash {
                    let mut stats = self.stats.write();
                    stats.cache_hits += 1;
                    stats.average_hit_time_us = 
                        (stats.average_hit_time_us + start_time.elapsed().as_micros() as u64) / 2;
                    
                    debug!("Cache hit for {} (memory)", unit.module_name);
                    return Some(cached.clone());
                }
            }
        }

        // Check disk cache
        let cache_file = self.cache_dir.join(format!("{}.cache", cache_key));
        if let Ok(data) = fs::read(&cache_file) {
            if let Ok(cached) = bincode::deserialize::<CachedAst>(&data) {
                if cached.content_hash == unit.content_hash {
                    // Add to memory cache
                    {
                        let mut memory_cache = self.memory_cache.lock();
                        memory_cache.put(cache_key, cached.clone());
                    }

                    let mut stats = self.stats.write();
                    stats.cache_hits += 1;
                    stats.average_hit_time_us = 
                        (stats.average_hit_time_us + start_time.elapsed().as_micros() as u64) / 2;
                    
                    debug!("Cache hit for {} (disk)", unit.module_name);
                    return Some(cached);
                }
            }
        }

        // Cache miss
        let mut stats = self.stats.write();
        stats.cache_misses += 1;
        stats.average_miss_time_us = 
            (stats.average_miss_time_us + start_time.elapsed().as_micros() as u64) / 2;
        
        debug!("Cache miss for {}", unit.module_name);
        None
    }

    /// Store AST in cache
    #[instrument(skip(self, unit, program))]
    pub fn store_ast(&self, unit: &CompilationUnit, program: Program, compilation_time_us: u64) -> Result<()> {
        let cache_key = format!("{}_{}", unit.module_name, unit.content_hash);
        
        let cached_ast = CachedAst {
            program: program.clone(),
            content_hash: unit.content_hash.clone(),
            cached_at: SystemTime::now(),
            compilation_time_us,
            dependencies: unit.dependencies.clone(),
        };

        // Store in memory cache
        {
            let mut memory_cache = self.memory_cache.lock();
            memory_cache.put(cache_key.clone(), cached_ast.clone());
        }

        // Store on disk
        let cache_file = self.cache_dir.join(format!("{}.cache", cache_key));
        let data = bincode::serialize(&cached_ast)
            .map_err(|e| Error::Runtime(format!("Failed to serialize AST: {}", e)))?;
        
        fs::write(&cache_file, &data)
            .map_err(|e| Error::Runtime(format!("Failed to write cache file: {}", e)))?;

        // Update statistics
        {
            let mut stats = self.stats.write();
            stats.total_cache_size_bytes += data.len() as u64;
        }

        debug!("Stored AST cache for {}", unit.module_name);
        Ok(())
    }

    /// Invalidate cache entries for a module and its dependents
    pub fn invalidate_module(&self, module_name: &str, affected_modules: &HashSet<String>) -> Result<()> {
        let mut invalidated_count = 0;

        // Remove from memory cache
        {
            let mut memory_cache = self.memory_cache.lock();
            let keys_to_remove: Vec<String> = memory_cache.iter()
                .filter(|(_, cached)| {
                    affected_modules.contains(&cached.module_name) ||
                    cached.dependencies.contains(&module_name.to_string())
                })
                .map(|(key, _)| key.clone())
                .collect();
            
            for key in keys_to_remove {
                memory_cache.pop(&key);
                invalidated_count += 1;
            }
        }

        // Remove from disk cache
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".cache") {
                        if let Ok(data) = fs::read(entry.path()) {
                            if let Ok(cached) = bincode::deserialize::<CachedAst>(&data) {
                                if affected_modules.contains(&cached.module_name) ||
                                   cached.dependencies.contains(&module_name.to_string()) {
                                    let _ = fs::remove_file(entry.path());
                                    invalidated_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.write();
            stats.cache_invalidations += invalidated_count;
        }

        info!("Invalidated {} cache entries for module {} and dependents", invalidated_count, module_name);
        Ok(())
    }

    /// Get cache statistics
    pub fn get_statistics(&self) -> CacheStatistics {
        self.stats.read().clone()
    }

    /// Clear all cache
    pub fn clear(&self) -> Result<()> {
        // Clear memory cache
        {
            let mut memory_cache = self.memory_cache.lock();
            memory_cache.clear();
        }

        // Clear disk cache
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".cache") {
                        let _ = fs::remove_file(entry.path());
                    }
                }
            }
        }

        // Reset statistics
        {
            let mut stats = self.stats.write();
            *stats = CacheStatistics::default();
        }

        info!("Cleared all AST cache");
        Ok(())
    }
}

/// Performance monitoring for compilation speed
#[derive(Debug, Clone)]
pub struct CompilationPerformanceMonitor {
    /// Phase timing data
    phase_timings: Arc<ParkingLotRwLock<HashMap<String, Vec<Duration>>>>,
    /// Memory usage tracking
    memory_usage: Arc<ParkingLotRwLock<Vec<(SystemTime, usize)>>>,
    /// Bottleneck detection
    bottlenecks: Arc<ParkingLotRwLock<HashMap<String, BottleneckInfo>>>,
}

#[derive(Debug, Clone)]
pub struct BottleneckInfo {
    pub phase_name: String,
    pub average_time: Duration,
    pub max_time: Duration,
    pub occurrence_count: usize,
    pub last_occurrence: SystemTime,
}

impl CompilationPerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            phase_timings: Arc::new(ParkingLotRwLock::new(HashMap::new())),
            memory_usage: Arc::new(ParkingLotRwLock::new(Vec::new())),
            bottlenecks: Arc::new(ParkingLotRwLock::new(HashMap::new())),
        }
    }

    /// Record phase timing
    pub fn record_phase_timing(&self, phase_name: &str, duration: Duration) {
        let mut timings = self.phase_timings.write();
        timings.entry(phase_name.to_string()).or_insert_with(Vec::new).push(duration);

        // Detect bottlenecks (phases taking >100ms)
        if duration.as_millis() > 100 {
            let mut bottlenecks = self.bottlenecks.write();
            let info = bottlenecks.entry(phase_name.to_string()).or_insert_with(|| BottleneckInfo {
                phase_name: phase_name.to_string(),
                average_time: Duration::default(),
                max_time: Duration::default(),
                occurrence_count: 0,
                last_occurrence: SystemTime::now(),
            });

            info.occurrence_count += 1;
            info.last_occurrence = SystemTime::now();
            info.max_time = info.max_time.max(duration);
            info.average_time = Duration::from_millis(
                (info.average_time.as_millis() + duration.as_millis()) as u64 / 2
            );
        }
    }

    /// Record memory usage
    pub fn record_memory_usage(&self, bytes: usize) {
        let mut usage = self.memory_usage.write();
        usage.push((SystemTime::now(), bytes));
        
        // Keep only last 1000 measurements
        if usage.len() > 1000 {
            usage.drain(0..usage.len()-1000);
        }
    }

    /// Get average phase timing
    pub fn get_average_phase_timing(&self, phase_name: &str) -> Option<Duration> {
        let timings = self.phase_timings.read();
        if let Some(phase_timings) = timings.get(phase_name) {
            if !phase_timings.is_empty() {
                let total: Duration = phase_timings.iter().sum();
                return Some(total / phase_timings.len() as u32);
            }
        }
        None
    }

    /// Get bottlenecks
    pub fn get_bottlenecks(&self) -> Vec<BottleneckInfo> {
        let bottlenecks = self.bottlenecks.read();
        bottlenecks.values().cloned().collect()
    }

    /// Generate performance report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("# Compilation Performance Report\n\n");

        // Phase timings
        report.push_str("## Phase Timings\n");
        let timings = self.phase_timings.read();
        let mut sorted_phases: Vec<_> = timings.iter().collect();
        sorted_phases.sort_by_key(|(_, durations)| {
            let total: Duration = durations.iter().sum();
            std::cmp::Reverse(total)
        });

        for (phase, durations) in sorted_phases {
            let total: Duration = durations.iter().sum();
            let avg = total / durations.len() as u32;
            let max = durations.iter().max().cloned().unwrap_or_default();
            report.push_str(&format!(
                "- {}: avg={}μs, max={}μs, total={}ms, count={}\n",
                phase, avg.as_micros(), max.as_micros(), total.as_millis(), durations.len()
            ));
        }

        // Bottlenecks
        report.push_str("\n## Detected Bottlenecks\n");
        let bottlenecks = self.bottlenecks.read();
        if bottlenecks.is_empty() {
            report.push_str("No bottlenecks detected.\n");
        } else {
            for info in bottlenecks.values() {
                report.push_str(&format!(
                    "- {}: avg={}ms, max={}ms, occurrences={}\n",
                    info.phase_name,
                    info.average_time.as_millis(),
                    info.max_time.as_millis(),
                    info.occurrence_count
                ));
            }
        }

        // Memory usage summary
        report.push_str("\n## Memory Usage\n");
        let usage = self.memory_usage.read();
        if !usage.is_empty() {
            let max_mem = usage.iter().map(|(_, bytes)| *bytes).max().unwrap_or(0);
            let avg_mem = usage.iter().map(|(_, bytes)| *bytes).sum::<usize>() / usage.len();
            report.push_str(&format!("- Peak memory: {} MB\n", max_mem / 1024 / 1024));
            report.push_str(&format!("- Average memory: {} MB\n", avg_mem / 1024 / 1024));
        }

        report
    }
}

/// Enhanced parallel type checking with dependency analysis
pub struct ParallelTypeChecker {
    /// Configuration
    config: CompilationSpeedConfig,
    /// Type cache
    type_cache: Arc<TypeCheckingOptimizer>,
    /// Performance monitor
    performance_monitor: Arc<CompilationPerformanceMonitor>,
}

impl ParallelTypeChecker {
    /// Create a new parallel type checker
    pub fn new(config: CompilationSpeedConfig, type_cache: Arc<TypeCheckingOptimizer>) -> Self {
        Self {
            config,
            type_cache,
            performance_monitor: Arc::new(CompilationPerformanceMonitor::new()),
        }
    }

    /// Perform parallel type checking on multiple programs
    #[instrument(skip(self, programs))]
    pub fn check_types_parallel(&self, programs: Vec<(String, Program)>) -> Result<Vec<(String, TypeCheckResult)>> {
        let start_time = Instant::now();
        info!("Starting parallel type checking for {} modules", programs.len());

        // Use rayon for parallel processing
        let results: Result<Vec<_>> = programs
            .into_par_iter()
            .map(|(module_name, program)| {
                let phase_start = Instant::now();
                let result = self.type_cache.check_types(&module_name, &program);
                self.performance_monitor.record_phase_timing(
                    &format!("type_check_{}", module_name),
                    phase_start.elapsed()
                );
                result.map(|r| (module_name, r))
            })
            .collect();

        let total_time = start_time.elapsed();
        self.performance_monitor.record_phase_timing("parallel_type_checking", total_time);
        
        info!("Parallel type checking completed in {}ms", total_time.as_millis());
        results
    }

    /// Get performance statistics
    pub fn get_performance_monitor(&self) -> Arc<CompilationPerformanceMonitor> {
        self.performance_monitor.clone()
    }
}

/// Main compilation speed optimizer
pub struct CompilationSpeedOptimizer {
    /// Configuration
    config: CompilationSpeedConfig,
    /// Dependency graph
    dependency_graph: Arc<Mutex<DependencyGraph>>,
    /// Parallel AST processor
    ast_processor: Arc<ParallelAstProcessor>,
    /// Type checking optimizer
    type_checker: Arc<TypeCheckingOptimizer>,
    /// AST cache for incremental compilation
    ast_cache: Arc<AstCache>,
    /// Parallel type checker
    parallel_type_checker: Arc<ParallelTypeChecker>,
    /// Performance monitor
    performance_monitor: Arc<CompilationPerformanceMonitor>,
    /// Compilation statistics
    stats: Arc<RwLock<CompilationStatistics>>,
}

#[derive(Debug, Clone)]
pub struct CompilationStatistics {
    pub total_units: usize,
    pub completed_units: usize,
    pub failed_units: usize,
    pub cached_units: usize,
    pub total_compilation_time: Duration,
    pub average_unit_time: Duration,
    pub parallelization_efficiency: f64,
    pub cache_hit_rate: f64,
    pub memory_cache_hits: u64,
    pub disk_cache_hits: u64,
    pub cache_misses: u64,
    pub peak_memory_usage_mb: usize,
    pub bottleneck_count: usize,
}

impl CompilationSpeedOptimizer {
    /// Create a new compilation speed optimizer
    pub fn new(config: &super::OptimizationConfig) -> Result<Self> {
        let speed_config = CompilationSpeedConfig {
            enable_parallel_compilation: config.enable_parallel_compilation,
            enable_incremental_compilation: config.enable_incremental_compilation,
            max_parallel_threads: config.max_parallel_threads,
            ..Default::default()
        };

        let ast_processor = Arc::new(ParallelAstProcessor::new(speed_config.clone())?);
        let type_checker = Arc::new(TypeCheckingOptimizer::new(speed_config.clone()));
        let ast_cache = Arc::new(AstCache::new(speed_config.cache_directory.clone())?);
        let parallel_type_checker = Arc::new(ParallelTypeChecker::new(speed_config.clone(), type_checker.clone()));
        let performance_monitor = Arc::new(CompilationPerformanceMonitor::new());

        Ok(Self {
            config: speed_config,
            dependency_graph: Arc::new(Mutex::new(DependencyGraph::new())),
            ast_processor,
            type_checker,
            ast_cache,
            parallel_type_checker,
            performance_monitor,
            stats: Arc::new(RwLock::new(CompilationStatistics {
                total_units: 0,
                completed_units: 0,
                failed_units: 0,
                cached_units: 0,
                total_compilation_time: Duration::default(),
                average_unit_time: Duration::default(),
                parallelization_efficiency: 0.0,
                cache_hit_rate: 0.0,
                memory_cache_hits: 0,
                disk_cache_hits: 0,
                cache_misses: 0,
                peak_memory_usage_mb: 0,
                bottleneck_count: 0,
            })),
        })
    }

    /// Compile multiple units with incremental caching and parallel processing
    #[instrument(skip(self, units))]
    pub fn compile_incremental(&self, mut units: Vec<CompilationUnit>) -> Result<Vec<(String, Result<Program>)>> {
        let start_time = Instant::now();
        self.performance_monitor.record_memory_usage(std::mem::size_of_val(&units));
        
        info!("Starting incremental compilation of {} units", units.len());

        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_units = units.len();
        }

        // Calculate content hashes for all units
        for unit in &mut units {
            unit.content_hash = AstCache::calculate_content_hash(unit);
        }

        // Check cache and separate cached vs. uncached units
        let mut cached_results = Vec::new();
        let mut units_to_compile = Vec::new();

        for unit in units {
            if self.config.enable_incremental_compilation {
                if let Some(cached_ast) = self.ast_cache.get_cached_ast(&unit) {
                    debug!("Using cached AST for {}", unit.module_name);
                    cached_results.push((unit.id.clone(), Ok(cached_ast.program)));
                    {
                        let mut stats = self.stats.write().unwrap();
                        stats.cached_units += 1;
                    }
                    continue;
                }
            }
            units_to_compile.push(unit);
        }

        info!("Cache hits: {}, units to compile: {}", cached_results.len(), units_to_compile.len());

        // Build dependency graph for units that need compilation
        {
            let mut graph = self.dependency_graph.lock().unwrap();
            for unit in &units_to_compile {
                for dep in &unit.dependencies {
                    graph.add_dependency(&unit.id, dep);
                }
            }
        }

        // Get compilation order
        let compilation_order = {
            let mut graph = self.dependency_graph.lock().unwrap();
            graph.topological_sort()?
        };

        // Group units by dependency level for parallel compilation
        let dependency_levels = self.group_by_dependency_level(&units_to_compile, &compilation_order)?;
        let mut compilation_results = Vec::new();

        // Compile each level in parallel with enhanced processing
        for (level_index, level) in dependency_levels.into_iter().enumerate() {
            let level_start = Instant::now();
            debug!("Compiling dependency level {} with {} units", level_index, level.len());
            
            // Process level in parallel using rayon
            let level_results: Vec<(String, Result<Program>)> = level
                .into_par_iter()
                .map(|unit| {
                    let unit_start = Instant::now();
                    self.performance_monitor.record_phase_timing(
                        &format!("unit_start_{}", unit.module_name),
                        Duration::from_nanos(1)
                    );

                    // Parse the unit
                    let parse_result = Self::parse_unit_with_timing(&unit, &self.performance_monitor);
                    
                    // Cache successful results
                    if let Ok(ref program) = parse_result {
                        let compilation_time_us = unit_start.elapsed().as_micros() as u64;
                        if let Err(e) = self.ast_cache.store_ast(&unit, program.clone(), compilation_time_us) {
                            warn!("Failed to cache AST for {}: {}", unit.module_name, e);
                        }
                    }

                    self.performance_monitor.record_phase_timing(
                        &format!("unit_complete_{}", unit.module_name),
                        unit_start.elapsed()
                    );

                    (unit.id, parse_result)
                })
                .collect();

            let level_time = level_start.elapsed();
            self.performance_monitor.record_phase_timing(
                &format!("dependency_level_{}", level_index),
                level_time
            );

            compilation_results.extend(level_results);
        }

        // Combine cached and newly compiled results
        let mut all_results = cached_results;
        all_results.extend(compilation_results);

        // Perform parallel type checking if enabled
        if self.config.enable_parallel_type_checking {
            let type_check_start = Instant::now();
            let programs_for_type_check: Vec<(String, Program)> = all_results
                .iter()
                .filter_map(|(id, result)| {
                    if let Ok(program) = result {
                        Some((id.clone(), program.clone()))
                    } else {
                        None
                    }
                })
                .collect();

            if !programs_for_type_check.is_empty() {
                match self.parallel_type_checker.check_types_parallel(programs_for_type_check) {
                    Ok(type_results) => {
                        debug!("Parallel type checking completed for {} modules", type_results.len());
                    }
                    Err(e) => {
                        warn!("Parallel type checking failed: {}", e);
                    }
                }
            }

            self.performance_monitor.record_phase_timing("parallel_type_checking", type_check_start.elapsed());
        }

        // Update comprehensive statistics
        let total_compilation_time = start_time.elapsed();
        let cache_stats = self.ast_cache.get_statistics();
        
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_compilation_time = total_compilation_time;
            stats.completed_units = all_results.iter().filter(|(_, result)| result.is_ok()).count();
            stats.failed_units = all_results.iter().filter(|(_, result)| result.is_err()).count();
            stats.memory_cache_hits = cache_stats.cache_hits;
            stats.cache_misses = cache_stats.cache_misses;
            stats.bottleneck_count = self.performance_monitor.get_bottlenecks().len();
            
            if stats.total_units > 0 {
                stats.average_unit_time = total_compilation_time / stats.total_units as u32;
                stats.parallelization_efficiency = 
                    (stats.total_units as f64 * stats.average_unit_time.as_secs_f64()) / 
                    total_compilation_time.as_secs_f64();
                stats.cache_hit_rate = cache_stats.cache_hits as f64 / 
                    (cache_stats.cache_hits + cache_stats.cache_misses) as f64;
            }
        }

        self.performance_monitor.record_phase_timing("total_compilation", total_compilation_time);
        info!("Incremental compilation completed in {}ms (cache hit rate: {:.1}%)", 
              total_compilation_time.as_millis(),
              cache_stats.cache_hits as f64 / (cache_stats.cache_hits + cache_stats.cache_misses) as f64 * 100.0);

        Ok(all_results)
    }

    /// Parse a compilation unit with performance timing
    fn parse_unit_with_timing(unit: &CompilationUnit, monitor: &CompilationPerformanceMonitor) -> Result<Program> {
        let parse_start = Instant::now();
        
        let lexer = Lexer::new(unit.source_code.clone());
        let mut parser = Parser::new(lexer)?;
        
        monitor.record_phase_timing(&format!("lexer_{}", unit.module_name), parse_start.elapsed());
        
        let parse_start = Instant::now();
        let program = parser.parse_program()?;
        monitor.record_phase_timing(&format!("parser_{}", unit.module_name), parse_start.elapsed());
        
        // Check for parse errors
        let errors = parser.errors();
        if !errors.is_empty() {
            return Err(Error::Parse(format!("Parse errors in {}: {}", 
                                           unit.module_name, errors.join(", "))));
        }

        Ok(program)
    }

    /// Group compilation units by dependency level
    fn group_by_dependency_level(
        &self,
        units: &[CompilationUnit],
        compilation_order: &[String],
    ) -> Result<Vec<Vec<CompilationUnit>>> {
        let unit_map: HashMap<String, &CompilationUnit> = 
            units.iter().map(|unit| (unit.id.clone(), unit)).collect();
        
        let graph = self.dependency_graph.lock().unwrap();
        let mut levels = Vec::new();
        let mut processed = HashSet::new();

        for unit_id in compilation_order {
            if processed.contains(unit_id) {
                continue;
            }

            // Find all units at this dependency level
            let mut current_level = Vec::new();
            let dependencies = graph.get_dependencies(unit_id);
            
            // Check if all dependencies are already processed
            if dependencies.iter().all(|dep| processed.contains(dep)) {
                if let Some(unit) = unit_map.get(unit_id) {
                    current_level.push((*unit).clone());
                    processed.insert(unit_id.clone());
                }
                
                // Look for other units at the same level
                for other_id in compilation_order {
                    if processed.contains(other_id) {
                        continue;
                    }
                    
                    let other_deps = graph.get_dependencies(other_id);
                    if other_deps.iter().all(|dep| processed.contains(dep)) {
                        if let Some(unit) = unit_map.get(other_id) {
                            current_level.push((*unit).clone());
                            processed.insert(other_id.clone());
                        }
                    }
                }
                
                if !current_level.is_empty() {
                    levels.push(current_level);
                }
            }
        }

        Ok(levels)
    }

    /// Invalidate cache for changed modules
    pub fn invalidate_cache_for_changes(&self, changed_files: &[PathBuf]) -> Result<()> {
        let mut modules_to_invalidate = HashSet::new();
        
        // Find modules corresponding to changed files
        for file_path in changed_files {
            if let Some(file_name) = file_path.file_stem() {
                if let Some(module_name) = file_name.to_str() {
                    modules_to_invalidate.insert(module_name.to_string());
                }
            }
        }

        // Get dependency graph to find affected modules
        let affected_modules = {
            let graph = self.dependency_graph.lock().unwrap();
            let mut all_affected = HashSet::new();
            for module in &modules_to_invalidate {
                let affected = graph.get_affected_modules(module);
                all_affected.extend(affected);
            }
            all_affected
        };

        // Invalidate cache for all affected modules
        for module in &modules_to_invalidate {
            self.ast_cache.invalidate_module(module, &affected_modules)?;
        }

        info!("Invalidated cache for {} changed modules, {} total affected", 
              modules_to_invalidate.len(), affected_modules.len());
        Ok(())
    }

    /// Get comprehensive compilation statistics
    pub fn get_statistics(&self) -> CompilationStatistics {
        let mut stats = self.stats.read().unwrap().clone();
        let cache_stats = self.ast_cache.get_statistics();
        
        // Update cache statistics
        stats.memory_cache_hits = cache_stats.cache_hits;
        stats.cache_misses = cache_stats.cache_misses;
        stats.bottleneck_count = self.performance_monitor.get_bottlenecks().len();
        
        // Calculate cache hit rate
        if cache_stats.cache_hits + cache_stats.cache_misses > 0 {
            stats.cache_hit_rate = cache_stats.cache_hits as f64 / 
                (cache_stats.cache_hits + cache_stats.cache_misses) as f64;
        }
        
        stats
    }

    /// Get performance monitor for detailed analysis
    pub fn get_performance_monitor(&self) -> Arc<CompilationPerformanceMonitor> {
        self.performance_monitor.clone()
    }

    /// Get AST cache for direct access
    pub fn get_ast_cache(&self) -> Arc<AstCache> {
        self.ast_cache.clone()
    }

    /// Clear all caches
    pub fn clear_caches(&self) -> Result<()> {
        self.ast_cache.clear()?;
        self.type_checker.clear_cache();
        info!("Cleared all compilation caches");
        Ok(())
    }

    /// Generate comprehensive compilation performance report
    pub fn generate_performance_report(&self) -> String {
        let stats = self.get_statistics();
        let processor_stats = self.ast_processor.get_statistics();
        let cache_stats = self.ast_cache.get_statistics();
        let bottlenecks = self.performance_monitor.get_bottlenecks();

        let mut report = String::new();
        report.push_str("# Comprehensive Compilation Performance Report\n\n");
        
        report.push_str("## Overall Statistics\n");
        report.push_str(&format!("- Total compilation units: {}\n", stats.total_units));
        report.push_str(&format!("- Successfully compiled: {}\n", stats.completed_units));
        report.push_str(&format!("- Failed compilations: {}\n", stats.failed_units));
        report.push_str(&format!("- Cached units: {}\n", stats.cached_units));
        report.push_str(&format!("- Total compilation time: {}ms\n", stats.total_compilation_time.as_millis()));
        report.push_str(&format!("- Average unit time: {}μs\n", stats.average_unit_time.as_micros()));
        report.push_str(&format!("- Parallelization efficiency: {:.1}%\n", stats.parallelization_efficiency * 100.0));
        report.push_str(&format!("- Cache hit rate: {:.1}%\n", stats.cache_hit_rate * 100.0));
        report.push_str(&format!("- Peak memory usage: {} MB\n\n", stats.peak_memory_usage_mb));
        
        report.push_str("## Cache Performance\n");
        report.push_str(&format!("- Memory cache hits: {}\n", cache_stats.cache_hits));
        report.push_str(&format!("- Cache misses: {}\n", cache_stats.cache_misses));
        report.push_str(&format!("- Cache invalidations: {}\n", cache_stats.cache_invalidations));
        report.push_str(&format!("- Total cache size: {} MB\n", cache_stats.total_cache_size_bytes / 1024 / 1024));
        report.push_str(&format!("- Average hit time: {}μs\n", cache_stats.average_hit_time_us));
        report.push_str(&format!("- Average miss time: {}μs\n\n", cache_stats.average_miss_time_us));
        
        report.push_str("## Parallel Processing\n");
        report.push_str(&format!("- Total workers: {}\n", processor_stats.total_workers));
        report.push_str(&format!("- Active workers: {}\n", processor_stats.active_workers));
        report.push_str(&format!("- Queue length: {}\n", processor_stats.queue_length));
        report.push_str(&format!("- Completed results: {}\n\n", processor_stats.completed_results));

        // Add performance monitoring details
        report.push_str("## Performance Analysis\n");
        if bottlenecks.is_empty() {
            report.push_str("- No performance bottlenecks detected\n\n");
        } else {
            report.push_str(&format!("- {} performance bottlenecks detected:\n", bottlenecks.len()));
            for bottleneck in &bottlenecks {
                report.push_str(&format!(
                    "  - {}: avg={}ms, max={}ms, occurrences={}\n",
                    bottleneck.phase_name,
                    bottleneck.average_time.as_millis(),
                    bottleneck.max_time.as_millis(),
                    bottleneck.occurrence_count
                ));
            }
            report.push_str("\n");
        }

        // Add compilation speed metrics
        if stats.total_units > 0 && stats.total_compilation_time.as_millis() > 0 {
            let units_per_second = (stats.total_units as f64 * 1000.0) / stats.total_compilation_time.as_millis() as f64;
            report.push_str("## Speed Metrics\n");
            report.push_str(&format!("- Compilation speed: {:.1} units/second\n", units_per_second));
            
            if stats.cached_units > 0 {
                let cache_speedup = stats.cached_units as f64 / stats.total_units as f64;
                report.push_str(&format!("- Cache acceleration: {:.1}x speedup\n", 1.0 + cache_speedup));
            }
            
            if stats.parallelization_efficiency > 0.0 {
                report.push_str(&format!("- Parallel efficiency: {:.1}x theoretical speedup\n", stats.parallelization_efficiency));
            }
        }

        // Add detailed performance monitor report
        report.push_str("\n## Detailed Performance Analysis\n");
        report.push_str(&self.performance_monitor.generate_report());

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        
        // Add dependencies: A -> B, B -> C
        graph.add_dependency("A", "B");
        graph.add_dependency("B", "C");
        
        let order = graph.topological_sort().unwrap();
        assert_eq!(order, vec!["C", "B", "A"]);
        
        let affected = graph.get_affected_modules("C");
        assert!(affected.contains("A"));
        assert!(affected.contains("B"));
        assert!(affected.contains("C"));
    }

    #[test]
    fn test_compilation_unit() {
        let mut unit = CompilationUnit {
            id: "test_module".to_string(),
            source_path: PathBuf::from("test.csd"),
            module_name: "test".to_string(),
            source_code: "facts x = 42;".to_string(),
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
            content_hash: String::new(),
        };
        
        unit.content_hash = AstCache::calculate_content_hash(&unit);
        
        assert_eq!(unit.module_name, "test");
        assert_eq!(unit.status, CompilationStatus::Pending);
        assert!(!unit.content_hash.is_empty());
    }

    #[test]
    fn test_parallel_ast_processor() {
        let config = CompilationSpeedConfig {
            max_parallel_threads: 2,
            ..Default::default()
        };
        
        let processor = ParallelAstProcessor::new(config).unwrap();
        
        let mut unit = CompilationUnit {
            id: "test".to_string(),
            source_path: PathBuf::from("test.csd"),
            module_name: "test".to_string(),
            source_code: "facts x = 42;".to_string(),
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
            content_hash: String::new(),
        };
        
        unit.content_hash = AstCache::calculate_content_hash(&unit);
        
        processor.submit_unit(unit).unwrap();
        processor.wait_for_completion().unwrap();
        
        let results = processor.get_results();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_ast_cache() {
        let temp_dir = TempDir::new().unwrap();
        let cache = AstCache::new(temp_dir.path().to_path_buf()).unwrap();
        
        let mut unit = CompilationUnit {
            id: "test_cache".to_string(),
            source_path: PathBuf::from("test.csd"),
            module_name: "test_cache".to_string(),
            source_code: "facts y = 123;".to_string(),
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
            content_hash: String::new(),
        };
        
        unit.content_hash = AstCache::calculate_content_hash(&unit);
        
        // Cache miss initially
        assert!(cache.get_cached_ast(&unit).is_none());
        
        // Parse and cache
        let lexer = Lexer::new(unit.source_code.clone());
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();
        
        cache.store_ast(&unit, program.clone(), 1000).unwrap();
        
        // Cache hit now
        let cached = cache.get_cached_ast(&unit);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().content_hash, unit.content_hash);
        
        let stats = cache.get_statistics();
        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.cache_misses, 1);
    }

    #[test]
    fn test_performance_monitor() {
        let monitor = CompilationPerformanceMonitor::new();
        
        // Record some timings
        monitor.record_phase_timing("test_phase", Duration::from_millis(50));
        monitor.record_phase_timing("slow_phase", Duration::from_millis(150));
        monitor.record_memory_usage(1024 * 1024); // 1MB
        
        // Check bottleneck detection
        let bottlenecks = monitor.get_bottlenecks();
        assert_eq!(bottlenecks.len(), 1); // Only slow_phase should be detected
        assert_eq!(bottlenecks[0].phase_name, "slow_phase");
        
        // Check average timing
        let avg = monitor.get_average_phase_timing("test_phase");
        assert!(avg.is_some());
        assert_eq!(avg.unwrap(), Duration::from_millis(50));
        
        // Generate report
        let report = monitor.generate_report();
        assert!(report.contains("test_phase"));
        assert!(report.contains("slow_phase"));
    }
}
