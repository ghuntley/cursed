/// Compilation Caching Infrastructure
/// 
/// Provides comprehensive compilation result caching to avoid recompiling
/// unchanged files, including AST caching, bytecode caching, and dependency tracking.

use crate::error::{Error, Result};
use crate::optimization::optimization_levels::{OptimizationLevel, LevelConfig};
use crate::optimization::compilation_speed::{CompilationUnit, CompilationStatus};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tracing::{debug, info, instrument, warn, error};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Cache directory
    pub cache_dir: PathBuf,
    /// Enable AST caching
    pub enable_ast_cache: bool,
    /// Enable bytecode caching
    pub enable_bytecode_cache: bool,
    /// Enable dependency caching
    pub enable_dependency_cache: bool,
    /// Maximum cache size (in bytes)
    pub max_cache_size: u64,
    /// Cache entry TTL (time to live)
    pub cache_ttl: Duration,
    /// Enable cache compression
    pub enable_compression: bool,
    /// Cache validation strategy
    pub validation_strategy: CacheValidationStrategy,
}

/// Cache validation strategies
#[derive(Debug, Clone, Copy)]
pub enum CacheValidationStrategy {
    /// Check file modification time only
    ModificationTime,
    /// Check file content hash
    ContentHash,
    /// Check both modification time and hash
    ModTimeAndHash,
    /// Check dependency chain
    DependencyChain,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            cache_dir: PathBuf::from(".cursed_cache"),
            enable_ast_cache: true,
            enable_bytecode_cache: true,
            enable_dependency_cache: true,
            max_cache_size: 1024 * 1024 * 1024, // 1GB
            cache_ttl: Duration::from_secs(24 * 60 * 60), // 24 hours
            enable_compression: true,
            validation_strategy: CacheValidationStrategy::ModTimeAndHash,
        }
    }
}

/// Cache entry metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Unique cache key
    pub key: String,
    /// Source file path
    pub source_path: PathBuf,
    /// Source file hash
    pub source_hash: String,
    /// Source file modification time
    pub source_mtime: SystemTime,
    /// Compilation options hash
    pub options_hash: String,
    /// Dependencies with their hashes
    pub dependencies: HashMap<PathBuf, String>,
    /// Cache creation time
    pub created_at: SystemTime,
    /// Last access time
    pub last_accessed: SystemTime,
    /// Cache entry size
    pub size: u64,
    /// Optimization level used
    pub optimization_level: OptimizationLevel,
    /// Cache entry type
    pub entry_type: CacheEntryType,
}

/// Types of cache entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheEntryType {
    /// Abstract Syntax Tree
    Ast,
    /// Compiled bytecode/LLVM IR
    Bytecode,
    /// Dependency information
    Dependencies,
    /// Type checking results
    TypeInfo,
    /// Optimization metadata
    OptimizationData,
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStatistics {
    /// Total cache hits
    pub cache_hits: usize,
    /// Total cache misses
    pub cache_misses: usize,
    /// Total cache size
    pub total_size: u64,
    /// Number of cache entries
    pub entry_count: usize,
    /// Cache cleanup operations
    pub cleanup_operations: usize,
    /// Time saved by caching
    pub time_saved: Duration,
    /// Space used by cache
    pub space_used: u64,
}

impl CacheStatistics {
    /// Calculate cache hit ratio
    pub fn hit_ratio(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            return 0.0;
        }
        self.cache_hits as f64 / total as f64
    }

    /// Calculate space efficiency
    pub fn space_efficiency(&self) -> f64 {
        if self.space_used == 0 {
            return 0.0;
        }
        self.total_size as f64 / self.space_used as f64
    }
}

/// Comprehensive cache manager
pub struct CacheManager {
    config: CacheConfig,
    cache_index: Arc<RwLock<HashMap<String, CacheEntry>>>,
    statistics: Arc<Mutex<CacheStatistics>>,
    dependency_tracker: DependencyTracker,
}

