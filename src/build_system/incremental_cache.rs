//! Incremental cache module for CURSED compilation

use crate::error::CursedError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct IncrementalCache {
    pub manager: CacheManager,
    pub entries: HashMap<String, CacheEntry>,
}

#[derive(Debug, Clone)]
pub struct CacheManager {
    pub cache_dir: PathBuf,
    pub max_entries: usize,
}

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: SystemTime,
    pub dependencies: Vec<String>,
}

impl Default for CacheManager {
    fn default() -> Self {
        Self {
            cache_dir: std::env::temp_dir().join("cursed_cache"),
            max_entries: 10000,
        }
    }
}

impl IncrementalCache {
    pub fn new(manager: CacheManager) -> Self {
        Self {
            manager,
            entries: HashMap::new(),
        }
    }
    
    pub fn get(&self, key: &str) -> Option<&CacheEntry> {
        self.entries.get(key)
    }
    
    pub fn set(&mut self, key: String, data: Vec<u8>, dependencies: Vec<String>) -> Result<(), CursedError> {
        let entry = CacheEntry {
            key: key.clone(),
            data,
            timestamp: SystemTime::now(),
            dependencies,
        };
        self.entries.insert(key, entry);
        Ok(())
    }
    
    pub fn invalidate(&mut self, key: &str) -> Result<(), CursedError> {
        self.entries.remove(key);
        Ok(())
    }
    
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}

impl CacheManager {
    pub fn new(cache_dir: PathBuf, max_entries: usize) -> Self {
        Self {
            cache_dir,
            max_entries,
        }
    }
    
    pub fn ensure_cache_dir(&self) -> Result<(), CursedError> {
        if !self.cache_dir.exists() {
            std::fs::create_dir_all(&self.cache_dir)
                .map_err(|e| CursedError::Io(format!("Failed to create cache directory: {}", e)))?;
        }
        Ok(())
    }
}
