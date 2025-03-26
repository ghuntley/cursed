// GC for CURSED VM
//
// This module contains the garbage collector for the CURSED VM. It tracks
// objects allocated by the VM and reclaims memory when it's no longer needed.

use std::alloc::Layout;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ptr::{self, null_mut, NonNull};
use std::rc::Rc;
use std::any::{TypeId};
use std::mem;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::iter::Iterator;
use std::slice::Iter;
use std::collections::{HashMap, HashSet};
use crate::prelude::VecExt;
use std::convert::TryFrom;
use crate::error::Error;
use crate::memory::allocator::AllocatorBase;
use crate::memory::tagged::{Tag, TaggedPtr, TAG_MASK, TAG_SHIFT, PTR_MASK, NonNullExt, TaggedPtrConstructor, TaggedDynPtr, TaggedPtrExt};
use num_traits::Saturating;
use crate::memory::tagged::TaggedPtrExtMut;

use super::allocator::Allocator;
use super::block::{BlockAllocator, BlockAllocatorExt};
use super::MemoryError;
use super::{align_up, MIN_ALIGNMENT, DEFAULT_BLOCK_SIZE};

/// Type alias for a function that traces an object's references
pub type TraceFunc = fn(*mut dyn Traceable, &mut dyn Visitor);

/// Type alias for a function that calculates an object's size
pub type SizeFunc = fn(*mut dyn Traceable) -> usize;

/// Trait for objects that can be traced by the garbage collector.
pub trait Traceable {
    /// Trace object references.
    fn trace(&self, visitor: &mut dyn Visitor);
    
    /// Get the size of this object.
    fn size(&self) -> usize;
}

/// Trait for objects that can be traced by the garbage collector.
pub trait Trace {
    /// Trace object references.
    fn trace(&self, visitor: &mut dyn Visitor);
}

/// Visitor for tracing object references.
pub trait Visitor {
    /// Visit an object
    fn visit(&mut self, obj: &dyn std::any::Any);
    
    /// Visit a pointer to another object.
    fn visit_ptr(&mut self, ptr: usize, tag: Tag);
}

/// Garbage collector statistics.
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

/// A structure to collect object indices for garbage collection
#[derive(Debug)]
pub struct ObjectIndexCollector {
    /// Set of visited object indices
    pub visited: HashSet<usize>,
    /// Reference to the garbage collector
    pub gc: *mut GarbageCollector,
}

impl Visitor for ObjectIndexCollector {
    fn visit(&mut self, _obj: &dyn std::any::Any) {
        // Default implementation that does nothing
    }
    
    fn visit_ptr(&mut self, ptr: usize, _tag: Tag) {
        if let Some(idx) = unsafe { (*self.gc).allocated_objects.get(&ptr) } {
            // Mark the object at this index
            unsafe {
                (*self.gc).mark_object(*idx);
            }
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
    pub objects: Vec<TaggedDynPtr>,
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
    /// Map of allocated objects address to their index in objects array
    pub allocated_objects: HashMap<usize, usize>,
    /// Current heap usage
    pub heap_used: usize,
    /// Trace functions for different tags
    pub trace_fns: HashMap<Tag, fn(*mut dyn Traceable, &mut ObjectIndexCollector)>,
}

// Default collection threshold: collect after 1000 allocations
const ALLOCATION_THRESHOLD: usize = 1000;
// Minimum memory pressure to trigger collection (60% by default)
const MIN_MEMORY_PRESSURE: f64 = 0.6;
// Maximum memory pressure before forced collection (85% by default)
const MAX_MEMORY_PRESSURE: f64 = 0.85;

// Define the page size constant for memory alignment
const PAGE_SIZE: usize = 4096;

/// Trait for GarbageCollector extensions
pub trait GarbageCollectorExt {
    /// Return GC stats
    fn stats(&self) -> GcStats;
    
    /// Mark an object
    fn mark_object(&mut self, obj_index: usize);
    
    /// Mark object references recursively
    fn mark_object_references(&mut self, obj_index: usize, obj: TaggedDynPtr);
    
    /// Get object size
    fn get_object_size(&self, ptr: TaggedDynPtr) -> usize;
    
    /// Deallocate object
    fn deallocate_object(&mut self, ptr: TaggedDynPtr);
    
    /// Run a garbage collection cycle
    fn run_collection(&mut self);
    
    /// Mark all objects reachable from roots
    fn mark(&mut self);
    
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
    
    /// Get memory capacity
    fn memory_capacity(&self) -> usize;
    
    /// Get memory usage
    fn memory_usage(&self) -> usize;
    
    /// Get the trace function for a given object
    fn get_trace_fn(&mut self, obj: TaggedDynPtr) -> Option<fn(*mut dyn Traceable, &mut ObjectIndexCollector)>;
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
            let obj = self.objects[obj_index];
            // Mark object references
            self.mark_object_references(obj_index, obj);
        }
    }

