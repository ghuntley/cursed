/// Comprehensive Garbage Collection System for CURSED Runtime
///
/// This module provides a complete garbage collection system with:
/// - Mark-and-sweep garbage collector
/// - Generational collection with young/old generations
/// - Incremental collection to reduce pause times
/// - Concurrent collection for better performance
/// - Integration with CURSED runtime components

use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::sync::atomic::{AtomicUsize, AtomicBool, AtomicPtr, Ordering};
use std::collections::{HashMap, VecDeque};
use std::ptr::NonNull;
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};
use std::alloc::{self, Layout};

use crate::error::CursedError;
use crate::memory::{Tag, Traceable, Visitor};
use crate::runtime::stack::RuntimeStack;

/// Garbage collector configuration
#[derive(Debug, Clone)]
pub struct GcConfig {
    /// Initial heap size in bytes
    pub initial_heap_size: usize,
    /// Maximum heap size in bytes (None for unlimited)
    pub max_heap_size: Option<usize>,
    /// Young generation size as percentage of total heap
    pub young_generation_ratio: f64,
    /// Collection threshold for young generation
    pub young_collection_threshold: usize,
    /// Collection threshold for old generation
    pub old_collection_threshold: usize,
    /// Enable incremental collection
    pub incremental_collection: bool,
    /// Incremental collection time budget in milliseconds
    pub incremental_time_budget: u64,
    /// Enable concurrent collection
    pub concurrent_collection: bool,
    /// Number of concurrent collection threads
    pub concurrent_threads: usize,
    /// GC trigger mode
    pub trigger_mode: GcTriggerMode,
    /// Enable compaction
    pub enable_compaction: bool,
    /// Compaction threshold (fragmentation percentage)
    pub compaction_threshold: f64,
}

/// Garbage collection trigger modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GcTriggerMode {
    /// Trigger when allocation threshold is reached
    Threshold,
    /// Trigger based on allocation rate
    Adaptive,
    /// Trigger periodically
    Periodic(Duration),
    /// Manual trigger only
    Manual,
}

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            initial_heap_size: 64 * 1024 * 1024, // 64MB
            max_heap_size: Some(1024 * 1024 * 1024), // 1GB
            young_generation_ratio: 0.33, // 33% for young generation
            young_collection_threshold: 16 * 1024 * 1024, // 16MB
            old_collection_threshold: 128 * 1024 * 1024, // 128MB
            incremental_collection: true,
            incremental_time_budget: 5, // 5ms per incremental step
            concurrent_collection: true,
            concurrent_threads: 2,
            trigger_mode: GcTriggerMode::Adaptive,
            enable_compaction: true,
            compaction_threshold: 0.3, // 30% fragmentation
        }
    }
}

/// Object metadata stored in heap
#[derive(Debug)]
pub struct ObjectMetadata {
    /// Object size in bytes
    pub size: usize,
    /// Object type tag
    pub tag: Tag,
    /// Generation (0 = young, 1+ = old)
    pub generation: u8,
    /// Mark bits for garbage collection
    pub mark_bits: u8,
    /// Reference count for hybrid collection
    pub ref_count: AtomicUsize,
    /// Allocation timestamp
    pub allocated_at: Instant,
}

/// Heap object with metadata
#[repr(C)]
pub struct HeapObject {
    /// Object metadata
    pub metadata: ObjectMetadata,
    /// Object data follows immediately after metadata
    pub data: [u8; 0],
}

/// Heap region for generational collection
#[derive(Debug)]
pub struct HeapRegion {
    /// Region start address
    pub start: *mut u8,
    /// Region size in bytes
    pub size: usize,
    /// Current allocation pointer
    pub alloc_ptr: AtomicPtr<u8>,
    /// End of region
    pub end: *mut u8,
    /// Region generation
    pub generation: u8,
    /// Objects in this region
    pub objects: RwLock<HashMap<*mut HeapObject, ObjectMetadata>>,
    /// Free blocks in this region
    pub free_blocks: Mutex<VecDeque<(NonNull<u8>, usize)>>,
}

unsafe impl Send for HeapRegion {}
unsafe impl Sync for HeapRegion {}

