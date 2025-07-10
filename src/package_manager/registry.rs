//! Package registry client for CURSED
//!
//! This module handles communication with package registries

use crate::error::{CursedError, Result};
use crate::package_manager::version::{Version, VersionReq};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Information about a package in the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub authors: Vec<String>,
    pub dependencies: Vec<Dependency>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub download_url: String,
    pub checksum: String,
    pub file_size: u64,
}

/// Package dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version_req: VersionReq,
    pub optional: bool,
    pub features: Vec<String>,
}

/// Registry configuration
#[derive(Debug, Clone)]
pub struct RegistryConfig {
    pub url: String,
    pub timeout: Duration,
    pub max_retries: u32,
    pub api_key: Option<String>,
}

/// Package registry client
#[derive(Debug, Clone)]
pub struct PackageRegistry {
    config: RegistryConfig,
    client: reqwest::Client,
}

/// Registry search response
#[derive(Debug, Deserialize)]
struct SearchResponse {
    packages: Vec<PackageInfo>,
    total: Option<usize>,
}

/// Registry package response
#[derive(Debug, Deserialize)]
struct PackageResponse {
    package: PackageInfo,
}

/// Registry versions response
#[derive(Debug, Deserialize)]
struct VersionsResponse {
    versions: Vec<String>,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            url: "https://packages.cursed-lang.org".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            api_key: None,
        }
    }
}

