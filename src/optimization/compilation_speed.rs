/// Compilation Speed Optimization for CURSED Compiler
/// 
/// Provides parallel compilation of independent modules, incremental compilation 
/// with dependency tracking, and build cache optimization.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use tracing::{debug, info, instrument, warn};
use crossbeam_channel::{bounded, Receiver, Sender};

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
#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone)]
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

        Ok(Self {
            config: speed_config,
            dependency_graph: Arc::new(Mutex::new(DependencyGraph::new())),
            ast_processor,
            type_checker,
            stats: Arc::new(RwLock::new(CompilationStatistics {
                total_units: 0,
                completed_units: 0,
                failed_units: 0,
                cached_units: 0,
                total_compilation_time: Duration::default(),
                average_unit_time: Duration::default(),
                parallelization_efficiency: 0.0,
                cache_hit_rate: 0.0,
            })),
        })
    }

    /// Compile multiple units in parallel
    #[instrument(skip(self, units))]
    pub fn compile_parallel(&self, units: Vec<CompilationUnit>) -> Result<Vec<(String, Result<Program>)>> {
        let start_time = Instant::now();
        info!("Starting parallel compilation of {} units", units.len());

        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_units = units.len();
        }

        // Build dependency graph
        {
            let mut graph = self.dependency_graph.lock().unwrap();
            for unit in &units {
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
        let dependency_levels = self.group_by_dependency_level(&units, &compilation_order)?;
        let mut all_results = Vec::new();

        // Compile each level in parallel
        for level in dependency_levels {
            debug!("Compiling dependency level with {} units", level.len());
            
            // Submit units to parallel processor
            for unit in &level {
                self.ast_processor.submit_unit(unit.clone())?;
            }

            // Wait for this level to complete
            self.ast_processor.wait_for_completion()?;

            // Collect results
            let results = self.ast_processor.get_results();
            all_results.extend(results);
        }

        // Update statistics
        let compilation_time = start_time.elapsed();
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_compilation_time = compilation_time;
            stats.completed_units = all_results.iter().filter(|(_, result)| result.is_ok()).count();
            stats.failed_units = all_results.iter().filter(|(_, result)| result.is_err()).count();
            
            if stats.total_units > 0 {
                stats.average_unit_time = compilation_time / stats.total_units as u32;
                stats.parallelization_efficiency = 
                    (stats.total_units as f64 * stats.average_unit_time.as_secs_f64()) / 
                    compilation_time.as_secs_f64();
            }
        }

        info!("Parallel compilation completed in {}ms", compilation_time.as_millis());
        Ok(all_results)
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

    /// Get compilation statistics
    pub fn get_statistics(&self) -> CompilationStatistics {
        let stats = self.stats.read().unwrap();
        stats.clone()
    }

    /// Generate compilation performance report
    pub fn generate_performance_report(&self) -> String {
        let stats = self.get_statistics();
        let processor_stats = self.ast_processor.get_statistics();

        let mut report = String::new();
        report.push_str("# Compilation Performance Report\n\n");
        
        report.push_str("## Overall Statistics\n");
        report.push_str(&format!("- Total compilation units: {}\n", stats.total_units));
        report.push_str(&format!("- Successfully compiled: {}\n", stats.completed_units));
        report.push_str(&format!("- Failed compilations: {}\n", stats.failed_units));
        report.push_str(&format!("- Cached units: {}\n", stats.cached_units));
        report.push_str(&format!("- Total compilation time: {}ms\n", stats.total_compilation_time.as_millis()));
        report.push_str(&format!("- Average unit time: {}μs\n", stats.average_unit_time.as_micros()));
        report.push_str(&format!("- Parallelization efficiency: {:.1}%\n", stats.parallelization_efficiency * 100.0));
        report.push_str(&format!("- Cache hit rate: {:.1}%\n\n", stats.cache_hit_rate * 100.0));
        
        report.push_str("## Parallel Processing\n");
        report.push_str(&format!("- Total workers: {}\n", processor_stats.total_workers));
        report.push_str(&format!("- Active workers: {}\n", processor_stats.active_workers));
        report.push_str(&format!("- Queue length: {}\n", processor_stats.queue_length));
        report.push_str(&format!("- Completed results: {}\n", processor_stats.completed_results));

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
        let unit = CompilationUnit {
            id: "test_module".to_string(),
            source_path: PathBuf::from("test.csd"),
            module_name: "test".to_string(),
            source_code: "facts x = 42;".to_string(),
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
        };
        
        assert_eq!(unit.module_name, "test");
        assert_eq!(unit.status, CompilationStatus::Pending);
    }

    #[test]
    fn test_parallel_ast_processor() {
        let config = CompilationSpeedConfig {
            max_parallel_threads: 2,
            ..Default::default()
        };
        
        let processor = ParallelAstProcessor::new(config).unwrap();
        
        let unit = CompilationUnit {
            id: "test".to_string(),
            source_path: PathBuf::from("test.csd"),
            module_name: "test".to_string(),
            source_code: "facts x = 42;".to_string(),
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
        };
        
        processor.submit_unit(unit).unwrap();
        processor.wait_for_completion().unwrap();
        
        let results = processor.get_results();
        assert!(!results.is_empty());
    }
}
