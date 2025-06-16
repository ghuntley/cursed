//! Profile Data Storage and Management System
//! 
//! Provides efficient storage, retrieval, and management of profile data including:
//! - Binary format storage for efficiency
//! - Profile data merging from multiple runs
//! - Data validation and sanitization
//! - Version compatibility and migration

use crate::error::{Error, Result};
use crate::optimization::pgo::{ProfileData, PgoSystemConfig, PgoError};
use std::collections::HashMap;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{BufReader, BufWriter, Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn, error, instrument};
use serde::{Serialize, Deserialize};

/// Profile storage system with comprehensive data management
pub struct ProfileStorage {
    /// Configuration for storage
    config: ProfileStorageConfig,
    /// Profile database for metadata management
    database: ProfileDatabase,
    /// Profile merger for combining multiple runs
    merger: ProfileMerger,
    /// Storage statistics
    statistics: StorageStatistics,
    /// Active storage format
    format: ProfileFormat,
}

/// Configuration for profile storage
#[derive(Debug, Clone)]
pub struct ProfileStorageConfig {
    /// Storage directory for profile data
    pub storage_directory: PathBuf,
    /// Enable data compression
    pub enable_compression: bool,
    /// Compression level (0-9)
    pub compression_level: u32,
    /// Maximum profile file size (bytes)
    pub max_file_size: usize,
    /// Enable automatic cleanup of old profiles
    pub enable_auto_cleanup: bool,
    /// Maximum age for profile retention
    pub max_profile_age: Duration,
    /// Enable profile validation on load
    pub enable_validation: bool,
    /// Profile format version
    pub format_version: ProfileVersion,
    /// Enable incremental storage
    pub enable_incremental: bool,
    /// Backup retention count
    pub backup_retention_count: usize,
    /// Enable encryption for sensitive data
    pub enable_encryption: bool,
    /// Encryption key derivation method
    pub encryption_method: EncryptionMethod,
}

/// Profile format versions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileVersion {
    V1_0,
    V1_1,
    V2_0,
}

impl ProfileVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProfileVersion::V1_0 => "1.0",
            ProfileVersion::V1_1 => "1.1", 
            ProfileVersion::V2_0 => "2.0",
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "1.0" => Ok(ProfileVersion::V1_0),
            "1.1" => Ok(ProfileVersion::V1_1),
            "2.0" => Ok(ProfileVersion::V2_0),
            _ => Err(Error::Other(format!("Unknown profile version: {}", s))),
        }
    }

    pub fn is_compatible_with(&self, other: &ProfileVersion) -> bool {
        // For now, all versions are compatible
        matches!((self, other), 
            (ProfileVersion::V1_0, ProfileVersion::V1_0) |
            (ProfileVersion::V1_0, ProfileVersion::V1_1) |
            (ProfileVersion::V1_1, ProfileVersion::V1_0) |
            (ProfileVersion::V1_1, ProfileVersion::V1_1) |
            (ProfileVersion::V2_0, _) |
            (_, ProfileVersion::V2_0)
        )
    }
}

/// Encryption methods for profile data
#[derive(Debug, Clone)]
pub enum EncryptionMethod {
    None,
    Aes256Gcm,
    ChaCha20Poly1305,
}

impl Default for ProfileStorageConfig {
    fn default() -> Self {
        Self {
            storage_directory: PathBuf::from("target/pgo-profiles"),
            enable_compression: true,
            compression_level: 6,
            max_file_size: 100 * 1024 * 1024, // 100MB
            enable_auto_cleanup: true,
            max_profile_age: Duration::from_secs(30 * 24 * 3600), // 30 days
            enable_validation: true,
            format_version: ProfileVersion::V1_0,
            enable_incremental: true,
            backup_retention_count: 5,
            enable_encryption: false,
            encryption_method: EncryptionMethod::None,
        }
    }
}