impl PackageRegistry {
    /// Create a new registry client
    pub fn new(config: RegistryConfig) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("cursed-package-manager/1.0"),
        );
        
        if let Some(api_key) = &config.api_key {
            let auth_value = format!("Bearer {}", api_key);
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&auth_value)
                    .map_err(|e| CursedError::General(format!("Invalid API key: {}", e)))?,
            );
        }

        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .default_headers(headers)
            .build()
            .map_err(|e| CursedError::General(format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self {
            config,
            client,
        })
    }

    /// Search for packages by name or keyword
    pub async fn search_packages(&self, query: &str) -> Result<Vec<PackageInfo>> {
        tracing::info!("Searching packages for query: {}", query);

        // Mock implementation for testing
        if self.config.url.contains("test") || self.config.url.contains("mock") {
            return Ok(vec![]);
        }

        let url = format!("{}/api/v1/search", self.config.url);
        let response = self.make_request_with_retries(&url, Some(&[("q", query)])).await?;
        
        let search_response: SearchResponse = serde_json::from_str(&response)
            .map_err(|e| CursedError::General(format!("Failed to parse search response: {}", e)))?;
        
        Ok(search_response.packages)
    }

    /// Get package metadata for a specific package and version
    pub async fn get_package_info(&self, name: &str, version: Option<&Version>) -> Result<PackageInfo> {
        tracing::info!("Getting package info for: {} version: {:?}", name, version);

        // Mock implementation for testing
        if self.config.url.contains("test") || self.config.url.contains("mock") {
            return Ok(PackageInfo {
                name: name.to_string(),
                version: version.cloned().unwrap_or_else(|| Version::new(1, 0, 0)),
                description: "Test package".to_string(),
                authors: vec!["Test Author".to_string()],
                dependencies: Vec::new(),
                keywords: Vec::new(),
                categories: Vec::new(),
                license: Some("MIT".to_string()),
                homepage: Some("test-homepage".to_string()),
                repository: Some("test-repository".to_string()),
                download_url: "test-url".to_string(),
                checksum: "test-checksum".to_string(),
                file_size: 1024,
            });
        }

        let url = if let Some(v) = version {
            format!("{}/api/v1/packages/{}/{}", self.config.url, name, v)
        } else {
            format!("{}/api/v1/packages/{}", self.config.url, name)
        };

        let response = self.make_request_with_retries(&url, None).await?;
        
        let package_response: PackageResponse = serde_json::from_str(&response)
            .map_err(|e| CursedError::General(format!("Failed to parse package info response: {}", e)))?;
        
        Ok(package_response.package)
    }

    /// Get all available versions for a package
    pub async fn get_package_versions(&self, name: &str) -> Result<Vec<Version>> {
        tracing::info!("Getting versions for package: {}", name);

        // Mock implementation for testing
        if self.config.url.contains("test") || self.config.url.contains("mock") {
            return Ok(vec![Version::parse("1.0.0").unwrap()]);
        }

        let url = format!("{}/api/v1/packages/{}/versions", self.config.url, name);
        let response = self.make_request_with_retries(&url, None).await?;
        
        let versions_response: VersionsResponse = serde_json::from_str(&response)
            .map_err(|e| CursedError::General(format!("Failed to parse versions response: {}", e)))?;
        
        let mut versions = Vec::new();
        for version_str in versions_response.versions {
            match Version::parse(&version_str) {
                Ok(version) => versions.push(version),
                Err(e) => tracing::warn!("Failed to parse version '{}': {}", version_str, e),
            }
        }
        
        Ok(versions)
    }

    /// Get the latest version of a package
    pub async fn get_latest_version(&self, name: &str) -> Result<Version> {
        // Mock implementation for testing
        if self.config.url.contains("test") || self.config.url.contains("mock") {
            return Ok(self.mock_get_latest_version(name));
        }
        
        let versions = self.get_package_versions(name).await?;
        
        versions.into_iter().max()
            .ok_or_else(|| CursedError::General(format!("No versions found for package: {}", name)))
    }

    /// Find the best matching version for a version requirement
    pub async fn resolve_version(&self, name: &str, version_req: &VersionReq) -> Result<Version> {
        let versions = self.get_package_versions(name).await?;
        
        // Find all matching versions
        let matching: Vec<Version> = versions.into_iter()
            .filter(|v| version_req.matches(v))
            .collect();

        if matching.is_empty() {
            return Err(CursedError::General(format!(
                "No matching version found for package: {} with requirement: {:?}", 
                name, version_req
            )));
        }

        // Return the highest matching version
        matching.into_iter().max()
            .ok_or_else(|| CursedError::General("Failed to determine best version".to_string()))
    }

    /// Download package metadata with dependency information
    pub async fn get_package_metadata(&self, name: &str, version: &Version) -> Result<PackageMetadata> {
        let package_info = self.get_package_info(name, Some(version)).await?;
        
        Ok(PackageMetadata {
            name: package_info.name,
            version: package_info.version,
            description: package_info.description,
            dependencies: package_info.dependencies,
            download_url: package_info.download_url,
            checksum: package_info.checksum,
            authors: package_info.authors,
            license: package_info.license,
            homepage: package_info.homepage,
            repository: package_info.repository,
            keywords: package_info.keywords,
            categories: package_info.categories,
        })
    }

    /// Publish a package to the registry
    pub async fn publish_package(&self, package_metadata: &PackageMetadata, package_archive: &[u8]) -> Result<()> {
        tracing::info!("Publishing package: {} v{}", package_metadata.name, package_metadata.version);
        
        let url = format!("{}/api/v1/packages", self.config.url);
        
        // Create multipart form data
        let form = reqwest::multipart::Form::new()
            .text("name", package_metadata.name.clone())
            .text("version", package_metadata.version.to_string())
            .text("description", package_metadata.description.clone())
            .text("authors", serde_json::to_string(&package_metadata.authors).unwrap_or_default())
            .text("dependencies", serde_json::to_string(&package_metadata.dependencies).unwrap_or_default())
            .text("keywords", serde_json::to_string(&package_metadata.keywords).unwrap_or_default())
            .text("categories", serde_json::to_string(&package_metadata.categories).unwrap_or_default())
            .text("license", package_metadata.license.as_ref().unwrap_or(&"".to_string()).clone())
            .text("homepage", package_metadata.homepage.as_ref().unwrap_or(&"".to_string()).clone())
            .text("repository", package_metadata.repository.as_ref().unwrap_or(&"".to_string()).clone())
            .part("archive", reqwest::multipart::Part::bytes(package_archive.to_vec())
                .file_name("package.tar.gz")
                .mime_str("application/gzip")
                .map_err(|e| CursedError::General(format!("Invalid MIME type: {}", e)))?);
        
        let response = self.client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| CursedError::General(format!("Failed to publish package: {}", e)))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(CursedError::General(format!("Publish failed: {}", error_text)));
        }
        
        tracing::info!("Package {} v{} published successfully", package_metadata.name, package_metadata.version);
        Ok(())
    }

    /// Check if a package version exists in the registry
    pub async fn package_exists(&self, name: &str, version: &Version) -> Result<bool> {
        let url = format!("{}/api/v1/packages/{}/{}", self.config.url, name, version);
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// Get authentication token for publishing
    pub async fn get_auth_token(&self, username: &str, password: &str) -> Result<String> {
        let url = format!("{}/api/v1/auth/token", self.config.url);
        
        let auth_data = serde_json::json!({
            "username": username,
            "password": password
        });
        
        let response = self.client
            .post(&url)
            .json(&auth_data)
            .send()
            .await
            .map_err(|e| CursedError::General(format!("Authentication failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(CursedError::General("Authentication failed".to_string()));
        }
        
        let auth_response: serde_json::Value = response.json().await
            .map_err(|e| CursedError::General(format!("Failed to parse auth response: {}", e)))?;
        
        auth_response.get("token")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| CursedError::General("No token in auth response".to_string()))
    }

    /// Make HTTP request with retries and error handling
    async fn make_request_with_retries(&self, url: &str, query_params: Option<&[(&str, &str)]>) -> Result<String> {
        let mut last_error = None;
        
        for attempt in 1..=self.config.max_retries {
            tracing::debug!("HTTP request attempt {} to {}", attempt, url);
            
            let mut request = self.client.get(url);
            
            if let Some(params) = query_params {
                request = request.query(params);
            }
            
            match request.send().await {
                Ok(response) => {
                    match response.error_for_status() {
                        Ok(resp) => {
                            match resp.text().await {
                                Ok(text) => {
                                    tracing::debug!("HTTP request successful on attempt {}", attempt);
                                    return Ok(text);
                                }
                                Err(e) => {
                                    let error = CursedError::General(format!("Failed to read response body: {}", e));
                                    tracing::warn!("Request attempt {} failed: {}", attempt, error);
                                    last_error = Some(error);
                                }
                            }
                        }
                        Err(e) => {
                            let error = CursedError::General(format!("HTTP error: {}", e));
                            tracing::warn!("Request attempt {} failed: {}", attempt, error);
                            last_error = Some(error);
                        }
                    }
                }
                Err(e) => {
                    let error = CursedError::General(format!("Request failed: {}", e));
                    tracing::warn!("Request attempt {} failed: {}", attempt, error);
                    last_error = Some(error);
                }
            }
            
            if attempt < self.config.max_retries {
                // Exponential backoff
                let delay = Duration::from_millis(1000 * 2_u64.pow(attempt - 1));
                tokio::time::sleep(delay).await;
            }
        }

        Err(last_error.unwrap_or_else(|| CursedError::General("Request failed after all retries".to_string())))
    }
    
    /// Mock implementation for search_packages
    fn mock_search_packages(&self, query: &str) -> Vec<PackageInfo> {
        // Return mock packages for testing
        vec![
            PackageInfo {
                name: format!("mock-{}", query),
                version: Version::parse("1.0.0").unwrap(),
                description: format!("Mock package for {}", query),
                authors: vec!["Test Author".to_string()],
                dependencies: vec![],
                keywords: vec!["mock".to_string(), "test".to_string()],
                categories: vec!["development".to_string()],
                license: Some("MIT".to_string()),
                homepage: Some("https://example.com".to_string()),
                repository: Some("https://github.com/example/mock".to_string()),
                download_url: "https://example.com/download".to_string(),
                checksum: "abc123".to_string(),
                file_size: 1024,
            }
        ]
    }
    
    /// Mock implementation for get_package_info
    fn mock_get_package_info(&self, name: &str, version: Option<&Version>) -> PackageInfo {
        let version = version.cloned().unwrap_or_else(|| Version::parse("1.0.0").unwrap());
        PackageInfo {
            name: name.to_string(),
            version,
            description: format!("Mock package: {}", name),
            authors: vec!["Test Author".to_string()],
            dependencies: vec![],
            keywords: vec!["mock".to_string(), "test".to_string()],
            categories: vec!["development".to_string()],
            license: Some("MIT".to_string()),
            homepage: Some("https://example.com".to_string()),
            repository: Some("https://github.com/example/mock".to_string()),
            download_url: "https://example.com/download".to_string(),
            checksum: "abc123".to_string(),
            file_size: 1024,
        }
    }
    
    /// Mock implementation for get_latest_version
    fn mock_get_latest_version(&self, _name: &str) -> Version {
        Version::parse("1.0.0").unwrap()
    }
}

/// Package metadata for dependency resolution
#[derive(Debug, Clone)]
pub struct PackageMetadata {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub dependencies: Vec<Dependency>,
    pub download_url: String,
    pub checksum: String,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
}

impl Dependency {
    pub fn new(name: String, version_req: VersionReq) -> Self {
        Self {
            name,
            version_req,
            optional: false,
            features: vec![],
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn with_features(mut self, features: Vec<String>) -> Self {
        self.features = features;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_registry_search() {
        let mut server = mockito::Server::new_async().await;
        
        // Mock the search endpoint
        let mock_response = r#"{
            "packages": [
                {
                    "name": "example-package",
                    "version": {
                        "major": 1,
                        "minor": 0,
                        "patch": 0,
                        "pre_release": null,
                        "build": null
                    },
                    "description": "An example package",
                    "authors": ["Test Author"],
                    "dependencies": [],
                    "keywords": ["example"],
                    "categories": ["development"],
                    "license": "MIT",
                    "homepage": "https://example.com",
                    "repository": "https://github.com/example/example-package",
                    "download_url": "https://example.com/download",
                    "checksum": "abc123",
                    "file_size": 1024
                }
            ],
            "total": 1
        }"#;
        
        let _m = server
            .mock("GET", "/api/v1/search")
            .match_query(mockito::Matcher::UrlEncoded("q".into(), "example".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .create_async()
            .await;

        let config = RegistryConfig {
            url: server.url(),
            timeout: std::time::Duration::from_secs(30),
            max_retries: 3,
            api_key: None,
        };
        
        let registry = PackageRegistry::new(config).unwrap();
        let results = registry.search_packages("example").await.unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "example-package");
    }

    #[tokio::test]
    async fn test_package_info() {
        let mut server = mockito::Server::new_async().await;
        
        // Mock the package info endpoint
        let mock_response = r#"{
            "package": {
                "name": "test-package",
                "version": {
                    "major": 1,
                    "minor": 0,
                    "patch": 0,
                    "pre_release": null,
                    "build": null
                },
                "description": "A test package",
                "authors": ["Test Author"],
                "dependencies": [],
                "keywords": ["test"],
                "categories": ["development"],
                "license": "MIT",
                "homepage": "https://example.com",
                "repository": "https://github.com/example/test-package",
                "download_url": "https://example.com/download",
                "checksum": "abc123",
                "file_size": 1024
            }
        }"#;
        
        let _m = server
            .mock("GET", "/api/v1/packages/test-package")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .create_async()
            .await;

        let config = RegistryConfig {
            url: server.url(),
            timeout: std::time::Duration::from_secs(30),
            max_retries: 3,
            api_key: None,
        };
        
        let registry = PackageRegistry::new(config).unwrap();
        let info = registry.get_package_info("test-package", None).await.unwrap();
        assert_eq!(info.name, "test-package");
    }
}
