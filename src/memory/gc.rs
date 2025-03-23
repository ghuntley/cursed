use std::alloc::Layout;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::rc::Rc;
use std::any::{TypeId};
use std::mem;
use std::fmt;
use std::ops::Deref;
use std::iter::Iterator;
use std::slice::Iter;
use std::collections::HashSet;
use crate::prelude::VecExt;
use std::convert::TryFrom;
use std::collections::HashMap;

use super::allocator::Allocator;
use super::block::BlockAllocator;
use super::tagged::{Tag, TaggedPtr, TAG_MASK, TAG_SHIFT, PTR_MASK, NonNullExt};
use super::MemoryError;
use super::{align_up, MIN_ALIGNMENT, DEFAULT_BLOCK_SIZE};
use crate::error::Error;

/// A trait for objects that can be garbage collected
pub trait Traceable {
    /// Visit all pointers that this object refers to
    fn trace(&self, visitor: &mut dyn Visitor);
    
    /// Get the size of this object in bytes
    fn size(&self) -> usize;
}

/// A visitor used during the tracing phase of garbage collection
pub trait Visitor {
    /// Visit a pointer to a traceable object
    fn visit(&mut self, obj: &dyn std::any::Any);
    
    /// Visit a raw pointer
    fn visit_ptr(&mut self, ptr: usize, tag: Tag);
}

/// A trait for objects that can be traced during garbage collection
pub trait Trace {
    /// Trace all pointers that this object refers to
    fn trace(&self, visitor: &mut dyn Visitor);
}

/// Function type for tracing object references
pub type TraceFunc = fn(*const u8, &mut dyn Visitor);

/// Function type for calculating object size
pub type SizeFunc = fn(*const u8) -> usize;

/// Statistics about the garbage collector
#[derive(Default, Debug, Clone)]
pub struct GcStats {
    /// Number of collections performed
    pub collections: usize,
    
    /// Number of objects collected
    pub collected_objects: usize,
    
    /// Number of objects allocated
    pub live_objects: usize,
    
    /// Number of bytes allocated
    pub bytes_allocated: usize,
    
    /// Number of bytes freed
    pub bytes_freed: usize,
}

/// Helper struct for collecting objects during garbage collection
struct ObjectCollector<'a> {
    /// Reference to the garbage collector
    gc: &'a mut GarbageCollector,
    /// Objects pending processing 
    pending: Vec<usize>,
}

impl<'a> Visitor for ObjectCollector<'a> {
    fn visit(&mut self, _obj: &dyn std::any::Any) {
        // Not implemented for now
    }
    
    fn visit_ptr(&mut self, ptr: usize, _tag: Tag) {
        // Find the object index by its address
        if let Some(&idx) = self.gc.allocated_objects.get(&ptr) {
            self.pending.push(idx);
        }
    }
}

/// A simple mark-and-sweep garbage collector
pub struct GarbageCollector {
    /// The objects being managed
    objects: Vec<GcObject>,
    
    /// The root objects that should never be collected
    roots: HashSet<usize>,
    
    /// The allocator used for memory allocation
    allocator: Rc<RefCell<BlockAllocator>>,
    
    /// Statistics about the garbage collector
    stats: GcStats,
    
    /// Counter for the number of allocations since last collection
    collect_counter: usize,
    
    /// Threshold for triggering garbage collection
    allocation_threshold: usize,
    
    /// Allocated objects mapped by address
    allocated_objects: HashMap<usize, usize>,
    
    /// Set of marked objects during collection
    marked: HashSet<usize>,
}

// Default collection threshold: collect after 1000 allocations
const ALLOCATION_THRESHOLD: usize = 1000;

impl GarbageCollector {
    /// Create a new garbage collector with the given heap size
    pub fn new(heap_size: usize) -> Self {
        let block_allocator = Rc::new(RefCell::new(BlockAllocator::new(heap_size)));
        
        Self {
            objects: Vec::new(),
            roots: HashSet::new(),
            allocator: block_allocator,
            stats: GcStats::default(),
            collect_counter: 0,
            allocation_threshold: ALLOCATION_THRESHOLD,
            allocated_objects: HashMap::new(),
            marked: HashSet::new(),
        }
    }
    
