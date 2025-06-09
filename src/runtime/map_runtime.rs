//! Runtime support for map operations in the CURSED language
//!
//! This module provides the runtime infrastructure for hash map operations including
//! creation, insertion, lookup, deletion, resizing, and integration with the garbage
//! collector. It serves as the bridge between LLVM-generated code and the map
//! implementation.

use std::ffi::c_void;
use std::ptr::{self, NonNull};
use std::sync::{Arc, RwLock, Mutex};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use tracing::{debug, error, info, warn, instrument};
use crate::memory::{GarbageCollector, ThreadSafeGc, Tag, Traceable, Visitor};
use crate::error::Error;

/// Hash function implementation using FNV-1a algorithm
///
/// This provides fast, high-quality hash values for various key types.
#[derive(Debug, Clone, Copy)]
pub struct FnvHasher {
    state: u64,
}

impl Default for FnvHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl FnvHasher {
    /// FNV-1a basis value (64-bit)
    const BASIS: u64 = 0xcbf29ce484222325;
    /// FNV-1a prime value (64-bit)
    const PRIME: u64 = 0x100000001b3;

    /// Create a new FNV hasher
    pub fn new() -> Self {
        Self {
            state: Self::BASIS,
        }
    }

    /// Hash a byte slice using FNV-1a
    pub fn hash_bytes(data: &[u8]) -> u64 {
        let mut hasher = Self::new();
        for &byte in data {
            hasher.state ^= byte as u64;
            hasher.state = hasher.state.wrapping_mul(Self::PRIME);
        }
        hasher.state
    }

    /// Hash a string using FNV-1a
    pub fn hash_string(s: &str) -> u64 {
        Self::hash_bytes(s.as_bytes())
    }

    /// Hash an integer (identity function for most cases)
    pub fn hash_int(value: i64) -> u64 {
        // For integers, we can use a simple transformation
        // to ensure good distribution while maintaining performance
        value as u64 ^ (value as u64).wrapping_mul(Self::PRIME)
    }

    /// Hash a character
    pub fn hash_char(c: char) -> u64 {
        Self::hash_int(c as u32 as i64)
    }

    /// Hash a boolean
    pub fn hash_bool(b: bool) -> u64 {
        if b { 1 } else { 0 }
    }

    /// Hash a float (bit representation)
    pub fn hash_float(f: f64) -> u64 {
        if f.is_nan() {
            // All NaNs hash to the same value
            0x7ff8000000000000
        } else if f == 0.0 {
            // Ensure +0.0 and -0.0 hash to the same value
            0
        } else {
            f.to_bits()
        }
    }
}

impl Hasher for FnvHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state ^= byte as u64;
            self.state = self.state.wrapping_mul(Self::PRIME);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

/// A key-value pair stored in the hash table
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MapEntry {
    /// Hash value of the key (cached for performance)
    pub hash: u64,
    /// Pointer to the key data
    pub key_ptr: *mut c_void,
    /// Size of the key in bytes
    pub key_size: usize,
    /// Pointer to the value data
    pub value_ptr: *mut c_void,
    /// Size of the value in bytes
    pub value_size: usize,
    /// Whether this entry is occupied
    pub occupied: bool,
    /// Whether this entry was deleted (tombstone)
    pub deleted: bool,
}

impl MapEntry {
    /// Create a new empty entry
    pub fn new() -> Self {
        Self {
            hash: 0,
            key_ptr: std::ptr::null_mut(),
            key_size: 0,
            value_ptr: std::ptr::null_mut(),
            value_size: 0,
            occupied: false,
            deleted: false,
        }
    }

    /// Create a new occupied entry
    pub fn with_data(
        hash: u64,
        key_ptr: *mut c_void,
        key_size: usize,
        value_ptr: *mut c_void,
        value_size: usize,
    ) -> Self {
        Self {
            hash,
            key_ptr,
            key_size,
            value_ptr,
            value_size,
            occupied: true,
            deleted: false,
        }
    }

    /// Check if this entry is available for insertion
    pub fn is_available(&self) -> bool {
        !self.occupied || self.deleted
    }

    /// Mark this entry as deleted (tombstone)
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.occupied = false;
    }

    /// Clear this entry completely
    pub fn clear(&mut self) {
        self.hash = 0;
        self.key_ptr = std::ptr::null_mut();
        self.key_size = 0;
        self.value_ptr = std::ptr::null_mut();
        self.value_size = 0;
        self.occupied = false;
        self.deleted = false;
    }
}

/// Hash table header structure representing a map's metadata
///
/// This follows a standard hash table representation with buckets, size, and capacity.
/// The layout is compatible with LLVM's map representation.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MapHeader {
    /// Pointer to the bucket array
    pub buckets: *mut MapEntry,
    /// Current number of entries in the map
    pub size: usize,
    /// Total capacity (number of buckets)
    pub capacity: usize,
    /// Number of deleted entries (tombstones)
    pub deleted_count: usize,
    /// Load factor threshold for resizing (as a percentage, e.g., 75 for 0.75)
    pub load_factor_threshold: u8,
}

impl MapHeader {
    /// Create a new empty map header
    pub fn new() -> Self {
        Self {
            buckets: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
            deleted_count: 0,
            load_factor_threshold: 75, // 0.75 load factor
        }
    }

    /// Create a map header with the given parameters
    pub fn with_params(
        buckets: *mut MapEntry,
        size: usize,
        capacity: usize,
        load_factor_threshold: u8,
    ) -> Self {
        Self {
            buckets,
            size,
            capacity,
            deleted_count: 0,
            load_factor_threshold,
        }
    }

    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Check if the map header is valid
    pub fn is_valid(&self) -> bool {
        if self.buckets.is_null() {
            return self.size == 0 && self.capacity == 0;
        }
        self.size <= self.capacity && self.deleted_count <= self.capacity
    }

    /// Get the current load factor (as a percentage)
    pub fn load_factor(&self) -> f64 {
        if self.capacity == 0 {
            0.0
        } else {
            (self.size + self.deleted_count) as f64 / self.capacity as f64
        }
    }

    /// Check if resizing is needed
    pub fn needs_resize(&self) -> bool {
        let current_load = self.load_factor();
        let threshold = self.load_factor_threshold as f64 / 100.0;
        current_load > threshold
    }
}

