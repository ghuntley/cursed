//! Memory Management System for CURSED Runtime
//!
//! This module provides the primary memory management interface that integrates
//! the garbage collector with the CURSED runtime system, replacing the minimal
//! memory management with a comprehensive GC-based system.

use std::sync::{Arc, Mutex, RwLock, OnceLock};
use std::ptr::NonNull;
use std::collections::HashMap;
use std::time::{Instant, Duration};

use crate::error::CursedError;
use crate::memory::{Tag, Traceable, Visitor};
use crate::runtime::gc::{GarbageCollector, GcConfig, GcStats, RootType, HeapObject};
use crate::runtime::stack::{RuntimeStack, StackId};
use crate::runtime::channels::ChannelError;

/// Platform error type for PAL integration
#[derive(Debug, Clone)]
pub enum PlatformError {
    /// Memory allocation failed
    AllocationFailed(String),
    /// Invalid size for allocation
    InvalidSize(usize),
    /// Out of memory
    OutOfMemory(usize),
    /// Platform-specific error
    PlatformSpecific(String),
}

impl std::fmt::Display for PlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformError::AllocationFailed(msg) => write!(f, "Allocation failed: {}", msg),
            PlatformError::InvalidSize(size) => write!(f, "Invalid size: {}", size),
            PlatformError::OutOfMemory(size) => write!(f, "Out of memory: {} bytes requested", size),
            PlatformError::PlatformSpecific(msg) => write!(f, "Platform error: {}", msg),
        }
    }
}

impl std::error::Error for PlatformError {}

/// Memory manager trait for platform abstraction layer
/// 
/// This trait provides the interface that PAL implementations expect for memory management.
/// It includes low-level memory allocation operations and platform-specific memory handling.
pub trait MemoryManager: Send + Sync {
    /// Allocate raw memory of the specified size
    /// 
    /// # Arguments
    /// * `size` - Number of bytes to allocate
    /// 
    /// # Returns
    /// * `Ok(*mut u8)` - Pointer to allocated memory
    /// * `Err(PlatformError)` - Allocation failed
    /// 
    /// # Safety
    /// The returned pointer must be properly aligned and point to valid memory
    /// that can be written to for `size` bytes.
    fn allocate(&self, size: usize) -> Result<*mut u8, PlatformError>;
    
    /// Deallocate previously allocated memory
    /// 
    /// # Arguments
    /// * `ptr` - Pointer to memory to deallocate (must have been returned by `allocate`)
    /// * `size` - Size of the allocation (must match the size passed to `allocate`)
    /// 
    /// # Returns
    /// * `Ok(())` - Deallocation successful
    /// * `Err(PlatformError)` - Deallocation failed
    /// 
    /// # Safety
    /// The pointer must have been returned by a previous call to `allocate` on this
    /// memory manager instance, and must not have been deallocated already.
    fn deallocate(&self, ptr: *mut u8, size: usize) -> Result<(), PlatformError>;
    
    /// Get the platform's memory page size
    /// 
    /// # Returns
    /// Memory page size in bytes (typically 4096 on most platforms)
    fn page_size(&self) -> usize;
    
    /// Get current memory usage statistics
    /// 
    /// # Returns
    /// Total allocated memory in bytes
    fn memory_usage(&self) -> usize;
    
    /// Check if a memory address is valid for the given size
    /// 
    /// # Arguments
    /// * `ptr` - Pointer to check
    /// * `size` - Size of memory region to validate
    /// 
    /// # Returns
    /// `true` if the memory region is valid and accessible
    fn is_valid_memory(&self, ptr: *const u8, size: usize) -> bool;
    
    /// Flush any pending memory operations to ensure consistency
    /// 
    /// This is particularly important on weakly-ordered memory architectures
    /// like ARM64 where memory operations may be reordered.
    fn memory_barrier(&self);
    
    /// Get memory usage statistics (optional)
    /// 
    /// # Returns
    /// Optional memory statistics if available
    fn get_stats(&self) -> Option<MemoryStats> {
        None // Default implementation returns None
    }
}

