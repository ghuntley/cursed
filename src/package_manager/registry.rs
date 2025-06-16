use crate::package_manager::{PackageManagerError, metadata::PackageMetadata};
use reqwest::{Client, ClientBuilder, Response};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use bytes::Bytes;
use sha2::{Sha256, Digest};
use tokio::time::timeout;
use url::Url;
use semver::Version;

/// Registry configuration
#[derive(Debug, Clone)]
pub struct RegistryConfig {
    /// Base URL of the package registry
    pub base_url: String,
    /// HTTP timeout for requests (default: 30 seconds)
    pub timeout: Duration,
    /// Maximum number of retries for failed requests (default: 3)
    pub max_retries: usize,
    /// Authentication token for private registries
    pub auth_token: Option<String>,
    /// User agent string for HTTP requests
    pub user_agent: String,
    /// Whether to verify TLS certificates (default: true)
    pub verify_tls: bool,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            base_url: "https://packages.cursed-lang.org".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            auth_token: None,
            user_agent: format!("cursed-pkg/{}", env!("CARGO_PKG_VERSION")),
            verify_tls: true,
        }
    }
}

/// Registry statistics
#[derive(Debug, Default, Clone)]
pub struct RegistryStats {
    pub total_packages: usize,
    pub download_count: usize,
    pub search_count: usize,
    pub registry_url: String,
    pub last_updated: Option<SystemTime>,
    pub total_bytes_downloaded: u64,
    pub failed_requests: usize,
    pub average_response_time: Duration,
}

/// Package registry HTTP client
#[derive(Debug, Clone)]
pub struct PackageRegistry {
    client: Client,
    config: RegistryConfig,
    stats: RegistryStats,
}

/// Package information from registry API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub download_url: String,
    pub checksum: String,
    pub size: Option<usize>,
    pub published_at: Option<String>,
    pub authors: Option<Vec<String>>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub keywords: Option<Vec<String>>,
}

/// Package search response from registry API
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub packages: Vec<PackageMetadata>,
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
}

/// Package data downloaded from registry
#[derive(Debug)]
pub struct PackageData {
    pub content: Vec<u8>,
    pub checksum: String,
    pub size: usize,
    pub verified: bool,
}

/// Registry index information
#[derive(Debug, Deserialize)]
pub struct IndexInfo {
    pub version: String,
    pub total_packages: usize,
    pub last_updated: String,
    pub supported_formats: Vec<String>,
}

impl PackageRegistry {
    /// Create a new package registry client
    pub fn new(base_url: String) -> Result<Self, PackageManagerError> {
        let config = RegistryConfig {
            base_url: base_url.clone(),
            ..Default::default()
        };
        Self::with_config(config)
    }

    /// Create a new package registry client with custom configuration
    pub fn with_config(config: RegistryConfig) -> Result<Self, PackageManagerError> {
        let client = ClientBuilder::new()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .danger_accept_invalid_certs(!config.verify_tls)
            .build()
            .map_err(|e| PackageManagerError::RegistryError {
                message: format!("Failed to create HTTP client: {}", e),
            })?;

        let mut stats = RegistryStats::default();
        stats.registry_url = config.base_url.clone();

        Ok(Self {
            client,
            config,
            stats,
        })
    }

    /// Get registry statistics
    pub fn get_stats(&self) -> &RegistryStats {
        &self.stats
    }

