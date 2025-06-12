use crate::package_manager::{PackageManagerError, metadata::PackageMetadata, registry::{PackageInfo, PackageRegistry}};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use semver::{Version, VersionReq};
use tracing::{info, warn, error, debug, instrument};

/// Dependency resolver statistics with detailed metrics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ResolverStats {
    pub resolved_count: usize,
    pub cached_count: usize,
    pub failed_count: usize,
    pub cache_size: usize,
    pub circular_dependencies_detected: usize,
    pub conflicts_resolved: usize,
    pub backtrack_attempts: usize,
    pub resolution_time_ms: u64,
}

/// Advanced dependency resolver with conflict resolution and cycle detection
#[derive(Debug)]
pub struct DependencyResolver {
    stats: ResolverStats,
    resolution_cache: HashMap<String, Vec<ResolvedDependency>>,
    version_cache: HashMap<String, Vec<Version>>,
    metadata_cache: HashMap<String, PackageMetadata>,
    max_depth: usize,
    allow_dev_dependencies: bool,
    conflict_resolution_strategy: ConflictResolutionStrategy,
    registry: Option<Arc<Mutex<PackageRegistry>>>,
}

/// Resolved dependency information with detailed context
#[derive(Debug, Clone)]
pub struct ResolvedDependency {
    pub package: PackageMetadata,
    pub depth: usize,
    pub required_by: Vec<String>,
    pub constraint: String,
    pub resolved_version: Version,
    pub is_dev_dependency: bool,
    pub optional: bool,
}

/// Dependency constraint information
#[derive(Debug, Clone)]
pub struct DependencyConstraint {
    pub name: String,
    pub version_req: VersionReq,
    pub required_by: String,
    pub is_dev: bool,
    pub optional: bool,
    pub features: Vec<String>,
}

/// Version selection result with conflict information
#[derive(Debug, Clone)]
pub struct VersionSelection {
    pub version: Version,
    pub satisfies: Vec<String>, // List of constraints this version satisfies
    pub conflicts: Vec<ConflictInfo>,
}

/// Conflict information for resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    pub package: String,
    pub conflicting_versions: Vec<String>,
    pub required_by: Vec<String>,
    pub reason: ConflictReason,
}

/// Types of conflicts that can occur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictReason {
    IncompatibleVersions,
    CircularDependency,
    MissingPackage,
    InvalidConstraint,
}

/// Strategy for resolving conflicts
#[derive(Debug, Clone, Copy)]
pub enum ConflictResolutionStrategy {
    LatestCompatible,
    ConservativeUpdate,
    MinimalChange,
    UserPrompt,
}

