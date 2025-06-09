/// Template Cache - High-performance caching system for CURSED templates
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime};
use tracing::{debug, instrument, warn};

use crate::error::CursedError;
use super::template_syntax::TemplateAst;

/// Template cache entry
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Compiled template AST
    pub ast: TemplateAst,
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
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of cache evictions
    pub evictions: u64,
    /// Total number of entries
    pub entries: usize,
    /// Memory usage estimate (in bytes)
    pub memory_usage: usize,
    /// Cache hit ratio (0.0 to 1.0)
    pub hit_ratio: f64,
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

/// Template cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries
    pub max_entries: usize,
    /// Maximum memory usage (in bytes)
    pub max_memory: usize,
    /// Eviction policy
    pub eviction_policy: EvictionPolicy,
    /// Enable cache statistics
    pub enable_stats: bool,
    /// Auto-refresh interval for file-based templates
    pub auto_refresh_interval: Option<Duration>,
    /// Enable compression for stored templates
    pub enable_compression: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 1000,
            max_memory: 100 * 1024 * 1024, // 100 MB
            eviction_policy: EvictionPolicy::Lru,
            enable_stats: true,
            auto_refresh_interval: Some(Duration::from_secs(60)),
            enable_compression: false,
        }
    }
}

/// High-performance template cache
#[derive(Debug)]
pub struct TemplateCache {
    /// Cache entries
    entries: Arc<RwLock<HashMap<String, CacheEntry>>>,
    /// Cache configuration
    config: CacheConfig,
    /// Cache statistics
    stats: Arc<RwLock<CacheStats>>,
    /// LRU access order (for LRU eviction)
    access_order: Arc<RwLock<Vec<String>>>,
}

impl TemplateCache {
    /// Create a new template cache with default configuration
    pub fn new(max_entries: usize) -> Self {
        let config = CacheConfig {
            max_entries,
            ..CacheConfig::default()
        };
        Self::with_config(config)
    }