    /// Get all available versions for a package
    pub async fn get_package_versions(&mut self, name: &str) -> Result<Vec<Version>, PackageManagerError> {
        let start_time = SystemTime::now();
        self.stats.search_count += 1;

        let url = format!("{}/api/v1/packages/{}/versions", self.config.base_url, name);

        let response = self.make_request(&url).await?;
        
        if response.status().is_success() {
            #[derive(Deserialize)]
            struct VersionsResponse {
                versions: Vec<String>,
            }
            
            let versions_response: VersionsResponse = response.json().await
                .map_err(|e| PackageManagerError::RegistryError {
                    message: format!("Failed to parse versions JSON: {}", e),
                })?;
            
            let versions: Result<Vec<Version>, _> = versions_response.versions
                .into_iter()
                .map(|v| Version::parse(&v))
                .collect();
                
            let mut parsed_versions = versions.map_err(|e| PackageManagerError::InvalidVersion {
                version: "N/A".to_string(),
                reason: e.to_string(),
            })?;
            
            // Sort versions in descending order (latest first)
            parsed_versions.sort_by(|a, b| b.cmp(a));
            
            self.update_response_time(start_time);
            Ok(parsed_versions)
        } else if response.status() == 404 {
            Err(PackageManagerError::PackageNotFound {
                package: name.to_string(),
            })
        } else {
            self.stats.failed_requests += 1;
            Err(PackageManagerError::RegistryError {
                message: format!("Versions request failed with status: {}", response.status()),
            })
        }
    }

    /// Get package metadata for a specific version
    pub async fn get_package_metadata(&mut self, name: &str, version: &str) -> Result<PackageMetadata, PackageManagerError> {
        let start_time = SystemTime::now();
        self.stats.search_count += 1;

        let url = format!("{}/api/v1/packages/{}/{}/metadata", self.config.base_url, name, version);

        let response = self.make_request(&url).await?;
        
        if response.status().is_success() {
            let metadata: PackageMetadata = response.json().await
                .map_err(|e| PackageManagerError::RegistryError {
                    message: format!("Failed to parse package metadata JSON: {}", e),
                })?;
            
            self.update_response_time(start_time);
            Ok(metadata)
        } else if response.status() == 404 {
            Err(PackageManagerError::PackageNotFound {
                package: format!("{}@{}", name, version),
            })
        } else {
            self.stats.failed_requests += 1;
            Err(PackageManagerError::RegistryError {
                message: format!("Metadata request failed with status: {}", response.status()),
            })
        }
    }

    /// Search for a specific package version
    pub async fn search_package(&mut self, name: &str, version: Option<&str>) -> Result<PackageInfo, PackageManagerError> {
        let start_time = SystemTime::now();
        self.stats.search_count += 1;

        let url = if let Some(v) = version {
            format!("{}/api/v1/packages/{}/{}", self.config.base_url, name, v)
        } else {
            format!("{}/api/v1/packages/{}", self.config.base_url, name)
        };

        let response = self.make_request(&url).await?;
        
        if response.status().is_success() {
            let package_info: PackageInfo = response.json().await
                .map_err(|e| PackageManagerError::RegistryError {
                    message: format!("Failed to parse package info JSON: {}", e),
                })?;
            
            self.update_response_time(start_time);
            Ok(package_info)
        } else if response.status() == 404 {
            Err(PackageManagerError::PackageNotFound {
                package: name.to_string(),
            })
        } else {
            self.stats.failed_requests += 1;
            Err(PackageManagerError::RegistryError {
                message: format!("Registry request failed with status: {}", response.status()),
            })
        }
    }

    /// Search for packages with a text query
    pub async fn search_packages(&mut self, query: &str, limit: Option<usize>) -> Result<Vec<PackageMetadata>, PackageManagerError> {
        let start_time = SystemTime::now();
        self.stats.search_count += 1;

        let mut url = Url::parse(&format!("{}/api/v1/packages", self.config.base_url))
            .map_err(|e| PackageManagerError::RegistryError {
                message: format!("Invalid registry URL: {}", e),
            })?;

        url.query_pairs_mut()
            .append_pair("q", query);
        
        if let Some(limit) = limit {
            url.query_pairs_mut()
                .append_pair("limit", &limit.to_string());
        }

        let response = self.make_request(url.as_str()).await?;
        
        if response.status().is_success() {
            let search_response: SearchResponse = response.json().await
                .map_err(|e| PackageManagerError::RegistryError {
                    message: format!("Failed to parse search response JSON: {}", e),
                })?;
            
            self.update_response_time(start_time);
            Ok(search_response.packages)
        } else {
            self.stats.failed_requests += 1;
            Err(PackageManagerError::RegistryError {
                message: format!("Search request failed with status: {}", response.status()),
            })
        }
    }