    fn mark_traced_objects(&mut self) {
        // This could be expanded depending on how objects are represented
        // For now, it's just a placeholder for additional marking logic
    }

    fn sweep(&mut self) {
        let mut freed_objects = 0;
        let mut freed_memory = 0;
        
        // Clone the allocated objects to avoid borrowing issues during iteration
        let allocated_objects = self.allocated_objects.clone();
        
        for (addr, obj_index) in allocated_objects {
            // If the object is not marked, free it
            if !self.marked.contains(&addr) {  // Changed from obj_index to addr
                // Remove from the allocation table
                self.allocated_objects.remove(&addr);
                
                // Find the object by address
                let obj = self.objects.iter().find(|obj| obj.as_usize() == addr).copied();
                
                if let Some(obj) = obj {
                    // Calculate the memory freed
                    let obj_size = self.get_object_size(obj);
                    freed_memory += obj_size;
                    
                    // Decrement heap used
                    self.heap_used = self.heap_used.saturating_sub(obj_size);
                    
                    // Deallocate the memory if it's a complex object
                    self.deallocate_object(obj);
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
        let capacity = GarbageCollectorExt::memory_capacity(self);
        if capacity == 0 {
            return 1.0;
        }
        
        GarbageCollectorExt::memory_usage(self) as f64 / capacity as f64
    }

    fn stats(&self) -> GcStats {
        self.stats.clone()
    }

    fn memory_capacity(&self) -> usize {
        BlockAllocatorExt::memory_capacity(&self.block_allocator)
    }

    fn memory_usage(&self) -> usize {
        self.heap_used
    }

    /// Get the size of an object
    fn get_object_size(&self, ptr: TaggedDynPtr) -> usize {
        ptr.size()
    }
    
    /// Deallocate an object
    fn deallocate_object(&mut self, ptr: TaggedDynPtr) {
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
                        std::alloc::dealloc(raw_ptr.as_ptr() as *mut u8, layout);
                    },
                    Tag::Array => {
                        // Drop the array
                        std::ptr::drop_in_place(raw_ptr.as_ptr() as *mut Vec<TaggedDynPtr>);
                        
                        // Deallocate the memory
                        let layout = Layout::new::<Vec<TaggedDynPtr>>();
                        std::alloc::dealloc(raw_ptr.as_ptr() as *mut u8, layout);
                    },
                    Tag::HashMap => {
                        // Drop the hashmap
                        std::ptr::drop_in_place(raw_ptr.as_ptr() as *mut std::collections::HashMap<TaggedDynPtr, TaggedDynPtr>);
                        
                        // Deallocate the memory
                        let layout = Layout::new::<std::collections::HashMap<TaggedDynPtr, TaggedDynPtr>>();
                        std::alloc::dealloc(raw_ptr.as_ptr() as *mut u8, layout);
                    },
                    Tag::Function => {
                        // Drop the function
                        std::ptr::drop_in_place(raw_ptr.as_ptr() as *mut Function);
                        
                        // Deallocate the memory
                        let layout = Layout::new::<Function>();
                        std::alloc::dealloc(raw_ptr.as_ptr() as *mut u8, layout);
                    },
                    _ => {
                        // For unknown types, try to use drop_in_place if we know the size
                        let size = self.get_object_size(ptr);
                        if size > 0 {
                            let layout = Layout::from_size_align(size, std::mem::align_of::<usize>())
                                .unwrap_or(Layout::new::<usize>());
                            std::alloc::dealloc(raw_ptr.as_ptr() as *mut u8, layout);
                        }
                    }
                }
            }
        }
    }

    fn mark_object_references(&mut self, obj_index: usize, obj: TaggedDynPtr) {
        // Create a collector to gather referenced objects
        let mut collector = ObjectIndexCollector {
            gc: self as *mut _,
            visited: HashSet::new(),
        };
        
        // Call the trace function if available
        if let Some(trace_fn) = self.get_trace_fn(obj) {
            // Safety: We've verified the object exists and has a trace function
            unsafe {
                if !obj.is_null() {
                    trace_fn(obj.as_raw_ptr() as *mut dyn Traceable, &mut collector);
                }
            }
        }
    }

    fn get_trace_fn(&mut self, obj: TaggedDynPtr) -> Option<fn(*mut dyn Traceable, &mut ObjectIndexCollector)> {
        self.trace_fns.get(&obj.tag()).copied()
    }
}

/// A companion object for GarbageCollector to provide static methods
pub struct GarbageCollectorCompanion;

impl GarbageCollectorCompanion {
    /// Create a new garbage collector with the specified heap size
    pub fn with_heap_size(heap_size: usize) -> Result<GarbageCollector, Box<dyn Error>> {
        let block_allocator = BlockAllocatorCompanion::new(heap_size)?;
        Ok(GarbageCollector {
            objects: Vec::new(),
            block_allocator,
            roots: HashSet::new(),
            marked: HashSet::new(),
            stats: GcStats::default(),
            trace_fns: HashMap::new(),
            size_fns: HashMap::new(),
        })
    }
    
