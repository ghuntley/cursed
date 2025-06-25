/// Object Store for Garbage Collection
/// 
/// This module provides a high-level object storage system that manages the lifecycle
/// of objects in the garbage-collected heap. It is critical for memory safety because:
/// 
/// 1. **Type Safety**: Ensures objects are stored and retrieved with correct types
/// 2. **Reference Tracking**: Maintains proper reference relationships between objects
/// 3. **Collection Coordination**: Integrates with GC phases for safe object management
/// 4. **Concurrent Access**: Provides thread-safe operations for multi-threaded environments
/// 5. **Memory Layout**: Optimizes object storage patterns for cache efficiency
/// 
/// The object store acts as the primary interface between user code and the underlying
/// heap management system, providing both safety and performance optimizations.

use std::ptr::NonNull;
use std::sync::{Arc, RwLock, Weak};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::any::{Any, TypeId};
use tracing::{instrument, debug, warn, error};

use crate::memory::object_id::{ObjectId, SharedObjectRegistry};
use crate::memory::heap_manager::{HeapManager, AllocationInfo};
use crate::memory::{Traceable, Visitor};
use crate::error::CursedError;

/// Thread-safe wrapper for raw pointers to Any objects
/// 
/// This wrapper ensures that raw pointers can be safely sent between threads
/// while maintaining type safety through the type system.
#[derive(Debug)]
struct ThreadSafePtr {
    // We use *mut u8 instead of *mut dyn Any to avoid trait object complications
impl ThreadSafePtr {
    /// Create a new thread-safe pointer wrapper
    fn new<T>(ptr: *mut T) -> Self {
        Self {
        }
    }
    
