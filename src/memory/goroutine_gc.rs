//! Goroutine-aware garbage collection implementation
//!
//! This module integrates the garbage collector with the goroutine runtime,
//! providing safe collection in concurrent environments.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock, Weak as StdWeak};
use std::thread::{self, ThreadId};
use std::time::{Duration, Instant};
use std::ffi::c_void;
use std::ptr::NonNull;

use tracing::{debug, error, info, trace, warn, instrument};

use crate::memory::{GarbageCollector, Traceable, Visitor, Tag, ThreadSafeGc};
use crate::memory::gc::{MarkState, CollectionTrigger, MemoryStats};
use crate::runtime::goroutine::GoroutineId;

/// Safe point types for GC synchronization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafePointType {
    /// Function entry point
    FunctionEntry,
    /// Function exit point  
    FunctionExit,
    /// Loop back edge
    LoopBackEdge,
    /// Allocation site
    Allocation,
    /// Explicit yield point
    Yield,
    /// Channel operation
    ChannelOp,
}

/// Stack frame information for goroutine stack scanning
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Stack pointer for this frame
    pub stack_ptr: usize,
    /// Base pointer for this frame
    pub base_ptr: usize,
    /// Size of this frame in bytes
    pub frame_size: usize,
    /// Function name (for debugging)
    pub function_name: String,
    /// Local variables that are GC-tracked
    pub gc_roots: Vec<usize>,
}

/// Goroutine state tracked by the GC
#[derive(Debug, Clone)]
pub struct GoroutineGcState {
    /// Goroutine ID
    pub id: GoroutineId,
    /// Thread ID where goroutine is running
    pub thread_id: Option<ThreadId>,
    /// Current stack frames
    pub stack_frames: Vec<StackFrame>,
    /// Local GC roots specific to this goroutine
    pub local_roots: HashSet<usize>,
    /// Objects allocated by this goroutine
    pub allocated_objects: HashSet<usize>,
    /// Whether this goroutine is at a safe point
    pub at_safe_point: bool,
    /// Last safe point type
    pub last_safe_point: Option<SafePointType>,
    /// Goroutine stack base and size
    pub stack_base: usize,
    pub stack_size: usize,
}

/// Global goroutine GC coordinator
pub struct GoroutineGarbageCollector {
    /// Underlying GC implementation
    pub gc: Arc<GarbageCollector>,
    /// Per-goroutine GC state
    pub goroutine_states: Arc<RwLock<HashMap<GoroutineId, GoroutineGcState>>>,
    /// Global safe point coordinator
    pub safe_point_coordinator: Arc<SafePointCoordinator>,
    /// Configuration
    pub config: GoroutineGcConfig,
}

/// Configuration for goroutine-aware GC
#[derive(Debug, Clone)]
pub struct GoroutineGcConfig {
    /// Maximum time to wait for all goroutines to reach safe points (ms)
    pub safe_point_timeout_ms: u64,
    /// Whether to enable incremental collection with goroutines
    pub incremental_enabled: bool,
    /// Maximum number of goroutines to scan per incremental step
    pub max_goroutines_per_step: usize,
    /// Whether to use conservative stack scanning
    pub conservative_stack_scan: bool,
    /// Stack scan chunk size for better performance
    pub stack_scan_chunk_size: usize,
}

impl Default for GoroutineGcConfig {
    fn default() -> Self {
        Self {
            safe_point_timeout_ms: 1000,
            incremental_enabled: true,
            max_goroutines_per_step: 10,
            conservative_stack_scan: true,
            stack_scan_chunk_size: 64 * 1024, // 64KB chunks
        }
    }
}

/// Coordinates safe points across all goroutines
pub struct SafePointCoordinator {
    /// Goroutines waiting at safe points
    waiting_goroutines: Mutex<HashSet<GoroutineId>>,
    /// Whether GC is requesting safe points
    gc_requesting_safe_points: Mutex<bool>,
    /// Notification for when all goroutines reach safe points
    all_safe_condvar: std::sync::Condvar,
}

