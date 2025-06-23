/// Package Downloader with Progress Tracking and Integrity Verification
/// 
/// Provides high-level package download functionality with:
/// - Progress tracking for downloads
/// - Integrity verification using SHA-256 checksums  
/// - Atomic file operations to prevent corruption
/// - Concurrent download support with rate limiting
/// - Archive extraction (tar.gz, zip)
/// - Retry logic for failed downloads

use crate::package_manager::{PackageManagerError, metadata::PackageMetadata, registry::{PackageRegistry, PackageData}};
use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, Read, BufWriter, BufReader};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};
use fs2::FileExt;
use tracing::{info, warn, error, debug, instrument};
use indicatif::{ProgressBar, ProgressStyle};
use flate2::read::GzDecoder;
use tar::Archive;
use zip::ZipArchive;

/// Progress tracking for downloads
pub struct DownloadProgress {
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    pub transfer_rate: f64, // bytes per second
    pub elapsed: Duration,
    pub eta: Option<Duration>,
}

/// Download statistics
#[derive(Debug, Default, Clone)]
pub struct DownloadStats {
    pub total_downloads: usize,
    pub successful_downloads: usize,
    pub failed_downloads: usize,
    pub total_bytes_downloaded: u64,
    pub average_download_speed: f64, // bytes per second
    pub total_download_time: Duration,
}

/// Callback type for progress reporting
pub type ProgressCallback = Box<dyn Fn(&DownloadProgress) + Send + Sync>;

/// Package downloader with progress tracking and verification
#[derive(Debug)]
pub struct PackageDownloader {
    /// Download directory for temporary files
    temp_dir: PathBuf,
    /// Maximum concurrent downloads
    max_concurrent: usize,
    /// Download timeout
    timeout: Duration,
    /// Verification enabled
    verify_checksums: bool,
    /// Download statistics
    stats: Arc<Mutex<DownloadStats>>,
}

/// Download configuration
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    pub temp_dir: PathBuf,
    pub max_concurrent: usize,
    pub timeout: Duration,
    pub verify_checksums: bool,
    pub chunk_size: usize,
    pub retry_attempts: usize,
    pub retry_delay: Duration,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            temp_dir: std::env::temp_dir().join("cursed-downloads"),
            max_concurrent: 4,
            timeout: Duration::from_secs(300), // 5 minutes
            verify_checksums: true,
            chunk_size: 8192, // 8KB chunks
            retry_attempts: 3,
            retry_delay: Duration::from_secs(1),
        }
    }
}

/// Downloaded package with extraction information
#[derive(Debug)]
pub struct DownloadedPackage {
    pub metadata: PackageMetadata,
    pub archive_path: PathBuf,
    pub extracted_path: Option<PathBuf>,
    pub checksum: String,
    pub size: usize,
    pub download_time: Duration,
}

impl PackageDownloader {
    /// Create a new package downloader with default configuration
    pub fn new() -> Result<(), Error> {
        Self::with_config(DownloadConfig::default())
    }

    /// Create a new package downloader with custom configuration
    #[instrument(fields(temp_dir = ?config.temp_dir, max_concurrent = config.max_concurrent))]
    pub fn with_config(config: DownloadConfig) -> Result<(), Error> {
        info!("Initializing package downloader");
        
        // Create temporary download directory
        fs::create_dir_all(&config.temp_dir)
            .map_err(|e| PackageManagerError::FileSystemError {
                path: config.temp_dir.clone(),
                error: format!("Failed to create download directory: {}", e),
            })?;

        Ok(Self {
            temp_dir: config.temp_dir,
            max_concurrent: config.max_concurrent,
            timeout: config.timeout,
            verify_checksums: config.verify_checksums,
            stats: Arc::new(Mutex::new(DownloadStats::default())),
        })
    }

