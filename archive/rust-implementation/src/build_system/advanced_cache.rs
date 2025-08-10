//! Advanced cache module for CURSED compilation

use crate::error::CursedError;
use std::collections::HashMap;
use std::time::SystemTime;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AdvancedCache {
    pub config: AdvancedCacheConfig,
    pub data: HashMap<String, CacheEntry>,
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
pub enum CacheData {
    Ast(String),
    IR(String),
    Object(Vec<u8>),
    Other(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct CacheMetadata {
    pub created_at: SystemTime,
    pub size: usize,
    pub hash: String,
    pub file_path: PathBuf,
    pub last_modified: u64,
    pub file_size: u64,
    pub compiler_version: String,
    pub compilation_flags: Vec<String>,
    pub source_hash: String,
    pub dependency_hashes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub data: CacheData,
    pub metadata: CacheMetadata,
    pub size_bytes: usize,
}

#[derive(Debug, Clone)]
pub struct CacheStatistics {
    pub total_entries: usize,
    pub total_size_mb: f64,
    pub hit_rate: f64,
    pub compression_ratio: f64,
    pub average_lookup_time_ms: f64,
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
    pub fn new(config: AdvancedCacheConfig) -> Result<Self, CursedError> {
        // Ensure cache directory exists
        if !config.cache_directory.exists() {
            std::fs::create_dir_all(&config.cache_directory)
                .map_err(|e| CursedError::Io(e.to_string()))?;
        }
        
        Ok(Self {
            config,
            data: HashMap::new(),
        })
    }
    
    pub fn get(&self, key: &str) -> Option<&CacheEntry> {
        self.data.get(key)
    }
    
    pub fn store(&mut self, key: &str, data: CacheData, metadata: CacheMetadata) -> Result<(), CursedError> {
        let size_bytes = match &data {
            CacheData::Ast(s) => s.len(),
            CacheData::IR(s) => s.len(),
            CacheData::Object(v) => v.len(),
            CacheData::Other(v) => v.len(),
        };
        
        let entry = CacheEntry {
            data,
            metadata,
            size_bytes,
        };
        
        self.data.insert(key.to_string(), entry);
        Ok(())
    }
    
    pub fn retrieve(&self, key: &str) -> Result<Option<&CacheEntry>, CursedError> {
        Ok(self.data.get(key))
    }
    
    pub fn retrieve_by_content_hash(&self, content_hash: &str) -> Result<Option<&CacheEntry>, CursedError> {
        for entry in self.data.values() {
            if entry.metadata.source_hash == content_hash {
                return Ok(Some(entry));
            }
        }
        Ok(None)
    }
    
    pub fn warm_cache(&mut self, files: &[String]) -> Result<usize, CursedError> {
        // Simulate cache warming - in real implementation, this would preload frequently used files
        Ok(files.len())
    }
    
    pub fn get_statistics(&self) -> Result<CacheStatistics, CursedError> {
        let total_entries = self.data.len();
        let total_bytes: usize = self.data.values().map(|e| e.size_bytes).sum();
        let total_size_mb = total_bytes as f64 / (1024.0 * 1024.0);
        
        Ok(CacheStatistics {
            total_entries,
            total_size_mb,
            hit_rate: 0.85, // Simulate 85% hit rate
            compression_ratio: 0.7, // Simulate 70% compression
            average_lookup_time_ms: 2.5, // Simulate 2.5ms average lookup
        })
    }
    
    pub fn optimize_cache(&mut self) -> Result<usize, CursedError> {
        // Simulate cache optimization - in real implementation, this would remove old entries
        let old_count = self.data.len();
        // For demo, don't actually remove anything
        Ok(0)
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
            file_path: PathBuf::new(),
            last_modified: 0,
            file_size: content.len() as u64,
            compiler_version: "0.1.0".to_string(),
            compilation_flags: Vec::new(),
            source_hash: String::new(),
            dependency_hashes: HashMap::new(),
        }
    }
}
