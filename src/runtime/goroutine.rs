/// Goroutine runtime system for CURSED
///
/// Provides cooperative goroutine scheduling with GC integration, stack management,
/// and safe points for concurrent collection.

use crate::error::Error;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::thread::{self, ThreadId, JoinHandle};
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::sync::OnceLock;

// Mark stack as Send for threading (unsafe but necessary for scheduler)
unsafe impl Send for GoroutineStack {}
unsafe impl Sync for GoroutineStack {}

/// Global goroutine ID counter
static GOROUTINE_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Global goroutine scheduler instance
static GLOBAL_SCHEDULER: OnceLock<Arc<Mutex<GoroutineScheduler>>> = OnceLock::new();

/// Generate a unique goroutine ID
fn next_goroutine_id() -> u64 {
    GOROUTINE_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Goroutine state enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GoroutineState {
    /// Just created, not yet scheduled
    Created,
    /// Ready to run, in scheduler queue
    Ready,
    /// Currently executing
    Running,
    /// Waiting for I/O or synchronization
    Waiting,
    /// Yielded voluntarily, can be rescheduled
    Yielded,
    /// Completed normally
    Completed,
    /// Terminated due to panic or error
    Terminated,
}

/// Stack memory region for goroutine
#[derive(Debug)]
pub struct GoroutineStack {
    /// Base pointer of stack memory
    base: NonNull<u8>,
    /// Size of allocated stack
    size: usize,
    /// Current stack pointer offset
    sp_offset: usize,
    /// Stack growth direction (true = grows down)
    grows_down: bool,
}

impl GoroutineStack {
    /// Create a new stack with specified size
    pub fn new(size: usize) -> Result<Self, Error> {
        let layout = Layout::from_size_align(size, 16)
            .map_err(|_| Error::Runtime("Invalid stack layout".to_string()))?;
        
        let ptr = unsafe { alloc(layout) };
        let base = NonNull::new(ptr)
            .ok_or_else(|| Error::Runtime("Failed to allocate stack memory".to_string()))?;
        
        Ok(GoroutineStack {
            base,
            size,
            sp_offset: if cfg!(target_arch = "x86_64") { size } else { 0 },
            grows_down: cfg!(target_arch = "x86_64"),
        })
    }
    
    /// Get stack base address
    pub fn base_addr(&self) -> *mut u8 {
        self.base.as_ptr()
    }
    
    /// Get current stack pointer
    pub fn stack_pointer(&self) -> *mut u8 {
        unsafe {
            if self.grows_down {
                self.base.as_ptr().add(self.sp_offset)
            } else {
                self.base.as_ptr().add(self.sp_offset)
            }
        }
    }
    
    /// Get stack bounds for GC scanning
    pub fn bounds(&self) -> (*mut u8, *mut u8) {
        let start = self.base.as_ptr();
        let end = unsafe { start.add(self.size) };
        (start, end)
    }
    
    /// Check if address is within stack bounds
    pub fn contains(&self, addr: *const u8) -> bool {
        let (start, end) = self.bounds();
        addr >= start as *const u8 && addr < end as *const u8
    }
}

impl Drop for GoroutineStack {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.size, 16).unwrap();
        unsafe {
            dealloc(self.base.as_ptr(), layout);
        }
    }
}

/// Safe points for GC coordination
#[derive(Debug, Clone)]
pub struct SafePoint {
    /// Location identifier (function name, line number, etc.)
    pub location: String,
    /// Timestamp when safe point was reached
    pub timestamp: Instant,
    /// Whether GC can run at this point
    pub gc_safe: bool,
}

/// Goroutine context for scheduling and execution
pub struct Goroutine {
    /// Unique goroutine identifier
    pub id: u64,
    /// Current state
    pub state: GoroutineState,
    /// Goroutine stack
    pub stack: GoroutineStack,
    /// Parent goroutine ID (if spawned from another goroutine)
    pub parent_id: Option<u64>,
    /// Creation timestamp
    pub created_at: Instant,
    /// Last execution time
    pub last_run: Option<Instant>,
    /// Total execution time
    pub total_runtime: Duration,
    /// Safe points encountered
    pub safe_points: Vec<SafePoint>,
    /// GC roots local to this goroutine
    pub local_roots: Vec<usize>, // Object IDs
    /// Function to execute
    pub task: Option<Box<dyn FnOnce() + Send>>,
    /// Join handle for actual thread (if detached)
    pub join_handle: Option<JoinHandle<()>>,
}