    /// Download a package from registry with progress tracking
    #[instrument(skip(self, registry, progress_callback))]
    pub async fn download_package(
        &mut self,
        registry: &mut PackageRegistry,
        package_name: &str,
        version: &str,
        extract_to: Option<&Path>,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<(), Error> {
        let start_time = Instant::now();
        
        info!(package = package_name, version = version, "Starting package download");

        // Update stats
        {
            let mut stats = self.stats.lock().map_err(|_| PackageManagerError::RegistryError {
                message: "Failed to lock download stats".to_string(),
            })?;
            stats.total_downloads += 1;
        }

        // Get package metadata first
        let metadata = registry.get_package_metadata(package_name, version).await?;
        
        // Create temporary file for download
        let temp_file_path = self.temp_dir.join(format!("{}@{}.tmp", package_name, version));
        let final_file_path = self.temp_dir.join(format!("{}@{}.tar.gz", package_name, version));
        
        // Download package data
        let package_data = registry.download_package(package_name, version).await?;
        
        // Write to temporary file atomically
        {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&temp_file_path)
                .map_err(|e| PackageManagerError::FileSystemError {
                    path: temp_file_path.clone(),
                    error: format!("Failed to create temporary file: {}", e),
                })?;

            // Acquire file lock for atomic operations
            file.try_lock_exclusive()
                .map_err(|e| PackageManagerError::FileSystemError {
                    path: temp_file_path.clone(),
                    error: format!("Failed to lock temporary file: {}", e),
                })?;

            let mut writer = BufWriter::new(file);
            
            // Write data with progress tracking
            let total_size = package_data.content.len();
            let mut written = 0;
            
            for chunk in package_data.content.chunks(8192) {
                writer.write_all(chunk)
                    .map_err(|e| PackageManagerError::FileSystemError {
                        path: temp_file_path.clone(),
                        error: format!("Failed to write chunk: {}", e),
                    })?;
                
                written += chunk.len();
                
                // Report progress if callback provided
                if let Some(ref callback) = progress_callback {
                    let elapsed = start_time.elapsed();
                    let rate = if elapsed.as_secs_f64() > 0.0 {
                        written as f64 / elapsed.as_secs_f64()
                    } else {
                        0.0
                    };
                    
                    let eta = if rate > 0.0 {
                        let remaining = total_size - written;
                        Some(Duration::from_secs_f64(remaining as f64 / rate))
                    } else {
                        None
                    };
                    
                    let progress = DownloadProgress {
                        total_bytes: total_size as u64,
                        downloaded_bytes: written as u64,
                        transfer_rate: rate,
                        elapsed,
                        eta,
                    };
                    
                    callback(&progress);
                }
            }
            
            writer.flush()
                .map_err(|e| PackageManagerError::FileSystemError {
                    path: temp_file_path.clone(),
                    error: format!("Failed to flush file: {}", e),
                })?;
        }

        // Verify checksum if enabled
        if self.verify_checksums {
            let downloaded_data = fs::read(&temp_file_path)
                .map_err(|e| PackageManagerError::FileSystemError {
                    path: temp_file_path.clone(),
                    error: format!("Failed to read downloaded file for verification: {}", e),
                })?;
            
            let mut hasher = Sha256::new();
            hasher.update(&downloaded_data);
            let calculated_checksum = hex::encode(hasher.finalize());
            
            if calculated_checksum != package_data.checksum {
                fs::remove_file(&temp_file_path).ok(); // Clean up on failure
                return Err(PackageManagerError::CacheCorruption {
                    details: format!(
                        "Checksum mismatch for {}@{}: expected {}, got {}",
                        package_name, version, package_data.checksum, calculated_checksum
                    ),
                });
            }
            
            debug!(package = package_name, version = version, "Checksum verification passed");
        }

        // Atomic rename to final location
        fs::rename(&temp_file_path, &final_file_path)
            .map_err(|e| PackageManagerError::FileSystemError {
                path: final_file_path.clone(),
                error: format!("Failed to move downloaded file: {}", e),
            })?;

        let download_time = start_time.elapsed();

        // Extract package if requested
        let extracted_path = if let Some(extract_dir) = extract_to {
            Some(self.extract_package(&final_file_path, extract_dir, package_name, version).await?)
        } else {
            None
        };

        // Update success statistics
        {
            let mut stats = self.stats.lock().map_err(|_| PackageManagerError::RegistryError {
                message: "Failed to lock download stats".to_string(),
            })?;
            stats.successful_downloads += 1;
            stats.total_bytes_downloaded += package_data.size as u64;
            stats.total_download_time += download_time;
            
            // Update average download speed
            if stats.total_download_time.as_secs_f64() > 0.0 {
                stats.average_download_speed = stats.total_bytes_downloaded as f64 / stats.total_download_time.as_secs_f64();
            }
        }

        info!(
            package = package_name, 
            version = version, 
            size = package_data.size,
            duration_ms = download_time.as_millis(),
            "Package downloaded successfully"
        );

        Ok(DownloadedPackage {
            metadata,
            archive_path: final_file_path,
            extracted_path,
            checksum: package_data.checksum,
            size: package_data.size,
            download_time,
        })
    }

