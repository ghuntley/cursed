/// Object Metadata and Header Management for CURSED Memory System
/// 
/// This module provides comprehensive metadata management for heap objects including:
/// 
/// 1. **Object Headers**: Standard headers with type information and GC state
/// 2. **Metadata Management**: Centralized tracking of object metadata
/// 3. **Memory Layout**: Proper alignment and size calculations
/// 4. **Type Information**: Runtime type identification for GC and debugging
/// 5. **Object Lifecycle**: Creation, access, and cleanup of metadata
/// 
/// The design ensures minimal overhead while providing rich debugging information
/// and enabling precise garbage collection through accurate type tracking.

use std::ptr::NonNull;
use std::sync::{Arc, RwLock, Mutex};
use std::collections::HashMap;
use std::mem;
use tracing::{instrument, debug, warn, error};

use crate::memory::object_id::{ObjectId, ObjectIdGenerator, SharedObjectRegistry};
use crate::memory::Tag;

/// Standard object header placed before each allocated object
/// 
/// This header contains essential information for garbage collection,
/// type identification, and memory management. It's designed to be
/// cache-friendly and provide fast access to commonly needed data.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ObjectHeader {
    /// Object identifier for GC tracking
    /// Size of the object (excluding header)
    /// Type tag for runtime type identification
    /// GC mark bit (0 = white, 1 = marked)
    /// Object generation (for generational GC)
    /// Reference count (for hybrid GC strategies)
    /// Type name hash for fast type comparison
    /// Padding to ensure proper alignment
impl ObjectHeader {
    /// Create a new object header
    pub fn new(object_id: ObjectId, size: usize, type_tag: Tag, type_name: &str) -> Self {
        Self {
        }
    }
    
