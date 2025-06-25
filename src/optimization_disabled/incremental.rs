// Incremental compilation support for faster build times

use crate::error::{CursedError, Result};
pub use crate::optimization::metrics::CompilationUnit;
use crate::optimization::dependency_analyzer::{DependencyGraph, DependencyAnalyzer};

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, Duration};
use std::path::{Path, PathBuf};
use tracing::{info, debug, warn, instrument};
use serde::{Deserialize, Serialize};

/// Incremental compilation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalState {
    pub last_build_time: SystemTime,
    pub file_timestamps: HashMap<PathBuf, SystemTime>,
    pub unit_hashes: HashMap<String, String>,
    pub dependency_graph: HashMap<String, Vec<String>>,
    pub build_artifacts: HashMap<String, PathBuf>,
}

impl Default for IncrementalState {
    fn default() -> Self {
        Self {
            last_build_time: SystemTime::UNIX_EPOCH,
            file_timestamps: HashMap::new(),
            unit_hashes: HashMap::new(),
            dependency_graph: HashMap::new(),
            build_artifacts: HashMap::new(),
        }
    }
}

/// Change detection result
#[derive(Debug, Clone)]
pub struct ChangeDetectionResult {
    pub changed_files: HashSet<PathBuf>,
    pub affected_units: HashSet<String>,
    pub unchanged_units: HashSet<String>,
    pub new_units: HashSet<String>,
    pub removed_units: HashSet<String>,
}

/// Incremental build plan
#[derive(Debug, Clone)]
pub struct IncrementalBuildPlan {
    pub units_to_compile: Vec<String>,
    pub units_to_skip: Vec<String>,
    pub compilation_order: Vec<String>,
    pub estimated_time_savings: Duration,
    pub estimated_units_saved: usize,
}

/// Configuration for incremental compilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalConfig {
    pub enable_incremental: bool,
    pub state_file_path: PathBuf,
    pub force_full_rebuild_interval_hours: u64,
    pub max_dependency_depth: usize,
    pub enable_fine_grained_tracking: bool,
    pub parallelism_level: usize,
}

impl Default for IncrementalConfig {
    fn default() -> Self {
        Self {
            enable_incremental: true,
            state_file_path: PathBuf::from(".cursed_incremental_state.json"),
            force_full_rebuild_interval_hours: 24 * 7, // Weekly full rebuild
            max_dependency_depth: 100,
            enable_fine_grained_tracking: true,
            parallelism_level: num_cpus::get(),
        }
    }
}

/// Incremental compilation manager
#[derive(Debug)]
pub struct IncrementalCompiler {
    config: IncrementalConfig,
    state: IncrementalState,
    dependency_analyzer: DependencyAnalyzer,
    statistics: IncrementalStatistics,
}

/// Statistics for incremental compilation
#[derive(Debug, Default, Clone)]
pub struct IncrementalStatistics {
    pub total_builds: usize,
    pub incremental_builds: usize,
    pub full_builds: usize,
    pub units_skipped_total: usize,
    pub time_saved_total: Duration,
    pub average_time_savings_percent: f64,
    pub cache_hit_rate: f64,
}

impl IncrementalCompiler {
    /// Create a new incremental compiler
    #[instrument]
    pub fn new(config: IncrementalConfig) -> Result<Self> {
        info!("Creating incremental compiler");

        let dependency_analyzer = DependencyAnalyzer::new()?;
        let state = Self::load_state(&config.state_file_path)?;

        Ok(Self {
            config,
            state,
            dependency_analyzer,
            statistics: IncrementalStatistics::default(),
        })
    }

    /// Analyze changes since last build and create incremental build plan
    #[instrument(skip(self, units))]
    pub fn analyze_changes(&mut self, units: &[CompilationUnit]) -> Result<IncrementalBuildPlan> {
        if !self.config.enable_incremental {
            return self.create_full_build_plan(units);
        }

        debug!("Analyzing changes for {} compilation units", units.len());

        // Detect file changes
        let change_result = self.detect_changes(units)?;
        
        // Build dependency graph
        let dependency_graph = self.dependency_analyzer.analyze_dependencies(units)?;
        
        // Determine units that need compilation
        let units_to_compile = self.compute_units_to_compile(&change_result, &dependency_graph)?;
        let units_to_skip: Vec<String> = units.iter()
            .map(|u| u.name.clone())
            .filter(|name| !units_to_compile.contains(name))
            .collect();

        // Determine compilation order
        let compilation_order = self.compute_compilation_order(&units_to_compile, &dependency_graph)?;

        // Estimate time savings
        let estimated_time_savings = self.estimate_time_savings(&units_to_skip, units)?;
        let estimated_units_saved = units_to_skip.len();

        let plan = IncrementalBuildPlan {
            units_to_compile,
            units_to_skip,
            compilation_order,
            estimated_time_savings,
            estimated_units_saved,
        };

        info!(
            "Incremental build plan: {} units to compile, {} units to skip, estimated time savings: {:.2?}",
            plan.units_to_compile.len(),
            plan.units_to_skip.len(),
            plan.estimated_time_savings
        );

        Ok(plan)
    }