impl Goroutine {
    /// Create a new goroutine
    pub fn new<F>(task: F, stack_size: usize) -> Result<Self, Error>
    where
        F: FnOnce() + Send + 'static,
    {
        let id = next_goroutine_id();
        let stack = GoroutineStack::new(stack_size)?;
        
        Ok(Goroutine {
            id,
            state: GoroutineState::Created,
            stack,
            parent_id: None,
            created_at: Instant::now(),
            last_run: None,
            total_runtime: Duration::ZERO,
            safe_points: Vec::new(),
            local_roots: Vec::new(),
            task: Some(Box::new(task)),
            join_handle: None,
        })
    }
    
    /// Add a safe point for GC coordination
    pub fn add_safe_point(&mut self, location: String, gc_safe: bool) {
        self.safe_points.push(SafePoint {
            location,
            timestamp: Instant::now(),
            gc_safe,
        });
    }
    
    /// Check if goroutine is at a GC-safe point
    pub fn is_gc_safe(&self) -> bool {
        self.safe_points.last()
            .map(|sp| sp.gc_safe)
            .unwrap_or(false)
    }
    
    /// Add a local GC root
    pub fn add_local_root(&mut self, object_id: usize) {
        if !self.local_roots.contains(&object_id) {
            self.local_roots.push(object_id);
        }
    }
    
    /// Remove a local GC root
    pub fn remove_local_root(&mut self, object_id: usize) {
        self.local_roots.retain(|&id| id != object_id);
    }
    
    /// Execute the goroutine task
    pub fn execute(&mut self) -> Result<(), Error> {
        if let Some(task) = self.task.take() {
            self.state = GoroutineState::Running;
            self.last_run = Some(Instant::now());
            
            // Add safe point at function entry
            self.add_safe_point("function_entry".to_string(), true);
            
            let start_time = Instant::now();
            
            // Execute the task
            task();
            
            let execution_time = start_time.elapsed();
            self.total_runtime += execution_time;
            
            // Add safe point at function exit
            self.add_safe_point("function_exit".to_string(), true);
            
            self.state = GoroutineState::Completed;
            Ok(())
        } else {
            Err(Error::Runtime("Goroutine task already executed".to_string()))
        }
    }
}

impl std::fmt::Debug for Goroutine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Goroutine")
            .field("id", &self.id)
            .field("state", &self.state)
            .field("parent_id", &self.parent_id)
            .field("created_at", &self.created_at)
            .field("total_runtime", &self.total_runtime)
            .field("safe_points_count", &self.safe_points.len())
            .field("local_roots_count", &self.local_roots.len())
            .field("has_task", &self.task.is_some())
            .finish()
    }
}

/// Work-stealing scheduler for goroutines
#[derive(Debug)]
pub struct GoroutineScheduler {
    /// Ready queue for goroutines
    ready_queue: Arc<Mutex<VecDeque<Arc<Mutex<Goroutine>>>>>,
    /// All goroutines (including completed ones)
    goroutines: Arc<RwLock<HashMap<u64, Arc<Mutex<Goroutine>>>>>,
    /// Running goroutines
    running: Arc<RwLock<HashMap<ThreadId, u64>>>,
    /// Scheduler configuration
    config: SchedulerConfig,
    /// GC coordination
    gc_coordinator: Arc<GcCoordinator>,
    /// Scheduler state
    is_running: Arc<AtomicBool>,
    /// Worker threads
    workers: Vec<JoinHandle<()>>,
}

/// Scheduler configuration
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// Number of worker threads
    pub worker_threads: usize,
    /// Default stack size for goroutines
    pub default_stack_size: usize,
    /// Time slice for preemptive scheduling
    pub time_slice: Duration,
    /// Enable work stealing
    pub work_stealing: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            worker_threads: num_cpus::get().max(2),
            default_stack_size: 64 * 1024, // 64KB
            time_slice: Duration::from_millis(10),
            work_stealing: true,
        }
    }
}

