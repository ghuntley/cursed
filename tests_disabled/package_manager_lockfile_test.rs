/// Comprehensive tests for package manager lock file functionality
/// 
/// Tests lock file generation, parsing, validation, and integration
/// with package installation and dependency resolution.

use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;

use cursed::package_manager::{
    PackageMetadata, LockFile, LockFileManager, LockedPackage, PackageSource, VersionSpec,
    lockfile::{LockFileError, LOCK_FILE_VERSION},
};

#[test]
fn test_lock_file_creation() {
    let lock_file = LockFile::new();
    assert_eq!(lock_file.version, LOCK_FILE_VERSION);
    assert!(lock_file.packages.is_empty());
    assert!(!lock_file.metadata.generated_at.is_empty());
    assert!(!lock_file.metadata.cursed_version.is_empty());
    assert!(!lock_file.metadata.platform.is_empty());
}

#[test]
fn test_lock_file_package_operations() {
    let mut lock_file = LockFile::new();
    
    let package1 = LockedPackage {
        name: "test-package".to_string(),
        version: "1.0.0".to_string(),
        source: PackageSource::Registry {
            url: "registry+https://packages.cursed-lang.org/".to_string(),
        },
        dependencies: vec!["dep1 1.0.0".to_string()],
        checksum: "sha256:abc123".to_string(),
        build_metadata: None,
    };
    
    let package2 = LockedPackage {
        name: "another-package".to_string(),
        version: "2.0.0".to_string(),
        source: PackageSource::Git {
            url: "https://github.com/example/repo.git".to_string(),
            reference: "main".to_string(),
        },
        dependencies: vec![],
        checksum: "sha256:def456".to_string(),
        build_metadata: Some({
            let mut metadata = HashMap::new();
            metadata.insert("feature".to_string(), "test".to_string());
            metadata
        }),
    };
    
    // Add packages
    lock_file.add_package(package1.clone());
    lock_file.add_package(package2.clone());
    
    assert_eq!(lock_file.packages.len(), 2);
    
    // Packages should be sorted by name
    assert_eq!(lock_file.packages[0].name, "another-package");
    assert_eq!(lock_file.packages[1].name, "test-package");
    
    // Test finding dependents
    let dependents = lock_file.find_dependents("dep1");
    assert_eq!(dependents.len(), 1);
    assert_eq!(dependents[0].name, "test-package");
    
    // Remove package
    lock_file.remove_package("test-package");
    assert_eq!(lock_file.packages.len(), 1);
    assert_eq!(lock_file.packages[0].name, "another-package");
}

#[test]
fn test_lock_file_manager_creation() {
    let temp_dir = TempDir::new().unwrap();
    let lock_path = temp_dir.path().join("CursedPackage.lock");
    
    let manager = LockFileManager::new(&lock_path);
    assert_eq!(manager.lock_file_path, lock_path);
    assert!(!manager.exists());
}

#[test]
fn test_lock_file_generation_and_saving() {
    let temp_dir = TempDir::new().unwrap();
    let lock_path = temp_dir.path().join("CursedPackage.lock");
    let mut manager = LockFileManager::new(&lock_path);
    
    let dependencies = vec![
        PackageMetadata {
            name: "serde".to_string(),
            version: "1.0.0".to_string(),
            description: "Serialization library".to_string(),
            authors: vec!["Serde Team".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: Some("https://github.com/serde-rs/serde".to_string()),
            license: Some("MIT".to_string()),
            keywords: vec!["serialization".to_string()],
            categories: vec!["encoding".to_string()],
        },
        PackageMetadata {
            name: "tokio".to_string(),
            version: "1.5.0".to_string(),
            description: "Async runtime".to_string(),
            authors: vec!["Tokio Team".to_string()],
            dependencies: {
                let mut deps = HashMap::new();
                deps.insert("serde".to_string(), VersionSpec::Simple("1.0.0".to_string()));
                deps
            },
            dev_dependencies: HashMap::new(),
            repository: Some("https://github.com/tokio-rs/tokio".to_string()),
            license: Some("MIT".to_string()),
            keywords: vec!["async".to_string()],
            categories: vec!["asynchronous".to_string()],
        },
    ];
    
    // Generate lock file
    manager.generate_from_dependencies(&dependencies, None).unwrap();
    
    // Save to disk
    manager.save().unwrap();
    
    // Verify file exists
    assert!(lock_path.exists());
    
    // Load and verify content
    let content = std::fs::read_to_string(&lock_path).unwrap();
    assert!(content.contains("version = 1"));
    assert!(content.contains("name = \"serde\""));
    assert!(content.contains("name = \"tokio\""));
    assert!(content.contains("checksum = \"sha256:"));
}

#[test]
fn test_lock_file_loading_and_validation() {
    let temp_dir = TempDir::new().unwrap();
    let lock_path = temp_dir.path().join("CursedPackage.lock");
    
    // Create a valid lock file
    let lock_content = r#"
version = 1

[[package]]
name = "test-package"
version = "1.0.0"
dependencies = ["dep1 1.0.0"]
checksum = "sha256:abc123"

[package.source]
type = "Registry"
url = "registry+https://packages.cursed-lang.org/"

[metadata]
generated_at = "2024-01-01T00:00:00Z"
cursed_version = "0.1.0"
platform = "linux-x86_64"
"#;
    
    std::fs::write(&lock_path, lock_content).unwrap();
    
    let mut manager = LockFileManager::new(&lock_path);
    
    // Load lock file
    manager.load().unwrap();
    
    // Validate
    manager.validate().unwrap();
    
    // Check loaded content
    let packages = manager.get_packages().unwrap();
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "test-package");
    assert_eq!(packages[0].version, "1.0.0");
    
    // Test getting locked version
    let locked_version = manager.get_locked_version("test-package").unwrap();
    assert_eq!(locked_version.version, "1.0.0");
    
    assert!(manager.get_locked_version("non-existent").is_none());
}

