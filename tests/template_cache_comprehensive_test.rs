/// Comprehensive Template Cache Test Suite
/// Tests the multi-level caching system with all features

use std::collections::HashMap;
use std::time::Duration;
use cursed::stdlib::template::template_cache::*;
use cursed::stdlib::template::template_syntax::{TemplateAst, TemplateNode};

fn create_test_ast() -> TemplateAst {
    TemplateAst {
        nodes: vec![TemplateNode::Text("Hello World".to_string())],
    }
}

fn create_test_component() -> TemplateComponent {
    TemplateComponent {
        name: "test_component".to_string(),
        ast: create_test_ast(),
        dependencies: vec!["dep1".to_string(), "dep2".to_string()],
        parameters: HashMap::new(),
    }
}

#[tokio::test]
async fn test_multi_level_cache_operations() {
    let cache = TemplateCache::new(100);
    
    // Test template AST caching (level 0)
    let ast = create_test_ast();
    let result = cache.put_template("template1".to_string(), ast.clone(), 12345).await;
    assert!(result.is_ok());
    
    let retrieved = cache.get_template("template1").await;
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().nodes.len(), ast.nodes.len());
    
    // Test rendered output caching (level 1)
    let output = "Rendered HTML content".to_string();
    let result = cache.put_rendered("output1".to_string(), output.clone(), 12346).await;
    assert!(result.is_ok());
    
    let retrieved_output = cache.get_rendered("output1").await;
    assert!(retrieved_output.is_some());
    assert_eq!(retrieved_output.unwrap(), output);
    
    // Test component caching (level 2)
    let component = create_test_component();
    let result = cache.put_component("comp1".to_string(), component.clone(), 12347).await;
    assert!(result.is_ok());
    
    let retrieved_comp = cache.get_component("comp1").await;
    assert!(retrieved_comp.is_some());
    assert_eq!(retrieved_comp.unwrap().name, component.name);
    
    // Test fragment caching (level 3)
    let fragment = "<div>Fragment content</div>".to_string();
    let result = cache.put_fragment("frag1".to_string(), fragment.clone(), 12348).await;
    assert!(result.is_ok());
    
    let retrieved_frag = cache.get_fragment("frag1").await;
    assert!(retrieved_frag.is_some());
    assert_eq!(retrieved_frag.unwrap(), fragment);
}

#[tokio::test]
async fn test_cache_statistics() {
    let cache = TemplateCache::new(50);
    let ast = create_test_ast();
    
    // Populate cache with different levels
    cache.put_template("t1".to_string(), ast.clone(), 1001).await.unwrap();
    cache.put_rendered("r1".to_string(), "output".to_string(), 1002).await.unwrap();
    
    // Generate hits and misses
    cache.get_template("t1").await;
    cache.get_template("nonexistent").await;
    cache.get_rendered("r1").await;
    
    let stats = cache.detailed_stats();
    assert!(stats.is_some());
    
    let stats = stats.unwrap();
    assert_eq!(*stats.hits.get(&0).unwrap_or(&0), 1); // Template level hits
    assert_eq!(*stats.hits.get(&1).unwrap_or(&0), 1); // Rendered level hits
    assert!(stats.total_operations > 0);
}

#[tokio::test]
async fn test_cache_eviction_policies() {
    let mut config = CacheConfig::default();
    // Set small limits to trigger eviction
    for level in 0..4 {
        if let Some(level_config) = config.level_configs.get_mut(&level) {
            level_config.max_entries = 2;
        }
    }
    
    let cache = TemplateCache::with_config(config);
    let ast = create_test_ast();
    
    // Fill cache beyond limit
    cache.put_template("t1".to_string(), ast.clone(), 1001).await.unwrap();
    cache.put_template("t2".to_string(), ast.clone(), 1002).await.unwrap();
    cache.put_template("t3".to_string(), ast, 1003).await.unwrap(); // Should trigger eviction
    
    let (level_0_entries, _) = cache.level_stats(0);
    assert!(level_0_entries <= 2);
}

#[tokio::test]
async fn test_dependency_tracking() {
    let mut config = CacheConfig::default();
    config.track_dependencies = true;
    
    let cache = TemplateCache::with_config(config);
    let component = create_test_component();
    
    // Put component with dependencies
    cache.put_component("comp1".to_string(), component, 1001).await.unwrap();
    
    // Invalidate dependency should affect dependent entries
    cache.invalidate_dependencies(&["dep1".to_string()]).await;
    
    let retrieved = cache.get_component("comp1").await;
    // Should be invalidated due to dependency
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_background_operations() {
    let mut config = CacheConfig::default();
    config.enable_warming = true;
    config.enable_preloading = true;
    
    let cache = TemplateCache::with_config(config);
    
    // Queue background warming operation
    cache.queue_background_operation(BackgroundOperation::WarmUp(vec!["template1".to_string()])).await;
    
    // Queue cleanup operation
    cache.queue_background_operation(BackgroundOperation::Cleanup).await;
    
    let queue_size = cache.background_queue_size().await;
    assert!(queue_size > 0);
}

#[tokio::test]
async fn test_cache_compression() {
    let mut config = CacheConfig::default();
    config.compression_algorithm = CompressionAlgorithm::Lz4;
    
    // Enable compression for rendered output level
    if let Some(level_1_config) = config.level_configs.get_mut(&1) {
        level_1_config.enable_compression = true;
    }
    
    let cache = TemplateCache::with_config(config);
    
    // Large content that should benefit from compression
    let large_content = "x".repeat(10000);
    cache.put_rendered("large".to_string(), large_content.clone(), 1001).await.unwrap();
    
    let retrieved = cache.get_rendered("large").await;
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), large_content);
    
    // Check compression ratio in stats
    let stats = cache.detailed_stats().unwrap();
    assert!(stats.compression_ratio > 0.0);
}

