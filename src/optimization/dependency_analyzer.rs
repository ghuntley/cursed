//! Dependency analysis for compilation units

use crate::error::{Result, CursedError};
use crate::optimization::metrics::CompilationUnit;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Duration;
use tracing::{debug, info, warn, instrument};
use serde::{Deserialize, Serialize};

/// Dependency graph for compilation units
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    dependencies: HashMap<String, Vec<String>>,
    reverse_dependencies: HashMap<String, Vec<String>>,
    units: HashMap<String, CompilationUnit>,
}

/// Compilation plan with optimal ordering
#[derive(Debug, Clone)]
pub struct CompilationPlan {
    pub batches: Vec<Vec<String>>,
    pub critical_path: Vec<String>,
    pub estimated_total_time: Duration,
    pub max_parallelism: usize,
}

/// Dependency cycle detection result
#[derive(Debug, Clone)]
pub struct CycleDetectionResult {
    pub has_cycles: bool,
    pub cycles: Vec<Vec<String>>,
    pub affected_units: HashSet<String>,
}

/// Dependency analyzer for build optimization
#[derive(Debug)]
pub struct DependencyAnalyzer {
    dependency_cache: HashMap<String, Vec<String>>,
    analysis_cache: HashMap<String, DependencyGraph>,
}

impl DependencyAnalyzer {
    /// Create a new dependency analyzer
    #[instrument]
    pub fn new() -> Result<Self> {
        info!("Creating dependency analyzer");
        
        Ok(Self {
            dependency_cache: HashMap::new(),
            analysis_cache: HashMap::new(),
        })
    }

    /// Analyze dependencies for a set of compilation units
    #[instrument(skip(self, units))]
    pub fn analyze_dependencies(&mut self, units: &[CompilationUnit]) -> Result<DependencyGraph> {
        debug!("Analyzing dependencies for {} units", units.len());
        
        let mut dependencies = HashMap::new();
        let mut reverse_dependencies = HashMap::new();
        let mut units_map = HashMap::new();
        
        // First pass: collect all units and initialize maps
        for unit in units {
            units_map.insert(unit.name.clone(), unit.clone());
            dependencies.insert(unit.name.clone(), Vec::new());
            reverse_dependencies.insert(unit.name.clone(), Vec::new());
        }
        
        // Second pass: analyze dependencies
        for unit in units {
            let unit_deps = self.analyze_unit_dependencies(unit)?;
            
            for dep in &unit_deps {
                // Only include dependencies that are part of our compilation set
                if units_map.contains_key(dep) {
                    dependencies.get_mut(&unit.name).unwrap().push(dep.clone());
                    reverse_dependencies.get_mut(dep).unwrap().push(unit.name.clone());
                }
            }
            
            // Cache the dependencies
            self.dependency_cache.insert(unit.name.clone(), unit_deps);
        }
        
        let graph = DependencyGraph {
            dependencies,
            reverse_dependencies,
            units: units_map,
        };
        
        // Validate the dependency graph
        self.validate_dependency_graph(&graph)?;
        
        info!("Dependency analysis completed for {} units", units.len());
        Ok(graph)
    }

    /// Analyze dependencies for a single compilation unit
    #[instrument(skip(self, unit))]
    fn analyze_unit_dependencies(&self, unit: &CompilationUnit) -> Result<Vec<String>> {
        // Check cache first
        if let Some(cached_deps) = self.dependency_cache.get(&unit.name) {
            return Ok(cached_deps.clone());
        }
        
        let mut dependencies = HashSet::new();
        
        // Add explicit dependencies from unit
        for dep in &unit.dependencies {
            dependencies.insert(dep.clone());
        }
        
        // Analyze source files for import dependencies
        for source_file in &unit.source_files {
            let file_deps = self.analyze_source_file_dependencies(source_file)?;
            dependencies.extend(file_deps);
        }
        
        Ok(dependencies.into_iter().collect())
    }
    
