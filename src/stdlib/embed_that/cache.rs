use crate::stdlib::embed_that::core::{ThatFile, tea, lit};
use crate::stdlib::embed_that::error::{EmbedError, EmbedResult};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Resource cache for embedded files
#[derive(Debug)]
pub struct ResourceCache {
    cache: Arc<RwLock<HashMap<tea, CacheEntry>>>,
    expiry_duration: Option<Duration>,
    max_size: Option<usize>,
    access_tracking: Arc<RwLock<HashMap<tea, Instant>>>,
}

/// Cache entry with expiration support
#[derive(Debug, Clone)]
struct CacheEntry {
    value: CacheValue,
    created_at: Instant,
    access_count: u64,
    last_accessed: Instant,
}

/// Different types of values that can be cached
#[derive(Debug, Clone)]
enum CacheValue {
    File(ThatFile),
    Data(Vec<u8>),
    String(tea),
    Json(serde_json::Value),
    Generic(Arc<dyn std::any::Any + Send + Sync>),
}

impl ResourceCache {
    /// Create a new resource cache without expiry
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            expiry_duration: None,
            max_size: None,
            access_tracking: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create a new resource cache with expiry duration
    pub fn new_with_expiry(expiry: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            expiry_duration: Some(expiry),
            max_size: None,
            access_tracking: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create a new resource cache with size limit
    pub fn new_with_size_limit(max_size: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            expiry_duration: None,
            max_size: Some(max_size),
            access_tracking: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create a new resource cache with both expiry and size limit
    pub fn new_with_expiry_and_size(expiry: Duration, max_size: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            expiry_duration: Some(expiry),
            max_size: Some(max_size),
            access_tracking: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Get a value from the cache
    pub fn get(&self, key: &tea) -> (Option<CacheValue>, lit) {
        self.cleanup_expired_entries();
        
        if let Ok(mut cache) = self.cache.write() {
            if let Some(entry) = cache.get_mut(key) {
                // Update access statistics
                entry.access_count += 1;
                entry.last_accessed = Instant::now();
                
                // Update access tracking
                if let Ok(mut tracking) = self.access_tracking.write() {
                    tracking.insert(key.clone(), Instant::now());
                }
                
                return (Some(entry.value.clone()), true);
            }
        }
        
        (None, false)
    }
    
    /// Set a value in the cache
    pub fn set(&self, key: tea, value: CacheValue) {
        self.cleanup_expired_entries();
        self.enforce_size_limit();
        
        let entry = CacheEntry {
            value,
            created_at: Instant::now(),
            access_count: 1,
            last_accessed: Instant::now(),
        };
        
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(key.clone(), entry);
        }
        
        if let Ok(mut tracking) = self.access_tracking.write() {
            tracking.insert(key, Instant::now());
        }
    }
    
    /// Delete a value from the cache
    pub fn delete(&self, key: &tea) {
        if let Ok(mut cache) = self.cache.write() {
            cache.remove(key);
        }
        
        if let Ok(mut tracking) = self.access_tracking.write() {
            tracking.remove(key);
        }
    }
    
    /// Clear all values from the cache
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
        
        if let Ok(mut tracking) = self.access_tracking.write() {
            tracking.clear();
        }
    }
    
    /// Load a file and cache it
    pub fn load_file(&self, path: &tea) -> EmbedResult<ThatFile> {
        // Check if already cached
        if let (Some(CacheValue::File(file)), true) = self.get(path) {
            return Ok(file);
        }
        
        // Load from embedded files
        let file = super::resource_loader::load_that_file(path)?;
        
        // Cache the file
        self.set(path.clone(), CacheValue::File(file.clone()));
        
        Ok(file)
    }
    
    /// Load and parse JSON, caching the result
    pub fn load_json<T>(&self, path: &tea, target: &mut T) -> EmbedResult<()>
    where
        T: serde::de::DeserializeOwned + Clone,
    {
        let cache_key = format!("json:{}", path);
        
        // Check if JSON value is already cached
        if let (Some(CacheValue::Json(json_value)), true) = self.get(&cache_key) {
            *target = serde_json::from_value(json_value)
                .map_err(|e| EmbedError::JsonParsingError { reason: e.to_string() })?;
            return Ok(());
        }
        
        // Load file and parse JSON
        let file = self.load_file(path)?;
        let content = file.content_string()?;
        
        let json_value: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| EmbedError::JsonParsingError { reason: e.to_string() })?;
        
        // Cache the JSON value
        self.set(cache_key, CacheValue::Json(json_value.clone()));
        
        // Convert to target type
        *target = serde_json::from_value(json_value)
            .map_err(|e| EmbedError::JsonParsingError { reason: e.to_string() })?;
        
        Ok(())
    }
    
