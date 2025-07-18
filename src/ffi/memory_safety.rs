//! Memory safety management for FFI operations
//!
//! This module provides comprehensive memory safety features for FFI operations,
//! including memory validation, leak detection, and automatic cleanup.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use crate::error::CursedError;

/// Memory safety manager for FFI operations
pub struct MemorySafetyManager {
    /// Track allocated memory regions
    allocations: Arc<RwLock<HashMap<*mut std::ffi::c_void, AllocationInfo>>>,
    
    /// Memory pool for efficient allocation
    memory_pool: Arc<Mutex<MemoryPool>>,
    
    /// Garbage collection integration
    gc_integration: Arc<Mutex<GcIntegration>>,
    
    /// Memory usage statistics
    stats: Arc<Mutex<MemoryStats>>,
    
    /// Safety configuration
    config: SafetyConfig,
}

/// Information about a memory allocation
#[derive(Debug, Clone)]
struct AllocationInfo {
    /// Size of the allocation
    size: usize,
    
    /// Allocation timestamp
    allocated_at: Instant,
    
    /// Allocation source
    source: AllocationSource,
    
    /// Whether the allocation is pinned (protected from GC)
    is_pinned: bool,
    
    /// Reference count for shared allocations
    ref_count: usize,
    
    /// Cleanup function
    cleanup_fn: Option<Box<dyn FnOnce() + Send + Sync>>,
}

/// Source of memory allocation
#[derive(Debug, Clone)]
enum AllocationSource {
    /// Allocated for C interop
    CInterop,
    
    /// Allocated for Python interop
    PythonInterop,
    
    /// Allocated for Go interop
    GoInterop,
    
    /// Allocated for WASM interop
    WasmInterop,
    
    /// Allocated for callback
    Callback,
    
    /// Allocated for temporary use
    Temporary,
}

/// Memory pool for efficient allocation
struct MemoryPool {
    /// Small allocations pool (< 1KB)
    small_pool: Vec<*mut std::ffi::c_void>,
    
    /// Medium allocations pool (1KB - 64KB)
    medium_pool: Vec<*mut std::ffi::c_void>,
    
    /// Large allocations pool (> 64KB)
    large_pool: Vec<*mut std::ffi::c_void>,
    
    /// Pool configuration
    config: PoolConfig,
}

/// Memory pool configuration
struct PoolConfig {
    /// Maximum size for small allocations
    small_threshold: usize,
    
    /// Maximum size for medium allocations
    medium_threshold: usize,
    
    /// Maximum number of pooled allocations
    max_pooled: usize,
    
    /// Pool cleanup interval
    cleanup_interval: Duration,
}

/// Garbage collection integration
struct GcIntegration {
    /// Pinned memory regions
    pinned_regions: HashMap<*mut std::ffi::c_void, usize>,
    
    /// GC roots for FFI objects
    gc_roots: Vec<*mut std::ffi::c_void>,
    
    /// Weak references to FFI objects
    weak_refs: Vec<*mut std::ffi::c_void>,
}

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// Total allocated memory
    pub total_allocated: usize,
    
    /// Peak memory usage
    pub peak_usage: usize,
    
    /// Number of active allocations
    pub active_allocations: usize,
    
    /// Number of leaked allocations
    pub leaked_allocations: usize,
    
    /// Total number of allocations
    pub total_allocations: u64,
    
    /// Total number of deallocations
    pub total_deallocations: u64,
    
    /// Average allocation size
    pub average_allocation_size: f64,
    
    /// Pool hit rate
    pub pool_hit_rate: f64,
}

/// Safety configuration
#[derive(Debug, Clone)]
pub struct SafetyConfig {
    /// Enable memory leak detection
    pub leak_detection: bool,
    
    /// Enable buffer overflow protection
    pub buffer_protection: bool,
    
    /// Enable pointer validation
    pub pointer_validation: bool,
    
    /// Enable automatic cleanup
    pub automatic_cleanup: bool,
    