impl SafePointCoordinator {
    pub fn new() -> Self {
        Self {
            waiting_goroutines: Mutex::new(HashSet::new()),
            gc_requesting_safe_points: Mutex::new(false),
            all_safe_condvar: std::sync::Condvar::new(),
        }
    }

    /// Request all goroutines to reach safe points
    pub fn request_safe_points(&self) {
        debug!("Requesting all goroutines to reach safe points");
        *self.gc_requesting_safe_points.lock().unwrap() = true;
    }

    /// Release goroutines from safe points
    pub fn release_safe_points(&self) {
        debug!("Releasing goroutines from safe points");
        let mut requesting = self.gc_requesting_safe_points.lock().unwrap();
        *requesting = false;
        self.all_safe_condvar.notify_all();
    }

    /// Goroutine reports reaching a safe point
    pub fn goroutine_at_safe_point(&self, goroutine_id: GoroutineId) -> bool {
        let mut waiting = self.waiting_goroutines.lock().unwrap();
        let requesting = self.gc_requesting_safe_points.lock().unwrap();
        
        if *requesting {
            debug!(goroutine_id = goroutine_id, "Goroutine reached safe point");
            waiting.insert(goroutine_id);
            
            // If this was the last goroutine we were waiting for, notify GC
            // Note: This is simplified - in reality we'd need to track all active goroutines
            self.all_safe_condvar.notify_all();
            true // Stay at safe point
        } else {
            false // Continue execution
        }
    }

    /// Wait for all goroutines to reach safe points
    pub fn wait_for_all_safe_points(&self, timeout: Duration) -> bool {
        let start = Instant::now();
        let _waiting = self.waiting_goroutines.lock().unwrap();
        
        let result = self.all_safe_condvar
            .wait_timeout(_waiting, timeout)
            .unwrap();
        
        let success = !result.1.timed_out();
        if success {
            debug!(elapsed_ms = start.elapsed().as_millis(), "All goroutines reached safe points");
        } else {
            warn!(timeout_ms = timeout.as_millis(), "Timeout waiting for safe points");
        }
        
        success
    }
}