/// Statistics for map operations
#[derive(Debug, Default, Clone)]
pub struct MapStatistics {
    /// Total number of maps created
    pub maps_created: u64,
    /// Total number of insertions
    pub insertions: u64,
    /// Total number of lookups
    pub lookups: u64,
    /// Total number of successful lookups
    pub lookup_hits: u64,
    /// Total number of deletions
    pub deletions: u64,
    /// Total number of resize operations
    pub resizes: u64,
    /// Total number of hash collisions
    pub collisions: u64,
    /// Total probe distance (for open addressing performance)
    pub total_probe_distance: u64,
    /// Maximum probe distance seen
    pub max_probe_distance: usize,
    /// Total bytes allocated for maps
    pub bytes_allocated: u64,
    /// Total bytes deallocated
    pub bytes_deallocated: u64,
    /// Number of failed allocations
    pub allocation_failures: u64,
}

impl MapStatistics {
    /// Record a map creation
    pub fn record_creation(&mut self) {
        self.maps_created += 1;
    }

    /// Record an insertion
    pub fn record_insertion(&mut self, probe_distance: usize) {
        self.insertions += 1;
        self.total_probe_distance += probe_distance as u64;
        self.max_probe_distance = self.max_probe_distance.max(probe_distance);
    }

    /// Record a lookup
    pub fn record_lookup(&mut self, hit: bool, probe_distance: usize) {
        self.lookups += 1;
        if hit {
            self.lookup_hits += 1;
        }
        self.total_probe_distance += probe_distance as u64;
        self.max_probe_distance = self.max_probe_distance.max(probe_distance);
    }

    /// Record a deletion
    pub fn record_deletion(&mut self) {
        self.deletions += 1;
    }

    /// Record a resize operation
    pub fn record_resize(&mut self) {
        self.resizes += 1;
    }

    /// Record a hash collision
    pub fn record_collision(&mut self) {
        self.collisions += 1;
    }

    /// Record an allocation
    pub fn record_allocation(&mut self, bytes: usize) {
        self.bytes_allocated += bytes as u64;
    }

    /// Record a deallocation
    pub fn record_deallocation(&mut self, bytes: usize) {
        self.bytes_deallocated += bytes as u64;
    }

    /// Record an allocation failure
    pub fn record_allocation_failure(&mut self) {
        self.allocation_failures += 1;
    }

    /// Get the hit rate as a percentage
    pub fn hit_rate(&self) -> f64 {
        if self.lookups == 0 {
            0.0
        } else {
            (self.lookup_hits as f64 / self.lookups as f64) * 100.0
        }
    }

    /// Get the average probe distance
    pub fn average_probe_distance(&self) -> f64 {
        let total_operations = self.insertions + self.lookups;
        if total_operations == 0 {
            0.0
        } else {
            self.total_probe_distance as f64 / total_operations as f64
        }
    }
}

/// Configuration for map runtime behavior
#[derive(Debug, Clone)]
pub struct MapConfiguration {
    /// Default initial capacity for new maps
    pub default_capacity: usize,
    /// Load factor threshold for resizing (0.0 to 1.0)
    pub load_factor_threshold: f64,
    /// Growth factor for map expansion (e.g., 2.0 for doubling)
    pub growth_factor: f64,
    /// Maximum capacity limit to prevent excessive memory usage
    pub max_capacity: usize,
    /// Whether to enable key equality checks (expensive but safer)
    pub key_equality_checking: bool,
    /// Whether to zero memory on allocation
    pub zero_memory: bool,
    /// Whether to use linear probing (true) or quadratic probing (false)
    pub linear_probing: bool,
    /// Maximum probe distance before forcing a resize
    pub max_probe_distance: usize,
}

impl Default for MapConfiguration {
    fn default() -> Self {
        Self {
            default_capacity: 16,
            load_factor_threshold: 0.75,
            growth_factor: 2.0,
            max_capacity: usize::MAX / 2, // Prevent overflow
            key_equality_checking: true,
            zero_memory: true,
            linear_probing: true,
            max_probe_distance: 8,
        }
    }
}

/// Metadata about an allocated map
#[derive(Debug, Clone)]
struct MapMetadata {
    /// Key type size in bytes
    key_size: usize,
    /// Value type size in bytes
    value_size: usize,
    /// Current capacity
    capacity: usize,
    /// Whether the map is managed by GC
    gc_managed: bool,
    /// Creation timestamp for debugging
    created_at: std::time::Instant,
}

/// Main runtime for map operations
///
/// This provides the core functionality for managing hash maps in the CURSED language,
/// including memory allocation, hash table operations, and integration with the garbage collector.
pub struct MapRuntime {
    /// Configuration for map behavior
    config: MapConfiguration,
    /// Statistics for monitoring
    stats: Arc<Mutex<MapStatistics>>,
    /// Registry of active maps for garbage collection
    map_registry: Arc<RwLock<HashMap<*mut MapEntry, MapMetadata>>>,
    /// Reference to the garbage collector
    gc: Option<Arc<GarbageCollector>>,
}

impl MapRuntime {
    /// Create a new map runtime with default configuration
    pub fn new() -> Self {
        Self {
            config: MapConfiguration::default(),
            stats: Arc::new(Mutex::new(MapStatistics::default())),
            map_registry: Arc::new(RwLock::new(HashMap::new())),
            gc: None,
        }
    }

    /// Create a new map runtime with custom configuration
    pub fn with_config(config: MapConfiguration) -> Self {
        Self {
            config,
            stats: Arc::new(Mutex::new(MapStatistics::default())),
            map_registry: Arc::new(RwLock::new(HashMap::new())),
            gc: None,
        }
    }

    /// Set the garbage collector
    pub fn set_gc(&mut self, gc: Arc<GarbageCollector>) {
        self.gc = Some(gc);
    }

    /// Get a copy of the current statistics
    pub fn get_statistics(&self) -> MapStatistics {
        self.stats.lock().unwrap().clone()
    }

    /// Reset statistics
    pub fn reset_statistics(&self) {
        *self.stats.lock().unwrap() = MapStatistics::default();
    }