    /// Execute incremental compilation based on build plan
    #[instrument(skip(self, plan, units))]
    pub fn execute_incremental_build(
        &mut self,
        plan: &IncrementalBuildPlan,
        units: &mut [CompilationUnit],
    ) -> Result<IncrementalBuildResult> {
        let start_time = std::time::Instant::now();
        
        info!("Executing incremental build for {} units", plan.units_to_compile.len());

        let mut compiled_units = Vec::new();
        let mut skipped_units = Vec::new();
        let mut errors = Vec::new();

        // Mark start of build
        self.state.last_build_time = SystemTime::now();

        // Compile units in dependency order
        for unit_name in &plan.compilation_order {
            if let Some(unit) = units.iter_mut().find(|u| u.name == *unit_name) {
                match self.compile_unit_incrementally(unit) {
                    Ok(result) => {
                        compiled_units.push(result);
                        // Update state for this unit
                        self.update_unit_state(unit)?;
                    }
                    Err(e) => {
                        errors.push(format!("Failed to compile {}: {}", unit_name, e));
                    }
                }
            }
        }

        // Track skipped units
        for unit_name in &plan.units_to_skip {
            if let Some(unit) = units.iter().find(|u| u.name == *unit_name) {
                skipped_units.push(IncrementalUnitResult {
                    unit_name: unit.name.clone(),
                    was_compiled: false,
                    compilation_time: Duration::from_secs(0),
                    cache_hit: true,
                    size_bytes: unit.estimated_size_bytes,
                });
            }
        }

        let total_time = start_time.elapsed();

        // Update statistics
        self.update_statistics(&plan, total_time);

        // Save state
        self.save_state()?;

        Ok(IncrementalBuildResult {
            compiled_units,
            skipped_units,
            total_time,
            time_saved: plan.estimated_time_savings,
            errors,
        })
    }

    /// Detect changes since last build
    #[instrument(skip(self, units))]
    fn detect_changes(&mut self, units: &[CompilationUnit]) -> Result<ChangeDetectionResult> {
        let mut changed_files = HashSet::new();
        let mut affected_units = HashSet::new();
        let mut unchanged_units = HashSet::new();
        let mut new_units = HashSet::new();
        let mut removed_units = HashSet::new();

        // Check for removed units
        for existing_unit in self.state.unit_hashes.keys() {
            if !units.iter().any(|u| u.name == *existing_unit) {
                removed_units.insert(existing_unit.clone());
            }
        }

        // Check each unit for changes
        for unit in units {
            let current_hash = self.compute_unit_hash(unit)?;
            
            if let Some(previous_hash) = self.state.unit_hashes.get(&unit.name) {
                if current_hash != *previous_hash {
                    affected_units.insert(unit.name.clone());
                    
                    // Check which specific files changed
                    for file_path in &unit.source_files {
                        let path = PathBuf::from(file_path);
                        if self.has_file_changed(&path)? {
                            changed_files.insert(path);
                        }
                    }
                } else {
                    unchanged_units.insert(unit.name.clone());
                }
            } else {
                // New unit
                new_units.insert(unit.name.clone());
            }
        }

        debug!(
            "Change detection: {} changed files, {} affected units, {} unchanged units, {} new units, {} removed units",
            changed_files.len(),
            affected_units.len(),
            unchanged_units.len(),
            new_units.len(),
            removed_units.len()
        );

        Ok(ChangeDetectionResult {
            changed_files,
            affected_units,
            unchanged_units,
            new_units,
            removed_units,
        })
    }