    /// Allocate a new object in the garbage collector
    pub fn allocate<T: 'static + Traceable>(
        &mut self, 
        value: T, 
        tag: Tag
    ) -> Result<Gc<T>, MemoryError> {
        // Check if we should run garbage collection
        if self.should_collect() {
            self.run_collection();
        }
        
        // Define trace and size functions for type T
        fn trace_fn<U: Traceable>(ptr: *const u8, visitor: &mut dyn Visitor) {
            let obj = unsafe { &*(ptr as *const U) };
            obj.trace(visitor);
        }
        
        fn size_fn<U: Traceable>(ptr: *const u8) -> usize {
            let obj = unsafe { &*(ptr as *const U) };
            obj.size()
        }
        
        // Calculate size and alignment for the object
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        let layout = Layout::from_size_align(size, align)
            .map_err(|_| MemoryError::InvalidLayout)?;
        
        // Allocate memory using the underlying allocator
        let ptr = self.allocator.borrow_mut().allocate(layout)
            .map_err(|e| MemoryError::GCError(format!("Allocation failed: {}", e)))?;
        
        // Initialize the object in the allocated memory
        let ptr_t = ptr.as_ptr() as *mut T;
        unsafe {
            std::ptr::write(ptr_t, value);
        }
        
        // Create a GcObject to track this allocation
        let gc_object = GcObject {
            ptr,
            layout,
            tag,
            marked: false,
            trace_fn: trace_fn::<T>,
            size_fn: size_fn::<T>,
        };
        
        // Record the allocation
        let obj_idx = self.objects.len();
        self.objects.push(gc_object);
        self.allocated_objects.insert(ptr.as_ptr() as usize, obj_idx);
        
        // Increment allocation counter
        self.collect_counter += 1;
        
        // Update statistics
        self.stats.live_objects = self.objects.len();
        self.stats.bytes_allocated += layout.size();
        
        // Create the Gc handle with pointer to the allocated object
        let gc = Gc {
            collector: Rc::new(RefCell::new(self.clone())),
            ptr: NonNull::new(ptr_t).unwrap(),
            tag,
            _phantom: PhantomData,
        };
        
        Ok(gc)
    }
    
    /// Add a root object that should never be collected
    pub fn add_root(&mut self, ptr: usize) {
        self.roots.insert(ptr);
    }
    
    /// Remove a root object
    pub fn remove_root(&mut self, ptr: usize) {
        self.roots.remove(&ptr);
    }
    
    /// Check if garbage collection should run
    pub fn should_collect(&self) -> bool {
        self.collect_counter >= self.allocation_threshold
    }
    
    /// Run a complete garbage collection cycle (mark and sweep)
    pub fn run_collection(&mut self) {
        self.mark();
        self.sweep();
        self.collect_counter = 0;
        self.stats.collections += 1;
    }
    
    /// Mark phase: mark all reachable objects
    fn mark(&mut self) {
        // Clear the marked set
        self.marked.clear();
        
        // Mark all roots
        for &root_index in &self.roots {
            self.mark_object(root_index);
        }
    }
    
    /// Mark a single object and its references
    fn mark_object(&mut self, obj_index: usize) {
        // Skip if already marked
        if self.marked.contains(&obj_index) {
            return;
        }
        
        // Mark this object
        self.marked.insert(obj_index);
        
        // Mark references from this object
        if let Some(obj) = self.objects.get(obj_index) {
            // Use the trace function to find all references from this object
            if let Some(trace_fn) = obj.trace_fn {
                // Create a visitor that will mark all referenced objects
                let mut collector = ObjectCollector {
                    gc: self,
                    pending: Vec::new(),
                };
                
                unsafe {
                    trace_fn(obj.ptr.as_ptr(), &mut collector as *mut _ as *mut dyn Visitor);
                }
                
                // Process any pending objects
                while let Some(idx) = collector.pending.pop() {
                    self.mark_object(idx);
                }
            }
        }
    }
    
    /// Sweep unreachable objects
    fn sweep(&mut self) {
        let mut i = 0;
        let mut freed_count = 0;
        let mut freed_bytes = 0;
        
        while i < self.objects.len() {
            if !self.marked.contains(&i) {
                // Free this unmarked object
                let obj = &self.objects[i];
                let size = unsafe { (obj.size_fn)(obj.ptr.as_ptr()) };
                
                // Remove from allocation map
                self.allocated_objects.remove(&(obj.ptr.as_ptr() as usize));
                
                // Deallocate memory
                unsafe {
                    self.allocator.borrow_mut().deallocate(obj.ptr, obj.layout);
                }
                
                // If this is not the last object, swap with the last one for efficient removal
                if i < self.objects.len() - 1 {
                    // Get the last object's pointer address
                    let last_idx = self.objects.len() - 1;
                    let last_obj_ptr = self.objects[last_idx].ptr.as_ptr() as usize;
                    
                    // Update the map entry for the last object to its new position at i
                    self.allocated_objects.insert(last_obj_ptr, i);
                    
                    // Swap and remove
                    self.objects.swap_remove(i);
                } else {
                    // If it's the last element, just pop it
                    self.objects.pop();
                }
                
                // Update statistics
                freed_count += 1;
                freed_bytes += size;
            } else {
                // Clear marked flag for the next collection cycle
                i += 1;
            }
        }
        
        // Reset marked objects
        self.marked.clear();
        
        // Update statistics
        self.stats.collected_objects += freed_count;
        self.stats.bytes_freed += freed_bytes;
        self.stats.live_objects = self.objects.len();
    }
    
    /// Get the capacity of the allocator
    pub fn capacity(&self) -> usize {
        // Use the block allocator's capacity
        self.allocator.borrow().total_size
    }
    
    /// Get garbage collector statistics
    pub fn stats(&self) -> &GcStats {
        &self.stats
    }
    
    /// Reset the garbage collector
    pub fn reset(&mut self) -> Result<(), MemoryError> {
        // Free all objects
        for obj in self.objects.drain(..) {
            unsafe {
                self.allocator.borrow_mut().deallocate(obj.ptr, obj.layout);
            }
        }
        
        // Clear roots
        self.roots.clear();
        
        // Reset allocator
        if let Err(e) = self.allocator.borrow_mut().reset() {
            return Err(MemoryError::GCError(format!("Failed to reset GC allocator: {:?}", e)));
        }
        
        // Reset stats
        self.stats = GcStats::default();
        
        Ok(())
    }
    
    /// Get the heap usage
    pub fn heap_usage(&self) -> usize {
        self.stats.bytes_allocated - self.stats.bytes_freed
    }
    
    /// Get the heap capacity
    pub fn heap_capacity(&self) -> usize {
        // Get the capacity from the underlying allocator
        self.allocator.borrow().capacity()
    }
}

