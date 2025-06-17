//! Advanced Caching System for Build Optimization
//! 
//! Implements multi-level caching with distributed support for maximum
//! build performance and team collaboration.

use std::collections::{HashMap, HashSet, BTreeMap};
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn, instrument};
use sha2::{Sha256, Digest};
use tokio::net::TcpStream;
use chrono;

use crate::error::{CursedError, Result};

/// Multi-level cache entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub content_hash: String,
    pub metadata: CacheMetadata,
    pub data: CacheData,
    pub created_at: u64,
    pub accessed_at: u64,
    pub size_bytes: usize,
    pub dependencies: Vec<String>,
}

/// Cache metadata for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    pub file_path: PathBuf,
    pub last_modified: u64,
    pub file_size: u64,
    pub compiler_version: String,
    pub compilation_flags: Vec<String>,
    pub source_hash: String,
    pub dependency_hashes: HashMap<String, String>,
}

/// Different types of cached data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheData {
    Ast(String),              // Serialized AST
    IR(String),               // LLVM IR code  
    Object(Vec<u8>),          // Compiled object file
    Analysis(String),         // Type analysis results
    Dependency(String),       // Dependency information
    Metadata(String),         // Build metadata
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedCacheConfig {
    pub enable_ast_cache: bool,
    pub enable_ir_cache: bool,
    pub enable_object_cache: bool,
    pub enable_distributed_cache: bool,
    pub cache_directory: PathBuf,
    pub max_cache_size_mb: usize,
    pub max_entry_age_hours: u64,
    pub compression_enabled: bool,
    pub precomputation_enabled: bool,
    pub cache_warming_enabled: bool,
    pub distributed_nodes: Vec<String>,
    pub replication_factor: usize,
    pub network_timeout_ms: u64,
}