impl CacheManager {
    /// Create a new cache manager
    #[instrument(skip(config))]
    pub fn new(config: &super::OptimizationConfig) -> Result<Self> {
        let cache_config = CacheConfig {
            cache_dir: PathBuf::from(".cursed_cache"),
            ..Default::default()
        };

        info!("Initializing cache manager at {:?}", cache_config.cache_dir);

        // Create cache directory if it doesn't exist
        if !cache_config.cache_dir.exists() {
            fs::create_dir_all(&cache_config.cache_dir)
                .map_err(|e| Error::IoError(format!("Failed to create cache directory: {}", e)))?;
        }

        let cache_index = Arc::new(RwLock::new(HashMap::new()));
        let statistics = Arc::new(Mutex::new(CacheStatistics::default()));
        let dependency_tracker = DependencyTracker::new();

        let mut cache_manager = Self {
            config: cache_config,
            cache_index,
            statistics,
            dependency_tracker,
        };

        // Load existing cache index
        cache_manager.load_cache_index()?;

        Ok(cache_manager)
    }

    /// Generate cache key for a compilation unit
    #[instrument(skip(self, unit, optimization_level))]
    pub fn generate_cache_key(
        &self,
        unit: &CompilationUnit,
        optimization_level: OptimizationLevel,
    ) -> Result<String> {
        let mut hasher = Sha256::new();
        
        // Hash source path
        hasher.update(unit.source_path.to_string_lossy().as_bytes());
        
        // Hash source content
        hasher.update(&unit.source_code);
        
        // Hash optimization level
        hasher.update(format!("{:?}", optimization_level).as_bytes());
        
        // Hash dependencies
        for dep in &unit.dependencies {
            hasher.update(dep.as_bytes());
        }
        
        let hash = hasher.finalize();
        Ok(format!("{:x}", hash))
    }

    /// Check if cache entry is valid
    #[instrument(skip(self, unit))]
    pub fn is_cache_valid(&self, unit: &CompilationUnit, cache_key: &str) -> Result<bool> {
        let cache_index = self.cache_index.read().unwrap();
        
        let entry = match cache_index.get(cache_key) {
            Some(entry) => entry,
            None => {
                debug!("Cache miss: entry not found for key {}", cache_key);
                return Ok(false);
            }
        };

        // Check TTL
        if entry.created_at.elapsed().unwrap_or(Duration::MAX) > self.config.cache_ttl {
            debug!("Cache entry expired for key {}", cache_key);
            return Ok(false);
        }

        // Check validation strategy
        match self.config.validation_strategy {
            CacheValidationStrategy::ModificationTime => {
                self.validate_modification_time(unit, entry)
            }
            CacheValidationStrategy::ContentHash => {
                self.validate_content_hash(unit, entry)
            }
            CacheValidationStrategy::ModTimeAndHash => {
                Ok(self.validate_modification_time(unit, entry)? && 
                self.validate_content_hash(unit, entry)?)
            }
            CacheValidationStrategy::DependencyChain => {
                self.validate_dependency_chain(unit, entry)
            }
        }
    }

    /// Validate modification time
    fn validate_modification_time(&self, unit: &CompilationUnit, entry: &CacheEntry) -> Result<bool> {
        Ok(unit.last_modified <= entry.source_mtime)
    }

    /// Validate content hash
    fn validate_content_hash(&self, unit: &CompilationUnit, entry: &CacheEntry) -> Result<bool> {
        let current_hash = self.calculate_content_hash(&unit.source_code)?;
        Ok(current_hash == entry.source_hash)
    }