    /// Memory allocation limit
    pub allocation_limit: Option<usize>,
    
    /// Allocation timeout
    pub allocation_timeout: Option<Duration>,
    
    /// Enable memory poisoning
    pub memory_poisoning: bool,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            leak_detection: true,
            buffer_protection: true,
            pointer_validation: true,
            automatic_cleanup: true,
            allocation_limit: Some(1024 * 1024 * 1024), // 1GB limit
            allocation_timeout: Some(Duration::from_secs(60)),
            memory_poisoning: false, // Disabled by default for performance
        }
    }
}

impl MemorySafetyManager {
    /// Create a new memory safety manager
    pub fn new() -> Self {
        Self {
            allocations: Arc::new(RwLock::new(HashMap::new())),
            memory_pool: Arc::new(Mutex::new(MemoryPool::new())),
            gc_integration: Arc::new(Mutex::new(GcIntegration::new())),
            stats: Arc::new(Mutex::new(MemoryStats::new())),
            config: SafetyConfig::default(),
        }
    }
    
    /// Create memory safety manager with custom configuration
    pub fn with_config(config: SafetyConfig) -> Self {
        Self {
            allocations: Arc::new(RwLock::new(HashMap::new())),
            memory_pool: Arc::new(Mutex::new(MemoryPool::new())),
            gc_integration: Arc::new(Mutex::new(GcIntegration::new())),
            stats: Arc::new(Mutex::new(MemoryStats::new())),
            config,
        }
    }
    
    /// Allocate memory with safety checks
    pub fn allocate(
        &self,
        size: usize,
        source: AllocationSource,
    ) -> Result<*mut std::ffi::c_void, CursedError> {
        // Check allocation limits
        if let Some(limit) = self.config.allocation_limit {
            let stats = self.stats.lock().unwrap();
            if stats.total_allocated + size > limit {
                return Err(CursedError::General(
                    "Memory allocation limit exceeded".to_string()
                ));
            }
        }
        
        // Try to get memory from pool first
        let ptr = {
            let mut pool = self.memory_pool.lock().unwrap();
            pool.allocate(size)
        };
        
        let ptr = if let Some(ptr) = ptr {
            ptr
        } else {
            // Allocate new memory
            let ptr = unsafe { libc::malloc(size) };
            if ptr.is_null() {
                return Err(CursedError::General(
                    "Memory allocation failed".to_string()
                ));
            }
            ptr
        };
        
        // Poison memory if enabled
        if self.config.memory_poisoning {
            unsafe {
                std::ptr::write_bytes(ptr as *mut u8, 0xDE, size);
            }
        }
        
        // Track allocation
        let allocation_info = AllocationInfo {
            size,
            allocated_at: Instant::now(),
            source,
            is_pinned: false,
            ref_count: 1,
            cleanup_fn: None,
        };
        
        {
            let mut allocations = self.allocations.write().unwrap();
            allocations.insert(ptr, allocation_info);
        }
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_allocated += size;
            stats.active_allocations += 1;
            stats.total_allocations += 1;
            
            if stats.total_allocated > stats.peak_usage {
                stats.peak_usage = stats.total_allocated;
            }
            
            stats.average_allocation_size = 
                (stats.average_allocation_size * (stats.total_allocations - 1) as f64 + size as f64) 
                / stats.total_allocations as f64;
        }
        
