/// Template Cache - High-performance multi-level caching system for CURSED templates
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock as AsyncRwLock;
use tracing::{debug, error, info, instrument, warn};

use crate::error::CursedError;
use super::template_syntax::{TemplateAst, TemplateNode};

/// Cache entry types for multi-level caching
#[derive(Debug, Clone)]
pub enum CacheEntryType {
    /// Template source AST
    TemplateAst(TemplateAst),
    /// Rendered output content
    RenderedOutput(String),
    /// Reusable template component
    Component(TemplateComponent),
    /// Partial template fragment
    Fragment(String),
}

/// Template component for caching
#[derive(Debug, Clone)]
pub struct TemplateComponent {
    pub name: String,
    pub ast: TemplateAst,
    pub dependencies: Vec<String>,
    pub parameters: HashMap<String, String>,
}

/// Multi-level template cache entry
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Cache entry type and data
    pub entry_type: CacheEntryType,
    /// Creation timestamp
    pub created_at: Instant,
    /// Last access timestamp
    pub last_accessed: Instant,
    /// Access count for statistics
    pub access_count: u64,
    /// Template source hash for invalidation
    pub source_hash: u64,
    /// File modification time (if applicable)
    pub file_modified: Option<SystemTime>,
    /// Entry size in bytes
    pub size: usize,
    /// Cache level (0=source, 1=rendered, 2=component, 3=fragment)
    pub level: u8,
    /// Dependency keys for cache invalidation
    pub dependencies: HashSet<String>,
    /// Compression flag
    pub is_compressed: bool,
}

/// Multi-level cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of cache hits per level
    pub hits: HashMap<u8, u64>,
    /// Number of cache misses per level  
    pub misses: HashMap<u8, u64>,
    /// Number of cache evictions per level
    pub evictions: HashMap<u8, u64>,
    /// Total number of entries per level
    pub entries: HashMap<u8, usize>,
    /// Memory usage estimate per level (in bytes)
    pub memory_usage: HashMap<u8, usize>,
    /// Cache hit ratio per level (0.0 to 1.0)
    pub hit_ratio: HashMap<u8, f64>,
    /// Cache warming operations
    pub warming_operations: u64,
    /// Background operations count
    pub background_operations: u64,
    /// Compression ratio (compressed_size / original_size)
    pub compression_ratio: f64,
    /// Total cache operations
    pub total_operations: u64,
}

/// Cache eviction policy
#[derive(Debug, Clone)]
pub enum EvictionPolicy {
    /// Least Recently Used
    Lru,
    /// Least Frequently Used
    Lfu,
    /// Time-based expiration
    Ttl(Duration),
    /// First In, First Out
    Fifo,
    /// Random eviction
    Random,
}

/// Cache persistence type
#[derive(Debug, Clone)]
pub enum CachePersistence {
    /// Memory-only cache
    Memory,
    /// Disk-based cache with path
    Disk(PathBuf),
    /// Hybrid memory + disk cache
    Hybrid { memory_limit: usize, disk_path: PathBuf },
}

/// Cache level configuration
#[derive(Debug, Clone)]
pub struct CacheLevelConfig {
    /// Maximum entries for this level
    pub max_entries: usize,
    /// Maximum memory for this level
    pub max_memory: usize,
    /// TTL for this level
    pub ttl: Option<Duration>,
    /// Enable compression for this level
    pub enable_compression: bool,
    /// Eviction policy for this level
    pub eviction_policy: EvictionPolicy,
}

/// Comprehensive template cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Per-level cache configuration
    pub level_configs: HashMap<u8, CacheLevelConfig>,
    /// Global maximum entries across all levels
    pub global_max_entries: usize,
    /// Global maximum memory usage (in bytes)
    pub global_max_memory: usize,
    /// Enable cache statistics
    pub enable_stats: bool,
    /// Auto-refresh interval for file-based templates
    pub auto_refresh_interval: Option<Duration>,
    /// Cache persistence strategy
    pub persistence: CachePersistence,
    /// Enable background cache warming
    pub enable_warming: bool,
    /// Preload commonly used templates
    pub enable_preloading: bool,
    /// Enable hot reload in development
    pub enable_hot_reload: bool,
    /// Development mode settings
    pub development_mode: bool,
    /// Parallel cache operations thread count
    pub parallel_threads: usize,
    /// Cache dependency tracking
    pub track_dependencies: bool,
    /// Compression algorithm
    pub compression_algorithm: CompressionAlgorithm,
}

/// Compression algorithm options
#[derive(Debug, Clone)]
pub enum CompressionAlgorithm {
    /// No compression
    None,
    /// Fast LZ4 compression
    Lz4,
    /// Balanced compression
    Zstd,
    /// High compression ratio
    Gzip,
}

