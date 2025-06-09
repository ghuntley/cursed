/// Mock infrastructure tests for CURSED package manager testing
use cursed::package_manager::{
    PackageManager, PackageManagerConfig,
    metadata::{PackageMetadata, VersionSpec},
    registry::{PackageRegistry, PackageInfo},
    cache::PackageCache,
};
use std::collections::HashMap;
use tempfile::TempDir;
use serde_json;

#[path = "common/mod.rs"]
mod common;

/// Mock package registry for testing
pub struct MockPackageRegistry {
    packages: HashMap<String, Vec<MockPackageEntry>>,
    download_data: HashMap<String, Vec<u8>>,
    network_error_simulation: bool,
    slow_response_simulation: bool,
}

#[derive(Debug, Clone)]
pub struct MockPackageEntry {
    pub metadata: PackageMetadata,
    pub download_url: String,
    pub checksum: String,
    pub size: u64,
    pub published_at: String,
}

impl MockPackageRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            packages: HashMap::new(),
            download_data: HashMap::new(),
            network_error_simulation: false,
            slow_response_simulation: false,
        };
        
        // Pre-populate with test packages
        registry.add_test_packages();
        registry
    }
    
    pub fn add_test_packages(&mut self) {
        // Web framework packages
        self.add_package("web-framework", "1.0.0", "A modern web framework for CURSED", vec![
            ("http-utils", "1.2.0"),
            ("template-engine", "2.1.0"),
        ]);
        
        self.add_package("web-framework", "1.1.0", "Updated web framework with new features", vec![
            ("http-utils", "1.3.0"),
            ("template-engine", "2.2.0"),
            ("security-middleware", "1.0.0"),
        ]);
        
        // Database packages
        self.add_package("database-driver", "2.1.0", "High-performance database driver", vec![
            ("connection-pool", "1.5.0"),
        ]);
        
        // Utility packages
        self.add_package("http-utils", "1.2.0", "HTTP utilities and helpers", vec![]);
        self.add_package("http-utils", "1.3.0", "Updated HTTP utilities with better performance", vec![]);
        
        self.add_package("template-engine", "2.1.0", "Fast template rendering engine", vec![
            ("string-utils", "1.0.0"),
        ]);
        
        self.add_package("template-engine", "2.2.0", "Template engine with new syntax", vec![
            ("string-utils", "1.1.0"),
        ]);
        
        // Authentication packages
        self.add_package("authentication", "1.5.0", "Authentication and authorization library", vec![
            ("crypto-utils", "2.0.0"),
            ("jwt-tokens", "1.3.0"),
        ]);
        
        // Support packages
        self.add_package("string-utils", "1.0.0", "String manipulation utilities", vec![]);
        self.add_package("string-utils", "1.1.0", "Enhanced string utilities", vec![]);
        self.add_package("crypto-utils", "2.0.0", "Cryptographic utilities", vec![]);
        self.add_package("jwt-tokens", "1.3.0", "JWT token generation and validation", vec![]);
        self.add_package("connection-pool", "1.5.0", "Database connection pooling", vec![]);
        self.add_package("security-middleware", "1.0.0", "Security middleware for web apps", vec![]);
        
        // Large framework simulation
        self.add_package("large-framework", "1.0.0", "A comprehensive framework with many dependencies", vec![
            ("web-framework", "1.1.0"),
            ("database-driver", "2.1.0"),
            ("authentication", "1.5.0"),
            ("logging-framework", "3.0.0"),
            ("configuration-manager", "1.2.0"),
            ("metrics-collector", "2.1.0"),
        ]);
        
        // Additional dependencies for large framework
        self.add_package("logging-framework", "3.0.0", "Advanced logging framework", vec![
            ("string-utils", "1.1.0"),
        ]);
        
        self.add_package("configuration-manager", "1.2.0", "Configuration management system", vec![]);
        self.add_package("metrics-collector", "2.1.0", "Application metrics collection", vec![]);
        
        // Packages for conflict testing
        self.add_package("conflict-test-1", "1.0.0", "Package that might have conflicts", vec![
            ("shared-dependency", "1.0.0"),
        ]);
        
        self.add_package("conflict-test-2", "1.0.0", "Another package that might conflict", vec![
            ("shared-dependency", "2.0.0"), // Different version
        ]);
        
        self.add_package("shared-dependency", "1.0.0", "Shared dependency v1", vec![]);
        self.add_package("shared-dependency", "2.0.0", "Shared dependency v2", vec![]);
        
        // Test packages for basic operations
        self.add_package("test-package", "1.0.0", "A simple test package", vec![]);
        self.add_package("package-with-deps", "2.0.0", "Package with dependencies", vec![
            ("test-package", "1.0.0"),
            ("string-utils", "1.0.0"),
        ]);
    }
    
    pub fn add_package(&mut self, name: &str, version: &str, description: &str, deps: Vec<(&str, &str)>) {
        let dependencies: HashMap<String, String> = deps.into_iter()
            .map(|(n, v)| (n.to_string(), v.to_string()))
            .collect();
        
        let metadata = PackageMetadata {
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
            authors: vec![format!("Test Author <test@{}.com>", name)],
            dependencies: dependencies.iter()
                .map(|(n, v)| (n.clone(), VersionSpec::Simple(v.clone())))
                .collect(),
            dev_dependencies: HashMap::new(),
            repository: Some(format!("https://github.com/cursed-packages/{}", name)),
            license: Some("MIT".to_string()),
            keywords: vec!["cursed".to_string(), "package".to_string()],
            categories: vec!["development".to_string()],
        };
        
        let entry = MockPackageEntry {
            metadata,
            download_url: format!("https://packages.cursed-lang.org/{}/{}/download", name, version),
            checksum: format!("mock-checksum-{}-{}", name, version),
            size: 1024 + (name.len() * 100) as u64, // Simulate varying sizes
            published_at: "2024-01-01T00:00:00Z".to_string(),
        };
        
        // Add package data
        let package_data = format!("mock package data for {} v{}", name, version);
        let package_key = format!("{}@{}", name, version);
        self.download_data.insert(package_key, package_data.into_bytes());
        
        self.packages.entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(entry);
    }
    
    pub fn search_packages(&self, query: &str, limit: Option<usize>) -> Vec<PackageInfo> {
        if self.network_error_simulation {
            return vec![];
        }
        
        let mut results = Vec::new();
        
        for (name, entries) in &self.packages {
            if name.contains(query) || 
               entries.iter().any(|e| e.metadata.description.to_lowercase().contains(&query.to_lowercase()) ||
                                     e.metadata.keywords.iter().any(|k| k.contains(query))) {
                
                // Return latest version for search results
                if let Some(latest) = entries.iter().max_by(|a, b| {
                    self.compare_versions(&a.metadata.version, &b.metadata.version)
                }) {
                    results.push(self.entry_to_package_info(latest));
                }
            }
        }
        
        results.sort_by(|a, b| a.name.cmp(&b.name));
        
        if let Some(limit) = limit {
            results.truncate(limit);
        }
        
        results
    }
    
    pub fn get_package(&self, name: &str, version: Option<&str>) -> Option<PackageInfo> {
        if self.network_error_simulation {
            return None;
        }
        
        if let Some(entries) = self.packages.get(name) {
            if let Some(version) = version {
                // Find specific version
                entries.iter()
                    .find(|e| e.metadata.version == version)
                    .map(|e| self.entry_to_package_info(e))
            } else {
                // Return latest version
                entries.iter()
                    .max_by(|a, b| self.compare_versions(&a.metadata.version, &b.metadata.version))
                    .map(|e| self.entry_to_package_info(e))
            }
        } else {
            None
        }
    }
    
    pub fn download_package(&self, name: &str, version: &str) -> Option<Vec<u8>> {
        if self.network_error_simulation {
            return None;
        }
        
        let package_key = format!("{}@{}", name, version);
        self.download_data.get(&package_key).cloned()
    }
    
    pub fn simulate_network_error(&mut self, enabled: bool) {
        self.network_error_simulation = enabled;
    }
    
    pub fn simulate_slow_response(&mut self, enabled: bool) {
        self.slow_response_simulation = enabled;
    }
    
    fn entry_to_package_info(&self, entry: &MockPackageEntry) -> PackageInfo {
        PackageInfo {
            name: entry.metadata.name.clone(),
            version: entry.metadata.version.clone(),
            description: entry.metadata.description.clone(),
            authors: entry.metadata.authors.clone(),
            download_url: entry.download_url.clone(),
            checksum: entry.checksum.clone(),
            size: entry.size,
            published_at: entry.published_at.clone(),
            dependencies: entry.metadata.dependencies.iter()
                .map(|(n, v)| match v {
                    VersionSpec::Simple(version) => (n.clone(), version.clone()),
                    VersionSpec::Complex { version: Some(v), .. } => (n.clone(), v.clone()),
                    _ => (n.clone(), "latest".to_string()),
                })
                .collect(),
        }
    }
    
    fn compare_versions(&self, a: &str, b: &str) -> std::cmp::Ordering {
        // Simple semantic version comparison
        let parse_version = |v: &str| -> Vec<u32> {
            v.split('.')
                .map(|part| part.parse().unwrap_or(0))
                .collect()
        };
        
        let va = parse_version(a);
        let vb = parse_version(b);
        va.cmp(&vb)
    }
}

