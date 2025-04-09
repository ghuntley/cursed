/*!
 * Standalone test for the garbage collector
 */

use std::cell::RefCell;
use std::collections::HashSet;
use std::ptr::NonNull;
use std::sync::{Arc, RwLock};
use std::any::TypeId;
use std::marker::PhantomData;

/// Enum to represent the color of a GC-managed object
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GcColor {
    /// Object is not currently being examined by the GC
    White,
    /// Object has been reached but not yet processed
    Gray,
    /// Object and all its references have been processed
    Black,
}

/// Tag for identifying object types in memory
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    Int,
    Float,
    String,
    Boolean,
    Array,
    Map,
    Function,
    Null,
    Object,
}

/// Trait for objects that can be traced by the garbage collector
pub trait Traceable: 'static {
    /// Trace all references from this object to other objects
    fn trace(&self, visitor: &mut dyn Visitor);
    
    /// Get the size of this object in bytes
    fn size(&self) -> usize;
    
    /// Get the type tag for this object
    fn tag(&self) -> Tag;
}

/// Extension trait for Traceable objects
pub trait Trace: Traceable {}

/// Trait for visitors that traverse object graphs
pub trait Visitor {
    /// Visit a traceable object
    fn visit(&mut self, obj: &dyn Traceable);
    
    /// Visit a pointer to an object
    fn visit_ptr(&mut self, ptr: usize, tag: Tag);
}

/// A garbage-collected reference
pub struct Gc<T: Traceable + Clone> {
    ptr: NonNull<T>,
    collector: Arc<GarbageCollector>,
    _marker: PhantomData<T>,
}

impl<T: Traceable + Clone> Gc<T> {
    /// Create a new garbage-collected reference
    pub fn new(value: T, collector: Arc<GarbageCollector>) -> Self {
        let ptr = collector.allocate(value);
        let gc_ref = Gc {
            ptr,
            collector: collector.clone(),
            _marker: PhantomData,
        };
        
        // Register the object with the collector
        let address = unsafe { ptr.as_ptr() as usize };
        collector.register_object(address, &gc_ref.clone_inner(), TypeId::of::<T>());
        
        gc_ref
    }
    
    /// Get a reference to the inner value
    pub fn inner(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
    
    /// Get a mutable reference to the inner value
    pub fn inner_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
    
    /// Clone the inner value
    fn clone_inner(&self) -> T {
        self.inner().clone()
    }
}

// Implement Clone for Gc<T>
impl<T: Traceable + Clone> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Gc {
            ptr: self.ptr,
            collector: self.collector.clone(),
            _marker: PhantomData,
        }
    }
}

// Debug implementation for Gc<T>
impl<T: Traceable + std::fmt::Debug + Clone> std::fmt::Debug for Gc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Gc")
            .field("inner", self.inner())
            .finish()
    }
}

/// A wrapper for objects managed by the garbage collector
struct GcObject {
    /// The memory address of the object
    address: usize,
    /// The type ID of the object
    type_id: TypeId,
    /// The object's current color in the GC algorithm
    color: GcColor,
    /// The size of the object in bytes
    size: usize,
    /// The type tag of the object
    tag: Tag,
}

/// Statistics about the garbage collector
#[derive(Debug, Clone, Copy)]
pub struct GcStats {
    /// Number of objects currently being managed
    pub object_count: usize,
    /// Total size of all managed objects in bytes
    pub total_size: usize,
    /// Number of garbage collections performed
    pub collection_count: usize,
}

/// The garbage collector implementation
#[derive(Clone)]
pub struct GarbageCollector {
    /// Inner state protected by a mutex
    inner: Arc<RwLock<GcState>>,
    /// Allocation threshold before triggering garbage collection
    allocation_threshold: usize,
}

/// The internal state of the garbage collector
struct GcState {
    /// Map of object addresses to their GC metadata
    objects: std::collections::HashMap<usize, GcObject>,
    /// Set of root objects that are always considered reachable
    roots: HashSet<usize>,
    /// Total size of all allocated objects
    total_size: usize,
    /// Number of collections performed
    collection_count: usize,
}

impl GarbageCollector {
    /// Create a new garbage collector with the given threshold
    pub fn new(threshold: usize) -> Arc<Self> {
        Arc::new(GarbageCollector {
            inner: Arc::new(RwLock::new(GcState {
                objects: std::collections::HashMap::new(),
                roots: HashSet::new(),
                total_size: 0,
                collection_count: 0,
            })),
            allocation_threshold: threshold,
        })
    }
    