#[test]
fn test_lock_file_validation_errors() {
    let temp_dir = TempDir::new().unwrap();
    let lock_path = temp_dir.path().join("CursedPackage.lock");
    
    // Test unsupported version
    let invalid_version_content = r#"
version = 999
[[package]]
name = "test"
version = "1.0.0"
checksum = "sha256:abc"
dependencies = []

[package.source]
type = "Registry"
url = "test"

[metadata]
generated_at = "2024-01-01T00:00:00Z"
cursed_version = "0.1.0"
platform = "linux-x86_64"
"#;
    
    std::fs::write(&lock_path, invalid_version_content).unwrap();
    let mut manager = LockFileManager::new(&lock_path);
    
    let result = manager.load();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), LockFileError::UnsupportedVersion { .. }));
    
    // Test empty package name
    let invalid_package_content = format!(r#"
version = {}

[[package]]
name = ""
version = "1.0.0"
checksum = "sha256:abc"
dependencies = []

[package.source]
type = "Registry"
url = "test"

[metadata]
generated_at = "2024-01-01T00:00:00Z"
cursed_version = "0.1.0"
platform = "linux-x86_64"
"#, LOCK_FILE_VERSION);
    
    std::fs::write(&lock_path, invalid_package_content).unwrap();
    let mut manager = LockFileManager::new(&lock_path);
    manager.load().unwrap();
    
    let result = manager.validate();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), LockFileError::ValidationFailed { .. }));
}

#[test]
fn test_lock_file_checksum_verification() {
    let temp_dir = TempDir::new().unwrap();
    let lock_path = temp_dir.path().join("CursedPackage.lock");
    let mut manager = LockFileManager::new(&lock_path);
    
    let package = PackageMetadata {
        name: "test-package".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package".to_string(),
        authors: vec!["Test Author".to_string()],
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
        repository: None,
        license: None,
        keywords: Vec::new(),
        categories: Vec::new(),
    };
    
    // Generate lock file with this package
    manager.generate_from_dependencies(&[package.clone()], None).unwrap();
    
    // Verify checksum
    let verification_result = manager.verify_package_checksum(&package).unwrap();
    assert!(verification_result);
    
    // Test with modified package (should fail verification)
    let mut modified_package = package.clone();
    modified_package.description = "Modified description".to_string();
    
    let verification_result = manager.verify_package_checksum(&modified_package).unwrap();
    assert!(!verification_result);
}

#[test]
fn test_lock_file_update_dependencies() {
    let temp_dir = TempDir::new().unwrap();
    let lock_path = temp_dir.path().join("CursedPackage.lock");
    let mut manager = LockFileManager::new(&lock_path);
    
    let initial_deps = vec![
        PackageMetadata {
            name: "package1".to_string(),
            version: "1.0.0".to_string(),
            description: "Package 1".to_string(),
            authors: Vec::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            license: None,
            keywords: Vec::new(),
            categories: Vec::new(),
        },
    ];
    
    // Initial generation
    manager.update_dependencies(&initial_deps).unwrap();
    assert!(lock_path.exists());
    
    let updated_deps = vec![
        initial_deps[0].clone(),
        PackageMetadata {
            name: "package2".to_string(),
            version: "2.0.0".to_string(),
            description: "Package 2".to_string(),
            authors: Vec::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            license: None,
            keywords: Vec::new(),
            categories: Vec::new(),
        },
    ];
    
    // Update with additional package
    manager.update_dependencies(&updated_deps).unwrap();
    
    // Load and verify
    manager.load().unwrap();
    let packages = manager.get_packages().unwrap();
    assert_eq!(packages.len(), 2);
}

