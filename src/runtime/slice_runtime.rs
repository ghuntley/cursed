//! Runtime support for slice operations in the CURSED language
//!
//! This module provides the runtime infrastructure for slice operations including
//! creation, growth, shrinking, memory management, and integration with the garbage
//! collector. It serves as the bridge between LLVM-generated code and the slice
//! implementation.

use std::ffi::c_void;
use std::ptr::{self, NonNull};
use std::sync::{Arc, RwLock, Mutex};
use std::collections::HashMap;
use std::marker::PhantomData;
use tracing::{debug, error, info, warn, instrument};
use crate::memory::{GarbageCollector, ThreadSafeGc};
// Note: We'll use a different approach since allocator is in a different module structure
use crate::error::Error;
use crate::runtime::slice_utils::*;

/// Header structure representing a slice's metadata
///
/// This follows the standard slice representation with pointer, length, and capacity.
/// The layout is compatible with LLVM's slice representation.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SliceHeader {
    /// Pointer to the data buffer
    pub ptr: *mut c_void,
    /// Current number of elements in the slice
    pub len: usize,
    /// Capacity of the allocated buffer
    pub capacity: usize,
}

impl SliceHeader {
    /// Create a new empty slice header
    pub fn new() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    /// Create a slice header with the given parameters
    pub fn with_params(ptr: *mut c_void, len: usize, capacity: usize) -> Self {
        Self {
            ptr,
            len,
            capacity,
        }
    }

    /// Check if the slice is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Check if the slice header is valid
    pub fn is_valid(&self) -> bool {
        if self.ptr.is_null() {
            return self.len == 0 && self.capacity == 0;
        }
        self.len <= self.capacity
    }

    /// Get the remaining capacity
    pub fn remaining_capacity(&self) -> usize {
        self.capacity.saturating_sub(self.len)
    }
}

/// Statistics for slice operations
#[derive(Debug, Default, Clone)]
pub struct SliceStatistics {
    /// Total number of slices created
    pub slices_created: u64,
    /// Total number of slice growth operations
    pub growths: u64,
    /// Total number of slice shrink operations
    pub shrinks: u64,
    /// Total number of allocations
    pub allocations: u64,
    /// Total number of deallocations
    pub deallocations: u64,
    /// Total bytes allocated for slices
    pub bytes_allocated: u64,
    /// Total bytes deallocated
    pub bytes_deallocated: u64,
    /// Number of failed allocations
    pub allocation_failures: u64,
    /// Number of bounds check failures
    pub bounds_failures: u64,
}

impl SliceStatistics {
    /// Record a slice creation
    pub fn record_creation(&mut self) {
        self.slices_created += 1;
    }

    /// Record a growth operation
    pub fn record_growth(&mut self) {
        self.growths += 1;
    }

    /// Record a shrink operation
    pub fn record_shrink(&mut self) {
        self.shrinks += 1;
    }

    /// Record an allocation
    pub fn record_allocation(&mut self, bytes: usize) {
        self.allocations += 1;
        self.bytes_allocated += bytes as u64;
    }

    /// Record a deallocation
    pub fn record_deallocation(&mut self, bytes: usize) {
        self.deallocations += 1;
        self.bytes_deallocated += bytes as u64;
    }

    /// Record an allocation failure
    pub fn record_allocation_failure(&mut self) {
        self.allocation_failures += 1;
    }

    /// Record a bounds check failure
    pub fn record_bounds_failure(&mut self) {
        self.bounds_failures += 1;
    }
}

/// Configuration for slice runtime behavior
#[derive(Debug, Clone)]
pub struct SliceConfiguration {
    /// Default initial capacity for new slices
    pub default_capacity: usize,
    /// Growth factor for slice expansion (e.g., 2.0 for doubling)
    pub growth_factor: f64,
    /// Maximum capacity limit to prevent excessive memory usage
    pub max_capacity: usize,
    /// Whether to enable bounds checking
    pub bounds_checking: bool,
    /// Whether to zero memory on allocation
    pub zero_memory: bool,
    /// Whether to shrink slices when they become significantly under-utilized
    pub auto_shrink: bool,
    /// Threshold for auto-shrinking (e.g., 0.25 means shrink when len < capacity/4)
    pub shrink_threshold: f64,
}

impl Default for SliceConfiguration {
    fn default() -> Self {
        Self {
            default_capacity: 8,
            growth_factor: 2.0,
            max_capacity: usize::MAX / 2, // Prevent overflow
            bounds_checking: true,
            zero_memory: true,
            auto_shrink: false,
            shrink_threshold: 0.25,
        }
    }
}

