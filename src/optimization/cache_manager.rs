// Compilation artifact caching for improved build performance

use crate::error::{Result, CursedError};
use crate::optimization::metrics::CompilationUnit;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use std::fs;
use tracing::{info, debug, warn, instrument};
use serde::{Deserialize, Serialize};

/// Cache entry metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub file_path: PathBuf,
    pub created_at: SystemTime,
    pub last_accessed: SystemTime,
    pub size_bytes: u64,
    pub source_hash: String,
    pub optimization_level: String,
    pub dependencies: Vec<String>,
}

/// Cache statistics
#[derive(Debug, Default, Clone)]
pub struct CacheStatistics {
    pub total_entries: usize,
    pub total_size_bytes: u64,
    pub hits: usize,
    pub misses: usize,
    pub evictions: usize,
    pub cache_hit_rate: f64,
}

/// Configuration for cache manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub cache_directory: PathBuf,
    pub max_cache_size_mb: u64,
    pub max_entries: usize,
    pub entry_ttl_hours: u64,
    pub enable_compression: bool,
    pub cleanup_interval_hours: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            cache_directory: PathBuf::from(".cursed_cache"),
            max_cache_size_mb: 1024, // 1GB
            max_entries: 10000,
            entry_ttl_hours: 24 * 7, // 1 week
            enable_compression: true,
            cleanup_interval_hours: 24,
        }
    }
}

/// Cache manager for compilation artifacts
#[derive(Debug)]
pub struct CacheManager {
    config: CacheConfig,
    entries: HashMap<String, CacheEntry>,
    statistics: CacheStatistics,
    last_cleanup: SystemTime,
}

impl CacheManager {
    /// Create a new cache manager
    #[instrument]
    pub fn new() -> Result<Self> {
        let config = CacheConfig::default();
        Self::new_with_config(config)
    }

    /// Create a new cache manager with custom configuration
    #[instrument]
    pub fn new_with_config(config: CacheConfig) -> Result<Self> {
        info!("Creating cache manager with directory: {:?}", config.cache_directory);

        // Create cache directory if it doesn't exist
        if !config.cache_directory.exists() {
            fs::create_dir_all(&config.cache_directory).map_err(|e| {
                CursedError::optimization_error(&format!("Failed to create cache directory: {}", e))
            })?;
        }

        let mut manager = Self {
            config,
            entries: HashMap::new(),
            statistics: CacheStatistics::default(),
            last_cleanup: SystemTime::now(),
        };

        // Load existing cache entries
        manager.load_cache_index()?;

        // Perform initial cleanup
        manager.cleanup_expired_entries()?;

        Ok(manager)
    }

    /// Check if a compilation unit result is cached
    #[instrument(skip(self, unit))]
    pub fn get_cached_result(&mut self, unit: &CompilationUnit) -> Result<Option<PathBuf>> {
        let cache_key = self.generate_cache_key(unit)?;
        
        if let Some(entry) = self.entries.get_mut(&cache_key) {
            // Check if entry is still valid
            if self.is_entry_valid(entry)? {
                // Update access time
                entry.last_accessed = SystemTime::now();
                
                // Update statistics
                self.statistics.hits += 1;
                self.update_hit_rate();
                
                debug!("Cache hit for unit: {}", unit.name);
                return Ok(Some(entry.file_path.clone()));
            } else {
                // Entry is expired, remove it
                warn!("Cache entry expired for unit: {}", unit.name);
                self.remove_entry(&cache_key)?;
            }
        }

        // Cache miss
        self.statistics.misses += 1;
        self.update_hit_rate();
        debug!("Cache miss for unit: {}", unit.name);
        
        Ok(None)
    }
    
    /// Get cached data and decompress it
    pub fn get_cached_data(&mut self, unit: &CompilationUnit) -> Result<Option<Vec<u8>>> {
        if let Some(file_path) = self.get_cached_result(unit)? {
            // Read compressed data from file
            let compressed_data = std::fs::read(&file_path).map_err(|e| {
                CursedError::optimization_error(&format!("Failed to read cache file: {}", e))
            })?;
            
            // Decompress the data
            let decompressed_data = self.decompress_data(&compressed_data)?;
            
            debug!("Retrieved and decompressed {} bytes from cache", decompressed_data.len());
            Ok(Some(decompressed_data))
        } else {
            Ok(None)
        }
    }

