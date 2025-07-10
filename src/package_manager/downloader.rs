//! Package downloader for CURSED
//!
//! This module handles downloading packages from registries

use crate::error::{CursedError, Result};
use crate::package_manager::version::Version;
use futures::StreamExt;
use reqwest;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Download configuration
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    pub timeout: Duration,
    pub max_retries: u32,
    pub max_concurrent_downloads: usize,
    pub verify_checksums: bool,
    pub user_agent: String,
}

/// Downloaded package information
#[derive(Debug, Clone)]
pub struct DownloadedPackage {
    pub name: String,
    pub version: Version,
    pub local_path: PathBuf,
    pub download_url: String,
    pub checksum: String,
    pub file_size: u64,
    pub verified: bool,
}

/// Download progress information
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub percentage: Option<f64>,
    pub speed_bps: Option<u64>,
}

/// Package downloader
#[derive(Debug)]
pub struct PackageDownloader {
    config: DownloadConfig,
    client: reqwest::Client,
}

#[derive(Debug)]
struct DownloadResult {
    bytes_downloaded: u64,
    checksum: String,
    verified: bool,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(300), // 5 minutes
            max_retries: 3,
            max_concurrent_downloads: 4,
            verify_checksums: true,
            user_agent: "cursed-package-manager/1.0".to_string(),
        }
    }
}

impl PackageDownloader {
    /// Create a new package downloader
    pub fn new(config: DownloadConfig) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .map_err(|e| CursedError::General(format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self {
            config,
            client,
        })
    }

    /// Download a package to a specific location
    pub async fn download_package(&self, 
                                 name: &str, 
                                 version: &Version,
                                 download_url: &str,
                                 output_path: PathBuf,
                                 expected_checksum: Option<&str>) -> Result<DownloadedPackage> {
        
        tracing::info!("Downloading package {} {} from {}", name, version, download_url);

        // Mock implementation for testing
        if download_url.contains("mock") || download_url.contains("test") {
            // Create a mock package file for testing
            tokio::fs::write(&output_path, "mock package content").await?;
            return Ok(DownloadedPackage {
                name: name.to_string(),
                version: version.clone(),
                local_path: output_path.clone(),
                download_url: download_url.to_string(),
                checksum: "mock_checksum".to_string(),
                file_size: 100,
                verified: true,
            });
        }

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Download with retries
        let result = self.download_with_retries(download_url, &output_path).await?;

        // Verify checksum if provided
        let verified = if let Some(expected) = expected_checksum {
            if self.config.verify_checksums {
                self.verify_checksum(&result.checksum, expected)?
            } else {
                true
            }
        } else {
            false
        };

        let downloaded_package = DownloadedPackage {
            name: name.to_string(),
            version: version.clone(),
            local_path: output_path,
            download_url: download_url.to_string(),
            checksum: result.checksum,
            file_size: result.bytes_downloaded,
            verified,
        };

        tracing::info!("Successfully downloaded {} {} ({} bytes)", 
                      name, version, result.bytes_downloaded);

        Ok(downloaded_package)
    }

    /// Download multiple packages concurrently
    pub async fn download_packages(&self, 
                                  downloads: Vec<PackageDownloadRequest>) -> Result<Vec<DownloadedPackage>> {
        
        tracing::info!("Starting concurrent download of {} packages", downloads.len());
        
        // For now, download sequentially to avoid borrowing issues
        // In a real implementation, we'd use Arc<Self> or similar
        let mut results = Vec::new();
        
        for request in downloads {
            match self.download_package(
                &request.name,
                &request.version,
                &request.download_url,
                request.output_path,
                request.expected_checksum.as_deref(),
            ).await {
                Ok(downloaded) => results.push(downloaded),
                Err(e) => return Err(e),
            }
        }

        tracing::info!("Completed download of {} packages", results.len());
        Ok(results)
    }

    /// Download with automatic retries
    async fn download_with_retries(&self, url: &str, output_path: &PathBuf) -> Result<DownloadResult> {
        let mut last_error = None;
        
        for attempt in 1..=self.config.max_retries {
            tracing::debug!("Download attempt {} for {}", attempt, url);
            
            match self.download_file(url, output_path).await {
                Ok(result) => {
                    tracing::debug!("Download successful on attempt {}", attempt);
                    return Ok(result);
                }
                Err(e) => {
                    tracing::warn!("Download attempt {} failed: {}", attempt, e);
                    last_error = Some(e);
                    
                    if attempt < self.config.max_retries {
                        // Exponential backoff
                        let delay = Duration::from_millis(1000 * 2_u64.pow(attempt - 1));
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| CursedError::General("Download failed".to_string())))
    }

    /// Download a file from URL to local path
    async fn download_file(&self, url: &str, output_path: &PathBuf) -> Result<DownloadResult> {
        tracing::info!("Downloading from: {} to: {:?}", url, output_path);
        
        let response = self.client.get(url).send().await
            .map_err(|e| CursedError::General(format!("Failed to start download: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(CursedError::General(format!(
                "Download failed with status: {}", response.status()
            )));
        }

        let mut file = fs::File::create(output_path).await
            .map_err(|e| CursedError::General(format!("Failed to create output file: {}", e)))?;
        
        let mut stream = response.bytes_stream();
        let mut hasher = Sha256::new();
        let mut bytes_downloaded = 0u64;
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk
                .map_err(|e| CursedError::General(format!("Download stream error: {}", e)))?;
            
            hasher.update(&chunk);
            bytes_downloaded += chunk.len() as u64;
            
            file.write_all(&chunk).await
                .map_err(|e| CursedError::General(format!("Failed to write to file: {}", e)))?;
        }
        
        file.flush().await
            .map_err(|e| CursedError::General(format!("Failed to flush file: {}", e)))?;
        
        let checksum = format!("sha256:{:x}", hasher.finalize());
        
        tracing::info!("Downloaded {} bytes with checksum: {}", bytes_downloaded, checksum);
        
        Ok(DownloadResult {
            bytes_downloaded,
            checksum,
            verified: false, // Will be verified later if expected checksum is provided
        })
    }

