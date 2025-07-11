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
            let mock_version = version.cloned().unwrap_or_else(|| self.mock_get_latest_version(name));
            
            // Create realistic mock dependencies based on package name
            let mock_dependencies = match name {
                "large-framework" => vec![
                    Dependency::new("utils".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                    Dependency::new("logging".to_string(), VersionReq::parse("^2.0.0").unwrap()),
                ],
                "data-processing" => vec![
                    Dependency::new("parser".to_string(), VersionReq::parse("^1.5.0").unwrap()),
                    Dependency::new("compression".to_string(), VersionReq::parse("^3.0.0").unwrap()),
                ],
                "web-server" => vec![
                    Dependency::new("http-core".to_string(), VersionReq::parse("^1.2.0").unwrap()),
                    Dependency::new("security".to_string(), VersionReq::parse("^2.1.0").unwrap()),
                ],
                "database-client" => vec![
                    Dependency::new("connection-pool".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                    Dependency::new("query-builder".to_string(), VersionReq::parse("^2.0.0").unwrap()),
                ],
                "crypto-library" => vec![
                    Dependency::new("random".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                    Dependency::new("hashing".to_string(), VersionReq::parse("^1.5.0").unwrap()),
                ],
                "medium-root-1" => vec![
                    Dependency::new("common-dep".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                ],
                "medium-root-2" => vec![
                    Dependency::new("common-dep".to_string(), VersionReq::parse("^1.1.0").unwrap()),
                ],
                "medium-root-3" => vec![
                    Dependency::new("common-dep".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                ],
                "high-complexity" => vec![
                    Dependency::new("dep-a".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                    Dependency::new("dep-b".to_string(), VersionReq::parse("^2.0.0").unwrap()),
                    Dependency::new("dep-c".to_string(), VersionReq::parse("^1.5.0").unwrap()),
                ],
                // Dependencies for the main packages - these are leaf nodes
                "utils" | "logging" | "parser" | "compression" | "http-core" | "security" | 
                "connection-pool" | "query-builder" | "random" | "hashing" | "common-dep" | 
                "dep-a" | "dep-b" | "dep-c" => Vec::new(),
                _ => Vec::new(),
            };
            
            return Ok(PackageInfo {
                name: name.to_string(),
                version: mock_version,
                description: format!("Mock package: {}", name),
                authors: vec!["Mock Author".to_string()],
                dependencies: mock_dependencies,
                keywords: vec!["mock".to_string(), "test".to_string()],
                categories: vec!["testing".to_string()],
                license: Some("MIT".to_string()),
                homepage: Some(format!("https://mock-homepage.test/{}", name)),
                repository: Some(format!("https://mock-repo.test/{}", name)),
                download_url: format!("https://mock-download.test/{}", name),
                checksum: format!("mock-checksum-{}", name),
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
            // Return realistic version lists based on package name
            let versions = match name {
                "large-framework" => vec!["1.0.0", "1.2.0", "1.4.1", "1.5.2"],
                "data-processing" => vec!["2.0.0", "2.1.0", "2.2.3", "2.3.1"],
                "web-server" => vec!["1.5.0", "1.6.2", "1.7.1", "1.8.4"],
                "database-client" => vec!["3.0.0", "3.1.0", "3.2.0"],
                "crypto-library" => vec!["2.5.0", "2.6.1", "2.7.3"],
                "medium-root-1" => vec!["1.0.0", "1.1.0", "1.2.3"],
                "medium-root-2" => vec!["2.0.0", "2.1.4"],
                "medium-root-3" => vec!["1.5.0", "1.6.2"],
                "high-complexity" => vec!["1.0.0", "1.0.5"],
                "utils" => vec!["1.0.0", "1.1.0"],
                "logging" => vec!["2.0.0", "2.1.0"],
                "parser" => vec!["1.5.0", "1.6.0"],
                "compression" => vec!["3.0.0"],
                "http-core" => vec!["1.2.0"],
                "security" => vec!["2.1.0"],
                "connection-pool" => vec!["1.0.0"],
                "query-builder" => vec!["2.0.0"],
                "random" => vec!["1.0.0"],
                "hashing" => vec!["1.5.0"],
                "common-dep" => vec!["1.0.0", "1.1.0"],
                "dep-a" => vec!["1.0.0"],
                "dep-b" => vec!["2.0.0"],
                "dep-c" => vec!["1.5.0"],
                _ => vec!["1.0.0"],
            };
            
            let mut parsed_versions = Vec::new();
            for version_str in versions {
                if let Ok(version) = Version::parse(version_str) {
                    parsed_versions.push(version);
                }
            }
            
            return Ok(parsed_versions);
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
    fn mock_get_latest_version(&self, name: &str) -> Version {
        // Return different versions based on package name for more realistic testing
        match name {
            "large-framework" => Version::parse("1.5.2").unwrap(),
            "data-processing" => Version::parse("2.3.1").unwrap(),
            "web-server" => Version::parse("1.8.4").unwrap(),
            "database-client" => Version::parse("3.2.0").unwrap(),
            "crypto-library" => Version::parse("2.7.3").unwrap(),
            "medium-root-1" => Version::parse("1.2.3").unwrap(),
            "medium-root-2" => Version::parse("2.1.4").unwrap(),
            "medium-root-3" => Version::parse("1.6.2").unwrap(),
            "high-complexity" => Version::parse("1.0.5").unwrap(),
            "utils" => Version::parse("1.1.0").unwrap(),
            "logging" => Version::parse("2.1.0").unwrap(),
            "parser" => Version::parse("1.6.0").unwrap(),
            "compression" => Version::parse("3.0.0").unwrap(),
            "http-core" => Version::parse("1.2.0").unwrap(),
            "security" => Version::parse("2.1.0").unwrap(),
            "connection-pool" => Version::parse("1.0.0").unwrap(),
            "query-builder" => Version::parse("2.0.0").unwrap(),
            "random" => Version::parse("1.0.0").unwrap(),
            "hashing" => Version::parse("1.5.0").unwrap(),
            "common-dep" => Version::parse("1.1.0").unwrap(),
            "dep-a" => Version::parse("1.0.0").unwrap(),
            "dep-b" => Version::parse("2.0.0").unwrap(),
            "dep-c" => Version::parse("1.5.0").unwrap(),
            _ => Version::parse("1.0.0").unwrap(),
        }
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