    /// Store a compilation result in the cache
    #[instrument(skip(self, unit, result_data))]
    pub fn store_result(
        &mut self,
        unit: &CompilationUnit,
        result_data: &[u8],
        optimization_level: String,
    ) -> Result<()> {
        let cache_key = self.generate_cache_key(unit)?;
        let file_path = self.config.cache_directory.join(format!("{}.cache", cache_key));

        // Check cache size limits before storing
        if self.would_exceed_limits(result_data.len() as u64)? {
            self.evict_entries_to_make_space(result_data.len() as u64)?;
        }

        // Write data to cache file with compression
        let data_to_write = self.compress_data(result_data)?;

        fs::write(&file_path, data_to_write).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to write cache file: {}", e))
        })?;

        // Create cache entry
        let entry = CacheEntry {
            key: cache_key.clone(),
            file_path: file_path.clone(),
            created_at: SystemTime::now(),
            last_accessed: SystemTime::now(),
            size_bytes: result_data.len() as u64,
            source_hash: self.compute_source_hash(unit)?,
            optimization_level,
            dependencies: unit.dependencies.clone(),
        };

        // Update statistics
        self.statistics.total_entries += 1;
        self.statistics.total_size_bytes += entry.size_bytes;

        // Store entry
        self.entries.insert(cache_key, entry);

        // Save cache index
        self.save_cache_index()?;

        debug!("Stored cache entry for unit: {}", unit.name);
        Ok(())
    }

    /// Invalidate cache entries for a compilation unit
    #[instrument(skip(self, unit))]
    pub fn invalidate(&mut self, unit: &CompilationUnit) -> Result<()> {
        let cache_key = self.generate_cache_key(unit)?;
        
        if self.entries.contains_key(&cache_key) {
            self.remove_entry(&cache_key)?;
            info!("Invalidated cache entry for unit: {}", unit.name);
        }

        Ok(())
    }

    /// Clear all cache entries
    #[instrument(skip(self))]
    pub fn clear_all(&mut self) -> Result<()> {
        info!("Clearing all cache entries");

        // Remove all cache files
        for entry in self.entries.values() {
            if let Err(e) = fs::remove_file(&entry.file_path) {
                warn!("Failed to remove cache file {:?}: {}", entry.file_path, e);
            }
        }

        // Clear in-memory state
        self.entries.clear();
        self.statistics = CacheStatistics::default();

        // Save empty index
        self.save_cache_index()?;

        Ok(())
    }

    /// Get current cache statistics
    pub fn get_statistics(&self) -> &CacheStatistics {
        &self.statistics
    }

    /// Perform cache cleanup (remove expired entries)
    #[instrument(skip(self))]
    pub fn cleanup(&mut self) -> Result<()> {
        if self.should_perform_cleanup() {
            self.cleanup_expired_entries()?;
            self.last_cleanup = SystemTime::now();
        }
        Ok(())
    }

    /// Generate a cache key for a compilation unit
    fn generate_cache_key(&self, unit: &CompilationUnit) -> Result<String> {
        let source_hash = self.compute_source_hash(unit)?;
        let deps_hash = self.compute_dependencies_hash(&unit.dependencies)?;
        
        Ok(format!("{}_{}", source_hash, deps_hash))
    }

    /// Compute hash of source files
    fn compute_source_hash(&self, unit: &CompilationUnit) -> Result<String> {
        // Simplified hash computation (in real implementation, would hash file contents)
        let combined = unit.source_files.join("|");
        Ok(format!("{:x}", self.simple_hash(combined.as_bytes())))
    }

    /// Compute hash of dependencies
    fn compute_dependencies_hash(&self, dependencies: &[String]) -> Result<String> {
        let combined = dependencies.join("|");
        Ok(format!("{:x}", self.simple_hash(combined.as_bytes())))
    }

    /// Simple hash function for demonstration
    fn simple_hash(&self, data: &[u8]) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }

    /// Check if a cache entry is still valid
    fn is_entry_valid(&self, entry: &CacheEntry) -> Result<bool> {
        // Check if file exists
        if !entry.file_path.exists() {
            return Ok(false);
        }

        // Check TTL
        let ttl = Duration::from_secs(self.config.entry_ttl_hours * 3600);
        if entry.created_at.elapsed().unwrap_or(Duration::MAX) > ttl {
            return Ok(false);
        }

        // In a real implementation, would also check source file modification times
        Ok(true)
    }

    /// Check if storing new data would exceed cache limits
    fn would_exceed_limits(&self, new_data_size: u64) -> Result<bool> {
        let would_exceed_size = self.statistics.total_size_bytes + new_data_size > 
            self.config.max_cache_size_mb * 1024 * 1024;
        let would_exceed_entries = self.statistics.total_entries + 1 > self.config.max_entries;
        
        Ok(would_exceed_size || would_exceed_entries)
    }

    /// Evict entries to make space for new data
    fn evict_entries_to_make_space(&mut self, needed_space: u64) -> Result<()> {
        debug!("Evicting cache entries to make space for {} bytes", needed_space);

        // Sort entries by last access time (LRU)
        let mut entries_by_access: Vec<_> = self.entries.iter().collect();
        entries_by_access.sort_by_key(|(_, entry)| entry.last_accessed);

        let mut freed_space = 0u64;
        let mut keys_to_remove = Vec::new();

        for (key, entry) in entries_by_access {
            keys_to_remove.push(key.clone());
            freed_space += entry.size_bytes;
            
            if freed_space >= needed_space {
                break;
            }
        }

        // Remove selected entries
        for key in keys_to_remove {
            self.remove_entry(&key)?;
            self.statistics.evictions += 1;
        }

        info!("Evicted {} entries, freed {} bytes", self.statistics.evictions, freed_space);
        Ok(())
    }

    /// Remove a cache entry
    fn remove_entry(&mut self, key: &str) -> Result<()> {
        if let Some(entry) = self.entries.remove(key) {
            // Remove file
            if let Err(e) = fs::remove_file(&entry.file_path) {
                warn!("Failed to remove cache file {:?}: {}", entry.file_path, e);
            }

            // Update statistics
            self.statistics.total_entries = self.statistics.total_entries.saturating_sub(1);
            self.statistics.total_size_bytes = self.statistics.total_size_bytes.saturating_sub(entry.size_bytes);
        }

        Ok(())
    }

    /// Clean up expired cache entries
    fn cleanup_expired_entries(&mut self) -> Result<()> {
        debug!("Cleaning up expired cache entries");

        let mut keys_to_remove = Vec::new();

        for (key, entry) in &self.entries {
            if !self.is_entry_valid(entry)? {
                keys_to_remove.push(key.clone());
            }
        }

        for key in keys_to_remove {
            self.remove_entry(&key)?;
        }

        self.save_cache_index()?;
        Ok(())
    }

    /// Check if cleanup should be performed
    fn should_perform_cleanup(&self) -> bool {
        let cleanup_interval = Duration::from_secs(self.config.cleanup_interval_hours * 3600);
        self.last_cleanup.elapsed().unwrap_or(Duration::ZERO) > cleanup_interval
    }

    /// Update cache hit rate
    fn update_hit_rate(&mut self) {
        let total_requests = self.statistics.hits + self.statistics.misses;
        if total_requests > 0 {
            self.statistics.cache_hit_rate = self.statistics.hits as f64 / total_requests as f64;
        }
    }

    /// Load cache index from disk
    fn load_cache_index(&mut self) -> Result<()> {
        let index_path = self.config.cache_directory.join("index.json");
        
        if index_path.exists() {
            let index_data = fs::read_to_string(&index_path).map_err(|e| {
                CursedError::optimization_error(&format!("Failed to read cache index: {}", e))
            })?;

            let entries: HashMap<String, CacheEntry> = serde_json::from_str(&index_data)
                .map_err(|e| {
                    CursedError::optimization_error(&format!("Failed to parse cache index: {}", e))
                })?;

            // Update statistics
            self.statistics.total_entries = entries.len();
            self.statistics.total_size_bytes = entries.values().map(|e| e.size_bytes).sum();

            self.entries = entries;
            
            debug!("Loaded {} cache entries from index", self.entries.len());
        }

        Ok(())
    }

    /// Save cache index to disk
    fn save_cache_index(&self) -> Result<()> {
        let index_path = self.config.cache_directory.join("index.json");
        
        let index_data = serde_json::to_string_pretty(&self.entries).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to serialize cache index: {}", e))
        })?;

        fs::write(&index_path, index_data).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to write cache index: {}", e))
        })?;

        Ok(())
    }

    /// Compress data for storage
    fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        if !self.config.enable_compression {
            return Ok(data.to_vec());
        }
        
        // Try to use compression if available, otherwise fall back to no compression
        #[cfg(feature = "compression")]
        {
            use std::io::Write;
            
            let mut encoder = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
            encoder.write_all(data).map_err(|e| {
                CursedError::optimization_error(&format!("Failed to compress data: {}", e))
            })?;
            
            let compressed = encoder.finish().map_err(|e| {
                CursedError::optimization_error(&format!("Failed to finish compression: {}", e))
            })?;
            
            debug!("Compressed {} bytes to {} bytes ({:.1}% ratio)", 
                   data.len(), compressed.len(), 
                   (compressed.len() as f64 / data.len() as f64) * 100.0);
            
            Ok(compressed)
        }
        
        #[cfg(not(feature = "compression"))]
        {
            // Fallback: simple compression using basic algorithm
            let compressed = self.simple_compress(data);
            debug!("Simple compressed {} bytes to {} bytes", data.len(), compressed.len());
            Ok(compressed)
        }
    }
    
    /// Decompress data from storage
    fn decompress_data(&self, compressed_data: &[u8]) -> Result<Vec<u8>> {
        if !self.config.enable_compression {
            return Ok(compressed_data.to_vec());
        }
        
        #[cfg(feature = "compression")]
        {
            use std::io::Read;
            
            let mut decoder = flate2::read::DeflateDecoder::new(compressed_data);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed).map_err(|e| {
                CursedError::optimization_error(&format!("Failed to decompress data: {}", e))
            })?;
            
            Ok(decompressed)
        }
        
        #[cfg(not(feature = "compression"))]
        {
            // Fallback: simple decompression
            let decompressed = self.simple_decompress(compressed_data)?;
            Ok(decompressed)
        }
    }
    
    /// Simple compression fallback
    fn simple_compress(&self, data: &[u8]) -> Vec<u8> {
        // Very basic run-length encoding for fallback
        let mut compressed = Vec::new();
        if data.is_empty() {
            return compressed;
        }
        
        let mut current_byte = data[0];
        let mut count = 1u8;
        
        for &byte in &data[1..] {
            if byte == current_byte && count < 255 {
                count += 1;
            } else {
                compressed.push(count);
                compressed.push(current_byte);
                current_byte = byte;
                count = 1;
            }
        }
        
        // Push the last run
        compressed.push(count);
        compressed.push(current_byte);
        
        compressed
    }
    
    /// Simple decompression fallback
    fn simple_decompress(&self, compressed_data: &[u8]) -> Result<Vec<u8>> {
        let mut decompressed = Vec::new();
        
        if compressed_data.len() % 2 != 0 {
            return Err(CursedError::optimization_error("Invalid compressed data format"));
        }
        
        for chunk in compressed_data.chunks(2) {
            let count = chunk[0];
            let byte_value = chunk[1];
            
            for _ in 0..count {
                decompressed.push(byte_value);
            }
        }
        
        Ok(decompressed)
    }

    /// Update cache configuration
    pub fn update_config(&mut self, new_config: CacheConfig) -> Result<()> {
        info!("Updating cache manager configuration");
        self.config = new_config;
        Ok(())
    }
}

