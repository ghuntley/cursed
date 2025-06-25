/// Query and entity caching system for CURSED ORM
/// 
/// Provides intelligent caching strategies with TTL, invalidation,
/// and multiple backend support (memory, Redis) for performance optimization.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};
use tracing::{instrument, debug, info, warn, error};

use super::super::{DatabaseError, DatabaseErrorKind, SqlValue};

/// fr fr Query cache for storing query results
#[derive(Debug)]
pub struct QueryCache {
    /// Cache entries
    entries: Arc<RwLock<HashMap<String, CacheEntry>>>,
    /// Cache configuration
    config: CacheConfig,
    /// Cache statistics
    stats: Arc<Mutex<CacheStats>>,
}

impl QueryCache {
    /// slay Create new query cache
    #[instrument]
    pub fn new(config: CacheConfig) -> Self {
        info!("Creating new query cache");
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(Mutex::new(CacheStats::default())),
        }
    }

    /// facts Get cached value
    #[instrument(skip(self))]
    pub fn get<T: Clone + 'static>(&self, key: &str) -> Option<T> {
        debug!(key = key, "Getting value from cache");
        
        if let Ok(entries) = self.entries.read() {
            if let Some(entry) = entries.get(key) {
                if !entry.is_expired() {
                    if let Ok(mut stats) = self.stats.lock() {
                        stats.hits += 1;
                    }
                    
                    debug!("Cache hit");
                    return entry.value.downcast_ref::<T>().cloned();
                } else {
                    debug!("Cache entry expired");
                }
            }
        }
        
        if let Ok(mut stats) = self.stats.lock() {
            stats.misses += 1;
        }
        
        debug!("Cache miss");
        None
    }

    /// periodt Set cached value with TTL
    #[instrument(skip(self, value))]
    pub fn set<T: Clone + Send + Sync + std::fmt::Debug + 'static>(&mut self, key: String, value: T, ttl: Duration) {
        debug!(key = %key, ttl = ?ttl, "Setting value in cache");
        
        let entry = CacheEntry {
            value: Box::new(value),
            expires_at: Instant::now() + ttl,
            created_at: SystemTime::now(),
            access_count: 0,
        };
        
        if let Ok(mut entries) = self.entries.write() {
            // Check if we need to evict entries
            if entries.len() >= self.config.max_size {
                self.evict_expired_entries(&mut entries);
                
                // If still at capacity, use LRU eviction
                if entries.len() >= self.config.max_size {
                    self.evict_lru_entry(&mut entries);
                }
            }
            
            entries.insert(key, entry);
            
            if let Ok(mut stats) = self.stats.lock() {
                stats.sets += 1;
                stats.current_size = entries.len();
            }
        }
        
        debug!("Value cached successfully");
    }

    /// bestie Remove value from cache
    #[instrument(skip(self))]
    pub fn remove(&mut self, key: &str) -> bool {
        debug!(key = key, "Removing value from cache");
        
        if let Ok(mut entries) = self.entries.write() {
            let removed = entries.remove(key).is_some();
            
            if removed {
                if let Ok(mut stats) = self.stats.lock() {
                    stats.evictions += 1;
                    stats.current_size = entries.len();
                }
                debug!("Value removed from cache");
            }
            
            removed
        } else {
            false
        }
    }

    /// yolo Clear all cache entries
    #[instrument(skip(self))]
    pub fn clear(&mut self) {
        debug!("Clearing all cache entries");
        
        if let Ok(mut entries) = self.entries.write() {
            let count = entries.len();
            entries.clear();
            
            if let Ok(mut stats) = self.stats.lock() {
                stats.evictions += count as u64;
                stats.current_size = 0;
            }
        }
        
        info!("Cache cleared");
    }

    /// slay Invalidate entries matching pattern
    #[instrument(skip(self))]
    pub fn invalidate_pattern(&mut self, pattern: &str) {
        debug!(pattern = pattern, "Invalidating entries matching pattern");
        
        if let Ok(mut entries) = self.entries.write() {
            let keys_to_remove: Vec<String> = entries
                .keys()
                .filter(|key| self.matches_pattern(key, pattern))
                .cloned()
                .collect();
            
            let mut removed_count = 0;
            for key in keys_to_remove {
                if entries.remove(&key).is_some() {
                    removed_count += 1;
                }
            }
            
            if let Ok(mut stats) = self.stats.lock() {
                stats.evictions += removed_count;
                stats.current_size = entries.len();
            }
            
            debug!(removed = removed_count, "Entries invalidated");
        }
    }

    /// lit Get cache statistics
    #[instrument(skip(self))]
    pub fn stats(&self) -> CacheStats {
        if let Ok(stats) = self.stats.lock() {
            stats.clone()
        } else {
            CacheStats::default()
        }
    }

    /// tea Clean up expired entries
    #[instrument(skip(self))]
    pub fn cleanup_expired(&mut self) {
        debug!("Cleaning up expired entries");
        
        if let Ok(mut entries) = self.entries.write() {
            self.evict_expired_entries(&mut entries);
            
            if let Ok(mut stats) = self.stats.lock() {
                stats.current_size = entries.len();
            }
        }
    }

    // Helper methods
    fn evict_expired_entries(&self, entries: &mut HashMap<String, CacheEntry>) {
        let now = Instant::now();
        let expired_keys: Vec<String> = entries
            .iter()
            .filter(|(_, entry)| entry.expires_at <= now)
            .map(|(key, _)| key.clone())
            .collect();
        
        let mut evicted_count = 0;
        for key in expired_keys {
            if entries.remove(&key).is_some() {
                evicted_count += 1;
            }
        }
        
        if evicted_count > 0 {
            debug!(evicted = evicted_count, "Expired entries evicted");
            
            if let Ok(mut stats) = self.stats.lock() {
                stats.evictions += evicted_count;
            }
        }
    }
    
    fn evict_lru_entry(&self, entries: &mut HashMap<String, CacheEntry>) {
        // Simple LRU: remove entry with oldest created_at time
        if let Some((lru_key, _)) = entries
            .iter()
            .min_by_key(|(_, entry)| entry.created_at)
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            entries.remove(&lru_key);
            debug!(key = %lru_key, "LRU entry evicted");
            
            if let Ok(mut stats) = self.stats.lock() {
                stats.evictions += 1;
            }
        }
    }
    
    fn matches_pattern(&self, key: &str, pattern: &str) -> bool {
        // Simple glob-style pattern matching
        if pattern.ends_with('*') {
            let prefix = &pattern[..pattern.len() - 1];
            key.starts_with(prefix)
        } else if pattern.starts_with('*') {
            let suffix = &pattern[1..];
            key.ends_with(suffix)
        } else {
            key == pattern
        }
    }
}