    /// Compute which units need compilation based on changes and dependencies
    fn compute_units_to_compile(
        &self,
        changes: &ChangeDetectionResult,
        dependency_graph: &DependencyGraph,
    ) -> Result<Vec<String>> {
        let mut units_to_compile = HashSet::new();

        // Add directly changed units
        units_to_compile.extend(changes.affected_units.iter().cloned());
        units_to_compile.extend(changes.new_units.iter().cloned());

        // Add transitively affected units through dependencies
        let mut work_queue: Vec<String> = units_to_compile.iter().cloned().collect();
        
        while let Some(current_unit) = work_queue.pop() {
            // Find all units that depend on the current unit
            for (unit_name, dependencies) in dependency_graph.get_dependencies() {
                if dependencies.contains(&current_unit) && !units_to_compile.contains(unit_name) {
                    units_to_compile.insert(unit_name.clone());
                    work_queue.push(unit_name.clone());
                }
            }
        }

        // Check if we should force a full rebuild
        if self.should_force_full_rebuild()? {
            warn!("Forcing full rebuild due to configured interval");
            units_to_compile.extend(changes.unchanged_units.iter().cloned());
        }

        Ok(units_to_compile.into_iter().collect())
    }

    /// Compute compilation order respecting dependencies
    fn compute_compilation_order(
        &self,
        units_to_compile: &[String],
        dependency_graph: &DependencyGraph,
    ) -> Result<Vec<String>> {
        // Topological sort of units to compile
        dependency_graph.topological_sort(units_to_compile)
    }

    /// Estimate time savings from incremental compilation
    fn estimate_time_savings(
        &self,
        units_to_skip: &[String],
        all_units: &[CompilationUnit],
    ) -> Result<Duration> {
        let mut total_estimated_time = Duration::from_secs(0);

        for unit_name in units_to_skip {
            if let Some(unit) = all_units.iter().find(|u| u.name == *unit_name) {
                // Estimate compilation time based on unit complexity
                let estimated_time = self.estimate_unit_compilation_time(unit);
                total_estimated_time += estimated_time;
            }
        }

        Ok(total_estimated_time)
    }

    /// Estimate compilation time for a unit
    fn estimate_unit_compilation_time(&self, unit: &CompilationUnit) -> Duration {
        // Base time estimation based on source files and size
        let base_time_ms = 100 + (unit.source_files.len() * 50) + (unit.estimated_size_bytes / 1000);
        Duration::from_millis(base_time_ms as u64)
    }

    /// Compile a single unit incrementally
    #[instrument(skip(self, unit))]
    fn compile_unit_incrementally(&self, unit: &mut CompilationUnit) -> Result<IncrementalUnitResult> {
        let start_time = std::time::Instant::now();
        
        debug!("Compiling unit incrementally: {}", unit.name);

        // Simulate compilation work
        std::thread::sleep(Duration::from_millis(50));

        let compilation_time = start_time.elapsed();

        Ok(IncrementalUnitResult {
            unit_name: unit.name.clone(),
            was_compiled: true,
            compilation_time,
            cache_hit: false,
            size_bytes: unit.estimated_size_bytes,
        })
    }

