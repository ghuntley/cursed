/// Static file serving with caching utilities
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use crate::config::StaticFileConfig;

/// Static file server with caching
pub struct StaticFileServer {
    config: StaticFileConfig,
    cache: HashMap<String, CachedFile>,
    max_cache_size: usize,
}

/// Cached file entry
#[derive(Debug, Clone)]
pub struct CachedFile {
    pub content: Vec<u8>,
    pub content_type: String,
    pub etag: String,
    pub last_modified: SystemTime,
    pub cached_at: SystemTime,
}

/// Static file cache manager
pub struct StaticFileCache {
    entries: HashMap<String, CachedFile>,
    max_size: usize,
    total_size: usize,
}

impl StaticFileServer {
    pub fn new(config: StaticFileConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
            max_cache_size: 100 * 1024 * 1024, // 100MB cache
        }
    }

    pub fn serve_file(&mut self, path: &str) -> Result<StaticFileResponse, StaticFileError> {
        // Check if file extension is allowed
        if !self.is_allowed_extension(path) {
            return Err(StaticFileError::NotAllowed);
        }

        // Check cache first
        if let Some(cached) = self.cache.get(path) {
            if !self.is_cache_expired(cached) {
                return Ok(StaticFileResponse {
                    content: cached.content.clone(),
                    content_type: cached.content_type.clone(),
                    etag: Some(cached.etag.clone()),
                    last_modified: Some(cached.last_modified),
                    cache_control: Some(format!("max-age={}", self.config.cache_max_age.as_secs())),
                });
            }
        }

        // Simulate file serving
        let content = format!("Content of file: {}", path).into_bytes();
        let content_type = self.get_content_type(path);
        let etag = format!("\"{}\"", self.calculate_etag(&content));
        let last_modified = SystemTime::now();

        // Cache the file
        if self.config.enable_caching {
            self.cache_file(path, &content, &content_type, &etag, last_modified);
        }

        Ok(StaticFileResponse {
            content,
            content_type,
            etag: Some(etag),
            last_modified: Some(last_modified),
            cache_control: Some(format!("max-age={}", self.config.cache_max_age.as_secs())),
        })
    }

    fn is_allowed_extension(&self, path: &str) -> bool {
        if let Some(ext) = std::path::Path::new(path).extension() {
            if let Some(ext_str) = ext.to_str() {
                let ext_with_dot = format!(".{}", ext_str);
                return self.config.allowed_extensions.contains(&ext_with_dot);
            }
        }
        false
    }

    fn get_content_type(&self, path: &str) -> String {
        match std::path::Path::new(path).extension().and_then(|ext| ext.to_str()) {
            Some("html") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("json") => "application/json",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            _ => "application/octet-stream",
        }.to_string()
    }

    fn calculate_etag(&self, content: &[u8]) -> String {
        // Simple hash for ETag
        let mut hash: u64 = 5381;
        for &byte in content {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
        }
        format!("{:x}", hash)
    }

    fn cache_file(&mut self, path: &str, content: &[u8], content_type: &str, etag: &str, last_modified: SystemTime) {
        let cached_file = CachedFile {
            content: content.to_vec(),
            content_type: content_type.to_string(),
            etag: etag.to_string(),
            last_modified,
            cached_at: SystemTime::now(),
        };

        self.cache.insert(path.to_string(), cached_file);
    }

    fn is_cache_expired(&self, cached: &CachedFile) -> bool {
        let elapsed = SystemTime::now()
            .duration_since(cached.cached_at)
            .unwrap_or_default();
        elapsed > self.config.cache_max_age
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn get_cache_stats(&self) -> CacheStats {
        let total_size: usize = self.cache.values()
            .map(|file| file.content.len())
            .sum();

        CacheStats {
            entries: self.cache.len(),
            total_size,
            max_size: self.max_cache_size,
        }
    }
}

impl StaticFileCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_size,
            total_size: 0,
        }
    }

    pub fn get(&self, key: &str) -> Option<&CachedFile> {
        self.entries.get(key)
    }

    pub fn put(&mut self, key: String, file: CachedFile) {
        self.entries.insert(key, file);
    }

    pub fn remove(&mut self, key: &str) -> Option<CachedFile> {
        self.entries.remove(key)
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.total_size = 0;
    }
}

/// Static file response
#[derive(Debug)]
pub struct StaticFileResponse {
    pub content: Vec<u8>,
    pub content_type: String,
    pub etag: Option<String>,
    pub last_modified: Option<SystemTime>,
    pub cache_control: Option<String>,
}

impl StaticFileResponse {
    pub fn get_headers(&self) -> Vec<(String, String)> {
        let mut headers = vec![
            ("Content-Type".to_string(), self.content_type.clone()),
            ("Content-Length".to_string(), self.content.len().to_string()),
        ];

        if let Some(etag) = &self.etag {
            headers.push(("ETag".to_string(), etag.clone()));
        }

        if let Some(cache_control) = &self.cache_control {
            headers.push(("Cache-Control".to_string(), cache_control.clone()));
        }

        headers
    }
}

/// Cache statistics
#[derive(Debug)]
pub struct CacheStats {
    pub entries: usize,
    pub total_size: usize,
    pub max_size: usize,
}

/// Static file errors
#[derive(Debug)]
pub enum StaticFileError {
    NotFound,
    NotAllowed,
    IoError(String),
    CacheError(String),
}

impl std::fmt::Display for StaticFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StaticFileError::NotFound => write!(f, "File not found"),
            StaticFileError::NotAllowed => write!(f, "File type not allowed"),
            StaticFileError::IoError(msg) => write!(f, "IO error: {}", msg),
            StaticFileError::CacheError(msg) => write!(f, "Cache error: {}", msg),
        }
    }
}

impl std::error::Error for StaticFileError {}
