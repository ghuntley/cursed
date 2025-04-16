//! Mark and Sweep Garbage Collection Algorithm
//!
//! This module implements the mark and sweep garbage collection algorithm
//! with three distinct phases: marking roots, tracing objects, and sweeping unreachable objects.

use std::collections::{HashMap, HashSet};
use std::ptr::NonNull;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::memory::{Traceable, Visitor, ThreadSafeVisitor, Tag, ThreadSafeGc};
use crate::debug_println;

/// Statistics about a garbage collection cycle
#[derive(Debug, Default, Clone)]
pub struct CollectionStats {
    /// Number of objects marked
    pub marked: usize,
    /// Number of objects swept
    pub swept: usize,
    /// Time spent in marking phase (ms)
    pub mark_time_ms: u64,
    /// Time spent in sweeping phase (ms)
    pub sweep_time_ms: u64,
    /// Total collection time (ms)
    pub total_time_ms: u64,
}

/// The phase of a garbage collection cycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionPhase {
    /// Marking phase - mark all reachable objects
    Mark,
    /// Sweeping phase - collect unreachable objects
    Sweep,
    /// Finished collection
    Finished,
}

/// Result of a garbage collection cycle
#[derive(Debug)]
pub enum CollectionResult {
    /// Collection was successful
    Success(CollectionStats),
    /// Collection timed out
    Timeout {
        /// Collection statistics so far
        stats: CollectionStats,
        /// The phase that timed out
        phase: CollectionPhase,
    },
    /// An error occurred during collection
    Error(String),
}

/// Perform a mark-and-sweep garbage collection cycle
pub fn mark_and_sweep(
    objects: &mut HashMap<usize, NonNull<dyn Traceable>>,
    root_set: &HashSet<usize>,
    timeout: Option<Duration>,
) -> CollectionResult {
    let start_time = Instant::now();
    let mut stats = CollectionStats::default();
    
    let mut marked = HashSet::new();
    let mut gray_objects = Vec::new();
    
    // Phase 1: Mark root objects
    debug_println!("Mark-sweep GC started with {} roots, {} objects total", 
                  root_set.len(), objects.len());
                  
    // Log all objects for debugging
    debug_println!("Objects in map:");
    for (&addr, _) in objects.iter() {
        debug_println!("  Object at 0x{:x}", addr);
    }
    
    // Log root set
    debug_println!("Root set:");
    for &root_id in root_set.iter() {
        debug_println!("  Root at 0x{:x}", root_id);
    }
    
    // Mark root objects
    for &root_id in root_set.iter() {
        if let Some(ptr) = objects.get(&root_id) {
            debug_println!("Marking root object at 0x{:x}", root_id);
            marked.insert(root_id);
            gray_objects.push(*ptr);
            stats.marked += 1;
        } else {
            debug_println!("WARNING: Root 0x{:x} not found in objects map", root_id);
        }
    }
    
    // Phase 2: Trace references
    // We need to handle the gray list manually to avoid borrow issues
    while !gray_objects.is_empty() {
        // Check for timeout
        if let Some(timeout) = timeout {
            if start_time.elapsed() > timeout {
                stats.mark_time_ms = start_time.elapsed().as_millis() as u64;
                debug_println!("Mark-sweep GC timed out during marking after {}ms", stats.mark_time_ms);
                return CollectionResult::Timeout {
                    stats,
                    phase: CollectionPhase::Mark,
                };
            }
        }
        
        // Get the next object to process
        let obj_ptr = gray_objects.pop().unwrap();
        let obj_addr = obj_ptr.as_ptr() as *const () as usize;
        
        // Create a visitor to trace the object's references
        let mut visitor = MarkVisitor::new(&mut gray_objects, &mut marked, objects);
        
        // Safety: We know the pointer is valid because it was in our objects map
        unsafe {
            let obj_ptr_mut = obj_ptr.as_ptr() as *mut dyn Traceable;
            let obj = &mut *obj_ptr_mut;
            
            // Trace the object's references
            debug_println!("Tracing object at address 0x{:x}", obj_addr);
            obj.trace(&mut visitor);
            
            // Stats
            stats.marked += visitor.marked_this_visit;
        }
    }
    
    let mark_time = start_time.elapsed();
    stats.mark_time_ms = mark_time.as_millis() as u64;
    debug_println!("Marking phase completed in {}ms, {} objects marked", stats.mark_time_ms, stats.marked);
    
    // Phase 3: Sweep unreachable objects
    let sweep_start = Instant::now();
    let mut to_remove = Vec::new();
    
    for (&addr, obj_ptr) in objects.iter() {
        // Check for timeout
        if let Some(timeout) = timeout {
            if start_time.elapsed() > timeout {
                stats.sweep_time_ms = sweep_start.elapsed().as_millis() as u64;
                stats.total_time_ms = start_time.elapsed().as_millis() as u64;
                debug_println!("Mark-sweep GC timed out during sweeping after {}ms", stats.total_time_ms);
                return CollectionResult::Timeout {
                    stats,
                    phase: CollectionPhase::Sweep,
                };
            }
        }
        
        if !marked.contains(&addr) {
            debug_println!("Object at 0x{:x} was not marked - adding to sweep list", addr);
            to_remove.push(addr);
            stats.swept += 1;
            
            // We need to finalize the object before removing it
            unsafe {
                // Get a mutable reference to finalize the object
                let obj_ptr_mut = obj_ptr.as_ptr() as *mut dyn Traceable;
                let obj = &mut *obj_ptr_mut;
                obj.finalize();
                debug_println!("Finalized object at 0x{:x}", addr);
            }
        } else {
            debug_println!("Object at 0x{:x} was marked - keeping", addr);
        }
    }
    
    // Remove swept objects
    debug_println!("About to remove {} unreachable objects", to_remove.len());
    for addr in to_remove {
        debug_println!("Removing unreachable object at 0x{:x}", addr);
        let removed = objects.remove(&addr);
        if removed.is_none() {
            debug_println!("ERROR: Object at 0x{:x} not found in objects map during sweeping", addr);
        }
    }
    debug_println!("After sweeping, {} objects remain in the map", objects.len());
    
    let sweep_time = sweep_start.elapsed();
    stats.sweep_time_ms = sweep_time.as_millis() as u64;
    stats.total_time_ms = start_time.elapsed().as_millis() as u64;
    
    debug_println!(
        "Sweeping phase completed in {}ms, {} objects swept",
        stats.sweep_time_ms, stats.swept
    );
    debug_println!(
        "Total GC cycle completed in {}ms, {} objects remain",
        stats.total_time_ms, objects.len()
    );
    
    CollectionResult::Success(stats)
}