impl Default for CacheConfig {
    fn default() -> Self {
        let mut level_configs = HashMap::new();
        
        // Level 0: Template AST cache
        level_configs.insert(0, CacheLevelConfig {
            max_entries: 500,
            max_memory: 50 * 1024 * 1024, // 50 MB
            ttl: Some(Duration::from_secs(3600)), // 1 hour
            enable_compression: false,
            eviction_policy: EvictionPolicy::Lru,
        });
        
        // Level 1: Rendered output cache  
        level_configs.insert(1, CacheLevelConfig {
            max_entries: 1000,
            max_memory: 100 * 1024 * 1024, // 100 MB
            ttl: Some(Duration::from_secs(300)), // 5 minutes
            enable_compression: true,
            eviction_policy: EvictionPolicy::Lru,
        });
        
        // Level 2: Component cache
        level_configs.insert(2, CacheLevelConfig {
            max_entries: 200,
            max_memory: 20 * 1024 * 1024, // 20 MB
            ttl: Some(Duration::from_secs(1800)), // 30 minutes
            enable_compression: false,
            eviction_policy: EvictionPolicy::Lfu,
        });
        
        // Level 3: Fragment cache
        level_configs.insert(3, CacheLevelConfig {
            max_entries: 2000,
            max_memory: 50 * 1024 * 1024, // 50 MB
            ttl: Some(Duration::from_secs(600)), // 10 minutes
            enable_compression: true,
            eviction_policy: EvictionPolicy::Lru,
        });

        Self {
            level_configs,
            global_max_entries: 2000,
            global_max_memory: 200 * 1024 * 1024, // 200 MB
            enable_stats: true,
            auto_refresh_interval: Some(Duration::from_secs(60)),
            persistence: CachePersistence::Memory,
            enable_warming: true,
            enable_preloading: false,
            enable_hot_reload: false,
            development_mode: false,
            parallel_threads: 4,
            track_dependencies: true,
            compression_algorithm: CompressionAlgorithm::Lz4,
        }
    }
}

/// Background operation types
#[derive(Debug, Clone)]
pub enum BackgroundOperation {
    /// Warm up specific templates
    WarmUp(Vec<String>),
    /// Preload templates
    Preload(Vec<String>),
    /// Cleanup expired entries
    Cleanup,
    /// Compress uncompressed entries
    Compress,
    /// Refresh file-based templates
    Refresh,
}

/// Multi-level high-performance template cache
#[derive(Debug)]
pub struct TemplateCache {
    /// Multi-level cache entries
    entries: Arc<AsyncRwLock<HashMap<String, CacheEntry>>>,
    /// Cache configuration
    config: Arc<CacheConfig>,
    /// Cache statistics
    stats: Arc<RwLock<CacheStats>>,
    /// LRU access order per level
    access_order: Arc<RwLock<HashMap<u8, VecDeque<String>>>>,
    /// Background operation queue
    operation_queue: Arc<Mutex<VecDeque<BackgroundOperation>>>,
    /// Dependency graph for invalidation
    dependencies: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    /// File watcher for hot reload
    file_watchers: Arc<RwLock<HashMap<String, SystemTime>>>,
    /// Preloaded template keys
    preloaded_keys: Arc<RwLock<HashSet<String>>>,
    /// Background task handle
    background_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

impl TemplateCache {
    /// Create a new template cache with default configuration
    pub fn new(max_entries: usize) -> Self {
        let mut config = CacheConfig::default();
        config.global_max_entries = max_entries;
        Self::with_config(config)
    }

    /// Create a new template cache with custom configuration
    pub fn with_config(config: CacheConfig) -> Self {
        let mut initial_stats = CacheStats {
            hits: HashMap::new(),
            misses: HashMap::new(),
            evictions: HashMap::new(),
            entries: HashMap::new(),
            memory_usage: HashMap::new(),
            hit_ratio: HashMap::new(),
            warming_operations: 0,
            background_operations: 0,
            compression_ratio: 1.0,
            total_operations: 0,
        };

        // Initialize per-level stats
        for level in 0..4 {
            initial_stats.hits.insert(level, 0);
            initial_stats.misses.insert(level, 0);
            initial_stats.evictions.insert(level, 0);
            initial_stats.entries.insert(level, 0);
            initial_stats.memory_usage.insert(level, 0);
            initial_stats.hit_ratio.insert(level, 0.0);
        }

        let mut initial_access_order = HashMap::new();
        for level in 0..4 {
            initial_access_order.insert(level, VecDeque::new());
        }

        let cache = Self {
            entries: Arc::new(AsyncRwLock::new(HashMap::new())),
            config: Arc::new(config),
            stats: Arc::new(RwLock::new(initial_stats)),
            access_order: Arc::new(RwLock::new(initial_access_order)),
            operation_queue: Arc::new(Mutex::new(VecDeque::new())),
            dependencies: Arc::new(RwLock::new(HashMap::new())),
            file_watchers: Arc::new(RwLock::new(HashMap::new())),
            preloaded_keys: Arc::new(RwLock::new(HashSet::new())),
            background_handle: Arc::new(Mutex::new(None)),
        };

        // Start background processing if enabled
        if cache.config.enable_warming {
            cache.start_background_processing();
        }

        cache
    }

