/// Dependency Analysis for Optimization
/// 
/// Analyzes compilation unit dependencies to optimize build order
/// and enable better parallelization and incremental compilation.

use crate::error::{Error, Result};
use crate::optimization::metrics::CompilationUnit;
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{debug, info, instrument};
use serde::{Deserialize, Serialize};

/// Dependency graph for compilation units
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// Dependencies: unit -> list of units it depends on
    dependencies: HashMap<String, Vec<String>>,
    /// Reverse dependencies: unit -> list of units that depend on it
    reverse_dependencies: HashMap<String, Vec<String>>,
    /// All units in the graph
    units: HashSet<String>,
}

impl DependencyGraph {
    /// Create new empty dependency graph
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            reverse_dependencies: HashMap::new(),
            units: HashSet::new(),
        }
    }
    
    /// Add a dependency relationship
    pub fn add_dependency(&mut self, unit: &str, dependency: &str) {
        // Add to dependencies
        self.dependencies
            .entry(unit.to_string())
            .or_insert_with(Vec::new)
            .push(dependency.to_string());
        
        // Add to reverse dependencies
        self.reverse_dependencies
            .entry(dependency.to_string())
            .or_insert_with(Vec::new)
            .push(unit.to_string());
        
        // Add units to the set
        self.units.insert(unit.to_string());
        self.units.insert(dependency.to_string());
    }
    
    /// Get dependencies for a unit
    pub fn get_dependencies_for(&self, unit: &str) -> Vec<String> {
        self.dependencies.get(unit).cloned().unwrap_or_default()
    }
    
    /// Get units that depend on this unit
    pub fn get_dependents_of(&self, unit: &str) -> Vec<String> {
        self.reverse_dependencies.get(unit).cloned().unwrap_or_default()
    }
    
    /// Get all dependencies map
    pub fn get_dependencies(&self) -> &HashMap<String, Vec<String>> {
        &self.dependencies
    }
    
    /// Topological sort of units
    pub fn topological_sort(&self, units: &[String]) -> Result<Vec<String>> {
        let mut result = Vec::new();
        let mut in_degree = HashMap::new();
        let mut queue = VecDeque::new();
        
        // Calculate in-degrees for the specific units
        for unit in units {
            let deps = self.get_dependencies_for(unit);
            let filtered_deps: Vec<_> = deps.iter()
                .filter(|dep| units.contains(dep))
                .collect();
            
            in_degree.insert(unit.clone(), filtered_deps.len());
            
            if filtered_deps.is_empty() {
                queue.push_back(unit.clone());
            }
        }
        
        // Process queue
        while let Some(unit) = queue.pop_front() {
            result.push(unit.clone());
            
            // Reduce in-degree of dependents
            for dependent in self.get_dependents_of(&unit) {
                if !units.contains(&dependent) {
                    continue;
                }
                
                if let Some(degree) = in_degree.get_mut(&dependent) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(dependent);
                    }
                }
            }
        }
        
        // Check for cycles
        if result.len() != units.len() {
            return Err(Error::OptimizationError("Circular dependency detected".to_string()));
        }
        
        Ok(result)
    }
    
    /// Find optimal compilation order for parallel execution
    pub fn find_optimal_compilation_order(&self, worker_count: usize) -> Result<CompilationPlan> {
        let all_units: Vec<String> = self.units.iter().cloned().collect();
        let ordered_units = self.topological_sort(&all_units)?;
        
        // Create batches for parallel execution
        let mut batches = Vec::new();
        let mut current_batch = Vec::new();
        let mut completed = HashSet::new();
        
        for unit in ordered_units {
            // Check if all dependencies are completed
            let deps = self.get_dependencies_for(&unit);
            let deps_ready = deps.iter().all(|dep| completed.contains(dep));
            
            if deps_ready {
                current_batch.push(unit.clone());
                
                // If batch is full or this is the last unit, finalize the batch
                if current_batch.len() >= worker_count {
                    batches.push(current_batch.clone());
                    for unit_name in &current_batch {
                        completed.insert(unit_name.clone());
                    }
                    current_batch.clear();
                }
            } else {
                // Finalize current batch and start new one
                if !current_batch.is_empty() {
                    batches.push(current_batch.clone());
                    for unit_name in &current_batch {
                        completed.insert(unit_name.clone());
                    }
                    current_batch.clear();
                }
                current_batch.push(unit);
            }
        }
        
        // Add final batch if not empty
        if !current_batch.is_empty() {
            batches.push(current_batch);
        }
        
        // Find critical path
        let critical_path = self.find_critical_path(&all_units)?;
        
        // Estimate total time
        let estimated_time = std::time::Duration::from_secs(batches.len() as u64 * 30); // 30 seconds per batch estimate
        
        Ok(CompilationPlan {
            batches,
            critical_path,
            estimated_total_time: estimated_time,
            max_parallelism: worker_count,
        })
    }
    
    /// Find the critical path (longest dependency chain)
    fn find_critical_path(&self, units: &[String]) -> Result<Vec<String>> {
        let mut longest_path = Vec::new();
        let mut max_length = 0;
        
        // Try each unit as a starting point
        for start_unit in units {
            let path = self.find_longest_path_from(start_unit, units)?;
            if path.len() > max_length {
                max_length = path.len();
                longest_path = path;
            }
        }
        
        Ok(longest_path)
    }
    
    /// Find longest dependency path from a unit
    fn find_longest_path_from(&self, start: &str, units: &[String]) -> Result<Vec<String>> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        
        self.dfs_longest_path(start, units, &mut visited, &mut path)?;
        
        Ok(path)
    }
    
    /// Depth-first search for longest path
    fn dfs_longest_path(
        &self,
        current: &str,
        units: &[String],
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Result<()> {
        if visited.contains(current) {
            return Ok(()); // Avoid cycles
        }
        
        visited.insert(current.to_string());
        path.push(current.to_string());
        
        let dependents = self.get_dependents_of(current);
        for dependent in dependents {
            if units.contains(&dependent) {
                self.dfs_longest_path(&dependent, units, visited, path)?;
            }
        }
        
        Ok(())
    }
    
    /// Check if the graph has cycles
    pub fn has_cycles(&self) -> bool {
        let all_units: Vec<String> = self.units.iter().cloned().collect();
        self.topological_sort(&all_units).is_err()
    }
    
    /// Get graph statistics
    pub fn get_statistics(&self) -> DependencyGraphStats {
        let total_dependencies: usize = self.dependencies.values().map(|deps| deps.len()).sum();
        let avg_dependencies = if self.units.is_empty() {
            0.0
        } else {
            total_dependencies as f64 / self.units.len() as f64
        };
        
        let max_dependencies = self.dependencies.values()
            .map(|deps| deps.len())
            .max()
            .unwrap_or(0);
        
        DependencyGraphStats {
            total_units: self.units.len(),
            total_dependencies,
            average_dependencies_per_unit: avg_dependencies,
            max_dependencies_per_unit: max_dependencies,
            has_cycles: self.has_cycles(),
        }
    }
}