    /// Validate dependency chain
    fn validate_dependency_chain(&self, unit: &CompilationUnit, entry: &CacheEntry) -> Result<bool> {
        // Check if any dependencies have changed
        for (dep_path, expected_hash) in &entry.dependencies {
            if let Ok(current_hash) = self.calculate_file_hash(dep_path) {
                if current_hash != *expected_hash {
                    debug!("Dependency changed: {:?}", dep_path);
                    return Ok(false);
                }
            } else {
                debug!("Dependency not found: {:?}", dep_path);
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Calculate content hash
    fn calculate_content_hash(&self, content: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Calculate file hash
    fn calculate_file_hash(&self, path: &Path) -> Result<String> {
        let mut file = File::open(path)
            .map_err(|e| Error::IoError(format!("Failed to open file: {}", e)))?;
        
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];
        
        loop {
            let bytes_read = file.read(&mut buffer)
                .map_err(|e| Error::IoError(format!("Failed to read file: {}", e)))?;
            
            if bytes_read == 0 {
                break;
            }
            
            hasher.update(&buffer[..bytes_read]);
        }
        
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Store cache entry
    #[instrument(skip(self, unit, data))]
    pub fn store_cache_entry(
        &self,
        unit: &CompilationUnit,
        optimization_level: OptimizationLevel,
        entry_type: CacheEntryType,
        data: &[u8],
    ) -> Result<()> {
        let cache_key = self.generate_cache_key(unit, optimization_level)?;
        
        debug!("Storing cache entry: {} (type: {:?})", cache_key, entry_type);

        // Create cache entry metadata
        let source_hash = self.calculate_content_hash(&unit.source_code)?;
        let dependencies = self.dependency_tracker.get_dependencies(&unit.source_path)?;
        
        let cache_entry = CacheEntry {
            key: cache_key.clone(),
            source_path: unit.source_path.clone(),
            source_hash,
            source_mtime: unit.last_modified,
            options_hash: format!("{:?}", optimization_level),
            dependencies,
            created_at: SystemTime::now(),
            last_accessed: SystemTime::now(),
            size: data.len() as u64,
            optimization_level,
            entry_type,
        };

        // Write data to cache file
        let cache_file_path = self.get_cache_file_path(&cache_key);
        self.write_cache_file(&cache_file_path, data)?;

        // Update cache index
        {
            let mut cache_index = self.cache_index.write().unwrap();
            cache_index.insert(cache_key, cache_entry);
        }

        // Update statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.entry_count += 1;
            stats.total_size += data.len() as u64;
        }

        // Check if cache cleanup is needed
        self.maybe_cleanup_cache()?;

        Ok(())
    }

    /// Retrieve cache entry
    #[instrument(skip(self))]
    pub fn retrieve_cache_entry(&self, cache_key: &str) -> Result<Option<Vec<u8>>> {
        let cache_file_path = self.get_cache_file_path(cache_key);
        
        if !cache_file_path.exists() {
            debug!("Cache file not found: {:?}", cache_file_path);
            return Ok(None);
        }

        let data = self.read_cache_file(&cache_file_path)?;

        // Update access time
        {
            let mut cache_index = self.cache_index.write().unwrap();
            if let Some(entry) = cache_index.get_mut(cache_key) {
                entry.last_accessed = SystemTime::now();
            }
        }

        // Update statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.cache_hits += 1;
        }

        debug!("Cache hit: {} ({} bytes)", cache_key, data.len());
        Ok(Some(data))
    }

    /// Record cache miss
    pub fn record_cache_miss(&self) {
        let mut stats = self.statistics.lock().unwrap();
        stats.cache_misses += 1;
    }

    /// Get cache file path
    fn get_cache_file_path(&self, cache_key: &str) -> PathBuf {
        self.config.cache_dir.join(format!("{}.cache", cache_key))
    }

    /// Write cache file
    fn write_cache_file(&self, path: &Path, data: &[u8]) -> Result<()> {
        let file = File::create(path)
            .map_err(|e| Error::IoError(format!("Failed to create cache file: {}", e)))?;
        
        let mut writer = BufWriter::new(file);
        
        if self.config.enable_compression {
            // In a real implementation, we would use actual compression
            // For now, just write the data directly
            writer.write_all(data)
                .map_err(|e| Error::IoError(format!("Failed to write cache file: {}", e)))?;
        } else {
            writer.write_all(data)
                .map_err(|e| Error::IoError(format!("Failed to write cache file: {}", e)))?;
        }
        
        Ok(())
    }

    /// Read cache file
    fn read_cache_file(&self, path: &Path) -> Result<Vec<u8>> {
        let file = File::open(path)
            .map_err(|e| Error::IoError(format!("Failed to open cache file: {}", e)))?;
        
        let mut reader = BufReader::new(file);
        let mut data = Vec::new();
        
        reader.read_to_end(&mut data)
            .map_err(|e| Error::IoError(format!("Failed to read cache file: {}", e)))?;
        
        Ok(data)
    }

    /// Load cache index from disk
    fn load_cache_index(&mut self) -> Result<()> {
        let index_path = self.config.cache_dir.join("index.json");
        
        if !index_path.exists() {
            info!("No existing cache index found");
            return Ok(());
        }

        let index_data = fs::read_to_string(&index_path)
            .map_err(|e| Error::IoError(format!("Failed to read cache index: {}", e)))?;
        
        let entries: HashMap<String, CacheEntry> = serde_json::from_str(&index_data)
            .map_err(|e| Error::ParseError(format!("Failed to parse cache index: {}", e)))?;

        {
            let mut cache_index = self.cache_index.write().unwrap();
            *cache_index = entries;
        }

        info!("Loaded cache index with {} entries", cache_index.read().unwrap().len());
        Ok(())
    }

    /// Save cache index to disk
    fn save_cache_index(&self) -> Result<()> {
        let index_path = self.config.cache_dir.join("index.json");
        let cache_index = self.cache_index.read().unwrap();
        
        let index_data = serde_json::to_string_pretty(&*cache_index)
            .map_err(|e| Error::Internal(format!("Failed to serialize cache index: {}", e)))?;
        
        fs::write(&index_path, index_data)
            .map_err(|e| Error::IoError(format!("Failed to write cache index: {}", e)))?;

        Ok(())
    }

    /// Maybe cleanup cache if it's getting too large
    fn maybe_cleanup_cache(&self) -> Result<()> {
        let current_size = {
            let stats = self.statistics.lock().unwrap();
            stats.total_size
        };

        if current_size > self.config.max_cache_size {
            info!("Cache size exceeded limit, starting cleanup");
            self.cleanup_cache()?;
        }

        Ok(())
    }

    /// Cleanup old cache entries
    #[instrument(skip(self))]
    pub fn cleanup_cache(&self) -> Result<()> {
        let mut entries_to_remove = Vec::new();
        let now = SystemTime::now();

        // Find entries to remove (LRU + TTL)
        {
            let cache_index = self.cache_index.read().unwrap();
            
            for (key, entry) in cache_index.iter() {
                // Remove expired entries
                if now.duration_since(entry.created_at).unwrap_or(Duration::MAX) > self.config.cache_ttl {
                    entries_to_remove.push(key.clone());
                }
            }
        }

        // Remove entries
        for key in &entries_to_remove {
            self.remove_cache_entry(key)?;
        }

        // Update statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.cleanup_operations += 1;
        }

        info!("Cache cleanup completed, removed {} entries", entries_to_remove.len());
        Ok(())
    }

    /// Remove a specific cache entry
    fn remove_cache_entry(&self, cache_key: &str) -> Result<()> {
        // Remove from index
        let entry_size = {
            let mut cache_index = self.cache_index.write().unwrap();
            cache_index.remove(cache_key).map(|e| e.size).unwrap_or(0)
        };

        // Remove cache file
        let cache_file_path = self.get_cache_file_path(cache_key);
        if cache_file_path.exists() {
            fs::remove_file(&cache_file_path)
                .map_err(|e| Error::IoError(format!("Failed to remove cache file: {}", e)))?;
        }

        // Update statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.entry_count = stats.entry_count.saturating_sub(1);
            stats.total_size = stats.total_size.saturating_sub(entry_size);
        }

        Ok(())
    }