    /// Allocate memory for an object and return a pointer to it
    pub fn allocate<T: Traceable + Clone>(&self, value: T) -> NonNull<T> {
        let mut inner = self.inner.write().unwrap();
        let size = value.size();
        
        // Check if we need to collect garbage
        if inner.total_size + size > self.allocation_threshold {
            drop(inner); // Release the lock before collection
            self.collect_garbage();
            inner = self.inner.write().unwrap();
        }
        
        // Allocate memory for the object
        let ptr = unsafe {
            let layout = std::alloc::Layout::new::<T>();
            let raw_ptr = std::alloc::alloc(layout);
            if raw_ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            let ptr = NonNull::new_unchecked(raw_ptr).cast::<T>();
            std::ptr::write(ptr.as_ptr(), value);
            ptr
        };
        
        // Update allocation size
        inner.total_size += size;
        
        ptr
    }
    
    /// Register an object with the collector
    pub fn register_object<T: Traceable + Clone>(&self, address: usize, obj: &T, type_id: TypeId) {
        let mut inner = self.inner.write().unwrap();
        
        let obj_info = GcObject {
            address,
            type_id,
            color: GcColor::White,
            size: obj.size(),
            tag: obj.tag(),
        };
        
        inner.objects.insert(address, obj_info);
    }
    
    /// Add an object to the root set
    pub fn add_root(&self, address: usize) {
        let mut inner = self.inner.write().unwrap();
        inner.roots.insert(address);
    }
    
    /// Remove an object from the root set
    pub fn remove_root(&self, address: usize) {
        let mut inner = self.inner.write().unwrap();
        inner.roots.remove(&address);
    }
    
    /// Collect garbage using mark-and-sweep algorithm
    pub fn collect_garbage(&self) {
        self.mark();
        self.sweep();
        
        // Update collection count
        let mut inner = self.inner.write().unwrap();
        inner.collection_count += 1;
    }
    
    /// Mark phase of the garbage collection algorithm
    fn mark(&self) {
        // Step 1: Reset all objects to white
        {
            let mut inner = self.inner.write().unwrap();
            for obj in inner.objects.values_mut() {
                obj.color = GcColor::White;
            }
        }
        
        // Step 2: Create a work list with root objects and mark objects as gray
        let mut work_list = Vec::new();
        
        // First collect the roots
        let roots = {
            let inner = self.inner.read().unwrap();
            inner.roots.clone()
        };
        
        // Then mark objects as gray and add to work list
        for root in &roots {
            let mut inner = self.inner.write().unwrap();
            if let Some(obj) = inner.objects.get_mut(root) {
                obj.color = GcColor::Gray;
                work_list.push(*root);
            }
        }
        
        // Step 3: Process the work list until it's empty
        while !work_list.is_empty() {
            let addr = work_list.pop().unwrap();
            
            // Mark as black and prepare for tracing
            let mut to_visit = false;
            let obj_type_id = {
                let mut inner = self.inner.write().unwrap();
                if let Some(obj) = inner.objects.get_mut(&addr) {
                    obj.color = GcColor::Black;
                    to_visit = true;
                    obj.type_id
                } else {
                    TypeId::of::<()>() // Default
                }
            };
            
            // Only continue if there's an object to trace
            if to_visit {
                // Create a visitor that adds gray objects to the work list
                let mut new_gray_objects = Vec::new();
                
                {
                    // Custom visitor that collects references
                    struct MarkVisitor<'a> {
                        gc: &'a GarbageCollector,
                        work_list: &'a mut Vec<usize>,
                    }
                    
                    impl<'a> Visitor for MarkVisitor<'a> {
                        fn visit(&mut self, _obj: &dyn Traceable) {
                            // Not used in this implementation
                        }
                        
                        fn visit_ptr(&mut self, ptr: usize, _tag: Tag) {
                            let mut inner = self.gc.inner.write().unwrap();
                            if let Some(obj) = inner.objects.get_mut(&ptr) {
                                if obj.color == GcColor::White {
                                    obj.color = GcColor::Gray;
                                    self.work_list.push(ptr);
                                }
                            }
                        }
                    }
                    
                    // Get a reference to the object and trace it
                    // This is unsafe but controlled - we're manually managing memory here
                    unsafe {
                        // Attempt to interpret the address as the right type based on TypeId
                        // In a real implementation, we'd use type information to create the right pointer type
                        let obj_ptr = addr as *const u8;
                        let mut visitor = MarkVisitor {
                            gc: self,
                            work_list: &mut new_gray_objects,
                        };
                        
                        // Trace the object's references
                        // Note: This is a simplified implementation. In production, you'd need proper type information.
                        if TypeId::of::<SimpleObject>() == obj_type_id {
                            let obj = &*(obj_ptr as *const SimpleObject);
                            obj.trace(&mut visitor);
                        } else if TypeId::of::<LinkedObject>() == obj_type_id {
                            let obj = &*(obj_ptr as *const LinkedObject);
                            obj.trace(&mut visitor);
                        }
                        // Add more type checks as needed
                    }
                }
                
                // Add newly discovered objects to the work list
                work_list.append(&mut new_gray_objects);
            }
        }
    }
    
    /// Sweep phase of the garbage collection algorithm
    fn sweep(&self) {
        let mut inner = self.inner.write().unwrap();
        
        // Collect addresses of unreachable objects
        let mut to_remove = Vec::new();
        for (addr, obj) in &inner.objects {
            if obj.color == GcColor::White {
                to_remove.push(*addr);
            }
        }
        
        // Remove unreachable objects
        for addr in to_remove {
            if let Some(obj) = inner.objects.remove(&addr) {
                // Deallocate the memory
                unsafe {
                    let ptr = NonNull::new_unchecked(addr as *mut u8);
                    let layout = std::alloc::Layout::from_size_align_unchecked(obj.size, 8); // 8-byte alignment
                    std::alloc::dealloc(ptr.as_ptr(), layout);
                }
                
                // Update total size
                inner.total_size -= obj.size;
            }
        }
    }
    
    /// Get statistics about the garbage collector
    pub fn stats(&self) -> GcStats {
        let inner = self.inner.read().unwrap();
        GcStats {
            object_count: inner.objects.len(),
            total_size: inner.total_size,
            collection_count: inner.collection_count,
        }
    }
}