    /// Update state for a compiled unit
    fn update_unit_state(&mut self, unit: &CompilationUnit) -> Result<()> {
        // Update unit hash
        let hash = self.compute_unit_hash(unit)?;
        self.state.unit_hashes.insert(unit.name.clone(), hash);

        // Update file timestamps
        for file_path in &unit.source_files {
            let path = PathBuf::from(file_path);
            if let Ok(metadata) = std::fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    self.state.file_timestamps.insert(path, modified);
                }
            }
        }

        Ok(())
    }

    /// Compute hash for a compilation unit
    fn compute_unit_hash(&self, unit: &CompilationUnit) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        
        // Hash unit metadata
        unit.name.hash(&mut hasher);
        unit.dependencies.hash(&mut hasher);
        
        // Hash file contents and timestamps
        for file_path in &unit.source_files {
            let path = PathBuf::from(file_path);
            
            // Hash file path
            file_path.hash(&mut hasher);
            
            // Hash file content if it exists
            if let Ok(content) = std::fs::read_to_string(&path) {
                content.hash(&mut hasher);
            }
            
            // Hash file modification time
            if let Ok(metadata) = std::fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                        duration.as_secs().hash(&mut hasher);
                    }
                }
            }
        }
        
        Ok(format!("{:x}", hasher.finish()))
    }

    /// Simple hash function
    fn simple_hash(&self, data: &[u8]) -> u64 {
        let mut hash = 0u64;
        for &byte in data {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }

    /// Check if a file has changed since last build
    fn has_file_changed(&self, file_path: &Path) -> Result<bool> {
        // Check if file exists
        if !file_path.exists() {
            debug!("File does not exist: {:?}", file_path);
            return Ok(true);
        }
        
        // Get current file metadata
        let metadata = std::fs::metadata(file_path).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to read file metadata for {:?}: {}", file_path, e))
        })?;
        
        let current_modified = metadata.modified().map_err(|e| {
            CursedError::optimization_error(&format!("Failed to get modification time for {:?}: {}", file_path, e))
        })?;
        
        // Check against stored timestamp
        if let Some(last_timestamp) = self.state.file_timestamps.get(file_path) {
            let changed = current_modified > *last_timestamp;
            if changed {
                debug!("File changed: {:?} (current: {:?}, last: {:?})", 
                       file_path, current_modified, last_timestamp);
            }
            Ok(changed)
        } else {
            // No previous timestamp recorded, assume changed
            debug!("No previous timestamp for file: {:?}", file_path);
            Ok(true)
        }
    }

    /// Check if a full rebuild should be forced
    fn should_force_full_rebuild(&self) -> Result<bool> {
        let force_interval = Duration::from_secs(self.config.force_full_rebuild_interval_hours * 3600);
        let time_since_last_full = self.state.last_build_time.elapsed().unwrap_or(Duration::MAX);
        
        Ok(time_since_last_full > force_interval)
    }

    /// Create a full build plan (no incremental compilation)
    fn create_full_build_plan(&self, units: &[CompilationUnit]) -> Result<IncrementalBuildPlan> {
        let units_to_compile: Vec<String> = units.iter().map(|u| u.name.clone()).collect();
        
        Ok(IncrementalBuildPlan {
            compilation_order: units_to_compile.clone(),
            units_to_compile,
            units_to_skip: Vec::new(),
            estimated_time_savings: Duration::from_secs(0),
            estimated_units_saved: 0,
        })
    }

    /// Update compilation statistics
    fn update_statistics(&mut self, plan: &IncrementalBuildPlan, total_time: Duration) {
        self.statistics.total_builds += 1;
        
        if plan.units_to_skip.is_empty() {
            self.statistics.full_builds += 1;
        } else {
            self.statistics.incremental_builds += 1;
            self.statistics.units_skipped_total += plan.units_to_skip.len();
            self.statistics.time_saved_total += plan.estimated_time_savings;
        }

        // Update average time savings
        if self.statistics.incremental_builds > 0 {
            let total_savings_secs = self.statistics.time_saved_total.as_secs_f64();
            let estimated_full_time = total_savings_secs + total_time.as_secs_f64();
            if estimated_full_time > 0.0 {
                self.statistics.average_time_savings_percent = 
                    (total_savings_secs / estimated_full_time) * 100.0;
            }
        }
    }

    /// Load incremental state from disk
    fn load_state(state_file_path: &Path) -> Result<IncrementalState> {
        if state_file_path.exists() {
            let data = std::fs::read_to_string(state_file_path).map_err(|e| {
                CursedError::optimization_error(&format!("Failed to read incremental state: {}", e))
            })?;

            serde_json::from_str(&data).map_err(|e| {
                CursedError::optimization_error(&format!("Failed to parse incremental state: {}", e))
            })
        } else {
            Ok(IncrementalState::default())
        }
    }

    /// Save incremental state to disk
    fn save_state(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self.state).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to serialize incremental state: {}", e))
        })?;

        std::fs::write(&self.config.state_file_path, data).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to write incremental state: {}", e))
        })?;

        Ok(())
    }

    /// Get current statistics
    pub fn get_statistics(&self) -> &IncrementalStatistics {
        &self.statistics
    }

    /// Reset incremental state (force full rebuild next time)
    pub fn reset_state(&mut self) -> Result<()> {
        info!("Resetting incremental compilation state");
        self.state = IncrementalState::default();
        self.save_state()?;
        Ok(())
    }

    /// Update configuration
    pub fn update_config(&mut self, new_config: IncrementalConfig) -> Result<()> {
        info!("Updating incremental compiler configuration");
        self.config = new_config;
        Ok(())
    }
}

/// Result from an incremental unit compilation
#[derive(Debug, Clone)]
pub struct IncrementalUnitResult {
    pub unit_name: String,
    pub was_compiled: bool,
    pub compilation_time: Duration,
    pub cache_hit: bool,
    pub size_bytes: usize,
}

/// Result from an incremental build
#[derive(Debug)]
pub struct IncrementalBuildResult {
    pub compiled_units: Vec<IncrementalUnitResult>,
    pub skipped_units: Vec<IncrementalUnitResult>,
    pub total_time: Duration,
    pub time_saved: Duration,
    pub errors: Vec<String>,
}