impl Clone for GarbageCollector {
    fn clone(&self) -> Self {
        Self {
            objects: self.objects.clone(),
            roots: self.roots.clone(),
            allocator: self.allocator.clone(),
            stats: self.stats.clone(),
            collect_counter: self.collect_counter,
            allocation_threshold: self.allocation_threshold,
            allocated_objects: self.allocated_objects.clone(),
            marked: self.marked.clone(),
        }
    }
}

// Implement Allocator for GarbageCollector
impl Allocator for GarbageCollector {
    fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, Error> {
        // Forward to the underlying allocator
        self.allocator.borrow_mut().allocate(layout)
    }
    
    unsafe fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout) {
        // Forward to the underlying allocator
        self.allocator.borrow_mut().deallocate(ptr, layout);
    }
    
    fn reset(&mut self) -> Result<(), Error> {
        // Clear all objects
        for obj in &self.objects {
            unsafe {
                self.allocator.borrow_mut().deallocate(obj.ptr, obj.layout);
            }
        }
        
        // Clear all state
        self.objects.clear();
        self.allocated_objects.clear();
        self.marked.clear();
        self.roots.clear();
        self.collect_counter = 0;
        
        // Reset the underlying allocator
        self.allocator.borrow_mut().reset()
    }
}

impl<T: 'static> Trace for Gc<T> where T: Traceable {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Since we don't have direct access to TaggedPtr's value field,
        // use raw pointer casting and visit the actual pointer
        let ptr_raw = self.ptr.as_ptr();
        visitor.visit_ptr(ptr_raw as usize, self.tag);
    }
}

