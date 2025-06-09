//! Dependency Resolution System
//! 
//! Resolves package dependencies, handles version constraints,
//! and builds dependency graphs for CURSED projects.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{debug, info, instrument, warn};

/// Dependency resolver
#[derive(Debug)]
pub struct DependencyResolver {
    /// Resolved dependency graph
    graph: DependencyGraph,
    
    /// Version constraint resolver
    constraint_resolver: VersionConstraintResolver,
    
    /// Package registry interface
    registry: MockPackageRegistry, // TODO: Replace with real registry
}

/// Dependency graph representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    /// Map of package name to dependency node
    pub nodes: HashMap<String, DependencyNode>,
    
    /// Edges representing dependencies
    pub edges: Vec<DependencyEdge>,
    
    /// Resolved versions for each package
    pub resolved_versions: HashMap<String, String>,
}

/// Node in the dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    /// Package name
    pub name: String,
    
    /// Resolved version
    pub version: String,
    
    /// Package metadata
    pub metadata: PackageMetadata,
    
    /// Direct dependencies
    pub dependencies: Vec<String>,
    
    /// Development dependencies
    pub dev_dependencies: Vec<String>,
    
    /// Build dependencies
    pub build_dependencies: Vec<String>,
}

/// Edge in the dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    /// Source package
    pub from: String,
    
    /// Target package
    pub to: String,
    
    /// Dependency type
    pub dependency_type: DependencyType,
    
    /// Version constraint
    pub constraint: String,
}

/// Types of dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Runtime,
    Development,
    Build,
}

/// Package metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    /// Package description
    pub description: Option<String>,
    
    /// License
    pub license: Option<String>,
    
    /// Repository URL
    pub repository: Option<String>,
    
    /// Keywords
    pub keywords: Vec<String>,
    
    /// Categories
    pub categories: Vec<String>,
}

/// Version constraint resolver
#[derive(Debug)]
pub struct VersionConstraintResolver {
    /// Available package versions
    available_versions: HashMap<String, Vec<String>>,
}

/// Mock package registry for testing
#[derive(Debug)]
pub struct MockPackageRegistry {
    packages: HashMap<String, Vec<PackageVersion>>,
}

/// Package version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersion {
    pub version: String,
    pub metadata: PackageMetadata,
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
    pub build_dependencies: HashMap<String, String>,
}

/// Dependency resolution error types
#[derive(Debug, thiserror::Error)]
pub enum DependencyError {
    #[error("Package not found: {package}")]
    PackageNotFound { package: String },
    
    #[error("Version constraint conflict: {package} requires {constraint1} and {constraint2}")]
    VersionConflict {
        package: String,
        constraint1: String,
        constraint2: String,
    },
    
    #[error("Circular dependency detected: {cycle}")]
    CircularDependency { cycle: String },
    
    #[error("Invalid version constraint: {constraint}")]
    InvalidConstraint { constraint: String },
    
    #[error("No compatible version found for {package} with constraint {constraint}")]
    NoCompatibleVersion { package: String, constraint: String },
}

impl DependencyResolver {
    /// Create a new dependency resolver
    pub fn new() -> Self {
        DependencyResolver {
            graph: DependencyGraph {
                nodes: HashMap::new(),
                edges: Vec::new(),
                resolved_versions: HashMap::new(),
            },
            constraint_resolver: VersionConstraintResolver::new(),
            registry: MockPackageRegistry::new(),
        }
    }
    
    /// Resolve dependencies for a project
    #[instrument(skip(self, dependencies))]
    pub fn resolve(&mut self, dependencies: &HashMap<String, String>) -> Result<DependencyGraph, DependencyError> {
        info!("Resolving {} dependencies", dependencies.len());
        
        // Clear previous resolution
        self.graph = DependencyGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
            resolved_versions: HashMap::new(),
        };
        
        // Build dependency graph
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        // Add root dependencies to queue
        for (package, constraint) in dependencies {
            queue.push_back((package.clone(), constraint.clone(), None));
        }
        