/// Memory management configuration
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    /// Garbage collector configuration
    pub gc_config: GcConfig,
    /// Enable memory tracking and debugging
    pub enable_tracking: bool,
    /// Memory limit per goroutine stack
    pub stack_memory_limit: Option<usize>,
    /// Global memory limit
    pub global_memory_limit: Option<usize>,
    /// Enable memory pressure detection
    pub enable_pressure_detection: bool,
    /// Memory pressure threshold (0.0-1.0)
    pub pressure_threshold: f64,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            gc_config: GcConfig::default(),
            enable_tracking: true,
            stack_memory_limit: Some(8 * 1024 * 1024), // 8MB per stack
            global_memory_limit: Some(2 * 1024 * 1024 * 1024), // 2GB global
            enable_pressure_detection: true,
            pressure_threshold: 0.8, // 80%
        }
    }
}

/// Memory allocation statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryStats {
    /// Total heap allocations
    pub heap_allocations: u64,
    /// Total heap deallocations
    pub heap_deallocations: u64,
    /// Current heap usage
    pub heap_usage: usize,
    /// Peak heap usage
    pub peak_heap_usage: usize,
    /// Stack allocations
    pub stack_allocations: u64,
    /// Stack deallocations
    pub stack_deallocations: u64,
    /// Current stack usage
    pub stack_usage: usize,
    /// Peak stack usage
    pub peak_stack_usage: usize,
    /// GC statistics
    pub gc_stats: GcStats,
    /// Memory pressure indicator (0.0-1.0)
    pub pressure_level: f64,
    /// Last pressure check time
    #[serde(skip)]
    pub last_pressure_check: Option<Instant>,
    /// Total allocations (for PAL compatibility)
    pub total_allocations: usize,
    /// Total deallocations (for PAL compatibility)
    pub total_deallocations: usize,
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self {
            heap_allocations: 0,
            heap_deallocations: 0,
            heap_usage: 0,
            peak_heap_usage: 0,
            stack_allocations: 0,
            stack_deallocations: 0,
            stack_usage: 0,
            peak_stack_usage: 0,
            gc_stats: Default::default(),
            pressure_level: 0.0,
            last_pressure_check: None,
            total_allocations: 0,
            total_deallocations: 0,
        }
    }
}

/// Memory allocation result
pub type MemoryResult<T> = Result<T, MemoryError>;