impl ProfileStorageConfig {
    /// Create config from PGO system config
    pub fn from_pgo_config(pgo_config: &PgoSystemConfig) -> Self {
        let mut config = Self::default();
        config.storage_directory = PathBuf::from(&pgo_config.profile_directory);
        config.max_profile_age = pgo_config.max_profile_age;
        config.format_version = pgo_config.profile_version;
        config.enable_validation = pgo_config.enable_validation;

        // Adjust based on optimization level
        match pgo_config.optimization_level {
            crate::optimization::pgo::OptimizationAggressiveness::Conservative => {
                config.enable_compression = false;
                config.enable_incremental = false;
            }
            crate::optimization::pgo::OptimizationAggressiveness::Moderate => {
                config.compression_level = 3;
                config.enable_incremental = true;
            }
            crate::optimization::pgo::OptimizationAggressiveness::Aggressive => {
                config.compression_level = 9;
                config.enable_incremental = true;
                config.enable_encryption = true;
                config.encryption_method = EncryptionMethod::Aes256Gcm;
            }
            crate::optimization::pgo::OptimizationAggressiveness::Experimental => {
                config.compression_level = 9;
                config.enable_incremental = true;
                config.enable_encryption = true;
                config.encryption_method = EncryptionMethod::ChaCha20Poly1305;
            }
        }

        config
    }
}

/// Profile storage format
#[derive(Debug, Clone)]
pub enum ProfileFormat {
    Binary,
    CompressedBinary,
    Json,
    CompressedJson,
    MessagePack,
    ProtoBuf,
}

/// Profile database for metadata management
pub struct ProfileDatabase {
    /// Database file path
    db_path: PathBuf,
    /// In-memory metadata cache
    metadata_cache: HashMap<String, ProfileMetadata>,
    /// Profile index for fast lookup
    profile_index: HashMap<String, ProfileIndexEntry>,
    /// Database statistics
    db_statistics: DatabaseStatistics,
}

/// Profile metadata for database storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMetadata {
    /// Unique profile identifier
    pub profile_id: String,
    /// Profile name/description
    pub profile_name: String,
    /// Creation timestamp
    pub created_at: SystemTime,
    /// Last modified timestamp
    pub modified_at: SystemTime,
    /// Profile format version
    pub format_version: ProfileVersion,
    /// Data quality score
    pub quality_score: f64,
    /// Profile file size
    pub file_size: usize,
    /// Compression ratio (if compressed)
    pub compression_ratio: Option<f64>,
    /// Profile tags for categorization
    pub tags: Vec<String>,
    /// Source program information
    pub source_info: SourceInfo,
    /// Collection statistics summary
    pub collection_summary: CollectionSummary,
    /// Custom metadata fields
    pub custom_fields: HashMap<String, String>,
}

/// Source program information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    /// Source file paths
    pub source_files: Vec<String>,
    /// Compiler version
    pub compiler_version: String,
    /// Target architecture
    pub target_arch: String,
    /// Optimization level used
    pub optimization_level: String,
    /// Build configuration
    pub build_config: String,
}

/// Collection statistics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionSummary {
    /// Total events collected
    pub total_events: u64,
    /// Collection duration
    pub collection_duration: Duration,
    /// Function count
    pub function_count: usize,
    /// Branch count
    pub branch_count: usize,
    /// Loop count
    pub loop_count: usize,
    /// Memory region count
    pub memory_region_count: usize,
}

/// Profile index entry for fast lookup
#[derive(Debug, Clone)]
pub struct ProfileIndexEntry {
    /// Profile identifier
    pub profile_id: String,
    /// File path
    pub file_path: PathBuf,
    /// File size
    pub file_size: usize,
    /// Last access time
    pub last_accessed: SystemTime,
    /// Access frequency
    pub access_count: u64,
    /// Index entry creation time
    pub indexed_at: SystemTime,
}

/// Database statistics
#[derive(Debug, Clone, Default)]
pub struct DatabaseStatistics {
    /// Total profiles stored
    pub total_profiles: usize,
    /// Total storage size
    pub total_storage_size: usize,
    /// Average profile size
    pub average_profile_size: usize,
    /// Database file size
    pub database_size: usize,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Queries per second
    pub queries_per_second: f64,
}

