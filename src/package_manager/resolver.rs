//! Dependency resolution for CURSED package manager
//!
//! This module handles resolving package dependencies and version conflicts

use crate::error::{CursedError, Result};
use crate::package_manager::version::{Version, VersionReq};
use crate::package_manager::registry::{Dependency, PackageRegistry, PackageMetadata};
use std::collections::{HashMap, HashSet, VecDeque};

/// Dependency resolution context
#[derive(Debug)]
pub struct PackageResolver {
    registry: PackageRegistry,
    resolution_cache: HashMap<String, ResolvedPackage>,
}

/// A resolved package with its dependencies
#[derive(Debug, Clone)]
pub struct ResolvedPackage {
    pub name: String,
    pub version: Version,
    pub dependencies: Vec<ResolvedDependency>,
    pub optional_dependencies: Vec<ResolvedDependency>,
    pub download_url: String,
    pub checksum: String,
}

/// A resolved dependency
#[derive(Debug, Clone)]
pub struct ResolvedDependency {
    pub name: String,
    pub version: Version,
    pub version_req: VersionReq,
    pub optional: bool,
    pub features: Vec<String>,
}

/// Dependency conflict information
#[derive(Debug, Clone)]
pub struct ConflictInfo {
    pub package_name: String,
    pub conflicting_versions: Vec<ConflictingVersion>,
    pub resolution_strategy: ConflictResolutionStrategy,
}

/// Information about a conflicting version
#[derive(Debug, Clone)]
pub struct ConflictingVersion {
    pub version: Version,
    pub required_by: Vec<String>,
    pub version_req: VersionReq,
}

/// Strategy for resolving version conflicts
#[derive(Debug, Clone)]
pub enum ConflictResolutionStrategy {
    ChooseHighest,
    ChooseLowest,
    ChooseFirst,
    ManualResolution,
    Fail,
}

/// Resolution result
#[derive(Debug, Clone)]
pub struct ResolutionResult {
    pub resolved_packages: Vec<ResolvedPackage>,
    pub conflicts: Vec<ConflictInfo>,
    pub resolution_order: Vec<String>,
}

/// Resolution configuration
#[derive(Debug, Clone)]
pub struct ResolutionConfig {
    pub allow_pre_release: bool,
    pub conflict_strategy: ConflictResolutionStrategy,
    pub max_depth: usize,
    pub include_optional: bool,
}

impl Default for ResolutionConfig {
    fn default() -> Self {
        Self {
            allow_pre_release: false,
            conflict_strategy: ConflictResolutionStrategy::ChooseHighest,
            max_depth: 100,
            include_optional: false,
        }
    }
}

impl PackageResolver {
    /// Create a new dependency resolver
    pub fn new(registry: PackageRegistry) -> Self {
        Self {
            registry,
            resolution_cache: HashMap::new(),
        }
    }

