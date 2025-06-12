/// Package Database Tests
/// 
/// Tests the local package database management system including:
/// - SQLite database operations and schema management
/// - Package metadata storage and retrieval
/// - Dependency tracking and integrity verification
/// - Installation history and audit trails

use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;
use chrono::Utc;

use cursed::package_manager::{
    PackageDatabase, SharedPackageDatabase, InstalledPackage, PackageMetadata, FileOperation,
    installer::FileOperationType,
    database::{DatabaseError, InstallAction, PackageDependency, InstallationHistory, DatabaseStatistics},
};

/// Test fixture for database operations
struct DatabaseTestFixture {
    temp_dir: TempDir,
    database: PackageDatabase,
}

impl DatabaseTestFixture {
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let db_path = temp_dir.path().join("test_packages.db");
        let database = PackageDatabase::new(&db_path).expect("Failed to create database");
        
        Self {
            temp_dir,
            database,
        }
    }
    
    fn create_test_package(&self, name: &str, version: &str) -> InstalledPackage {
        let metadata = PackageMetadata {
            name: name.to_string(),
            version: version.to_string(),
            description: format!("Test package {}", name),
            authors: vec!["test@example.com".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: Some(format!("https://github.com/test/{}", name)),
            license: Some("MIT".to_string()),
            keywords: vec!["test".to_string(), "database".to_string()],
            categories: vec!["development".to_string()],
        };
        
        let file_operations = vec![
            FileOperation {
                operation_type: FileOperationType::Created,
                path: self.temp_dir.path().join(format!("{}/src/main.csd", name)),
                backup_path: None,
                permissions: Some(0o644),
                size: 1024,
                checksum: Some("abc123def456".to_string()),
            },
            FileOperation {
                operation_type: FileOperationType::Created,
                path: self.temp_dir.path().join(format!("{}/CursedPackage.toml", name)),
                backup_path: None,
                permissions: Some(0o644),
                size: 512,
                checksum: Some("def456ghi789".to_string()),
            },
        ];
        
        InstalledPackage {
            name: name.to_string(),
            version: version.to_string(),
            install_time: Utc::now(),
            install_path: self.temp_dir.path().join(name),
            file_operations,
            metadata,
        }
    }
    
    fn create_package_with_dependencies(&self, name: &str, version: &str, deps: Vec<(&str, &str)>) -> InstalledPackage {
        let mut dependencies = HashMap::new();
        let mut dev_dependencies = HashMap::new();
        
        for (dep_name, dep_version) in deps {
            dependencies.insert(dep_name.to_string(), dep_version.to_string());
        }
        
        // Add some dev dependencies for testing
        dev_dependencies.insert("test-utils".to_string(), "^1.0.0".to_string());
        
        let metadata = PackageMetadata {
            name: name.to_string(),
            version: version.to_string(),
            description: format!("Test package {} with dependencies", name),
            authors: vec!["test@example.com".to_string()],
            dependencies,
            dev_dependencies,
            repository: Some(format!("https://github.com/test/{}", name)),
            license: Some("MIT".to_string()),
            keywords: vec!["test".to_string(), "dependencies".to_string()],
            categories: vec!["development".to_string()],
        };
        
        let file_operations = vec![
            FileOperation {
                operation_type: FileOperationType::Created,
                path: self.temp_dir.path().join(format!("{}/src/lib.csd", name)),
                backup_path: None,
                permissions: Some(0o644),
                size: 2048,
                checksum: Some("xyz789abc123".to_string()),
            },
        ];
        
        InstalledPackage {
            name: name.to_string(),
            version: version.to_string(),
            install_time: Utc::now(),
            install_path: self.temp_dir.path().join(name),
            file_operations,
            metadata,
        }
    }
}

#[test]
fn test_database_creation_and_schema() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("schema_test.db");
    
    // Create database
    let db_result = PackageDatabase::new(&db_path);
    assert!(db_result.is_ok(), "Failed to create database: {:?}", db_result.err());
    
    // Verify database file exists
    assert!(db_path.exists());
    
    // Verify we can create another connection
    let db2_result = PackageDatabase::new(&db_path);
    assert!(db2_result.is_ok(), "Failed to open existing database: {:?}", db2_result.err());
}