/// Profile merger for combining multiple runs
pub struct ProfileMerger {
    /// Merge configuration
    config: MergeConfig,
    /// Merge statistics
    statistics: MergeStatistics,
}

/// Configuration for profile merging
#[derive(Debug, Clone)]
pub struct MergeConfig {
    /// Enable weighted merging based on quality
    pub enable_weighted_merge: bool,
    /// Maximum profiles to merge at once
    pub max_merge_count: usize,
    /// Quality threshold for inclusion
    pub quality_threshold: f64,
    /// Enable outlier detection and removal
    pub enable_outlier_removal: bool,
    /// Statistical significance threshold
    pub significance_threshold: f64,
}

impl Default for MergeConfig {
    fn default() -> Self {
        Self {
            enable_weighted_merge: true,
            max_merge_count: 10,
            quality_threshold: 0.5,
            enable_outlier_removal: true,
            significance_threshold: 0.95,
        }
    }
}

/// Merge operation statistics
#[derive(Debug, Clone, Default)]
pub struct MergeStatistics {
    /// Total merges performed
    pub total_merges: usize,
    /// Profiles merged
    pub profiles_merged: usize,
    /// Average merge time
    pub average_merge_time: Duration,
    /// Quality improvement from merging
    pub quality_improvement: f64,
    /// Data reduction ratio
    pub data_reduction_ratio: f64,
}

/// Storage operation statistics
#[derive(Debug, Clone, Default)]
pub struct StorageStatistics {
    /// Profiles stored
    pub profiles_stored: usize,
    /// Profiles loaded
    pub profiles_loaded: usize,
    /// Total bytes written
    pub bytes_written: usize,
    /// Total bytes read
    pub bytes_read: usize,
    /// Average storage time
    pub average_storage_time: Duration,
    /// Average load time
    pub average_load_time: Duration,
    /// Compression efficiency
    pub compression_efficiency: f64,
    /// Storage errors
    pub storage_errors: usize,
    /// Validation failures
    pub validation_failures: usize,
}

impl ProfileStorage {
    /// Create new profile storage system
    #[instrument(skip(config))]
    pub fn new(config: ProfileStorageConfig) -> Result<Self> {
        info!("Creating profile storage system at: {}", config.storage_directory.display());

        // Create storage directory if it doesn't exist
        create_dir_all(&config.storage_directory)?;

        // Initialize database
        let db_path = config.storage_directory.join("profiles.db");
        let database = ProfileDatabase::new(db_path)?;

        // Initialize merger
        let merger = ProfileMerger::new(MergeConfig::default())?;

        // Determine storage format
        let format = if config.enable_compression {
            ProfileFormat::CompressedBinary
        } else {
            ProfileFormat::Binary
        };

        Ok(Self {
            config,
            database,
            merger,
            statistics: StorageStatistics::default(),
            format,
        })
    }

    /// Initialize storage system
    #[instrument(skip(self, storage_path))]
    pub fn initialize(&mut self, storage_path: &Path) -> Result<()> {
        info!("Initializing profile storage at: {}", storage_path.display());

        // Update storage directory
        self.config.storage_directory = storage_path.to_path_buf();

        // Create directory structure
        create_dir_all(&self.config.storage_directory)?;
        create_dir_all(self.config.storage_directory.join("profiles"))?;
        create_dir_all(self.config.storage_directory.join("backups"))?;
        create_dir_all(self.config.storage_directory.join("temp"))?;

        // Initialize database
        self.database.initialize()?;

        // Perform auto-cleanup if enabled
        if self.config.enable_auto_cleanup {
            self.cleanup_old_profiles()?;
        }

        info!("Profile storage initialized successfully");
        Ok(())
    }