/// Mock file system for testing cache operations
pub struct MockFileSystem {
    files: HashMap<String, Vec<u8>>,
    directories: HashSet<String>,
    read_only_files: HashSet<String>,
}

impl MockFileSystem {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            directories: HashSet::new(),
            read_only_files: HashSet::new(),
        }
    }
    
    pub fn create_directory(&mut self, path: &str) {
        self.directories.insert(path.to_string());
    }
    
    pub fn write_file(&mut self, path: &str, content: Vec<u8>) -> Result<(), std::io::Error> {
        if self.read_only_files.contains(path) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "File is read-only"
            ));
        }
        
        self.files.insert(path.to_string(), content);
        Ok(())
    }
    
    pub fn read_file(&self, path: &str) -> Option<&Vec<u8>> {
        self.files.get(path)
    }
    
    pub fn delete_file(&mut self, path: &str) -> Result<(), std::io::Error> {
        if self.read_only_files.contains(path) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "File is read-only"
            ));
        }
        
        self.files.remove(path);
        Ok(())
    }
    
    pub fn file_exists(&self, path: &str) -> bool {
        self.files.contains_key(path)
    }
    
    pub fn set_read_only(&mut self, path: &str, read_only: bool) {
        if read_only {
            self.read_only_files.insert(path.to_string());
        } else {
            self.read_only_files.remove(path);
        }
    }
    
    pub fn list_files(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }
}

