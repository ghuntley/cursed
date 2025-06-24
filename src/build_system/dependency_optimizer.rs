
//! Smart Dependency Analysis for Optimized Compilation
//! 
//! Provides intelligent dependency resolution that minimizes compilation work
//! through advanced graph analysis and parallel execution optimization.

use std::collections::{HashMap, HashSet, VecDeque, BTreeMap};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn, instrument};

use crate::error::{CursedError, Result};

/// Represents a compilation unit with its dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationUnit {
    pub id: String,
    pub path: PathBuf,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
    pub last_modified: u64,
    pub compilation_time: Duration,
    pub complexity_score: u32,
    pub is_dirty: bool,
    pub cache_key: String,
}

/// Dependency graph for smart compilation ordering
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    pub nodes: HashMap<String, CompilationUnit>,
    pub edges: HashMap<String, Vec<String>>,
    pub reverse_edges: HashMap<String, Vec<String>>,
    pub changed_files: HashSet<String>,
    pub compilation_layers: Vec<Vec<String>>,
}

/// Configuration for dependency optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyOptimizerConfig {
    pub max_parallel_jobs: usize,
    pub enable_partial_rebuilds: bool,
    pub enable_smart_ordering: bool,
    pub enable_dependency_pruning: bool,
    pub complexity_threshold: u32,
    pub cache_dependency_graph: bool,
    pub incremental_analysis: bool,
    pub max_graph_depth: usize,
}

impl Default for DependencyOptimizerConfig {
    fn default() -> Self {
        Self {
            max_parallel_jobs: num_cpus::get(),
            enable_partial_rebuilds: true,
            enable_smart_ordering: true,
            enable_dependency_pruning: true,
            complexity_threshold: 1000,
            cache_dependency_graph: true,
            incremental_analysis: true,
            max_graph_depth: 100,
        }
    }
}

/// Smart dependency analyzer and optimizer
pub struct DependencyOptimizer {
    config: DependencyOptimizerConfig,
    dependency_graph: Arc<RwLock<DependencyGraph>>,
    compilation_history: Arc<Mutex<HashMap<String, Vec<Duration>>>>,
    parallel_executor: Arc<Mutex<ParallelExecutor>>,
    cache: Arc<RwLock<DependencyCache>>,
}

/// Parallel execution coordinator
#[derive(Debug)]
pub struct ParallelExecutor {
    active_jobs: HashMap<String, Instant>,
    completed_jobs: HashSet<String>,
    failed_jobs: HashMap<String, String>,
    job_queue: VecDeque<String>,
    max_concurrent: usize,
}

/// Cache for dependency analysis results
#[derive(Debug, Clone)]
pub struct DependencyCache {
    graph_cache: HashMap<String, DependencyGraph>,
    analysis_cache: HashMap<String, AnalysisResult>,
    timestamp_cache: HashMap<String, u64>,
    invalidation_keys: HashSet<String>,
}

/// Result of dependency analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub compilation_order: Vec<Vec<String>>,
    pub affected_units: HashSet<String>,
    pub estimated_time: Duration,
    pub parallelism_factor: f64,
    pub cache_hits: usize,
    pub optimization_suggestions: Vec<String>,
}

/// Build optimization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStats {
    pub total_units: usize,
    pub rebuilt_units: usize,
    pub cached_units: usize,
    pub parallel_efficiency: f64,
    pub time_saved: Duration,
    pub cache_hit_rate: f64,
}

