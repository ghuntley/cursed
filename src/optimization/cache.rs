/// Compilation Cache System
/// 
/// Provides intelligent caching of compiled artifacts, IR, and build metadata
/// to speed up compilation by reusing previously computed results.

use crate::error::{CursedError, Result};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use flate2::{Compression, read::GzDecoder, write::GzEncoder};

/// Cache entry metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub file_path: PathBuf,
    pub source_hash: String,
    pub dependencies_hash: String,
    pub compilation_flags: Vec<String>,
    pub created_at: u64,
    pub last_accessed: u64,
    pub access_count: usize,
    pub file_size: usize,
    pub cache_type: CacheType,
}

/// Types of cached content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheType {
    CompiledObject,
    LlvmIr,
    AstSerialized,
    PreprocessedSource,
    DependencyInfo,
    OptimizationMetadata,
}

impl CacheType {
    pub fn file_extension(&self) -> &'static str {
        match self {
            CacheType::CompiledObject => "o",
            CacheType::LlvmIr => "ll",
            CacheType::AstSerialized => "ast",
            CacheType::PreprocessedSource => "i",
            CacheType::DependencyInfo => "dep",
            CacheType::OptimizationMetadata => "opt",
        }
    }
    
    pub fn should_compress(&self) -> bool {
        match self {
            CacheType::LlvmIr | CacheType::PreprocessedSource | CacheType::AstSerialized => true,
            _ => false,
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_size_mb: usize,
    pub max_entries: usize,
    pub ttl_hours: usize,
    pub compression_enabled: bool,
    pub eviction_strategy: EvictionStrategy,
}

/// Cache eviction strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionStrategy {
    LeastRecentlyUsed,
    LeastFrequentlyUsed,
    FirstInFirstOut,
    SizeBasedPriority,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size_mb: 1024, // 1GB default
            max_entries: 10000,
            ttl_hours: 24 * 7, // 1 week
            compression_enabled: true,
            eviction_strategy: EvictionStrategy::LeastRecentlyUsed,
        }
    }
}

/// Main compilation cache
pub struct CompilationCache {
    cache_dir: PathBuf,
    config: CacheConfig,
    entries: HashMap<String, CacheEntry>,
    metadata_file: PathBuf,
    stats: CacheStats,
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
    pub evictions: usize,
    pub total_size_bytes: usize,
    pub entry_count: usize,
    pub compression_ratio: f64,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            evictions: 0,
            total_size_bytes: 0,
            entry_count: 0,
            compression_ratio: 1.0,
        }
    }
}

impl CompilationCache {
    /// Create new compilation cache
    pub fn new(cache_dir: &Path) -> Result<Self> {
        let config = CacheConfig::default();
        Self::with_config(cache_dir, config)
    }
    
    /// Create cache with custom configuration
    pub fn with_config(cache_dir: &Path, config: CacheConfig) -> Result<Self> {
        let cache_directory = cache_dir.to_path_buf();
        let metadata_file = cache_directory.join("cache_metadata.json");
        
        // Create cache directory structure
        fs::create_dir_all(&cache_directory)
            .map_err(|e| CursedError::General(format!("Failed to create cache directory: {}", e)))?;
        
        for cache_type in [
            CacheType::CompiledObject,
            CacheType::LlvmIr,
            CacheType::AstSerialized,
            CacheType::PreprocessedSource,
            CacheType::DependencyInfo,
            CacheType::OptimizationMetadata,
        ] {
            let subdir = cache_directory.join(cache_type.file_extension());
            fs::create_dir_all(&subdir)
                .map_err(|e| CursedError::General(format!("Failed to create cache subdirectory: {}", e)))?;
        }
        
        // Load existing metadata
        let entries = if metadata_file.exists() {
            Self::load_metadata(&metadata_file)?
        } else {
            HashMap::new()
        };
        
        let mut cache = Self {
            cache_dir: cache_directory,
            config,
            entries,
            metadata_file,
            stats: CacheStats::default(),
        };
        
        // Update stats
        cache.recalculate_stats();
        
        // Clean expired entries
        cache.clean_expired_entries()?;
        
        Ok(cache)
    }
    