        Ok(ptr)
    }
    
    /// Deallocate memory with safety checks
    pub fn deallocate(&self, ptr: *mut std::ffi::c_void) -> Result<(), CursedError> {
        // Validate pointer
        if self.config.pointer_validation {
            self.validate_pointer(ptr)?;
        }
        
        // Get allocation info
        let allocation_info = {
            let mut allocations = self.allocations.write().unwrap();
            allocations.remove(&ptr)
        };
        
        if let Some(info) = allocation_info {
            // Decrease reference count
            if info.ref_count > 1 {
                // Still has references, put it back
                let mut new_info = info.clone();
                new_info.ref_count -= 1;
                let mut allocations = self.allocations.write().unwrap();
                allocations.insert(ptr, new_info);
                return Ok(());
            }
            
            // Execute cleanup function if present
            if let Some(cleanup_fn) = info.cleanup_fn {
                cleanup_fn();
            }
            
            // Poison memory before deallocation
            if self.config.memory_poisoning {
                unsafe {
                    std::ptr::write_bytes(ptr as *mut u8, 0xDD, info.size);
                }
            }
            
            // Try to return to pool
            let returned_to_pool = {
                let mut pool = self.memory_pool.lock().unwrap();
                pool.deallocate(ptr, info.size)
            };
            
            if !returned_to_pool {
                // Free memory directly
                unsafe {
                    libc::free(ptr);
                }
            }
            
            // Update statistics
            {
                let mut stats = self.stats.lock().unwrap();
                stats.total_allocated -= info.size;
                stats.active_allocations -= 1;
                stats.total_deallocations += 1;
            }
            
            Ok(())
        } else {
            Err(CursedError::General("Invalid pointer for deallocation".to_string()))
        }
    }
    
    /// Validate a pointer
    pub fn validate_pointer(&self, ptr: *mut std::ffi::c_void) -> Result<(), CursedError> {
        if ptr.is_null() {
            return Err(CursedError::General("Null pointer".to_string()));
        }
        
        // Check if pointer is in our allocation registry
        let allocations = self.allocations.read().unwrap();
        if !allocations.contains_key(&ptr) {
            return Err(CursedError::General("Untracked pointer".to_string()));
        }
        
        // Additional platform-specific validation could go here
        
        Ok(())
    }
    
    /// Pin memory to prevent garbage collection
    pub fn pin_memory(&self, ptr: *mut std::ffi::c_void, size: usize) -> Result<(), CursedError> {
        // Update allocation info
        {
            let mut allocations = self.allocations.write().unwrap();
            if let Some(info) = allocations.get_mut(&ptr) {
                info.is_pinned = true;
            } else {
                return Err(CursedError::General("Pointer not found for pinning".to_string()));
            }
        }
        
        // Add to GC integration
        {
            let mut gc = self.gc_integration.lock().unwrap();
            gc.pinned_regions.insert(ptr, size);
        }
        
        Ok(())
    }
    
    /// Unpin memory
    pub fn unpin_memory(&self, ptr: *mut std::ffi::c_void) -> Result<(), CursedError> {
        // Update allocation info
        {
            let mut allocations = self.allocations.write().unwrap();
            if let Some(info) = allocations.get_mut(&ptr) {
                info.is_pinned = false;
            } else {
                return Err(CursedError::General("Pointer not found for unpinning".to_string()));
            }
        }
        
        // Remove from GC integration
        {
            let mut gc = self.gc_integration.lock().unwrap();
            gc.pinned_regions.remove(&ptr);
        }
        
        Ok(())
    }
    
    /// Create a shared reference to memory
    pub fn create_shared_ref(&self, ptr: *mut std::ffi::c_void) -> Result<(), CursedError> {
        let mut allocations = self.allocations.write().unwrap();
        if let Some(info) = allocations.get_mut(&ptr) {
            info.ref_count += 1;
            Ok(())
        } else {
            Err(CursedError::General("Pointer not found for shared reference".to_string()))
        }
    }
    
    /// Set cleanup function for allocation
    pub fn set_cleanup_function<F>(&self, ptr: *mut std::ffi::c_void, cleanup_fn: F) -> Result<(), CursedError>
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        let mut allocations = self.allocations.write().unwrap();
        if let Some(info) = allocations.get_mut(&ptr) {
            info.cleanup_fn = Some(Box::new(cleanup_fn));
            Ok(())
        } else {
            Err(CursedError::General("Pointer not found for cleanup function".to_string()))
        }
    }
    
    /// Detect memory leaks
    pub fn detect_leaks(&self) -> Vec<LeakInfo> {
        if !self.config.leak_detection {
            return Vec::new();
        }
        
        let mut leaks = Vec::new();
        let allocations = self.allocations.read().unwrap();
        let now = Instant::now();
        
        for (ptr, info) in allocations.iter() {
            // Consider allocation a leak if it's been alive for more than 10 minutes
            if now.duration_since(info.allocated_at) > Duration::from_secs(600) {
                leaks.push(LeakInfo {
                    ptr: *ptr,
                    size: info.size,
                    age: now.duration_since(info.allocated_at),
                    source: info.source.clone(),
                });
            }
        }
        
        // Update leak statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.leaked_allocations = leaks.len();
        }
        
        leaks
    }
    
    /// Cleanup all allocations
    pub fn cleanup(&self) -> Result<(), CursedError> {
        let mut allocations = self.allocations.write().unwrap();
        let mut errors = Vec::new();
        
        // Cleanup all allocations
        for (ptr, info) in allocations.drain() {
            // Execute cleanup function if present
            if let Some(cleanup_fn) = info.cleanup_fn {
                cleanup_fn();
            }
            
            // Poison memory before deallocation
            if self.config.memory_poisoning {
                unsafe {
                    std::ptr::write_bytes(ptr as *mut u8, 0xDD, info.size);
                }
            }
            
            // Free memory
            unsafe {
                libc::free(ptr);
            }
        }
        
        // Clear GC integration
        {
            let mut gc = self.gc_integration.lock().unwrap();
            gc.pinned_regions.clear();
            gc.gc_roots.clear();
            gc.weak_refs.clear();
        }
        
        // Reset statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_allocated = 0;
            stats.active_allocations = 0;
        }
        
        Ok(())
    }
    
    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }
    
    /// Check if allocation is valid
    pub fn is_valid_allocation(&self, ptr: *mut std::ffi::c_void) -> bool {
        let allocations = self.allocations.read().unwrap();
        allocations.contains_key(&ptr)
    }
    
    /// Get allocation size
    pub fn get_allocation_size(&self, ptr: *mut std::ffi::c_void) -> Option<usize> {
        let allocations = self.allocations.read().unwrap();
        allocations.get(&ptr).map(|info| info.size)
    }
    
    /// Check if allocation is pinned
    pub fn is_pinned(&self, ptr: *mut std::ffi::c_void) -> bool {
        let allocations = self.allocations.read().unwrap();
        allocations.get(&ptr).map(|info| info.is_pinned).unwrap_or(false)
    }
}

