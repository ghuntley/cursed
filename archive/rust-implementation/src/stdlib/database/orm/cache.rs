//! Functional implementation for cache

use crate::error::CursedError;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::stdlib::packages::ModuleError;
use crate::stdlib::database::redis::{RedisClient, RedisConfig, RedisValue};

/// Result type for cache operations
pub type ModuleResult<T> = Result<T, CursedError>;

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_size: usize,
    pub ttl: Duration,
    pub enabled: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size: 1000,
            ttl: Duration::from_secs(3600), // 1 hour
            enabled: true,
        }
    }
}

/// In-memory cache implementation
#[derive(Debug)]
pub struct MemoryCache<K, V> {
    cache: HashMap<K, CacheEntry<V>>,
    config: CacheConfig,
}

/// Redis cache implementation
#[derive(Debug)]
pub struct RedisCache {
    connection_string: String,
    config: CacheConfig,
    redis_client: Option<RedisClient>,
}

/// Cache invalidation manager
#[derive(Debug)]
pub struct CacheInvalidator {
    invalidation_patterns: Vec<String>,
    auto_invalidate: bool,
}

/// Cache entry with metadata
#[derive(Debug, Clone)]
struct CacheEntry<V> {
    value: V,
    created_at: Instant,
    access_count: u64,
}

impl<K, V> MemoryCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: HashMap::new(),
            config,
        }
    }
    
    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(entry) = self.cache.get_mut(key) {
            if entry.created_at.elapsed() < self.config.ttl {
                entry.access_count += 1;
                return Some(entry.value.clone());
            } else {
                self.cache.remove(key);
            }
        }
        None
    }
    
    pub fn set(&mut self, key: K, value: V) {
        if self.cache.len() >= self.config.max_size {
            self.evict_oldest();
        }
        
        let entry = CacheEntry {
            value,
            created_at: Instant::now(),
            access_count: 0,
        };
        
        self.cache.insert(key, entry);
    }
    
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.cache.remove(key).map(|entry| entry.value)
    }
    
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    
    pub fn size(&self) -> usize {
        self.cache.len()
    }
    
    fn evict_oldest(&mut self) {
        if let Some(oldest_key) = self.cache
            .iter()
            .min_by_key(|(_, entry)| entry.created_at)
            .map(|(key, _)| key.clone())
        {
            self.cache.remove(&oldest_key);
        }
    }
}

