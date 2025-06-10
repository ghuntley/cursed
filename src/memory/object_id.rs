/// Object Identification System for Garbage Collection
/// 
/// This module provides a robust system for uniquely identifying objects in the heap,
/// which is crucial for memory safety in garbage collection. Each object gets a unique
/// identifier that remains stable throughout its lifetime, enabling:
/// 
/// 1. **Cycle Detection**: Prevents infinite loops during mark phase
/// 2. **Reference Tracking**: Maintains proper object relationships  
/// 3. **Debugging Support**: Enables detailed memory leak analysis
/// 4. **Thread Safety**: Ensures concurrent access to object metadata
/// 
/// The system uses atomic counters for thread-safe ID generation and maintains
/// a registry for fast lookup of object metadata during collection cycles.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::fmt;
use tracing::{instrument, debug, warn};

/// Unique identifier for objects in the heap
/// 
/// Object IDs are never reused during a program's execution to prevent
/// dangling reference bugs and maintain referential integrity during GC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ObjectId(u64);

impl ObjectId {
    /// Create a new ObjectId with the given value
    /// 
    /// This is primarily used internally by the ObjectIdGenerator.
    /// External code should use ObjectIdGenerator::next() instead.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
    
    /// Get the raw ID value
    pub fn as_u64(self) -> u64 {
        self.0
    }
    
    /// Get a null/invalid object ID for sentinel values
    pub const fn null() -> Self {
        Self(0)
    }
    
    /// Check if this is a null/invalid ID
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
    
    /// Create ObjectId from raw u64 value
    /// 
    /// This should only be used for deserialization or low-level operations.
    pub fn from_raw(value: u64) -> Self {
        Self(value)
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "obj#{}", self.0)
    }
}

/// Thread-safe generator for unique object IDs
/// 
/// Uses atomic operations to ensure thread safety without requiring locks
/// for ID generation, which is critical for performance in concurrent
/// allocation scenarios.
#[derive(Debug)]
pub struct ObjectIdGenerator {
    next_id: AtomicU64,
}

impl ObjectIdGenerator {
    /// Create a new ID generator starting from ID 1
    /// (ID 0 is reserved for null/invalid objects)
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
        }
    }
    
    /// Generate the next unique object ID
    /// 
    /// This method is thread-safe and will never return the same ID twice,
    /// even under heavy concurrent load.
    #[instrument(skip(self))]
    pub fn next(&self) -> ObjectId {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        
        // Check for overflow (extremely unlikely but safety first)
        if id == u64::MAX {
            panic!("Object ID counter overflow - this should never happen in practice");
        }
        
        debug!("Generated new object ID: {}", id);
        ObjectId::new(id)
    }
    
    /// Get the current counter value (for debugging/stats)
    pub fn current_count(&self) -> u64 {
        self.next_id.load(Ordering::SeqCst)
    }
}

impl Default for ObjectIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata associated with each object in the heap
/// 
/// This information is crucial for proper garbage collection behavior
/// and debugging memory issues.
#[derive(Debug, Clone)]
pub struct ObjectMetadata {
    /// Unique identifier for this object
    pub id: ObjectId,
    /// Size of the object in bytes (for memory usage tracking)
    pub size: usize,
    /// Type name for debugging purposes
    pub type_name: String,
    /// Generation for generational GC (future enhancement)
    pub generation: u32,
    /// Mark bit for mark-and-sweep collection
    pub marked: bool,
    /// Reference count for hybrid GC approaches
    pub ref_count: usize,
    /// Creation timestamp for age tracking
    pub created_at: std::time::Instant,
}

impl ObjectMetadata {
    /// Create new object metadata
    pub fn new(id: ObjectId, size: usize, type_name: String) -> Self {
        Self {
            id,
            size,
            type_name,
            generation: 0,
            marked: false,
            ref_count: 0,
            created_at: std::time::Instant::now(),
        }
    }
    
    /// Mark this object as reachable during GC
    pub fn mark(&mut self) {
        self.marked = true;
    }
    
    /// Unmark this object (for next GC cycle)
    pub fn unmark(&mut self) {
        self.marked = false;
    }
    
    /// Check if object is marked as reachable
    pub fn is_marked(&self) -> bool {
        self.marked
    }
    
    /// Get the size of the object
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Get the creation timestamp
    pub fn created_at(&self) -> std::time::Instant {
        self.created_at
    }
    
    /// Increment reference count
    pub fn inc_ref(&mut self) {
        self.ref_count += 1;
    }
    
    /// Decrement reference count
    pub fn dec_ref(&mut self) {
        if self.ref_count > 0 {
            self.ref_count -= 1;
        }
    }
}