    /// Create a new template cache with custom configuration
    pub fn with_config(config: CacheConfig) -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(CacheStats {
                hits: 0,
                misses: 0,
                evictions: 0,
                entries: 0,
                memory_usage: 0,
                hit_ratio: 0.0,
            })),
            access_order: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get a template from the cache
    #[instrument(skip(self))]
    pub fn get(&self, key: &str) -> Option<TemplateAst> {
        debug!(key = key, "Cache lookup");

        let mut entries = self.entries.write().ok()?;
        let mut stats = self.stats.write().ok()?;

        if let Some(entry) = entries.get_mut(key) {
            // Update access information
            entry.last_accessed = Instant::now();
            entry.access_count += 1;

            // Update LRU order
            if let Ok(mut access_order) = self.access_order.write() {
                access_order.retain(|k| k != key);
                access_order.push(key.to_string());
            }

            stats.hits += 1;
            self.update_hit_ratio(&mut stats);

            debug!(key = key, "Cache hit");
            Some(entry.ast.clone())
        } else {
            stats.misses += 1;
            self.update_hit_ratio(&mut stats);

            debug!(key = key, "Cache miss");
            None
        }
    }

    /// Put a template into the cache
    #[instrument(skip(self, ast))]
    pub fn put(&self, key: String, ast: TemplateAst, source_hash: u64) -> Result<(), CursedError> {
        debug!(key = key, "Cache put");

        let mut entries = self.entries.write()
            .map_err(|_| CursedError::TemplateError {
                message: "Failed to acquire cache lock".to_string(),
                source_location: None,
            })?;

        // Check if we need to evict entries
        if entries.len() >= self.config.max_entries {
            self.evict_entry(&mut entries)?;
        }

        let entry = CacheEntry {
            ast,
            created_at: Instant::now(),
            last_accessed: Instant::now(),
            access_count: 0,
            source_hash,
            file_modified: None,
        };

        entries.insert(key.clone(), entry);

        // Update LRU order
        if let Ok(mut access_order) = self.access_order.write() {
            access_order.push(key.clone());
        }

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.entries = entries.len();
            stats.memory_usage = self.estimate_memory_usage(&entries);
        }

        debug!(key = key, "Cache put completed");
        Ok(())
    }

    /// Remove a template from the cache
    #[instrument(skip(self))]
    pub fn remove(&self, key: &str) -> Option<TemplateAst> {
        debug!(key = key, "Cache remove");

        let mut entries = self.entries.write().ok()?;
        let result = entries.remove(key).map(|entry| entry.ast);

        // Update LRU order
        if let Ok(mut access_order) = self.access_order.write() {
            access_order.retain(|k| k != key);
        }

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.entries = entries.len();
            stats.memory_usage = self.estimate_memory_usage(&entries);
        }

        result
    }

    /// Clear all entries from the cache
    #[instrument(skip(self))]
    pub fn clear(&self) {
        debug!("Clearing cache");

        if let Ok(mut entries) = self.entries.write() {
            entries.clear();
        }

        if let Ok(mut access_order) = self.access_order.write() {
            access_order.clear();
        }

        if let Ok(mut stats) = self.stats.write() {
            stats.entries = 0;
            stats.memory_usage = 0;
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> (usize, usize) {
        if let Ok(entries) = self.entries.read() {
            (entries.len(), self.config.max_entries)
        } else {
            (0, self.config.max_entries)
        }
    }

    /// Get detailed cache statistics
    pub fn detailed_stats(&self) -> Option<CacheStats> {
        self.stats.read().ok().map(|stats| stats.clone())
    }

    /// Check if cache contains a key
    pub fn contains(&self, key: &str) -> bool {
        self.entries.read()
            .map(|entries| entries.contains_key(key))
            .unwrap_or(false)
    }

    /// Get all cache keys
    pub fn keys(&self) -> Vec<String> {
        self.entries.read()
            .map(|entries| entries.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Validate cache entry (check if source has changed)
    #[instrument(skip(self))]
    pub fn validate_entry(&self, key: &str, current_hash: u64) -> bool {
        if let Ok(entries) = self.entries.read() {
            if let Some(entry) = entries.get(key) {
                return entry.source_hash == current_hash;
            }
        }
        false
    }

    /// Invalidate cache entry
    #[instrument(skip(self))]
    pub fn invalidate(&self, key: &str) {
        debug!(key = key, "Invalidating cache entry");
        self.remove(key);
    }

    /// Evict an entry based on the eviction policy
    fn evict_entry(&self, entries: &mut HashMap<String, CacheEntry>) -> Result<(), CursedError> {
        let key_to_evict = match &self.config.eviction_policy {
            EvictionPolicy::Lru => self.find_lru_key(entries),
            EvictionPolicy::Lfu => self.find_lfu_key(entries),
            EvictionPolicy::Ttl(duration) => self.find_expired_key(entries, *duration),
            EvictionPolicy::Fifo => self.find_fifo_key(),
            EvictionPolicy::Random => self.find_random_key(entries),
        };

        if let Some(key) = key_to_evict {
            debug!(key = key, "Evicting cache entry");
            entries.remove(&key);

            // Update LRU order
            if let Ok(mut access_order) = self.access_order.write() {
                access_order.retain(|k| k != &key);
            }

            // Update statistics
            if let Ok(mut stats) = self.stats.write() {
                stats.evictions += 1;
            }
        }

        Ok(())
    }

    fn find_lru_key(&self, entries: &HashMap<String, CacheEntry>) -> Option<String> {
        entries.iter()
            .min_by_key(|(_, entry)| entry.last_accessed)
            .map(|(key, _)| key.clone())
    }

    fn find_lfu_key(&self, entries: &HashMap<String, CacheEntry>) -> Option<String> {
        entries.iter()
            .min_by_key(|(_, entry)| entry.access_count)
            .map(|(key, _)| key.clone())
    }

    fn find_expired_key(&self, entries: &HashMap<String, CacheEntry>, ttl: Duration) -> Option<String> {
        let now = Instant::now();
        entries.iter()
            .find(|(_, entry)| now.duration_since(entry.created_at) > ttl)
            .map(|(key, _)| key.clone())
    }

    fn find_fifo_key(&self) -> Option<String> {
        if let Ok(access_order) = self.access_order.read() {
            access_order.first().cloned()
        } else {
            None
        }
    }

    fn find_random_key(&self, entries: &HashMap<String, CacheEntry>) -> Option<String> {
        use rand::seq::IteratorRandom;
        let mut rng = rand::thread_rng();
        entries.keys().choose(&mut rng).cloned()
    }

    fn update_hit_ratio(&self, stats: &mut CacheStats) {
        let total = stats.hits + stats.misses;
        stats.hit_ratio = if total > 0 {
            stats.hits as f64 / total as f64
        } else {
            0.0
        };
    }

    fn estimate_memory_usage(&self, entries: &HashMap<String, CacheEntry>) -> usize {
        // Rough estimate of memory usage
        entries.iter()
            .map(|(key, entry)| {
                key.len() + self.estimate_ast_size(&entry.ast) + std::mem::size_of::<CacheEntry>()
            })
            .sum()
    }

    fn estimate_ast_size(&self, ast: &TemplateAst) -> usize {
        // Rough estimate of AST memory usage
        ast.nodes.len() * 100 // Simplified estimation
    }

    /// Cleanup expired entries
    #[instrument(skip(self))]
    pub fn cleanup_expired(&self) {
        if let EvictionPolicy::Ttl(ttl) = &self.config.eviction_policy {
            let now = Instant::now();
            let mut expired_keys = Vec::new();

            if let Ok(entries) = self.entries.read() {
                for (key, entry) in entries.iter() {
                    if now.duration_since(entry.created_at) > *ttl {
                        expired_keys.push(key.clone());
                    }
                }
            }

            for key in expired_keys {
                self.remove(&key);
            }
        }
    }

    /// Start background cleanup task
    pub fn start_cleanup_task(&self) -> tokio::task::JoinHandle<()> {
        let cache = TemplateCache {
            entries: Arc::clone(&self.entries),
            config: self.config.clone(),
            stats: Arc::clone(&self.stats),
            access_order: Arc::clone(&self.access_order),
        };

        tokio::task::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                cache.cleanup_expired();
            }
        })
    }
}