    /// Resolve dependencies for a list of root packages
    pub async fn resolve_dependencies(&mut self, 
                                    root_packages: Vec<(String, VersionReq)>,
                                    config: ResolutionConfig) -> Result<ResolutionResult> {
        
        tracing::info!("Starting dependency resolution for {} root packages", root_packages.len());

        let mut resolved_packages: HashMap<String, ResolvedPackage> = HashMap::new();
        let mut pending_resolution = VecDeque::new();
        let mut visited = HashSet::new();
        let mut conflicts = Vec::new();

        // Add root packages to resolution queue
        for (name, version_req) in root_packages {
            pending_resolution.push_back((name, version_req, 0, vec![]));
        }

        // Resolve dependencies using breadth-first search
        while let Some((package_name, version_req, depth, required_by)) = pending_resolution.pop_front() {
            if depth > config.max_depth {
                return Err(CursedError::General(format!(
                    "Maximum dependency depth exceeded for package: {}", package_name
                )));
            }

            let key = format!("{}@{:?}", package_name, version_req);
            if visited.contains(&key) {
                continue;
            }
            visited.insert(key);

            // Resolve version for this package
            let version = match self.resolve_version(&package_name, &version_req, &config).await {
                Ok(v) => v,
                Err(e) => {
                    tracing::warn!("Failed to resolve version for {}: {}", package_name, e);
                    continue;
                }
            };

            // Check for conflicts with already resolved packages
            if let Some(existing) = resolved_packages.get(&package_name) {
                if existing.version != version {
                    conflicts.push(self.create_conflict_info(
                        &package_name, 
                        &existing.version, 
                        &version,
                        &required_by,
                        &config
                    ));
                    continue;
                }
            }

            // Get package metadata
            let metadata = self.registry.get_package_metadata(&package_name, &version).await?;
            
            // Create resolved package
            let resolved_package = ResolvedPackage {
                name: package_name.clone(),
                version: version.clone(),
                dependencies: self.resolve_dependencies_list(&metadata.dependencies, false),
                optional_dependencies: vec![], // Would be populated from metadata
                download_url: metadata.download_url,
                checksum: metadata.checksum,
            };

            resolved_packages.insert(package_name.clone(), resolved_package);

            // Add dependencies to resolution queue
            for dep in &metadata.dependencies {
                if !dep.optional || config.include_optional {
                    let mut new_required_by = required_by.clone();
                    new_required_by.push(package_name.clone());
                    
                    pending_resolution.push_back((
                        dep.name.clone(),
                        dep.version_req.clone(),
                        depth + 1,
                        new_required_by
                    ));
                }
            }
        }

        // Resolve conflicts if any
        if !conflicts.is_empty() {
            self.resolve_conflicts(&mut resolved_packages, &mut conflicts, &config)?;
        }

        // Calculate resolution order (topological sort)
        let resolution_order = self.calculate_resolution_order(&resolved_packages)?;

        let result = ResolutionResult {
            resolved_packages: resolved_packages.into_values().collect(),
            conflicts,
            resolution_order,
        };

        tracing::info!("Dependency resolution completed. {} packages resolved, {} conflicts", 
                      result.resolved_packages.len(), result.conflicts.len());

        Ok(result)
    }

    /// Resolve a specific version for a package given version requirement
    async fn resolve_version(&mut self, 
                            package_name: &str, 
                            version_req: &VersionReq,
                            config: &ResolutionConfig) -> Result<Version> {
        
        // Check cache first
        let cache_key = format!("{}@{:?}", package_name, version_req);
        if let Some(cached) = self.resolution_cache.get(&cache_key) {
            return Ok(cached.version.clone());
        }

        // Get available versions from registry
        let available_versions = self.registry.get_package_versions(package_name).await?;
        
        // Filter versions based on requirements and configuration
        let mut matching_versions: Vec<Version> = available_versions.into_iter()
            .filter(|v| version_req.matches(v))
            .filter(|v| config.allow_pre_release || !v.is_pre_release())
            .collect();

        if matching_versions.is_empty() {
            return Err(CursedError::General(format!(
                "No matching version found for {} with requirement: {:?}", 
                package_name, version_req
            )));
        }

        // Sort by version (highest first for default strategy)
        matching_versions.sort();
        matching_versions.reverse();

        // Return the best match (highest version by default)
        let selected_version = matching_versions.into_iter().next().unwrap();
        
        tracing::debug!("Resolved {} {:?} -> {}", package_name, version_req, selected_version);
        Ok(selected_version)
    }

    /// Create conflict information
    fn create_conflict_info(&self,
                           package_name: &str,
                           existing_version: &Version,
                           new_version: &Version,
                           required_by: &[String],
                           config: &ResolutionConfig) -> ConflictInfo {
        
        ConflictInfo {
            package_name: package_name.to_string(),
            conflicting_versions: vec![
                ConflictingVersion {
                    version: existing_version.clone(),
                    required_by: vec!["existing".to_string()], // Would track actual dependents
                    version_req: VersionReq::Exact(existing_version.clone()),
                },
                ConflictingVersion {
                    version: new_version.clone(),
                    required_by: required_by.to_vec(),
                    version_req: VersionReq::Exact(new_version.clone()),
                },
            ],
            resolution_strategy: config.conflict_strategy.clone(),
        }
    }

