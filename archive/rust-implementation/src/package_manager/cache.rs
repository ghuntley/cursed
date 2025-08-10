//! Package cache management for CURSED
//!
//! This module handles caching of downloaded packages and metadata

use crate::error::{CursedError, Result};
use crate::package_manager::version::Version;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub cache_dir: PathBuf,
    pub max_size: u64,
    pub max_age: Duration,
    pub cleanup_interval: Duration,
}

/// Cached package information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedPackage {
    pub name: String,
    pub version: Version,
    pub download_url: String,
    pub local_path: PathBuf,
    pub checksum: String,
    pub cached_at: SystemTime,
    pub access_count: u32,
    pub last_accessed: SystemTime,
    pub file_size: u64,
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_packages: usize,
    pub total_size: u64,
    pub hit_rate: f64,
    pub cache_dir: PathBuf,
}

/// Package cache manager
#[derive(Debug)]
pub struct PackageCache {
    config: CacheConfig,
    cache_index: HashMap<String, CachedPackage>,
    cache_hits: u64,
    cache_misses: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            cache_dir: PathBuf::from("target/package_cache"),
            max_size: 1024 * 1024 * 1024, // 1GB
            max_age: Duration::from_secs(7 * 24 * 60 * 60), // 1 week
            cleanup_interval: Duration::from_secs(24 * 60 * 60), // 1 day
        }
    }
}

impl PackageCache {
    /// Create a new package cache
    pub fn new(config: CacheConfig) -> Result<Self> {
        // Ensure cache directory exists
        fs::create_dir_all(&config.cache_dir)?;

        let mut cache = Self {
            config,
            cache_index: HashMap::new(),
            cache_hits: 0,
            cache_misses: 0,
        };

        // Load existing cache index
        cache.load_cache_index()?;

        Ok(cache)
    }

    /// Check if a package is cached
    pub fn is_cached(&mut self, name: &str, version: &Version) -> bool {
        let key = self.make_cache_key(name, version);
        
        if let Some(cached) = self.cache_index.get_mut(&key) {
            // Update access statistics
            cached.access_count += 1;
            cached.last_accessed = SystemTime::now();
            
            // Check if cached file still exists
            if cached.local_path.exists() {
                self.cache_hits += 1;
                return true;
            } else {
                // Remove stale cache entry
                self.cache_index.remove(&key);
            }
        }
        
        self.cache_misses += 1;
        false
    }

    /// Get cached package path
    pub fn get_cached_path(&self, name: &str, version: &Version) -> Option<PathBuf> {
        let key = self.make_cache_key(name, version);
        self.cache_index.get(&key).map(|cached| cached.local_path.clone())
    }

    /// Cache a downloaded package
    pub fn cache_package(&mut self, name: &str, version: &Version, data: &[u8], 
                        download_url: String, checksum: String) -> Result<PathBuf> {
        let now = SystemTime::now();
        let cache_path = self.get_package_cache_path(name, version);
        
        // Ensure parent directory exists
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write package data to cache
        fs::write(&cache_path, data)?;

        // Create cache entry
        let cached_package = CachedPackage {
            name: name.to_string(),
            version: version.clone(),
            download_url,
            local_path: cache_path.clone(),
            checksum,
            cached_at: now,
            access_count: 1,
            last_accessed: now,
            file_size: data.len() as u64,
        };

        let key = self.make_cache_key(name, version);
        self.cache_index.insert(key, cached_package);

        // Save updated cache index
        self.save_cache_index()?;

        // Cleanup old entries if needed
        self.cleanup_if_needed()?;

        Ok(cache_path)
    }

    /// Remove a package from cache
    pub fn remove_package(&mut self, name: &str, version: &Version) -> Result<()> {
        let key = self.make_cache_key(name, version);
        
        if let Some(cached) = self.cache_index.remove(&key) {
            // Remove the cached file
            if cached.local_path.exists() {
                fs::remove_file(&cached.local_path)?;
            }
            
            // Remove empty parent directories
            if let Some(parent) = cached.local_path.parent() {
                let _ = fs::remove_dir(parent); // Ignore errors for non-empty dirs
            }
            
            self.save_cache_index()?;
            tracing::info!("Removed {} {} from cache", name, version);
        }

        Ok(())
    }

    /// Clear entire cache
    pub fn clear_cache(&mut self) -> Result<()> {
        tracing::info!("Clearing package cache");
        
        // Remove all cached files
        for cached in self.cache_index.values() {
            if cached.local_path.exists() {
                let _ = fs::remove_file(&cached.local_path);
            }
        }

        // Clear the cache directory
        if self.config.cache_dir.exists() {
            fs::remove_dir_all(&self.config.cache_dir)?;
            fs::create_dir_all(&self.config.cache_dir)?;
        }

        // Clear index
        self.cache_index.clear();
        self.save_cache_index()?;

        Ok(())
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        let total_size = self.cache_index.values()
            .map(|cached| cached.file_size)
            .sum();

        let hit_rate = if self.cache_hits + self.cache_misses > 0 {
            self.cache_hits as f64 / (self.cache_hits + self.cache_misses) as f64
        } else {
            0.0
        };

        CacheStats {
            total_packages: self.cache_index.len(),
            total_size,
            hit_rate,
            cache_dir: self.config.cache_dir.clone(),
        }
    }