/// Visitor implementation for marking objects during garbage collection
pub struct MarkVisitor<'a> {
    /// Objects that need to be processed next
    gray_list: &'a mut Vec<NonNull<dyn Traceable>>,
    /// Set of marked object addresses
    marked: &'a mut HashSet<usize>,
    /// Map of all objects
    objects: &'a HashMap<usize, NonNull<dyn Traceable>>,
    /// Number of objects marked in this visit
    pub marked_this_visit: usize,
}

impl<'a> MarkVisitor<'a> {
    /// Create a new visitor for marking objects
    pub fn new(
        gray_list: &'a mut Vec<NonNull<dyn Traceable>>,
        marked: &'a mut HashSet<usize>,
        objects: &'a HashMap<usize, NonNull<dyn Traceable>>,
    ) -> Self {
        Self {
            gray_list,
            marked,
            objects,
            marked_this_visit: 0,
        }
    }
}

impl<'a> Visitor for MarkVisitor<'a> {
    fn visit(&mut self, ptr: NonNull<dyn Traceable>) {
        let addr = ptr.as_ptr() as *const () as usize;
        
        // If already marked, skip
        if self.marked.contains(&addr) {
            return;
        }
        
        // Mark the object
        self.marked.insert(addr);
        self.marked_this_visit += 1;
        debug_println!("Marked object at address 0x{:x}", addr);
        
        // Add to gray list for further processing
        // Only if it's one of our managed objects
        if self.objects.contains_key(&addr) {
            self.gray_list.push(ptr);
        }
    }
}

/// A dummy traceable object used for testing
#[derive(Debug)]
pub(super) struct DummyTraceable {
    pub id: usize,
}

impl DummyTraceable {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

impl Traceable for DummyTraceable {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // No implementation needed for testing
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
    