    /// Get a template from the cache (async for multi-level support)
    #[instrument(skip(self))]
    pub async fn get(&self, key: &str) -> Option<CacheEntryType> {
        debug!(key = key, "Multi-level cache lookup");
        
        // Check if file has been modified before returning cached entry
        if self.is_file_modified(key).await {
            debug!(key = key, "Template file has been modified, invalidating cache");
            self.invalidate(key).await;
            return None;
        }
        
        let entries = self.entries.read().await;
        let entry = entries.get(key)?;
        
        // Update access information
        let level = entry.level;
        let entry_type = entry.entry_type.clone();
        drop(entries);

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            *stats.hits.entry(level).or_insert(0) += 1;
            stats.total_operations += 1;
            self.update_hit_ratio_for_level(&mut stats, level);
        }

        // Update LRU order
        if let Ok(mut access_order) = self.access_order.write() {
            if let Some(level_order) = access_order.get_mut(&level) {
                level_order.retain(|k| k != key);
                level_order.push_back(key.to_string());
            }
        }

        debug!(key = key, level = level, "Cache hit");
        Some(entry_type)
    }

    /// Get template AST specifically (backward compatibility)
    #[instrument(skip(self))]
    pub async fn get_template(&self, key: &str) -> Option<TemplateAst> {
        match self.get(key).await? {
            CacheEntryType::TemplateAst(ast) => Some(ast),
            _ => None,
        }
    }

    /// Get rendered output from cache
    #[instrument(skip(self))]
    pub async fn get_rendered(&self, key: &str) -> Option<String> {
        match self.get(key).await? {
            CacheEntryType::RenderedOutput(output) => Some(output),
            _ => None,
        }
    }

    /// Get component from cache
    #[instrument(skip(self))]
    pub async fn get_component(&self, key: &str) -> Option<TemplateComponent> {
        match self.get(key).await? {
            CacheEntryType::Component(component) => Some(component),
            _ => None,
        }
    }

    /// Get fragment from cache
    #[instrument(skip(self))]
    pub async fn get_fragment(&self, key: &str) -> Option<String> {
        match self.get(key).await? {
            CacheEntryType::Fragment(fragment) => Some(fragment),
            _ => None,
        }
    }

    /// Put a template AST into the cache (level 0)
    #[instrument(skip(self, ast))]
    pub async fn put_template(&self, key: String, ast: TemplateAst, source_hash: u64) -> Result<(), Error> {
        self.put_entry(key, CacheEntryType::TemplateAst(ast), 0, source_hash, Vec::new()).await
    }

    /// Put rendered output into the cache (level 1)
    #[instrument(skip(self, output))]
    pub async fn put_rendered(&self, key: String, output: String, source_hash: u64) -> Result<(), Error> {
        self.put_entry(key, CacheEntryType::RenderedOutput(output), 1, source_hash, Vec::new()).await
    }

    /// Put component into the cache (level 2)
    #[instrument(skip(self, component))]
    pub async fn put_component(&self, key: String, component: TemplateComponent, source_hash: u64) -> Result<(), Error> {
        let dependencies = component.dependencies.clone();
        self.put_entry(key, CacheEntryType::Component(component), 2, source_hash, dependencies).await
    }

    /// Put fragment into the cache (level 3)
    #[instrument(skip(self, fragment))]
    pub async fn put_fragment(&self, key: String, fragment: String, source_hash: u64) -> Result<(), Error> {
        self.put_entry(key, CacheEntryType::Fragment(fragment), 3, source_hash, Vec::new()).await
    }

    /// Generic method to put any entry type into the cache
    #[instrument(skip(self, entry_type))]
    async fn put_entry(&self, key: String, entry_type: CacheEntryType, level: u8, source_hash: u64, dependencies: Vec<String>) -> Result<(), Error> {
        debug!(key = key, level = level, "Cache put entry");

        // Check level-specific limits
        if let Some(level_config) = self.config.level_configs.get(&level) {
            let current_level_entries = {
                let entries = self.entries.read().await;
                entries.values().filter(|e| e.level == level).count()
            };

            if current_level_entries >= level_config.max_entries {
                self.evict_entry_for_level(level).await?;
            }
        }

        let size = self.estimate_entry_size(&entry_type);
        let compressed_entry_type = if self.should_compress(&entry_type, level) {
            self.compress_entry_type(entry_type).await?
        } else {
            entry_type
        };

        let mut dep_set = HashSet::new();
        for dep in dependencies {
            dep_set.insert(dep);
        }

        let entry = CacheEntry {
            entry_type: compressed_entry_type,
            created_at: Instant::now(),
            last_accessed: Instant::now(),
            access_count: 0,
            source_hash,
            file_modified: None,
            size,
            level,
            dependencies: dep_set.clone(),
            is_compressed: self.should_compress(&CacheEntryType::Fragment("".to_string()), level),
        };

        // Insert entry
        {
            let mut entries = self.entries.write().await;
            entries.insert(key.clone(), entry);
        }

        // Update dependency tracking
        if self.config.track_dependencies && !dep_set.is_empty() {
            if let Ok(mut deps) = self.dependencies.write() {
                for dep in dep_set {
                    deps.entry(dep).or_insert_with(HashSet::new).insert(key.clone());
                }
            }
        }

        // Update LRU order
        if let Ok(mut access_order) = self.access_order.write() {
            access_order.entry(level).or_insert_with(VecDeque::new).push_back(key.clone());
        }

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            *stats.entries.entry(level).or_insert(0) += 1;
            let current_memory = stats.memory_usage.entry(level).or_insert(0);
            *current_memory += size;
            stats.total_operations += 1;
        }