use std::collections::HashSet;

#[tokio::test]
async fn test_mock_registry_basic_operations() {
    common::tracing::setup();
    let _timer = common::timing::Timer::new("mock_registry_basic");
    
    let registry = MockPackageRegistry::new();
    
    // Test search functionality
    let web_packages = registry.search_packages("web", Some(5));
    assert!(!web_packages.is_empty());
    assert!(web_packages.iter().any(|p| p.name.contains("web")));
    
    // Test get specific package
    let package = registry.get_package("test-package", Some("1.0.0"));
    assert!(package.is_some());
    let package = package.unwrap();
    assert_eq!(package.name, "test-package");
    assert_eq!(package.version, "1.0.0");
    
    // Test get latest version
    let latest = registry.get_package("web-framework", None);
    assert!(latest.is_some());
    let latest = latest.unwrap();
    assert_eq!(latest.name, "web-framework");
    // Should be version 1.1.0 (latest)
    assert_eq!(latest.version, "1.1.0");
    
    // Test download
    let data = registry.download_package("test-package", "1.0.0");
    assert!(data.is_some());
    let data = data.unwrap();
    assert!(!data.is_empty());
    assert!(String::from_utf8(data).unwrap().contains("test-package"));
}

#[tokio::test]
async fn test_mock_registry_search_capabilities() {
    common::tracing::setup();
    let _timer = common::timing::Timer::new("mock_registry_search");
    
    let registry = MockPackageRegistry::new();
    
    // Test different search terms
    let test_cases = vec![
        ("web", vec!["web-framework"]),
        ("database", vec!["database-driver"]),
        ("auth", vec!["authentication"]),
        ("string", vec!["string-utils"]),
        ("framework", vec!["web-framework", "large-framework", "logging-framework"]),
    ];
    
    for (query, expected_names) in test_cases {
        let results = registry.search_packages(query, None);
        
        for expected_name in expected_names {
            assert!(results.iter().any(|p| p.name == expected_name),
                "Expected to find '{}' when searching for '{}'", expected_name, query);
        }
    }
    
    // Test search limits
    let limited_results = registry.search_packages("", Some(3));
    assert!(limited_results.len() <= 3);
    
    let unlimited_results = registry.search_packages("", None);
    assert!(unlimited_results.len() >= limited_results.len());
}

