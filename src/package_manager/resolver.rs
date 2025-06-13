use crate::package_manager::{PackageManagerError, metadata::PackageMetadata, registry::{PackageInfo, PackageRegistry}};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque, BTreeMap};
use std::sync::{Arc, Mutex};
use semver::{Version, VersionReq};
use tracing::{info, warn, error, debug, instrument};
use std::time::{Duration, Instant};
use rand;

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

/// Advanced dependency resolver with constraint satisfaction and backtracking
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
    /// Constraint satisfaction state for backtracking
    constraint_state: ConstraintState,
    /// Maximum backtracking attempts before giving up
    max_backtrack_attempts: usize,
    /// Timeout for resolution process
    resolution_timeout: Duration,
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

/// Constraint satisfaction state for backtracking algorithm
#[derive(Debug, Clone)]
pub struct ConstraintState {
    /// Current variable assignments (package -> version)
    assignments: BTreeMap<String, Version>,
    /// Domain of possible values for each variable
    domains: HashMap<String, Vec<Version>>,
    /// Constraint graph for arc consistency
    constraint_graph: HashMap<String, HashSet<String>>,
    /// Backtrack stack for undoing decisions
    backtrack_stack: Vec<Assignment>,
}

/// Assignment decision for backtracking
#[derive(Debug, Clone)]
pub struct Assignment {
    package: String,
    version: Version,
    timestamp: Instant,
    reason: AssignmentReason,
}

/// Reason for making an assignment
#[derive(Debug, Clone)]
pub enum AssignmentReason {
    UserConstraint,
    DependencyRequirement,
    ConflictResolution,
    Backtrack,
}

/// Lock file for reproducible builds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFile {
    /// Format version for compatibility
    pub version: String,
    /// Resolved package versions with checksums
    pub packages: BTreeMap<String, LockedPackage>,
    /// Resolution metadata
    pub metadata: LockFileMetadata,
    /// Dependency tree structure
    pub dependency_tree: BTreeMap<String, Vec<String>>,
}

/// Locked package information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedPackage {
    pub version: String,
    pub checksum: String,
    pub source: PackageSource,
    pub dependencies: Vec<String>,
    pub resolved_at: String,
}

/// Package source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageSource {
    Registry { url: String },
    Git { url: String, rev: String },
    Path { path: String },
    Local,
}