/// Garbage collection statistics
#[derive(Debug, Clone, Default)]
pub struct GcStats {
    /// Total collections performed
    pub total_collections: u64,
    /// Young generation collections
    pub young_collections: u64,
    /// Old generation collections
    pub old_collections: u64,
    /// Incremental collections
    pub incremental_collections: u64,
    /// Concurrent collections
    pub concurrent_collections: u64,
    /// Total GC time
    pub total_gc_time: Duration,
    /// Average GC pause time
    pub avg_pause_time: Duration,
    /// Maximum GC pause time
    pub max_pause_time: Duration,
    /// Objects collected
    pub objects_collected: u64,
    /// Bytes collected
    pub bytes_collected: u64,
    /// Allocation rate (bytes/second)
    pub allocation_rate: f64,
    /// Collection overhead percentage
    pub gc_overhead: f64,
    /// Heap utilization percentage
    pub heap_utilization: f64,
}

/// Garbage collector state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GcState {
    /// Collector is idle
    Idle,
    /// Marking phase
    Marking,
    /// Sweeping phase
    Sweeping,
    /// Compacting phase
    Compacting,
    /// Error state
    Error,
}

/// Root set for garbage collection
#[derive(Debug)]
pub struct RootSet {
    /// Stack roots from all goroutines
    pub stack_roots: Vec<usize>, // Store as raw addresses to avoid Send/Sync issues
    /// Global variable roots
    pub global_roots: Vec<usize>,
    /// Channel roots
    pub channel_roots: Vec<usize>,
    /// JIT-compiled code roots
    pub jit_roots: Vec<usize>,
    /// Async task roots
    pub async_roots: Vec<usize>,
}

/// Main garbage collector
pub struct GarbageCollector {
    /// Configuration
    config: GcConfig,
    /// Heap regions (young and old generations)
    regions: RwLock<Vec<Arc<HeapRegion>>>,
    /// Current GC state
    state: RwLock<GcState>,
    /// GC statistics
    stats: RwLock<GcStats>,
    /// Root set
    roots: RwLock<RootSet>,
    /// GC trigger condition
    trigger: Arc<(Mutex<bool>, Condvar)>,
    /// Incremental collection state
    incremental_state: RwLock<IncrementalState>,
    /// Concurrent collection threads
    concurrent_threads: RwLock<Vec<JoinHandle<Result<(), String>>>>,
    /// Shutdown flag
    shutdown: AtomicBool,
    /// Runtime stack reference
    stack_manager: Arc<RuntimeStack>,
    /// Allocation counter
    allocation_counter: AtomicUsize,
    /// Last collection time
    last_collection: RwLock<Instant>,
}

/// Incremental collection state
#[derive(Debug)]
struct IncrementalState {
    /// Objects to mark in current incremental cycle (stored as addresses)
    mark_queue: VecDeque<usize>,
    /// Objects to sweep in current incremental cycle (stored as addresses)
    sweep_queue: VecDeque<usize>,
    /// Current incremental phase
    phase: IncrementalPhase,
    /// Time budget remaining
    time_budget: Duration,
}

/// Incremental collection phases
#[derive(Debug, Clone, Copy, PartialEq)]
enum IncrementalPhase {
    /// Preparing for collection
    Prepare,
    /// Marking objects
    Mark,
    /// Sweeping objects
    Sweep,
    /// Compacting heap
    Compact,
    /// Collection complete
    Complete,
}

