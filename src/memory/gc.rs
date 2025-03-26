use std::alloc::Layout;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::rc::Rc;
use std::any::{TypeId};
use std::mem;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::iter::Iterator;
use std::slice::Iter;
use std::collections::HashSet;
use crate::prelude::{VecExt, RawPtrExt, TaggedPtrExt};
use std::convert::TryFrom;
use std::collections::HashMap;
use crate::error::Error;
use crate::memory::allocator::AllocatorBase;
use crate::memory::tagged::TaggedPtrExtMut;
use num_traits::Saturating;

use super::allocator::Allocator;
use super::block::{BlockAllocator, BlockAllocatorExt};
use super::tagged::{Tag, TaggedPtr, TAG_MASK, TAG_SHIFT, PTR_MASK, NonNullExt, TaggedPtrConstructor};
use super::MemoryError;
use super::{align_up, MIN_ALIGNMENT, DEFAULT_BLOCK_SIZE};
use crate::memory::trace::{Traceable, Visitor};
use crate::memory::stats::GcStats;

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
    /// Number of garbage collection cycles
    pub collections: usize,
    /// Number of live objects
    pub live_objects: usize,
    /// Number of objects freed
    pub freed_objects: usize,
    /// Total memory allocated
    pub total_allocated: usize,
    /// Total memory freed
    pub total_freed: usize,
    /// Current heap size
    pub current_heap_size: usize,
    /// Maximum heap size
    pub max_heap_size: usize,
}

impl fmt::Display for GcStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GC Statistics:")?;
        writeln!(f, "  Collections performed: {}", self.collections)?;
        writeln!(f, "  Live objects: {}", self.live_objects)?;
        writeln!(f, "  Objects freed: {}", self.freed_objects)?;
        writeln!(f, "  Bytes allocated: {}", self.total_allocated)?;
        writeln!(f, "  Bytes freed: {}", self.total_freed)?;
        
        let collection_rate = if self.collections > 0 && self.freed_objects > 0 {
            self.freed_objects as f64 / self.collections as f64
        } else {
            0.0
        };
        
        write!(f, "  Avg. objects collected per cycle: {:.2}", collection_rate)
    }
}

/// Helper struct for collecting objects during garbage collection
#[derive(Debug)]
struct ObjectCollector<'a> {
    /// Reference to the garbage collector
    gc: &'a mut GarbageCollector,
    /// Objects pending processing 
    pending: Vec<usize>,
}

impl<'a> Visitor for ObjectCollector<'a> {
    fn visit(&mut self, _obj: &dyn std::any::Any) {
        // Not used in this implementation
    }
    
    fn visit_ptr(&mut self, ptr: usize, _tag: Tag) {
        // Find the index of the object with this address
        if let Some(&index) = self.gc.allocated_objects.get(&ptr) {
            // Add to pending queue for later processing
            self.pending.push(index);
        }
    }
}

/// An object managed by the garbage collector
#[derive(Debug, Clone)]
pub struct GcObject {
    /// Pointer to the object in memory
    pub ptr: NonNull<u8>,
    
    /// Memory layout of the object
    pub layout: Layout,
    
    /// Tag for the object type
    pub tag: Tag,
    
    /// Whether the object is marked during collection
    pub marked: bool,
    
    /// Function to trace this object's references
    pub trace_fn: TraceFunc,
    
    /// Function to calculate this object's size
    pub size_fn: SizeFunc,
}

/// A garbage collector that manages object lifetimes.
#[derive(Debug)]
pub struct GarbageCollector {
    /// Objects managed by the garbage collector
    pub objects: Vec<TaggedPtr<dyn Traceable>>,
    /// The block allocator used for memory allocation
    pub block_allocator: BlockAllocator,
    /// Set of root object indices
    pub roots: HashSet<usize>,
    /// Set of marked objects during collection
    pub marked: HashSet<usize>,
    /// Garbage collector statistics
    pub stats: GcStats,
    /// Allocation threshold counter
    pub allocation_threshold: usize,
    /// Collection counter
    pub collect_counter: usize,
    /// Map of allocated objects by address
    pub allocated_objects: HashMap<usize, TaggedPtr<dyn Traceable>>,
    /// Current heap usage
    pub heap_used: usize,
    /// Trace functions for different tags
    pub trace_fns: HashMap<Tag, fn(*mut dyn Traceable, &mut ObjectIndexCollector)>,
}

