//! Profile data storage for CURSED PGO system

use crate::error::{CursedError, Result};
use crate::optimization::pgo::profile_collector::ProfileData;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// Configuration for profile storage
#[derive(Debug, Clone)]
pub struct ProfileStorageConfig {
    pub storage_directory: PathBuf,
    pub max_profiles: usize,
    pub max_profile_age: Duration,
    pub compression_enabled: bool,
    pub backup_enabled: bool,
    pub format_version: u32,
}

impl Default for ProfileStorageConfig {
    fn default() -> Self {
        Self {
            storage_directory: PathBuf::from("target/pgo-profiles"),
            max_profiles: 100,
            max_profile_age: Duration::from_secs(30 * 24 * 3600), // 30 days
            compression_enabled: true,
            backup_enabled: true,
            format_version: 1,
        }
    }
}

/// Profile storage system
pub struct ProfileStorage {
    config: ProfileStorageConfig,
    metadata: HashMap<String, ProfileMetadata>,
}

/// Metadata for a stored profile
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProfileMetadata {
    pub profile_id: String,
    pub creation_time: SystemTime,
    pub file_path: PathBuf,
    pub file_size: u64,
    pub checksum: String,
    pub format_version: u32,
    pub compression_type: Option<String>,
    pub description: Option<String>,
}

impl ProfileStorage {
    /// Create a new profile storage system
    pub fn new(config: ProfileStorageConfig) -> Result<Self> {
        let mut storage = Self {
            config,
            metadata: HashMap::new(),
        };
        
        storage.initialize()?;
        Ok(storage)
    }

    /// Initialize storage system
    pub fn initialize(&mut self) -> Result<()> {
        // Create storage directory
        std::fs::create_dir_all(&self.config.storage_directory).map_err(|e| {
            CursedError::General(format!("Failed to create storage directory: {}", e))
        })?;

        // Load existing profile metadata
        self.load_metadata()?;
        
        // Clean up old profiles
        self.cleanup_old_profiles()?;
        
        tracing::info!("Profile storage initialized at: {}", self.config.storage_directory.display());
        Ok(())
    }

    /// Store profile data
    pub fn store_profile(&mut self, profile_data: &ProfileData, profile_id: &str) -> Result<()> {
        let profile_path = self.config.storage_directory.join(format!("{}.profile", profile_id));
        
        // Serialize profile data
        let serialized = self.serialize_profile(profile_data)?;
        
        // Apply compression if enabled
        let data = if self.config.compression_enabled {
            self.compress_data(&serialized)?
        } else {
            serialized
        };
        
        // Write to file
        std::fs::write(&profile_path, &data).map_err(|e| {
            CursedError::General(format!("Failed to write profile: {}", e))
        })?;
        
        // Calculate checksum
        let checksum = self.calculate_checksum(&data);
        
        // Store metadata
        let metadata = ProfileMetadata {
            profile_id: profile_id.to_string(),
            creation_time: SystemTime::now(),
            file_path: profile_path,
            file_size: data.len() as u64,
            checksum,
            format_version: self.config.format_version,
            compression_type: if self.config.compression_enabled { 
                Some("deflate".to_string()) 
            } else { 
                None 
            },
            description: None,
        };
        
        self.metadata.insert(profile_id.to_string(), metadata);
        self.save_metadata()?;
        
        tracing::info!("Profile stored: {}", profile_id);
        Ok(())
    }

    /// Load profile data
    pub fn load_profile(&self, profile_id: &str) -> Result<ProfileData> {
        let metadata = self.metadata.get(profile_id)
            .ok_or_else(|| CursedError::General(format!("Profile not found: {}", profile_id)))?;
        
        // Read file
        let data = std::fs::read(&metadata.file_path).map_err(|e| {
            CursedError::General(format!("Failed to read profile: {}", e))
        })?;
        
        // Verify checksum
        let checksum = self.calculate_checksum(&data);
        if checksum != metadata.checksum {
            return Err(CursedError::General("Profile checksum mismatch".to_string()));
        }
        
        // Decompress if needed
        let decompressed = if metadata.compression_type.is_some() {
            self.decompress_data(&data)?
        } else {
            data
        };
        
        // Deserialize profile data
        let profile_data = self.deserialize_profile(&decompressed)?;
        
        tracing::info!("Profile loaded: {}", profile_id);
        Ok(profile_data)
    }

    /// List all stored profiles
    pub fn list_profiles(&self) -> Vec<String> {
        self.metadata.keys().cloned().collect()
    }

    /// Delete a profile
    pub fn delete_profile(&mut self, profile_id: &str) -> Result<()> {
        if let Some(metadata) = self.metadata.remove(profile_id) {
            // Delete file
            if metadata.file_path.exists() {
                std::fs::remove_file(&metadata.file_path).map_err(|e| {
                    CursedError::General(format!("Failed to delete profile file: {}", e))
                })?;
            }
            
            self.save_metadata()?;
            tracing::info!("Profile deleted: {}", profile_id);
        }
        
        Ok(())
    }