    /// Generate cache key for source file
    pub fn generate_key(
        &self,
        source_path: &Path,
        dependencies: &[PathBuf],
        flags: &[String],
        cache_type: CacheType,
    ) -> Result<String> {
        let mut hasher = Sha256::new();
        
        // Include source file path
        hasher.update(source_path.to_string_lossy().as_bytes());
        
        // Include source content hash
        let source_content = fs::read(source_path)
            .map_err(|e| CursedError::General(format!("Failed to read source file: {}", e)))?;
        hasher.update(&source_content);
        
        // Include dependencies
        for dep in dependencies {
            hasher.update(dep.to_string_lossy().as_bytes());
            if dep.exists() {
                let dep_content = fs::read(dep)
                    .map_err(|e| CursedError::General(format!("Failed to read dependency: {}", e)))?;
                hasher.update(&dep_content);
            }
        }
        
        // Include compilation flags
        for flag in flags {
            hasher.update(flag.as_bytes());
        }
        
        // Include cache type
        hasher.update(&[cache_type as u8]);
        
        let hash = hasher.finalize();
        Ok(format!("{:x}", hash))
    }
    
    /// Check if entry exists in cache
    pub fn contains(&mut self, key: &str) -> bool {
        if let Some(entry) = self.entries.get_mut(key) {
            // Update access info
            entry.last_accessed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            entry.access_count += 1;
            
            // Check if entry is still valid
            let cache_file = self.get_cache_file_path(key, entry.cache_type);
            if cache_file.exists() {
                self.stats.hits += 1;
                true
            } else {
                // File was deleted, remove entry
                self.entries.remove(key);
                self.stats.misses += 1;
                false
            }
        } else {
            self.stats.misses += 1;
            false
        }
    }
    
