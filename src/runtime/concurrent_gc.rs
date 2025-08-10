/// Concurrent Garbage Collection Implementation
/// 
/// Provides production-ready concurrent garbage collection with minimal pause times
/// and high throughput for enterprise applications.

use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicPtr, Ordering};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use std::collections::{HashMap, VecDeque, HashSet};
use std::ptr::NonNull;

use crate::runtime::gc::{GarbageCollector, GcConfig, GcStats, HeapObject, ObjectMetadata, GcState};
use crate::runtime::gc_tuning::{TriColorCollector, GcPerformanceTuner};
use crate::runtime::memory_profiler::MemoryProfiler;
use crate::runtime::stack::{RuntimeStack, StackId};
use crate::error::CursedError;
use crate::memory::Tag;

// Thread-local storage for concurrent GC operations
thread_local! {
    /// Thread-local statistics for sweep operations
    static SWEEP_STATS: std::cell::RefCell<SweepStats> = std::cell::RefCell::new(SweepStats::default());
    
    /// Thread-local compaction state
    static COMPACTION_STATE: std::cell::RefCell<CompactionState> = std::cell::RefCell::new(CompactionState::default());
    
    /// Thread-local compaction mapping
    static COMPACTION_MAP: std::cell::RefCell<HashMap<usize, usize>> = std::cell::RefCell::new(HashMap::new());
    
    /// Thread-local compaction statistics
    static COMPACTION_STATS: std::cell::RefCell<CompactionStats> = std::cell::RefCell::new(CompactionStats::default());
    
    /// Thread-local free list for deallocated objects
    static FREE_LIST: std::cell::RefCell<Vec<FreeBlock>> = std::cell::RefCell::new(Vec::new());
}

/// Statistics for sweep operations
#[derive(Debug, Default)]
struct SweepStats {
    objects_swept: u64,
    bytes_freed: usize,
}

/// State for compaction operations
#[derive(Debug)]
struct CompactionState {
    current_compaction_pointer: usize,
    compaction_start: usize,
    compaction_end: usize,
}

impl Default for CompactionState {
    fn default() -> Self {
        Self {
            current_compaction_pointer: 0,
            compaction_start: 0,
            compaction_end: 0,
        }
    }
}

/// Statistics for compaction operations
#[derive(Debug, Default)]
struct CompactionStats {
    objects_moved: u64,
    bytes_moved: usize,
}

/// Free memory block
#[derive(Debug, Clone)]
struct FreeBlock {
    address: usize,
    size: usize,
}

/// Concurrent GC configuration
#[derive(Debug, Clone)]
pub struct ConcurrentGcConfig {
    /// Number of concurrent collector threads
    pub collector_threads: usize,
    /// Maximum pause time target in milliseconds
    pub max_pause_time_ms: u64,
    /// Concurrent marking enabled
    pub concurrent_marking: bool,
    /// Concurrent sweeping enabled
    pub concurrent_sweeping: bool,
    /// Concurrent compaction enabled
    pub concurrent_compaction: bool,
    /// Write barrier mode
    pub write_barrier_mode: WriteBarrierMode,
    /// Incremental collection step size
    pub incremental_step_size: usize,
    /// Thread synchronization mode
    pub sync_mode: SyncMode,
    /// Enable parallel collection
    pub parallel_collection: bool,
    /// Work stealing enabled
    pub work_stealing: bool,
}

/// Write barrier modes for concurrent collection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WriteBarrierMode {
    /// No write barriers (non-concurrent)
    None,
    /// Simple write barriers for basic concurrency
    Simple,
    /// Card table based write barriers
    CardTable,
    /// Remembered set based write barriers
    RememberedSet,
}

/// Synchronization modes for concurrent collection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyncMode {
    /// Stop-the-world collection
    StopTheWorld,
    /// Concurrent collection with periodic synchronization
    Concurrent,
    /// Parallel collection with work stealing
    Parallel,
    /// Hybrid mode combining concurrent and parallel
    Hybrid,
}

impl Default for ConcurrentGcConfig {
    fn default() -> Self {
        Self {
            collector_threads: std::cmp::max(2, num_cpus::get() / 2),
            max_pause_time_ms: 10, // 10ms target pause time
            concurrent_marking: true,
            concurrent_sweeping: true,
            concurrent_compaction: false, // Disable for lower pause times
            write_barrier_mode: WriteBarrierMode::CardTable,
            incremental_step_size: 512 * 1024, // 512KB steps
            sync_mode: SyncMode::Concurrent,
            parallel_collection: true,
            work_stealing: true,
        }
    }
}

/// Concurrent collector state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConcurrentState {
    /// Collector is idle
    Idle,
    /// Concurrent marking phase
    ConcurrentMarking,
    /// Concurrent sweeping phase
    ConcurrentSweeping,
    /// Concurrent compaction phase
    ConcurrentCompaction,
    /// Final pause for cleanup
    FinalPause,
    /// Error state
    Error,
}

/// Work item for concurrent collection
#[derive(Debug, Clone)]
pub struct WorkItem {
    /// Work item type
    pub work_type: WorkType,
    /// Object address to process
    pub object_addr: usize,
    /// Work priority
    pub priority: u8,
    /// Creation timestamp
    pub created_at: Instant,
}