impl DependencyOptimizer {
    /// Create a new dependency optimizer with configuration
    #[instrument]
    pub fn new(config: DependencyOptimizerConfig) -> Self {
        info!("Creating dependency optimizer with {} max parallel jobs", config.max_parallel_jobs);
        
        Self {
            parallel_executor: Arc::new(Mutex::new(ParallelExecutor {
                active_jobs: HashMap::new(),
                completed_jobs: HashSet::new(),
                failed_jobs: HashMap::new(),
                job_queue: VecDeque::new(),
                max_concurrent: config.max_parallel_jobs,
            })),
            dependency_graph: Arc::new(RwLock::new(DependencyGraph {
                nodes: HashMap::new(),
                edges: HashMap::new(),
                reverse_edges: HashMap::new(),
                changed_files: HashSet::new(),
                compilation_layers: Vec::new(),
            })),
            compilation_history: Arc::new(Mutex::new(HashMap::new())),
            cache: Arc::new(RwLock::new(DependencyCache {
                graph_cache: HashMap::new(),
                analysis_cache: HashMap::new(),
                timestamp_cache: HashMap::new(),
                invalidation_keys: HashSet::new(),
            })),
            config,
        }
    }

    /// Analyze dependencies and create optimized compilation plan
    #[instrument(skip(self, units))]
    pub fn analyze_dependencies(&self, units: &[CompilationUnit]) -> Result<AnalysisResult> {
        let start = Instant::now();
        
        // Build dependency graph
        let mut graph = self.build_dependency_graph(units)?;
        
        // Detect cycles
        self.detect_cycles(&graph)?;
        
        // Calculate compilation layers for parallelization
        let layers = self.calculate_compilation_layers(&mut graph)?;
        
        // Determine affected units based on changes
        let affected_units = self.calculate_affected_units(&graph)?;
        
        // Optimize compilation order
        let optimized_order = self.optimize_compilation_order(&graph, &layers)?;
        
        // Calculate estimated time and parallelism
        let estimated_time = self.estimate_compilation_time(&optimized_order, &graph)?;
        let parallelism_factor = self.calculate_parallelism_factor(&optimized_order)?;
        
        // Generate optimization suggestions
        let suggestions = self.generate_optimization_suggestions(&graph)?;
        
        let result = AnalysisResult {
            compilation_order: optimized_order,
            affected_units,
            estimated_time,
            parallelism_factor,
            cache_hits: 0, // Will be updated during actual compilation
            optimization_suggestions: suggestions,
        };
        
        // Cache the result
        if self.config.cache_dependency_graph {
            self.cache_analysis_result(&result)?;
        }
        
        info!(
            analysis_time = ?start.elapsed(),
            total_units = units.len(),
            affected_units = result.affected_units.len(),
            "Dependency analysis completed"
        );
        
        Ok(result)
    }

    /// Build dependency graph from compilation units
    #[instrument(skip(self, units))]
    fn build_dependency_graph(&self, units: &[CompilationUnit]) -> Result<DependencyGraph> {
        let mut graph = DependencyGraph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            reverse_edges: HashMap::new(),
            changed_files: HashSet::new(),
            compilation_layers: Vec::new(),
        };
        
        // Add all nodes
        for unit in units {
            graph.nodes.insert(unit.id.clone(), unit.clone());
            graph.edges.entry(unit.id.clone()).or_insert_with(Vec::new);
            
            // Track changed files
            if unit.is_dirty {
                graph.changed_files.insert(unit.id.clone());
            }
        }
        
        // Build edges
        for unit in units {
            for dep_id in &unit.dependencies {
                // Forward edge
                graph.edges.entry(dep_id.clone())
                    .or_insert_with(Vec::new)
                    .push(unit.id.clone());
                
                // Reverse edge
                graph.reverse_edges.entry(unit.id.clone())
                    .or_insert_with(Vec::new)
                    .push(dep_id.clone());
            }
        }
        