/// Garbage collector statistics
#[derive(Debug, Clone)]
pub struct GcStats {
    /// Number of garbage collection cycles
    pub collections: usize,
    /// Number of live objects
    pub live_objects: usize,
    /// Number of objects freed
    pub freed_objects: usize,
    /// Total memory allocated
    pub total_allocated: usize,
    /// Total memory freed
    pub total_freed: usize,
    /// Current heap size
    pub current_heap_size: usize,
    /// Maximum heap size
    pub max_heap_size: usize,
}

impl Default for GcStats {
    fn default() -> Self {
        Self {
            collections: 0,
            total_allocated: 0,
            total_freed: 0,
            current_heap_size: 0,
            max_heap_size: 0,
            live_objects: 0,
            freed_objects: 0,
        }
    }
}

impl Clone for GcStats {
    fn clone(&self) -> Self {
        Self {
            collections: self.collections,
            total_allocated: self.total_allocated,
            total_freed: self.total_freed,
            current_heap_size: self.current_heap_size,
            max_heap_size: self.max_heap_size,
            live_objects: self.live_objects,
            freed_objects: self.freed_objects,
        }
    }
}

// Default collection threshold: collect after 1000 allocations
const ALLOCATION_THRESHOLD: usize = 1000;
// Minimum memory pressure to trigger collection (60% by default)
const MIN_MEMORY_PRESSURE: f64 = 0.6;
// Maximum memory pressure before forced collection (85% by default)
const MAX_MEMORY_PRESSURE: f64 = 0.85;

// Define the page size constant for memory alignment
const PAGE_SIZE: usize = 4096;

/// Extension trait for GarbageCollector references
pub trait GarbageCollectorExt {
    /// Run a garbage collection cycle
    fn run_collection(&mut self);
    
    /// Mark all objects reachable from roots
    fn mark(&mut self);
    
    /// Mark a specific object
    fn mark_object(&mut self, obj_index: usize);
    
    /// Mark all objects traced from roots
    fn mark_traced_objects(&mut self);
    
    /// Sweep unreachable objects
    fn sweep(&mut self);
    
    /// Adjust the collection threshold based on current usage
    fn adjust_collection_threshold(&mut self);
    
    /// Check if the garbage collector should run a collection
    fn should_collect(&self) -> bool;
    
    /// Get the memory pressure (ratio of used to capacity)
    fn memory_pressure(&self) -> f64;
    
    /// Get memory usage statistics
    fn stats(&self) -> &GcStats;
    
    /// Get memory capacity
    fn memory_capacity(&self) -> usize;
    
    /// Get memory usage
    fn memory_usage(&self) -> usize;
}

impl GarbageCollectorExt for GarbageCollector {
    fn run_collection(&mut self) {
        // Mark phase: identify all reachable objects
        self.mark();
        
        // Sweep phase: remove unreachable objects
        self.sweep();
        
        // Adjust collection threshold based on current usage
        self.adjust_collection_threshold();
    }

    fn mark(&mut self) {
        // Clear the marked set before marking
        self.marked.clear();
        
        // Mark all root objects first 
        let root_indices: Vec<usize> = self.roots.iter().cloned().collect();
        
        // Process roots without mutable borrow issues
        for root_idx in root_indices {
            if let Some(obj) = self.objects.get(root_idx) {
                // Add to marked set
                self.marked.insert(root_idx);
                
                // Mark the object's references
                self.mark_object_references(root_idx, *obj);
            }
        }
    }

