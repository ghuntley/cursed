use crate::package_manager::{PackageManagerError, metadata::PackageMetadata, registry::PackageData};
use crate::error::Error;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write, BufReader, BufWriter};
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use sha2::{Sha256, Digest};
use fs2::FileExt;
use tracing::{info, warn, error, debug, instrument};

/// Package cache manager with LRU eviction and integrity verification
#[derive(Debug)]
pub struct PackageCache {
    cache_dir: PathBuf,
    max_size: usize,
    hit_count: usize,
    miss_count: usize,
    corruption_count: usize,
    eviction_count: usize,
}

/// Cache index entry for tracking packages and access times
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheEntry {
    pub name: String,
    pub version: String,
    pub size: usize,
    pub checksum: String,
    pub last_accessed: SystemTime,
    pub created: SystemTime,
    pub access_count: usize,
}

/// Cache index for tracking all cached packages
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CacheIndex {
    pub entries: HashMap<String, CacheEntry>, // key: "name@version"
    pub total_size: usize,
    pub last_cleanup: Option<SystemTime>,
    pub version: u32, // For future compatibility
}

impl CacheIndex {
    /// Generate cache key from package name and version
    pub fn cache_key(name: &str, version: &str) -> String {
        format!("{}@{}", name, version)
    }

    /// Check if index needs cleanup based on age and corruption
    pub fn needs_cleanup(&self) -> bool {
        if let Some(last_cleanup) = self.last_cleanup {
            if let Ok(duration) = SystemTime::now().duration_since(last_cleanup) {
                return duration > Duration::from_secs(86400); // 24 hours
            }
        }
        true // No cleanup recorded, needs cleanup
    }
}

/// Enhanced cache statistics with performance metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_packages: usize,
    pub total_size: usize,
    pub hit_count: usize,
    pub miss_count: usize,
    pub corruption_count: usize,
    pub eviction_count: usize,
    pub max_size: usize,
    pub hit_ratio: f64,
    pub average_package_size: f64,
}

impl CacheStats {
    /// Format size in human-readable format
    pub fn format_size(size: usize) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = size as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        format!("{:.1} {}", size, UNITS[unit_index])
    }
    
    /// Calculate cache usage percentage
    pub fn usage_percentage(&self) -> f64 {
        if self.max_size == 0 {
            0.0
        } else {
            (self.total_size as f64 / self.max_size as f64) * 100.0
        }
    }

    /// Calculate hit ratio
    pub fn calculate_hit_ratio(hit_count: usize, miss_count: usize) -> f64 {
        let total = hit_count + miss_count;
        if total == 0 {
            0.0
        } else {
            (hit_count as f64 / total as f64) * 100.0
        }
    }
}