impl<T: 'static> Gc<T> {
    /// Get a reference to the underlying value
    pub fn get(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
    
    /// Get a mutable reference to the object
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
    
    /// Run garbage collection
    pub fn collect(&self) {
        self.collector.borrow_mut().run_collection();
    }
    
    /// Get the tag of the object
    pub fn tag(&self) -> Tag {
        self.tag
    }
    
    /// Get the pointer to the object
    pub fn ptr(&self) -> Option<NonNull<T>> {
        Some(self.ptr)
    }
}

impl<T: 'static> Deref for Gc<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T: 'static> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Self {
            collector: self.collector.clone(),
            ptr: self.ptr,
            tag: self.tag,
            _phantom: PhantomData,
        }
    }
}

// Helper trait to adapt the new visitor to TaggedPtr
trait VisitAdapter {
    fn visit_tagged<T: Traceable>(&self, visitor: &mut dyn Visitor);
}

impl<T: Traceable> VisitAdapter for TaggedPtr<T> {
    fn visit_tagged<U: Traceable>(&self, visitor: &mut dyn Visitor) {
        if let Some(ptr) = self.ptr() {
            visitor.visit_ptr(ptr.as_ptr() as usize, self.tag());
        }
    }
}

/// A simple example of a traceable object
#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestObject {
        value: i32,
        next: Option<Gc<TestObject>>,
    }
    
    impl Traceable for TestObject {
        fn trace(&self, visitor: &mut dyn Visitor) {
            if let Some(ref next) = self.next {
                if let Some(ptr) = next.ptr() {
                    visitor.visit_ptr(ptr.as_ptr() as usize, next.tag());
                }
            }
        }
        
        fn size(&self) -> usize {
            std::mem::size_of::<TestObject>()
        }
    }
    
    #[test]
    fn test_garbage_collection() {
        let mut gc = GarbageCollector::new(1024 * 1024);
        
        // Allocate some objects
        let obj1 = gc.allocate::<TestObject>(TestObject { value: 1, next: None }, Tag::Integer)
            .unwrap();
        
        if let Some(obj1_mut) = obj1.get_mut() {
            obj1_mut.value = 1;
            obj1_mut.next = None;
        }
        
        let obj2 = gc.allocate::<TestObject>(TestObject { value: 2, next: Some(obj1.clone()) }, Tag::Integer)
            .unwrap();
        
        let obj3 = gc.allocate::<TestObject>(TestObject { value: 3, next: Some(obj2.clone()) }, Tag::Integer)
            .unwrap();
        
        // Make obj3 a root
        if let Some(ptr) = obj3.ptr() {
            gc.roots.insert(ptr.as_ptr() as usize);
        }
        
        // Collect garbage
        gc.collect();
        
        // All objects should still be alive because obj3 references obj2 and obj2 references obj1
        assert_eq!(gc.objects.len(), 3);
        
        // Drop obj3 as a root and break the chain
        if let Some(ptr) = obj3.ptr() {
            gc.roots.remove(&(ptr.as_ptr() as usize));
        }
        
        if let Some(obj3_mut) = obj3.get_mut() {
            obj3_mut.next = None;
        }
        
        // Collect garbage again
        gc.collect();
        
        // Now obj1 and obj2 should be collected
        assert_eq!(gc.objects.len(), 1);
        
        let stats = gc.stats();
        assert_eq!(stats.collections, 2);
        assert_eq!(stats.collected_objects, 2);
        assert_eq!(stats.live_objects, 1);
    }
}

/// Object managed by the garbage collector
pub struct GcObject {
    /// Pointer to the object
    ptr: NonNull<u8>,
    
    /// Layout of the allocated memory
    layout: Layout,
    
    /// Type tag for the object
    tag: Tag,
    
    /// Whether the object is marked during GC
    marked: bool,
    
    /// Function to trace references in the object
    trace_fn: TraceFunc,
    
    /// Function to get the size of the object
    size_fn: SizeFunc,
}

/// A reference to a garbage-collected object
pub struct Gc<T: 'static> {
    /// The garbage collector that manages this object
    collector: Rc<RefCell<GarbageCollector>>,
    
    /// Pointer to the object
    ptr: NonNull<T>,
    
    /// The tag of the object
    tag: Tag,
    
    /// Phantom data to track the type
    _phantom: PhantomData<T>,
} 