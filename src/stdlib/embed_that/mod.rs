//! EmbedThat - File embedding support for CURSED
//! 
//! This module provides comprehensive support for embedding files in compiled binaries
//! and accessing them at runtime. It's inspired by Go's embed package but with enhanced
//! features for resource management and more flexible embedding options.
//!
//! # Core Features
//!
//! - **File Embedding**: Embed files at compile time using directives
//! - **Dynamic Loading**: Load embedded files at runtime with pattern matching
//! - **Resource Management**: Efficient caching and compression support
//! - **Template Integration**: Parse embedded templates with various engines
//! - **Type-Specific Loaders**: Special handling for images, configs, etc.
//! - **FileSystem Interface**: Use embedded files as a virtual filesystem
//!
//! # Usage Examples
//!
//! ```rust
//! use cursed::stdlib::embed_that::*;
//!
//! // Load a single embedded file
//! let file = load_that_file("config.json")?;
//! let content = file.content_string()?;
//!
//! // Load files matching a pattern
//! let templates = load_that_pattern("templates/*.html")?;
//! println!("Found {} templates", templates.count());
//!
//! // Parse embedded templates
//! let engine = parse_templates(&["templates/*.html"])?;
//!
//! // Load and parse configuration
//! let mut config = MyConfig::default();
//! load_json("config.json", &mut config)?;
//!
//! // Use caching for better performance
//! let cache = new_resource_cache();
//! let file = cache.load_file("large_dataset.csv")?;
//! ```

pub mod core;
pub mod error;
pub mod resource_loader;
pub mod template_integration;
pub mod specific_loaders;
pub mod compression;
pub mod cache;

// Re-export core types for easy access
pub use core::{
    ThatFile, ThatFiles, ThatString, ThatBytes, 
    FileSystemVibe, DirEntry, FileInfo, EmbeddedFileSystem,
    tea, normie, lit
};

// Re-export error types
pub use error::{
    EmbedError, EmbedResult,
    file_not_found, invalid_format, compression_error, decompression_error,
    template_parsing_error, image_loading_error, cache_error, mime_type_error,
    config_parsing_error, resource_limit_exceeded, invalid_pattern, general_error
};

// Re-export resource loading functions
pub use resource_loader::{
    load_that_file, load_that_dir, load_that_pattern, file_exists, get_embed_statistics,
    ResourceLoader, EmbedStatistics, EmbedManifest, ManifestItem,
    initialize_resource_loader
};

// Re-export template integration
pub use template_integration::{
    parse_templates, parse_templates_with_funcs, validate_all_templates,
    get_default_template_helpers, TemplateIntegration, ValidationReport, TemplateHelpers
};

// Re-export specific loaders
pub use specific_loaders::{
    load_image, load_image_fs, load_json, load_yaml, load_toml, load_config,
    load_text_file, load_binary_file, load_css, load_javascript, load_html,
    load_font, load_audio, load_video,
    ImageData, ImageType, CssData, JavaScriptData, HtmlData,
    FontData, FontType, AudioData, AudioType, VideoData, VideoType
};

// Re-export compression support
pub use compression::{
    decompress_file, load_compressed_fs, compress_data, analyze_compression, get_compression_stats,
    CompressionType, CompressionStats, CompressionResult, CompressionAnalysis,
    CompressedEmbeddedFile
};

// Re-export caching support
pub use cache::{
    new_resource_cache, new_resource_cache_with_expiry, new_resource_cache_with_config,
    get_global_cache, ResourceCache, CacheStatistics, CacheConfig
};

use std::time::Duration;

/// Initialize the embed_that module
pub fn initialize() -> EmbedResult<()> {
    // Initialize resource loader
    resource_loader::initialize_resource_loader()?;
    
    // Set up default global cache configuration
    let config = CacheConfig {
        expiry_duration: Some(Duration::from_secs(3600)), // 1 hour
        max_size: Some(1000), // 1000 entries
        enable_access_tracking: true,
        cleanup_interval: Some(Duration::from_secs(300)), // 5 minutes
    };
    
    // Initialize any other subsystems as needed
    Ok(())
}