#[test]
fn test_add_and_get_package() {
    let mut fixture = DatabaseTestFixture::new();
    let package = fixture.create_test_package("test-package", "1.0.0");
    
    // Add package
    let add_result = fixture.database.add_package(&package);
    assert!(add_result.is_ok(), "Failed to add package: {:?}", add_result.err());
    
    // Get package
    let get_result = fixture.database.get_package("test-package");
    assert!(get_result.is_ok(), "Failed to get package: {:?}", get_result.err());
    
    let retrieved = get_result.unwrap();
    assert_eq!(retrieved.name, "test-package");
    assert_eq!(retrieved.version, "1.0.0");
    assert_eq!(retrieved.metadata.description, "Test package test-package");
    assert_eq!(retrieved.file_operations.len(), 2);
}

#[test]
fn test_package_not_found() {
    let fixture = DatabaseTestFixture::new();
    
    let get_result = fixture.database.get_package("non-existent");
    assert!(get_result.is_err());
    
    if let Err(DatabaseError::Sqlite(rusqlite::Error::QueryReturnedNoRows)) = get_result {
        // Expected error
    } else {
        panic!("Expected QueryReturnedNoRows error, got: {:?}", get_result);
    }
}

#[test]
fn test_remove_package() {
    let mut fixture = DatabaseTestFixture::new();
    let package = fixture.create_test_package("test-remove", "1.0.0");
    
    // Add package
    fixture.database.add_package(&package).unwrap();
    
    // Verify it exists
    assert!(fixture.database.get_package("test-remove").is_ok());
    
    // Remove package
    let remove_result = fixture.database.remove_package("test-remove");
    assert!(remove_result.is_ok(), "Failed to remove package: {:?}", remove_result.err());
    
    // Verify it's gone
    let get_result = fixture.database.get_package("test-remove");
    assert!(get_result.is_err());
}

#[test]
fn test_remove_non_existent_package() {
    let mut fixture = DatabaseTestFixture::new();
    
    let remove_result = fixture.database.remove_package("non-existent");
    assert!(remove_result.is_err());
    
    if let Err(DatabaseError::PackageNotFound { name }) = remove_result {
        assert_eq!(name, "non-existent");
    } else {
        panic!("Expected PackageNotFound error, got: {:?}", remove_result);
    }
}

#[test]
fn test_list_packages() {
    let mut fixture = DatabaseTestFixture::new();
    
    // Initially empty
    let empty_list = fixture.database.list_packages().unwrap();
    assert!(empty_list.is_empty());
    
    // Add packages
    let package1 = fixture.create_test_package("package-a", "1.0.0");
    let package2 = fixture.create_test_package("package-b", "2.0.0");
    let package3 = fixture.create_test_package("package-c", "1.5.0");
    
    fixture.database.add_package(&package1).unwrap();
    fixture.database.add_package(&package2).unwrap();
    fixture.database.add_package(&package3).unwrap();
    
    // List packages
    let packages = fixture.database.list_packages().unwrap();
    assert_eq!(packages.len(), 3);
    
    // Should be sorted by name
    assert_eq!(packages[0].name, "package-a");
    assert_eq!(packages[1].name, "package-b");
    assert_eq!(packages[2].name, "package-c");
}

#[test]
fn test_package_dependencies() {
    let mut fixture = DatabaseTestFixture::new();
    let deps = vec![("dep1", "^1.0.0"), ("dep2", "~2.1.0")];
    let package = fixture.create_package_with_dependencies("test-deps", "1.0.0", deps);
    
    // Add package
    fixture.database.add_package(&package).unwrap();
    
    // Get dependencies
    let dependencies = fixture.database.get_dependencies("test-deps").unwrap();
    assert_eq!(dependencies.len(), 3); // 2 regular + 1 dev dependency
    
    // Check regular dependencies
    let regular_deps: Vec<_> = dependencies.iter()
        .filter(|d| !d.is_dev_dependency)
        .collect();
    assert_eq!(regular_deps.len(), 2);
    
    // Check dev dependencies
    let dev_deps: Vec<_> = dependencies.iter()
        .filter(|d| d.is_dev_dependency)
        .collect();
    assert_eq!(dev_deps.len(), 1);
    assert_eq!(dev_deps[0].dependency_name, "test-utils");
}

