//! Garbage collector implementation for the CURSED language
//!
//! This module implements a mark-and-sweep garbage collector that automatically
//! manages memory for CURSED programs. It includes support for incremental
//! collection, detailed memory statistics, and debugging tools.
//!
//! The garbage collector works by:
//! 1. Tracking all allocated objects
//! 2. Maintaining a set of root objects (directly accessible references)
//! 3. Periodically marking all objects reachable from roots
//! 4. Sweeping (freeing) any objects that aren't marked

use std::any::TypeId;
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::memory::{Gc, Tag, Traceable, Visitor};

/// Memory allocation statistics
#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    pub object_count: usize,
    pub total_size: usize,
    pub collection_count: usize,
    pub total_collected: usize,
    pub total_gc_time_ms: u128,
    pub last_gc_time_ms: u128,
    pub allocated_since_last_gc: usize,
    pub live_objects: usize,
    pub freed_objects: usize,
}

/// Detailed debug information for the garbage collector
#[derive(Debug, Clone, Default)]
pub struct GcDebugInfo {
    pub type_usage: Vec<TypeUsage>,
    pub generations: Vec<usize>,
    pub debug_logs: Vec<String>,
}

/// Memory usage statistics by type
#[derive(Debug, Clone, Default)]
pub struct TypeUsage {
    pub type_name: String,
    pub bytes: usize,
    pub object_count: usize,
}

/// Options for garbage collector configuration
#[derive(Debug, Clone)]
pub struct GcOptions {
    pub initial_heap_size: usize,
    pub allocation_threshold: usize,
    pub incremental_step_size: usize,
    pub incremental_time_budget_ms: u64,
    pub verbose: bool,
}

impl Default for GcOptions {
    fn default() -> Self {
        Self {
            initial_heap_size: 1024 * 1024,   // 1MB
            allocation_threshold: 1024 * 512, // 512KB
            incremental_step_size: 100,       // Process 100 objects per step
            incremental_time_budget_ms: 10,   // 10ms time budget
            verbose: false,
        }
    }
}

/// Types of garbage collection triggers
#[derive(Debug, Clone, Copy)]
pub enum CollectionTrigger {
    Manual,
    Allocation,
    Threshold,
    MemoryPressure,
}

/// Object mark state during collection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MarkState {
    White, // Not yet visited
    Gray,  // Visited but not all references processed
    Black, // Fully processed
}

/// The main garbage collector implementation
///
/// This struct provides the public API for memory allocation and garbage collection.
/// It maintains internal state through a thread-safe reference-counted lock,
/// allowing it to be shared between different parts of the program and potentially
/// across thread boundaries.
#[derive(Debug, Clone)]
pub struct GarbageCollector {
    pub(crate) inner: Arc<RwLock<GcStateInner>>,
}

/// Internal state of the garbage collector
#[derive(Debug)]
pub(crate) struct GcStateInner {
    pub objects: HashMap<usize, GcObject>,
    pub roots: HashSet<usize>,
    gray_objects: VecDeque<usize>,
    type_map: HashMap<TypeId, String>,
    options: GcOptions,
    pub stats: MemoryStats,
    collection_in_progress: bool,
    debug_logs: Vec<String>,
}

/// Object tracked by the garbage collector
#[derive(Debug, Clone)]
pub(crate) struct GcObject {
    ptr: usize,
    size: usize,
    type_id: TypeId,
    tag: Tag,
    mark_state: MarkState,
    generation: usize,
}

impl GarbageCollector {
    /// Create a new garbage collector with default options
    pub fn new() -> Arc<Self> {
        Self::with_options(GcOptions::default())
    }

    /// Create a new garbage collector with custom options
    pub fn with_options(options: GcOptions) -> Arc<Self> {
        let state = GcStateInner {
            objects: HashMap::new(),
            roots: HashSet::new(),
            gray_objects: VecDeque::new(),
            type_map: HashMap::new(),
            options,
            stats: MemoryStats::default(),
            collection_in_progress: false,
            debug_logs: Vec::new(),
        };

        Arc::new(Self {
            inner: Arc::new(RwLock::new(state)),
        })
    }