/// Memory management errors
#[derive(Debug, Clone)]
pub enum MemoryError {
    /// Out of memory
    OutOfMemory { requested: usize, available: usize },
    /// Memory limit exceeded
    LimitExceeded { limit: usize, current: usize },
    /// Invalid allocation size
    InvalidSize(usize),
    /// Memory corruption detected
    Corruption(String),
    /// GC error
    GcError(String),
    /// Stack overflow
    StackOverflow { stack_id: StackId, size: usize },
    /// Channel allocation error
    ChannelError(String),
    /// Initialization failed
    InitializationFailed(String),
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::OutOfMemory { requested, available } => {
                write!(f, "Out of memory: requested {} bytes, {} available", requested, available)
            }
            MemoryError::LimitExceeded { limit, current } => {
                write!(f, "Memory limit exceeded: {} current, {} limit", current, limit)
            }
            MemoryError::InvalidSize(size) => {
                write!(f, "Invalid allocation size: {}", size)
            }
            MemoryError::Corruption(msg) => {
                write!(f, "Memory corruption: {}", msg)
            }
            MemoryError::GcError(msg) => {
                write!(f, "Garbage collection error: {}", msg)
            }
            MemoryError::StackOverflow { stack_id, size } => {
                write!(f, "Stack overflow: stack {} requested {}", stack_id, size)
            }
            MemoryError::ChannelError(msg) => {
                write!(f, "Channel allocation error: {}", msg)
            }
            MemoryError::InitializationFailed(msg) => {
                write!(f, "Initialization failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for MemoryError {}

impl From<CursedError> for MemoryError {
    fn from(err: CursedError) -> Self {
        MemoryError::GcError(err.to_string())
    }
}

impl From<ChannelError> for MemoryError {
    fn from(err: ChannelError) -> Self {
        MemoryError::ChannelError(err.to_string())
    }
}

/// Object handle for managed memory
#[derive(Debug, Clone)]
pub struct ObjectHandle {
    /// Pointer to heap object
    pub ptr: NonNull<HeapObject>,
    /// Object generation
    pub generation: u8,
    /// Allocation timestamp
    pub allocated_at: Instant,
}

unsafe impl Send for ObjectHandle {}
unsafe impl Sync for ObjectHandle {}

impl ObjectHandle {
    /// Create new object handle
    pub fn new(ptr: NonNull<HeapObject>, generation: u8) -> Self {
        Self {
            ptr,
            generation,
            allocated_at: Instant::now(),
        }
    }
    
    /// Get object metadata
    pub fn metadata(&self) -> &crate::runtime::gc::ObjectMetadata {
        unsafe { &(*self.ptr.as_ptr()).metadata }
    }
    
    /// Get object data pointer
    pub fn data_ptr(&self) -> *mut u8 {
        unsafe { &mut (*self.ptr.as_ptr()).data as *mut [u8; 0] as *mut u8 }
    }
    
    /// Get object size
    pub fn size(&self) -> usize {
        self.metadata().size
    }
    
    /// Get object tag
    pub fn tag(&self) -> Tag {
        self.metadata().tag
    }
}

/// Main memory manager for CURSED runtime
pub struct RuntimeMemoryManager {
    /// Configuration
    config: MemoryConfig,
    /// Garbage collector
    gc: Arc<GarbageCollector>,
    /// Stack manager
    stack_manager: Arc<RuntimeStack>,
    /// Memory statistics
    stats: RwLock<MemoryStats>,
    /// Object tracking (if enabled) - using object IDs instead of raw pointers for thread safety
    tracked_objects: RwLock<HashMap<usize, ObjectHandle>>,
    /// Root object registry - using object IDs instead of raw pointers
    root_registry: RwLock<HashMap<String, usize>>,
    /// Memory pressure state
    pressure_state: RwLock<PressureState>,
}

/// Memory pressure state
struct PressureState {
    /// Current pressure level (0.0-1.0)
    level: f64,
    /// Last check time
    last_check: Instant,
    /// Pressure callbacks
    callbacks: Vec<Box<dyn Fn(f64) + Send + Sync>>,
}

impl std::fmt::Debug for PressureState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PressureState")
            .field("level", &self.level)
            .field("last_check", &self.last_check)
            .field("callbacks", &format!("Vec<Box<dyn Fn(f64)>> with {} callbacks", self.callbacks.len()))
            .finish()
    }
}

impl RuntimeMemoryManager {
    /// Create new memory manager
    pub fn new(config: MemoryConfig, stack_manager: Arc<RuntimeStack>) -> Result<Self, MemoryError> {
        let gc = GarbageCollector::new_with_config(config.gc_config.clone(), Arc::clone(&stack_manager))?;
        
        Ok(Self {
            config,
            gc: Arc::new(gc),
            stack_manager,
            stats: RwLock::new(MemoryStats::default()),
            tracked_objects: RwLock::new(HashMap::new()),
            root_registry: RwLock::new(HashMap::new()),
            pressure_state: RwLock::new(PressureState {
                level: 0.0,
                last_check: Instant::now(),
                callbacks: Vec::new(),
            }),
        })
    }
    
    /// Allocate object in managed heap
    pub fn allocate<T: Traceable + 'static>(&self, data: T) -> MemoryResult<ObjectHandle> {
        let size = data.size();
        let tag = data.get_tag();
        
        // Check memory limits
        self.check_memory_limits(size)?;
        
        // Allocate through GC
        let obj_ptr = self.gc.allocate(size, tag)?;
        
        // Initialize object data
        unsafe {
            let data_ptr = &mut (*obj_ptr.as_ptr()).data as *mut [u8; 0] as *mut T;
            std::ptr::write(data_ptr, data);
        }
        
        let handle = ObjectHandle::new(obj_ptr, 0);
        
        // Track object if enabled
        if self.config.enable_tracking {
            let mut tracked = self.tracked_objects.write().unwrap();
            tracked.insert(obj_ptr.as_ptr() as usize, handle.clone());
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.heap_allocations += 1;
            stats.heap_usage += size;
            if stats.heap_usage > stats.peak_heap_usage {
                stats.peak_heap_usage = stats.heap_usage;
            }
        }
        
        // Check memory pressure
        if self.config.enable_pressure_detection {
            self.check_memory_pressure()?;
        }
        
        Ok(handle)
    }
    
    /// Allocate raw memory in heap
    pub fn allocate_raw(&self, size: usize, tag: Tag) -> MemoryResult<ObjectHandle> {
        // Check memory limits
        self.check_memory_limits(size)?;
        
        // Allocate through GC
        let obj_ptr = self.gc.allocate(size, tag)?;
        
        let handle = ObjectHandle::new(obj_ptr, 0);
        
        // Track object if enabled
        if self.config.enable_tracking {
            let mut tracked = self.tracked_objects.write().unwrap();
            tracked.insert(obj_ptr.as_ptr() as usize, handle.clone());
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.heap_allocations += 1;
            stats.heap_usage += size;
            if stats.heap_usage > stats.peak_heap_usage {
                stats.peak_heap_usage = stats.heap_usage;
            }
        }
        
        Ok(handle)
    }
    