/// Main runtime for slice operations
///
/// This provides the core functionality for managing slices in the CURSED language,
/// including memory allocation, growth, and integration with the garbage collector.
pub struct SliceRuntime {
    /// Configuration for slice behavior
    config: SliceConfiguration,
    /// Statistics for monitoring
    stats: Arc<Mutex<SliceStatistics>>,
    /// Registry of active slices for garbage collection
    slice_registry: Arc<RwLock<HashMap<*mut c_void, SliceMetadata>>>,
    /// Reference to the garbage collector
    gc: Option<Arc<GarbageCollector>>,
}

/// Metadata about an allocated slice
#[derive(Debug, Clone)]
struct SliceMetadata {
    /// Element size in bytes
    element_size: usize,
    /// Current capacity
    capacity: usize,
    /// Whether the slice is managed by GC
    gc_managed: bool,
    /// Creation timestamp for debugging
    created_at: std::time::Instant,
}

impl SliceRuntime {
    /// Create a new slice runtime with default configuration
    pub fn new() -> Self {
        Self {
            config: SliceConfiguration::default(),
            stats: Arc::new(Mutex::new(SliceStatistics::default())),
            slice_registry: Arc::new(RwLock::new(HashMap::new())),
            gc: None,
        }
    }

    /// Create a new slice runtime with custom configuration
    pub fn with_config(config: SliceConfiguration) -> Self {
        Self {
            config,
            stats: Arc::new(Mutex::new(SliceStatistics::default())),
            slice_registry: Arc::new(RwLock::new(HashMap::new())),
            gc: None,
        }
    }

    /// Set the garbage collector
    pub fn set_gc(&mut self, gc: Arc<GarbageCollector>) {
        self.gc = Some(gc);
    }

    /// Get a copy of the current statistics
    pub fn get_statistics(&self) -> SliceStatistics {
        self.stats.lock().unwrap().clone()
    }

    /// Reset statistics
    pub fn reset_statistics(&self) {
        *self.stats.lock().unwrap() = SliceStatistics::default();
    }

    /// Create a new slice with the specified element size and initial capacity
    #[instrument(skip(self))]
    pub fn create_slice(&self, element_size: usize, initial_capacity: Option<usize>) -> Result<SliceHeader, Error> {
        let capacity = initial_capacity.unwrap_or(self.config.default_capacity);
        
        debug!(
            element_size = element_size,
            capacity = capacity,
            "Creating new slice"
        );

        if capacity > self.config.max_capacity {
            self.stats.lock().unwrap().record_allocation_failure();
            return Err(Error::new("SliceError", "Slice capacity exceeds maximum limit".to_string(), None));
        }

        let total_size = element_size.checked_mul(capacity)
            .ok_or_else(|| Error::new("SliceError", "Slice size overflow".to_string(), None))?;

        // Allocate memory
        let ptr = if let Some(ref gc) = self.gc {
            // Use garbage collector for allocation
            self.allocate_with_gc(total_size)?
        } else {
            // Use system allocator
            self.allocate_system(total_size)?
        };

        // Register the slice
        let metadata = SliceMetadata {
            element_size,
            capacity,
            gc_managed: self.gc.is_some(),
            created_at: std::time::Instant::now(),
        };

        {
            let mut registry = self.slice_registry.write().unwrap();
            registry.insert(ptr, metadata);
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.record_creation();
            stats.record_allocation(total_size);
        }

        // Zero memory if configured
        if self.config.zero_memory {
            unsafe {
                ptr::write_bytes(ptr as *mut u8, 0, total_size);
            }
        }

        Ok(SliceHeader::with_params(ptr, 0, capacity))
    }

    /// Grow a slice to accommodate more elements
    #[instrument(skip(self))]
    pub fn grow_slice(&self, header: &mut SliceHeader, element_size: usize, min_additional: usize) -> Result<(), Error> {
        let required_capacity = header.len.checked_add(min_additional)
            .ok_or_else(|| Error::new("SliceError", "Slice length overflow".to_string(), None))?;

        if required_capacity <= header.capacity {
            return Ok(()); // No growth needed
        }

        let new_capacity = self.calculate_new_capacity(header.capacity, required_capacity)?;
        
        debug!(
            old_capacity = header.capacity,
            new_capacity = new_capacity,
            element_size = element_size,
            "Growing slice"
        );

        self.resize_slice(header, element_size, new_capacity)
    }