        debug!(key = key, level = level, "Cache put completed");
        Ok(())
    }

    /// Backward compatibility method
    #[instrument(skip(self, ast))]
    pub async fn put(&self, key: String, ast: TemplateAst, source_hash: u64) -> Result<(), Error> {
        self.put_template(key, ast, source_hash).await
    }

    /// Get detailed cache statistics
    pub fn detailed_stats(&self) -> Option<CacheStats> {
        self.stats.read().ok().map(|stats| stats.clone())
    }

    /// Legacy eviction method (not used in new implementation)
    fn evict_entry(&self, _entries: &mut HashMap<String, CacheEntry>) -> Result<(), Error> {
        // This method is kept for backward compatibility but not used
        // The new implementation uses evict_entry_for_level
        Ok(())
    }

    /// Legacy methods (kept for compatibility but not used)
    fn find_lru_key(&self, _entries: &HashMap<String, CacheEntry>) -> Option<String> { None }
    fn find_lfu_key(&self, _entries: &HashMap<String, CacheEntry>) -> Option<String> { None }
    fn find_expired_key(&self, _entries: &HashMap<String, CacheEntry>, _ttl: Duration) -> Option<String> { None }
    fn find_fifo_key(&self) -> Option<String> { None }
    fn find_random_key(&self, _entries: &HashMap<String, CacheEntry>) -> Option<String> { None }
    fn update_hit_ratio(&self, _stats: &mut CacheStats) { }
    fn estimate_memory_usage(&self, _entries: &HashMap<String, CacheEntry>) -> usize { 0 }

    /// Advanced helper methods for multi-level caching
    
    fn estimate_entry_size(&self, entry_type: &CacheEntryType) -> usize {
        match entry_type {
            CacheEntryType::TemplateAst(ast) => self.estimate_ast_size(ast),
            CacheEntryType::RenderedOutput(output) => output.len(),
            CacheEntryType::Component(comp) => {
                comp.name.len() + self.estimate_ast_size(&comp.ast) + 
                comp.dependencies.iter().map(|d| d.len()).sum::<usize>()
            }
            CacheEntryType::Fragment(fragment) => fragment.len(),
        }
    }

    fn estimate_ast_size(&self, ast: &TemplateAst) -> usize {
        // More sophisticated AST size estimation
        ast.nodes.iter().map(|node| match node {
            TemplateNode::Text(text) => text.len(),
            _ => 50, // Default estimate for other node types
        }).sum::<usize>() + 50 // Base overhead
    }

    fn should_compress(&self, entry_type: &CacheEntryType, level: u8) -> bool {
        if let Some(level_config) = self.config.level_configs.get(&level) {
            level_config.enable_compression && self.estimate_entry_size(entry_type) > 1024
        } else {
            false
        }
    }

    async fn compress_entry_type(&self, entry_type: CacheEntryType) -> Result<(), Error> {
        // Simplified compression - in a real implementation, you'd use actual compression
        match entry_type {
            CacheEntryType::RenderedOutput(output) => {
                // Simulate compression by reducing size estimate
                Ok(CacheEntryType::RenderedOutput(output))
            }
            CacheEntryType::Fragment(fragment) => {
                Ok(CacheEntryType::Fragment(fragment))
            }
            other => Ok(other), // Don't compress AST and components
        }
    }

    async fn evict_entry_for_level(&self, level: u8) -> Result<(), Error> {
        if let Some(level_config) = self.config.level_configs.get(&level) {
            let key_to_evict = match &level_config.eviction_policy {
                EvictionPolicy::Lru => self.find_lru_key_for_level(level).await,
                EvictionPolicy::Lfu => self.find_lfu_key_for_level(level).await,
                EvictionPolicy::Ttl(duration) => self.find_expired_key_for_level(level, *duration).await,
                EvictionPolicy::Fifo => self.find_fifo_key_for_level(level).await,
                EvictionPolicy::Random => self.find_random_key_for_level(level).await,
            };

            if let Some(key) = key_to_evict {
                debug!(key = key, level = level, "Evicting cache entry");
                self.remove_async(&key).await;

                // Update statistics
                if let Ok(mut stats) = self.stats.write() {
                    *stats.evictions.entry(level).or_insert(0) += 1;
                }
            }
        }
        Ok(())
    }

    async fn find_lru_key_for_level(&self, level: u8) -> Option<String> {
        let entries = self.entries.read().await;
        entries.iter()
            .filter(|(_, entry)| entry.level == level)
            .min_by_key(|(_, entry)| entry.last_accessed)
            .map(|(key, _)| key.clone())
    }

    async fn find_lfu_key_for_level(&self, level: u8) -> Option<String> {
        let entries = self.entries.read().await;
        entries.iter()
            .filter(|(_, entry)| entry.level == level)
            .min_by_key(|(_, entry)| entry.access_count)
            .map(|(key, _)| key.clone())
    }

    async fn find_expired_key_for_level(&self, level: u8, ttl: Duration) -> Option<String> {
        let now = Instant::now();
        let entries = self.entries.read().await;
        entries.iter()
            .filter(|(_, entry)| entry.level == level)
            .find(|(_, entry)| now.duration_since(entry.created_at) > ttl)
            .map(|(key, _)| key.clone())
    }

    async fn find_fifo_key_for_level(&self, level: u8) -> Option<String> {
        if let Ok(access_order) = self.access_order.read() {
            access_order.get(&level)?.front().cloned()
        } else {
            None
        }
    }

    async fn find_random_key_for_level(&self, level: u8) -> Option<String> {
        // Simple random selection without Send issues
        let entries = self.entries.read().await;
        let filtered: Vec<_> = entries.iter()
            .filter(|(_, entry)| entry.level == level)
            .map(|(key, _)| key.clone())
            .collect();
        
        if filtered.is_empty() {
            None
        } else {
            // Use system time as simple randomness source
            let idx = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as usize % filtered.len();
            filtered.get(idx).cloned()
        }
    }

    fn update_hit_ratio_for_level(&self, stats: &mut CacheStats, level: u8) {
        let hits = *stats.hits.get(&level).unwrap_or(&0);
        let misses = *stats.misses.get(&level).unwrap_or(&0);
        let total = hits + misses;
        
        let ratio = if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        };
        
        stats.hit_ratio.insert(level, ratio);
    }

    /// Remove entry asynchronously
    async fn remove_async(&self, key: &str) -> Option<CacheEntryType> {
        debug!(key = key, "Async cache remove");

        let entry = {
            let mut entries = self.entries.write().await;
            entries.remove(key)?
        };

        let level = entry.level;
        let entry_type = entry.entry_type;

        // Update LRU order
        if let Ok(mut access_order) = self.access_order.write() {
            if let Some(level_order) = access_order.get_mut(&level) {
                level_order.retain(|k| k != key);
            }
        }

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            if let Some(entries_count) = stats.entries.get_mut(&level) {
                *entries_count = entries_count.saturating_sub(1);
            }
            if let Some(memory_usage) = stats.memory_usage.get_mut(&level) {
                *memory_usage = memory_usage.saturating_sub(entry.size);
            }
        }

        Some(entry_type)
    }



    /// Advanced cache management methods
    
    /// Get level-specific statistics  
    pub fn level_stats(&self, level: u8) -> (usize, usize) {
        if let Ok(stats) = self.stats.read() {
            let entries = *stats.entries.get(&level).unwrap_or(&0);
            let max_entries = self.config.level_configs.get(&level)
                .map(|c| c.max_entries)
                .unwrap_or(0);
            (entries, max_entries)
        } else {
            (0, 0)
        }
    }

    /// Validate entry asynchronously
    #[instrument(skip(self))]
    pub async fn validate_entry(&self, key: &str, current_hash: u64) -> bool {
        let entries = self.entries.read().await;
        if let Some(entry) = entries.get(key) {
            entry.source_hash == current_hash
        } else {
            false
        }
    }

    /// Invalidate dependencies
    #[instrument(skip(self))]
    pub async fn invalidate_dependencies(&self, dependencies: &[String]) {
        if !self.config.track_dependencies {
            return;
        }

        let mut keys_to_invalidate = HashSet::new();
        
        if let Ok(deps) = self.dependencies.read() {
            for dep in dependencies {
                if let Some(dependent_keys) = deps.get(dep) {
                    keys_to_invalidate.extend(dependent_keys.clone());
                }
            }
        }

        for key in keys_to_invalidate {
            self.remove_async(&key).await;
        }
    }

    /// Queue background operation
    #[instrument(skip(self))]
    pub async fn queue_background_operation(&self, operation: BackgroundOperation) {
        if let Ok(mut queue) = self.operation_queue.lock() {
            queue.push_back(operation);
        }
    }

    /// Get background queue size
    pub async fn background_queue_size(&self) -> usize {
        if let Ok(queue) = self.operation_queue.lock() {
            queue.len()
        } else {
            0
        }
    }

    /// Mark file as changed for hot reload
    #[instrument(skip(self))]
    pub async fn mark_file_changed(&self, template_name: &str) {
        if self.config.enable_hot_reload {
            // Remove from cache to force reload
            self.remove_async(template_name).await;
            
            // Update file watcher timestamp
            if let Ok(mut watchers) = self.file_watchers.write() {
                watchers.insert(template_name.to_string(), SystemTime::now());
            }
        }
    }

    /// Cleanup expired entries asynchronously
    #[instrument(skip(self))]
    pub async fn cleanup_expired(&self) {
        for level in 0..4 {
            if let Some(level_config) = self.config.level_configs.get(&level) {
                if let Some(ttl) = level_config.ttl {
                    let now = Instant::now();
                    let mut expired_keys = Vec::new();

                    {
                        let entries = self.entries.read().await;
                        for (key, entry) in entries.iter() {
                            if entry.level == level && now.duration_since(entry.created_at) > ttl {
                                expired_keys.push(key.clone());
                            }
                        }
                    }

                    for key in expired_keys {
                        self.remove_async(&key).await;
                    }
                }
            }
        }
    }

    /// Start background processing
    fn start_background_processing(&self) {
        let cache_clone = self.clone();
        let handle = tokio::task::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                // Process background operations
                let operations = {
                    if let Ok(mut queue) = cache_clone.operation_queue.lock() {
                        let mut ops = Vec::new();
                        while let Some(operation) = queue.pop_front() {
                            ops.push(operation);
                        }
                        ops
                    } else {
                        Vec::new()
                    }
                };
                
                for operation in operations {
                    cache_clone.process_background_operation(operation).await;
                }
                
                // Cleanup expired entries
                cache_clone.cleanup_expired().await;
                
                // Update background operation stats
                if let Ok(mut stats) = cache_clone.stats.write() {
                    stats.background_operations += 1;
                }
            }
        });

        if let Ok(mut bg_handle) = self.background_handle.lock() {
            *bg_handle = Some(handle);
        }
    }

    /// Process background operation
    async fn process_background_operation(&self, operation: BackgroundOperation) {
        debug!("Processing background operation: {:?}", operation);
        
        match operation {
            BackgroundOperation::WarmUp(keys) => {
                for key in keys {
                    // Simulate warming up by accessing the entry
                    self.get(&key).await;
                }
                if let Ok(mut stats) = self.stats.write() {
                    stats.warming_operations += 1;
                }
            }
            BackgroundOperation::Preload(keys) => {
                for key in keys {
                    if let Ok(mut preloaded) = self.preloaded_keys.write() {
                        preloaded.insert(key);
                    }
                }
            }
            BackgroundOperation::Cleanup => {
                self.cleanup_expired().await;
            }
            BackgroundOperation::Compress => {
                self.compress_uncompressed_entries().await;
            }
            BackgroundOperation::Refresh => {
                self.refresh_file_based_templates().await;
            }
        }
    }

    /// Compress uncompressed entries
    async fn compress_uncompressed_entries(&self) {
        // This would compress entries that weren't compressed on insertion
        // Implementation depends on actual compression algorithm
        debug!("Compressing uncompressed entries");
    }

    /// Refresh file-based templates
    async fn refresh_file_based_templates(&self) {
        use std::fs;
        use std::path::Path;
        
        if let Ok(watchers) = self.file_watchers.read() {
            for (template_name, cached_timestamp) in watchers.iter() {
                // Check file modification time
                let template_path = Path::new("templates").join(template_name);
                
                match fs::metadata(&template_path) {
                    Ok(metadata) => {
                        if let Ok(modified_time) = metadata.modified() {
                            // If file has been modified since cache, invalidate
                            if modified_time > *cached_timestamp {
                                debug!("Template '{}' has been modified, invalidating cache", template_name);
                                
                                // Remove from cache
                                if let Ok(mut cache) = self.cache.write() {
                                    cache.remove(template_name);
                                }
                                
                                // Update timestamp in watcher
                                drop(watchers);
                                if let Ok(mut watchers_mut) = self.file_watchers.write() {
                                    watchers_mut.insert(template_name.clone(), modified_time);
                                }
                                break;
                            } else {
                                debug!("Template '{}' is up to date", template_name);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to check metadata for template '{}': {}", template_name, e);
                        // If file doesn't exist, remove from cache and watchers
                        if let Ok(mut cache) = self.cache.write() {
                            cache.remove(template_name);
                        }
                        drop(watchers);
                        if let Ok(mut watchers_mut) = self.file_watchers.write() {
                            watchers_mut.remove(template_name);
                        }
                        break;
                    }
                }
            }
        }
    }

    /// Check if a template file has been modified since caching
    async fn is_file_modified(&self, template_name: &str) -> bool {
        use std::fs;
        use std::path::Path;
        
        // Check file watchers for cached timestamp
        if let Ok(watchers) = self.file_watchers.read() {
            if let Some(cached_timestamp) = watchers.get(template_name) {
                let template_path = Path::new("templates").join(template_name);
                
                match fs::metadata(&template_path) {
                    Ok(metadata) => {
                        if let Ok(modified_time) = metadata.modified() {
                            return modified_time > *cached_timestamp;
                        }
                    }
                    Err(_) => {
                        // File doesn't exist or can't be accessed
                        return true;
                    }
                }
            }
        }
        
        // If no cached timestamp, consider it modified
        false
    }

    /// Invalidate a cached template entry
    async fn invalidate(&self, template_name: &str) {
        // Remove from cache
        let mut entries = self.entries.write().await;
        entries.remove(template_name);
        drop(entries);
        
        // Remove from file watchers
        if let Ok(mut watchers) = self.file_watchers.write() {
            watchers.remove(template_name);
        }
        
        debug!("Invalidated cache entry for template: {}", template_name);
    }

    /// Start background cleanup task (legacy compatibility)
    pub fn start_cleanup_task(&self) -> tokio::task::JoinHandle<()> {
        let cache_clone = self.clone();
        tokio::task::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                cache_clone.cleanup_expired().await;
            }
        })
    }
}