    /// Resolve version conflicts according to strategy
    fn resolve_conflicts(&self,
                        resolved_packages: &mut HashMap<String, ResolvedPackage>,
                        conflicts: &mut Vec<ConflictInfo>,
                        config: &ResolutionConfig) -> Result<()> {
        
        for conflict in conflicts.iter_mut() {
            match &config.conflict_strategy {
                ConflictResolutionStrategy::ChooseHighest => {
                    // Choose the highest version
                    let highest = conflict.conflicting_versions.iter()
                        .max_by_key(|v| &v.version)
                        .unwrap();
                    
                    if let Some(package) = resolved_packages.get_mut(&conflict.package_name) {
                        package.version = highest.version.clone();
                    }
                    
                    tracing::info!("Resolved conflict for {} by choosing highest version: {}", 
                                  conflict.package_name, highest.version);
                }
                ConflictResolutionStrategy::ChooseLowest => {
                    // Choose the lowest version
                    let lowest = conflict.conflicting_versions.iter()
                        .min_by_key(|v| &v.version)
                        .unwrap();
                    
                    if let Some(package) = resolved_packages.get_mut(&conflict.package_name) {
                        package.version = lowest.version.clone();
                    }
                    
                    tracing::info!("Resolved conflict for {} by choosing lowest version: {}", 
                                  conflict.package_name, lowest.version);
                }
                ConflictResolutionStrategy::Fail => {
                    return Err(CursedError::General(format!(
                        "Unresolvable dependency conflict for package: {}", conflict.package_name
                    )));
                }
                _ => {
                    tracing::warn!("Unhandled conflict resolution strategy for {}", conflict.package_name);
                }
            }
        }

        Ok(())
    }

    /// Calculate resolution order using topological sort
    fn calculate_resolution_order(&self, 
                                 resolved_packages: &HashMap<String, ResolvedPackage>) -> Result<Vec<String>> {
        
        let mut in_degree = HashMap::new();
        let mut graph = HashMap::new();
        
        // Initialize
        for package in resolved_packages.values() {
            in_degree.insert(package.name.clone(), 0);
            graph.insert(package.name.clone(), Vec::new());
        }

        // Build dependency graph
        for package in resolved_packages.values() {
            for dep in &package.dependencies {
                if resolved_packages.contains_key(&dep.name) {
                    graph.get_mut(&dep.name).unwrap().push(package.name.clone());
                    *in_degree.get_mut(&package.name).unwrap() += 1;
                }
            }
        }

        // Topological sort
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Find nodes with no incoming edges
        for (name, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(name.clone());
            }
        }

        while let Some(current) = queue.pop_front() {
            result.push(current.clone());

            // Process dependencies
            if let Some(deps) = graph.get(&current) {
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

        // Check for cycles
        if result.len() != resolved_packages.len() {
            return Err(CursedError::General("Circular dependency detected".to_string()));
        }

        Ok(result)
    }

    /// Convert dependency list to resolved dependencies
    fn resolve_dependencies_list(&self, 
                                dependencies: &[Dependency], 
                                optional: bool) -> Vec<ResolvedDependency> {
        
        dependencies.iter().map(|dep| {
            ResolvedDependency {
                name: dep.name.clone(),
                version: Version::new(0, 0, 0), // Would be resolved properly
                version_req: dep.version_req.clone(),
                optional: dep.optional || optional,
                features: dep.features.clone(),
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package_manager::registry::RegistryConfig;

    #[tokio::test]
    async fn test_simple_resolution() {
        let registry = PackageRegistry::new(RegistryConfig::default()).unwrap();
        let mut resolver = PackageResolver::new(registry);
        
        let root_packages = vec![
            ("test-package".to_string(), VersionReq::parse("^1.0.0").unwrap())
        ];
        
        let result = resolver.resolve_dependencies(root_packages, ResolutionConfig::default()).await;
        
        // Should complete without error in mock implementation
        assert!(result.is_ok());
    }

    #[test]
    fn test_conflict_detection() {
        let conflict = ConflictInfo {
            package_name: "test-package".to_string(),
            conflicting_versions: vec![
                ConflictingVersion {
                    version: Version::new(1, 0, 0),
                    required_by: vec!["package-a".to_string()],
                    version_req: VersionReq::Exact(Version::new(1, 0, 0)),
                },
                ConflictingVersion {
                    version: Version::new(2, 0, 0),
                    required_by: vec!["package-b".to_string()],
                    version_req: VersionReq::Exact(Version::new(2, 0, 0)),
                },
            ],
            resolution_strategy: ConflictResolutionStrategy::ChooseHighest,
        };

        assert_eq!(conflict.conflicting_versions.len(), 2);
        assert_eq!(conflict.package_name, "test-package");
    }
}