        // Process dependency queue
        while let Some((package, constraint, parent)) = queue.pop_front() {
            if visited.contains(&package) {
                continue;
            }
            
            visited.insert(package.clone());
            
            // Resolve version for this package
            let resolved_version = self.resolve_version(&package, &constraint)?;
            self.graph.resolved_versions.insert(package.clone(), resolved_version.clone());
            
            // Get package information
            let package_info = self.registry.get_package(&package, &resolved_version)?;
            
            // Create dependency node
            let node = DependencyNode {
                name: package.clone(),
                version: resolved_version.clone(),
                metadata: package_info.metadata.clone(),
                dependencies: package_info.dependencies.keys().cloned().collect(),
                dev_dependencies: package_info.dev_dependencies.keys().cloned().collect(),
                build_dependencies: package_info.build_dependencies.keys().cloned().collect(),
            };
            
            self.graph.nodes.insert(package.clone(), node);
            
            // Add edge if this is not a root dependency
            if let Some(parent_package) = parent {
                self.graph.edges.push(DependencyEdge {
                    from: parent_package,
                    to: package.clone(),
                    dependency_type: DependencyType::Runtime,
                    constraint: constraint.clone(),
                });
            }
            
            // Add transitive dependencies to queue
            for (dep_package, dep_constraint) in &package_info.dependencies {
                if !visited.contains(dep_package) {
                    queue.push_back((dep_package.clone(), dep_constraint.clone(), Some(package.clone())));
                }
            }
        }
        
        // Check for circular dependencies
        self.check_circular_dependencies()?;
        
        // Validate version constraints
        self.validate_constraints()?;
        
        info!("Successfully resolved {} packages", self.graph.nodes.len());
        Ok(self.graph.clone())
    }
    
    /// Resolve version for a package given constraints
    fn resolve_version(&self, package: &str, constraint: &str) -> Result<String, DependencyError> {
        debug!("Resolving version for {}: {}", package, constraint);
        
        let available_versions = self.registry.get_versions(package)?;
        
        let compatible_version = self.constraint_resolver
            .find_compatible_version(&available_versions, constraint)?;
        
        debug!("Resolved {} to version {}", package, compatible_version);
        Ok(compatible_version)
    }
    
    /// Check for circular dependencies
    fn check_circular_dependencies(&self) -> Result<(), DependencyError> {
        debug!("Checking for circular dependencies");
        
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for package in self.graph.nodes.keys() {
            if !visited.contains(package) {
                if let Some(cycle) = self.detect_cycle_dfs(package, &mut visited, &mut rec_stack) {
                    return Err(DependencyError::CircularDependency { cycle });
                }
            }
        }
        
        Ok(())
    }
    
    /// DFS to detect cycles
    fn detect_cycle_dfs(
        &self,
        package: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> Option<String> {
        visited.insert(package.to_string());
        rec_stack.insert(package.to_string());
        
        if let Some(node) = self.graph.nodes.get(package) {
            for dependency in &node.dependencies {
                if !visited.contains(dependency) {
                    if let Some(cycle) = self.detect_cycle_dfs(dependency, visited, rec_stack) {
                        return Some(format!("{} -> {}", package, cycle));
                    }
                } else if rec_stack.contains(dependency) {
                    return Some(format!("{} -> {}", package, dependency));
                }
            }
        }
        
        rec_stack.remove(package);
        None
    }
    
    /// Validate all version constraints are satisfied
    fn validate_constraints(&self) -> Result<(), DependencyError> {
        debug!("Validating version constraints");
        
        for edge in &self.graph.edges {
            let resolved_version = self.graph.resolved_versions.get(&edge.to)
                .ok_or_else(|| DependencyError::PackageNotFound { package: edge.to.clone() })?;
            
            if !self.constraint_resolver.satisfies_constraint(resolved_version, &edge.constraint)? {
                return Err(DependencyError::VersionConflict {
                    package: edge.to.clone(),
                    constraint1: edge.constraint.clone(),
                    constraint2: format!("resolved to {}", resolved_version),
                });
            }
        }
        
        Ok(())
    }
    
    /// Get dependency graph in topological order
    pub fn topological_sort(&self) -> Result<Vec<String>, DependencyError> {
        let mut in_degree = HashMap::new();
        let mut adj_list = HashMap::new();
        
        // Initialize in-degree and adjacency list
        for package in self.graph.nodes.keys() {
            in_degree.insert(package.clone(), 0);
            adj_list.insert(package.clone(), Vec::new());
        }
        
        // Build adjacency list and calculate in-degrees
        for edge in &self.graph.edges {
            adj_list.get_mut(&edge.from).unwrap().push(edge.to.clone());
            *in_degree.get_mut(&edge.to).unwrap() += 1;
        }
        
        // Kahn's algorithm for topological sorting
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        // Add nodes with in-degree 0 to queue
        for (package, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(package.clone());
            }
        }
        
        while let Some(package) = queue.pop_front() {
            result.push(package.clone());
            
            // Reduce in-degree of adjacent nodes
            if let Some(neighbors) = adj_list.get(&package) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        
        // Check if all nodes were processed (no cycles)
        if result.len() != self.graph.nodes.len() {
            return Err(DependencyError::CircularDependency {
                cycle: "Unable to determine exact cycle".to_string(),
            });
        }
        
        Ok(result)
    }
}

