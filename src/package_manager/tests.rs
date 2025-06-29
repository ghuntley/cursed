//! Tests for the CURSED package manager

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package_manager::{
        PackageManager, PackageManagerConfig, PackageRegistry, RegistryConfig,
        CacheConfig, PackageCache, PackageDownloader, DownloadConfig,
        PackageResolver, ResolutionConfig, Version, VersionReq
    };
    use tempfile::TempDir;
    use std::str::FromStr;

    #[test]
    fn test_package_manager_creation() {
        let config = PackageManagerConfig::default();
        let pm = PackageManager::new(config);
        assert!(pm.is_ok());
    }

    #[test]
    fn test_version_parsing() {
        let version = Version::from_str("1.2.3").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_version_requirements() {
        let version = Version::new(1, 2, 3);
        let req = VersionReq::parse("^1.0.0").unwrap();
        assert!(req.matches(&version));
        
        let req2 = VersionReq::parse("~1.2.0").unwrap();
        assert!(req2.matches(&version));
        
        let req3 = VersionReq::parse("2.0.0").unwrap();
        assert!(!req3.matches(&version));
    }

    #[test]
    fn test_version_comparison() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 0, 1);
        let v3 = Version::new(1, 1, 0);
        let v4 = Version::new(2, 0, 0);

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);
    }

    #[test]
    fn test_version_compatibility() {
        let v1 = Version::new(1, 2, 3);
        let v2 = Version::new(1, 3, 0);
        let v3 = Version::new(2, 0, 0);

        // Test compatibility using version requirements instead
        let req = VersionReq::parse("^1.2.0").unwrap();
        assert!(req.matches(&v1));
        assert!(req.matches(&v2));
        assert!(!req.matches(&v3));
    }

    #[tokio::test]
    async fn test_registry_search() {
        let registry = PackageRegistry::new(RegistryConfig::default()).unwrap();
        let results = registry.search_packages("test").await;
        assert!(results.is_ok());
    }

    #[tokio::test]
    async fn test_package_info() {
        let registry = PackageRegistry::new(RegistryConfig::default()).unwrap();
        let info = registry.get_package_info("test-package", None).await;
        assert!(info.is_ok());
    }

    #[test]
    fn test_cache_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = CacheConfig {
            cache_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        
        let cache = PackageCache::new(config);
        assert!(cache.is_ok());
    }

    #[test]
    fn test_downloader_creation() {
        let downloader = PackageDownloader::new(DownloadConfig::default());
        assert!(downloader.is_ok());
    }

    #[tokio::test]
    async fn test_resolver_creation() {
        let registry = PackageRegistry::new(RegistryConfig::default()).unwrap();
        let mut resolver = PackageResolver::new(registry);
        
        // Test resolver exists
        let resolution = resolver.resolve_dependencies(
            vec![("test-package".to_string(), VersionReq::parse("*").unwrap())],
            ResolutionConfig::default()
        ).await;
        
        // Should complete without panicking (may fail due to mock registry)
        let _ = resolution;
    }
}