    /// Get storage statistics
    pub fn get_statistics(&self) -> StorageStatistics {
        let total_size = self.metadata.values().map(|m| m.file_size).sum();
        let oldest_profile = self.metadata.values()
            .min_by_key(|m| m.creation_time)
            .map(|m| m.creation_time);
        
        StorageStatistics {
            total_profiles: self.metadata.len(),
            total_size_bytes: total_size,
            oldest_profile_time: oldest_profile,
            storage_directory: self.config.storage_directory.clone(),
        }
    }

    /// Serialize profile data to bytes
    fn serialize_profile(&self, profile_data: &ProfileData) -> Result<Vec<u8>> {
        // Simple JSON serialization for now
        let json = serde_json::to_string(profile_data).map_err(|e| {
            CursedError::General(format!("Failed to serialize profile: {}", e))
        })?;
        
        Ok(json.into_bytes())
    }

    /// Deserialize profile data from bytes
    fn deserialize_profile(&self, data: &[u8]) -> Result<ProfileData> {
        let json = String::from_utf8(data.to_vec()).map_err(|e| {
            CursedError::General(format!("Invalid UTF-8 in profile data: {}", e))
        })?;
        
        serde_json::from_str(&json).map_err(|e| {
            CursedError::General(format!("Failed to deserialize profile: {}", e))
        })
    }

    /// Compress data using deflate
    fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        use flate2::write::DeflateEncoder;
        use flate2::Compression;
        use std::io::Write;
        
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data).map_err(|e| {
            CursedError::General(format!("Compression failed: {}", e))
        })?;
        
        encoder.finish().map_err(|e| {
            CursedError::General(format!("Compression finalization failed: {}", e))
        })
    }

    /// Decompress data
    fn decompress_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        use flate2::read::DeflateDecoder;
        use std::io::Read;
        
        let mut decoder = DeflateDecoder::new(data);
        let mut result = Vec::new();
        decoder.read_to_end(&mut result).map_err(|e| {
            CursedError::General(format!("Decompression failed: {}", e))
        })?;
        
        Ok(result)
    }

    /// Calculate checksum for data
    fn calculate_checksum(&self, data: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Load metadata from disk
    fn load_metadata(&mut self) -> Result<()> {
        let metadata_path = self.config.storage_directory.join("metadata.json");
        
        if !metadata_path.exists() {
            return Ok(());
        }
        
        let data = std::fs::read_to_string(&metadata_path).map_err(|e| {
            CursedError::General(format!("Failed to read metadata: {}", e))
        })?;
        
        self.metadata = serde_json::from_str(&data).map_err(|e| {
            CursedError::General(format!("Failed to parse metadata: {}", e))
        })?;
        
        Ok(())
    }

    /// Save metadata to disk
    fn save_metadata(&self) -> Result<()> {
        let metadata_path = self.config.storage_directory.join("metadata.json");
        
        let data = serde_json::to_string_pretty(&self.metadata).map_err(|e| {
            CursedError::General(format!("Failed to serialize metadata: {}", e))
        })?;
        
        std::fs::write(&metadata_path, data).map_err(|e| {
            CursedError::General(format!("Failed to write metadata: {}", e))
        })?;
        
        Ok(())
    }

    /// Clean up old profiles
    fn cleanup_old_profiles(&mut self) -> Result<()> {
        let now = SystemTime::now();
        let mut to_remove = Vec::new();
        
        for (profile_id, metadata) in &self.metadata {
            if let Ok(age) = now.duration_since(metadata.creation_time) {
                if age > self.config.max_profile_age {
                    to_remove.push(profile_id.clone());
                }
            }
        }
        
        for profile_id in to_remove {
            self.delete_profile(&profile_id)?;
        }
        
        // Also enforce max profile count
        if self.metadata.len() > self.config.max_profiles {
            let mut sorted_profiles: Vec<_> = self.metadata.values().collect();
            sorted_profiles.sort_by_key(|m| m.creation_time);
            
            let to_remove_count = self.metadata.len() - self.config.max_profiles;
            let to_remove_ids: Vec<String> = sorted_profiles.iter()
                .take(to_remove_count)
                .map(|m| m.profile_id.clone())
                .collect();
                
            for profile_id in to_remove_ids {
                self.delete_profile(&profile_id)?;
            }
        }
        
        Ok(())
    }
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStatistics {
    pub total_profiles: usize,
    pub total_size_bytes: u64,
    pub oldest_profile_time: Option<SystemTime>,
    pub storage_directory: PathBuf,
}

// Re-export for compatibility  
pub use crate::optimization::pgo::profile_collector::ProfileData as StoredProfileData;