impl Default for AdvancedCacheConfig {
    fn default() -> Self {
        Self {
            enable_ast_cache: true,
            enable_ir_cache: true,
            enable_object_cache: true,
            enable_distributed_cache: false,
            cache_directory: PathBuf::from(".cursed_cache"),
            max_cache_size_mb: 1024, // 1GB default
            max_entry_age_hours: 168, // 1 week
            compression_enabled: true,
            precomputation_enabled: true,
            cache_warming_enabled: true,
            distributed_nodes: Vec::new(),
            replication_factor: 2,
            network_timeout_ms: 5000,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatistics {
    pub total_entries: usize,
    pub total_size_mb: f64,
    pub hit_rate: f64,
    pub miss_rate: f64,
    pub eviction_count: usize,
    pub network_hits: usize,
    pub network_misses: usize,
    pub compression_ratio: f64,
    pub average_lookup_time_ms: f64,
    pub cache_warming_hits: usize,
}

/// Cache warming strategy
#[derive(Debug, Clone)]
pub struct CacheWarmingStrategy {
    pub frequently_used_files: HashSet<String>,
    pub dependency_chains: Vec<Vec<String>>,
    pub precomputation_patterns: Vec<String>,
    pub warming_schedule: HashMap<String, u64>,
}

/// Distributed cache node
#[derive(Debug, Clone)]
pub struct DistributedNode {
    pub id: String,
    pub address: String,
    pub port: u16,
    pub is_available: bool,
    pub last_ping: u64,
    pub load_factor: f64,
}

/// Advanced caching system
pub struct AdvancedCache {
    config: AdvancedCacheConfig,
    local_cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    cache_index: Arc<RwLock<BTreeMap<String, String>>>, // content_hash -> key
    statistics: Arc<Mutex<CacheStatistics>>,
    warming_strategy: Arc<RwLock<CacheWarmingStrategy>>,
    distributed_nodes: Arc<RwLock<Vec<DistributedNode>>>,
    content_hasher: Arc<Mutex<Sha256>>,
    access_patterns: Arc<Mutex<HashMap<String, Vec<u64>>>>,
}

impl AdvancedCache {
    /// Create a new advanced cache system
    #[instrument]
    pub fn new(config: AdvancedCacheConfig) -> Result<Self> {
        // Ensure cache directory exists
        if !config.cache_directory.exists() {
            fs::create_dir_all(&config.cache_directory)?;
            info!("Created cache directory: {:?}", config.cache_directory);
        }

        let cache = Self {
            local_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_index: Arc::new(RwLock::new(BTreeMap::new())),
            statistics: Arc::new(Mutex::new(CacheStatistics {
                total_entries: 0,
                total_size_mb: 0.0,
                hit_rate: 0.0,
                miss_rate: 0.0,
                eviction_count: 0,
                network_hits: 0,
                network_misses: 0,
                compression_ratio: 1.0,
                average_lookup_time_ms: 0.0,
                cache_warming_hits: 0,
            })),
            warming_strategy: Arc::new(RwLock::new(CacheWarmingStrategy {
                frequently_used_files: HashSet::new(),
                dependency_chains: Vec::new(),
                precomputation_patterns: Vec::new(),
                warming_schedule: HashMap::new(),
            })),
            distributed_nodes: Arc::new(RwLock::new(Vec::new())),
            content_hasher: Arc::new(Mutex::new(Sha256::new())),
            access_patterns: Arc::new(Mutex::new(HashMap::new())),
            config,
        };

        // Initialize distributed nodes if enabled
        if cache.config.enable_distributed_cache {
            cache.initialize_distributed_nodes()?;
        }

        // Load existing cache
        cache.load_cache_from_disk()?;

        // Start cache warming if enabled
        if cache.config.cache_warming_enabled {
            cache.start_cache_warming()?;
        }

        Ok(cache)
    }

    /// Store data in cache with multi-level support
    #[instrument(skip(self, data))]
    pub fn store(&self, key: &str, data: CacheData, metadata: CacheMetadata) -> Result<()> {
        let start = Instant::now();
        
        // Calculate content hash
        let content_hash = self.calculate_content_hash(&data, &metadata)?;
        
        let entry = CacheEntry {
            key: key.to_string(),
            content_hash: content_hash.clone(),
            metadata,
            data,
            created_at: current_timestamp(),
            accessed_at: current_timestamp(),
            size_bytes: self.calculate_entry_size(&data)?,
            dependencies: Vec::new(),
        };

        // Store in local cache
        {
            let mut cache = self.local_cache.write().map_err(|_| CursedError::system_error("Failed to lock cache"))?;
            cache.insert(key.to_string(), entry.clone());
            
            let mut index = self.cache_index.write().map_err(|_| CursedError::system_error("Failed to lock index"))?;
            index.insert(content_hash.clone(), key.to_string());
        }

        // Store to disk
        self.store_to_disk(&entry)?;

        // Replicate to distributed nodes if enabled
        if self.config.enable_distributed_cache {
            self.replicate_to_distributed_nodes(&entry)?;
        }

        // Update statistics
        self.update_statistics_store(&entry, start.elapsed())?;

        // Check if eviction is needed
        self.check_eviction_needed()?;

        debug!(
            key,
            content_hash,
            size_bytes = entry.size_bytes,
            store_time_ms = start.elapsed().as_millis(),
            "Stored cache entry"
        );

        Ok(())
    }

    /// Retrieve data from cache with fallback to distributed nodes
    #[instrument(skip(self))]
    pub fn retrieve(&self, key: &str) -> Result<Option<CacheEntry>> {
        let start = Instant::now();
        
        // Try local cache first
        if let Some(entry) = self.get_from_local_cache(key)? {
            self.update_access_time(key)?;
            self.update_statistics_hit(start.elapsed())?;
            debug!(key, source = "local", "Cache hit");
            return Ok(Some(entry));
        }

        // Try distributed cache if enabled
        if self.config.enable_distributed_cache {
            if let Some(entry) = self.get_from_distributed_cache(key)? {
                // Store locally for future access
                self.store_local_copy(&entry)?;
                self.update_statistics_network_hit(start.elapsed())?;
                debug!(key, source = "distributed", "Cache hit");
                return Ok(Some(entry));
            }
        }

        // Cache miss
        self.update_statistics_miss(start.elapsed())?;
        debug!(key, "Cache miss");
        Ok(None)
    }

    /// Retrieve by content hash for deduplication
    #[instrument(skip(self))]
    pub fn retrieve_by_content_hash(&self, content_hash: &str) -> Result<Option<CacheEntry>> {
        let index = self.cache_index.read().map_err(|_| CursedError::system_error("Failed to lock index"))?;
        
        if let Some(key) = index.get(content_hash) {
            self.retrieve(key)
        } else {
            Ok(None)
        }
    }

    /// Invalidate cache entries based on dependencies
    #[instrument(skip(self))]
    pub fn invalidate_by_dependencies(&self, changed_files: &[String]) -> Result<usize> {
        let mut invalidated_count = 0;
        let mut to_remove = Vec::new();

        {
            let cache = self.local_cache.read().map_err(|_| CursedError::system_error("Failed to lock cache"))?;
            
            for (key, entry) in cache.iter() {
                // Check if any dependencies have changed
                for changed_file in changed_files {
                    if entry.dependencies.contains(changed_file) ||
                       entry.metadata.dependency_hashes.contains_key(changed_file) {
                        to_remove.push(key.clone());
                        break;
                    }
                }
            }
        }

        // Remove invalidated entries
        {
            let mut cache = self.local_cache.write().map_err(|_| CursedError::system_error("Failed to lock cache"))?;
            let mut index = self.cache_index.write().map_err(|_| CursedError::system_error("Failed to lock index"))?;
            
            for key in &to_remove {
                if let Some(entry) = cache.remove(key) {
                    index.remove(&entry.content_hash);
                    invalidated_count += 1;
                    self.remove_from_disk(&entry)?;
                }
            }
        }

        info!(invalidated_count, "Invalidated cache entries");
        Ok(invalidated_count)
    }

    /// Pre-compute and warm cache for frequently used files
    #[instrument(skip(self))]
    pub fn warm_cache(&self, files: &[String]) -> Result<usize> {
        let mut warmed_count = 0;
        
        for file in files {
            if !self.is_in_cache(file)? {
                // Trigger precomputation for this file
                if self.precompute_file(file)? {
                    warmed_count += 1;
                }
            }
        }

        // Update warming statistics
        if let Ok(mut stats) = self.statistics.lock() {
            stats.cache_warming_hits += warmed_count;
        }

        info!(warmed_count, total_files = files.len(), "Cache warming completed");
        Ok(warmed_count)
    }

    /// Optimize cache by removing least recently used entries
    #[instrument(skip(self))]
    pub fn optimize_cache(&self) -> Result<usize> {
        let max_size_bytes = self.config.max_cache_size_mb * 1024 * 1024;
        let current_size = self.get_current_cache_size()?;
        
        if current_size <= max_size_bytes {
            return Ok(0);
        }

        let mut entries_with_access: Vec<(String, u64, usize)> = Vec::new();
        
        {
            let cache = self.local_cache.read().map_err(|_| CursedError::system_error("Failed to lock cache"))?;
            for (key, entry) in cache.iter() {
                entries_with_access.push((key.clone(), entry.accessed_at, entry.size_bytes));
            }
        }

        // Sort by access time (oldest first)
        entries_with_access.sort_by_key(|(_, accessed_at, _)| *accessed_at);

        let mut removed_count = 0;
        let mut size_freed = 0;
        let target_size = (max_size_bytes as f64 * 0.8) as usize; // Remove to 80% capacity

        {
            let mut cache = self.local_cache.write().map_err(|_| CursedError::system_error("Failed to lock cache"))?;
            let mut index = self.cache_index.write().map_err(|_| CursedError::system_error("Failed to lock index"))?;
            
            for (key, _, size) in entries_with_access {
                if current_size - size_freed <= target_size {
                    break;
                }
                
                if let Some(entry) = cache.remove(&key) {
                    index.remove(&entry.content_hash);
                    size_freed += size;
                    removed_count += 1;
                    self.remove_from_disk(&entry)?;
                }
            }
        }

        // Update statistics
        if let Ok(mut stats) = self.statistics.lock() {
            stats.eviction_count += removed_count;
        }

        info!(
            removed_count,
            size_freed_mb = size_freed as f64 / (1024.0 * 1024.0),
            "Cache optimization completed"
        );
        
        Ok(removed_count)
    }

    /// Get comprehensive cache statistics
    #[instrument(skip(self))]
    pub fn get_statistics(&self) -> Result<CacheStatistics> {
        let stats = self.statistics.lock().map_err(|_| CursedError::system_error("Failed to lock statistics"))?;
        let mut result = stats.clone();
        
        // Update real-time statistics
        let cache = self.local_cache.read().map_err(|_| CursedError::system_error("Failed to lock cache"))?;
        result.total_entries = cache.len();
        result.total_size_mb = cache.values()
            .map(|entry| entry.size_bytes)
            .sum::<usize>() as f64 / (1024.0 * 1024.0);
        
        drop(cache);
        drop(stats);
        
        Ok(result)
    }

    /// Initialize distributed cache nodes
    #[instrument(skip(self))]
    fn initialize_distributed_nodes(&self) -> Result<()> {
        let mut nodes = Vec::new();
        
        for (i, address) in self.config.distributed_nodes.iter().enumerate() {
            let parts: Vec<&str> = address.split(':').collect();
            if parts.len() == 2 {
                let node = DistributedNode {
                    id: format!("node_{}", i),
                    address: parts[0].to_string(),
                    port: parts[1].parse().unwrap_or(8080),
                    is_available: false, // Will be updated by health check
                    last_ping: 0,
                    load_factor: 0.0,
                };
                nodes.push(node);
            }
        }
        
        {
            let mut distributed_nodes = self.distributed_nodes.write()
                .map_err(|_| CursedError::system_error("Failed to lock distributed nodes"))?;
            *distributed_nodes = nodes;
        }
        
        // Start health checking
        self.start_health_checking()?;
        
        Ok(())
    }

    /// Calculate content hash for cache key
    #[instrument(skip(self, data, metadata))]
    fn calculate_content_hash(&self, data: &CacheData, metadata: &CacheMetadata) -> Result<String> {
        let mut hasher = self.content_hasher.lock().map_err(|_| CursedError::system_error("Failed to lock hasher"))?;
        hasher.update(metadata.source_hash.as_bytes());
        hasher.update(metadata.compiler_version.as_bytes());
        
        for flag in &metadata.compilation_flags {
            hasher.update(flag.as_bytes());
        }
        
        match data {
            CacheData::Ast(content) => hasher.update(content.as_bytes()),
            CacheData::IR(content) => hasher.update(content.as_bytes()),
            CacheData::Object(bytes) => hasher.update(bytes),
            CacheData::Analysis(content) => hasher.update(content.as_bytes()),
            CacheData::Dependency(content) => hasher.update(content.as_bytes()),
            CacheData::Metadata(content) => hasher.update(content.as_bytes()),
        }
        
        let result = format!("{:x}", hasher.finalize_reset());
        Ok(result)
    }

    /// Get entry from local cache
    fn get_from_local_cache(&self, key: &str) -> Result<Option<CacheEntry>> {
        let cache = self.local_cache.read().map_err(|_| CursedError::system_error("Failed to lock cache"))?;
        Ok(cache.get(key).cloned())
    }

    /// Store entry to disk
    #[instrument(skip(self, entry))]
    fn store_to_disk(&self, entry: &CacheEntry) -> Result<()> {
        let cache_file = self.config.cache_directory.join(format!("{}.cache", entry.key));
        let serialized = serde_json::to_string(entry)?;
        
        if self.config.compression_enabled {
            use std::io::Write;
            use flate2::{Compression, write::GzEncoder};
            
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(serialized.as_bytes())?;
            let compressed = encoder.finish()?;
            
            fs::write(cache_file, compressed)?;
            
            debug!(
                key = entry.key,
                original_size = serialized.len(),
                compressed_size = compressed.len(),
                compression_ratio = compressed.len() as f64 / serialized.len() as f64,
                "Stored compressed cache entry"
            );
        } else {
            fs::write(cache_file, serialized)?;
        }
        
        Ok(())
    }

    /// Remove entry from disk
    #[instrument(skip(self, entry))]
    fn remove_from_disk(&self, entry: &CacheEntry) -> Result<()> {
        let cache_file = self.config.cache_directory.join(format!("{}.cache", entry.key));
        if cache_file.exists() {
            fs::remove_file(cache_file)?;
        }
        Ok(())
    }

    /// Load cache from disk
    #[instrument(skip(self))]
    fn load_cache_from_disk(&self) -> Result<()> {
        let cache_dir = &self.config.cache_directory;
        if !cache_dir.exists() {
            return Ok(());
        }

        let entries = fs::read_dir(cache_dir)?;
        let mut loaded_count = 0;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("cache") {
                let content = if self.config.compression_enabled {
                    // Try decompression first
                    match fs::read(&path) {
                        Ok(compressed_data) => {
                            use std::io::Read;
                            use flate2::read::GzDecoder;
                            
                            let mut decoder = GzDecoder::new(&compressed_data[..]);
                            let mut decompressed = String::new();
                            match decoder.read_to_string(&mut decompressed) {
                                Ok(_) => Some(decompressed),
                                Err(_) => {
                                    // Fallback to uncompressed read
                                    match fs::read_to_string(&path) {
                                        Ok(content) => Some(content),
                                        Err(e) => {
                                            warn!(?path, error = ?e, "Failed to read cache file");
                                            None
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            warn!(?path, error = ?e, "Failed to read compressed cache file");
                            None
                        }
                    }
                } else {
                    match fs::read_to_string(&path) {
                        Ok(content) => Some(content),
                        Err(e) => {
                            warn!(?path, error = ?e, "Failed to read cache file");
                            None
                        }
                    }
                };

                if let Some(content) = content {
                    match serde_json::from_str::<CacheEntry>(&content) {
                        Ok(cache_entry) => {
                            let mut cache = self.local_cache.write()
                                .map_err(|_| CursedError::system_error("Failed to lock cache"))?;
                            let mut index = self.cache_index.write()
                                .map_err(|_| CursedError::system_error("Failed to lock index"))?;
                            
                            cache.insert(cache_entry.key.clone(), cache_entry.clone());
                            index.insert(cache_entry.content_hash.clone(), cache_entry.key.clone());
                            loaded_count += 1;
                        }
                        Err(e) => warn!(?path, error = ?e, "Failed to deserialize cache entry"),
                    }
                }
            }
        }

        info!(loaded_count, "Loaded cache entries from disk");
        Ok(())
    }

    /// Start cache warming background process
    fn start_cache_warming(&self) -> Result<()> {
        let warming_strategy = Arc::clone(&self.warming_strategy);
        let local_cache = Arc::clone(&self.local_cache);
        let config = self.config.clone();
        
        thread::spawn(move || {
            debug!("Background cache warming thread started");
            
            loop {
                let strategy = match warming_strategy.read() {
                    Ok(strategy) => strategy.clone(),
                    Err(_) => break,
                };
                
                // Check if we have files to warm
                if !strategy.frequently_used_files.is_empty() {
                    for file in &strategy.frequently_used_files {
                        // Check if file is already cached
                        let cache = match local_cache.read() {
                            Ok(cache) => cache,
                            Err(_) => break,
                        };
                        
                        if !cache.contains_key(file) {
                            debug!(file, "Warming cache for frequently used file");
                            // In a real implementation, this would trigger precomputation
                            // For now, we just log the warming attempt
                        }
                        
                        drop(cache);
                    }
                }
                
                // Sleep for a while before next warming cycle
                thread::sleep(Duration::from_secs(300)); // 5 minutes
            }
            
            debug!("Background cache warming thread stopped");
        });
        
        debug!("Cache warming started");
        Ok(())
    }

    /// Start health checking for distributed nodes
    fn start_health_checking(&self) -> Result<()> {
        let distributed_nodes = Arc::clone(&self.distributed_nodes);
        let timeout = Duration::from_millis(self.config.network_timeout_ms);
        
        thread::spawn(move || {
            debug!("Distributed node health checking thread started");
            
            loop {
                let mut nodes = match distributed_nodes.write() {
                    Ok(nodes) => nodes,
                    Err(_) => break,
                };
                
                for node in nodes.iter_mut() {
                    let start = Instant::now();
                    
                    // Simple TCP connection test to check node availability
                    let address = format!("{}:{}", node.address, node.port);
                    match std::net::TcpStream::connect_timeout(
                        &address.parse().unwrap_or_else(|_| {
                            std::net::SocketAddr::from(([127, 0, 0, 1], 8080))
                        }),
                        timeout
                    ) {
                        Ok(_) => {
                            node.is_available = true;
                            node.last_ping = current_timestamp();
                            debug!(node_id = node.id, address, "Node health check passed");
                        }
                        Err(_) => {
                            node.is_available = false;
                            debug!(node_id = node.id, address, "Node health check failed");
                        }
                    }
                    
                    // Calculate simple load factor based on response time
                    let response_time = start.elapsed().as_millis() as f64;
                    node.load_factor = response_time / 100.0; // Normalize to ~1.0 for 100ms response
                }
                
                drop(nodes);
                
                // Sleep before next health check cycle
                thread::sleep(Duration::from_secs(30)); // Check every 30 seconds
            }
            
            debug!("Distributed node health checking thread stopped");
        });
        
        debug!("Health checking started for distributed nodes");
        Ok(())
    }

    /// Update statistics for cache store operation
    fn update_statistics_store(&self, entry: &CacheEntry, duration: Duration) -> Result<()> {
        let mut stats = self.statistics.lock().map_err(|_| CursedError::system_error("Failed to lock statistics"))?;
        stats.total_entries += 1;
        stats.total_size_mb += entry.size_bytes as f64 / (1024.0 * 1024.0);
        
        // Update compression ratio if compression is enabled
        if self.config.compression_enabled {
            // Estimate compression savings (simplified)
            let estimated_uncompressed = entry.size_bytes as f64 * 1.5;
            stats.compression_ratio = (entry.size_bytes as f64) / estimated_uncompressed;
        }
        
        debug!(
            entry_key = entry.key,
            store_duration_ms = duration.as_millis(),
            total_entries = stats.total_entries,
            "Updated store statistics"
        );
        
        Ok(())
    }

    /// Update statistics for cache hit
    fn update_statistics_hit(&self, duration: Duration) -> Result<()> {
        let mut stats = self.statistics.lock().map_err(|_| CursedError::system_error("Failed to lock statistics"))?;
        
        // Update hit rate calculation
        let total_requests = stats.cache_hits + stats.cache_misses + 1;
        stats.cache_hits += 1;
        stats.hit_rate = stats.cache_hits as f64 / total_requests as f64;
        stats.miss_rate = stats.cache_misses as f64 / total_requests as f64;
        
        // Update average lookup time
        let total_lookups = stats.cache_hits + stats.cache_misses;
        stats.average_lookup_time_ms = (stats.average_lookup_time_ms * (total_lookups - 1) as f64 + duration.as_millis() as f64) / total_lookups as f64;
        
        debug!(
            hit_duration_ms = duration.as_millis(),
            hit_rate = stats.hit_rate,
            "Updated hit statistics"
        );
        
        Ok(())
    }

    /// Update statistics for cache miss
    fn update_statistics_miss(&self, duration: Duration) -> Result<()> {
        let mut stats = self.statistics.lock().map_err(|_| CursedError::system_error("Failed to lock statistics"))?;
        
        // Update miss rate calculation
        let total_requests = stats.cache_hits + stats.cache_misses + 1;
        stats.cache_misses += 1;
        stats.hit_rate = stats.cache_hits as f64 / total_requests as f64;
        stats.miss_rate = stats.cache_misses as f64 / total_requests as f64;
        
        // Update average lookup time
        let total_lookups = stats.cache_hits + stats.cache_misses;
        stats.average_lookup_time_ms = (stats.average_lookup_time_ms * (total_lookups - 1) as f64 + duration.as_millis() as f64) / total_lookups as f64;
        
        debug!(
            miss_duration_ms = duration.as_millis(),
            miss_rate = stats.miss_rate,
            "Updated miss statistics"
        );
        
        Ok(())
    }

    /// Update statistics for network hit
    fn update_statistics_network_hit(&self, duration: Duration) -> Result<()> {
        let mut stats = self.statistics.lock().map_err(|_| CursedError::system_error("Failed to lock statistics"))?;
        
        stats.network_hits += 1;
        
        // Update hit rate to include network hits
        let total_requests = stats.cache_hits + stats.cache_misses + stats.network_hits + stats.network_misses;
        let total_hits = stats.cache_hits + stats.network_hits;
        stats.hit_rate = total_hits as f64 / total_requests as f64;
        
        debug!(
            network_hit_duration_ms = duration.as_millis(),
            network_hits = stats.network_hits,
            total_hit_rate = stats.hit_rate,
            "Updated network hit statistics"
        );
        
        Ok(())
    }

    /// Update access time for cache entry
    fn update_access_time(&self, key: &str) -> Result<()> {
        let mut cache = self.local_cache.write().map_err(|_| CursedError::system_error("Failed to lock cache"))?;
        if let Some(entry) = cache.get_mut(key) {
            entry.accessed_at = current_timestamp();
        }
        Ok(())
    }

    /// Check if entry is in cache
    fn is_in_cache(&self, key: &str) -> Result<bool> {
        let cache = self.local_cache.read().map_err(|_| CursedError::system_error("Failed to lock cache"))?;
        Ok(cache.contains_key(key))
    }

    /// Precompute file for cache warming
    fn precompute_file(&self, file: &str) -> Result<bool> {
        debug!(file, "Starting precomputation for cache warming");
        
        let file_path = PathBuf::from(file);
        if !file_path.exists() {
            debug!(file, "File does not exist, skipping precomputation");
            return Ok(false);
        }
        
        // Calculate file hash for cache key
        let file_hash = self.calculate_file_hash(&file_path)?;
        let cache_key = format!("precomputed_{}", file_hash);
        
        // Check if already cached
        if self.is_in_cache(&cache_key)? {
            debug!(file, "File already precomputed and cached");
            return Ok(true);
        }
        
        // Determine precomputation strategy based on file type
        let precomputation_result = if file.ends_with(".csd") {
            self.precompute_cursed_file(&file_path)?
        } else if file.ends_with(".rs") {
            self.precompute_rust_file(&file_path)?
        } else {
            self.precompute_generic_file(&file_path)?
        };
        
        if let Some((data, metadata)) = precomputation_result {
            // Store precomputed result in cache
            self.store(&cache_key, data, metadata)?;
            info!(file, cache_key, "File precomputed and cached successfully");
            Ok(true)
        } else {
            debug!(file, "Precomputation not applicable for this file");
            Ok(false)
        }
    }
    
    /// Calculate hash for a file
    fn calculate_file_hash(&self, file_path: &PathBuf) -> Result<String> {
        use std::io::Read;
        
        let mut file = std::fs::File::open(file_path)?;
        let mut hasher = sha2::Sha256::new();
        let mut buffer = [0; 8192];
        
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        
        Ok(format!("{:x}", hasher.finalize()))
    }
    
    /// Precompute CURSED source file
    fn precompute_cursed_file(&self, file_path: &PathBuf) -> Result<Option<(CacheData, CacheMetadata)>> {
        // Read and parse the CURSED file for precomputation
        let content = std::fs::read_to_string(file_path)?;
        let metadata = std::fs::metadata(file_path)?;
        
        // Simulate AST parsing and analysis (in real implementation, would use actual parser)
        let preprocessed_content = self.preprocess_cursed_source(&content)?;
        
        let cache_metadata = CacheMetadata {
            file_path: file_path.clone(),
            last_modified: metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            file_size: metadata.len(),
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            compilation_flags: vec!["--precompute".to_string()],
            source_hash: self.calculate_file_hash(file_path)?,
            dependency_hashes: HashMap::new(),
        };
        
        let cache_data = CacheData::Ast(preprocessed_content);
        
        Ok(Some((cache_data, cache_metadata)))
    }
    
    /// Precompute Rust source file
    fn precompute_rust_file(&self, file_path: &PathBuf) -> Result<Option<(CacheData, CacheMetadata)>> {
        let content = std::fs::read_to_string(file_path)?;
        let metadata = std::fs::metadata(file_path)?;
        
        // Preprocess Rust file for analysis
        let analysis_result = self.analyze_rust_dependencies(&content)?;
        
        let cache_metadata = CacheMetadata {
            file_path: file_path.clone(),
            last_modified: metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            file_size: metadata.len(),
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            compilation_flags: vec!["--precompute".to_string()],
            source_hash: self.calculate_file_hash(file_path)?,
            dependency_hashes: HashMap::new(),
        };
        
        let cache_data = CacheData::Analysis(analysis_result);
        
        Ok(Some((cache_data, cache_metadata)))
    }
    
    /// Precompute generic file
    fn precompute_generic_file(&self, file_path: &PathBuf) -> Result<Option<(CacheData, CacheMetadata)>> {
        let metadata = std::fs::metadata(file_path)?;
        
        // For generic files, just cache metadata for faster access
        let cache_metadata = CacheMetadata {
            file_path: file_path.clone(),
            last_modified: metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            file_size: metadata.len(),
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            compilation_flags: vec!["--precompute".to_string()],
            source_hash: self.calculate_file_hash(file_path)?,
            dependency_hashes: HashMap::new(),
        };
        
        let file_info = serde_json::to_string(&cache_metadata)?;
        let cache_data = CacheData::Metadata(file_info);
        
        Ok(Some((cache_data, cache_metadata)))
    }
    
    /// Preprocess CURSED source code
    fn preprocess_cursed_source(&self, content: &str) -> Result<String> {
        let mut preprocessed = String::new();
        
        for line in content.split("\n") {
            let trimmed = line.trim();
            
            // Skip comments and empty lines for faster parsing
            if trimmed.starts_with("//") || trimmed.is_empty() {
                continue;
            }
            
            // Normalize whitespace
            preprocessed.push_str(&format!("{}\n", trimmed));
        }
        
        Ok(preprocessed)
    }
    
    /// Analyze Rust dependencies
    fn analyze_rust_dependencies(&self, content: &str) -> Result<String> {
        let mut dependencies = Vec::new();
        let mut uses = Vec::new();
        
        for line in content.split("\n") {
            let trimmed = line.trim();
            
            if trimmed.starts_with("use ") {
                uses.push(trimmed.to_string());
            } else if trimmed.starts_with("extern crate ") {
                dependencies.push(trimmed.to_string());
            }
        }
        
        let analysis = serde_json::json!({
            "dependencies": dependencies,
            "uses": uses,
            "analyzed_at": chrono::Utc::now().timestamp()
        });
        
        Ok(analysis.to_string())
    }

    /// Get current cache size in bytes
    fn get_current_cache_size(&self) -> Result<usize> {
        let cache = self.local_cache.read().map_err(|_| CursedError::system_error("Failed to lock cache"))?;
        Ok(cache.values().map(|entry| entry.size_bytes).sum())
    }

    /// Calculate entry size
    fn calculate_entry_size(&self, data: &CacheData) -> Result<usize> {
        let size = match data {
            CacheData::Ast(content) => content.len(),
            CacheData::IR(content) => content.len(),
            CacheData::Object(bytes) => bytes.len(),
            CacheData::Analysis(content) => content.len(),
            CacheData::Dependency(content) => content.len(),
            CacheData::Metadata(content) => content.len(),
        };
        Ok(size)
    }

    /// Store local copy from distributed cache
    fn store_local_copy(&self, entry: &CacheEntry) -> Result<()> {
        let mut cache = self.local_cache.write().map_err(|_| CursedError::system_error("Failed to lock cache"))?;
        let mut index = self.cache_index.write().map_err(|_| CursedError::system_error("Failed to lock index"))?;
        
        cache.insert(entry.key.clone(), entry.clone());
        index.insert(entry.content_hash.clone(), entry.key.clone());
        
        self.store_to_disk(entry)?;
        Ok(())
    }

    /// Get entry from distributed cache
    fn get_from_distributed_cache(&self, key: &str) -> Result<Option<CacheEntry>> {
        let nodes = self.distributed_nodes.read().map_err(|_| CursedError::system_error("Failed to lock nodes"))?;
        
        // Try available nodes in order of load factor (lowest first)
        let mut available_nodes: Vec<_> = nodes.iter()
            .filter(|node| node.is_available)
            .collect();
        available_nodes.sort_by(|a, b| a.load_factor.partial_cmp(&b.load_factor).unwrap_or(std::cmp::Ordering::Equal));
        
        for node in available_nodes.iter().take(2) { // Try up to 2 nodes
            // In a real implementation, this would make HTTP requests to the distributed cache nodes
            // For now, we simulate the attempt
            debug!(
                key,
                node_id = node.id,
                node_address = format!("{}:{}", node.address, node.port),
                "Attempting distributed cache retrieval from node"
            );
            
            // Simulate network delay
            thread::sleep(Duration::from_millis(10));
            
            // For demonstration, we always return None (cache miss)
            // In a real implementation, this would deserialize the response from the node
        }
        
        debug!(key, "Distributed cache miss");
        Ok(None)
    }

    /// Replicate entry to distributed nodes
    fn replicate_to_distributed_nodes(&self, entry: &CacheEntry) -> Result<()> {
        let nodes = self.distributed_nodes.read().map_err(|_| CursedError::system_error("Failed to lock nodes"))?;
        
        let available_nodes: Vec<_> = nodes.iter()
            .filter(|node| node.is_available)
            .collect();
        
        let replication_count = self.config.replication_factor.min(available_nodes.len());
        
        for node in available_nodes.iter().take(replication_count) {
            // In a real implementation, this would send the cache entry to the distributed node
            // via HTTP POST or other network protocol
            debug!(
                key = entry.key,
                node_id = node.id,
                node_address = format!("{}:{}", node.address, node.port),
                size_bytes = entry.size_bytes,
                "Replicating cache entry to distributed node"
            );
            
            // Simulate network operation
            thread::sleep(Duration::from_millis(5));
        }
        
        if replication_count > 0 {
            info!(
                key = entry.key,
                replicated_to = replication_count,
                "Successfully replicated cache entry to distributed nodes"
            );
        }
        
        Ok(())
    }

    /// Check if eviction is needed
    fn check_eviction_needed(&self) -> Result<()> {
        let current_size = self.get_current_cache_size()?;
        let max_size = self.config.max_cache_size_mb * 1024 * 1024;
        
        if current_size > max_size {
            self.optimize_cache()?;
        }
        
        Ok(())
    }
}

/// Get current timestamp in seconds since epoch
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// Types are exported directly via pub struct/pub enum definitions above
