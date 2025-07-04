//! Advanced cache module for CURSED compilation

use crate::error::CursedError;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct AdvancedCache {
    pub config: AdvancedCacheConfig,
    pub data: HashMap<String, CacheData>,
}

#[derive(Debug, Clone)]
pub struct AdvancedCacheConfig {
    pub max_size: usize,
    pub ttl: std::time::Duration,
    pub compression_enabled: bool,
    pub cache_directory: std::path::PathBuf,
    pub enable_ast_cache: bool,
    pub enable_ir_cache: bool,
    pub enable_object_cache: bool,
    pub enable_distributed_cache: bool,
    pub distributed_nodes: Vec<String>,
    pub cache_warming_enabled: bool,
    pub precomputation_enabled: bool,
    pub max_cache_size_mb: usize,
    pub max_entry_age_hours: u64,
}

#[derive(Debug, Clone)]
pub struct CacheData {
    pub content: Vec<u8>,
    pub metadata: CacheMetadata,
}

#[derive(Debug, Clone)]
pub struct CacheMetadata {
    pub created_at: SystemTime,
    pub size: usize,
    pub hash: String,
}

impl Default for AdvancedCacheConfig {
    fn default() -> Self {
        Self {
            max_size: 100_000_000, // 100MB
            ttl: std::time::Duration::from_secs(3600), // 1 hour
            compression_enabled: true,
            cache_directory: std::path::PathBuf::from("./cache"),
            enable_ast_cache: true,
            enable_ir_cache: true,
            enable_object_cache: true,
            enable_distributed_cache: false,
            distributed_nodes: Vec::new(),
            cache_warming_enabled: false,
            precomputation_enabled: false,
            max_cache_size_mb: 100,
            max_entry_age_hours: 24,
        }
    }
}

impl AdvancedCache {
    pub fn new(config: AdvancedCacheConfig) -> Self {
        Self {
            config,
            data: HashMap::new(),
        }
    }
    
    pub fn get(&self, key: &str) -> Option<&CacheData> {
        self.data.get(key)
    }
    
    pub fn set(&mut self, key: String, data: CacheData) -> Result<(), CursedError> {
        self.data.insert(key, data);
        Ok(())
    }
    
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl CacheMetadata {
    pub fn new(content: &[u8]) -> Self {
        Self {
            created_at: SystemTime::now(),
            size: content.len(),
            hash: format!("{:x}", content.iter().fold(0u64, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u64))),
        }
    }
}
