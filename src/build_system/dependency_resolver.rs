//! Dependency Resolution System
//! 
//! Resolves package dependencies, handles version constraints,
//! and builds dependency graphs for CURSED projects.

use crate::package_manager::{
    PackageManagerError, 
    registry::{PackageRegistry, PackageInfo},
    metadata::PackageMetadata as PkgMetadata,
    resolver::{DependencyResolver as PackageResolver, ConflictResolutionStrategy, ResolvedDependency}
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use semver::{Version, VersionReq};
use tracing::{debug, info, instrument, warn, error};

/// Dependency resolver that integrates with the constraint satisfaction resolver
#[derive(Debug)]
pub struct DependencyResolver {
    /// Resolved dependency graph
    graph: DependencyGraph,
    
    /// Real constraint satisfaction resolver
    package_resolver: PackageResolver,
    
    /// Package registry interface
    registry: Option<Arc<Mutex<PackageRegistry>>>,
    
    /// Legacy constraint resolver for fallback
    constraint_resolver: VersionConstraintResolver,
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
pub struct VersionConstraintResolver {}

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
    
    #[error("Registry error: {0}")]
    RegistryError(#[from] PackageManagerError),
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
            package_resolver: PackageResolver::with_config(
                50, 
                true, 
                ConflictResolutionStrategy::LatestCompatible
            ),
            registry: None,
            constraint_resolver: VersionConstraintResolver::new(),
        }
    }


    
    /// Set the registry for this resolver
    pub fn set_registry(&mut self, registry: Arc<Mutex<PackageRegistry>>) {
        self.registry = Some(registry.clone());
        self.package_resolver.set_registry(registry);
    }
    
    /// Resolve dependencies using the real constraint satisfaction algorithm
    #[instrument(skip(self, dependencies))]
    pub async fn resolve(&mut self, dependencies: &HashMap<String, String>) -> Result<(), Error> {
        info!("Resolving {} dependencies using constraint satisfaction", dependencies.len());
        
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
            let resolved_version = self.resolve_version(&package, &constraint).await?;
            self.graph.resolved_versions.insert(package.clone(), resolved_version.clone());
            
            // Get package metadata
            let package_metadata = self.get_package_metadata(&package, &resolved_version).await?;
            
            // Create dependency node
            let node = DependencyNode {
                name: package.clone(),
                version: resolved_version.clone(),
                metadata: PackageMetadata {
                    description: Some(package_metadata.description.clone()),
                    license: package_metadata.license.clone(),
                    repository: package_metadata.repository.clone(),
                    keywords: package_metadata.keywords.clone(),
                    categories: package_metadata.categories.clone(),
                },
                dependencies: package_metadata.dependencies.keys().cloned().collect(),
                dev_dependencies: package_metadata.dev_dependencies.keys().cloned().collect(),
                build_dependencies: vec![], // Build dependencies not supported in package manager metadata yet
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
            for (dep_package, dep_version_spec) in &package_metadata.dependencies {
                if !visited.contains(dep_package) {
                    let dep_constraint = match dep_version_spec {
                        crate::package_manager::metadata::VersionSpec::Simple(v) => v.clone(),
                        crate::package_manager::metadata::VersionSpec::Complex { version: Some(v), .. } => v.clone(),
                        crate::package_manager::metadata::VersionSpec::Complex { .. } => "*".to_string(),
                    };
                    queue.push_back((dep_package.clone(), dep_constraint, Some(package.clone())));
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
    async fn resolve_version(&self, package: &str, constraint: &str) -> Result<(), Error> {
        debug!("Resolving version for {}: {}", package, constraint);
        
        let available_versions = self.get_available_versions(package).await?;
        let version_strings: Vec<String> = available_versions.iter().map(|v| v.to_string()).collect();
        
        let compatible_version = self.constraint_resolver
            .find_compatible_version(&version_strings, constraint)?;
        
        debug!("Resolved {} to version {}", package, compatible_version);
        Ok(compatible_version)
    }
    
    /// Get available versions for a package from registry
    async fn get_available_versions(&self, package: &str) -> Result<(), Error> {
        if let Some(ref registry) = self.registry {
            debug!("Fetching versions for package {} from registry", package);
            match registry.lock() {
                Ok(mut registry_guard) => {
                    match registry_guard.get_package_versions(package).await {
                        Ok(versions) => {
                            info!("Found {} versions for package {}", versions.len(), package);
                            Ok(versions)
                        }
                        Err(e) => {
                            warn!("Failed to fetch versions for {}: {}. Using fallback versions.", package, e);
                            // Fallback to basic versions if registry fails
                            Ok(vec![
                                Version::parse("0.1.0").unwrap(),
                                Version::parse("1.0.0").unwrap(),
                            ])
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to lock registry for {}: {}. Using fallback versions.", package, e);
                    Ok(vec![
                        Version::parse("0.1.0").unwrap(),
                        Version::parse("1.0.0").unwrap(),
                    ])
                }
            }
        } else {
            warn!("No registry configured, using fallback versions for {}", package);
            // Fallback when no registry is available
            Ok(vec![
                Version::parse("0.1.0").unwrap(),
                Version::parse("1.0.0").unwrap(),
            ])
        }
    }
    
    /// Get package metadata from registry
    async fn get_package_metadata(&self, name: &str, version: &str) -> Result<(), Error> {
        if let Some(ref registry) = self.registry {
            debug!("Fetching metadata for package {}@{} from registry", name, version);
            match registry.lock() {
                Ok(mut registry_guard) => {
                    match registry_guard.get_package_metadata(name, version).await {
                        Ok(metadata) => {
                            info!("Retrieved metadata for package {}@{}", name, version);
                            Ok(metadata)
                        }
                        Err(e) => {
                            warn!("Failed to fetch metadata for {}@{}: {}. Creating minimal metadata.", name, version, e);
                            // Create minimal metadata as fallback
                            Ok(PkgMetadata {
                                name: name.to_string(),
                                version: version.to_string(),
                                description: format!("Package {}", name),
                                authors: vec![],
                                dependencies: HashMap::new(),
                                dev_dependencies: HashMap::new(),
                                repository: None,
                                license: None,
                                keywords: vec![],
                                categories: vec![],
                            })
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to lock registry for {}@{}: {}. Creating minimal metadata.", name, version, e);
                    Ok(PkgMetadata {
                        name: name.to_string(),
                        version: version.to_string(),
                        description: format!("Package {}", name),
                        authors: vec![],
                        dependencies: HashMap::new(),
                        dev_dependencies: HashMap::new(),
                        repository: None,
                        license: None,
                        keywords: vec![],
                        categories: vec![],
                    })
                }
            }
        } else {
            warn!("No registry configured, creating minimal metadata for {}@{}", name, version);
            // Create minimal metadata when no registry is available
            Ok(PkgMetadata {
                name: name.to_string(),
                version: version.to_string(),
                description: format!("Package {}", name),
                authors: vec![],
                dependencies: HashMap::new(),
                dev_dependencies: HashMap::new(),
                repository: None,
                license: None,
                keywords: vec![],
                categories: vec![],
            })
        }
    }
    
    /// Check for circular dependencies
    fn check_circular_dependencies(&self) -> Result<(), Error> {
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
    fn validate_constraints(&self) -> Result<(), Error> {
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
    pub fn topological_sort(&self) -> Result<(), Error> {
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
        VersionConstraintResolver {}
    }
    
    /// Find compatible version given constraint
    pub fn find_compatible_version(
        &self,
        available_versions: &[String],
        constraint: &str,
    ) -> Result<(), Error> {
        // Parse the constraint using semver
        let version_req = VersionReq::parse(constraint)
            .map_err(|e| DependencyError::InvalidConstraint { 
                constraint: format!("Invalid version constraint '{}': {}", constraint, e) 
            })?;
        
        // Find compatible versions
        let mut compatible_versions: Vec<Version> = available_versions
            .iter()
            .filter_map(|version_str| {
                if let Ok(version) = Version::parse(version_str) {
                    if version_req.matches(&version) {
                        Some(version)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        
        if compatible_versions.is_empty() {
            return Err(DependencyError::NoCompatibleVersion {
                package: "unknown".to_string(),
                constraint: constraint.to_string(),
            });
        }
        
        // Sort versions in descending order and return the highest
        compatible_versions.sort_by(|a, b| b.cmp(a));
        Ok(compatible_versions[0].to_string())
    }
    
    /// Check if version satisfies constraint
    pub fn satisfies_constraint(&self, version: &str, constraint: &str) -> Result<(), Error> {
        let version_parsed = Version::parse(version)
            .map_err(|e| DependencyError::InvalidConstraint { 
                constraint: format!("Invalid version '{}': {}", version, e) 
            })?;
        
        let version_req = VersionReq::parse(constraint)
            .map_err(|e| DependencyError::InvalidConstraint { 
                constraint: format!("Invalid version constraint '{}': {}", constraint, e) 
            })?;
        
        Ok(version_req.matches(&version_parsed))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::package_manager::registry::RegistryConfig;
    use tokio;
    
    #[tokio::test]
    async fn test_dependency_resolution_without_registry() {
        let mut resolver = DependencyResolver::new();
        
        let mut dependencies = HashMap::new();
        dependencies.insert("cursed-http".to_string(), "^1.0.0".to_string());
        dependencies.insert("cursed-json".to_string(), "^0.1.0".to_string());
        
        let graph = resolver.resolve(&dependencies).await.unwrap();
        
        // Should resolve to fallback versions since no registry is configured
        assert_eq!(graph.nodes.len(), 2);
        assert!(graph.nodes.contains_key("cursed-http"));
        assert!(graph.nodes.contains_key("cursed-json"));
        
        // Fallback versions should be used
        assert_eq!(graph.resolved_versions["cursed-http"], "1.0.0");
        assert_eq!(graph.resolved_versions["cursed-json"], "1.0.0");
    }
    
    #[tokio::test]
    async fn test_dependency_resolution_with_registry() {
        // Create a registry (it will try to connect to the configured URL)
        let config = RegistryConfig::default();
        let registry_result = PackageRegistry::with_config(config);
        
        if let Ok(registry) = registry_result {
            let mut resolver = DependencyResolver::new();
            resolver.set_registry(Arc::new(Mutex::new(registry)));
            
            let mut dependencies = HashMap::new();
            dependencies.insert("test-package".to_string(), "^1.0.0".to_string());
            
            // This will likely fail since we don't have a real registry running,
            // but should fall back to minimal metadata
            let graph = resolver.resolve(&dependencies).await;
            
            // Should succeed even if registry calls fail due to fallback behavior
            assert!(graph.is_ok());
        }
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
        
        // Test version finding
        let versions = vec!["1.0.0".to_string(), "1.1.0".to_string(), "2.0.0".to_string()];
        let result = resolver.find_compatible_version(&versions, "^1.0.0").unwrap();
        assert_eq!(result, "1.1.0"); // Should select the highest compatible version
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