    /// List all cached packages
    pub fn list_cached_packages(&self) -> Vec<&CachedPackage> {
        self.cache_index.values().collect()
    }

    /// Cleanup old and large cache entries
    pub fn cleanup_if_needed(&mut self) -> Result<()> {
        let stats = self.get_stats();
        
        // Check if cache is too large
        if stats.total_size > self.config.max_size {
            self.cleanup_by_size()?;
        }

        // Check for old entries
        self.cleanup_by_age()?;

        Ok(())
    }

    /// Cleanup cache by size (LRU eviction)
    fn cleanup_by_size(&mut self) -> Result<()> {
        let target_size = (self.config.max_size as f64 * 0.8) as u64; // Clean to 80% of max
        let mut current_size = self.get_stats().total_size;

        if current_size <= target_size {
            return Ok(());
        }

        tracing::info!("Cache size cleanup: {} -> {}", current_size, target_size);

        // Sort by last accessed time (oldest first)
        let mut entries: Vec<_> = self.cache_index.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        entries.sort_by_key(|(_, cached)| cached.last_accessed);

        // Remove oldest entries until we reach target size
        for (key, cached) in entries {
            if current_size <= target_size {
                break;
            }

            if cached.local_path.exists() {
                fs::remove_file(&cached.local_path)?;
            }
            
            current_size -= cached.file_size;
            self.cache_index.remove(&key);
            
            tracing::debug!("Evicted {} {} from cache", cached.name, cached.version);
        }

        self.save_cache_index()?;
        Ok(())
    }

    /// Cleanup cache by age
    fn cleanup_by_age(&mut self) -> Result<()> {
        let now = SystemTime::now();
        let max_age = self.config.max_age;

        let mut to_remove = Vec::new();

        for (key, cached) in &self.cache_index {
            if let Ok(age) = now.duration_since(cached.cached_at) {
                if age > max_age {
                    to_remove.push(key.clone());
                }
            }
        }

        for key in to_remove {
            if let Some(cached) = self.cache_index.remove(&key) {
                if cached.local_path.exists() {
                    fs::remove_file(&cached.local_path)?;
                }
                tracing::debug!("Removed expired cache entry: {} {}", cached.name, cached.version);
            }
        }

        if !self.cache_index.is_empty() {
            self.save_cache_index()?;
        }

        Ok(())
    }

    /// Generate cache key for a package
    fn make_cache_key(&self, name: &str, version: &Version) -> String {
        format!("{}@{}", name, version)
    }

    /// Get the file path for a cached package
    fn get_package_cache_path(&self, name: &str, version: &Version) -> PathBuf {
        self.config.cache_dir
            .join(name)
            .join(format!("{}.tar.gz", version))
    }

    /// Get the cache index file path
    fn get_cache_index_path(&self) -> PathBuf {
        self.config.cache_dir.join("cache_index.json")
    }

    /// Load cache index from disk
    fn load_cache_index(&mut self) -> Result<()> {
        let index_path = self.get_cache_index_path();
        
        if !index_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&index_path)?;
        self.cache_index = serde_json::from_str(&content)
            .map_err(|e| CursedError::General(format!("Failed to parse cache index: {}", e)))?;

        tracing::debug!("Loaded cache index with {} entries", self.cache_index.len());
        Ok(())
    }

    /// Save cache index to disk
    fn save_cache_index(&self) -> Result<()> {
        let index_path = self.get_cache_index_path();
        let content = serde_json::to_string_pretty(&self.cache_index)
            .map_err(|e| CursedError::General(format!("Failed to serialize cache index: {}", e)))?;

        fs::write(&index_path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cache_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = CacheConfig {
            cache_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let cache = PackageCache::new(config).unwrap();
        assert_eq!(cache.cache_index.len(), 0);
    }

    #[test]
    fn test_package_caching() {
        let temp_dir = TempDir::new().unwrap();
        let config = CacheConfig {
            cache_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let mut cache = PackageCache::new(config).unwrap();
        let version = Version::new(1, 0, 0);
        let data = b"test package data";
        
        let cached_path = cache.cache_package(
            "test-package", 
            &version, 
            data, 
            "http://example.com/test.tar.gz".to_string(),
            "sha256:test".to_string()
        ).unwrap();

        assert!(cached_path.exists());
        assert!(cache.is_cached("test-package", &version));
    }
}
