//! Archive extraction utilities for CURSED packages
//!
//! This module handles extracting various archive formats (tar.gz, zip, etc.)

use crate::error::{CursedError, Result};
use flate2::read::GzDecoder;
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use tar::Archive;
use zip::ZipArchive;

/// Supported archive formats
#[derive(Debug, Clone, PartialEq)]
pub enum ArchiveFormat {
    TarGz,
    Zip,
    Tar,
}

/// Archive extraction configuration
#[derive(Debug, Clone)]
pub struct ExtractionConfig {
    pub overwrite_existing: bool,
    pub preserve_permissions: bool,
    pub strip_components: usize,
}

/// Result of archive extraction
#[derive(Debug)]
pub struct ExtractionResult {
    pub extracted_files: Vec<PathBuf>,
    pub total_size: u64,
    pub format: ArchiveFormat,
}

impl Default for ExtractionConfig {
    fn default() -> Self {
        Self {
            overwrite_existing: false,
            preserve_permissions: true,
            strip_components: 0,
        }
    }
}

impl ArchiveFormat {
    /// Detect archive format from file extension
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let filename = path.file_name()
            .ok_or_else(|| CursedError::General("Invalid file path".to_string()))?
            .to_string_lossy()
            .to_lowercase();

        if filename.ends_with(".tar.gz") || filename.ends_with(".tgz") {
            Ok(ArchiveFormat::TarGz)
        } else if filename.ends_with(".zip") {
            Ok(ArchiveFormat::Zip)
        } else if filename.ends_with(".tar") {
            Ok(ArchiveFormat::Tar)
        } else {
            Err(CursedError::General(format!(
                "Unsupported archive format: {}",
                filename
            )))
        }
    }
}

/// Extract archive to destination directory
pub fn extract_archive<P: AsRef<Path>>(
    archive_path: P,
    destination: P,
    config: ExtractionConfig,
) -> Result<ExtractionResult> {
    let archive_path = archive_path.as_ref();
    let destination = destination.as_ref();
    let format = ArchiveFormat::from_path(archive_path)?;

    tracing::info!(
        "Extracting {:?} archive: {:?} to {:?}",
        format,
        archive_path,
        destination
    );

    // Create destination directory if it doesn't exist
    fs::create_dir_all(destination)
        .map_err(|e| CursedError::General(format!("Failed to create destination directory: {}", e)))?;

    match format {
        ArchiveFormat::TarGz => extract_tar_gz(archive_path, destination, config),
        ArchiveFormat::Tar => extract_tar(archive_path, destination, config),
        ArchiveFormat::Zip => extract_zip(archive_path, destination, config),
    }
}

/// Extract tar.gz archive
fn extract_tar_gz<P: AsRef<Path>>(
    archive_path: P,
    destination: P,
    config: ExtractionConfig,
) -> Result<ExtractionResult> {
    let file = fs::File::open(&archive_path)
        .map_err(|e| CursedError::General(format!("Failed to open archive: {}", e)))?;
    
    let reader = BufReader::new(file);
    let gz_decoder = GzDecoder::new(reader);
    let mut archive = Archive::new(gz_decoder);
    
    extract_tar_archive(&mut archive, destination.as_ref(), config)
}

/// Extract tar archive
fn extract_tar<P: AsRef<Path>>(
    archive_path: P,
    destination: P,
    config: ExtractionConfig,
) -> Result<ExtractionResult> {
    let file = fs::File::open(&archive_path)
        .map_err(|e| CursedError::General(format!("Failed to open archive: {}", e)))?;
    
    let reader = BufReader::new(file);
    let mut archive = Archive::new(reader);
    
    extract_tar_archive(&mut archive, destination.as_ref(), config)
}

/// Extract tar archive (common implementation)
fn extract_tar_archive<R: Read>(
    archive: &mut Archive<R>,
    destination: &Path,
    config: ExtractionConfig,
) -> Result<ExtractionResult> {
    let mut extracted_files = Vec::new();
    let mut total_size = 0u64;

    for entry_result in archive.entries()
        .map_err(|e| CursedError::General(format!("Failed to read archive entries: {}", e)))? 
    {
        let mut entry = entry_result
            .map_err(|e| CursedError::General(format!("Failed to read archive entry: {}", e)))?;

        let path = entry.path()
            .map_err(|e| CursedError::General(format!("Invalid entry path: {}", e)))?;

        // Strip components if requested
        let relative_path = if config.strip_components > 0 {
            let components: Vec<_> = path.components().collect();
            if components.len() <= config.strip_components {
                continue; // Skip this entry
            }
            PathBuf::from_iter(components.into_iter().skip(config.strip_components))
        } else {
            path.into_owned()
        };

        let output_path = destination.join(&relative_path);

        // Check if file already exists
        if output_path.exists() && !config.overwrite_existing {
            tracing::warn!("Skipping existing file: {:?}", output_path);
            continue;
        }

        // Extract the entry
        entry.unpack(&output_path)
            .map_err(|e| CursedError::General(format!("Failed to extract entry: {}", e)))?;

        let size = entry.size();
        total_size += size;
        extracted_files.push(relative_path);

        tracing::debug!("Extracted: {:?} ({} bytes)", output_path, size);
    }

    tracing::info!(
        "Extracted {} files ({} bytes total)",
        extracted_files.len(),
        total_size
    );

    Ok(ExtractionResult {
        extracted_files,
        total_size,
        format: ArchiveFormat::TarGz,
    })
}