    fn mark_object(&mut self, obj_index: usize) {
        // If already marked, nothing to do
        if self.marked.contains(&obj_index) {
            return;
        }
        
        // Mark this object
        self.marked.insert(obj_index);
        
        // If the object has references to other objects, visit them
        if obj_index < self.objects.len() {
            let mut pending = Vec::new();
            pending.push(obj_index);
            
            struct ObjectIndexCollector<'a> {
                gc: &'a GarbageCollector,
                indices: &'a mut Vec<usize>,
            }
            
            impl<'a> Visitor for ObjectIndexCollector<'a> {
                fn visit(&mut self, _obj: &dyn std::any::Any) {
                    // Not used in this context
                }
                
                fn visit_ptr(&mut self, ptr: usize, _tag: Tag) {
                    // If this pointer is in our allocation map, add it to pending
                    if let Some(&obj_index) = self.gc.allocated_objects.get(&ptr) {
                        if !self.gc.marked.contains(&obj_index) {
                            self.indices.push(obj_index);
                        }
                    }
                }
            }
            
            // Process the pending queue
            while let Some(idx) = pending.pop() {
                // Skip if already processed or out of bounds
                if idx >= self.objects.len() {
                    continue;
                }
                
                // Mark the current object
                self.marked.insert(idx);
                
                // Collect objects referenced by this object
                let mut collector = ObjectIndexCollector {
                    gc: self,
                    indices: &mut pending,
                };
                
                // TODO: Call trace on the object to visit its references
                // This depends on how objects are implemented in your system
            }
        }
    }

    fn mark_traced_objects(&mut self) {
        // This could be expanded depending on how objects are represented
        // For now, it's just a placeholder for additional marking logic
    }

    fn sweep(&mut self) {
        let mut freed_objects = 0;
        let mut freed_memory = 0;
        
        // Clone the allocated_objects to avoid borrowing issues
        let allocated_objects = self.allocated_objects.clone();
        
        // Collect objects that aren't marked
        for (addr, obj_index) in allocated_objects {
            // If the object is not marked, free it
            if !self.marked.contains(&obj_index) {
                // Remove from the allocation table
                self.allocated_objects.remove(&addr);
                
                if let Some(obj) = self.objects.get(obj_index) {
                    // Calculate the memory freed
                    let obj_size = self.get_object_size(*obj);
                    freed_memory += obj_size;
                    
                    // Decrement heap used
                    self.heap_used = self.heap_used.saturating_sub(obj_size);
                    
                    // Deallocate the memory if it's a complex object
                    self.deallocate_object(*obj);
                }
                
                freed_objects += 1;
            }
        }
        
        // Update GC statistics
        self.stats.freed_objects += freed_objects;
        self.stats.total_freed += freed_memory;
        self.stats.collections += 1;
    }

    fn adjust_collection_threshold(&mut self) {
        // Adjust the allocation threshold based on memory pressure
        let pressure = self.memory_pressure();
        
        if pressure > 0.9 {
            // If memory pressure is high, trigger collection more frequently
            self.allocation_threshold = (self.allocation_threshold as f64 * 0.8) as usize;
        } else if pressure < 0.5 {
            // If memory pressure is low, collect less frequently
            self.allocation_threshold = (self.allocation_threshold as f64 * 1.2) as usize;
        }
        
        // Ensure a minimum threshold
        if self.allocation_threshold < 10 {
            self.allocation_threshold = 10;
        }
        
        // Reset the collection counter
        self.collect_counter = 0;
    }

    fn should_collect(&self) -> bool {
        // Check if we've crossed the collection threshold
        self.heap_used >= self.allocation_threshold
    }

    fn memory_pressure(&self) -> f64 {
        let used = AllocatorBase::memory_usage(&self.block_allocator);
        let capacity = AllocatorBase::memory_capacity(&self.block_allocator);
        
        if capacity == 0 {
            return 0.0;
        }
        
        used as f64 / capacity as f64
    }

    fn stats(&self) -> &GcStats {
        &self.stats
    }

    fn memory_capacity(&self) -> usize {
        BlockAllocatorExt::memory_capacity(&self.block_allocator)
    }

    fn memory_usage(&self) -> usize {
        BlockAllocatorExt::memory_usage(&self.block_allocator)
    }

    /// Get the size of an object
    fn get_object_size(&self, ptr: TaggedPtr<dyn Traceable>) -> usize {
        ptr.size_of()
    }
    
    /// Deallocate an object
    fn deallocate_object(&mut self, ptr: TaggedPtr<dyn Traceable>) {
        // Skip deallocating immediate values
        if ptr.is_immediate() {
            return;
        }
        
        // Based on tag, properly deallocate heap memory
        if let Some(raw_ptr) = ptr.as_non_null() {
            unsafe {
                match ptr.tag() {
                    Tag::String => {
                        // Drop the string
                        std::ptr::drop_in_place(raw_ptr.as_ptr() as *mut String);
                        
                        // Deallocate the memory
                        let layout = Layout::new::<String>();
                        std::alloc::dealloc(raw_ptr.as_ptr(), layout);
                    },
                    Tag::Array => {
                        // Drop the array
                        std::ptr::drop_in_place(raw_ptr.as_ptr() as *mut Vec<TaggedPtr<dyn Traceable>>);
                        
                        // Deallocate the memory
                        let layout = Layout::new::<Vec<TaggedPtr<dyn Traceable>>>();
                        std::alloc::dealloc(raw_ptr.as_ptr(), layout);
                    },
                    Tag::HashMap => {
                        // Drop the hashmap
                        std::ptr::drop_in_place(raw_ptr.as_ptr() as *mut std::collections::HashMap<TaggedPtr<dyn Traceable>, TaggedPtr<dyn Traceable>>);
                        
                        // Deallocate the memory
                        let layout = Layout::new::<std::collections::HashMap<TaggedPtr<dyn Traceable>, TaggedPtr<dyn Traceable>>>();
                        std::alloc::dealloc(raw_ptr.as_ptr(), layout);
                    },
                    Tag::Function => {
                        // Drop the function
                        std::ptr::drop_in_place(raw_ptr.as_ptr() as *mut Function);
                        
                        // Deallocate the memory
                        let layout = Layout::new::<Function>();
                        std::alloc::dealloc(raw_ptr.as_ptr(), layout);
                    },
                    _ => {
                        // For unknown types, try to use drop_in_place if we know the size
                        let size = self.get_object_size(ptr);
                        if size > 0 {
                            let layout = Layout::from_size_align(size, std::mem::align_of::<usize>())
                                .unwrap_or(Layout::new::<usize>());
                            std::alloc::dealloc(raw_ptr.as_ptr(), layout);
                        }
                    }
                }
            }
        }
    }
}

