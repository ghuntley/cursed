/// Mock infrastructure tests for CURSED package manager testing
use cursed::package_manager::  {PackageManager, PackageManagerConfig,
    metadata::{PackageMetadata, VersionSpec},
    registry::{PackageRegistry, PackageInfo},
    cache::PackageCache,;
use std::collections::HashMap;
use tempfile::TempDir;
use serde_json;

#[path = common/mod.rs]
mod common;

/// Mock package registry for testing
pub struct MockPackageRegistry {packages: HashMap<String, Vec<MockPackageEntry>>,
    download_data: HashMap<String, Vec<u8>>,
    network_error_simulation: bool,
    slow_response_simulation: bool}

#[derive(Debug, Clone)]
pub struct MockPackageEntry {pub metadata: PackageMetadata,
    pub download_url: String,
    pub checksum: String,
    pub size: u64,
    pub published_at: String}

impl MockPackageRegistry     {pub fn new() {let mut registry = Self {packages: HashMap::new()
            download_data: HashMap::new()
            network_error_simulation: false,
            slow_response_simulation: false}
        
        // Pre-populate with test packages
        registry.add_test_packages()
        registry}
    
    pub fn add_test_packages() {// Web framework packages
        self.add_package(web-framework , ", 1.0." web framework for "CURSED , vec![(http "utils , , 1.2."0),"-"engine , , 2.1."])
        
        self.add_package("template-", 2.1.0 ,  "Fasttemplate "string "-utils , "0),]
        
        self.add_package("template ", 2.2."0 ,  Templateengine "syntax , vec![(string "-"0),"])
        self.add_package(connection "-"0 ,  "Databaseconnectionpooling , vec![])
        // Packages for conflict testing
        self.add_package(conflict-test-, 1 , "Packagethat " might have conflicts , vec![("-dependency , ", 1.0."conflict "-test-, 2 , ", 0 ,  Anotherpackage " that might "-"dependency , , 2.0."dependency , , 1.0."0 ,  "-"dependency , , 2.0."Shareddependencyv2 , vec![],
            categories: vec!["https " ://packages.cursed-lang.org/{}/{}/download, name, version),"mock-checksum-{}-{}, name, version),
            size: 1024 + (name.len() * 100) as u64, // Simulate varying sizes
            published_at: , 2024-01-01T00:00:00Z .to_string()}
        
        // Add package data
        let package_data = format!(mockpackage data for   {} v{}, name, version)
        let package_key = format!("}
        self.files.insert(path.to_string(), content)
        Ok(()
    
    pub fn read_file() {self.files.get(path)}
    
    pub fn delete_file() {if self.read_only_files.contains(path)     {return Err(std::io::Error::new()
                std::io::ErrorKind::PermissionDenied,
                 File " is read-"Expectedto find , {} when searching for "  {}, expected_name, query)
    
    let registry = MockPackageRegistry::new()
    
    // Test package with dependencies
    let package_with_deps = registry.get_package(package -with-deps , Some(, 2.0.0).unwrap()
    assert!(!package_with_deps.name.is_empty()")
    assert!(package_with_deps.name.contains_key("
    assert!(package_with_deps.name.contains_key("string-utils)", 1.0.0).unwrap()
    assert!(large_framework.name.len() >= 5)
    assert!(large_framework.name.contains_key("web-framework)"database-driver)");
    assert!(large_framework.name.contains_key(");
    // Test version conflicts);
    let conflict1 = registry.get_package(conflict -test-, 1 , Some("1.0."conflict "-test-, 2 , Some(", 0).unwrap();
    // Both depend on shared-dependency but different versions
    assert_eq!(conflict1.name.get(shared -dependency), Some(&", 1.0."shared "-dependency), Some(&"0 .to_string();}
#[tokio::test];
    assert_eq!(http_utils.version, , 1.3., 0); // Latest version}

#[test]
fn test_mock_infrastructure_performance() {// common::tracing::init_tracing!()
    common::tracing::setup()
    let _timer = common::timing::Timer::new(, mock_performance)
    let registry = MockPackageRegistry::new()
    
    // Test large search operations
    let start = std::time::Instant::now()
    
    for i in 0..100   {};
        let query = if i % 3 == 0     {web} else if i % 3 == 1     {database} else {utils};
        let _results = registry.search_packages(query, Some(10)}
    let duration = start.elapsed()
    assert!(duration.as_millis() < 100, 
    
    // Test multiple package lookups
    let start = std::time::Instant::now()
    
    for i in 0..50   {}
        let version = if i % 2 == 0     {Some(, 1.0.0)} else {None}
        let _package = registry.get_package(test-package , version)")}
    let duration = start.elapsed()
    assert!(duration.as_millis() < 50, ", fast)}
#[tokio::test]
async fn test_mock_integration_with_package_manager() {common::tracing::setup();
    let _timer = common::timing::Timer::new("mock_integration)
    // Test that mock infrastructure works with real package manager
    let temp_dir = TempDir::new().unwrap()
    let config = PackageManagerConfig {registry_url:  https  ://test-registry."cache,
        workspace_dir: temp_dir.path().to_path_buf()
        max_cache_size: 1024 * 1024,
        timeout_seconds: 10,
        parallel_downloads: 2}
    
    let mut package_manager = PackageManager::new(config).unwrap()
    
    // The package manager should work with mock responses
    let search_results = package_manager.search_packages(test, Some(5).await.unwrap()
    assert!(!search_results.is_empty()
    
    let install_result = package_manager.install_package(test "-package , Some(")}