impl Drop for CacheManager {
    fn drop(&mut self) {
        let _ = self.save_cache_index();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_cache_manager_creation() {
        let temp_dir = env::temp_dir().join("cursed_cache_test");
        let config = CacheConfig {
            cache_directory: temp_dir,
            ..Default::default()
        };

        let cache_manager = CacheManager::new_with_config(config);
        assert!(cache_manager.is_ok());
    }

    #[test]
    fn test_cache_key_generation() {
        let temp_dir = env::temp_dir().join("cursed_cache_test_2");
        let config = CacheConfig {
            cache_directory: temp_dir,
            ..Default::default()
        };

        let cache_manager = CacheManager::new_with_config(config).unwrap();
        
        let mut unit = CompilationUnit::new("test_unit".to_string());
        unit.source_files.push("test.csd".to_string());
        
        let key1 = cache_manager.generate_cache_key(&unit).unwrap();
        let key2 = cache_manager.generate_cache_key(&unit).unwrap();
        
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_cache_statistics() {
        let temp_dir = env::temp_dir().join("cursed_cache_test_3");
        let config = CacheConfig {
            cache_directory: temp_dir,
            ..Default::default()
        };

        let mut cache_manager = CacheManager::new_with_config(config).unwrap();
        
        let unit = CompilationUnit::new("test_unit".to_string());
        
        // Test cache miss
        let result = cache_manager.get_cached_result(&unit);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
        
        let stats = cache_manager.get_statistics();
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hits, 0);
    }
}