    /// Deallocate object (manually trigger collection)
    pub fn deallocate(&self, handle: &ObjectHandle) -> MemoryResult<()> {
        // Remove from tracking
        if self.config.enable_tracking {
            let mut tracked = self.tracked_objects.write().unwrap();
            tracked.remove(&(handle.ptr.as_ptr() as usize));
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.heap_deallocations += 1;
            stats.heap_usage = stats.heap_usage.saturating_sub(handle.size());
        }
        
        Ok(())
    }
    
    /// Add root object
    pub fn add_root(&self, name: String, handle: &ObjectHandle) -> MemoryResult<()> {
        // Add to GC roots
        self.gc.add_root(handle.ptr.as_ptr() as *mut u8)?;
        
        // Add to registry
        {
            let mut registry = self.root_registry.write().unwrap();
            registry.insert(name, handle.ptr.as_ptr() as usize);
        }
        
        Ok(())
    }
    
    /// Remove root object
    pub fn remove_root(&self, name: &str) -> MemoryResult<()> {
        let mut registry = self.root_registry.write().unwrap();
        if let Some(obj_ptr) = registry.remove(name) {
            self.gc.remove_root(obj_ptr as *mut u8)?;
        }
        
        Ok(())
    }
    
    /// Force garbage collection
    pub fn collect_garbage(&self) -> MemoryResult<crate::memory::gc::GcStats> {
        self.gc.collect()?;
        let gc_stats = self.gc.get_stats()?;
        
        // Convert GCStats to GcStats
        let stats = crate::memory::gc::GcStats {
            total_collections: gc_stats.total_collections,
            total_time_ms: gc_stats.average_collection_time.as_millis() as u64,
            objects_collected: gc_stats.objects_swept,
            bytes_collected: 0, // Not tracked in GCStats
            last_collection_time_ms: gc_stats.average_collection_time.as_millis() as u64,
            last_objects_collected: gc_stats.objects_swept as usize,
            avg_pause_time: gc_stats.average_collection_time,
            max_pause_time: gc_stats.average_collection_time,
            gc_overhead: 0.0, // Not tracked in GCStats
            heap_utilization: 0.0, // Not tracked in GCStats
            allocation_rate: 0.0, // Not tracked in GCStats
            total_gc_time: gc_stats.average_collection_time,
        };
        
        // Update memory statistics
        {
            let mut mem_stats = self.stats.write().unwrap();
            mem_stats.gc_stats = stats.clone();
        }
        
        Ok(stats)
    }
    
    /// Allocate stack for goroutine
    pub fn allocate_stack(&self, size: Option<usize>) -> MemoryResult<StackId> {
        let stack_size = size.unwrap_or(8 * 1024 * 1024); // 8MB default
        
        // Check stack memory limits
        if let Some(limit) = self.config.stack_memory_limit {
            if stack_size > limit {
                return Err(MemoryError::LimitExceeded {
                    limit,
                    current: stack_size,
                });
            }
        }
        
        let stack_id = self.stack_manager.allocate_stack(Some(stack_size))?;
        
        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.stack_allocations += 1;
            stats.stack_usage += stack_size;
            if stats.stack_usage > stats.peak_stack_usage {
                stats.peak_stack_usage = stats.stack_usage;
            }
        }
        