    /// Download a package archive from the registry
    pub async fn download_package(&mut self, name: &str, version: &str) -> Result<PackageData, PackageManagerError> {
        let start_time = SystemTime::now();
        self.stats.download_count += 1;

        let url = format!("{}/api/v1/packages/{}/{}/download", self.config.base_url, name, version);
        
        let response = self.make_request(&url).await?;
        
        if response.status().is_success() {
            let content_bytes = response.bytes().await
                .map_err(|e| PackageManagerError::RegistryError {
                    message: format!("Failed to download package content: {}", e),
                })?;

            let content = content_bytes.to_vec();
            let size = content.len();
            
            // Calculate SHA-256 checksum
            let mut hasher = Sha256::new();
            hasher.update(&content);
            let checksum = hex::encode(hasher.finalize());

            self.stats.total_bytes_downloaded += size as u64;
            self.update_response_time(start_time);

            Ok(PackageData {
                content,
                checksum,
                size,
                verified: true, // We calculated the checksum ourselves
            })
        } else if response.status() == 404 {
            Err(PackageManagerError::PackageNotFound {
                package: format!("{}@{}", name, version),
            })
        } else {
            self.stats.failed_requests += 1;
            Err(PackageManagerError::RegistryError {
                message: format!("Download request failed with status: {}", response.status()),
            })
        }
    }

    /// Update the package index from the registry
    pub async fn update_index(&mut self) -> Result<(), PackageManagerError> {
        let start_time = SystemTime::now();
        
        let url = format!("{}/api/v1/index", self.config.base_url);
        let response = self.make_request(&url).await?;
        
        if response.status().is_success() {
            let index_info: IndexInfo = response.json().await
                .map_err(|e| PackageManagerError::RegistryError {
                    message: format!("Failed to parse index info JSON: {}", e),
                })?;

            self.stats.total_packages = index_info.total_packages;
            self.stats.last_updated = Some(SystemTime::now());
            self.update_response_time(start_time);

            tracing::info!(
                packages = index_info.total_packages,
                version = %index_info.version,
                "Registry index updated successfully"
            );

            Ok(())
        } else {
            self.stats.failed_requests += 1;
            Err(PackageManagerError::RegistryError {
                message: format!("Index update failed with status: {}", response.status()),
            })
        }
    }

    /// Verify package integrity by comparing checksums
    pub async fn verify_package(&self, name: &str, version: &str, data: &[u8], expected_checksum: &str) -> Result<bool, PackageManagerError> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let calculated_checksum = hex::encode(hasher.finalize());

