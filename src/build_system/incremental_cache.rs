use crate::error::CursedError;
// Incremental Build Cache
// 
// Provides caching mechanisms for incremental builds to speed up
// recompilation by tracking file dependencies and build artifacts.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tracing::{debug, info, instrument, warn};

/// Incremental build cache manager
#[derive(Debug)]
pub struct IncrementalCache {
/// Cache entry for a build target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Target name
    
    /// Timestamp when cached
    
    /// Source file checksums
    
    /// Dependency checksums
    
    /// Output files produced
    
    /// Build artifacts
    
    /// Number of files processed
    
    /// Build profile used
    
    /// CURSED compiler version
/// Cache metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    /// Cache format version
    
    /// Creation time
    
    /// Last cleanup time
    
    /// Total cache entries
    
    /// Total cache size in bytes
/// Cache manager for coordinating multiple caches
#[derive(Debug)]
pub struct CacheManager {
/// Cache error types
#[derive(Debug, thiserror::CursedError)]
pub enum CacheError {
    #[error("IO error: {0}")]
    
    #[error("Serialization error: {0}")]
    
    #[error("Cache corruption: {0}")]
    
    #[error("Cache not found: {0}")]
    
    #[error("Version mismatch: expected {expected}, found {found}")]
impl IncrementalCache {
    /// Create a new incremental cache
    pub fn new(cache_dir: PathBuf) -> crate::error::Result<()> {
        std::fs::create_dir_all(&cache_dir)?;
        
        let metadata_path = cache_dir.join("metadata.json");
        let entries_path = cache_dir.join("entries.json");
        
        let metadata = if metadata_path.exists() {
            let content = std::fs::read_to_string(&metadata_path)?;
            serde_json::from_str(&content)
                .map_err(|e| CacheError::SerializationError(e.to_string()))?
        } else {
            CacheMetadata {
            }
        
        let entries = if entries_path.exists() {
            let content = std::fs::read_to_string(&entries_path)?;
            serde_json::from_str(&content)
                .map_err(|e| CacheError::SerializationError(e.to_string()))?
        } else {
            HashMap::new()
        
        // Validate cache version
        if metadata.version != 1 {
            return Err(CacheError::VersionMismatch {
            });
        Ok(IncrementalCache {
        })
    /// Get cache entry for a target
    #[instrument(skip(self))]
    pub fn get(&self, target_name: &str) -> Option<&CacheEntry> {
        debug!("Looking up cache entry for target: {}", target_name);
        self.entries.get(target_name)
    /// Insert a new cache entry
    #[instrument(skip(self, outputs, artifacts))]
    pub fn insert(
    ) -> crate::error::Result<()> {
        info!("Caching build result for target: {}", target_name);
        
        // Calculate source checksums for validation
        let mut source_checksums = HashMap::new();
        let mut dependency_checksums = HashMap::new();
        
        // For now, we'll use a simple approach - in a real implementation,
        // this would track all source files and dependencies
        if let Ok(current_dir) = std::env::current_dir() {
            let src_dir = current_dir.join("src");
            if src_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(src_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("csd") {
                            match calculate_file_checksum(&path) {
                                Ok(checksum) => {
                                    source_checksums.insert(path, checksum);
                                }
                            }
                        }
                    }
                }
            }
        }

        let entry = CacheEntry {
        
        self.entries.insert(target_name.to_string(), entry);
        self.metadata.entry_count = self.entries.len();
        
        self.save_to_disk()?;
        Ok(())
    /// Remove a cache entry
    #[instrument(skip(self))]
    pub fn remove(&mut self, target_name: &str) -> crate::error::Result<()> {
        debug!("Removing cache entry for target: {}", target_name);
        
        let removed = self.entries.remove(target_name).is_some();
        if removed {
            self.metadata.entry_count = self.entries.len();
            self.save_to_disk()?;
        Ok(removed)
    /// Clear all cache entries
    #[instrument(skip(self))]
    pub fn clear(&mut self) -> crate::error::Result<()> {
        info!("Clearing all cache entries");
        
        self.entries.clear();
        self.metadata.entry_count = 0;
        self.metadata.cache_size = 0;
        
        self.save_to_disk()?;
        Ok(())
    /// Check if target needs rebuilding based on source changes
    #[instrument(skip(self))]
    pub fn needs_rebuild(&self, target_name: &str, source_paths: &[PathBuf]) -> crate::error::Result<()> {
        let entry = match self.get(target_name) {
            None => {
                debug!("No cache entry found for target: {}", target_name);
                return Ok(true);
            }
        
        // Check if any source files have been modified
        for path in source_paths {
            if !path.exists() {
                continue;
            let metadata = std::fs::metadata(path)?;
            let modified = metadata.modified()?;
            
            if modified > entry.timestamp {
                debug!("Source file {} modified since last build", path.display());
                return Ok(true);
            // Check checksum if available
            if let Some(cached_checksum) = entry.source_checksums.get(path) {
                let current_checksum = calculate_file_checksum(path)?;
                if &current_checksum != cached_checksum {
                    debug!("Source file {} checksum changed", path.display());
                    return Ok(true);
                }
            }
        debug!("Target {} is up to date", target_name);
        Ok(false)
    /// Calculate cache size
    pub fn calculate_size(&self) -> u64 {
        let mut size = 0;
        
        for entry in self.entries.values() {
            for output in &entry.outputs {
                if let Ok(metadata) = std::fs::metadata(output) {
                    size += metadata.len();
                }
            }
            
            for artifact in entry.artifacts.values() {
                if let Ok(metadata) = std::fs::metadata(artifact) {
                    size += metadata.len();
                }
            }
        size
    /// Cleanup old cache entries
    #[instrument(skip(self))]
    pub fn cleanup(&mut self, max_age: std::time::Duration) -> crate::error::Result<()> {
        let cutoff_time = SystemTime::now() - max_age;
        let mut removed_count = 0;
        
        let mut to_remove = Vec::new();
        for (target_name, entry) in &self.entries {
            if entry.timestamp < cutoff_time {
                to_remove.push(target_name.clone());
            }
        }
        
        for target_name in to_remove {
            self.remove(&target_name)?;
            removed_count += 1;
        if removed_count > 0 {
            info!("Cleaned up {} old cache entries", removed_count);
            self.metadata.last_cleanup = SystemTime::now();
        Ok(removed_count)
    /// Save cache to disk
    fn save_to_disk(&self) -> crate::error::Result<()> {
        let metadata_path = self.cache_dir.join("metadata.json");
        let entries_path = self.cache_dir.join("entries.json");
        
        let metadata_json = serde_json::to_string_pretty(&self.metadata)
            .map_err(|e| CacheError::SerializationError(e.to_string()))?;
        
        let entries_json = serde_json::to_string_pretty(&self.entries)
            .map_err(|e| CacheError::SerializationError(e.to_string()))?;
        
        std::fs::write(&metadata_path, metadata_json)?;
        std::fs::write(&entries_path, entries_json)?;
        
        Ok(())
    /// Get current build profile from environment or context
    fn get_current_build_profile(&self) -> String {
        // Try to get profile from environment variable
        if let Ok(profile) = std::env::var("CURSED_BUILD_PROFILE") {
            return profile;
        // Check for common profile indicators
        if std::env::var("CARGO_CFG_DEBUG_ASSERTIONS").is_ok() {
            return "debug".to_string();
        // Try to detect from build flags
        if let Ok(flags) = std::env::var("RUSTFLAGS") {
            if flags.contains("-O") || flags.contains("--opt-level") {
                return "release".to_string();
            }
        }
        
        // Check Cargo profile from environment
        if let Ok(cargo_profile) = std::env::var("CARGO_PROFILE") {
            return cargo_profile;
        // Default fallback
        "dev".to_string()
    /// Validate cache entry against current build context
    pub fn validate_cache_entry(&self, entry: &CacheEntry) -> bool {
        let current_profile = self.get_current_build_profile();
        
        // Check if profile matches
        if entry.profile != current_profile {
            debug!("Cache entry profile mismatch: {} != {}", entry.profile, current_profile);
            return false;
        // Check compiler version compatibility
        let current_version = env!("CARGO_PKG_VERSION");
        if entry.compiler_version != current_version {
            debug!("Cache entry compiler version mismatch: {} != {}", entry.compiler_version, current_version);
            return false;
        // Check if source files still exist and haven't changed
        for (source_path, cached_checksum) in &entry.source_checksums {
            if !source_path.exists() {
                debug!("Source file no longer exists: {}", source_path.display());
                return false;
            match calculate_file_checksum(source_path) {
                Ok(current_checksum) => {
                    if &current_checksum != cached_checksum {
                        debug!("Source file checksum changed: {}", source_path.display());
                        return false;
                    }
                }
                Err(_) => {
                    debug!("Failed to calculate checksum for: {}", source_path.display());
                    return false;
                }
            }
        true
    /// Enhanced cache invalidation with dependency tracking
    pub fn invalidate_dependents(&mut self, changed_files: &[PathBuf]) -> crate::error::Result<()> {
        let mut invalidated = 0;
        let mut to_invalidate = Vec::new();
        
        // Build dependency graph
        let mut dependency_graph: HashMap<String, Vec<String>> = HashMap::new();
        for (target_name, entry) in &self.entries {
            let mut deps = Vec::new();
            
            // Add source file dependencies
            for source_path in entry.source_checksums.keys() {
                if let Some(path_str) = source_path.to_str() {
                    deps.push(path_str.to_string());
                }
            }
            
            // Add explicit dependencies from metadata
            for dep in entry.dependency_checksums.keys() {
                deps.push(dep.clone());
            dependency_graph.insert(target_name.clone(), deps);
        // Find targets affected by changed files
        for changed_file in changed_files {
            let changed_file_str = changed_file.to_string_lossy().to_string();
            
            // Direct dependencies
            for (target_name, entry) in &self.entries {
                for source_path in entry.source_checksums.keys() {
                    if source_path == changed_file {
                        to_invalidate.push(target_name.clone());
                        break;
                    }
                }
            // Transitive dependencies
            self.find_transitive_dependents(&changed_file_str, &dependency_graph, &mut to_invalidate);
        // Remove invalidated entries
        for target_name in to_invalidate {
            if self.entries.remove(&target_name).is_some() {
                invalidated += 1;
                info!("Invalidated cache entry for target: {}", target_name);
            }
        }
        
        if invalidated > 0 {
            self.metadata.entry_count = self.entries.len();
            self.save_to_disk()?;
        Ok(invalidated)
    /// Find targets transitively dependent on a changed file
    fn find_transitive_dependents(
    ) {
        for (target_name, dependencies) in dependency_graph {
            if dependencies.contains(&changed_file.to_string()) && !to_invalidate.contains(target_name) {
                to_invalidate.push(target_name.clone());
                
                // Recursively find dependents of this target
                self.find_transitive_dependents(target_name, dependency_graph, to_invalidate);
            }
        }
    /// Get cache statistics
    pub fn get_statistics(&self) -> CacheStatistics {
        // Calculate hit rate based on cache lookups vs hits
        // In a real implementation, this would track actual lookup/hit counts
        let hit_rate = if self.entries.len() > 0 {
            // Estimate hit rate based on cache occupancy and entry age
            let now = SystemTime::now();
            let fresh_entries = self.entries.values()
                .filter(|entry| {
                    now.duration_since(entry.timestamp)
                        .unwrap_or_default()
                        .as_secs() < 3600 // Consider entries fresh if less than 1 hour old
                })
                .count();
            
            if self.entries.len() > 0 {
                (fresh_entries as f64 / self.entries.len() as f64) * 0.8 // Estimate 80% hit rate for fresh entries
            } else {
                0.0
            }
        } else {
            0.0
        
        CacheStatistics {
        }
    }
impl CacheManager {
    /// Create a new cache manager
    pub fn new(cache_dir: PathBuf) -> crate::error::Result<()> {
        std::fs::create_dir_all(&cache_dir)?;
        
        Ok(CacheManager {
        })
    /// Get or create a cache for a specific project
    pub fn get_cache(&mut self, project_name: &str) -> crate::error::Result<()> {
        if !self.caches.contains_key(project_name) {
            let cache_dir = self.global_cache_dir.join(project_name);
            let cache = IncrementalCache::new(cache_dir)?;
            self.caches.insert(project_name.to_string(), cache);
        Ok(self.caches.get_mut(project_name).unwrap())
    /// Cleanup all caches
    pub fn cleanup_all(&mut self, max_age: std::time::Duration) -> crate::error::Result<()> {
        let mut total_removed = 0;
        
        for cache in self.caches.values_mut() {
            total_removed += cache.cleanup(max_age)?;
        Ok(total_removed)
    /// Get global cache statistics
    pub fn get_global_statistics(&self) -> GlobalCacheStatistics {
        let mut stats = GlobalCacheStatistics {
        
        let mut total_age = std::time::Duration::from_secs(0);
        let mut entry_count = 0;
        
        for cache in self.caches.values() {
            let cache_stats = cache.get_statistics();
            stats.total_entries += cache_stats.entry_count;
            stats.total_size += cache_stats.cache_size;
            
            if let Ok(age) = SystemTime::now().duration_since(cache_stats.created) {
                total_age += age;
                entry_count += 1;
            }
        }
        
        if entry_count > 0 {
            stats.average_entry_age = total_age / entry_count as u32;
        stats
    }
}

/// Cache statistics for a single cache
#[derive(Debug, Clone)]
pub struct CacheStatistics {
/// Global cache statistics across all projects
#[derive(Debug, Clone)]
pub struct GlobalCacheStatistics {
/// Calculate SHA-256 checksum of a file
fn calculate_file_checksum(path: &Path) -> crate::error::Result<()> {
    use std::io::Read;
    
    let mut file = std::fs::File::open(path)?;
    let mut hasher = sha2::Sha256::new();
    let mut buffer = [0; 8192];
    
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    Ok(format!("{:x}", hasher.finalize()))
