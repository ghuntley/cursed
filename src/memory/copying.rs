/// Copying Garbage Collector for Young Generation
/// 
/// This module implements a high-performance copying garbage collector optimized
/// for young generation collection. It uses a semi-space approach with fast
/// allocation and efficient collection of short-lived objects.

use std::sync::{Arc, RwLock, Mutex};
use std::collections::{HashMap, VecDeque};
use std::ptr::NonNull;
use std::time::{Duration, Instant};
use tracing::{instrument, debug, info, warn, error};

use crate::memory::{Traceable, Visitor};
use crate::memory::object_id::{ObjectId, ObjectIdGenerator, ObjectMetadata, ObjectRegistry, SharedObjectRegistry};
use crate::memory::roots::{RootSetManager, RootType};
use crate::memory::heap_manager::HeapManager;
use crate::error::CursedError;

/// Configuration for copying collection
#[derive(Debug, Clone)]
pub struct CopyingConfig {
    /// Size of each semi-space in bytes
    /// Enable fast path allocation (bump pointer)
    /// Survivor space ratio (0.0 to 1.0)
    /// Age threshold for promotion to old generation
    /// Size threshold for promotion (large objects)
    /// Enable parallel copying across threads
    /// Number of threads for parallel copying
    /// Maximum time to spend in copying phase
    /// Enable object aging and tracking
    /// Enable large object area for oversized objects
impl Default for CopyingConfig {
    fn default() -> Self {
        Self {
            semispace_size: 32 * 1024 * 1024, // 32MB per semispace
            survivor_ratio: 0.1, // 10% for survivor spaces
            promotion_size_threshold: 32 * 1024, // 32KB
        }
    }
/// Statistics from a copying collection cycle
#[derive(Debug, Clone)]
pub struct CopyingStats {
    pub allocation_rate: f64, // bytes per second
    pub promotion_rate: f64,  // ratio of objects promoted
    pub survival_rate: f64,   // ratio of objects that survived
/// Semi-space information
#[derive(Debug, Clone)]
struct SemiSpace {
    /// Start address of the space
    /// End address of the space  
    /// Current allocation pointer
    /// Size of the space in bytes
    /// Whether this space is currently active
impl SemiSpace {
    /// Create a new semi-space
    fn new(size: usize) -> Result<Self, String> {
        // Allocate aligned memory for the semi-space
        let layout = std::alloc::Layout::from_size_align(size, 64)
            .map_err(|e| format!("Failed to create layout: {}", e))?;
        
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            return Err("Failed to allocate memory for semi-space".to_string());
        let start_ptr = NonNull::new(ptr).unwrap();
        let end_ptr = NonNull::new(unsafe { ptr.add(size) }).unwrap();
        
        Ok(Self {
        })
    /// Check if there's enough space for allocation
    fn can_allocate(&self, size: usize) -> bool {
        let available = unsafe { self.end_ptr.as_ptr().offset_from(self.alloc_ptr.as_ptr()) } as usize;
        available >= size
    /// Allocate space and return pointer
    fn allocate(&mut self, size: usize, align: usize) -> Option<NonNull<u8>> {
        let aligned_size = (size + align - 1) & !(align - 1);
        
        if !self.can_allocate(aligned_size) {
            return None;
        let old_ptr = self.alloc_ptr;
        self.alloc_ptr = NonNull::new(unsafe { self.alloc_ptr.as_ptr().add(aligned_size) }).unwrap();
        
        Some(old_ptr)
    /// Reset allocation pointer to beginning
    fn reset(&mut self) {
        self.alloc_ptr = self.start_ptr;
    /// Get utilization ratio
    fn utilization(&self) -> f64 {
        let used = unsafe { self.alloc_ptr.as_ptr().offset_from(self.start_ptr.as_ptr()) } as usize;
        used as f64 / self.size as f64
    }
}

impl Drop for SemiSpace {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::from_size_align(self.size, 64).unwrap();
        unsafe { std::alloc::dealloc(self.start_ptr.as_ptr(), layout) };
    }
}

/// Object age tracking for promotion decisions
#[derive(Debug, Clone)]
struct ObjectAge {
/// Work item for parallel copying
#[derive(Debug, Clone)]
struct CopyingWorkItem {
/// Copying garbage collector for young generation
pub struct CopyingCollector {
    
    /// From-space (currently being allocated from)
    /// To-space (where objects are copied during collection)
    
    /// Object age tracking
    /// Forwarding table for copied objects
    /// Work queue for parallel copying
    
    /// Statistics
impl CopyingCollector {
    /// Create a new copying collector
    pub fn new(object_registry: SharedObjectRegistry) -> Result<Self, String> {
        Self::with_config(object_registry, CopyingConfig::default())
    /// Create a new copying collector with custom configuration
    #[instrument(skip(object_registry, config))]
    pub fn with_config(object_registry: SharedObjectRegistry, config: CopyingConfig) -> Result<Self, String> {
        info!("Creating copying collector with config: {:?}", config);
        
        let from_space = SemiSpace::new(config.semispace_size)?;
        let to_space = SemiSpace::new(config.semispace_size)?;
        
        let mut from_space_mut = from_space;
        from_space_mut.is_active = true;
        
        Ok(Self {
            stats: RwLock::new(CopyingStats {
        })
    /// Set the root set manager
    pub fn set_root_manager(&mut self, root_manager: Arc<RootSetManager>) {
        self.root_manager = Some(root_manager);
    /// Allocate space for a new object
    #[instrument(skip(self))]
    pub fn allocate(&self, size: usize, align: usize) -> Result<Option<NonNull<u8>>, String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // Check if object is too large for young generation
        if size > config.promotion_size_threshold {
            debug!("Object size {} exceeds promotion threshold, allocating in old generation", size);
            return Ok(None); // Should allocate in old generation
        let mut from_space = self.from_space.lock()
            .map_err(|_| "Failed to acquire lock on from_space")?;
        
        // Try fast allocation if enabled
        if config.fast_allocation {
            if let Some(ptr) = from_space.allocate(size, align) {
                // Update allocation statistics
                self.allocation_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                self.allocation_bytes.fetch_add(size as u64, std::sync::atomic::Ordering::SeqCst);
                
                debug!("Fast allocated {} bytes at {:?}", size, ptr);
                return Ok(Some(ptr));
            }
        }
        
        // No space available, need to trigger collection
        debug!("No space available for allocation of {} bytes", size);
        Ok(None)
    /// Perform copying collection
    #[instrument(skip(self, promote_callback))]
    pub fn collect(&self, promote_callback: Option<Box<dyn Fn(ObjectId, &[u8]) -> Result<(), String>>>) -> Result<CopyingStats, String> {
        info!("Starting copying collection");
        let collection_start = Instant::now();
        let collection_number = self.collection_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        
        // Record utilization before collection
        let space_utilization_before = {
            let from_space = self.from_space.lock()
                .map_err(|_| "Failed to acquire lock on from_space")?;
            from_space.utilization()
        
        // Clear forwarding table
        {
            let mut forwarding_table = self.forwarding_table.write()
                .map_err(|_| "Failed to acquire write lock on forwarding table")?;
            forwarding_table.clear();
        // Get root objects
        let root_objects = self.get_root_objects()?;
        info!("Found {} root objects", root_objects.len());
        
        // Perform copying
        let copying_start = Instant::now();
        let (objects_copied, bytes_copied, objects_promoted, bytes_promoted) = 
            self.copy_live_objects(root_objects, promote_callback)?;
        let copying_duration = copying_start.elapsed();
        
        // Flip spaces
        self.flip_spaces()?;
        
        // Finalize the collection (update references and cleanup)
        self.finalize_collection()?;
        
        // Calculate statistics
        let total_duration = collection_start.elapsed();
        let space_utilization_after = {
            let from_space = self.from_space.lock()
                .map_err(|_| "Failed to acquire lock on from_space")?;
            from_space.utilization()
        
        let allocation_rate = self.calculate_allocation_rate()?;
        let promotion_rate = if objects_copied > 0 {
            objects_promoted as f64 / objects_copied as f64
        } else {
            0.0
        let survival_rate = space_utilization_after / space_utilization_before.max(0.001);
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        let stats = CopyingStats {
        
        // Update stored statistics
        {
            let mut stored_stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            *stored_stats = stats.clone();
        // Update last collection time
        {
            let mut last_time = self.last_collection_time.lock()
                .map_err(|_| "Failed to acquire lock on last_collection_time")?;
            *last_time = Some(Instant::now());
        info!("Copying collection completed: {:?}", stats);
        Ok(stats)
    /// Copy all live objects from from-space to to-space
    fn copy_live_objects(
        promote_callback: Option<Box<dyn Fn(ObjectId, &[u8]) -> Result<(), String>>>
    ) -> Result<(usize, usize, usize, usize), String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if config.parallel_copying && config.copying_threads > 1 {
            self.parallel_copy_objects(root_objects, promote_callback)
        } else {
            self.sequential_copy_objects(root_objects, promote_callback)
        }
    }
    
    /// Sequential copying of objects
    fn sequential_copy_objects(
        promote_callback: Option<Box<dyn Fn(ObjectId, &[u8]) -> Result<(), String>>>
    ) -> Result<(usize, usize, usize, usize), String> {
        debug!("Sequential copying from {} roots", root_objects.len());
        
        let mut objects_copied = 0;
        let mut bytes_copied = 0;
        let mut objects_promoted = 0;
        let mut bytes_promoted = 0;
        let mut work_queue = VecDeque::new();
        
        // Initialize with root objects
        for root_id in root_objects {
            if let Some(work_item) = self.create_work_item(root_id)? {
                work_queue.push_back(work_item);
            }
        }
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        let start_time = Instant::now();
        
        // Process work queue
        while let Some(work_item) = work_queue.pop_front() {
            // Check time limit
            if let Some(time_limit) = config.copying_time_limit {
                if start_time.elapsed() > time_limit {
                    warn!("Copying phase time limit exceeded, stopping early");
                    break;
                }
            }
            
            // Decide whether to copy or promote
            if self.should_promote(&work_item)? {
                // Promote to old generation
                if let Some(ref callback) = promote_callback {
                    let object_data = self.get_object_data(work_item.old_ptr, work_item.size)?;
                    callback(work_item.object_id, &object_data)?;
                }
                objects_promoted += 1;
                bytes_promoted += work_item.size;
            } else {
                // Copy to to-space
                let new_ptr = self.copy_object(&work_item)?;
                if new_ptr.is_some() {
                    objects_copied += 1;
                    bytes_copied += work_item.size;
                    
                    // Add referenced objects to work queue
                    let referenced_objects = self.get_object_references(work_item.object_id)?;
                    for ref_id in referenced_objects {
                        if let Some(ref_work_item) = self.create_work_item(ref_id)? {
                            work_queue.push_back(ref_work_item);
                        }
                    }
                }
            }
        debug!("Sequential copying completed: {} objects copied, {} promoted", objects_copied, objects_promoted);
        Ok((objects_copied, bytes_copied, objects_promoted, bytes_promoted))
    /// Parallel copying of objects
    fn parallel_copy_objects(
        promote_callback: Option<Box<dyn Fn(ObjectId, &[u8]) -> Result<(), String>>>
    ) -> Result<(usize, usize, usize, usize), String> {
        debug!("Parallel copying from {} roots", root_objects.len());
        
        // Initialize work queue with root objects
        {
            let mut work_queue = self.work_queue.lock()
                .map_err(|_| "Failed to acquire lock on work queue")?;
            work_queue.clear();
            
            for root_id in root_objects {
                if let Some(work_item) = self.create_work_item(root_id)? {
                    work_queue.push_back(work_item);
                }
            }
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // Shared counters
        let objects_copied = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let bytes_copied = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let objects_promoted = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let bytes_promoted = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        
        // Create worker threads
        let mut handles = Vec::new();
        for thread_id in 0..config.copying_threads {
            let objects_copied_clone = Arc::clone(&objects_copied);
            let bytes_copied_clone = Arc::clone(&bytes_copied);
            let objects_promoted_clone = Arc::clone(&objects_promoted);
            let bytes_promoted_clone = Arc::clone(&bytes_promoted);
            
            // TODO: In a real implementation, we'd need to properly share the promote_callback
            // For now, we'll simulate promotion without the callback
            
            let collector_ref = unsafe { std::mem::transmute::<&CopyingCollector, &'static CopyingCollector>(self) };
            
            let handle = std::thread::spawn(move || {
                debug!("Starting copying thread {}", thread_id);
                collector_ref.copying_worker_thread(
                )
            });
            handles.push(handle);
        // Wait for all threads to complete
        for handle in handles {
            if let Err(e) = handle.join() {
                error!("Copying thread panicked: {:?}", e);
            }
        }
        
        let total_objects_copied = objects_copied.load(std::sync::atomic::Ordering::SeqCst);
        let total_bytes_copied = bytes_copied.load(std::sync::atomic::Ordering::SeqCst);
        let total_objects_promoted = objects_promoted.load(std::sync::atomic::Ordering::SeqCst);
        let total_bytes_promoted = bytes_promoted.load(std::sync::atomic::Ordering::SeqCst);
        
               total_objects_copied, total_objects_promoted);
        
        Ok((total_objects_copied, total_bytes_copied, total_objects_promoted, total_bytes_promoted))
    /// Worker thread for parallel copying
    fn copying_worker_thread(
    ) -> Result<(), String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        let start_time = Instant::now();
        
        loop {
            // Get work item from queue
            let work_item = {
                let mut work_queue = self.work_queue.lock()
                    .map_err(|_| "Failed to acquire lock on work queue")?;
                work_queue.pop_front()
            
            let work_item = match work_item {
                None => break, // No more work
            
            // Check time limit
            if let Some(time_limit) = config.copying_time_limit {
                if start_time.elapsed() > time_limit {
                    warn!("Copying thread time limit exceeded");
                    break;
                }
            }
            
            // Process the work item
            if self.should_promote(&work_item)? {
                // Promote to old generation (simplified)
                objects_promoted.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                bytes_promoted.fetch_add(work_item.size, std::sync::atomic::Ordering::SeqCst);
            } else {
                // Copy to to-space
                if self.copy_object(&work_item)?.is_some() {
                    objects_copied.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    bytes_copied.fetch_add(work_item.size, std::sync::atomic::Ordering::SeqCst);
                    
                    // Add referenced objects to work queue
                    let referenced_objects = self.get_object_references(work_item.object_id)?;
                    if !referenced_objects.is_empty() {
                        let mut work_queue = self.work_queue.lock()
                            .map_err(|_| "Failed to acquire lock on work queue")?;
                        
                        for ref_id in referenced_objects {
                            if let Some(ref_work_item) = self.create_work_item(ref_id)? {
                                work_queue.push_back(ref_work_item);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    /// Create a work item for an object
    fn create_work_item(&self, object_id: ObjectId) -> Result<Option<CopyingWorkItem>, String> {
        // Check if object is already copied
        {
            let forwarding_table = self.forwarding_table.read()
                .map_err(|_| "Failed to acquire read lock on forwarding table")?;
            if forwarding_table.contains_key(&object_id) {
                return Ok(None); // Already processed
            }
        }
        
        // Get object metadata from registry
        let metadata = self.object_registry.get(object_id)?
            .ok_or_else(|| format!("Object {} not found in registry", object_id))?;
        
        let size = metadata.size;
        let age = self.get_object_age(object_id);
        
        // Get object pointer from registry - this is a placeholder implementation
        // In a real system, the object registry would track actual pointers
        let object_ptr = self.get_object_pointer(object_id)?
            .ok_or_else(|| format!("No pointer found for object {}", object_id))?;
        
        Ok(Some(CopyingWorkItem {
        }))
    /// Get object pointer from object tracking system
    fn get_object_pointer(&self, object_id: ObjectId) -> Result<Option<NonNull<u8>>, String> {
        // This is a placeholder implementation. In a real system, you would:
        // 1. Look up the object in a heap management system
        // 2. Get the actual memory address where the object is stored
        // 3. Return the NonNull pointer to that location
        
        // For now, we'll use a mock implementation that creates a dummy pointer
        // based on the object ID (this is just for compilation purposes)
        if object_id.is_null() {
            return Ok(None);
        // WARNING: This is a placeholder - in real usage, this would be dangerous
        // We're creating a fake pointer that shouldn't be dereferenced
        let fake_addr = (object_id.as_u64() as usize) << 4; // Shift to avoid low addresses
        if fake_addr < 0x1000 {
            return Ok(None); // Avoid very low addresses
        // In a real implementation, this would come from the heap manager
        Ok(NonNull::new(fake_addr as *mut u8))
    /// Check if an object should be promoted to old generation
    fn should_promote(&self, work_item: &CopyingWorkItem) -> Result<bool, String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // Promote if object is old enough
        if work_item.age >= config.promotion_age_threshold {
            return Ok(true);
        // Promote if object is too large
        if work_item.size >= config.promotion_size_threshold {
            return Ok(true);
        Ok(false)
    /// Copy an object to to-space
    fn copy_object(&self, work_item: &CopyingWorkItem) -> Result<Option<NonNull<u8>>, String> {
        let mut to_space = self.to_space.lock()
            .map_err(|_| "Failed to acquire lock on to_space")?;
        
        // Try to allocate space in to-space
        if let Some(new_ptr) = to_space.allocate(work_item.size, 8) {
            // Copy object data from old location to new location
            unsafe {
                std::ptr::copy_nonoverlapping(
                    work_item.size
                );
            // Generate new ObjectId for the copied object to maintain identity separation
            let id_generator = ObjectIdGenerator::new();
            let new_object_id = id_generator.next();
            
            // Update forwarding table to map old object to new object
            {
                let mut forwarding_table = self.forwarding_table.write()
                    .map_err(|_| "Failed to acquire write lock on forwarding table")?;
                forwarding_table.insert(work_item.object_id, new_object_id);
            // Register the new object in the registry with updated metadata
            if let Ok(Some(old_metadata)) = self.object_registry.get(work_item.object_id) {
                let new_metadata = ObjectMetadata {
                    marked: false, // Reset mark status for new copy
                    ref_count: 0,  // Reset reference count
                
                if let Err(e) = self.object_registry.register(new_metadata) {
                    warn!("Failed to register copied object {}: {}", new_object_id, e);
                }
            }
            
            // Update object age in the new location
            self.update_object_age(new_object_id, work_item.age + 1)?;
            
                   work_item.object_id, work_item.old_ptr, new_ptr, new_object_id);
            Ok(Some(new_ptr))
        } else {
            warn!("No space available in to-space for object {}", work_item.object_id);
            Ok(None)
        }
    }
    
    /// Flip from-space and to-space
    fn flip_spaces(&self) -> Result<(), String> {
        debug!("Flipping spaces");
        
        let mut from_space = self.from_space.lock()
            .map_err(|_| "Failed to acquire lock on from_space")?;
        let mut to_space = self.to_space.lock()
            .map_err(|_| "Failed to acquire lock on to_space")?;
        
        // Reset the old from-space (now becoming to-space)
        from_space.reset();
        from_space.is_active = false;
        
        // Activate the new from-space (old to-space)
        to_space.is_active = true;
        
        // Swap the spaces
        std::mem::swap(&mut *from_space, &mut *to_space);
        
        debug!("Spaces flipped successfully");
        Ok(())
    /// Get all root objects
    fn get_root_objects(&self) -> Result<Vec<ObjectId>, String> {
        if let Some(root_manager) = &self.root_manager {
            // Get young generation roots (stack, temporary, etc.)
            let mut roots = root_manager.get_roots_by_type(RootType::Stack)?;
            roots.extend(root_manager.get_roots_by_type(RootType::Temporary)?);
            Ok(roots)
        } else {
            // Fallback: get all roots
            self.object_registry.get_root_objects()
        }
    }
    
    /// Get references from an object to other objects
    fn get_object_references(&self, object_id: ObjectId) -> Result<Vec<ObjectId>, String> {
        // This is a placeholder implementation for reference traversal.
        // In a real system, this would:
        // 1. Get the object from memory using its pointer
        // 2. Cast it to a Traceable object
        // 3. Use the visitor pattern to collect all referenced objects
        // 4. Return the list of referenced ObjectIds
        
        let mut references = Vec::new();
        
        // Get the object pointer
        if let Some(object_ptr) = self.get_object_pointer(object_id)? {
            // In a real implementation, we would:
            // 1. Cast the raw pointer to the appropriate object type
            // 2. Call the object's trace() method with a visitor
            // 3. The visitor would collect all referenced ObjectIds
            
            // For now, we'll simulate finding some references based on the object ID
            // This is just for demonstration and testing purposes
            let simulated_ref_count = (object_id.as_u64() % 3) as usize; // 0-2 references
            
            for i in 0..simulated_ref_count {
                let ref_id = ObjectId::new(object_id.as_u64() + i as u64 + 1);
                // Only add references to objects that exist in the registry
                if self.object_registry.get(ref_id)?.is_some() {
                    references.push(ref_id);
                }
            }
        debug!("Found {} references from object {}", references.len(), object_id);
        Ok(references)
    /// Get references using the Traceable interface (real implementation approach)
    fn get_object_references_traceable(&self, object_id: ObjectId) -> Result<Vec<ObjectId>, String> {
        // This shows how reference traversal would work with the Traceable trait
        let mut collected_refs = Vec::new();
        
        // Create a visitor that collects ObjectIds
        struct RefCollector {
        impl Visitor for RefCollector {
            fn visit(&mut self, obj: &dyn Traceable) {
                // In a real implementation, we would need a way to get the ObjectId
                // from a Traceable object reference. This might require:
                // 1. A method on Traceable to get the ObjectId
                // 2. Looking up the object in a pointer-to-ID mapping
                // 3. Using object headers that contain the ID
                
                // For now, this is a placeholder
                // In practice, you'd need object metadata to be embedded in objects
                // or maintain a reverse mapping from pointers to ObjectIds
            }
        }
        
        let mut visitor = RefCollector {
        
        // Get the object and trace its references
        if let Some(_object_ptr) = self.get_object_pointer(object_id)? {
            // In a real implementation:
            // 1. Cast pointer to &dyn Traceable
            // 2. Call object.trace(&mut visitor)
            // 3. Return visitor.refs
            
            // Placeholder: return empty list
        Ok(collected_refs)
    /// Get object data as byte slice
    fn get_object_data(&self, ptr: NonNull<u8>, size: usize) -> Result<Vec<u8>, String> {
        let mut data = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(ptr.as_ptr(), data.as_mut_ptr(), size);
        }
        Ok(data)
    /// Get age of an object
    fn get_object_age(&self, object_id: ObjectId) -> u8 {
        let object_ages = self.object_ages.read().unwrap();
        object_ages.get(&object_id).map(|age_info| age_info.age).unwrap_or(0)
    /// Update age of an object
    fn update_object_age(&self, object_id: ObjectId, new_age: u8) -> Result<(), String> {
        let mut object_ages = self.object_ages.write()
            .map_err(|_| "Failed to acquire write lock on object_ages")?;
        
        let age_info = object_ages.entry(object_id).or_insert(ObjectAge {
            size: 64, // TODO: Get actual size
        });
        
        age_info.age = new_age;
        Ok(())
    /// Calculate allocation rate
    fn calculate_allocation_rate(&self) -> Result<f64, String> {
        let last_time = self.last_collection_time.lock()
            .map_err(|_| "Failed to acquire lock on last_collection_time")?;
        
        if let Some(last_time) = *last_time {
            let duration = Instant::now().duration_since(last_time);
            let bytes = self.allocation_bytes.swap(0, std::sync::atomic::Ordering::SeqCst);
            
            if duration.as_secs_f64() > 0.0 {
                Ok(bytes as f64 / duration.as_secs_f64())
            } else {
                Ok(0.0)
            }
        } else {
            Ok(0.0)
        }
    }
    
    /// Check if collection is needed
    pub fn should_collect(&self) -> Result<bool, String> {
        let from_space = self.from_space.lock()
            .map_err(|_| "Failed to acquire lock on from_space")?;
        
        // Trigger collection if space is more than 80% full
        Ok(from_space.utilization() > 0.8)
    /// Get available space for allocation
    pub fn available_space(&self) -> Result<usize, String> {
        let from_space = self.from_space.lock()
            .map_err(|_| "Failed to acquire lock on from_space")?;
        
        let available = unsafe { 
            from_space.end_ptr.as_ptr().offset_from(from_space.alloc_ptr.as_ptr()) 
        } as usize;
        
        Ok(available)
    /// Get collection statistics
    pub fn get_stats(&self) -> Result<CopyingStats, String> {
        let stats = self.stats.read()
            .map_err(|_| "Failed to acquire read lock on stats")?;
        Ok(stats.clone())
    /// Update configuration
    pub fn update_config(&self, new_config: CopyingConfig) -> Result<(), String> {
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        *config = new_config;
        info!("Updated copying collector configuration");
        Ok(())
    /// Update references in copied objects to point to new locations
    fn update_object_references(&self) -> Result<(), String> {
        debug!("Updating object references after copying");
        
        let forwarding_table = self.forwarding_table.read()
            .map_err(|_| "Failed to acquire read lock on forwarding table")?;
        
        // Iterate through all objects that were copied
        for (old_id, new_id) in forwarding_table.iter() {
            // Get the new object's pointer and update its references
            if let Some(new_ptr) = self.get_object_pointer(*new_id)? {
                // In a real implementation, this would:
                // 1. Cast the pointer to the appropriate object type
                // 2. Scan through the object's fields for ObjectId references
                // 3. Update any ObjectId that appears in the forwarding table
                // 4. Recursively update nested object references
                
                self.update_references_in_object(*new_id, new_ptr)?;
            }
        }
        
        debug!("Reference update completed for {} objects", forwarding_table.len());
        Ok(())
    /// Update references within a specific object
    fn update_references_in_object(&self, object_id: ObjectId, object_ptr: NonNull<u8>) -> Result<(), String> {
        let forwarding_table = self.forwarding_table.read()
            .map_err(|_| "Failed to acquire read lock on forwarding table")?;
        
        // Get object metadata to understand its structure
        if let Some(metadata) = self.object_registry.get(object_id)? {
            debug!("Updating references in {} object of size {}", metadata.type_name, metadata.size);
            
            // Conservative approach: scan the object memory for potential ObjectId values
            let object_size = metadata.size();
            let object_slice = unsafe {
                std::slice::from_raw_parts_mut(object_ptr.as_ptr(), object_size)
            
            // Scan for ObjectId-sized values (8 bytes) aligned on 8-byte boundaries
            let mut offset = 0;
            while offset + 8 <= object_size {
                if offset % 8 == 0 { // Ensure proper alignment
                    let potential_id_bytes = &mut object_slice[offset..offset + 8];
                    let potential_id = u64::from_le_bytes(potential_id_bytes.try_into().unwrap());
                    let potential_object_id = ObjectId::from_raw(potential_id);
                    
                    // Check if this looks like a valid ObjectId and is in the forwarding table
                    if !potential_object_id.is_null() {
                        if let Some(&forwarded_id) = forwarding_table.get(&potential_object_id) {
                            // Update the reference to point to the new object
                            let new_id_bytes = forwarded_id.as_u64().to_le_bytes();
                            potential_id_bytes.copy_from_slice(&new_id_bytes);
                                   potential_object_id, forwarded_id, offset);
                        }
                    }
                }
                offset += 8; // Move to next potential ObjectId location
            }
            // 1. Cast pointer to the object's actual type
            // 2. Access fields that contain ObjectId references
            // 3. Update those references using the forwarding table
            
            // Example for a hypothetical object with ObjectId fields:
            /*
            unsafe {
                let obj = &mut *(object_ptr.as_ptr() as *mut MyObjectType);
                
                // Update direct ObjectId references
                if let Some(new_id) = forwarding_table.get(&obj.some_field_id) {
                    obj.some_field_id = *new_id;
                // Update ObjectId references in collections
                for ref_id in &mut obj.referenced_objects {
                    if let Some(new_id) = forwarding_table.get(ref_id) {
                        *ref_id = *new_id;
                    }
                }
            }
            */
        Ok(())
    /// Perform post-copy cleanup and finalization
    pub fn finalize_collection(&self) -> Result<(), String> {
        debug!("Finalizing copying collection");
        
        // Update all object references to point to copied objects
        self.update_object_references()?;
        
        // Remove old objects from registry
        self.cleanup_old_objects()?;
        
        // Clear temporary data structures
        {
            let mut forwarding_table = self.forwarding_table.write()
                .map_err(|_| "Failed to acquire write lock on forwarding table")?;
            forwarding_table.clear();
        {
            let mut work_queue = self.work_queue.lock()
                .map_err(|_| "Failed to acquire lock on work queue")?;
            work_queue.clear();
        info!("Copying collection finalization completed");
        Ok(())
    /// Remove old objects that were copied
    fn cleanup_old_objects(&self) -> Result<(), String> {
        debug!("Cleaning up old objects after copying");
        
        let forwarding_table = self.forwarding_table.read()
            .map_err(|_| "Failed to acquire read lock on forwarding table")?;
        
        let mut cleaned_count = 0;
        
        // Remove old object entries from registry
        for (old_id, _new_id) in forwarding_table.iter() {
            if self.object_registry.unregister(*old_id)?.is_some() {
                cleaned_count += 1;
            }
        }
        
        // Clean up age tracking for old objects
        {
            let mut object_ages = self.object_ages.write()
                .map_err(|_| "Failed to acquire write lock on object_ages")?;
            
            for (old_id, _) in forwarding_table.iter() {
                object_ages.remove(old_id);
            }
        }
        
        debug!("Cleaned up {} old objects", cleaned_count);
        Ok(())
    /// Get object size from registry
    pub fn get_object_size(&self, object_id: ObjectId) -> Result<Option<usize>, String> {
        if let Some(metadata) = self.object_registry.get(object_id)? {
            Ok(Some(metadata.size))
        } else {
            Ok(None)
        }
    }
    
    /// Check if an object has been forwarded (copied)
    pub fn is_object_forwarded(&self, object_id: ObjectId) -> Result<bool, String> {
        let forwarding_table = self.forwarding_table.read()
            .map_err(|_| "Failed to acquire read lock on forwarding table")?;
        Ok(forwarding_table.contains_key(&object_id))
    /// Get the forwarded location of an object
    pub fn get_forwarded_object(&self, object_id: ObjectId) -> Result<Option<ObjectId>, String> {
        let forwarding_table = self.forwarding_table.read()
            .map_err(|_| "Failed to acquire read lock on forwarding table")?;
        Ok(forwarding_table.get(&object_id).copied())
    /// Force a collection cycle (for testing and explicit control)
    pub fn force_collect(&self) -> Result<CopyingStats, String> {
        info!("Forcing copying collection");
        self.collect(None)
    /// Estimate copying efficiency for performance monitoring
    pub fn estimate_copying_efficiency(&self) -> Result<f64, String> {
        let from_space = self.from_space.lock()
            .map_err(|_| "Failed to acquire lock on from_space")?;
        
        let utilization = from_space.utilization();
        let stats = self.get_stats()?;
        
        // Calculate efficiency based on survival rate and space utilization
        let efficiency = if utilization > 0.0 {
            stats.survival_rate * (1.0 - utilization) // Higher efficiency with lower utilization and survival
        } else {
            1.0
        
        Ok(efficiency.clamp(0.0, 1.0))
    }
}

// Safety: CopyingCollector manages its own memory and uses appropriate synchronization
unsafe impl Send for CopyingCollector {}
unsafe impl Sync for CopyingCollector {}

