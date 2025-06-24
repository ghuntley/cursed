// Compilation Cache for Distributed System
//
// Manages caching of compilation artifacts across the distributed system
// to avoid redundant compilation work and improve overall performance.

use crate::error::{Error as CursedError, Result};

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{debug, error, info, instrument, warn};

/// Cache entry containing compilation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Associated job ID
    pub job_id: String,
    /// Compiled output
    pub output: Vec<u8>,
    /// When this entry was created
    pub created_at: SystemTime,
    /// Number of times this entry has been accessed
    pub access_count: usize,
}

/// Cache strategy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheStrategy {
    /// Least Recently Used with Time-To-Live
    LruWithTtl {
        max_entries: usize,
        ttl: Duration,
    },
    /// Least Frequently Used
    LFU {
        max_entries: usize,
    },
    /// Time-based expiration only
    TimeOnly {
        ttl: Duration,
    },
    /// Size-based cache with LRU eviction
    SizeLimited {
        max_size_bytes: usize,
    },
    /// Adaptive strategy that changes based on usage patterns
    Adaptive {
        max_entries: usize,
        max_size_bytes: usize,
        ttl: Duration,
    },
}

impl Default for CacheStrategy {
    fn default() -> Self {
        Self::LruWithTtl {
            max_entries: 10000,
            ttl: Duration::from_secs(24 * 60 * 60), // 24 hours
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Caching strategy
    pub strategy: CacheStrategy,
    /// Persistent storage directory
    pub storage_dir: Option<PathBuf>,
    /// Enable persistent storage
    pub persistent: bool,
    /// Enable distributed cache sharing
    pub distributed: bool,
    /// Cache warming on startup
    pub warm_on_startup: bool,
    /// Compression for cache entries
    pub compression_enabled: bool,
    /// Maximum individual entry size
    pub max_entry_size: usize,
    /// Enable cache statistics
    pub stats_enabled: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            strategy: CacheStrategy::default(),
            storage_dir: Some(PathBuf::from("./cache")),
            persistent: true,
            distributed: false,
            warm_on_startup: true,
            compression_enabled: true,
            max_entry_size: 100 * 1024 * 1024, // 100MB
            stats_enabled: true,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// Total cache hits
    pub hits: usize,
    /// Total cache misses
    pub misses: usize,
    /// Cache hit rate (0.0 - 1.0)
    pub hit_rate: f64,
    /// Number of entries currently in cache
    pub entry_count: usize,
    /// Total size of cache in bytes
    pub total_size_bytes: usize,
    /// Number of evictions
    pub evictions: usize,
    /// Number of entries loaded from persistent storage
    pub loads_from_disk: usize,
    /// Number of entries saved to persistent storage
    pub saves_to_disk: usize,
    /// Average entry access time
    pub average_access_time: Duration,
    /// Cache efficiency score (0.0 - 1.0)
    pub efficiency_score: f64,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            hit_rate: 0.0,
            entry_count: 0,
            total_size_bytes: 0,
            evictions: 0,
            loads_from_disk: 0,
            saves_to_disk: 0,
            average_access_time: Duration::ZERO,
            efficiency_score: 0.0,
        }
    }
}

/// Cache entry metadata for management
#[derive(Debug, Clone)]
struct CacheEntryMetadata {
    key: String,
    size_bytes: usize,
    last_accessed: SystemTime,
    access_count: usize,
    created_at: SystemTime,
    frequency_score: f64,
}

/// Compilation cache manager
pub struct CompilationCache {
    config: CacheConfig,
    entries: Arc<RwLock<HashMap<String, CacheEntry>>>,
    metadata: Arc<RwLock<HashMap<String, CacheEntryMetadata>>>,
    access_order: Arc<RwLock<VecDeque<String>>>, // LRU tracking
    stats: Arc<RwLock<CacheStats>>,
    is_enabled: bool,
}