    /// Store data in cache
    pub fn store(
        &mut self,
        key: &str,
        data: &[u8],
        source_path: &Path,
        dependencies: &[PathBuf],
        flags: &[String],
        cache_type: CacheType,
    ) -> Result<()> {
        let cache_file = self.get_cache_file_path(key, cache_type);
        
        // Ensure parent directory exists
        if let Some(parent) = cache_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| CursedError::General(format!("Failed to create cache directory: {}", e)))?;
        }
        
        // Write data (with compression if enabled)
        let final_size = if self.config.compression_enabled && cache_type.should_compress() {
            self.write_compressed(&cache_file, data)?
        } else {
            fs::write(&cache_file, data)
                .map_err(|e| CursedError::General(format!("Failed to write cache file: {}", e)))?;
            data.len()
        };
        
        // Calculate hashes
        let source_hash = self.calculate_file_hash(source_path)?;
        let dependencies_hash = self.calculate_dependencies_hash(dependencies)?;
        
        // Create cache entry
        let entry = CacheEntry {
            key: key.to_string(),
            file_path: cache_file,
            source_hash,
            dependencies_hash,
            compilation_flags: flags.to_vec(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            last_accessed: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            access_count: 1,
            file_size: final_size,
            cache_type,
        };
        
        self.entries.insert(key.to_string(), entry);
        self.stats.entry_count += 1;
        self.stats.total_size_bytes += final_size;
        
        // Check if eviction is needed
        self.maybe_evict()?;
        
        // Save metadata
        self.save_metadata()?;
        
        Ok(())
    }
    
    /// Retrieve data from cache
    pub fn retrieve(&mut self, key: &str) -> Result<Option<Vec<u8>>> {
        if !self.contains(key) {
            return Ok(None);
        }
        
        let entry = self.entries.get(key).unwrap();
        let cache_file = &entry.file_path;
        
        // Read data (with decompression if needed)
        let data = if self.config.compression_enabled && entry.cache_type.should_compress() {
            self.read_compressed(cache_file)?
        } else {
            fs::read(cache_file)
                .map_err(|e| CursedError::General(format!("Failed to read cache file: {}", e)))?
        };
        
        Ok(Some(data))
    }
    
    /// Remove entry from cache
    pub fn remove(&mut self, key: &str) -> Result<bool> {
        if let Some(entry) = self.entries.remove(key) {
            // Remove file
            if entry.file_path.exists() {
                fs::remove_file(&entry.file_path)
                    .map_err(|e| CursedError::General(format!("Failed to remove cache file: {}", e)))?;
            }
            
            self.stats.entry_count -= 1;
            self.stats.total_size_bytes = self.stats.total_size_bytes.saturating_sub(entry.file_size);
            
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Clear all cache entries
    pub fn clear_all(&mut self) -> Result<()> {
        // Remove all files
        for entry in self.entries.values() {
            if entry.file_path.exists() {
                let _ = fs::remove_file(&entry.file_path);
            }
        }
        
        // Clear subdirectories
        for cache_type in [
            CacheType::CompiledObject,
            CacheType::LlvmIr,
            CacheType::AstSerialized,
            CacheType::PreprocessedSource,
            CacheType::DependencyInfo,
            CacheType::OptimizationMetadata,
        ] {
            let subdir = self.cache_dir.join(cache_type.file_extension());
            if subdir.exists() {
                let _ = fs::remove_dir_all(&subdir);
                let _ = fs::create_dir_all(&subdir);
            }
        }
        
        self.entries.clear();
        self.stats = CacheStats::default();
        
        // Save empty metadata
        self.save_metadata()?;
        
        Ok(())
    }
    
    /// Get cache statistics
    pub fn get_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        stats.insert("hits".to_string(), self.stats.hits);
        stats.insert("misses".to_string(), self.stats.misses);
        stats.insert("hit_rate".to_string(), 
            if self.stats.hits + self.stats.misses > 0 {
                (100 * self.stats.hits) / (self.stats.hits + self.stats.misses)
            } else {
                0
            });
        stats.insert("evictions".to_string(), self.stats.evictions);
        stats.insert("entry_count".to_string(), self.stats.entry_count);
        stats.insert("total_size_mb".to_string(), self.stats.total_size_bytes / 1024 / 1024);
        stats.insert("compression_ratio".to_string(), (self.stats.compression_ratio * 100.0) as usize);
        
        // Cache type breakdown
        for cache_type in [
            CacheType::CompiledObject,
            CacheType::LlvmIr,
            CacheType::AstSerialized,
            CacheType::PreprocessedSource,
            CacheType::DependencyInfo,
            CacheType::OptimizationMetadata,
        ] {
            let count = self.entries.values()
                .filter(|e| e.cache_type == cache_type)
                .count();
            stats.insert(format!("{:?}_count", cache_type).to_lowercase(), count);
        }
        
        stats
    }
    
    /// Get cache file path for key and type
    fn get_cache_file_path(&self, key: &str, cache_type: CacheType) -> PathBuf {
        let subdir = self.cache_dir.join(cache_type.file_extension());
        let filename = if self.config.compression_enabled && cache_type.should_compress() {
            format!("{}.{}.gz", key, cache_type.file_extension())
        } else {
            format!("{}.{}", key, cache_type.file_extension())
        };
        subdir.join(filename)
    }
    
    /// Write compressed data
    fn write_compressed(&self, path: &Path, data: &[u8]) -> Result<usize> {
        let file = File::create(path)
            .map_err(|e| CursedError::General(format!("Failed to create compressed file: {}", e)))?;
        
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(data)
            .map_err(|e| CursedError::General(format!("Failed to write compressed data: {}", e)))?;
        
        encoder.finish()
            .map_err(|e| CursedError::General(format!("Failed to finalize compressed file: {}", e)))
            .map(|_| {
                // Return compressed size
                path.metadata().map(|m| m.len() as usize).unwrap_or(data.len())
            })
    }
    
    /// Read compressed data
    fn read_compressed(&self, path: &Path) -> Result<Vec<u8>> {
        let file = File::open(path)
            .map_err(|e| CursedError::General(format!("Failed to open compressed file: {}", e)))?;
        
        let mut decoder = GzDecoder::new(file);
        let mut data = Vec::new();
        decoder.read_to_end(&mut data)
            .map_err(|e| CursedError::General(format!("Failed to decompress data: {}", e)))?;
        
        Ok(data)
    }
    
    /// Calculate file hash
    fn calculate_file_hash(&self, file_path: &Path) -> Result<String> {
        let content = fs::read(file_path)
            .map_err(|e| CursedError::General(format!("Failed to read file for hashing: {}", e)))?;
        
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash = hasher.finalize();
        
        Ok(format!("{:x}", hash))
    }
    
    /// Calculate dependencies hash
    fn calculate_dependencies_hash(&self, dependencies: &[PathBuf]) -> Result<String> {
        let mut hasher = Sha256::new();
        
        for dep in dependencies {
            hasher.update(dep.to_string_lossy().as_bytes());
            if dep.exists() {
                let content = fs::read(dep)
                    .map_err(|e| CursedError::General(format!("Failed to read dependency: {}", e)))?;
                hasher.update(&content);
            }
        }
        
        let hash = hasher.finalize();
        Ok(format!("{:x}", hash))
    }
    
    /// Clean expired entries
    fn clean_expired_entries(&mut self) -> Result<()> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let ttl_seconds = self.config.ttl_hours as u64 * 3600;
        let mut expired_keys = Vec::new();
        
        for (key, entry) in &self.entries {
            if current_time.saturating_sub(entry.last_accessed) > ttl_seconds {
                expired_keys.push(key.clone());
            }
        }
        
        for key in expired_keys {
            self.remove(&key)?;
            self.stats.evictions += 1;
        }
        
        Ok(())
    }
    
    /// Maybe evict entries based on cache limits
    fn maybe_evict(&mut self) -> Result<()> {
        // Check size limit
        let max_size_bytes = self.config.max_size_mb * 1024 * 1024;
        
        // Check entry count limit
        while self.stats.entry_count > self.config.max_entries 
            || self.stats.total_size_bytes > max_size_bytes {
            
            if let Some(key_to_evict) = self.select_eviction_candidate() {
                self.remove(&key_to_evict)?;
                self.stats.evictions += 1;
            } else {
                break;
            }
        }
        
        Ok(())
    }
    
    /// Select candidate for eviction based on strategy
    fn select_eviction_candidate(&self) -> Option<String> {
        match self.config.eviction_strategy {
            EvictionStrategy::LeastRecentlyUsed => {
                self.entries.iter()
                    .min_by_key(|(_, entry)| entry.last_accessed)
                    .map(|(key, _)| key.clone())
            }
            EvictionStrategy::LeastFrequentlyUsed => {
                self.entries.iter()
                    .min_by_key(|(_, entry)| entry.access_count)
                    .map(|(key, _)| key.clone())
            }
            EvictionStrategy::FirstInFirstOut => {
                self.entries.iter()
                    .min_by_key(|(_, entry)| entry.created_at)
                    .map(|(key, _)| key.clone())
            }
            EvictionStrategy::SizeBasedPriority => {
                // Evict largest files first
                self.entries.iter()
                    .max_by_key(|(_, entry)| entry.file_size)
                    .map(|(key, _)| key.clone())
            }
        }
    }
    
    /// Recalculate cache statistics
    fn recalculate_stats(&mut self) {
        self.stats.entry_count = self.entries.len();
        self.stats.total_size_bytes = self.entries.values()
            .map(|entry| entry.file_size)
            .sum();
        
        // Calculate compression ratio
        let compressed_entries: Vec<_> = self.entries.values()
            .filter(|e| self.config.compression_enabled && e.cache_type.should_compress())
            .collect();
        
        if !compressed_entries.is_empty() {
            // This is a simplified calculation; in practice, you'd compare
            // original vs compressed sizes
            self.stats.compression_ratio = 0.7; // Assume 30% compression
        }
    }
    
    /// Load cache metadata
    fn load_metadata(path: &Path) -> Result<HashMap<String, CacheEntry>> {
        let file = File::open(path)
            .map_err(|e| CursedError::General(format!("Failed to open metadata file: {}", e)))?;
        
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
            .map_err(|e| CursedError::General(format!("Failed to parse metadata: {}", e)))
    }
    
    /// Save cache metadata
    fn save_metadata(&self) -> Result<()> {
        let file = File::create(&self.metadata_file)
            .map_err(|e| CursedError::General(format!("Failed to create metadata file: {}", e)))?;
        
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.entries)
            .map_err(|e| CursedError::General(format!("Failed to write metadata: {}", e)))?;
        
        Ok(())
    }
}