    /// Store profile data
    #[instrument(skip(self, profile_data))]
    pub fn store_profile(&mut self, profile_data: &ProfileData) -> Result<String> {
        let start_time = std::time::Instant::now();
        info!("Storing profile data");

        // Generate unique profile ID
        let profile_id = self.generate_profile_id(profile_data)?;

        // Validate profile data if enabled
        if self.config.enable_validation {
            self.validate_profile_data(profile_data)?;
        }

        // Create metadata
        let metadata = self.create_metadata(&profile_id, profile_data)?;

        // Determine storage path
        let storage_path = self.get_profile_path(&profile_id);

        // Store profile data
        let file_size = self.write_profile_data(&storage_path, profile_data)?;

        // Update metadata with file size
        let mut metadata = metadata;
        metadata.file_size = file_size;

        // Store metadata in database
        self.database.store_metadata(&profile_id, &metadata)?;

        // Update statistics
        self.statistics.profiles_stored += 1;
        self.statistics.bytes_written += file_size;
        self.statistics.average_storage_time = 
            ((self.statistics.average_storage_time * (self.statistics.profiles_stored - 1) as u32) + 
             start_time.elapsed()) / self.statistics.profiles_stored as u32;

        info!(
            profile_id = %profile_id,
            file_size = file_size,
            storage_time = ?start_time.elapsed(),
            "Profile stored successfully"
        );

        Ok(profile_id)
    }

    /// Load profile data
    #[instrument(skip(self, profile_path))]
    pub fn load_profile(&mut self, profile_path: &Path) -> Result<ProfileData> {
        let start_time = std::time::Instant::now();
        info!("Loading profile from: {}", profile_path.display());

        // Determine if path is profile ID or file path
        let (profile_id, actual_path) = if profile_path.is_absolute() {
            // Direct file path
            let profile_id = self.extract_profile_id_from_path(profile_path)?;
            (profile_id, profile_path.to_path_buf())
        } else {
            // Profile ID
            let profile_id = profile_path.to_string_lossy().to_string();
            let actual_path = self.get_profile_path(&profile_id);
            (profile_id, actual_path)
        };

        // Load metadata if available
        let metadata = self.database.load_metadata(&profile_id).ok();

        // Validate file exists
        if !actual_path.exists() {
            return Err(Error::Other(format!("Profile file not found: {}", actual_path.display())));
        }

        // Check file age if configured
        if let Some(meta) = &metadata {
            let age = SystemTime::now().duration_since(meta.created_at).unwrap_or_default();
            if age > self.config.max_profile_age {
                return Err(PgoError::ProfileTooOld { 
                    age, 
                    max_age: self.config.max_profile_age 
                }.into());
            }
        }

        // Read profile data
        let profile_data = self.read_profile_data(&actual_path)?;

        // Validate profile data if enabled
        if self.config.enable_validation {
            self.validate_profile_data(&profile_data)?;
        }

        // Update access statistics
        if let Ok(metadata) = self.database.get_metadata_mut(&profile_id) {
            metadata.modified_at = SystemTime::now();
        }

        // Update storage statistics
        let file_size = std::fs::metadata(&actual_path)?.len() as usize;
        self.statistics.profiles_loaded += 1;
        self.statistics.bytes_read += file_size;
        self.statistics.average_load_time = 
            ((self.statistics.average_load_time * (self.statistics.profiles_loaded - 1) as u32) + 
             start_time.elapsed()) / self.statistics.profiles_loaded as u32;

        info!(
            profile_id = %profile_id,
            file_size = file_size,
            load_time = ?start_time.elapsed(),
            "Profile loaded successfully"
        );

        Ok(profile_data)
    }