#[tokio::test]
async fn test_mock_registry_dependency_simulation() {
    common::tracing::setup();
    let _timer = common::timing::Timer::new("mock_registry_dependencies");
    
    let registry = MockPackageRegistry::new();
    
    // Test package with dependencies
    let package_with_deps = registry.get_package("package-with-deps", Some("2.0.0")).unwrap();
    assert!(!package_with_deps.dependencies.is_empty());
    assert!(package_with_deps.dependencies.contains_key("test-package"));
    assert!(package_with_deps.dependencies.contains_key("string-utils"));
    
    // Test large framework dependencies
    let large_framework = registry.get_package("large-framework", Some("1.0.0")).unwrap();
    assert!(large_framework.dependencies.len() >= 5);
    assert!(large_framework.dependencies.contains_key("web-framework"));
    assert!(large_framework.dependencies.contains_key("database-driver"));
    assert!(large_framework.dependencies.contains_key("authentication"));
    
    // Test version conflicts
    let conflict1 = registry.get_package("conflict-test-1", Some("1.0.0")).unwrap();
    let conflict2 = registry.get_package("conflict-test-2", Some("1.0.0")).unwrap();
    
    // Both depend on shared-dependency but different versions
    assert_eq!(conflict1.dependencies.get("shared-dependency"), Some(&"1.0.0".to_string()));
    assert_eq!(conflict2.dependencies.get("shared-dependency"), Some(&"2.0.0".to_string()));
}

#[tokio::test]
async fn test_mock_registry_error_simulation() {
    common::tracing::setup();
    let _timer = common::timing::Timer::new("mock_registry_errors");
    
    let mut registry = MockPackageRegistry::new();
    
    // Test normal operation
    let normal_result = registry.search_packages("test", Some(5));
    assert!(!normal_result.is_empty());
    
    // Enable network error simulation
    registry.simulate_network_error(true);
    
    let error_result = registry.search_packages("test", Some(5));
    assert!(error_result.is_empty());
    
    let no_package = registry.get_package("test-package", Some("1.0.0"));
    assert!(no_package.is_none());
    
    let no_download = registry.download_package("test-package", "1.0.0");
    assert!(no_download.is_none());
    
    // Disable error simulation
    registry.simulate_network_error(false);
    
    let restored_result = registry.search_packages("test", Some(5));
    assert!(!restored_result.is_empty());
}