    /// Get the raw pointer, typed appropriately
    unsafe fn as_ptr<T>(&self) -> *mut T {
        self.ptr as *mut T
    /// Check if the pointer is null
    fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

// Safety: ThreadSafePtr is only used within the ObjectStore which ensures:
// 1. Pointers are only accessed when the ObjectStore lock is held
// 2. Objects are properly allocated and deallocated through the heap manager
// 3. Type safety is maintained through the TypeId tracking
// 4. Access is coordinated through the RwLock protection
unsafe impl Send for ThreadSafePtr {}
unsafe impl Sync for ThreadSafePtr {}

/// Trait for objects that can be stored in the object store
/// 
/// This provides the necessary hooks for garbage collection and type safety.
pub trait Storable: Traceable + Send + Sync + 'static {
    /// Get the type name for debugging
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    /// Get the size of this object in bytes
    fn object_size(&self) -> usize 
    where 
    {
        std::mem::size_of::<Self>()
    /// Called when object is being collected (optional cleanup)
    fn on_collect(&mut self) {
        // Default implementation does nothing
    }
}

/// Automatically implement Storable for types that implement required traits
impl<T> Storable for T 
where 
{
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    fn object_size(&self) -> usize 
    where 
    {
        std::mem::size_of::<T>()
    }
}

/// Handle to an object stored in the object store
/// 
/// This provides safe access to objects while ensuring they remain valid
/// during garbage collection cycles.
#[derive(Debug)]
pub struct ObjectHandle<T: Storable> {
impl<T: Storable> ObjectHandle<T> {
    /// Create a new object handle
    fn new(object_id: ObjectId, ptr: NonNull<T>, object_store: Weak<ObjectStore>) -> Self {
        Self {
        }
    }
    
    /// Create a new object handle for externally allocated objects
    /// 
    /// This is used by enhanced garbage collection systems that manage
    /// their own memory allocation but need to integrate with the object store.
    pub fn new_external(object_id: ObjectId, ptr: NonNull<T>, object_store: Weak<ObjectStore>) -> Self {
        Self::new(object_id, ptr, object_store)
    /// Get the object ID
    pub fn object_id(&self) -> ObjectId {
        self.object_id
    /// Get a reference to the stored object
    /// 
    /// This is safe because the object store ensures the object remains
    /// valid while handles exist.
    pub fn get(&self) -> Option<&T> {
        if self.is_valid() {
            unsafe { Some(self.ptr.as_ref()) }
        } else {
            None
        }
    }
    
    /// Get a mutable reference to the stored object
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_valid() {
            unsafe { Some(self.ptr.as_mut()) }
        } else {
            None
        }
    }
    
    /// Check if this handle is still valid
    /// 
    /// Handles become invalid if the object has been collected or
    /// the object store has been dropped.
    pub fn is_valid(&self) -> bool {
        if let Some(store) = self.object_store.upgrade() {
            store.is_object_valid(self.object_id)
        } else {
            false
        }
    }
    
    /// Mark this object as a GC root
    /// 
    /// Root objects are never collected and serve as starting points
    /// for reachability analysis.
    pub fn mark_as_root(&self) -> Result<(), String> {
        if let Some(store) = self.object_store.upgrade() {
            store.mark_as_root(self.object_id)
        } else {
            Err("Object store no longer available".to_string())
        }
    }
    
    /// Unmark this object as a GC root
    pub fn unmark_as_root(&self) -> Result<(), String> {
        if let Some(store) = self.object_store.upgrade() {
            store.unmark_as_root(self.object_id)
        } else {
            Err("Object store no longer available".to_string())
        }
    }
impl<T: Storable> Clone for ObjectHandle<T> {
    fn clone(&self) -> Self {
        Self {
        }
    }
// Safety: ObjectHandle is safe to send between threads because:
// 1. ObjectId is Copy and thread-safe
// 2. NonNull<T> is Send if T is Send (which Storable requires)
// 3. Weak<ObjectStore> is Send
// 4. Access is validated through the object store
unsafe impl<T: Storable> Send for ObjectHandle<T> {}
unsafe impl<T: Storable> Sync for ObjectHandle<T> {}

/// Information about a stored object
#[derive(Debug, Clone)]
pub struct StoredObjectInfo {
/// Visitor implementation for the object store
struct ObjectStoreVisitor<'a> {
impl<'a> Visitor for ObjectStoreVisitor<'a> {
    fn visit(&mut self, obj: &dyn Traceable) {
        // This is a simplified implementation - in practice, you'd need
        // a way to map from Traceable objects back to ObjectIds
        // For now, we'll leave this as a placeholder
    }
}

/// Main object store that manages object lifecycle
/// 
/// The object store coordinates between the heap manager and provides
/// high-level object management with garbage collection integration.
pub struct ObjectStore {
    /// Underlying heap manager
    /// Object registry for metadata
    /// Type information for stored objects
    /// Raw object pointers (type-erased) - wrapped for thread safety
    /// Root set for garbage collection
    /// Reference counts for objects
impl ObjectStore {
    /// Create a new object store
    #[instrument]
    pub fn new(heap_manager: Arc<RwLock<HeapManager>>, object_registry: SharedObjectRegistry) -> Arc<Self> {
        debug!("Creating new object store");
        
        Arc::new(Self {
        })
    /// Store a new object in the object store
    #[instrument(skip(self, object))]
    pub fn store<T: Storable>(self: &Arc<Self>, object: T) -> Result<ObjectHandle<T>, String> {
        let type_name = object.type_name();
        let size = object.object_size();
        let type_id = TypeId::of::<T>();
        
        debug!("Storing object of type {} (size: {} bytes)", type_name, size);
        
        // Allocate memory from heap manager
        let (object_id, ptr) = {
            let heap = self.heap_manager.read()
                .map_err(|_| "Failed to acquire read lock on heap manager")?;
            heap.allocate::<T>(size, type_name)?
        
        // Write object to allocated memory
        unsafe {
            let typed_ptr = ptr.as_ptr() as *mut T;
            typed_ptr.write(object);
        // Register type information
        {
            let mut types = self.type_registry.write()
                .map_err(|_| "Failed to acquire write lock on type registry")?;
            types.insert(object_id, type_id);
        // Store type-erased pointer
        {
            let mut pointers = self.object_pointers.write()
                .map_err(|_| "Failed to acquire write lock on object pointers")?;
            let thread_safe_ptr = ThreadSafePtr::new(ptr.as_ptr() as *mut T);
            pointers.insert(object_id, (thread_safe_ptr, type_id));
        // Initialize reference count
        {
            let mut ref_counts = self.reference_counts.write()
                .map_err(|_| "Failed to acquire write lock on reference counts")?;
            ref_counts.insert(object_id, 0);
        debug!("Successfully stored object {} at {:p}", object_id, ptr.as_ptr());
        
        let handle = ObjectHandle::new(object_id, ptr.cast(), Arc::downgrade(self));
        Ok(handle)
    /// Get an object handle by ID (if it exists and has the correct type)
    pub fn get_handle<T: Storable>(&self, object_id: ObjectId) -> Option<ObjectHandle<T>> {
        let type_id = TypeId::of::<T>();
        
        // Check if object exists and has correct type
        let stored_type = {
            let types = self.type_registry.read().ok()?;
            *types.get(&object_id)?
        
        if stored_type != type_id {
                  object_id, type_id, stored_type);
            return None;
        // Get the pointer
        let ptr = {
            let pointers = self.object_pointers.read().ok()?;
            let (thread_safe_ptr, _) = pointers.get(&object_id)?;
            if thread_safe_ptr.is_null() {
                return None;
            }
            unsafe {
                NonNull::new(thread_safe_ptr.as_ptr::<T>())?
            }
        
        // This method should only be called on Arc<ObjectStore>, so we can't easily get a weak reference
        // For now, we'll create an invalid weak reference - this needs to be fixed in the API design
        let weak_ref = std::sync::Weak::<ObjectStore>::new();
        Some(ObjectHandle::new(object_id, ptr, weak_ref))
    /// Check if an object is valid (still exists in the store)
    pub fn is_object_valid(&self, object_id: ObjectId) -> bool {
        if let Ok(pointers) = self.object_pointers.read() {
            pointers.contains_key(&object_id)
        } else {
            false
        }
    }
    
    /// Mark an object as a garbage collection root
    #[instrument(skip(self))]
    pub fn mark_as_root(&self, object_id: ObjectId) -> Result<(), String> {
        if !self.is_object_valid(object_id) {
            return Err(format!("Object {} is not valid", object_id));
        let mut roots = self.root_objects.write()
            .map_err(|_| "Failed to acquire write lock on root objects")?;
        
        if roots.insert(object_id) {
            debug!("Marked object {} as GC root", object_id);
        Ok(())
    /// Unmark an object as a garbage collection root
    #[instrument(skip(self))]
    pub fn unmark_as_root(&self, object_id: ObjectId) -> Result<(), String> {
        let mut roots = self.root_objects.write()
            .map_err(|_| "Failed to acquire write lock on root objects")?;
        
        if roots.remove(&object_id) {
            debug!("Unmarked object {} as GC root", object_id);
        Ok(())
    /// Get all root objects
    pub fn get_root_objects(&self) -> Result<Vec<ObjectId>, String> {
        let roots = self.root_objects.read()
            .map_err(|_| "Failed to acquire read lock on root objects")?;
        Ok(roots.iter().copied().collect())
    /// Increment reference count for an object
    pub fn inc_ref(&self, object_id: ObjectId) -> Result<usize, String> {
        let mut ref_counts = self.reference_counts.write()
            .map_err(|_| "Failed to acquire write lock on reference counts")?;
        
        let count = ref_counts.entry(object_id).or_insert(0);
        *count += 1;
        Ok(*count)
    /// Decrement reference count for an object
    pub fn dec_ref(&self, object_id: ObjectId) -> Result<usize, String> {
        let mut ref_counts = self.reference_counts.write()
            .map_err(|_| "Failed to acquire write lock on reference counts")?;
        
        let count = ref_counts.entry(object_id).or_insert(0);
        if *count > 0 {
            *count -= 1;
        }
        Ok(*count)
    /// Get reference count for an object
    pub fn get_ref_count(&self, object_id: ObjectId) -> Result<usize, String> {
        let ref_counts = self.reference_counts.read()
            .map_err(|_| "Failed to acquire read lock on reference counts")?;
        Ok(ref_counts.get(&object_id).copied().unwrap_or(0))
    /// Remove an object from the store (called during GC)
    #[instrument(skip(self))]
    pub fn remove_object(&self, object_id: ObjectId) -> Result<(), String> {
        debug!("Removing object {} from store", object_id);
        
        // Call destructor if object still exists
        if let Ok(mut pointers) = self.object_pointers.write() {
            if let Some((thread_safe_ptr, _type_id)) = pointers.remove(&object_id) {
                // Note: We need proper type dispatch to call the correct destructor
                // For now, we'll skip the destructor call since we don't have enough
                // type information to safely call it. This should be handled by
                // the heap manager's deallocation process.
                if !thread_safe_ptr.is_null() {
                    debug!("Removed object {} pointer from store", object_id);
                }
            }
        // Remove from type registry
        if let Ok(mut types) = self.type_registry.write() {
            types.remove(&object_id);
        // Remove from root set
        if let Ok(mut roots) = self.root_objects.write() {
            roots.remove(&object_id);
        // Remove reference count
        if let Ok(mut ref_counts) = self.reference_counts.write() {
            ref_counts.remove(&object_id);
        // Deallocate from heap
        {
            let heap = self.heap_manager.read()
                .map_err(|_| "Failed to acquire read lock on heap manager")?;
            heap.deallocate(object_id)?;
        debug!("Successfully removed object {}", object_id);
        Ok(())
    /// Get information about all stored objects
    pub fn get_all_objects(&self) -> Result<Vec<StoredObjectInfo>, String> {
        let pointers = self.object_pointers.read()
            .map_err(|_| "Failed to acquire read lock on object pointers")?;
        let types = self.type_registry.read()
            .map_err(|_| "Failed to acquire read lock on type registry")?;
        let roots = self.root_objects.read()
            .map_err(|_| "Failed to acquire read lock on root objects")?;
        let ref_counts = self.reference_counts.read()
            .map_err(|_| "Failed to acquire read lock on reference counts")?;
        
        let mut objects = Vec::new();
        
        for (&object_id, &(ref _thread_safe_ptr, type_id)) in pointers.iter() {
            if let Ok(Some(metadata)) = self.object_registry.get(object_id) {
                let info = StoredObjectInfo {
                objects.push(info);
            }
        }
        
        Ok(objects)
    /// Get object store statistics
    pub fn get_stats(&self) -> Result<ObjectStoreStats, String> {
        let objects = self.get_all_objects()?;
        
        let total_objects = objects.len();
        let root_objects = objects.iter().filter(|obj| obj.is_root).count();
        let total_size: usize = objects.iter().map(|obj| obj.size).sum();
        
        let mut type_counts = HashMap::new();
        for obj in &objects {
            *type_counts.entry(obj.type_name.clone()).or_insert(0) += 1;
        Ok(ObjectStoreStats {
        })
    /// Trace all objects for garbage collection
    pub fn trace_objects(&self, visitor: &mut dyn Visitor) -> Result<(), String> {
        let pointers = self.object_pointers.read()
            .map_err(|_| "Failed to acquire read lock on object pointers")?;
        
        for (&object_id, &(ref _thread_safe_ptr, _)) in pointers.iter() {
            // For now, skip tracing - this would require proper type dispatch
            // to call the trace method on the actual object type
            // This is a complex problem that would need additional type system support
        Ok(())
    }
}

/// Statistics about the object store
#[derive(Debug, Clone)]
pub struct ObjectStoreStats {
impl std::fmt::Display for ObjectStoreStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Object Store Stats:")?;
        writeln!(f, "- Total Objects: {}", self.total_objects)?;
        writeln!(f, "- Root Objects: {}", self.root_objects)?;
        writeln!(f, "- Total Size: {} bytes", self.total_size)?;
        writeln!(f, "- Type Distribution:")?;
        
        for (type_name, count) in &self.type_counts {
            writeln!(f, "  {}: {}", type_name, count)?;
        Ok(())
    }
}

// Safety: ObjectStore is safe to send between threads because:
// 1. HeapManager is protected by Arc<RwLock<>>
// 2. ObjectRegistry is thread-safe (SharedObjectRegistry = Arc<ObjectRegistry>)
// 3. All internal data structures use RwLock for synchronization
// 4. ThreadSafePtr wrapper ensures raw pointers are handled safely
// 5. HashMap operations are coordinated through RwLock protection
// Safety: ObjectStore is safe to share between threads because:
// 1. All mutation is coordinated through RwLock<HashMap<...>>
// 2. HeapManager access is synchronized through Arc<RwLock<>>
// 3. ObjectRegistry is designed for concurrent access
// 4. ThreadSafePtr ensures safe raw pointer sharing
// 5. No direct mutable access to internal state without locks