impl VersionConstraintResolver {
    /// Create a new version constraint resolver
    pub fn new() -> Self {
        VersionConstraintResolver {
            available_versions: HashMap::new(),
        }
    }
    
    /// Find compatible version given constraint
    pub fn find_compatible_version(
        &self,
        available_versions: &[String],
        constraint: &str,
    ) -> Result<String, DependencyError> {
        // Simple constraint matching - support ^, ~, and exact versions
        let compatible_versions: Vec<_> = available_versions
            .iter()
            .filter(|version| self.satisfies_constraint(version, constraint).unwrap_or(false))
            .collect();
        
        if compatible_versions.is_empty() {
            return Err(DependencyError::NoCompatibleVersion {
                package: "unknown".to_string(),
                constraint: constraint.to_string(),
            });
        }
        
        // Return the highest compatible version
        let mut sorted_versions = compatible_versions.clone();
        sorted_versions.sort_by(|a, b| version_compare(b, a));
        
        Ok(sorted_versions[0].clone())
    }
    
    /// Check if version satisfies constraint
    pub fn satisfies_constraint(&self, version: &str, constraint: &str) -> Result<bool, DependencyError> {
        if constraint.starts_with('^') {
            // Caret constraint: ^1.2.3 allows >=1.2.3 <2.0.0
            let constraint_version = &constraint[1..];
            self.satisfies_caret_constraint(version, constraint_version)
        } else if constraint.starts_with('~') {
            // Tilde constraint: ~1.2.3 allows >=1.2.3 <1.3.0
            let constraint_version = &constraint[1..];
            self.satisfies_tilde_constraint(version, constraint_version)
        } else {
            // Exact version constraint
            Ok(version == constraint)
        }
    }
    
    fn satisfies_caret_constraint(&self, version: &str, constraint: &str) -> Result<bool, DependencyError> {
        let version_parts = parse_version(version)?;
        let constraint_parts = parse_version(constraint)?;
        
        // Major version must match
        if version_parts[0] != constraint_parts[0] {
            return Ok(false);
        }
        
        // Version must be >= constraint
        Ok(matches!(version_compare(version, constraint), std::cmp::Ordering::Greater | std::cmp::Ordering::Equal))
    }
    
    fn satisfies_tilde_constraint(&self, version: &str, constraint: &str) -> Result<bool, DependencyError> {
        let version_parts = parse_version(version)?;
        let constraint_parts = parse_version(constraint)?;
        
        // Major and minor versions must match
        if version_parts[0] != constraint_parts[0] || version_parts[1] != constraint_parts[1] {
            return Ok(false);
        }
        
        // Patch version must be >= constraint
        Ok(version_parts[2] >= constraint_parts[2])
    }
}