    /// Create a new map with the specified key and value sizes and initial capacity
    #[instrument(skip(self))]
    pub fn create_map(
        &self,
        key_size: usize,
        value_size: usize,
        initial_capacity: Option<usize>,
    ) -> Result<MapHeader, Error> {
        let capacity = initial_capacity
            .unwrap_or(self.config.default_capacity)
            .max(1); // Ensure minimum capacity of 1

        debug!(
            key_size = key_size,
            value_size = value_size,
            capacity = capacity,
            "Creating new map"
        );

        if capacity > self.config.max_capacity {
            self.stats.lock().unwrap().record_allocation_failure();
            return Err(Error::new(
                "MapError",
                "Map capacity exceeds maximum limit".to_string(),
                None,
            ));
        }

        // Calculate total size needed for the bucket array
        let bucket_size = std::mem::size_of::<MapEntry>();
        let total_size = bucket_size
            .checked_mul(capacity)
            .ok_or_else(|| Error::new("MapError", "Map size overflow".to_string(), None))?;

        // Allocate memory for buckets
        let buckets_ptr = if let Some(ref gc) = self.gc {
            // Use garbage collector for allocation
            self.allocate_with_gc(total_size)?
        } else {
            // Use system allocator
            self.allocate_system(total_size)?
        } as *mut MapEntry;

        // Initialize all buckets as empty
        if self.config.zero_memory {
            unsafe {
                ptr::write_bytes(buckets_ptr as *mut u8, 0, total_size);
            }
        } else {
            // Initialize each bucket explicitly
            for i in 0..capacity {
                unsafe {
                    ptr::write(buckets_ptr.add(i), MapEntry::new());
                }
            }
        }

        // Register the map
        let metadata = MapMetadata {
            key_size,
            value_size,
            capacity,
            gc_managed: self.gc.is_some(),
            created_at: std::time::Instant::now(),
        };

        {
            let mut registry = self.map_registry.write().unwrap();
            registry.insert(buckets_ptr, metadata);
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.record_creation();
            stats.record_allocation(total_size);
        }

        Ok(MapHeader::with_params(
            buckets_ptr,
            0,
            capacity,
            (self.config.load_factor_threshold * 100.0) as u8,
        ))
    }

    /// Insert a key-value pair into the map
    #[instrument(skip(self, key_data, value_data))]
    pub fn insert_entry(
        &self,
        header: &mut MapHeader,
        key_hash: u64,
        key_data: *const c_void,
        key_size: usize,
        value_data: *const c_void,
        value_size: usize,
    ) -> Result<bool, Error> {
        debug!(
            hash = key_hash,
            key_size = key_size,
            value_size = value_size,
            "Inserting entry into map"
        );

        if header.buckets.is_null() || header.capacity == 0 {
            return Err(Error::new(
                "MapError",
                "Invalid map header".to_string(),
                None,
            ));
        }

        // Check if resize is needed before insertion
        if header.needs_resize() {
            self.resize_map(header, key_size, value_size)?;
        }

        // Find insertion position using linear or quadratic probing
        let mut probe_distance = 0;
        let mut index = (key_hash as usize) % header.capacity;
        let start_index = index;

        loop {
            let entry = unsafe { &mut *header.buckets.add(index) };

            if entry.is_available() {
                // Found an available slot
                if entry.deleted {
                    // Reusing a tombstone
                    header.deleted_count -= 1;
                }

                // Allocate memory for key and value
                let key_ptr = self.allocate_key_value_memory(key_size)?;
                let value_ptr = self.allocate_key_value_memory(value_size)?;

                // Copy key and value data
                unsafe {
                    ptr::copy_nonoverlapping(key_data as *const u8, key_ptr as *mut u8, key_size);
                    ptr::copy_nonoverlapping(value_data as *const u8, value_ptr as *mut u8, value_size);
                }

                // Update the entry
                *entry = MapEntry::with_data(key_hash, key_ptr, key_size, value_ptr, value_size);

                header.size += 1;

                // Update statistics
                {
                    let mut stats = self.stats.lock().unwrap();
                    stats.record_insertion(probe_distance);
                    if probe_distance > 0 {
                        stats.record_collision();
                    }
                }

                return Ok(true);
            } else if entry.hash == key_hash && self.config.key_equality_checking {
                // Check for key equality if enabled
                if self.keys_equal(entry.key_ptr, entry.key_size, key_data, key_size) {
                    // Key already exists, update the value
                    self.deallocate_key_value_memory(entry.value_ptr, entry.value_size);

                    let value_ptr = self.allocate_key_value_memory(value_size)?;
                    unsafe {
                        ptr::copy_nonoverlapping(value_data as *const u8, value_ptr as *mut u8, value_size);
                    }

                    entry.value_ptr = value_ptr;
                    entry.value_size = value_size;

                    return Ok(false); // Key was updated, not inserted
                }
            }

            // Move to next probe position
            probe_distance += 1;
            if probe_distance > self.config.max_probe_distance {
                // Force resize due to excessive probing
                self.resize_map(header, key_size, value_size)?;
                // Retry insertion after resize
                return self.insert_entry(header, key_hash, key_data, key_size, value_data, value_size);
            }

            if self.config.linear_probing {
                index = (index + 1) % header.capacity;
            } else {
                // Quadratic probing
                index = (start_index + (probe_distance * probe_distance)) % header.capacity;
            }

            // Check for infinite loop (should not happen with proper load factor)
            if index == start_index && probe_distance > 0 {
                return Err(Error::new(
                    "MapError",
                    "Hash table is full".to_string(),
                    None,
                ));
            }
        }
    }

    /// Look up a value by key in the map
    #[instrument(skip(self, key_data))]
    pub fn lookup_entry(
        &self,
        header: &MapHeader,
        key_hash: u64,
        key_data: *const c_void,
        key_size: usize,
    ) -> Result<Option<(*const c_void, usize)>, Error> {
        debug!(
            hash = key_hash,
            key_size = key_size,
            "Looking up entry in map"
        );

        if header.buckets.is_null() || header.capacity == 0 {
            return Ok(None);
        }

        let mut probe_distance = 0;
        let mut index = (key_hash as usize) % header.capacity;
        let start_index = index;

        loop {
            let entry = unsafe { &*header.buckets.add(index) };

            if !entry.occupied && !entry.deleted {
                // Empty slot, key not found
                self.stats.lock().unwrap().record_lookup(false, probe_distance);
                return Ok(None);
            }

            if entry.occupied && entry.hash == key_hash {
                if !self.config.key_equality_checking || 
                   self.keys_equal(entry.key_ptr, entry.key_size, key_data, key_size) {
                    // Found the key
                    self.stats.lock().unwrap().record_lookup(true, probe_distance);
                    return Ok(Some((entry.value_ptr as *const c_void, entry.value_size)));
                }
            }

            // Move to next probe position
            probe_distance += 1;
            if probe_distance > self.config.max_probe_distance {
                // Give up after maximum probe distance
                self.stats.lock().unwrap().record_lookup(false, probe_distance);
                return Ok(None);
            }

            if self.config.linear_probing {
                index = (index + 1) % header.capacity;
            } else {
                // Quadratic probing
                index = (start_index + (probe_distance * probe_distance)) % header.capacity;
            }

            // Check for infinite loop
            if index == start_index && probe_distance > 0 {
                self.stats.lock().unwrap().record_lookup(false, probe_distance);
                return Ok(None);
            }
        }
    }