#[test]
fn test_get_dependents() {
    let mut fixture = DatabaseTestFixture::new();
    
    // Create packages with dependencies
    let package1 = fixture.create_package_with_dependencies("pkg1", "1.0.0", vec![("common", "^1.0.0")]);
    let package2 = fixture.create_package_with_dependencies("pkg2", "1.0.0", vec![("common", "^1.0.0"), ("utils", "^2.0.0")]);
    let package3 = fixture.create_package_with_dependencies("pkg3", "1.0.0", vec![("utils", "^2.0.0")]);
    
    fixture.database.add_package(&package1).unwrap();
    fixture.database.add_package(&package2).unwrap();
    fixture.database.add_package(&package3).unwrap();
    
    // Get dependents of "common"
    let common_dependents = fixture.database.get_dependents("common").unwrap();
    assert_eq!(common_dependents.len(), 2);
    
    let dependent_names: Vec<_> = common_dependents.iter()
        .map(|d| d.package_name.as_str())
        .collect();
    assert!(dependent_names.contains(&"pkg1"));
    assert!(dependent_names.contains(&"pkg2"));
    
    // Get dependents of "utils"
    let utils_dependents = fixture.database.get_dependents("utils").unwrap();
    assert_eq!(utils_dependents.len(), 2);
}

#[test]
fn test_is_installed() {
    let mut fixture = DatabaseTestFixture::new();
    let package = fixture.create_test_package("test-installed", "1.0.0");
    
    // Initially not installed
    assert!(!fixture.database.is_installed("test-installed").unwrap());
    
    // Add package
    fixture.database.add_package(&package).unwrap();
    
    // Now should be installed
    assert!(fixture.database.is_installed("test-installed").unwrap());
}

#[test]
fn test_package_replacement() {
    let mut fixture = DatabaseTestFixture::new();
    let package_v1 = fixture.create_test_package("test-replace", "1.0.0");
    let package_v2 = fixture.create_test_package("test-replace", "2.0.0");
    
    // Add v1
    fixture.database.add_package(&package_v1).unwrap();
    let retrieved_v1 = fixture.database.get_package("test-replace").unwrap();
    assert_eq!(retrieved_v1.version, "1.0.0");
    
    // Replace with v2
    fixture.database.add_package(&package_v2).unwrap();
    let retrieved_v2 = fixture.database.get_package("test-replace").unwrap();
    assert_eq!(retrieved_v2.version, "2.0.0");
    
    // Should still only have one package
    let packages = fixture.database.list_packages().unwrap();
    let replace_packages: Vec<_> = packages.iter()
        .filter(|p| p.name == "test-replace")
        .collect();
    assert_eq!(replace_packages.len(), 1);
}

#[test]
fn test_installation_history() {
    let fixture = DatabaseTestFixture::new();
    
    // Initially no history
    let initial_history = fixture.database.get_installation_history(None, None).unwrap();
    assert!(initial_history.is_empty());
    
    // History is automatically recorded when packages are added/removed
    // For this test, we'll check the functionality exists
    let package_history = fixture.database.get_installation_history(Some("test-package"), Some(10)).unwrap();
    assert!(package_history.is_empty()); // No package added yet
}

#[test]
fn test_database_statistics() {
    let mut fixture = DatabaseTestFixture::new();
    
    // Initial statistics
    let initial_stats = fixture.database.get_statistics().unwrap();
    assert_eq!(initial_stats.package_count, 0);
    assert_eq!(initial_stats.total_installed_size, 0);
    
    // Add packages
    let package1 = fixture.create_test_package("stats-test-1", "1.0.0");
    let package2 = fixture.create_test_package("stats-test-2", "1.0.0");
    
    fixture.database.add_package(&package1).unwrap();
    fixture.database.add_package(&package2).unwrap();
    
    // Updated statistics
    let updated_stats = fixture.database.get_statistics().unwrap();
    assert_eq!(updated_stats.package_count, 2);
    assert!(updated_stats.total_installed_size > 0);
    assert!(updated_stats.database_size > 0);
}