    /// Load and cache string content
    pub fn load_string(&self, path: &tea) -> EmbedResult<tea> {
        let cache_key = format!("string:{}", path);
        
        // Check if string is already cached
        if let (Some(CacheValue::String(content)), true) = self.get(&cache_key) {
            return Ok(content);
        }
        
        // Load file and get string content
        let file = self.load_file(path)?;
        let content = file.content_string()?;
        
        // Cache the string
        self.set(cache_key, CacheValue::String(content.clone()));
        
        Ok(content)
    }
    
    /// Load and cache binary data
    pub fn load_binary(&self, path: &tea) -> EmbedResult<Vec<u8>> {
        let cache_key = format!("binary:{}", path);
        
        // Check if binary data is already cached
        if let (Some(CacheValue::Data(data)), true) = self.get(&cache_key) {
            return Ok(data);
        }
        
        // Load file and get binary content
        let file = self.load_file(path)?;
        let content = file.content();
        
        // Cache the binary data
        self.set(cache_key, CacheValue::Data(content.clone()));
        
        Ok(content)
    }
    
    /// Get cache statistics
    pub fn get_statistics(&self) -> CacheStatistics {
        let cache = self.cache.read().unwrap_or_else(|_| {
            // Return empty statistics if lock is poisoned
            return parking_lot::RwLock::new(HashMap::new()).read();
        });
        
        let total_entries = cache.len();
        let total_access_count: u64 = cache.values().map(|e| e.access_count).sum();
        
        let mut entries_by_type = HashMap::new();
        for entry in cache.values() {
            let type_name = match &entry.value {
                CacheValue::File(_) => "File",
                CacheValue::Data(_) => "Data", 
                CacheValue::String(_) => "String",
                CacheValue::Json(_) => "Json",
                CacheValue::Generic(_) => "Generic",
            };
            *entries_by_type.entry(type_name.to_string()).or_insert(0) += 1;
        }
        
        // Calculate memory usage estimate
        let estimated_memory = cache.values().map(|entry| {
            match &entry.value {
                CacheValue::File(file) => file.size() as usize,
                CacheValue::Data(data) => data.len(),
                CacheValue::String(s) => s.len(),
                CacheValue::Json(json) => json.to_string().len(),
                CacheValue::Generic(_) => 64, // Rough estimate
            }
        }).sum();
        
        CacheStatistics {
            total_entries,
            total_access_count,
            entries_by_type,
            estimated_memory_usage: estimated_memory,
            expiry_duration: self.expiry_duration,
            max_size: self.max_size,
        }
    }
    
    /// Clean up expired entries
    fn cleanup_expired_entries(&self) {
        if let Some(expiry_duration) = self.expiry_duration {
            let now = Instant::now();
            
            if let Ok(mut cache) = self.cache.write() {
                cache.retain(|_, entry| {
                    now.duration_since(entry.created_at) < expiry_duration
                });
            }
            
            if let Ok(mut tracking) = self.access_tracking.write() {
                tracking.retain(|_, &mut access_time| {
                    now.duration_since(access_time) < expiry_duration
                });
            }
        }
    }
    
    /// Enforce size limit using LRU eviction
    fn enforce_size_limit(&self) {
        if let Some(max_size) = self.max_size {
            if let Ok(mut cache) = self.cache.write() {
                if cache.len() >= max_size {
                    // Find least recently used entries
                    let mut entries: Vec<_> = cache.iter().collect();
                    entries.sort_by_key(|(_, entry)| entry.last_accessed);
                    
                    // Remove oldest entries until we're under the size limit
                    let to_remove = cache.len() - max_size + 1;
                    for (key, _) in entries.into_iter().take(to_remove) {
                        cache.remove(key);
                    }
                }
            }
        }
    }
    
    /// Check if a key exists in the cache
    pub fn contains_key(&self, key: &tea) -> lit {
        self.cleanup_expired_entries();
        
        if let Ok(cache) = self.cache.read() {
            cache.contains_key(key)
        } else {
            false
        }
    }
    
