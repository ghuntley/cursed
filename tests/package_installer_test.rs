/// Comprehensive Package Installation Tests
/// 
/// Tests the complete package installation system including:
/// - Package extraction and installation
/// - File conflict resolution and rollback
/// - Script execution safety and sandboxing
/// - Database operations and integrity
/// - Upgrade/downgrade scenarios

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tempfile::TempDir;
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::{Builder, Header};

use cursed::package_manager::{
    PackageInstaller, InstallerConfig, PackageDatabase, SharedPackageDatabase,
    ScriptExecutor, InstallScript, ScriptContext, ScriptInterpreter,
    PackageMetadata, InstalledPackage, FileOperation,
};

/// Test fixture for package installation
struct InstallationTestFixture {
    temp_dir: TempDir,
    database: SharedPackageDatabase,
    installer_config: InstallerConfig,
}

impl InstallationTestFixture {
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let db_path = temp_dir.path().join("packages.db");
        let database = SharedPackageDatabase::new(&db_path).expect("Failed to create database");
        
        let installer_config = InstallerConfig {
            project_root: temp_dir.path().to_path_buf(),
            extract_to: temp_dir.path().join("packages"),
            allow_overwrites: true,
            backup_existing: true,
            verify_checksums: true,
            enable_scripts: true,
            max_file_size: 10 * 1024 * 1024, // 10MB for tests
            preserve_permissions: true,
        };
        