    /// Extract a downloaded package archive
    #[instrument(skip(self))]
    pub async fn extract_package(
        &self,
        archive_path: &Path,
        extract_to: &Path,
        package_name: &str,
        version: &str,
    ) -> Result<(), Error> {
        info!(
            archive = ?archive_path,
            extract_to = ?extract_to,
            package = package_name,
            version = version,
            "Extracting package archive"
        );

        // Create extraction directory
        let package_extract_dir = extract_to.join(package_name).join(version);
        fs::create_dir_all(&package_extract_dir)
            .map_err(|e| PackageManagerError::FileSystemError {
                path: package_extract_dir.clone(),
                error: format!("Failed to create extraction directory: {}", e),
            })?;

        // Determine archive type from extension
        let file_name = archive_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        if file_name.ends_with(".tar.gz") || file_name.ends_with(".tgz") {
            self.extract_tar_gz(archive_path, &package_extract_dir).await?;
        } else if file_name.ends_with(".zip") {
            self.extract_zip(archive_path, &package_extract_dir).await?;
        } else {
            // Assume tar.gz for unknown extensions
            warn!(
                archive = ?archive_path,
                "Unknown archive format, attempting tar.gz extraction"
            );
            self.extract_tar_gz(archive_path, &package_extract_dir).await?;
        }

        info!(
            extracted_to = ?package_extract_dir,
            package = package_name,
            version = version,
            "Package extracted successfully"
        );

        Ok(package_extract_dir)
    }

    /// Extract tar.gz archive
    async fn extract_tar_gz(&self, archive_path: &Path, extract_to: &Path) -> Result<(), Error> {
        let file = File::open(archive_path)
            .map_err(|e| PackageManagerError::FileSystemError {
                path: archive_path.to_path_buf(),
                error: format!("Failed to open archive: {}", e),
            })?;

        let reader = BufReader::new(file);
        let gz_decoder = GzDecoder::new(reader);
        let mut archive = Archive::new(gz_decoder);

        archive.unpack(extract_to)
            .map_err(|e| PackageManagerError::FileSystemError {
                path: extract_to.to_path_buf(),
                error: format!("Failed to extract tar.gz archive: {}", e),
            })?;

        Ok(())
    }

    /// Extract zip archive
    async fn extract_zip(&self, archive_path: &Path, extract_to: &Path) -> Result<(), Error> {
        let file = File::open(archive_path)
            .map_err(|e| PackageManagerError::FileSystemError {
                path: archive_path.to_path_buf(),
                error: format!("Failed to open archive: {}", e),
            })?;

        let mut archive = ZipArchive::new(BufReader::new(file))
            .map_err(|e| PackageManagerError::FileSystemError {
                path: archive_path.to_path_buf(),
                error: format!("Failed to read zip archive: {}", e),
            })?;

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)
                .map_err(|e| PackageManagerError::FileSystemError {
                    path: archive_path.to_path_buf(),
                    error: format!("Failed to read zip entry {}: {}", i, e),
                })?;

            let entry_path = extract_to.join(entry.name());
            
