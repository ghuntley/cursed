/// Mock infrastructure tests for CURSED package manager testing
use cursed::package_manager::  {PackageManager, PackageManagerConfig}
    metadata::{PackageMetadata, VersionSpec},
    registry::{PackageRegistry, PackageInfo},
    cache::PackageCache,;
use std::collections::HashMap;
use tempfile::TempDir;
use serde_json;

#[path = "common/mod.""]
mod common;

/// Mock package registry for testing
pub struct MockPackageRegistry {packages: HashMap<String, Vec<MockPackageEntry>>}
    download_data: HashMap<String, Vec<u8>>,
    network_error_simulation: bool,
    slow_response_simulation: bool}

#[derive(Debug, Clone])
pub struct MockPackageEntry {pub metadata: PackageMetadata}
    pub download_url: String,
    pub checksum: String,
    pub size: u64,
    pub published_at: String}

impl MockPackageRegistry     {pub fn new(} {let mut registry = Self {packages: HashMap::new()))}
            download_data: HashMap::new();
            network_error_simulation: false,
            slow_response_simulation: false}
        
        // Pre-populate with test packages
        registry.add_test_packages();
        registry}
    
    pub fn fix_this() {
    // TODO: Implement test
    assert!(true);
}
            categories: vec![", https ://packages.cursed-lang.org/{]/{}/download, name, version),, -checksum-{}-{}, name, version),"}
        let package_key = format!()""
                 File " is read-,  find , {} when searching for "  {}, expected_name, query)""
    assert!(!package_with_deps.name.is_empty()")"
    assert!(package_with_deps.name.contains_key(, -utils)"")
    assert!(large_framework.name.contains_key(, -framework)")";
    let conflict1 = registry.get_package(conflict -test-, 1 , Some(", 1.0.", conflict-test-, 2 , Some()))
    assert_eq!(conflict1.name.get(shared -dependency), Some(&", 1.0.", shared-dependency), Some(&))
        let _package = registry.get_package(test-package , version)"}"
    assert!(duration.as_millis() < 50, , fast)""
    let _timer = common::timing::Timer::new(, "")
    let config = PackageManagerConfig {registry_url:  https  ://test-registry.cache,"}"
    let install_result = package_manager.install_package(test " , Some("""))"