impl GarbageCollector {
    /// Create a new garbage collector
    pub fn new() -> Result<Self, Error> {
        Self::with_heap_size(1024 * 1024) // Default to 1MB heap
    }
    
    /// Create a new garbage collector with a specific heap size
    pub fn with_heap_size(heap_size: usize) -> Result<Self, Error> {
        Ok(Self {
            objects: Vec::new(),
            block_allocator: BlockAllocator::new(heap_size).map_err(|e| Error::Memory(format!("Failed to create block allocator: {:?}", e)))?,
            roots: HashSet::new(),
            marked: HashSet::new(),
            stats: GcStats::default(),
            allocation_threshold: heap_size / 2,
            collect_counter: 0,
            allocated_objects: HashMap::new(),
            heap_used: 0,
            trace_fns: HashMap::new(),
        })
    }

    /// Get the size of an object
    pub fn get_object_size(&self, ptr: TaggedPtr<dyn Traceable>) -> usize {
        ptr.size()
    }

    /// Get the trace function for a tag
    pub fn get_trace_fn(&self, obj: TaggedPtr<dyn Traceable>) -> Option<fn(*mut dyn Traceable, &mut dyn Visitor)> {
        obj.as_ref().map(|obj| obj.trace as fn(*mut dyn Traceable, &mut dyn Visitor))
    }

    /// Mark object references
    pub fn mark_object_references(&mut self, obj_index: usize, obj: TaggedPtr<dyn Traceable>) {
        if let Some(trace_fn) = self.get_trace_fn(obj) {
            let mut collector = ObjectIndexCollector {
                gc: self,
                current_index: obj_index,
            };
            trace_fn(obj.as_raw_ptr(), &mut collector);
        }
    }

    /// Get total memory in use
    pub fn get_total_in_use(&self) -> usize {
        self.heap_used
    }

    /// Get total memory managed
    pub fn get_total_managed(&self) -> usize {
        self.block_allocator.capacity()
    }

    /// Get garbage collector statistics
    pub fn stats(&self) -> &GcStats {
        &self.stats
    }

    pub fn deallocate(&mut self, ptr: TaggedPtr<dyn Traceable>) {
        if let Some(raw_ptr) = ptr.as_non_null() {
            let size = ptr.size();
            unsafe {
                self.block_allocator.deallocate(raw_ptr, Layout::from_size_align(size, 8).unwrap());
            }
            self.heap_used = self.heap_used.saturating_sub(size);
        }
    }