/// Registry for fast lookup of object metadata during GC
/// 
/// This provides O(1) lookup of object information needed during
/// mark and sweep phases. Thread-safe through RwLock.
#[derive(Debug)]
pub struct ObjectRegistry {
    objects: RwLock<HashMap<ObjectId, ObjectMetadata>>,
}

impl ObjectRegistry {
    /// Create a new empty object registry
    pub fn new() -> Self {
        Self {
            objects: RwLock::new(HashMap::new()),
        }
    }
    
    /// Register a new object in the registry
    #[instrument(skip(self))]
    pub fn register(&self, metadata: ObjectMetadata) -> Result<(), String> {
        match self.objects.write() {
            Ok(mut objects) => {
                let id = metadata.id;
                if objects.contains_key(&id) {
                    warn!("Attempting to register duplicate object ID: {}", id);
                    return Err(format!("Object {} already exists", id));
                }
                
                debug!("Registering object {} (type: {}, size: {} bytes)", 
                       id, metadata.type_name, metadata.size);
                objects.insert(id, metadata);
                Ok(())
            }
            Err(_) => Err("Failed to acquire write lock on object registry".to_string())
        }
    }
    
    /// Unregister an object from the registry
    #[instrument(skip(self))]
    pub fn unregister(&self, id: ObjectId) -> Result<Option<ObjectMetadata>, String> {
        match self.objects.write() {
            Ok(mut objects) => {
                debug!("Unregistering object {}", id);
                Ok(objects.remove(&id))
            }
            Err(_) => Err("Failed to acquire write lock on object registry".to_string())
        }
    }
    
    /// Get object metadata by ID
    pub fn get(&self, id: ObjectId) -> Result<Option<ObjectMetadata>, String> {
        match self.objects.read() {
            Ok(objects) => Ok(objects.get(&id).cloned()),
            Err(_) => Err("Failed to acquire read lock on object registry".to_string())
        }
    }
    