    /// Merge multiple profiles
    #[instrument(skip(self, profile_ids))]
    pub fn merge_profiles(&mut self, profile_ids: &[String]) -> Result<ProfileData> {
        info!("Merging {} profiles", profile_ids.len());

        if profile_ids.is_empty() {
            return Err(Error::Other("No profiles to merge".to_string()));
        }

        if profile_ids.len() > self.merger.config.max_merge_count {
            return Err(Error::Other(format!(
                "Too many profiles to merge: {} > {}", 
                profile_ids.len(), 
                self.merger.config.max_merge_count
            )));
        }

        // Load all profiles
        let mut profiles = Vec::new();
        for profile_id in profile_ids {
            let profile_path = self.get_profile_path(profile_id);
            let profile_data = self.load_profile(&profile_path)?;
            
            // Check quality threshold
            if profile_data.metadata.quality_score >= self.merger.config.quality_threshold {
                profiles.push(profile_data);
            } else {
                warn!("Skipping low-quality profile: {} (quality: {:.2})", 
                      profile_id, profile_data.metadata.quality_score);
            }
        }

        if profiles.is_empty() {
            return Err(Error::Other("No profiles meet quality threshold for merging".to_string()));
        }

        // Perform merge
        let merged_profile = self.merger.merge_profiles(&profiles)?;

        info!(
            merged_profiles = profiles.len(),
            merged_quality = %merged_profile.metadata.quality_score,
            "Profile merge completed successfully"
        );

        Ok(merged_profile)
    }

    /// List available profiles
    pub fn list_profiles(&self) -> Result<Vec<ProfileMetadata>> {
        self.database.list_all_profiles()
    }

    /// Delete profile
    #[instrument(skip(self, profile_id))]
    pub fn delete_profile(&mut self, profile_id: &str) -> Result<()> {
        info!("Deleting profile: {}", profile_id);

        // Get profile path
        let profile_path = self.get_profile_path(profile_id);

        // Create backup if configured
        if self.config.backup_retention_count > 0 {
            self.create_backup(&profile_path, profile_id)?;
        }

        // Delete file
        if profile_path.exists() {
            std::fs::remove_file(&profile_path)?;
        }

        // Remove from database
        self.database.delete_metadata(profile_id)?;

        info!("Profile deleted successfully: {}", profile_id);
        Ok(())
    }

    /// Get storage statistics
    pub fn get_statistics(&self) -> StorageStatistics {
        self.statistics.clone()
    }

    /// Cleanup old profiles
    #[instrument(skip(self))]
    pub fn cleanup_old_profiles(&mut self) -> Result<usize> {
        info!("Starting cleanup of old profiles");

        let mut deleted_count = 0;
        let cutoff_time = SystemTime::now() - self.config.max_profile_age;

        let profiles = self.database.list_all_profiles()?;
        for metadata in profiles {
            if metadata.created_at < cutoff_time {
                if let Err(e) = self.delete_profile(&metadata.profile_id) {
                    warn!("Failed to delete old profile {}: {}", metadata.profile_id, e);
                } else {
                    deleted_count += 1;
                }
            }
        }

        info!("Cleanup completed, deleted {} old profiles", deleted_count);
        Ok(deleted_count)
    }

    /// Validate profile format and integrity
    pub fn validate_profile_data(&self, profile_data: &ProfileData) -> Result<()> {
        // Check format version compatibility
        if !self.config.format_version.is_compatible_with(&ProfileVersion::V1_0) {
            return Err(PgoError::IncompatibleFormat {
                expected: self.config.format_version.as_str().to_string(),
                found: "1.0".to_string(),
            }.into());
        }

        // Check data quality
        if profile_data.metadata.quality_score < 0.5 {
            return Err(PgoError::InsufficientQuality {
                actual: profile_data.metadata.quality_score,
                required: 0.5,
            }.into());
        }

        // Validate data consistency
        if profile_data.function_profiles.is_empty() && 
           profile_data.branch_profiles.is_empty() &&
           profile_data.loop_profiles.is_empty() {
            return Err(Error::Other("Profile data appears to be empty".to_string()));
        }

        Ok(())
    }

    // Private helper methods

    fn generate_profile_id(&self, profile_data: &ProfileData) -> Result<String> {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        
        // Hash timestamp and metadata for uniqueness
        profile_data.timestamp.hash(&mut hasher);
        profile_data.metadata.command_line.hash(&mut hasher);
        profile_data.function_profiles.len().hash(&mut hasher);
        
        let hash = hasher.finish();
        Ok(format!("profile_{:016x}", hash))
    }