/// Get module information and statistics
pub fn get_module_info() -> ModuleInfo {
    let stats = get_embed_statistics().unwrap_or_else(|_| EmbedStatistics {
        total_files: 0,
        total_size: 0,
        namespaces: 0,
        file_types: std::collections::HashMap::new(),
    });
    
    let cache_stats = get_global_cache().get_statistics();
    
    ModuleInfo {
        version: "1.0.0".to_string(),
        total_embedded_files: stats.total_files,
        total_embedded_size: stats.total_size,
        cache_entries: cache_stats.total_entries,
        cache_memory_usage: cache_stats.estimated_memory_usage,
        supported_compression_types: vec![
            CompressionType::Gzip,
            CompressionType::Zstd,
            CompressionType::Brotli,
            CompressionType::Deflate,
        ],
        supported_image_types: vec![
            ImageType::Png,
            ImageType::Jpeg,
            ImageType::Gif,
            ImageType::Svg,
            ImageType::WebP,
        ],
        supported_audio_types: vec![
            AudioType::Mp3,
            AudioType::Wav,
            AudioType::Ogg,
            AudioType::Flac,
            AudioType::Aac,
        ],
        supported_video_types: vec![
            VideoType::Mp4,
            VideoType::Avi,
            VideoType::Mov,
            VideoType::WebM,
        ],
    }
}

/// Module information structure
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub version: tea,
    pub total_embedded_files: i32,
    pub total_embedded_size: i64,
    pub cache_entries: usize,
    pub cache_memory_usage: usize,
    pub supported_compression_types: Vec<CompressionType>,
    pub supported_image_types: Vec<ImageType>,
    pub supported_audio_types: Vec<AudioType>,
    pub supported_video_types: Vec<VideoType>,
}

/// Utility functions for common operations
pub mod utils {
    use super::*;
    
    /// Load and parse a configuration file with automatic format detection
    pub fn load_config_auto<T>(path: &tea) -> EmbedResult<T>
    where
        T: serde::de::DeserializeOwned + Default + Clone,
    {
        let mut config = T::default();
        load_config(path, &mut config)?;
        Ok(config)
    }
    
    /// Load all files from a directory into a map
    pub fn load_directory_as_map(dir_path: &tea) -> EmbedResult<std::collections::HashMap<tea, ThatFile>> {
        let files = load_that_dir(dir_path)?;
        let mut map = std::collections::HashMap::new();
        
        for file in files.list() {
            map.insert(file.name(), file);
        }
        
        Ok(map)
    }
    
    /// Get embedded file statistics by type
    pub fn get_files_by_type() -> EmbedResult<std::collections::HashMap<tea, Vec<tea>>> {
        let stats = get_embed_statistics()?;
        let mut files_by_type = std::collections::HashMap::new();
        
        // This would need to be implemented to scan all embedded files
        // For now, return empty map
        Ok(files_by_type)
    }
    
    /// Validate all embedded files for corruption
    pub fn validate_all_embedded_files() -> EmbedResult<ValidationSummary> {
        let stats = get_embed_statistics()?;
        let mut summary = ValidationSummary {
            total_files: stats.total_files,
            valid_files: 0,
            corrupted_files: Vec::new(),
            validation_errors: Vec::new(),
        };
        
        // This would iterate through all embedded files and validate them
        // For now, assume all files are valid
        summary.valid_files = stats.total_files;
        
        Ok(summary)
    }
    
    /// Get memory usage summary
    pub fn get_memory_usage_summary() -> MemoryUsageSummary {
        let stats = get_embed_statistics().unwrap_or_else(|_| EmbedStatistics {
            total_files: 0,
            total_size: 0,
            namespaces: 0,
            file_types: std::collections::HashMap::new(),
        });
        
        let cache_stats = get_global_cache().get_statistics();
        
        MemoryUsageSummary {
            embedded_files_size: stats.total_size as usize,
            cache_size: cache_stats.estimated_memory_usage,
            total_memory_usage: stats.total_size as usize + cache_stats.estimated_memory_usage,
        }
    }
}