impl MockPackageRegistry {
    /// Create a new mock registry with sample packages
    pub fn new() -> Self {
        let mut registry = MockPackageRegistry {
            packages: HashMap::new(),
        };
        
        // Add some sample packages
        registry.add_package("cursed-http", vec![
            PackageVersion {
                version: "1.0.0".to_string(),
                metadata: PackageMetadata {
                    description: Some("HTTP client for CURSED".to_string()),
                    license: Some("MIT".to_string()),
                    repository: Some("https://github.com/cursed-lang/http".to_string()),
                    keywords: Vec::from(["http".to_string(), "client".to_string()]),
                    categories: Vec::from(["network".to_string()]),
                },
                dependencies: HashMap::new(),
                dev_dependencies: HashMap::new(),
                build_dependencies: HashMap::new(),
            },
            PackageVersion {
                version: "1.1.0".to_string(),
                metadata: PackageMetadata {
                    description: Some("HTTP client for CURSED".to_string()),
                    license: Some("MIT".to_string()),
                    repository: Some("https://github.com/cursed-lang/http".to_string()),
                    keywords: Vec::from(["http".to_string(), "client".to_string()]),
                    categories: Vec::from(["network".to_string()]),
                },
                dependencies: HashMap::new(),
                dev_dependencies: HashMap::new(),
                build_dependencies: HashMap::new(),
            },
        ]);
        
        registry.add_package("cursed-json", vec![
            PackageVersion {
                version: "2.0.0".to_string(),
                metadata: PackageMetadata {
                    description: Some("JSON parser for CURSED".to_string()),
                    license: Some("MIT".to_string()),
                    repository: Some("https://github.com/cursed-lang/json".to_string()),
                    keywords: Vec::from(["json".to_string(), "parser".to_string()]),
                    categories: Vec::from(["parsing".to_string()]),
                },
                dependencies: HashMap::new(),
                dev_dependencies: HashMap::new(),
                build_dependencies: HashMap::new(),
            },
        ]);
        
        registry
    }
    
    fn add_package(&mut self, name: &str, versions: Vec<PackageVersion>) {
        self.packages.insert(name.to_string(), versions);
    }
    
    pub fn get_package(&self, name: &str, version: &str) -> Result<&PackageVersion, DependencyError> {
        let versions = self.packages.get(name)
            .ok_or_else(|| DependencyError::PackageNotFound { package: name.to_string() })?;
        
        versions.iter()
            .find(|v| v.version == version)
            .ok_or_else(|| DependencyError::PackageNotFound { package: format!("{}@{}", name, version) })
    }
    
    pub fn get_versions(&self, name: &str) -> Result<Vec<String>, DependencyError> {
        let versions = self.packages.get(name)
            .ok_or_else(|| DependencyError::PackageNotFound { package: name.to_string() })?;
        
        Ok(versions.iter().map(|v| v.version.clone()).collect())
    }
}

/// Parse version string into components
fn parse_version(version: &str) -> Result<Vec<u32>, DependencyError> {
    version
        .split('.')
        .map(|part| {
            part.parse::<u32>()
                .map_err(|_| DependencyError::InvalidConstraint {
                    constraint: version.to_string(),
                })
        })
        .collect()
}