impl GoroutineGarbageCollector {
    /// Create a new goroutine-aware garbage collector
    pub fn new(gc: Arc<GarbageCollector>) -> Self {
        Self::with_config(gc, GoroutineGcConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(
        gc: Arc<GarbageCollector>,
        config: GoroutineGcConfig,
    ) -> Self {
        Self {
            gc,
            goroutine_states: Arc::new(RwLock::new(HashMap::new())),
            safe_point_coordinator: Arc::new(SafePointCoordinator::new()),
            config,
        }
    }

    /// Register a new goroutine with the GC
    #[instrument(skip(self), fields(goroutine_id = goroutine_id))]
    pub fn register_goroutine(&self, goroutine_id: GoroutineId, stack_base: usize, stack_size: usize) {
        debug!("Registering goroutine with GC");
        
        let state = GoroutineGcState {
            id: goroutine_id,
            thread_id: Some(thread::current().id()),
            stack_frames: Vec::new(),
            local_roots: HashSet::new(),
            allocated_objects: HashSet::new(),
            at_safe_point: false,
            last_safe_point: None,
            stack_base,
            stack_size,
        };

        if let Ok(mut states) = self.goroutine_states.write() {
            states.insert(goroutine_id, state);
            debug!(total_goroutines = states.len(), "Goroutine registered");
        }
    }

    /// Unregister a goroutine from the GC
    #[instrument(skip(self), fields(goroutine_id = goroutine_id))]
    pub fn unregister_goroutine(&self, goroutine_id: GoroutineId) {
        debug!("Unregistering goroutine from GC");
        
        if let Ok(mut states) = self.goroutine_states.write() {
            if let Some(state) = states.remove(&goroutine_id) {
                // Clean up any objects allocated by this goroutine that are no longer reachable
                self.cleanup_goroutine_objects(&state);
                debug!(remaining_goroutines = states.len(), "Goroutine unregistered");
            }
        }
    }

    /// Add a GC root for a specific goroutine
    pub fn add_goroutine_root(&self, goroutine_id: GoroutineId, ptr: usize) {
        if let Ok(mut states) = self.goroutine_states.write() {
            if let Some(state) = states.get_mut(&goroutine_id) {
                state.local_roots.insert(ptr);
                debug!(goroutine_id = goroutine_id, ptr = format!("0x{:x}", ptr), "Added goroutine-local root");
            }
        }
        
        // Also add to global GC roots
        self.gc.add_root(ptr);
    }

    /// Remove a GC root for a specific goroutine
    pub fn remove_goroutine_root(&self, goroutine_id: GoroutineId, ptr: usize) {
        if let Ok(mut states) = self.goroutine_states.write() {
            if let Some(state) = states.get_mut(&goroutine_id) {
                state.local_roots.remove(&ptr);
                debug!(goroutine_id = goroutine_id, ptr = format!("0x{:x}", ptr), "Removed goroutine-local root");
            }
        }
        
        // Also remove from global GC roots
        self.gc.remove_root(ptr);
    }

    /// Record allocation by a goroutine
    pub fn record_allocation(&self, goroutine_id: GoroutineId, ptr: usize) {
        if let Ok(mut states) = self.goroutine_states.write() {
            if let Some(state) = states.get_mut(&goroutine_id) {
                state.allocated_objects.insert(ptr);
                trace!(goroutine_id = goroutine_id, ptr = format!("0x{:x}", ptr), "Recorded allocation");
            }
        }
    }

    /// Perform goroutine-aware garbage collection
    #[instrument(skip(self))]
    pub fn collect_garbage_goroutine_aware(&self) -> Result<GoroutineGcStats, String> {
        info!("Starting goroutine-aware garbage collection");
        let start_time = Instant::now();
        let mut stats = GoroutineGcStats::default();

        // Step 1: Request all goroutines to reach safe points
        self.safe_point_coordinator.request_safe_points();
        
        let safe_point_timeout = Duration::from_millis(self.config.safe_point_timeout_ms);
        if !self.safe_point_coordinator.wait_for_all_safe_points(safe_point_timeout) {
            warn!("Not all goroutines reached safe points within timeout, proceeding anyway");
        }

        // Step 2: Scan all goroutine stacks and local roots
        let goroutine_roots = self.scan_all_goroutine_stacks(&mut stats)?;
        
        // Step 3: Add goroutine roots to global root set temporarily
        for &root in &goroutine_roots {
            self.gc.add_root(root);
        }

        // Step 4: Perform regular GC collection
        info!("Running underlying garbage collection");
        self.gc.collect_garbage();

        // Step 5: Remove temporary roots
        for &root in &goroutine_roots {
            self.gc.remove_root(root);
        }

        // Step 6: Update goroutine-specific statistics
        stats.total_goroutines = self.get_active_goroutine_count();
        stats.total_time_ms = start_time.elapsed().as_millis();

        // Step 7: Release goroutines from safe points
        self.safe_point_coordinator.release_safe_points();

        info!(
            goroutines = stats.total_goroutines,
            stack_roots = stats.stack_roots_found,
            time_ms = stats.total_time_ms,
            "Goroutine-aware collection completed"
        );

        Ok(stats)
    }

    /// Scan stacks of all goroutines for GC roots
    fn scan_all_goroutine_stacks(&self, stats: &mut GoroutineGcStats) -> Result<HashSet<usize>, String> {
        debug!("Scanning all goroutine stacks");
        let mut all_roots = HashSet::new();

        let states = self.goroutine_states.read()
            .map_err(|_| "Failed to acquire read lock on goroutine states")?;

        stats.total_goroutines = states.len();

        for (goroutine_id, state) in states.iter() {
            debug!(goroutine_id = *goroutine_id, "Scanning goroutine stack");
            
            // Add local roots
            for &root in &state.local_roots {
                all_roots.insert(root);
                stats.stack_roots_found += 1;
            }

            // Scan the actual stack if we have stack information
            if state.stack_base != 0 && state.stack_size > 0 {
                let stack_roots = self.scan_goroutine_stack(state)?;
                for root in stack_roots {
                    all_roots.insert(root);
                    stats.stack_roots_found += 1;
                }
            }

            stats.scanned_goroutines += 1;
        }

        debug!(total_roots = all_roots.len(), "Stack scanning completed");
        Ok(all_roots)
    }

    /// Scan a specific goroutine's stack for GC roots
    fn scan_goroutine_stack(&self, state: &GoroutineGcState) -> Result<HashSet<usize>, String> {
        let mut roots = HashSet::new();

        if self.config.conservative_stack_scan {
            // Conservative scanning: treat every pointer-aligned value as potential root
            roots.extend(self.conservative_stack_scan(state.stack_base, state.stack_size)?);
        } else {
            // Precise scanning: use stack frame information
            for frame in &state.stack_frames {
                roots.extend(&frame.gc_roots);
            }
        }

        debug!(
            goroutine_id = state.id,
            roots_found = roots.len(),
            "Goroutine stack scan completed"
        );

        Ok(roots)
    }

    /// Conservative stack scanning implementation
    fn conservative_stack_scan(&self, stack_base: usize, stack_size: usize) -> Result<HashSet<usize>, String> {
        let mut roots = HashSet::new();
        
        // Scan stack in chunks for better performance
        let chunk_size = self.config.stack_scan_chunk_size.min(stack_size);
        let mut offset = 0;

        while offset < stack_size {
            let current_chunk_size = (stack_size - offset).min(chunk_size);
            let chunk_start = stack_base - offset;
            let chunk_end = chunk_start - current_chunk_size;

            // Scan this chunk
            self.scan_memory_region(chunk_end, current_chunk_size, &mut roots)?;
            
            offset += current_chunk_size;
        }

        Ok(roots)
    }

    /// Scan a memory region for potential GC pointers
    fn scan_memory_region(&self, start_addr: usize, size: usize, roots: &mut HashSet<usize>) -> Result<(), String> {
        // Safety: This is conservative scanning, so we need to be careful about invalid memory
        let word_size = std::mem::size_of::<usize>();
        let num_words = size / word_size;

        for i in 0..num_words {
            let word_addr = start_addr + (i * word_size);
            
            // Try to read the word safely
            let potential_ptr = unsafe {
                // In a real implementation, we'd use proper memory safety checks
                // For now, we'll assume the stack memory is valid
                std::ptr::read(word_addr as *const usize)
            };

            // Check if this looks like a valid GC pointer
            if self.is_valid_gc_pointer(potential_ptr) {
                roots.insert(potential_ptr);
                trace!(ptr = format!("0x{:x}", potential_ptr), "Found potential GC root in stack");
            }
        }

        Ok(())
    }

    /// Check if a pointer value could be a valid GC pointer
    fn is_valid_gc_pointer(&self, ptr: usize) -> bool {
        // Basic sanity checks
        if ptr == 0 || ptr < 0x1000 { // Null or very low addresses
            return false;
        }

        // Check if it's properly aligned
        if ptr % std::mem::align_of::<usize>() != 0 {
            return false;
        }

        // Check if the GC knows about this object
        self.gc.is_alive(ptr)
    }

    /// Get the number of active goroutines
    pub fn get_active_goroutine_count(&self) -> usize {
        self.goroutine_states.read()
            .map(|states| states.len())
            .unwrap_or(0)
    }

    /// Update goroutine safe point status
    pub fn goroutine_safe_point(&self, goroutine_id: GoroutineId, safe_point_type: SafePointType) {
        // Update the goroutine state
        if let Ok(mut states) = self.goroutine_states.write() {
            if let Some(state) = states.get_mut(&goroutine_id) {
                state.at_safe_point = true;
                state.last_safe_point = Some(safe_point_type);
            }
        }

        // Notify the safe point coordinator
        if self.safe_point_coordinator.goroutine_at_safe_point(goroutine_id) {
            // GC is requesting safe points, so wait here
            self.wait_for_gc_release(goroutine_id);
        }
    }

    /// Wait for GC to release from safe point
    fn wait_for_gc_release(&self, goroutine_id: GoroutineId) {
        debug!(goroutine_id = goroutine_id, "Waiting for GC to release safe point");
        
        // Simple spin-wait with yield (in practice, we'd use proper synchronization)
        while *self.safe_point_coordinator.gc_requesting_safe_points.lock().unwrap() {
            thread::yield_now();
            
            // Update our state
            if let Ok(mut states) = self.goroutine_states.write() {
                if let Some(state) = states.get_mut(&goroutine_id) {
                    state.at_safe_point = false;
                }
            }
        }
        
        debug!(goroutine_id = goroutine_id, "Released from safe point");
    }

    /// Clean up objects allocated by a terminated goroutine
    fn cleanup_goroutine_objects(&self, state: &GoroutineGcState) {
        debug!(goroutine_id = state.id, "Cleaning up goroutine objects");
        
        // Remove local roots from global root set
        for &root in &state.local_roots {
            self.gc.remove_root(root);
        }

        // The allocated objects will be collected in the next GC cycle
        debug!(
            goroutine_id = state.id,
            local_roots = state.local_roots.len(),
            allocated_objects = state.allocated_objects.len(),
            "Goroutine cleanup completed"
        );
    }

    /// Get goroutine-aware memory statistics
    pub fn get_goroutine_stats(&self) -> GoroutineGcStats {
        let mut stats = GoroutineGcStats::default();
        
        if let Ok(states) = self.goroutine_states.read() {
            stats.total_goroutines = states.len();
            
            for state in states.values() {
                stats.total_local_roots += state.local_roots.len();
                stats.total_allocated_objects += state.allocated_objects.len();
                if state.at_safe_point {
                    stats.goroutines_at_safe_points += 1;
                }
            }
        }

        // Add underlying GC stats
        let gc_stats = self.gc.stats();
        stats.gc_stats = gc_stats;

        stats
    }
}

/// Statistics for goroutine-aware garbage collection
#[derive(Debug, Clone, Default)]
pub struct GoroutineGcStats {
    pub total_goroutines: usize,
    pub scanned_goroutines: usize,
    pub goroutines_at_safe_points: usize,
    pub stack_roots_found: usize,
    pub total_local_roots: usize,
    pub total_allocated_objects: usize,
    pub total_time_ms: u128,
    pub gc_stats: MemoryStats,
}

// FFI functions for integration with LLVM-generated code

/// Register a goroutine with the GC system
#[no_mangle]
pub extern "C" fn cursed_gc_register_goroutine(
    goroutine_id: u64,
    stack_base: *mut c_void,
    stack_size: usize,
) {
    debug!(goroutine_id = goroutine_id, "Registering goroutine via FFI");
    
    // Get the global goroutine GC
    let goroutine_gc = get_global_goroutine_gc();
    goroutine_gc.register_goroutine(goroutine_id, stack_base as usize, stack_size);
}

/// Unregister a goroutine from the GC system
#[no_mangle]
pub extern "C" fn cursed_gc_unregister_goroutine(goroutine_id: u64) {
    debug!(goroutine_id = goroutine_id, "Unregistering goroutine via FFI");
    
    let goroutine_gc = get_global_goroutine_gc();
    goroutine_gc.unregister_goroutine(goroutine_id);
}

/// Notify GC of a safe point
#[no_mangle]
pub extern "C" fn cursed_gc_safe_point(goroutine_id: u64, safe_point_type: u32) {
    let safe_point = match safe_point_type {
        0 => SafePointType::FunctionEntry,
        1 => SafePointType::FunctionExit,
        2 => SafePointType::LoopBackEdge,
        3 => SafePointType::Allocation,
        4 => SafePointType::Yield,
        5 => SafePointType::ChannelOp,
        _ => SafePointType::Yield,
    };
    
    let goroutine_gc = get_global_goroutine_gc();
    goroutine_gc.goroutine_safe_point(goroutine_id, safe_point);
}

/// Add a goroutine-local root
#[no_mangle]
pub extern "C" fn cursed_gc_add_goroutine_root(goroutine_id: u64, ptr: *mut c_void) {
    let goroutine_gc = get_global_goroutine_gc();
    goroutine_gc.add_goroutine_root(goroutine_id, ptr as usize);
}

/// Remove a goroutine-local root
#[no_mangle]
pub extern "C" fn cursed_gc_remove_goroutine_root(goroutine_id: u64, ptr: *mut c_void) {
    let goroutine_gc = get_global_goroutine_gc();
    goroutine_gc.remove_goroutine_root(goroutine_id, ptr as usize);
}

/// Trigger goroutine-aware garbage collection
#[no_mangle]
pub extern "C" fn cursed_gc_collect_goroutine_aware() -> i32 {
    debug!("Triggering goroutine-aware GC via FFI");
    
    let goroutine_gc = get_global_goroutine_gc();
    match goroutine_gc.collect_garbage_goroutine_aware() {
        Ok(_) => 0,
        Err(e) => {
            error!(error = %e, "Goroutine-aware GC failed");
            1
        }
    }
}

// Global instance management
use once_cell::sync::Lazy;

static GLOBAL_GOROUTINE_GC: Lazy<Arc<GoroutineGarbageCollector>> = Lazy::new(|| {
    let gc = crate::memory::get_global_gc();
    Arc::new(GoroutineGarbageCollector::new(gc))
});

/// Get the global goroutine-aware garbage collector
pub fn get_global_goroutine_gc() -> Arc<GoroutineGarbageCollector> {
    GLOBAL_GOROUTINE_GC.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_safe_point_coordinator() {
        let coordinator = SafePointCoordinator::new();
        
        // Test basic safe point functionality
        coordinator.request_safe_points();
        assert!(coordinator.goroutine_at_safe_point(1));
        
        coordinator.release_safe_points();
        assert!(!coordinator.goroutine_at_safe_point(1));
    }

    #[test]
    fn test_goroutine_registration() {
        let gc = Arc::new(GarbageCollector::new());
        let scheduler = crate::runtime::goroutine::get_global_scheduler();
        let goroutine_gc = GoroutineGarbageCollector::new(gc);

        // Register a goroutine
        goroutine_gc.register_goroutine(1, 0x1000, 0x2000);
        assert_eq!(goroutine_gc.get_active_goroutine_count(), 1);

        // Unregister the goroutine
        goroutine_gc.unregister_goroutine(1);
        assert_eq!(goroutine_gc.get_active_goroutine_count(), 0);
    }

    #[test]
    fn test_goroutine_roots() {
        let gc = Arc::new(GarbageCollector::new());
        let scheduler = crate::runtime::goroutine::get_global_scheduler();
        let goroutine_gc = GoroutineGarbageCollector::new(gc);

        goroutine_gc.register_goroutine(1, 0x1000, 0x2000);
        
        // Add a root
        goroutine_gc.add_goroutine_root(1, 0x5000);
        
        let stats = goroutine_gc.get_goroutine_stats();
        assert_eq!(stats.total_local_roots, 1);
        
        // Remove the root
        goroutine_gc.remove_goroutine_root(1, 0x5000);
        
        let stats = goroutine_gc.get_goroutine_stats();
        assert_eq!(stats.total_local_roots, 0);
    }
}