    /// Shrink a slice to reduce memory usage
    #[instrument(skip(self))]
    pub fn shrink_slice(&self, header: &mut SliceHeader, element_size: usize) -> Result<(), Error> {
        if !self.config.auto_shrink {
            return Ok(()); // Auto-shrinking disabled
        }

        let threshold_capacity = (header.len as f64 / self.config.shrink_threshold) as usize;
        if header.capacity <= threshold_capacity {
            return Ok(()); // No shrinking needed
        }

        let new_capacity = (header.len as f64 * (1.0 / self.config.shrink_threshold)) as usize;
        let new_capacity = new_capacity.max(self.config.default_capacity);

        debug!(
            old_capacity = header.capacity,
            new_capacity = new_capacity,
            len = header.len,
            "Shrinking slice"
        );

        self.resize_slice(header, element_size, new_capacity)
    }

    /// Resize a slice to a specific capacity
    #[instrument(skip(self))]
    fn resize_slice(&self, header: &mut SliceHeader, element_size: usize, new_capacity: usize) -> Result<(), Error> {
        let old_size = element_size * header.capacity;
        let new_size = element_size * new_capacity;

        // Allocate new memory
        let new_ptr = if let Some(ref gc) = self.gc {
            self.allocate_with_gc(new_size)?
        } else {
            self.allocate_system(new_size)?
        };

        // Copy existing data
        if !header.ptr.is_null() && header.len > 0 {
            let copy_len = header.len.min(new_capacity);
            let copy_size = element_size * copy_len;
            
            unsafe {
                ptr::copy_nonoverlapping(
                    header.ptr as *const u8,
                    new_ptr as *mut u8,
                    copy_size
                );
            }
        }

        // Zero new memory if configured
        if self.config.zero_memory && new_capacity > header.len {
            let zero_start = unsafe { (new_ptr as *mut u8).add(element_size * header.len) };
            let zero_size = element_size * (new_capacity - header.len);
            unsafe {
                ptr::write_bytes(zero_start, 0, zero_size);
            }
        }

        // Deallocate old memory
        if !header.ptr.is_null() {
            self.deallocate_memory(header.ptr, old_size);
        }

        // Update registry
        {
            let mut registry = self.slice_registry.write().unwrap();
            if let Some(mut metadata) = registry.remove(&header.ptr) {
                metadata.capacity = new_capacity;
                registry.insert(new_ptr, metadata);
            }
        }

        // Update header
        header.ptr = new_ptr;
        header.capacity = new_capacity;

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            if new_capacity > header.capacity {
                stats.record_growth();
            } else {
                stats.record_shrink();
            }
            stats.record_allocation(new_size);
            stats.record_deallocation(old_size);
        }

        Ok(())
    }

    /// Calculate the new capacity for growth
    fn calculate_new_capacity(&self, current: usize, required: usize) -> Result<usize, Error> {
        let grown = (current as f64 * self.config.growth_factor) as usize;
        let new_capacity = grown.max(required).max(self.config.default_capacity);
        
        if new_capacity > self.config.max_capacity {
            return Err(Error::new("SliceError", "Slice capacity exceeds maximum limit".to_string(), None));
        }

        Ok(new_capacity)
    }

    /// Allocate memory using garbage collector
    fn allocate_with_gc(&self, size: usize) -> Result<*mut c_void, Error> {
        // This would integrate with the actual GC implementation
        // For now, fall back to system allocation
        self.allocate_system(size)
    }

    /// Allocate memory using system allocator
    fn allocate_system(&self, size: usize) -> Result<*mut c_void, Error> {
        use std::alloc::{alloc, Layout};

        if size == 0 {
            return Ok(std::ptr::null_mut());
        }

        let layout = Layout::from_size_align(size, 8)
            .map_err(|_| Error::new("SliceError", "Invalid memory layout".to_string(), None))?;

        let ptr = unsafe { alloc(layout) };
        
        if ptr.is_null() {
            Err(Error::new("SliceError", "Memory allocation failed".to_string(), None))
        } else {
            Ok(ptr as *mut c_void)
        }
    }

    /// Deallocate memory
    fn deallocate_memory(&self, ptr: *mut c_void, size: usize) {
        if ptr.is_null() || size == 0 {
            return;
        }

        if let Some(ref gc) = self.gc {
            // Would integrate with GC deallocation
            // For now, use system deallocation
        }

        use std::alloc::{dealloc, Layout};
        
        if let Ok(layout) = Layout::from_size_align(size, 8) {
            unsafe {
                dealloc(ptr as *mut u8, layout);
            }
        }
    }

    /// Deallocate a slice completely
    #[instrument(skip(self))]
    pub fn deallocate_slice(&self, header: &mut SliceHeader, element_size: usize) {
        if header.ptr.is_null() {
            return;
        }

        let size = element_size * header.capacity;
        
        debug!(
            ptr = ?header.ptr,
            capacity = header.capacity,
            element_size = element_size,
            "Deallocating slice"
        );

        // Remove from registry
        {
            let mut registry = self.slice_registry.write().unwrap();
            registry.remove(&header.ptr);
        }

        // Deallocate memory
        self.deallocate_memory(header.ptr, size);

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.record_deallocation(size);
        }

        // Clear header
        header.ptr = std::ptr::null_mut();
        header.len = 0;
        header.capacity = 0;
    }

    /// Check if an index is within bounds
    pub fn check_bounds(&self, header: &SliceHeader, index: usize) -> bool {
        if !self.config.bounds_checking {
            return true;
        }

        let in_bounds = index < header.len;
        if !in_bounds {
            self.stats.lock().unwrap().record_bounds_failure();
        }
        in_bounds
    }

    /// Get the number of registered slices
    pub fn slice_count(&self) -> usize {
        self.slice_registry.read().unwrap().len()
    }

    /// Clean up all slices (for shutdown)
    pub fn cleanup(&self) {
        let registry = self.slice_registry.read().unwrap();
        for (ptr, metadata) in registry.iter() {
            let size = metadata.element_size * metadata.capacity;
            self.deallocate_memory(*ptr, size);
        }
        drop(registry);

        self.slice_registry.write().unwrap().clear();
    }
}