        Ok(stack_id)
    }
    
    /// Deallocate stack
    pub fn deallocate_stack(&self, stack_id: StackId) -> MemoryResult<()> {
        // Get stack info for statistics
        let stack_info = self.stack_manager.get_stack_info(stack_id)?;
        
        self.stack_manager.deallocate_stack(stack_id)?;
        
        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.stack_deallocations += 1;
            stats.stack_usage = stats.stack_usage.saturating_sub(stack_info.size);
        }
        
        Ok(())
    }
    
    /// Check memory limits before allocation
    fn check_memory_limits(&self, size: usize) -> MemoryResult<()> {
        if size == 0 {
            return Err(MemoryError::InvalidSize(size));
        }
        
        if let Some(global_limit) = self.config.global_memory_limit {
            let stats = self.stats.read().unwrap();
            let total_usage = stats.heap_usage + stats.stack_usage;
            
            if total_usage + size > global_limit {
                return Err(MemoryError::LimitExceeded {
                    limit: global_limit,
                    current: total_usage + size,
                });
            }
        }
        
        Ok(())
    }
    
    /// Check memory pressure and trigger callbacks
    fn check_memory_pressure(&self) -> MemoryResult<()> {
        let mut pressure_state = self.pressure_state.write().unwrap();
        
        // Only check pressure periodically
        if pressure_state.last_check.elapsed() < Duration::from_secs(1) {
            return Ok(());
        }
        
        pressure_state.last_check = Instant::now();
        
        // Calculate pressure level
        let stats = self.stats.read().unwrap();
        let pressure = if let Some(global_limit) = self.config.global_memory_limit {
            let total_usage = stats.heap_usage + stats.stack_usage;
            total_usage as f64 / global_limit as f64
        } else {
            // Use GC heap utilization as pressure indicator
            stats.gc_stats.heap_utilization
        };
        
        pressure_state.level = pressure;
        
        // Update main stats
        drop(stats);
        {
            let mut main_stats = self.stats.write().unwrap();
            main_stats.pressure_level = pressure;
            main_stats.last_pressure_check = Some(pressure_state.last_check);
        }
        
        // Trigger callbacks if pressure is high
        if pressure > self.config.pressure_threshold {
            for callback in &pressure_state.callbacks {
                callback(pressure);
            }
        }
        
        Ok(())
    }
    
    /// Register memory pressure callback
    pub fn register_pressure_callback<F>(&self, callback: F)
    where
        F: Fn(f64) + Send + Sync + 'static,
    {
        let mut pressure_state = self.pressure_state.write().unwrap();
        pressure_state.callbacks.push(Box::new(callback));
    }
    
    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        let mut stats = self.stats.read().unwrap().clone();
        if let Ok(gc_stats) = self.gc.get_stats() {
            // Convert GCStats to GcStats
            stats.gc_stats = crate::memory::gc::GcStats {
                total_collections: gc_stats.total_collections,
                total_time_ms: gc_stats.average_collection_time.as_millis() as u64,
                objects_collected: gc_stats.objects_swept,
                bytes_collected: 0, // Not tracked in GCStats
                last_collection_time_ms: gc_stats.average_collection_time.as_millis() as u64,
                last_objects_collected: gc_stats.objects_swept as usize,
                avg_pause_time: gc_stats.average_collection_time,
                max_pause_time: gc_stats.average_collection_time,
                gc_overhead: 0.0, // Not tracked in GCStats
                heap_utilization: 0.0, // Not tracked in GCStats  
                allocation_rate: 0.0, // Not tracked in GCStats
                total_gc_time: gc_stats.average_collection_time,
            };
        }
        stats
    }
    
    /// Get tracked objects count
    pub fn tracked_objects_count(&self) -> usize {
        if self.config.enable_tracking {
            self.tracked_objects.read().unwrap().len()
        } else {
            0
        }
    }
    
    /// Get root objects count
    pub fn root_objects_count(&self) -> usize {
        self.root_registry.read().unwrap().len()
    }
    
    /// Dump memory information for debugging
    pub fn dump_memory_info(&self) -> String {
        let stats = self.get_stats();
        let tracked_count = self.tracked_objects_count();
        let root_count = self.root_objects_count();
        
        format!(
            "Memory Info:\n\
             Heap: {} allocations, {} bytes used, {} peak\n\
             Stack: {} allocations, {} bytes used, {} peak\n\
             GC: {} collections, {:.2}ms avg pause\n\
             Tracked objects: {}\n\
             Root objects: {}\n\
             Pressure level: {:.2}%",
            stats.heap_allocations,
            stats.heap_usage,
            stats.peak_heap_usage,
            stats.stack_allocations,
            stats.stack_usage,
            stats.peak_stack_usage,
            stats.gc_stats.total_collections,
            stats.gc_stats.avg_pause_time.as_millis(),
            tracked_count,
            root_count,
            stats.pressure_level * 100.0
        )
    }
    
    /// Perform memory health check
    pub fn health_check(&self) -> MemoryResult<bool> {
        let stats = self.get_stats();
        
        // Check pressure level
        if stats.pressure_level > 0.9 {
            return Err(MemoryError::LimitExceeded {
                limit: self.config.global_memory_limit.unwrap_or(usize::MAX),
                current: stats.heap_usage + stats.stack_usage,
            });
        }
        
        // Check GC health
        if stats.gc_stats.gc_overhead > 0.5 {
            return Err(MemoryError::GcError(format!(
                "GC overhead too high: {:.2}%",
                stats.gc_stats.gc_overhead * 100.0
            )));
        }
        
        Ok(true)
    }
    
    /// Shutdown memory manager
    pub fn shutdown(&self) -> MemoryResult<()> {
        // Shutdown GC
        self.gc.shutdown()?;
        
        // Clear tracked objects
        if self.config.enable_tracking {
            self.tracked_objects.write().unwrap().clear();
        }
        
        // Clear root registry
        self.root_registry.write().unwrap().clear();
        
        Ok(())
    }
}