    fn finalize(&mut self) {
        // No implementation needed for testing
    }
}

/// Result of an incremental garbage collection step
pub enum IncrementalResult {
    /// Collection is in progress, more work to do
    Progress {
        /// Collection statistics so far
        stats: CollectionStats,
        /// Number of objects still to process
        remaining: usize,
    },
    /// Collection is complete
    Complete(CollectionStats),
}

/// Perform an incremental mark-and-sweep step
pub fn incremental_mark_and_sweep(
    objects: &mut HashMap<usize, NonNull<dyn Traceable>>,
    root_set: &HashSet<usize>,
    max_objects: usize,
) -> Result<IncrementalResult, String> {
    let start_time = Instant::now();
    let mut stats = CollectionStats::default();
    
    let mut marked = HashSet::new();
    let mut gray_objects = Vec::new();
    
    // Mark root objects up to max_objects
    let mut processed = 0;
    for &root_id in root_set.iter() {
        if processed >= max_objects {
            // More work to do
            stats.mark_time_ms = start_time.elapsed().as_millis() as u64;
            return Ok(IncrementalResult::Progress {
                stats,
                remaining: root_set.len() - processed,
            });
        }
        
        if let Some(ptr) = objects.get(&root_id) {
            marked.insert(root_id);
            gray_objects.push(*ptr);
            stats.marked += 1;
            processed += 1;
        }
    }
    
    // Process gray objects up to max_objects
    while !gray_objects.is_empty() {
        if processed >= max_objects {
            // More work to do
            stats.mark_time_ms = start_time.elapsed().as_millis() as u64;
            return Ok(IncrementalResult::Progress {
                stats,
                remaining: gray_objects.len(),
            });
        }
        
        // Process next gray object
        let obj_ptr = gray_objects.pop().unwrap();
        let obj_addr = obj_ptr.as_ptr() as *const () as usize;
        
        // Create a visitor to trace the object's references
        let mut visitor = MarkVisitor::new(&mut gray_objects, &mut marked, objects);
        
        // Safety: We know the pointer is valid because it was in our objects map
        unsafe {
            let obj_ptr_mut = obj_ptr.as_ptr() as *mut dyn Traceable;
            let obj = &mut *obj_ptr_mut;
            
            // Trace the object's references
            debug_println!("Incremental GC: Tracing object at address 0x{:x}", obj_addr);
            obj.trace(&mut visitor);
            
            // Stats
            stats.marked += visitor.marked_this_visit;
        }
        
        processed += 1;
    }
    
    // Complete the collection if we're done
    stats.total_time_ms = start_time.elapsed().as_millis() as u64;
    Ok(IncrementalResult::Complete(stats))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    
    /// A dummy traceable object for testing
    struct DummyTraceable {
        id: usize,
    }
    
    impl DummyTraceable {
        fn new(id: usize) -> Self {
            Self { id }
        }
    }
    
    impl Traceable for DummyTraceable {
        fn trace(&self, _visitor: &mut dyn Visitor) {
            // No implementation needed for tests
        }
        
        fn size(&self) -> usize {
            std::mem::size_of::<Self>()
        }
        
        fn tag(&self) -> Tag {
            Tag::Object
        }
        
        fn finalize(&mut self) {
            // No implementation needed for tests
        }
    }
    
    // Test object that links to other objects
    struct TestObject {
        id: usize,
        refs: RefCell<Vec<usize>>,
        finalized: RefCell<bool>,
    }
    
    impl TestObject {
        fn new(id: usize) -> Self {
            Self {
                id,
                refs: RefCell::new(Vec::new()),
                finalized: RefCell::new(false),
            }
        }
        
        fn add_ref(&self, id: usize) {
            self.refs.borrow_mut().push(id);
        }
    }
    
    impl Traceable for TestObject {
        fn trace(&self, visitor: &mut dyn Visitor) {
            // Trace the references to other objects
            for &id in self.refs.borrow().iter() {
                // For the test, we can't actually trace the objects directly
                // Instead, simulate visiting them by using NonNull::new_unchecked
                // This is fine for tests since we're controlling the addresses
                unsafe {
                    // Create a fake pointer - in real code this would be a real pointer
                    // obtained from the object state
                    // We can't directly cast id to *mut dyn Traceable as it's a fat pointer
                    // For the test, we'll just use a temporary object pointer that satisfies the trait
                    let obj_ptr = Box::into_raw(Box::new(DummyTraceable::new(id)));
                    let ptr = NonNull::new_unchecked(obj_ptr as *mut dyn Traceable);
                    visitor.visit(ptr);
                }
            }
        }
        
        fn size(&self) -> usize {
            std::mem::size_of::<Self>()
        }
        
        fn tag(&self) -> Tag {
            Tag::Object
        }
        
        fn finalize(&mut self) {
            *self.finalized.borrow_mut() = true;
        }
    }
    
    #[test]
    fn test_mark_sweep_basic() {
        let mut objects = HashMap::new();
        let mut root_set = HashSet::new();
        
        // Create some objects
        let obj1 = Box::new(TestObject::new(1));
        let obj2 = Box::new(TestObject::new(2));
        let obj3 = Box::new(TestObject::new(3));
        
        // Link them
        obj1.add_ref(2);
        obj2.add_ref(3);
        
        // Convert to raw pointers and add to objects map
        let obj1_ptr = unsafe { NonNull::new_unchecked(Box::into_raw(obj1) as *mut dyn Traceable) };
        let obj2_ptr = unsafe { NonNull::new_unchecked(Box::into_raw(obj2) as *mut dyn Traceable) };
        let obj3_ptr = unsafe { NonNull::new_unchecked(Box::into_raw(obj3) as *mut dyn Traceable) };
        
        objects.insert(1, obj1_ptr);
        objects.insert(2, obj2_ptr);
        objects.insert(3, obj3_ptr);
        
        // Add obj1 to root set
        root_set.insert(1);
        
        // Run GC
        let result = mark_and_sweep(&mut objects, &root_set, None);
        
        // Check result
        if let CollectionResult::Success(stats) = result {
            // All objects should be reachable and marked
            assert!(stats.marked >= 1, "At least the root object should be marked");
            assert_eq!(stats.swept, 0, "No objects should be swept since they're all reachable");
            assert_eq!(objects.len(), 3, "All three objects should remain in the map");
        } else {
            panic!("GC failed");
        }
        
        // Clean up any remaining objects
        for (_, ptr) in objects.drain() {
            unsafe {
                let _boxed = Box::from_raw(ptr.as_ptr() as *mut TestObject);
                // Box is dropped here
            }
        }
    }
}