/// Information about a memory leak
#[derive(Debug, Clone)]
pub struct LeakInfo {
    pub ptr: *mut std::ffi::c_void,
    pub size: usize,
    pub age: Duration,
    pub source: AllocationSource,
}

impl MemoryPool {
    fn new() -> Self {
        Self {
            small_pool: Vec::new(),
            medium_pool: Vec::new(),
            large_pool: Vec::new(),
            config: PoolConfig {
                small_threshold: 1024,
                medium_threshold: 64 * 1024,
                max_pooled: 1000,
                cleanup_interval: Duration::from_secs(300),
            },
        }
    }
    
    fn allocate(&mut self, size: usize) -> Option<*mut std::ffi::c_void> {
        let pool = if size <= self.config.small_threshold {
            &mut self.small_pool
        } else if size <= self.config.medium_threshold {
            &mut self.medium_pool
        } else {
            &mut self.large_pool
        };
        
        pool.pop()
    }
    
    fn deallocate(&mut self, ptr: *mut std::ffi::c_void, size: usize) -> bool {
        let pool = if size <= self.config.small_threshold {
            &mut self.small_pool
        } else if size <= self.config.medium_threshold {
            &mut self.medium_pool
        } else {
            &mut self.large_pool
        };
        
        if pool.len() < self.config.max_pooled {
            pool.push(ptr);
            true
        } else {
            false
        }
    }
}