    /// Analyze a source file for import dependencies
    fn analyze_source_file_dependencies(&self, file_path: &str) -> Result<Vec<String>> {
        let mut dependencies = Vec::new();
        
        // Read the source file
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to read source file {}: {}", file_path, e))
        })?;
        
        // Simple import analysis - look for import statements
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("import") || trimmed.starts_with("use") {
                if let Some(module_name) = self.extract_module_name(trimmed) {
                    dependencies.push(module_name);
                }
            }
        }
        
        Ok(dependencies)
    }
    
    /// Extract module name from import statement
    fn extract_module_name(&self, import_line: &str) -> Option<String> {
        // Simple regex-like parsing for import statements
        // import "module::path" or use module::path
        
        if let Some(start) = import_line.find('"') {
            if let Some(end) = import_line.rfind('"') {
                if start < end {
                    let module_path = &import_line[start + 1..end];
                    // Convert module path to unit name
                    return Some(module_path.replace("::", "_").replace("/", "_"));
                }
            }
        }
        
        // Handle use statements without quotes
        if import_line.starts_with("use") {
            let parts: Vec<&str> = import_line.split_whitespace().collect();
            if parts.len() >= 2 {
                let module_path = parts[1].trim_end_matches(';');
                return Some(module_path.replace("::", "_"));
            }
        }
        
        None
    }
    
    /// Validate dependency graph for cycles and other issues
    fn validate_dependency_graph(&self, graph: &DependencyGraph) -> Result<()> {
        debug!("Validating dependency graph");
        
        // Check for cycles
        let cycle_result = graph.detect_cycles()?;
        if cycle_result.has_cycles {
            warn!("Dependency cycles detected: {:?}", cycle_result.cycles);
            // For now, just warn - in production might want to error
        }
        
        // Check for missing dependencies
        for (unit_name, deps) in &graph.dependencies {
            for dep in deps {
                if !graph.units.contains_key(dep) {
                    warn!("Unit {} has dependency on missing unit: {}", unit_name, dep);
                }
            }
        }
        
        Ok(())
    }
}

impl DependencyGraph {
    /// Get dependencies map
    pub fn get_dependencies(&self) -> &HashMap<String, Vec<String>> {
        &self.dependencies
    }
    
    /// Get reverse dependencies map
    pub fn get_reverse_dependencies(&self) -> &HashMap<String, Vec<String>> {
        &self.reverse_dependencies
    }
    
    /// Get units map
    pub fn get_units(&self) -> &HashMap<String, CompilationUnit> {
        &self.units
    }
    
    /// Perform topological sort on specified units
    pub fn topological_sort(&self, units_to_sort: &[String]) -> Result<Vec<String>> {
        let mut in_degree = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        // Initialize in-degree count for units to sort
        for unit in units_to_sort {
            let deps = self.dependencies.get(unit).map(|v| v.len()).unwrap_or(0);
            // Only count dependencies that are also in units_to_sort
            let filtered_deps = self.dependencies.get(unit)
                .map(|deps| deps.iter().filter(|dep| units_to_sort.contains(dep)).count())
                .unwrap_or(0);
            in_degree.insert(unit.clone(), filtered_deps);
            
            if filtered_deps == 0 {
                queue.push_back(unit.clone());
            }
        }
        
        // Process queue
        while let Some(current) = queue.pop_front() {
            result.push(current.clone());
            
            // Reduce in-degree for dependents
            if let Some(dependents) = self.reverse_dependencies.get(&current) {
                for dependent in dependents {
                    if units_to_sort.contains(dependent) {
                        if let Some(degree) = in_degree.get_mut(dependent) {
                            *degree -= 1;
                            if *degree == 0 {
                                queue.push_back(dependent.clone());
                            }
                        }
                    }
                }
            }
        }
        
        // Check if all units were processed
        if result.len() != units_to_sort.len() {
            return Err(CursedError::optimization_error(
                "Circular dependency detected in compilation units"
            ));
        }
        
        Ok(result)
    }
    