/// fr fr Cache entry with metadata
#[derive(Debug, Clone)]
struct CacheEntry {
    /// Cached value
    value: Box<dyn CacheValue>,
    /// Expiration time
    expires_at: Instant,
    /// Creation time
    created_at: SystemTime,
    /// Access count for LRU
    access_count: u64,
}

impl CacheEntry {
    /// Check if entry has expired
    fn is_expired(&self) -> bool {
        Instant::now() >= self.expires_at
    }
}

/// fr fr Trait for values that can be cached
trait CacheValue: Send + Sync + std::fmt::Debug + 'static {
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn CacheValue>;
}

impl<T: Clone + Send + Sync + std::fmt::Debug + 'static> CacheValue for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn CacheValue> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CacheValue> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl dyn CacheValue + 'static {
    fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

/// fr fr Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries
    pub max_size: usize,
    /// Default TTL for cache entries
    pub default_ttl: Duration,
    /// Enable query result caching
    pub enable_query_cache: bool,
    /// Enable entity caching
    pub enable_entity_cache: bool,
    /// Cleanup interval for expired entries
    pub cleanup_interval: Duration,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size: 10000,
            default_ttl: Duration::from_secs(3600), // 1 hour
            enable_query_cache: true,
            enable_entity_cache: true,
            cleanup_interval: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// fr fr Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Cache hits
    pub hits: u64,
    /// Cache misses
    pub misses: u64,
    /// Cache sets
    pub sets: u64,
    /// Cache evictions
    pub evictions: u64,
    /// Current cache size
    pub current_size: usize,
}

impl CacheStats {
    /// Calculate hit ratio
    pub fn hit_ratio(&self) -> f64 {
        if self.hits + self.misses == 0 {
            0.0
        } else {
            self.hits as f64 / (self.hits + self.misses) as f64
        }
    }
}

/// fr fr Cache strategy enum for different caching approaches
#[derive(Debug, Clone, PartialEq)]
pub enum CacheStrategy {
    /// No caching
    None,
    /// In-memory caching only
    Memory,
    /// Redis caching
    Redis,
    /// Multi-level caching (memory + Redis)
    MultiLevel,
}

/// fr fr Entity cache for caching entity instances
#[derive(Debug)]
pub struct EntityCache {
    /// Cache implementation
    cache: QueryCache,
    /// Entity type tracking
    entity_types: Arc<Mutex<HashMap<String, String>>>,
}

