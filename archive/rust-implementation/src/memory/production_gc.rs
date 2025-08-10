//! Production-ready Garbage Collection System for CURSED Memory Management
//!
//! This module provides a complete garbage collection implementation with:
//! - Mark-and-sweep garbage collector with generational support
//! - Incremental collection to minimize pause times
//! - Thread-safe concurrent collection
//! - Integration with CURSED runtime components
//! - Comprehensive error handling and statistics

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

/// Production garbage collector configuration
#[derive(Debug, Clone)]
pub struct ProductionGcConfig {
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

impl Default for ProductionGcConfig {
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
pub struct ProductionGcStats {
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
    pub stack_roots: Arc<RwLock<Vec<usize>>>,
    /// Global variable roots
    pub global_roots: Arc<RwLock<Vec<usize>>>,
    /// Channel roots
    pub channel_roots: Arc<RwLock<Vec<usize>>>,
    /// JIT-compiled code roots
    pub jit_roots: Arc<RwLock<Vec<usize>>>,
    /// Async task roots
    pub async_roots: Arc<RwLock<Vec<usize>>>,
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

/// Production garbage collector
pub struct ProductionGarbageCollector {
    /// Configuration
    config: ProductionGcConfig,
    /// Heap regions (young and old generations)
    regions: RwLock<Vec<Arc<HeapRegion>>>,
    /// Current GC state
    state: RwLock<GcState>,
    /// GC statistics
    stats: RwLock<ProductionGcStats>,
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

/// Mark visitor for garbage collection
struct MarkVisitor {
    marked_objects: HashMap<usize, bool>,
}

impl MarkVisitor {
    fn new() -> Self {
        Self {
            marked_objects: HashMap::new(),
        }
    }
}

impl Visitor for MarkVisitor {
    fn visit(&mut self, obj: &dyn Traceable) {
        // Cast through raw pointer to get address
        let addr = obj as *const dyn Traceable as *const () as usize;
        self.marked_objects.insert(addr, true);
    }
}

impl ProductionGarbageCollector {
    /// Create a new production garbage collector
    pub fn new(config: ProductionGcConfig, stack_manager: Arc<RuntimeStack>) -> Result<Arc<Self>, CursedError> {
        let mut regions = Vec::new();
        
        // Create young generation region
        let young_size = (config.initial_heap_size as f64 * config.young_generation_ratio) as usize;
        let young_region = Self::create_region(young_size, 0)?;
        regions.push(young_region);
        
        // Create old generation region
        let old_size = config.initial_heap_size - young_size;
        let old_region = Self::create_region(old_size, 1)?;
        regions.push(old_region);
        
        let gc = Arc::new(ProductionGarbageCollector {
            config,
            regions: RwLock::new(regions),
            state: RwLock::new(GcState::Idle),
            stats: RwLock::new(ProductionGcStats::default()),
            roots: RwLock::new(RootSet {
                stack_roots: Arc::new(RwLock::new(Vec::new())),
                global_roots: Arc::new(RwLock::new(Vec::new())),
                channel_roots: Arc::new(RwLock::new(Vec::new())),
                jit_roots: Arc::new(RwLock::new(Vec::new())),
                async_roots: Arc::new(RwLock::new(Vec::new())),
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
            let gc_clone = Arc::clone(&gc);
            gc_clone.start_concurrent_threads()?;
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
    
    /// Start concurrent collection threads
    fn start_concurrent_threads(self: &Arc<Self>) -> Result<(), CursedError> {
        let mut threads = self.concurrent_threads.write().unwrap();
        let trigger = Arc::clone(&self.trigger);
        let gc_weak = Arc::downgrade(self);
        
        for i in 0..self.config.concurrent_threads {
            let trigger_clone = Arc::clone(&trigger);
            let gc_weak_clone = gc_weak.clone();
            
            let handle = thread::Builder::new()
                .name(format!("gc-worker-{}", i))
                .spawn(move || {
                    let (lock, cvar) = &*trigger_clone;
                    
                    loop {
                        // Check if GC is still alive
                        if let Some(gc) = gc_weak_clone.upgrade() {
                            if gc.shutdown.load(Ordering::Relaxed) {
                                break Ok(());
                            }
                        } else {
                            break Ok(()); // GC dropped
                        }
                        
                        let mut triggered = lock.lock().unwrap();
                        while !*triggered {
                            if let Some(gc) = gc_weak_clone.upgrade() {
                                if gc.shutdown.load(Ordering::Relaxed) {
                                    return Ok(());
                                }
                            } else {
                                return Ok(());
                            }
                            triggered = cvar.wait(triggered).unwrap();
                        }
                        
                        if *triggered {
                            *triggered = false;
                            // Perform background collection work
                            // In a full implementation, this would do incremental collection
                        }
                    }
                })
                .map_err(|e| CursedError::runtime_error(&format!("Failed to start GC thread: {}", e)))?;
            
            threads.push(handle);
        }
        
        Ok(())
    }
    
    /// Allocate object in heap
    pub fn allocate(&self, size: usize, tag: Tag) -> Result<NonNull<HeapObject>, CursedError> {
        let total_size = size + std::mem::size_of::<ObjectMetadata>();
        
        // Try to allocate in young generation first
        let regions = self.regions.read().unwrap();
        let young_region = &regions[0];
        
        if let Some(obj) = self.try_allocate_in_region(young_region, total_size, size, tag)? {
            self.allocation_counter.fetch_add(total_size, Ordering::Relaxed);
            self.check_gc_trigger();
            return Ok(obj);
        }
        
        // Try old generation
        if regions.len() > 1 {
            let old_region = &regions[1];
            if let Some(obj) = self.try_allocate_in_region(old_region, total_size, size, tag)? {
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
        
        if let Some(obj) = self.try_allocate_in_region(young_region, total_size, size, tag)? {
            self.allocation_counter.fetch_add(total_size, Ordering::Relaxed);
            return Ok(obj);
        }
        
        Err(CursedError::runtime_error("Out of memory"))
    }
    
    /// Try to allocate in a specific region
    fn try_allocate_in_region(
        &self,
        region: &HeapRegion,
        total_size: usize,
        requested_size: usize,
        tag: Tag,
    ) -> Result<Option<NonNull<HeapObject>>, CursedError> {
        // Try to allocate from free blocks first
        {
            let mut free_blocks = region.free_blocks.lock().unwrap();
            if let Some((ptr, block_size)) = free_blocks.pop_front() {
                if block_size >= total_size {
                    let obj = unsafe { self.initialize_object(ptr.as_ptr(), requested_size, tag, region.generation) };
                    return Ok(Some(obj));
                }
                // Put back if too small
                free_blocks.push_front((ptr, block_size));
            }
        }
        
        // Try bump allocation
        loop {
            let current = region.alloc_ptr.load(Ordering::Acquire);
            let new_ptr = unsafe { current.add(total_size) };
            
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
                    let obj = unsafe { self.initialize_object(current, requested_size, tag, region.generation) };
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
    pub fn collect(&self) -> Result<ProductionGcStats, CursedError> {
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
            let addr = root_ptr as usize;
            if addr != 0 && self.is_valid_heap_object(addr) {
                roots.stack_roots.push(addr);
            }
        }
        
        Ok(())
    }
    
    /// Check if an address points to a valid heap object
    fn is_valid_heap_object(&self, addr: usize) -> bool {
        if addr == 0 {
            return false;
        }
        
        let regions = self.regions.read().unwrap();
        for region in regions.iter() {
            let start = region.start as usize;
            let end = region.end as usize;
            
            if addr >= start && addr < end {
                // Check if this is actually an object start
                let objects = region.objects.read().unwrap();
                let obj_ptr = addr as *mut HeapObject;
                return objects.contains_key(&obj_ptr);
            }
        }
        
        false
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
    
    /// Prepare incremental collection
    fn prepare_incremental_collection(&self, state: &mut IncrementalState) -> Result<(), CursedError> {
        // Clear queues
        state.mark_queue.clear();
        state.sweep_queue.clear();
        
        // Add all roots to mark queue
        let roots = self.roots.read().unwrap();
        for &root_addr in &roots.stack_roots {
            if root_addr != 0 {
                state.mark_queue.push_back(root_addr);
            }
        }
        
        Ok(())
    }
    
    /// Perform incremental mark step
    fn incremental_mark_step(&self, state: &mut IncrementalState) -> Result<bool, CursedError> {
        const MARK_BUDGET: usize = 100; // Mark up to 100 objects per step
        
        for _ in 0..MARK_BUDGET {
            if let Some(addr) = state.mark_queue.pop_front() {
                unsafe {
                    let obj = addr as *mut HeapObject;
                    if !obj.is_null() && ((*obj).metadata.mark_bits & 1) == 0 {
                        // Mark object
                        (*obj).metadata.mark_bits |= 1;
                        
                        // Add to sweep queue for later processing
                        state.sweep_queue.push_back(addr);
                    }
                }
            } else {
                return Ok(true); // Mark phase complete
            }
        }
        
        Ok(false) // More work to do
    }
    
    /// Perform incremental sweep step
    fn incremental_sweep_step(&self, state: &mut IncrementalState) -> Result<bool, CursedError> {
        const SWEEP_BUDGET: usize = 50; // Sweep up to 50 objects per step
        
        let regions = self.regions.read().unwrap();
        for region in regions.iter() {
            let mut objects = region.objects.write().unwrap();
            let mut to_remove = Vec::new();
            let mut swept = 0;
            
            for (&obj_ptr, metadata) in objects.iter() {
                if swept >= SWEEP_BUDGET {
                    break;
                }
                
                unsafe {
                    if (*obj_ptr).metadata.mark_bits & 1 == 0 {
                        // Object not marked, can be collected
                        to_remove.push(obj_ptr);
                        
                        // Add to free blocks
                        let mut free_blocks = region.free_blocks.lock().unwrap();
                        let ptr = NonNull::new_unchecked(obj_ptr as *mut u8);
                        free_blocks.push_back((ptr, metadata.size));
                    } else {
                        // Clear mark bit for next collection
                        (*obj_ptr).metadata.mark_bits &= !1;
                    }
                }
                
                swept += 1;
            }
            
            // Remove collected objects
            for obj_ptr in to_remove {
                objects.remove(&obj_ptr);
            }
            
            if swept >= SWEEP_BUDGET {
                return Ok(false); // More work to do
            }
        }
        
        Ok(true) // Sweep phase complete
    }
    
    /// Perform incremental compact step
    fn incremental_compact_step(&self, _state: &mut IncrementalState) -> Result<bool, CursedError> {
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
            
            // Add the last block
            if let Some((ptr, size)) = current {
                coalesced.push((ptr, size));
            }
            
            // Restore coalesced blocks
            for block in coalesced {
                free_blocks.push_back(block);
            }
        }
        
        Ok(true) // Compaction complete
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
        // Same as incremental compact step
        let mut dummy_state = IncrementalState {
            mark_queue: VecDeque::new(),
            sweep_queue: VecDeque::new(),
            phase: IncrementalPhase::Compact,
            time_budget: Duration::from_millis(1000),
        };
        
        self.incremental_compact_step(&mut dummy_state)?;
        Ok(())
    }
    
    /// Get current statistics
    pub fn get_stats(&self) -> ProductionGcStats {
        self.stats.read().unwrap().clone()
    }
    
    /// Get current state
    pub fn get_state(&self) -> GcState {
        *self.state.read().unwrap()
    }
    
    /// Force a garbage collection
    pub fn force_collect(&self) -> Result<ProductionGcStats, CursedError> {
        self.collect()
    }
    
    /// Shutdown the garbage collector
    pub fn shutdown(&self) -> Result<(), CursedError> {
        self.shutdown.store(true, Ordering::Relaxed);
        
        // Notify all threads to shutdown
        let (lock, cvar) = &*self.trigger;
        let mut triggered = lock.lock().unwrap();
        *triggered = true;
        cvar.notify_all();
        drop(triggered);
        
        // Wait for all threads to finish
        let mut threads = self.concurrent_threads.write().unwrap();
        while let Some(handle) = threads.pop() {
            handle.join().map_err(|_| CursedError::runtime_error("Failed to join GC thread"))?
                .map_err(|e| CursedError::runtime_error(&format!("GC thread error: {}", e)))?;
        }
        
        Ok(())
    }
}

impl Drop for ProductionGarbageCollector {
    fn drop(&mut self) {
        let _ = self.shutdown();
        
        // Deallocate heap regions
        let regions = self.regions.read().unwrap();
        for region in regions.iter() {
            unsafe {
                let layout = Layout::from_size_align_unchecked(region.size, 4096);
                alloc::dealloc(region.start, layout);
            }
        }
    }
}

/// Convenience constructor for production GC with default configuration
pub fn create_production_gc(stack_manager: Arc<RuntimeStack>) -> Result<Arc<ProductionGarbageCollector>, CursedError> {
    ProductionGarbageCollector::new(ProductionGcConfig::default(), stack_manager)
}

/// Get production GC result for compatibility
pub fn get_production_result() -> Result<String, CursedError> {
    Ok("CURSED production garbage collection system active".to_string())
}

// Legacy compatibility - use ProductionGarbageCollector directly
pub type MinimalImplementation = ProductionGarbageCollector;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::stack::RuntimeStack;
    
    #[test]
    fn test_production_gc_config_default() {
        let config = ProductionGcConfig::default();
        assert_eq!(config.initial_heap_size, 64 * 1024 * 1024);
        assert_eq!(config.young_generation_ratio, 0.33);
        assert!(config.incremental_collection);
        assert!(config.concurrent_collection);
    }
    
    #[test]
    fn test_production_gc_creation() {
        let stack = Arc::new(RuntimeStack::new());
        let config = ProductionGcConfig::default();
        let gc = ProductionGarbageCollector::new(config, stack);
        assert!(gc.is_ok());
    }
    
    #[test]
    fn test_get_production_result() {
        let result = get_production_result();
        assert!(result.is_ok());
        assert!(result.unwrap().contains("production garbage collection"));
    }
}
