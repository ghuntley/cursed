/// Package Manager Test Verification
/// 
/// This file verifies that all the package manager fixes are working correctly

use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_package_manager_comprehensive_fix_verification() {
    // Test 1: Version parsing with pre-release versions
    let version = cursed::package_manager::version::Version::parse("1.0.0-alpha").unwrap();
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert_eq!(version.pre_release, Some("alpha".to_string()));

    // Test 2: Version requirements with comparison operators
    let req = cursed::package_manager::version::VersionReq::parse(">=1.0.0").unwrap();
    let version = cursed::package_manager::version::Version::parse("1.2.3").unwrap();
    assert!(req.matches(&version));

    // Test 3: Lock file operations
    let temp_dir = TempDir::new().unwrap();
    let lock_file_path = temp_dir.path().join("test.lock");
    let mut lock_manager = cursed::package_manager::lock_file::LockFileManager::new(&lock_file_path);
    
    let package = cursed::package_manager::lock_file::LockedPackage {
        name: "test-package".to_string(),
        version: "1.0.0".to_string(),
        source: "registry".to_string(),
        checksum: Some("abc123".to_string()),
        dependencies: vec!["dep1".to_string()],
    };
    
    lock_manager.add_package(package);
    lock_manager.save().unwrap();
    
    let mut new_lock_manager = cursed::package_manager::lock_file::LockFileManager::new(&lock_file_path);
    new_lock_manager.load().unwrap();
    let packages = new_lock_manager.get_packages().unwrap_or_default();
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "test-package");

    // Test 4: Workspace configuration
    let workspace_dir = TempDir::new().unwrap();
    let members = vec!["package1".to_string(), "package2".to_string()];
    let workspace = cursed::package_manager::workspace::WorkspaceManager::init_workspace(
        workspace_dir.path(), 
        members.clone()
    ).unwrap();
    
    assert_eq!(workspace.members().len(), 2);
    assert!(workspace.members().iter().any(|m| m.name == "package1"));
    assert!(workspace.members().iter().any(|m| m.name == "package2"));

    // Test 5: Package manager with offline mode
    let config_dir = TempDir::new().unwrap();
    let config = cursed::package_manager::PackageManagerConfig {
        cache_dir: config_dir.path().join("cache"),
        registry_url: "https://test-registry.cursed-lang.org".to_string(),
        offline_mode: true,
        verify_signatures: false,
        workspace_dir: config_dir.path().to_path_buf(),
        max_cache_size: 1024 * 1024,
        timeout_seconds: 10,
        parallel_downloads: 1,
    };
    
    let pkg_manager = cursed::package_manager::PackageManager::new(config).unwrap();
    
    // In offline mode, this should fail
    let result = pkg_manager.get_package_info("non-existent", None).await;
    assert!(result.is_err());
    
    println!("✅ All package manager fixes verified successfully!");
}
