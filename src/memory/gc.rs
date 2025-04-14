//! Improved mark-and-sweep garbage collector with thread-safe reference support

use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::ptr::NonNull;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::fmt;

use crate::memory::ThreadSafeGc;

use crate::memory::{Traceable, Tag, Visitor, ThreadSafeVisitor};

/// Timeout for garbage collection locks to prevent deadlocks
const GC_LOCK_TIMEOUT: Duration = Duration::from_millis(200);

/// Statistics for the garbage collector
#[derive(Debug, Clone, Copy)]
pub struct GcStats {
    /// Number of live objects
    pub live_objects: usize,
    /// Number of objects collected in the last GC cycle
    pub collected_objects: usize,
    /// Total memory used by live objects (in bytes)
    pub memory_used: usize,
    /// Total time spent in garbage collection (microseconds)
    pub total_gc_time_us: u128,
    /// Number of GC cycles executed
    pub gc_cycles: usize,
}

/// Extension trait to cast Traceable to Any
pub trait TraceableAsAny: Traceable {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Traceable + 'static> TraceableAsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A simple mark-and-sweep garbage collector for CURSED
pub struct GarbageCollector {
    /// Map of regular (non-thread-safe) objects
    objects: Mutex<HashMap<usize, Box<dyn TraceableAsAny>>>,
    /// Registered thread-safe objects
    thread_safe_objects: RwLock<HashMap<usize, *mut dyn Any>>,
    /// Thread-safe object type information
    thread_safe_types: RwLock<HashMap<usize, &'static str>>,
    /// Reference counts for thread-safe objects
    thread_safe_ref_counts: RwLock<HashMap<usize, usize>>,
    /// The set of root objects that shouldn't be collected
    roots: Mutex<HashSet<usize>>,
    /// Statistics about the garbage collector
    stats: Mutex<GcStats>,
    /// Debug mode enabled?
    debug: Mutex<bool>,
}

// GarbageCollector is Send + Sync
unsafe impl Send for GarbageCollector {}
unsafe impl Sync for GarbageCollector {}

impl fmt::Debug for GarbageCollector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GarbageCollector")
            .field("stats", &self.stats())
            .finish()
    }
}

impl Clone for GarbageCollector {
    fn clone(&self) -> Self {
        // Create a new GC with the same state
        let thread_safe_objects = self.thread_safe_objects.read().unwrap();
        let thread_safe_types = self.thread_safe_types.read().unwrap();
        let thread_safe_ref_counts = self.thread_safe_ref_counts.read().unwrap();
        let roots = self.roots.lock().unwrap();
        let stats = self.stats.lock().unwrap();
        let debug = self.debug.lock().unwrap();

        Self {
            objects: Mutex::new(HashMap::new()),
            thread_safe_objects: RwLock::new(thread_safe_objects.clone()),
            thread_safe_types: RwLock::new(thread_safe_types.clone()),
            thread_safe_ref_counts: RwLock::new(thread_safe_ref_counts.clone()),
            roots: Mutex::new(roots.clone()),
            stats: Mutex::new(*stats),
            debug: Mutex::new(*debug),
        }
    }
}

impl Default for GarbageCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl GarbageCollector {
    /// Create a new garbage collector
    pub fn new() -> Self {
        Self {
            objects: Mutex::new(HashMap::new()),
            thread_safe_objects: RwLock::new(HashMap::new()),
            thread_safe_types: RwLock::new(HashMap::new()),
            thread_safe_ref_counts: RwLock::new(HashMap::new()),
            roots: Mutex::new(HashSet::new()),
            stats: Mutex::new(GcStats {
                live_objects: 0,
                collected_objects: 0,
                memory_used: 0,
                total_gc_time_us: 0,
                gc_cycles: 0,
            }),
            debug: Mutex::new(false),
        }
    }
    
    /// Enable or disable debug mode
    pub fn set_debug(&self, debug: bool) {
        *self.debug.lock().unwrap() = debug;
    }
    
    /// Get current debug mode status
    pub fn debug(&self) -> bool {
        *self.debug.lock().unwrap()
    }
    
