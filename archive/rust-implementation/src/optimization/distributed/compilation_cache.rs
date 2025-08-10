use crate::error::{Result, CursedError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Cache entry for compiled results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub id: String,
    pub key: String,
    pub output: Vec<u8>,
    pub metadata: CacheMetadata,
    pub created_at: SystemTime,
    pub last_accessed: SystemTime,
    pub access_count: u32,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    pub source_files: Vec<PathBuf>,
    pub optimization_level: u32,
    pub target_platform: String,
    pub compiler_version: String,
    pub compilation_time: Duration,
    pub worker_id: String,
}

impl CacheEntry {
    pub fn new(key: String, output: Vec<u8>, metadata: CacheMetadata) -> Self {
        let now = SystemTime::now();
        Self {
            id: Uuid::new_v4().to_string(),
            key,
            size_bytes: output.len() as u64,
            output,
            metadata,
            created_at: now,
            last_accessed: now,
            access_count: 1,
        }
    }

    pub fn touch(&mut self) {
        self.last_accessed = SystemTime::now();
        self.access_count += 1;
    }

    pub fn age(&self) -> Duration {
        self.created_at.elapsed().unwrap_or(Duration::from_secs(0))
    }

    pub fn is_expired(&self, ttl: Duration) -> bool {
        self.age() > ttl
    }
}

/// Cache strategy for managing entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheStrategy {
    /// Least Recently Used with Time-To-Live
    LruWithTtl {
        max_entries: usize,
        ttl: Duration,
    },
    /// Least Frequently Used
    Lfu {
        max_entries: usize,
    },
    /// Size-based eviction
    SizeBased {
        max_size_bytes: u64,
    },
    /// Hybrid strategy combining multiple approaches
    Hybrid {
        max_entries: usize,
        max_size_bytes: u64,
        ttl: Duration,
    },
}

impl Default for CacheStrategy {
    fn default() -> Self {
        CacheStrategy::LruWithTtl {
            max_entries: 1000,
            ttl: Duration::from_secs(24 * 60 * 60), // 24 hours
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_entries: usize,
    pub total_size_bytes: u64,
    pub hit_count: u64,
    pub miss_count: u64,
    pub eviction_count: u64,
    pub hit_rate: f64,
    pub average_entry_size: f64,
    pub oldest_entry_age: Duration,
    pub newest_entry_age: Duration,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            total_entries: 0,
            total_size_bytes: 0,
            hit_count: 0,
            miss_count: 0,
            eviction_count: 0,
            hit_rate: 0.0,
            average_entry_size: 0.0,
            oldest_entry_age: Duration::from_secs(0),
            newest_entry_age: Duration::from_secs(0),
        }
    }
}

/// Compilation cache for distributed system
#[derive(Debug)]
pub struct CompilationCache {
    strategy: CacheStrategy,
    entries: RwLock<HashMap<String, CacheEntry>>,
    stats: RwLock<CacheStats>,
    cache_dir: Option<PathBuf>,
    enabled: bool,
}

impl CompilationCache {
    pub fn new(strategy: CacheStrategy) -> Result<Self> {
        Ok(Self {
            strategy,
            entries: RwLock::new(HashMap::new()),
            stats: RwLock::new(CacheStats::default()),
            cache_dir: None,
            enabled: true,
        })
    }

    pub fn disabled() -> Self {
        Self {
            strategy: CacheStrategy::default(),
            entries: RwLock::new(HashMap::new()),
            stats: RwLock::new(CacheStats::default()),
            cache_dir: None,
            enabled: false,
        }
    }

