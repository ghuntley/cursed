// Dependency analysis for optimization ordering
// 
// Provides analysis of dependencies between optimization passes and compilation units
// to ensure correct ordering and parallel execution opportunities.

use std::collections::{HashMap, HashSet, VecDeque};
use crate::error::Result;

/// Analyzes dependencies between compilation units and optimization passes
#[derive(Debug, Clone)]
pub struct DependencyAnalyzer {
    dependency_graph: HashMap<String, HashSet<String>>,
    compilation_units: Vec<CompilationUnit>,
}

/// Represents a compilation unit with its dependencies
#[derive(Debug, Clone)]
pub struct CompilationUnit {
    pub name: String,
    pub dependencies: Vec<String>,
    pub optimization_level: u8,
}

/// Results of dependency analysis
#[derive(Debug, Clone)]
pub struct DependencyAnalysisResult {
    pub execution_order: Vec<String>,
    pub parallel_groups: Vec<Vec<String>>,
    pub cycles_detected: Vec<Vec<String>>,
}

/// Dependency graph representation
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    pub nodes: HashMap<String, CompilationUnit>,
    pub edges: HashMap<String, Vec<String>>,
}

/// Compilation plan generated from dependency analysis
#[derive(Debug, Clone)]
pub struct CompilationPlan {
    pub phases: Vec<CompilationPhase>,
    pub total_estimated_time: Duration,
    pub parallelization_factor: f64,
}

/// Individual compilation phase
#[derive(Debug, Clone)]
pub struct CompilationPhase {
    pub name: String,
    pub units: Vec<String>,
    pub dependencies: Vec<String>,
    pub estimated_duration: Duration,
}

impl DependencyAnalyzer {
    /// Creates a new dependency analyzer
    pub fn new() -> Self {
        Self {
            dependency_graph: HashMap::new(),
            compilation_units: Vec::new(),
        }
    }

    /// Adds a compilation unit to the analysis
    pub fn add_compilation_unit(&mut self, unit: CompilationUnit) {
        self.compilation_units.push(unit);
    }

    /// Analyzes dependencies and returns execution plan
    pub fn analyze(&mut self) -> Result<DependencyAnalysisResult> {
        self.build_dependency_graph();
        let execution_order = self.topological_sort()?;
        let parallel_groups = self.identify_parallel_groups(&execution_order);
        let cycles_detected = self.detect_cycles();

        Ok(DependencyAnalysisResult {
            execution_order,
            parallel_groups,
            cycles_detected,
        })
    }

    fn build_dependency_graph(&mut self) {
        for unit in &self.compilation_units {
            self.dependency_graph.insert(unit.name.clone(), unit.dependencies.iter().cloned().collect());
        }
    }

    fn topological_sort(&self) -> Result<Vec<String>> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        // Calculate in-degrees
        for (node, _) in &self.dependency_graph {
            in_degree.insert(node.clone(), 0);
        }
        
        for (_, deps) in &self.dependency_graph {
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }

        // Add nodes with no dependencies to queue
        for (node, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node.clone());
            }
        }

        // Process queue
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());
            
            if let Some(deps) = self.dependency_graph.get(&node) {
                for dep in deps {
                    if let Some(degree) = in_degree.get_mut(dep) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dep.clone());
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    fn identify_parallel_groups(&self, execution_order: &[String]) -> Vec<Vec<String>> {
        // Simple grouping - units at the same dependency level can run in parallel
        let mut groups = Vec::new();
        let mut current_group = Vec::new();
        
        for unit_name in execution_order {
            if self.can_run_in_parallel(unit_name, &current_group) {
                current_group.push(unit_name.clone());
            } else {
                if !current_group.is_empty() {
                    groups.push(current_group);
                }
                current_group = vec![unit_name.clone()];
            }
        }
        
        if !current_group.is_empty() {
            groups.push(current_group);
        }
        
        groups
    }

    fn can_run_in_parallel(&self, unit_name: &str, current_group: &[String]) -> bool {
        if let Some(deps) = self.dependency_graph.get(unit_name) {
            // Check if any of the current group members are dependencies
            for group_member in current_group {
                if deps.contains(group_member) {
                    return false;
                }
            }
        }
        true
    }

    fn detect_cycles(&self) -> Vec<Vec<String>> {
        // Simple cycle detection using DFS
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node in self.dependency_graph.keys() {
            if !visited.contains(node) {
                self.dfs_cycle_detection(node, &mut visited, &mut rec_stack, &mut Vec::new(), &mut cycles);
            }
        }

        cycles
    }

    fn dfs_cycle_detection(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(deps) = self.dependency_graph.get(node) {
            for dep in deps {
                if !visited.contains(dep) {
                    self.dfs_cycle_detection(dep, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(dep) {
                    // Found cycle
                    if let Some(start_idx) = path.iter().position(|x| x == dep) {
                        cycles.push(path[start_idx..].to_vec());
                    }
                }
            }
        }

        path.pop();
        rec_stack.remove(node);
    }
}

impl Default for DependencyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_analysis() {
        let mut analyzer = DependencyAnalyzer::new();
        
        analyzer.add_compilation_unit(CompilationUnit {
            name: "unit_a".to_string(),
            dependencies: vec![],
            optimization_level: 1,
        });
        
        analyzer.add_compilation_unit(CompilationUnit {
            name: "unit_b".to_string(),
            dependencies: vec!["unit_a".to_string()],
            optimization_level: 1,
        });

        let result = analyzer.analyze().unwrap();
        assert!(!result.execution_order.is_empty());
        assert!(result.cycles_detected.is_empty());
    }
}