    /// Hash function for type names
    fn hash_type_name(type_name: &str) -> u32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        type_name.hash(&mut hasher);
        hasher.finish() as u32
    /// Check if object is marked for GC
    pub fn is_marked(&self) -> bool {
        self.mark_bit != 0
    /// Mark object for GC
    pub fn mark(&mut self) {
        self.mark_bit = 1;
    /// Unmark object for GC
    pub fn unmark(&mut self) {
        self.mark_bit = 0;
    /// Increment reference count
    pub fn inc_ref(&mut self) {
        self.ref_count = self.ref_count.saturating_add(1);
    /// Decrement reference count
    pub fn dec_ref(&mut self) -> u16 {
        self.ref_count = self.ref_count.saturating_sub(1);
        self.ref_count
    /// Get object size including header
    pub fn total_size(&self) -> usize {
        mem::size_of::<ObjectHeader>() + self.size as usize
    /// Validate header integrity
    pub fn validate(&self) -> Result<(), String> {
        // Check for reasonable size (not too large)
        if self.size > 1024 * 1024 * 1024 { // 1GB limit
            return Err(format!("Object size {} exceeds reasonable limit", self.size));
        // Check generation is reasonable
        if self.generation > 10 {
            return Err(format!("Object generation {} is unrealistic", self.generation));
        // Header looks valid
        Ok(())
    }
}

/// Extended object metadata stored separately from the object
/// 
/// This contains additional information that's not performance-critical
/// but useful for debugging, profiling, and advanced GC features.
#[derive(Debug, Clone)]
pub struct ObjectMetadata {
    /// Object identifier
    /// Object size (excluding header)
    /// Type name for debugging
    /// Allocation timestamp
    /// Last access timestamp (for aging strategies)
    /// Allocation site information
    /// Object alignment requirement
    /// Pointer to the object header
    /// Pointer to the object data
    /// Whether this object is a GC root
    /// Custom metadata fields
impl ObjectMetadata {
    /// Create new object metadata
               header_ptr: NonNull<ObjectHeader>, data_ptr: NonNull<u8>) -> Self {
        let now = std::time::Instant::now();
        
        Self {
        }
    }
    
    /// Update last access time
    pub fn touch(&mut self) {
        self.last_accessed = std::time::Instant::now();
    /// Get object age in milliseconds
    pub fn age_ms(&self) -> u64 {
        self.allocated_at.elapsed().as_millis() as u64
    /// Get time since last access in milliseconds
    pub fn idle_time_ms(&self) -> u64 {
        self.last_accessed.elapsed().as_millis() as u64
    /// Mark as GC root
    pub fn set_root(&mut self, is_root: bool) {
        self.is_root = is_root;
    /// Add custom metadata field
    pub fn set_custom_field(&mut self, key: String, value: String) {
        self.custom_fields.insert(key, value);
    /// Get custom metadata field
    pub fn get_custom_field(&self, key: &str) -> Option<&String> {
        self.custom_fields.get(key)
    /// Get header safely
    pub fn get_header(&self) -> Result<&ObjectHeader, String> {
        unsafe {
            self.header_ptr.as_ref()
        }.validate()?;
        
        Ok(unsafe { self.header_ptr.as_ref() })
    /// Get mutable header safely
    pub fn get_header_mut(&self) -> Result<&mut ObjectHeader, String> {
        unsafe {
            self.header_ptr.as_ref()
        }.validate()?;
        
        Ok(unsafe { &mut *self.header_ptr.as_ptr() })
    }
}

/// Memory layout calculator for objects with headers
#[derive(Debug)]
pub struct MemoryLayout {
    /// Header size (aligned)
    /// Object size (aligned)
    /// Total allocation size
    /// Header offset from allocation start
    /// Object offset from allocation start
    /// Alignment used
impl MemoryLayout {
    /// Calculate memory layout for an object
    pub fn calculate(object_size: usize, alignment: usize) -> Self {
        let header_size = mem::size_of::<ObjectHeader>();
        let min_alignment = alignment.max(mem::align_of::<ObjectHeader>());
        
        // Align header size
        let aligned_header_size = Self::align_up(header_size, min_alignment);
        
        // Object comes after header
        let object_offset = aligned_header_size;
        
        // Align object size
        let aligned_object_size = Self::align_up(object_size, alignment);
        
        // Total allocation size
        let total_size = object_offset + aligned_object_size;
        
        Self {
        }
    }
    
    /// Align size up to alignment boundary
    fn align_up(size: usize, alignment: usize) -> usize {
        (size + alignment - 1) & !(alignment - 1)
    /// Get header pointer from allocation
    pub fn get_header_ptr(&self, allocation: NonNull<u8>) -> NonNull<ObjectHeader> {
        unsafe {
            let header_ptr = allocation.as_ptr().add(self.header_offset) as *mut ObjectHeader;
            NonNull::new_unchecked(header_ptr)
        }
    }
    
    /// Get object data pointer from allocation
    pub fn get_object_ptr(&self, allocation: NonNull<u8>) -> NonNull<u8> {
        unsafe {
            let object_ptr = allocation.as_ptr().add(self.object_offset);
            NonNull::new_unchecked(object_ptr)
        }
    }
/// Metadata manager for centralized object metadata tracking
/// 
/// This manager maintains metadata for all allocated objects and provides
/// efficient lookup by pointer or object ID. It's designed for thread-safe
/// concurrent access from multiple GC threads.
pub struct MetadataManager {
    /// Object ID generator
    /// Metadata by object ID
    /// Metadata by pointer for fast lookup
    /// Object registry for GC integration
    /// Default alignment for objects
    /// Statistics
impl MetadataManager {
    /// Create a new metadata manager
    #[instrument]
    pub fn new(default_alignment: usize) -> Result<Self, String> {
        debug!("Creating metadata manager with {}-byte alignment", default_alignment);
        
        Ok(Self {
        })
    /// Set object registry for GC integration
    pub fn set_object_registry(&mut self, registry: SharedObjectRegistry) {
        self.object_registry = Some(registry);
    /// Initialize object metadata and headers
    #[instrument(skip(self))]
                            alignment: usize, type_name: &str) -> Result<ObjectId, String> {
        let object_id = self.id_generator.next();
        let actual_alignment = alignment.max(self.default_alignment);
        
               object_id, size, actual_alignment);
        
        // Calculate memory layout
        let layout = MemoryLayout::calculate(size, actual_alignment);
        
        // Get header and object pointers
        let header_ptr = layout.get_header_ptr(allocation);
        let object_ptr = layout.get_object_ptr(allocation);
        
        // Determine type tag
        let type_tag = Self::determine_type_tag(type_name);
        
        // Initialize header
        let header = ObjectHeader::new(object_id, size, type_tag, type_name);
        unsafe {
            *header_ptr.as_ptr() = header;
        // Create metadata
        let metadata = ObjectMetadata::new(
            object_ptr
        );
        
        // Store metadata
        {
            let mut by_id = self.metadata_by_id.write()
                .map_err(|_| "Failed to acquire metadata by ID write lock")?;
            by_id.insert(object_id, metadata);
        {
            let mut by_ptr = self.metadata_by_ptr.write()
                .map_err(|_| "Failed to acquire metadata by pointer write lock")?;
            by_ptr.insert(object_ptr.as_ptr() as usize, object_id);
        // Register with object registry if available
        if let Some(registry) = &self.object_registry {
            let obj_metadata = crate::memory::object_id::ObjectMetadata::new(
                object_id, size, type_name.to_string()
            );
            let _ = registry.register(obj_metadata);
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            stats.record_initialization(size, type_name);
               object_id, header_ptr.as_ptr(), object_ptr.as_ptr());
        
        Ok(object_id)
    /// Get metadata by object ID
    pub fn get_metadata_by_id(&self, object_id: ObjectId) -> Result<Option<ObjectMetadata>, String> {
        let by_id = self.metadata_by_id.read()
            .map_err(|_| "Failed to acquire metadata by ID read lock")?;
        
        Ok(by_id.get(&object_id).cloned())
    /// Get metadata by object pointer
    pub fn get_metadata(&self, ptr: NonNull<u8>) -> Result<ObjectMetadata, String> {
        let object_id = {
            let by_ptr = self.metadata_by_ptr.read()
                .map_err(|_| "Failed to acquire metadata by pointer read lock")?;
            
            by_ptr.get(&(ptr.as_ptr() as usize))
                .copied()
                .ok_or_else(|| format!("No metadata found for pointer {:p}", ptr.as_ptr()))?
        
        let by_id = self.metadata_by_id.read()
            .map_err(|_| "Failed to acquire metadata by ID read lock")?;
        
        by_id.get(&object_id)
            .cloned()
            .ok_or_else(|| format!("Metadata not found for object {}", object_id))
    /// Remove object metadata
    #[instrument(skip(self))]
    pub fn remove_metadata(&self, ptr: NonNull<u8>) -> Result<(), String> {
        debug!("Removing metadata for pointer {:p}", ptr.as_ptr());
        
        let object_id = {
            let mut by_ptr = self.metadata_by_ptr.write()
                .map_err(|_| "Failed to acquire metadata by pointer write lock")?;
            
            by_ptr.remove(&(ptr.as_ptr() as usize))
                .ok_or_else(|| format!("No metadata found for pointer {:p}", ptr.as_ptr()))?
        
        let metadata = {
            let mut by_id = self.metadata_by_id.write()
                .map_err(|_| "Failed to acquire metadata by ID write lock")?;
            
            by_id.remove(&object_id)
                .ok_or_else(|| format!("Metadata not found for object {}", object_id))?
        
        // Unregister from object registry if available
        if let Some(registry) = &self.object_registry {
            let _ = registry.unregister(object_id);
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            stats.record_removal(metadata.size, &metadata.type_name);
        debug!("Removed metadata for object {}", object_id);
        Ok(())
    /// Update object access time
    pub fn touch_object(&self, ptr: NonNull<u8>) -> Result<(), String> {
        let object_id = {
            let by_ptr = self.metadata_by_ptr.read()
                .map_err(|_| "Failed to acquire metadata by pointer read lock")?;
            
            by_ptr.get(&(ptr.as_ptr() as usize))
                .copied()
                .ok_or_else(|| format!("No metadata found for pointer {:p}", ptr.as_ptr()))?
        
        let mut by_id = self.metadata_by_id.write()
            .map_err(|_| "Failed to acquire metadata by ID write lock")?;
        
        if let Some(metadata) = by_id.get_mut(&object_id) {
            metadata.touch();
        Ok(())
    /// Mark object as GC root
    pub fn mark_as_root(&self, object_id: ObjectId, is_root: bool) -> Result<(), String> {
        let mut by_id = self.metadata_by_id.write()
            .map_err(|_| "Failed to acquire metadata by ID write lock")?;
        
        if let Some(metadata) = by_id.get_mut(&object_id) {
            metadata.set_root(is_root);
        Ok(())
    /// Get all root objects
    pub fn get_root_objects(&self) -> Result<Vec<ObjectId>, String> {
        let by_id = self.metadata_by_id.read()
            .map_err(|_| "Failed to acquire metadata by ID read lock")?;
        
        let roots = by_id.values()
            .filter(|metadata| metadata.is_root)
            .map(|metadata| metadata.object_id)
            .collect();
        
        Ok(roots)
    /// Get objects older than the specified age
    pub fn get_objects_older_than(&self, age_ms: u64) -> Result<Vec<ObjectId>, String> {
        let by_id = self.metadata_by_id.read()
            .map_err(|_| "Failed to acquire metadata by ID read lock")?;
        
        let old_objects = by_id.values()
            .filter(|metadata| metadata.age_ms() > age_ms)
            .map(|metadata| metadata.object_id)
            .collect();
        
        Ok(old_objects)
    /// Get statistics
    pub fn get_statistics(&self) -> Result<MetadataStatistics, String> {
        let stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        
        let by_id = self.metadata_by_id.read()
            .map_err(|_| "Failed to acquire metadata by ID read lock")?;
        
        let mut result = stats.clone();
        result.current_objects = by_id.len();
        result.current_metadata_size = by_id.len() * mem::size_of::<ObjectMetadata>();
        
        Ok(result)
    /// Determine type tag from type name
    fn determine_type_tag(type_name: &str) -> Tag {
        match type_name {
        }
    }
/// Statistics for metadata management
#[derive(Debug, Clone)]
pub struct MetadataStatistics {
    /// Total objects initialized
    /// Total objects removed
    /// Current number of objects
    /// Current metadata memory usage
    /// Total metadata memory ever allocated
    /// Access count (touches)
    /// Root object count
    /// Type distribution
impl MetadataStatistics {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn record_initialization(&mut self, size: usize, type_name: &str) {
        self.total_initialized += 1;
        self.total_metadata_allocated += mem::size_of::<ObjectMetadata>();
        
        let count = self.type_distribution.entry(type_name.to_string()).or_insert(0);
        *count += 1;
    pub fn record_removal(&mut self, _size: usize, type_name: &str) {
        self.total_removed += 1;
        
        if let Some(count) = self.type_distribution.get_mut(type_name) {
            *count = count.saturating_sub(1);
        }
    }
    
    pub fn record_access(&mut self) {
        self.total_accesses += 1;
    }
}

impl std::fmt::Display for MetadataStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            "Metadata Statistics:\n\
             - Current Objects: {}\n\
             - Total Initialized: {}\n\
             - Total Removed: {}\n\
             - Metadata Memory: {:.2} KB\n\
             - Total Accesses: {}\n\
            self.current_metadata_size as f64 / 1024.0,
            self.root_objects
        )
    }
}