#[test]
fn test_dependency_string_parsing() {
    use cursed::package_manager::lockfile::{parse_dependency_string, format_dependency_string};
    
    // Test valid dependency strings
    assert_eq!(parse_dependency_string("serde 1.0.0"), Some(("serde", "1.0.0")));
    assert_eq!(parse_dependency_string("tokio 1.5.0"), Some(("tokio", "1.5.0")));
    
    // Test invalid dependency strings
    assert_eq!(parse_dependency_string("invalid"), None);
    assert_eq!(parse_dependency_string(""), None);
    assert_eq!(parse_dependency_string("   "), None);
    
    // Test formatting
    assert_eq!(format_dependency_string("serde", "1.0.0"), "serde 1.0.0");
    assert_eq!(format_dependency_string("package-name", "2.1.3"), "package-name 2.1.3");
}

#[test]
fn test_package_source_variants() {
    // Test Registry source
    let registry_source = PackageSource::Registry {
        url: "registry+https://packages.cursed-lang.org/".to_string(),
    };
    
    // Test Git source
    let git_source = PackageSource::Git {
        url: "https://github.com/example/repo.git".to_string(),
        reference: "main".to_string(),
    };
    
    // Test Path source
    let path_source = PackageSource::Path {
        path: "../local-package".to_string(),
    };
    
    // All sources should serialize/deserialize correctly
    let registry_json = serde_json::to_string(&registry_source).unwrap();
    let git_json = serde_json::to_string(&git_source).unwrap();
    let path_json = serde_json::to_string(&path_source).unwrap();
    
    assert!(registry_json.contains("Registry"));
    assert!(git_json.contains("Git"));
    assert!(path_json.contains("Path"));
    
    let _: PackageSource = serde_json::from_str(&registry_json).unwrap();
    let _: PackageSource = serde_json::from_str(&git_json).unwrap();
    let _: PackageSource = serde_json::from_str(&path_json).unwrap();
}

#[test]
fn test_lock_file_not_found_error() {
    let temp_dir = TempDir::new().unwrap();
    let non_existent_path = temp_dir.path().join("does-not-exist.lock");
    let mut manager = LockFileManager::new(&non_existent_path);
    
    let result = manager.load();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), LockFileError::NotFound { .. }));
}

#[test]
fn test_lock_file_workspace_integration() {
    let temp_dir = TempDir::new().unwrap();
    let lock_path = temp_dir.path().join("CursedPackage.lock");
    let mut manager = LockFileManager::new(&lock_path);
    
    let dependencies = vec![
        PackageMetadata {
            name: "workspace-dep".to_string(),
            version: "1.0.0".to_string(),
            description: "Workspace dependency".to_string(),
            authors: Vec::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            license: None,
            keywords: Vec::new(),
            categories: Vec::new(),
        },
    ];
    
    let workspace_root = Some("/workspace/root".to_string());
    
    // Generate with workspace root
    manager.generate_from_dependencies(&dependencies, workspace_root.clone()).unwrap();
    manager.save().unwrap();
    
    // Load and verify workspace metadata
    manager.load().unwrap();
    let lock_file = manager.lock_file.as_ref().unwrap();
    assert_eq!(lock_file.metadata.workspace_root, workspace_root);
}

#[test]
fn test_lock_file_roundtrip_serialization() {
    let original_lock_file = LockFile {
        version: LOCK_FILE_VERSION,
        packages: vec![
            LockedPackage {
                name: "test-package".to_string(),
                version: "1.0.0".to_string(),
                source: PackageSource::Registry {
                    url: "registry+https://packages.cursed-lang.org/".to_string(),
                },
                dependencies: vec!["dep1 1.0.0".to_string(), "dep2 2.0.0".to_string()],
                checksum: "sha256:abc123".to_string(),
                build_metadata: Some({
                    let mut metadata = HashMap::new();
                    metadata.insert("feature".to_string(), "full".to_string());
                    metadata.insert("platform".to_string(), "linux".to_string());
                    metadata
                }),
            },
        ],
        metadata: cursed::package_manager::lockfile::LockFileMetadata {
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            cursed_version: "0.1.0".to_string(),
            platform: "linux-x86_64".to_string(),
            workspace_root: Some("/workspace".to_string()),
        },
    };
    
    // Serialize to TOML
    let toml_content = toml::to_string_pretty(&original_lock_file).unwrap();
    
    // Deserialize from TOML
    let deserialized_lock_file: LockFile = toml::from_str(&toml_content).unwrap();
    
    // Verify roundtrip preservation
    assert_eq!(original_lock_file.version, deserialized_lock_file.version);
    assert_eq!(original_lock_file.packages.len(), deserialized_lock_file.packages.len());
    
    let original_pkg = &original_lock_file.packages[0];
    let deserialized_pkg = &deserialized_lock_file.packages[0];
    
    assert_eq!(original_pkg.name, deserialized_pkg.name);
    assert_eq!(original_pkg.version, deserialized_pkg.version);
    assert_eq!(original_pkg.dependencies, deserialized_pkg.dependencies);
    assert_eq!(original_pkg.checksum, deserialized_pkg.checksum);
    assert_eq!(original_pkg.build_metadata, deserialized_pkg.build_metadata);
    
    assert_eq!(original_lock_file.metadata.workspace_root, deserialized_lock_file.metadata.workspace_root);
}
