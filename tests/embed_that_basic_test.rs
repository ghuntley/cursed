use cursed::stdlib::embed_that::*;
use std::time::Duration;

#[test]
fn test_embed_that_core_types() {
    // Test ThatFile creation
    let content = b"Hello, EmbedThat!".to_vec();
    let file = ThatFile::new("test.txt".to_string(), content.clone());
    
    assert_eq!(file.name(), "test.txt");
    assert_eq!(file.size(), content.len() as i64);
    assert_eq!(file.content(), content);
    assert!(file.is_text());
    assert!(!file.is_image());
    
    // Test content string conversion
    let content_str = file.content_string().unwrap();
    assert_eq!(content_str, "Hello, EmbedThat!");
    
    // Test hash generation
    let hash = file.hash();
    assert!(!hash.is_empty());
    assert_eq!(hash.len(), 64); // SHA-256 hex length
}

#[test]
fn test_that_files_collection() {
    let mut files = ThatFiles::new();
    assert_eq!(files.count(), 0);
    assert!(files.names().is_empty());
    
    // Add some test files
    let file1 = ThatFile::new("file1.txt".to_string(), b"Content 1".to_vec());
    let file2 = ThatFile::new("file2.json".to_string(), b"{}".to_vec());
    
    files.add_file(file1.clone());
    files.add_file(file2.clone());
    
    assert_eq!(files.count(), 2);
    assert_eq!(files.names().len(), 2);
    assert!(files.names().contains(&"file1.txt".to_string()));
    assert!(files.names().contains(&"file2.json".to_string()));
    
    // Test getting files
    let (retrieved_file, found) = files.get(&"file1.txt".to_string());
    assert!(found);
    assert_eq!(retrieved_file.name(), "file1.txt");
    
    // Test filtering by extension
    let txt_files = files.filter_by_ext(&"txt".to_string());
    assert_eq!(txt_files.count(), 1);
    
    let json_files = files.filter_by_ext(&"json".to_string());
    assert_eq!(json_files.count(), 1);
}

#[test]
fn test_that_string_and_bytes() {
    // Test ThatString
    let content = "Hello, World!".to_string();
    let that_string = ThatString::new(content.clone());
    
    assert_eq!(that_string.string(), content);
    assert_eq!(that_string.bytes(), content.as_bytes().to_vec());
    
    let lines = that_string.split("\n");
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], content);
    
    // Test ThatBytes
    let bytes = b"Binary data".to_vec();
    let that_bytes = ThatBytes::new(bytes.clone());
    
    let string_result = that_bytes.string().unwrap();
    assert_eq!(string_result, "Binary data");
}

#[test]
fn test_resource_cache() {
    let cache = new_resource_cache();
    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);
    
    // Test cache with expiry
    let cache_with_expiry = new_resource_cache_with_expiry(Duration::from_secs(60));
    assert!(cache_with_expiry.is_empty());
    
    // Test cache configuration
    let config = CacheConfig {
        expiry_duration: Some(Duration::from_secs(30)),
        max_size: Some(100),
        enable_access_tracking: true,
        cleanup_interval: Some(Duration::from_secs(10)),
    };
    
    let configured_cache = new_resource_cache_with_config(config);
    assert!(configured_cache.is_empty());
}

#[test]
fn test_compression_types() {
    // Test compression type properties
    assert_eq!(CompressionType::Gzip.extension(), ".gz");
    assert_eq!(CompressionType::Zstd.extension(), ".zst");
    assert_eq!(CompressionType::Brotli.extension(), ".br");
    assert_eq!(CompressionType::None.extension(), "");
    
    assert_eq!(CompressionType::Gzip.mime_type(), "application/gzip");
    assert_eq!(CompressionType::None.mime_type(), "application/octet-stream");
}

#[test]
fn test_image_types() {
    // Test image type variants
    let types = vec![
        ImageType::Png,
        ImageType::Jpeg,
        ImageType::Gif,
        ImageType::Svg,
        ImageType::WebP,
        ImageType::Unknown,
    ];
    
    assert_eq!(types.len(), 6);
}