impl CompilationCache {
    /// Create a new compilation cache
    #[instrument]
    pub fn new(strategy: CacheStrategy) -> Result<Self> {
        let config = CacheConfig {
            strategy,
            ..CacheConfig::default()
        };

        Ok(Self {
            config,
            entries: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(HashMap::new())),
            access_order: Arc::new(RwLock::new(VecDeque::new())),
            stats: Arc::new(RwLock::new(CacheStats::default())),
            is_enabled: true,
        })
    }

    /// Create a disabled cache (no-op)
    pub fn disabled() -> Self {
        Self {
            config: CacheConfig::default(),
            entries: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(HashMap::new())),
            access_order: Arc::new(RwLock::new(VecDeque::new())),
            stats: Arc::new(RwLock::new(CacheStats::default())),
            is_enabled: false,
        }
    }

    /// Initialize the cache
    #[instrument(skip(self))]
    pub async fn initialize(&mut self) -> Result<()> {
        if !self.is_enabled {
            return Ok(());
        }

        // Create storage directory if persistent storage is enabled
        if self.config.persistent {
            if let Some(storage_dir) = &self.config.storage_dir {
                fs::create_dir_all(storage_dir).await
                    .map_err(|e| CursedError::system_error(&format!("Failed to create cache directory: {}", e)))?;
            }
        }

        // Load existing entries if warming is enabled
        if self.config.warm_on_startup {
            self.warm_cache().await?;
        }

        info!("Compilation cache initialized");
        Ok(())
    }

    /// Shutdown the cache
    #[instrument(skip(self))]
    pub async fn shutdown(&mut self) -> Result<()> {
        if !self.is_enabled {
            return Ok(());
        }

        // Save all entries to persistent storage
        if self.config.persistent {
            self.save_all_to_disk().await?;
        }

        info!("Compilation cache shutdown");
        Ok(())
    }

    /// Get an entry from the cache
    #[instrument(skip(self))]
    pub async fn get(&self, key: &str) -> Result<Option<CacheEntry>> {
        if !self.is_enabled {
            return Ok(None);
        }

        let start_time = std::time::Instant::now();

        // Check memory cache first
        let entry = {
            let entries = self.entries.read()
                .map_err(|_| CursedError::system_error("Failed to lock entries"))?;
            entries.get(key).cloned()
        };

        if let Some(mut entry) = entry {
            // Update access tracking
            self.update_access_tracking(key).await?;
            
            // Update statistics
            self.update_hit_stats(start_time.elapsed()).await?;
            
            entry.access_count += 1;
            debug!(key, "Cache hit");
            return Ok(Some(entry));
        }

        // Try loading from persistent storage
        if self.config.persistent {
            if let Some(entry) = self.load_from_disk(key).await? {
                // Store in memory cache
                self.put_internal(key.to_string(), entry.clone()).await?;
                
                // Update statistics
                self.update_hit_stats(start_time.elapsed()).await?;
                self.update_disk_load_stats().await?;
                
                debug!(key, "Cache hit from disk");
                return Ok(Some(entry));
            }
        }

        // Cache miss
        self.update_miss_stats().await?;
        debug!(key, "Cache miss");
        Ok(None)
    }

    /// Put an entry into the cache
    #[instrument(skip(self, entry))]
    pub async fn put(&self, key: String, entry: CacheEntry) -> Result<()> {
        if !self.is_enabled {
            return Ok(());
        }

        // Check entry size
        let entry_size = self.estimate_entry_size(&entry);
        if entry_size > self.config.max_entry_size {
            warn!(key, size = entry_size, "Entry too large for cache");
            return Ok(());
        }

        self.put_internal(key, entry).await
    }

    /// Internal put implementation
    pub async fn put_internal(&self, key: String, entry: CacheEntry) -> Result<()> {
        let entry_size = self.estimate_entry_size(&entry);

        // Check if eviction is needed
        self.maybe_evict(&key, entry_size).await?;

        // Add to memory cache
        {
            let mut entries = self.entries.write()
                .map_err(|_| CursedError::system_error("Failed to lock entries"))?;
            entries.insert(key.clone(), entry.clone());
        }

        // Update metadata
        {
            let mut metadata = self.metadata.write()
                .map_err(|_| CursedError::system_error("Failed to lock metadata"))?;
            
            let meta = CacheEntryMetadata {
                key: key.clone(),
                size_bytes: entry_size,
                last_accessed: SystemTime::now(),
                access_count: 1,
                created_at: entry.created_at,
                frequency_score: 1.0,
            };
            
            metadata.insert(key.clone(), meta);
        }

        // Update access order for LRU
        {
            let mut access_order = self.access_order.write()
                .map_err(|_| CursedError::system_error("Failed to lock access order"))?;
            access_order.push_back(key.clone());
        }

        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
            stats.entry_count += 1;
            stats.total_size_bytes += entry_size;
        }

        // Save to persistent storage if enabled
        if self.config.persistent {
            self.save_to_disk(&key, &entry).await?;
        }

        debug!(key, size = entry_size, "Entry cached");
        Ok(())
    }

    /// Remove an entry from the cache
    #[instrument(skip(self))]
    pub async fn remove(&self, key: &str) -> Result<Option<CacheEntry>> {
        if !self.is_enabled {
            return Ok(None);
        }

        let removed_entry = {
            let mut entries = self.entries.write()
                .map_err(|_| CursedError::system_error("Failed to lock entries"))?;
            entries.remove(key)
        };

        if removed_entry.is_some() {
            // Remove metadata
            let entry_size = {
                let mut metadata = self.metadata.write()
                    .map_err(|_| CursedError::system_error("Failed to lock metadata"))?;
                metadata.remove(key).map(|m| m.size_bytes).unwrap_or(0)
            };

            // Remove from access order
            {
                let mut access_order = self.access_order.write()
                    .map_err(|_| CursedError::system_error("Failed to lock access order"))?;
                access_order.retain(|k| k != key);
            }

            // Update statistics
            {
                let mut stats = self.stats.write()
                    .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
                stats.entry_count = stats.entry_count.saturating_sub(1);
                stats.total_size_bytes = stats.total_size_bytes.saturating_sub(entry_size);
            }

            // Remove from persistent storage
            if self.config.persistent {
                self.remove_from_disk(key).await?;
            }

            debug!(key, "Entry removed from cache");
        }

        Ok(removed_entry)
    }

    /// Clear the entire cache
    #[instrument(skip(self))]
    pub async fn clear(&self) -> Result<()> {
        if !self.is_enabled {
            return Ok(());
        }

        {
            let mut entries = self.entries.write()
                .map_err(|_| CursedError::system_error("Failed to lock entries"))?;
            entries.clear();
        }

        {
            let mut metadata = self.metadata.write()
                .map_err(|_| CursedError::system_error("Failed to lock metadata"))?;
            metadata.clear();
        }

        {
            let mut access_order = self.access_order.write()
                .map_err(|_| CursedError::system_error("Failed to lock access order"))?;
            access_order.clear();
        }

        {
            let mut stats = self.stats.write()
                .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
            stats.entry_count = 0;
            stats.total_size_bytes = 0;
        }

        // Clear persistent storage
        if self.config.persistent {
            self.clear_disk_storage().await?;
        }

        info!("Cache cleared");
        Ok(())
    }

    /// Get current cache statistics
    pub async fn get_stats(&self) -> Result<CacheStats> {
        let stats = self.stats.read()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        Ok(stats.clone())
    }

    /// Check if an entry exists in cache
    pub async fn contains(&self, key: &str) -> Result<bool> {
        if !self.is_enabled {
            return Ok(false);
        }

        let entries = self.entries.read()
            .map_err(|_| CursedError::system_error("Failed to lock entries"))?;
        Ok(entries.contains_key(key))
    }

    /// Get current cache size in bytes
    pub async fn size_bytes(&self) -> Result<usize> {
        let stats = self.stats.read()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        Ok(stats.total_size_bytes)
    }

    /// Get current entry count
    pub async fn entry_count(&self) -> Result<usize> {
        let stats = self.stats.read()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        Ok(stats.entry_count)
    }

    /// Update access tracking for an entry
    pub async fn update_access_tracking(&self, key: &str) -> Result<()> {
        // Update metadata
        {
            let mut metadata = self.metadata.write()
                .map_err(|_| CursedError::system_error("Failed to lock metadata"))?;
            
            if let Some(meta) = metadata.get_mut(key) {
                meta.last_accessed = SystemTime::now();
                meta.access_count += 1;
                meta.frequency_score = meta.frequency_score * 0.9 + 1.0; // Exponential decay
            }
        }

        // Update LRU order
        {
            let mut access_order = self.access_order.write()
                .map_err(|_| CursedError::system_error("Failed to lock access order"))?;
            
            // Remove from current position and add to end
            access_order.retain(|k| k != key);
            access_order.push_back(key.to_string());
        }

        Ok(())
    }

    /// Check if eviction is needed and perform it
    pub async fn maybe_evict(&self, new_key: &str, new_entry_size: usize) -> Result<()> {
        match &self.config.strategy {
            CacheStrategy::LruWithTtl { max_entries, ttl } => {
                self.evict_expired(*ttl).await?;
                self.evict_lru_if_needed(*max_entries, new_entry_size).await?;
            }
            CacheStrategy::LFU { max_entries } => {
                self.evict_lfu_if_needed(*max_entries, new_entry_size).await?;
            }
            CacheStrategy::TimeOnly { ttl } => {
                self.evict_expired(*ttl).await?;
            }
            CacheStrategy::SizeLimited { max_size_bytes } => {
                self.evict_size_limited(*max_size_bytes, new_entry_size).await?;
            }
            CacheStrategy::Adaptive { max_entries, max_size_bytes, ttl } => {
                self.evict_expired(*ttl).await?;
                self.evict_adaptive(*max_entries, *max_size_bytes, new_entry_size).await?;
            }
        }

        Ok(())
    }

    /// Evict expired entries
    pub async fn evict_expired(&self, ttl: Duration) -> Result<()> {
        let now = SystemTime::now();
        let mut keys_to_remove = Vec::new();

        {
            let metadata = self.metadata.read()
                .map_err(|_| CursedError::system_error("Failed to lock metadata"))?;
            
            for (key, meta) in metadata.iter() {
                if let Ok(age) = now.duration_since(meta.created_at) {
                    if age > ttl {
                        keys_to_remove.push(key.clone());
                    }
                }
            }
        }

        for key in keys_to_remove {
            self.remove(&key).await?;
            self.update_eviction_stats().await?;
        }

        Ok(())
    }

    /// Evict using LRU strategy if needed
    pub async fn evict_lru_if_needed(&self, max_entries: usize, _new_entry_size: usize) -> Result<()> {
        let current_count = {
            let stats = self.stats.read()
                .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
            stats.entry_count
        };

        if current_count >= max_entries {
            let keys_to_remove = {
                let access_order = self.access_order.read()
                    .map_err(|_| CursedError::system_error("Failed to lock access order"))?;
                
                let remove_count = current_count - max_entries + 1;
                access_order.iter().take(remove_count).cloned().collect::<Vec<_>>()
            };

            for key in keys_to_remove {
                self.remove(&key).await?;
                self.update_eviction_stats().await?;
            }
        }

        Ok(())
    }

    /// Evict using LFU strategy if needed
    pub async fn evict_lfu_if_needed(&self, max_entries: usize, _new_entry_size: usize) -> Result<()> {
        let current_count = {
            let stats = self.stats.read()
                .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
            stats.entry_count
        };

        if current_count >= max_entries {
            let key_to_remove = {
                let metadata = self.metadata.read()
                    .map_err(|_| CursedError::system_error("Failed to lock metadata"))?;
                
                metadata.iter()
                    .min_by(|a, b| a.1.frequency_score.partial_cmp(&b.1.frequency_score).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(k, _)| k.clone())
            };

            if let Some(key) = key_to_remove {
                self.remove(&key).await?;
                self.update_eviction_stats().await?;
            }
        }

        Ok(())
    }

    /// Evict based on size limits
    pub async fn evict_size_limited(&self, max_size_bytes: usize, new_entry_size: usize) -> Result<()> {
        let current_size = {
            let stats = self.stats.read()
                .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
            stats.total_size_bytes
        };

        if current_size + new_entry_size > max_size_bytes {
            // Use LRU eviction to free space
            let target_size = max_size_bytes - new_entry_size;
            let mut freed_size = current_size;

            let keys_to_remove = {
                let access_order = self.access_order.read()
                    .map_err(|_| CursedError::system_error("Failed to lock access order"))?;
                let metadata = self.metadata.read()
                    .map_err(|_| CursedError::system_error("Failed to lock metadata"))?;
                
                let mut keys = Vec::new();
                for key in access_order.iter() {
                    if freed_size <= target_size {
                        break;
                    }
                    if let Some(meta) = metadata.get(key) {
                        freed_size -= meta.size_bytes;
                        keys.push(key.clone());
                    }
                }
                keys
            };

            for key in keys_to_remove {
                self.remove(&key).await?;
                self.update_eviction_stats().await?;
            }
        }

        Ok(())
    }

    /// Evict using adaptive strategy
    pub async fn evict_adaptive(&self, max_entries: usize, max_size_bytes: usize, new_entry_size: usize) -> Result<()> {
        // Combine multiple strategies
        self.evict_lru_if_needed(max_entries, new_entry_size).await?;
        self.evict_size_limited(max_size_bytes, new_entry_size).await?;
        Ok(())
    }

    /// Estimate the size of a cache entry
    pub fn estimate_entry_size(&self, entry: &CacheEntry) -> usize {
        // Basic size estimation
        entry.output.len() + 
        entry.job_id.len() + 
        std::mem::size_of::<SystemTime>() + 
        std::mem::size_of::<usize>()
    }

    /// Warm the cache by loading existing entries
    pub async fn warm_cache(&self) -> Result<()> {
        if !self.config.persistent {
            return Ok(());
        }

        let storage_dir = self.config.storage_dir.as_ref()
            .ok_or_else(|| CursedError::system_error("No storage directory configured"))?;

        if !storage_dir.exists() {
            return Ok(());
        }

        let mut entries = fs::read_dir(storage_dir).await
            .map_err(|e| CursedError::system_error(&format!("Failed to read cache directory: {}", e)))?;

        let mut loaded_count = 0;
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| CursedError::system_error(&format!("Failed to read directory entry: {}", e)))? {
            
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".cache") {
                    let key = file_name.strip_suffix(".cache").unwrap();
                    if let Ok(Some(cache_entry)) = self.load_from_disk(key).await {
                        let _ = self.put_internal(key.to_string(), cache_entry).await;
                        loaded_count += 1;
                    }
                }
            }
        }

        info!(entries_loaded = loaded_count, "Cache warmed");
        Ok(())
    }

    /// Save an entry to disk
    pub async fn save_to_disk(&self, key: &str, entry: &CacheEntry) -> Result<()> {
        let storage_dir = self.config.storage_dir.as_ref()
            .ok_or_else(|| CursedError::system_error("No storage directory configured"))?;

        let file_path = storage_dir.join(format!("{}.cache", key));
        
        let serialized = bincode::serialize(entry)
            .map_err(|e| CursedError::system_error(&format!("Failed to serialize cache entry: {}", e)))?;

        let mut file = fs::File::create(&file_path).await
            .map_err(|e| CursedError::system_error(&format!("Failed to create cache file: {}", e)))?;

        file.write_all(&serialized).await
            .map_err(|e| CursedError::system_error(&format!("Failed to write cache file: {}", e)))?;

        file.sync_all().await
            .map_err(|e| CursedError::system_error(&format!("Failed to sync cache file: {}", e)))?;

        // Update statistics
        self.update_disk_save_stats().await?;

        Ok(())
    }

    /// Load an entry from disk
    pub async fn load_from_disk(&self, key: &str) -> Result<Option<CacheEntry>> {
        let storage_dir = self.config.storage_dir.as_ref()
            .ok_or_else(|| CursedError::system_error("No storage directory configured"))?;

        let file_path = storage_dir.join(format!("{}.cache", key));
        
        if !file_path.exists() {
            return Ok(None);
        }

        let mut file = fs::File::open(&file_path).await
            .map_err(|e| CursedError::system_error(&format!("Failed to open cache file: {}", e)))?;

        let mut data = Vec::new();
        file.read_to_end(&mut data).await
            .map_err(|e| CursedError::system_error(&format!("Failed to read cache file: {}", e)))?;

        let entry = bincode::deserialize(&data)
            .map_err(|e| CursedError::system_error(&format!("Failed to deserialize cache entry: {}", e)))?;

        Ok(Some(entry))
    }

    /// Remove an entry from disk
    pub async fn remove_from_disk(&self, key: &str) -> Result<()> {
        let storage_dir = self.config.storage_dir.as_ref()
            .ok_or_else(|| CursedError::system_error("No storage directory configured"))?;

        let file_path = storage_dir.join(format!("{}.cache", key));
        
        if file_path.exists() {
            fs::remove_file(&file_path).await
                .map_err(|e| CursedError::system_error(&format!("Failed to remove cache file: {}", e)))?;
        }

        Ok(())
    }

    /// Save all entries to disk
    pub async fn save_all_to_disk(&self) -> Result<()> {
        let entries = {
            let entries = self.entries.read()
                .map_err(|_| CursedError::system_error("Failed to lock entries"))?;
            entries.clone()
        };

        for (key, entry) in entries {
            if let Err(e) = self.save_to_disk(&key, &entry).await {
                warn!(key, error = ?e, "Failed to save cache entry to disk");
            }
        }

        Ok(())
    }

    /// Clear disk storage
    pub async fn clear_disk_storage(&self) -> Result<()> {
        let storage_dir = self.config.storage_dir.as_ref()
            .ok_or_else(|| CursedError::system_error("No storage directory configured"))?;

        if storage_dir.exists() {
            fs::remove_dir_all(storage_dir).await
                .map_err(|e| CursedError::system_error(&format!("Failed to clear cache directory: {}", e)))?;
            
            fs::create_dir_all(storage_dir).await
                .map_err(|e| CursedError::system_error(&format!("Failed to recreate cache directory: {}", e)))?;
        }

        Ok(())
    }

    /// Update hit statistics
    pub async fn update_hit_stats(&self, access_time: Duration) -> Result<()> {
        let mut stats = self.stats.write()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        
        stats.hits += 1;
        let total_accesses = stats.hits + stats.misses;
        stats.hit_rate = stats.hits as f64 / total_accesses as f64;
        
        // Update average access time
        if stats.hits > 1 {
            let weight = 1.0 / stats.hits as f64;
            let old_avg = stats.average_access_time.as_nanos() as f64;
            let new_avg = old_avg * (1.0 - weight) + access_time.as_nanos() as f64 * weight;
            stats.average_access_time = Duration::from_nanos(new_avg as u64);
        } else {
            stats.average_access_time = access_time;
        }

        // Update efficiency score
        stats.efficiency_score = stats.hit_rate * 0.7 + (1.0 - stats.average_access_time.as_secs_f64() / 0.001) * 0.3;
        stats.efficiency_score = stats.efficiency_score.max(0.0).min(1.0);

        Ok(())
    }

    /// Update miss statistics
    pub async fn update_miss_stats(&self) -> Result<()> {
        let mut stats = self.stats.write()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        
        stats.misses += 1;
        let total_accesses = stats.hits + stats.misses;
        stats.hit_rate = stats.hits as f64 / total_accesses as f64;

        Ok(())
    }

    /// Update eviction statistics
    pub async fn update_eviction_stats(&self) -> Result<()> {
        let mut stats = self.stats.write()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        stats.evictions += 1;
        Ok(())
    }

    /// Update disk load statistics
    pub async fn update_disk_load_stats(&self) -> Result<()> {
        let mut stats = self.stats.write()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        stats.loads_from_disk += 1;
        Ok(())
    }

    /// Update disk save statistics
    pub async fn update_disk_save_stats(&self) -> Result<()> {
        let mut stats = self.stats.write()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        stats.saves_to_disk += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_entry(id: &str, output: &[u8]) -> CacheEntry {
        CacheEntry {
            job_id: id.to_string(),
            output: output.to_vec(),
            created_at: SystemTime::now(),
            access_count: 0,
        }
    }

    #[tokio::test]
    async fn test_cache_creation() {
        let cache = CompilationCache::new(CacheStrategy::default());
        assert!(cache.is_ok());
    }

    #[tokio::test]
    async fn test_cache_put_get() {
        let cache = CompilationCache::new(CacheStrategy::default()).unwrap();
        let entry = create_test_entry("test_job", b"compiled output");
        
        let put_result = cache.put("test_key".to_string(), entry.clone()).await;
        assert!(put_result.is_ok());
        
        let get_result = cache.get("test_key").await;
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let cache = CompilationCache::new(CacheStrategy::default()).unwrap();
        
        let get_result = cache.get("nonexistent_key").await;
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_cache_removal() {
        let cache = CompilationCache::new(CacheStrategy::default()).unwrap();
        let entry = create_test_entry("test_job", b"compiled output");
        
        cache.put("test_key".to_string(), entry).await.unwrap();
        assert!(cache.contains("test_key").await.unwrap());
        
        let removed = cache.remove("test_key").await.unwrap();
        assert!(removed.is_some());
        assert!(!cache.contains("test_key").await.unwrap());
    }

    #[tokio::test]
    async fn test_lru_eviction() {
        let strategy = CacheStrategy::LruWithTtl {
            max_entries: 2,
            ttl: Duration::from_secs(3600),
        };
        let cache = CompilationCache::new(strategy).unwrap();
        
        // Fill cache to capacity
        cache.put("key1".to_string(), create_test_entry("job1", b"output1")).await.unwrap();
        cache.put("key2".to_string(), create_test_entry("job2", b"output2")).await.unwrap();
        
        // Add one more - should evict oldest
        cache.put("key3".to_string(), create_test_entry("job3", b"output3")).await.unwrap();
        
        // key1 should be evicted, key2 and key3 should remain
        assert!(!cache.contains("key1").await.unwrap());
        assert!(cache.contains("key2").await.unwrap());
        assert!(cache.contains("key3").await.unwrap());
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let cache = CompilationCache::new(CacheStrategy::default()).unwrap();
        let entry = create_test_entry("test_job", b"compiled output");
        
        // Initial stats
        let stats = cache.get_stats().await.unwrap();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        
        // Add entry and test hit
        cache.put("test_key".to_string(), entry).await.unwrap();
        cache.get("test_key").await.unwrap();
        
        let stats = cache.get_stats().await.unwrap();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.entry_count, 1);
        
        // Test miss
        cache.get("nonexistent").await.unwrap();
        
        let stats = cache.get_stats().await.unwrap();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hit_rate, 0.5);
    }

    #[tokio::test]
    async fn test_disabled_cache() {
        let cache = CompilationCache::disabled();
        let entry = create_test_entry("test_job", b"compiled output");
        
        // Operations should succeed but do nothing
        assert!(cache.put("test_key".to_string(), entry).await.is_ok());
        assert!(cache.get("test_key").await.unwrap().is_none());
        assert!(!cache.contains("test_key").await.unwrap());
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache = CompilationCache::new(CacheStrategy::default()).unwrap();
        
        // Add some entries
        cache.put("key1".to_string(), create_test_entry("job1", b"output1")).await.unwrap();
        cache.put("key2".to_string(), create_test_entry("job2", b"output2")).await.unwrap();
        
        assert_eq!(cache.entry_count().await.unwrap(), 2);
        
        // Clear cache
        cache.clear().await.unwrap();
        
        assert_eq!(cache.entry_count().await.unwrap(), 0);
        assert!(!cache.contains("key1").await.unwrap());
        assert!(!cache.contains("key2").await.unwrap());
    }
}