    /// Detect cycles in the dependency graph
    pub fn detect_cycles(&self) -> Result<CycleDetectionResult> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut cycles = Vec::new();
        
        for unit in self.units.keys() {
            if !visited.contains(unit) {
                if let Some(cycle) = self.dfs_detect_cycle(unit, &mut visited, &mut rec_stack)? {
                    cycles.push(cycle);
                }
            }
        }
        
        let has_cycles = !cycles.is_empty();
        let affected_units = cycles.iter().flatten().cloned().collect();
        
        Ok(CycleDetectionResult {
            has_cycles,
            cycles,
            affected_units,
        })
    }
    
    /// DFS-based cycle detection
    fn dfs_detect_cycle(
        &self,
        unit: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> Result<Option<Vec<String>>> {
        visited.insert(unit.to_string());
        rec_stack.insert(unit.to_string());
        
        if let Some(dependencies) = self.dependencies.get(unit) {
            for dep in dependencies {
                if !visited.contains(dep) {
                    if let Some(cycle) = self.dfs_detect_cycle(dep, visited, rec_stack)? {
                        return Ok(Some(cycle));
                    }
                } else if rec_stack.contains(dep) {
                    // Found a cycle
                    let mut cycle = vec![dep.clone(), unit.to_string()];
                    return Ok(Some(cycle));
                }
            }
        }
        
        rec_stack.remove(unit);
        Ok(None)
    }
    
    /// Find optimal compilation order for parallel execution
    pub fn find_optimal_compilation_order(&self, max_parallelism: usize) -> Result<CompilationPlan> {
        debug!("Finding optimal compilation order with max parallelism: {}", max_parallelism);
        
        let mut batches = Vec::new();
        let mut processed = HashSet::new();
        let mut remaining_units: HashSet<String> = self.units.keys().cloned().collect();
        
        // Calculate estimated times for each unit
        let unit_times: HashMap<String, Duration> = self.units.iter()
            .map(|(name, unit)| {
                let estimated_ms = 100 + (unit.source_files.len() * 50) + (unit.estimated_size_bytes / 1000);
                (name.clone(), Duration::from_millis(estimated_ms as u64))
            })
            .collect();
        
        let mut total_estimated_time = Duration::from_secs(0);
        
        // Build batches level by level
        while !remaining_units.is_empty() {
            let mut current_batch = Vec::new();
            
            // Find units with no unprocessed dependencies
            for unit in &remaining_units {
                let unit_deps = self.dependencies.get(unit).unwrap_or(&Vec::new());
                let unprocessed_deps: Vec<_> = unit_deps.iter()
                    .filter(|dep| !processed.contains(*dep))
                    .collect();
                
                if unprocessed_deps.is_empty() {
                    current_batch.push(unit.clone());
                }
            }
            
            if current_batch.is_empty() {
                // This indicates a cycle - try to break it
                warn!("Detected dependency cycle, selecting arbitrary unit to break deadlock");
                if let Some(unit) = remaining_units.iter().next() {
                    current_batch.push(unit.clone());
                }
            }
            
            // Limit batch size to max parallelism
            current_batch.truncate(max_parallelism);
            
            // Sort by estimated compilation time (longest first for better load balancing)
            current_batch.sort_by(|a, b| {
                let time_a = unit_times.get(a).unwrap_or(&Duration::from_secs(0));
                let time_b = unit_times.get(b).unwrap_or(&Duration::from_secs(0));
                time_b.cmp(time_a)
            });
            
            // Calculate batch time (longest unit in parallel batch)
            let batch_time = current_batch.iter()
                .map(|unit| unit_times.get(unit).unwrap_or(&Duration::from_secs(0)))
                .max()
                .unwrap_or(&Duration::from_secs(0));
            total_estimated_time += *batch_time;
            
            // Update processed and remaining sets
            for unit in &current_batch {
                processed.insert(unit.clone());
                remaining_units.remove(unit);
            }
            
            batches.push(current_batch);
        }
        
        // Find critical path
        let critical_path = self.find_critical_path(&unit_times)?;
        
        info!("Generated compilation plan with {} batches, estimated time: {:?}", 
               batches.len(), total_estimated_time);
        
        Ok(CompilationPlan {
            batches,
            critical_path,
            estimated_total_time: total_estimated_time,
            max_parallelism,
        })
    }
    
    /// Find the critical path (longest dependency chain)
    fn find_critical_path(&self, unit_times: &HashMap<String, Duration>) -> Result<Vec<String>> {
        let mut longest_path = Vec::new();
        let mut longest_time = Duration::from_secs(0);
        
        // Try each unit as a starting point
        for unit in self.units.keys() {
            let (path, time) = self.find_longest_path_from_unit(unit, unit_times)?;
            if time > longest_time {
                longest_time = time;
                longest_path = path;
            }
        }
        
        debug!("Critical path found with {} units and time {:?}", longest_path.len(), longest_time);
        Ok(longest_path)
    }
    
    /// Find longest path from a specific unit
    fn find_longest_path_from_unit(
        &self, 
        start_unit: &str, 
        unit_times: &HashMap<String, Duration>
    ) -> Result<(Vec<String>, Duration)> {
        let mut visited = HashSet::new();
        self.dfs_longest_path(start_unit, unit_times, &mut visited)
    }
    
    /// DFS to find longest path
    fn dfs_longest_path(
        &self,
        unit: &str,
        unit_times: &HashMap<String, Duration>,
        visited: &mut HashSet<String>,
    ) -> Result<(Vec<String>, Duration)> {
        if visited.contains(unit) {
            return Ok((Vec::new(), Duration::from_secs(0)));
        }
        
        visited.insert(unit.to_string());
        
        let unit_time = *unit_times.get(unit).unwrap_or(&Duration::from_secs(0));
        let mut longest_path = vec![unit.to_string()];
        let mut longest_time = unit_time;
        
        // Check all dependencies
        if let Some(dependencies) = self.dependencies.get(unit) {
            for dep in dependencies {
                let (dep_path, dep_time) = self.dfs_longest_path(dep, unit_times, visited)?;
                let total_time = unit_time + dep_time;
                
                if total_time > longest_time {
                    longest_time = total_time;
                    longest_path = vec![unit.to_string()];
                    longest_path.extend(dep_path);
                }
            }
        }
        
        visited.remove(unit);
        Ok((longest_path, longest_time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_analyzer_creation() {
        let analyzer = DependencyAnalyzer::new();
        assert!(analyzer.is_ok());
    }

    #[test]
    fn test_simple_dependency_graph() {
        let mut analyzer = DependencyAnalyzer::new().unwrap();
        
        let mut unit1 = CompilationUnit::new("unit1".to_string());
        let mut unit2 = CompilationUnit::new("unit2".to_string());
        unit2.add_dependency("unit1".to_string());
        
        let units = vec![unit1, unit2];
        let graph = analyzer.analyze_dependencies(&units).unwrap();
        
        assert_eq!(graph.dependencies.get("unit1").unwrap().len(), 0);
        assert_eq!(graph.dependencies.get("unit2").unwrap().len(), 1);
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = DependencyGraph {
            dependencies: HashMap::new(),
            reverse_dependencies: HashMap::new(),
            units: HashMap::new(),
        };
        
        graph.dependencies.insert("a".to_string(), vec!["b".to_string()]);
        graph.dependencies.insert("b".to_string(), vec![]);
        graph.dependencies.insert("c".to_string(), vec!["a".to_string()]);
        
        let units = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let sorted = graph.topological_sort(&units).unwrap();
        
        // b should come before a, a should come before c
        let b_pos = sorted.iter().position(|x| x == "b").unwrap();
        let a_pos = sorted.iter().position(|x| x == "a").unwrap();
        let c_pos = sorted.iter().position(|x| x == "c").unwrap();
        
        assert!(b_pos < a_pos);
        assert!(a_pos < c_pos);
    }
}