#[test]
fn test_mime_type_detection() {
    // Test various file types
    let png_file = ThatFile::new("image.png".to_string(), vec![]);
    assert_eq!(png_file.mime_type(), "image/png");
    assert!(png_file.is_image());
    
    let html_file = ThatFile::new("page.html".to_string(), vec![]);
    assert_eq!(html_file.mime_type(), "text/html");
    assert!(html_file.is_text());
    
    let mp3_file = ThatFile::new("song.mp3".to_string(), vec![]);
    assert_eq!(mp3_file.mime_type(), "audio/mpeg");
    assert!(mp3_file.is_audio());
    
    let mp4_file = ThatFile::new("video.mp4".to_string(), vec![]);
    assert_eq!(mp4_file.mime_type(), "video/mp4");
    assert!(mp4_file.is_video());
}

#[test]
fn test_embedded_filesystem() {
    let mut files = ThatFiles::new();
    
    // Add test files
    files.add_file(ThatFile::new("root.txt".to_string(), b"Root file".to_vec()));
    files.add_file(ThatFile::new("dir/nested.txt".to_string(), b"Nested file".to_vec()));
    
    let fs = files.make_fs();
    
    // Test file reading
    let content = fs.read_file(&"root.txt".to_string()).unwrap();
    assert_eq!(content, b"Root file");
    
    // Test file stats
    let stat = fs.stat(&"root.txt".to_string()).unwrap();
    assert_eq!(stat.name, "root.txt");
    assert_eq!(stat.size, 9);
    assert!(!stat.is_dir);
}

#[test]
fn test_module_initialization() {
    // Test module initialization
    let result = initialize();
    assert!(result.is_ok());
    
    // Test module info
    let info = get_module_info();
    assert_eq!(info.version, "1.0.0");
    assert!(!info.supported_compression_types.is_empty());
    assert!(!info.supported_image_types.is_empty());
}

#[test]
fn test_error_types() {
    // Test error creation functions
    let file_error = file_not_found("missing.txt");
    assert!(matches!(file_error, EmbedError::FileNotFound { .. }));
    
    let format_error = invalid_format("bad.file", "Invalid format");
    assert!(matches!(format_error, EmbedError::InvalidFormat { .. }));
    
    let compression_err = compression_error("Compression failed");
    assert!(matches!(compression_err, EmbedError::CompressionError { .. }));
    
    let template_err = template_parsing_error("Template error");
    assert!(matches!(template_err, EmbedError::TemplateParsingError { .. }));
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
    
    // Test larger sizes
    let large_summary = MemoryUsageSummary {
        embedded_files_size: 1024 * 1024, // 1 MB
        cache_size: 1024 * 1024 * 1024,   // 1 GB
        total_memory_usage: 1024 * 1024 * 1024 + 1024 * 1024,
    };
    
    assert_eq!(large_summary.embedded_size_formatted(), "1.0 MB");
    assert_eq!(large_summary.cache_size_formatted(), "1.0 GB");
    assert_eq!(large_summary.total_size_formatted(), "1.0 GB");
}

#[test]
fn test_constants() {
    use cursed::stdlib::embed_that::constants::*;
    
    // Test template patterns
    assert!(!TEMPLATE_PATTERNS.is_empty());
    assert!(TEMPLATE_PATTERNS.contains(&"templates/*.html"));
    
    // Test static asset patterns
    assert!(!STATIC_ASSET_PATTERNS.is_empty());
    assert!(STATIC_ASSET_PATTERNS.contains(&"static/*"));
    
    // Test config patterns
    assert!(!CONFIG_PATTERNS.is_empty());
    assert!(CONFIG_PATTERNS.contains(&"config/*.json"));
    
    // Test cache constants
    assert_eq!(DEFAULT_CACHE_EXPIRY_SECONDS, 3600);
    assert_eq!(DEFAULT_CACHE_MAX_SIZE, 1000);
    
    // Test compression constants
    assert_eq!(MIN_COMPRESSION_SIZE, 1024);
    assert_eq!(COMPRESSION_RATIO_THRESHOLD, 0.9);
}
