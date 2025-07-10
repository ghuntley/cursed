//! Integration tests for package manager search and publish functionality

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_package_validation() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = temp_dir.path().join("test_package");
        
        // Create package directory structure
        fs::create_dir_all(&package_path).unwrap();
        fs::create_dir_all(package_path.join("src")).unwrap();
        
        // Create package.toml
        let package_toml = r#"
[package]
name = "test-validation"
version = "1.0.0"
description = "Test package for validation"
authors = ["Test Author <test@example.com>"]
license = "MIT"

[dependencies]
"#;
        fs::write(package_path.join("package.toml"), package_toml).unwrap();
        
        // Create source file
        fs::write(
            package_path.join("src/mod.csd"),
            "slay hello() { vibez.spill(\"Hello!\") }"
        ).unwrap();
        
        // Create README
        fs::write(
            package_path.join("README.md"),
            "# Test Package\n\nThis is a test package."
        ).unwrap();
        
        let config = PackageManagerConfig::default();
        let pkg_manager = PackageManager::new(config).unwrap();
        
        // Test package structure validation
        let validation_result = pkg_manager.validate_package_structure(&package_path);
        assert!(validation_result.is_ok(), "Package validation should succeed");
    }

    #[tokio::test]
    async fn test_package_validation_missing_files() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = temp_dir.path().join("test_package");
        
        // Create package directory but missing required files
        fs::create_dir_all(&package_path).unwrap();
        
        let config = PackageManagerConfig::default();
        let pkg_manager = PackageManager::new(config).unwrap();
        
        // Test package structure validation with missing files
        let validation_result = pkg_manager.validate_package_structure(&package_path);
        assert!(validation_result.is_err(), "Package validation should fail for missing files");
        
        let error_message = validation_result.unwrap_err().to_string();
        assert!(error_message.contains("Required file missing"), "Error should mention missing files");
    }

    #[tokio::test]
    async fn test_package_archive_creation() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = temp_dir.path().join("test_package");
        
        // Create package directory structure
        fs::create_dir_all(&package_path).unwrap();
        fs::create_dir_all(package_path.join("src")).unwrap();
        
        // Create package files
        fs::write(package_path.join("package.toml"), "[package]\nname = \"test\"").unwrap();
        fs::write(package_path.join("src/mod.csd"), "// test source").unwrap();
        fs::write(package_path.join("README.md"), "# Test").unwrap();
        
        // Create files that should be excluded
        fs::create_dir_all(package_path.join(".git")).unwrap();
        fs::write(package_path.join(".git/config"), "git config").unwrap();
        fs::create_dir_all(package_path.join("target")).unwrap();
        fs::write(package_path.join("target/debug"), "debug binary").unwrap();
        
        let config = PackageManagerConfig::default();
        let pkg_manager = PackageManager::new(config).unwrap();
        
        // Test archive creation
        let archive_result = pkg_manager.create_package_archive(&package_path);
        assert!(archive_result.is_ok(), "Archive creation should succeed");
        
        let archive_data = archive_result.unwrap();
        assert!(!archive_data.is_empty(), "Archive should not be empty");
        
        // Archive should be compressed (gzip format starts with 0x1f, 0x8b)
        assert_eq!(archive_data[0], 0x1f, "Archive should start with gzip magic number");
        assert_eq!(archive_data[1], 0x8b, "Archive should have correct gzip format");
    }

    #[test]
    fn test_package_manager_creation() {
        let config = PackageManagerConfig::default();
        let pkg_manager_result = PackageManager::new(config);
        assert!(pkg_manager_result.is_ok(), "Package manager creation should succeed");
    }

    #[test]
    fn test_package_manager_config_default() {
        let config = PackageManagerConfig::default();
        assert_eq!(config.registry_url, "https://packages.cursed-lang.org");
        assert!(!config.offline_mode);
        assert!(config.verify_signatures);
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.parallel_downloads, 4);
    }

    #[tokio::test]
    async fn test_dry_run_publish() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = temp_dir.path().join("test_package");
        
        // Create valid package directory structure
        fs::create_dir_all(&package_path).unwrap();
        fs::create_dir_all(package_path.join("src")).unwrap();
        
        // Create package.toml
        let package_toml = r#"
[package]
name = "test-dry-run"
version = "1.0.0"
description = "Test package for dry run"
authors = ["Test Author <test@example.com>"]
license = "MIT"

[dependencies]
"#;
        fs::write(package_path.join("package.toml"), package_toml).unwrap();
        
        // Create source file
        fs::write(
            package_path.join("src/mod.csd"),
            "slay hello() { vibez.spill(\"Hello from dry run!\") }"
        ).unwrap();
        
        let config = PackageManagerConfig::default();
        let pkg_manager = PackageManager::new(config).unwrap();
        
        // Test dry run publish (should not fail even without network)
        let dry_run_result = pkg_manager.publish_package(
            package_path.to_str().unwrap(),
            true // dry_run = true
        ).await;
        
        assert!(dry_run_result.is_ok(), "Dry run publish should succeed: {:?}", dry_run_result);
    }

    #[tokio::test]
    async fn test_invalid_package_toml() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = temp_dir.path().join("test_package");
        
        // Create package directory structure
        fs::create_dir_all(&package_path).unwrap();
        fs::create_dir_all(package_path.join("src")).unwrap();
        
        // Create invalid package.toml (missing [package] section)
        let invalid_toml = r#"
name = "invalid-package"
version = "1.0.0"
"#;
        fs::write(package_path.join("package.toml"), invalid_toml).unwrap();
        
        // Create source file
        fs::write(package_path.join("src/mod.csd"), "// source").unwrap();
        
        let config = PackageManagerConfig::default();
        let pkg_manager = PackageManager::new(config).unwrap();
        
        // Test publish with invalid TOML
        let publish_result = pkg_manager.publish_package(
            package_path.to_str().unwrap(),
            true // dry_run = true
        ).await;
        
        assert!(publish_result.is_err(), "Publish should fail with invalid TOML");
        
        let error_message = publish_result.unwrap_err().to_string();
        assert!(error_message.contains("package"), "Error should mention package section");
    }

    #[tokio::test]
    async fn test_missing_package_directory() {
        let config = PackageManagerConfig::default();
        let pkg_manager = PackageManager::new(config).unwrap();
        
        // Test publish with non-existent directory
        let publish_result = pkg_manager.publish_package(
            "/non/existent/path",
            true // dry_run = true
        ).await;
        
        assert!(publish_result.is_err(), "Publish should fail with missing directory");
        
        let error_message = publish_result.unwrap_err().to_string();
        assert!(error_message.contains("does not exist"), "Error should mention missing directory");
    }
}