/// Resolution context for tracking state during resolution
#[derive(Debug)]
struct ResolutionContext {
    resolved: HashMap<String, ResolvedDependency>,
    constraints: HashMap<String, Vec<DependencyConstraint>>,
    visiting: HashSet<String>, // For cycle detection
    depth: usize,
    max_depth: usize,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self::with_config(50, true, ConflictResolutionStrategy::LatestCompatible)
    }

    pub fn with_config(
        max_depth: usize, 
        allow_dev_dependencies: bool,
        conflict_strategy: ConflictResolutionStrategy
    ) -> Self {
        Self {
            stats: ResolverStats::default(),
            resolution_cache: HashMap::new(),
            version_cache: HashMap::new(),
            metadata_cache: HashMap::new(),
            max_depth,
            allow_dev_dependencies,
            conflict_resolution_strategy: conflict_strategy,
            registry: None,
        }
    }

    /// Create a new resolver with a registry connection
    pub fn with_registry(registry: Arc<Mutex<PackageRegistry>>) -> Self {
        Self {
            stats: ResolverStats::default(),
            resolution_cache: HashMap::new(),
            version_cache: HashMap::new(),
            metadata_cache: HashMap::new(),
            max_depth: 50,
            allow_dev_dependencies: true,
            conflict_resolution_strategy: ConflictResolutionStrategy::LatestCompatible,
            registry: Some(registry),
        }
    }

    /// Set the registry for this resolver
    pub fn set_registry(&mut self, registry: Arc<Mutex<PackageRegistry>>) {
        self.registry = Some(registry);
    }
    
    /// Get resolver statistics
    pub fn get_stats(&self) -> &ResolverStats {
        &self.stats
    }

    /// Clear internal caches
    pub fn clear_cache(&mut self) {
        self.resolution_cache.clear();
        self.version_cache.clear();
        self.metadata_cache.clear();
        self.stats.cache_size = 0;
    }

    /// Main dependency resolution entry point
    #[instrument(skip(self, package))]
    pub async fn resolve_dependencies(&mut self, package: &PackageInfo) -> Result<Vec<ResolvedDependency>, PackageManagerError> {
        let start_time = std::time::Instant::now();
        info!("Starting dependency resolution for {}@{}", package.name, package.version);

        // Check cache first
        let cache_key = format!("{}@{}", package.name, package.version);
        if let Some(cached_result) = self.resolution_cache.get(&cache_key) {
            self.stats.cached_count += 1;
            debug!("Using cached resolution for {}", cache_key);
            return Ok(cached_result.clone());
        }

        // Create initial package metadata from PackageInfo
        let root_metadata = PackageMetadata {
            name: package.name.clone(),
            version: package.version.clone(),
            description: package.description.clone(),
            authors: package.authors.clone().unwrap_or_default(),
            dependencies: HashMap::new(), // Will be populated from registry
            dev_dependencies: HashMap::new(), // Will be populated from registry
            repository: package.repository.clone(),
            license: package.license.clone(),
            keywords: package.keywords.clone().unwrap_or_default(),
            categories: vec![], // Categories not available in PackageInfo
        };

        // Initialize resolution context
        let mut context = ResolutionContext {
            resolved: HashMap::new(),
            constraints: HashMap::new(),
            visiting: HashSet::new(),
            depth: 0,
            max_depth: self.max_depth,
        };

        // Start resolution with iterative approach to avoid recursion
        let mut to_process = VecDeque::new();
        to_process.push_back(root_metadata);
        
        while let Some(package) = to_process.pop_front() {
            if let Err(e) = self.resolve_package(&package, &mut context, &mut to_process).await {
                self.stats.failed_count += 1;
                error!("Dependency resolution failed for {}: {}", package.name, e);
                return Err(e);
            }
        }

        // After processing all packages
        {
                let mut resolved_deps: Vec<ResolvedDependency> = context.resolved.into_values().collect();
                
                // Sort by depth and name for consistent ordering
                resolved_deps.sort_by(|a, b| {
                    a.depth.cmp(&b.depth).then_with(|| a.package.name.cmp(&b.package.name))
                });

                // Cache the result
                self.resolution_cache.insert(cache_key, resolved_deps.clone());
                self.stats.cache_size = self.resolution_cache.len();
                self.stats.resolved_count += 1;
                
                let elapsed = start_time.elapsed();
                self.stats.resolution_time_ms = elapsed.as_millis() as u64;
                
                info!("Dependency resolution completed successfully for {} with {} dependencies", 
                      package.name, resolved_deps.len());
                      
                Ok(resolved_deps)
        }
    }

    /// Non-recursive dependency resolution with cycle detection
    async fn resolve_package(&mut self, package: &PackageMetadata, context: &mut ResolutionContext, to_process: &mut VecDeque<PackageMetadata>) -> Result<(), PackageManagerError> {
        let package_key = format!("{}@{}", package.name, package.version);
        
        // Check depth limit
        if context.depth > context.max_depth {
            return Err(PackageManagerError::DependencyError { 
                reason: format!("Maximum dependency depth {} exceeded for {}", context.max_depth, package_key) 
            });
        }

        // Cycle detection
        if context.visiting.contains(&package.name) {
            self.stats.circular_dependencies_detected += 1;
            let cycle_path: Vec<_> = context.visiting.iter().cloned().collect();
            warn!("Circular dependency detected: {} -> {}", cycle_path.join(" -> "), package.name);
            
            return Err(PackageManagerError::CircularDependency { 
                cycle: cycle_path 
            });
        }

        // Mark as visiting for cycle detection
        context.visiting.insert(package.name.clone());
        context.depth += 1;

        // Add current package to resolved set if not root
        if context.depth > 1 {
            let resolved_dep = ResolvedDependency {
                package: package.clone(),
                depth: context.depth - 1,
                required_by: vec![],
                constraint: "*".to_string(),
                resolved_version: Version::parse(&package.version)
                    .map_err(|e| PackageManagerError::InvalidVersion { 
                        version: package.version.clone(), 
                        reason: e.to_string() 
                    })?,
                is_dev_dependency: false,
                optional: false,
            };
            context.resolved.insert(package.name.clone(), resolved_dep);
        }

        // Process regular dependencies
        for (dep_name, version_constraint) in &package.dependencies {
            let version_str = match version_constraint {
                crate::package_manager::metadata::VersionSpec::Simple(v) => v.clone(),
                crate::package_manager::metadata::VersionSpec::Complex { version: Some(v), .. } => v.clone(),
                crate::package_manager::metadata::VersionSpec::Complex { .. } => "*".to_string(),
            };
            self.process_dependency(dep_name, &version_str, &package.name, false, context, to_process).await?;
        }

        // Process dev dependencies if enabled
        if self.allow_dev_dependencies {
            for (dep_name, version_constraint) in &package.dev_dependencies {
                let version_str = match version_constraint {
                    crate::package_manager::metadata::VersionSpec::Simple(v) => v.clone(),
                    crate::package_manager::metadata::VersionSpec::Complex { version: Some(v), .. } => v.clone(),
                    crate::package_manager::metadata::VersionSpec::Complex { .. } => "*".to_string(),
                };
                self.process_dependency(dep_name, &version_str, &package.name, true, context, to_process).await?;
            }
        }

        // Remove from visiting set
        context.visiting.remove(&package.name);
        context.depth -= 1;

        Ok(())
    }

    /// Process a single dependency with constraint handling
    async fn process_dependency(
        &mut self,
        dep_name: &str,
        version_constraint: &str,
        required_by: &str,
        is_dev: bool,
        context: &mut ResolutionContext,
        to_process: &mut VecDeque<PackageMetadata>
    ) -> Result<(), PackageManagerError> {
        
        // Parse version constraint
        let version_req = VersionReq::parse(version_constraint)
            .map_err(|e| PackageManagerError::InvalidVersion { 
                version: version_constraint.to_string(), 
                reason: format!("Invalid version constraint: {}", e) 
            })?;

        // Create constraint
        let constraint = DependencyConstraint {
            name: dep_name.to_string(),
            version_req: version_req.clone(),
            required_by: required_by.to_string(),
            is_dev,
            optional: false,
            features: vec![],
        };

        // Add to constraints
        context.constraints.entry(dep_name.to_string()).or_insert_with(Vec::new).push(constraint);

        // Check if already resolved with compatible version
        if let Some(resolved) = context.resolved.get(dep_name) {
            if version_req.matches(&resolved.resolved_version) {
                debug!("Dependency {} already resolved with compatible version {}", dep_name, resolved.resolved_version);
                return Ok(());
            } else {
                // Version conflict
                self.stats.conflicts_resolved += 1;
                return self.handle_version_conflict(dep_name, &version_req, context, to_process).await;
            }
        }

        // Resolve new dependency
        match self.select_version(dep_name, &version_req, context).await {
            Ok(selected_version) => {
                debug!("Selected version {} for dependency {}", selected_version.version, dep_name);
                
                // Get metadata for the selected version and add to resolution
                let metadata = self.get_dependency_metadata(dep_name, &selected_version.version).await?;
                
                let resolved_dep = ResolvedDependency {
                    package: metadata.clone(),
                    depth: context.depth,
                    required_by: vec![required_by.to_string()],
                    constraint: version_constraint.to_string(),
                    resolved_version: selected_version.version,
                    is_dev_dependency: is_dev,
                    optional: false,
                };
                
                context.resolved.insert(dep_name.to_string(), resolved_dep);
                
                // Add to processing queue for dependency resolution
                to_process.push_back(metadata);
                
                Ok(())
            }
            Err(e) => {
                warn!("Failed to resolve dependency {}: {}", dep_name, e);
                Err(e)
            }
        }
    }

    /// Select the best version for a dependency given constraints
    async fn select_version(&mut self, package_name: &str, version_req: &VersionReq, context: &ResolutionContext) -> Result<VersionSelection, PackageManagerError> {
        
        // Get available versions (mock implementation)
        let available_versions = self.get_available_versions(package_name).await?;
        
        // Find compatible versions
        let compatible_versions: Vec<_> = available_versions.iter()
            .filter(|v| version_req.matches(v))
            .collect();

        if compatible_versions.is_empty() {
            return Err(PackageManagerError::DependencyNotFound { 
                name: package_name.to_string(),
                constraint: version_req.to_string() 
            });
        }

        // Apply conflict resolution strategy
        let selected_version = match self.conflict_resolution_strategy {
            ConflictResolutionStrategy::LatestCompatible => {
                (*compatible_versions.iter().max().unwrap()).clone()
            }
            ConflictResolutionStrategy::ConservativeUpdate => {
                // Select minimum compatible version
                (*compatible_versions.iter().min().unwrap()).clone()
            }
            ConflictResolutionStrategy::MinimalChange => {
                // Try to select version closest to existing resolutions
                self.select_minimal_change_version(&compatible_versions, context)
            }
            ConflictResolutionStrategy::UserPrompt => {
                // For now, fallback to latest compatible
                (*compatible_versions.iter().max().unwrap()).clone()
            }
        };

        Ok(VersionSelection {
            version: selected_version,
            satisfies: vec![version_req.to_string()],
            conflicts: vec![], // TODO: Implement conflict detection
        })
    }

    /// Handle version conflicts between dependencies
    async fn handle_version_conflict(&mut self, package_name: &str, new_req: &VersionReq, context: &mut ResolutionContext, to_process: &mut VecDeque<PackageMetadata>) -> Result<(), PackageManagerError> {
        
        self.stats.backtrack_attempts += 1;
        
        // Get all constraints for this package
        let constraints = context.constraints.get(package_name).cloned().unwrap_or_default();
        
        // Try to find a version that satisfies all constraints
        let available_versions = self.get_available_versions(package_name).await?;
        
        for version in available_versions.iter().rev() { // Try latest first
            let satisfies_all = constraints.iter().all(|c| c.version_req.matches(version)) && new_req.matches(version);
            
            if satisfies_all {
                debug!("Found compatible version {} for conflicting constraints on {}", version, package_name);
                
                // Update resolved dependency with new metadata
                if let Some(resolved) = context.resolved.get_mut(package_name) {
                    resolved.resolved_version = version.clone();
                    
                    // Get updated metadata for the new version
                    let updated_metadata = self.get_dependency_metadata(package_name, version).await?;
                    resolved.package = updated_metadata.clone();
                    
                    // Add updated package to processing queue if it has dependencies
                    if !updated_metadata.dependencies.is_empty() || 
                       (!updated_metadata.dev_dependencies.is_empty() && self.allow_dev_dependencies) {
                        to_process.push_back(updated_metadata);
                    }
                }
                
                return Ok(());
            }
        }

        // No compatible version found
        let conflict = ConflictInfo {
            package: package_name.to_string(),
            conflicting_versions: constraints.iter().map(|c| c.version_req.to_string()).collect(),
            required_by: constraints.iter().map(|c| c.required_by.clone()).collect(),
            reason: ConflictReason::IncompatibleVersions,
        };

        Err(PackageManagerError::DependencyVersionConflict { 
            package: package_name.to_string(),
            constraints: constraints.iter().map(|c| c.version_req.to_string()).collect(),
            available: available_versions.iter().map(|v| v.to_string()).collect()
        })
    }

    /// Select version with minimal change from existing resolutions
    fn select_minimal_change_version(&self, compatible_versions: &[&Version], context: &ResolutionContext) -> Version {
        // For now, just return the latest compatible version
        // TODO: Implement sophisticated minimal change algorithm
        (*compatible_versions.iter().max().unwrap()).clone()
    }

    /// Get available versions for a package from registry
    async fn get_available_versions(&mut self, package_name: &str) -> Result<Vec<Version>, PackageManagerError> {
        // Check cache first
        if let Some(cached_versions) = self.version_cache.get(package_name) {
            debug!("Using cached versions for package {}", package_name);
            return Ok(cached_versions.clone());
        }

        // Get versions from registry
        let versions = if let Some(ref registry) = self.registry {
            debug!("Fetching versions for package {} from registry", package_name);
            match registry.lock() {
                Ok(mut registry_guard) => {
                    match registry_guard.get_package_versions(package_name).await {
                        Ok(versions) => {
                            info!("Found {} versions for package {}", versions.len(), package_name);
                            versions
                        }
                        Err(e) => {
                            warn!("Failed to fetch versions for {}: {}. Using fallback versions.", package_name, e);
                            // Fallback to basic versions if registry fails
                            vec![
                                Version::parse("0.1.0").unwrap(),
                                Version::parse("1.0.0").unwrap(),
                            ]
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to lock registry for {}: {}. Using fallback versions.", package_name, e);
                    vec![
                        Version::parse("0.1.0").unwrap(),
                        Version::parse("1.0.0").unwrap(),
                    ]
                }
            }
        } else {
            warn!("No registry configured, using fallback versions for {}", package_name);
            // Fallback when no registry is available
            vec![
                Version::parse("0.1.0").unwrap(),
                Version::parse("1.0.0").unwrap(),
            ]
        };

        // Cache the result
        self.version_cache.insert(package_name.to_string(), versions.clone());
        self.stats.cache_size = self.version_cache.len() + self.metadata_cache.len();

        Ok(versions)
    }

    /// Get dependency metadata from registry with caching
    async fn get_dependency_metadata(&mut self, name: &str, version: &Version) -> Result<PackageMetadata, PackageManagerError> {
        let cache_key = format!("{}@{}", name, version);
        
        // Check cache first
        if let Some(cached_metadata) = self.metadata_cache.get(&cache_key) {
            debug!("Using cached metadata for package {}@{}", name, version);
            return Ok(cached_metadata.clone());
        }

        // Get metadata from registry
        let metadata = if let Some(ref registry) = self.registry {
            debug!("Fetching metadata for package {}@{} from registry", name, version);
            match registry.lock() {
                Ok(mut registry_guard) => {
                    match registry_guard.get_package_metadata(name, &version.to_string()).await {
                        Ok(metadata) => {
                            info!("Retrieved metadata for package {}@{}", name, version);
                            metadata
                        }
                        Err(e) => {
                            warn!("Failed to fetch metadata for {}@{}: {}. Creating minimal metadata.", name, version, e);
                            // Create minimal metadata as fallback
                            PackageMetadata {
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
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to lock registry for {}@{}: {}. Creating minimal metadata.", name, version, e);
                    PackageMetadata {
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
                    }
                }
            }
        } else {
            warn!("No registry configured, creating minimal metadata for {}@{}", name, version);
            // Create minimal metadata when no registry is available
            PackageMetadata {
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
            }
        };

        // Cache the result
        self.metadata_cache.insert(cache_key, metadata.clone());
        self.stats.cache_size = self.version_cache.len() + self.metadata_cache.len();

        Ok(metadata)
    }

    /// Check for conflicts in current resolution state
    pub fn check_conflicts(&self, dependencies: &[ResolvedDependency]) -> Vec<ConflictInfo> {
        let mut conflicts = Vec::new();
        let mut package_versions: HashMap<String, Vec<&ResolvedDependency>> = HashMap::new();

        // Group by package name
        for dep in dependencies {
            package_versions.entry(dep.package.name.clone())
                .or_insert_with(Vec::new)
                .push(dep);
        }

        // Check for version conflicts
        for (package_name, deps) in package_versions {
            if deps.len() > 1 {
                let versions: Vec<String> = deps.iter()
                    .map(|d| d.resolved_version.to_string())
                    .collect();
                
                let required_by: Vec<String> = deps.iter()
                    .flat_map(|d| d.required_by.iter().cloned())
                    .collect();

                conflicts.push(ConflictInfo {
                    package: package_name,
                    conflicting_versions: versions,
                    required_by,
                    reason: ConflictReason::IncompatibleVersions,
                });
            }
        }

        conflicts
    }

    /// Generate dependency tree representation
    pub fn generate_tree(&self, dependencies: &[ResolvedDependency]) -> String {
        let mut tree = String::new();
        let mut by_depth: HashMap<usize, Vec<&ResolvedDependency>> = HashMap::new();

        // Group by depth
        for dep in dependencies {
            by_depth.entry(dep.depth).or_insert_with(Vec::new).push(dep);
        }

        // Sort depths and generate tree
        let mut depths: Vec<_> = by_depth.keys().cloned().collect();
        depths.sort();

        for depth in depths {
            if let Some(deps) = by_depth.get(&depth) {
                let mut sorted_deps = deps.clone();
                sorted_deps.sort_by(|a, b| a.package.name.cmp(&b.package.name));

                for dep in sorted_deps {
                    let indent = "  ".repeat(depth);
                    let dev_marker = if dep.is_dev_dependency { " [dev]" } else { "" };
                    let optional_marker = if dep.optional { " [optional]" } else { "" };
                    
                    tree.push_str(&format!(
                        "{}├── {}@{}{}{}\n",
                        indent,
                        dep.package.name,
                        dep.resolved_version,
                        dev_marker,
                        optional_marker
                    ));
                }
            }
        }

        tree
    }

    /// Export resolution result to different formats
    pub fn export_resolution(&self, dependencies: &[ResolvedDependency], format: ExportFormat) -> Result<String, PackageManagerError> {
        match format {
            ExportFormat::Json => {
                // Create a simplified representation for JSON export
                let simplified: Vec<_> = dependencies.iter().map(|dep| {
                    serde_json::json!({
                        "name": dep.package.name,
                        "version": dep.resolved_version.to_string(),
                        "depth": dep.depth,
                        "required_by": dep.required_by,
                        "constraint": dep.constraint,
                        "is_dev_dependency": dep.is_dev_dependency,
                        "optional": dep.optional
                    })
                }).collect();
                
                serde_json::to_string_pretty(&simplified)
                    .map_err(|e| PackageManagerError::InvalidMetadata { 
                        reason: format!("Failed to serialize resolution: {}", e) 
                    })
            }
            ExportFormat::Yaml => {
                // Create a simplified representation for YAML export
                let simplified: Vec<_> = dependencies.iter().map(|dep| {
                    serde_json::json!({
                        "name": dep.package.name,
                        "version": dep.resolved_version.to_string(),
                        "depth": dep.depth,
                        "required_by": dep.required_by,
                        "constraint": dep.constraint,
                        "is_dev_dependency": dep.is_dev_dependency,
                        "optional": dep.optional
                    })
                }).collect();
                
                serde_yaml::to_string(&simplified)
                    .map_err(|e| PackageManagerError::InvalidMetadata { 
                        reason: format!("Failed to serialize resolution: {}", e) 
                    })
            }
            ExportFormat::Tree => Ok(self.generate_tree(dependencies)),
        }
    }
}

/// Export format options
#[derive(Debug, Clone)]
pub enum ExportFormat {
    Json,
    Yaml,
    Tree,
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_package_info() -> PackageInfo {
        PackageInfo {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Test package".to_string(),
            authors: Some(vec!["Test Author".to_string()]),
            keywords: Some(vec!["test".to_string()]),
            download_url: "https://example.com/download".to_string(),
            checksum: "test-checksum".to_string(),
            size: Some(1024),
            published_at: Some("2023-01-01T00:00:00Z".to_string()),
            repository: None,
            license: Some("MIT".to_string()),
        }
    }

    #[tokio::test]
    async fn test_resolver_creation() {
        let resolver = DependencyResolver::new();
        assert_eq!(resolver.max_depth, 50);
        assert!(resolver.allow_dev_dependencies);
    }

    #[tokio::test]
    async fn test_basic_resolution() {
        let mut resolver = DependencyResolver::new();
        let package = create_test_package_info();
        
        let result = resolver.resolve_dependencies(&package).await;
        assert!(result.is_ok());
        
        let dependencies = result.unwrap();
        assert!(!dependencies.is_empty());
        
        // Check stats
        let stats = resolver.get_stats();
        assert!(stats.resolved_count > 0);
    }

    #[tokio::test]
    async fn test_version_requirement_parsing() {
        let valid_reqs = vec!["1.0.0", "^1.0", "~1.2", ">=1.0.0", "1.0.0 - 2.0.0"];
        
        for req_str in valid_reqs {
            let result = VersionReq::parse(req_str);
            assert!(result.is_ok(), "Failed to parse: {}", req_str);
        }
    }

    #[tokio::test]
    async fn test_conflict_detection() {
        let resolver = DependencyResolver::new();
        
        let deps = vec![
            ResolvedDependency {
                package: PackageMetadata {
                    name: "test-pkg".to_string(),
                    version: "1.0.0".to_string(),
                    description: "Test".to_string(),
                    authors: vec![],
                    dependencies: HashMap::new(),
                    dev_dependencies: HashMap::new(),
                    repository: None,
                    license: None,
                    keywords: vec![],
                    categories: vec![],
                },
                depth: 1,
                required_by: vec!["root".to_string()],
                constraint: "1.0".to_string(),
                resolved_version: Version::parse("1.0.0").unwrap(),
                is_dev_dependency: false,
                optional: false,
            },
            ResolvedDependency {
                package: PackageMetadata {
                    name: "test-pkg".to_string(),
                    version: "2.0.0".to_string(),
                    description: "Test".to_string(),
                    authors: vec![],
                    dependencies: HashMap::new(),
                    dev_dependencies: HashMap::new(),
                    repository: None,
                    license: None,
                    keywords: vec![],
                    categories: vec![],
                },
                depth: 1,
                required_by: vec!["other".to_string()],
                constraint: "2.0".to_string(),
                resolved_version: Version::parse("2.0.0").unwrap(),
                is_dev_dependency: false,
                optional: false,
            },
        ];

        let conflicts = resolver.check_conflicts(&deps);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].package, "test-pkg");
    }

    #[tokio::test]
    async fn test_tree_generation() {
        let resolver = DependencyResolver::new();
        
        let deps = vec![
            ResolvedDependency {
                package: PackageMetadata {
                    name: "dep1".to_string(),
                    version: "1.0.0".to_string(),
                    description: "Test".to_string(),
                    authors: vec![],
                    dependencies: HashMap::new(),
                    dev_dependencies: HashMap::new(),
                    repository: None,
                    license: None,
                    keywords: vec![],
                    categories: vec![],
                },
                depth: 1,
                required_by: vec!["root".to_string()],
                constraint: "1.0".to_string(),
                resolved_version: Version::parse("1.0.0").unwrap(),
                is_dev_dependency: false,
                optional: false,
            },
        ];

        let tree = resolver.generate_tree(&deps);
        assert!(tree.contains("dep1@1.0.0"));
        assert!(tree.contains("├──"));
    }

    #[tokio::test]
    async fn test_cache_functionality() {
        let mut resolver = DependencyResolver::new();
        let package = create_test_package_info();
        
        // First resolution
        let result1 = resolver.resolve_dependencies(&package).await;
        assert!(result1.is_ok());
        
        // Second resolution should use cache
        let result2 = resolver.resolve_dependencies(&package).await;
        assert!(result2.is_ok());
        
        let stats = resolver.get_stats();
        assert!(stats.cached_count > 0);
    }

    #[tokio::test]
    async fn test_export_formats() {
        let resolver = DependencyResolver::new();
        
        let deps = vec![
            ResolvedDependency {
                package: PackageMetadata {
                    name: "test".to_string(),
                    version: "1.0.0".to_string(),
                    description: "Test".to_string(),
                    authors: vec![],
                    dependencies: HashMap::new(),
                    dev_dependencies: HashMap::new(),
                    repository: None,
                    license: None,
                    keywords: vec![],
                    categories: vec![],
                },
                depth: 1,
                required_by: vec!["root".to_string()],
                constraint: "1.0".to_string(),
                resolved_version: Version::parse("1.0.0").unwrap(),
                is_dev_dependency: false,
                optional: false,
            },
        ];

        // Test JSON export
        let json_result = resolver.export_resolution(&deps, ExportFormat::Json);
        assert!(json_result.is_ok());
        
        // Test Tree export
        let tree_result = resolver.export_resolution(&deps, ExportFormat::Tree);
        assert!(tree_result.is_ok());
    }
}