impl Default for SliceRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SliceRuntime {
    fn drop(&mut self) {
        self.cleanup();
    }
}

/// Thread-safe wrapper for SliceRuntime
pub type ThreadSafeSliceRuntime = Arc<RwLock<SliceRuntime>>;

/// Create a thread-safe slice runtime
pub fn create_thread_safe_runtime() -> ThreadSafeSliceRuntime {
    Arc::new(RwLock::new(SliceRuntime::new()))
}

/// Create a thread-safe slice runtime with configuration
pub fn create_thread_safe_runtime_with_config(config: SliceConfiguration) -> ThreadSafeSliceRuntime {
    Arc::new(RwLock::new(SliceRuntime::with_config(config)))
}

// ================================================================================================
// FFI Functions for LLVM Integration
// ================================================================================================

/// Create a new slice from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_slice_create(
    element_size: u64,
    initial_capacity: u64,
    runtime_ptr: *mut c_void,
) -> SliceHeader {
    debug!(
        element_size = element_size,
        initial_capacity = initial_capacity,
        "Creating slice via FFI"
    );

    if runtime_ptr.is_null() {
        error!("Null runtime pointer in cursed_slice_create");
        return SliceHeader::new();
    }

    let runtime = unsafe { &*(runtime_ptr as *const SliceRuntime) };
    
    match runtime.create_slice(element_size as usize, Some(initial_capacity as usize)) {
        Ok(header) => {
            debug!(ptr = ?header.ptr, capacity = header.capacity, "Slice created successfully");
            header
        },
        Err(e) => {
            error!(error = ?e, "Failed to create slice");
            SliceHeader::new()
        }
    }
}

/// Grow a slice from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_slice_grow(
    header_ptr: *mut SliceHeader,
    element_size: u64,
    min_additional: u64,
    runtime_ptr: *mut c_void,
) -> i32 {
    if header_ptr.is_null() || runtime_ptr.is_null() {
        error!("Null pointer in cursed_slice_grow");
        return -1;
    }

    let header = unsafe { &mut *header_ptr };
    let runtime = unsafe { &*(runtime_ptr as *const SliceRuntime) };

    match runtime.grow_slice(header, element_size as usize, min_additional as usize) {
        Ok(_) => {
            debug!(new_capacity = header.capacity, "Slice grown successfully");
            0
        },
        Err(e) => {
            error!(error = ?e, "Failed to grow slice");
            -1
        }
    }
}

/// Deallocate a slice from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_slice_deallocate(
    header_ptr: *mut SliceHeader,
    element_size: u64,
    runtime_ptr: *mut c_void,
) {
    if header_ptr.is_null() || runtime_ptr.is_null() {
        error!("Null pointer in cursed_slice_deallocate");
        return;
    }

    let header = unsafe { &mut *header_ptr };
    let runtime = unsafe { &*(runtime_ptr as *const SliceRuntime) };

    debug!(ptr = ?header.ptr, capacity = header.capacity, "Deallocating slice");
    runtime.deallocate_slice(header, element_size as usize);
}