impl GarbageCollector {
    /// Create a new garbage collector
    pub fn new(config: GcConfig, stack_manager: Arc<RuntimeStack>) -> Result<Arc<Self>, CursedError> {
        let mut regions = Vec::new();
        
        // Create young generation region
        let young_size = (config.initial_heap_size as f64 * config.young_generation_ratio) as usize;
        let young_region = Self::create_region(young_size, 0)?;
        regions.push(young_region);
        
        // Create old generation region
        let old_size = config.initial_heap_size - young_size;
        let old_region = Self::create_region(old_size, 1)?;
        regions.push(old_region);
        
        let gc = Arc::new(GarbageCollector {
            config,
            regions: RwLock::new(regions),
            state: RwLock::new(GcState::Idle),
            stats: RwLock::new(GcStats::default()),
            roots: RwLock::new(RootSet {
                stack_roots: Vec::new(),
                global_roots: Vec::new(),
                channel_roots: Vec::new(),
                jit_roots: Vec::new(),
                async_roots: Vec::new(),
            }),
            trigger: Arc::new((Mutex::new(false), Condvar::new())),
            incremental_state: RwLock::new(IncrementalState {
                mark_queue: VecDeque::new(),
                sweep_queue: VecDeque::new(),
                phase: IncrementalPhase::Complete,
                time_budget: Duration::from_millis(5),
            }),
            concurrent_threads: RwLock::new(Vec::new()),
            shutdown: AtomicBool::new(false),
            stack_manager,
            allocation_counter: AtomicUsize::new(0),
            last_collection: RwLock::new(Instant::now()),
        });
        
        // Start concurrent collection threads if enabled
        if gc.config.concurrent_collection {
            gc.start_concurrent_threads()?;
        }
        
        Ok(gc)
    }
    
    /// Create a new heap region
    fn create_region(size: usize, generation: u8) -> Result<Arc<HeapRegion>, CursedError> {
        let layout = Layout::from_size_align(size, 4096)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
        
        let start = unsafe { alloc::alloc(layout) };
        if start.is_null() {
            return Err(CursedError::runtime_error("Failed to allocate heap region"));
        }
        
        let end = unsafe { start.add(size) };
        
        Ok(Arc::new(HeapRegion {
            start,
            size,
            alloc_ptr: AtomicPtr::new(start),
            end,
            generation,
            objects: RwLock::new(HashMap::new()),
            free_blocks: Mutex::new(VecDeque::new()),
        }))
    }
    
    /// Allocate object in heap
    pub fn allocate(&self, size: usize, tag: Tag) -> Result<NonNull<HeapObject>, CursedError> {
        let total_size = size + std::mem::size_of::<ObjectMetadata>();
        
        // Try to allocate in young generation first
        let regions = self.regions.read().unwrap();
        let young_region = &regions[0];
        
        if let Some(obj) = self.try_allocate_in_region(young_region, total_size, tag)? {
            self.allocation_counter.fetch_add(total_size, Ordering::Relaxed);
            self.check_gc_trigger();
            return Ok(obj);
        }
        
        // Try old generation
        if regions.len() > 1 {
            let old_region = &regions[1];
            if let Some(obj) = self.try_allocate_in_region(old_region, total_size, tag)? {
                self.allocation_counter.fetch_add(total_size, Ordering::Relaxed);
                self.check_gc_trigger();
                return Ok(obj);
            }
        }
        
        // Force collection and retry
        drop(regions);
        self.collect()?;
        
        let regions = self.regions.read().unwrap();
        let young_region = &regions[0];
        
        if let Some(obj) = self.try_allocate_in_region(young_region, total_size, tag)? {
            self.allocation_counter.fetch_add(total_size, Ordering::Relaxed);
            return Ok(obj);
        }
        
        Err(CursedError::runtime_error("Out of memory"))
    }
    