    fn create_metadata(&self, profile_id: &str, profile_data: &ProfileData) -> Result<ProfileMetadata> {
        let now = SystemTime::now();
        
        Ok(ProfileMetadata {
            profile_id: profile_id.to_string(),
            profile_name: format!("Profile {}", profile_id),
            created_at: now,
            modified_at: now,
            format_version: self.config.format_version,
            quality_score: profile_data.metadata.quality_score,
            file_size: 0, // Will be updated after writing
            compression_ratio: if self.config.enable_compression { Some(0.7) } else { None },
            tags: vec!["pgo".to_string(), "auto-generated".to_string()],
            source_info: SourceInfo {
                source_files: vec!["unknown".to_string()],
                compiler_version: profile_data.metadata.compiler_version.clone(),
                target_arch: profile_data.metadata.target_architecture.clone(),
                optimization_level: "unknown".to_string(),
                build_config: "unknown".to_string(),
            },
            collection_summary: CollectionSummary {
                total_events: profile_data.collection_stats.total_events,
                collection_duration: profile_data.collection_duration,
                function_count: profile_data.function_profiles.len(),
                branch_count: profile_data.branch_profiles.len(),
                loop_count: profile_data.loop_profiles.len(),
                memory_region_count: profile_data.memory_profiles.len(),
            },
            custom_fields: HashMap::new(),
        })
    }

    fn get_profile_path(&self, profile_id: &str) -> PathBuf {
        self.config.storage_directory
            .join("profiles")
            .join(format!("{}.profile", profile_id))
    }

    fn extract_profile_id_from_path(&self, path: &Path) -> Result<String> {
        path.file_stem()
            .and_then(|stem| stem.to_str())
            .map(|s| s.to_string())
            .ok_or_else(|| Error::Other("Invalid profile path".to_string()))
    }

    fn write_profile_data(&self, path: &PathBuf, profile_data: &ProfileData) -> Result<usize> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        let mut writer = BufWriter::new(file);
        
        match self.format {
            ProfileFormat::Binary => {
                let serialized = bincode::serialize(profile_data)
                    .map_err(|e| Error::Other(format!("Serialization failed: {}", e)))?;
                writer.write_all(&serialized)?;
                Ok(serialized.len())
            }
            ProfileFormat::CompressedBinary => {
                let serialized = bincode::serialize(profile_data)
                    .map_err(|e| Error::Other(format!("Serialization failed: {}", e)))?;
                
                // Simulate compression (in real implementation would use actual compression)
                let compressed = self.compress_data(&serialized)?;
                writer.write_all(&compressed)?;
                Ok(compressed.len())
            }
            ProfileFormat::Json => {
                let serialized = serde_json::to_vec_pretty(profile_data)
                    .map_err(|e| Error::Other(format!("JSON serialization failed: {}", e)))?;
                writer.write_all(&serialized)?;
                Ok(serialized.len())
            }
            _ => Err(Error::Other("Unsupported storage format".to_string())),
        }
    }

    fn read_profile_data(&self, path: &PathBuf) -> Result<ProfileData> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        match self.format {
            ProfileFormat::Binary => {
                bincode::deserialize(&buffer)
                    .map_err(|e| Error::Other(format!("Deserialization failed: {}", e)))
            }
            ProfileFormat::CompressedBinary => {
                let decompressed = self.decompress_data(&buffer)?;
                bincode::deserialize(&decompressed)
                    .map_err(|e| Error::Other(format!("Deserialization failed: {}", e)))
            }
            ProfileFormat::Json => {
                serde_json::from_slice(&buffer)
                    .map_err(|e| Error::Other(format!("JSON deserialization failed: {}", e)))
            }
            _ => Err(Error::Other("Unsupported storage format".to_string())),
        }
    }

    fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Simulate compression - in real implementation would use zlib, lz4, etc.
        let mut compressed = Vec::with_capacity(data.len() / 2);
        compressed.extend_from_slice(b"COMPRESSED:");
        compressed.extend_from_slice(data);
        Ok(compressed)
    }

    fn decompress_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Simulate decompression
        if data.starts_with(b"COMPRESSED:") {
            Ok(data[11..].to_vec())
        } else {
            Ok(data.to_vec())
        }
    }

    fn create_backup(&self, source_path: &PathBuf, profile_id: &str) -> Result<()> {
        let backup_dir = self.config.storage_directory.join("backups");
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let backup_path = backup_dir.join(format!("{}_{}.backup", profile_id, timestamp));
        
        std::fs::copy(source_path, backup_path)?;
        
        // Cleanup old backups
        self.cleanup_old_backups(profile_id)?;
        
        Ok(())
    }

    fn cleanup_old_backups(&self, profile_id: &str) -> Result<()> {
        let backup_dir = self.config.storage_directory.join("backups");
        let mut backups = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(&backup_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with(&format!("{}_", profile_id)) && name.ends_with(".backup") {
                        backups.push((entry.path(), entry.metadata().ok()?.modified().ok()?));
                    }
                }
            }
        }
        
        // Sort by modification time (newest first)
        backups.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Remove excess backups
        for (path, _) in backups.into_iter().skip(self.config.backup_retention_count) {
            let _ = std::fs::remove_file(path);
        }
        
        Ok(())
    }
}