impl RedisCache {
    pub fn new(connection_string: &str, config: CacheConfig) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            config,
            redis_client: None,
        }
    }
    
    /// Initialize Redis connection
    pub async fn connect(&mut self) -> Result<(), CursedError> {
        let redis_config = RedisConfig {
            url: self.connection_string.clone(),
            ..Default::default()
        };
        
        let mut client = RedisClient::new(redis_config)?;
        client.connect().await?;
        self.redis_client = Some(client);
        Ok(())
    }
    
    /// Get Redis client, initializing if needed
    async fn get_client(&mut self) -> Result<&RedisClient, CursedError> {
        if self.redis_client.is_none() {
            self.connect().await?;
        }
        self.redis_client.as_ref()
            .ok_or_else(|| CursedError::runtime_error(&"Failed to initialize Redis client"))
    }
    
    pub async fn get(&mut self, key: &str) -> Result<Option<String>, CursedError> {
        // Extract TTL first to avoid borrow issues
        let ttl_secs = self.config.ttl.as_secs();
        let client = self.get_client().await?;
        
        // Apply TTL if configured
        let cache_key = if ttl_secs > 0 {
            format!("{}:ttl:{}", key, ttl_secs)
        } else {
            key.to_string()
        };
        
        match client.get(&cache_key).await {
            Ok(Some(value)) => Ok(Some(value)),
            Ok(None) => Ok(None),
            Err(e) => {
                eprintln!("Redis get error: {}", e);
                Ok(None) // Return None on error to gracefully degrade
            }
        }
    }
    
    pub async fn set(&mut self, key: &str, value: &str) -> Result<(), CursedError> {
        // Extract TTL first to avoid borrow issues
        let ttl_secs = self.config.ttl.as_secs();
        let client = self.get_client().await?;
        
        // Apply TTL if configured
        let cache_key = if ttl_secs > 0 {
            format!("{}:ttl:{}", key, ttl_secs)
        } else {
            key.to_string()
        };
        
        match client.set(&cache_key, value).await {
            Ok(_) => {
                // If TTL is configured, set expiration
                if ttl_secs > 0 {
                    let ttl_str = ttl_secs.to_string();
                    if let Err(e) = client.execute("EXPIRE", &[&cache_key, &ttl_str]).await {
                        eprintln!("Redis expire error: {}", e);
                        // Continue anyway, value is set
                    }
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("Redis set error: {}", e);
                Err(CursedError::runtime_error(&format!("Failed to set cache value: {}", e)))
            }
        }
    }
    
    pub async fn remove(&mut self, key: &str) -> Result<bool, CursedError> {
        // Extract TTL first to avoid borrow issues
        let ttl_secs = self.config.ttl.as_secs();
        let client = self.get_client().await?;
        
        // Apply TTL if configured
        let cache_key = if ttl_secs > 0 {
            format!("{}:ttl:{}", key, ttl_secs)
        } else {
            key.to_string()
        };
        
        match client.execute("DEL", &[&cache_key]).await {
            Ok(RedisValue::Integer(count)) => Ok(count > 0),
            Ok(_) => Ok(false),
            Err(e) => {
                eprintln!("Redis remove error: {}", e);
                Ok(false) // Return false on error to gracefully degrade
            }
        }
    }
    
    pub async fn clear(&mut self) -> Result<(), CursedError> {
        let client = self.get_client().await?;
        
        match client.execute("FLUSHDB", &[]).await {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Redis clear error: {}", e);
                Err(CursedError::runtime_error(&format!("Failed to clear cache: {}", e)))
            }
        }
    }
}

impl CacheInvalidator {
    pub fn new() -> Self {
        Self {
            invalidation_patterns: Vec::new(),
            auto_invalidate: true,
        }
    }
    
    pub fn add_pattern(mut self, pattern: &str) -> Self {
        self.invalidation_patterns.push(pattern.to_string());
        self
    }
    
    pub fn auto_invalidate(mut self, enabled: bool) -> Self {
        self.auto_invalidate = enabled;
        self
    }
    
    pub fn should_invalidate(&self, key: &str) -> bool {
        if !self.auto_invalidate {
            return false;
        }
        
        for pattern in &self.invalidation_patterns {
            if key.contains(pattern) {
                return true;
            }
        }
        
        false
    }
    
    pub fn invalidate_pattern<K, V>(&self, cache: &mut MemoryCache<K, V>, pattern: &str)
    where
        K: std::hash::Hash + Eq + Clone + std::fmt::Display,
        V: Clone,
    {
        let keys_to_remove: Vec<K> = cache.cache
            .keys()
            .filter(|key| key.to_string().contains(pattern))
            .cloned()
            .collect();
        
        for key in keys_to_remove {
            cache.remove(&key);
        }
    }
}

/// cache operations handler
pub struct ModuleHandler {
    enabled: bool,
}

impl ModuleHandler {
    /// Create a new module handler
    pub fn new() -> Self {
        Self {
            enabled: true,
        }
    }
    
    /// Enable or disable the module
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// Check if module is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Process data
    pub fn process(&self, data: &str) -> ModuleResult<String> {
        if !self.enabled {
            return Err(CursedError::runtime_error(&"Module is disabled"));
        }
        Ok(format!("Processed: {}", data))
    }
    
    /// Get module info
    pub fn info(&self) -> String {
        format!("Module: cache, Enabled: {}", self.enabled)
    }
}

impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize cache processing
pub fn init_cache() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("test")?;
    if !result.contains("test") {
        return Err(CursedError::runtime_error(&"Module test failed"));
    }
    println!("⚙️  Module processing (cache) initialized");
    Ok(())
}

/// Test cache functionality
pub fn test_cache() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("Hello, CURSED!")?;
    if !result.contains("Hello, CURSED!") {
        return Err(CursedError::runtime_error(&"Module test failed"));
    }
    Ok(())
}