/// A memory manager that uses the garbage collector
pub struct MemoryManager {
    /// The garbage collector
    gc: Arc<GarbageCollector>,
}

impl MemoryManager {
    /// Create a new memory manager with default settings
    pub fn new() -> Self {
        // Create a garbage collector with 10MB threshold
        let gc = GarbageCollector::new(1024 * 1024 * 10);
        
        MemoryManager { gc }
    }
    
    /// Create a new memory manager with a specific threshold
    pub fn with_threshold(threshold: usize) -> Self {
        let gc = GarbageCollector::new(threshold);
        MemoryManager { gc }
    }
    
    /// Allocate a new object in memory
    pub fn allocate<T: Traceable + Clone>(&self, value: T) -> Gc<T> {
        Gc::new(value, self.gc.clone())
    }
    
    /// Get a reference to the garbage collector
    pub fn gc(&self) -> Arc<GarbageCollector> {
        self.gc.clone()
    }
    
    /// Collect garbage
    pub fn collect_garbage(&self) {
        self.gc.collect_garbage();
    }
    
    /// Get statistics about memory usage
    pub fn stats(&self) -> GcStats {
        self.gc.stats()
    }
}

// Test objects

// A simple object for testing
#[derive(Debug, Clone)]
struct SimpleObject {
    value: i64,
}

impl Traceable for SimpleObject {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // Simple object has no references
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<SimpleObject>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Int
    }
}

impl Trace for SimpleObject {}

// A linked object for testing reference tracing
#[derive(Debug, Clone)]
struct LinkedObject {
    value: i64,
    next: Option<Gc<LinkedObject>>,
}

impl Traceable for LinkedObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        if let Some(next) = &self.next {
            visitor.visit_ptr(next.ptr.as_ptr() as usize, Tag::Object);
        }
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<LinkedObject>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

impl Trace for LinkedObject {}

// A cycle-friendly object for testing cycles
#[derive(Debug, Clone)]
struct CyclicObject {
    value: i64,
    next: RefCell<Option<Gc<CyclicObject>>>,
}

impl Traceable for CyclicObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        if let Some(next) = &*self.next.borrow() {
            visitor.visit_ptr(next.ptr.as_ptr() as usize, Tag::Object);
        }
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<CyclicObject>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

