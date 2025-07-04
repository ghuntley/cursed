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