        Self {
            temp_dir,
            database,
            installer_config,
        }
    }
    
    fn create_test_package(&self, name: &str, version: &str) -> (PackageMetadata, Vec<u8>) {
        let metadata = PackageMetadata {
            name: name.to_string(),
            version: version.to_string(),
            description: format!("Test package {}", name),
            authors: vec!["test@example.com".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            license: Some("MIT".to_string()),
            keywords: vec!["test".to_string()],
            categories: vec!["development".to_string()],
        };
        
        // Create tar.gz package
        let mut archive_data = Vec::new();
        {
            let encoder = GzEncoder::new(&mut archive_data, Compression::default());
            let mut tar = Builder::new(encoder);
            
            // Add package metadata
            let metadata_content = toml::to_string(&metadata).unwrap();
            let mut header = Header::new_gnu();
            header.set_path("CursedPackage.toml").unwrap();
            header.set_size(metadata_content.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            tar.append(&header, metadata_content.as_bytes()).unwrap();
            
            // Add source files
            let main_content = format!(
                "// {} v{}\nslay main() {{\n    capicola(\"Hello from {}!\");\n}}\n",
                name, version, name
            );
            let mut header = Header::new_gnu();
            header.set_path("src/main.csd").unwrap();
            header.set_size(main_content.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            tar.append(&header, main_content.as_bytes()).unwrap();
            
            // Add README
            let readme_content = format!("# {}\n\nTest package for {}", name, name);
            let mut header = Header::new_gnu();
            header.set_path("README.md").unwrap();
            header.set_size(readme_content.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            tar.append(&header, readme_content.as_bytes()).unwrap();
            
            tar.finish().unwrap();
        }
        
        (metadata, archive_data)
    }
    
    fn create_installer(&self) -> PackageInstaller {
        let db_arc = Arc::new(Mutex::new(
            PackageDatabase::new(self.temp_dir.path().join("packages.db")).unwrap()
        ));
        PackageInstaller::new(db_arc, self.installer_config.clone()).unwrap()
    }
}

#[tokio::test]
async fn test_basic_package_installation() {
    let fixture = InstallationTestFixture::new();
    let mut installer = fixture.create_installer();
    
    // Create test package
    let (metadata, package_data) = fixture.create_test_package("test-basic", "1.0.0");
    
    // Install package
    let result = installer.install_package(&metadata, &package_data).await;
    assert!(result.is_ok(), "Package installation failed: {:?}", result.err());
    
    let installed = result.unwrap();
    assert_eq!(installed.name, "test-basic");
    assert_eq!(installed.version, "1.0.0");
    assert!(!installed.file_operations.is_empty());
    
    // Verify files exist
    let package_dir = fixture.installer_config.extract_to.join("test-basic");
    assert!(package_dir.join("CursedPackage.toml").exists());
    assert!(package_dir.join("src/main.csd").exists());
    assert!(package_dir.join("README.md").exists());
    
    // Verify database entry
    let db_installed = installer.get_installed_package("test-basic");
    assert!(db_installed.is_ok());
    let db_package = db_installed.unwrap();
    assert_eq!(db_package.name, "test-basic");
    assert_eq!(db_package.version, "1.0.0");
}

#[tokio::test]
async fn test_package_upgrade() {
    let fixture = InstallationTestFixture::new();
    let mut installer = fixture.create_installer();
    
    // Install version 1.0.0
    let (metadata_v1, package_data_v1) = fixture.create_test_package("test-upgrade", "1.0.0");
    let result_v1 = installer.install_package(&metadata_v1, &package_data_v1).await;
    assert!(result_v1.is_ok());
    
    // Install version 2.0.0 (upgrade)
    let (metadata_v2, package_data_v2) = fixture.create_test_package("test-upgrade", "2.0.0");
    let result_v2 = installer.install_package(&metadata_v2, &package_data_v2).await;
    assert!(result_v2.is_ok());
    
    // Verify upgrade
    let installed = installer.get_installed_package("test-upgrade").unwrap();
    assert_eq!(installed.version, "2.0.0");
    
    // Verify only one version is installed
    let all_packages = installer.list_installed().unwrap();
    let upgrade_packages: Vec<_> = all_packages.iter()
        .filter(|p| p.name == "test-upgrade")
        .collect();
    assert_eq!(upgrade_packages.len(), 1);
}

#[tokio::test]
async fn test_package_uninstallation() {
    let fixture = InstallationTestFixture::new();
    let mut installer = fixture.create_installer();
    
    // Install package
    let (metadata, package_data) = fixture.create_test_package("test-uninstall", "1.0.0");
    let install_result = installer.install_package(&metadata, &package_data).await;
    assert!(install_result.is_ok());
    
    // Verify installation
    assert!(installer.get_installed_package("test-uninstall").is_ok());
    
    // Uninstall package
    let uninstall_result = installer.uninstall_package("test-uninstall");
    assert!(uninstall_result.is_ok(), "Uninstallation failed: {:?}", uninstall_result.err());
    
    // Verify removal
    let get_result = installer.get_installed_package("test-uninstall");
    assert!(get_result.is_err());
    
    // Verify files removed
    let package_dir = fixture.installer_config.extract_to.join("test-uninstall");
    assert!(!package_dir.exists() || fs::read_dir(&package_dir).unwrap().next().is_none());
}

#[tokio::test]
async fn test_file_conflict_handling() {
    let fixture = InstallationTestFixture::new();
    let mut installer = fixture.create_installer();
    
    // Create conflicting file
    let package_dir = fixture.installer_config.extract_to.join("test-conflict");
    fs::create_dir_all(&package_dir).unwrap();
    let conflict_file = package_dir.join("README.md");
    fs::write(&conflict_file, "Existing content").unwrap();
    
    // Install package with conflicting file
    let (metadata, package_data) = fixture.create_test_package("test-conflict", "1.0.0");
    let result = installer.install_package(&metadata, &package_data).await;
    
    // Should succeed due to allow_overwrites = true
    assert!(result.is_ok());
    
    // Verify backup was created
    let installed = result.unwrap();
    let readme_operations: Vec<_> = installed.file_operations.iter()
        .filter(|op| op.path.file_name().unwrap() == "README.md")
        .collect();
    assert!(!readme_operations.is_empty());
    
    // Should have backup path
    let readme_op = readme_operations[0];
    assert!(readme_op.backup_path.is_some());
}

#[test]
fn test_package_verification() {
    let fixture = InstallationTestFixture::new();
    let installer = fixture.create_installer();
    
    // Create mock installed package
    let package_dir = fixture.installer_config.extract_to.join("test-verify");
    fs::create_dir_all(&package_dir).unwrap();
    fs::write(package_dir.join("test.txt"), "test content").unwrap();
    
    // Mock package record
    let metadata = PackageMetadata {
        name: "test-verify".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package".to_string(),
        authors: vec!["test@example.com".to_string()],
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
        repository: None,
        license: None,
        keywords: vec![],
        categories: vec![],
    };
    
    let file_op = FileOperation {
        operation_type: cursed::package_manager::installer::FileOperationType::Created,
        path: package_dir.join("test.txt"),
        backup_path: None,
        permissions: Some(0o644),
        size: 12,
        checksum: None,
    };
    
    let installed_package = InstalledPackage {
        name: "test-verify".to_string(),
        version: "1.0.0".to_string(),
        install_time: chrono::Utc::now(),
        install_path: package_dir,
        file_operations: vec![file_op],
        metadata,
    };
    
    // Add to database manually for this test
    {
        let db = Arc::new(Mutex::new(
            PackageDatabase::new(fixture.temp_dir.path().join("verify.db")).unwrap()
        ));
        let mut db_lock = db.lock().unwrap();
        db_lock.add_package(&installed_package).unwrap();
    }
    
    // Create installer with the database
    let db_arc = Arc::new(Mutex::new(
        PackageDatabase::new(fixture.temp_dir.path().join("verify.db")).unwrap()
    ));
    let installer_with_data = PackageInstaller::new(db_arc, fixture.installer_config.clone()).unwrap();
    
    // Verify package
    let verification_result = installer_with_data.verify_package("test-verify");
    assert!(verification_result.is_ok());
    assert!(verification_result.unwrap());
}

#[test]
fn test_list_installed_packages() {
    let fixture = InstallationTestFixture::new();
    let installer = fixture.create_installer();
    
    // Initially should be empty
    let packages = installer.list_installed().unwrap();
    assert!(packages.is_empty());
    
    // Test with non-existent package
    let get_result = installer.get_installed_package("non-existent");
    assert!(get_result.is_err());
}

#[tokio::test]
async fn test_script_execution() {
    let temp_dir = TempDir::new().unwrap();
    let script_executor = ScriptExecutor::new(true, temp_dir.path().to_path_buf());
    
    // Create test script
    let script = InstallScript {
        name: "test_script".to_string(),
        phase: "post-install".to_string(),
        interpreter: ScriptInterpreter::Shell,
        content: "echo 'Hello from install script'".to_string(),
        timeout_seconds: Some(30),
        environment: HashMap::new(),
        required: false,
    };
    
    let context = ScriptContext {
        package_name: "test-package".to_string(),
        package_version: "1.0.0".to_string(),
        install_dir: temp_dir.path().to_path_buf(),
        temp_dir: temp_dir.path().to_path_buf(),
    };
    
    // Execute script
    let result = script_executor.execute_script(&script, &context).await;
    assert!(result.is_ok(), "Script execution failed: {:?}", result.err());
    
    let script_result = result.unwrap();
    assert!(script_result.success);
    assert!(script_result.stdout.contains("Hello from install script"));
}

#[tokio::test]
async fn test_dangerous_script_rejection() {
    let temp_dir = TempDir::new().unwrap();
    let script_executor = ScriptExecutor::new(true, temp_dir.path().to_path_buf());
    
    // Create dangerous script
    let dangerous_script = InstallScript {
        name: "dangerous_script".to_string(),
        phase: "post-install".to_string(),
        interpreter: ScriptInterpreter::Shell,
        content: "rm -rf /".to_string(), // Dangerous command
        timeout_seconds: Some(30),
        environment: HashMap::new(),
        required: false,
    };
    
    let context = ScriptContext {
        package_name: "test-package".to_string(),
        package_version: "1.0.0".to_string(),
        install_dir: temp_dir.path().to_path_buf(),
        temp_dir: temp_dir.path().to_path_buf(),
    };
    
    // Should reject dangerous script
    let result = script_executor.execute_script(&dangerous_script, &context).await;
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(e.to_string().contains("SecurityViolation"));
    }
}

#[test]
fn test_script_interpreter_detection() {
    let temp_dir = TempDir::new().unwrap();
    let script_executor = ScriptExecutor::new(true, temp_dir.path().to_path_buf());
    
    // Test shell interpreter (should be available)
    assert!(script_executor.check_interpreter(&ScriptInterpreter::Shell));
    
    // Test custom interpreter (should not be available)
    assert!(!script_executor.check_interpreter(&ScriptInterpreter::Custom("nonexistent".to_string())));
}

#[tokio::test]
async fn test_script_timeout() {
    let temp_dir = TempDir::new().unwrap();
    let script_executor = ScriptExecutor::new(true, temp_dir.path().to_path_buf());
    
    // Create script that sleeps longer than timeout
    let timeout_script = InstallScript {
        name: "timeout_script".to_string(),
        phase: "post-install".to_string(),
        interpreter: ScriptInterpreter::Shell,
        content: "sleep 5".to_string(),
        timeout_seconds: Some(1), // 1 second timeout
        environment: HashMap::new(),
        required: false,
    };
    
    let context = ScriptContext {
        package_name: "test-package".to_string(),
        package_version: "1.0.0".to_string(),
        install_dir: temp_dir.path().to_path_buf(),
        temp_dir: temp_dir.path().to_path_buf(),
    };
    
    // Should timeout
    let result = script_executor.execute_script(&timeout_script, &context).await;
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(e.to_string().contains("timeout") || e.to_string().contains("Timeout"));
    }
}

#[test]
fn test_installer_config_defaults() {
    let config = InstallerConfig::default();
    assert!(config.allow_overwrites);
    assert!(config.backup_existing);
    assert!(config.verify_checksums);
    assert!(config.enable_scripts);
    assert!(config.preserve_permissions);
    assert_eq!(config.max_file_size, 100 * 1024 * 1024);
}

#[test]
fn test_script_statistics() {
    let temp_dir = TempDir::new().unwrap();
    let script_executor = ScriptExecutor::new(true, temp_dir.path().to_path_buf());
    
    let stats = script_executor.get_statistics();
    assert!(stats.enabled);
    assert!(stats.sandbox_enabled);
    assert_eq!(stats.timeout_seconds, 300); // Default timeout
}

/// Integration test for complete package lifecycle
#[tokio::test]
async fn test_complete_package_lifecycle() {
    let fixture = InstallationTestFixture::new();
    let mut installer = fixture.create_installer();
    
    // 1. Install package
    let (metadata, package_data) = fixture.create_test_package("lifecycle-test", "1.0.0");
    let install_result = installer.install_package(&metadata, &package_data).await;
    assert!(install_result.is_ok());
    
    // 2. Verify installation
    let verification = installer.verify_package("lifecycle-test");
    assert!(verification.is_ok());
    assert!(verification.unwrap());
    
    // 3. List packages
    let packages = installer.list_installed().unwrap();
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "lifecycle-test");
    
    // 4. Upgrade package
    let (metadata_v2, package_data_v2) = fixture.create_test_package("lifecycle-test", "2.0.0");
    let upgrade_result = installer.install_package(&metadata_v2, &package_data_v2).await;
    assert!(upgrade_result.is_ok());
    
    // 5. Verify upgrade
    let updated_package = installer.get_installed_package("lifecycle-test").unwrap();
    assert_eq!(updated_package.version, "2.0.0");
    
    // 6. Uninstall package
    let uninstall_result = installer.uninstall_package("lifecycle-test");
    assert!(uninstall_result.is_ok());
    
    // 7. Verify removal
    let final_packages = installer.list_installed().unwrap();
    assert!(final_packages.is_empty());
}