    /// Get cache statistics
    pub fn get_statistics(&self) -> CacheStatistics {
        self.statistics.lock().unwrap().clone()
    }

    /// Print cache summary
    pub fn print_summary(&self) {
        let stats = self.get_statistics();
        
        println!("💾 Cache Manager Summary:");
        println!("   Cache directory: {:?}", self.config.cache_dir);
        println!("   Total entries: {}", stats.entry_count);
        println!("   Cache hits: {}", stats.cache_hits);
        println!("   Cache misses: {}", stats.cache_misses);
        println!("   Hit ratio: {:.1}%", stats.hit_ratio() * 100.0);
        println!("   Total size: {:.1} MB", stats.total_size as f64 / 1024.0 / 1024.0);
        println!("   Space used: {:.1} MB", stats.space_used as f64 / 1024.0 / 1024.0);
        println!("   Space efficiency: {:.2}x", stats.space_efficiency());
        println!("   Time saved: {:?}", stats.time_saved);
        println!("   Cleanup operations: {}", stats.cleanup_operations);
    }

    /// Clear all cache entries
    pub fn clear_cache(&self) -> Result<()> {
        info!("Clearing all cache entries");

        // Remove all cache files
        if self.config.cache_dir.exists() {
            fs::remove_dir_all(&self.config.cache_dir)
                .map_err(|e| Error::IoError(format!("Failed to remove cache directory: {}", e)))?;
            
            fs::create_dir_all(&self.config.cache_dir)
                .map_err(|e| Error::IoError(format!("Failed to recreate cache directory: {}", e)))?;
        }

        // Clear index
        {
            let mut cache_index = self.cache_index.write().unwrap();
            cache_index.clear();
        }

        // Reset statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            *stats = CacheStatistics::default();
        }