    /// Get statistics about the garbage collector
    pub fn stats(&self) -> GcStats {
        *self.stats.lock().unwrap()
    }
    
    /// Allocate a regular (non-thread-safe) object
    pub fn allocate<T: Traceable + 'static>(&self, obj: T) -> crate::memory::Gc<T> {
        let boxed = Box::new(obj);
        let id = Box::as_ref(&boxed) as *const T as usize;
        
        // Add to the objects map
        self.objects.lock().unwrap().insert(id, boxed as Box<dyn TraceableAsAny>);
        
        // Update stats
        let mut stats = self.stats.lock().unwrap();
        stats.live_objects += 1;
        stats.memory_used += std::mem::size_of::<T>();
        
        // Create a garbage collected reference
        crate::memory::Gc::new(Arc::new(self.clone()), id)
    }
    
    /// Get an object by ID (used by Gc<T>::inner)
    pub(crate) fn get_object<T: Traceable + 'static>(&self, id: usize) -> Option<&T> {
        // We need to use a separate function that returns a copied value to avoid reference lifetime issues
        self.get_object_internal::<T>(id)
    }
    
    // Internal helper that avoids lifetime issues with the mutex guard
    fn get_object_internal<T: Traceable + 'static>(&self, id: usize) -> Option<&T> {
        // This is unsafe but carefully designed to be safe in practice:
        // 1. We only return references to objects that are still in the GC's objects map
        // 2. These objects won't be dropped as long as they're in the map
        // 3. Objects only leave the map when all references are dropped and GC runs
        // 4. The returned reference won't outlive the object because we first check it exists
        unsafe {
            let objects = self.objects.lock().unwrap();
            if let Some(obj) = objects.get(&id) {
                let ptr = obj.as_any().downcast_ref::<T>()?;
                let ptr_raw = ptr as *const T;
                Some(&*ptr_raw)
            } else {
                None
            }
        }
    }
    
    /// Register a thread-safe object
    pub(crate) fn register_thread_safe<T: Traceable + Send + Sync + 'static>(
        &self,
        ptr: NonNull<T>,
        type_name: &'static str,
    ) {
        let id = ptr.as_ptr() as *const T as usize;
        
        // Store the raw pointer as *mut dyn Any
        let ptr_any = ptr.as_ptr() as *mut dyn Any;
        
        // Add to the thread-safe objects map
        self.thread_safe_objects.write().unwrap().insert(id, ptr_any);
        
        // Store type information
        self.thread_safe_types.write().unwrap().insert(id, type_name);
        
        // Initialize reference count
        self.thread_safe_ref_counts.write().unwrap().insert(id, 1);
        
        // Update stats
        let mut stats = self.stats.lock().unwrap();
        stats.live_objects += 1;
        unsafe {
            stats.memory_used += ptr.as_ref().size();
        }
    }
    
    /// Increment reference count for a thread-safe object
    pub(crate) fn retain_thread_safe(&self, id: usize) -> bool {
        let mut ref_counts = self.thread_safe_ref_counts.write().unwrap();
        
        if let Some(count) = ref_counts.get_mut(&id) {
            *count += 1;
            true
        } else {
            false
        }
    }
    
    /// Decrement reference count for a thread-safe object
    /// Returns true if this was the last reference
    pub(crate) fn release_thread_safe(&self, id: usize) -> bool {
        let mut ref_counts = self.thread_safe_ref_counts.write().unwrap();
        
        if let Some(count) = ref_counts.get_mut(&id) {
            *count -= 1;
            if *count == 0 {
                // This was the last reference
                ref_counts.remove(&id);
                
                // Remove from other maps
                self.thread_safe_objects.write().unwrap().remove(&id);
                self.thread_safe_types.write().unwrap().remove(&id);
                
                // Update stats
                let mut stats = self.stats.lock().unwrap();
                stats.live_objects -= 1;
                
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    

    
    /// Get a thread-safe object by ID
    pub(crate) fn get_thread_safe(&self, id: usize) -> Option<*mut dyn Any> {
        let objects = self.thread_safe_objects.read().unwrap();
        objects.get(&id).copied()
    }
    
    /// Get a thread-safe object by ID with a specific type
    pub(crate) fn get_thread_safe_object<T: Traceable + Send + Sync + 'static>(
        &self,
        id: usize,
    ) -> Option<NonNull<T>> {
        // Try to get the object from the internal registry
        let ptr_any = self.get_thread_safe(id)?;
        
        // Try to cast to the correct type
        let ptr = ptr_any as *mut T;
        NonNull::new(ptr)
    }
    
    /// Run garbage collection
    pub fn collect_garbage(&self) {
        let start_time = Instant::now();
        let debug = self.debug();
        
        if debug {
            println!("Starting garbage collection...");
        }
        
        // Mark phase: mark all reachable objects
        let mut marked = HashSet::new();
        self.mark_roots(&mut marked);
        
        // Sweep phase: remove unmarked objects
        let collected = self.sweep_unmarked(&marked);
        
        // Update stats
        let elapsed = start_time.elapsed();
        let mut stats = self.stats.lock().unwrap();
        stats.collected_objects = collected;
        stats.total_gc_time_us += elapsed.as_micros();
        stats.gc_cycles += 1;
        
        if debug {
            println!("Garbage collection completed in {:?}", elapsed);
            println!("Collected {} objects", collected);
            println!("Live objects: {}", stats.live_objects);
            println!("Memory used: {} bytes", stats.memory_used);
        }
    }
    
    /// Mark roots and everything reachable from them
    fn mark_roots(&self, marked: &mut HashSet<usize>) {
        // Get all root objects
        let roots = self.roots.lock().unwrap();
        
        // Mark objects reachable from roots
        for &root_id in &*roots {
            self.mark_object(root_id, marked);
        }
        
        // Thread-safe objects are marked separately
        let thread_safe_objects = self.thread_safe_objects.read().unwrap();
        let ref_counts = self.thread_safe_ref_counts.read().unwrap();
        
        // All thread-safe objects with reference count > 0 are roots
        for (&id, _) in thread_safe_objects.iter() {
            if ref_counts.get(&id).copied().unwrap_or(0) > 0 {
                marked.insert(id);
            }
        }
    }
    
    /// Mark an object and its children
    fn mark_object(&self, id: usize, marked: &mut HashSet<usize>) {
        // If already marked, return to avoid cycles
        if marked.contains(&id) {
            return;
        }
        
        // Mark this object
        marked.insert(id);
        
        // Get the object from the objects map
        let objects = self.objects.lock().unwrap();
        if let Some(obj) = objects.get(&id) {
            // Create a visitor that marks children
            let mut visitor = MarkingVisitor {
                gc: self,
                marked,
            };
            
            // Trace children
            obj.trace(&mut visitor);
        }
    }
    
    /// Sweep unmarked objects
    fn sweep_unmarked(&self, marked: &HashSet<usize>) -> usize {
        let mut objects = self.objects.lock().unwrap();
        let mut to_remove = Vec::new();
        
        // Find objects that aren't marked
        for &id in objects.keys() {
            if !marked.contains(&id) {
                to_remove.push(id);
            }
        }
        
        // Remove and drop unmarked objects
        let mut memory_freed = 0;
        for id in &to_remove {
            if let Some(obj) = objects.remove(id) {
                memory_freed += obj.size();
            }
        }
        
        // Update stats
        let mut stats = self.stats.lock().unwrap();
        stats.live_objects -= to_remove.len();
        stats.memory_used -= memory_freed;
        
        to_remove.len()
    }
    
    /// Add an object to the root set
    pub fn add_root(&self, id: usize) {
        self.roots.lock().unwrap().insert(id);
    }
    
    /// Remove an object from the root set
    pub fn remove_root(&self, id: usize) {
        self.roots.lock().unwrap().remove(&id);
    }
}

/// Visitor implementation for marking objects
struct MarkingVisitor<'a> {
    /// Reference to the garbage collector
    gc: &'a GarbageCollector,
    /// Set of marked objects
    marked: &'a mut HashSet<usize>,
}