impl Clone for TemplateCache {
    fn clone(&self) -> Self {
        Self {
            entries: Arc::clone(&self.entries),
            config: Arc::clone(&self.config),
            stats: Arc::clone(&self.stats),
            access_order: Arc::clone(&self.access_order),
            operation_queue: Arc::clone(&self.operation_queue),
            dependencies: Arc::clone(&self.dependencies),
            file_watchers: Arc::clone(&self.file_watchers),
            preloaded_keys: Arc::clone(&self.preloaded_keys),
            background_handle: Arc::clone(&self.background_handle),
        }
    }
}

impl Default for TemplateCache {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl TemplateCache {
    /// Legacy compatibility methods for existing code
    
    /// Synchronous get method (backward compatibility)
    pub fn get_sync(&self, key: &str) -> Option<TemplateAst> {
        let rt = tokio::runtime::Handle::try_current().ok()?;
        rt.block_on(self.get_template(key))
    }

    /// Synchronous remove method
    #[instrument(skip(self))]
    pub fn remove(&self, key: &str) -> Option<TemplateAst> {
        let rt = tokio::runtime::Handle::try_current().ok()?;
        match rt.block_on(self.remove_async(key))? {
            CacheEntryType::TemplateAst(ast) => Some(ast),
            _ => None,
        }
    }