    /// Create a new garbage collector with default heap size
    pub fn new() -> Result<GarbageCollector, Error> {
        Self::with_heap_size(1024 * 1024) // Default to 1MB heap
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
        if let Some(ptr) = if self.ptr.is_null() { None } else { unsafe { Some(&*self.ptr.as_ptr()) } } {
            visitor.visit_ptr(ptr as *const T as usize, self.tag);
        }
    }
}

impl<T: Traceable + 'static> Gc<T> {
    /// Create a new Gc-managed object
    pub fn new(value: T, collector: Rc<RefCell<GarbageCollector>>, tag: Tag) -> Self {
        let raw_ptr = Box::into_raw(Box::new(value));
        Self {
            collector,
            ptr: TaggedPtr::new(raw_ptr, tag),
            tag,
            _phantom: PhantomData,
        }
    }
    
    /// Get a reference to the underlying value
    pub fn get(&self) -> Option<&T> {
        if self.ptr.is_null() {
            None
        } else {
            unsafe { Some(&*self.ptr.as_ptr()) }
        }
    }
    
    /// Get a mutable reference to the underlying value
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.ptr.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.ptr.as_ptr()) }
        }
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
    pub fn ptr(&self) -> Option<TaggedPtr<T>> {
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
        if self.ptr.is_null() {
            panic!("Attempted to dereference null Gc pointer")
        } else {
            unsafe { &*self.ptr.as_ptr() }
        }
    }
}

impl<T: 'static> DerefMut for Gc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.ptr.is_null() {
            panic!("Attempted to mutably dereference null Gc pointer")
        } else {
            unsafe { &mut *self.ptr.as_ptr() }
        }
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

// Helper trait to adapt the new visitor to TaggedDynPtr
trait VisitAdapter {
    fn visit_tagged(&self, visitor: &mut dyn Visitor);
}

impl VisitAdapter for TaggedDynPtr {
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
    pub env: Option<HashMap<String, TaggedDynPtr>>,
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
        env: HashMap<String, TaggedDynPtr>,
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
    pub fn env(&self) -> Option<&HashMap<String, TaggedDynPtr>> {
        self.env.as_ref()
    }
}

/// Implement Traceable for Function
impl Traceable for Function {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Import TaggedPtrExt from prelude for this impl block
        use crate::prelude::TaggedPtrExt;
        
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
            size += env.len() * (std::mem::size_of::<String>() + std::mem::size_of::<TaggedDynPtr>());
        }
        
        size
    }
}

impl GarbageCollector {
    /// Get the size of an object
    pub fn get_object_size(&self, ptr: TaggedDynPtr) -> usize {
        ptr.size()
    }

    /// Get the trace function for the given object
    pub fn get_trace_fn(&mut self, obj: TaggedDynPtr) -> Option<fn(*mut dyn Traceable, &mut ObjectIndexCollector)> {
        self.trace_fns.get(&obj.tag()).copied()
    }

    /// Mark object references recursively
    pub fn mark_object_references(&mut self, obj_index: usize, obj: TaggedDynPtr) {
        // Create a collector to gather referenced objects
        let mut collector = ObjectIndexCollector {
            gc: self as *mut _,
            visited: HashSet::new(),
        };
        
        // Call the trace function if available
        if let Some(trace_fn) = self.get_trace_fn(obj) {
            // Safety: We've verified the object exists and has a trace function
            unsafe {
                if !obj.is_null() {
                    trace_fn(obj.as_raw_ptr() as *mut dyn Traceable, &mut collector);
                }
            }
        }
    }

    /// Get total memory in use
    pub fn get_total_in_use(&self) -> usize {
        self.heap_used
    }

    /// Get total memory managed
    pub fn get_total_managed(&self) -> usize {
        BlockAllocatorExt::capacity(&self.block_allocator)
    }

    /// Get garbage collector statistics
    pub fn stats(&self) -> &GcStats {
        &self.stats
    }

    pub fn deallocate(&mut self, ptr: TaggedDynPtr) {
        if let Some(raw_ptr) = ptr.as_non_null() {
            let size = self.get_object_size(ptr);
            unsafe {
                let u8_ptr = NonNull::new(raw_ptr.as_ptr() as *mut u8).unwrap();
                self.block_allocator.deallocate(u8_ptr, Layout::from_size_align(size, 8).unwrap());
            }
        }
    }

    pub fn mark(&mut self, obj: TaggedDynPtr) {
        if let Some(trace_fn) = self.get_trace_fn(obj) {
            let mut collector = ObjectIndexCollector {
                visited: HashSet::new(),
                gc: self,
            };
            trace_fn(obj.as_raw_ptr(), &mut collector);
        }
    }
} 