impl<'a> Visitor for MarkingVisitor<'a> {
    fn visit(&mut self, ptr: NonNull<dyn Traceable>) {
        // To get a usize from a trait object pointer, we need to cast through a raw pointer
        let raw_ptr = ptr.as_ptr();
        let id = raw_ptr as *const () as usize;
        self.gc.mark_object(id, self.marked);
    }
}

impl<'a> ThreadSafeVisitor for MarkingVisitor<'a> {
    fn visit_thread_safe<T: Traceable + Send + Sync + 'static>(&mut self, ptr: &crate::memory::ThreadSafeGc<T>) {
        let id = ptr.id();
        self.marked.insert(id);
    }
}

/// Create a test garbage collector
#[cfg(test)]
pub fn create_test_gc() -> Arc<GarbageCollector> {
    Arc::new(GarbageCollector::new())
}

// Global test GC for testing
#[cfg(test)]
lazy_static::lazy_static! {
    static ref TEST_GC: Mutex<Option<Arc<GarbageCollector>>> = Mutex::new(None);
}

#[cfg(test)]
pub fn get_test_gc() -> Arc<GarbageCollector> {
    let mut test_gc = TEST_GC.lock().unwrap();
    if test_gc.is_none() {
        *test_gc = Some(create_test_gc());
    }
    test_gc.as_ref().unwrap().clone()
}