impl Default for TemplateCache {
    fn default() -> Self {
        Self::new(1000)
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

    #[test]
    fn test_cache_put_and_get() {
        let cache = TemplateCache::new(10);
        let ast = create_test_ast();
        let source_hash = CacheKeyGenerator::hash_source("test template");

        cache.put("test".to_string(), ast.clone(), source_hash).unwrap();
        
        let retrieved = cache.get("test");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().nodes.len(), ast.nodes.len());
    }

    #[test]
    fn test_cache_miss() {
        let cache = TemplateCache::new(10);
        
        let result = cache.get("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_cache_remove() {
        let cache = TemplateCache::new(10);
        let ast = create_test_ast();
        let source_hash = CacheKeyGenerator::hash_source("test template");

        cache.put("test".to_string(), ast, source_hash).unwrap();
        assert!(cache.contains("test"));
        
        cache.remove("test");
        assert!(!cache.contains("test"));
    }

    #[test]
    fn test_cache_clear() {
        let cache = TemplateCache::new(10);
        let ast = create_test_ast();
        let source_hash = CacheKeyGenerator::hash_source("test template");

        cache.put("test1".to_string(), ast.clone(), source_hash).unwrap();
        cache.put("test2".to_string(), ast, source_hash).unwrap();
        
        let (entries, _) = cache.stats();
        assert_eq!(entries, 2);
        
        cache.clear();
        
        let (entries, _) = cache.stats();
        assert_eq!(entries, 0);
    }

    #[test]
    fn test_cache_eviction() {
        let cache = TemplateCache::new(2); // Small cache for testing eviction
        let ast = create_test_ast();
        let source_hash = CacheKeyGenerator::hash_source("test template");

        cache.put("test1".to_string(), ast.clone(), source_hash).unwrap();
        cache.put("test2".to_string(), ast.clone(), source_hash).unwrap();
        cache.put("test3".to_string(), ast, source_hash).unwrap(); // Should trigger eviction

        let (entries, _) = cache.stats();
        assert_eq!(entries, 2);
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

    #[test]
    fn test_cache_validation() {
        let cache = TemplateCache::new(10);
        let ast = create_test_ast();
        let source_hash = CacheKeyGenerator::hash_source("test template");

        cache.put("test".to_string(), ast, source_hash).unwrap();
        
        // Valid hash should return true
        assert!(cache.validate_entry("test", source_hash));
        
        // Different hash should return false
        let different_hash = CacheKeyGenerator::hash_source("different template");
        assert!(!cache.validate_entry("test", different_hash));
        
        // Non-existent key should return false
        assert!(!cache.validate_entry("nonexistent", source_hash));
    }

    #[test]
    fn test_cache_statistics() {
        let cache = TemplateCache::new(10);
        let ast = create_test_ast();
        let source_hash = CacheKeyGenerator::hash_source("test template");

        cache.put("test".to_string(), ast, source_hash).unwrap();
        
        // Miss
        cache.get("nonexistent");
        
        // Hit
        cache.get("test");
        
        let stats = cache.detailed_stats().unwrap();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.entries, 1);
        assert!(stats.hit_ratio > 0.0);
    }
}