        Ok(())
    }
}

impl Drop for CacheManager {
    fn drop(&mut self) {
        if let Err(e) = self.save_cache_index() {
            error!("Failed to save cache index: {}", e);
        }
    }
}

/// Dependency tracker for cache validation
pub struct DependencyTracker {
    dependencies: RwLock<HashMap<PathBuf, HashSet<PathBuf>>>,
}

impl DependencyTracker {
    pub fn new() -> Self {
        Self {
            dependencies: RwLock::new(HashMap::new()),
        }
    }

    /// Add dependency relationship
    pub fn add_dependency(&self, source: &Path, dependency: &Path) {
        let mut deps = self.dependencies.write().unwrap();
        deps.entry(source.to_path_buf())
            .or_insert_with(HashSet::new)
            .insert(dependency.to_path_buf());
    }

    /// Get dependencies for a source file
    pub fn get_dependencies(&self, source: &Path) -> Result<HashMap<PathBuf, String>> {
        let deps = self.dependencies.read().unwrap();
        let mut result = HashMap::new();

        if let Some(dep_set) = deps.get(source) {
            for dep_path in dep_set {
                if let Ok(hash) = self.calculate_file_hash(dep_path) {
                    result.insert(dep_path.clone(), hash);
                }
            }
        }

        Ok(result)
    }

    /// Calculate file hash (simplified implementation)
    fn calculate_file_hash(&self, path: &Path) -> Result<String> {
        if let Ok(content) = fs::read_to_string(path) {
            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());
            Ok(format!("{:x}", hasher.finalize()))
        } else {
            Err(Error::IoError(format!("Failed to read file: {:?}", path)))
        }
    }
}

/// Optimization cache (for backward compatibility)
pub struct OptimizationCache;

/// Cache strategy (for backward compatibility)
pub struct CacheStrategy;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_cache_manager() -> (CacheManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config = super::super::OptimizationConfig::default();
        let mut cache_manager = CacheManager::new(&config).unwrap();
        cache_manager.config.cache_dir = temp_dir.path().to_path_buf();
        (cache_manager, temp_dir)
    }

    #[test]
    fn test_cache_key_generation() {
        let (cache_manager, _temp_dir) = create_test_cache_manager();
        
        let unit = CompilationUnit {
            id: "test".to_string(),
            source_path: PathBuf::from("test.csd"),
            module_name: "test".to_string(),
            source_code: "let x = 42;".to_string(),
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
        };

        let key1 = cache_manager.generate_cache_key(&unit, OptimizationLevel::Standard).unwrap();
        let key2 = cache_manager.generate_cache_key(&unit, OptimizationLevel::Standard).unwrap();
        let key3 = cache_manager.generate_cache_key(&unit, OptimizationLevel::Aggressive).unwrap();

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_cache_statistics() {
        let mut stats = CacheStatistics::default();
        stats.cache_hits = 80;
        stats.cache_misses = 20;

        assert_eq!(stats.hit_ratio(), 0.8);
    }

    #[test]
    fn test_dependency_tracker() {
        let tracker = DependencyTracker::new();
        let source = Path::new("main.csd");
        let dep = Path::new("module.csd");

        tracker.add_dependency(source, dep);
        
        // Dependencies would be empty since files don't exist in test
        let deps = tracker.get_dependencies(source).unwrap();
        assert!(deps.is_empty());
    }
}