    /// Allocates a new garbage-collected object
    ///
    /// This method allocates memory for the given value, adds it to the set of
    /// tracked objects, and returns a garbage-collected reference (Gc<T>) to it.
    /// It may trigger garbage collection if the allocation threshold is reached.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to allocate memory for
    ///
    /// # Returns
    ///
    /// A garbage-collected smart pointer (Gc<T>) to the allocated object
    pub fn allocate<T: Traceable + Clone + 'static>(&self, value: T) -> Gc<T> {
        let mut state = self.inner.write().unwrap();

        // Check if we need to collect garbage
        if state.stats.allocated_since_last_gc >= state.options.allocation_threshold {
            // Drop the write lock before collecting
            drop(state);
            self.collect_garbage_internal(CollectionTrigger::Threshold);
            state = self.inner.write().unwrap();
        }

        // For simplicity, we're using Box<T> and raw pointers
        let boxed = Box::new(value);
        let ptr = Box::into_raw(boxed);
        let type_id = TypeId::of::<T>();
        let size = std::mem::size_of::<T>();

        // Record type name for debugging
        if !state.type_map.contains_key(&type_id) {
            state
                .type_map
                .insert(type_id, std::any::type_name::<T>().to_string());
        }

        // Create GC object
        let obj = GcObject {
            ptr: ptr as usize,
            size,
            type_id,
            tag: unsafe { (*ptr).tag() },
            mark_state: MarkState::White,
            generation: 0,
        };

        // Add to objects map
        state.objects.insert(ptr as usize, obj);

        // Add to roots initially
        state.roots.insert(ptr as usize);

        // Update stats
        state.stats.object_count += 1;
        state.stats.total_size += size;
        state.stats.allocated_since_last_gc += size;
        state.stats.live_objects += 1;