#[tokio::test]
async fn test_mock_file_system() {
    common::tracing::setup();
    let _timer = common::timing::Timer::new("mock_filesystem");
    
    let mut fs = MockFileSystem::new();
    
    // Test basic file operations
    fs.create_directory("/test/cache");
    fs.write_file("/test/cache/package.json", b"test content".to_vec()).unwrap();
    
    assert!(fs.file_exists("/test/cache/package.json"));
    let content = fs.read_file("/test/cache/package.json").unwrap();
    assert_eq!(content, b"test content");
    
    // Test file listing
    fs.write_file("/test/cache/another.json", b"more content".to_vec()).unwrap();
    let files = fs.list_files();
    assert_eq!(files.len(), 2);
    
    // Test read-only files
    fs.set_read_only("/test/cache/package.json", true);
    
    let write_result = fs.write_file("/test/cache/package.json", b"new content".to_vec());
    assert!(write_result.is_err());
    
    let delete_result = fs.delete_file("/test/cache/package.json");
    assert!(delete_result.is_err());
    
    // Test removing read-only
    fs.set_read_only("/test/cache/package.json", false);
    fs.write_file("/test/cache/package.json", b"updated content".to_vec()).unwrap();
    
    let updated_content = fs.read_file("/test/cache/package.json").unwrap();
    assert_eq!(updated_content, b"updated content");
}

#[tokio::test]
async fn test_mock_registry_version_handling() {
    common::tracing::setup();
    let _timer = common::timing::Timer::new("mock_registry_versions");
    
    let registry = MockPackageRegistry::new();
    
    // Test multiple versions of same package
    let v1 = registry.get_package("web-framework", Some("1.0.0"));
    let v2 = registry.get_package("web-framework", Some("1.1.0"));
    let latest = registry.get_package("web-framework", None);
    
    assert!(v1.is_some());
    assert!(v2.is_some());
    assert!(latest.is_some());
    
    assert_eq!(v1.unwrap().version, "1.0.0");
    assert_eq!(v2.unwrap().version, "1.1.0");
    assert_eq!(latest.unwrap().version, "1.1.0"); // Should be latest
    
    // Test version comparison
    let http_utils_versions = registry.search_packages("http-utils", None);
    assert!(!http_utils_versions.is_empty());
    
    // Should return latest version in search results
    let http_utils = &http_utils_versions[0];
    assert_eq!(http_utils.version, "1.3.0"); // Latest version
}

#[test]
fn test_mock_infrastructure_performance() {
    // init_tracing!();
    common::tracing::setup();
    let _timer = common::timing::Timer::new("mock_performance");
    
    let registry = MockPackageRegistry::new();
    
    // Test large search operations
    let start = std::time::Instant::now();
    
    for i in 0..100 {
        let query = if i % 3 == 0 { "web" } else if i % 3 == 1 { "database" } else { "utils" };
        let _results = registry.search_packages(query, Some(10));
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 100, "Mock registry operations should be fast");
    
    // Test multiple package lookups
    let start = std::time::Instant::now();
    
    for i in 0..50 {
        let version = if i % 2 == 0 { Some("1.0.0") } else { None };
        let _package = registry.get_package("test-package", version);
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 50, "Package lookups should be very fast");
}

#[tokio::test]
async fn test_mock_integration_with_package_manager() {
    common::tracing::setup();
    let _timer = common::timing::Timer::new("mock_integration");
    
    // Test that mock infrastructure works with real package manager
    let temp_dir = TempDir::new().unwrap();
    let config = PackageManagerConfig {
        registry_url: "https://test-registry.com".to_string(),
        cache_dir: temp_dir.path().join("cache"),
        workspace_dir: temp_dir.path().to_path_buf(),
        max_cache_size: 1024 * 1024,
        timeout_seconds: 10,
        parallel_downloads: 2,
    };
    
    let mut package_manager = PackageManager::new(config).unwrap();
    
    // The package manager should work with mock responses
    let search_results = package_manager.search_packages("test", Some(5)).await.unwrap();
    assert!(!search_results.is_empty());
    
    let install_result = package_manager.install_package("test-package", Some("1.0.0")).await.unwrap();
    assert!(!install_result.is_empty());
    
    let cached_packages = package_manager.list_installed().unwrap();
    assert!(!cached_packages.is_empty());
}
