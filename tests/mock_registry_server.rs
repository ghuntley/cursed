/// Mock registry server for testing package download functionality
/// 
/// Provides a simple HTTP server that mimics a package registry
/// for integration testing without requiring real network access.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json;

/// Mock package information
#[derive(Debug, Clone)]
pub struct MockPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub content: Vec<u8>,
    pub checksum: String,
}

/// Simple mock registry server
pub struct MockRegistryServer {
    packages: Arc<Mutex<HashMap<String, MockPackage>>>,
    port: u16,
}

impl MockRegistryServer {
    pub fn new(port: u16) -> Self {
        let mut packages = HashMap::new();
        
        // Add some mock packages
        let test_package = MockPackage {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: "A test package for CURSED".to_string(),
            content: b"fake package content".to_vec(),
            checksum: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
        };
        packages.insert("test-package@1.0.0".to_string(), test_package);
        
        let json_package = MockPackage {
            name: "cursed-json".to_string(),
            version: "2.1.0".to_string(),
            description: "JSON parsing and generation for CURSED".to_string(),
            content: b"fake json package content".to_vec(),
            checksum: "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3".to_string(),
        };
        packages.insert("cursed-json@2.1.0".to_string(), json_package);
        
        let http_package = MockPackage {
            name: "cursed-http".to_string(),
            version: "1.5.0".to_string(),
            description: "HTTP client and server for CURSED".to_string(),
            content: b"fake http package content".to_vec(),
            checksum: "b5d4045c3f466fa91fe2cc6abe79232a1a57cdf104f7a26e716e0a1e2789df78".to_string(),
        };
        packages.insert("cursed-http@1.5.0".to_string(), http_package);
        
        Self {
            packages: Arc::new(Mutex::new(packages)),
            port,
        }
    }
    
    pub fn url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }
    
    /// Start the mock server (this is a simplified version for testing)
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        // In a real implementation, this would start an HTTP server
        // For now, we'll just simulate the responses in the test
        println!("Mock registry server would start on port {}", self.port);
        Ok(())
    }
    
    /// Get package metadata (simulated)
    pub async fn get_package_metadata(&self, name: &str, version: &str) -> Option<MockPackage> {
        let packages = self.packages.lock().await;
        let key = format!("{}@{}", name, version);
        packages.get(&key).cloned()
    }
    
    /// Search packages (simulated)
    pub async fn search_packages(&self, query: &str) -> Vec<MockPackage> {
        let packages = self.packages.lock().await;
        packages.values()
            .filter(|pkg| {
                pkg.name.contains(query) || 
                pkg.description.to_lowercase().contains(&query.to_lowercase())
            })
            .cloned()
            .collect()
    }
    
    /// List all packages (simulated)
    pub async fn list_all_packages(&self) -> Vec<MockPackage> {
        let packages = self.packages.lock().await;
        packages.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_server_creation() {
        let server = MockRegistryServer::new(8080);
        assert_eq!(server.port, 8080);
        assert_eq!(server.url(), "http://localhost:8080");
    }

    #[tokio::test]
    async fn test_mock_package_retrieval() {
        let server = MockRegistryServer::new(8081);
        
        let package = server.get_package_metadata("test-package", "1.0.0").await;
        assert!(package.is_some());
        
        let pkg = package.unwrap();
        assert_eq!(pkg.name, "test-package");
        assert_eq!(pkg.version, "1.0.0");
        assert_eq!(pkg.description, "A test package for CURSED");
    }

    #[tokio::test]
    async fn test_mock_package_search() {
        let server = MockRegistryServer::new(8082);
        
        let results = server.search_packages("json").await;
        assert!(!results.is_empty());
        assert!(results.iter().any(|pkg| pkg.name.contains("json")));
        
        let results = server.search_packages("http").await;
        assert!(!results.is_empty());
        assert!(results.iter().any(|pkg| pkg.name.contains("http")));
        
        let results = server.search_packages("nonexistent").await;
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_mock_list_all_packages() {
        let server = MockRegistryServer::new(8083);
        
        let packages = server.list_all_packages().await;
        assert_eq!(packages.len(), 3); // test-package, cursed-json, cursed-http
        
        let names: Vec<String> = packages.iter().map(|p| p.name.clone()).collect();
        assert!(names.contains(&"test-package".to_string()));
        assert!(names.contains(&"cursed-json".to_string()));
        assert!(names.contains(&"cursed-http".to_string()));
    }
}