        debug!(nodes = graph.nodes.len(), edges = graph.edges.len(), "Built dependency graph");
        Ok(graph)
    }

    /// Detect cycles in dependency graph
    #[instrument(skip(self, graph))]
    fn detect_cycles(&self, graph: &DependencyGraph) -> Result<()> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for node_id in graph.nodes.keys() {
            if !visited.contains(node_id) {
                if self.has_cycle_dfs(node_id, graph, &mut visited, &mut rec_stack)? {
                    return Err(CursedError::system_error(&format!(
                        "Circular dependency detected involving node: {}", node_id
                    )));
                }
            }
        }
        
        Ok(())
    }

    /// DFS helper for cycle detection
    fn has_cycle_dfs(
        &self,
        node_id: &str,
        graph: &DependencyGraph,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> Result<bool> {
        visited.insert(node_id.to_string());
        rec_stack.insert(node_id.to_string());
        
        if let Some(dependencies) = graph.reverse_edges.get(node_id) {
            for dep_id in dependencies {
                if !visited.contains(dep_id) {
                    if self.has_cycle_dfs(dep_id, graph, visited, rec_stack)? {
                        return Ok(true);
                    }
                } else if rec_stack.contains(dep_id) {
                    return Ok(true);
                }
            }
        }
        
        rec_stack.remove(node_id);
        Ok(false)
    }

    /// Calculate compilation layers for parallel execution
    #[instrument(skip(self, graph))]
    fn calculate_compilation_layers(&self, graph: &mut DependencyGraph) -> Result<Vec<Vec<String>>> {
        let mut layers = Vec::new();
        let mut remaining_nodes: HashSet<String> = graph.nodes.keys().cloned().collect();
        let mut dependency_counts: HashMap<String, usize> = HashMap::new();
        
        // Initialize dependency counts
        for node_id in &remaining_nodes {
            let count = graph.reverse_edges.get(node_id).map_or(0, |deps| deps.len());
            dependency_counts.insert(node_id.clone(), count);
        }
        
        while !remaining_nodes.is_empty() {
            let mut current_layer = Vec::new();
            
            // Find nodes with no dependencies
            let ready_nodes: Vec<String> = remaining_nodes
                .iter()
                .filter(|&node_id| dependency_counts.get(node_id).unwrap_or(&0) == &0)
                .cloned()
                .collect();
            
            if ready_nodes.is_empty() {
                return Err(CursedError::system_error(
                    "Unable to resolve dependencies - possible circular dependency"
                ));
            }
            
            // Sort by compilation complexity for optimal scheduling
            let mut sorted_ready = ready_nodes;
            sorted_ready.sort_by(|a, b| {
                let complexity_a = graph.nodes.get(a).map_or(0, |u| u.complexity_score);
                let complexity_b = graph.nodes.get(b).map_or(0, |u| u.complexity_score);
                complexity_b.cmp(&complexity_a) // Descending order
            });
            
            for node_id in &sorted_ready {
                current_layer.push(node_id.clone());
                remaining_nodes.remove(node_id);
                
                // Update dependency counts for dependents
                if let Some(dependents) = graph.edges.get(node_id) {
                    for dependent in dependents {
                        if let Some(count) = dependency_counts.get_mut(dependent) {
                            *count = count.saturating_sub(1);
                        }
                    }
                }
            }
            
            layers.push(current_layer);
        }
        
        graph.compilation_layers = layers.clone();
        debug!(layers = layers.len(), "Calculated compilation layers");
        Ok(layers)
    }

    /// Calculate affected units based on changes
    #[instrument(skip(self, graph))]
    fn calculate_affected_units(&self, graph: &DependencyGraph) -> Result<HashSet<String>> {
        let mut affected = HashSet::new();
        let mut to_process: VecDeque<String> = graph.changed_files.iter().cloned().collect();
        
        while let Some(node_id) = to_process.pop_front() {
            if affected.insert(node_id.clone()) {
                // Add all dependents to processing queue
                if let Some(dependents) = graph.edges.get(&node_id) {
                    for dependent in dependents {
                        to_process.push_back(dependent.clone());
                    }
                }
            }
        }
        
        debug!(affected_count = affected.len(), "Calculated affected units");
        Ok(affected)
    }

    /// Optimize compilation order within layers
    #[instrument(skip(self, graph, layers))]
    fn optimize_compilation_order(
        &self,
        graph: &DependencyGraph,
        layers: &[Vec<String>],
    ) -> Result<Vec<Vec<String>>> {
        let mut optimized_layers = Vec::new();
        
        for layer in layers {
            let mut optimized_layer = layer.clone();
            
            // Sort by compilation time (longest first for better parallelization)
            optimized_layer.sort_by(|a, b| {
                let time_a = graph.nodes.get(a).map_or(Duration::ZERO, |u| u.compilation_time);
                let time_b = graph.nodes.get(b).map_or(Duration::ZERO, |u| u.compilation_time);
                time_b.cmp(&time_a)
            });
            
            optimized_layers.push(optimized_layer);
        }
        
        Ok(optimized_layers)
    }

    /// Estimate total compilation time
    #[instrument(skip(self, layers, graph))]
    fn estimate_compilation_time(
        &self,
        layers: &[Vec<String>],
        graph: &DependencyGraph,
    ) -> Result<Duration> {
        let mut total_time = Duration::ZERO;
        
        for layer in layers {
            let mut layer_times: Vec<Duration> = layer
                .iter()
                .map(|id| graph.nodes.get(id).map_or(Duration::ZERO, |u| u.compilation_time))
                .collect();
            
            layer_times.sort_by(|a, b| b.cmp(a));
            
            // Calculate parallel execution time
            let parallel_chunks = layer_times.chunks(self.config.max_parallel_jobs);
            let layer_time = parallel_chunks
                .map(|chunk| chunk.iter().max().unwrap_or(&Duration::ZERO))
                .sum();
            
            total_time += layer_time;
        }
        
        Ok(total_time)
    }

    /// Calculate parallelism factor
    #[instrument(skip(self, layers))]
    fn calculate_parallelism_factor(&self, layers: &[Vec<String>]) -> Result<f64> {
        let total_units: usize = layers.iter().map(|layer| layer.len()).sum();
        let max_parallel_units = layers.iter().map(|layer| layer.len()).max().unwrap_or(1);
        let average_parallel_units = total_units as f64 / layers.len() as f64;
        
        Ok(average_parallel_units / self.config.max_parallel_jobs as f64)
    }

    /// Generate optimization suggestions
    #[instrument(skip(self, graph))]
    fn generate_optimization_suggestions(&self, graph: &DependencyGraph) -> Result<Vec<String>> {
        let mut suggestions = Vec::new();
        
        // Check for highly connected nodes (bottlenecks)
        let mut high_degree_nodes = Vec::new();
        for (node_id, dependents) in &graph.edges {
            if dependents.len() > 10 {
                high_degree_nodes.push((node_id.clone(), dependents.len()));
            }
        }
        
        if !high_degree_nodes.is_empty() {
            high_degree_nodes.sort_by(|a, b| b.1.cmp(&a.1));
            suggestions.push(format!(
                "Consider refactoring high-degree nodes to reduce dependencies: {:?}",
                high_degree_nodes.iter().take(3).map(|(id, _)| id).collect::<Vec<_>>()
            ));
        }
        
        // Check for unbalanced layers
        let layer_sizes: Vec<usize> = graph.compilation_layers.iter().map(|layer| layer.len()).collect();
        let max_layer_size = layer_sizes.iter().max().unwrap_or(&0);
        let avg_layer_size = layer_sizes.iter().sum::<usize>() as f64 / layer_sizes.len() as f64;
        
        if *max_layer_size as f64 > avg_layer_size * 2.0 {
            suggestions.push("Consider breaking up large compilation layers to improve parallelism".to_string());
        }
        
        // Check for very long dependency chains
        if graph.compilation_layers.len() > 20 {
            suggestions.push("Long dependency chains detected - consider architecture refactoring".to_string());
        }
        
        Ok(suggestions)
    }

    /// Cache analysis result
    #[instrument(skip(self, result))]
    fn cache_analysis_result(&self, result: &AnalysisResult) -> Result<()> {
        if let Ok(mut cache) = self.cache.write() {
            let cache_key = format!("{:?}", result.compilation_order);
            cache.analysis_cache.insert(cache_key, result.clone());
            debug!("Cached analysis result");
        }
        Ok(())
    }

    /// Execute compilation with optimized scheduling
    #[instrument(skip(self, analysis_result, compile_fn))]
    pub fn execute_optimized_compilation(
        &self,
        analysis_result: &AnalysisResult,
        compile_fn: impl Fn(&str) -> Result<()> + Send + Sync + 'static,
    ) -> Result<OptimizationStats> {
        let start = Instant::now();
        let compile_fn = Arc::new(compile_fn);
        let mut stats = OptimizationStats {
            total_units: analysis_result.compilation_order.iter().map(|layer| layer.len()).sum(),
            rebuilt_units: 0,
            cached_units: 0,
            parallel_efficiency: 0.0,
            time_saved: Duration::ZERO,
            cache_hit_rate: 0.0,
        };
        
        for layer in &analysis_result.compilation_order {
            self.execute_layer_parallel(layer, compile_fn.clone(), &mut stats)?;
        }
        
        let total_time = start.elapsed();
        stats.parallel_efficiency = analysis_result.estimated_time.as_secs_f64() / total_time.as_secs_f64();
        stats.time_saved = analysis_result.estimated_time.saturating_sub(total_time);
        stats.cache_hit_rate = stats.cached_units as f64 / stats.total_units as f64;
        
        info!(
            total_time = ?total_time,
            rebuilt_units = stats.rebuilt_units,
            cached_units = stats.cached_units,
            parallel_efficiency = stats.parallel_efficiency,
            "Optimized compilation completed"
        );
        
        Ok(stats)
    }

    /// Execute a single layer in parallel
    #[instrument(skip(self, layer, compile_fn, stats))]
    fn execute_layer_parallel(
        &self,
        layer: &[String],
        compile_fn: Arc<dyn Fn(&str) -> Result<()> + Send + Sync>,
        stats: &mut OptimizationStats,
    ) -> Result<()> {
        use std::thread;
        use std::sync::mpsc;
        
        let (tx, rx) = mpsc::channel();
        let mut handles = Vec::new();
        
        // Spawn worker threads
        for chunk in layer.chunks(self.config.max_parallel_jobs) {
            for unit_id in chunk {
                let unit_id = unit_id.clone();
                let tx = tx.clone();
                let compile_fn = compile_fn.clone();
                
                let handle = thread::spawn(move || {
                    let result = compile_fn(&unit_id);
                    tx.send((unit_id, result)).unwrap();
                });
                
                handles.push(handle);
            }
        }
        
        drop(tx); // Close the channel
        
        // Collect results
        for (unit_id, result) in rx {
            match result {
                Ok(()) => {
                    stats.rebuilt_units += 1;
                    debug!(unit_id, "Compilation successful");
                }
                Err(e) => {
                    warn!(unit_id, error = ?e, "Compilation failed");
                    return Err(e);
                }
            }
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().map_err(|_| CursedError::system_error("Thread join failed"))?;
        }
        
        Ok(())
    }

    /// Update dependency graph with new file changes
    #[instrument(skip(self, changed_files))]
    pub fn update_changed_files(&self, changed_files: &[String]) -> Result<()> {
        if let Ok(mut graph) = self.dependency_graph.write() {
            for file in changed_files {
                graph.changed_files.insert(file.clone());
                
                // Mark the unit as dirty
                if let Some(unit) = graph.nodes.get_mut(file) {
                    unit.is_dirty = true;
                }
            }
            
            // Invalidate cache
            if let Ok(mut cache) = self.cache.write() {
                cache.invalidation_keys.extend(changed_files.iter().cloned());
            }
            
            debug!(changed_files = changed_files.len(), "Updated changed files");
        }
        
        Ok(())
    }

    /// Get optimization statistics
    pub fn get_statistics(&self) -> Result<OptimizationStats> {
        let graph = self.dependency_graph.read().map_err(|_| CursedError::system_error("Failed to read dependency graph"))?;
        
        Ok(OptimizationStats {
            total_units: graph.nodes.len(),
            rebuilt_units: graph.changed_files.len(),
            cached_units: graph.nodes.len() - graph.changed_files.len(),
            parallel_efficiency: 0.0, // Will be calculated during execution
            time_saved: Duration::ZERO,
            cache_hit_rate: 0.0,
        })
    }
}

// Types are exported directly via pub struct/pub enum definitions above