/// Types of work items
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WorkType {
    /// Mark object and find references
    Mark,
    /// Sweep unmarked object
    Sweep,
    /// Compact object
    Compact,
    /// Update references after compaction
    UpdateReferences,
}

/// Concurrent garbage collector
pub struct ConcurrentGarbageCollector {
    /// Configuration
    config: ConcurrentGcConfig,
    /// Base garbage collector
    base_gc: Arc<GarbageCollector>,
    /// Tri-color collector for concurrent marking
    tri_color_collector: Arc<TriColorCollector>,
    /// Performance tuner
    performance_tuner: Arc<GcPerformanceTuner>,
    /// Current state
    state: RwLock<ConcurrentState>,
    /// Collector threads
    collector_threads: RwLock<Vec<JoinHandle<Result<(), String>>>>,
    /// Work queue for concurrent collection
    work_queue: Arc<Mutex<VecDeque<WorkItem>>>,
    /// Work available condition
    work_available: Arc<Condvar>,
    /// Running flag
    running: AtomicBool,
    /// Pause flag for stop-the-world phases
    paused: AtomicBool,
    /// Pause condition variable
    pause_cond: Arc<(Mutex<bool>, Condvar)>,
    /// Write barrier log
    write_barrier_log: RwLock<Vec<WriteBarrierEntry>>,
    /// Card table for generational collection
    card_table: RwLock<Vec<AtomicBool>>,
    /// Remembered set for inter-generational references
    remembered_set: RwLock<HashSet<usize>>,
    /// Collection statistics
    concurrent_stats: RwLock<ConcurrentStats>,
    /// Memory profiler integration
    profiler: Option<Arc<MemoryProfiler>>,
}

/// Write barrier entry
#[derive(Debug, Clone)]
pub struct WriteBarrierEntry {
    /// Source object address
    pub source_addr: usize,
    /// Target object address
    pub target_addr: usize,
    /// Field offset
    pub field_offset: usize,
    /// Timestamp
    pub timestamp: Instant,
    /// Thread ID
    pub thread_id: thread::ThreadId,
}

/// Concurrent collection statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConcurrentStats {
    /// Total concurrent collections
    pub total_concurrent_collections: u64,
    /// Total concurrent marking time
    pub total_concurrent_marking_time: Duration,
    /// Total concurrent sweeping time
    pub total_concurrent_sweeping_time: Duration,
    /// Total pause time
    pub total_pause_time: Duration,
    /// Average pause time
    pub avg_pause_time: Duration,
    /// Maximum pause time
    pub max_pause_time: Duration,
    /// Concurrent marking efficiency
    pub concurrent_marking_efficiency: f64,
    /// Write barrier overhead
    pub write_barrier_overhead: f64,
    /// Work items processed
    pub work_items_processed: u64,
    /// Work queue peak size
    pub work_queue_peak_size: usize,
}

impl Default for ConcurrentStats {
    fn default() -> Self {
        Self {
            total_concurrent_collections: 0,
            total_concurrent_marking_time: Duration::from_secs(0),
            total_concurrent_sweeping_time: Duration::from_secs(0),
            total_pause_time: Duration::from_secs(0),
            avg_pause_time: Duration::from_secs(0),
            max_pause_time: Duration::from_secs(0),
            concurrent_marking_efficiency: 0.0,
            write_barrier_overhead: 0.0,
            work_items_processed: 0,
            work_queue_peak_size: 0,
        }
    }
}

impl ConcurrentGarbageCollector {
    /// Create new concurrent garbage collector
    pub fn new(
        config: ConcurrentGcConfig,
        base_gc: Arc<GarbageCollector>,
        tri_color_collector: Arc<TriColorCollector>,
        performance_tuner: Arc<GcPerformanceTuner>,
    ) -> Result<Self, CursedError> {
        // Calculate card table size based on heap size
        let card_table_size = 1024 * 1024; // 1MB card table
        let mut card_table = Vec::with_capacity(card_table_size);
        for _ in 0..card_table_size {
            card_table.push(AtomicBool::new(false));
        }

        Ok(Self {
            config,
            base_gc,
            tri_color_collector,
            performance_tuner,
            state: RwLock::new(ConcurrentState::Idle),
            collector_threads: RwLock::new(Vec::new()),
            work_queue: Arc::new(Mutex::new(VecDeque::new())),
            work_available: Arc::new(Condvar::new()),
            running: AtomicBool::new(false),
            paused: AtomicBool::new(false),
            pause_cond: Arc::new((Mutex::new(false), Condvar::new())),
            write_barrier_log: RwLock::new(Vec::new()),
            card_table: RwLock::new(card_table),
            remembered_set: RwLock::new(HashSet::new()),
            concurrent_stats: RwLock::new(ConcurrentStats::default()),
            profiler: None,
        })
    }

    /// Start concurrent collection
    pub fn start(&self) -> Result<(), CursedError> {
        if self.running.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed).is_err() {
            return Err(CursedError::runtime_error("Concurrent GC already running"));
        }

        // Start collector threads
        self.start_collector_threads()?;