    pub fn mark(&mut self, obj: TaggedPtr<dyn Traceable>) {
        if let Some(trace_fn) = self.get_trace_fn(obj) {
            let mut collector = ObjectIndexCollector {
                marked: HashSet::new(),
            };
            trace_fn(obj.as_raw_ptr(), &mut collector);
        }
    }
}

/// A visitor that marks objects during garbage collection
struct ObjectVisitor<'a> {
    gc: &'a mut GarbageCollector,
}

impl<'a> ObjectVisitor<'a> {
    pub fn new(gc: &'a mut GarbageCollector) -> Self {
        Self { gc }
    }
}

impl<'a> Visitor for ObjectVisitor<'a> {
    fn visit_ptr(&mut self, ptr: usize, _tag: Tag) {
        // If this pointer is in our allocation map, mark it
        if let Some(&obj_index) = self.gc.allocated_objects.get(&ptr) {
            self.gc.mark_object(obj_index);
        }
    }
    
    fn visit(&mut self, _obj: &dyn std::any::Any) {
        // Default implementation that does nothing
    }
}

impl Clone for GarbageCollector {
    fn clone(&self) -> Self {
        Self {
            objects: self.objects.clone(),
            block_allocator: self.block_allocator.clone(),
            roots: self.roots.clone(),
            marked: self.marked.clone(),
            stats: self.stats.clone(),
            allocation_threshold: self.allocation_threshold,
            collect_counter: self.collect_counter,
            allocated_objects: self.allocated_objects.clone(),
            heap_used: self.heap_used,
            trace_fns: self.trace_fns.clone(),
        }
    }
}

impl AllocatorBase for GarbageCollector {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, Error> {
        self.block_allocator.allocate(layout)
    }
    
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.block_allocator.deallocate(ptr, layout);
    }
    
    fn reset(&mut self) {
        self.objects.clear();
        self.marked.clear();
        self.stats = GcStats {
            collections: 0,
            live_objects: 0,
            freed_objects: 0,
            total_allocated: 0,
            total_freed: 0,
            current_heap_size: 0,
            max_heap_size: 0,
        };
        
        self.block_allocator.reset();
    }
    
    fn memory_capacity(&self) -> usize {
        BlockAllocatorExt::memory_capacity(&self.block_allocator)
    }
    
    fn memory_usage(&self) -> usize {
        BlockAllocatorExt::memory_usage(&self.block_allocator)
    }
}

impl Allocator for GarbageCollector {}

impl<T: 'static + Traceable> Trace for Gc<T> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        if let Some(ptr) = self.ptr.as_ref() {
            visitor.visit_ptr(ptr as *const T as usize, self.tag);
        }
    }
}

impl<T: 'static> Gc<T> {
    /// Create a new Gc-managed object
    pub fn new(value: T, collector: Rc<RefCell<GarbageCollector>>, tag: Tag) -> Self {
        Self {
            collector,
            ptr: TaggedPtr::new(Some(NonNull::new(Box::into_raw(Box::new(value)) as *mut T).unwrap()), tag),
            tag,
            _phantom: PhantomData,
        }
    }
    
    /// Get a reference to the underlying value
    pub fn get(&self) -> Option<&T> {
        self.ptr.as_ref()
    }
    
    /// Get a mutable reference to the underlying value
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.ptr.as_mut()
    }
    
    /// Get a reference to the underlying value
    pub fn as_ref(&self) -> &T {
        self.get().unwrap()
    }
    
    /// Get a mutable reference to the underlying value
    pub fn as_mut(&mut self) -> &mut T {
        self.get_mut().unwrap()
    }
    
    /// Run the garbage collector
    pub fn collect(&self) {
        self.collector.borrow_mut().run_collection();
    }
    
    /// Get the tag for this object
    pub fn tag(&self) -> Tag {
        self.tag
    }
    
    /// Get the raw pointer for this object
    pub fn ptr(&self) -> Option<TaggedPtr> {
        let ptr = self.ptr;
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }

    /// Get the collector for this object
    pub fn collector(&self) -> &Rc<RefCell<GarbageCollector>> {
        &self.collector
    }

    /// Get the phantom data for this object
    pub fn phantom(&self) -> &PhantomData<T> {
        &self._phantom
    }
}