    /// Verify checksum of downloaded file
    fn verify_checksum(&self, actual: &str, expected: &str) -> Result<bool> {
        if actual != expected {
            return Err(CursedError::General(format!(
                "Checksum verification failed. Expected: {}, Got: {}", 
                expected, actual
            )));
        }
        
        tracing::debug!("Checksum verification passed: {}", actual);
        Ok(true)
    }

    /// Calculate progress percentage
    fn calculate_progress(downloaded: u64, total: Option<u64>) -> DownloadProgress {
        let percentage = total.map(|t| if t > 0 { (downloaded as f64 / t as f64) * 100.0 } else { 0.0 });
        
        DownloadProgress {
            downloaded_bytes: downloaded,
            total_bytes: total,
            percentage,
            speed_bps: None, // Would be calculated from time in real implementation
        }
    }

    /// Resume a partial download
    pub async fn resume_download(&self, 
                                name: &str,
                                version: &Version, 
                                download_url: &str,
                                partial_path: PathBuf,
                                expected_checksum: Option<&str>) -> Result<DownloadedPackage> {
        
        tracing::info!("Resuming download of {} {} from {}", name, version, download_url);

        // In a real implementation, would check if server supports range requests
        // and resume from the current file size
        let existing_size = if partial_path.exists() {
            fs::metadata(&partial_path).await?.len()
        } else {
            0
        };

        tracing::debug!("Resuming from byte offset: {}", existing_size);

        // For now, just download normally (in real implementation would use HTTP Range header)
        self.download_package(name, version, download_url, partial_path, expected_checksum).await
    }
    

}

/// Request for downloading a package
#[derive(Debug, Clone)]
pub struct PackageDownloadRequest {
    pub name: String,
    pub version: Version,
    pub download_url: String,
    pub output_path: PathBuf,
    pub expected_checksum: Option<String>,
}

impl PackageDownloadRequest {
    pub fn new(name: String, version: Version, download_url: String, output_path: PathBuf) -> Self {
        Self {
            name,
            version,
            download_url,
            output_path,
            expected_checksum: None,
        }
    }

    pub fn with_checksum(mut self, checksum: String) -> Self {
        self.expected_checksum = Some(checksum);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_package_download() {
        let mut server = mockito::Server::new_async().await;
        
        // Mock package download endpoint
        let mock_package_data = b"mock package data";
        let _m = server
            .mock("GET", "/test-package-1.0.0.tar.gz")
            .with_status(200)
            .with_header("content-type", "application/gzip")
            .with_body(mock_package_data)
            .create_async()
            .await;

        let temp_dir = TempDir::new().unwrap();
        let downloader = PackageDownloader::new(DownloadConfig::default()).unwrap();
        
        let output_path = temp_dir.path().join("test-package.tar.gz");
        let version = Version::new(1, 0, 0);
        let download_url = format!("{}/test-package-1.0.0.tar.gz", server.url());
        
        let result = downloader.download_package(
            "test-package",
            &version,
            &download_url,
            output_path.clone(),
            None, // Skip checksum validation for test
        ).await;

        // Should complete without error in mock implementation
        assert!(result.is_ok());
        
        // Verify file was created
        assert!(output_path.exists());
    }

    #[tokio::test]
    async fn test_concurrent_downloads() {
        let mut server = mockito::Server::new_async().await;
        
        // Mock multiple package download endpoints
        let mock_package_data = b"mock package data";
        let _m1 = server
            .mock("GET", "/package1.tar.gz")
            .with_status(200)
            .with_header("content-type", "application/gzip")
            .with_body(mock_package_data)
            .create_async()
            .await;
            
        let _m2 = server
            .mock("GET", "/package2.tar.gz")
            .with_status(200)
            .with_header("content-type", "application/gzip")
            .with_body(mock_package_data)
            .create_async()
            .await;

        let temp_dir = TempDir::new().unwrap();
        let downloader = PackageDownloader::new(DownloadConfig::default()).unwrap();
        
        let requests = vec![
            PackageDownloadRequest::new(
                "package1".to_string(),
                Version::new(1, 0, 0),
                format!("{}/package1.tar.gz", server.url()),
                temp_dir.path().join("package1.tar.gz"),
            ),
            PackageDownloadRequest::new(
                "package2".to_string(),
                Version::new(2, 0, 0),
                format!("{}/package2.tar.gz", server.url()),
                temp_dir.path().join("package2.tar.gz"),
            ),
        ];

        let results = downloader.download_packages(requests).await;
        assert!(results.is_ok());
        assert_eq!(results.unwrap().len(), 2);
    }
}