    /// Delete a key from the map
    #[instrument(skip(self, key_data))]
    pub fn delete_entry(
        &self,
        header: &mut MapHeader,
        key_hash: u64,
        key_data: *const c_void,
        key_size: usize,
    ) -> Result<bool, Error> {
        debug!(
            hash = key_hash,
            key_size = key_size,
            "Deleting entry from map"
        );

        if header.buckets.is_null() || header.capacity == 0 {
            return Ok(false);
        }

        let mut probe_distance = 0;
        let mut index = (key_hash as usize) % header.capacity;
        let start_index = index;

        loop {
            let entry = unsafe { &mut *header.buckets.add(index) };

            if !entry.occupied && !entry.deleted {
                // Empty slot, key not found
                return Ok(false);
            }

            if entry.occupied && entry.hash == key_hash {
                if !self.config.key_equality_checking || 
                   self.keys_equal(entry.key_ptr, entry.key_size, key_data, key_size) {
                    // Found the key, delete it
                    self.deallocate_key_value_memory(entry.key_ptr, entry.key_size);
                    self.deallocate_key_value_memory(entry.value_ptr, entry.value_size);
                    
                    entry.mark_deleted();
                    header.size -= 1;
                    header.deleted_count += 1;

                    self.stats.lock().unwrap().record_deletion();
                    return Ok(true);
                }
            }

            // Move to next probe position
            probe_distance += 1;
            if probe_distance > self.config.max_probe_distance {
                // Give up after maximum probe distance
                return Ok(false);
            }

            if self.config.linear_probing {
                index = (index + 1) % header.capacity;
            } else {
                // Quadratic probing
                index = (start_index + (probe_distance * probe_distance)) % header.capacity;
            }

            // Check for infinite loop
            if index == start_index && probe_distance > 0 {
                return Ok(false);
            }
        }
    }

    /// Resize the map to a larger capacity
    #[instrument(skip(self))]
    fn resize_map(
        &self,
        header: &mut MapHeader,
        key_size: usize,
        value_size: usize,
    ) -> Result<(), Error> {
        let old_capacity = header.capacity;
        let new_capacity = ((old_capacity as f64) * self.config.growth_factor) as usize;

        debug!(
            old_capacity = old_capacity,
            new_capacity = new_capacity,
            "Resizing map"
        );

        if new_capacity > self.config.max_capacity {
            return Err(Error::new(
                "MapError",
                "Map capacity exceeds maximum limit".to_string(),
                None,
            ));
        }

        // Create new bucket array
        let bucket_size = std::mem::size_of::<MapEntry>();
        let total_size = bucket_size * new_capacity;
        let new_buckets = if let Some(ref gc) = self.gc {
            self.allocate_with_gc(total_size)?
        } else {
            self.allocate_system(total_size)?
        } as *mut MapEntry;

        // Initialize new buckets
        if self.config.zero_memory {
            unsafe {
                ptr::write_bytes(new_buckets as *mut u8, 0, total_size);
            }
        } else {
            for i in 0..new_capacity {
                unsafe {
                    ptr::write(new_buckets.add(i), MapEntry::new());
                }
            }
        }

        // Move all entries from old buckets to new buckets
        let old_buckets = header.buckets;
        for i in 0..old_capacity {
            let old_entry = unsafe { &*old_buckets.add(i) };
            if old_entry.occupied {
                // Rehash and insert into new bucket array
                let new_index = (old_entry.hash as usize) % new_capacity;
                let mut probe_distance = 0;
                let mut index = new_index;

                loop {
                    let new_entry = unsafe { &mut *new_buckets.add(index) };
                    if !new_entry.occupied {
                        // Copy the entry
                        *new_entry = old_entry.clone();
                        break;
                    }

                    // Linear probing in new table
                    probe_distance += 1;
                    index = (index + 1) % new_capacity;

                    // Sanity check
                    if probe_distance >= new_capacity {
                        return Err(Error::new(
                            "MapError",
                            "Failed to rehash during resize".to_string(),
                            None,
                        ));
                    }
                }
            }
        }

        // Update registry
        {
            let mut registry = self.map_registry.write().unwrap();
            if let Some(mut metadata) = registry.remove(&old_buckets) {
                metadata.capacity = new_capacity;
                registry.insert(new_buckets, metadata);
            }
        }

        // Deallocate old buckets
        if !old_buckets.is_null() {
            let old_size = bucket_size * old_capacity;
            self.deallocate_memory(old_buckets as *mut c_void, old_size);
        }

        // Update header
        header.buckets = new_buckets;
        header.capacity = new_capacity;
        header.deleted_count = 0; // Reset since we only moved occupied entries

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.record_resize();
            stats.record_allocation(total_size);
            if !old_buckets.is_null() {
                stats.record_deallocation(bucket_size * old_capacity);
            }
        }