impl ProfileDatabase {
    /// Create new profile database
    pub fn new(db_path: PathBuf) -> Result<Self> {
        Ok(Self {
            db_path,
            metadata_cache: HashMap::new(),
            profile_index: HashMap::new(),
            db_statistics: DatabaseStatistics::default(),
        })
    }

    /// Initialize database
    pub fn initialize(&mut self) -> Result<()> {
        // In a real implementation, this would create/open SQLite database
        // For now, we'll use in-memory storage
        debug!("Database initialized at: {}", self.db_path.display());
        Ok(())
    }

    /// Store metadata
    pub fn store_metadata(&mut self, profile_id: &str, metadata: &ProfileMetadata) -> Result<()> {
        self.metadata_cache.insert(profile_id.to_string(), metadata.clone());
        
        let index_entry = ProfileIndexEntry {
            profile_id: profile_id.to_string(),
            file_path: PathBuf::from(format!("profiles/{}.profile", profile_id)),
            file_size: metadata.file_size,
            last_accessed: SystemTime::now(),
            access_count: 0,
            indexed_at: SystemTime::now(),
        };
        
        self.profile_index.insert(profile_id.to_string(), index_entry);
        self.db_statistics.total_profiles += 1;
        
        Ok(())
    }

    /// Load metadata
    pub fn load_metadata(&self, profile_id: &str) -> Result<ProfileMetadata> {
        self.metadata_cache.get(profile_id)
            .cloned()
            .ok_or_else(|| Error::Other(format!("Profile metadata not found: {}", profile_id)))
    }

    /// Get mutable metadata reference
    pub fn get_metadata_mut(&mut self, profile_id: &str) -> Result<&mut ProfileMetadata> {
        self.metadata_cache.get_mut(profile_id)
            .ok_or_else(|| Error::Other(format!("Profile metadata not found: {}", profile_id)))
    }

    /// Delete metadata
    pub fn delete_metadata(&mut self, profile_id: &str) -> Result<()> {
        self.metadata_cache.remove(profile_id);
        self.profile_index.remove(profile_id);
        self.db_statistics.total_profiles = self.db_statistics.total_profiles.saturating_sub(1);
        Ok(())
    }

    /// List all profiles
    pub fn list_all_profiles(&self) -> Result<Vec<ProfileMetadata>> {
        Ok(self.metadata_cache.values().cloned().collect())
    }
}

impl ProfileMerger {
    /// Create new profile merger
    pub fn new(config: MergeConfig) -> Result<Self> {
        Ok(Self {
            config,
            statistics: MergeStatistics::default(),
        })
    }