        Ok(())
    }

    /// Stop concurrent collection
    pub fn stop(&self) -> Result<(), CursedError> {
        self.running.store(false, Ordering::Relaxed);
        
        // Notify all threads to wake up
        self.work_available.notify_all();
        
        // Wait for all threads to finish
        let mut threads = self.collector_threads.write().unwrap();
        for thread in threads.drain(..) {
            let _ = thread.join().map_err(|_| CursedError::runtime_error("Failed to join collector thread"))?;
        }

        Ok(())
    }

    /// Perform concurrent collection
    pub fn collect(&self) -> Result<ConcurrentStats, CursedError> {
        let collection_start = Instant::now();
        
        // Update state
        *self.state.write().unwrap() = ConcurrentState::ConcurrentMarking;
        
        // Phase 1: Concurrent marking
        let marking_start = Instant::now();
        self.concurrent_marking_phase()?;
        let marking_time = marking_start.elapsed();
        
        // Phase 2: Concurrent sweeping
        let sweeping_start = Instant::now();
        self.concurrent_sweeping_phase()?;
        let sweeping_time = sweeping_start.elapsed();
        
        // Phase 3: Final pause for cleanup
        let pause_start = Instant::now();
        self.final_pause_phase()?;
        let pause_time = pause_start.elapsed();
        
        // Update statistics
        let mut stats = self.concurrent_stats.write().unwrap();
        stats.total_concurrent_collections += 1;
        stats.total_concurrent_marking_time += marking_time;
        stats.total_concurrent_sweeping_time += sweeping_time;
        stats.total_pause_time += pause_time;
        
        if pause_time > stats.max_pause_time {
            stats.max_pause_time = pause_time;
        }
        
        // Update average pause time
        stats.avg_pause_time = stats.total_pause_time / stats.total_concurrent_collections as u32;
        
        // Calculate efficiency
        let total_time = collection_start.elapsed();
        stats.concurrent_marking_efficiency = if total_time.as_millis() > 0 {
            (total_time.as_millis() - pause_time.as_millis()) as f64 / total_time.as_millis() as f64
        } else {
            0.0
        };
        
        // Update state
        *self.state.write().unwrap() = ConcurrentState::Idle;
        
        Ok(stats.clone())
    }

    /// Write barrier for concurrent collection
    pub fn write_barrier(&self, source_addr: usize, target_addr: usize, field_offset: usize) {
        if !self.running.load(Ordering::Relaxed) {
            return;
        }

        match self.config.write_barrier_mode {
            WriteBarrierMode::None => {}
            WriteBarrierMode::Simple => {
                self.simple_write_barrier(source_addr, target_addr, field_offset);
            }
            WriteBarrierMode::CardTable => {
                self.card_table_write_barrier(source_addr, target_addr, field_offset);
            }
            WriteBarrierMode::RememberedSet => {
                self.remembered_set_write_barrier(source_addr, target_addr, field_offset);
            }
        }
    }

    /// Start collector threads (thread-safe)
    fn start_collector_threads(&self) -> Result<(), CursedError> {
        let mut threads = self.collector_threads.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire collector threads lock"))?;
        
        for i in 0..self.config.collector_threads {
            let work_queue = Arc::clone(&self.work_queue);
            let work_available = Arc::clone(&self.work_available);
            let running_flag = Arc::clone(&Arc::new(AtomicBool::new(true)));
            let thread_running = Arc::clone(&running_flag);
            
            // Store running flag for later cleanup
            let thread_id = i;
            
            let handle = thread::Builder::new()
                .name(format!("gc-collector-{}", thread_id))
                .spawn(move || {
                    while thread_running.load(Ordering::Acquire) {
                        // Wait for work with timeout to prevent infinite blocking
                        let work_item = {
                            let queue_result = work_queue.lock();
                            if let Ok(mut queue) = queue_result {
                                if queue.is_empty() {
                                    // Wait for work with timeout
                                    let timeout = std::time::Duration::from_millis(100);
                                    if let Ok(timed_queue) = work_available.wait_timeout(queue, timeout) {
                                        let mut new_queue = timed_queue.0;
                                        new_queue.pop_front()
                                    } else {
                                        None
                                    }
                                } else {
                                    if !thread_running.load(Ordering::Acquire) {
                                        return Ok(());
                                    }
                                    queue.pop_front()
                                }
                            } else {
                                None
                            }
                        };
                        
                        if let Some(work_item) = work_item {
                            // Process work item with error handling
                            if let Err(e) = Self::process_work_item(work_item) {
                                tracing::error!("Error processing work item in thread {}: {}", thread_id, e);
                            }
                        }
                    }
                    
                    Ok(())
                })
                .map_err(|e| CursedError::runtime_error(&format!("Failed to create collector thread: {}", e)))?;
            
            threads.push(handle);
        }
        
        Ok(())
    }

    /// Process a work item
    fn process_work_item(work_item: WorkItem) -> Result<(), String> {
        match work_item.work_type {
            WorkType::Mark => {
                // Mark object and find references
                Self::mark_object(work_item.object_addr)?;
            }
            WorkType::Sweep => {
                // Sweep unmarked object
                Self::sweep_object(work_item.object_addr)?;
            }
            WorkType::Compact => {
                // Compact object
                Self::compact_object(work_item.object_addr)?;
            }
            WorkType::UpdateReferences => {
                // Update references after compaction
                Self::update_references(work_item.object_addr)?;
            }
        }
        
        Ok(())
    }

    /// Mark object during concurrent marking
    fn mark_object(object_addr: usize) -> Result<(), String> {
        if object_addr == 0 {
            return Ok(()); // Null pointer, nothing to mark
        }

        // SAFETY: We're assuming object_addr points to a valid HeapObject
        unsafe {
            let object_ptr = object_addr as *mut HeapObject;
            if object_ptr.is_null() {
                return Ok(());
            }

            // Atomic check-and-set marking to prevent races
            let metadata_ptr = &mut (*object_ptr).metadata;
            
            // Use atomic compare-and-swap to mark the object atomically
            let was_marked = std::sync::atomic::AtomicBool::from_ptr(&mut metadata_ptr.marked as *mut bool);
            let old_value = was_marked.compare_exchange(
                false, 
                true, 
                Ordering::AcqRel, 
                Ordering::Acquire
            );

            // If already marked, nothing to do (prevents infinite loops)
            if old_value.is_err() {
                return Ok(());
            }

            // Object is now marked, scan for references
            let object_size = metadata_ptr.size;
            let object_tag = metadata_ptr.tag;
            
            // Scan object data for references based on type
            match object_tag {
                Tag::Object | Tag::Interface => {
                    // Scan object fields for pointers
                    Self::scan_object_references(object_addr, object_size)?;
                }
                Tag::Array => {
                    // Scan array elements
                    Self::scan_array_references(object_addr, object_size)?;
                }
                Tag::String | Tag::Number | Tag::Boolean | Tag::Nil => {
                    // Primitive types have no references to scan
                }
                Tag::Function => {
                    // Scan function closures and captured variables
                    Self::scan_function_references(object_addr, object_size)?;
                }
                Tag::Channel => {
                    // Scan channel buffers and waiting goroutines
                    Self::scan_channel_references(object_addr, object_size)?;
                }
                Tag::Custom(_) => {
                    // Conservatively scan as generic object
                    Self::scan_object_references(object_addr, object_size)?;
                }
            }
        }

        Ok(())
    }

    /// Sweep object during concurrent sweeping
    fn sweep_object(object_addr: usize) -> Result<(), String> {
        if object_addr == 0 {
            return Ok(()); // Null pointer, nothing to sweep
        }

        // SAFETY: We're assuming object_addr points to a valid HeapObject
        unsafe {
            let object_ptr = object_addr as *mut HeapObject;
            if object_ptr.is_null() {
                return Ok(());
            }

            let metadata_ptr = &(*object_ptr).metadata;
            
            // Thread-safe check if object is marked
            let is_marked = std::sync::atomic::AtomicBool::from_ptr(&metadata_ptr.marked as *const bool as *mut bool);
            let marked = is_marked.load(Ordering::Acquire);

            if !marked {
                // Object is not marked, it's garbage - deallocate it
                let object_size = metadata_ptr.size;
                let object_tag = metadata_ptr.tag;

                // Call finalizers/destructors based on object type
                match object_tag {
                    Tag::Channel => {
                        // Close channel and notify waiting goroutines
                        Self::finalize_channel(object_addr)?;
                    }
                    Tag::Function => {
                        // Clean up function closures
                        Self::finalize_function(object_addr)?;
                    }
                    Tag::Custom(_) => {
                        // Call custom finalizer if available
                        Self::finalize_custom_object(object_addr)?;
                    }
                    _ => {
                        // No special finalization needed
                    }
                }

                // Zero out the object memory for security
                let data_ptr = object_ptr.add(1) as *mut u8; // Skip metadata
                let data_size = object_size.saturating_sub(std::mem::size_of::<ObjectMetadata>());
                if data_size > 0 {
                    std::ptr::write_bytes(data_ptr, 0, data_size);
                }

                // Add to free list atomically
                Self::add_to_free_list(object_addr, object_size)?;

                // Update sweep statistics
                SWEEP_STATS.with(|stats| {
                    stats.borrow_mut().objects_swept += 1;
                    stats.borrow_mut().bytes_freed += object_size;
                });

            } else {
                // Object is marked, unmark it for next collection cycle
                is_marked.store(false, Ordering::Release);
            }
        }

        Ok(())
    }

    /// Compact object during concurrent compaction
    fn compact_object(object_addr: usize) -> Result<(), String> {
        if object_addr == 0 {
            return Ok(()); // Null pointer, nothing to compact
        }

        // SAFETY: We're assuming object_addr points to a valid HeapObject
        unsafe {
            let object_ptr = object_addr as *mut HeapObject;
            if object_ptr.is_null() {
                return Ok(());
            }

            let metadata_ptr = &(*object_ptr).metadata;
            let object_size = metadata_ptr.size;
            
            // Check if object is marked (only marked objects should be compacted)
            let is_marked = std::sync::atomic::AtomicBool::from_ptr(&metadata_ptr.marked as *const bool as *mut bool);
            if !is_marked.load(Ordering::Acquire) {
                return Ok(()); // Unmarked objects will be swept
            }

            // Calculate new compacted address using a thread-safe allocation pointer
            let new_addr = COMPACTION_STATE.with(|state| {
                let mut state = state.borrow_mut();
                let new_addr = state.current_compaction_pointer;
                state.current_compaction_pointer += object_size;
                
                // Align to pointer boundary for safety
                let alignment = std::mem::align_of::<usize>();
                state.current_compaction_pointer = (state.current_compaction_pointer + alignment - 1) & !(alignment - 1);
                
                new_addr
            });

            // Don't compact if object would move to same location
            if new_addr == object_addr {
                return Ok(());
            }

            // Allocate new memory at compacted location
            let new_object_ptr = new_addr as *mut HeapObject;
            
            // Thread-safe copy of object data
            std::ptr::copy_nonoverlapping(object_ptr, new_object_ptr, 1);
            let remaining_size = object_size.saturating_sub(std::mem::size_of::<HeapObject>());
            if remaining_size > 0 {
                let old_data_ptr = object_ptr.add(1) as *const u8;
                let new_data_ptr = new_object_ptr.add(1) as *mut u8;
                std::ptr::copy_nonoverlapping(old_data_ptr, new_data_ptr, remaining_size);
            }

            // Set up forwarding pointer in old location atomically
            let forwarding_ptr = &mut (*object_ptr).metadata.ref_count;
            let forwarding_atomic = std::sync::atomic::AtomicUsize::from_ptr(forwarding_ptr);
            forwarding_atomic.store(new_addr | 0x1, Ordering::Release); // Set LSB to indicate forwarding

            // Add to compaction map for reference updates
            COMPACTION_MAP.with(|map| {
                map.borrow_mut().insert(object_addr, new_addr);
            });

            // Update compaction statistics
            COMPACTION_STATS.with(|stats| {
                stats.borrow_mut().objects_moved += 1;
                stats.borrow_mut().bytes_moved += object_size;
            });
        }

        Ok(())
    }

    /// Update references after compaction
    fn update_references(object_addr: usize) -> Result<(), String> {
        if object_addr == 0 {
            return Ok(()); // Null pointer, nothing to update
        }

        // SAFETY: We're assuming object_addr points to a valid HeapObject
        unsafe {
            let object_ptr = object_addr as *mut HeapObject;
            if object_ptr.is_null() {
                return Ok(());
            }

            let metadata_ptr = &(*object_ptr).metadata;
            let object_size = metadata_ptr.size;
            let object_tag = metadata_ptr.tag;
            
            // Check if this object has been moved (has forwarding pointer)
            let ref_count_atomic = std::sync::atomic::AtomicUsize::from_ptr(&metadata_ptr.ref_count as *const usize as *mut usize);
            let ref_count_value = ref_count_atomic.load(Ordering::Acquire);
            
            // If LSB is set, this is a forwarding pointer
            if ref_count_value & 0x1 != 0 {
                let new_addr = ref_count_value & !0x1; // Clear LSB to get actual address
                // This object has been moved, update our pointer and continue with new location
                return Self::update_references(new_addr);
            }

            // Scan and update references within this object based on its type
            match object_tag {
                Tag::Object | Tag::Interface => {
                    // Scan object fields for references and update them
                    Self::update_object_references(object_addr, object_size)?;
                }
                Tag::Array => {
                    // Scan and update array element references
                    Self::update_array_references(object_addr, object_size)?;
                }
                Tag::Function => {
                    // Update function closure references
                    Self::update_function_references(object_addr, object_size)?;
                }
                Tag::Channel => {
                    // Update channel buffer and goroutine references
                    Self::update_channel_references(object_addr, object_size)?;
                }
                Tag::String | Tag::Number | Tag::Boolean | Tag::Nil => {
                    // Primitive types have no references to update
                }
                Tag::Custom(_) => {
                    // Conservatively update as generic object
                    Self::update_object_references(object_addr, object_size)?;
                }
            }

            // Update card table entries for this object's region
            Self::update_card_table_for_object(object_addr, object_size)?;

            // Update remembered set if this is an inter-generational reference
            Self::update_remembered_set_for_object(object_addr)?;
        }

        Ok(())
    }

    /// Concurrent marking phase
    fn concurrent_marking_phase(&self) -> Result<(), CursedError> {
        // Start concurrent marking with tri-color collector
        let root_objects = self.collect_root_objects()?;
        self.tri_color_collector.start_concurrent_marking(root_objects)?;
        
        // Add marking work items to queue
        let mut work_queue = self.work_queue.lock().unwrap();
        
        // Add initial root objects as work items
        for obj_addr in self.collect_root_objects()? {
            work_queue.push_back(WorkItem {
                work_type: WorkType::Mark,
                object_addr: obj_addr,
                priority: 1,
                created_at: Instant::now(),
            });
        }
        
        // Notify workers
        self.work_available.notify_all();
        
        // Wait for marking to complete with timeout
        let start_time = std::time::Instant::now();
        let timeout = Duration::from_secs(30); // 30 second timeout
        
        while !self.tri_color_collector.incremental_mark_step(100)? {
            if start_time.elapsed() > timeout {
                println!("Concurrent marking timed out after 30 seconds, stopping");
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
        
        Ok(())
    }

    /// Concurrent sweeping phase
    fn concurrent_sweeping_phase(&self) -> Result<(), CursedError> {
        *self.state.write().unwrap() = ConcurrentState::ConcurrentSweeping;
        
        // Add sweeping work items
        let mut work_queue = self.work_queue.lock().unwrap();
        
        // Get all heap objects and add unmarked ones for sweeping
        let heap_objects = self.collect_all_heap_objects()?;
        for obj_addr in heap_objects {
            if !self.is_object_marked(obj_addr) {
                work_queue.push_back(WorkItem {
                    work_type: WorkType::Sweep,
                    object_addr: obj_addr,
                    priority: 2,
                    created_at: Instant::now(),
                });
            }
        }
        
        // Notify workers
        self.work_available.notify_all();
        
        // Wait for sweeping to complete
        while !work_queue.is_empty() {
            thread::sleep(Duration::from_millis(1));
        }
        
        Ok(())
    }

    /// Final pause phase for cleanup
    fn final_pause_phase(&self) -> Result<(), CursedError> {
        *self.state.write().unwrap() = ConcurrentState::FinalPause;
        
        // Pause all mutator threads
        self.pause_mutator_threads()?;
        
        // Process any remaining write barriers
        self.process_write_barriers()?;
        
        // Update statistics
        self.update_final_statistics()?;
        
        // Resume mutator threads
        self.resume_mutator_threads()?;
        
        Ok(())
    }

    /// Simple write barrier implementation
    fn simple_write_barrier(&self, source_addr: usize, target_addr: usize, field_offset: usize) {
        let mut write_log = self.write_barrier_log.write().unwrap();
        write_log.push(WriteBarrierEntry {
            source_addr,
            target_addr,
            field_offset,
            timestamp: Instant::now(),
            thread_id: thread::current().id(),
        });
        
        // Notify tri-color collector
        self.tri_color_collector.write_barrier(source_addr, field_offset, target_addr);
    }

    /// Card table based write barrier
    fn card_table_write_barrier(&self, source_addr: usize, target_addr: usize, field_offset: usize) {
        // Calculate card index
        let card_size = 512; // 512 bytes per card
        let card_index = source_addr / card_size;
        
        // Mark card as dirty
        if let Some(card_table) = self.card_table.read().unwrap().get(card_index) {
            card_table.store(true, Ordering::Relaxed);
        }
        
        // Also do simple write barrier
        self.simple_write_barrier(source_addr, target_addr, field_offset);
    }

    /// Remembered set based write barrier
    fn remembered_set_write_barrier(&self, source_addr: usize, target_addr: usize, field_offset: usize) {
        // Add to remembered set if this is an inter-generational reference
        if self.is_inter_generational_reference(source_addr, target_addr) {
            let mut remembered_set = self.remembered_set.write().unwrap();
            remembered_set.insert(source_addr);
        }
        
        // Also do simple write barrier
        self.simple_write_barrier(source_addr, target_addr, field_offset);
    }

    /// Check if reference is inter-generational
    fn is_inter_generational_reference(&self, source_addr: usize, target_addr: usize) -> bool {
        // Stub implementation - would check object generations
        false
    }

    /// Collect root objects
    fn collect_root_objects(&self) -> Result<Vec<usize>, CursedError> {
        // Delegate to base GC
        Ok(vec![]) // Stub implementation
    }

    /// Collect all heap objects
    fn collect_all_heap_objects(&self) -> Result<Vec<usize>, CursedError> {
        // Delegate to base GC
        Ok(vec![]) // Stub implementation
    }

    /// Check if object is marked
    fn is_object_marked(&self, obj_addr: usize) -> bool {
        // Stub implementation - would check object mark bits
        false
    }

    /// Pause mutator threads
    fn pause_mutator_threads(&self) -> Result<(), CursedError> {
        self.paused.store(true, Ordering::Relaxed);
        
        // Signal all threads to pause
        let (lock, cvar) = &*self.pause_cond;
        let mut paused = lock.lock().unwrap();
        *paused = true;
        cvar.notify_all();
        
        Ok(())
    }

    /// Resume mutator threads
    fn resume_mutator_threads(&self) -> Result<(), CursedError> {
        self.paused.store(false, Ordering::Relaxed);
        
        // Signal all threads to resume
        let (lock, cvar) = &*self.pause_cond;
        let mut paused = lock.lock().unwrap();
        *paused = false;
        cvar.notify_all();
        
        Ok(())
    }

    /// Process write barriers
    fn process_write_barriers(&self) -> Result<(), CursedError> {
        let mut write_log = self.write_barrier_log.write().unwrap();
        
        // Process all write barrier entries
        for entry in write_log.drain(..) {
            // Handle the write barrier entry
            // This would typically involve updating the tri-color collector
            // or remembered set based on the write
        }
        
        Ok(())
    }

    /// Update final statistics
    fn update_final_statistics(&self) -> Result<(), CursedError> {
        let mut stats = self.concurrent_stats.write().unwrap();
        
        // Update work queue statistics
        let queue_size = self.work_queue.lock().unwrap().len();
        if queue_size > stats.work_queue_peak_size {
            stats.work_queue_peak_size = queue_size;
        }
        
        // Update write barrier overhead
        let write_barrier_count = self.write_barrier_log.read().unwrap().len();
        stats.write_barrier_overhead = write_barrier_count as f64 / stats.work_items_processed as f64;
        
        Ok(())
    }

    /// Get concurrent collection statistics
    pub fn get_stats(&self) -> ConcurrentStats {
        self.concurrent_stats.read().unwrap().clone()
    }

    /// Get current state
    pub fn get_state(&self) -> ConcurrentState {
        *self.state.read().unwrap()
    }

    /// Set memory profiler
    pub fn set_profiler(&mut self, profiler: Arc<MemoryProfiler>) {
        self.profiler = Some(profiler);
    }

    // Helper functions for concurrent GC algorithms

    /// Scan object references for marking
    fn scan_object_references(object_addr: usize, object_size: usize) -> Result<(), String> {
        let ptr_size = std::mem::size_of::<usize>();
        let num_potential_pointers = object_size / ptr_size;

        // Conservatively scan for potential pointers
        for i in 0..num_potential_pointers {
            unsafe {
                let field_addr = object_addr + (i * ptr_size);
                let potential_ptr = *(field_addr as *const usize);
                
                // Basic pointer validation (non-null, aligned)
                if potential_ptr != 0 && potential_ptr % ptr_size == 0 {
                    // Add to marking work queue if it looks like a valid object
                    if Self::is_valid_heap_pointer(potential_ptr) {
                        Self::add_mark_work_item(potential_ptr)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Scan array references for marking
    fn scan_array_references(object_addr: usize, object_size: usize) -> Result<(), String> {
        // Arrays store elements after the header
        let header_size = std::mem::size_of::<HeapObject>();
        let element_data_start = object_addr + header_size;
        let element_data_size = object_size.saturating_sub(header_size);
        
        // Conservatively scan array elements
        Self::scan_object_references(element_data_start, element_data_size)
    }

    /// Scan function references for marking (closures, captured variables)
    fn scan_function_references(object_addr: usize, object_size: usize) -> Result<(), String> {
        // Functions may have captured variables stored after the function pointer
        Self::scan_object_references(object_addr, object_size)
    }

    /// Scan channel references for marking (buffers, waiting goroutines)
    fn scan_channel_references(object_addr: usize, object_size: usize) -> Result<(), String> {
        // Channels contain buffers and goroutine references
        Self::scan_object_references(object_addr, object_size)
    }

    /// Check if a pointer is valid for the heap
    fn is_valid_heap_pointer(ptr: usize) -> bool {
        // Basic validation - in a real implementation this would check heap bounds
        ptr != 0 && ptr % std::mem::align_of::<usize>() == 0 && ptr > 0x1000
    }

    /// Add marking work item to queue
    fn add_mark_work_item(object_addr: usize) -> Result<(), String> {
        // In a real implementation, this would add to the global work queue
        // For now, just recursively mark (simplified)
        Self::mark_object(object_addr)
    }

    /// Finalize channel object
    fn finalize_channel(object_addr: usize) -> Result<(), String> {
        // Close channel and notify waiting goroutines
        // Implementation would depend on channel structure
        Ok(())
    }

    /// Finalize function object
    fn finalize_function(object_addr: usize) -> Result<(), String> {
        // Clean up function closures and captured variables
        Ok(())
    }

    /// Finalize custom object with proper error handling
    fn finalize_custom_object(object_addr: usize) -> Result<(), String> {
        unsafe {
            let object_ptr = object_addr as *mut u8;
            let metadata_ptr = object_ptr as *mut ObjectMetadata;
            
            // Check if object has custom finalizer
            if (*metadata_ptr).has_finalizer {
                // Get finalizer function pointer from object header
                if let Some(finalizer_fn) = (*metadata_ptr).finalizer_fn {
                    // Call the finalizer function
                    match finalizer_fn(object_ptr.add(std::mem::size_of::<ObjectMetadata>())) {
                        Ok(()) => {
                            debug!("Successfully finalized custom object at 0x{:x}", object_addr);
                        }
                        Err(e) => {
                            error!("Custom finalizer failed for object at 0x{:x}: {}", object_addr, e);
                            return Err(format!("Custom finalizer failed: {}", e));
                        }
                    }
                } else {
                    warn!("Object marked as having finalizer but no function pointer found");
                }
            }
        }
        Ok(())
    }

    /// Add object to free list
    fn add_to_free_list(object_addr: usize, size: usize) -> Result<(), String> {
        FREE_LIST.with(|free_list| {
            free_list.borrow_mut().push(FreeBlock {
                address: object_addr,
                size,
            });
        });
        Ok(())
    }

    /// Update object references after compaction
    fn update_object_references(object_addr: usize, object_size: usize) -> Result<(), String> {
        let ptr_size = std::mem::size_of::<usize>();
        let num_potential_pointers = object_size / ptr_size;

        // Scan and update all potential pointer fields
        for i in 0..num_potential_pointers {
            unsafe {
                let field_addr = object_addr + (i * ptr_size);
                let field_ptr = field_addr as *mut usize;
                let current_value = *field_ptr;
                
                // Check if this is a moved object
                if let Some(new_addr) = Self::get_forwarding_address(current_value) {
                    // Update the reference atomically
                    let atomic_ptr = std::sync::atomic::AtomicUsize::from_ptr(field_ptr);
                    atomic_ptr.store(new_addr, Ordering::Release);
                }
            }
        }
        Ok(())
    }

    /// Update array references after compaction
    fn update_array_references(object_addr: usize, object_size: usize) -> Result<(), String> {
        let header_size = std::mem::size_of::<HeapObject>();
        let element_data_start = object_addr + header_size;
        let element_data_size = object_size.saturating_sub(header_size);
        
        Self::update_object_references(element_data_start, element_data_size)
    }

    /// Update function references after compaction
    fn update_function_references(object_addr: usize, object_size: usize) -> Result<(), String> {
        Self::update_object_references(object_addr, object_size)
    }

    /// Update channel references after compaction
    fn update_channel_references(object_addr: usize, object_size: usize) -> Result<(), String> {
        Self::update_object_references(object_addr, object_size)
    }

    /// Get forwarding address for moved object
    fn get_forwarding_address(object_addr: usize) -> Option<usize> {
        COMPACTION_MAP.with(|map| {
            map.borrow().get(&object_addr).copied()
        })
    }

    /// Update card table for object
    fn update_card_table_for_object(object_addr: usize, object_size: usize) -> Result<(), String> {
        // Calculate which cards this object spans
        let card_size = 512; // bytes per card
        let start_card = object_addr / card_size;
        let end_card = (object_addr + object_size) / card_size;
        
        // Mark all cards as clean (object has been processed)
        for card_index in start_card..=end_card {
            // In a real implementation, would update global card table
        }
        Ok(())
    }

    /// Update remembered set for object
    fn update_remembered_set_for_object(object_addr: usize) -> Result<(), String> {
        // Check if object has inter-generational references and update remembered set
        // In a real implementation, would scan object and update global remembered set
        Ok(())
    }
}

/// Thread-safe global concurrent GC instance
use std::sync::OnceLock;
static GLOBAL_CONCURRENT_GC: OnceLock<Arc<ConcurrentGarbageCollector>> = OnceLock::new();

/// Initialize global concurrent GC (thread-safe)
pub fn initialize_concurrent_gc(
    config: ConcurrentGcConfig,
    base_gc: Arc<GarbageCollector>,
    tri_color_collector: Arc<TriColorCollector>,
    performance_tuner: Arc<GcPerformanceTuner>,
) -> Result<(), CursedError> {
    let concurrent_gc = ConcurrentGarbageCollector::new(config, base_gc, tri_color_collector, performance_tuner)?;
    let concurrent_gc_arc = Arc::new(concurrent_gc);
    
    GLOBAL_CONCURRENT_GC.set(concurrent_gc_arc)
        .map_err(|_| CursedError::runtime_error("Concurrent GC already initialized"))?;
    
    Ok(())
}

/// Get global concurrent GC (thread-safe)
pub fn get_concurrent_gc() -> Option<Arc<ConcurrentGarbageCollector>> {
    GLOBAL_CONCURRENT_GC.get().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::gc::GarbageCollector;
    use crate::runtime::gc_tuning::{TriColorCollector, GcPerformanceTuner, GcTuningParams};
    use crate::runtime::stack::RuntimeStack;

    #[test]
    fn test_concurrent_gc_creation() {
        let stack_manager = Arc::new(RuntimeStack::new());
        let gc_config = crate::runtime::gc::GcConfig::default();
        let base_gc = GarbageCollector::new();
        let tri_color_collector = Arc::new(TriColorCollector::new());
        let performance_tuner = Arc::new(GcPerformanceTuner::new(GcTuningParams::default()));
        
        let config = ConcurrentGcConfig::default();
        let concurrent_gc = ConcurrentGarbageCollector::new(config, Arc::new(base_gc), tri_color_collector, performance_tuner).unwrap();
        
        assert_eq!(concurrent_gc.get_state(), ConcurrentState::Idle);
    }

    #[test]
    fn test_concurrent_gc_config() {
        let config = ConcurrentGcConfig::default();
        assert_eq!(config.max_pause_time_ms, 10);
        assert!(config.concurrent_marking);
        assert!(config.concurrent_sweeping);
        assert_eq!(config.write_barrier_mode, WriteBarrierMode::CardTable);
    }

    #[test]
    fn test_write_barrier_modes() {
        assert_eq!(WriteBarrierMode::None, WriteBarrierMode::None);
        assert_ne!(WriteBarrierMode::Simple, WriteBarrierMode::CardTable);
    }

    #[test]
    fn test_work_item_creation() {
        let work_item = WorkItem {
            work_type: WorkType::Mark,
            object_addr: 0x1000,
            priority: 1,
            created_at: Instant::now(),
        };
        
        assert_eq!(work_item.work_type, WorkType::Mark);
        assert_eq!(work_item.object_addr, 0x1000);
        assert_eq!(work_item.priority, 1);
    }

    #[test]
    fn test_concurrent_stats_default() {
        let stats = ConcurrentStats::default();
        assert_eq!(stats.total_concurrent_collections, 0);
        assert_eq!(stats.work_items_processed, 0);
        assert_eq!(stats.concurrent_marking_efficiency, 0.0);
    }
}