impl<T: 'static> Deref for Gc<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.ptr.as_ref().unwrap()
    }
}

impl<T: 'static> DerefMut for Gc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ptr.as_mut().unwrap()
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
    fn visit_tagged(&self, visitor: &mut dyn Visitor);
}

impl<T: Traceable> VisitAdapter for TaggedPtr<T> {
    fn visit_tagged(&self, visitor: &mut dyn Visitor) {
        if !self.is_null() {
            visitor.visit_ptr(self.as_usize(), self.tag());
        }
    }
}

/// A reference to a garbage-collected object
#[derive(Debug)]
pub struct Gc<T: 'static> {
    /// Reference to the garbage collector
    pub collector: Rc<RefCell<GarbageCollector>>,
    
    /// Pointer to the object
    pub ptr: TaggedPtr<T>,
    
    /// Type tag for the object
    pub tag: Tag,
    
    /// Phantom data to track the type parameter
    pub _phantom: PhantomData<T>,
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
        let mut gc = GarbageCollector::with_heap_size(1024 * 1024).unwrap();
        
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

// Implementation of TaggedPtrConstructor for GarbageCollector
impl TaggedPtrConstructor for GarbageCollector {
    type T = GarbageCollector;
    
    fn new(ptr: Option<NonNull<Self::T>>, _tag: Tag) -> TaggedPtr<Self::T> {
        TaggedPtr {
            ptr,
            tag: Tag::Null,
            _phantom: PhantomData,
        }
    }
}

// Define the Function type for GC to properly handle function objects
/// A function in the CURSED language
#[derive(Debug, Clone)]
pub struct Function {
    /// The name of the function, if any
    pub name: Option<String>,
    /// The bytecode instructions for the function
    pub instructions: Vec<u8>,
    /// The number of parameters the function takes
    pub parameters: usize,
    /// The locals used in the function
    pub locals: usize,
    /// The environment for closures (captured variables)
    pub env: Option<HashMap<String, TaggedPtr<dyn Traceable>>>,
}

impl Function {
    /// Create a new function
    pub fn new(
        name: Option<String>,
        instructions: Vec<u8>,
        parameters: usize,
        locals: usize,
    ) -> Self {
        Self {
            name,
            instructions,
            parameters,
            locals,
            env: None,
        }
    }
    
    /// Create a new function with an environment (closure)
    pub fn with_env(
        name: Option<String>,
        instructions: Vec<u8>,
        parameters: usize,
        locals: usize,
        env: HashMap<String, TaggedPtr<dyn Traceable>>,
    ) -> Self {
        Self {
            name,
            instructions,
            parameters,
            locals,
            env: Some(env),
        }
    }
    
    /// Get the name of the function
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    
    /// Get the bytecode instructions
    pub fn instructions(&self) -> &[u8] {
        &self.instructions
    }
    
    /// Get the number of parameters
    pub fn parameters(&self) -> usize {
        self.parameters
    }
    
    /// Get the number of locals
    pub fn locals(&self) -> usize {
        self.locals
    }
    
    /// Get the environment, if any
    pub fn env(&self) -> Option<&HashMap<String, TaggedPtr<dyn Traceable>>> {
        self.env.as_ref()
    }
}

/// Implement Traceable for Function
impl Traceable for Function {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Trace any captured variables in the environment
        if let Some(env) = &self.env {
            for (_, value) in env {
                visitor.visit_ptr(value.as_usize(), value.tag());
            }
        }
    }
    
    fn size(&self) -> usize {
        // Base size
        let mut size = std::mem::size_of::<Function>();
        
        // Add size of name if present
        if let Some(ref name) = self.name {
            size += name.len();
        }
        
        // Add size of instructions
        size += self.instructions.len();
        
        // Add size of environment if present
        if let Some(ref env) = self.env {
            size += env.len() * (std::mem::size_of::<String>() + std::mem::size_of::<TaggedPtr<dyn Traceable>>());
        }
        
        size
    }
}

// Fix TaggedPtr extension methods
impl<T: Traceable> TaggedPtrExt for TaggedPtr<T> {
    fn as_usize(&self) -> usize {
        self.as_ptr() as usize
    }
} 