impl EntityCache {
    /// slay Create new entity cache
    #[instrument]
    pub fn new(config: CacheConfig) -> Self {
        info!("Creating new entity cache");
        Self {
            cache: QueryCache::new(config),
            entity_types: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// facts Cache entity by primary key
    #[instrument(skip(self, entity))]
    pub fn cache_entity<T: super::entity::Entity + Clone + Send + Sync + std::fmt::Debug + 'static>(
        &mut self,
        entity: T,
        ttl: Duration,
    ) -> crate::error::Result<()> {
        debug!(entity = T::table_name(), "Caching entity");
        
        if let Some(pk_value) = entity.primary_key_value() {
            let cache_key = format!("entity:{}:{}", T::table_name(), pk_value);
            self.cache.set(cache_key.clone(), entity, ttl);
            
            // Track entity type
            if let Ok(mut types) = self.entity_types.lock() {
                types.insert(cache_key, T::table_name().to_string());
            }
            
            debug!("Entity cached successfully");
            Ok(())
        } else {
            Err(DatabaseError::validation_error("Entity must have primary key to be cached"))
        }
    }

    /// periodt Get cached entity by primary key
    #[instrument(skip(self))]
    pub fn get_entity<T: super::entity::Entity + Clone + 'static>(
        &self,
        primary_key: SqlValue,
    ) -> Option<T> {
        let cache_key = format!("entity:{}:{}", T::table_name(), primary_key);
        debug!(key = %cache_key, "Getting cached entity");
        
        self.cache.get::<T>(&cache_key)
    }

    /// bestie Invalidate cached entity
    #[instrument(skip(self))]
    pub fn invalidate_entity<T: super::entity::Entity>(&mut self, primary_key: SqlValue) -> bool {
        let cache_key = format!("entity:{}:{}", T::table_name(), primary_key);
        debug!(key = %cache_key, "Invalidating cached entity");
        
        self.cache.remove(&cache_key)
    }

    /// yolo Invalidate all entities of a type
    #[instrument(skip(self))]
    pub fn invalidate_entity_type<T: super::entity::Entity>(&mut self) {
        let pattern = format!("entity:{}:*", T::table_name());
        debug!(pattern = %pattern, "Invalidating all entities of type");
        
        self.cache.invalidate_pattern(&pattern);
    }

    /// slay Get cache statistics
    #[instrument(skip(self))]
    pub fn stats(&self) -> CacheStats {
        self.cache.stats()
    }
}

/// fr fr Memory cache implementation
#[derive(Debug)]
pub struct MemoryCache {
    /// Query cache
    query_cache: QueryCache,
    /// Entity cache
    entity_cache: EntityCache,
}

impl MemoryCache {
    /// slay Create new memory cache
    #[instrument]
    pub fn new(config: CacheConfig) -> Self {
        info!("Creating new memory cache");
        Self {
            query_cache: QueryCache::new(config.clone()),
            entity_cache: EntityCache::new(config),
        }
    }

    /// facts Get query cache
    pub fn query_cache(&self) -> &QueryCache {
        &self.query_cache
    }

    /// periodt Get mutable query cache
    pub fn query_cache_mut(&mut self) -> &mut QueryCache {
        &mut self.query_cache
    }

    /// bestie Get entity cache
    pub fn entity_cache(&self) -> &EntityCache {
        &self.entity_cache
    }

    /// yolo Get mutable entity cache
    pub fn entity_cache_mut(&mut self) -> &mut EntityCache {
        &mut self.entity_cache
    }

    /// slay Clear all caches
    #[instrument(skip(self))]
    pub fn clear_all(&mut self) {
        info!("Clearing all memory caches");
        self.query_cache.clear();
        self.entity_cache.cache.clear();
    }

    /// lit Get combined cache statistics
    #[instrument(skip(self))]
    pub fn combined_stats(&self) -> CombinedCacheStats {
        CombinedCacheStats {
            query_stats: self.query_cache.stats(),
            entity_stats: self.entity_cache.stats(),
        }
    }
}

/// fr fr Redis cache implementation (placeholder)
#[derive(Debug)]
pub struct RedisCache {
    /// Redis connection details
    connection_string: String,
    /// Cache configuration
    config: CacheConfig,
}

impl RedisCache {
    /// slay Create new Redis cache
    #[instrument]
    pub fn new(connection_string: String, config: CacheConfig) -> Self {
        info!("Creating new Redis cache");
        Self {
            connection_string,
            config,
        }
    }

    /// facts Get value from Redis
    #[instrument(skip(self))]
    pub async fn get(&self, key: &str) -> crate::error::Result<()> {
        debug!(key = key, "Getting value from Redis");
        // Placeholder implementation
        Ok(None)
    }