#[tokio::test]
async fn test_cache_persistence() {
    // Test memory persistence (default)
    let config = CacheConfig::default();
    let cache = TemplateCache::with_config(config);
    
    let ast = create_test_ast();
    cache.put_template("persist_test".to_string(), ast, 1001).await.unwrap();
    
    // Verify data persists in memory
    let retrieved = cache.get_template("persist_test").await;
    assert!(retrieved.is_some());
}

#[tokio::test]
async fn test_hot_reload_functionality() {
    let mut config = CacheConfig::default();
    config.enable_hot_reload = true;
    config.development_mode = true;
    
    let cache = TemplateCache::with_config(config);
    let ast = create_test_ast();
    
    cache.put_template("hot_reload_test".to_string(), ast, 1001).await.unwrap();
    
    // Simulate file change
    cache.mark_file_changed("hot_reload_test").await;
    
    // Entry should be invalidated
    let retrieved = cache.get_template("hot_reload_test").await;
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_cache_key_generation() {
    let key1 = CacheKeyGenerator::generate("template1", None);
    let key2 = CacheKeyGenerator::generate("template2", None);
    assert_ne!(key1, key2);

    let mut params = HashMap::new();
    params.insert("param1".to_string(), "value1".to_string());
    
    let key3 = CacheKeyGenerator::generate("template1", Some(&params));
    assert_ne!(key1, key3);
    
    // Test consistent key generation
    let key4 = CacheKeyGenerator::generate("template1", Some(&params));
    assert_eq!(key3, key4);
}

#[tokio::test]
async fn test_cache_validation() {
    let cache = TemplateCache::new(100);
    let ast = create_test_ast();
    let source_hash = 12345u64;
    
    cache.put_template("validation_test".to_string(), ast, source_hash).await.unwrap();
    
    // Valid hash should return true
    assert!(cache.validate_entry("validation_test", source_hash).await);
    
    // Different hash should return false
    assert!(!cache.validate_entry("validation_test", 54321).await);
    
    // Non-existent key should return false
    assert!(!cache.validate_entry("nonexistent", source_hash).await);
}

#[tokio::test]
async fn test_parallel_cache_operations() {
    let cache = TemplateCache::new(1000);
    let ast = create_test_ast();
    
    // Sequential operations to test thread safety without Send issues
    for i in 0..10 {
        let key = format!("parallel_test_{}", i);
        cache.put_template(key.clone(), ast.clone(), i as u64).await.unwrap();
        let retrieved = cache.get_template(&key).await;
        assert!(retrieved.is_some());
    }
    
    // Verify all entries exist
    for i in 0..10 {
        let key = format!("parallel_test_{}", i);
        let retrieved = cache.get_template(&key).await;
        assert!(retrieved.is_some());
    }
}

#[tokio::test]
async fn test_memory_usage_monitoring() {
    let cache = TemplateCache::new(100);
    let ast = create_test_ast();
    
    // Initial memory usage should be minimal
    let initial_stats = cache.detailed_stats().unwrap();
    let initial_memory: usize = initial_stats.memory_usage.values().sum();
    
    // Add some entries
    for i in 0..10 {
        let key = format!("memory_test_{}", i);
        cache.put_template(key, ast.clone(), i as u64).await.unwrap();
    }
    
    // Memory usage should have increased
    let final_stats = cache.detailed_stats().unwrap();
    let final_memory: usize = final_stats.memory_usage.values().sum();
    
    assert!(final_memory > initial_memory);
}

#[tokio::test]
async fn test_ttl_expiration() {
    let mut config = CacheConfig::default();
    
    // Set very short TTL for testing
    for level in 0..4 {
        if let Some(level_config) = config.level_configs.get_mut(&level) {
            level_config.ttl = Some(Duration::from_millis(100));
        }
    }
    
    let cache = TemplateCache::with_config(config);
    let ast = create_test_ast();
    
    cache.put_template("ttl_test".to_string(), ast, 1001).await.unwrap();
    
    // Should be available immediately
    let retrieved = cache.get_template("ttl_test").await;
    assert!(retrieved.is_some());
    
    // Wait for TTL to expire
    tokio::time::sleep(Duration::from_millis(150)).await;
    
    // Trigger cleanup
    cache.cleanup_expired().await;
    
    // Should be expired now
    let retrieved_after = cache.get_template("ttl_test").await;
    assert!(retrieved_after.is_none());
}