        Ok(())
    }

    /// Check if two keys are equal
    fn keys_equal(
        &self,
        key1_ptr: *const c_void,
        key1_size: usize,
        key2_ptr: *const c_void,
        key2_size: usize,
    ) -> bool {
        if key1_size != key2_size {
            return false;
        }
        
        if key1_ptr == key2_ptr {
            return true;
        }

        unsafe {
            let slice1 = std::slice::from_raw_parts(key1_ptr as *const u8, key1_size);
            let slice2 = std::slice::from_raw_parts(key2_ptr as *const u8, key2_size);
            slice1 == slice2
        }
    }

    /// Allocate memory for key or value data
    fn allocate_key_value_memory(&self, size: usize) -> Result<*mut c_void, Error> {
        if size == 0 {
            return Ok(std::ptr::null_mut());
        }

        if let Some(ref gc) = self.gc {
            self.allocate_with_gc(size)
        } else {
            self.allocate_system(size)
        }
    }

    /// Deallocate memory for key or value data
    fn deallocate_key_value_memory(&self, ptr: *mut c_void, size: usize) {
        if ptr.is_null() || size == 0 {
            return;
        }
        self.deallocate_memory(ptr, size);
    }

    /// Allocate memory using garbage collector
    fn allocate_with_gc(&self, size: usize) -> Result<*mut c_void, Error> {
        // This would integrate with the actual GC implementation
        // For now, fall back to system allocation
        self.allocate_system(size)
    }

    /// Allocate memory using system allocator
    fn allocate_system(&self, size: usize) -> Result<*mut c_void, Error> {
        use std::alloc::{alloc, Layout};

        if size == 0 {
            return Ok(std::ptr::null_mut());
        }

        let layout = Layout::from_size_align(size, 8)
            .map_err(|_| Error::new("MapError", "Invalid memory layout".to_string(), None))?;

        let ptr = unsafe { alloc(layout) };

        if ptr.is_null() {
            Err(Error::new("MapError", "Memory allocation failed".to_string(), None))
        } else {
            Ok(ptr as *mut c_void)
        }
    }

    /// Deallocate memory
    fn deallocate_memory(&self, ptr: *mut c_void, size: usize) {
        if ptr.is_null() || size == 0 {
            return;
        }

        if let Some(ref gc) = self.gc {
            // Would integrate with GC deallocation
            // For now, use system deallocation
        }

        use std::alloc::{dealloc, Layout};

        if let Ok(layout) = Layout::from_size_align(size, 8) {
            unsafe {
                dealloc(ptr as *mut u8, layout);
            }
        }
    }

    /// Deallocate a map completely
    #[instrument(skip(self))]
    pub fn deallocate_map(&self, header: &mut MapHeader, key_size: usize, value_size: usize) {
        if header.buckets.is_null() {
            return;
        }

        debug!(
            ptr = ?header.buckets,
            capacity = header.capacity,
            size = header.size,
            "Deallocating map"
        );

        // Deallocate all key-value pairs
        for i in 0..header.capacity {
            let entry = unsafe { &mut *header.buckets.add(i) };
            if entry.occupied {
                self.deallocate_key_value_memory(entry.key_ptr, entry.key_size);
                self.deallocate_key_value_memory(entry.value_ptr, entry.value_size);
            }
        }

        // Deallocate bucket array
        let bucket_size = std::mem::size_of::<MapEntry>();
        let total_size = bucket_size * header.capacity;
        self.deallocate_memory(header.buckets as *mut c_void, total_size);

        // Remove from registry
        {
            let mut registry = self.map_registry.write().unwrap();
            registry.remove(&header.buckets);
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.record_deallocation(total_size);
        }

        // Clear header
        header.buckets = std::ptr::null_mut();
        header.size = 0;
        header.capacity = 0;
        header.deleted_count = 0;
    }

    /// Get the number of registered maps
    pub fn map_count(&self) -> usize {
        self.map_registry.read().unwrap().len()
    }

    /// Clean up all maps (for shutdown)
    pub fn cleanup(&self) {
        let registry = self.map_registry.read().unwrap();
        for (buckets_ptr, metadata) in registry.iter() {
            // Deallocate all entries in each map
            for i in 0..metadata.capacity {
                let entry = unsafe { &*buckets_ptr.add(i) };
                if entry.occupied {
                    self.deallocate_key_value_memory(entry.key_ptr, entry.key_size);
                    self.deallocate_key_value_memory(entry.value_ptr, entry.value_size);
                }
            }

            // Deallocate bucket array
            let bucket_size = std::mem::size_of::<MapEntry>();
            let total_size = bucket_size * metadata.capacity;
            self.deallocate_memory(*buckets_ptr as *mut c_void, total_size);
        }
        drop(registry);

        self.map_registry.write().unwrap().clear();
    }
}

impl Default for MapRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MapRuntime {
    fn drop(&mut self) {
        self.cleanup();
    }
}

/// Thread-safe wrapper for MapRuntime
pub type ThreadSafeMapRuntime = Arc<RwLock<MapRuntime>>;

/// Create a thread-safe map runtime
pub fn create_thread_safe_map_runtime() -> ThreadSafeMapRuntime {
    Arc::new(RwLock::new(MapRuntime::new()))
}

/// Create a thread-safe map runtime with configuration
pub fn create_thread_safe_map_runtime_with_config(config: MapConfiguration) -> ThreadSafeMapRuntime {
    Arc::new(RwLock::new(MapRuntime::with_config(config)))
}

// ================================================================================================
// FFI Functions for LLVM Integration
// ================================================================================================

/// Create a new map from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_map_create(
    key_size: u64,
    value_size: u64,
    initial_capacity: u64,
    runtime_ptr: *mut c_void,
) -> MapHeader {
    debug!(
        key_size = key_size,
        value_size = value_size,
        initial_capacity = initial_capacity,
        "Creating map via FFI"
    );

    if runtime_ptr.is_null() {
        error!("Null runtime pointer in cursed_map_create");
        return MapHeader::new();
    }

    let runtime = unsafe { &*(runtime_ptr as *const MapRuntime) };

    match runtime.create_map(
        key_size as usize,
        value_size as usize,
        Some(initial_capacity as usize),
    ) {
        Ok(header) => {
            debug!(
                ptr = ?header.buckets,
                capacity = header.capacity,
                "Map created successfully"
            );
            header
        }
        Err(e) => {
            error!(error = ?e, "Failed to create map");
            MapHeader::new()
        }
    }
}

/// Insert a key-value pair into a map from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_map_insert(
    header_ptr: *mut MapHeader,
    key_hash: u64,
    key_data: *const c_void,
    key_size: u64,
    value_data: *const c_void,
    value_size: u64,
    runtime_ptr: *mut c_void,
) -> i32 {
    if header_ptr.is_null() || runtime_ptr.is_null() {
        error!("Null pointer in cursed_map_insert");
        return -1;
    }

    let header = unsafe { &mut *header_ptr };
    let runtime = unsafe { &*(runtime_ptr as *const MapRuntime) };

    match runtime.insert_entry(
        header,
        key_hash,
        key_data,
        key_size as usize,
        value_data,
        value_size as usize,
    ) {
        Ok(inserted) => {
            debug!(
                inserted = inserted,
                hash = key_hash,
                "Entry inserted successfully"
            );
            if inserted { 1 } else { 0 } // 1 for new insertion, 0 for update
        }
        Err(e) => {
            error!(error = ?e, "Failed to insert entry");
            -1
        }
    }
}

/// Look up a value by key in a map from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_map_lookup(
    header_ptr: *const MapHeader,
    key_hash: u64,
    key_data: *const c_void,
    key_size: u64,
    value_ptr_out: *mut *const c_void,
    value_size_out: *mut u64,
    runtime_ptr: *mut c_void,
) -> i32 {
    if header_ptr.is_null() || runtime_ptr.is_null() || value_ptr_out.is_null() || value_size_out.is_null() {
        error!("Null pointer in cursed_map_lookup");
        return -1;
    }

    let header = unsafe { &*header_ptr };
    let runtime = unsafe { &*(runtime_ptr as *const MapRuntime) };

    match runtime.lookup_entry(header, key_hash, key_data, key_size as usize) {
        Ok(Some((value_ptr, value_size))) => {
            unsafe {
                *value_ptr_out = value_ptr;
                *value_size_out = value_size as u64;
            }
            debug!(
                hash = key_hash,
                value_size = value_size,
                "Entry found successfully"
            );
            1 // Found
        }
        Ok(None) => {
            unsafe {
                *value_ptr_out = std::ptr::null();
                *value_size_out = 0;
            }
            debug!(hash = key_hash, "Entry not found");
            0 // Not found
        }
        Err(e) => {
            error!(error = ?e, "Failed to lookup entry");
            unsafe {
                *value_ptr_out = std::ptr::null();
                *value_size_out = 0;
            }
            -1 // Error
        }
    }
}