impl PackageCache {
    #[instrument(fields(cache_dir = ?cache_dir, max_size = max_size))]
    pub fn new(cache_dir: PathBuf, max_size: usize) -> Result<(), Error> {
        info!("Initializing package cache");
        
        // Create cache directory structure
        fs::create_dir_all(&cache_dir)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: cache_dir.clone(), 
                error: format!("Failed to create cache directory: {}", e) 
            })?;
            
        let packages_dir = cache_dir.join("packages");
        let locks_dir = cache_dir.join("locks");
        let temp_dir = cache_dir.join("temp");
        
        for dir in [&packages_dir, &locks_dir, &temp_dir] {
            fs::create_dir_all(dir)
                .map_err(|e| PackageManagerError::FileSystemError { 
                    path: dir.clone(), 
                    error: format!("Failed to create cache subdirectory: {}", e) 
                })?;
        }
        
        let mut cache = Self { 
            cache_dir, 
            max_size,
            hit_count: 0,
            miss_count: 0,
            corruption_count: 0,
            eviction_count: 0,
        };

        // Perform initial cleanup if needed
        if cache.load_index()?.needs_cleanup() {
            info!("Performing initial cache cleanup");
            cache.clean()?;
        }
        
        Ok(cache)
    }

    /// Get path to cache index file
    fn index_path(&self) -> PathBuf {
        self.cache_dir.join("index.json")
    }

    /// Get path to package directory
    fn package_dir(&self, name: &str, version: &str) -> PathBuf {
        self.cache_dir.join("packages").join(name).join(version)
    }

    /// Get path to package data file
    fn package_data_path(&self, name: &str, version: &str) -> PathBuf {
        self.package_dir(name, version).join("package.tar.gz")
    }

    /// Get path to package metadata file
    fn package_metadata_path(&self, name: &str, version: &str) -> PathBuf {
        self.package_dir(name, version).join("metadata.json")
    }

    /// Get path to package checksum file
    fn package_checksum_path(&self, name: &str, version: &str) -> PathBuf {
        self.package_dir(name, version).join("checksum.sha256")
    }

    /// Get path to lock file for a package
    fn lock_path(&self, name: &str, version: &str) -> PathBuf {
        self.cache_dir.join("locks").join(format!("{}@{}.lock", name, version))
    }

    /// Get path to temporary file for atomic operations
    fn temp_path(&self, name: &str, version: &str) -> PathBuf {
        self.cache_dir.join("temp").join(format!("{}@{}.tmp", name, version))
    }

    /// Load cache index from disk with migration support
    #[instrument(skip(self))]
    fn load_index(&self) -> Result<(), Error> {
        let index_path = self.index_path();
        if !index_path.exists() {
            debug!("Cache index not found, creating new index");
            return Ok(CacheIndex::default());
        }

        let file = File::open(&index_path)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: index_path.clone(), 
                error: format!("Failed to open cache index: {}", e) 
            })?;

        let reader = BufReader::new(file);
        let mut index: CacheIndex = serde_json::from_reader(reader)
            .map_err(|e| PackageManagerError::InvalidMetadata { 
                reason: format!("Failed to parse cache index: {}", e) 
            })?;

        // Migrate old index format if needed
        if index.version == 0 {
            debug!("Migrating cache index to newer version");
            index.version = 1;
            index.last_cleanup = Some(SystemTime::now());
            self.save_index(&index)?;
        }
        
        Ok(index)
    }

    /// Save cache index to disk atomically
    #[instrument(skip(self, index))]
    fn save_index(&self, index: &CacheIndex) -> Result<(), Error> {
        let index_path = self.index_path();
        let temp_path = index_path.with_extension("tmp");

        // Write to temporary file first
        {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&temp_path)
                .map_err(|e| PackageManagerError::FileSystemError { 
                    path: temp_path.clone(), 
                    error: format!("Failed to create temporary index file: {}", e) 
                })?;

            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, index)
                .map_err(|e| PackageManagerError::InvalidMetadata { 
                    reason: format!("Failed to write cache index: {}", e) 
                })?;
        }

        // Atomic rename
        fs::rename(&temp_path, &index_path)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: index_path.clone(), 
                error: format!("Failed to update cache index: {}", e) 
            })?;

        debug!("Cache index saved successfully");
        Ok(())
    }

    /// Calculate SHA256 checksum of data
    fn calculate_checksum(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Verify package integrity using checksum
    #[instrument(skip(self))]
    fn verify_package(&self, name: &str, version: &str) -> Result<(), Error> {
        let data_path = self.package_data_path(name, version);
        let checksum_path = self.package_checksum_path(name, version);

        if !data_path.exists() || !checksum_path.exists() {
            debug!("Package files missing for {}@{}", name, version);
            return Ok(false);
        }

        // Read stored checksum
        let stored_checksum = fs::read_to_string(&checksum_path)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: checksum_path.clone(), 
                error: format!("Failed to read checksum: {}", e) 
            })?;
        let stored_checksum = stored_checksum.trim();

        // Calculate actual checksum
        let data = fs::read(&data_path)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: data_path.clone(), 
                error: format!("Failed to read package data: {}", e) 
            })?;
        let actual_checksum = Self::calculate_checksum(&data);

        let is_valid = stored_checksum == actual_checksum;
        if !is_valid {
            warn!("Checksum mismatch for {}@{}: expected {}, got {}", 
                  name, version, stored_checksum, actual_checksum);
        }

        Ok(is_valid)
    }

    /// Acquire exclusive lock for package operations with timeout
    #[instrument(skip(self))]
    fn acquire_lock(&self, name: &str, version: &str) -> Result<(), Error> {
        let lock_path = self.lock_path(name, version);
        let lock_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&lock_path)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: lock_path.clone(), 
                error: format!("Failed to create lock file: {}", e) 
            })?;

        // Try to acquire lock with timeout
        let start = SystemTime::now();
        let timeout = Duration::from_secs(30);
        
        loop {
            match lock_file.try_lock_exclusive() {
                Ok(()) => {
                    debug!("Acquired lock for {}@{}", name, version);
                    return Ok(lock_file);
                }
                Err(_) => {
                    if start.elapsed().unwrap_or_default() > timeout {
                        return Err(PackageManagerError::LockTimeout { 
                            package: format!("{}@{}", name, version),
                            timeout_seconds: timeout.as_secs() 
                        });
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }

    /// Evict packages using advanced LRU strategy with access frequency consideration
    #[instrument(skip(self))]
    fn evict_packages(&mut self, target_free_space: usize) -> Result<(), Error> {
        let mut index = self.load_index()?;
        
        if index.total_size + target_free_space <= self.max_size {
            return Ok(()); // No eviction needed
        }

        info!("Starting cache eviction to free {} bytes", target_free_space);

        // Sort entries by LRU with access frequency weighting
        let mut entries: Vec<_> = index.entries.values().cloned().collect();
        entries.sort_by(|a, b| {
            // Primary sort: last accessed time (older first)
            let time_cmp = a.last_accessed.cmp(&b.last_accessed);
            if time_cmp != std::cmp::Ordering::Equal {
                return time_cmp;
            }
            // Secondary sort: access count (less accessed first)
            a.access_count.cmp(&b.access_count)
        });

        let mut freed_space = 0;
        let target_to_free = (index.total_size + target_free_space) - self.max_size;

        for entry in entries {
            if freed_space >= target_to_free {
                break;
            }

            info!("Evicting package {}@{} (size: {}, last accessed: {:?})", 
                  entry.name, entry.version, entry.size, entry.last_accessed);

            // Acquire lock before removal
            let _lock = self.acquire_lock(&entry.name, &entry.version)?;

            // Remove package files
            let package_dir = self.package_dir(&entry.name, &entry.version);
            if package_dir.exists() {
                fs::remove_dir_all(&package_dir)
                    .map_err(|e| PackageManagerError::FileSystemError { 
                        path: package_dir.clone(), 
                        error: format!("Failed to remove package directory: {}", e) 
                    })?;
            }

            // Update index
            let key = CacheIndex::cache_key(&entry.name, &entry.version);
            index.entries.remove(&key);
            index.total_size = index.total_size.saturating_sub(entry.size);
            freed_space += entry.size;
            self.eviction_count += 1;
        }

        self.save_index(&index)?;
        info!("Cache eviction completed, freed {} bytes", freed_space);
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn get_package(&mut self, name: &str, version: &str) -> Result<(), Error> {
        let _lock = self.acquire_lock(name, version)?;
        
        // Check if package exists and verify integrity
        if !self.verify_package(name, version)? {
            self.miss_count += 1;
            debug!("Cache miss for {}@{}: integrity check failed", name, version);
            return Ok(None);
        }

        // Read metadata
        let metadata_path = self.package_metadata_path(name, version);
        if !metadata_path.exists() {
            self.miss_count += 1;
            debug!("Cache miss for {}@{}: metadata not found", name, version);
            return Ok(None);
        }

        let file = File::open(&metadata_path)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: metadata_path.clone(), 
                error: format!("Failed to open metadata: {}", e) 
            })?;

        let reader = BufReader::new(file);
        let metadata: PackageMetadata = serde_json::from_reader(reader)
            .map_err(|e| PackageManagerError::InvalidMetadata { 
                reason: format!("Failed to parse package metadata: {}", e) 
            })?;

        // Update access time and count in index
        let mut index = self.load_index()?;
        let key = CacheIndex::cache_key(name, version);
        if let Some(entry) = index.entries.get_mut(&key) {
            entry.last_accessed = SystemTime::now();
            entry.access_count += 1;
            self.save_index(&index)?;
        }

        self.hit_count += 1;
        debug!("Cache hit for {}@{}", name, version);
        Ok(Some(metadata))
    }

    #[instrument(skip(self, metadata, data))]
    pub fn store_package(&mut self, metadata: &PackageMetadata, data: &PackageData) -> Result<(), Error> {
        let name = &metadata.name;
        let version = &metadata.version;
        
        info!("Storing package {}@{} (size: {} bytes)", name, version, data.content.len());
        
        let _lock = self.acquire_lock(name, version)?;

        // Calculate package size and evict if necessary
        let package_size = data.content.len();
        if package_size > self.max_size {
            return Err(PackageManagerError::PackageTooLarge { 
                size: package_size, 
                max_size: self.max_size 
            });
        }

        self.evict_packages(package_size)?;

        // Create package directory
        let package_dir = self.package_dir(name, version);
        fs::create_dir_all(&package_dir)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: package_dir.clone(), 
                error: format!("Failed to create package directory: {}", e) 
            })?;

        // Write package data atomically
        let data_path = self.package_data_path(name, version);
        let temp_data_path = self.temp_path(name, version);
        
        fs::write(&temp_data_path, &data.content)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: temp_data_path.clone(), 
                error: format!("Failed to write package data: {}", e) 
            })?;
        
        fs::rename(&temp_data_path, &data_path)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: data_path.clone(), 
                error: format!("Failed to move package data: {}", e) 
            })?;

        // Write metadata
        let metadata_path = self.package_metadata_path(name, version);
        let metadata_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&metadata_path)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: metadata_path.clone(), 
                error: format!("Failed to create metadata file: {}", e) 
            })?;

        let writer = BufWriter::new(metadata_file);
        serde_json::to_writer_pretty(writer, metadata)
            .map_err(|e| PackageManagerError::InvalidMetadata { 
                reason: format!("Failed to write metadata: {}", e) 
            })?;

        // Calculate and write checksum
        let checksum = Self::calculate_checksum(&data.content);
        let checksum_path = self.package_checksum_path(name, version);
        fs::write(&checksum_path, &checksum)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: checksum_path.clone(), 
                error: format!("Failed to write checksum: {}", e) 
            })?;

        // Update cache index
        let mut index = self.load_index()?;
        let key = CacheIndex::cache_key(name, version);
        let now = SystemTime::now();
        
        let entry = CacheEntry {
            name: name.clone(),
            version: version.clone(),
            size: package_size,
            checksum,
            last_accessed: now,
            created: now,
            access_count: 1,
        };

        // If package already exists, subtract old size
        if let Some(old_entry) = index.entries.get(&key) {
            index.total_size = index.total_size.saturating_sub(old_entry.size);
        }

        index.entries.insert(key, entry);
        index.total_size += package_size;

        self.save_index(&index)?;
        info!("Successfully stored package {}@{}", name, version);
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn remove_package(&mut self, name: &str) -> Result<(), Error> {
        info!("Removing all versions of package {}", name);
        
        let mut index = self.load_index()?;
        let mut removed_entries = Vec::new();

        // Find all versions of the package
        for (key, entry) in &index.entries {
            if entry.name == name {
                removed_entries.push((key.clone(), entry.clone()));
            }
        }

        if removed_entries.is_empty() {
            debug!("Package {} not found in cache", name);
            return Ok(()); // Package not found, nothing to remove
        }

        // Remove each version
        for (key, entry) in removed_entries {
            let _lock = self.acquire_lock(&entry.name, &entry.version)?;
            
            // Remove package directory
            let package_dir = self.package_dir(&entry.name, &entry.version);
            if package_dir.exists() {
                fs::remove_dir_all(&package_dir)
                    .map_err(|e| PackageManagerError::FileSystemError { 
                        path: package_dir.clone(), 
                        error: format!("Failed to remove package directory: {}", e) 
                    })?;
            }

            // Update index
            index.entries.remove(&key);
            index.total_size = index.total_size.saturating_sub(entry.size);
            info!("Removed package {}@{}", entry.name, entry.version);
        }

        // Try to remove the parent package directory if empty
        let package_parent_dir = self.cache_dir.join("packages").join(name);
        if package_parent_dir.exists() {
            if let Ok(mut entries) = fs::read_dir(&package_parent_dir) {
                if entries.next().is_none() {
                    let _ = fs::remove_dir(&package_parent_dir); // Ignore errors
                }
            }
        }

        self.save_index(&index)?;
        info!("Successfully removed all versions of package {}", name);
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn list_packages(&self) -> Result<(), Error> {
        let index = self.load_index()?;
        let mut packages = Vec::new();

        for entry in index.entries.values() {
            // Verify package still exists and has valid integrity
            if self.verify_package(&entry.name, &entry.version)? {
                let metadata_path = self.package_metadata_path(&entry.name, &entry.version);
                if metadata_path.exists() {
                    let file = File::open(&metadata_path)
                        .map_err(|e| PackageManagerError::FileSystemError { 
                            path: metadata_path.clone(), 
                            error: format!("Failed to open metadata: {}", e) 
                        })?;

                    let reader = BufReader::new(file);
                    if let Ok(metadata) = serde_json::from_reader::<_, PackageMetadata>(reader) {
                        packages.push(metadata);
                    } else {
                        warn!("Failed to parse metadata for {}@{}", entry.name, entry.version);
                    }
                }
            } else {
                warn!("Package {}@{} failed integrity check", entry.name, entry.version);
            }
        }

        // Sort by name and version for consistent ordering
        packages.sort_by(|a, b| {
            a.name.cmp(&b.name).then_with(|| a.version.cmp(&b.version))
        });

        Ok(packages)
    }

    #[instrument(skip(self))]
    pub fn clean(&mut self) -> Result<(), Error> {
        info!("Starting cache cleanup");
        
        let mut index = self.load_index()?;
        let mut invalid_entries = Vec::new();
        let mut corrupted_count = 0;

        // Find packages with invalid checksums or missing files
        for (key, entry) in &index.entries {
            if !self.verify_package(&entry.name, &entry.version)? {
                invalid_entries.push((key.clone(), entry.clone()));
                corrupted_count += 1;
            }
        }

        // Remove invalid packages
        for (key, entry) in invalid_entries {
            let _lock = self.acquire_lock(&entry.name, &entry.version)?;
            
            let package_dir = self.package_dir(&entry.name, &entry.version);
            if package_dir.exists() {
                fs::remove_dir_all(&package_dir)
                    .map_err(|e| PackageManagerError::FileSystemError { 
                        path: package_dir.clone(), 
                        error: format!("Failed to remove corrupted package: {}", e) 
                    })?;
            }

            index.entries.remove(&key);
            index.total_size = index.total_size.saturating_sub(entry.size);
            warn!("Removed corrupted package {}@{}", entry.name, entry.version);
        }

        // Clean up empty lock files
        let locks_dir = self.cache_dir.join("locks");
        if locks_dir.exists() {
            if let Ok(entries) = fs::read_dir(&locks_dir) {
                for entry in entries.flatten() {
                    if entry.path().extension().and_then(|s| s.to_str()) == Some("lock") {
                        if let Ok(file) = File::open(&entry.path()) {
                            // Try to acquire lock to see if it's in use
                            if file.try_lock_exclusive().is_ok() {
                                let _ = fs::remove_file(&entry.path()); // Clean up unused lock
                            }
                        }
                    }
                }
            }
        }

        // Clean up empty package directories
        let packages_dir = self.cache_dir.join("packages");
        if packages_dir.exists() {
            if let Ok(entries) = fs::read_dir(&packages_dir) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        if let Ok(mut sub_entries) = fs::read_dir(&entry.path()) {
                            if sub_entries.next().is_none() {
                                let _ = fs::remove_dir(&entry.path());
                            }
                        }
                    }
                }
            }
        }

        // Clean up temporary files
        let temp_dir = self.cache_dir.join("temp");
        if temp_dir.exists() {
            if let Ok(entries) = fs::read_dir(&temp_dir) {
                for entry in entries.flatten() {
                    let _ = fs::remove_file(&entry.path());
                }
            }
        }

        // Update cleanup timestamp
        index.last_cleanup = Some(SystemTime::now());
        self.corruption_count += corrupted_count;

        self.save_index(&index)?;
        info!("Cache cleanup completed, removed {} corrupted packages", corrupted_count);
        Ok(())
    }

    pub fn stats(&self) -> Result<(), Error> {
        let index = self.load_index()?;
        let hit_ratio = CacheStats::calculate_hit_ratio(self.hit_count, self.miss_count);
        let average_package_size = if index.entries.is_empty() {
            0.0
        } else {
            index.total_size as f64 / index.entries.len() as f64
        };
        
        Ok(CacheStats {
            total_packages: index.entries.len(),
            total_size: index.total_size,
            hit_count: self.hit_count,
            miss_count: self.miss_count,
            corruption_count: self.corruption_count,
            eviction_count: self.eviction_count,
            max_size: self.max_size,
            hit_ratio,
            average_package_size,
        })
    }
    
    /// Alias for stats() method for consistency
    pub fn get_stats(&self) -> Result<(), Error> {
        self.stats()
    }

    /// Get package data (binary content) from cache
    #[instrument(skip(self))]
    pub fn get_package_data(&mut self, name: &str, version: &str) -> Result<(), Error> {
        let _lock = self.acquire_lock(name, version)?;
        
        // Check if package exists and verify integrity
        if !self.verify_package(name, version)? {
            self.miss_count += 1;
            return Ok(None);
        }

        let data_path = self.package_data_path(name, version);
        if !data_path.exists() {
            self.miss_count += 1;
            return Ok(None);
        }

        let data = fs::read(&data_path)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: data_path.clone(), 
                error: format!("Failed to read package data: {}", e) 
            })?;
        
        // Update access time and count in index
        let mut index = self.load_index()?;
        let key = CacheIndex::cache_key(name, version);
        if let Some(entry) = index.entries.get_mut(&key) {
            entry.last_accessed = SystemTime::now();
            entry.access_count += 1;
            self.save_index(&index)?;
        }

        self.hit_count += 1;
        Ok(Some(data))
    }

    /// Check if package exists in cache without updating access time
    pub fn contains_package(&self, name: &str, version: &str) -> Result<(), Error> {
        let index = self.load_index()?;
        let key = CacheIndex::cache_key(name, version);
        
        if !index.entries.contains_key(&key) {
            return Ok(false);
        }

        // Verify package integrity
        self.verify_package(name, version)
    }

    /// Get cache entry information without retrieving the package
    pub fn get_package_info(&self, name: &str, version: &str) -> Result<(), Error> {
        let index = self.load_index()?;
        let key = CacheIndex::cache_key(name, version);
        
        if let Some(entry) = index.entries.get(&key) {
            // Verify package still exists and has valid integrity
            if self.verify_package(name, version)? {
                Ok(Some(entry.clone()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Get all package entries from cache (useful for listing)
    pub fn get_all_entries(&self) -> Result<(), Error> {
        let index = self.load_index()?;
        let mut entries: Vec<CacheEntry> = index.entries.values().cloned().collect();
        
        // Sort by name and version
        entries.sort_by(|a, b| {
            a.name.cmp(&b.name).then_with(|| a.version.cmp(&b.version))
        });
        
        Ok(entries)
    }

    /// Prune cache to target size using advanced LRU strategy
    #[instrument(skip(self))]
    pub fn prune_to_size(&mut self, target_size: usize) -> Result<(), Error> {
        let index = self.load_index()?;
        if index.total_size <= target_size {
            return Ok(());
        }

        let space_to_free = index.total_size - target_size;
        info!("Pruning cache to free {} bytes", space_to_free);
        
        self.evict_packages(space_to_free)
    }

    /// Force rebuild of cache index by scanning filesystem
    #[instrument(skip(self))]
    pub fn rebuild_index(&mut self) -> Result<(), Error> {
        info!("Rebuilding cache index from filesystem");
        
        let mut new_index = CacheIndex::default();
        new_index.version = 1;
        new_index.last_cleanup = Some(SystemTime::now());

        let packages_dir = self.cache_dir.join("packages");
        if !packages_dir.exists() {
            self.save_index(&new_index)?;
            return Ok(());
        }

        // Scan package directories
        let package_entries = fs::read_dir(&packages_dir)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: packages_dir.clone(), 
                error: format!("Failed to read packages directory: {}", e) 
            })?;

        for package_entry in package_entries.flatten() {
            if !package_entry.path().is_dir() {
                continue;
            }

            let package_name = package_entry.file_name().to_string_lossy().to_string();
            
            let version_entries = fs::read_dir(&package_entry.path())
                .map_err(|e| PackageManagerError::FileSystemError { 
                    path: package_entry.path(), 
                    error: format!("Failed to read package versions: {}", e) 
                })?;

            for version_entry in version_entries.flatten() {
                if !version_entry.path().is_dir() {
                    continue;
                }

                let version = version_entry.file_name().to_string_lossy().to_string();
                
                // Verify package integrity
                if self.verify_package(&package_name, &version)? {
                    let data_path = self.package_data_path(&package_name, &version);
                    let checksum_path = self.package_checksum_path(&package_name, &version);
                    
                    if let (Ok(data), Ok(checksum)) = (fs::read(&data_path), fs::read_to_string(&checksum_path)) {
                        let metadata = version_entry.path().metadata()
                            .map_err(|e| PackageManagerError::FileSystemError { 
                                path: version_entry.path(), 
                                error: format!("Failed to get metadata: {}", e) 
                            })?;

                        let created = metadata.created().unwrap_or(SystemTime::UNIX_EPOCH);
                        let modified = metadata.modified().unwrap_or(created);

                        let key = CacheIndex::cache_key(&package_name, &version);
                        let entry = CacheEntry {
                            name: package_name.clone(),
                            version: version.clone(),
                            size: data.len(),
                            checksum: checksum.trim().to_string(),
                            last_accessed: modified,
                            created,
                            access_count: 0,
                        };

                        new_index.entries.insert(key, entry);
                        new_index.total_size += data.len();
                    }
                } else {
                    warn!("Removing corrupted package during index rebuild: {}@{}", package_name, version);
                    let package_dir = self.package_dir(&package_name, &version);
                    let _ = fs::remove_dir_all(&package_dir);
                }
            }
        }

        self.save_index(&new_index)?;
        info!("Cache index rebuilt with {} packages", new_index.entries.len());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_cache() -> (PackageCache, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let cache = PackageCache::new(temp_dir.path().to_path_buf(), 1024 * 1024).unwrap();
        (cache, temp_dir)
    }

    #[test]
    fn test_cache_creation() {
        let (cache, _temp_dir) = create_test_cache();
        assert_eq!(cache.max_size, 1024 * 1024);
        assert_eq!(cache.hit_count, 0);
        assert_eq!(cache.miss_count, 0);
    }

    #[test]
    fn test_checksum_calculation() {
        let data = b"test data";
        let checksum = PackageCache::calculate_checksum(data);
        assert!(!checksum.is_empty());
        assert_eq!(checksum.len(), 64); // SHA256 hex string length
    }

    #[test]
    fn test_cache_key_generation() {
        let key = CacheIndex::cache_key("test-package", "1.0.0");
        assert_eq!(key, "test-package@1.0.0");
    }

    #[test]
    fn test_stats_calculation() {
        let stats = CacheStats {
            total_packages: 5,
            total_size: 512,
            hit_count: 10,
            miss_count: 2,
            corruption_count: 1,
            eviction_count: 3,
            max_size: 1024,
            hit_ratio: 83.3,
            average_package_size: 102.4,
        };

        assert_eq!(stats.usage_percentage(), 50.0);
        assert_eq!(CacheStats::calculate_hit_ratio(10, 2), 83.33333333333334);
    }

    #[test]
    fn test_size_formatting() {
        assert_eq!(CacheStats::format_size(512), "512.0 B");
        assert_eq!(CacheStats::format_size(1536), "1.5 KB");
        assert_eq!(CacheStats::format_size(1048576), "1.0 MB");
    }
}