/// Implementation of MemoryManager trait for RuntimeMemoryManager
impl MemoryManager for RuntimeMemoryManager {
    fn allocate(&self, size: usize) -> Result<*mut u8, PlatformError> {
        if size == 0 {
            return Err(PlatformError::InvalidSize(size));
        }
        
        // Use the existing allocate_raw method
        match self.allocate_raw(size, Tag::Object) {
            Ok(handle) => Ok(handle.data_ptr()),
            Err(MemoryError::OutOfMemory { requested, available }) => {
                Err(PlatformError::OutOfMemory(requested))
            }
            Err(MemoryError::InvalidSize(s)) => Err(PlatformError::InvalidSize(s)),
            Err(e) => Err(PlatformError::AllocationFailed(e.to_string())),
        }
    }
    
    fn deallocate(&self, ptr: *mut u8, size: usize) -> Result<(), PlatformError> {
        // Since our GC handles deallocation automatically, we just validate the pointer
        if ptr.is_null() {
            return Err(PlatformError::InvalidSize(0));
        }
        
        // In a real implementation, we would need to map ptr back to ObjectHandle
        // For now, just return success as GC will handle cleanup
        Ok(())
    }
    
    fn page_size(&self) -> usize {
        // Use standard page size - could be platform-specific
        4096
    }
    
    fn memory_usage(&self) -> usize {
        let stats = self.get_stats();
        stats.heap_usage + stats.stack_usage
    }
    
    fn is_valid_memory(&self, ptr: *const u8, size: usize) -> bool {
        if ptr.is_null() || size == 0 {
            return false;
        }
        
        // Basic pointer alignment check
        (ptr as usize) % std::mem::align_of::<u8>() == 0
    }
    
    fn memory_barrier(&self) {
        // Use atomic fence for memory ordering
        std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
    }
    
    fn get_stats(&self) -> Option<MemoryStats> {
        Some(self.get_stats())
    }
}

/// Global memory manager instance
static GLOBAL_MEMORY_MANAGER: OnceLock<Arc<RuntimeMemoryManager>> = OnceLock::new();

/// Initialize global memory manager
pub fn initialize_memory_manager(
    config: MemoryConfig,
    stack_manager: Arc<RuntimeStack>,
) -> Result<(), MemoryError> {
    let manager = RuntimeMemoryManager::new(config, stack_manager)?;
    let _ = GLOBAL_MEMORY_MANAGER.set(Arc::new(manager));
    Ok(())
}

/// Get global memory manager
pub fn get_global_memory_manager() -> Option<Arc<RuntimeMemoryManager>> {
    GLOBAL_MEMORY_MANAGER.get().map(|manager| Arc::clone(manager))
}

/// Shutdown global memory manager
pub fn shutdown_memory_manager() -> Result<(), MemoryError> {
    if let Some(manager) = get_global_memory_manager() {
        manager.shutdown()?;
    }
    Ok(())
}

/// Convenience function to allocate object using global manager
pub fn allocate<T: Traceable + 'static>(data: T) -> MemoryResult<ObjectHandle> {
    get_global_memory_manager()
        .ok_or_else(|| MemoryError::GcError("Memory manager not initialized".to_string()))?
        .allocate(data)
}

/// Convenience function to allocate raw memory using global manager
pub fn allocate_raw(size: usize, tag: Tag) -> MemoryResult<ObjectHandle> {
    get_global_memory_manager()
        .ok_or_else(|| MemoryError::GcError("Memory manager not initialized".to_string()))?
        .allocate_raw(size, tag)
}