    pub fn with_persistent_storage(mut self, cache_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&cache_dir)?;
        self.cache_dir = Some(cache_dir);
        Ok(self)
    }

    pub async fn initialize(&mut self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        tracing::info!("Initializing compilation cache with strategy: {:?}", self.strategy);
        
        // Load persistent cache if available
        if let Some(cache_dir) = &self.cache_dir {
            self.load_persistent_cache(cache_dir).await?;
        }
        
        // Start cache maintenance
        self.start_maintenance().await?;
        
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        tracing::info!("Shutting down compilation cache");
        
        // Save persistent cache if available
        if let Some(cache_dir) = &self.cache_dir {
            self.save_persistent_cache(cache_dir).await?;
        }
        
        // Clear in-memory cache
        let mut entries = self.entries.write().await;
        entries.clear();
        
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<CacheEntry>> {
        if !self.enabled {
            return Ok(None);
        }

        let mut entries = self.entries.write().await;
        let mut stats = self.stats.write().await;
        
        if let Some(entry) = entries.get_mut(key) {
            // Check if entry is expired
            match &self.strategy {
                CacheStrategy::LruWithTtl { ttl, .. } => {
                    if entry.is_expired(*ttl) {
                        entries.remove(key);
                        stats.miss_count += 1;
                        stats.eviction_count += 1;
                        return Ok(None);
                    }
                }
                CacheStrategy::Hybrid { ttl, .. } => {
                    if entry.is_expired(*ttl) {
                        entries.remove(key);
                        stats.miss_count += 1;
                        stats.eviction_count += 1;
                        return Ok(None);
                    }
                }
                _ => {}
            }
            
            // Update access information
            entry.touch();
            stats.hit_count += 1;
            
            tracing::debug!("Cache hit for key: {}", key);
            Ok(Some(entry.clone()))
        } else {
            stats.miss_count += 1;
            tracing::debug!("Cache miss for key: {}", key);
            Ok(None)
        }
    }

    pub async fn put(&self, entry: CacheEntry) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let mut entries = self.entries.write().await;
        let mut stats = self.stats.write().await;
        
        // Check if we need to evict entries
        self.evict_if_needed(&mut entries, &mut stats, entry.size_bytes).await?;
        
        // Insert new entry
        let key = entry.key.clone();
        stats.total_size_bytes += entry.size_bytes;
        entries.insert(key.clone(), entry);
        
        // Update statistics
        stats.total_entries = entries.len();
        self.update_cache_stats(&mut stats, &entries).await;
        
        tracing::debug!("Cached entry for key: {}", key);
        Ok(())
    }

    pub async fn remove(&self, key: &str) -> Result<bool> {
        if !self.enabled {
            return Ok(false);
        }

        let mut entries = self.entries.write().await;
        let mut stats = self.stats.write().await;
        
        if let Some(entry) = entries.remove(key) {
            stats.total_size_bytes -= entry.size_bytes;
            stats.total_entries = entries.len();
            tracing::debug!("Removed cache entry for key: {}", key);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn clear(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let mut entries = self.entries.write().await;
        let mut stats = self.stats.write().await;
        
        entries.clear();
        stats.total_entries = 0;
        stats.total_size_bytes = 0;
        stats.eviction_count += entries.len() as u64;
        
        tracing::info!("Cleared compilation cache");
        Ok(())
    }

    pub async fn get_stats(&self) -> CacheStats {
        if !self.enabled {
            return CacheStats::default();
        }

        let stats = self.stats.read().await;
        stats.clone()
    }

    pub async fn optimize(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let mut entries = self.entries.write().await;
        let mut stats = self.stats.write().await;
        
        tracing::info!("Optimizing compilation cache");
        
        // Force eviction of expired entries
        match &self.strategy {
            CacheStrategy::LruWithTtl { ttl, .. } | CacheStrategy::Hybrid { ttl, .. } => {
                let mut expired_keys = Vec::new();
                for (key, entry) in entries.iter() {
                    if entry.is_expired(*ttl) {
                        expired_keys.push(key.clone());
                    }
                }
                
                for key in expired_keys {
                    if let Some(entry) = entries.remove(&key) {
                        stats.total_size_bytes -= entry.size_bytes;
                        stats.eviction_count += 1;
                    }
                }
            }
            _ => {}
        }
        
        // Update statistics
        stats.total_entries = entries.len();
        self.update_cache_stats(&mut stats, &entries).await;
        
        tracing::info!("Cache optimization completed. {} entries remaining", entries.len());
        Ok(())
    }

    // Private helper methods
    
    async fn evict_if_needed(
        &self,
        entries: &mut HashMap<String, CacheEntry>,
        stats: &mut CacheStats,
        new_entry_size: u64,
    ) -> Result<()> {
        match &self.strategy {
            CacheStrategy::LruWithTtl { max_entries, .. } => {
                while entries.len() >= *max_entries {
                    self.evict_lru(entries, stats).await?;
                }
            }
            CacheStrategy::Lfu { max_entries } => {
                while entries.len() >= *max_entries {
                    self.evict_lfu(entries, stats).await?;
                }
            }
            CacheStrategy::SizeBased { max_size_bytes } => {
                while stats.total_size_bytes + new_entry_size > *max_size_bytes {
                    self.evict_lru(entries, stats).await?;
                }
            }
            CacheStrategy::Hybrid { max_entries, max_size_bytes, .. } => {
                while entries.len() >= *max_entries || 
                      stats.total_size_bytes + new_entry_size > *max_size_bytes {
                    self.evict_lru(entries, stats).await?;
                }
            }
        }
        
        Ok(())
    }

    async fn evict_lru(
        &self,
        entries: &mut HashMap<String, CacheEntry>,
        stats: &mut CacheStats,
    ) -> Result<()> {
        if let Some((key, entry)) = entries.iter()
            .min_by_key(|(_, entry)| entry.last_accessed)
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            entries.remove(&key);
            stats.total_size_bytes -= entry.size_bytes;
            stats.eviction_count += 1;
            tracing::debug!("Evicted LRU cache entry: {}", key);
        }
        
        Ok(())
    }

    async fn evict_lfu(
        &self,
        entries: &mut HashMap<String, CacheEntry>,
        stats: &mut CacheStats,
    ) -> Result<()> {
        if let Some((key, entry)) = entries.iter()
            .min_by_key(|(_, entry)| entry.access_count)
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            entries.remove(&key);
            stats.total_size_bytes -= entry.size_bytes;
            stats.eviction_count += 1;
            tracing::debug!("Evicted LFU cache entry: {}", key);
        }
        
        Ok(())
    }

    async fn update_cache_stats(
        &self,
        stats: &mut CacheStats,
        entries: &HashMap<String, CacheEntry>,
    ) {
        let total_requests = stats.hit_count + stats.miss_count;
        stats.hit_rate = if total_requests > 0 {
            stats.hit_count as f64 / total_requests as f64
        } else {
            0.0
        };
        
        stats.average_entry_size = if entries.is_empty() {
            0.0
        } else {
            stats.total_size_bytes as f64 / entries.len() as f64
        };
        
        // Find oldest and newest entries
        if let Some(oldest) = entries.values().min_by_key(|e| e.created_at) {
            stats.oldest_entry_age = oldest.age();
        }
        
        if let Some(newest) = entries.values().max_by_key(|e| e.created_at) {
            stats.newest_entry_age = newest.age();
        }
    }

    async fn start_maintenance(&self) -> Result<()> {
        tracing::debug!("Starting cache maintenance");
        
        // Simulate maintenance startup
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        Ok(())
    }

    async fn load_persistent_cache(&self, _cache_dir: &PathBuf) -> Result<()> {
        tracing::debug!("Loading persistent cache");
        
        // Simulate loading from disk
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        Ok(())
    }

    async fn save_persistent_cache(&self, _cache_dir: &PathBuf) -> Result<()> {
        tracing::debug!("Saving persistent cache");
        
        // Simulate saving to disk
        tokio::time::sleep(Duration::from_millis(30)).await;
        
        Ok(())
    }
}
