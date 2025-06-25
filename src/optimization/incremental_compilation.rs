/// Incremental Compilation System
/// 
/// This module provides incremental compilation features including
/// change detection, dependency tracking, and compilation caching.

use crate::error::{CursedError, Result};
use crate::optimization::OptimizationConfig;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use tracing::{debug, info, instrument, warn};

/// Configuration for incremental compilation
#[derive(Debug, Clone)]
pub struct IncrementalConfig {
    /// Cache directory for incremental compilation artifacts
    /// Maximum cache size in bytes
    /// Enable dependency tracking
    /// Enable fine-grained change detection
    /// Parallel compilation of changed modules
    /// Maximum number of compilation units to process in parallel
    /// Cache retention time
impl Default for IncrementalConfig {
    fn default() -> Self {
        Self {
            max_cache_size: 1024 * 1024 * 1024, // 1GB
            cache_retention: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
        }
    }
/// Source file metadata for change detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    /// File path
    /// File content hash
    /// Last modification time
    /// File size in bytes
    /// Dependencies of this file
    /// Files that depend on this file
impl FileMetadata {
    /// Create metadata for a file
    pub fn new(path: &Path) -> Result<Self> {
        let metadata = std::fs::metadata(path)
            .map_err(|e| CursedError::Io(e.into()))?;
        
        let content = std::fs::read_to_string(path)
            .map_err(|e| CursedError::Io(e.into()))?;
        
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let content_hash = format!("{:x}", hasher.finalize());
        
        Ok(Self {
        })
    /// Check if file has changed
    pub fn has_changed(&self, other: &FileMetadata) -> bool {
        self.content_hash != other.content_hash ||
        self.modified != other.modified ||
        self.size != other.size
    }
}

/// Dependency graph for compilation units
#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    /// Adjacency list representation
    /// Reverse dependencies for efficient lookups
    /// File metadata cache
impl DependencyGraph {
    /// Add a dependency relationship
    pub fn add_dependency(&mut self, file: PathBuf, dependency: PathBuf) {
        self.dependencies.entry(file.clone())
            .or_default()
            .insert(dependency.clone());
        
        self.dependents.entry(dependency)
            .or_default()
            .insert(file);
    /// Get all files that depend on the given file
    pub fn get_dependents(&self, file: &Path) -> Vec<PathBuf> {
        self.dependents.get(file)
            .map(|deps| deps.iter().cloned().collect())
            .unwrap_or_default()
    /// Get all dependencies of the given file
    pub fn get_dependencies(&self, file: &Path) -> Vec<PathBuf> {
        self.dependencies.get(file)
            .map(|deps| deps.iter().cloned().collect())
            .unwrap_or_default()
    /// Topological sort for compilation order
    pub fn topological_sort(&self) -> Result<Vec<PathBuf>> {
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();
        let mut result = Vec::new();
        
        for file in self.dependencies.keys() {
            if !visited.contains(file) {
                self.dfs_visit(file, &mut visited, &mut temp_visited, &mut result)?;
            }
        }
        
        result.reverse();
        Ok(result)
    fn dfs_visit(
    ) -> Result<()> {
        if temp_visited.contains(file) {
            return Err(CursedError::Parse(format!("Circular dependency detected: {}", file.display())));
        if visited.contains(file) {
            return Ok(());
        temp_visited.insert(file.to_path_buf());
        
        if let Some(deps) = self.dependencies.get(file) {
            for dep in deps {
                self.dfs_visit(dep, visited, temp_visited, result)?;
            }
        }
        
        temp_visited.remove(file);
        visited.insert(file.to_path_buf());
        result.push(file.to_path_buf());
        
        Ok(())
    }
}

/// Change detection system
#[derive(Debug)]
pub struct ChangeDetector {
    /// Configuration
    /// Current file metadata
    /// Previous file metadata from cache
    /// Dependency graph
impl ChangeDetector {
    /// Create a new change detector
    pub fn new(config: IncrementalConfig) -> Result<Self> {
        let mut detector = Self {
        
        detector.load_cached_metadata()?;
        Ok(detector)
    /// Scan directory for source files
    #[instrument(skip(self))]
    pub fn scan_directory(&mut self, dir: &Path) -> Result<()> {
        for entry in std::fs::read_dir(dir).map_err(|e| CursedError::Io(e.into()))? {
            let entry = entry.map_err(|e| CursedError::Io(e.into()))?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "csd" || ext == "cursed" {
                        self.analyze_file(&path)?;
                    }
                }
            } else if path.is_dir() {
                self.scan_directory(&path)?;
            }
        }
        
        Ok(())
    /// Analyze a single file for changes and dependencies
    #[instrument(skip(self))]
    pub fn analyze_file(&mut self, path: &Path) -> Result<()> {
        let metadata = FileMetadata::new(path)?;
        
        // Extract dependencies from file content
        if self.config.enable_dependency_tracking {
            self.extract_dependencies(path, &metadata)?;
        self.current_metadata.insert(path.to_path_buf(), metadata);
        Ok(())
    /// Extract dependencies from file content
    fn extract_dependencies(&mut self, path: &Path, _metadata: &FileMetadata) -> Result<()> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| CursedError::Io(e.into()))?;
        
        // Simple dependency extraction based on import statements
        for line in content.split("\n") {
            let line = line.trim();
            if line.starts_with("import ") || line.starts_with("use ") {
                if let Some(dep_path) = self.parse_import_path(line, path) {
                    self.dependency_graph.add_dependency(path.to_path_buf(), dep_path);
                }
            }
        Ok(())
    /// Parse import path from import statement
    fn parse_import_path(&self, line: &str, current_file: &Path) -> Option<PathBuf> {
        // Extract quoted strings from import statements
        if let Some(start) = line.find('"') {
            if let Some(end) = line[start + 1..].find('"') {
                let import_path = &line[start + 1..start + 1 + end];
                
                // Resolve relative to current file
                if let Some(parent) = current_file.parent() {
                    let resolved = parent.join(import_path);
                    if resolved.exists() {
                        return Some(resolved);
                    }
                }
                
                // Try with .csd extension
                let with_ext = PathBuf::from(format!("{}.csd", import_path));
                if let Some(parent) = current_file.parent() {
                    let resolved = parent.join(&with_ext);
                    if resolved.exists() {
                        return Some(resolved);
                    }
                }
            }
        }
        
        None
    /// Get files that have changed
    pub fn get_changed_files(&self) -> Vec<PathBuf> {
        let mut changed = Vec::new();
        
        for (path, current) in &self.current_metadata {
            if let Some(cached) = self.cached_metadata.get(path) {
                if current.has_changed(cached) {
                    changed.push(path.clone());
                }
            } else {
                // New file
                changed.push(path.clone());
            }
        }
        
        changed
    /// Get files that need recompilation due to dependency changes
    pub fn get_affected_files(&self, changed_files: &[PathBuf]) -> Vec<PathBuf> {
        let mut affected = HashSet::new();
        
        for changed_file in changed_files {
            affected.insert(changed_file.clone());
            
            // Add all files that depend on the changed file
            let dependents = self.dependency_graph.get_dependents(changed_file);
            for dependent in dependents {
                affected.insert(dependent);
            }
        }
        
        affected.into_iter().collect()
    /// Save current metadata to cache
    pub fn save_metadata_cache(&self) -> Result<()> {
        std::fs::create_dir_all(&self.config.cache_dir)
            .map_err(|e| CursedError::Io(e.into()))?;
        
        let cache_file = self.config.cache_dir.join("metadata.json");
        let json = serde_json::to_string_pretty(&self.current_metadata)
            .map_err(|e| CursedError::Parse(format!("Failed to serialize metadata: {}", e)))?;
        
        std::fs::write(cache_file, json)
            .map_err(|e| CursedError::Io(e.into()))?;
        
        debug!("Saved metadata cache with {} files", self.current_metadata.len());
        Ok(())
    /// Load cached metadata
    fn load_cached_metadata(&mut self) -> Result<()> {
        let cache_file = self.config.cache_dir.join("metadata.json");
        
        if cache_file.exists() {
            let content = std::fs::read_to_string(&cache_file)
                .map_err(|e| CursedError::Io(e.into()))?;
            
            self.cached_metadata = serde_json::from_str(&content)
                .map_err(|e| CursedError::Parse(format!("Failed to parse cached metadata: {}", e)))?;
            
            debug!("Loaded metadata cache with {} files", self.cached_metadata.len());
        Ok(())
    }
}

/// Compilation cache for storing compiled artifacts
#[derive(Debug)]
pub struct CompilationCache {
    /// Configuration
    /// Cache entries
    /// Cache statistics
/// Cache entry for compiled artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Source file hash
    /// Compiled artifact path
    /// Compilation timestamp
    /// Dependencies hash
    /// Compilation time
    /// Artifact size
/// Cache statistics
#[derive(Debug, Default)]
pub struct CacheStatistics {
    /// Cache hits
    /// Cache misses
    /// Total cache size
    /// Number of entries
    /// Time saved by cache hits
impl CompilationCache {
    /// Create a new compilation cache
    pub fn new(config: IncrementalConfig) -> Result<Self> {
        std::fs::create_dir_all(&config.cache_dir)
            .map_err(|e| CursedError::Io(e.into()))?;
        
        let cache = Self {
        
        cache.load_cache()?;
        Ok(cache)
    /// Get cached artifact for source file
    #[instrument(skip(self))]
    pub fn get(&self, source_hash: &str, dependencies_hash: &str) -> Option<CacheEntry> {
        let entries = self.entries.lock().unwrap();
        
        if let Some(entry) = entries.get(source_hash) {
            if entry.dependencies_hash == dependencies_hash {
                // Verify artifact still exists
                if entry.artifact_path.exists() {
                    let mut stats = self.stats.lock().unwrap();
                    stats.hits += 1;
                    stats.time_saved += entry.compilation_time;
                    
                    debug!("Cache hit for {}", source_hash);
                    return Some(entry.clone());
                }
            }
        let mut stats = self.stats.lock().unwrap();
        stats.misses += 1;
        
        debug!("Cache miss for {}", source_hash);
        None
    /// Store compiled artifact in cache
    #[instrument(skip(self))]
    pub fn store(
    ) -> Result<()> {
        let artifact_size = std::fs::metadata(&artifact_path)
            .map(|m| m.len())
            .unwrap_or(0);
        
        let entry = CacheEntry {
        
        {
            let mut entries = self.entries.lock().unwrap();
            entries.insert(source_hash, entry);
            
            let mut stats = self.stats.lock().unwrap();
            stats.entry_count = entries.len();
            stats.total_size += artifact_size;
        self.cleanup_if_needed()?;
        debug!("Stored cache entry with size {} bytes", artifact_size);
        
        Ok(())
    /// Clean up old cache entries if cache is too large
    fn cleanup_if_needed(&self) -> Result<()> {
        let mut entries = self.entries.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        if stats.total_size > self.config.max_cache_size {
            // Sort entries by last access time and remove oldest
            let mut sorted_entries: Vec<_> = entries.iter().collect();
            sorted_entries.sort_by_key(|(_, entry)| entry.compiled_at);
            
            let mut removed_size = 0u64;
            let mut to_remove = Vec::new();
            
            for (key, entry) in sorted_entries {
                if stats.total_size - removed_size <= self.config.max_cache_size * 9 / 10 {
                    break;
                removed_size += entry.artifact_size;
                to_remove.push(key.clone());
                
                // Remove artifact file
                let _ = std::fs::remove_file(&entry.artifact_path);
            for key in to_remove {
                entries.remove(&key);
            stats.total_size -= removed_size;
            stats.entry_count = entries.len();
            
            info!("Cleaned up cache: removed {} bytes", removed_size);
        Ok(())
    /// Load cache from disk
    fn load_cache(&self) -> Result<()> {
        let cache_file = self.config.cache_dir.join("cache.json");
        
        if cache_file.exists() {
            let content = std::fs::read_to_string(&cache_file)
                .map_err(|e| CursedError::Io(e.into()))?;
            
            let entries: HashMap<String, CacheEntry> = serde_json::from_str(&content)
                .map_err(|e| CursedError::Parse(format!("Failed to parse cache: {}", e)))?;
            
            let mut stats = self.stats.lock().unwrap();
            stats.entry_count = entries.len();
            stats.total_size = entries.values().map(|e| e.artifact_size).sum();
            
            *self.entries.lock().unwrap() = entries;
            
            debug!("Loaded cache with {} entries", stats.entry_count);
        Ok(())
    /// Save cache to disk
    pub fn save_cache(&self) -> Result<()> {
        let cache_file = self.config.cache_dir.join("cache.json");
        let entries = self.entries.lock().unwrap();
        
        let json = serde_json::to_string_pretty(&*entries)
            .map_err(|e| CursedError::Parse(format!("Failed to serialize cache: {}", e)))?;
        
        std::fs::write(cache_file, json)
            .map_err(|e| CursedError::Io(e.into()))?;
        
        debug!("Saved cache with {} entries", entries.len());
        Ok(())
    /// Get cache statistics
    pub fn stats(&self) -> CacheStatistics {
        self.stats.lock().unwrap().clone()
    }
}

/// Main incremental compiler
pub struct IncrementalCompiler {
    /// Configuration
    /// Change detector
    /// Compilation cache
    /// Compilation statistics
/// Incremental compilation statistics
#[derive(Debug, Default)]
pub struct IncrementalCompilationStats {
    /// Total files processed
    /// Files compiled from cache
    /// Files compiled from source
    /// Total compilation time
    /// Time saved by incremental compilation
    /// Files skipped (unchanged)
impl IncrementalCompiler {
    /// Create a new incremental compiler
    pub fn new(config: &OptimizationConfig) -> Result<Self> {
        let incremental_config = IncrementalConfig::default();
        
        let change_detector = ChangeDetector::new(incremental_config.clone())?;
        let compilation_cache = CompilationCache::new(incremental_config.clone())?;
        
        Ok(Self {
        })
    /// Perform incremental compilation on a directory
    #[instrument(skip(self, compile_fn))]
    pub fn compile_directory<F>(
    ) -> Result<IncrementalCompilationResult>
    where
    {
        let start_time = std::time::Instant::now();
        
        // Scan for changes
        self.change_detector.scan_directory(dir)?;
        
        // Get changed and affected files
        let changed_files = self.change_detector.get_changed_files();
        let affected_files = self.change_detector.get_affected_files(&changed_files);
        
              changed_files.len(), affected_files.len());
        
        let mut compiled_files = Vec::new();
        let mut compilation_results = Vec::new();
        
        // Compile affected files
        if self.config.parallel_incremental && affected_files.len() > 1 {
            // Parallel compilation
            compilation_results = self.compile_parallel(&affected_files, compile_fn)?;
        } else {
            // Sequential compilation
            for file in &affected_files {
                let result = self.compile_single_file(file, &compile_fn)?;
                compilation_results.push(result);
            }
        }
        
        // Update statistics
        self.stats.files_processed = affected_files.len();
        self.stats.total_time = start_time.elapsed();
        
        for result in &compilation_results {
            compiled_files.push(result.output_path.clone());
            if result.from_cache {
                self.stats.cache_hits += 1;
            } else {
                self.stats.cache_misses += 1;
            }
        }
        
        // Save metadata cache
        self.change_detector.save_metadata_cache()?;
        self.compilation_cache.save_cache()?;
        
        Ok(IncrementalCompilationResult {
        })
    /// Compile a single file with caching
    fn compile_single_file<F>(
    ) -> Result<SingleCompilationResult>
    where
    {
        let metadata = FileMetadata::new(file)?;
        let deps_hash = self.calculate_dependencies_hash(file);
        
        // Check cache first
        if let Some(cache_entry) = self.compilation_cache.get(&metadata.content_hash, &deps_hash) {
            return Ok(SingleCompilationResult {
            });
        // Compile from source
        let (output_path, compilation_time) = compile_fn(file)?;
        
        // Store in cache
        self.compilation_cache.store(
        )?;
        
        Ok(SingleCompilationResult {
        })
    /// Compile files in parallel
    fn compile_parallel<F>(
    ) -> Result<Vec<SingleCompilationResult>>
    where
    {
        use rayon::prelude::*;
        
        let results: Result<Vec<_>> = files
            .par_iter()
            .map(|file| self.compile_single_file(file, &compile_fn))
            .collect();
        
        results
    /// Calculate hash of all dependencies
    fn calculate_dependencies_hash(&self, file: &Path) -> String {
        let deps = self.change_detector.dependency_graph.get_dependencies(file);
        let mut hasher = Sha256::new();
        
        for dep in deps {
            if let Ok(metadata) = FileMetadata::new(&dep) {
                hasher.update(metadata.content_hash.as_bytes());
            }
        }
        
        format!("{:x}", hasher.finalize())
    /// Get compilation statistics
    pub fn stats(&self) -> &IncrementalCompilationStats {
        &self.stats
    /// Get cache statistics
    pub fn cache_stats(&self) -> CacheStatistics {
        self.compilation_cache.stats()
    }
}

/// Result of incremental compilation
#[derive(Debug)]
pub struct IncrementalCompilationResult {
    /// Files that were compiled
    /// Number of cache hits
    /// Number of cache misses
    /// Total compilation time
    /// Number of files that changed
    /// Number of files affected by changes
/// Result of compiling a single file
#[derive(Debug)]
pub struct SingleCompilationResult {
    /// Input file path
    /// Output file path
    /// Compilation time (0 if from cache)
    /// Whether this was served from cache
impl IncrementalCompilationResult {
    /// Calculate cache hit ratio
    pub fn cache_hit_ratio(&self) -> f64 {
        if self.cache_hits + self.cache_misses == 0 {
            0.0
        } else {
            self.cache_hits as f64 / (self.cache_hits + self.cache_misses) as f64
        }
    }
    
    /// Print summary
    pub fn print_summary(&self) {
        println!("📊 Incremental Compilation Summary");
        println!("  Files changed: {}", self.files_changed);
        println!("  Files affected: {}", self.files_affected);
        println!("  Cache hits: {} ({:.1}%)", self.cache_hits, self.cache_hit_ratio() * 100.0);
        println!("  Cache misses: {}", self.cache_misses);
        println!("  Total time: {:?}", self.total_time);
        println!("  Compiled files: {}", self.compiled_files.len());
    }
}