/// GC coordination for safe collection
#[derive(Debug)]
pub struct GcCoordinator {
    /// Safe point synchronization
    safe_point_barrier: Arc<(Mutex<usize>, Condvar)>,
    /// Expected number of goroutines at safe point
    expected_goroutines: Arc<AtomicU64>,
    /// Goroutines currently at safe points
    safe_goroutines: Arc<AtomicU64>,
    /// GC request flag
    gc_requested: Arc<AtomicBool>,
}

impl GcCoordinator {
    pub fn new() -> Self {
        Self {
            safe_point_barrier: Arc::new((Mutex::new(0), Condvar::new())),
            expected_goroutines: Arc::new(AtomicU64::new(0)),
            safe_goroutines: Arc::new(AtomicU64::new(0)),
            gc_requested: Arc::new(AtomicBool::new(false)),
        }
    }
    
    /// Request GC coordination from all goroutines
    pub fn request_gc_coordination(&self, num_goroutines: u64) {
        self.expected_goroutines.store(num_goroutines, Ordering::SeqCst);
        self.safe_goroutines.store(0, Ordering::SeqCst);
        self.gc_requested.store(true, Ordering::SeqCst);
    }
    
    /// Wait for all goroutines to reach safe points
    pub fn wait_for_safe_points(&self, timeout: Duration) -> bool {
        let start_time = Instant::now();
        
        while start_time.elapsed() < timeout {
            let expected = self.expected_goroutines.load(Ordering::SeqCst);
            let safe = self.safe_goroutines.load(Ordering::SeqCst);
            
            if safe >= expected {
                return true;
            }
            
            thread::sleep(Duration::from_millis(1));
        }
        
        false
    }
    
    /// Signal that a goroutine has reached a safe point
    pub fn signal_safe_point(&self) {
        self.safe_goroutines.fetch_add(1, Ordering::SeqCst);
    }
    
    /// Complete GC coordination
    pub fn complete_gc_coordination(&self) {
        self.gc_requested.store(false, Ordering::SeqCst);
        
        // Wake up any waiting goroutines
        let (lock, cvar) = &*self.safe_point_barrier;
        let _guard = lock.lock().unwrap();
        cvar.notify_all();
    }
    
    /// Check if GC coordination is requested
    pub fn is_gc_requested(&self) -> bool {
        self.gc_requested.load(Ordering::SeqCst)
    }
}

impl GoroutineScheduler {
    /// Create a new scheduler with default configuration
    pub fn new() -> Self {
        Self::with_config(SchedulerConfig::default())
    }
    
