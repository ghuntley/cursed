#[cfg(test)]
mod tests {
    use super::*;
    use crate::package_manager::{PackageManagerConfig, PackageManager};
    
    #[test]
    fn test_package_manager_config_fields() {
        // Test that all configuration fields are accessible
        let config = PackageManagerConfig::default();
        
        // Verify workspace_dir field (was missing)
        assert_eq!(config.workspace_dir, std::path::PathBuf::from("."));
        
        // Verify max_cache_size field (was missing)
        assert_eq!(config.max_cache_size, 1024 * 1024 * 1024); // 1GB
        
        // Verify other fields
        assert_eq!(config.cache_dir, std::path::PathBuf::from("target/packages"));
        assert_eq!(config.registry_url, "https://packages.cursed-lang.org");
        assert_eq!(config.offline_mode, false);
        assert_eq!(config.verify_signatures, true);
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.parallel_downloads, 4);
    }
    
    #[test]
    fn test_custom_package_manager_config() {
        // Test creating custom configuration with all fields
        let custom_config = PackageManagerConfig {
            cache_dir: std::path::PathBuf::from("/tmp/cursed_cache"),
            registry_url: "https://custom-registry.example.com".to_string(),
            offline_mode: true,
            verify_signatures: false,
            workspace_dir: std::path::PathBuf::from("/home/user/project"),
            max_cache_size: 512 * 1024 * 1024, // 512MB
            timeout_seconds: 60,
            parallel_downloads: 8,
        };
        
        assert_eq!(custom_config.workspace_dir, std::path::PathBuf::from("/home/user/project"));
        assert_eq!(custom_config.max_cache_size, 512 * 1024 * 1024);
        assert_eq!(custom_config.offline_mode, true);
        assert_eq!(custom_config.parallel_downloads, 8);
    }
    
    #[test]
    fn test_package_manager_creation() {
        // Test that PackageManager can be created with configuration
        use tempfile::TempDir;
        
        let temp_dir = TempDir::new().unwrap();
        let config = PackageManagerConfig {
            cache_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        
        // This should not panic and should create a PackageManager successfully
        match PackageManager::new(config) {
            Ok(_manager) => {
                // Success - configuration is valid
            }
            Err(e) => {
                panic!("PackageManager creation failed: {}", e);
            }
        }
    }
}