    /// Clear all entries from the cache
    #[instrument(skip(self))]
    pub async fn clear(&self) {
        debug!("Clearing all cache levels");

        {
            let mut entries = self.entries.write().await;
            entries.clear();
        }

        if let Ok(mut access_order) = self.access_order.write() {
            for level_order in access_order.values_mut() {
                level_order.clear();
            }
        }

        if let Ok(mut stats) = self.stats.write() {
            for level in 0..4 {
                stats.entries.insert(level, 0);
                stats.memory_usage.insert(level, 0);
            }
        }

        if let Ok(mut deps) = self.dependencies.write() {
            deps.clear();
        }

        if let Ok(mut watchers) = self.file_watchers.write() {
            watchers.clear();
        }
    }

    /// Get cache statistics (backward compatibility)
    pub fn stats(&self) -> (usize, usize) {
        if let Ok(stats) = self.stats.read() {
            let total_entries: usize = stats.entries.values().sum();
            (total_entries, self.config.global_max_entries)
        } else {
            (0, self.config.global_max_entries)
        }
    }

    /// Check if cache contains a key
    pub async fn contains(&self, key: &str) -> bool {
        let entries = self.entries.read().await;
        entries.contains_key(key)
    }

    /// Get all cache keys
    pub async fn keys(&self) -> Vec<String> {
        let entries = self.entries.read().await;
        entries.keys().cloned().collect()
    }