    /// periodt Set value in Redis
    #[instrument(skip(self, value))]
    pub async fn set(&self, key: &str, value: Vec<u8>, ttl: Duration) -> crate::error::Result<()> {
        debug!(key = key, ttl = ?ttl, "Setting value in Redis");
        // Placeholder implementation
        Ok(())
    }

    /// bestie Delete value from Redis
    #[instrument(skip(self))]
    pub async fn delete(&self, key: &str) -> crate::error::Result<()> {
        debug!(key = key, "Deleting value from Redis");
        // Placeholder implementation
        Ok(true)
    }
}

/// fr fr Cache invalidator for smart cache invalidation
#[derive(Debug)]
pub struct CacheInvalidator {
    /// Memory cache reference
    memory_cache: Arc<Mutex<MemoryCache>>,
    /// Redis cache reference
    redis_cache: Option<Arc<RedisCache>>,
    /// Invalidation rules
    invalidation_rules: Arc<Mutex<HashMap<String, Vec<InvalidationRule>>>>,
}

impl CacheInvalidator {
    /// slay Create new cache invalidator
    #[instrument(skip(memory_cache, redis_cache))]
    pub fn new(
        memory_cache: Arc<Mutex<MemoryCache>>,
        redis_cache: Option<Arc<RedisCache>>,
    ) -> Self {
        info!("Creating new cache invalidator");
        Self {
            memory_cache,
            redis_cache,
            invalidation_rules: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// facts Add invalidation rule
    #[instrument(skip(self))]
    pub fn add_rule(&self, entity_type: &str, rule: InvalidationRule) {
        debug!(entity = entity_type, rule = ?rule, "Adding invalidation rule");
        
        if let Ok(mut rules) = self.invalidation_rules.lock() {
            rules.entry(entity_type.to_string()).or_insert_with(Vec::new).push(rule);
        }
    }

    /// periodt Invalidate caches based on entity change
    #[instrument(skip(self))]
    pub async fn invalidate_for_entity(&self, entity_type: &str, operation: CacheOperation) -> crate::error::Result<()> {
        info!(entity = entity_type, operation = ?operation, "Invalidating caches for entity");
        
        let rules = if let Ok(invalidation_rules) = self.invalidation_rules.lock() {
            invalidation_rules.get(entity_type).cloned().unwrap_or_default()
        } else {
            Vec::new()
        };

        for rule in rules {
            if rule.matches_operation(&operation) {
                self.execute_invalidation_rule(&rule).await?;
            }
        }

        Ok(())
    }

    async fn execute_invalidation_rule(&self, rule: &InvalidationRule) -> crate::error::Result<()> {
        match rule {
            InvalidationRule::InvalidatePattern { pattern } => {
                if let Ok(mut cache) = self.memory_cache.lock() {
                    cache.query_cache_mut().invalidate_pattern(pattern);
                }
            }
            InvalidationRule::InvalidateEntityType { entity_type } => {
                // Would need generic handling for different entity types
                debug!(entity_type = entity_type, "Invalidating entity type cache");
            }
            InvalidationRule::ClearAll => {
                if let Ok(mut cache) = self.memory_cache.lock() {
                    cache.clear_all();
                }
            }
        }
        Ok(())
    }
}

/// fr fr Cache invalidation rules
#[derive(Debug, Clone)]
pub enum InvalidationRule {
    /// Invalidate entries matching pattern
    InvalidatePattern { pattern: String },
    /// Invalidate all entities of a type
    InvalidateEntityType { entity_type: String },
    /// Clear all caches
    ClearAll,
}

impl InvalidationRule {
    fn matches_operation(&self, _operation: &CacheOperation) -> bool {
        // Simplified - would implement proper matching logic
        true
    }
}

/// fr fr Cache operations that trigger invalidation
#[derive(Debug, Clone)]
pub enum CacheOperation {
    Create,
    Update,
    Delete,
    BulkUpdate,
    BulkDelete,
}

/// fr fr Combined cache statistics
#[derive(Debug, Clone)]
pub struct CombinedCacheStats {
    pub query_stats: CacheStats,
    pub entity_stats: CacheStats,
}

impl CombinedCacheStats {
    /// Calculate overall hit ratio
    pub fn overall_hit_ratio(&self) -> f64 {
        let total_hits = self.query_stats.hits + self.entity_stats.hits;
        let total_requests = self.query_stats.hits + self.query_stats.misses + 
                           self.entity_stats.hits + self.entity_stats.misses;
        
        if total_requests == 0 {
            0.0
        } else {
            total_hits as f64 / total_requests as f64
        }
    }
}