/// Compilation plan with dependency-aware ordering
#[derive(Debug, Clone)]
pub struct CompilationPlan {
    /// Batches of units that can be compiled in parallel
    pub batches: Vec<Vec<String>>,
    /// Critical path (longest dependency chain)
    pub critical_path: Vec<String>,
    /// Estimated total compilation time
    pub estimated_total_time: std::time::Duration,
    /// Maximum parallelism possible
    pub max_parallelism: usize,
}

/// Statistics about the dependency graph
#[derive(Debug, Clone)]
pub struct DependencyGraphStats {
    pub total_units: usize,
    pub total_dependencies: usize,
    pub average_dependencies_per_unit: f64,
    pub max_dependencies_per_unit: usize,
    pub has_cycles: bool,
}

/// Dependency analyzer for compilation units
pub struct DependencyAnalyzer {
    cache: HashMap<String, DependencyGraph>,
}

impl DependencyAnalyzer {
    /// Create new dependency analyzer
    pub fn new() -> Result<Self> {
        Ok(Self {
            cache: HashMap::new(),
        })
    }
    
    /// Analyze dependencies for compilation units
    #[instrument(skip(self, units))]
    pub fn analyze_dependencies(&mut self, units: &[CompilationUnit]) -> Result<DependencyGraph> {
        info!("Analyzing dependencies for {} compilation units", units.len());
        
        let mut graph = DependencyGraph::new();
        
        // Build dependency graph from compilation units
        for unit in units {
            for dependency in &unit.dependencies {
                graph.add_dependency(&unit.name, dependency);
            }
        }
        
        // Validate the graph
        if graph.has_cycles() {
            return Err(Error::OptimizationError("Circular dependencies detected in compilation units".to_string()));
        }
        
        let stats = graph.get_statistics();
        debug!("Dependency analysis complete: {} units, {} dependencies, {:.1} avg deps/unit", 
               stats.total_units, stats.total_dependencies, stats.average_dependencies_per_unit);
        
        Ok(graph)
    }
    