    /// Invalidate cache entry
    #[instrument(skip(self))]
    pub async fn invalidate(&self, key: &str) {
        debug!(key = key, "Invalidating cache entry");
        self.remove_async(key).await;
    }

    /// Get cache configuration
    pub fn config(&self) -> &CacheConfig {
        &self.config
    }

    /// Update cache configuration
    pub fn update_config(&mut self, new_config: CacheConfig) {
        self.config = Arc::new(new_config);
    }

    /// Get total memory usage across all levels
    pub fn total_memory_usage(&self) -> usize {
        if let Ok(stats) = self.stats.read() {
            stats.memory_usage.values().sum()
        } else {
            0
        }
    }

    /// Get hit ratio for specific level
    pub fn level_hit_ratio(&self, level: u8) -> f64 {
        if let Ok(stats) = self.stats.read() {
            *stats.hit_ratio.get(&level).unwrap_or(&0.0)
        } else {
            0.0
        }
    }

    /// Enable/disable development mode
    pub fn set_development_mode(&mut self, enabled: bool) {
        Arc::make_mut(&mut self.config).development_mode = enabled;
        Arc::make_mut(&mut self.config).enable_hot_reload = enabled;
    }

    /// Warm up cache with specific templates
    pub async fn warm_up(&self, template_keys: Vec<String>) {
        self.queue_background_operation(BackgroundOperation::WarmUp(template_keys)).await;
    }