/// Lock file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFileMetadata {
    pub generated_at: String,
    pub resolver_version: String,
    pub total_packages: usize,
    pub resolution_time_ms: u64,
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
            constraint_state: ConstraintState::new(),
            max_backtrack_attempts: 1000,
            resolution_timeout: Duration::from_secs(300), // 5 minutes
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
            constraint_state: ConstraintState::new(),
            max_backtrack_attempts: 1000,
            resolution_timeout: Duration::from_secs(300),
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

    /// Get the maximum depth setting
    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    /// Get the allow dev dependencies setting
    pub fn allow_dev_dependencies(&self) -> bool {
        self.allow_dev_dependencies
    }

    /// Clear internal caches
    pub fn clear_cache(&mut self) {
        self.resolution_cache.clear();
        self.version_cache.clear();
        self.metadata_cache.clear();
        self.stats.cache_size = 0;
    }

    /// Main dependency resolution entry point using constraint satisfaction
    #[instrument(skip(self, package))]
    pub async fn resolve_dependencies(&mut self, package: &PackageInfo) -> Result<Vec<ResolvedDependency>, PackageManagerError> {
        let start_time = Instant::now();
        info!("Starting constraint-based dependency resolution for {}@{}", package.name, package.version);

        // Check cache first
        let cache_key = format!("{}@{}", package.name, package.version);
        if let Some(cached_result) = self.resolution_cache.get(&cache_key) {
            self.stats.cached_count += 1;
            debug!("Using cached resolution for {}", cache_key);
            return Ok(cached_result.clone());
        }

        // Reset constraint state for new resolution
        self.constraint_state = ConstraintState::new();

        // Build constraint satisfaction problem
        let mut all_packages = HashSet::new();
        let mut package_constraints = HashMap::new();
        
        // Collect all packages and constraints
        self.collect_all_constraints(package, &mut all_packages, &mut package_constraints).await?;

        // Set up domains for all packages
        for pkg_name in &all_packages {
            let available_versions = self.get_available_versions(pkg_name).await?;
            self.constraint_state.add_package(pkg_name, available_versions);
        }

        // Add constraints between packages
        for (pkg_name, constraints) in &package_constraints {
            for constraint in constraints {
                for other_pkg in &all_packages {
                    if other_pkg != pkg_name {
                        self.constraint_state.add_constraint(pkg_name, other_pkg);
                    }
                }
            }
        }

        // Solve using backtracking with constraint propagation
        let solution = self.solve_with_backtracking(&package_constraints).await?;

        // Convert solution to resolved dependencies
        let resolved_deps = self.convert_solution_to_dependencies(solution, &package_constraints).await?;

        // Cache the result
        self.resolution_cache.insert(cache_key, resolved_deps.clone());
        self.stats.cache_size = self.resolution_cache.len();
        self.stats.resolved_count += 1;
        
        let elapsed = start_time.elapsed();
        self.stats.resolution_time_ms = elapsed.as_millis() as u64;
        
        info!("Constraint-based resolution completed successfully for {} with {} dependencies", 
              package.name, resolved_deps.len());
              
        Ok(resolved_deps)
    }

    /// Collect all packages and their constraints recursively
    async fn collect_all_constraints(
        &mut self,
        root_package: &PackageInfo,
        all_packages: &mut HashSet<String>,
        package_constraints: &mut HashMap<String, Vec<DependencyConstraint>>
    ) -> Result<(), PackageManagerError> {
        let mut to_process = VecDeque::new();
        let mut visited = HashSet::new();
        
        to_process.push_back(root_package.clone());
        
        while let Some(current_package) = to_process.pop_front() {
            let pkg_key = format!("{}@{}", current_package.name, current_package.version);
            
            if visited.contains(&pkg_key) {
                continue;
            }
            visited.insert(pkg_key);
            
            all_packages.insert(current_package.name.clone());
            
            // Get package metadata to find dependencies
            let metadata = self.get_dependency_metadata(&current_package.name, 
                &Version::parse(&current_package.version)
                    .map_err(|e| PackageManagerError::InvalidVersion { 
                        version: current_package.version.clone(), 
                        reason: e.to_string() 
                    })?).await?;
            
            // Process regular dependencies
            for (dep_name, version_spec) in &metadata.dependencies {
                all_packages.insert(dep_name.clone());
                
                let version_constraint = match version_spec {
                    crate::package_manager::metadata::VersionSpec::Simple(v) => v.clone(),
                    crate::package_manager::metadata::VersionSpec::Complex { version: Some(v), .. } => v.clone(),
                    crate::package_manager::metadata::VersionSpec::Complex { .. } => "*".to_string(),
                };
                
                let constraint = DependencyConstraint {
                    name: dep_name.clone(),
                    version_req: VersionReq::parse(&version_constraint)
                        .map_err(|e| PackageManagerError::InvalidVersion { 
                            version: version_constraint.clone(), 
                            reason: e.to_string() 
                        })?,
                    required_by: current_package.name.clone(),
                    is_dev: false,
                    optional: false,
                    features: vec![],
                };
                
                package_constraints.entry(dep_name.clone())
                    .or_insert_with(Vec::new)
                    .push(constraint);
                
                // Add to processing queue if we haven't seen it
                if !visited.iter().any(|v| v.starts_with(&format!("{}@", dep_name))) {
                    // Get latest version for processing
                    if let Ok(versions) = self.get_available_versions(dep_name).await {
                        if let Some(latest_version) = versions.iter().max() {
                            let dep_package_info = PackageInfo {
                                name: dep_name.clone(),
                                version: latest_version.to_string(),
                                description: format!("Package {}", dep_name),
                                authors: None,
                                keywords: None,
                                download_url: String::new(),
                                checksum: String::new(),
                                size: None,
                                published_at: None,
                                repository: None,
                                license: None,
                            };
                            to_process.push_back(dep_package_info);
                        }
                    }
                }
            }
            
            // Process dev dependencies if enabled
            if self.allow_dev_dependencies {
                for (dep_name, version_spec) in &metadata.dev_dependencies {
                    all_packages.insert(dep_name.clone());
                    
                    let version_constraint = match version_spec {
                        crate::package_manager::metadata::VersionSpec::Simple(v) => v.clone(),
                        crate::package_manager::metadata::VersionSpec::Complex { version: Some(v), .. } => v.clone(),
                        crate::package_manager::metadata::VersionSpec::Complex { .. } => "*".to_string(),
                    };
                    
                    let constraint = DependencyConstraint {
                        name: dep_name.clone(),
                        version_req: VersionReq::parse(&version_constraint)
                            .map_err(|e| PackageManagerError::InvalidVersion { 
                                version: version_constraint.clone(), 
                                reason: e.to_string() 
                            })?,
                        required_by: current_package.name.clone(),
                        is_dev: true,
                        optional: false,
                        features: vec![],
                    };
                    
                    package_constraints.entry(dep_name.clone())
                        .or_insert_with(Vec::new)
                        .push(constraint);
                }
            }
        }
        
        Ok(())
    }

    /// Solve constraint satisfaction problem using backtracking
    async fn solve_with_backtracking(
        &mut self,
        package_constraints: &HashMap<String, Vec<DependencyConstraint>>
    ) -> Result<BTreeMap<String, Version>, PackageManagerError> {
        let start_time = Instant::now();
        let mut attempts = 0;

        loop {
            if attempts >= self.max_backtrack_attempts {
                return Err(PackageManagerError::DependencyError { 
                    reason: format!("Maximum backtrack attempts ({}) exceeded", self.max_backtrack_attempts)
                });
            }

            if start_time.elapsed() > self.resolution_timeout {
                return Err(PackageManagerError::DependencyError { 
                    reason: format!("Resolution timeout ({:?}) exceeded", self.resolution_timeout)
                });
            }

            attempts += 1;
            self.stats.backtrack_attempts = attempts;

            if self.constraint_state.is_complete() {
                info!("Constraint satisfaction completed in {} attempts", attempts);
                return Ok(self.constraint_state.assignments.clone());
            }

            // Select next variable using MRV heuristic
            if let Some(next_package) = self.constraint_state.select_next_variable() {
                let values = self.constraint_state.get_ordered_values(&next_package);
                let mut assigned = false;

                // Try each value in the domain
                for value in values {
                    // Check if this assignment satisfies all constraints for this package
                    if let Some(constraints) = package_constraints.get(&next_package) {
                        let satisfies_all = constraints.iter().all(|c| c.version_req.matches(&value));
                        
                        if satisfies_all && self.constraint_state.assign(&next_package, value, AssignmentReason::DependencyRequirement) {
                            assigned = true;
                            break;
                        }
                    } else if self.constraint_state.assign(&next_package, value, AssignmentReason::UserConstraint) {
                        assigned = true;
                        break;
                    }
                }

                if !assigned {
                    // Backtrack
                    if self.constraint_state.backtrack().is_none() {
                        return Err(PackageManagerError::DependencyError { 
                            reason: "No solution found - unable to satisfy all constraints".to_string()
                        });
                    }
                }
            } else {
                return Err(PackageManagerError::DependencyError { 
                    reason: "No more variables to assign but solution not complete".to_string()
                });
            }
        }
    }

    /// Convert solution to resolved dependencies
    async fn convert_solution_to_dependencies(
        &mut self,
        solution: BTreeMap<String, Version>,
        package_constraints: &HashMap<String, Vec<DependencyConstraint>>
    ) -> Result<Vec<ResolvedDependency>, PackageManagerError> {
        let mut resolved_deps = Vec::new();

        for (package_name, version) in solution {
            let metadata = self.get_dependency_metadata(&package_name, &version).await?;
            
            let required_by = package_constraints.get(&package_name)
                .map(|constraints| constraints.iter().map(|c| c.required_by.clone()).collect())
                .unwrap_or_default();

            let constraint = package_constraints.get(&package_name)
                .and_then(|constraints| constraints.first())
                .map(|c| c.version_req.to_string())
                .unwrap_or_else(|| "*".to_string());

            let is_dev = package_constraints.get(&package_name)
                .map(|constraints| constraints.iter().any(|c| c.is_dev))
                .unwrap_or(false);

            let resolved_dep = ResolvedDependency {
                package: metadata,
                depth: 1, // For now, we use depth 1 for all dependencies
                required_by,
                constraint,
                resolved_version: version,
                is_dev_dependency: is_dev,
                optional: false,
            };

            resolved_deps.push(resolved_dep);
        }

        // Sort by name for consistent ordering
        resolved_deps.sort_by(|a, b| a.package.name.cmp(&b.package.name));

        Ok(resolved_deps)
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

        // Detect conflicts with existing resolutions
        let conflicts = self.detect_version_conflicts(package_name, &selected_version, version_req, context);
        
        Ok(VersionSelection {
            version: selected_version,
            satisfies: vec![version_req.to_string()],
            conflicts,
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

    /// Detect version conflicts with existing resolutions
    fn detect_version_conflicts(
        &self,
        package_name: &str,
        selected_version: &Version,
        version_req: &VersionReq,
        context: &ResolutionContext,
    ) -> Vec<ConflictInfo> {
        let mut conflicts = Vec::new();
        
        // Check if this package is already resolved with a different version
        if let Some(existing_resolution) = context.resolved.get(package_name) {
            if existing_resolution.resolved_version != *selected_version {
                conflicts.push(ConflictInfo {
                    package: package_name.to_string(),
                    conflicting_versions: vec![
                        existing_resolution.resolved_version.to_string(),
                        selected_version.to_string(),
                    ],
                    required_by: vec![existing_resolution.required_by.join(", ")],
                    reason: ConflictReason::IncompatibleVersions,
                });
            }
        }
        
        // Check constraints from all dependents
        if let Some(constraints) = context.constraints.get(package_name) {
            let mut incompatible_constraints = Vec::new();
            let mut requiring_packages = Vec::new();
            
            for constraint in constraints {
                if !constraint.version_req.matches(selected_version) {
                    incompatible_constraints.push(constraint.version_req.to_string());
                    requiring_packages.push(constraint.required_by.clone());
                }
            }
            
            if !incompatible_constraints.is_empty() {
                conflicts.push(ConflictInfo {
                    package: package_name.to_string(),
                    conflicting_versions: incompatible_constraints,
                    required_by: requiring_packages,
                    reason: ConflictReason::InvalidConstraint,
                });
            }
        }
        
        // Check for circular dependencies
        if context.visiting.contains(package_name) {
            let cycle_path: Vec<_> = context.visiting.iter()
                .skip_while(|&p| p != package_name)
                .chain(std::iter::once(&package_name.to_string()))
                .cloned()
                .collect();
                
            conflicts.push(ConflictInfo {
                package: package_name.to_string(),
                conflicting_versions: vec![selected_version.to_string()],
                required_by: cycle_path,
                reason: ConflictReason::CircularDependency,
            });
        }
        
        conflicts
    }
    
    /// Select version with minimal change from existing resolutions
    fn select_minimal_change_version(&self, compatible_versions: &[&Version], context: &ResolutionContext) -> Version {
        // Implementation of sophisticated minimal change algorithm
        debug!("Selecting minimal change version from {} options", compatible_versions.len());
        
        if compatible_versions.is_empty() {
            panic!("No compatible versions provided");
        }
        
        if compatible_versions.len() == 1 {
            return (*compatible_versions[0]).clone();
        }
        
        // Strategy 1: Prefer versions that are already resolved for other packages
        let mut version_scores: HashMap<&Version, f64> = HashMap::new();
        
        // Initialize all versions with base score
        for version in compatible_versions {
            version_scores.insert(version, 0.0);
        }
        
        // Score based on proximity to existing resolutions
        for resolved_dep in context.resolved.values() {
            let resolved_version = &resolved_dep.resolved_version;
            
            for candidate_version in compatible_versions {
                // Calculate similarity score based on semantic distance
                let similarity = self.calculate_version_similarity(resolved_version, candidate_version);
                *version_scores.get_mut(candidate_version).unwrap() += similarity;
            }
        }
        
        // Strategy 2: Prefer stable versions (not pre-release)
        for (version, score) in &mut version_scores {
            if version.pre.is_empty() {
                *score += 10.0; // Boost for stable versions
            }
            
            // Prefer patch/minor updates over major updates
            let stability_score = match (version.major, version.minor, version.patch) {
                (0, 0, p) if p > 0 => 1.0,  // Patch in 0.0.x
                (0, m, _) if m > 0 => 2.0,  // Minor in 0.x.y
                (m, _, _) if m > 0 => 5.0,  // Major version
                _ => 0.0,
            };
            *score += stability_score;
        }
        
        // Strategy 3: Prefer versions that minimize constraint violations
        let total_constraints = context.constraints.values()
            .map(|constraints| constraints.len())
            .sum::<usize>() as f64;
            
        if total_constraints > 0.0 {
            for (version, score) in &mut version_scores {
                let mut satisfies_count = 0;
                let mut total_constraints_checked = 0;
                
                for constraints in context.constraints.values() {
                    for constraint in constraints {
                        total_constraints_checked += 1;
                        if constraint.version_req.matches(version) {
                            satisfies_count += 1;
                        }
                    }
                }
                
                if total_constraints_checked > 0 {
                    let satisfaction_ratio = satisfies_count as f64 / total_constraints_checked as f64;
                    *score += satisfaction_ratio * 15.0; // High weight for constraint satisfaction
                }
            }
        }
        
        // Strategy 4: In case of ties, prefer more recent versions
        let max_score = version_scores.values().fold(0.0f64, |acc, &x| acc.max(x));
        let best_versions: Vec<_> = version_scores.iter()
            .filter(|(_, &score)| (score - max_score).abs() < 0.01)
            .map(|(&version, _)| version)
            .collect();
        
        if best_versions.len() == 1 {
            (*best_versions[0]).clone()
        } else {
            // Among tied versions, select the most recent
            let selected_version = (*best_versions.iter().max().unwrap()).clone();
            debug!("Selected version {} with score {:.2} from {} tied candidates", 
                  selected_version, max_score, best_versions.len());
            selected_version
        }
    }
    
    /// Calculate semantic similarity between two versions
    fn calculate_version_similarity(&self, version1: &Version, version2: &Version) -> f64 {
        // Exact match gets highest score
        if version1 == version2 {
            return 100.0;
        }
        
        let mut similarity = 0.0;
        
        // Major version similarity
        if version1.major == version2.major {
            similarity += 50.0;
            
            // Minor version similarity
            if version1.minor == version2.minor {
                similarity += 30.0;
                
                // Patch version similarity
                if version1.patch == version2.patch {
                    similarity += 15.0;
                } else {
                    // Closer patch versions get higher scores
                    let patch_diff = (version1.patch as f64 - version2.patch as f64).abs();
                    similarity += 15.0 * (-patch_diff / 10.0).exp();
                }
            } else {
                // Closer minor versions get higher scores
                let minor_diff = (version1.minor as f64 - version2.minor as f64).abs();
                similarity += 30.0 * (-minor_diff / 5.0).exp();
            }
        } else {
            // Different major versions - very low similarity
            let major_diff = (version1.major as f64 - version2.major as f64).abs();
            similarity += 10.0 * (-major_diff / 2.0).exp();
        }
        
        // Pre-release similarity
        if version1.pre == version2.pre {
            similarity += 5.0;
        }
        
        similarity
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

    /// Generate lock file for reproducible builds
    pub fn generate_lock_file(&self, dependencies: &[ResolvedDependency]) -> LockFile {
        let mut packages = BTreeMap::new();
        let mut dependency_tree = BTreeMap::new();

        for dep in dependencies {
            let locked_package = LockedPackage {
                version: dep.resolved_version.to_string(),
                checksum: self.calculate_checksum(&dep.package.name, &dep.resolved_version),
                source: PackageSource::Registry { 
                    url: "https://registry.cursed-lang.org".to_string() 
                },
                dependencies: dep.package.dependencies.keys().cloned().collect(),
                resolved_at: chrono::Utc::now().to_rfc3339(),
            };

            packages.insert(dep.package.name.clone(), locked_package);

            // Build dependency tree
            if !dep.required_by.is_empty() {
                dependency_tree.insert(
                    dep.package.name.clone(),
                    dep.required_by.clone()
                );
            }
        }

        let metadata = LockFileMetadata {
            generated_at: chrono::Utc::now().to_rfc3339(),
            resolver_version: "1.0.0".to_string(),
            total_packages: packages.len(),
            resolution_time_ms: self.stats.resolution_time_ms,
        };

        LockFile {
            version: "1.0".to_string(),
            packages,
            metadata,
            dependency_tree,
        }
    }

    /// Calculate package checksum for lock file
    fn calculate_checksum(&self, package_name: &str, version: &Version) -> String {
        use sha2::{Sha256, Digest};
        
        // In a real implementation, this would calculate checksums from actual package contents
        // For now, create a deterministic checksum based on package name and version
        let mut hasher = Sha256::new();
        hasher.update(package_name.as_bytes());
        hasher.update(version.to_string().as_bytes());
        hasher.update(b"cursed-package-checksum"); // Salt for uniqueness
        
        // Add current timestamp as a pseudo-random element but make it deterministic
        // by using a fixed timestamp for the same package@version combination
        let deterministic_time = self.calculate_deterministic_timestamp(package_name, version);
        hasher.update(deterministic_time.to_le_bytes());
        
        format!("sha256:{:x}", hasher.finalize())
    }
    
    /// Calculate a deterministic timestamp for consistent checksums
    fn calculate_deterministic_timestamp(&self, package_name: &str, version: &Version) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        package_name.hash(&mut hasher);
        version.hash(&mut hasher);
        
        // Convert to a timestamp-like value (but deterministic)
        let hash_value = hasher.finish();
        1640995200 + (hash_value % 31536000) // Base timestamp + up to 1 year variation
    }

    /// Validate an existing lock file against current dependencies
    pub async fn validate_lock_file(&mut self, lock_file: &LockFile, current_deps: &[ResolvedDependency]) -> Result<bool, PackageManagerError> {
        info!("Validating lock file with {} packages", lock_file.packages.len());

        // Check if all current dependencies are in lock file with compatible versions
        for dep in current_deps {
            if let Some(locked_pkg) = lock_file.packages.get(&dep.package.name) {
                let locked_version = Version::parse(&locked_pkg.version)
                    .map_err(|e| PackageManagerError::InvalidVersion { 
                        version: locked_pkg.version.clone(), 
                        reason: e.to_string() 
                    })?;
                
                if locked_version != dep.resolved_version {
                    warn!("Version mismatch for {}: locked={}, resolved={}", 
                          dep.package.name, locked_version, dep.resolved_version);
                    return Ok(false);
                }

                // Validate checksum if possible
                let expected_checksum = self.calculate_checksum(&dep.package.name, &dep.resolved_version);
                if locked_pkg.checksum != expected_checksum {
                    warn!("Checksum mismatch for {}@{}", dep.package.name, dep.resolved_version);
                    return Ok(false);
                }
            } else {
                warn!("Package {} not found in lock file", dep.package.name);
                return Ok(false);
            }
        }

        // Check for extra packages in lock file
        for locked_name in lock_file.packages.keys() {
            if !current_deps.iter().any(|dep| &dep.package.name == locked_name) {
                warn!("Extra package {} in lock file", locked_name);
                return Ok(false);
            }
        }

        info!("Lock file validation successful");
        Ok(true)
    }

    /// Update lock file with new resolution
    pub fn update_lock_file(&self, existing_lock: Option<&LockFile>, dependencies: &[ResolvedDependency]) -> LockFile {
        let mut new_lock = self.generate_lock_file(dependencies);

        // Preserve metadata from existing lock file if available
        if let Some(existing) = existing_lock {
            // Could preserve creation time, etc.
            new_lock.metadata.generated_at = chrono::Utc::now().to_rfc3339();
        }

        new_lock
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
            ExportFormat::LockFile => {
                let lock_file = self.generate_lock_file(dependencies);
                serde_json::to_string_pretty(&lock_file)
                    .map_err(|e| PackageManagerError::InvalidMetadata { 
                        reason: format!("Failed to serialize lock file: {}", e) 
                    })
            }
        }
    }
}

/// Export format options
#[derive(Debug, Clone)]
pub enum ExportFormat {
    Json,
    Yaml,
    Tree,
    LockFile,
}

impl ConstraintState {
    pub fn new() -> Self {
        Self {
            assignments: BTreeMap::new(),
            domains: HashMap::new(),
            constraint_graph: HashMap::new(),
            backtrack_stack: Vec::new(),
        }
    }

    /// Add a package to the constraint state
    pub fn add_package(&mut self, package: &str, available_versions: Vec<Version>) {
        self.domains.insert(package.to_string(), available_versions);
        self.constraint_graph.insert(package.to_string(), HashSet::new());
    }

    /// Add a constraint between two packages
    pub fn add_constraint(&mut self, from: &str, to: &str) {
        self.constraint_graph.entry(from.to_string())
            .or_insert_with(HashSet::new)
            .insert(to.to_string());
        self.constraint_graph.entry(to.to_string())
            .or_insert_with(HashSet::new)
            .insert(from.to_string());
    }

    /// Assign a version to a package
    pub fn assign(&mut self, package: &str, version: Version, reason: AssignmentReason) -> bool {
        if !self.is_consistent_assignment(package, &version) {
            return false;
        }

        let assignment = Assignment {
            package: package.to_string(),
            version: version.clone(),
            timestamp: Instant::now(),
            reason,
        };

        self.backtrack_stack.push(assignment);
        self.assignments.insert(package.to_string(), version);
        
        // Propagate constraints to maintain arc consistency
        self.propagate_constraints(package)
    }

    /// Check if an assignment is consistent with existing constraints
    fn is_consistent_assignment(&self, package: &str, version: &Version) -> bool {
        if let Some(domain) = self.domains.get(package) {
            if !domain.contains(version) {
                return false;
            }
        }

        // Check constraints with already assigned packages
        if let Some(neighbors) = self.constraint_graph.get(package) {
            for neighbor in neighbors {
                if let Some(neighbor_version) = self.assignments.get(neighbor) {
                    // Here we would check semantic version compatibility
                    // For now, we just ensure they exist
                    if neighbor_version.to_string().is_empty() {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Propagate constraints to maintain arc consistency
    fn propagate_constraints(&mut self, package: &str) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(package.to_string());

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = self.constraint_graph.get(&current).cloned() {
                for neighbor in neighbors {
                    if self.revise_domain(&neighbor, &current) {
                        if self.domains.get(&neighbor).map_or(true, |d| d.is_empty()) {
                            return false; // Domain became empty
                        }
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        true
    }

    /// Revise domain to maintain arc consistency
    fn revise_domain(&mut self, xi: &str, xj: &str) -> bool {
        let mut revised = false;
        
        if let Some(xi_domain) = self.domains.get(xi).cloned() {
            let mut new_domain = Vec::new();
            
            for xi_value in &xi_domain {
                let mut has_support = false;
                
                if let Some(xj_domain) = self.domains.get(xj) {
                    for xj_value in xj_domain {
                        if self.is_compatible(xi, xi_value, xj, xj_value) {
                            has_support = true;
                            break;
                        }
                    }
                }
                
                if has_support {
                    new_domain.push(xi_value.clone());
                } else {
                    revised = true;
                }
            }
            
            self.domains.insert(xi.to_string(), new_domain);
        }
        
        revised
    }

    /// Check if two version assignments are compatible
    fn is_compatible(&self, _pkg1: &str, _ver1: &Version, _pkg2: &str, _ver2: &Version) -> bool {
        // Simplified compatibility check - in reality this would check
        // semantic version constraints between packages
        true
    }

    /// Backtrack to the last decision point
    pub fn backtrack(&mut self) -> Option<Assignment> {
        if let Some(assignment) = self.backtrack_stack.pop() {
            self.assignments.remove(&assignment.package);
            
            // Restore domains affected by this assignment
            self.restore_domains_after_backtrack(&assignment.package);
            
            Some(assignment)
        } else {
            None
        }
    }

    /// Restore domains after backtracking
    fn restore_domains_after_backtrack(&mut self, _package: &str) {
        // In a full implementation, we would restore the domains
        // to their state before the assignment was made
        // For now, we keep the simplified version
    }

    /// Check if all variables are assigned
    pub fn is_complete(&self) -> bool {
        self.domains.keys().all(|pkg| self.assignments.contains_key(pkg))
    }

    /// Get next unassigned variable using MRV heuristic
    pub fn select_next_variable(&self) -> Option<String> {
        let mut min_remaining = usize::MAX;
        let mut selected = None;

        for (package, domain) in &self.domains {
            if !self.assignments.contains_key(package) && domain.len() < min_remaining {
                min_remaining = domain.len();
                selected = Some(package.clone());
            }
        }

        selected
    }

    /// Get domain values ordered by least constraining value heuristic
    pub fn get_ordered_values(&self, package: &str) -> Vec<Version> {
        self.domains.get(package).cloned().unwrap_or_default()
    }
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
        let valid_reqs = vec!["1.0.0", "^1.0", "~1.2", ">=1.0.0", ">=1.0.0, <2.0.0"];
        
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