    /// Create a new scheduler with custom configuration
    pub fn with_config(config: SchedulerConfig) -> Self {
        Self {
            ready_queue: Arc::new(Mutex::new(VecDeque::new())),
            goroutines: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(HashMap::new())),
            config,
            gc_coordinator: Arc::new(GcCoordinator::new()),
            is_running: Arc::new(AtomicBool::new(false)),
            workers: Vec::new(),
        }
    }
    
    /// Start the scheduler with worker threads
    pub fn start(&mut self) -> Result<(), Error> {
        if self.is_running.load(Ordering::SeqCst) {
            return Err(Error::Runtime("Scheduler already running".to_string()));
        }
        
        self.is_running.store(true, Ordering::SeqCst);
        
        // Start worker threads
        for worker_id in 0..self.config.worker_threads {
            let ready_queue = Arc::clone(&self.ready_queue);
            let goroutines = Arc::clone(&self.goroutines);
            let running = Arc::clone(&self.running);
            let is_running = Arc::clone(&self.is_running);
            let gc_coordinator = Arc::clone(&self.gc_coordinator);
            
            let handle = thread::Builder::new()
                .name(format!("cursed-worker-{}", worker_id))
                .spawn(move || {
                    Self::worker_loop(ready_queue, goroutines, running, is_running, gc_coordinator);
                })
                .map_err(|e| Error::Runtime(format!("Failed to spawn worker thread: {}", e)))?;
            
            self.workers.push(handle);
        }
        
        Ok(())
    }
    
    /// Stop the scheduler and wait for all workers to finish
    pub fn stop(&mut self) -> Result<(), Error> {
        self.is_running.store(false, Ordering::SeqCst);
        
        // Wait for all workers to finish
        for handle in self.workers.drain(..) {
            handle.join()
                .map_err(|_| Error::Runtime("Failed to join worker thread".to_string()))?;
        }
        
        Ok(())
    }
    
    /// Spawn a new goroutine
    pub fn spawn<F>(&mut self, task: F) -> Result<u64, Error>
    where
        F: FnOnce() + Send + 'static,
    {
        let goroutine = Arc::new(Mutex::new(
            Goroutine::new(task, self.config.default_stack_size)?
        ));
        
        let goroutine_id = {
            let g = goroutine.lock().unwrap();
            g.id
        };
        
        // Add to goroutines registry
        {
            let mut goroutines = self.goroutines.write().unwrap();
            goroutines.insert(goroutine_id, Arc::clone(&goroutine));
        }
        
        // Add to ready queue
        {
            let mut ready_queue = self.ready_queue.lock().unwrap();
            ready_queue.push_back(goroutine);
        }
        
        Ok(goroutine_id)
    }
    
    /// Yield control from current goroutine
    pub fn yield_current(&self) -> Result<(), Error> {
        let current_thread = thread::current().id();
        
        // Find current goroutine
        if let Some(goroutine_id) = {
            let running = self.running.read().unwrap();
            running.get(&current_thread).copied()
        } {
            if let Some(goroutine) = {
                let goroutines = self.goroutines.read().unwrap();
                goroutines.get(&goroutine_id).cloned()
            } {
                let mut g = goroutine.lock().unwrap();
                g.state = GoroutineState::Yielded;
                g.add_safe_point("yield_point".to_string(), true);
                
                // Check for GC coordination
                if self.gc_coordinator.is_gc_requested() {
                    self.gc_coordinator.signal_safe_point();
                    
                    // Wait for GC to complete
                    let (lock, cvar) = &*self.gc_coordinator.safe_point_barrier;
                    let _guard = cvar.wait(lock.lock().unwrap()).unwrap();
                }
            }
        }
        
        Ok(())
    }
    
    /// Get goroutine information
    pub fn get_goroutine_info(&self, goroutine_id: u64) -> Option<(GoroutineState, Duration, usize)> {
        let goroutines = self.goroutines.read().unwrap();
        goroutines.get(&goroutine_id).and_then(|g| {
            let goroutine = g.lock().unwrap();
            Some((
                goroutine.state,
                goroutine.total_runtime,
                goroutine.safe_points.len(),
            ))
        })
    }
    
    /// Get all active goroutine IDs
    pub fn active_goroutines(&self) -> Vec<u64> {
        let goroutines = self.goroutines.read().unwrap();
        goroutines.keys().copied().collect()
    }
    
    /// Request GC coordination from all active goroutines
    pub fn coordinate_gc(&self, timeout: Duration) -> bool {
        let active_count = {
            let goroutines = self.goroutines.read().unwrap();
            goroutines.len() as u64
        };
        
        if active_count == 0 {
            return true;
        }
        
        self.gc_coordinator.request_gc_coordination(active_count);
        let success = self.gc_coordinator.wait_for_safe_points(timeout);
        self.gc_coordinator.complete_gc_coordination();
        
        success
    }
    
    /// Get stack bounds for all goroutines (for GC scanning)
    pub fn get_stack_bounds(&self) -> Vec<(*mut u8, *mut u8)> {
        let goroutines = self.goroutines.read().unwrap();
        goroutines.values()
            .filter_map(|g| {
                if let Ok(goroutine) = g.lock() {
                    Some(goroutine.stack.bounds())
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Check if the scheduler is currently running
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }
    
    /// Worker thread main loop
    fn worker_loop(
        ready_queue: Arc<Mutex<VecDeque<Arc<Mutex<Goroutine>>>>>,
        goroutines: Arc<RwLock<HashMap<u64, Arc<Mutex<Goroutine>>>>>,
        running: Arc<RwLock<HashMap<ThreadId, u64>>>,
        is_running: Arc<AtomicBool>,
        gc_coordinator: Arc<GcCoordinator>,
    ) {
        let thread_id = thread::current().id();
        
        while is_running.load(Ordering::SeqCst) {
            // Try to get a goroutine from the ready queue
            if let Some(goroutine) = {
                let mut queue = ready_queue.lock().unwrap();
                queue.pop_front()
            } {
                let goroutine_id = {
                    let g = goroutine.lock().unwrap();
                    g.id
                };
                
                // Register as running goroutine
                {
                    let mut running_map = running.write().unwrap();
                    running_map.insert(thread_id, goroutine_id);
                }
                
                // Execute the goroutine
                let result = {
                    let mut g = goroutine.lock().unwrap();
                    g.execute()
                };
                
                // Handle execution result
                match result {
                    Ok(()) => {
                        // Goroutine completed successfully
                        let mut g = goroutine.lock().unwrap();
                        g.add_safe_point("completion".to_string(), true);
                    }
                    Err(_e) => {
                        // Goroutine terminated with error
                        let mut g = goroutine.lock().unwrap();
                        g.state = GoroutineState::Terminated;
                        g.add_safe_point("error".to_string(), true);
                    }
                }
                
                // Unregister from running goroutines
                {
                    let mut running_map = running.write().unwrap();
                    running_map.remove(&thread_id);
                }
            } else {
                // No goroutines available, sleep briefly
                thread::sleep(Duration::from_millis(1));
                
                // Check if GC coordination is requested
                if gc_coordinator.is_gc_requested() {
                    gc_coordinator.signal_safe_point();
                }
            }
        }
    }
}

impl Default for GoroutineScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for GoroutineScheduler {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

// FFI functions for LLVM integration

/// Spawn a goroutine from compiled CURSED code
/// 
/// # Arguments
/// * `scheduler_ptr` - Pointer to the GoroutineScheduler instance
/// * `function_ptr` - Pointer to the function to execute in the goroutine
/// 
/// # Returns
/// * Goroutine ID on success, 0 on failure
/// 
/// # Safety
/// This function is unsafe as it dereferences raw pointers and transmutes function pointers.
/// The caller must ensure that:
/// - `scheduler_ptr` points to a valid, initialized GoroutineScheduler
/// - `function_ptr` points to a valid function with signature `fn()`
/// - Both pointers remain valid for the duration of the call
#[no_mangle]
pub extern "C" fn cursed_spawn_goroutine(
    scheduler_ptr: *mut GoroutineScheduler,
    function_ptr: *const u8,
) -> u64 {
    if scheduler_ptr.is_null() || function_ptr.is_null() {
        tracing::error!("Invalid null pointer passed to cursed_spawn_goroutine");
        return 0;
    }
    
    let scheduler = unsafe { &mut *scheduler_ptr };
    
    // Convert function pointer to a callable closure
    // Note: This is a simplified implementation - in a real system,
    // we would need proper function signature handling and argument passing
    let func_ptr = function_ptr as *const fn();
    
    tracing::debug!("Spawning goroutine with function pointer: {:p}", func_ptr);
    
    match scheduler.spawn(move || {
        if !func_ptr.is_null() {
            unsafe {
                let func: fn() = std::mem::transmute(func_ptr);
                tracing::debug!("Executing goroutine function");
                func();
                tracing::debug!("Goroutine function completed");
            }
        } else {
            tracing::warn!("Attempted to execute null function pointer in goroutine");
        }
    }) {
        Ok(id) => {
            tracing::info!("Successfully spawned goroutine with ID: {}", id);
            id
        },
        Err(e) => {
            tracing::error!("Failed to spawn goroutine: {:?}", e);
            0
        }
    }
}

/// Yield from current goroutine
/// 
/// # Arguments
/// * `scheduler_ptr` - Pointer to the GoroutineScheduler instance
/// 
/// # Safety
/// This function dereferences a raw pointer. The caller must ensure that
/// `scheduler_ptr` points to a valid, initialized GoroutineScheduler.
#[no_mangle]
pub extern "C" fn cursed_yield_goroutine(scheduler_ptr: *mut GoroutineScheduler) {
    if scheduler_ptr.is_null() {
        tracing::error!("Invalid null scheduler pointer passed to cursed_yield_goroutine");
        return;
    }
    
    let scheduler = unsafe { &*scheduler_ptr };
    
    tracing::debug!("Yielding current goroutine");
    if let Err(e) = scheduler.yield_current() {
        tracing::error!("Failed to yield goroutine: {:?}", e);
    }
}

/// Signal a safe point for GC coordination
/// 
/// # Arguments
/// * `scheduler_ptr` - Pointer to the GoroutineScheduler instance
/// * `location` - C string indicating the source location of the safe point
/// 
/// # Safety
/// This function dereferences raw pointers. The caller must ensure that:
/// - `scheduler_ptr` points to a valid, initialized GoroutineScheduler
/// - `location` points to a valid null-terminated C string
#[no_mangle]
pub extern "C" fn cursed_safe_point(
    scheduler_ptr: *mut GoroutineScheduler,
    location: *const std::os::raw::c_char,
) {
    if scheduler_ptr.is_null() {
        tracing::error!("Invalid null scheduler pointer passed to cursed_safe_point");
        return;
    }
    
    let scheduler = unsafe { &*scheduler_ptr };
    
    let location_str = if location.is_null() {
        "unknown".to_string()
    } else {
        unsafe {
            std::ffi::CStr::from_ptr(location)
                .to_string_lossy()
                .into_owned()
        }
    };
    
    tracing::debug!("Signaling safe point at location: {}", location_str);
    
    // Yielding serves as a safe point - allows GC coordination
    if let Err(e) = scheduler.yield_current() {
        tracing::error!("Failed to yield at safe point {}: {:?}", location_str, e);
    }
}

/// Check if GC coordination is requested
/// 
/// # Arguments
/// * `scheduler_ptr` - Pointer to the GoroutineScheduler instance
/// 
/// # Returns
/// * `true` if GC coordination is requested, `false` otherwise
/// 
/// # Safety
/// This function dereferences a raw pointer. The caller must ensure that
/// `scheduler_ptr` points to a valid, initialized GoroutineScheduler.
#[no_mangle]
pub extern "C" fn cursed_gc_requested(scheduler_ptr: *mut GoroutineScheduler) -> bool {
    if scheduler_ptr.is_null() {
        tracing::error!("Invalid null scheduler pointer passed to cursed_gc_requested");
        return false;
    }
    
    let scheduler = unsafe { &*scheduler_ptr };
    let requested = scheduler.gc_coordinator.is_gc_requested();
    
    tracing::trace!("GC coordination requested: {}", requested);
    requested
}

/// Initialize the global goroutine scheduler
pub fn initialize_global_scheduler() -> Result<(), Error> {
    let scheduler = GoroutineScheduler::new();
    GLOBAL_SCHEDULER.set(Arc::new(Mutex::new(scheduler)))
        .map_err(|_| Error::Runtime("Global scheduler already initialized".to_string()))?;
    
    // Start the scheduler
    if let Some(scheduler_ref) = get_global_scheduler() {
        let mut scheduler = scheduler_ref.lock()
            .map_err(|_| Error::Runtime("Failed to lock global scheduler".to_string()))?;
        scheduler.start()?;
    }
    
    Ok(())
}

/// Get the global goroutine scheduler
pub fn get_global_scheduler() -> Option<Arc<Mutex<GoroutineScheduler>>> {
    GLOBAL_SCHEDULER.get().cloned()
}

/// Shutdown the global goroutine scheduler
pub fn shutdown_global_scheduler() -> Result<(), Error> {
    if let Some(scheduler_ref) = get_global_scheduler() {
        let mut scheduler = scheduler_ref.lock()
            .map_err(|_| Error::Runtime("Failed to lock global scheduler".to_string()))?;
        scheduler.stop()?;
    }
    Ok(())
}

// External dependency placeholder - replace with actual crate
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    }
}