/// Copy elements between slices from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_slice_copy(
    src_ptr: *const SliceHeader,
    dst_ptr: *mut SliceHeader,
    element_size: u64,
    src_start: u64,
    dst_start: u64,
    count: u64,
) -> u64 {
    if src_ptr.is_null() || dst_ptr.is_null() {
        error!("Null pointer in cursed_slice_copy");
        return 0;
    }

    let src = unsafe { &*src_ptr };
    let dst = unsafe { &mut *dst_ptr };

    unsafe {
        slice_copy(
            src,
            dst,
            element_size as usize,
            src_start as usize,
            dst_start as usize,
            count as usize,
        ) as u64
    }
}

/// Fill a slice with a value from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_slice_fill(
    slice_ptr: *mut SliceHeader,
    element_size: u64,
    value_ptr: *const c_void,
    start: u64,
    count: u64,
) -> u64 {
    if slice_ptr.is_null() || value_ptr.is_null() {
        error!("Null pointer in cursed_slice_fill");
        return 0;
    }

    let slice = unsafe { &mut *slice_ptr };

    unsafe {
        slice_fill(
            slice,
            element_size as usize,
            value_ptr,
            start as usize,
            count as usize,
        ) as u64
    }
}

/// Get element pointer from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_slice_get_element_ptr(
    slice_ptr: *const SliceHeader,
    element_size: u64,
    index: u64,
) -> *mut c_void {
    if slice_ptr.is_null() {
        error!("Null pointer in cursed_slice_get_element_ptr");
        return std::ptr::null_mut();
    }

    let slice = unsafe { &*slice_ptr };

    unsafe {
        slice_get_element_ptr(slice, element_size as usize, index as usize)
    }
}

/// Check bounds from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_slice_check_bounds(
    slice_ptr: *const SliceHeader,
    index: u64,
    runtime_ptr: *mut c_void,
) -> i32 {
    if slice_ptr.is_null() || runtime_ptr.is_null() {
        error!("Null pointer in cursed_slice_check_bounds");
        return 0;
    }

    let slice = unsafe { &*slice_ptr };
    let runtime = unsafe { &*(runtime_ptr as *const SliceRuntime) };

    if runtime.check_bounds(slice, index as usize) {
        1
    } else {
        0
    }
}

/// Set slice length from LLVM-generated code
///
/// # Safety
///
/// This function should only be called from LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_slice_set_length(
    slice_ptr: *mut SliceHeader,
    new_len: u64,
) -> i32 {
    if slice_ptr.is_null() {
        error!("Null pointer in cursed_slice_set_length");
        return 0;
    }

    let slice = unsafe { &mut *slice_ptr };

    if slice_set_length(slice, new_len as usize) {
        1
    } else {
        0
    }
}

/// Create a global slice runtime instance for FFI
static mut GLOBAL_SLICE_RUNTIME: Option<SliceRuntime> = None;
static GLOBAL_RUNTIME_INIT: std::sync::Once = std::sync::Once::new();

/// Initialize the global slice runtime
#[no_mangle]
pub extern "C" fn cursed_slice_runtime_init() -> *mut c_void {
    GLOBAL_RUNTIME_INIT.call_once(|| {
        unsafe {
            GLOBAL_SLICE_RUNTIME = Some(SliceRuntime::new());
        }
    });

    unsafe {
        GLOBAL_SLICE_RUNTIME.as_ref().unwrap() as *const SliceRuntime as *mut c_void
    }
}

/// Get the global slice runtime pointer
#[no_mangle]
pub extern "C" fn cursed_slice_runtime_get() -> *mut c_void {
    unsafe {
        match GLOBAL_SLICE_RUNTIME.as_ref() {
            Some(runtime) => runtime as *const SliceRuntime as *mut c_void,
            None => {
                error!("Global slice runtime not initialized");
                std::ptr::null_mut()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_header_creation() {
        let header = SliceHeader::new();
        assert!(header.ptr.is_null());
        assert_eq!(header.len, 0);
        assert_eq!(header.capacity, 0);
        assert!(header.is_empty());
        assert!(header.is_valid());
    }

    #[test]
    fn test_slice_runtime_creation() {
        let runtime = SliceRuntime::new();
        assert_eq!(runtime.slice_count(), 0);
        
        let stats = runtime.get_statistics();
        assert_eq!(stats.slices_created, 0);
    }

    #[test]
    fn test_slice_creation() {
        let runtime = SliceRuntime::new();
        let result = runtime.create_slice(4, Some(10));
        
        assert!(result.is_ok());
        let header = result.unwrap();
        assert!(!header.ptr.is_null());
        assert_eq!(header.len, 0);
        assert_eq!(header.capacity, 10);
        assert!(header.is_valid());
    }
}