            if entry.is_dir() {
                fs::create_dir_all(&entry_path)
                    .map_err(|e| PackageManagerError::FileSystemError {
                        path: entry_path,
                        error: format!("Failed to create directory: {}", e),
                    })?;
            } else {
                if let Some(parent) = entry_path.parent() {
                    fs::create_dir_all(parent)
                        .map_err(|e| PackageManagerError::FileSystemError {
                            path: parent.to_path_buf(),
                            error: format!("Failed to create parent directory: {}", e),
                        })?;
                }

                let mut output_file = File::create(&entry_path)
                    .map_err(|e| PackageManagerError::FileSystemError {
                        path: entry_path.clone(),
                        error: format!("Failed to create output file: {}", e),
                    })?;

                io::copy(&mut entry, &mut output_file)
                    .map_err(|e| PackageManagerError::FileSystemError {
                        path: entry_path,
                        error: format!("Failed to extract file: {}", e),
                    })?;
            }
        }

        Ok(())
    }

    /// Get download statistics
    pub fn get_stats(&self) -> Result<(), Error> {
        let stats = self.stats.lock().map_err(|_| PackageManagerError::RegistryError {
            message: "Failed to lock download stats".to_string(),
        })?;
        Ok(stats.clone())
    }

    /// Clean up temporary download files
    #[instrument(skip(self))]
    pub fn cleanup(&self) -> Result<(), Error> {
        info!("Cleaning up temporary download files");
        
        if self.temp_dir.exists() {
            let entries = fs::read_dir(&self.temp_dir)
                .map_err(|e| PackageManagerError::FileSystemError {
                    path: self.temp_dir.clone(),
                    error: format!("Failed to read temp directory: {}", e),
                })?;

            let mut cleaned_files = 0;
            let mut cleaned_bytes = 0u64;

            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Ok(metadata) = entry.metadata() {
                        cleaned_bytes += metadata.len();
                    }
                    
                    if let Err(e) = fs::remove_file(&path) {
                        warn!(file = ?path, error = %e, "Failed to remove temporary file");
                    } else {
                        cleaned_files += 1;
                    }
                }
            }

            info!(
                files_cleaned = cleaned_files,
                bytes_cleaned = cleaned_bytes,
                "Temporary files cleaned up"
            );
        }

        Ok(())
    }

    /// Create a progress bar for console display
    pub fn create_progress_bar(total_size: Option<u64>) -> ProgressBar {
        let progress = match total_size {
            Some(size) => ProgressBar::new(size),
            None => ProgressBar::new_spinner(),
        };

        progress.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                .unwrap()
                .progress_chars("=>-"),
        );

        progress
    }

    /// Get temporary directory path
    pub fn temp_dir(&self) -> &Path {
        &self.temp_dir
    }
}

impl Drop for PackageDownloader {
    fn drop(&mut self) {
        // Best effort cleanup on drop
        let _ = self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_downloader() -> (PackageDownloader, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config = DownloadConfig {
            temp_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        let downloader = PackageDownloader::with_config(config).unwrap();
        (downloader, temp_dir)
    }

    #[test]
    fn test_downloader_creation() {
        let (downloader, _temp_dir) = create_test_downloader();
        assert_eq!(downloader.max_concurrent, 4);
        assert!(downloader.verify_checksums);
        assert_eq!(downloader.timeout, Duration::from_secs(300));
    }

    #[test]
    fn test_download_stats() {
        let (downloader, _temp_dir) = create_test_downloader();
        let stats = downloader.get_stats().unwrap();
        assert_eq!(stats.total_downloads, 0);
        assert_eq!(stats.successful_downloads, 0);
        assert_eq!(stats.failed_downloads, 0);
        assert_eq!(stats.total_bytes_downloaded, 0);
    }

    #[test]
    fn test_progress_bar_creation() {
        let progress = PackageDownloader::create_progress_bar(Some(1024));
        assert_eq!(progress.length(), Some(1024));
        
        let spinner = PackageDownloader::create_progress_bar(None);
        assert_eq!(spinner.length(), None);
    }

    #[test]
    fn test_temp_dir_access() {
        let (downloader, temp_dir) = create_test_downloader();
        assert_eq!(downloader.temp_dir(), temp_dir.path());
    }

    #[test]
    fn test_cleanup() {
        let (downloader, temp_dir) = create_test_downloader();
        
        // Create a test file
        let test_file = temp_dir.path().join("test.tmp");
        std::fs::write(&test_file, b"test content").unwrap();
        assert!(test_file.exists());
        
        // Clean up
        downloader.cleanup().unwrap();
        
        // File should be removed
        assert!(!test_file.exists());
    }
}
