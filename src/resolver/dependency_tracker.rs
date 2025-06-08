//! Dependency tracking and circular dependency detection

use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{debug, instrument};

/// Tracks package dependencies and detects circular dependencies
#[derive(Debug, Clone)]
pub struct DependencyTracker {
    /// Graph of package dependencies: package -> set of dependencies
    dependencies: HashMap<String, HashSet<String>>,
    /// Reverse dependency graph: package -> set of packages that depend on it
    dependents: HashMap<String, HashSet<String>>,
}

impl DependencyTracker {
    /// Create a new dependency tracker
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }

    /// Add a dependency relationship
    #[instrument(skip(self))]
    pub fn add_dependency(&mut self, package: &str, dependency: &str) {
        debug!("Adding dependency: {} -> {}", package, dependency);
        
        // Add to dependencies graph
        self.dependencies
            .entry(package.to_string())
            .or_insert_with(HashSet::new)
            .insert(dependency.to_string());
        
        // Add to reverse dependency graph
        self.dependents
            .entry(dependency.to_string())
            .or_insert_with(HashSet::new)
            .insert(package.to_string());
    }

    /// Remove a dependency relationship
    pub fn remove_dependency(&mut self, package: &str, dependency: &str) {
        if let Some(deps) = self.dependencies.get_mut(package) {
            deps.remove(dependency);
            if deps.is_empty() {
                self.dependencies.remove(package);
            }
        }
        
        if let Some(deps) = self.dependents.get_mut(dependency) {
            deps.remove(package);
            if deps.is_empty() {
                self.dependents.remove(dependency);
            }
        }
    }

    /// Get direct dependencies of a package
    pub fn get_dependencies(&self, package: &str) -> Vec<String> {
        self.dependencies
            .get(package)
            .map(|deps| deps.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get packages that depend on the given package
    pub fn get_dependents(&self, package: &str) -> Vec<String> {
        self.dependents
            .get(package)
            .map(|deps| deps.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get all transitive dependencies of a package
    #[instrument(skip(self))]
    pub fn get_transitive_dependencies(&self, package: &str) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        
        to_visit.push_back(package.to_string());
        
        while let Some(current) = to_visit.pop_front() {
            if visited.contains(&current) {
                continue;
            }
            
            visited.insert(current.clone());
            
            if let Some(deps) = self.dependencies.get(&current) {
                for dep in deps {
                    if !visited.contains(dep) {
                        to_visit.push_back(dep.clone());
                    }
                }
            }
        }
        
        // Remove the original package from the result
        visited.remove(package);
        visited
    }

    /// Detect circular dependencies using DFS
    #[instrument(skip(self))]
    pub fn detect_cycles(&self) -> Option<Vec<String>> {
        let mut white = HashSet::new(); // Unvisited
        let mut gray = HashSet::new();  // Currently being processed
        let mut black = HashSet::new(); // Completely processed
        
        // Initialize all packages as white
        for package in self.dependencies.keys() {
            white.insert(package.clone());
        }
        
        // DFS from each unvisited package
        while let Some(package) = white.iter().next().cloned() {
            white.remove(&package);
            
            if let Some(cycle) = self.dfs_detect_cycle(&package, &mut white, &mut gray, &mut black) {
                return Some(cycle);
            }
        }
        
        None
    }

    /// DFS helper for cycle detection
    fn dfs_detect_cycle(
        &self,
        package: &str,
        white: &mut HashSet<String>,
        gray: &mut HashSet<String>,
        black: &mut HashSet<String>,
    ) -> Option<Vec<String>> {
        // Move package from white to gray
        white.remove(package);
        gray.insert(package.to_string());
        
        // Visit all dependencies
        if let Some(deps) = self.dependencies.get(package) {
            for dep in deps {
                if gray.contains(dep) {
                    // Found a back edge - cycle detected
                    return Some(vec![package.to_string(), dep.clone()]);
                }
                
                if white.contains(dep) {
                    if let Some(mut cycle) = self.dfs_detect_cycle(dep, white, gray, black) {
                        // Add current package to the cycle path
                        cycle.insert(0, package.to_string());
                        return Some(cycle);
                    }
                }
            }
        }
        
        // Move package from gray to black
        gray.remove(package);
        black.insert(package.to_string());
        
        None
    }

    /// Check if there's a dependency path from one package to another
    pub fn has_dependency_path(&self, from: &str, to: &str) -> bool {
        if from == to {
            return true;
        }
        
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        
        to_visit.push_back(from.to_string());
        
        while let Some(current) = to_visit.pop_front() {
            if visited.contains(&current) {
                continue;
            }
            
            visited.insert(current.clone());
            
            if let Some(deps) = self.dependencies.get(&current) {
                for dep in deps {
                    if dep == to {
                        return true;
                    }
                    
                    if !visited.contains(dep) {
                        to_visit.push_back(dep.clone());
                    }
                }
            }
        }
        
        false
    }

    /// Get all packages in the dependency graph
    pub fn all_packages(&self) -> HashSet<String> {
        let mut packages = HashSet::new();
        
        for package in self.dependencies.keys() {
            packages.insert(package.clone());
        }
        
        for package in self.dependents.keys() {
            packages.insert(package.clone());
        }
        
        packages
    }

    /// Get packages with no dependencies (leaf packages)
    pub fn leaf_packages(&self) -> Vec<String> {
        self.all_packages()
            .into_iter()
            .filter(|package| {
                self.dependencies
                    .get(package)
                    .map(|deps| deps.is_empty())
                    .unwrap_or(true)
            })
            .collect()
    }

    /// Get packages with no dependents (root packages)
    pub fn root_packages(&self) -> Vec<String> {
        self.all_packages()
            .into_iter()
            .filter(|package| {
                self.dependents
                    .get(package)
                    .map(|deps| deps.is_empty())
                    .unwrap_or(true)
            })
            .collect()
    }

    /// Clear all dependency information
    pub fn clear(&mut self) {
        self.dependencies.clear();
        self.dependents.clear();
    }

    /// Get a topological sort of all packages
    /// Returns None if there are cycles
    #[instrument(skip(self))]
    pub fn topological_sort(&self) -> Option<Vec<String>> {
        // Check for cycles first
        if self.detect_cycles().is_some() {
            return None;
        }
        
        let mut result = Vec::new();
        let mut in_degree = HashMap::new();
        let mut queue = VecDeque::new();
        
        // Calculate in-degrees
        for package in self.all_packages() {
            let degree = self.dependents
                .get(&package)
                .map(|deps| deps.len())
                .unwrap_or(0);
            in_degree.insert(package.clone(), degree);
            
            if degree == 0 {
                queue.push_back(package);
            }
        }
        
        // Process packages with no dependencies first
        while let Some(package) = queue.pop_front() {
            result.push(package.clone());
            
            // Reduce in-degree of dependent packages
            if let Some(deps) = self.dependencies.get(&package) {
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
        
        // Check if all packages were processed
        if result.len() == self.all_packages().len() {
            Some(result)
        } else {
            None // Cycle detected
        }
    }

    /// Get dependency statistics
    pub fn stats(&self) -> DependencyStats {
        let packages = self.all_packages();
        let total_packages = packages.len();
        let total_dependencies = self.dependencies
            .values()
            .map(|deps| deps.len())
            .sum();
        
        let max_dependencies = self.dependencies
            .values()
            .map(|deps| deps.len())
            .max()
            .unwrap_or(0);
        
        let packages_with_no_deps = self.leaf_packages().len();
        let packages_with_no_dependents = self.root_packages().len();
        
        DependencyStats {
            total_packages,
            total_dependencies,
            max_dependencies,
            packages_with_no_deps,
            packages_with_no_dependents,
            has_cycles: self.detect_cycles().is_some(),
        }
    }

    /// Start loading a package (for tracking)
    pub fn start_loading(&mut self, package_name: &str) -> Result<(), super::errors::ResolverError> {
        debug!("Starting to load package: {}", package_name);
        Ok(())
    }

    /// Finish loading a package (for tracking)
    pub fn finish_loading(&mut self, package_name: &str) {
        debug!("Finished loading package: {}", package_name);
    }

    /// Check if package A depends on package B
    pub fn depends_on(&self, package_a: &str, package_b: &str) -> bool {
        if let Some(deps) = self.dependencies.get(package_a) {
            deps.contains(package_b)
        } else {
            false
        }
    }
}

impl Default for DependencyTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the dependency graph
#[derive(Debug, Clone)]
pub struct DependencyStats {
    /// Total number of packages
    pub total_packages: usize,
    /// Total number of dependency relationships
    pub total_dependencies: usize,
    /// Maximum number of dependencies for any single package
    pub max_dependencies: usize,
    /// Number of packages with no dependencies
    pub packages_with_no_deps: usize,
    /// Number of packages with no dependents
    pub packages_with_no_dependents: usize,
    /// Whether the dependency graph has cycles
    pub has_cycles: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_dependency_tracking() {
        let mut tracker = DependencyTracker::new();
        
        tracker.add_dependency("app", "utils");
        tracker.add_dependency("app", "core");
        tracker.add_dependency("utils", "core");
        
        assert_eq!(tracker.get_dependencies("app"), vec!["utils", "core"]);
        assert_eq!(tracker.get_dependencies("utils"), vec!["core"]);
        assert_eq!(tracker.get_dependencies("core"), Vec::<String>::new());
        
        assert_eq!(tracker.get_dependents("core"), vec!["app", "utils"]);
        assert_eq!(tracker.get_dependents("utils"), vec!["app"]);
        assert_eq!(tracker.get_dependents("app"), Vec::<String>::new());
    }

    #[test]
    fn test_cycle_detection() {
        let mut tracker = DependencyTracker::new();
        
        // No cycle
        tracker.add_dependency("a", "b");
        tracker.add_dependency("b", "c");
        assert!(tracker.detect_cycles().is_none());
        
        // Add cycle
        tracker.add_dependency("c", "a");
        assert!(tracker.detect_cycles().is_some());
    }

    #[test]
    fn test_transitive_dependencies() {
        let mut tracker = DependencyTracker::new();
        
        tracker.add_dependency("app", "utils");
        tracker.add_dependency("utils", "core");
        tracker.add_dependency("core", "base");
        
        let transitive = tracker.get_transitive_dependencies("app");
        assert_eq!(transitive.len(), 3);
        assert!(transitive.contains("utils"));
        assert!(transitive.contains("core"));
        assert!(transitive.contains("base"));
    }

    #[test]
    fn test_topological_sort() {
        let mut tracker = DependencyTracker::new();
        
        tracker.add_dependency("app", "utils");
        tracker.add_dependency("app", "core");
        tracker.add_dependency("utils", "core");
        
        let sorted = tracker.topological_sort().unwrap();
        
        // core should come before utils and app
        // utils should come before app
        let core_pos = sorted.iter().position(|p| p == "core").unwrap();
        let utils_pos = sorted.iter().position(|p| p == "utils").unwrap();
        let app_pos = sorted.iter().position(|p| p == "app").unwrap();
        
        assert!(core_pos < utils_pos);
        assert!(core_pos < app_pos);
        assert!(utils_pos < app_pos);
    }
}