    /// Get all keys currently in the cache
    pub fn keys(&self) -> Vec<tea> {
        self.cleanup_expired_entries();
        
        if let Ok(cache) = self.cache.read() {
            cache.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get the current size of the cache
    pub fn len(&self) -> usize {
        self.cleanup_expired_entries();
        
        if let Ok(cache) = self.cache.read() {
            cache.len()
        } else {
            0
        }
    }
    
    /// Check if the cache is empty
    pub fn is_empty(&self) -> lit {
        self.len() == 0
    }
}

impl Default for ResourceCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ResourceCache {
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
            expiry_duration: self.expiry_duration,
            max_size: self.max_size,
            access_tracking: Arc::clone(&self.access_tracking),
        }
    }
}

/// Statistics about cache usage
#[derive(Debug, Clone)]
pub struct CacheStatistics {
    pub total_entries: usize,
    pub total_access_count: u64,
    pub entries_by_type: HashMap<tea, u32>,
    pub estimated_memory_usage: usize,
    pub expiry_duration: Option<Duration>,
    pub max_size: Option<usize>,
}

impl CacheStatistics {
    /// Get the cache hit rate (requires tracking hits/misses separately)
    pub fn hit_rate(&self) -> f64 {
        // This would require additional tracking in a full implementation
        0.0
    }
    
    /// Get the average access count per entry
    pub fn average_access_count(&self) -> f64 {
        if self.total_entries > 0 {
            self.total_access_count as f64 / self.total_entries as f64
        } else {
            0.0
        }
    }
    
    /// Get memory usage in a human-readable format
    pub fn memory_usage_human(&self) -> tea {
        let bytes = self.estimated_memory_usage;
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

/// Global cache instance for convenient access
static GLOBAL_CACHE: once_cell::sync::Lazy<ResourceCache> = 
    once_cell::sync::Lazy::new(|| ResourceCache::new());

/// Cache configuration options
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub expiry_duration: Option<Duration>,
    pub max_size: Option<usize>,
    pub enable_access_tracking: bool,
    pub cleanup_interval: Option<Duration>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            expiry_duration: Some(Duration::from_secs(3600)), // 1 hour default
            max_size: Some(1000), // 1000 entries default
            enable_access_tracking: true,
            cleanup_interval: Some(Duration::from_secs(300)), // 5 minutes
        }
    }
}

/// Public API functions for resource caching
pub fn new_resource_cache() -> ResourceCache {
    ResourceCache::new()
}

pub fn new_resource_cache_with_expiry(expiry: Duration) -> ResourceCache {
    ResourceCache::new_with_expiry(expiry)
}

pub fn new_resource_cache_with_config(config: CacheConfig) -> ResourceCache {
    match (config.expiry_duration, config.max_size) {
        (Some(expiry), Some(size)) => ResourceCache::new_with_expiry_and_size(expiry, size),
        (Some(expiry), None) => ResourceCache::new_with_expiry(expiry),
        (None, Some(size)) => ResourceCache::new_with_size_limit(size),
        (None, None) => ResourceCache::new(),
    }
}

/// Get the global cache instance
pub fn get_global_cache() -> &'static ResourceCache {
    &GLOBAL_CACHE
}

/// Helper functions for working with the global cache
pub mod global {
    use super::*;
    
    pub fn get(key: &tea) -> (Option<CacheValue>, lit) {
        GLOBAL_CACHE.get(key)
    }
    
    pub fn set_string(key: tea, value: tea) {
        GLOBAL_CACHE.set(key, CacheValue::String(value));
    }
    
    pub fn set_data(key: tea, value: Vec<u8>) {
        GLOBAL_CACHE.set(key, CacheValue::Data(value));
    }
    
    pub fn set_file(key: tea, value: ThatFile) {
        GLOBAL_CACHE.set(key, CacheValue::File(value));
    }
    
    pub fn load_file(path: &tea) -> EmbedResult<ThatFile> {
        GLOBAL_CACHE.load_file(path)
    }
    
    pub fn load_string(path: &tea) -> EmbedResult<tea> {
        GLOBAL_CACHE.load_string(path)
    }
    
    pub fn load_json<T>(path: &tea, target: &mut T) -> EmbedResult<()>
    where
        T: serde::de::DeserializeOwned + Clone,
    {
        GLOBAL_CACHE.load_json(path, target)
    }
    
    pub fn clear() {
        GLOBAL_CACHE.clear();
    }
    
    pub fn get_statistics() -> CacheStatistics {
        GLOBAL_CACHE.get_statistics()
    }
}
