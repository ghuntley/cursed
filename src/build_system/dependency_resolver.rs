// Dependency Resolution System
// 
// Resolves package dependencies, handles version constraints,
// and builds dependency graphs for CURSED projects.

use crate::package_manager::{
    resolver::{DependencyResolver as PackageResolver, ConflictResolutionStrategy, ResolvedDependency}
// };
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use semver::{Version, VersionReq};
use tracing::{debug, info, instrument, warn, error};
use crate::error::CursedError;

/// Dependency resolver that integrates with the constraint satisfaction resolver
#[derive(Debug)]
pub struct DependencyResolver {
    /// Resolved dependency graph
    
    /// Real constraint satisfaction resolver
    
    /// Package registry interface
    
    /// Legacy constraint resolver for fallback
/// Dependency graph representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    /// Map of package name to dependency node
    
    /// Edges representing dependencies
    
    /// Resolved versions for each package
/// Node in the dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    /// Package name
    
    /// Resolved version
    
    /// Package metadata
    
    /// Direct dependencies
    
    /// Development dependencies
    
    /// Build dependencies
/// Edge in the dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    /// Source package
    
    /// Target package
    
    /// Dependency type
    
    /// Version constraint
/// Types of dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
/// Package metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    /// Package description
    
    /// License
    
    /// Repository URL
    
    /// Keywords
    
    /// Categories
/// Version constraint resolver
#[derive(Debug)]
/// Dependency resolution error types
#[derive(Debug, thiserror::CursedError)]
pub enum DependencyError {
    #[error("Package not found: {package}")]
    
    #[error("Version constraint conflict: {package} requires {constraint1} and {constraint2}")]
    VersionConflict {
    
    #[error("Circular dependency detected: {cycle}")]
    
    #[error("Invalid version constraint: {constraint}")]
    
    #[error("No compatible version found for {package} with constraint {constraint}")]
    
    #[error("Registry error: {0}")]
impl DependencyResolver {
    /// Create a new dependency resolver
    pub fn new() -> Self {
        DependencyResolver {
            graph: DependencyGraph {
            package_resolver: PackageResolver::with_config(
                ConflictResolutionStrategy::LatestCompatible
        }
    }


    
    /// Set the registry for this resolver
    pub fn set_registry(&mut self, registry: Arc<Mutex<PackageRegistry>>) {
        self.registry = Some(registry.clone());
        self.package_resolver.set_registry(registry);
    /// Resolve dependencies using the real constraint satisfaction algorithm
    #[instrument(skip(self, dependencies))]
    pub async fn resolve(&mut self, dependencies: &HashMap<String, String>) -> crate::error::Result<()> {
        info!("Resolving {} dependencies using constraint satisfaction", dependencies.len());
        
        // Clear previous resolution
        self.graph = DependencyGraph {
        
        // Build dependency graph
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        // Add root dependencies to queue
        for (package, constraint) in dependencies {
            queue.push_back((package.clone(), constraint.clone(), None));
        // Process dependency queue
        while let Some((package, constraint, parent)) = queue.pop_front() {
            if visited.contains(&package) {
                continue;
            visited.insert(package.clone());
            
            // Resolve version for this package
            let resolved_version = self.resolve_version(&package, &constraint).await?;
            self.graph.resolved_versions.insert(package.clone(), resolved_version.clone());
            
            // Get package metadata
            let package_metadata = self.get_package_metadata(&package, &resolved_version).await?;
            
            // Create dependency node
            let node = DependencyNode {
                metadata: PackageMetadata {
                build_dependencies: vec![], // Build dependencies not supported in package manager metadata yet
            
            self.graph.nodes.insert(package.clone(), node);
            
            // Add edge if this is not a root dependency
            if let Some(parent_package) = parent {
                self.graph.edges.push(DependencyEdge {
                });
            // Add transitive dependencies to queue
            for (dep_package, dep_version_spec) in &package_metadata.dependencies {
                if !visited.contains(dep_package) {
                    let dep_constraint = match dep_version_spec {
                    queue.push_back((dep_package.clone(), dep_constraint, Some(package.clone())));
                }
            }
        // Check for circular dependencies
        self.check_circular_dependencies()?;
        
        // Validate version constraints
        self.validate_constraints()?;
        
        info!("Successfully resolved {} packages", self.graph.nodes.len());
        Ok(self.graph.clone())
    /// Resolve version for a package given constraints
    async fn resolve_version(&self, package: &str, constraint: &str) -> crate::error::Result<()> {
        debug!("Resolving version for {}: {}", package, constraint);
        
        let available_versions = self.get_available_versions(package).await?;
        let version_strings: Vec<String> = available_versions.iter().map(|v| v.to_string()).collect();
        
        let compatible_version = self.constraint_resolver
            .find_compatible_version(&version_strings, constraint)?;
        
        debug!("Resolved {} to version {}", package, compatible_version);
        Ok(compatible_version)
    /// Get available versions for a package from registry
    async fn get_available_versions(&self, package: &str) -> crate::error::Result<()> {
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
                            ])
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to lock registry for {}: {}. Using fallback versions.", package, e);
                    Ok(vec![
                    ])
                }
            }
        } else {
            warn!("No registry configured, using fallback versions for {}", package);
            // Fallback when no registry is available
            Ok(vec![
            ])
        }
    }
    
    /// Get package metadata from registry
    async fn get_package_metadata(&self, name: &str, version: &str) -> crate::error::Result<()> {
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
                            })
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to lock registry for {}@{}: {}. Creating minimal metadata.", name, version, e);
                    Ok(PkgMetadata {
                    })
                }
            }
        } else {
            warn!("No registry configured, creating minimal metadata for {}@{}", name, version);
            // Create minimal metadata when no registry is available
            Ok(PkgMetadata {
            })
        }
    }
    
    /// Check for circular dependencies
    fn check_circular_dependencies(&self) -> crate::error::Result<()> {
        debug!("Checking for circular dependencies");
        
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for package in self.graph.nodes.keys() {
            if !visited.contains(package) {
                if let Some(cycle) = self.detect_cycle_dfs(package, &mut visited, &mut rec_stack) {
                    return Err(DependencyError::CircularDependency { cycle });
                }
            }
        Ok(())
    /// DFS to detect cycles
    fn detect_cycle_dfs(
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
        rec_stack.remove(package);
        None
    /// Validate all version constraints are satisfied
    fn validate_constraints(&self) -> crate::error::Result<()> {
        debug!("Validating version constraints");
        
        for edge in &self.graph.edges {
            let resolved_version = self.graph.resolved_versions.get(&edge.to)
                .ok_or_else(|| DependencyError::PackageNotFound { package: edge.to.clone() })?;
            
            if !self.constraint_resolver.satisfies_constraint(resolved_version, &edge.constraint)? {
                return Err(DependencyError::VersionConflict {
                });
            }
        }
        
        Ok(())
    /// Get dependency graph in topological order
    pub fn topological_sort(&self) -> crate::error::Result<()> {
        let mut in_degree = HashMap::new();
        let mut adj_list = HashMap::new();
        
        // Initialize in-degree and adjacency list
        for package in self.graph.nodes.keys() {
            in_degree.insert(package.clone(), 0);
            adj_list.insert(package.clone(), Vec::new());
        // Build adjacency list and calculate in-degrees
        for edge in &self.graph.edges {
            adj_list.get_mut(&edge.from).unwrap().push(edge.to.clone());
            *in_degree.get_mut(&edge.to).unwrap() += 1;
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
            });
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
    ) -> crate::error::Result<()> {
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
            });
        // Sort versions in descending order and return the highest
        compatible_versions.sort_by(|a, b| b.cmp(a));
        Ok(compatible_versions[0].to_string())
    /// Check if version satisfies constraint
    pub fn satisfies_constraint(&self, version: &str, constraint: &str) -> crate::error::Result<()> {
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