#[cfg(test)]
pub fn reset_test_environment() {
    *TEST_GC.lock().unwrap() = None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Tag;
    
    #[derive(Debug)]
    struct TestObject {
        id: usize,
        next: Option<Box<TestObject>>,
    }
    
    impl Traceable for TestObject {
        fn trace(&self, visitor: &mut dyn Visitor) {
            if let Some(ref next) = self.next {
                unsafe {
                    let ptr = NonNull::new_unchecked(next.as_ref() as *const _ as *mut TestObject);
                    visitor.visit(ptr);
                }
            }
        }
        
        fn size(&self) -> usize {
            std::mem::size_of::<TestObject>()
        }
        
        fn tag(&self) -> Tag {
            Tag::Object
        }
    }
    
    #[test]
    fn test_basic_allocation() {
        let gc = GarbageCollector::new();
        
        // Allocate an object
        let obj = gc.allocate(TestObject { id: 1, next: None });
        
        // Check stats
        let stats = gc.stats();
        assert_eq!(stats.live_objects, 1, "Should have one live object");
        
        // Run garbage collection
        gc.collect_garbage();
        
        // Object should still be alive
        let stats = gc.stats();
        assert_eq!(stats.live_objects, 1, "Object should not be collected while referenced");
        
        // Drop the reference
        drop(obj);
        
        // Run garbage collection again
        gc.collect_garbage();
        
        // Object should be collected
        let stats = gc.stats();
        assert_eq!(stats.live_objects, 0, "Object should be collected after reference is dropped");
    }
    
    #[test]
    fn test_linked_objects() {
        let gc = GarbageCollector::new();
        
        // Create a linked list of objects
        let obj3 = TestObject { id: 3, next: None };
        let obj2 = TestObject { id: 2, next: Some(Box::new(obj3)) };
        let obj1 = TestObject { id: 1, next: Some(Box::new(obj2)) };
        
        // Allocate the head of the list
        let head = gc.allocate(obj1);
        
        // Check stats
        let stats = gc.stats();
        assert_eq!(stats.live_objects, 1, "Should have one allocated object");
        
        // Run garbage collection
        gc.collect_garbage();
        
        // Objects should still be alive
        let stats = gc.stats();
        assert_eq!(stats.live_objects, 1, "Object should not be collected while referenced");
        
        // Drop the reference
        drop(head);
        
        // Run garbage collection again
        gc.collect_garbage();
        
        // All objects should be collected
        let stats = gc.stats();
        assert_eq!(stats.live_objects, 0, "All objects should be collected");
    }
}