/// Compare two version strings
fn version_compare(a: &str, b: &str) -> std::cmp::Ordering {
    let a_parts = parse_version(a).unwrap_or_default();
    let b_parts = parse_version(b).unwrap_or_default();
    
    for i in 0..3 {
        let a_part = a_parts.get(i).unwrap_or(&0);
        let b_part = b_parts.get(i).unwrap_or(&0);
        
        match a_part.cmp(b_part) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }
    
    std::cmp::Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dependency_resolution() {
        let mut resolver = DependencyResolver::new();
        
        let mut dependencies = HashMap::new();
        dependencies.insert("cursed-http".to_string(), "^1.0.0".to_string());
        dependencies.insert("cursed-json".to_string(), "^2.0.0".to_string());
        
        let graph = resolver.resolve(&dependencies).unwrap();
        
        assert_eq!(graph.nodes.len(), 2);
        assert!(graph.nodes.contains_key("cursed-http"));
        assert!(graph.nodes.contains_key("cursed-json"));
        
        assert_eq!(graph.resolved_versions["cursed-http"], "1.1.0");
        assert_eq!(graph.resolved_versions["cursed-json"], "2.0.0");
    }
    
    #[test]
    fn test_version_constraints() {
        let resolver = VersionConstraintResolver::new();
        
        // Test caret constraints
        assert!(resolver.satisfies_constraint("1.2.3", "^1.0.0").unwrap());
        assert!(resolver.satisfies_constraint("1.0.0", "^1.0.0").unwrap());
        assert!(!resolver.satisfies_constraint("2.0.0", "^1.0.0").unwrap());
        
        // Test tilde constraints
        assert!(resolver.satisfies_constraint("1.2.5", "~1.2.3").unwrap());
        assert!(!resolver.satisfies_constraint("1.3.0", "~1.2.3").unwrap());
        
        // Test exact constraints
        assert!(resolver.satisfies_constraint("1.0.0", "1.0.0").unwrap());
        assert!(!resolver.satisfies_constraint("1.0.1", "1.0.0").unwrap());
    }
    
    #[test]
    fn test_version_comparison() {
        assert_eq!(version_compare("1.0.0", "1.0.0"), std::cmp::Ordering::Equal);
        assert_eq!(version_compare("1.0.1", "1.0.0"), std::cmp::Ordering::Greater);
        assert_eq!(version_compare("1.0.0", "1.0.1"), std::cmp::Ordering::Less);
        assert_eq!(version_compare("2.0.0", "1.9.9"), std::cmp::Ordering::Greater);
    }
    
    #[test]
    fn test_topological_sort() {
        let mut resolver = DependencyResolver::new();
        
        // Create a simple dependency chain: A -> B -> C
        resolver.graph.nodes.insert("A".to_string(), DependencyNode {
            name: "A".to_string(),
            version: "1.0.0".to_string(),
            metadata: PackageMetadata {
                description: None,
                license: None,
                repository: None,
                keywords: Vec::from([]),
                categories: Vec::from([]),
            },
            dependencies: Vec::from(["B".to_string()]),
            dev_dependencies: Vec::from([]),
            build_dependencies: Vec::from([]),
        });
        
        resolver.graph.nodes.insert("B".to_string(), DependencyNode {
            name: "B".to_string(),
            version: "1.0.0".to_string(),
            metadata: PackageMetadata {
                description: None,
                license: None,
                repository: None,
                keywords: Vec::from([]),
                categories: Vec::from([]),
            },
            dependencies: Vec::from(["C".to_string()]),
            dev_dependencies: Vec::from([]),
            build_dependencies: Vec::from([]),
        });
        
        resolver.graph.nodes.insert("C".to_string(), DependencyNode {
            name: "C".to_string(),
            version: "1.0.0".to_string(),
            metadata: PackageMetadata {
                description: None,
                license: None,
                repository: None,
                keywords: Vec::from([]),
                categories: Vec::from([]),
            },
            dependencies: Vec::from([]),
            dev_dependencies: Vec::from([]),
            build_dependencies: Vec::from([]),
        });
        
        resolver.graph.edges.push(DependencyEdge {
            from: "A".to_string(),
            to: "B".to_string(),
            dependency_type: DependencyType::Runtime,
            constraint: "1.0.0".to_string(),
        });
        
        resolver.graph.edges.push(DependencyEdge {
            from: "B".to_string(),
            to: "C".to_string(),
            dependency_type: DependencyType::Runtime,
            constraint: "1.0.0".to_string(),
        });
        
        let sorted = resolver.topological_sort().unwrap();
        
        // C should come before B, B should come before A
        let c_pos = sorted.iter().position(|x| x == "C").unwrap();
        let b_pos = sorted.iter().position(|x| x == "B").unwrap();
        let a_pos = sorted.iter().position(|x| x == "A").unwrap();
        
        assert!(c_pos < b_pos);
        assert!(b_pos < a_pos);
    }
}