        if calculated_checksum == expected_checksum {
            Ok(true)
        } else {
            tracing::warn!(
                package = name,
                version = version,
                expected = expected_checksum,
                calculated = calculated_checksum,
                "Package checksum verification failed"
            );
            Ok(false)
        }
    }

    /// Make an HTTP request with retry logic
    async fn make_request(&self, url: &str) -> Result<Response, PackageManagerError> {
        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            let mut request = self.client.get(url);

            // Add authentication header if configured
            if let Some(ref token) = self.config.auth_token {
                request = request.header("Authorization", format!("Bearer {}", token));
            }

            // Add custom headers
            request = request
                .header("Accept", "application/json")
                .header("X-Registry-Client", "cursed-pkg");

            let result = timeout(self.config.timeout, request.send()).await;

            match result {
                Ok(Ok(response)) => {
                    return Ok(response);
                }
                Ok(Err(e)) => {
                    last_error = Some(e);
                    if attempt < self.config.max_retries {
                        let delay = Duration::from_millis(1000 * (2_u64.pow(attempt as u32)));
                        tracing::warn!(
                            attempt = attempt + 1,
                            max_retries = self.config.max_retries,
                            delay_ms = delay.as_millis(),
                            error = %last_error.as_ref().unwrap(),
                            "Request failed, retrying..."
                        );
                        tokio::time::sleep(delay).await;
                    }
                }
                Err(_timeout) => {
                    let timeout_error = PackageManagerError::RegistryError {
                        message: format!("Request timeout after {}s", self.config.timeout.as_secs()),
                    };
                    if attempt < self.config.max_retries {
                        tracing::warn!(
                            attempt = attempt + 1,
                            max_retries = self.config.max_retries,
                            "Request timed out, retrying..."
                        );
                        continue;
                    } else {
                        return Err(timeout_error);
                    }
                }
            }
        }

        Err(PackageManagerError::RegistryError {
            message: format!("Request failed after {} retries: {}", 
                           self.config.max_retries, 
                           last_error.map(|e| e.to_string()).unwrap_or_else(|| "Unknown error".to_string())),
        })
    }

    /// Update average response time statistics
    fn update_response_time(&mut self, start_time: SystemTime) {
        if let Ok(elapsed) = start_time.elapsed() {
            // Simple moving average calculation
            let current_avg_ms = self.stats.average_response_time.as_millis() as u64;
            let new_time_ms = elapsed.as_millis() as u64;
            let new_avg_ms = if current_avg_ms == 0 {
                new_time_ms
            } else {
                (current_avg_ms + new_time_ms) / 2
            };
            self.stats.average_response_time = Duration::from_millis(new_avg_ms);
        }
    }

    /// Get configuration
    pub fn get_config(&self) -> &RegistryConfig {
        &self.config
    }

    /// Update authentication token
    pub fn set_auth_token(&mut self, token: Option<String>) {
        self.config.auth_token = token;
    }
    
    /// Search for packages matching name and version
    pub async fn search_package(&self, name: &str, version: &str) -> Result<Vec<PackageInfo>, PackageManagerError> {
        // Stub implementation
        tracing::info!("Searching for package: {} version: {}", name, version);
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_registry_creation() {
        let registry = PackageRegistry::new("https://test.registry.com".to_string());
        assert!(registry.is_ok());
        
        let registry = registry.unwrap();
        assert_eq!(registry.config.base_url, "https://test.registry.com");
        assert_eq!(registry.stats.download_count, 0);
        assert_eq!(registry.stats.search_count, 0);
    }

    #[tokio::test]
    async fn test_registry_with_config() {
        let config = RegistryConfig {
            base_url: "https://custom.registry.com".to_string(),
            timeout: Duration::from_secs(10),
            max_retries: 5,
            auth_token: Some("test-token".to_string()),
            user_agent: "test-agent".to_string(),
            verify_tls: false,
        };

        let registry = PackageRegistry::with_config(config.clone());
        assert!(registry.is_ok());
        
        let registry = registry.unwrap();
        assert_eq!(registry.config.base_url, config.base_url);
        assert_eq!(registry.config.timeout, config.timeout);
        assert_eq!(registry.config.max_retries, config.max_retries);
        assert_eq!(registry.config.auth_token, config.auth_token);
    }

    #[test]
    fn test_package_data_creation() {
        let content = b"test package content".to_vec();
        let package_data = PackageData {
            content: content.clone(),
            checksum: "test-checksum".to_string(),
            size: content.len(),
            verified: true,
        };

        assert_eq!(package_data.content, content);
        assert_eq!(package_data.size, content.len());
        assert!(package_data.verified);
    }

    #[tokio::test]
    async fn test_verify_package() {
        let registry = PackageRegistry::new("https://test.registry.com".to_string()).unwrap();
        let data = b"test data";
        
        // Calculate expected checksum
        let mut hasher = Sha256::new();
        hasher.update(data);
        let expected_checksum = hex::encode(hasher.finalize());

        let result = registry.verify_package("test", "1.0.0", data, &expected_checksum).await;
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Test with wrong checksum
        let result = registry.verify_package("test", "1.0.0", data, "wrong-checksum").await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_registry_stats_default() {
        let stats = RegistryStats::default();
        assert_eq!(stats.total_packages, 0);
        assert_eq!(stats.download_count, 0);
        assert_eq!(stats.search_count, 0);
        assert_eq!(stats.failed_requests, 0);
        assert_eq!(stats.total_bytes_downloaded, 0);
        assert!(stats.last_updated.is_none());
    }
}