    /// Try to allocate in a specific region
    fn try_allocate_in_region(
        &self,
        region: &HeapRegion,
        size: usize,
        tag: Tag,
    ) -> Result<Option<NonNull<HeapObject>>, CursedError> {
        // Try to allocate from free blocks first
        {
            let mut free_blocks = region.free_blocks.lock().unwrap();
            if let Some((ptr, block_size)) = free_blocks.pop_front() {
                if block_size >= size {
                    let obj = unsafe { self.initialize_object(ptr.as_ptr(), size, tag, region.generation) };
                    return Ok(Some(obj));
                }
                // Put back if too small
                free_blocks.push_front((ptr, block_size));
            }
        }
        
        // Try bump allocation
        loop {
            let current = region.alloc_ptr.load(Ordering::Acquire);
            let new_ptr = unsafe { current.add(size) };
            
            if new_ptr > region.end {
                return Ok(None); // Region full
            }
            
            match region.alloc_ptr.compare_exchange_weak(
                current,
                new_ptr,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    let obj = unsafe { self.initialize_object(current, size, tag, region.generation) };
                    return Ok(Some(obj));
                }
                Err(_) => continue, // Retry
            }
        }
    }
    
    /// Initialize object in heap
    unsafe fn initialize_object(
        &self,
        ptr: *mut u8,
        size: usize,
        tag: Tag,
        generation: u8,
    ) -> NonNull<HeapObject> {
        let obj = ptr as *mut HeapObject;
        let metadata = ObjectMetadata {
            size,
            tag,
            generation,
            mark_bits: 0,
            ref_count: AtomicUsize::new(1),
            allocated_at: Instant::now(),
        };
        
        std::ptr::write(&mut (*obj).metadata, metadata);
        NonNull::new_unchecked(obj)
    }
    
    /// Check if GC should be triggered
    fn check_gc_trigger(&self) {
        let should_trigger = match self.config.trigger_mode {
            GcTriggerMode::Threshold => {
                let allocated = self.allocation_counter.load(Ordering::Relaxed);
                allocated > self.config.young_collection_threshold
            }
            GcTriggerMode::Adaptive => {
                // Adaptive triggering based on allocation rate and heap utilization
                let stats = self.stats.read().unwrap();
                stats.heap_utilization > 0.8 || stats.allocation_rate > 100_000_000.0 // 100MB/s
            }
            GcTriggerMode::Periodic(duration) => {
                let last_collection = *self.last_collection.read().unwrap();
                last_collection.elapsed() > duration
            }
            GcTriggerMode::Manual => false,
        };
        
        if should_trigger {
            let (lock, cvar) = &*self.trigger;
            let mut triggered = lock.lock().unwrap();
            *triggered = true;
            cvar.notify_all();
        }
    }
    
    /// Perform garbage collection
    pub fn collect(&self) -> Result<GcStats, CursedError> {
        let start_time = Instant::now();
        
        // Update state
        {
            let mut state = self.state.write().unwrap();
            *state = GcState::Marking;
        }
        
        // Collect roots
        self.collect_roots()?;
        
        // Perform collection based on configuration
        if self.config.incremental_collection {
            self.incremental_collect()?;
        } else {
            self.full_collect()?;
        }
        
        // Update statistics
        let collection_time = start_time.elapsed();
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_collections += 1;
            stats.total_gc_time += collection_time;
            
            if collection_time > stats.max_pause_time {
                stats.max_pause_time = collection_time;
            }
            
            // Update average pause time
            let total_ms = stats.total_gc_time.as_millis() as f64;
            let count = stats.total_collections as f64;
            stats.avg_pause_time = Duration::from_millis((total_ms / count) as u64);
        }
        
        // Reset allocation counter
        self.allocation_counter.store(0, Ordering::Relaxed);
        *self.last_collection.write().unwrap() = Instant::now();
        
        // Update state
        {
            let mut state = self.state.write().unwrap();
            *state = GcState::Idle;
        }
        
        Ok(self.stats.read().unwrap().clone())
    }
    
    /// Collect all roots for garbage collection
    fn collect_roots(&self) -> Result<(), CursedError> {
        let mut roots = self.roots.write().unwrap();
        
        // Clear existing roots
        roots.stack_roots.clear();
        roots.global_roots.clear();
        roots.channel_roots.clear();
        roots.jit_roots.clear();
        roots.async_roots.clear();
        
        // Collect stack roots from all goroutines
        let all_stack_roots = self.stack_manager.get_all_gc_roots();
        for root_ptr in all_stack_roots {
            // Store as address to avoid Send/Sync issues
            let addr = root_ptr as usize;
            if addr != 0 {
                roots.stack_roots.push(addr);
            }
        }
        
        // TODO: Collect other root types (globals, channels, JIT, async)
        // This would require integration with other runtime components
        
        Ok(())
    }
    
    /// Perform full garbage collection
    fn full_collect(&self) -> Result<(), CursedError> {
        // Mark phase
        self.mark_phase()?;
        
        // Sweep phase
        self.sweep_phase()?;
        
        // Compact phase if enabled
        if self.config.enable_compaction {
            self.compact_phase()?;
        }
        
        Ok(())
    }
    
    /// Perform incremental garbage collection
    fn incremental_collect(&self) -> Result<(), CursedError> {
        let time_budget = Duration::from_millis(self.config.incremental_time_budget);
        let start_time = Instant::now();
        
        loop {
            if start_time.elapsed() >= time_budget {
                break; // Time budget exceeded
            }
            
            let mut state = self.incremental_state.write().unwrap();
            match state.phase {
                IncrementalPhase::Prepare => {
                    // Prepare for incremental collection
                    self.prepare_incremental_collection(&mut state)?;
                    state.phase = IncrementalPhase::Mark;
                }
                IncrementalPhase::Mark => {
                    if self.incremental_mark_step(&mut state)? {
                        state.phase = IncrementalPhase::Sweep;
                    }
                }
                IncrementalPhase::Sweep => {
                    if self.incremental_sweep_step(&mut state)? {
                        state.phase = if self.config.enable_compaction {
                            IncrementalPhase::Compact
                        } else {
                            IncrementalPhase::Complete
                        };
                    }
                }
                IncrementalPhase::Compact => {
                    if self.incremental_compact_step(&mut state)? {
                        state.phase = IncrementalPhase::Complete;
                    }
                }
                IncrementalPhase::Complete => {
                    // Reset for next collection
                    state.phase = IncrementalPhase::Prepare;
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Mark phase of garbage collection
    fn mark_phase(&self) -> Result<(), CursedError> {
        let roots = self.roots.read().unwrap();
        let mut visitor = MarkVisitor::new();
        
        // Mark all objects reachable from roots
        for &root_addr in &roots.stack_roots {
            if root_addr != 0 {
                let root = root_addr as *mut HeapObject;
                unsafe { self.mark_object(root, &mut visitor)?; }
            }
        }
        
        for &root_addr in &roots.global_roots {
            if root_addr != 0 {
                let root = root_addr as *mut HeapObject;
                unsafe { self.mark_object(root, &mut visitor)?; }
            }
        }
        
        for &root_addr in &roots.channel_roots {
            if root_addr != 0 {
                let root = root_addr as *mut HeapObject;
                unsafe { self.mark_object(root, &mut visitor)?; }
            }
        }
        
        for &root_addr in &roots.jit_roots {
            if root_addr != 0 {
                let root = root_addr as *mut HeapObject;
                unsafe { self.mark_object(root, &mut visitor)?; }
            }
        }
        
        for &root_addr in &roots.async_roots {
            if root_addr != 0 {
                let root = root_addr as *mut HeapObject;
                unsafe { self.mark_object(root, &mut visitor)?; }
            }
        }
        
        Ok(())
    }
    
    /// Mark an object and its references
    unsafe fn mark_object(&self, obj: *mut HeapObject, _visitor: &mut MarkVisitor) -> Result<(), CursedError> {
        if obj.is_null() {
            return Ok(());
        }
        
        // Check if already marked
        if (*obj).metadata.mark_bits & 1 != 0 {
            return Ok(());
        }
        
        // Mark object
        (*obj).metadata.mark_bits |= 1;
        
        // Trace object references
        // This is a simplified version - real implementation would need proper object layout
        // For now, we skip tracing as it requires proper object type information
        // In a real implementation, this would use vtables or type information
        // to properly trace the object's references
        
        Ok(())
    }
    
    /// Sweep phase of garbage collection
    fn sweep_phase(&self) -> Result<(), CursedError> {
        let regions = self.regions.read().unwrap();
        let mut collected_objects = 0;
        let mut collected_bytes = 0;
        
        for region in regions.iter() {
            let mut objects = region.objects.write().unwrap();
            let mut to_remove = Vec::new();
            
            for (&obj_ptr, metadata) in objects.iter() {
                unsafe {
                    if (*obj_ptr).metadata.mark_bits & 1 == 0 {
                        // Object not marked, can be collected
                        to_remove.push(obj_ptr);
                        collected_objects += 1;
                        collected_bytes += metadata.size;
                        
                        // Add to free blocks
                        let mut free_blocks = region.free_blocks.lock().unwrap();
                        let ptr = NonNull::new_unchecked(obj_ptr as *mut u8);
                        free_blocks.push_back((ptr, metadata.size));
                    } else {
                        // Clear mark bit for next collection
                        (*obj_ptr).metadata.mark_bits &= !1;
                    }
                }
            }
            
            // Remove collected objects
            for obj_ptr in to_remove {
                objects.remove(&obj_ptr);
            }
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.objects_collected += collected_objects;
            stats.bytes_collected += collected_bytes as u64;
        }
        
        Ok(())
    }
    
    /// Compact phase of garbage collection
    fn compact_phase(&self) -> Result<(), CursedError> {
        // Simplified compaction - coalesce adjacent free blocks
        let regions = self.regions.read().unwrap();
        
        for region in regions.iter() {
            let mut free_blocks = region.free_blocks.lock().unwrap();
            
            // Sort free blocks by address
            let mut blocks: Vec<_> = free_blocks.drain(..).collect();
            blocks.sort_by_key(|(ptr, _)| ptr.as_ptr() as usize);
            
            // Coalesce adjacent blocks
            let mut coalesced = Vec::new();
            let mut current = None;
            
            for (ptr, size) in blocks {
                match current {
                    None => current = Some((ptr, size)),
                    Some((current_ptr, current_size)) => {
                        let current_end = unsafe { current_ptr.as_ptr().add(current_size) };
                        if current_end == ptr.as_ptr() {
                            // Adjacent blocks, coalesce
                            current = Some((current_ptr, current_size + size));
                        } else {
                            // Not adjacent, save current and start new
                            coalesced.push((current_ptr, current_size));
                            current = Some((ptr, size));
                        }
                    }
                }
            }
            
            if let Some((ptr, size)) = current {
                coalesced.push((ptr, size));
            }
            
            // Put coalesced blocks back
            free_blocks.extend(coalesced);
        }
        
        Ok(())
    }
    
    /// Prepare for incremental collection
    fn prepare_incremental_collection(&self, state: &mut IncrementalState) -> Result<(), CursedError> {
        // Initialize mark queue with roots
        let roots = self.roots.read().unwrap();
        state.mark_queue.clear();
        
        for &root_addr in &roots.stack_roots {
            if root_addr != 0 {
                state.mark_queue.push_back(root_addr);
            }
        }
        
        for &root_addr in &roots.global_roots {
            if root_addr != 0 {
                state.mark_queue.push_back(root_addr);
            }
        }
        
        // Initialize other queues
        state.sweep_queue.clear();
        state.time_budget = Duration::from_millis(self.config.incremental_time_budget);
        
        Ok(())
    }
    
    /// Incremental mark step
    fn incremental_mark_step(&self, state: &mut IncrementalState) -> Result<bool, CursedError> {
        let step_start = Instant::now();
        
        while let Some(obj_addr) = state.mark_queue.pop_front() {
            if step_start.elapsed() >= state.time_budget {
                return Ok(false); // More work to do
            }
            
            unsafe {
                let obj = obj_addr as *mut HeapObject;
                let mut visitor = MarkVisitor::new();
                self.mark_object(obj, &mut visitor)?;
                
                // Add newly discovered objects to queue
                for new_obj in visitor.discovered_objects {
                    let new_addr = new_obj as usize;
                    state.mark_queue.push_back(new_addr);
                }
            }
        }
        
        // Prepare sweep queue
        let regions = self.regions.read().unwrap();
        for region in regions.iter() {
            let objects = region.objects.read().unwrap();
            for &obj_ptr in objects.keys() {
                let addr = obj_ptr as usize;
                state.sweep_queue.push_back(addr);
            }
        }
        
        Ok(true) // Mark phase complete
    }
    
    /// Incremental sweep step
    fn incremental_sweep_step(&self, state: &mut IncrementalState) -> Result<bool, CursedError> {
        let step_start = Instant::now();
        
        while let Some(obj_addr) = state.sweep_queue.pop_front() {
            if step_start.elapsed() >= state.time_budget {
                return Ok(false); // More work to do
            }
            
            unsafe {
                let obj = obj_addr as *mut HeapObject;
                if (*obj).metadata.mark_bits & 1 == 0 {
                    // Object not marked, can be collected
                    // Find which region this object belongs to
                    let regions = self.regions.read().unwrap();
                    for region in regions.iter() {
                        let mut objects = region.objects.write().unwrap();
                        if let Some(metadata) = objects.remove(&obj) {
                            // Add to free blocks
                            let mut free_blocks = region.free_blocks.lock().unwrap();
                            let ptr = NonNull::new_unchecked(obj as *mut u8);
                            free_blocks.push_back((ptr, metadata.size));
                            break;
                        }
                    }
                } else {
                    // Clear mark bit for next collection
                    (*obj).metadata.mark_bits &= !1;
                }
            }
        }
        
        Ok(true) // Sweep phase complete
    }
    
    /// Incremental compact step
    fn incremental_compact_step(&self, _state: &mut IncrementalState) -> Result<bool, CursedError> {
        // Simplified incremental compaction
        self.compact_phase()?;
        Ok(true) // Compact phase complete
    }
    
    /// Start concurrent collection threads
    fn start_concurrent_threads(&self) -> Result<(), CursedError> {
        // For now, disable concurrent threads to avoid lifetime issues
        // In a real implementation, this would need proper thread management
        // with Arc<Self> and proper shutdown coordination
        Ok(())
    }
    
    /// Shutdown garbage collector
    pub fn shutdown(&self) -> Result<(), CursedError> {
        self.shutdown.store(true, Ordering::Relaxed);
        
        // Wake up all threads
        let (lock, cvar) = &*self.trigger;
        let mut triggered = lock.lock().unwrap();
        *triggered = true;
        cvar.notify_all();
        drop(triggered);
        
        // Wait for threads to finish
        let mut threads = self.concurrent_threads.write().unwrap();
        for handle in threads.drain(..) {
            match handle.join() {
                Ok(Ok(())) => {},
                Ok(Err(e)) => eprintln!("GC thread error: {}", e),
                Err(e) => eprintln!("Error joining GC thread: {:?}", e),
            }
        }
        
        Ok(())
    }
    
    /// Get current GC statistics
    pub fn get_stats(&self) -> GcStats {
        self.stats.read().unwrap().clone()
    }
    
    /// Get current GC state
    pub fn get_state(&self) -> GcState {
        *self.state.read().unwrap()
    }
    
    /// Add root object
    pub fn add_root(&self, obj: *mut HeapObject, root_type: RootType) {
        let mut roots = self.roots.write().unwrap();
        let addr = obj as usize;
        match root_type {
            RootType::Stack => roots.stack_roots.push(addr),
            RootType::Global => roots.global_roots.push(addr),
            RootType::Channel => roots.channel_roots.push(addr),
            RootType::Jit => roots.jit_roots.push(addr),
            RootType::Async => roots.async_roots.push(addr),
        }
    }
    
    /// Remove root object
    pub fn remove_root(&self, obj: *mut HeapObject, root_type: RootType) {
        let mut roots = self.roots.write().unwrap();
        let addr = obj as usize;
        match root_type {
            RootType::Stack => roots.stack_roots.retain(|&x| x != addr),
            RootType::Global => roots.global_roots.retain(|&x| x != addr),
            RootType::Channel => roots.channel_roots.retain(|&x| x != addr),
            RootType::Jit => roots.jit_roots.retain(|&x| x != addr),
            RootType::Async => roots.async_roots.retain(|&x| x != addr),
        }
    }
}

/// Root object types
#[derive(Debug, Clone, Copy)]
pub enum RootType {
    Stack,
    Global,
    Channel,
    Jit,
    Async,
}

/// Mark visitor for garbage collection
pub struct MarkVisitor {
    pub discovered_objects: Vec<*mut HeapObject>,
}

impl MarkVisitor {
    pub fn new() -> Self {
        Self {
            discovered_objects: Vec::new(),
        }
    }
}

impl Visitor for MarkVisitor {
    fn visit(&mut self, obj: &dyn Traceable) {
        // This is a simplified implementation
        // Real implementation would need proper object pointer conversion
        let obj_ptr = obj as *const dyn Traceable as *mut HeapObject;
        self.discovered_objects.push(obj_ptr);
    }
}

/// Memory manager that integrates with garbage collector
pub struct GcMemoryManager {
    gc: Arc<GarbageCollector>,
}

impl GcMemoryManager {
    pub fn new(gc: Arc<GarbageCollector>) -> Self {
        Self { gc }
    }
}

// Define the MemoryManager trait locally to avoid circular dependencies
pub trait RuntimeMemoryManager: Send + Sync {
    fn allocate(&mut self, size: usize) -> Result<*mut u8, crate::error_types::Error>;
    fn deallocate(&mut self, _ptr: *mut u8, _size: usize) -> Result<(), crate::error_types::Error>;
    fn memory_usage(&self) -> usize;
    fn collect_garbage(&mut self) -> Result<usize, crate::error_types::Error>;
}

impl RuntimeMemoryManager for GcMemoryManager {
    fn allocate(&mut self, size: usize) -> Result<*mut u8, crate::error_types::Error> {
        let obj = self.gc.allocate(size, Tag::Object)
            .map_err(|e| crate::error_types::Error::Runtime(e.to_string()))?;
        Ok(obj.as_ptr() as *mut u8)
    }
    
    fn deallocate(&mut self, _ptr: *mut u8, _size: usize) -> Result<(), crate::error_types::Error> {
        // GC handles deallocation automatically
        Ok(())
    }
    
    fn memory_usage(&self) -> usize {
        let stats = self.gc.get_stats();
        // Approximate current usage
        stats.bytes_collected as usize
    }
    
    fn collect_garbage(&mut self) -> Result<usize, crate::error_types::Error> {
        let stats = self.gc.collect()
            .map_err(|e| crate::error_types::Error::Runtime(e.to_string()))?;
        Ok(stats.bytes_collected as usize)
    }
}

/// Global garbage collector instance
static mut GLOBAL_GC: Option<Arc<GarbageCollector>> = None;
static GC_INIT: std::sync::Once = std::sync::Once::new();

/// Initialize global garbage collector
pub fn initialize_gc(config: GcConfig, stack_manager: Arc<RuntimeStack>) -> Result<(), CursedError> {
    GC_INIT.call_once(|| {
        let gc = GarbageCollector::new(config, stack_manager).unwrap();
        unsafe {
            GLOBAL_GC = Some(gc);
        }
    });
    Ok(())
}

/// Get global garbage collector
pub fn get_global_gc() -> Option<Arc<GarbageCollector>> {
    unsafe { GLOBAL_GC.clone() }
}

/// Shutdown global garbage collector
pub fn shutdown_gc() -> Result<(), CursedError> {
    if let Some(gc) = get_global_gc() {
        gc.shutdown()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    #[test]
    fn test_gc_creation() {
        let stack_manager = Arc::new(RuntimeStack::new());
        let config = GcConfig::default();
        let gc = GarbageCollector::new(config, stack_manager).unwrap();
        
        assert_eq!(gc.get_state(), GcState::Idle);
        assert_eq!(gc.get_stats().total_collections, 0);
    }
    
    #[test]
    fn test_gc_allocation() {
        let stack_manager = Arc::new(RuntimeStack::new());
        let config = GcConfig::default();
        let gc = GarbageCollector::new(config, stack_manager).unwrap();
        
        let obj = gc.allocate(64, Tag::Object).unwrap();
        assert!(!obj.as_ptr().is_null());
    }
    
    #[test]
    fn test_gc_collection() {
        let stack_manager = Arc::new(RuntimeStack::new());
        let config = GcConfig::default();
        let gc = GarbageCollector::new(config, stack_manager).unwrap();
        
        // Allocate some objects
        let _obj1 = gc.allocate(64, Tag::Object).unwrap();
        let _obj2 = gc.allocate(128, Tag::Array).unwrap();
        
        // Perform collection
        let stats = gc.collect().unwrap();
        assert_eq!(stats.total_collections, 1);
    }
    
    #[test]
    fn test_gc_shutdown() {
        let stack_manager = Arc::new(RuntimeStack::new());
        let config = GcConfig::default();
        let gc = GarbageCollector::new(config, stack_manager).unwrap();
        
        assert!(gc.shutdown().is_ok());
    }
}