    /// Merge multiple profiles
    pub fn merge_profiles(&mut self, profiles: &[ProfileData]) -> Result<ProfileData> {
        let start_time = std::time::Instant::now();
        
        if profiles.is_empty() {
            return Err(Error::Other("No profiles to merge".to_string()));
        }

        if profiles.len() == 1 {
            return Ok(profiles[0].clone());
        }

        // Use first profile as base
        let mut merged = profiles[0].clone();
        
        // Merge function profiles
        for profile in &profiles[1..] {
            for (func_name, func_profile) in &profile.function_profiles {
                if let Some(existing) = merged.function_profiles.get_mut(func_name) {
                    // Merge function data
                    existing.call_count += func_profile.call_count;
                    existing.total_execution_time += func_profile.total_execution_time;
                    existing.average_execution_time = 
                        existing.total_execution_time / existing.call_count as u32;
                    
                    if func_profile.min_execution_time < existing.min_execution_time {
                        existing.min_execution_time = func_profile.min_execution_time;
                    }
                    if func_profile.max_execution_time > existing.max_execution_time {
                        existing.max_execution_time = func_profile.max_execution_time;
                    }
                } else {
                    merged.function_profiles.insert(func_name.clone(), func_profile.clone());
                }
            }
        }

        // Merge branch profiles
        for profile in &profiles[1..] {
            for (branch_id, branch_profile) in &profile.branch_profiles {
                if let Some(existing) = merged.branch_profiles.get_mut(branch_id) {
                    existing.total_executions += branch_profile.total_executions;
                    existing.taken_count += branch_profile.taken_count;
                    existing.not_taken_count += branch_profile.not_taken_count;
                    
                    // Recalculate prediction accuracy
                    let taken_ratio = existing.taken_count as f64 / existing.total_executions as f64;
                    existing.prediction_accuracy = if taken_ratio > 0.5 {
                        taken_ratio
                    } else {
                        1.0 - taken_ratio
                    };
                } else {
                    merged.branch_profiles.insert(branch_id.clone(), branch_profile.clone());
                }
            }
        }

        // Merge loop profiles
        for profile in &profiles[1..] {
            for (loop_id, loop_profile) in &profile.loop_profiles {
                if let Some(existing) = merged.loop_profiles.get_mut(loop_id) {
                    existing.total_executions += loop_profile.total_executions;
                    existing.total_iterations += loop_profile.total_iterations;
                    existing.average_iterations = 
                        existing.total_iterations as f64 / existing.total_executions as f64;
                    
                    if loop_profile.min_iterations < existing.min_iterations {
                        existing.min_iterations = loop_profile.min_iterations;
                    }
                    if loop_profile.max_iterations > existing.max_iterations {
                        existing.max_iterations = loop_profile.max_iterations;
                    }
                } else {
                    merged.loop_profiles.insert(loop_id.clone(), loop_profile.clone());
                }
            }
        }

        // Update metadata
        merged.metadata.quality_score = self.calculate_merged_quality_score(profiles)?;
        merged.collection_duration = profiles.iter()
            .map(|p| p.collection_duration)
            .max()
            .unwrap_or_default();

        // Update statistics
        self.statistics.total_merges += 1;
        self.statistics.profiles_merged += profiles.len();
        self.statistics.average_merge_time = 
            ((self.statistics.average_merge_time * (self.statistics.total_merges - 1) as u32) + 
             start_time.elapsed()) / self.statistics.total_merges as u32;

        Ok(merged)
    }

    fn calculate_merged_quality_score(&self, profiles: &[ProfileData]) -> Result<f64> {
        if profiles.is_empty() {
            return Ok(0.0);
        }

        if self.config.enable_weighted_merge {
            // Weighted average based on individual quality scores
            let total_weight: f64 = profiles.iter()
                .map(|p| p.metadata.quality_score)
                .sum();
            
            let weighted_sum: f64 = profiles.iter()
                .map(|p| p.metadata.quality_score * p.metadata.quality_score)
                .sum();

            if total_weight > 0.0 {
                Ok(weighted_sum / total_weight)
            } else {
                Ok(0.0)
            }
        } else {
            // Simple average
            let average = profiles.iter()
                .map(|p| p.metadata.quality_score)
                .sum::<f64>() / profiles.len() as f64;
            
            // Bonus for having multiple profiles
            let bonus = (profiles.len() as f64 - 1.0) * 0.1;
            Ok((average + bonus).min(1.0))
        }
    }
}