/// Delete a key from a map from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_map_delete(
    header_ptr: *mut MapHeader,
    key_hash: u64,
    key_data: *const c_void,
    key_size: u64,
    runtime_ptr: *mut c_void,
) -> i32 {
    if header_ptr.is_null() || runtime_ptr.is_null() {
        error!("Null pointer in cursed_map_delete");
        return -1;
    }

    let header = unsafe { &mut *header_ptr };
    let runtime = unsafe { &*(runtime_ptr as *const MapRuntime) };

    match runtime.delete_entry(header, key_hash, key_data, key_size as usize) {
        Ok(deleted) => {
            debug!(
                deleted = deleted,
                hash = key_hash,
                "Delete operation completed"
            );
            if deleted { 1 } else { 0 } // 1 for deleted, 0 for not found
        }
        Err(e) => {
            error!(error = ?e, "Failed to delete entry");
            -1
        }
    }
}

/// Deallocate a map from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_map_deallocate(
    header_ptr: *mut MapHeader,
    key_size: u64,
    value_size: u64,
    runtime_ptr: *mut c_void,
) {
    if header_ptr.is_null() || runtime_ptr.is_null() {
        error!("Null pointer in cursed_map_deallocate");
        return;
    }

    let header = unsafe { &mut *header_ptr };
    let runtime = unsafe { &*(runtime_ptr as *const MapRuntime) };

    debug!(
        ptr = ?header.buckets,
        capacity = header.capacity,
        size = header.size,
        "Deallocating map"
    );
    runtime.deallocate_map(header, key_size as usize, value_size as usize);
}

/// Get map size from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_map_size(header_ptr: *const MapHeader) -> u64 {
    if header_ptr.is_null() {
        error!("Null pointer in cursed_map_size");
        return 0;
    }

    let header = unsafe { &*header_ptr };
    header.size as u64
}

/// Check if map is empty from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_map_is_empty(header_ptr: *const MapHeader) -> i32 {
    if header_ptr.is_null() {
        error!("Null pointer in cursed_map_is_empty");
        return 1; // Consider null map as empty
    }

    let header = unsafe { &*header_ptr };
    if header.is_empty() { 1 } else { 0 }
}

/// Hash a string using FNV-1a from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_hash_string(data: *const u8, len: u64) -> u64 {
    if data.is_null() || len == 0 {
        return 0;
    }

    let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };
    FnvHasher::hash_bytes(slice)
}

/// Hash an integer from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_hash_int(value: i64) -> u64 {
    FnvHasher::hash_int(value)
}

/// Hash a character from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_hash_char(value: u32) -> u64 {
    FnvHasher::hash_char(char::from_u32(value).unwrap_or('\0'))
}

/// Hash a boolean from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_hash_bool(value: i32) -> u64 {
    FnvHasher::hash_bool(value != 0)
}

/// Hash a float from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_hash_float(value: f64) -> u64 {
    FnvHasher::hash_float(value)
}

/// Hash arbitrary bytes from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_hash_bytes(data: *const u8, len: u64) -> u64 {
    if data.is_null() || len == 0 {
        return 0;
    }

    let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };
    FnvHasher::hash_bytes(slice)
}

/// Create a global map runtime instance for FFI
static mut GLOBAL_MAP_RUNTIME: Option<MapRuntime> = None;
static GLOBAL_MAP_RUNTIME_INIT: std::sync::Once = std::sync::Once::new();

/// Initialize the global map runtime
#[no_mangle]
pub extern "C" fn cursed_map_runtime_init() -> *mut c_void {
    GLOBAL_MAP_RUNTIME_INIT.call_once(|| {
        unsafe {
            GLOBAL_MAP_RUNTIME = Some(MapRuntime::new());
        }
    });

    unsafe {
        GLOBAL_MAP_RUNTIME.as_ref().unwrap() as *const MapRuntime as *mut c_void
    }
}