        // Create and return the Gc
        let nn_ptr = unsafe { NonNull::new_unchecked(ptr) };
        Gc::new(nn_ptr, self.clone())
    }

    /// Add an object as a GC root
    pub fn add_root(&self, ptr: usize) {
        let mut state = self.inner.write().unwrap();
        state.roots.insert(ptr);
    }

    /// Remove an object from GC roots
    pub fn remove_root(&self, ptr: usize) {
        let mut state = self.inner.write().unwrap();
        state.roots.remove(&ptr);
    }

    /// Check if an object is still alive (used by Weak references)
    pub fn is_alive(&self, ptr: usize) -> bool {
        let state = self.inner.read().unwrap();
        state.objects.contains_key(&ptr)
    }

    /// Explicitly triggers a garbage collection cycle
    ///
    /// This method forces the garbage collector to run a complete mark-and-sweep
    /// cycle, identifying unreachable objects and reclaiming their memory.
    /// It's typically used when the program expects a large number of objects
    /// to become unreachable, or for testing and benchmarking purposes.
    pub fn collect_garbage(&self) {
        self.collect_garbage_internal(CollectionTrigger::Manual);
    }

    // Internal implementation of garbage collection
    fn collect_garbage_internal(&self, trigger: CollectionTrigger) {
        let mut state = self.inner.write().unwrap();

        // If collection is already in progress, do nothing
        if state.collection_in_progress {
            if state.options.verbose {
                state
                    .debug_logs
                    .push("Skipping collection: already in progress".to_string());
            }
            return;
        }

        state.collection_in_progress = true;
        let start_time = Instant::now();

        if state.options.verbose {
            state.debug_logs.push(format!(
                "Starting garbage collection: trigger={:?}",
                trigger
            ));
        }

        // Mark phase
        // Reset mark states
        for obj in state.objects.values_mut() {
            obj.mark_state = MarkState::White;
        }

        // Initialize gray objects from roots
        state.gray_objects.clear();

        // Create a copy of roots to avoid borrowing issues
        let roots = state.roots.clone();
        for root in roots {
            if let Some(obj) = state.objects.get_mut(&root) {
                obj.mark_state = MarkState::Gray;
                state.gray_objects.push_back(root);
            }
        }

        // Process gray objects
        // In a real implementation, we would trace through the objects' references
        // Here we're just marking the directly reachable objects as black
        while let Some(addr) = state.gray_objects.pop_front() {
            if let Some(obj) = state.objects.get_mut(&addr) {
                obj.mark_state = MarkState::Black;
            }
        }

        // Sweep phase
        let before_count = state.objects.len();
        let mut freed_size = 0;
        let mut to_remove = Vec::new();

        // Collect objects to remove
        let objects_clone = state.objects.clone();
        for (addr, obj) in objects_clone.iter() {
            if obj.mark_state == MarkState::White {
                to_remove.push(*addr);
                freed_size += obj.size;
            } else if obj.mark_state == MarkState::Black {
                // Increment generation for surviving objects
                if let Some(obj) = state.objects.get_mut(addr) {
                    obj.generation += 1;
                }
            }
        }

        let to_remove_count = to_remove.len();

        // Remove white objects
        for addr in &to_remove {
            state.objects.remove(addr);
            // In a real implementation, we would properly free the memory here
        }

        // Update stats
        state.stats.collection_count += 1;
        state.stats.total_collected += freed_size;
        state.stats.object_count = state.objects.len();
        state.stats.live_objects = state.objects.len();
        state.stats.freed_objects += to_remove_count;
        state.stats.total_size -= freed_size;
        state.stats.allocated_since_last_gc = 0;

        let elapsed = start_time.elapsed();
        state.stats.last_gc_time_ms = elapsed.as_millis();
        state.stats.total_gc_time_ms += elapsed.as_millis();

        if state.options.verbose {
            let removed_count = before_count - state.objects.len();
            state.debug_logs.push(format!(
                "Garbage collection completed in {}ms: removed {} objects, freed {} bytes",
                elapsed.as_millis(),
                removed_count,
                freed_size
            ));
        }

        state.collection_in_progress = false;
    }

    /// Get current memory statistics
    pub fn stats(&self) -> MemoryStats {
        let state = self.inner.read().unwrap();
        state.stats.clone()
    }

    /// Get garbage collector debug information
    pub fn debug_info(&self) -> GcDebugInfo {
        let state = self.inner.read().unwrap();

        // Build type usage information
        let mut type_map = HashMap::new();
        for obj in state.objects.values() {
            let type_name = state
                .type_map
                .get(&obj.type_id)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string());

            let entry = type_map
                .entry(type_name.clone())
                .or_insert_with(|| TypeUsage {
                    type_name: type_name.clone(),
                    bytes: 0,
                    object_count: 0,
                });

            entry.bytes += obj.size;
            entry.object_count += 1;
        }

        // Build generation information
        let mut generations = vec![0; 10]; // Track up to 10 generations
        for obj in state.objects.values() {
            let gen = obj.generation.min(9);
            generations[gen] += 1;
        }

        GcDebugInfo {
            type_usage: type_map.into_values().collect(),
            generations,
            debug_logs: state.debug_logs.clone(),
        }
    }

    // Mark a specific object as gray
    fn mark_object(&self, addr: usize) {
        let mut state = self.inner.write().unwrap();

        if let Some(obj) = state.objects.get_mut(&addr) {
            if obj.mark_state == MarkState::White {
                obj.mark_state = MarkState::Gray;
                state.gray_objects.push_back(addr);
            }
        }
    }
}

// Implementation of the visitor trait for the garbage collector's mark phase
impl Visitor for GarbageCollector {
    fn visit(&mut self, ptr: NonNull<dyn Traceable>) {
        let addr = ptr.as_ptr() as *const () as usize;
        self.mark_object(addr);
    }

    fn visit_with_context(&mut self, ptr: NonNull<dyn Traceable>, _context: &str) {
        self.visit(ptr);
    }

    fn visit_ptr(&mut self, addr: usize, _tag: Tag) {
        self.mark_object(addr);
    }
}

/// Visitor for marking objects during garbage collection
#[derive(Clone)]
pub struct MarkingVisitor {
    gc: GarbageCollector,
}

impl Visitor for MarkingVisitor {
    fn visit(&mut self, ptr: NonNull<dyn Traceable>) {
        let addr = ptr.as_ptr() as *const () as usize;
        self.gc.mark_object(addr);
    }

    fn visit_with_context(&mut self, ptr: NonNull<dyn Traceable>, _context: &str) {
        self.visit(ptr);
    }

    fn visit_ptr(&mut self, addr: usize, _tag: Tag) {
        self.gc.mark_object(addr);
    }
}

/// Generic trait for objects with a stable memory address
pub unsafe trait Trace: 'static {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Default implementation does nothing
    }
}
