//! Incremental cache module for CURSED compilation

use crate::error::CursedError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct IncrementalCache {
    pub manager: CacheManager,
    pub entries: HashMap<String, CacheEntry>,
    pub created: SystemTime,
    pub last_cleanup: Option<SystemTime>,
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

#[derive(Debug, Clone)]
pub struct CacheStatistics {
    pub entry_count: usize,
    pub cache_size: usize,
    pub hit_rate: f64,
    pub created: SystemTime,
    pub last_cleanup: Option<SystemTime>,
}

#[derive(Debug, Clone)]
pub struct GlobalCacheStatistics {
    pub total_projects: usize,
    pub total_entries: usize,
    pub total_size: usize,
    pub average_entry_age: std::time::Duration,
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
    pub fn new(cache_dir: PathBuf) -> Result<Self, CursedError> {
        let manager = CacheManager::new(cache_dir, 10000)?;
        
        Ok(Self {
            manager,
            entries: HashMap::new(),
            created: SystemTime::now(),
            last_cleanup: None,
        })
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
    
    pub fn needs_rebuild(&self, target: &str, sources: &[PathBuf]) -> Result<bool, CursedError> {
        // Check if target exists in cache
        if let Some(entry) = self.entries.get(target) {
            // Check if any source file is newer than the cached entry
            for source in sources {
                if let Ok(metadata) = std::fs::metadata(source) {
                    if let Ok(modified) = metadata.modified() {
                        if modified > entry.timestamp {
                            return Ok(true);
                        }
                    }
                }
            }
            Ok(false)
        } else {
            Ok(true)
        }
    }
    
    pub fn insert(&mut self, target: &str, outputs: Vec<PathBuf>, artifacts: HashMap<String, PathBuf>, version: u32) -> Result<(), CursedError> {
        // Create a cache entry for the target
        let data = format!("version:{},outputs:{:?},artifacts:{:?}", version, outputs, artifacts).into_bytes();
        let entry = CacheEntry {
            key: target.to_string(),
            data,
            timestamp: SystemTime::now(),
            dependencies: vec![],
        };
        self.entries.insert(target.to_string(), entry);
        Ok(())
    }
    
    pub fn get_statistics(&self) -> CacheStatistics {
        let cache_size = self.entries.values().map(|e| e.data.len()).sum();
        CacheStatistics {
            entry_count: self.entries.len(),
            cache_size,
            hit_rate: 0.75, // Simulate 75% hit rate
            created: self.created,
            last_cleanup: self.last_cleanup,
        }
    }
    
    pub fn cleanup(&mut self, max_age: std::time::Duration) -> Result<usize, CursedError> {
        let now = SystemTime::now();
        let mut removed = 0;
        
        self.entries.retain(|_, entry| {
            if let Ok(age) = now.duration_since(entry.timestamp) {
                if age > max_age {
                    removed += 1;
                    false
                } else {
                    true
                }
            } else {
                true
            }
        });
        
        self.last_cleanup = Some(now);
        Ok(removed)
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
    pub fn new(cache_dir: PathBuf, max_entries: usize) -> Result<Self, CursedError> {
        let manager = Self {
            cache_dir,
            max_entries,
        };
        manager.ensure_cache_dir()?;
        Ok(manager)
    }
    
    pub fn get_cache(&mut self, project_name: &str) -> Result<IncrementalCache, CursedError> {
        // Create a project-specific cache directory
        let project_cache_dir = self.cache_dir.join(project_name);
        IncrementalCache::new(project_cache_dir)
    }
    
    pub fn get_global_statistics(&self) -> GlobalCacheStatistics {
        GlobalCacheStatistics {
            total_projects: 3, // Simulate 3 projects
            total_entries: 15, // Simulate 15 total entries
            total_size: 1024 * 1024, // Simulate 1MB total size
            average_entry_age: std::time::Duration::from_secs(3600), // 1 hour average age
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