    /// Get cached dependency graph
    pub fn get_cached_graph(&self, cache_key: &str) -> Option<&DependencyGraph> {
        self.cache.get(cache_key)
    }
    
    /// Cache dependency graph
    pub fn cache_graph(&mut self, cache_key: String, graph: DependencyGraph) {
        self.cache.insert(cache_key, graph);
    }
    
    /// Clear dependency cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DependencyAnalyzer {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dependency_graph_creation() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("main", "utils");
        graph.add_dependency("utils", "core");
        
        assert_eq!(graph.get_dependencies_for("main"), vec!["utils"]);
        assert_eq!(graph.get_dependencies_for("utils"), vec!["core"]);
        assert_eq!(graph.get_dependents_of("utils"), vec!["main"]);
    }
    
    #[test]
    fn test_topological_sort() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("main", "utils");
        graph.add_dependency("utils", "core");
        graph.add_dependency("main", "core");
        
        let units = vec!["main".to_string(), "utils".to_string(), "core".to_string()];
        let sorted = graph.topological_sort(&units).unwrap();
        
        // core should come before utils, utils before main
        let core_pos = sorted.iter().position(|x| x == "core").unwrap();
        let utils_pos = sorted.iter().position(|x| x == "utils").unwrap();
        let main_pos = sorted.iter().position(|x| x == "main").unwrap();
        
        assert!(core_pos < utils_pos);
        assert!(utils_pos < main_pos);
    }
    
    #[test]
    fn test_cycle_detection() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", "b");
        graph.add_dependency("b", "c");
        graph.add_dependency("c", "a"); // Creates a cycle
        
        assert!(graph.has_cycles());
    }
    
    #[test]
    fn test_compilation_plan() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("main", "utils");
        graph.add_dependency("utils", "core");
        
        let plan = graph.find_optimal_compilation_order(2).unwrap();
        
        assert!(!plan.batches.is_empty());
        assert!(plan.max_parallelism == 2);
    }
    
    #[test]
    fn test_dependency_analyzer() {
        let mut analyzer = DependencyAnalyzer::new().unwrap();
        
        let mut units = vec![
            CompilationUnit::new("main".to_string()),
            CompilationUnit::new("utils".to_string()),
        ];
        
        units[0].dependencies.push("utils".to_string());
        
        let graph = analyzer.analyze_dependencies(&units).unwrap();
        let stats = graph.get_statistics();
        
        assert_eq!(stats.total_units, 2);
        assert_eq!(stats.total_dependencies, 1);
        assert!(!stats.has_cycles);
    }
}