/// Extract zip archive
fn extract_zip<P: AsRef<Path>>(
    archive_path: P,
    destination: P,
    config: ExtractionConfig,
) -> Result<ExtractionResult> {
    let file = fs::File::open(&archive_path)
        .map_err(|e| CursedError::General(format!("Failed to open archive: {}", e)))?;
    
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)
        .map_err(|e| CursedError::General(format!("Failed to read ZIP archive: {}", e)))?;

    let mut extracted_files = Vec::new();
    let mut total_size = 0u64;
    let destination = destination.as_ref();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| CursedError::General(format!("Failed to read ZIP entry: {}", e)))?;

        let path = PathBuf::from(file.name());

        // Strip components if requested
        let relative_path = if config.strip_components > 0 {
            let components: Vec<_> = path.components().collect();
            if components.len() <= config.strip_components {
                continue; // Skip this entry
            }
            PathBuf::from_iter(components.into_iter().skip(config.strip_components))
        } else {
            path
        };

        let output_path = destination.join(&relative_path);

        // Check if file already exists
        if output_path.exists() && !config.overwrite_existing {
            tracing::warn!("Skipping existing file: {:?}", output_path);
            continue;
        }

        // Create parent directories
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| CursedError::General(format!("Failed to create parent directory: {}", e)))?;
        }

        // Extract file or directory
        if file.is_dir() {
            fs::create_dir_all(&output_path)
                .map_err(|e| CursedError::General(format!("Failed to create directory: {}", e)))?;
        } else {
            let mut output_file = fs::File::create(&output_path)
                .map_err(|e| CursedError::General(format!("Failed to create output file: {}", e)))?;
            
            std::io::copy(&mut file, &mut output_file)
                .map_err(|e| CursedError::General(format!("Failed to extract file: {}", e)))?;
        }

        let size = file.size();
        total_size += size;
        extracted_files.push(relative_path);

        tracing::debug!("Extracted: {:?} ({} bytes)", output_path, size);
    }

    tracing::info!(
        "Extracted {} files ({} bytes total)",
        extracted_files.len(),
        total_size
    );

    Ok(ExtractionResult {
        extracted_files,
        total_size,
        format: ArchiveFormat::Zip,
    })
}

/// Verify archive integrity before extraction
pub fn verify_archive<P: AsRef<Path>>(archive_path: P) -> Result<bool> {
    let archive_path = archive_path.as_ref();
    let format = ArchiveFormat::from_path(archive_path)?;

    tracing::debug!("Verifying {:?} archive: {:?}", format, archive_path);

    match format {
        ArchiveFormat::TarGz => verify_tar_gz(archive_path),
        ArchiveFormat::Tar => verify_tar(archive_path),
        ArchiveFormat::Zip => verify_zip(archive_path),
    }
}

/// Verify tar.gz archive
fn verify_tar_gz<P: AsRef<Path>>(archive_path: P) -> Result<bool> {
    let file = fs::File::open(&archive_path)
        .map_err(|e| CursedError::General(format!("Failed to open archive: {}", e)))?;
    
    let reader = BufReader::new(file);
    let gz_decoder = GzDecoder::new(reader);
    let mut archive = Archive::new(gz_decoder);
    
    // Try to read all entries to verify integrity
    for entry_result in archive.entries()
        .map_err(|e| CursedError::General(format!("Archive verification failed: {}", e)))?
    {
        let _entry = entry_result
            .map_err(|e| CursedError::General(format!("Archive verification failed: {}", e)))?;
    }
    
    Ok(true)
}

/// Verify tar archive
fn verify_tar<P: AsRef<Path>>(archive_path: P) -> Result<bool> {
    let file = fs::File::open(&archive_path)
        .map_err(|e| CursedError::General(format!("Failed to open archive: {}", e)))?;
    
    let reader = BufReader::new(file);
    let mut archive = Archive::new(reader);
    
    // Try to read all entries to verify integrity
    for entry_result in archive.entries()
        .map_err(|e| CursedError::General(format!("Archive verification failed: {}", e)))?
    {
        let _entry = entry_result
            .map_err(|e| CursedError::General(format!("Archive verification failed: {}", e)))?;
    }
    
    Ok(true)
}

/// Verify zip archive
fn verify_zip<P: AsRef<Path>>(archive_path: P) -> Result<bool> {
    let file = fs::File::open(&archive_path)
        .map_err(|e| CursedError::General(format!("Failed to open archive: {}", e)))?;
    
    let reader = BufReader::new(file);
    let archive = ZipArchive::new(reader)
        .map_err(|e| CursedError::General(format!("Archive verification failed: {}", e)))?;

    // Just opening the ZIP archive successfully is a good verification
    tracing::debug!("ZIP archive contains {} entries", archive.len());
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_archive_format_detection() {
        assert_eq!(ArchiveFormat::from_path("package.tar.gz").unwrap(), ArchiveFormat::TarGz);
        assert_eq!(ArchiveFormat::from_path("package.tgz").unwrap(), ArchiveFormat::TarGz);
        assert_eq!(ArchiveFormat::from_path("package.zip").unwrap(), ArchiveFormat::Zip);
        assert_eq!(ArchiveFormat::from_path("package.tar").unwrap(), ArchiveFormat::Tar);
        
        assert!(ArchiveFormat::from_path("package.txt").is_err());
    }

    #[test]
    fn test_extraction_config_default() {
        let config = ExtractionConfig::default();
        assert!(!config.overwrite_existing);
        assert!(config.preserve_permissions);
        assert_eq!(config.strip_components, 0);
    }
}