/// Get the global map runtime pointer
#[no_mangle]
pub extern "C" fn cursed_map_runtime_get() -> *mut c_void {
    unsafe {
        match GLOBAL_MAP_RUNTIME.as_ref() {
            Some(runtime) => runtime as *const MapRuntime as *mut c_void,
            None => {
                error!("Global map runtime not initialized");
                std::ptr::null_mut()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_fnv_hasher() {
        // Test string hashing
        let hash1 = FnvHasher::hash_string("hello");
        let hash2 = FnvHasher::hash_string("hello");
        let hash3 = FnvHasher::hash_string("world");
        
        assert_eq!(hash1, hash2, "Same strings should hash to same value");
        assert_ne!(hash1, hash3, "Different strings should hash to different values");

        // Test integer hashing
        let int_hash1 = FnvHasher::hash_int(42);
        let int_hash2 = FnvHasher::hash_int(42);
        let int_hash3 = FnvHasher::hash_int(43);
        
        assert_eq!(int_hash1, int_hash2, "Same integers should hash to same value");
        assert_ne!(int_hash1, int_hash3, "Different integers should hash to different values");

        // Test character hashing
        let char_hash1 = FnvHasher::hash_char('a');
        let char_hash2 = FnvHasher::hash_char('a');
        let char_hash3 = FnvHasher::hash_char('b');
        
        assert_eq!(char_hash1, char_hash2, "Same characters should hash to same value");
        assert_ne!(char_hash1, char_hash3, "Different characters should hash to different values");

        // Test boolean hashing
        let bool_hash1 = FnvHasher::hash_bool(true);
        let bool_hash2 = FnvHasher::hash_bool(true);
        let bool_hash3 = FnvHasher::hash_bool(false);
        
        assert_eq!(bool_hash1, bool_hash2, "Same booleans should hash to same value");
        assert_ne!(bool_hash1, bool_hash3, "Different booleans should hash to different values");

        // Test float hashing
        let float_hash1 = FnvHasher::hash_float(3.14);
        let float_hash2 = FnvHasher::hash_float(3.14);
        let float_hash3 = FnvHasher::hash_float(2.71);
        
        assert_eq!(float_hash1, float_hash2, "Same floats should hash to same value");
        assert_ne!(float_hash1, float_hash3, "Different floats should hash to different values");

        // Test NaN handling
        let nan_hash1 = FnvHasher::hash_float(f64::NAN);
        let nan_hash2 = FnvHasher::hash_float(f64::NAN);
        assert_eq!(nan_hash1, nan_hash2, "All NaNs should hash to same value");

        // Test zero handling
        let zero_hash1 = FnvHasher::hash_float(0.0);
        let zero_hash2 = FnvHasher::hash_float(-0.0);
        assert_eq!(zero_hash1, zero_hash2, "+0.0 and -0.0 should hash to same value");
    }

    #[test]
    fn test_map_entry() {
        let mut entry = MapEntry::new();
        assert!(!entry.occupied);
        assert!(!entry.deleted);
        assert!(entry.is_available());

        entry.occupied = true;
        assert!(!entry.is_available());

        entry.mark_deleted();
        assert!(!entry.occupied);
        assert!(entry.deleted);
        assert!(entry.is_available());

        entry.clear();
        assert!(!entry.occupied);
        assert!(!entry.deleted);
        assert!(entry.is_available());
    }

    #[test]
    fn test_map_header() {
        let header = MapHeader::new();
        assert!(header.buckets.is_null());
        assert_eq!(header.size, 0);
        assert_eq!(header.capacity, 0);
        assert!(header.is_empty());
        assert!(header.is_valid());
        assert_eq!(header.load_factor(), 0.0);
        assert!(!header.needs_resize());

        let mut header = MapHeader::with_params(
            std::ptr::null_mut(),
            5,
            10,
            75,
        );
        assert_eq!(header.size, 5);
        assert_eq!(header.capacity, 10);
        assert!(!header.is_empty());
        assert!(header.is_valid());
        assert_eq!(header.load_factor(), 0.5);
        assert!(!header.needs_resize());

        // Test resize threshold
        header.size = 8;
        assert!(header.needs_resize());
    }

    #[test]
    fn test_map_runtime_creation() {
        let runtime = MapRuntime::new();
        assert_eq!(runtime.map_count(), 0);
        
        let stats = runtime.get_statistics();
        assert_eq!(stats.maps_created, 0);
        assert_eq!(stats.insertions, 0);
        assert_eq!(stats.lookups, 0);
    }

    #[test]
    fn test_map_creation() {
        let runtime = MapRuntime::new();
        let result = runtime.create_map(4, 8, Some(16));
        
        assert!(result.is_ok());
        let header = result.unwrap();
        assert!(!header.buckets.is_null());
        assert_eq!(header.size, 0);
        assert_eq!(header.capacity, 16);
        assert!(header.is_valid());
        assert_eq!(runtime.map_count(), 1);
    }

    #[test]
    fn test_map_insert_and_lookup() {
        let runtime = MapRuntime::new();
        let mut header = runtime.create_map(4, 4, Some(8)).unwrap();

        // Test insertion
        let key = 42i32;
        let value = 100i32;
        let key_hash = FnvHasher::hash_int(key as i64);

        let result = runtime.insert_entry(
            &mut header,
            key_hash,
            &key as *const i32 as *const c_void,
            4,
            &value as *const i32 as *const c_void,
            4,
        );

        assert!(result.is_ok());
        assert!(result.unwrap()); // Should be a new insertion
        assert_eq!(header.size, 1);

        // Test lookup
        let lookup_result = runtime.lookup_entry(
            &header,
            key_hash,
            &key as *const i32 as *const c_void,
            4,
        );

        assert!(lookup_result.is_ok());
        let (value_ptr, value_size) = lookup_result.unwrap().unwrap();
        assert_eq!(value_size, 4);
        
        let retrieved_value = unsafe { *(value_ptr as *const i32) };
        assert_eq!(retrieved_value, value);

        // Test lookup of non-existent key
        let nonexistent_key = 999i32;
        let nonexistent_hash = FnvHasher::hash_int(nonexistent_key as i64);
        
        let lookup_result = runtime.lookup_entry(
            &header,
            nonexistent_hash,
            &nonexistent_key as *const i32 as *const c_void,
            4,
        );

        assert!(lookup_result.is_ok());
        assert!(lookup_result.unwrap().is_none());
    }

    #[test]
    fn test_map_update_existing_key() {
        let runtime = MapRuntime::new();
        let mut header = runtime.create_map(4, 4, Some(8)).unwrap();

        let key = 42i32;
        let value1 = 100i32;
        let value2 = 200i32;
        let key_hash = FnvHasher::hash_int(key as i64);

        // Insert initial value
        let result1 = runtime.insert_entry(
            &mut header,
            key_hash,
            &key as *const i32 as *const c_void,
            4,
            &value1 as *const i32 as *const c_void,
            4,
        );
        assert!(result1.is_ok());
        assert!(result1.unwrap()); // New insertion

        // Update with new value
        let result2 = runtime.insert_entry(
            &mut header,
            key_hash,
            &key as *const i32 as *const c_void,
            4,
            &value2 as *const i32 as *const c_void,
            4,
        );
        assert!(result2.is_ok());
        assert!(!result2.unwrap()); // Update, not new insertion

        // Verify updated value
        let lookup_result = runtime.lookup_entry(
            &header,
            key_hash,
            &key as *const i32 as *const c_void,
            4,
        );

        assert!(lookup_result.is_ok());
        let (value_ptr, _) = lookup_result.unwrap().unwrap();
        let retrieved_value = unsafe { *(value_ptr as *const i32) };
        assert_eq!(retrieved_value, value2);
        assert_eq!(header.size, 1); // Size should not change on update
    }

    #[test]
    fn test_map_deletion() {
        let runtime = MapRuntime::new();
        let mut header = runtime.create_map(4, 4, Some(8)).unwrap();

        let key = 42i32;
        let value = 100i32;
        let key_hash = FnvHasher::hash_int(key as i64);

        // Insert a key
        runtime.insert_entry(
            &mut header,
            key_hash,
            &key as *const i32 as *const c_void,
            4,
            &value as *const i32 as *const c_void,
            4,
        ).unwrap();

        assert_eq!(header.size, 1);

        // Delete the key
        let delete_result = runtime.delete_entry(
            &mut header,
            key_hash,
            &key as *const i32 as *const c_void,
            4,
        );

        assert!(delete_result.is_ok());
        assert!(delete_result.unwrap()); // Should be deleted
        assert_eq!(header.size, 0);
        assert_eq!(header.deleted_count, 1);

        // Verify key is no longer found
        let lookup_result = runtime.lookup_entry(
            &header,
            key_hash,
            &key as *const i32 as *const c_void,
            4,
        );

        assert!(lookup_result.is_ok());
        assert!(lookup_result.unwrap().is_none());

        // Try to delete again
        let delete_result2 = runtime.delete_entry(
            &mut header,
            key_hash,
            &key as *const i32 as *const c_void,
            4,
        );

        assert!(delete_result2.is_ok());
        assert!(!delete_result2.unwrap()); // Should not be deleted (not found)
    }

    #[test]
    fn test_map_multiple_entries() {
        let runtime = MapRuntime::new();
        let mut header = runtime.create_map(4, 4, Some(16)).unwrap();

        // Insert multiple key-value pairs
        for i in 0..10 {
            let key = i;
            let value = i * 10;
            let key_hash = FnvHasher::hash_int(key as i64);

            let result = runtime.insert_entry(
                &mut header,
                key_hash,
                &key as *const i32 as *const c_void,
                4,
                &value as *const i32 as *const c_void,
                4,
            );

            assert!(result.is_ok());
            assert!(result.unwrap());
        }

        assert_eq!(header.size, 10);

        // Verify all entries can be found
        for i in 0..10 {
            let key = i;
            let expected_value = i * 10;
            let key_hash = FnvHasher::hash_int(key as i64);

            let lookup_result = runtime.lookup_entry(
                &header,
                key_hash,
                &key as *const i32 as *const c_void,
                4,
            );

            assert!(lookup_result.is_ok());
            let (value_ptr, _) = lookup_result.unwrap().unwrap();
            let retrieved_value = unsafe { *(value_ptr as *const i32) };
            assert_eq!(retrieved_value, expected_value);
        }
    }

    #[test]
    fn test_map_statistics() {
        let runtime = MapRuntime::new();
        let mut header = runtime.create_map(4, 4, Some(8)).unwrap();

        let stats_before = runtime.get_statistics();
        assert_eq!(stats_before.maps_created, 1);
        assert_eq!(stats_before.insertions, 0);
        assert_eq!(stats_before.lookups, 0);

        // Insert some entries
        for i in 0..5 {
            let key = i;
            let value = i * 10;
            let key_hash = FnvHasher::hash_int(key as i64);

            runtime.insert_entry(
                &mut header,
                key_hash,
                &key as *const i32 as *const c_void,
                4,
                &value as *const i32 as *const c_void,
                4,
            ).unwrap();
        }

        // Perform some lookups
        for i in 0..5 {
            let key = i;
            let key_hash = FnvHasher::hash_int(key as i64);

            runtime.lookup_entry(
                &header,
                key_hash,
                &key as *const i32 as *const c_void,
                4,
            ).unwrap();
        }

        let stats_after = runtime.get_statistics();
        assert_eq!(stats_after.maps_created, 1);
        assert_eq!(stats_after.insertions, 5);
        assert_eq!(stats_after.lookups, 5);
        assert_eq!(stats_after.lookup_hits, 5);
        assert_eq!(stats_after.hit_rate(), 100.0);
    }

    #[test]
    fn test_map_string_keys() {
        let runtime = MapRuntime::new();
        let mut header = runtime.create_map(16, 4, Some(8)).unwrap();

        let key = "hello";
        let value = 42i32;
        let key_hash = FnvHasher::hash_string(key);

        // Insert string key
        let result = runtime.insert_entry(
            &mut header,
            key_hash,
            key.as_ptr() as *const c_void,
            key.len(),
            &value as *const i32 as *const c_void,
            4,
        );

        assert!(result.is_ok());
        assert!(result.unwrap());

        // Lookup string key
        let lookup_result = runtime.lookup_entry(
            &header,
            key_hash,
            key.as_ptr() as *const c_void,
            key.len(),
        );

        assert!(lookup_result.is_ok());
        let (value_ptr, _) = lookup_result.unwrap().unwrap();
        let retrieved_value = unsafe { *(value_ptr as *const i32) };
        assert_eq!(retrieved_value, value);
    }

    #[test]
    fn test_ffi_hash_functions() {
        // Test string hashing FFI
        let test_str = "hello world";
        let hash1 = cursed_hash_string(test_str.as_ptr(), test_str.len() as u64);
        let hash2 = FnvHasher::hash_string(test_str);
        assert_eq!(hash1, hash2);

        // Test integer hashing FFI
        let test_int = 12345i64;
        let hash3 = cursed_hash_int(test_int);
        let hash4 = FnvHasher::hash_int(test_int);
        assert_eq!(hash3, hash4);

        // Test character hashing FFI
        let test_char = 'A' as u32;
        let hash5 = cursed_hash_char(test_char);
        let hash6 = FnvHasher::hash_char('A');
        assert_eq!(hash5, hash6);

        // Test boolean hashing FFI
        let hash7 = cursed_hash_bool(1);
        let hash8 = FnvHasher::hash_bool(true);
        assert_eq!(hash7, hash8);

        let hash9 = cursed_hash_bool(0);
        let hash10 = FnvHasher::hash_bool(false);
        assert_eq!(hash9, hash10);

        // Test float hashing FFI
        let test_float = 3.14159f64;
        let hash11 = cursed_hash_float(test_float);
        let hash12 = FnvHasher::hash_float(test_float);
        assert_eq!(hash11, hash12);
    }

    #[test]
    fn test_map_configuration() {
        let config = MapConfiguration {
            default_capacity: 32,
            load_factor_threshold: 0.5,
            growth_factor: 3.0,
            max_capacity: 1000,
            key_equality_checking: false,
            zero_memory: false,
            linear_probing: false,
            max_probe_distance: 16,
        };

        let runtime = MapRuntime::with_config(config.clone());
        assert_eq!(runtime.config.default_capacity, 32);
        assert_eq!(runtime.config.load_factor_threshold, 0.5);
        assert_eq!(runtime.config.growth_factor, 3.0);
        assert!(!runtime.config.key_equality_checking);
        assert!(!runtime.config.linear_probing);
    }

    #[test]
    fn test_map_cleanup() {
        let runtime = MapRuntime::new();
        
        // Create multiple maps
        for i in 0..5 {
            runtime.create_map(4, 4, Some(8)).unwrap();
        }

        assert_eq!(runtime.map_count(), 5);

        // Cleanup should clear all maps
        runtime.cleanup();
        assert_eq!(runtime.map_count(), 0);
    }
}