    /// Preload templates
    pub async fn preload(&self, template_keys: Vec<String>) {
        self.queue_background_operation(BackgroundOperation::Preload(template_keys)).await;
    }
}

/// Cache key generator for consistent cache keys
pub struct CacheKeyGenerator;

impl CacheKeyGenerator {
    /// Generate a cache key from template name and parameters
    pub fn generate(template_name: &str, params: Option<&HashMap<String, String>>) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        template_name.hash(&mut hasher);
        
        if let Some(params) = params {
            let mut sorted_params: Vec<_> = params.iter().collect();
            sorted_params.sort_by_key(|(k, _)| *k);
            
            for (key, value) in sorted_params {
                key.hash(&mut hasher);
                value.hash(&mut hasher);
            }
        }
        
        format!("template_{:x}", hasher.finish())
    }

    /// Generate a hash for template source
    pub fn hash_source(source: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        source.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::template::template_syntax::TemplateNode;

    fn create_test_ast() -> TemplateAst {
        TemplateAst {
            nodes: vec![TemplateNode::Text("Hello World".to_string())],
        }
    }

    #[tokio::test]
    async fn test_cache_put_and_get() {
        let cache = TemplateCache::new(10);
        let ast = create_test_ast();
        let source_hash = CacheKeyGenerator::hash_source("test template");

        cache.put_template("test".to_string(), ast.clone(), source_hash).await.unwrap();
        
        let retrieved = cache.get_template("test").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().nodes.len(), ast.nodes.len());
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let cache = TemplateCache::new(10);
        
        let result = cache.get_template("nonexistent").await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_cache_remove() {
        let cache = TemplateCache::new(10);
        let ast = create_test_ast();
        let source_hash = CacheKeyGenerator::hash_source("test template");

        cache.put_template("test".to_string(), ast, source_hash).await.unwrap();
        assert!(cache.contains("test").await);
        
        cache.remove("test");
        assert!(!cache.contains("test").await);
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache = TemplateCache::new(10);
        let ast = create_test_ast();
        let source_hash = CacheKeyGenerator::hash_source("test template");

        cache.put_template("test1".to_string(), ast.clone(), source_hash).await.unwrap();
        cache.put_template("test2".to_string(), ast, source_hash).await.unwrap();
        
        let (entries, _) = cache.stats();
        assert_eq!(entries, 2);
        
        cache.clear().await;
        
        let (entries, _) = cache.stats();
        assert_eq!(entries, 0);
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let cache = TemplateCache::new(2); // Small cache for testing eviction
        let ast = create_test_ast();
        let source_hash = CacheKeyGenerator::hash_source("test template");

        cache.put_template("test1".to_string(), ast.clone(), source_hash).await.unwrap();
        cache.put_template("test2".to_string(), ast.clone(), source_hash).await.unwrap();
        cache.put_template("test3".to_string(), ast, source_hash).await.unwrap(); // Should trigger eviction

        let (entries, _) = cache.stats();
        assert!(entries <= 2); // May be less due to eviction
    }

    #[test]
    fn test_cache_key_generation() {
        let key1 = CacheKeyGenerator::generate("template1", None);
        let key2 = CacheKeyGenerator::generate("template2", None);
        assert_ne!(key1, key2);

        let mut params = HashMap::new();
        params.insert("param1".to_string(), "value1".to_string());
        
        let key3 = CacheKeyGenerator::generate("template1", Some(&params));
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_source_hash() {
        let hash1 = CacheKeyGenerator::hash_source("template source 1");
        let hash2 = CacheKeyGenerator::hash_source("template source 2");
        let hash3 = CacheKeyGenerator::hash_source("template source 1");
        
        assert_ne!(hash1, hash2);
        assert_eq!(hash1, hash3);
    }

    #[tokio::test]
    async fn test_cache_validation() {
        let cache = TemplateCache::new(10);
        let ast = create_test_ast();
        let source_hash = CacheKeyGenerator::hash_source("test template");

        cache.put_template("test".to_string(), ast, source_hash).await.unwrap();
        
        // Valid hash should return true
        assert!(cache.validate_entry("test", source_hash).await);
        
        // Different hash should return false
        let different_hash = CacheKeyGenerator::hash_source("different template");
        assert!(!cache.validate_entry("test", different_hash).await);
        
        // Non-existent key should return false
        assert!(!cache.validate_entry("nonexistent", source_hash).await);
    }

    #[tokio::test]
    async fn test_cache_statistics() {
        let cache = TemplateCache::new(10);
        let ast = create_test_ast();
        let source_hash = CacheKeyGenerator::hash_source("test template");

        cache.put_template("test".to_string(), ast, source_hash).await.unwrap();
        
        // Miss
        cache.get_template("nonexistent").await;
        
        // Hit
        cache.get_template("test").await;
        
        let stats = cache.detailed_stats().unwrap();
        assert!(*stats.hits.get(&0).unwrap_or(&0) >= 1);
        assert!(*stats.misses.get(&0).unwrap_or(&0) >= 1);
        assert!(stats.total_operations > 0);
    }
}