impl Trace for CyclicObject {}

// Main function with tests
fn main() {
    println!("Testing garbage collector implementation");
    
    // Test 1: Simple allocation and garbage collection
    {
        println!("\nTest 1: Simple allocation and collection");
        let mm = MemoryManager::new();
        
        // Allocate some objects in an inner scope
        {
            for i in 0..5 {
                let _obj = mm.allocate(SimpleObject { value: i });
                println!("  Allocated object with value {}", i);
            }
            
            let stats_before = mm.stats();
            println!("  Stats before collection: {} objects, {} bytes", 
                stats_before.object_count, stats_before.total_size);
        }
        
        // Objects should be garbage collected here
        mm.collect_garbage();
        
        let stats_after = mm.stats();
        println!("  Stats after collection: {} objects, {} bytes", 
            stats_after.object_count, stats_after.total_size);
        assert!(stats_after.object_count == 0, "Objects should have been collected");
    }
    
    // Test 2: Linked objects
    {
        println!("\nTest 2: Linked objects");
        let mm = MemoryManager::new();
        
        // Create a linked list of objects
        let obj3 = mm.allocate(LinkedObject { value: 3, next: None });
        let obj2 = mm.allocate(LinkedObject { value: 2, next: Some(obj3.clone()) });
        let obj1 = mm.allocate(LinkedObject { value: 1, next: Some(obj2.clone()) });
        
        println!("  Created linked list: {} -> {} -> {}", 
            obj1.inner().value, obj2.inner().value, obj3.inner().value);
        
        let stats_before = mm.stats();
        println!("  Stats before collection: {} objects, {} bytes", 
            stats_before.object_count, stats_before.total_size);
        
        // Force a collection - should keep all objects since they're referenced
        mm.collect_garbage();
        
        let stats_after = mm.stats();
        println!("  Stats after collection: {} objects, {} bytes", 
            stats_after.object_count, stats_after.total_size);
        assert_eq!(stats_after.object_count, 3, "All 3 linked objects should still exist");
        
        // Check the linked structure is still intact
        assert_eq!(obj1.inner().value, 1);
        assert_eq!(obj1.inner().next.as_ref().unwrap().inner().value, 2);
        assert_eq!(obj1.inner().next.as_ref().unwrap().inner().next.as_ref().unwrap().inner().value, 3);
        println!("  Linked list integrity verified");
    }
    
    // Test 3: Cyclic references
    {
        println!("\nTest 3: Cyclic references");
        let mm = MemoryManager::new();
        
        // Create objects with cyclic references
        let obj1 = mm.allocate(CyclicObject { 
            value: 1, 
            next: RefCell::new(None),
        });
        
        let obj2 = mm.allocate(CyclicObject { 
            value: 2, 
            next: RefCell::new(Some(obj1.clone())),
        });
        
        // Create a cycle
        *obj1.inner().next.borrow_mut() = Some(obj2.clone());
        
        println!("  Created cycle: {} <-> {}", obj1.inner().value, obj2.inner().value);
        
        let stats_before = mm.stats();
        println!("  Stats before collection: {} objects, {} bytes", 
            stats_before.object_count, stats_before.total_size);
        
        // Force a collection - should keep both objects since they're referenced by obj1
        mm.collect_garbage();
        
        let stats_after = mm.stats();
        println!("  Stats after collection: {} objects, {} bytes", 
            stats_after.object_count, stats_after.total_size);
        
        assert_eq!(stats_after.object_count, 2, "Both cyclic objects should still exist");
        
        // Verify the cycle is intact
        assert_eq!(obj1.inner().value, 1);
        assert_eq!(obj1.inner().next.borrow().as_ref().unwrap().inner().value, 2);
        assert_eq!(obj2.inner().next.borrow().as_ref().unwrap().inner().value, 1);
        println!("  Cycle integrity verified");
        
        // Now drop our reference to obj1 and obj2, creating an unreachable cycle
        drop(obj1);
        drop(obj2);
        
        // Force another collection - should collect both objects
        mm.collect_garbage();
        
        let stats_final = mm.stats();
        println!("  Stats after dropping references: {} objects, {} bytes", 
            stats_final.object_count, stats_final.total_size);
        assert_eq!(stats_final.object_count, 0, "All objects should be collected");
    }
    
    println!("\nAll tests passed! Garbage collector implementation is working.");
}