#[test]
fn test_database_integrity() {
    let fixture = DatabaseTestFixture::new();
    
    // Fresh database should pass integrity check
    let integrity_result = fixture.database.verify_integrity();
    assert!(integrity_result.is_ok());
    assert!(integrity_result.unwrap());
}

#[test]
fn test_database_vacuum() {
    let fixture = DatabaseTestFixture::new();
    
    // Vacuum should succeed
    let vacuum_result = fixture.database.vacuum();
    assert!(vacuum_result.is_ok());
}

#[test]
fn test_shared_database() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("shared_test.db");
    
    // Create shared database
    let shared_db = SharedPackageDatabase::new(&db_path).unwrap();
    
    // Test basic operations through shared interface
    let result = shared_db.with_db(|db| {
        let packages = db.list_packages()?;
        assert!(packages.is_empty());
        Ok(packages.len())
    });
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_file_operations_storage() {
    let mut fixture = DatabaseTestFixture::new();
    let package = fixture.create_test_package("file-ops-test", "1.0.0");
    
    // Add package
    fixture.database.add_package(&package).unwrap();
    
    // Retrieve and verify file operations
    let retrieved = fixture.database.get_package("file-ops-test").unwrap();
    assert_eq!(retrieved.file_operations.len(), 2);
    
    // Check specific file operation details
    let main_file_op = retrieved.file_operations.iter()
        .find(|op| op.path.file_name().unwrap() == "main.csd")
        .unwrap();
    
    assert_eq!(main_file_op.size, 1024);
    assert_eq!(main_file_op.checksum.as_ref().unwrap(), "abc123def456");
    assert_eq!(main_file_op.permissions.unwrap(), 0o644);
    assert!(main_file_op.backup_path.is_none());
}

#[test]
fn test_metadata_serialization() {
    let mut fixture = DatabaseTestFixture::new();
    let package = fixture.create_test_package("metadata-test", "1.0.0");
    
    // Add package
    fixture.database.add_package(&package).unwrap();
    
    // Retrieve and verify metadata
    let retrieved = fixture.database.get_package("metadata-test").unwrap();
    assert_eq!(retrieved.metadata.name, "metadata-test");
    assert_eq!(retrieved.metadata.description, "Test package metadata-test");
    assert_eq!(retrieved.metadata.authors, vec!["test@example.com"]);
    assert_eq!(retrieved.metadata.license, Some("MIT".to_string()));
    assert_eq!(retrieved.metadata.keywords, vec!["test", "database"]);
    assert_eq!(retrieved.metadata.categories, vec!["development"]);
    assert_eq!(retrieved.metadata.repository, Some("https://github.com/test/metadata-test".to_string()));
}

/// Integration test for complex database operations
#[test]
fn test_complex_database_operations() {
    let mut fixture = DatabaseTestFixture::new();
    
    // Create a package with dependencies
    let deps = vec![("foundation", "^1.0.0"), ("utils", "^2.0.0")];
    let package = fixture.create_package_with_dependencies("complex-test", "1.0.0", deps);
    
    // Add package
    fixture.database.add_package(&package).unwrap();
    
    // Verify all aspects
    let retrieved = fixture.database.get_package("complex-test").unwrap();
    assert_eq!(retrieved.name, "complex-test");
    assert_eq!(retrieved.version, "1.0.0");
    
    // Check dependencies
    let dependencies = fixture.database.get_dependencies("complex-test").unwrap();
    assert_eq!(dependencies.len(), 3); // 2 regular + 1 dev
    
    // Check installation status
    assert!(fixture.database.is_installed("complex-test").unwrap());
    
    // Check statistics
    let stats = fixture.database.get_statistics().unwrap();
    assert_eq!(stats.package_count, 1);
    assert!(stats.total_installed_size > 0);
    
    // Verify integrity
    assert!(fixture.database.verify_integrity().unwrap());
}