/// Convenience function to collect garbage using global manager
pub fn collect_garbage() -> MemoryResult<GcStats> {
    get_global_memory_manager()
        .ok_or_else(|| MemoryError::GcError("Memory manager not initialized".to_string()))?
        .collect_garbage()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_manager_creation() {
        let stack_manager = Arc::new(RuntimeStack::new());
        let config = MemoryConfig::default();
        let manager = RuntimeMemoryManager::new(config, stack_manager).unwrap();
        
        assert_eq!(manager.tracked_objects_count(), 0);
        assert_eq!(manager.root_objects_count(), 0);
    }
    
    #[test]
    fn test_raw_allocation() {
        let stack_manager = Arc::new(RuntimeStack::new());
        let config = MemoryConfig::default();
        let manager = RuntimeMemoryManager::new(config, stack_manager).unwrap();
        
        let handle = manager.allocate_raw(64, Tag::Object).unwrap();
        assert_eq!(handle.size(), 64);
        assert_eq!(handle.tag(), Tag::Object);
    }
    
    #[test]
    fn test_stack_allocation() {
        let stack_manager = Arc::new(RuntimeStack::new());
        let config = MemoryConfig::default();
        let manager = RuntimeMemoryManager::new(config, stack_manager).unwrap();
        
        let stack_id = manager.allocate_stack(Some(1024 * 1024)).unwrap();
        assert!(manager.deallocate_stack(stack_id).is_ok());
    }
    
    #[test]
    fn test_memory_limits() {
        let stack_manager = Arc::new(RuntimeStack::new());
        let mut config = MemoryConfig::default();
        config.global_memory_limit = Some(1024); // 1KB limit
        
        let manager = RuntimeMemoryManager::new(config, stack_manager).unwrap();
        
        // This should fail due to memory limit
        let result = manager.allocate_raw(2048, Tag::Object);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_memory_stats() {
        let stack_manager = Arc::new(RuntimeStack::new());
        let config = MemoryConfig::default();
        let manager = RuntimeMemoryManager::new(config, stack_manager).unwrap();
        
        let stats = manager.get_stats();
        assert_eq!(stats.heap_allocations, 0);
        assert_eq!(stats.stack_allocations, 0);
    }
}

/// Platform-specific memory manager implementations

/// x86_64-specific memory manager
pub struct X86_64MemoryManager {
    base: RuntimeMemoryManager,
}

impl X86_64MemoryManager {
    pub fn new_macos() -> Result<Self, PlatformError> {
        let stack_manager = Arc::new(RuntimeStack::new());
        let config = MemoryConfig::default();
        let base = RuntimeMemoryManager::new(config, stack_manager)
            .map_err(|e| PlatformError::AllocationFailed(e.to_string()))?;
        
        Ok(Self { base })
    }
    
    pub fn new_linux() -> Result<Self, PlatformError> {
        Self::new_macos() // Same implementation for now
    }
    
    pub fn new_windows() -> Result<Self, PlatformError> {
        Self::new_macos() // Same implementation for now
    }
}

impl MemoryManager for X86_64MemoryManager {
    fn allocate(&self, size: usize) -> Result<*mut u8, PlatformError> {
        // Use the existing allocate_raw method
        match self.base.allocate_raw(size, Tag::Object) {
            Ok(handle) => Ok(handle.data_ptr()),
            Err(MemoryError::OutOfMemory { requested, available }) => {
                Err(PlatformError::OutOfMemory(requested))
            }
            Err(MemoryError::InvalidSize(s)) => Err(PlatformError::InvalidSize(s)),
            Err(e) => Err(PlatformError::AllocationFailed(e.to_string())),
        }
    }
    
    fn deallocate(&self, ptr: *mut u8, size: usize) -> Result<(), PlatformError> {
        // Since our GC handles deallocation automatically, we just validate the pointer
        if ptr.is_null() {
            return Err(PlatformError::InvalidSize(0));
        }
        
        // In a real implementation, we would need to map ptr back to ObjectHandle
        // For now, just return success as GC will handle cleanup
        Ok(())
    }
    
    fn page_size(&self) -> usize {
        4096 // x86_64 standard page size
    }
    
    fn memory_usage(&self) -> usize {
        self.base.memory_usage()
    }
    
    fn is_valid_memory(&self, ptr: *const u8, size: usize) -> bool {
        self.base.is_valid_memory(ptr, size)
    }
    
    fn memory_barrier(&self) {
        // x86_64-specific memory barrier (use mfence instruction)
        std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
    }
    
    fn get_stats(&self) -> Option<MemoryStats> {
        Some(self.base.get_stats())
    }
}

// Additional platform-specific memory managers
pub struct Arm64MemoryManager {
    inner: RuntimeMemoryManager,
}

impl Arm64MemoryManager {
    pub fn new_macos() -> Result<Self, PlatformError> {
        let stack_manager = Arc::new(RuntimeStack::new());
        let config = MemoryConfig::default();
        let inner = RuntimeMemoryManager::new(config, stack_manager)
            .map_err(|e| PlatformError::AllocationFailed(e.to_string()))?;
        Ok(Self { inner })
    }
    
    pub fn new_linux() -> Result<Self, PlatformError> {
        let stack_manager = Arc::new(RuntimeStack::new());
        let config = MemoryConfig::default();
        let inner = RuntimeMemoryManager::new(config, stack_manager)
            .map_err(|e| PlatformError::AllocationFailed(e.to_string()))?;
        Ok(Self { inner })
    }
}

impl MemoryManager for Arm64MemoryManager {
    fn allocate(&self, size: usize) -> Result<*mut u8, PlatformError> {
        // Use the allocate_raw method to get a direct memory pointer
        match self.inner.allocate_raw(size, crate::memory::Tag::Object) {
            Ok(handle) => Ok(handle.data_ptr()),
            Err(MemoryError::OutOfMemory { requested, available }) => {
                Err(PlatformError::OutOfMemory(requested))
            }
            Err(MemoryError::InvalidSize(s)) => Err(PlatformError::InvalidSize(s)),
            Err(e) => Err(PlatformError::AllocationFailed(e.to_string())),
        }
    }
    
    fn deallocate(&self, ptr: *mut u8, size: usize) -> Result<(), PlatformError> {
        // Since our GC handles deallocation automatically, we just validate the pointer
        if ptr.is_null() {
            return Err(PlatformError::InvalidSize(0));
        }
        // For now, just return success as GC will handle cleanup
        Ok(())
    }
    
    fn page_size(&self) -> usize {
        self.inner.page_size()
    }
    
    fn memory_usage(&self) -> usize {
        self.inner.memory_usage()
    }
    
    fn is_valid_memory(&self, ptr: *const u8, size: usize) -> bool {
        self.inner.is_valid_memory(ptr, size)
    }
    
    fn memory_barrier(&self) {
        // ARM64 memory barrier
        std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
    }
    
    fn get_stats(&self) -> Option<MemoryStats> {
        Some(self.inner.get_stats())
    }
}

pub struct WasmMemoryManager {
    inner: RuntimeMemoryManager,
}

impl WasmMemoryManager {
    pub fn new(_runtime_type: crate::runtime::pal::wasm::WasmRuntimeType) -> Result<Self, PlatformError> {
        let stack_manager = Arc::new(RuntimeStack::new());
        let config = MemoryConfig::default();
        let inner = RuntimeMemoryManager::new(config, stack_manager)
            .map_err(|e| PlatformError::AllocationFailed(e.to_string()))?;
        Ok(Self { inner })
    }
}

impl MemoryManager for WasmMemoryManager {
    fn allocate(&self, size: usize) -> Result<*mut u8, PlatformError> {
        // Use the allocate_raw method to get a direct memory pointer
        match self.inner.allocate_raw(size, crate::memory::Tag::Object) {
            Ok(handle) => Ok(handle.data_ptr()),
            Err(MemoryError::OutOfMemory { requested, available }) => {
                Err(PlatformError::OutOfMemory(requested))
            }
            Err(MemoryError::InvalidSize(s)) => Err(PlatformError::InvalidSize(s)),
            Err(e) => Err(PlatformError::AllocationFailed(e.to_string())),
        }
    }
    
    fn deallocate(&self, ptr: *mut u8, size: usize) -> Result<(), PlatformError> {
        // Since our GC handles deallocation automatically, we just validate the pointer
        if ptr.is_null() {
            return Err(PlatformError::InvalidSize(0));
        }
        // For now, just return success as GC will handle cleanup
        Ok(())
    }
    
    fn page_size(&self) -> usize {
        self.inner.page_size()
    }
    
    fn memory_usage(&self) -> usize {
        self.inner.memory_usage()
    }
    
    fn is_valid_memory(&self, ptr: *const u8, size: usize) -> bool {
        self.inner.is_valid_memory(ptr, size)
    }
    
    fn memory_barrier(&self) {
        // WASM memory barrier (usually no-op)
        std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
    }
    
    fn get_stats(&self) -> Option<MemoryStats> {
        Some(self.inner.get_stats())
    }
}