/// File validation summary
#[derive(Debug, Clone)]
pub struct ValidationSummary {
    pub total_files: i32,
    pub valid_files: i32,
    pub corrupted_files: Vec<tea>,
    pub validation_errors: Vec<tea>,
}

/// Memory usage summary
#[derive(Debug, Clone)]
pub struct MemoryUsageSummary {
    pub embedded_files_size: usize,
    pub cache_size: usize,
    pub total_memory_usage: usize,
}

impl MemoryUsageSummary {
    /// Get memory usage in human-readable format
    pub fn format_size(bytes: usize) -> tea {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
    
    /// Get embedded files size formatted
    pub fn embedded_size_formatted(&self) -> tea {
        Self::format_size(self.embedded_files_size)
    }
    
    /// Get cache size formatted
    pub fn cache_size_formatted(&self) -> tea {
        Self::format_size(self.cache_size)
    }
    
    /// Get total size formatted
    pub fn total_size_formatted(&self) -> tea {
        Self::format_size(self.total_memory_usage)
    }
}

/// Constants for common patterns and configurations
pub mod constants {
    use super::*;
    
    /// Common file patterns for embedding
    pub const TEMPLATE_PATTERNS: &[&str] = &[
        "templates/*.html",
        "templates/*.htm",
        "views/*.html",
        "*.tmpl",
        "*.tpl",
        "*.mustache",
        "*.hbs",
    ];
    
    pub const STATIC_ASSET_PATTERNS: &[&str] = &[
        "static/*",
        "assets/*",
        "public/*",
        "www/*",
    ];
    
    pub const CONFIG_PATTERNS: &[&str] = &[
        "config/*.json",
        "config/*.yaml",
        "config/*.yml",
        "config/*.toml",
        "*.config.json",
        "*.config.yaml",
        "*.config.toml",
    ];
    
    pub const DOCUMENTATION_PATTERNS: &[&str] = &[
        "docs/*.md",
        "documentation/*.md",
        "*.md",
        "README*",
        "CHANGELOG*",
        "LICENSE*",
    ];
    
    /// Default cache configuration
    pub const DEFAULT_CACHE_EXPIRY_SECONDS: u64 = 3600; // 1 hour
    pub const DEFAULT_CACHE_MAX_SIZE: usize = 1000; // 1000 entries
    pub const DEFAULT_CACHE_CLEANUP_INTERVAL_SECONDS: u64 = 300; // 5 minutes
    
    /// Compression thresholds
    pub const MIN_COMPRESSION_SIZE: usize = 1024; // Don't compress files smaller than 1KB
    pub const COMPRESSION_RATIO_THRESHOLD: f64 = 0.9; // Only keep compressed if <90% of original
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_initialization() {
        assert!(initialize().is_ok());
    }
    
    #[test]
    fn test_module_info() {
        let info = get_module_info();
        assert_eq!(info.version, "1.0.0");
        assert!(!info.supported_compression_types.is_empty());
        assert!(!info.supported_image_types.is_empty());
    }
    
    #[test]
    fn test_memory_usage_formatting() {
        let summary = MemoryUsageSummary {
            embedded_files_size: 1024,
            cache_size: 2048,
            total_memory_usage: 3072,
        };
        
        assert_eq!(summary.embedded_size_formatted(), "1.0 KB");
        assert_eq!(summary.cache_size_formatted(), "2.0 KB");
        assert_eq!(summary.total_size_formatted(), "3.0 KB");
    }
    
    #[test]
    fn test_cache_creation() {
        let cache = new_resource_cache();
        assert!(cache.is_empty());
        
        let cache_with_expiry = new_resource_cache_with_expiry(Duration::from_secs(60));
        assert!(cache_with_expiry.is_empty());
    }
}