impl GcIntegration {
    fn new() -> Self {
        Self {
            pinned_regions: HashMap::new(),
            gc_roots: Vec::new(),
            weak_refs: Vec::new(),
        }
    }
}

impl MemoryStats {
    fn new() -> Self {
        Self {
            total_allocated: 0,
            peak_usage: 0,
            active_allocations: 0,
            leaked_allocations: 0,
            total_allocations: 0,
            total_deallocations: 0,
            average_allocation_size: 0.0,
            pool_hit_rate: 0.0,
        }
    }
}

impl Default for MemorySafetyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_allocation() {
        let manager = MemorySafetyManager::new();
        
        let ptr = manager.allocate(1024, AllocationSource::CInterop).unwrap();
        assert!(!ptr.is_null());
        assert!(manager.is_valid_allocation(ptr));
        assert_eq!(manager.get_allocation_size(ptr), Some(1024));
        
        manager.deallocate(ptr).unwrap();
        assert!(!manager.is_valid_allocation(ptr));
    }
    
    #[test]
    fn test_memory_pinning() {
        let manager = MemorySafetyManager::new();
        
        let ptr = manager.allocate(1024, AllocationSource::CInterop).unwrap();
        assert!(!manager.is_pinned(ptr));
        
        manager.pin_memory(ptr, 1024).unwrap();
        assert!(manager.is_pinned(ptr));
        
        manager.unpin_memory(ptr).unwrap();
        assert!(!manager.is_pinned(ptr));
        
        manager.deallocate(ptr).unwrap();
    }
    
    #[test]
    fn test_shared_references() {
        let manager = MemorySafetyManager::new();
        
        let ptr = manager.allocate(1024, AllocationSource::CInterop).unwrap();
        
        // Create shared reference
        manager.create_shared_ref(ptr).unwrap();
        
        // First deallocation should not free memory (ref count > 1)
        manager.deallocate(ptr).unwrap();
        assert!(manager.is_valid_allocation(ptr));
        
        // Second deallocation should free memory
        manager.deallocate(ptr).unwrap();
        assert!(!manager.is_valid_allocation(ptr));
    }
    
    #[test]
    fn test_memory_statistics() {
        let manager = MemorySafetyManager::new();
        
        let ptr1 = manager.allocate(1024, AllocationSource::CInterop).unwrap();
        let ptr2 = manager.allocate(2048, AllocationSource::PythonInterop).unwrap();
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_allocated, 3072);
        assert_eq!(stats.active_allocations, 2);
        assert_eq!(stats.total_allocations, 2);
        
        manager.deallocate(ptr1).unwrap();
        manager.deallocate(ptr2).unwrap();
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_allocated, 0);
        assert_eq!(stats.active_allocations, 0);
        assert_eq!(stats.total_deallocations, 2);
    }
    
    #[test]
    fn test_cleanup_function() {
        let manager = MemorySafetyManager::new();
        
        let ptr = manager.allocate(1024, AllocationSource::CInterop).unwrap();
        
        let cleanup_called = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let cleanup_called_clone = cleanup_called.clone();
        
        manager.set_cleanup_function(ptr, move || {
            cleanup_called_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        }).unwrap();
        
        manager.deallocate(ptr).unwrap();
        
        assert!(cleanup_called.load(std::sync::atomic::Ordering::SeqCst));
    }
    
    #[test]
    fn test_memory_limit() {
        let config = SafetyConfig {
            allocation_limit: Some(1024),
            ..Default::default()
        };
        
        let manager = MemorySafetyManager::with_config(config);
        
        let ptr1 = manager.allocate(512, AllocationSource::CInterop).unwrap();
        let ptr2 = manager.allocate(512, AllocationSource::CInterop).unwrap();
        
        // This should fail due to limit
        let result = manager.allocate(1, AllocationSource::CInterop);
        assert!(result.is_err());
        
        manager.deallocate(ptr1).unwrap();
        manager.deallocate(ptr2).unwrap();
    }
}