    /// Mark an object as reachable
    #[instrument(skip(self))]
    pub fn mark_object(&self, id: ObjectId) -> Result<bool, String> {
        match self.objects.write() {
            Ok(mut objects) => {
                if let Some(metadata) = objects.get_mut(&id) {
                    metadata.mark();
                    debug!("Marked object {} as reachable", id);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(_) => Err("Failed to acquire write lock on object registry".to_string())
        }
    }
    
    /// Get all unmarked objects (for sweep phase)
    pub fn get_unmarked_objects(&self) -> Result<Vec<ObjectId>, String> {
        match self.objects.read() {
            Ok(objects) => {
                let unmarked: Vec<ObjectId> = objects
                    .iter()
                    .filter(|(_, metadata)| !metadata.is_marked())
                    .map(|(id, _)| *id)
                    .collect();
                
                debug!("Found {} unmarked objects for collection", unmarked.len());
                Ok(unmarked)
            }
            Err(_) => Err("Failed to acquire read lock on object registry".to_string())
        }
    }
    
    /// Unmark all objects (prepare for next GC cycle)
    #[instrument(skip(self))]
    pub fn unmark_all(&self) -> Result<(), String> {
        match self.objects.write() {
            Ok(mut objects) => {
                for metadata in objects.values_mut() {
                    metadata.unmark();
                }
                debug!("Unmarked all {} objects", objects.len());
                Ok(())
            }
            Err(_) => Err("Failed to acquire write lock on object registry".to_string())
        }
    }
    
    /// Get total number of registered objects
    pub fn object_count(&self) -> Result<usize, String> {
        match self.objects.read() {
            Ok(objects) => Ok(objects.len()),
            Err(_) => Err("Failed to acquire read lock on object registry".to_string())
        }
    }
    
    /// Get total memory usage of all registered objects
    pub fn total_memory_usage(&self) -> Result<usize, String> {
        match self.objects.read() {
            Ok(objects) => {
                let total = objects.values().map(|meta| meta.size).sum();
                Ok(total)
            }
            Err(_) => Err("Failed to acquire read lock on object registry".to_string())
        }
    }
    
    /// Get all object IDs (for GC traversal)
    pub fn get_all_objects(&self) -> Result<Vec<ObjectId>, String> {
        match self.objects.read() {
            Ok(objects) => {
                let all_ids: Vec<ObjectId> = objects.keys().copied().collect();
                Ok(all_ids)
            }
            Err(_) => Err("Failed to acquire read lock on object registry".to_string())
        }
    }
    
    /// Get root objects (placeholder - should be managed by RootSetManager)
    pub fn get_root_objects(&self) -> Result<Vec<ObjectId>, String> {
        // For now, return empty list - this should be handled by RootSetManager
        Ok(Vec::new())
    }
}

impl Default for ObjectRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared object registry for use across the GC system
pub type SharedObjectRegistry = Arc<ObjectRegistry>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::Arc;
    
    #[test]
    fn test_object_id_creation() {
        let id = ObjectId::new(42);
        assert_eq!(id.as_u64(), 42);
        assert_eq!(format!("{}", id), "obj#42");
        
        let null_id = ObjectId::null();
        assert!(null_id.is_null());
        assert_eq!(null_id.as_u64(), 0);
    }
    
    #[test]
    fn test_id_generator() {
        let generator = ObjectIdGenerator::new();
        
        let id1 = generator.next();
        let id2 = generator.next();
        let id3 = generator.next();
        
        assert!(id1.as_u64() < id2.as_u64());
        assert!(id2.as_u64() < id3.as_u64());
        assert_eq!(generator.current_count(), 4); // Next ID to be generated
    }
    
    #[test]
    fn test_concurrent_id_generation() {
        let generator = Arc::new(ObjectIdGenerator::new());
        let mut handles = vec![];
        
        // Spawn multiple threads generating IDs
        for _ in 0..10 {
            let gen = generator.clone();
            handles.push(thread::spawn(move || {
                let mut ids = vec![];
                for _ in 0..100 {
                    ids.push(gen.next());
                }
                ids
            }));
        }
        
        let mut all_ids = vec![];
        for handle in handles {
            all_ids.extend(handle.join().unwrap());
        }
        
        // Verify all IDs are unique
        all_ids.sort_by_key(|id| id.as_u64());
        for i in 1..all_ids.len() {
            assert!(all_ids[i-1].as_u64() < all_ids[i].as_u64(), 
                   "Found duplicate or out-of-order IDs");
        }
        
        assert_eq!(all_ids.len(), 1000);
    }
    
    #[test]
    fn test_object_metadata() {
        let id = ObjectId::new(1);
        let mut metadata = ObjectMetadata::new(id, 64, "TestObject".to_string());
        
        assert_eq!(metadata.id, id);
        assert_eq!(metadata.size, 64);
        assert_eq!(metadata.type_name, "TestObject");
        assert!(!metadata.is_marked());
        assert_eq!(metadata.ref_count, 0);
        
        metadata.mark();
        assert!(metadata.is_marked());
        
        metadata.inc_ref();
        assert_eq!(metadata.ref_count, 1);
        
        metadata.dec_ref();
        assert_eq!(metadata.ref_count, 0);
    }
    
    #[test]
    fn test_object_registry() {
        let registry = ObjectRegistry::new();
        let id1 = ObjectId::new(1);
        let id2 = ObjectId::new(2);
        
        let metadata1 = ObjectMetadata::new(id1, 32, "Object1".to_string());
        let metadata2 = ObjectMetadata::new(id2, 64, "Object2".to_string());
        
        // Register objects
        assert!(registry.register(metadata1).is_ok());
        assert!(registry.register(metadata2).is_ok());
        
        // Verify registration
        assert_eq!(registry.object_count().unwrap(), 2);
        assert_eq!(registry.total_memory_usage().unwrap(), 96);
        
        // Test marking
        assert!(registry.mark_object(id1).unwrap());
        assert!(!registry.mark_object(ObjectId::new(999)).unwrap()); // Non-existent
        
        // Test unmarked objects
        let unmarked = registry.get_unmarked_objects().unwrap();
        assert_eq!(unmarked.len(), 1);
        assert_eq!(unmarked[0], id2);
        
        // Test unmark all
        assert!(registry.unmark_all().is_ok());
        let unmarked = registry.get_unmarked_objects().unwrap();
        assert_eq!(unmarked.len(), 2);
        
        // Test unregistration
        let removed = registry.unregister(id1).unwrap();
        assert!(removed.is_some());
        assert_eq!(registry.object_count().unwrap(), 1);
    }
    
    #[test]
    fn test_registry_concurrent_access() {
        let registry = Arc::new(ObjectRegistry::new());
        let mut handles = vec![];
        
        // Spawn threads that register objects concurrently
        for i in 0..5 {
            let reg = registry.clone();
            handles.push(thread::spawn(move || {
                for j in 0..20 {
                    let id = ObjectId::new((i * 20 + j) as u64 + 1);
                    let metadata = ObjectMetadata::new(id, 32, format!("Object{}", id.as_u64()));
                    let _ = reg.register(metadata);
                }
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(registry.object_count().unwrap(), 100);
    }
}
